use super::*;

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

pub(crate) fn contains_secret_marker(value: &str) -> bool {
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

