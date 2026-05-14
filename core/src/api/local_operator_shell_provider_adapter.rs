//! Local provider adapter contract, registry, validation, and dry-run helpers.

use super::{
    attach_local_session_evidence_export,
    local_operator_shell_boundary_markers::{
        local_provider_adapter_dry_run_boundary_statuses,
        local_provider_adapter_dry_run_capability_surface,
        local_provider_adapter_dry_run_effect_statuses,
    },
    LocalOperatorShellState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterKind {
    DeterministicFakeAdapter,
    LocalModelAdapterContract,
    UnsupportedLocalModel,
    UnsupportedCloudModel,
    UnsupportedNetworkAdapter,
    UnsupportedShellAdapter,
    UnsupportedFilesystemAdapter,
    Unknown,
}

impl LocalProviderAdapterKind {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "deterministic_fake_adapter" => Some(Self::DeterministicFakeAdapter),
            "local_model_adapter_contract" => Some(Self::LocalModelAdapterContract),
            "unsupported_local_model" => Some(Self::UnsupportedLocalModel),
            "unsupported_cloud_model" => Some(Self::UnsupportedCloudModel),
            "unsupported_network_adapter" => Some(Self::UnsupportedNetworkAdapter),
            "unsupported_shell_adapter" => Some(Self::UnsupportedShellAdapter),
            "unsupported_filesystem_adapter" => Some(Self::UnsupportedFilesystemAdapter),
            "unknown" => Some(Self::Unknown),
            _ => None,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::DeterministicFakeAdapter => "deterministic_fake_adapter",
            Self::LocalModelAdapterContract => "local_model_adapter_contract",
            Self::UnsupportedLocalModel => "unsupported_local_model",
            Self::UnsupportedCloudModel => "unsupported_cloud_model",
            Self::UnsupportedNetworkAdapter => "unsupported_network_adapter",
            Self::UnsupportedShellAdapter => "unsupported_shell_adapter",
            Self::UnsupportedFilesystemAdapter => "unsupported_filesystem_adapter",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterValidationStatus {
    RegistryProjected,
    AdapterDeclaredNonExecuting,
    AdapterRejected,
    UnsupportedAdapter,
    InvalidAdapterDeclaration,
}

impl LocalProviderAdapterValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::RegistryProjected => "registry_projected",
            Self::AdapterDeclaredNonExecuting => "adapter_declared_non_executing",
            Self::AdapterRejected => "adapter_rejected",
            Self::UnsupportedAdapter => "unsupported_adapter",
            Self::InvalidAdapterDeclaration => "invalid_adapter_declaration",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalProviderAdapterValidationError {
    MissingAdapterKind,
    MalformedAdapterKind,
    UnsupportedAdapter,
    CloudOrNetworkAdapterRejected,
    ShellAdapterRejected,
    FilesystemAdapterRejected,
    ExecutablePathRejected,
    EndpointFieldRejected,
    CommandFieldRejected,
    PathFieldRejected,
    SecretFieldRejected,
    ExecutionFlagRejected,
    ProviderTrustFlagRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    SigningClaimRejected,
    PublishingClaimRejected,
    UnknownFieldRejected,
}

impl LocalProviderAdapterValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingAdapterKind => "missing_adapter_kind",
            Self::MalformedAdapterKind => "malformed_adapter_kind",
            Self::UnsupportedAdapter => "unsupported_adapter",
            Self::CloudOrNetworkAdapterRejected => "cloud_or_network_adapter_rejected",
            Self::ShellAdapterRejected => "shell_adapter_rejected",
            Self::FilesystemAdapterRejected => "filesystem_adapter_rejected",
            Self::ExecutablePathRejected => "executable_path_rejected",
            Self::EndpointFieldRejected => "endpoint_field_rejected",
            Self::CommandFieldRejected => "command_field_rejected",
            Self::PathFieldRejected => "path_field_rejected",
            Self::SecretFieldRejected => "secret_field_rejected",
            Self::ExecutionFlagRejected => "execution_flag_rejected",
            Self::ProviderTrustFlagRejected => "provider_trust_flag_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::SigningClaimRejected => "signing_claim_rejected",
            Self::PublishingClaimRejected => "publishing_claim_rejected",
            Self::UnknownFieldRejected => "unknown_field_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterExecutionStatus {
    ExecutionNotAvailableInPhase153,
}

impl LocalProviderAdapterExecutionStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ExecutionNotAvailableInPhase153 => "execution_not_available_in_phase_153",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterTrustStatus {
    NoProviderTrust,
}

impl LocalProviderAdapterTrustStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoProviderTrust => "no_provider_trust",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterBoundaryStatus {
    ContractOnly,
    NoExecution,
    NoProviderTrust,
    NoNetwork,
    NoShell,
    NoSecrets,
    NoProductionPersistence,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl LocalProviderAdapterBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ContractOnly => "contract_only",
            Self::NoExecution => "no_execution",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoNetwork => "no_network",
            Self::NoShell => "no_shell",
            Self::NoSecrets => "no_secrets",
            Self::NoProductionPersistence => "no_production_persistence",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NoPublicUseEffect => "no_public_use_effect",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterCapabilitySurface {
    pub contract_only: bool,
    pub no_execution: bool,
    pub no_provider_trust: bool,
    pub no_network: bool,
    pub no_shell: bool,
    pub no_secrets: bool,
    pub no_production_persistence: bool,
    pub no_readiness_effect: bool,
    pub no_release_effect: bool,
    pub no_deployment_effect: bool,
    pub no_public_use_effect: bool,
    pub summary: String,
}

pub fn local_provider_adapter_capability_surface() -> LocalProviderAdapterCapabilitySurface {
    LocalProviderAdapterCapabilitySurface {
        contract_only: true,
        no_execution: true,
        no_provider_trust: true,
        no_network: true,
        no_shell: true,
        no_secrets: true,
        no_production_persistence: true,
        no_readiness_effect: true,
        no_release_effect: true,
        no_deployment_effect: true,
        no_public_use_effect: true,
        summary: "Adapter contract only; no model execution is available in Phase 153. No network, shell, secret, or production persistence capability is enabled.".to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterContract {
    pub adapter_kind: LocalProviderAdapterKind,
    pub capability_surface: LocalProviderAdapterCapabilitySurface,
    pub execution_status: LocalProviderAdapterExecutionStatus,
    pub trust_status: LocalProviderAdapterTrustStatus,
    pub boundary_statuses: Vec<LocalProviderAdapterBoundaryStatus>,
}

pub fn local_provider_adapter_contract(
    adapter_kind: LocalProviderAdapterKind,
) -> LocalProviderAdapterContract {
    LocalProviderAdapterContract {
        adapter_kind,
        capability_surface: local_provider_adapter_capability_surface(),
        execution_status: LocalProviderAdapterExecutionStatus::ExecutionNotAvailableInPhase153,
        trust_status: LocalProviderAdapterTrustStatus::NoProviderTrust,
        boundary_statuses: vec![
            LocalProviderAdapterBoundaryStatus::ContractOnly,
            LocalProviderAdapterBoundaryStatus::NoExecution,
            LocalProviderAdapterBoundaryStatus::NoProviderTrust,
            LocalProviderAdapterBoundaryStatus::NoNetwork,
            LocalProviderAdapterBoundaryStatus::NoShell,
            LocalProviderAdapterBoundaryStatus::NoSecrets,
            LocalProviderAdapterBoundaryStatus::NoProductionPersistence,
            LocalProviderAdapterBoundaryStatus::NoReadinessEffect,
            LocalProviderAdapterBoundaryStatus::NoReleaseEffect,
            LocalProviderAdapterBoundaryStatus::NoDeploymentEffect,
            LocalProviderAdapterBoundaryStatus::NoPublicUseEffect,
        ],
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterDeclaration {
    pub adapter_kind: LocalProviderAdapterKind,
    pub declaration_id: String,
    pub status: LocalProviderAdapterValidationStatus,
    pub contract: LocalProviderAdapterContract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterConfigurationCandidate {
    pub adapter_kind: Option<String>,
    pub declaration_id: Option<String>,
    pub fields: Vec<(String, String)>,
}

impl LocalProviderAdapterConfigurationCandidate {
    pub fn deterministic_fake_adapter() -> Self {
        Self {
            adapter_kind: Some("deterministic_fake_adapter".to_string()),
            declaration_id: Some("local-adapter-declaration-deterministic-fake".to_string()),
            fields: Vec::new(),
        }
    }

    pub fn local_model_adapter_contract() -> Self {
        Self {
            adapter_kind: Some("local_model_adapter_contract".to_string()),
            declaration_id: Some("local-adapter-declaration-local-model-contract".to_string()),
            fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterValidation {
    pub status: LocalProviderAdapterValidationStatus,
    pub adapter_kind: Option<LocalProviderAdapterKind>,
    pub declaration_id: Option<String>,
    pub error_codes: Vec<LocalProviderAdapterValidationError>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterRegistryProjection {
    pub registry_status: LocalProviderAdapterValidationStatus,
    pub supported_adapter_kinds: Vec<String>,
    pub rejected_adapter_kinds: Vec<String>,
    pub declarations: Vec<LocalProviderAdapterDeclaration>,
    pub last_validation: LocalProviderAdapterValidation,
    pub capability_surface: LocalProviderAdapterCapabilitySurface,
    pub execution_status: String,
    pub trust_status: String,
    pub boundary_statuses: Vec<String>,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterRegistry {
    pub declarations: Vec<LocalProviderAdapterDeclaration>,
    pub last_validation: LocalProviderAdapterValidation,
}

pub fn initial_local_provider_adapter_registry() -> LocalProviderAdapterRegistry {
    LocalProviderAdapterRegistry {
        declarations: Vec::new(),
        last_validation: LocalProviderAdapterValidation {
            status: LocalProviderAdapterValidationStatus::RegistryProjected,
            adapter_kind: None,
            declaration_id: None,
            error_codes: Vec::new(),
            reason: "initial local provider adapter registry projected; no adapter declarations execute in Phase 153".to_string(),
        },
    }
}

pub fn project_local_provider_adapter_registry(
    registry: &LocalProviderAdapterRegistry,
) -> LocalProviderAdapterRegistryProjection {
    let boundary_statuses =
        local_provider_adapter_contract(LocalProviderAdapterKind::DeterministicFakeAdapter)
            .boundary_statuses
            .iter()
            .map(|status| status.code().to_string())
            .collect();
    LocalProviderAdapterRegistryProjection {
        registry_status: LocalProviderAdapterValidationStatus::RegistryProjected,
        supported_adapter_kinds: vec![
            "deterministic_fake_adapter".to_string(),
            "local_model_adapter_contract".to_string(),
        ],
        rejected_adapter_kinds: vec![
            "unsupported_local_model".to_string(),
            "unsupported_cloud_model".to_string(),
            "unsupported_network_adapter".to_string(),
            "unsupported_shell_adapter".to_string(),
            "unsupported_filesystem_adapter".to_string(),
            "unknown".to_string(),
        ],
        declarations: registry.declarations.clone(),
        last_validation: registry.last_validation.clone(),
        capability_surface: local_provider_adapter_capability_surface(),
        execution_status: LocalProviderAdapterExecutionStatus::ExecutionNotAvailableInPhase153
            .code()
            .to_string(),
        trust_status: LocalProviderAdapterTrustStatus::NoProviderTrust
            .code()
            .to_string(),
        boundary_statuses,
        note: "Adapter contract only; no model execution is available in Phase 153. Accepted adapter declarations are non-executing. Adapter declaration does not grant provider trust. No network, shell, secret, or production persistence capability is enabled.".to_string(),
    }
}

pub fn validate_local_provider_adapter_declaration(
    candidate: &LocalProviderAdapterConfigurationCandidate,
) -> LocalProviderAdapterValidation {
    let mut errors = std::collections::BTreeSet::new();
    let parsed_kind = match candidate.adapter_kind.as_deref() {
        None => {
            errors.insert(LocalProviderAdapterValidationError::MissingAdapterKind);
            None
        }
        Some(kind) if kind.trim().is_empty() => {
            errors.insert(LocalProviderAdapterValidationError::MissingAdapterKind);
            None
        }
        Some(kind) if kind.trim() != kind => {
            errors.insert(LocalProviderAdapterValidationError::MalformedAdapterKind);
            None
        }
        Some(kind) => match LocalProviderAdapterKind::parse(kind) {
            Some(LocalProviderAdapterKind::DeterministicFakeAdapter) => {
                Some(LocalProviderAdapterKind::DeterministicFakeAdapter)
            }
            Some(LocalProviderAdapterKind::LocalModelAdapterContract) => {
                Some(LocalProviderAdapterKind::LocalModelAdapterContract)
            }
            Some(LocalProviderAdapterKind::UnsupportedCloudModel)
            | Some(LocalProviderAdapterKind::UnsupportedNetworkAdapter) => {
                errors.insert(LocalProviderAdapterValidationError::CloudOrNetworkAdapterRejected);
                LocalProviderAdapterKind::parse(kind)
            }
            Some(LocalProviderAdapterKind::UnsupportedShellAdapter) => {
                errors.insert(LocalProviderAdapterValidationError::ShellAdapterRejected);
                Some(LocalProviderAdapterKind::UnsupportedShellAdapter)
            }
            Some(LocalProviderAdapterKind::UnsupportedFilesystemAdapter) => {
                errors.insert(LocalProviderAdapterValidationError::FilesystemAdapterRejected);
                Some(LocalProviderAdapterKind::UnsupportedFilesystemAdapter)
            }
            Some(other) => {
                errors.insert(LocalProviderAdapterValidationError::UnsupportedAdapter);
                Some(other)
            }
            None => {
                errors.insert(LocalProviderAdapterValidationError::UnsupportedAdapter);
                None
            }
        },
    };

    for (key, value) in &candidate.fields {
        reject_forbidden_provider_adapter_declaration_field(key, value, &mut errors);
    }

    let error_codes: Vec<_> = errors.into_iter().collect();
    if error_codes.is_empty()
        && matches!(
            parsed_kind,
            Some(LocalProviderAdapterKind::DeterministicFakeAdapter)
                | Some(LocalProviderAdapterKind::LocalModelAdapterContract)
        )
    {
        return LocalProviderAdapterValidation {
            status: LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting,
            adapter_kind: parsed_kind,
            declaration_id: candidate.declaration_id.clone(),
            error_codes,
            reason: "adapter declaration accepted as contract-only local projection; no provider execution, trust, network, shell, secrets, or production persistence is enabled".to_string(),
        };
    }

    LocalProviderAdapterValidation {
        status: if error_codes.contains(&LocalProviderAdapterValidationError::UnsupportedAdapter) {
            LocalProviderAdapterValidationStatus::UnsupportedAdapter
        } else {
            LocalProviderAdapterValidationStatus::InvalidAdapterDeclaration
        },
        adapter_kind: parsed_kind,
        declaration_id: candidate.declaration_id.clone(),
        error_codes,
        reason: "adapter declaration rejected fail-closed; prior registry projection remains unchanged and no provider execution occurs".to_string(),
    }
}

pub fn reject_forbidden_provider_adapter_declaration_field(
    key: &str,
    value: &str,
    errors: &mut std::collections::BTreeSet<LocalProviderAdapterValidationError>,
) {
    let lowered_key = key.to_ascii_lowercase();
    let combined = format!("{}={}", lowered_key, value.to_ascii_lowercase());
    if lowered_key == "label" || lowered_key == "description" {
        return;
    }
    if ["endpoint", "url", "host", "port", "http", "network"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::EndpointFieldRejected);
    } else if ["command", "args", "shell", "process"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::CommandFieldRejected);
    } else if ["executable", "binary"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::ExecutablePathRejected);
    } else if ["path", "file", "directory"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::PathFieldRejected);
    } else if ["secret", "token", "api_key", "apikey", "credential"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::SecretFieldRejected);
    } else if [
        "provider_execution",
        "execution_requested",
        "execution_flag",
    ]
    .iter()
    .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::ExecutionFlagRejected);
    } else if ["trust_granted", "provider_trust", "trust_claimed"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::ProviderTrustFlagRejected);
    } else if ["readiness", "ready"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::ReadinessClaimRejected);
    } else if ["release"].iter().any(|needle| combined.contains(needle)) {
        errors.insert(LocalProviderAdapterValidationError::ReleaseClaimRejected);
    } else if ["deployment", "deploy"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::DeploymentClaimRejected);
    } else if ["public_use", "public-use"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::PublicUseClaimRejected);
    } else if ["signing"].iter().any(|needle| combined.contains(needle)) {
        errors.insert(LocalProviderAdapterValidationError::SigningClaimRejected);
    } else if ["publishing", "publish"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::PublishingClaimRejected);
    } else {
        errors.insert(LocalProviderAdapterValidationError::UnknownFieldRejected);
    }
}

