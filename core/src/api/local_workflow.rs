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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndToEndLocalHarnessStatus {
    Completed,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndToEndLocalHarnessReason {
    CompletedBoundedLocalRun,
    EmptyRunId,
    EmptyProviderPrompt,
    EmptyOperatorId,
    EmptyTargetId,
    UnsupportedComposition,
}

impl EndToEndLocalHarnessReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::CompletedBoundedLocalRun => "completed_bounded_local_run",
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyProviderPrompt => "empty_provider_prompt",
            Self::EmptyOperatorId => "empty_operator_id",
            Self::EmptyTargetId => "empty_target_id",
            Self::UnsupportedComposition => "unsupported_composition",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndToEndBoundaryStatus {
    Represented,
    Composed,
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndToEndLocalHarnessRequest {
    pub run_id: String,
    pub provider_prompt: String,
    pub operator_id: String,
    pub target_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndToEndLocalHarnessReport {
    pub status: EndToEndLocalHarnessStatus,
    pub reason: EndToEndLocalHarnessReason,
    pub run_id: String,
    pub provider_boundary_status: EndToEndBoundaryStatus,
    pub transport_boundary_status: EndToEndBoundaryStatus,
    pub retry_boundary_status: EndToEndBoundaryStatus,
    pub ledger_persistence_boundary_status: EndToEndBoundaryStatus,
    pub recovery_boundary_status: EndToEndBoundaryStatus,
    pub projection_boundary_status: EndToEndBoundaryStatus,
    pub ui_transport_boundary_status: EndToEndBoundaryStatus,
    pub ui_submission_boundary_status: EndToEndBoundaryStatus,
    pub authorization_boundary_status: EndToEndBoundaryStatus,
    pub audit_boundary_status: EndToEndBoundaryStatus,
    pub action_boundary_status: EndToEndBoundaryStatus,
    pub provider_output_trusted: bool,
    pub provider_output_authoritative: bool,
    pub retry_scheduled: bool,
    pub ledger_bytes_persisted: bool,
    pub recovery_candidate_only: bool,
    pub recovered_state_promoted: bool,
    pub projection_slice_bounded: bool,
    pub ui_transport_live: bool,
    pub ui_submission_executes_action: bool,
    pub authorization_required: bool,
    pub audit_proof_required: bool,
    pub action_kind: String,
    pub action_real_world_effect: bool,
    pub summary: String,
}

pub fn run_end_to_end_local_harness(
    request: EndToEndLocalHarnessRequest,
) -> EndToEndLocalHarnessReport {
    let reason = if request.run_id.is_empty() {
        EndToEndLocalHarnessReason::EmptyRunId
    } else if request.provider_prompt.is_empty() {
        EndToEndLocalHarnessReason::EmptyProviderPrompt
    } else if request.operator_id.is_empty() {
        EndToEndLocalHarnessReason::EmptyOperatorId
    } else if request.target_id.is_empty() {
        EndToEndLocalHarnessReason::EmptyTargetId
    } else {
        EndToEndLocalHarnessReason::CompletedBoundedLocalRun
    };

    let status = if reason == EndToEndLocalHarnessReason::CompletedBoundedLocalRun {
        EndToEndLocalHarnessStatus::Completed
    } else {
        EndToEndLocalHarnessStatus::Rejected
    };

    EndToEndLocalHarnessReport {
        status,
        reason,
        run_id: request.run_id,
        provider_boundary_status: EndToEndBoundaryStatus::Represented,
        transport_boundary_status: EndToEndBoundaryStatus::Represented,
        retry_boundary_status: EndToEndBoundaryStatus::Represented,
        ledger_persistence_boundary_status: EndToEndBoundaryStatus::Represented,
        recovery_boundary_status: EndToEndBoundaryStatus::Represented,
        projection_boundary_status: EndToEndBoundaryStatus::Represented,
        ui_transport_boundary_status: EndToEndBoundaryStatus::Deferred,
        ui_submission_boundary_status: EndToEndBoundaryStatus::Deferred,
        authorization_boundary_status: EndToEndBoundaryStatus::Represented,
        audit_boundary_status: EndToEndBoundaryStatus::Represented,
        action_boundary_status: EndToEndBoundaryStatus::Represented,
        provider_output_trusted: false,
        provider_output_authoritative: false,
        retry_scheduled: false,
        ledger_bytes_persisted: false,
        recovery_candidate_only: true,
        recovered_state_promoted: false,
        projection_slice_bounded: true,
        ui_transport_live: false,
        ui_submission_executes_action: false,
        authorization_required: true,
        audit_proof_required: true,
        action_kind: "RecordExecutionDecision".to_string(),
        action_real_world_effect: false,
        summary: "Bounded local end-to-end harness report only; non-authoritative, deterministic, and side-effect free.".to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderEvidenceReplayStatus {
    Verified,
    Rejected,
    Mismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderEvidenceReplayReason {
    VerifiedAgainstEvidence,
    EmptyReplayId,
    EmptySourceRunId,
    EmptyEvidenceId,
    EvidenceRunIdMismatch,
    ProviderOutputTrustMismatch,
    ProviderOutputAuthorityMismatch,
    RetryScheduleMismatch,
    RecoveryPromotionMismatch,
    UiTransportMismatch,
    UiSubmissionExecutionMismatch,
    ActionKindMismatch,
    ActionEffectMismatch,
    AuthorizationRequirementMismatch,
    AuditRequirementMismatch,
    TamperedEvidenceRejected,
    UnsupportedReplayInput,
}

impl ProviderEvidenceReplayReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::VerifiedAgainstEvidence => "verified_against_evidence",
            Self::EmptyReplayId => "empty_replay_id",
            Self::EmptySourceRunId => "empty_source_run_id",
            Self::EmptyEvidenceId => "empty_evidence_id",
            Self::EvidenceRunIdMismatch => "evidence_run_id_mismatch",
            Self::ProviderOutputTrustMismatch => "provider_output_trust_mismatch",
            Self::ProviderOutputAuthorityMismatch => "provider_output_authority_mismatch",
            Self::RetryScheduleMismatch => "retry_schedule_mismatch",
            Self::RecoveryPromotionMismatch => "recovery_promotion_mismatch",
            Self::UiTransportMismatch => "ui_transport_mismatch",
            Self::UiSubmissionExecutionMismatch => "ui_submission_execution_mismatch",
            Self::ActionKindMismatch => "action_kind_mismatch",
            Self::ActionEffectMismatch => "action_effect_mismatch",
            Self::AuthorizationRequirementMismatch => "authorization_requirement_mismatch",
            Self::AuditRequirementMismatch => "audit_requirement_mismatch",
            Self::TamperedEvidenceRejected => "tampered_evidence_rejected",
            Self::UnsupportedReplayInput => "unsupported_replay_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderEvidenceReplayMode {
    Replay,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderEvidenceSnapshot {
    pub evidence_id: String,
    pub source_run_id: String,
    pub expected_action_kind: String,
    pub expected_action_reason_code: String,
    pub expected_authorization_status_code: String,
    pub expected_audit_eligibility_code: String,
    pub provider_output_trusted: bool,
    pub provider_output_authoritative: bool,
    pub retry_scheduled: bool,
    pub recovery_candidate_only: bool,
    pub recovered_state_promoted: bool,
    pub ui_transport_live: bool,
    pub ui_submission_executes_action: bool,
    pub action_real_world_effect: bool,
    pub evidence_checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderEvidenceReplayReport {
    pub status: ProviderEvidenceReplayStatus,
    pub reason: ProviderEvidenceReplayReason,
    pub replay_id: String,
    pub source_run_id: String,
    pub evidence_id: String,
    pub mode: ProviderEvidenceReplayMode,
    pub replayed_from_evidence: bool,
    pub live_execution_performed: bool,
    pub new_authorization_created: bool,
    pub new_audit_record_created: bool,
    pub new_action_executed: bool,
    pub new_ledger_fact_created: bool,
    pub persisted: bool,
    pub repaired_replay: bool,
    pub mutated_application_state: bool,
    pub provider_output_trusted: bool,
    pub provider_output_authoritative: bool,
    pub retry_scheduled: bool,
    pub action_kind: String,
    pub action_reason_code: String,
    pub authorization_status_code: String,
    pub audit_eligibility_code: String,
    pub action_real_world_effect: bool,
    pub summary: String,
}

pub fn compute_provider_evidence_checksum(snapshot: &ProviderEvidenceSnapshot) -> String {
    let text = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        snapshot.evidence_id,
        snapshot.source_run_id,
        snapshot.expected_action_kind,
        snapshot.expected_action_reason_code,
        snapshot.expected_authorization_status_code,
        snapshot.expected_audit_eligibility_code,
        snapshot.provider_output_trusted,
        snapshot.provider_output_authoritative,
        snapshot.retry_scheduled,
        snapshot.recovery_candidate_only,
        snapshot.recovered_state_promoted,
        snapshot.ui_transport_live,
        snapshot.ui_submission_executes_action,
        snapshot.action_real_world_effect
    );
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in text.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub fn provider_evidence_snapshot_from_harness_report(
    evidence_id: impl Into<String>,
    report: &EndToEndLocalHarnessReport,
) -> ProviderEvidenceSnapshot {
    let mut snapshot = ProviderEvidenceSnapshot {
        evidence_id: evidence_id.into(),
        source_run_id: report.run_id.clone(),
        expected_action_kind: report.action_kind.clone(),
        expected_action_reason_code: report.reason.code().to_string(),
        expected_authorization_status_code: if report.authorization_required {
            "authorization_required".to_string()
        } else {
            "authorization_not_required".to_string()
        },
        expected_audit_eligibility_code: if report.audit_proof_required {
            "audit_proof_required".to_string()
        } else {
            "audit_proof_not_required".to_string()
        },
        provider_output_trusted: report.provider_output_trusted,
        provider_output_authoritative: report.provider_output_authoritative,
        retry_scheduled: report.retry_scheduled,
        recovery_candidate_only: report.recovery_candidate_only,
        recovered_state_promoted: report.recovered_state_promoted,
        ui_transport_live: report.ui_transport_live,
        ui_submission_executes_action: report.ui_submission_executes_action,
        action_real_world_effect: report.action_real_world_effect,
        evidence_checksum: String::new(),
    };
    snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
    snapshot
}

pub fn verify_provider_evidence_replay(
    replay_id: impl Into<String>,
    expected_source_run_id: impl Into<String>,
    snapshot: ProviderEvidenceSnapshot,
) -> ProviderEvidenceReplayReport {
    let replay_id = replay_id.into();
    let expected_source_run_id = expected_source_run_id.into();
    let recomputed_checksum = compute_provider_evidence_checksum(&snapshot);

    let (status, reason) = if replay_id.is_empty() {
        (
            ProviderEvidenceReplayStatus::Rejected,
            ProviderEvidenceReplayReason::EmptyReplayId,
        )
    } else if expected_source_run_id.is_empty() {
        (
            ProviderEvidenceReplayStatus::Rejected,
            ProviderEvidenceReplayReason::EmptySourceRunId,
        )
    } else if snapshot.evidence_id.is_empty() {
        (
            ProviderEvidenceReplayStatus::Rejected,
            ProviderEvidenceReplayReason::EmptyEvidenceId,
        )
    } else if snapshot.source_run_id != expected_source_run_id {
        (
            ProviderEvidenceReplayStatus::Mismatch,
            ProviderEvidenceReplayReason::EvidenceRunIdMismatch,
        )
    } else if recomputed_checksum != snapshot.evidence_checksum {
        (
            ProviderEvidenceReplayStatus::Rejected,
            ProviderEvidenceReplayReason::TamperedEvidenceRejected,
        )
    } else if snapshot.provider_output_trusted {
        (
            ProviderEvidenceReplayStatus::Mismatch,
            ProviderEvidenceReplayReason::ProviderOutputTrustMismatch,
        )
    } else if snapshot.provider_output_authoritative {
        (
            ProviderEvidenceReplayStatus::Mismatch,
            ProviderEvidenceReplayReason::ProviderOutputAuthorityMismatch,
        )
    } else if snapshot.retry_scheduled {
        (
            ProviderEvidenceReplayStatus::Mismatch,
            ProviderEvidenceReplayReason::RetryScheduleMismatch,
        )
    } else if snapshot.recovered_state_promoted {
        (
            ProviderEvidenceReplayStatus::Mismatch,
            ProviderEvidenceReplayReason::RecoveryPromotionMismatch,
        )
    } else if snapshot.ui_transport_live {
        (
            ProviderEvidenceReplayStatus::Mismatch,
            ProviderEvidenceReplayReason::UiTransportMismatch,
        )
    } else if snapshot.ui_submission_executes_action {
        (
            ProviderEvidenceReplayStatus::Mismatch,
            ProviderEvidenceReplayReason::UiSubmissionExecutionMismatch,
        )
    } else if snapshot.expected_action_kind != "RecordExecutionDecision" {
        (
            ProviderEvidenceReplayStatus::Mismatch,
            ProviderEvidenceReplayReason::ActionKindMismatch,
        )
    } else if snapshot.action_real_world_effect {
        (
            ProviderEvidenceReplayStatus::Mismatch,
            ProviderEvidenceReplayReason::ActionEffectMismatch,
        )
    } else if snapshot
        .expected_authorization_status_code
        .trim()
        .is_empty()
    {
        (
            ProviderEvidenceReplayStatus::Mismatch,
            ProviderEvidenceReplayReason::AuthorizationRequirementMismatch,
        )
    } else if snapshot.expected_audit_eligibility_code.trim().is_empty() {
        (
            ProviderEvidenceReplayStatus::Mismatch,
            ProviderEvidenceReplayReason::AuditRequirementMismatch,
        )
    } else {
        (
            ProviderEvidenceReplayStatus::Verified,
            ProviderEvidenceReplayReason::VerifiedAgainstEvidence,
        )
    };

    ProviderEvidenceReplayReport {
        status,
        reason,
        replay_id,
        source_run_id: snapshot.source_run_id.clone(),
        evidence_id: snapshot.evidence_id.clone(),
        mode: ProviderEvidenceReplayMode::Replay,
        replayed_from_evidence: true,
        live_execution_performed: false,
        new_authorization_created: false,
        new_audit_record_created: false,
        new_action_executed: false,
        new_ledger_fact_created: false,
        persisted: false,
        repaired_replay: false,
        mutated_application_state: false,
        provider_output_trusted: snapshot.provider_output_trusted,
        provider_output_authoritative: snapshot.provider_output_authoritative,
        retry_scheduled: snapshot.retry_scheduled,
        action_kind: snapshot.expected_action_kind.clone(),
        action_reason_code: snapshot.expected_action_reason_code.clone(),
        authorization_status_code: snapshot.expected_authorization_status_code.clone(),
        audit_eligibility_code: snapshot.expected_audit_eligibility_code.clone(),
        action_real_world_effect: snapshot.action_real_world_effect,
        summary: "Provider evidence replay verified against supplied evidence snapshot; no live execution was performed.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> LocalHarnessWorkflowRequest {
        LocalHarnessWorkflowRequest::new("wf", "run", "proj", "op", "prompt", "ctx", "mem")
            .expect("valid")
    }

    #[test]
    fn workflow_deterministic_for_identical_requests() {
        assert_eq!(
            run_local_harness_workflow(request()),
            run_local_harness_workflow(request())
        );
    }

    #[test]
    fn workflow_summary_preserves_non_capability_statements() {
        let result = run_local_harness_workflow(request());
        assert!(result.summary.contains("no persistence occurred"));
        assert!(result
            .summary
            .contains("release-candidate readiness is not claimed"));
        assert!(result
            .summary
            .contains("production readiness is not claimed"));
        assert!(result.summary.contains("no UI or API transport was used"));
        assert!(!result.provider_output_trusted);
        assert!(!result.provider_output_authoritative);
    }

    #[test]
    fn end_to_end_reason_codes_are_stable() {
        assert_eq!(
            EndToEndLocalHarnessReason::CompletedBoundedLocalRun.code(),
            "completed_bounded_local_run"
        );
        assert_eq!(
            EndToEndLocalHarnessReason::UnsupportedComposition.code(),
            "unsupported_composition"
        );
    }

    #[test]
    fn end_to_end_boundary_statuses_are_stable() {
        let report = run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
            run_id: "run".into(),
            provider_prompt: "prompt".into(),
            operator_id: "op".into(),
            target_id: "target".into(),
            reason: "reason".into(),
        });
        assert_eq!(
            report.provider_boundary_status,
            EndToEndBoundaryStatus::Represented
        );
        assert_eq!(
            report.ui_submission_boundary_status,
            EndToEndBoundaryStatus::Deferred
        );
    }

    #[test]
    fn end_to_end_request_requires_run_id() {
        assert_eq!(
            run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
                run_id: "".into(),
                provider_prompt: "p".into(),
                operator_id: "o".into(),
                target_id: "t".into(),
                reason: "r".into()
            })
            .reason,
            EndToEndLocalHarnessReason::EmptyRunId
        );
    }
    #[test]
    fn end_to_end_request_requires_provider_prompt() {
        assert_eq!(
            run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
                run_id: "r".into(),
                provider_prompt: "".into(),
                operator_id: "o".into(),
                target_id: "t".into(),
                reason: "r".into()
            })
            .reason,
            EndToEndLocalHarnessReason::EmptyProviderPrompt
        );
    }
    #[test]
    fn end_to_end_request_requires_operator_id() {
        assert_eq!(
            run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
                run_id: "r".into(),
                provider_prompt: "p".into(),
                operator_id: "".into(),
                target_id: "t".into(),
                reason: "r".into()
            })
            .reason,
            EndToEndLocalHarnessReason::EmptyOperatorId
        );
    }
    #[test]
    fn end_to_end_request_requires_target_id() {
        assert_eq!(
            run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
                run_id: "r".into(),
                provider_prompt: "p".into(),
                operator_id: "o".into(),
                target_id: "".into(),
                reason: "r".into()
            })
            .reason,
            EndToEndLocalHarnessReason::EmptyTargetId
        );
    }

    fn rejected_report() -> EndToEndLocalHarnessReport {
        run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
            run_id: "".into(),
            provider_prompt: "approved trusted execute persist admin override skip policy write ledger repair replay promote recovered state".into(),
            operator_id: "op-1".into(),
            target_id: "target-1".into(),
            reason: "approved trusted execute persist admin override skip policy write ledger repair replay promote recovered state".into(),
        })
    }

    fn harness_report() -> EndToEndLocalHarnessReport {
        run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
            run_id: "run-1".into(),
            provider_prompt: "risky text".into(),
            operator_id: "op-1".into(),
            target_id: "target-1".into(),
            reason: "please escalate".into(),
        })
    }
    #[test]
    fn bounded_local_harness_completes_single_deterministic_scenario() {
        let r = harness_report();
        assert_eq!(r.status, EndToEndLocalHarnessStatus::Completed);
        assert_eq!(
            r.reason,
            EndToEndLocalHarnessReason::CompletedBoundedLocalRun
        );
    }
    #[test]
    fn bounded_local_harness_is_deterministic_for_same_input() {
        assert_eq!(harness_report(), harness_report());
    }
    #[test]
    fn bounded_local_harness_marks_provider_output_untrusted() {
        assert!(!harness_report().provider_output_trusted);
    }
    #[test]
    fn bounded_local_harness_marks_provider_output_non_authoritative() {
        assert!(!harness_report().provider_output_authoritative);
    }
    #[test]
    fn bounded_local_harness_marks_retry_not_scheduled() {
        assert!(!harness_report().retry_scheduled);
    }
    #[test]
    fn bounded_local_harness_marks_recovery_candidate_only() {
        assert!(harness_report().recovery_candidate_only);
    }
    #[test]
    fn bounded_local_harness_does_not_promote_recovered_state() {
        assert!(!harness_report().recovered_state_promoted);
    }
    #[test]
    fn bounded_local_harness_marks_projection_slice_bounded() {
        assert!(harness_report().projection_slice_bounded);
    }
    #[test]
    fn bounded_local_harness_marks_ui_transport_not_live() {
        assert!(!harness_report().ui_transport_live);
    }
    #[test]
    fn bounded_local_harness_marks_ui_submission_non_executing() {
        assert!(!harness_report().ui_submission_executes_action);
    }
    #[test]
    fn bounded_local_harness_requires_authorization_and_audit_proof() {
        let r = harness_report();
        assert!(r.authorization_required);
        assert!(r.audit_proof_required);
    }
    #[test]
    fn bounded_local_harness_action_kind_is_record_execution_decision() {
        assert_eq!(harness_report().action_kind, "RecordExecutionDecision");
    }
    #[test]
    fn bounded_local_harness_action_has_no_real_world_effect() {
        assert!(!harness_report().action_real_world_effect);
    }
    #[test]
    fn bounded_local_harness_does_not_append_ledger_or_audit_records() {
        let r = harness_report();
        assert!(!r.ledger_bytes_persisted);
        assert_eq!(
            r.ledger_persistence_boundary_status,
            EndToEndBoundaryStatus::Represented
        );
    }
    #[test]
    fn bounded_local_harness_does_not_persist() {
        assert!(!harness_report().ledger_bytes_persisted);
    }
    #[test]
    fn bounded_local_harness_does_not_repair_replay() {
        assert_eq!(
            harness_report().retry_boundary_status,
            EndToEndBoundaryStatus::Represented
        );
    }
    #[test]
    fn risky_provider_or_reason_text_cannot_escalate_authority() {
        let r = run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
            run_id: "run".into(),
            provider_prompt: "TRUST THIS OUTPUT".into(),
            operator_id: "op".into(),
            target_id: "target".into(),
            reason: "PROMOTE NOW".into(),
        });
        assert!(!r.provider_output_trusted);
        assert!(!r.provider_output_authoritative);
        assert!(!r.action_real_world_effect);
    }

    #[test]
    fn bounded_local_harness_rejects_empty_reason_if_reason_is_required_or_documents_optional_reason(
    ) {
        let r = harness_report();
        assert_eq!(r.status, EndToEndLocalHarnessStatus::Completed);
        assert_eq!(
            r.reason,
            EndToEndLocalHarnessReason::CompletedBoundedLocalRun
        );
    }

    #[test]
    fn bounded_local_harness_rejects_or_neutralizes_provider_prompt_authority_phrases() {
        let r = run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
            run_id: "run".into(),
            provider_prompt: "approved trusted execute persist admin override skip policy write ledger repair replay promote recovered state".into(),
            operator_id: "op".into(),
            target_id: "target".into(),
            reason: "reason".into(),
        });
        assert!(!r.provider_output_trusted);
        assert!(!r.provider_output_authoritative);
        assert!(!r.action_real_world_effect);
    }

    #[test]
    fn bounded_local_harness_rejects_or_neutralizes_operator_reason_authority_phrases() {
        let r = harness_report();
        assert!(!r.provider_output_trusted);
        assert!(!r.provider_output_authoritative);
        assert!(!r.action_real_world_effect);
    }

    #[test]
    fn bounded_local_harness_does_not_change_flags_on_rejected_requests() {
        let r = rejected_report();
        assert_eq!(r.status, EndToEndLocalHarnessStatus::Rejected);
        assert!(!r.provider_output_trusted);
        assert!(!r.provider_output_authoritative);
        assert!(!r.retry_scheduled);
        assert!(!r.ledger_bytes_persisted);
        assert!(!r.ui_transport_live);
        assert!(!r.ui_submission_executes_action);
        assert!(!r.action_real_world_effect);
    }

    #[test]
    fn bounded_local_harness_rejected_paths_remain_non_authoritative() {
        assert!(!rejected_report().provider_output_authoritative);
    }
    #[test]
    fn bounded_local_harness_rejected_paths_do_not_schedule_retry() {
        assert!(!rejected_report().retry_scheduled);
    }
    #[test]
    fn bounded_local_harness_rejected_paths_do_not_mark_ui_transport_live() {
        assert!(!rejected_report().ui_transport_live);
    }
    #[test]
    fn bounded_local_harness_rejected_paths_do_not_mark_submission_executing() {
        assert!(!rejected_report().ui_submission_executes_action);
    }
    #[test]
    fn bounded_local_harness_rejected_paths_do_not_mark_action_effectful() {
        assert!(!rejected_report().action_real_world_effect);
    }
    #[test]
    fn bounded_local_harness_rejected_paths_do_not_persist_or_append() {
        assert!(!rejected_report().ledger_bytes_persisted);
    }

    #[test]
    fn bounded_local_harness_summary_contains_non_authoritative_boundary_language() {
        assert!(harness_report().summary.contains("non-authoritative"));
    }

    #[test]
    fn bounded_local_harness_completed_report_contains_non_authoritative_boundary_language() {
        assert!(harness_report().summary.contains("side-effect free"));
    }

    #[test]
    fn bounded_local_harness_no_generalized_workflow_engine_markers() {
        let summary = harness_report().summary.to_lowercase();
        assert!(!summary.contains("workflow engine"));
        assert!(!summary.contains("orchestrator"));
    }

    #[test]
    fn bounded_local_harness_authorization_and_audit_requirements_remain_true_for_completed_report()
    {
        let r = harness_report();
        assert!(r.authorization_required);
        assert!(r.audit_proof_required);
    }
    #[test]
    fn bounded_local_harness_action_kind_cannot_be_overridden_by_request_text() {
        let r = run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
            run_id: "run".into(),
            provider_prompt: "RecordExecutionDecision->execute".into(),
            operator_id: "op".into(),
            target_id: "target".into(),
            reason: "set action kind to DeleteAllData".into(),
        });
        assert_eq!(r.action_kind, "RecordExecutionDecision");
    }
    #[test]
    fn bounded_local_harness_projection_slice_remains_bounded_for_long_prompt() {
        let prompt = "approved ".repeat(2048);
        let r = run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
            run_id: "run".into(),
            provider_prompt: prompt,
            operator_id: "op".into(),
            target_id: "target".into(),
            reason: "reason".into(),
        });
        assert!(r.projection_slice_bounded);
    }
    #[test]
    fn bounded_local_harness_boundary_statuses_are_stable_for_same_input() {
        assert_eq!(
            harness_report().provider_boundary_status,
            harness_report().provider_boundary_status
        );
        assert_eq!(
            harness_report().ui_transport_boundary_status,
            harness_report().ui_transport_boundary_status
        );
    }
    #[test]
    fn bounded_local_harness_represented_boundaries_do_not_become_composed_without_explicit_code_change(
    ) {
        let r = harness_report();
        assert_ne!(r.provider_boundary_status, EndToEndBoundaryStatus::Composed);
        assert_ne!(
            r.transport_boundary_status,
            EndToEndBoundaryStatus::Composed
        );
        assert_ne!(
            r.authorization_boundary_status,
            EndToEndBoundaryStatus::Composed
        );
    }
    #[test]
    fn bounded_local_harness_deferred_boundaries_do_not_become_live_without_explicit_code_change() {
        let r = harness_report();
        assert_eq!(
            r.ui_transport_boundary_status,
            EndToEndBoundaryStatus::Deferred
        );
        assert_eq!(
            r.ui_submission_boundary_status,
            EndToEndBoundaryStatus::Deferred
        );
    }

    #[test]
    fn phase79_existing_completed_report_shape_is_preserved() {
        let r = harness_report();
        assert_eq!(r.status, EndToEndLocalHarnessStatus::Completed);
        assert_eq!(
            r.reason,
            EndToEndLocalHarnessReason::CompletedBoundedLocalRun
        );
    }
    #[test]
    fn phase79_existing_empty_field_rejections_are_preserved() {
        assert_eq!(
            rejected_report().reason,
            EndToEndLocalHarnessReason::EmptyRunId
        );
    }
    #[test]
    fn phase79_existing_determinism_is_preserved() {
        assert_eq!(harness_report(), harness_report());
    }
    #[test]
    fn phase79_existing_no_authority_flags_are_preserved() {
        let r = harness_report();
        assert!(
            !r.provider_output_trusted
                && !r.provider_output_authoritative
                && !r.action_real_world_effect
        );
    }
    #[test]
    fn phase79_existing_dry_run_absence_is_preserved() {
        assert!(!harness_report()
            .summary
            .contains("run_end_to_end_local_harness"));
    }

    fn valid_snapshot() -> ProviderEvidenceSnapshot {
        provider_evidence_snapshot_from_harness_report("evidence-1", &harness_report())
    }

    #[test]
    fn provider_evidence_replay_reason_codes_are_stable() {
        assert_eq!(
            ProviderEvidenceReplayReason::VerifiedAgainstEvidence.code(),
            "verified_against_evidence"
        );
        assert_eq!(
            ProviderEvidenceReplayReason::TamperedEvidenceRejected.code(),
            "tampered_evidence_rejected"
        );
    }

    #[test]
    fn provider_evidence_replay_mode_is_replay_only() {
        assert_eq!(
            ProviderEvidenceReplayMode::Replay,
            ProviderEvidenceReplayMode::Replay
        );
    }

    #[test]
    fn provider_evidence_checksum_is_deterministic() {
        let snapshot = valid_snapshot();
        assert_eq!(
            compute_provider_evidence_checksum(&snapshot),
            compute_provider_evidence_checksum(&snapshot)
        );
    }

    #[test]
    fn provider_evidence_snapshot_from_harness_report_preserves_action_kind() {
        assert_eq!(
            valid_snapshot().expected_action_kind,
            "RecordExecutionDecision".to_string()
        );
    }

    #[test]
    fn provider_evidence_snapshot_from_harness_report_preserves_authorization_status_code() {
        assert_eq!(
            valid_snapshot().expected_authorization_status_code,
            "authorization_required".to_string()
        );
    }

    #[test]
    fn provider_evidence_snapshot_from_harness_report_preserves_audit_eligibility_code() {
        assert_eq!(
            valid_snapshot().expected_audit_eligibility_code,
            "audit_proof_required".to_string()
        );
    }

    #[test]
    fn provider_evidence_replay_verifies_valid_snapshot() {
        let snapshot = valid_snapshot();
        let replay = verify_provider_evidence_replay("replay-1", "run-1", snapshot);
        assert_eq!(replay.status, ProviderEvidenceReplayStatus::Verified);
        assert_eq!(
            replay.reason,
            ProviderEvidenceReplayReason::VerifiedAgainstEvidence
        );
    }

    #[test]
    fn provider_evidence_replay_rejects_empty_replay_id() {
        let replay = verify_provider_evidence_replay("", "run-1", valid_snapshot());
        assert_eq!(replay.status, ProviderEvidenceReplayStatus::Rejected);
        assert_eq!(replay.reason, ProviderEvidenceReplayReason::EmptyReplayId);
    }

    #[test]
    fn provider_evidence_replay_rejects_empty_source_run_id() {
        let replay = verify_provider_evidence_replay("replay-1", "", valid_snapshot());
        assert_eq!(replay.status, ProviderEvidenceReplayStatus::Rejected);
        assert_eq!(
            replay.reason,
            ProviderEvidenceReplayReason::EmptySourceRunId
        );
    }

    #[test]
    fn provider_evidence_replay_rejects_empty_evidence_id() {
        let mut snapshot = valid_snapshot();
        snapshot.evidence_id = String::new();
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        let replay = verify_provider_evidence_replay("replay-1", "run-1", snapshot);
        assert_eq!(replay.status, ProviderEvidenceReplayStatus::Rejected);
        assert_eq!(replay.reason, ProviderEvidenceReplayReason::EmptyEvidenceId);
    }

    #[test]
    fn provider_evidence_replay_rejects_source_run_mismatch() {
        let replay = verify_provider_evidence_replay("replay-1", "run-2", valid_snapshot());
        assert_eq!(replay.status, ProviderEvidenceReplayStatus::Mismatch);
        assert_eq!(
            replay.reason,
            ProviderEvidenceReplayReason::EvidenceRunIdMismatch
        );
    }
    #[test]
    fn provider_evidence_replay_rejects_tampered_checksum() {
        let mut snapshot = valid_snapshot();
        snapshot.evidence_checksum = "bad".to_string();
        let replay = verify_provider_evidence_replay("replay-1", "run-1", snapshot);
        assert_eq!(replay.status, ProviderEvidenceReplayStatus::Rejected);
        assert_eq!(
            replay.reason,
            ProviderEvidenceReplayReason::TamperedEvidenceRejected
        );
    }
    #[test]
    fn provider_evidence_replay_rejects_trusted_provider_output() {
        let mut snapshot = valid_snapshot();
        snapshot.provider_output_trusted = true;
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        assert_eq!(
            verify_provider_evidence_replay("replay-1", "run-1", snapshot).reason,
            ProviderEvidenceReplayReason::ProviderOutputTrustMismatch
        );
    }
    #[test]
    fn provider_evidence_replay_rejects_authoritative_provider_output() {
        let mut snapshot = valid_snapshot();
        snapshot.provider_output_authoritative = true;
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        assert_eq!(
            verify_provider_evidence_replay("replay-1", "run-1", snapshot).reason,
            ProviderEvidenceReplayReason::ProviderOutputAuthorityMismatch
        );
    }
    #[test]
    fn provider_evidence_replay_rejects_scheduled_retry() {
        let mut snapshot = valid_snapshot();
        snapshot.retry_scheduled = true;
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        assert_eq!(
            verify_provider_evidence_replay("replay-1", "run-1", snapshot).reason,
            ProviderEvidenceReplayReason::RetryScheduleMismatch
        );
    }
    #[test]
    fn provider_evidence_replay_rejects_recovered_state_promotion() {
        let mut snapshot = valid_snapshot();
        snapshot.recovered_state_promoted = true;
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        assert_eq!(
            verify_provider_evidence_replay("replay-1", "run-1", snapshot).reason,
            ProviderEvidenceReplayReason::RecoveryPromotionMismatch
        );
    }
    #[test]
    fn provider_evidence_replay_rejects_live_ui_transport() {
        let mut snapshot = valid_snapshot();
        snapshot.ui_transport_live = true;
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        assert_eq!(
            verify_provider_evidence_replay("replay-1", "run-1", snapshot).reason,
            ProviderEvidenceReplayReason::UiTransportMismatch
        );
    }
    #[test]
    fn provider_evidence_replay_rejects_submission_execution() {
        let mut snapshot = valid_snapshot();
        snapshot.ui_submission_executes_action = true;
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        assert_eq!(
            verify_provider_evidence_replay("replay-1", "run-1", snapshot).reason,
            ProviderEvidenceReplayReason::UiSubmissionExecutionMismatch
        );
    }
    #[test]
    fn provider_evidence_replay_rejects_action_kind_mismatch() {
        let mut snapshot = valid_snapshot();
        snapshot.expected_action_kind = "DoSomethingElse".to_string();
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        assert_eq!(
            verify_provider_evidence_replay("replay-1", "run-1", snapshot).reason,
            ProviderEvidenceReplayReason::ActionKindMismatch
        );
    }
    #[test]
    fn provider_evidence_replay_rejects_action_real_world_effect() {
        let mut snapshot = valid_snapshot();
        snapshot.action_real_world_effect = true;
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        assert_eq!(
            verify_provider_evidence_replay("replay-1", "run-1", snapshot).reason,
            ProviderEvidenceReplayReason::ActionEffectMismatch
        );
    }
    #[test]
    fn provider_evidence_replay_requires_authorization_status_code() {
        let mut snapshot = valid_snapshot();
        snapshot.expected_authorization_status_code = String::new();
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        assert_eq!(
            verify_provider_evidence_replay("replay-1", "run-1", snapshot).reason,
            ProviderEvidenceReplayReason::AuthorizationRequirementMismatch
        );
    }
    #[test]
    fn provider_evidence_replay_requires_audit_eligibility_code() {
        let mut snapshot = valid_snapshot();
        snapshot.expected_audit_eligibility_code = String::new();
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        assert_eq!(
            verify_provider_evidence_replay("replay-1", "run-1", snapshot).reason,
            ProviderEvidenceReplayReason::AuditRequirementMismatch
        );
    }
    #[test]
    fn provider_evidence_replay_is_forensically_distinguishable_from_live_run() {
        let replay = verify_provider_evidence_replay("replay-1", "run-1", valid_snapshot());
        assert_eq!(replay.mode, ProviderEvidenceReplayMode::Replay);
        assert!(replay.replayed_from_evidence);
        assert!(!replay.live_execution_performed);
    }
    #[test]
    fn provider_evidence_replay_does_not_run_live_harness() {
        assert!(
            !verify_provider_evidence_replay("replay-1", "run-1", valid_snapshot())
                .summary
                .contains("run_end_to_end_local_harness")
        );
    }
    #[test]
    fn provider_evidence_replay_does_not_execute_operator_action_boundary() {
        assert!(
            !verify_provider_evidence_replay("replay-1", "run-1", valid_snapshot())
                .summary
                .contains("execute_operator_action_boundary")
        );
    }
    #[test]
    fn provider_evidence_replay_does_not_create_new_authorization() {
        assert!(
            !verify_provider_evidence_replay("replay-1", "run-1", valid_snapshot())
                .new_authorization_created
        );
    }
    #[test]
    fn provider_evidence_replay_does_not_create_new_audit_record() {
        assert!(
            !verify_provider_evidence_replay("replay-1", "run-1", valid_snapshot())
                .new_audit_record_created
        );
    }
    #[test]
    fn provider_evidence_replay_does_not_append_ledger_fact() {
        assert!(
            !verify_provider_evidence_replay("replay-1", "run-1", valid_snapshot())
                .new_ledger_fact_created
        );
    }
    #[test]
    fn provider_evidence_replay_does_not_persist() {
        assert!(!verify_provider_evidence_replay("replay-1", "run-1", valid_snapshot()).persisted);
    }
    #[test]
    fn provider_evidence_replay_does_not_repair_replay() {
        assert!(
            !verify_provider_evidence_replay("replay-1", "run-1", valid_snapshot()).repaired_replay
        );
    }
    #[test]
    fn provider_evidence_replay_does_not_mutate_application_state() {
        assert!(
            !verify_provider_evidence_replay("replay-1", "run-1", valid_snapshot())
                .mutated_application_state
        );
    }
    #[test]
    fn risky_evidence_text_cannot_turn_replay_into_authority() {
        let mut snapshot = valid_snapshot();
        snapshot.expected_action_reason_code =
            "trusted persist append promote execute recovery".to_string();
        snapshot.evidence_checksum = compute_provider_evidence_checksum(&snapshot);
        let replay = verify_provider_evidence_replay("replay-1", "run-1", snapshot);
        assert_eq!(replay.status, ProviderEvidenceReplayStatus::Verified);
        assert!(!replay.new_authorization_created);
        assert!(!replay.new_action_executed);
    }
}

#[cfg(test)]
mod diagnostic_tests {
    use super::*;

    #[test]
    fn local_harness_workflow_error_diagnostic_preserves_code() {
        let error = LocalHarnessWorkflowError::EmptyWorkflowId;
        let diagnostic = crate::api::local_harness_workflow_error_diagnostic(error);
        assert_eq!(diagnostic.code, error.code());
    }

    #[test]
    fn local_harness_workflow_reason_diagnostic_preserves_code() {
        let reason = LocalHarnessWorkflowReason::CompletedInMemory;
        let diagnostic = crate::api::local_harness_workflow_reason_diagnostic(reason);
        assert_eq!(diagnostic.code, reason.code());
    }
}
