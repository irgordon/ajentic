#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiSurface {
    Cli,
    Http,
    Ipc,
    EventStream,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorIntentType {
    Approve,
    Reject,
    Retry,
    MemoryWrite,
    MemoryDelete,
    RunStart,
    RunStop,
    ToolRequest,
    PolicyCheck,
    StateTransitionRequest,
    ContextRebuildRequest,
    ReplayRequest,
    MemorySnapshotRequest,
    MemoryDisableRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorIntent {
    pub intent_type: OperatorIntentType,
    pub reason: String,
}

impl OperatorIntent {
    pub fn new(intent_type: OperatorIntentType, reason: impl Into<String>) -> Self {
        Self {
            intent_type,
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorRoute {
    Approval,
    Rejection,
    Retry,
    MemoryWrite,
    MemoryDelete,
    RunStart,
    RunStop,
    ToolRequest,
    PolicyCheck,
    StateTransitionRequest,
    ContextRebuildRequest,
    ReplayRequest,
    MemorySnapshotRequest,
    MemoryDisableRequest,
}

impl OperatorRoute {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Approval => "approval",
            Self::Rejection => "rejection",
            Self::Retry => "retry",
            Self::MemoryWrite => "memory_write",
            Self::MemoryDelete => "memory_delete",
            Self::RunStart => "run_start",
            Self::RunStop => "run_stop",
            Self::ToolRequest => "tool_request",
            Self::PolicyCheck => "policy_check",
            Self::StateTransitionRequest => "state_transition_request",
            Self::ContextRebuildRequest => "context_rebuild_request",
            Self::ReplayRequest => "replay_request",
            Self::MemorySnapshotRequest => "memory_snapshot_request",
            Self::MemoryDisableRequest => "memory_disable_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorRouteResult {
    pub route: OperatorRoute,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorRouteError {
    EmptyIntentReason,
}
impl OperatorRouteError {
    pub fn code(&self) -> &'static str {
        "empty_intent_reason"
    }
}

pub fn route_operator_intent(
    intent: &OperatorIntent,
) -> Result<OperatorRouteResult, OperatorRouteError> {
    if intent.reason.is_empty() {
        return Err(OperatorRouteError::EmptyIntentReason);
    }
    let route = match intent.intent_type {
        OperatorIntentType::Approve => OperatorRoute::Approval,
        OperatorIntentType::Reject => OperatorRoute::Rejection,
        OperatorIntentType::Retry => OperatorRoute::Retry,
        OperatorIntentType::MemoryWrite => OperatorRoute::MemoryWrite,
        OperatorIntentType::MemoryDelete => OperatorRoute::MemoryDelete,
        OperatorIntentType::RunStart => OperatorRoute::RunStart,
        OperatorIntentType::RunStop => OperatorRoute::RunStop,
        OperatorIntentType::ToolRequest => OperatorRoute::ToolRequest,
        OperatorIntentType::PolicyCheck => OperatorRoute::PolicyCheck,
        OperatorIntentType::StateTransitionRequest => OperatorRoute::StateTransitionRequest,
        OperatorIntentType::ContextRebuildRequest => OperatorRoute::ContextRebuildRequest,
        OperatorIntentType::ReplayRequest => OperatorRoute::ReplayRequest,
        OperatorIntentType::MemorySnapshotRequest => OperatorRoute::MemorySnapshotRequest,
        OperatorIntentType::MemoryDisableRequest => OperatorRoute::MemoryDisableRequest,
    };
    Ok(OperatorRouteResult {
        route,
        reason: "operator_intent_routed",
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationSourceKind {
    LocalLlm,
    Ide,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationOutputStatus {
    Received,
    Rejected,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationTrust {
    Untrusted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationRequest {
    pub id: String,
    pub source_kind: IntegrationSourceKind,
    pub prompt_summary: String,
    pub operator_context_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationOutput {
    pub id: String,
    pub request_id: String,
    pub source_kind: IntegrationSourceKind,
    pub content: String,
    pub status: IntegrationOutputStatus,
    pub trust: IntegrationTrust,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationBoundaryError {
    EmptyRequestId,
    EmptyPromptSummary,
    EmptyOperatorContextSummary,
    EmptyOutputId,
    EmptyOutputRequestId,
    EmptyOutputContent,
}
impl IntegrationBoundaryError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyRequestId => "empty_request_id",
            Self::EmptyPromptSummary => "empty_prompt_summary",
            Self::EmptyOperatorContextSummary => "empty_operator_context_summary",
            Self::EmptyOutputId => "empty_output_id",
            Self::EmptyOutputRequestId => "empty_output_request_id",
            Self::EmptyOutputContent => "empty_output_content",
        }
    }
}

impl IntegrationRequest {
    pub fn new(
        id: impl Into<String>,
        source_kind: IntegrationSourceKind,
        prompt_summary: impl Into<String>,
        operator_context_summary: impl Into<String>,
    ) -> Result<Self, IntegrationBoundaryError> {
        let id = id.into();
        if id.is_empty() {
            return Err(IntegrationBoundaryError::EmptyRequestId);
        }
        let prompt_summary = prompt_summary.into();
        if prompt_summary.is_empty() {
            return Err(IntegrationBoundaryError::EmptyPromptSummary);
        }
        let operator_context_summary = operator_context_summary.into();
        if operator_context_summary.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOperatorContextSummary);
        }
        Ok(Self {
            id,
            source_kind,
            prompt_summary,
            operator_context_summary,
        })
    }
}

impl IntegrationOutput {
    pub fn new_untrusted(
        id: impl Into<String>,
        request_id: impl Into<String>,
        source_kind: IntegrationSourceKind,
        content: impl Into<String>,
        status: IntegrationOutputStatus,
    ) -> Result<Self, IntegrationBoundaryError> {
        let id = id.into();
        if id.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOutputId);
        }
        let request_id = request_id.into();
        if request_id.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOutputRequestId);
        }
        let content = content.into();
        if content.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOutputContent);
        }
        Ok(Self {
            id,
            request_id,
            source_kind,
            content,
            status,
            trust: IntegrationTrust::Untrusted,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalRuntimeMode {
    DryRun,
    ControlledRun,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderMode {
    Stub,
    LocalModel,
    CloudModel,
    Disabled,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeSafetyLevel {
    Strict,
    PermissivePreview,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeSafetyDefaults {
    pub require_policy_pass: bool,
    pub require_validation_pass: bool,
    pub require_replay_verification: bool,
    pub allow_provider_network: bool,
    pub allow_file_io: bool,
    pub allow_ui_mutation: bool,
}

impl RuntimeSafetyDefaults {
    pub fn strict() -> Self {
        Self {
            require_policy_pass: true,
            require_validation_pass: true,
            require_replay_verification: true,
            allow_provider_network: false,
            allow_file_io: false,
            allow_ui_mutation: false,
        }
    }

    pub fn preview() -> Self {
        Self::strict()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalWorkspaceMetadata {
    pub workspace_id: String,
    pub workspace_root: String,
    pub operator_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalRuntimeConfig {
    pub config_id: String,
    pub runtime_mode: LocalRuntimeMode,
    pub provider_mode: LocalProviderMode,
    pub safety_level: RuntimeSafetyLevel,
    pub workspace: LocalWorkspaceMetadata,
    pub safety_defaults: RuntimeSafetyDefaults,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalRuntimeConfigError {
    EmptyConfigId,
    EmptyWorkspaceId,
    EmptyWorkspaceRoot,
    EmptyOperatorLabel,
    UnsafeDefaultEnabled,
    SecretsNotAllowed,
}

impl LocalRuntimeConfigError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyConfigId => "empty_config_id",
            Self::EmptyWorkspaceId => "empty_workspace_id",
            Self::EmptyWorkspaceRoot => "empty_workspace_root",
            Self::EmptyOperatorLabel => "empty_operator_label",
            Self::UnsafeDefaultEnabled => "unsafe_default_enabled",
            Self::SecretsNotAllowed => "secrets_not_allowed",
        }
    }
}

fn contains_secret_marker(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    ["api_key", "apikey", "secret", "token", "bearer", "password"]
        .iter()
        .any(|marker| normalized.contains(marker))
}

impl LocalWorkspaceMetadata {
    pub fn new(
        workspace_id: impl Into<String>,
        workspace_root: impl Into<String>,
        operator_label: impl Into<String>,
    ) -> Result<Self, LocalRuntimeConfigError> {
        let workspace_id = workspace_id.into();
        if workspace_id.is_empty() {
            return Err(LocalRuntimeConfigError::EmptyWorkspaceId);
        }
        let workspace_root = workspace_root.into();
        if workspace_root.is_empty() {
            return Err(LocalRuntimeConfigError::EmptyWorkspaceRoot);
        }
        let operator_label = operator_label.into();
        if operator_label.is_empty() {
            return Err(LocalRuntimeConfigError::EmptyOperatorLabel);
        }
        Ok(Self {
            workspace_id,
            workspace_root,
            operator_label,
        })
    }
}

impl LocalRuntimeConfig {
    pub fn new(
        config_id: impl Into<String>,
        runtime_mode: LocalRuntimeMode,
        provider_mode: LocalProviderMode,
        safety_level: RuntimeSafetyLevel,
        workspace: LocalWorkspaceMetadata,
        safety_defaults: RuntimeSafetyDefaults,
    ) -> Result<Self, LocalRuntimeConfigError> {
        let config_id = config_id.into();
        if config_id.is_empty() {
            return Err(LocalRuntimeConfigError::EmptyConfigId);
        }
        if safety_defaults.allow_provider_network
            || safety_defaults.allow_file_io
            || safety_defaults.allow_ui_mutation
        {
            return Err(LocalRuntimeConfigError::UnsafeDefaultEnabled);
        }
        if contains_secret_marker(&config_id)
            || contains_secret_marker(&workspace.workspace_id)
            || contains_secret_marker(&workspace.workspace_root)
            || contains_secret_marker(&workspace.operator_label)
        {
            return Err(LocalRuntimeConfigError::SecretsNotAllowed);
        }
        Ok(Self {
            config_id,
            runtime_mode,
            provider_mode,
            safety_level,
            workspace,
            safety_defaults,
        })
    }
}

pub fn local_runtime_config_allows_authority_bypass(config: &LocalRuntimeConfig) -> bool {
    config.safety_defaults.allow_provider_network
        || config.safety_defaults.allow_file_io
        || config.safety_defaults.allow_ui_mutation
}

pub fn integration_source_to_provider_kind(
    source_kind: IntegrationSourceKind,
) -> crate::execution::ProviderKind {
    match source_kind {
        IntegrationSourceKind::LocalLlm => crate::execution::ProviderKind::Local,
        IntegrationSourceKind::Ide => crate::execution::ProviderKind::Ide,
        IntegrationSourceKind::Unknown => crate::execution::ProviderKind::Unknown,
    }
}

pub fn integration_request_to_provider_request(
    request: &IntegrationRequest,
) -> Result<crate::execution::ProviderRequest, crate::execution::ProviderBoundaryError> {
    crate::execution::ProviderRequest::new(
        request.id.clone(),
        integration_source_to_provider_kind(request.source_kind),
        request.prompt_summary.clone(),
    )
}

pub fn integration_output_to_provider_output(
    output: &IntegrationOutput,
) -> Result<crate::execution::ProviderOutput, crate::execution::ProviderBoundaryError> {
    let status = match output.status {
        IntegrationOutputStatus::Received => crate::execution::ProviderOutputStatus::Received,
        IntegrationOutputStatus::Rejected => crate::execution::ProviderOutputStatus::Rejected,
        IntegrationOutputStatus::Unknown => crate::execution::ProviderOutputStatus::Unknown,
    };
    crate::execution::ProviderOutput::new_untrusted(
        output.id.clone(),
        output.request_id.clone(),
        integration_source_to_provider_kind(output.source_kind),
        output.content.clone(),
        status,
    )
}

pub fn integration_output_is_authoritative(_output: &IntegrationOutput) -> bool {
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationStateError {
    EmptyStateId,
    EmptyProjectionId,
    EmptyRunId,
    EmptyContextPacketId,
    EmptyMemorySnapshotId,
    UnsafeRuntimeConfig,
    ProjectionFailed,
}

impl ApplicationStateError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyStateId => "empty_state_id",
            Self::EmptyProjectionId => "empty_projection_id",
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyContextPacketId => "empty_context_packet_id",
            Self::EmptyMemorySnapshotId => "empty_memory_snapshot_id",
            Self::UnsafeRuntimeConfig => "unsafe_runtime_config",
            Self::ProjectionFailed => "projection_failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationContextMetadata {
    pub packet_id: String,
    pub slice_count: usize,
    pub source_count: usize,
    pub budget_used: usize,
    pub budget_max: usize,
}

impl ApplicationContextMetadata {
    pub fn new(
        packet_id: impl Into<String>,
        slice_count: usize,
        source_count: usize,
        budget_used: usize,
        budget_max: usize,
    ) -> Result<Self, ApplicationStateError> {
        let packet_id = packet_id.into();
        if packet_id.is_empty() {
            return Err(ApplicationStateError::EmptyContextPacketId);
        }
        Ok(Self {
            packet_id,
            slice_count,
            source_count,
            budget_used,
            budget_max,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationMemoryMetadata {
    pub snapshot_id: String,
    pub active_entries: usize,
    pub disabled_entries: usize,
    pub rejected_entries: usize,
}

impl ApplicationMemoryMetadata {
    pub fn new(
        snapshot_id: impl Into<String>,
        active_entries: usize,
        disabled_entries: usize,
        rejected_entries: usize,
    ) -> Result<Self, ApplicationStateError> {
        let snapshot_id = snapshot_id.into();
        if snapshot_id.is_empty() {
            return Err(ApplicationStateError::EmptyMemorySnapshotId);
        }
        Ok(Self {
            snapshot_id,
            active_entries,
            disabled_entries,
            rejected_entries,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalApplicationState {
    pub state_id: String,
    pub projection_id: String,
    pub run_id: String,
    pub runtime_config: LocalRuntimeConfig,
    pub harness_state: crate::state::HarnessState,
    pub controlled_run: crate::execution::ControlledRunResult,
    pub provider_output: crate::execution::ProviderOutput,
    pub integration_output: IntegrationOutput,
    pub ledger: crate::ledger::Ledger,
    pub replay_report: crate::replay::ReplayReport,
    pub audit_projections: Vec<crate::audit::AuditProjection>,
    pub context: ApplicationContextMetadata,
    pub memory: ApplicationMemoryMetadata,
}

impl LocalApplicationState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        state_id: impl Into<String>,
        projection_id: impl Into<String>,
        run_id: impl Into<String>,
        runtime_config: LocalRuntimeConfig,
        harness_state: crate::state::HarnessState,
        controlled_run: crate::execution::ControlledRunResult,
        provider_output: crate::execution::ProviderOutput,
        integration_output: IntegrationOutput,
        ledger: crate::ledger::Ledger,
        replay_report: crate::replay::ReplayReport,
        audit_projections: Vec<crate::audit::AuditProjection>,
        context: ApplicationContextMetadata,
        memory: ApplicationMemoryMetadata,
    ) -> Result<Self, ApplicationStateError> {
        let state_id = state_id.into();
        if state_id.is_empty() {
            return Err(ApplicationStateError::EmptyStateId);
        }
        let projection_id = projection_id.into();
        if projection_id.is_empty() {
            return Err(ApplicationStateError::EmptyProjectionId);
        }
        let run_id = run_id.into();
        if run_id.is_empty() {
            return Err(ApplicationStateError::EmptyRunId);
        }
        if local_runtime_config_allows_authority_bypass(&runtime_config) {
            return Err(ApplicationStateError::UnsafeRuntimeConfig);
        }
        Ok(Self {
            state_id,
            projection_id,
            run_id,
            runtime_config,
            harness_state,
            controlled_run,
            provider_output,
            integration_output,
            ledger,
            replay_report,
            audit_projections,
            context,
            memory,
        })
    }

    pub fn state_id(&self) -> &str {
        &self.state_id
    }
    pub fn run_id(&self) -> &str {
        &self.run_id
    }
    pub fn ledger_event_count(&self) -> usize {
        self.ledger.events().len()
    }
    pub fn last_ledger_revision(&self) -> Option<u64> {
        self.ledger.last_revision()
    }

    pub fn derive_read_projection(
        &self,
    ) -> Result<ApplicationReadProjection, ApplicationStateError> {
        ApplicationReadProjection::new(
            self.projection_id.clone(),
            self.run_id.clone(),
            &self.runtime_config,
            &self.harness_state,
            &self.controlled_run,
            &self.provider_output,
            &self.integration_output,
            &self.ledger,
            &self.replay_report,
            &self.audit_projections,
            self.context.packet_id.clone(),
            self.context.slice_count,
            self.context.source_count,
            self.context.budget_used,
            self.context.budget_max,
            self.memory.snapshot_id.clone(),
            self.memory.active_entries,
            self.memory.disabled_entries,
            self.memory.rejected_entries,
        )
        .map_err(|error| match error {
            ReadProjectionError::UnsafeRuntimeConfig => ApplicationStateError::UnsafeRuntimeConfig,
            _ => ApplicationStateError::ProjectionFailed,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadProjectionStatus {
    Ready,
    Blocked,
    Rejected,
    Unknown,
    NotAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadProjectionAuthority {
    Rust,
    Ui,
    Operator,
    Provider,
    Integration,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSafetyProjection {
    pub safety_level: RuntimeSafetyLevel,
    pub require_policy_pass: bool,
    pub require_validation_pass: bool,
    pub require_replay_verification: bool,
    pub allow_provider_network: bool,
    pub allow_file_io: bool,
    pub allow_ui_mutation: bool,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleReadProjection {
    pub lifecycle: crate::state::LifecycleState,
    pub revision: u64,
    pub status: ReadProjectionStatus,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunReadProjection {
    pub run_id: String,
    pub status: crate::execution::ControlledRunStatus,
    pub reason: crate::execution::ControlledRunReason,
    pub clean_output_available: bool,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderReadProjection {
    pub provider_kind: crate::execution::ProviderKind,
    pub output_status: crate::execution::ProviderOutputStatus,
    pub output_trust: crate::execution::ProviderOutputTrust,
    pub authoritative: bool,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationReadProjection {
    pub source_kind: IntegrationSourceKind,
    pub output_status: IntegrationOutputStatus,
    pub output_trust: IntegrationTrust,
    pub authoritative: bool,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerReadProjection {
    pub event_count: usize,
    pub last_revision: Option<u64>,
    pub status: ReadProjectionStatus,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReadProjection {
    pub readiness: crate::replay::ReplayReadiness,
    pub integrity: crate::replay::ReplayIntegrity,
    pub events_replayed: usize,
    pub status: ReadProjectionStatus,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditReadProjection {
    pub projection_count: usize,
    pub latest_summary: String,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextReadProjection {
    pub packet_id: String,
    pub slice_count: usize,
    pub source_count: usize,
    pub budget_used: usize,
    pub budget_max: usize,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryReadProjection {
    pub snapshot_id: String,
    pub active_entries: usize,
    pub disabled_entries: usize,
    pub rejected_entries: usize,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputReadProjection {
    pub clean_output_available: bool,
    pub raw_output_trusted: bool,
    pub clean_output_summary: Option<String>,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationReadProjection {
    pub projection_id: String,
    pub runtime_config_id: String,
    pub safety: RuntimeSafetyProjection,
    pub lifecycle: LifecycleReadProjection,
    pub run: RunReadProjection,
    pub provider: ProviderReadProjection,
    pub integration: IntegrationReadProjection,
    pub ledger: LedgerReadProjection,
    pub replay: ReplayReadProjection,
    pub audit: AuditReadProjection,
    pub context: ContextReadProjection,
    pub memory: MemoryReadProjection,
    pub output: OutputReadProjection,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_runtime_config() -> LocalRuntimeConfig {
        LocalRuntimeConfig::new(
            "cfg-1",
            LocalRuntimeMode::DryRun,
            LocalProviderMode::Stub,
            RuntimeSafetyLevel::Strict,
            LocalWorkspaceMetadata::new("ws", "/tmp/ws", "op").unwrap(),
            RuntimeSafetyDefaults::strict(),
        )
        .unwrap()
    }
    fn sample_controlled_run() -> crate::execution::ControlledRunResult {
        crate::execution::ControlledRunResult {
            status: crate::execution::ControlledRunStatus::Accepted,
            reason: crate::execution::ControlledRunReason::RunAccepted,
            execution_decision: crate::execution::ExecutionDecisionReport::allowed(),
            promotion_decision: crate::execution::PromotionDecisionReport::allowed(),
            ledger: crate::ledger::Ledger::empty(),
            promotion_replay: crate::execution::PromotionReplayVerificationReport::verified(
                0, 0, 0,
            ),
            clean_output_summary: Some("clean".to_string()),
        }
    }
    #[test]
    fn integration_request_requires_id() {
        assert_eq!(
            IntegrationRequest::new("", IntegrationSourceKind::LocalLlm, "p", "c"),
            Err(IntegrationBoundaryError::EmptyRequestId)
        );
    }
    #[test]
    fn integration_request_requires_prompt_summary() {
        assert_eq!(
            IntegrationRequest::new("id", IntegrationSourceKind::LocalLlm, "", "c"),
            Err(IntegrationBoundaryError::EmptyPromptSummary)
        );
    }
    #[test]
    fn integration_request_requires_operator_context_summary() {
        assert_eq!(
            IntegrationRequest::new("id", IntegrationSourceKind::LocalLlm, "p", ""),
            Err(IntegrationBoundaryError::EmptyOperatorContextSummary)
        );
    }
    #[test]
    fn integration_output_requires_id() {
        assert_eq!(
            IntegrationOutput::new_untrusted(
                "",
                "req",
                IntegrationSourceKind::Ide,
                "c",
                IntegrationOutputStatus::Received
            ),
            Err(IntegrationBoundaryError::EmptyOutputId)
        );
    }
    #[test]
    fn integration_output_requires_request_id() {
        assert_eq!(
            IntegrationOutput::new_untrusted(
                "id",
                "",
                IntegrationSourceKind::Ide,
                "c",
                IntegrationOutputStatus::Received
            ),
            Err(IntegrationBoundaryError::EmptyOutputRequestId)
        );
    }
    #[test]
    fn integration_output_requires_content() {
        assert_eq!(
            IntegrationOutput::new_untrusted(
                "id",
                "req",
                IntegrationSourceKind::Ide,
                "",
                IntegrationOutputStatus::Received
            ),
            Err(IntegrationBoundaryError::EmptyOutputContent)
        );
    }
    #[test]
    fn integration_output_is_always_untrusted() {
        let o = IntegrationOutput::new_untrusted(
            "id",
            "req",
            IntegrationSourceKind::LocalLlm,
            "x",
            IntegrationOutputStatus::Received,
        )
        .unwrap();
        assert_eq!(o.trust, IntegrationTrust::Untrusted);
    }

    #[test]
    fn integration_output_keywords_remain_untrusted() {
        let o = IntegrationOutput::new_untrusted(
            "id",
            "req",
            IntegrationSourceKind::LocalLlm,
            "approved validated safe execute promote",
            IntegrationOutputStatus::Rejected,
        )
        .unwrap();
        assert_eq!(o.trust, IntegrationTrust::Untrusted);
    }
    #[test]
    fn integration_output_is_not_authoritative() {
        for source in [
            IntegrationSourceKind::LocalLlm,
            IntegrationSourceKind::Ide,
            IntegrationSourceKind::Unknown,
        ] {
            let o = IntegrationOutput::new_untrusted(
                "id",
                "req",
                source,
                "x",
                IntegrationOutputStatus::Received,
            )
            .unwrap();
            assert!(!integration_output_is_authoritative(&o));
        }
    }
    #[test]
    fn integration_boundary_error_codes_are_stable() {
        assert_eq!(
            IntegrationBoundaryError::EmptyRequestId.code(),
            "empty_request_id"
        );
        assert_eq!(
            IntegrationBoundaryError::EmptyPromptSummary.code(),
            "empty_prompt_summary"
        );
        assert_eq!(
            IntegrationBoundaryError::EmptyOperatorContextSummary.code(),
            "empty_operator_context_summary"
        );
        assert_eq!(
            IntegrationBoundaryError::EmptyOutputId.code(),
            "empty_output_id"
        );
        assert_eq!(
            IntegrationBoundaryError::EmptyOutputRequestId.code(),
            "empty_output_request_id"
        );
        assert_eq!(
            IntegrationBoundaryError::EmptyOutputContent.code(),
            "empty_output_content"
        );
    }
    #[test]
    fn integration_source_maps_local_llm_to_local_provider() {
        assert_eq!(
            integration_source_to_provider_kind(IntegrationSourceKind::LocalLlm),
            crate::execution::ProviderKind::Local
        );
    }
    #[test]
    fn integration_source_maps_ide_to_ide_provider() {
        assert_eq!(
            integration_source_to_provider_kind(IntegrationSourceKind::Ide),
            crate::execution::ProviderKind::Ide
        );
    }
    #[test]
    fn integration_source_maps_unknown_to_unknown_provider() {
        assert_eq!(
            integration_source_to_provider_kind(IntegrationSourceKind::Unknown),
            crate::execution::ProviderKind::Unknown
        );
    }
    #[test]
    fn integration_request_maps_to_provider_request() {
        let r =
            IntegrationRequest::new("id", IntegrationSourceKind::Ide, "prompt", "context").unwrap();
        let p = integration_request_to_provider_request(&r).unwrap();
        assert_eq!(p.id, "id");
        assert_eq!(p.provider_kind, crate::execution::ProviderKind::Ide);
        assert_eq!(p.prompt_summary, "prompt");
        assert_eq!(r.operator_context_summary, "context");
    }

    #[test]
    fn integration_request_does_not_copy_operator_context_into_provider_prompt() {
        let r = IntegrationRequest::new(
            "id",
            IntegrationSourceKind::Ide,
            "provider prompt only",
            "operator context is separate",
        )
        .unwrap();
        let p = integration_request_to_provider_request(&r).unwrap();
        assert_eq!(p.prompt_summary, "provider prompt only");
        assert_ne!(p.prompt_summary, r.operator_context_summary);
    }
    #[test]
    fn integration_output_maps_to_untrusted_provider_output() {
        let o = IntegrationOutput::new_untrusted(
            "out",
            "req",
            IntegrationSourceKind::LocalLlm,
            "approved validated safe execute promote",
            IntegrationOutputStatus::Received,
        )
        .unwrap();
        let p = integration_output_to_provider_output(&o).unwrap();
        assert_eq!(p.trust, crate::execution::ProviderOutputTrust::Untrusted);
        assert_eq!(p.provider_kind, crate::execution::ProviderKind::Local);
    }
    #[test]
    fn integration_output_status_maps_to_provider_output_status() {
        for (i, e) in [
            (
                IntegrationOutputStatus::Received,
                crate::execution::ProviderOutputStatus::Received,
            ),
            (
                IntegrationOutputStatus::Rejected,
                crate::execution::ProviderOutputStatus::Rejected,
            ),
            (
                IntegrationOutputStatus::Unknown,
                crate::execution::ProviderOutputStatus::Unknown,
            ),
        ] {
            let o = IntegrationOutput::new_untrusted(
                "id",
                "req",
                IntegrationSourceKind::Unknown,
                "x",
                i,
            )
            .unwrap();
            assert_eq!(integration_output_to_provider_output(&o).unwrap().status, e);
        }
    }
    #[test]
    fn integration_output_content_does_not_infer_validation() {
        let o = IntegrationOutput::new_untrusted(
            "id",
            "req",
            IntegrationSourceKind::Ide,
            "validated approved",
            IntegrationOutputStatus::Unknown,
        )
        .unwrap();
        let p = integration_output_to_provider_output(&o).unwrap();
        assert_eq!(p.trust, crate::execution::ProviderOutputTrust::Untrusted);
        assert_eq!(p.status, crate::execution::ProviderOutputStatus::Unknown);
    }
    #[test]
    fn integration_output_content_does_not_infer_policy() {
        let o = IntegrationOutput::new_untrusted(
            "id",
            "req",
            IntegrationSourceKind::Ide,
            "safe policy allow",
            IntegrationOutputStatus::Rejected,
        )
        .unwrap();
        let p = integration_output_to_provider_output(&o).unwrap();
        assert_eq!(p.trust, crate::execution::ProviderOutputTrust::Untrusted);
        assert_eq!(p.status, crate::execution::ProviderOutputStatus::Rejected);
    }
    #[test]
    fn integration_output_content_does_not_infer_execution() {
        let o = IntegrationOutput::new_untrusted(
            "id",
            "req",
            IntegrationSourceKind::Ide,
            "execute now promote",
            IntegrationOutputStatus::Received,
        )
        .unwrap();
        let p = integration_output_to_provider_output(&o).unwrap();
        assert_eq!(p.trust, crate::execution::ProviderOutputTrust::Untrusted);
        assert_eq!(p.status, crate::execution::ProviderOutputStatus::Received);
    }
    #[test]
    fn runtime_safety_strict_defaults_are_closed() {
        let defaults = RuntimeSafetyDefaults::strict();
        assert!(defaults.require_policy_pass);
        assert!(defaults.require_validation_pass);
        assert!(defaults.require_replay_verification);
        assert!(!defaults.allow_provider_network);
        assert!(!defaults.allow_file_io);
        assert!(!defaults.allow_ui_mutation);
    }

    #[test]
    fn runtime_safety_preview_defaults_do_not_enable_io_network_or_ui_mutation() {
        let defaults = RuntimeSafetyDefaults::preview();
        assert!(defaults.require_policy_pass);
        assert!(defaults.require_validation_pass);
        assert!(defaults.require_replay_verification);
        assert!(!defaults.allow_provider_network);
        assert!(!defaults.allow_file_io);
        assert!(!defaults.allow_ui_mutation);
    }

    #[test]
    fn workspace_metadata_requires_workspace_id() {
        assert_eq!(
            LocalWorkspaceMetadata::new("", "root", "op"),
            Err(LocalRuntimeConfigError::EmptyWorkspaceId)
        );
    }

    #[test]
    fn workspace_metadata_requires_workspace_root() {
        assert_eq!(
            LocalWorkspaceMetadata::new("id", "", "op"),
            Err(LocalRuntimeConfigError::EmptyWorkspaceRoot)
        );
    }

    #[test]
    fn workspace_metadata_requires_operator_label() {
        assert_eq!(
            LocalWorkspaceMetadata::new("id", "root", ""),
            Err(LocalRuntimeConfigError::EmptyOperatorLabel)
        );
    }

    #[test]
    fn workspace_metadata_does_not_require_existing_path() {
        let workspace = LocalWorkspaceMetadata::new("id", "/path/does/not/exist", "op").unwrap();
        assert_eq!(workspace.workspace_root, "/path/does/not/exist");
    }

    fn valid_workspace() -> LocalWorkspaceMetadata {
        LocalWorkspaceMetadata::new("workspace", "/caller/supplied/path", "operator").unwrap()
    }

    #[test]
    fn local_runtime_config_requires_config_id() {
        assert_eq!(
            LocalRuntimeConfig::new(
                "",
                LocalRuntimeMode::DryRun,
                LocalProviderMode::Stub,
                RuntimeSafetyLevel::Strict,
                valid_workspace(),
                RuntimeSafetyDefaults::strict()
            ),
            Err(LocalRuntimeConfigError::EmptyConfigId)
        );
    }

    #[test]
    fn local_runtime_config_rejects_provider_network_default() {
        let mut defaults = RuntimeSafetyDefaults::strict();
        defaults.allow_provider_network = true;
        assert_eq!(
            LocalRuntimeConfig::new(
                "cfg",
                LocalRuntimeMode::DryRun,
                LocalProviderMode::Stub,
                RuntimeSafetyLevel::Strict,
                valid_workspace(),
                defaults
            ),
            Err(LocalRuntimeConfigError::UnsafeDefaultEnabled)
        );
    }

    #[test]
    fn local_runtime_config_rejects_file_io_default() {
        let mut defaults = RuntimeSafetyDefaults::strict();
        defaults.allow_file_io = true;
        assert_eq!(
            LocalRuntimeConfig::new(
                "cfg",
                LocalRuntimeMode::DryRun,
                LocalProviderMode::Stub,
                RuntimeSafetyLevel::Strict,
                valid_workspace(),
                defaults
            ),
            Err(LocalRuntimeConfigError::UnsafeDefaultEnabled)
        );
    }

    #[test]
    fn local_runtime_config_rejects_ui_mutation_default() {
        let mut defaults = RuntimeSafetyDefaults::strict();
        defaults.allow_ui_mutation = true;
        assert_eq!(
            LocalRuntimeConfig::new(
                "cfg",
                LocalRuntimeMode::DryRun,
                LocalProviderMode::Stub,
                RuntimeSafetyLevel::Strict,
                valid_workspace(),
                defaults
            ),
            Err(LocalRuntimeConfigError::UnsafeDefaultEnabled)
        );
    }

    #[test]
    fn local_runtime_config_rejects_secret_markers_in_config_id() {
        assert_eq!(
            LocalRuntimeConfig::new(
                "my_api_key",
                LocalRuntimeMode::DryRun,
                LocalProviderMode::Stub,
                RuntimeSafetyLevel::Strict,
                valid_workspace(),
                RuntimeSafetyDefaults::strict()
            ),
            Err(LocalRuntimeConfigError::SecretsNotAllowed)
        );
    }

    #[test]
    fn local_runtime_config_rejects_secret_markers_in_workspace_metadata() {
        let workspace =
            LocalWorkspaceMetadata::new("workspace", "/root", "Bearer operator").unwrap();
        assert_eq!(
            LocalRuntimeConfig::new(
                "cfg",
                LocalRuntimeMode::DryRun,
                LocalProviderMode::Stub,
                RuntimeSafetyLevel::Strict,
                workspace,
                RuntimeSafetyDefaults::strict()
            ),
            Err(LocalRuntimeConfigError::SecretsNotAllowed)
        );
    }

    #[test]
    fn local_runtime_config_does_not_allow_authority_bypass() {
        let config = LocalRuntimeConfig::new(
            "cfg",
            LocalRuntimeMode::ControlledRun,
            LocalProviderMode::Disabled,
            RuntimeSafetyLevel::PermissivePreview,
            valid_workspace(),
            RuntimeSafetyDefaults::preview(),
        )
        .unwrap();
        assert!(!local_runtime_config_allows_authority_bypass(&config));
    }

    #[test]
    fn local_runtime_config_error_codes_are_stable() {
        assert_eq!(
            LocalRuntimeConfigError::EmptyConfigId.code(),
            "empty_config_id"
        );
        assert_eq!(
            LocalRuntimeConfigError::EmptyWorkspaceId.code(),
            "empty_workspace_id"
        );
        assert_eq!(
            LocalRuntimeConfigError::EmptyWorkspaceRoot.code(),
            "empty_workspace_root"
        );
        assert_eq!(
            LocalRuntimeConfigError::EmptyOperatorLabel.code(),
            "empty_operator_label"
        );
        assert_eq!(
            LocalRuntimeConfigError::UnsafeDefaultEnabled.code(),
            "unsafe_default_enabled"
        );
        assert_eq!(
            LocalRuntimeConfigError::SecretsNotAllowed.code(),
            "secrets_not_allowed"
        );
    }

    #[test]
    fn local_runtime_config_does_not_call_provider_or_controlled_flow() {
        let workspace = LocalWorkspaceMetadata::new("workspace", "/root", "operator").unwrap();
        let config = LocalRuntimeConfig::new(
            "cfg",
            LocalRuntimeMode::DryRun,
            LocalProviderMode::Stub,
            RuntimeSafetyLevel::Strict,
            workspace,
            RuntimeSafetyDefaults::strict(),
        )
        .unwrap();
        assert_eq!(config.provider_mode, LocalProviderMode::Stub);
    }

    #[test]
    fn local_runtime_config_does_not_read_or_canonicalize_workspace_path() {
        let workspace =
            LocalWorkspaceMetadata::new("workspace", "./relative/../caller-path", "operator")
                .unwrap();
        let config = LocalRuntimeConfig::new(
            "cfg",
            LocalRuntimeMode::DryRun,
            LocalProviderMode::Disabled,
            RuntimeSafetyLevel::Strict,
            workspace,
            RuntimeSafetyDefaults::strict(),
        )
        .unwrap();
        assert_eq!(config.workspace.workspace_root, "./relative/../caller-path");
    }

    #[test]
    fn integration_mapping_does_not_call_controlled_flow() {
        let r = IntegrationRequest::new("id", IntegrationSourceKind::LocalLlm, "p", "c").unwrap();
        let o = IntegrationOutput::new_untrusted(
            "out",
            "id",
            IntegrationSourceKind::LocalLlm,
            "content",
            IntegrationOutputStatus::Received,
        )
        .unwrap();
        assert!(integration_request_to_provider_request(&r).is_ok());
        assert!(integration_output_to_provider_output(&o).is_ok());
    }

    fn projection_fixtures() -> (
        LocalRuntimeConfig,
        crate::state::HarnessState,
        crate::execution::ControlledRunResult,
        crate::execution::ProviderOutput,
        IntegrationOutput,
        crate::ledger::Ledger,
        crate::replay::ReplayReport,
        Vec<crate::audit::AuditProjection>,
    ) {
        let workspace =
            LocalWorkspaceMetadata::new("workspace", "/workspace/ajentic", "operator").unwrap();
        let runtime = LocalRuntimeConfig::new(
            "cfg-1",
            LocalRuntimeMode::DryRun,
            LocalProviderMode::Stub,
            RuntimeSafetyLevel::Strict,
            workspace,
            RuntimeSafetyDefaults::strict(),
        )
        .unwrap();
        let state = crate::state::HarnessState {
            revision: 3,
            lifecycle: crate::state::LifecycleState::Passed,
        };
        let run = crate::execution::ControlledRunResult {
            status: crate::execution::ControlledRunStatus::Accepted,
            reason: crate::execution::ControlledRunReason::RunAccepted,
            execution_decision: crate::execution::ExecutionDecisionReport::allowed(),
            promotion_decision: crate::execution::PromotionDecisionReport::blocked(
                crate::execution::PromotionDecisionReason::ExecutionNotAllowed,
            ),
            ledger: crate::ledger::Ledger::empty(),
            promotion_replay: crate::execution::PromotionReplayVerificationReport::verified(
                1, 1, 1,
            ),
            clean_output_summary: Some("clean".to_string()),
        };
        let provider = crate::execution::ProviderOutput::new_untrusted(
            "p-out",
            "p-req",
            crate::execution::ProviderKind::Local,
            "raw",
            crate::execution::ProviderOutputStatus::Received,
        )
        .unwrap();
        let integration = IntegrationOutput::new_untrusted(
            "i-out",
            "i-req",
            IntegrationSourceKind::LocalLlm,
            "raw",
            IntegrationOutputStatus::Received,
        )
        .unwrap();
        let actor =
            crate::ledger::LedgerActor::new(crate::ledger::LedgerActorType::System, "actor")
                .unwrap();
        let payload = crate::ledger::LedgerPayload::new("summary").unwrap();
        let event = crate::ledger::LedgerEvent::new(
            "e1",
            1,
            crate::ledger::LedgerEventType::StateTransition,
            actor,
            vec!["ev1".to_string()],
            payload,
        )
        .unwrap();
        let ledger = crate::ledger::Ledger::empty().append(event).unwrap();
        let replay = crate::replay::ReplayReport::replayable(1);
        let audit = vec![crate::audit::project_replay_summary(&replay)];
        (
            runtime,
            state,
            run,
            provider,
            integration,
            ledger,
            replay,
            audit,
        )
    }

    #[test]
    fn read_projection_error_codes_are_stable() {
        assert_eq!(
            ReadProjectionError::EmptyProjectionId.code(),
            "empty_projection_id"
        );
        assert_eq!(
            ReadProjectionError::EmptyRuntimeConfigId.code(),
            "empty_runtime_config_id"
        );
        assert_eq!(ReadProjectionError::EmptyRunId.code(), "empty_run_id");
        assert_eq!(
            ReadProjectionError::EmptyContextPacketId.code(),
            "empty_context_packet_id"
        );
        assert_eq!(
            ReadProjectionError::EmptyMemorySnapshotId.code(),
            "empty_memory_snapshot_id"
        );
        assert_eq!(
            ReadProjectionError::UnsafeRuntimeConfig.code(),
            "unsafe_runtime_config"
        );
    }

    #[test]
    fn application_read_projection_requires_projection_id() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        assert_eq!(
            ApplicationReadProjection::new(
                "",
                "run-1",
                &runtime,
                &state,
                &run,
                &provider,
                &integration,
                &ledger,
                &replay,
                &audit,
                "ctx",
                1,
                1,
                1,
                2,
                "mem",
                1,
                0,
                0
            ),
            Err(ReadProjectionError::EmptyProjectionId)
        );
    }
    #[test]
    fn application_read_projection_requires_runtime_config_id() {
        let (mut runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        runtime.config_id = "".into();
        assert_eq!(
            ApplicationReadProjection::new(
                "proj",
                "run-1",
                &runtime,
                &state,
                &run,
                &provider,
                &integration,
                &ledger,
                &replay,
                &audit,
                "ctx",
                1,
                1,
                1,
                2,
                "mem",
                1,
                0,
                0
            ),
            Err(ReadProjectionError::EmptyRuntimeConfigId)
        );
    }
    #[test]
    fn application_read_projection_requires_run_id() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        assert_eq!(
            ApplicationReadProjection::new(
                "proj",
                "",
                &runtime,
                &state,
                &run,
                &provider,
                &integration,
                &ledger,
                &replay,
                &audit,
                "ctx",
                1,
                1,
                1,
                2,
                "mem",
                1,
                0,
                0
            ),
            Err(ReadProjectionError::EmptyRunId)
        );
    }
    #[test]
    fn application_read_projection_requires_context_packet_id() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        assert_eq!(
            ApplicationReadProjection::new(
                "proj",
                "run",
                &runtime,
                &state,
                &run,
                &provider,
                &integration,
                &ledger,
                &replay,
                &audit,
                "",
                1,
                1,
                1,
                2,
                "mem",
                1,
                0,
                0
            ),
            Err(ReadProjectionError::EmptyContextPacketId)
        );
    }
    #[test]
    fn application_read_projection_requires_memory_snapshot_id() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        assert_eq!(
            ApplicationReadProjection::new(
                "proj",
                "run",
                &runtime,
                &state,
                &run,
                &provider,
                &integration,
                &ledger,
                &replay,
                &audit,
                "ctx",
                1,
                1,
                1,
                2,
                "",
                1,
                0,
                0
            ),
            Err(ReadProjectionError::EmptyMemorySnapshotId)
        );
    }
    #[test]
    fn application_read_projection_rejects_unsafe_runtime_config() {
        let (mut runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        runtime.safety_defaults.allow_file_io = true;
        assert_eq!(
            ApplicationReadProjection::new(
                "proj",
                "run",
                &runtime,
                &state,
                &run,
                &provider,
                &integration,
                &ledger,
                &replay,
                &audit,
                "ctx",
                1,
                1,
                1,
                2,
                "mem",
                1,
                0,
                0
            ),
            Err(ReadProjectionError::UnsafeRuntimeConfig)
        );
    }
    #[test]
    fn safety_projection_exposes_runtime_safety_defaults() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            2,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert!(p.safety.require_policy_pass);
        assert!(p.safety.require_validation_pass);
        assert!(p.safety.require_replay_verification);
    }
    #[test]
    fn safety_projection_exposes_runtime_safety_level() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            2,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert_eq!(p.safety.safety_level, RuntimeSafetyLevel::Strict);
    }
    #[test]
    fn provider_projection_marks_provider_output_untrusted() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            2,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert_eq!(
            p.provider.output_trust,
            crate::execution::ProviderOutputTrust::Untrusted
        );
    }
    #[test]
    fn provider_projection_marks_provider_output_non_authoritative() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            2,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert!(!p.provider.authoritative);
    }
    #[test]
    fn integration_projection_marks_integration_output_untrusted() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            2,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert_eq!(p.integration.output_trust, IntegrationTrust::Untrusted);
    }
    #[test]
    fn integration_projection_marks_integration_output_non_authoritative() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            2,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert!(!p.integration.authoritative);
    }
    #[test]
    fn output_projection_marks_raw_output_untrusted() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            2,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert!(!p.output.raw_output_trusted);
    }
    #[test]
    fn ledger_projection_reports_event_count_and_last_revision() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            2,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert_eq!(p.ledger.event_count, 1);
        assert_eq!(p.ledger.last_revision, Some(1));
    }
    #[test]
    fn replay_projection_reports_readiness_integrity_and_event_count() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            2,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert_eq!(p.replay.readiness, crate::replay::ReplayReadiness::Ready);
        assert_eq!(p.replay.integrity, crate::replay::ReplayIntegrity::Valid);
        assert_eq!(p.replay.events_replayed, 1);
    }
    #[test]
    fn audit_projection_reports_projection_count_and_latest_summary() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            2,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert_eq!(p.audit.projection_count, 1);
        assert_eq!(p.audit.latest_summary, audit[0].summary);
    }
    #[test]
    fn context_projection_uses_supplied_metadata_only() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx-supplied",
            5,
            7,
            9,
            11,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert_eq!(p.context.packet_id, "ctx-supplied");
        assert_eq!(p.context.slice_count, 5);
        assert_eq!(p.context.source_count, 7);
        assert_eq!(p.context.budget_used, 9);
        assert_eq!(p.context.budget_max, 11);
    }
    #[test]
    fn memory_projection_uses_supplied_metadata_only() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            1,
            "mem-supplied",
            10,
            2,
            3,
        )
        .unwrap();
        assert_eq!(p.memory.snapshot_id, "mem-supplied");
        assert_eq!(p.memory.active_entries, 10);
        assert_eq!(p.memory.disabled_entries, 2);
        assert_eq!(p.memory.rejected_entries, 3);
    }
    #[test]
    fn application_read_projection_does_not_execute_controlled_flow() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run-explicit",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            1,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert_eq!(p.run.run_id, "run-explicit");
    }
    #[test]
    fn application_read_projection_does_not_verify_or_repair_replay() {
        let (runtime, state, run, provider, integration, ledger, _replay, audit) =
            projection_fixtures();
        let replay = crate::replay::ReplayReport::unknown();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            1,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert_eq!(p.replay.readiness, crate::replay::ReplayReadiness::NotReady);
    }
    #[test]
    fn application_read_projection_does_not_read_files_or_call_network() {
        let (runtime, state, run, provider, integration, ledger, replay, audit) =
            projection_fixtures();
        let p = ApplicationReadProjection::new(
            "proj",
            "run",
            &runtime,
            &state,
            &run,
            &provider,
            &integration,
            &ledger,
            &replay,
            &audit,
            "ctx",
            1,
            1,
            1,
            1,
            "mem",
            1,
            0,
            0,
        )
        .unwrap();
        assert_eq!(p.output.summary, "raw provider output remains untrusted");
    }
    fn sample_app_state() -> LocalApplicationState {
        let provider_output = crate::execution::ProviderOutput::new_untrusted(
            "po",
            "pr",
            crate::execution::ProviderKind::Local,
            "raw",
            crate::execution::ProviderOutputStatus::Received,
        )
        .unwrap();
        let integration_output = IntegrationOutput::new_untrusted(
            "io",
            "pr",
            IntegrationSourceKind::LocalLlm,
            "raw",
            IntegrationOutputStatus::Received,
        )
        .unwrap();
        let actor =
            crate::ledger::LedgerActor::new(crate::ledger::LedgerActorType::System, "actor")
                .unwrap();
        let payload = crate::ledger::LedgerPayload::new("summary").unwrap();
        let event = crate::ledger::LedgerEvent::new(
            "e1",
            1,
            crate::ledger::LedgerEventType::StateTransition,
            actor,
            vec!["ev".to_string()],
            payload,
        )
        .unwrap();
        let ledger = crate::ledger::Ledger::empty().append(event).unwrap();
        let replay_report = crate::replay::ReplayReport::replayable(1);
        let audit = crate::audit::AuditProjection::new(
            crate::audit::AuditProjectionType::Timeline,
            vec![crate::audit::AuditSourceRef::new(
                crate::audit::AuditSourceType::LedgerEvent,
                "e1",
            )
            .unwrap()],
            "sum",
            vec!["d".to_string()],
        )
        .unwrap();
        LocalApplicationState::new(
            "state-1",
            "projection-1",
            "run-1",
            sample_runtime_config(),
            crate::state::HarnessState::genesis(),
            sample_controlled_run(),
            provider_output,
            integration_output,
            ledger,
            replay_report,
            vec![audit],
            ApplicationContextMetadata::new("pkt", 2, 3, 4, 5).unwrap(),
            ApplicationMemoryMetadata::new("mem", 6, 7, 8).unwrap(),
        )
        .unwrap()
    }
    #[test]
    fn application_state_error_codes_are_stable() {
        assert_eq!(ApplicationStateError::EmptyStateId.code(), "empty_state_id");
        assert_eq!(
            ApplicationStateError::ProjectionFailed.code(),
            "projection_failed"
        );
    }
    #[test]
    fn application_context_metadata_requires_packet_id() {
        assert_eq!(
            ApplicationContextMetadata::new("", 1, 1, 1, 1),
            Err(ApplicationStateError::EmptyContextPacketId)
        );
    }
    #[test]
    fn application_memory_metadata_requires_snapshot_id() {
        assert_eq!(
            ApplicationMemoryMetadata::new("", 1, 1, 1),
            Err(ApplicationStateError::EmptyMemorySnapshotId)
        );
    }
    #[test]
    fn local_application_state_requires_state_id() {
        let s = sample_app_state();
        assert_eq!(
            LocalApplicationState::new(
                "",
                s.projection_id,
                s.run_id,
                s.runtime_config,
                s.harness_state,
                s.controlled_run,
                s.provider_output,
                s.integration_output,
                s.ledger,
                s.replay_report,
                s.audit_projections,
                s.context,
                s.memory
            ),
            Err(ApplicationStateError::EmptyStateId)
        );
    }
    #[test]
    fn local_application_state_requires_projection_id() {
        let s = sample_app_state();
        assert_eq!(
            LocalApplicationState::new(
                "s",
                "",
                s.run_id,
                s.runtime_config,
                s.harness_state,
                s.controlled_run,
                s.provider_output,
                s.integration_output,
                s.ledger,
                s.replay_report,
                s.audit_projections,
                s.context,
                s.memory
            ),
            Err(ApplicationStateError::EmptyProjectionId)
        );
    }
    #[test]
    fn local_application_state_requires_run_id() {
        let s = sample_app_state();
        assert_eq!(
            LocalApplicationState::new(
                "s",
                "p",
                "",
                s.runtime_config,
                s.harness_state,
                s.controlled_run,
                s.provider_output,
                s.integration_output,
                s.ledger,
                s.replay_report,
                s.audit_projections,
                s.context,
                s.memory
            ),
            Err(ApplicationStateError::EmptyRunId)
        );
    }
    #[test]
    fn local_application_state_rejects_unsafe_runtime_config() {
        let mut cfg = sample_runtime_config();
        cfg.safety_defaults.allow_provider_network = true;
        let s = sample_app_state();
        assert_eq!(
            LocalApplicationState::new(
                "s",
                "p",
                "r",
                cfg,
                s.harness_state,
                s.controlled_run,
                s.provider_output,
                s.integration_output,
                s.ledger,
                s.replay_report,
                s.audit_projections,
                s.context,
                s.memory
            ),
            Err(ApplicationStateError::UnsafeRuntimeConfig)
        );
    }
    #[test]
    fn local_application_state_stores_runtime_config() {
        assert_eq!(sample_app_state().runtime_config.config_id, "cfg-1");
    }
    #[test]
    fn local_application_state_stores_harness_state() {
        assert_eq!(sample_app_state().harness_state.revision, 0);
    }
    #[test]
    fn local_application_state_stores_provider_and_integration_outputs_as_untrusted() {
        let s = sample_app_state();
        assert_eq!(
            s.provider_output.trust,
            crate::execution::ProviderOutputTrust::Untrusted
        );
        assert_eq!(s.integration_output.trust, IntegrationTrust::Untrusted);
    }
    #[test]
    fn local_application_state_reports_ledger_event_count() {
        assert_eq!(sample_app_state().ledger_event_count(), 1);
    }
    #[test]
    fn local_application_state_reports_last_ledger_revision() {
        assert_eq!(sample_app_state().last_ledger_revision(), Some(1));
    }
    #[test]
    fn derive_read_projection_returns_runtime_safety_posture() {
        assert_eq!(
            sample_app_state()
                .derive_read_projection()
                .unwrap()
                .safety
                .safety_level,
            RuntimeSafetyLevel::Strict
        );
    }
    #[test]
    fn derive_read_projection_returns_provider_untrusted_projection() {
        assert_eq!(
            sample_app_state()
                .derive_read_projection()
                .unwrap()
                .provider
                .output_trust,
            crate::execution::ProviderOutputTrust::Untrusted
        );
    }
    #[test]
    fn derive_read_projection_returns_integration_untrusted_projection() {
        assert_eq!(
            sample_app_state()
                .derive_read_projection()
                .unwrap()
                .integration
                .output_trust,
            IntegrationTrust::Untrusted
        );
    }
    #[test]
    fn derive_read_projection_returns_context_metadata() {
        assert_eq!(
            sample_app_state()
                .derive_read_projection()
                .unwrap()
                .context
                .packet_id,
            "pkt"
        );
    }
    #[test]
    fn derive_read_projection_returns_memory_metadata() {
        assert_eq!(
            sample_app_state()
                .derive_read_projection()
                .unwrap()
                .memory
                .snapshot_id,
            "mem"
        );
    }
    #[test]
    fn derive_read_projection_is_idempotent_for_same_state() {
        let s = sample_app_state();
        assert_eq!(
            s.derive_read_projection().unwrap(),
            s.derive_read_projection().unwrap()
        );
    }
    #[test]
    fn derive_read_projection_does_not_mutate_ledger() {
        let s = sample_app_state();
        let before = s.ledger.clone();
        let _ = s.derive_read_projection().unwrap();
        assert_eq!(s.ledger, before);
    }
    #[test]
    fn derive_read_projection_does_not_mutate_audit_projection_count() {
        let s = sample_app_state();
        let before = s.audit_projections.len();
        let _ = s.derive_read_projection().unwrap();
        assert_eq!(s.audit_projections.len(), before);
    }
    #[test]
    fn derive_read_projection_does_not_execute_controlled_flow() {
        let s = sample_app_state();
        let _ = s.derive_read_projection().unwrap();
        assert_eq!(
            s.controlled_run.status,
            crate::execution::ControlledRunStatus::Accepted
        );
    }
    #[test]
    fn derive_read_projection_does_not_verify_or_repair_replay() {
        let s = sample_app_state();
        let _ = s.derive_read_projection().unwrap();
        assert_eq!(s.replay_report.events_replayed, 1);
    }
    #[test]
    fn derive_read_projection_does_not_read_files_or_call_network() {
        let p = sample_app_state().derive_read_projection().unwrap();
        assert!(!p.safety.allow_file_io && !p.safety.allow_provider_network);
    }
}