pub fn apply_local_provider_adapter_declaration(
    state: &LocalOperatorShellState,
    candidate: LocalProviderAdapterConfigurationCandidate,
) -> Result<LocalOperatorShellState, LocalProviderAdapterValidation> {
    let validation = validate_local_provider_adapter_declaration(&candidate);
    if validation.status != LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting {
        return Err(validation);
    }
    let adapter_kind = validation
        .adapter_kind
        .expect("accepted adapter declaration includes adapter kind");
    let declaration_id = validation
        .declaration_id
        .clone()
        .unwrap_or_else(|| format!("local-adapter-declaration-{}", adapter_kind.code()));
    let declaration = LocalProviderAdapterDeclaration {
        adapter_kind,
        declaration_id,
        status: validation.status,
        contract: local_provider_adapter_contract(adapter_kind),
    };
    let mut next = state.clone();
    next.local_provider_adapter_registry = LocalProviderAdapterRegistry {
        declarations: vec![declaration],
        last_validation: validation,
    };
    Ok(attach_local_session_evidence_export(next))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterDryRunStatus {
    NotRun,
    DryRunExecuted,
    DryRunRejected,
    AdapterRequired,
    UnsupportedAdapter,
    InvalidDryRunRequest,
}

impl LocalProviderAdapterDryRunStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotRun => "not_run",
            Self::DryRunExecuted => "dry_run_executed",
            Self::DryRunRejected => "dry_run_rejected",
            Self::AdapterRequired => "adapter_required",
            Self::UnsupportedAdapter => "unsupported_adapter",
            Self::InvalidDryRunRequest => "invalid_dry_run_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalProviderAdapterDryRunError {
    NoAdapterDeclared,
    AdapterNotAccepted,
    UnsupportedAdapterKind,
    LocalModelAdapterNotExecutableInPhase154,
    CloudAdapterRejected,
    NetworkAdapterRejected,
    ShellAdapterRejected,
    FilesystemAdapterRejected,
    ExecutablePathRejected,
    EndpointFieldRejected,
    CommandFieldRejected,
    PathFieldRejected,
    SecretFieldRejected,
    ExecutionClaimRejected,
    TrustClaimRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    SigningClaimRejected,
    PublishingClaimRejected,
    ActionClaimRejected,
    PersistenceClaimRejected,
}

