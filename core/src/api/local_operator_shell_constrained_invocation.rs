use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllowlistedLocalProviderKind {
    AllowlistedLocalDeterministicProvider,
    UnsupportedLocalProvider,
    UnsupportedCloudProvider,
    UnsupportedNetworkProvider,
    UnsupportedShellProvider,
}

impl AllowlistedLocalProviderKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AllowlistedLocalDeterministicProvider => {
                "allowlisted_local_deterministic_provider"
            }
            Self::UnsupportedLocalProvider => "unsupported_local_provider",
            Self::UnsupportedCloudProvider => "unsupported_cloud_provider",
            Self::UnsupportedNetworkProvider => "unsupported_network_provider",
            Self::UnsupportedShellProvider => "unsupported_shell_provider",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstrainedLocalProviderInvocationStatus {
    NotInvoked,
    InvocationExecuted,
    InvocationRejected,
    AllowlistedProviderRequired,
    UnsupportedProvider,
    InvalidInvocationRequest,
}

impl ConstrainedLocalProviderInvocationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotInvoked => "not_invoked",
            Self::InvocationExecuted => "invocation_executed",
            Self::InvocationRejected => "invocation_rejected",
            Self::AllowlistedProviderRequired => "allowlisted_provider_required",
            Self::UnsupportedProvider => "unsupported_provider",
            Self::InvalidInvocationRequest => "invalid_invocation_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstrainedLocalProviderInvocationError {
    NoAdapterDeclared,
    AdapterNotAccepted,
    ProviderNotAllowlisted,
    ArbitraryCommandRejected,
    ShellFieldRejected,
    ProcessFieldRejected,
    ArgsFieldRejected,
    EndpointFieldRejected,
    NetworkFieldRejected,
    SecretFieldRejected,
    PathFieldRejected,
    TrustClaimRejected,
    ProviderOutputApprovalClaimRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    CandidateMaterializationClaimRejected,
    ActionClaimRejected,
    PersistenceClaimRejected,
}

impl ConstrainedLocalProviderInvocationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoAdapterDeclared => "no_adapter_declared",
            Self::AdapterNotAccepted => "adapter_not_accepted",
            Self::ProviderNotAllowlisted => "provider_not_allowlisted",
            Self::ArbitraryCommandRejected => "arbitrary_command_rejected",
            Self::ShellFieldRejected => "shell_field_rejected",
            Self::ProcessFieldRejected => "process_field_rejected",
            Self::ArgsFieldRejected => "args_field_rejected",
            Self::EndpointFieldRejected => "endpoint_field_rejected",
            Self::NetworkFieldRejected => "network_field_rejected",
            Self::SecretFieldRejected => "secret_field_rejected",
            Self::PathFieldRejected => "path_field_rejected",
            Self::TrustClaimRejected => "trust_claim_rejected",
            Self::ProviderOutputApprovalClaimRejected => "provider_output_approval_claim_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::CandidateMaterializationClaimRejected => {
                "candidate_materialization_claim_rejected"
            }
            Self::ActionClaimRejected => "action_claim_rejected",
            Self::PersistenceClaimRejected => "persistence_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstrainedLocalProviderInvocationBoundaryStatus {
    ConstrainedLocalInvocationOnly,
    AllowlistedProviderOnly,
    NoArbitraryCommand,
    NoShell,
    NoNetwork,
    NoCloud,
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

impl ConstrainedLocalProviderInvocationBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ConstrainedLocalInvocationOnly => "constrained_local_invocation_only",
            Self::AllowlistedProviderOnly => "allowlisted_provider_only",
            Self::NoArbitraryCommand => "no_arbitrary_command",
            Self::NoShell => "no_shell",
            Self::NoNetwork => "no_network",
            Self::NoCloud => "no_cloud",
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
pub enum ConstrainedLocalProviderInvocationTrustStatus {
    UntrustedDescriptive,
}

