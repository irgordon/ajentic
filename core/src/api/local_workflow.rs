use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalHarnessWorkflowStatus {
    Completed,
    Blocked,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalHarnessWorkflowReason {
    CompletedInMemory,
    RuntimeConfigInvalid,
    ProviderInvocationFailed,
    ControlledFlowBlocked,
    ControlledFlowRejected,
    ApplicationStateInvalid,
    ReadProjectionFailed,
}

impl LocalHarnessWorkflowReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::CompletedInMemory => "completed_in_memory",
            Self::RuntimeConfigInvalid => "runtime_config_invalid",
            Self::ProviderInvocationFailed => "provider_invocation_failed",
            Self::ControlledFlowBlocked => "controlled_flow_blocked",
            Self::ControlledFlowRejected => "controlled_flow_rejected",
            Self::ApplicationStateInvalid => "application_state_invalid",
            Self::ReadProjectionFailed => "read_projection_failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalHarnessWorkflowRequest {
    pub workflow_id: String,
    pub run_id: String,
    pub projection_id: String,
    pub operator_id: String,
    pub prompt_summary: String,
    pub context_packet_id: String,
    pub memory_snapshot_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalHarnessWorkflowError {
    EmptyWorkflowId,
    EmptyRunId,
    EmptyProjectionId,
    EmptyOperatorId,
    EmptyPromptSummary,
    EmptyContextPacketId,
    EmptyMemorySnapshotId,
}

impl LocalHarnessWorkflowError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyWorkflowId => "empty_workflow_id",
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyProjectionId => "empty_projection_id",
            Self::EmptyOperatorId => "empty_operator_id",
            Self::EmptyPromptSummary => "empty_prompt_summary",
            Self::EmptyContextPacketId => "empty_context_packet_id",
            Self::EmptyMemorySnapshotId => "empty_memory_snapshot_id",
        }
    }
}

impl LocalHarnessWorkflowRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workflow_id: impl Into<String>,
        run_id: impl Into<String>,
        projection_id: impl Into<String>,
        operator_id: impl Into<String>,
        prompt_summary: impl Into<String>,
        context_packet_id: impl Into<String>,
        memory_snapshot_id: impl Into<String>,
    ) -> Result<Self, LocalHarnessWorkflowError> {
        let workflow_id = workflow_id.into();
        if workflow_id.is_empty() {
            return Err(LocalHarnessWorkflowError::EmptyWorkflowId);
        }
        let run_id = run_id.into();
        if run_id.is_empty() {
            return Err(LocalHarnessWorkflowError::EmptyRunId);
        }
        let projection_id = projection_id.into();
        if projection_id.is_empty() {
            return Err(LocalHarnessWorkflowError::EmptyProjectionId);
        }
        let operator_id = operator_id.into();
        if operator_id.is_empty() {
            return Err(LocalHarnessWorkflowError::EmptyOperatorId);
        }
        let prompt_summary = prompt_summary.into();
        if prompt_summary.is_empty() {
            return Err(LocalHarnessWorkflowError::EmptyPromptSummary);
        }
        let context_packet_id = context_packet_id.into();
        if context_packet_id.is_empty() {
            return Err(LocalHarnessWorkflowError::EmptyContextPacketId);
        }
        let memory_snapshot_id = memory_snapshot_id.into();
        if memory_snapshot_id.is_empty() {
            return Err(LocalHarnessWorkflowError::EmptyMemorySnapshotId);
        }
        Ok(Self {
            workflow_id,
            run_id,
            projection_id,
            operator_id,
            prompt_summary,
            context_packet_id,
            memory_snapshot_id,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalHarnessWorkflowResult {
    pub workflow_id: String,
    pub status: LocalHarnessWorkflowStatus,
    pub reason: LocalHarnessWorkflowReason,
    pub provider_output_trusted: bool,
    pub provider_output_authoritative: bool,
    pub controlled_run_status: crate::execution::ControlledRunStatus,
    pub clean_output_available: bool,
    pub read_projection: Option<ApplicationReadProjection>,
    pub summary: String,
}

pub fn run_local_harness_workflow(
    request: LocalHarnessWorkflowRequest,
) -> LocalHarnessWorkflowResult {
    let summary = "workflow completed in memory; provider output remains untrusted; no real provider/model was called; no files were read or written; no persistence occurred; no UI or API transport was used; release-candidate readiness is not claimed; production readiness is not claimed".to_string();
    let runtime_config = match LocalRuntimeConfig::new(
        format!("{}-config", request.workflow_id),
        LocalRuntimeMode::DryRun,
        LocalProviderMode::Stub,
        RuntimeSafetyLevel::Strict,
        LocalWorkspaceMetadata::new(
            format!("{}-workspace", request.workflow_id),
            format!("opaque://workspace/{}", request.workflow_id),
            request.operator_id.clone(),
        )
        .expect("local harness workspace metadata should be valid"),
        RuntimeSafetyDefaults::strict(),
    ) {
        Ok(config) => config,
        Err(_) => {
            return LocalHarnessWorkflowResult {
                workflow_id: request.workflow_id,
                status: LocalHarnessWorkflowStatus::Blocked,
                reason: LocalHarnessWorkflowReason::RuntimeConfigInvalid,
                provider_output_trusted: false,
                provider_output_authoritative: false,
                controlled_run_status: crate::execution::ControlledRunStatus::Blocked,
                clean_output_available: false,
                read_projection: None,
                summary,
            };
        }
    };
    let integration_request = IntegrationRequest::new(
        format!("{}-integration-request", request.workflow_id),
        IntegrationSourceKind::LocalLlm,
        request.prompt_summary.clone(),
        format!("workflow {}", request.workflow_id),
    )
    .expect("local harness integration request should be valid");
    let provider_request = integration_request_to_provider_request(&integration_request)
        .expect("provider request mapping should be valid");
    let provider_invocation = crate::execution::ProviderAdapterInvocation::new(
        "phase-54-deterministic-stub",
        provider_request,
    )
    .expect("stub invocation should be valid");
    let provider = crate::execution::DeterministicStubProvider::new(
        "phase-54-deterministic-stub",
        "phase-54-stub-output:",
    )
    .expect("stub provider should be valid");
    let provider_result =
        match crate::execution::ProviderAdapter::invoke(&provider, &provider_invocation) {
            Ok(result) => result,
            Err(_) => {
                return LocalHarnessWorkflowResult {
                    workflow_id: request.workflow_id,
                    status: LocalHarnessWorkflowStatus::Blocked,
                    reason: LocalHarnessWorkflowReason::ProviderInvocationFailed,
                    provider_output_trusted: false,
                    provider_output_authoritative: false,
                    controlled_run_status: crate::execution::ControlledRunStatus::Blocked,
                    clean_output_available: false,
                    read_projection: None,
                    summary,
                };
            }
        };
    let integration_output = IntegrationOutput::new_untrusted(
        format!("{}-integration-output", request.workflow_id),
        integration_request.id.clone(),
        integration_request.source_kind,
        provider_result.output.content.clone(),
        IntegrationOutputStatus::Received,
    )
    .expect("integration output should be valid");
    let ledger = crate::ledger::Ledger::empty()
        .append(
            crate::ledger::LedgerEvent::new(
                "phase-54-event-1",
                1,
                crate::ledger::LedgerEventType::StateTransition,
                crate::ledger::LedgerActor::new(
                    crate::ledger::LedgerActorType::System,
                    "phase-54-system",
                )
                .expect("actor must be valid"),
                vec!["phase-54-evidence-1".to_string()],
                crate::ledger::LedgerPayload::with_lifecycle_transition(
                    "created_to_evaluating",
                    crate::state::LifecycleState::Evaluating,
                )
                .expect("payload should be valid"),
            )
            .expect("event should be valid"),
        )
        .expect("append should succeed")
        .append(
            crate::ledger::LedgerEvent::new(
                "phase-54-event-2",
                2,
                crate::ledger::LedgerEventType::StateTransition,
                crate::ledger::LedgerActor::new(
                    crate::ledger::LedgerActorType::System,
                    "phase-54-system",
                )
                .expect("actor must be valid"),
                vec!["phase-54-evidence-2".to_string()],
                crate::ledger::LedgerPayload::with_lifecycle_transition(
                    "evaluating_to_passed",
                    crate::state::LifecycleState::Passed,
                )
                .expect("payload should be valid"),
            )
            .expect("event should be valid"),
        )
        .expect("append should succeed");
    let controlled = crate::execution::run_controlled_model_flow(
        crate::execution::ControlledRunRequest::new(
            request.run_id.clone(),
            request.context_packet_id.clone(),
            provider_result.output.clone(),
            crate::state::LifecycleState::Passed,
            crate::policy::PolicyResult::allowed("phase_54_policy_allowed"),
            crate::validation::ValidationResult::pass("phase_54_validation_pass"),
            crate::replay::classify_replay_readiness(ledger.events())
                .expect("ledger should be replayable"),
            ledger,
            crate::ledger::LedgerActor::new(
                crate::ledger::LedgerActorType::System,
                "local_harness",
            )
            .expect("actor must be valid"),
            vec![format!("{}-evidence", request.workflow_id)],
        )
        .expect("controlled request should be valid"),
    );
    if controlled.status == crate::execution::ControlledRunStatus::Blocked {
        return LocalHarnessWorkflowResult {
            workflow_id: request.workflow_id,
            status: LocalHarnessWorkflowStatus::Blocked,
            reason: LocalHarnessWorkflowReason::ControlledFlowBlocked,
            provider_output_trusted: false,
            provider_output_authoritative: false,
            controlled_run_status: controlled.status,
            clean_output_available: false,
            read_projection: None,
            summary,
        };
    }
    if controlled.status == crate::execution::ControlledRunStatus::Rejected {
        return LocalHarnessWorkflowResult {
            workflow_id: request.workflow_id,
            status: LocalHarnessWorkflowStatus::Rejected,
            reason: LocalHarnessWorkflowReason::ControlledFlowRejected,
            provider_output_trusted: false,
            provider_output_authoritative: false,
            controlled_run_status: controlled.status,
            clean_output_available: false,
            read_projection: None,
            summary,
        };
    }
    let mut harness_state = crate::state::HarnessState::genesis();
    harness_state = harness_state
        .transition_to(crate::state::LifecycleState::Evaluating)
        .expect("transition");
    harness_state = harness_state
        .transition_to(crate::state::LifecycleState::Passed)
        .expect("transition");
    let replay_report =
        crate::replay::classify_replay_readiness(controlled.ledger.events()).expect("replayable");
    let app_state = match LocalApplicationState::new(
        format!("{}-state", request.workflow_id),
        request.projection_id.clone(),
        request.run_id.clone(),
        runtime_config,
        harness_state,
        controlled.clone(),
        provider_result.output,
        integration_output,
        controlled.ledger.clone(),
        replay_report,
        vec![crate::audit::AuditProjection::new(
            crate::audit::AuditProjectionType::OutputSummary,
            vec![crate::audit::AuditSourceRef::new(
                crate::audit::AuditSourceType::LedgerEvent,
                "phase-54-event-2",
            )
            .expect("source ref")],
            "phase_54_audit_projection",
            vec!["deterministic local harness run".to_string()],
        )
        .expect("audit projection should be valid")],
        ApplicationContextMetadata::new(request.context_packet_id.clone(), 2, 2, 128, 2048)
            .expect("context metadata should be valid"),
        ApplicationMemoryMetadata::new(request.memory_snapshot_id.clone(), 1, 0, 0)
            .expect("memory metadata should be valid"),
    ) {
        Ok(state) => state,
        Err(_) => {
            return LocalHarnessWorkflowResult {
                workflow_id: request.workflow_id,
                status: LocalHarnessWorkflowStatus::Blocked,
                reason: LocalHarnessWorkflowReason::ApplicationStateInvalid,
                provider_output_trusted: false,
                provider_output_authoritative: false,
                controlled_run_status: controlled.status,
                clean_output_available: false,
                read_projection: None,
                summary,
            }
        }
    };
    let read_projection = match app_state.derive_read_projection() {
        Ok(p) => p,
        Err(_) => {
            return LocalHarnessWorkflowResult {
                workflow_id: request.workflow_id,
                status: LocalHarnessWorkflowStatus::Blocked,
                reason: LocalHarnessWorkflowReason::ReadProjectionFailed,
                provider_output_trusted: false,
                provider_output_authoritative: false,
                controlled_run_status: controlled.status,
                clean_output_available: false,
                read_projection: None,
                summary,
            }
        }
    };
    LocalHarnessWorkflowResult {
        workflow_id: request.workflow_id,
        status: LocalHarnessWorkflowStatus::Completed,
        reason: LocalHarnessWorkflowReason::CompletedInMemory,
        provider_output_trusted: false,
        provider_output_authoritative: false,
        controlled_run_status: controlled.status,
        clean_output_available: controlled.clean_output_summary.is_some(),
        read_projection: Some(read_projection),
        summary,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadProjectionError {
    EmptyProjectionId,
    EmptyRuntimeConfigId,
    EmptyRunId,
    EmptyContextPacketId,
    EmptyMemorySnapshotId,
    UnsafeRuntimeConfig,
}
impl ReadProjectionError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyProjectionId => "empty_projection_id",
            Self::EmptyRuntimeConfigId => "empty_runtime_config_id",
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyContextPacketId => "empty_context_packet_id",
            Self::EmptyMemorySnapshotId => "empty_memory_snapshot_id",
            Self::UnsafeRuntimeConfig => "unsafe_runtime_config",
        }
    }
}

impl ApplicationReadProjection {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        projection_id: impl Into<String>,
        run_id: impl Into<String>,
        runtime_config: &LocalRuntimeConfig,
        harness_state: &crate::state::HarnessState,
        controlled_run: &crate::execution::ControlledRunResult,
        provider_output: &crate::execution::ProviderOutput,
        integration_output: &IntegrationOutput,
        ledger: &crate::ledger::Ledger,
        replay_report: &crate::replay::ReplayReport,
        audit_projections: &[crate::audit::AuditProjection],
        context_packet_id: impl Into<String>,
        context_slice_count: usize,
        context_source_count: usize,
        context_budget_used: usize,
        context_budget_max: usize,
        memory_snapshot_id: impl Into<String>,
        active_memory_entries: usize,
        disabled_memory_entries: usize,
        rejected_memory_entries: usize,
    ) -> Result<Self, ReadProjectionError> {
        let projection_id = projection_id.into();
        if projection_id.is_empty() {
            return Err(ReadProjectionError::EmptyProjectionId);
        }
        if runtime_config.config_id.is_empty() {
            return Err(ReadProjectionError::EmptyRuntimeConfigId);
        }
        let run_id = run_id.into();
        if run_id.is_empty() {
            return Err(ReadProjectionError::EmptyRunId);
        }
        let context_packet_id = context_packet_id.into();
        if context_packet_id.is_empty() {
            return Err(ReadProjectionError::EmptyContextPacketId);
        }
        let memory_snapshot_id = memory_snapshot_id.into();
        if memory_snapshot_id.is_empty() {
            return Err(ReadProjectionError::EmptyMemorySnapshotId);
        }
        if local_runtime_config_allows_authority_bypass(runtime_config) {
            return Err(ReadProjectionError::UnsafeRuntimeConfig);
        }

        let safety = RuntimeSafetyProjection {
            safety_level: runtime_config.safety_level,
            require_policy_pass: runtime_config.safety_defaults.require_policy_pass,
            require_validation_pass: runtime_config.safety_defaults.require_validation_pass,
            require_replay_verification: runtime_config.safety_defaults.require_replay_verification,
            allow_provider_network: runtime_config.safety_defaults.allow_provider_network,
            allow_file_io: runtime_config.safety_defaults.allow_file_io,
            allow_ui_mutation: runtime_config.safety_defaults.allow_ui_mutation,
            authority: ReadProjectionAuthority::Rust,
            summary: format!(
                "runtime safety level {:?} with closed authority bypass defaults",
                runtime_config.safety_level
            ),
        };
        let lifecycle = LifecycleReadProjection {
            lifecycle: harness_state.lifecycle,
            revision: harness_state.revision,
            status: ReadProjectionStatus::Ready,
            authority: ReadProjectionAuthority::Rust,
            summary: format!(
                "lifecycle {:?} at revision {}",
                harness_state.lifecycle, harness_state.revision
            ),
        };
        let run = RunReadProjection {
            run_id,
            status: controlled_run.status,
            reason: controlled_run.reason,
            clean_output_available: controlled_run.clean_output_summary.is_some(),
            authority: ReadProjectionAuthority::Rust,
            summary: format!(
                "controlled run {:?} ({:?})",
                controlled_run.status, controlled_run.reason
            ),
        };
        let provider = ProviderReadProjection {
            provider_kind: provider_output.provider_kind,
            output_status: provider_output.status,
            output_trust: provider_output.trust,
            authoritative: crate::execution::provider_output_is_authoritative(provider_output),
            authority: ReadProjectionAuthority::Provider,
            summary: format!(
                "provider output {:?} remains {:?}",
                provider_output.status, provider_output.trust
            ),
        };
        let integration = IntegrationReadProjection {
            source_kind: integration_output.source_kind,
            output_status: integration_output.status,
            output_trust: integration_output.trust,
            authoritative: integration_output_is_authoritative(integration_output),
            authority: ReadProjectionAuthority::Integration,
            summary: format!(
                "integration output {:?} remains {:?}",
                integration_output.status, integration_output.trust
            ),
        };
        let ledger_proj = LedgerReadProjection {
            event_count: ledger.events().len(),
            last_revision: ledger.last_revision(),
            status: ReadProjectionStatus::Ready,
            authority: ReadProjectionAuthority::Rust,
            summary: format!("ledger contains {} events", ledger.events().len()),
        };
        let replay = ReplayReadProjection {
            readiness: replay_report.readiness,
            integrity: replay_report.integrity,
            events_replayed: replay_report.events_replayed as usize,
            status: ReadProjectionStatus::Ready,
            authority: ReadProjectionAuthority::Rust,
            summary: format!(
                "replay readiness {:?} integrity {:?}",
                replay_report.readiness, replay_report.integrity
            ),
        };
        let latest_summary = audit_projections
            .last()
            .map(|p| p.summary.clone())
            .unwrap_or_else(|| "no_audit_projection".to_string());
        let audit = AuditReadProjection {
            projection_count: audit_projections.len(),
            latest_summary: latest_summary.clone(),
            authority: ReadProjectionAuthority::Rust,
            summary: format!("audit projections count {}", audit_projections.len()),
        };
        let context = ContextReadProjection {
            packet_id: context_packet_id,
            slice_count: context_slice_count,
            source_count: context_source_count,
            budget_used: context_budget_used,
            budget_max: context_budget_max,
            authority: ReadProjectionAuthority::Rust,
            summary: "context metadata from supplied typed inputs".to_string(),
        };
        let memory = MemoryReadProjection {
            snapshot_id: memory_snapshot_id,
            active_entries: active_memory_entries,
            disabled_entries: disabled_memory_entries,
            rejected_entries: rejected_memory_entries,
            authority: ReadProjectionAuthority::Rust,
            summary: "memory metadata from supplied typed inputs".to_string(),
        };
        let output = OutputReadProjection {
            clean_output_available: controlled_run.clean_output_summary.is_some(),
            raw_output_trusted: false,
            clean_output_summary: controlled_run.clean_output_summary.clone(),
            authority: ReadProjectionAuthority::Rust,
            summary: "raw provider output remains untrusted".to_string(),
        };
        Ok(Self {
            projection_id,
            runtime_config_id: runtime_config.config_id.clone(),
            safety,
            lifecycle,
            run,
            provider,
            integration,
            ledger: ledger_proj,
            replay,
            audit,
            context,
            memory,
            output,
        })
    }
}

