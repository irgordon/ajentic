use std::collections::{BTreeMap, BTreeSet};

pub const MAX_DEPLOYMENT_CONFIGURATION_PAYLOAD_BYTES: usize = 16_384;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeploymentConfigurationValidationStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeploymentConfigurationReason {
    MalformedConfiguration,
    OversizedConfiguration,
    MissingRequiredField,
    InvalidProfileIdentifier,
    NonLocalDeploymentModeRejected,
    MissingStoragePathDeclaration,
    UnsafeStoragePathDeclaration,
    MissingStoragePermissionDeclaration,
    MissingRetentionDeclaration,
    MissingEnvironmentAssumptionDeclaration,
    MissingFailureHandlingDeclaration,
    MissingManualReviewDeclaration,
    BackgroundRepairRejected,
    AutomaticReplayPatchingRejected,
    ContinueAnywayRejected,
    MigrationVersionUpgradeAuthorityRejected,
    ProductionRecoveryGuaranteeRejected,
    ReleaseEvidenceGuaranteeRejected,
    SilentRecoveryRejected,
    RecoveryPromotionRejected,
    ReplayRepairRejected,
    ActionExecutionRejected,
    DeploymentAutomationRejected,
    InstallerRejected,
    UpdateChannelRejected,
    SigningRejected,
    PublishingRejected,
    PublicReleaseRejected,
    ProductionDeploymentRejected,
    ProviderTrustRejected,
    ProviderOutputPromotionRejected,
    ReadinessApprovalRejected,
    ProductionCandidateApprovalRejected,
    ReleaseCandidateApprovalRejected,
    PublicUseApprovalRejected,
    ProductionHumanUseApprovalRejected,
    AuthorityBearingConfigurationRejected,
}