impl ConstrainedLocalProviderInvocationTrustStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UntrustedDescriptive => "untrusted_descriptive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstrainedLocalProviderInvocationEffectStatus {
    NoProviderTrust,
    NoCandidateMaterialization,
    NoActionExecution,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl ConstrainedLocalProviderInvocationEffectStatus {
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
pub struct ConstrainedLocalProviderInvocationCapabilitySurface {
    pub constrained_local_invocation_only: bool,
    pub allowlisted_provider_only: bool,
    pub allowlisted_provider_kind: AllowlistedLocalProviderKind,
    pub no_arbitrary_command: bool,
    pub no_shell: bool,
    pub no_network: bool,
    pub no_cloud: bool,
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
pub struct ConstrainedLocalProviderInvocationRequest {
    pub provider_kind: AllowlistedLocalProviderKind,
    pub input_summary: String,
    pub fields: Vec<(String, String)>,
}

impl ConstrainedLocalProviderInvocationRequest {
    pub fn allowlisted_default() -> Self {
        Self {
            provider_kind: AllowlistedLocalProviderKind::AllowlistedLocalDeterministicProvider,
            input_summary: "phase 156 constrained local provider invocation input".to_string(),
            fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstrainedLocalProviderInvocationResult {
    pub result_id: String,
    pub provider_kind: AllowlistedLocalProviderKind,
    pub adapter_kind: LocalProviderAdapterKind,
    pub adapter_declaration_id: String,
    pub output_summary: String,
    pub output_trust_status: ConstrainedLocalProviderInvocationTrustStatus,
    pub boundary_statuses: Vec<ConstrainedLocalProviderInvocationBoundaryStatus>,
    pub effect_statuses: Vec<ConstrainedLocalProviderInvocationEffectStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstrainedLocalProviderInvocationProjection {
    pub status: ConstrainedLocalProviderInvocationStatus,
    pub provider_kind: Option<AllowlistedLocalProviderKind>,
    pub adapter_kind: Option<LocalProviderAdapterKind>,
    pub adapter_declaration_id: Option<String>,
    pub result: Option<ConstrainedLocalProviderInvocationResult>,
    pub error_codes: Vec<ConstrainedLocalProviderInvocationError>,
    pub boundary_statuses: Vec<ConstrainedLocalProviderInvocationBoundaryStatus>,
    pub output_trust_status: ConstrainedLocalProviderInvocationTrustStatus,
    pub effect_statuses: Vec<ConstrainedLocalProviderInvocationEffectStatus>,
    pub capability_surface: ConstrainedLocalProviderInvocationCapabilitySurface,
    pub registry_declaration_count: usize,
    pub reason: String,
}

pub fn initial_constrained_local_provider_invocation_projection(
) -> ConstrainedLocalProviderInvocationProjection {
    ConstrainedLocalProviderInvocationProjection {
        status: ConstrainedLocalProviderInvocationStatus::NotInvoked,
        provider_kind: None,
        adapter_kind: None,
        adapter_declaration_id: None,
        result: None,
        error_codes: Vec::new(),
        boundary_statuses: constrained_local_provider_invocation_boundary_statuses(),
        output_trust_status: ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive,
        effect_statuses: constrained_local_provider_invocation_effect_statuses(),
        capability_surface: constrained_local_provider_invocation_capability_surface(),
        registry_declaration_count: 0,
        reason: "constrained local provider invocation not_invoked; accepted deterministic_fake_adapter declaration and allowlisted_local_deterministic_provider are required for Phase 156 invocation".to_string(),
    }
}

pub fn reject_constrained_local_provider_invocation(
    status: ConstrainedLocalProviderInvocationStatus,
    provider_kind: Option<AllowlistedLocalProviderKind>,
    adapter_kind: Option<LocalProviderAdapterKind>,
    adapter_declaration_id: Option<String>,
    registry_declaration_count: usize,
    errors: Vec<ConstrainedLocalProviderInvocationError>,
) -> ConstrainedLocalProviderInvocationProjection {
    ConstrainedLocalProviderInvocationProjection {
        status,
        provider_kind,
        adapter_kind,
        adapter_declaration_id,
        result: None,
        error_codes: errors,
        boundary_statuses: constrained_local_provider_invocation_boundary_statuses(),
        output_trust_status: ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive,
        effect_statuses: constrained_local_provider_invocation_effect_statuses(),
        capability_surface: constrained_local_provider_invocation_capability_surface(),
        registry_declaration_count,
        reason: "constrained local provider invocation rejected fail-closed; prior accepted invocation projection is preserved when present and no provider trust, candidate, action, readiness, release, deployment, public-use, command, shell, network, cloud, secret, environment, or persistence effect occurs".to_string(),
    }
}

pub fn validate_constrained_local_provider_invocation_request(
    registry: &LocalProviderAdapterRegistry,
    request: &ConstrainedLocalProviderInvocationRequest,
) -> Result<LocalProviderAdapterDeclaration, Box<ConstrainedLocalProviderInvocationProjection>> {
    let mut errors = std::collections::BTreeSet::new();
    match request.provider_kind {
        AllowlistedLocalProviderKind::AllowlistedLocalDeterministicProvider => {}
        AllowlistedLocalProviderKind::UnsupportedCloudProvider => {
            errors.insert(ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted);
            errors.insert(ConstrainedLocalProviderInvocationError::NetworkFieldRejected);
        }
        AllowlistedLocalProviderKind::UnsupportedNetworkProvider => {
            errors.insert(ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted);
            errors.insert(ConstrainedLocalProviderInvocationError::NetworkFieldRejected);
        }
        AllowlistedLocalProviderKind::UnsupportedShellProvider => {
            errors.insert(ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted);
            errors.insert(ConstrainedLocalProviderInvocationError::ShellFieldRejected);
        }
        AllowlistedLocalProviderKind::UnsupportedLocalProvider => {
            errors.insert(ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted);
        }
    }

    let declaration = match registry.declarations.last() {
        Some(declaration) => declaration,
        None => {
            errors.insert(ConstrainedLocalProviderInvocationError::NoAdapterDeclared);
            let error_codes: Vec<_> = errors.into_iter().collect();
            return Err(Box::new(reject_constrained_local_provider_invocation(
                ConstrainedLocalProviderInvocationStatus::AllowlistedProviderRequired,
                Some(request.provider_kind),
                None,
                None,
                registry.declarations.len(),
                error_codes,
            )));
        }
    };

    if declaration.status != LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting {
        errors.insert(ConstrainedLocalProviderInvocationError::AdapterNotAccepted);
    }
    if declaration.adapter_kind != LocalProviderAdapterKind::DeterministicFakeAdapter {
        errors.insert(ConstrainedLocalProviderInvocationError::AdapterNotAccepted);
    }

    for (key, value) in &request.fields {
        reject_forbidden_constrained_local_provider_invocation_field(key, value, &mut errors);
    }
    if !request.input_summary.trim().is_empty() {
        reject_forbidden_constrained_local_provider_invocation_field(
            "input_summary",
            &request.input_summary,
            &mut errors,
        );
    }

    let error_codes: Vec<_> = errors.into_iter().collect();
    if error_codes.is_empty() {
        Ok(declaration.clone())
    } else {
        let status = if error_codes.iter().any(|error| {
            matches!(
                error,
                ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted
                    | ConstrainedLocalProviderInvocationError::NetworkFieldRejected
                    | ConstrainedLocalProviderInvocationError::ShellFieldRejected
                    | ConstrainedLocalProviderInvocationError::AdapterNotAccepted
            )
        }) {
            ConstrainedLocalProviderInvocationStatus::UnsupportedProvider
        } else {
            ConstrainedLocalProviderInvocationStatus::InvalidInvocationRequest
        };
        Err(Box::new(reject_constrained_local_provider_invocation(
            status,
            Some(request.provider_kind),
            Some(declaration.adapter_kind),
            Some(declaration.declaration_id.clone()),
            registry.declarations.len(),
            error_codes,
        )))
    }
}

pub fn reject_forbidden_constrained_local_provider_invocation_field(
    key: &str,
    value: &str,
    errors: &mut std::collections::BTreeSet<ConstrainedLocalProviderInvocationError>,
) {
    let lowered_key = key.to_ascii_lowercase();
    let combined = format!("{}={}", lowered_key, value.to_ascii_lowercase());
    let safe_input =
        lowered_key == "label" || lowered_key == "description" || lowered_key == "input_summary";
    let forbidden = [
        "endpoint",
        "url",
        "host",
        "port",
        "http",
        "network",
        "cloud",
        "command",
        "shell",
        "process",
        "args",
        "path",
        "file",
        "directory",
        "secret",
        "token",
        "api_key",
        "apikey",
        "credential",
        "trust",
        "approved_output",
        "provider_output_approval",
        "readiness",
        "ready",
        "release",
        "deployment",
        "deploy",
        "public_use",
        "public-use",
        "candidate",
        "materialization",
        "action",
        "persistence",
    ]
    .iter()
    .any(|needle| combined.contains(needle));
    if safe_input && !forbidden {
        return;
    }
    if combined.contains("command") {
        errors.insert(ConstrainedLocalProviderInvocationError::ArbitraryCommandRejected);
    } else if combined.contains("shell") {
        errors.insert(ConstrainedLocalProviderInvocationError::ShellFieldRejected);
    } else if combined.contains("process") {
        errors.insert(ConstrainedLocalProviderInvocationError::ProcessFieldRejected);
    } else if combined.contains("args") {
        errors.insert(ConstrainedLocalProviderInvocationError::ArgsFieldRejected);
    } else if ["endpoint", "url", "host", "port", "http"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::EndpointFieldRejected);
    } else if ["network", "cloud"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::NetworkFieldRejected);
    } else if ["secret", "token", "api_key", "apikey", "credential"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::SecretFieldRejected);
    } else if ["path", "file", "directory"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::PathFieldRejected);
    } else if combined.contains("approved_output") || combined.contains("provider_output_approval")
    {
        errors.insert(ConstrainedLocalProviderInvocationError::ProviderOutputApprovalClaimRejected);
    } else if combined.contains("trust") {
        errors.insert(ConstrainedLocalProviderInvocationError::TrustClaimRejected);
    } else if ["readiness", "ready"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::ReadinessClaimRejected);
    } else if combined.contains("release") {
        errors.insert(ConstrainedLocalProviderInvocationError::ReleaseClaimRejected);
    } else if ["deployment", "deploy"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::DeploymentClaimRejected);
    } else if ["public_use", "public-use"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::PublicUseClaimRejected);
    } else if ["candidate", "materialization"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors
            .insert(ConstrainedLocalProviderInvocationError::CandidateMaterializationClaimRejected);
    } else if combined.contains("action") {
        errors.insert(ConstrainedLocalProviderInvocationError::ActionClaimRejected);
    } else if combined.contains("persistence") {
        errors.insert(ConstrainedLocalProviderInvocationError::PersistenceClaimRejected);
    }
}