impl LocalProviderAdapterDryRunError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoAdapterDeclared => "no_adapter_declared",
            Self::AdapterNotAccepted => "adapter_not_accepted",
            Self::UnsupportedAdapterKind => "unsupported_adapter_kind",
            Self::LocalModelAdapterNotExecutableInPhase154 => {
                "local_model_adapter_not_executable_in_phase_154"
            }
            Self::CloudAdapterRejected => "cloud_adapter_rejected",
            Self::NetworkAdapterRejected => "network_adapter_rejected",
            Self::ShellAdapterRejected => "shell_adapter_rejected",
            Self::FilesystemAdapterRejected => "filesystem_adapter_rejected",
            Self::ExecutablePathRejected => "executable_path_rejected",
            Self::EndpointFieldRejected => "endpoint_field_rejected",
            Self::CommandFieldRejected => "command_field_rejected",
            Self::PathFieldRejected => "path_field_rejected",
            Self::SecretFieldRejected => "secret_field_rejected",
            Self::ExecutionClaimRejected => "execution_claim_rejected",
            Self::TrustClaimRejected => "trust_claim_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::SigningClaimRejected => "signing_claim_rejected",
            Self::PublishingClaimRejected => "publishing_claim_rejected",
            Self::ActionClaimRejected => "action_claim_rejected",
            Self::PersistenceClaimRejected => "persistence_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterDryRunBoundaryStatus {
    ControlledDryRunOnly,
    DeterministicFakeAdapterOnly,
    NoRealModelExecution,
    NoProcessSpawn,
    NoNetwork,
    NoShell,
    NoSecrets,
    NoProviderTrust,
    NoCandidateMaterialization,
    NoActionExecution,
    NoProductionPersistence,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl LocalProviderAdapterDryRunBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ControlledDryRunOnly => "controlled_dry_run_only",
            Self::DeterministicFakeAdapterOnly => "deterministic_fake_adapter_only",
            Self::NoRealModelExecution => "no_real_model_execution",
            Self::NoProcessSpawn => "no_process_spawn",
            Self::NoNetwork => "no_network",
            Self::NoShell => "no_shell",
            Self::NoSecrets => "no_secrets",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoCandidateMaterialization => "no_candidate_materialization",
            Self::NoActionExecution => "no_action_execution",
            Self::NoProductionPersistence => "no_production_persistence",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NoPublicUseEffect => "no_public_use_effect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterDryRunTrustStatus {
    UntrustedDescriptive,
}

impl LocalProviderAdapterDryRunTrustStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UntrustedDescriptive => "untrusted_descriptive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterDryRunEffectStatus {
    NoProviderTrust,
    NoCandidateMaterialization,
    NoActionExecution,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl LocalProviderAdapterDryRunEffectStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoCandidateMaterialization => "no_candidate_materialization",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NoPublicUseEffect => "no_public_use_effect",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterDryRunCapabilitySurface {
    pub controlled_dry_run_only: bool,
    pub deterministic_fake_adapter_only: bool,
    pub no_real_model_execution: bool,
    pub no_process_spawn: bool,
    pub no_network: bool,
    pub no_shell: bool,
    pub no_secrets: bool,
    pub no_provider_trust: bool,
    pub no_candidate_materialization: bool,
    pub no_action_execution: bool,
    pub no_production_persistence: bool,
    pub no_readiness_effect: bool,
    pub no_release_effect: bool,
    pub no_deployment_effect: bool,
    pub no_public_use_effect: bool,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterDryRunRequest {
    pub input_summary: String,
    pub fields: Vec<(String, String)>,
}

impl LocalProviderAdapterDryRunRequest {
    pub fn deterministic_default() -> Self {
        Self {
            input_summary: "phase 154 deterministic fake adapter dry-run input".to_string(),
            fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterDryRunResult {
    pub result_id: String,
    pub adapter_kind: LocalProviderAdapterKind,
    pub adapter_declaration_id: String,
    pub output_summary: String,
    pub output_trust_status: LocalProviderAdapterDryRunTrustStatus,
    pub boundary_statuses: Vec<LocalProviderAdapterDryRunBoundaryStatus>,
    pub effect_statuses: Vec<LocalProviderAdapterDryRunEffectStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterDryRunProjection {
    pub status: LocalProviderAdapterDryRunStatus,
    pub adapter_kind: Option<LocalProviderAdapterKind>,
    pub adapter_declaration_id: Option<String>,
    pub result: Option<LocalProviderAdapterDryRunResult>,
    pub error_codes: Vec<LocalProviderAdapterDryRunError>,
    pub boundary_statuses: Vec<LocalProviderAdapterDryRunBoundaryStatus>,
    pub output_trust_status: LocalProviderAdapterDryRunTrustStatus,
    pub effect_statuses: Vec<LocalProviderAdapterDryRunEffectStatus>,
    pub capability_surface: LocalProviderAdapterDryRunCapabilitySurface,
    pub registry_declaration_count: usize,
    pub reason: String,
}

pub fn initial_local_provider_adapter_dry_run_projection() -> LocalProviderAdapterDryRunProjection {
    LocalProviderAdapterDryRunProjection {
        status: LocalProviderAdapterDryRunStatus::NotRun,
        adapter_kind: None,
        adapter_declaration_id: None,
        result: None,
        error_codes: Vec::new(),
        boundary_statuses: local_provider_adapter_dry_run_boundary_statuses(),
        output_trust_status: LocalProviderAdapterDryRunTrustStatus::UntrustedDescriptive,
        effect_statuses: local_provider_adapter_dry_run_effect_statuses(),
        capability_surface: local_provider_adapter_dry_run_capability_surface(),
        registry_declaration_count: 0,
        reason: "adapter dry-run not_run; deterministic_fake_adapter declaration is required before Phase 154 dry run".to_string(),
    }
}

fn reject_local_provider_adapter_dry_run(
    status: LocalProviderAdapterDryRunStatus,
    adapter_kind: Option<LocalProviderAdapterKind>,
    adapter_declaration_id: Option<String>,
    registry_declaration_count: usize,
    errors: Vec<LocalProviderAdapterDryRunError>,
) -> LocalProviderAdapterDryRunProjection {
    LocalProviderAdapterDryRunProjection {
        status,
        adapter_kind,
        adapter_declaration_id,
        result: None,
        error_codes: errors,
        boundary_statuses: local_provider_adapter_dry_run_boundary_statuses(),
        output_trust_status: LocalProviderAdapterDryRunTrustStatus::UntrustedDescriptive,
        effect_statuses: local_provider_adapter_dry_run_effect_statuses(),
        capability_surface: local_provider_adapter_dry_run_capability_surface(),
        registry_declaration_count,
        reason: "adapter dry-run rejected fail-closed; prior accepted shell state is preserved and no provider trust, candidate, action, readiness, release, deployment, public-use, process, network, shell, secret, environment, or persistence effect occurs".to_string(),
    }
}

pub fn validate_local_provider_adapter_dry_run_request(
    registry: &LocalProviderAdapterRegistry,
    request: &LocalProviderAdapterDryRunRequest,
) -> Result<LocalProviderAdapterDeclaration, Box<LocalProviderAdapterDryRunProjection>> {
    let mut errors = std::collections::BTreeSet::new();
    let declaration = match registry.declarations.last() {
        Some(declaration) => declaration,
        None => {
            return Err(Box::new(reject_local_provider_adapter_dry_run(
                LocalProviderAdapterDryRunStatus::AdapterRequired,
                None,
                None,
                registry.declarations.len(),
                vec![LocalProviderAdapterDryRunError::NoAdapterDeclared],
            )));
        }
    };

    if declaration.status != LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting {
        errors.insert(LocalProviderAdapterDryRunError::AdapterNotAccepted);
    }
    match declaration.adapter_kind {
        LocalProviderAdapterKind::DeterministicFakeAdapter => {}
        LocalProviderAdapterKind::LocalModelAdapterContract => {
            errors
                .insert(LocalProviderAdapterDryRunError::LocalModelAdapterNotExecutableInPhase154);
        }
        LocalProviderAdapterKind::UnsupportedCloudModel => {
            errors.insert(LocalProviderAdapterDryRunError::CloudAdapterRejected);
        }
        LocalProviderAdapterKind::UnsupportedNetworkAdapter => {
            errors.insert(LocalProviderAdapterDryRunError::NetworkAdapterRejected);
        }
        LocalProviderAdapterKind::UnsupportedShellAdapter => {
            errors.insert(LocalProviderAdapterDryRunError::ShellAdapterRejected);
        }
        LocalProviderAdapterKind::UnsupportedFilesystemAdapter => {
            errors.insert(LocalProviderAdapterDryRunError::FilesystemAdapterRejected);
        }
        LocalProviderAdapterKind::UnsupportedLocalModel | LocalProviderAdapterKind::Unknown => {
            errors.insert(LocalProviderAdapterDryRunError::UnsupportedAdapterKind);
        }
    }

    for (key, value) in &request.fields {
        reject_forbidden_provider_adapter_dry_run_field(key, value, &mut errors);
    }
    if !request.input_summary.trim().is_empty() {
        reject_forbidden_provider_adapter_dry_run_field(
            "input_summary",
            &request.input_summary,
            &mut errors,
        );
    }

    let error_codes: Vec<_> = errors.into_iter().collect();
    if error_codes.is_empty() {
        Ok(declaration.clone())
    } else {
        Err(Box::new(reject_local_provider_adapter_dry_run(
            if error_codes.iter().any(|error| {
                matches!(
                    error,
                    LocalProviderAdapterDryRunError::LocalModelAdapterNotExecutableInPhase154
                        | LocalProviderAdapterDryRunError::CloudAdapterRejected
                        | LocalProviderAdapterDryRunError::NetworkAdapterRejected
                        | LocalProviderAdapterDryRunError::ShellAdapterRejected
                        | LocalProviderAdapterDryRunError::FilesystemAdapterRejected
                        | LocalProviderAdapterDryRunError::UnsupportedAdapterKind
                )
            }) {
                LocalProviderAdapterDryRunStatus::UnsupportedAdapter
            } else {
                LocalProviderAdapterDryRunStatus::InvalidDryRunRequest
            },
            Some(declaration.adapter_kind),
            Some(declaration.declaration_id.clone()),
            registry.declarations.len(),
            error_codes,
        )))
    }
}

pub fn reject_forbidden_provider_adapter_dry_run_field(
    key: &str,
    value: &str,
    errors: &mut std::collections::BTreeSet<LocalProviderAdapterDryRunError>,
) {
    let lowered_key = key.to_ascii_lowercase();
    let combined = format!("{}={}", lowered_key, value.to_ascii_lowercase());
    if lowered_key == "label"
        || lowered_key == "description"
        || lowered_key == "input_summary"
            && !combined.contains("endpoint")
            && !combined.contains("url")
            && !combined.contains("host")
            && !combined.contains("port")
            && !combined.contains("command")
            && !combined.contains("args")
            && !combined.contains("process")
            && !combined.contains("shell")
            && !combined.contains("path")
            && !combined.contains("file")
            && !combined.contains("directory")
            && !combined.contains("token")
            && !combined.contains("secret")
            && !combined.contains("api_key")
            && !combined.contains("apikey")
            && !combined.contains("credential")
            && !combined.contains("execution")
            && !combined.contains("trust")
            && !combined.contains("readiness")
            && !combined.contains("release")
            && !combined.contains("deployment")
            && !combined.contains("public-use")
            && !combined.contains("public_use")
            && !combined.contains("signing")
            && !combined.contains("publishing")
            && !combined.contains("action")
            && !combined.contains("persistence")
    {
        return;
    }
    if ["endpoint", "url", "host", "port", "http", "network"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::EndpointFieldRejected);
    } else if ["command", "args", "process", "shell"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::CommandFieldRejected);
    } else if ["executable", "binary"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::ExecutablePathRejected);
    } else if ["path", "file", "directory"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::PathFieldRejected);
    } else if ["secret", "token", "api_key", "apikey", "credential"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::SecretFieldRejected);
    } else if ["execution"].iter().any(|needle| combined.contains(needle)) {
        errors.insert(LocalProviderAdapterDryRunError::ExecutionClaimRejected);
    } else if ["trust"].iter().any(|needle| combined.contains(needle)) {
        errors.insert(LocalProviderAdapterDryRunError::TrustClaimRejected);
    } else if ["readiness", "ready"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::ReadinessClaimRejected);
    } else if combined.contains("release") {
        errors.insert(LocalProviderAdapterDryRunError::ReleaseClaimRejected);
    } else if ["deployment", "deploy"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::DeploymentClaimRejected);
    } else if ["public_use", "public-use"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::PublicUseClaimRejected);
    } else if combined.contains("signing") {
        errors.insert(LocalProviderAdapterDryRunError::SigningClaimRejected);
    } else if ["publishing", "publish"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::PublishingClaimRejected);
    } else if combined.contains("action") {
        errors.insert(LocalProviderAdapterDryRunError::ActionClaimRejected);
    } else if combined.contains("persistence") {
        errors.insert(LocalProviderAdapterDryRunError::PersistenceClaimRejected);
    }
}

