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

#[cfg(test)]
mod tests {
    use super::*;

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
}