fn deterministic_constrained_local_provider_invocation_checksum(input: &str) -> u64 {
    input.bytes().fold(156_u64, |acc, byte| {
        acc.wrapping_mul(33).wrapping_add(byte as u64)
    })
}

pub fn execute_allowlisted_local_deterministic_provider_invocation(
    declaration: &LocalProviderAdapterDeclaration,
    request: &ConstrainedLocalProviderInvocationRequest,
) -> ConstrainedLocalProviderInvocationResult {
    let checksum = deterministic_constrained_local_provider_invocation_checksum(&format!(
        "{}|{}|{}|{}",
        declaration.declaration_id,
        declaration.adapter_kind.code(),
        request.provider_kind.code(),
        request.input_summary
    ));
    ConstrainedLocalProviderInvocationResult {
        result_id: format!("constrained-local-provider-invocation-{checksum:016x}"),
        provider_kind: request.provider_kind,
        adapter_kind: declaration.adapter_kind,
        adapter_declaration_id: declaration.declaration_id.clone(),
        output_summary: format!(
            "allowlisted_local_deterministic_provider descriptive output for input_bytes={} checksum={checksum:016x}",
            request.input_summary.len()
        ),
        output_trust_status: ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive,
        boundary_statuses: constrained_local_provider_invocation_boundary_statuses(),
        effect_statuses: constrained_local_provider_invocation_effect_statuses(),
    }
}