fn deterministic_adapter_dry_run_checksum(input: &str) -> u64 {
    input.bytes().fold(154_u64, |acc, byte| {
        acc.wrapping_mul(31).wrapping_add(byte as u64)
    })
}

pub fn execute_deterministic_fake_adapter_dry_run(
    declaration: &LocalProviderAdapterDeclaration,
    request: &LocalProviderAdapterDryRunRequest,
) -> LocalProviderAdapterDryRunResult {
    let checksum = deterministic_adapter_dry_run_checksum(&format!(
        "{}|{}|{}",
        declaration.declaration_id,
        declaration.adapter_kind.code(),
        request.input_summary
    ));
    LocalProviderAdapterDryRunResult {
        result_id: format!("local-provider-adapter-dry-run-{checksum:016x}"),
        adapter_kind: declaration.adapter_kind,
        adapter_declaration_id: declaration.declaration_id.clone(),
        output_summary: format!(
            "deterministic_fake_adapter dry-run descriptive output for input_bytes={} checksum={checksum:016x}",
            request.input_summary.len()
        ),
        output_trust_status: LocalProviderAdapterDryRunTrustStatus::UntrustedDescriptive,
        boundary_statuses: local_provider_adapter_dry_run_boundary_statuses(),
        effect_statuses: local_provider_adapter_dry_run_effect_statuses(),
    }
}