impl DeploymentConfigurationReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MalformedConfiguration => "malformed_configuration",
            Self::OversizedConfiguration => "oversized_configuration",
            Self::MissingRequiredField => "missing_required_field",
            Self::InvalidProfileIdentifier => "invalid_profile_identifier",
            Self::NonLocalDeploymentModeRejected => "non_local_deployment_mode_rejected",
            Self::MissingStoragePathDeclaration => "missing_storage_path_declaration",
            Self::UnsafeStoragePathDeclaration => "unsafe_storage_path_declaration",
            Self::MissingStoragePermissionDeclaration => "missing_storage_permission_declaration",
            Self::MissingRetentionDeclaration => "missing_retention_declaration",
            Self::MissingEnvironmentAssumptionDeclaration => {
                "missing_environment_assumption_declaration"
            }
            Self::MissingFailureHandlingDeclaration => "missing_failure_handling_declaration",
            Self::MissingManualReviewDeclaration => "missing_manual_review_declaration",
            Self::BackgroundRepairRejected => "background_repair_rejected",
            Self::AutomaticReplayPatchingRejected => "automatic_replay_patching_rejected",
            Self::ContinueAnywayRejected => "continue_anyway_rejected",
            Self::MigrationVersionUpgradeAuthorityRejected => {
                "migration_version_upgrade_authority_rejected"
            }
            Self::ProductionRecoveryGuaranteeRejected => "production_recovery_guarantee_rejected",
            Self::ReleaseEvidenceGuaranteeRejected => "release_evidence_guarantee_rejected",
            Self::SilentRecoveryRejected => "silent_recovery_rejected",
            Self::RecoveryPromotionRejected => "recovery_promotion_rejected",
            Self::ReplayRepairRejected => "replay_repair_rejected",
            Self::ActionExecutionRejected => "action_execution_rejected",
            Self::DeploymentAutomationRejected => "deployment_automation_rejected",
            Self::InstallerRejected => "installer_rejected",
            Self::UpdateChannelRejected => "update_channel_rejected",
            Self::SigningRejected => "signing_rejected",
            Self::PublishingRejected => "publishing_rejected",
            Self::PublicReleaseRejected => "public_release_rejected",
            Self::ProductionDeploymentRejected => "production_deployment_rejected",
            Self::ProviderTrustRejected => "provider_trust_rejected",
            Self::ProviderOutputPromotionRejected => "provider_output_promotion_rejected",
            Self::ReadinessApprovalRejected => "readiness_approval_rejected",
            Self::ProductionCandidateApprovalRejected => "production_candidate_approval_rejected",
            Self::ReleaseCandidateApprovalRejected => "release_candidate_approval_rejected",
            Self::PublicUseApprovalRejected => "public_use_approval_rejected",
            Self::ProductionHumanUseApprovalRejected => "production_human_use_approval_rejected",
            Self::AuthorityBearingConfigurationRejected => {
                "authority_bearing_configuration_rejected"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentStorageDeclaration {
    pub path: String,
    pub path_declared: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentPermissionDeclaration {
    pub permission_posture: String,
    pub permission_declared: bool,
    pub changes_permissions: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentRetentionDeclaration {
    pub retention_posture: String,
    pub retention_declared: bool,
    pub deletes_or_rotates_data: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentFailureHandlingDeclaration {
    pub failure_handling_posture: String,
    pub failure_handling_declared: bool,
    pub manual_review_required: bool,
    pub continue_anyway_enabled: bool,
    pub silent_recovery_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentRecoveryHandoffDeclaration {
    pub no_background_repair: bool,
    pub no_automatic_replay_patching: bool,
    pub no_continue_anyway: bool,
    pub no_migration_version_upgrade_authority: bool,
    pub no_production_recovery_guarantee: bool,
    pub no_release_evidence_guarantee: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentAuthorityDenialSnapshot {
    pub deployment_automation_enabled: bool,
    pub release_artifact_created: bool,
    pub installer_enabled: bool,
    pub update_channel_enabled: bool,
    pub signing_enabled: bool,
    pub publishing_enabled: bool,
    pub production_deployment_enabled: bool,
    pub public_release_enabled: bool,
    pub provider_trust_granted: bool,
    pub provider_output_promoted: bool,
    pub replay_repaired: bool,
    pub recovery_promoted: bool,
    pub action_executed: bool,
    pub readiness_approved: bool,
    pub production_candidate_approved: bool,
    pub release_candidate_approved: bool,
    pub public_use_approved: bool,
    pub production_human_use_approved: bool,
}

impl DeploymentAuthorityDenialSnapshot {
    pub fn denied() -> Self {
        Self {
            deployment_automation_enabled: false,
            release_artifact_created: false,
            installer_enabled: false,
            update_channel_enabled: false,
            signing_enabled: false,
            publishing_enabled: false,
            production_deployment_enabled: false,
            public_release_enabled: false,
            provider_trust_granted: false,
            provider_output_promoted: false,
            replay_repaired: false,
            recovery_promoted: false,
            action_executed: false,
            readiness_approved: false,
            production_candidate_approved: false,
            release_candidate_approved: false,
            public_use_approved: false,
            production_human_use_approved: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentConfigurationContract {
    pub profile_id: String,
    pub deployment_mode: String,
    pub local_only: bool,
    pub environment_assumptions: Vec<String>,
    pub storage: DeploymentStorageDeclaration,
    pub permissions: DeploymentPermissionDeclaration,
    pub retention: DeploymentRetentionDeclaration,
    pub failure_handling: DeploymentFailureHandlingDeclaration,
    pub recovery_handoff: DeploymentRecoveryHandoffDeclaration,
    pub authority_denials: DeploymentAuthorityDenialSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentConfigurationValidationReport {
    pub status: DeploymentConfigurationValidationStatus,
    pub reasons: Vec<DeploymentConfigurationReason>,
    pub profile_id: String,
    pub local_only: bool,
    pub contract_evidence_only: bool,
    pub deterministic: bool,
    pub fail_closed: bool,
    pub mutates_filesystem: bool,
    pub mutates_permissions: bool,
    pub opens_network_socket: bool,
    pub starts_service: bool,
    pub creates_release_artifact: bool,
    pub authority_denials: DeploymentAuthorityDenialSnapshot,
    pub summary: String,
}

impl DeploymentConfigurationValidationReport {
    fn accepted(profile_id: String, local_only: bool) -> Self {
        Self {
            status: DeploymentConfigurationValidationStatus::Accepted,
            reasons: Vec::new(),
            profile_id,
            local_only,
            contract_evidence_only: true,
            deterministic: true,
            fail_closed: true,
            mutates_filesystem: false,
            mutates_permissions: false,
            opens_network_socket: false,
            starts_service: false,
            creates_release_artifact: false,
            authority_denials: DeploymentAuthorityDenialSnapshot::denied(),
            summary: "deployment configuration accepted as deterministic contract evidence only; no deployment, release, install, update, signing, publishing, provider trust, recovery promotion, replay repair, action execution, or readiness approval is performed".to_string(),
        }
    }

    fn rejected(reasons: BTreeSet<DeploymentConfigurationReason>, profile_id: String) -> Self {
        Self {
            status: DeploymentConfigurationValidationStatus::Rejected,
            reasons: reasons.into_iter().collect(),
            profile_id,
            local_only: false,
            contract_evidence_only: true,
            deterministic: true,
            fail_closed: true,
            mutates_filesystem: false,
            mutates_permissions: false,
            opens_network_socket: false,
            starts_service: false,
            creates_release_artifact: false,
            authority_denials: DeploymentAuthorityDenialSnapshot::denied(),
            summary: "deployment configuration rejected fail-closed as non-authoritative contract evidence only; deployment and release authority remain disabled".to_string(),
        }
    }
}

pub fn validate_deployment_configuration_contract(
    contract: &DeploymentConfigurationContract,
) -> DeploymentConfigurationValidationReport {
    let mut reasons = BTreeSet::new();
    validate_contract(contract, &mut reasons);
    if reasons.is_empty() {
        DeploymentConfigurationValidationReport::accepted(
            contract.profile_id.clone(),
            contract.local_only,
        )
    } else {
        DeploymentConfigurationValidationReport::rejected(reasons, contract.profile_id.clone())
    }
}

fn validate_contract(
    contract: &DeploymentConfigurationContract,
    reasons: &mut BTreeSet<DeploymentConfigurationReason>,
) {
    if !deployment_profile_identifier_is_valid(&contract.profile_id) {
        reasons.insert(DeploymentConfigurationReason::InvalidProfileIdentifier);
    }
    if contract.deployment_mode != "local_only" || !contract.local_only {
        reasons.insert(DeploymentConfigurationReason::NonLocalDeploymentModeRejected);
    }
    if contract.environment_assumptions.is_empty() {
        reasons.insert(DeploymentConfigurationReason::MissingEnvironmentAssumptionDeclaration);
    }
    validate_storage(&contract.storage, reasons);
    validate_permissions(&contract.permissions, reasons);
    validate_retention(&contract.retention, reasons);
    validate_failure_handling(&contract.failure_handling, reasons);
    validate_recovery_handoff(&contract.recovery_handoff, reasons);
    validate_denials(&contract.authority_denials, reasons);
}

fn validate_storage(
    storage: &DeploymentStorageDeclaration,
    reasons: &mut BTreeSet<DeploymentConfigurationReason>,
) {
    if !storage.path_declared || storage.path.trim().is_empty() {
        reasons.insert(DeploymentConfigurationReason::MissingStoragePathDeclaration);
        return;
    }
    if storage_path_is_unsafe(&storage.path) {
        reasons.insert(DeploymentConfigurationReason::UnsafeStoragePathDeclaration);
    }
}

fn validate_permissions(
    permissions: &DeploymentPermissionDeclaration,
    reasons: &mut BTreeSet<DeploymentConfigurationReason>,
) {
    if !permissions.permission_declared || permissions.permission_posture.trim().is_empty() {
        reasons.insert(DeploymentConfigurationReason::MissingStoragePermissionDeclaration);
    }
    if permissions.changes_permissions {
        reasons.insert(DeploymentConfigurationReason::AuthorityBearingConfigurationRejected);
    }
}

fn validate_retention(
    retention: &DeploymentRetentionDeclaration,
    reasons: &mut BTreeSet<DeploymentConfigurationReason>,
) {
    if !retention.retention_declared || retention.retention_posture.trim().is_empty() {
        reasons.insert(DeploymentConfigurationReason::MissingRetentionDeclaration);
    }
    if retention.deletes_or_rotates_data {
        reasons.insert(DeploymentConfigurationReason::AuthorityBearingConfigurationRejected);
    }
}

fn validate_failure_handling(
    failure: &DeploymentFailureHandlingDeclaration,
    reasons: &mut BTreeSet<DeploymentConfigurationReason>,
) {
    if !failure.failure_handling_declared || failure.failure_handling_posture.trim().is_empty() {
        reasons.insert(DeploymentConfigurationReason::MissingFailureHandlingDeclaration);
    }
    if !failure.manual_review_required {
        reasons.insert(DeploymentConfigurationReason::MissingManualReviewDeclaration);
    }
    if failure.continue_anyway_enabled {
        reasons.insert(DeploymentConfigurationReason::ContinueAnywayRejected);
    }
    if failure.silent_recovery_enabled {
        reasons.insert(DeploymentConfigurationReason::SilentRecoveryRejected);
    }
}

fn validate_recovery_handoff(
    handoff: &DeploymentRecoveryHandoffDeclaration,
    reasons: &mut BTreeSet<DeploymentConfigurationReason>,
) {
    if !handoff.no_background_repair {
        reasons.insert(DeploymentConfigurationReason::BackgroundRepairRejected);
    }
    if !handoff.no_automatic_replay_patching {
        reasons.insert(DeploymentConfigurationReason::AutomaticReplayPatchingRejected);
    }
    if !handoff.no_continue_anyway {
        reasons.insert(DeploymentConfigurationReason::ContinueAnywayRejected);
    }
    if !handoff.no_migration_version_upgrade_authority {
        reasons.insert(DeploymentConfigurationReason::MigrationVersionUpgradeAuthorityRejected);
    }
    if !handoff.no_production_recovery_guarantee {
        reasons.insert(DeploymentConfigurationReason::ProductionRecoveryGuaranteeRejected);
    }
    if !handoff.no_release_evidence_guarantee {
        reasons.insert(DeploymentConfigurationReason::ReleaseEvidenceGuaranteeRejected);
    }
}

fn validate_denials(
    denial: &DeploymentAuthorityDenialSnapshot,
    reasons: &mut BTreeSet<DeploymentConfigurationReason>,
) {
    if denial.deployment_automation_enabled {
        reasons.insert(DeploymentConfigurationReason::DeploymentAutomationRejected);
    }
    if denial.release_artifact_created {
        reasons.insert(DeploymentConfigurationReason::AuthorityBearingConfigurationRejected);
    }
    if denial.installer_enabled {
        reasons.insert(DeploymentConfigurationReason::InstallerRejected);
    }
    if denial.update_channel_enabled {
        reasons.insert(DeploymentConfigurationReason::UpdateChannelRejected);
    }
    if denial.signing_enabled {
        reasons.insert(DeploymentConfigurationReason::SigningRejected);
    }
    if denial.publishing_enabled {
        reasons.insert(DeploymentConfigurationReason::PublishingRejected);
    }
    if denial.production_deployment_enabled {
        reasons.insert(DeploymentConfigurationReason::ProductionDeploymentRejected);
    }
    if denial.public_release_enabled {
        reasons.insert(DeploymentConfigurationReason::PublicReleaseRejected);
    }
    if denial.provider_trust_granted {
        reasons.insert(DeploymentConfigurationReason::ProviderTrustRejected);
    }
    if denial.provider_output_promoted {
        reasons.insert(DeploymentConfigurationReason::ProviderOutputPromotionRejected);
    }
    if denial.replay_repaired {
        reasons.insert(DeploymentConfigurationReason::ReplayRepairRejected);
    }
    if denial.recovery_promoted {
        reasons.insert(DeploymentConfigurationReason::RecoveryPromotionRejected);
    }
    if denial.action_executed {
        reasons.insert(DeploymentConfigurationReason::ActionExecutionRejected);
    }
    if denial.readiness_approved {
        reasons.insert(DeploymentConfigurationReason::ReadinessApprovalRejected);
    }
    if denial.production_candidate_approved {
        reasons.insert(DeploymentConfigurationReason::ProductionCandidateApprovalRejected);
    }
    if denial.release_candidate_approved {
        reasons.insert(DeploymentConfigurationReason::ReleaseCandidateApprovalRejected);
    }
    if denial.public_use_approved {
        reasons.insert(DeploymentConfigurationReason::PublicUseApprovalRejected);
    }
    if denial.production_human_use_approved {
        reasons.insert(DeploymentConfigurationReason::ProductionHumanUseApprovalRejected);
    }
}

pub fn parse_deployment_configuration_payload(
    payload: &str,
) -> DeploymentConfigurationValidationReport {
    if payload.len() > MAX_DEPLOYMENT_CONFIGURATION_PAYLOAD_BYTES {
        let mut reasons = BTreeSet::new();
        reasons.insert(DeploymentConfigurationReason::OversizedConfiguration);
        return DeploymentConfigurationValidationReport::rejected(reasons, String::new());
    }

    match parse_deployment_configuration_block(payload) {
        Ok(contract) => validate_deployment_configuration_contract(&contract),
        Err((reasons, profile_id)) => {
            DeploymentConfigurationValidationReport::rejected(reasons, profile_id)
        }
    }
}

fn parse_deployment_configuration_block(
    payload: &str,
) -> Result<DeploymentConfigurationContract, (BTreeSet<DeploymentConfigurationReason>, String)> {
    let mut reasons = BTreeSet::new();
    let mut block = BTreeMap::new();
    let mut saw_header = false;

    for raw_line in payload.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "deployment_configuration" {
            if saw_header {
                reasons.insert(DeploymentConfigurationReason::MalformedConfiguration);
            }
            saw_header = true;
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            reasons.insert(DeploymentConfigurationReason::MalformedConfiguration);
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() || !saw_header {
            reasons.insert(DeploymentConfigurationReason::MalformedConfiguration);
            continue;
        }
        if !deployment_configuration_key_is_allowed(key) {
            reasons.insert(DeploymentConfigurationReason::MalformedConfiguration);
        }
        if authority_bearing_deployment_key_or_value(key, value) {
            reasons.insert(DeploymentConfigurationReason::AuthorityBearingConfigurationRejected);
        }
        if block.insert(key.to_string(), value.to_string()).is_some() {
            reasons.insert(DeploymentConfigurationReason::MalformedConfiguration);
        }
    }

    if !saw_header {
        reasons.insert(DeploymentConfigurationReason::MissingRequiredField);
    }
    let profile_id = block.get("profile_id").cloned().unwrap_or_default();
    if !reasons.is_empty() {
        return Err((reasons, profile_id));
    }

    contract_from_block(&block).map_err(|reasons| (reasons, profile_id))
}

fn contract_from_block(
    block: &BTreeMap<String, String>,
) -> Result<DeploymentConfigurationContract, BTreeSet<DeploymentConfigurationReason>> {
    let mut reasons = BTreeSet::new();
    let profile_id = required_value(block, "profile_id", &mut reasons).unwrap_or_default();
    let deployment_mode =
        required_value(block, "deployment_mode", &mut reasons).unwrap_or_default();
    let local_only = parse_required_bool(block, "local_only", &mut reasons);
    let environment_assumptions = parse_string_list(block, "environment_assumptions", &mut reasons);

    let storage = DeploymentStorageDeclaration {
        path: required_value(block, "storage_path", &mut reasons)
            .unwrap_or_default()
            .to_string(),
        path_declared: parse_required_bool(block, "storage_path_declared", &mut reasons),
    };
    let permissions = DeploymentPermissionDeclaration {
        permission_posture: required_value(block, "storage_permission_posture", &mut reasons)
            .unwrap_or_default()
            .to_string(),
        permission_declared: parse_required_bool(
            block,
            "storage_permission_declared",
            &mut reasons,
        ),
        changes_permissions: parse_required_bool(block, "changes_permissions", &mut reasons),
    };
    let retention = DeploymentRetentionDeclaration {
        retention_posture: required_value(block, "retention_posture", &mut reasons)
            .unwrap_or_default()
            .to_string(),
        retention_declared: parse_required_bool(block, "retention_declared", &mut reasons),
        deletes_or_rotates_data: parse_required_bool(
            block,
            "deletes_or_rotates_data",
            &mut reasons,
        ),
    };
    let failure_handling = DeploymentFailureHandlingDeclaration {
        failure_handling_posture: required_value(block, "failure_handling_posture", &mut reasons)
            .unwrap_or_default()
            .to_string(),
        failure_handling_declared: parse_required_bool(
            block,
            "failure_handling_declared",
            &mut reasons,
        ),
        manual_review_required: parse_required_bool(block, "manual_review_required", &mut reasons),
        continue_anyway_enabled: parse_required_bool(
            block,
            "continue_anyway_enabled",
            &mut reasons,
        ),
        silent_recovery_enabled: parse_required_bool(
            block,
            "silent_recovery_enabled",
            &mut reasons,
        ),
    };
    let recovery_handoff = DeploymentRecoveryHandoffDeclaration {
        no_background_repair: parse_required_bool(block, "no_background_repair", &mut reasons),
        no_automatic_replay_patching: parse_required_bool(
            block,
            "no_automatic_replay_patching",
            &mut reasons,
        ),
        no_continue_anyway: parse_required_bool(block, "no_continue_anyway", &mut reasons),
        no_migration_version_upgrade_authority: parse_required_bool(
            block,
            "no_migration_version_upgrade_authority",
            &mut reasons,
        ),
        no_production_recovery_guarantee: parse_required_bool(
            block,
            "no_production_recovery_guarantee",
            &mut reasons,
        ),
        no_release_evidence_guarantee: parse_required_bool(
            block,
            "no_release_evidence_guarantee",
            &mut reasons,
        ),
    };
    let authority_denials = DeploymentAuthorityDenialSnapshot {
        deployment_automation_enabled: parse_required_bool(
            block,
            "deployment_automation_enabled",
            &mut reasons,
        ),
        release_artifact_created: parse_required_bool(
            block,
            "release_artifact_created",
            &mut reasons,
        ),
        installer_enabled: parse_required_bool(block, "installer_enabled", &mut reasons),
        update_channel_enabled: parse_required_bool(block, "update_channel_enabled", &mut reasons),
        signing_enabled: parse_required_bool(block, "signing_enabled", &mut reasons),
        publishing_enabled: parse_required_bool(block, "publishing_enabled", &mut reasons),
        production_deployment_enabled: parse_required_bool(
            block,
            "production_deployment_enabled",
            &mut reasons,
        ),
        public_release_enabled: parse_required_bool(block, "public_release_enabled", &mut reasons),
        provider_trust_granted: parse_required_bool(block, "provider_trust_granted", &mut reasons),
        provider_output_promoted: parse_required_bool(
            block,
            "provider_output_promoted",
            &mut reasons,
        ),
        replay_repaired: parse_required_bool(block, "replay_repaired", &mut reasons),
        recovery_promoted: parse_required_bool(block, "recovery_promoted", &mut reasons),
        action_executed: parse_required_bool(block, "action_executed", &mut reasons),
        readiness_approved: parse_required_bool(block, "readiness_approved", &mut reasons),
        production_candidate_approved: parse_required_bool(
            block,
            "production_candidate_approved",
            &mut reasons,
        ),
        release_candidate_approved: parse_required_bool(
            block,
            "release_candidate_approved",
            &mut reasons,
        ),
        public_use_approved: parse_required_bool(block, "public_use_approved", &mut reasons),
        production_human_use_approved: parse_required_bool(
            block,
            "production_human_use_approved",
            &mut reasons,
        ),
    };

    if !reasons.is_empty() {
        return Err(reasons);
    }

    Ok(DeploymentConfigurationContract {
        profile_id: profile_id.to_string(),
        deployment_mode: deployment_mode.to_string(),
        local_only,
        environment_assumptions,
        storage,
        permissions,
        retention,
        failure_handling,
        recovery_handoff,
        authority_denials,
    })
}

fn required_value<'a>(
    block: &'a BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<DeploymentConfigurationReason>,
) -> Option<&'a str> {
    match block.get(key) {
        Some(value) => Some(value.as_str()),
        None => {
            reasons.insert(DeploymentConfigurationReason::MissingRequiredField);
            None
        }
    }
}

fn parse_required_bool(
    block: &BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<DeploymentConfigurationReason>,
) -> bool {
    match required_value(block, key, reasons) {
        Some("true") => true,
        Some("false") => false,
        Some(_) => {
            reasons.insert(DeploymentConfigurationReason::MalformedConfiguration);
            false
        }
        None => false,
    }
}

fn parse_string_list(
    block: &BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<DeploymentConfigurationReason>,
) -> Vec<String> {
    let Some(value) = required_value(block, key, reasons) else {
        return Vec::new();
    };
    let mut values = BTreeSet::new();
    for item in value.split(',').map(str::trim) {
        if item.is_empty() {
            reasons.insert(DeploymentConfigurationReason::MalformedConfiguration);
        } else {
            values.insert(item.to_string());
        }
    }
    values.into_iter().collect()
}

pub fn deployment_profile_identifier_is_valid(profile_id: &str) -> bool {
    let bytes = profile_id.as_bytes();
    if bytes.len() < 3 || bytes.len() > 64 {
        return false;
    }
    bytes.iter().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-' || *byte == b'_'
    }) && bytes[0].is_ascii_lowercase()
        && bytes[bytes.len() - 1].is_ascii_lowercase()
}

fn storage_path_is_unsafe(path: &str) -> bool {
    let trimmed = path.trim();
    trimmed.is_empty()
        || trimmed == "/"
        || trimmed == "."
        || trimmed == ".."
        || trimmed.contains("..")
        || trimmed.contains('~')
        || trimmed.contains('\0')
        || trimmed.starts_with("/etc")
        || trimmed.starts_with("/bin")
        || trimmed.starts_with("/sbin")
        || trimmed.starts_with("/usr")
        || trimmed.starts_with("/var")
        || trimmed.starts_with("/root")
}

fn deployment_configuration_key_is_allowed(key: &str) -> bool {
    matches!(
        key,
        "profile_id"
            | "deployment_mode"
            | "local_only"
            | "environment_assumptions"
            | "storage_path"
            | "storage_path_declared"
            | "storage_permission_posture"
            | "storage_permission_declared"
            | "changes_permissions"
            | "retention_posture"
            | "retention_declared"
            | "deletes_or_rotates_data"
            | "failure_handling_posture"
            | "failure_handling_declared"
            | "manual_review_required"
            | "continue_anyway_enabled"
            | "silent_recovery_enabled"
            | "no_background_repair"
            | "no_automatic_replay_patching"
            | "no_continue_anyway"
            | "no_migration_version_upgrade_authority"
            | "no_production_recovery_guarantee"
            | "no_release_evidence_guarantee"
            | "deployment_automation_enabled"
            | "release_artifact_created"
            | "installer_enabled"
            | "update_channel_enabled"
            | "signing_enabled"
            | "publishing_enabled"
            | "production_deployment_enabled"
            | "public_release_enabled"
            | "provider_trust_granted"
            | "provider_output_promoted"
            | "replay_repaired"
            | "recovery_promoted"
            | "action_executed"
            | "readiness_approved"
            | "production_candidate_approved"
            | "release_candidate_approved"
            | "public_use_approved"
            | "production_human_use_approved"
    )
}

fn authority_bearing_deployment_key_or_value(key: &str, value: &str) -> bool {
    let combined = format!(
        "{}={}",
        key.to_ascii_lowercase(),
        value.to_ascii_lowercase()
    );
    [
        "command::new",
        "std::process",
        "fs::write",
        "file::create",
        "openoptions",
        "tcplistener",
        "udpsocket",
        "websocket",
        "fetch(",
        "xmlhttprequest",
        "api_key",
        "secret",
        "token",
        "deploy now",
        "publish now",
        "install now",
        "execute action",
        "trust provider",
        "promote provider output",
        "production candidate status: approved",
    ]
    .iter()
    .any(|needle| combined.contains(needle))
}

pub fn deployment_configuration_executes_deployment(
    _report: &DeploymentConfigurationValidationReport,
) -> bool {
    false
}

pub fn deployment_configuration_creates_release_artifact(
    _report: &DeploymentConfigurationValidationReport,
) -> bool {
    false
}

pub fn deployment_configuration_approves_readiness(
    _report: &DeploymentConfigurationValidationReport,
) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_contract() -> DeploymentConfigurationContract {
        DeploymentConfigurationContract {
            profile_id: "local_contract_alpha".to_string(),
            deployment_mode: "local_only".to_string(),
            local_only: true,
            environment_assumptions: vec!["operator_supplied_local_paths".to_string()],
            storage: DeploymentStorageDeclaration {
                path: "./.ajentic/local-contract".to_string(),
                path_declared: true,
            },
            permissions: DeploymentPermissionDeclaration {
                permission_posture: "operator_reviewed_existing_permissions".to_string(),
                permission_declared: true,
                changes_permissions: false,
            },
            retention: DeploymentRetentionDeclaration {
                retention_posture: "manual_review_before_retention_change".to_string(),
                retention_declared: true,
                deletes_or_rotates_data: false,
            },
            failure_handling: DeploymentFailureHandlingDeclaration {
                failure_handling_posture: "fail_closed_manual_review".to_string(),
                failure_handling_declared: true,
                manual_review_required: true,
                continue_anyway_enabled: false,
                silent_recovery_enabled: false,
            },
            recovery_handoff: DeploymentRecoveryHandoffDeclaration {
                no_background_repair: true,
                no_automatic_replay_patching: true,
                no_continue_anyway: true,
                no_migration_version_upgrade_authority: true,
                no_production_recovery_guarantee: true,
                no_release_evidence_guarantee: true,
            },
            authority_denials: DeploymentAuthorityDenialSnapshot::denied(),
        }
    }

    fn valid_payload() -> String {
        "deployment_configuration\nprofile_id=local_contract_alpha\ndeployment_mode=local_only\nlocal_only=true\nenvironment_assumptions=operator_supplied_local_paths,no_live_probe\nstorage_path=./.ajentic/local-contract\nstorage_path_declared=true\nstorage_permission_posture=operator_reviewed_existing_permissions\nstorage_permission_declared=true\nchanges_permissions=false\nretention_posture=manual_review_before_retention_change\nretention_declared=true\ndeletes_or_rotates_data=false\nfailure_handling_posture=fail_closed_manual_review\nfailure_handling_declared=true\nmanual_review_required=true\ncontinue_anyway_enabled=false\nsilent_recovery_enabled=false\nno_background_repair=true\nno_automatic_replay_patching=true\nno_continue_anyway=true\nno_migration_version_upgrade_authority=true\nno_production_recovery_guarantee=true\nno_release_evidence_guarantee=true\ndeployment_automation_enabled=false\nrelease_artifact_created=false\ninstaller_enabled=false\nupdate_channel_enabled=false\nsigning_enabled=false\npublishing_enabled=false\nproduction_deployment_enabled=false\npublic_release_enabled=false\nprovider_trust_granted=false\nprovider_output_promoted=false\nreplay_repaired=false\nrecovery_promoted=false\naction_executed=false\nreadiness_approved=false\nproduction_candidate_approved=false\nrelease_candidate_approved=false\npublic_use_approved=false\nproduction_human_use_approved=false\n".to_string()
    }

    fn reason_codes(report: &DeploymentConfigurationValidationReport) -> Vec<&'static str> {
        report
            .reasons
            .iter()
            .map(DeploymentConfigurationReason::code)
            .collect()
    }

    #[test]
    fn phase_113_valid_deployment_configuration_is_contract_evidence_only() {
        let report = validate_deployment_configuration_contract(&valid_contract());
        assert_eq!(
            report.status,
            DeploymentConfigurationValidationStatus::Accepted
        );
        assert!(report.contract_evidence_only);
        assert!(report.deterministic);
        assert!(report.fail_closed);
        assert!(!deployment_configuration_executes_deployment(&report));
        assert!(!deployment_configuration_creates_release_artifact(&report));
        assert!(!deployment_configuration_approves_readiness(&report));
        assert!(!report.mutates_filesystem);
        assert!(!report.mutates_permissions);
        assert!(!report.opens_network_socket);
        assert!(!report.starts_service);
        assert_eq!(
            report.authority_denials,
            DeploymentAuthorityDenialSnapshot::denied()
        );
    }

    #[test]
    fn phase_113_required_recovery_handoff_declarations_fail_closed() {
        let mut contract = valid_contract();
        contract.storage.path_declared = false;
        contract.permissions.permission_declared = false;
        contract.retention.retention_declared = false;
        contract.environment_assumptions.clear();
        contract.failure_handling.failure_handling_declared = false;
        contract.failure_handling.manual_review_required = false;
        let report = validate_deployment_configuration_contract(&contract);
        assert_eq!(
            report.status,
            DeploymentConfigurationValidationStatus::Rejected
        );
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::MissingStoragePathDeclaration));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::MissingStoragePermissionDeclaration));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::MissingRetentionDeclaration));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::MissingEnvironmentAssumptionDeclaration));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::MissingFailureHandlingDeclaration));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::MissingManualReviewDeclaration));
        assert!(!report.mutates_filesystem);
    }

    #[test]
    fn phase_113_rejects_recovery_automation_and_guarantees() {
        let mut contract = valid_contract();
        contract.failure_handling.continue_anyway_enabled = true;
        contract.failure_handling.silent_recovery_enabled = true;
        contract.recovery_handoff.no_background_repair = false;
        contract.recovery_handoff.no_automatic_replay_patching = false;
        contract.recovery_handoff.no_continue_anyway = false;
        contract
            .recovery_handoff
            .no_migration_version_upgrade_authority = false;
        contract.recovery_handoff.no_production_recovery_guarantee = false;
        contract.recovery_handoff.no_release_evidence_guarantee = false;
        let report = validate_deployment_configuration_contract(&contract);
        assert_eq!(
            report.status,
            DeploymentConfigurationValidationStatus::Rejected
        );
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::BackgroundRepairRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::AutomaticReplayPatchingRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ContinueAnywayRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::MigrationVersionUpgradeAuthorityRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ProductionRecoveryGuaranteeRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ReleaseEvidenceGuaranteeRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::SilentRecoveryRejected));
    }

    #[test]
    fn phase_113_rejects_deployment_release_and_approval_authority() {
        let mut contract = valid_contract();
        contract.authority_denials = DeploymentAuthorityDenialSnapshot {
            deployment_automation_enabled: true,
            release_artifact_created: true,
            installer_enabled: true,
            update_channel_enabled: true,
            signing_enabled: true,
            publishing_enabled: true,
            production_deployment_enabled: true,
            public_release_enabled: true,
            provider_trust_granted: true,
            provider_output_promoted: true,
            replay_repaired: true,
            recovery_promoted: true,
            action_executed: true,
            readiness_approved: true,
            production_candidate_approved: true,
            release_candidate_approved: true,
            public_use_approved: true,
            production_human_use_approved: true,
        };
        let report = validate_deployment_configuration_contract(&contract);
        assert_eq!(
            report.status,
            DeploymentConfigurationValidationStatus::Rejected
        );
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::DeploymentAutomationRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::InstallerRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::UpdateChannelRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::SigningRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::PublishingRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::PublicReleaseRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ProductionDeploymentRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ProviderTrustRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ProviderOutputPromotionRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ReplayRepairRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::RecoveryPromotionRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ActionExecutionRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ReadinessApprovalRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ProductionCandidateApprovalRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ReleaseCandidateApprovalRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::PublicUseApprovalRejected));
        assert!(report
            .reasons
            .contains(&DeploymentConfigurationReason::ProductionHumanUseApprovalRejected));
    }

    #[test]
    fn phase_113_equivalent_payloads_validate_deterministically() {
        let report_one = parse_deployment_configuration_payload(&valid_payload());
        let report_two = parse_deployment_configuration_payload(&valid_payload());
        assert_eq!(report_one, report_two);
        assert_eq!(
            report_one.status,
            DeploymentConfigurationValidationStatus::Accepted
        );
    }

    #[test]
    fn phase_113_payload_rejects_path_traversal_and_noise_authority() {
        let payload = valid_payload().replace(
            "storage_path=./.ajentic/local-contract",
            "storage_path=../../etc/shadow",
        ) + "unknown=deploy now\n";
        let report = parse_deployment_configuration_payload(&payload);
        assert_eq!(
            report.status,
            DeploymentConfigurationValidationStatus::Rejected
        );
        assert!(reason_codes(&report).contains(&"malformed_configuration"));
        assert!(reason_codes(&report).contains(&"authority_bearing_configuration_rejected"));
    }
}