pub fn project_constrained_local_provider_invocation(
    registry: &LocalProviderAdapterRegistry,
    result: ConstrainedLocalProviderInvocationResult,
) -> ConstrainedLocalProviderInvocationProjection {
    ConstrainedLocalProviderInvocationProjection {
        status: ConstrainedLocalProviderInvocationStatus::InvocationExecuted,
        provider_kind: Some(result.provider_kind),
        adapter_kind: Some(result.adapter_kind),
        adapter_declaration_id: Some(result.adapter_declaration_id.clone()),
        result: Some(result),
        error_codes: Vec::new(),
        boundary_statuses: constrained_local_provider_invocation_boundary_statuses(),
        output_trust_status: ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive,
        effect_statuses: constrained_local_provider_invocation_effect_statuses(),
        capability_surface: constrained_local_provider_invocation_capability_surface(),
        registry_declaration_count: registry.declarations.len(),
        reason: "constrained local provider invocation executed through exactly one allowlisted local provider path; output remains untrusted_descriptive and no provider trust, candidate, action, readiness, release, deployment, public-use, command, shell, network, cloud, secret, environment, or persistence effect occurs".to_string(),
    }
}

pub fn execute_constrained_local_provider_invocation(
    state: &LocalOperatorShellState,
    request: ConstrainedLocalProviderInvocationRequest,
) -> Result<LocalOperatorShellState, Box<ConstrainedLocalProviderInvocationProjection>> {
    let declaration = validate_constrained_local_provider_invocation_request(
        &state.local_provider_adapter_registry,
        &request,
    )?;
    let result =
        execute_allowlisted_local_deterministic_provider_invocation(&declaration, &request);
    let mut next = state.clone();
    next.constrained_local_provider_invocation = project_constrained_local_provider_invocation(
        &state.local_provider_adapter_registry,
        result,
    );
    let bridge = project_invocation_output_into_provider_pipeline(&next).map_err(|errors| {
        Box::new(reject_constrained_local_provider_invocation(
            ConstrainedLocalProviderInvocationStatus::InvocationRejected,
            next.constrained_local_provider_invocation.provider_kind,
            next.constrained_local_provider_invocation.adapter_kind,
            next.constrained_local_provider_invocation
                .adapter_declaration_id
                .clone(),
            next.local_provider_adapter_registry.declarations.len(),
            errors
                .into_iter()
                .map(|_| {
                    ConstrainedLocalProviderInvocationError::ProviderOutputApprovalClaimRejected
                })
                .collect(),
        ))
    })?;
    next.provider_execution = provider_execution_projection_from_invocation_bridge(&next, &bridge);
    next.provider_output_validation = validate_local_provider_output(&next.provider_execution);
    next.local_provider_output_pipeline = derive_local_provider_output_pipeline_projection(&next);
    Ok(attach_local_session_evidence_export(next))
}