pub fn project_local_provider_adapter_dry_run(
    registry: &LocalProviderAdapterRegistry,
    result: LocalProviderAdapterDryRunResult,
) -> LocalProviderAdapterDryRunProjection {
    LocalProviderAdapterDryRunProjection {
        status: LocalProviderAdapterDryRunStatus::DryRunExecuted,
        adapter_kind: Some(result.adapter_kind),
        adapter_declaration_id: Some(result.adapter_declaration_id.clone()),
        result: Some(result),
        error_codes: Vec::new(),
        boundary_statuses: local_provider_adapter_dry_run_boundary_statuses(),
        output_trust_status: LocalProviderAdapterDryRunTrustStatus::UntrustedDescriptive,
        effect_statuses: local_provider_adapter_dry_run_effect_statuses(),
        capability_surface: local_provider_adapter_dry_run_capability_surface(),
        registry_declaration_count: registry.declarations.len(),
        reason: "controlled adapter dry run executed through deterministic_fake_adapter only; output remains untrusted_descriptive and no provider trust, candidate, action, readiness, release, deployment, public-use, process, network, shell, secret, environment, or persistence effect occurs".to_string(),
    }
}

pub fn apply_local_provider_adapter_dry_run(
    state: &LocalOperatorShellState,
    request: LocalProviderAdapterDryRunRequest,
) -> Result<LocalOperatorShellState, Box<LocalProviderAdapterDryRunProjection>> {
    let declaration = validate_local_provider_adapter_dry_run_request(
        &state.local_provider_adapter_registry,
        &request,
    )?;
    let result = execute_deterministic_fake_adapter_dry_run(&declaration, &request);
    let mut next = state.clone();
    next.local_provider_adapter_dry_run =
        project_local_provider_adapter_dry_run(&state.local_provider_adapter_registry, result);
    Ok(attach_local_session_evidence_export(next))
}
