use std::collections::{BTreeMap, BTreeSet};

pub const MAX_LOCAL_DEPLOYMENT_CANDIDATE_PAYLOAD_BYTES: usize = 16_384;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalDeploymentCandidateValidationStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalDeploymentCandidateReason {
    LocalOnlyContractEvidenceAccepted,
    MalformedCandidate,
    OversizedCandidate,
    MissingRequiredField,
    MissingCandidateIdentifier,
    MissingLocalOnlyMode,
    MissingPhase113DeploymentConfigurationEvidenceReference,
    MissingPhase114PolicyGovernanceEvidenceReference,
    MissingPhase115SecurityAuditEvidenceReference,
    MissingResidualRiskAcknowledgement,
    MissingStorageConfigurationReference,
    MissingRecoveryHandoffReference,
    MissingManualReviewPosture,
    MissingUnsupportedPublicProductionReleaseDeclaration,
    NonLocalCandidateModeRejected,
    PublicAvailabilityClaimRejected,
    ProductionReadinessClaimRejected,
    DeploymentAutomationRejected,
    ReleaseArtifactCreationRejected,
    InstallerRejected,
    UpdateChannelRejected,
    SigningRejected,
    PublishingRejected,
    GithubReleaseRejected,
    ReleaseTagRejected,
    ProductionDeploymentRejected,
    PublicReleaseRejected,
    PublicUseApprovalRejected,
    ProductionHumanUseApprovalRejected,
    ProductionCandidateApprovalRejected,
    ReleaseCandidateApprovalRejected,
    ReadinessApprovalRejected,
    ProviderTrustRejected,
    ProviderOutputPromotionRejected,
    ReplayRepairRejected,
    RecoveryPromotionRejected,
    ActionExecutionRejected,
    FilesystemMutationRejected,
    PermissionMutationRejected,
    NetworkSocketRejected,
    ServiceStartRejected,
    ProcessLaunchRejected,
    AuthorityBearingCandidateRejected,
}

impl LocalDeploymentCandidateReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalOnlyContractEvidenceAccepted => "local_only_contract_evidence_accepted",
            Self::MalformedCandidate => "malformed_candidate",
            Self::OversizedCandidate => "oversized_candidate",
            Self::MissingRequiredField => "missing_required_field",
            Self::MissingCandidateIdentifier => "missing_candidate_identifier",
            Self::MissingLocalOnlyMode => "missing_local_only_mode",
            Self::MissingPhase113DeploymentConfigurationEvidenceReference => {
                "missing_phase_113_deployment_configuration_evidence_reference"
            }
            Self::MissingPhase114PolicyGovernanceEvidenceReference => {
                "missing_phase_114_policy_governance_evidence_reference"
            }
            Self::MissingPhase115SecurityAuditEvidenceReference => {
                "missing_phase_115_security_audit_evidence_reference"
            }
            Self::MissingResidualRiskAcknowledgement => "missing_residual_risk_acknowledgement",
            Self::MissingStorageConfigurationReference => "missing_storage_configuration_reference",
            Self::MissingRecoveryHandoffReference => "missing_recovery_handoff_reference",
            Self::MissingManualReviewPosture => "missing_manual_review_posture",
            Self::MissingUnsupportedPublicProductionReleaseDeclaration => {
                "missing_unsupported_public_production_release_declaration"
            }
            Self::NonLocalCandidateModeRejected => "non_local_candidate_mode_rejected",
            Self::PublicAvailabilityClaimRejected => "public_availability_claim_rejected",
            Self::ProductionReadinessClaimRejected => "production_readiness_claim_rejected",
            Self::DeploymentAutomationRejected => "deployment_automation_rejected",
            Self::ReleaseArtifactCreationRejected => "release_artifact_creation_rejected",
            Self::InstallerRejected => "installer_rejected",
            Self::UpdateChannelRejected => "update_channel_rejected",
            Self::SigningRejected => "signing_rejected",
            Self::PublishingRejected => "publishing_rejected",
            Self::GithubReleaseRejected => "github_release_rejected",
            Self::ReleaseTagRejected => "release_tag_rejected",
            Self::ProductionDeploymentRejected => "production_deployment_rejected",
            Self::PublicReleaseRejected => "public_release_rejected",
            Self::PublicUseApprovalRejected => "public_use_approval_rejected",
            Self::ProductionHumanUseApprovalRejected => "production_human_use_approval_rejected",
            Self::ProductionCandidateApprovalRejected => "production_candidate_approval_rejected",
            Self::ReleaseCandidateApprovalRejected => "release_candidate_approval_rejected",
            Self::ReadinessApprovalRejected => "readiness_approval_rejected",
            Self::ProviderTrustRejected => "provider_trust_rejected",
            Self::ProviderOutputPromotionRejected => "provider_output_promotion_rejected",
            Self::ReplayRepairRejected => "replay_repair_rejected",
            Self::RecoveryPromotionRejected => "recovery_promotion_rejected",
            Self::ActionExecutionRejected => "action_execution_rejected",
            Self::FilesystemMutationRejected => "filesystem_mutation_rejected",
            Self::PermissionMutationRejected => "permission_mutation_rejected",
            Self::NetworkSocketRejected => "network_socket_rejected",
            Self::ServiceStartRejected => "service_start_rejected",
            Self::ProcessLaunchRejected => "process_launch_rejected",
            Self::AuthorityBearingCandidateRejected => "authority_bearing_candidate_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDeploymentCandidateEvidenceReferences {
    pub phase_113_deployment_configuration_evidence: String,
    pub phase_114_policy_governance_evidence: String,
    pub phase_115_security_audit_evidence: String,
    pub storage_configuration_reference: String,
    pub recovery_handoff_reference: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDeploymentCandidateResidualRiskAcknowledgement {
    pub acknowledged: bool,
    pub acknowledgement_reference: String,
    pub residual_risks_reviewed: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDeploymentCandidateAuthorityDenialSnapshot {
    pub deployment_automation_enabled: bool,
    pub release_artifact_created: bool,
    pub installer_enabled: bool,
    pub update_channel_enabled: bool,
    pub signing_enabled: bool,
    pub publishing_enabled: bool,
    pub github_release_created: bool,
    pub release_tag_created: bool,
    pub production_deployment_enabled: bool,
    pub public_release_enabled: bool,
    pub public_use_approved: bool,
    pub production_human_use_approved: bool,
    pub provider_trust_granted: bool,
    pub provider_output_promoted: bool,
    pub replay_repaired: bool,
    pub recovery_promoted: bool,
    pub action_executed: bool,
    pub readiness_approved: bool,
    pub production_candidate_approved: bool,
    pub release_candidate_approved: bool,
}

impl LocalDeploymentCandidateAuthorityDenialSnapshot {
    pub fn denied() -> Self {
        Self {
            deployment_automation_enabled: false,
            release_artifact_created: false,
            installer_enabled: false,
            update_channel_enabled: false,
            signing_enabled: false,
            publishing_enabled: false,
            github_release_created: false,
            release_tag_created: false,
            production_deployment_enabled: false,
            public_release_enabled: false,
            public_use_approved: false,
            production_human_use_approved: false,
            provider_trust_granted: false,
            provider_output_promoted: false,
            replay_repaired: false,
            recovery_promoted: false,
            action_executed: false,
            readiness_approved: false,
            production_candidate_approved: false,
            release_candidate_approved: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDeploymentCandidateBoundary {
    pub candidate_identifier: String,
    pub candidate_mode: String,
    pub local_only: bool,
    pub evidence_references: LocalDeploymentCandidateEvidenceReferences,
    pub residual_risk_acknowledgement: LocalDeploymentCandidateResidualRiskAcknowledgement,
    pub manual_review_required: bool,
    pub supported_local_validation_commands: Vec<String>,
    pub unsupported_public_production_release_declarations: Vec<String>,
    pub authority_denials: LocalDeploymentCandidateAuthorityDenialSnapshot,
    pub mutates_filesystem: bool,
    pub mutates_permissions: bool,
    pub opens_network_socket: bool,
    pub starts_service: bool,
    pub launches_process: bool,
    pub public_availability_claimed: bool,
    pub production_readiness_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDeploymentCandidateValidationReport {
    pub status: LocalDeploymentCandidateValidationStatus,
    pub reasons: Vec<LocalDeploymentCandidateReason>,
    pub reason_codes: Vec<&'static str>,
    pub candidate_identifier: String,
    pub local_only: bool,
    pub candidate_evidence_only: bool,
    pub non_authoritative: bool,
    pub manual_review_required: bool,
    pub deterministic: bool,
    pub fail_closed: bool,
    pub mutates_filesystem: bool,
    pub mutates_permissions: bool,
    pub opens_network_socket: bool,
    pub starts_service: bool,
    pub launches_process: bool,
    pub deployment_automation_enabled: bool,
    pub release_artifact_created: bool,
    pub installer_enabled: bool,
    pub update_channel_enabled: bool,
    pub signing_enabled: bool,
    pub publishing_enabled: bool,
    pub github_release_created: bool,
    pub release_tag_created: bool,
    pub production_deployment_enabled: bool,
    pub public_release_enabled: bool,
    pub public_use_approved: bool,
    pub production_human_use_approved: bool,
    pub provider_trust_granted: bool,
    pub provider_output_promoted: bool,
    pub replay_repaired: bool,
    pub recovery_promoted: bool,
    pub action_executed: bool,
    pub readiness_approved: bool,
    pub production_candidate_approved: bool,
    pub release_candidate_approved: bool,
    pub authority_denial_snapshot: LocalDeploymentCandidateAuthorityDenialSnapshot,
    pub summary: String,
}

impl LocalDeploymentCandidateValidationReport {
    fn accepted(candidate_identifier: String) -> Self {
        let reasons = vec![LocalDeploymentCandidateReason::LocalOnlyContractEvidenceAccepted];
        Self::from_parts(
            LocalDeploymentCandidateValidationStatus::Accepted,
            reasons,
            candidate_identifier,
            true,
            true,
            "local deployment candidate accepted as deterministic local-only contract evidence; no deployment, release, installation, update channel, signing, publishing, production deployment, public use, provider trust, replay repair, recovery promotion, action execution, or readiness approval is performed".to_string(),
        )
    }

    fn rejected(
        reasons: BTreeSet<LocalDeploymentCandidateReason>,
        candidate_identifier: String,
    ) -> Self {
        Self::from_parts(
            LocalDeploymentCandidateValidationStatus::Rejected,
            reasons.into_iter().collect(),
            candidate_identifier,
            false,
            false,
            "local deployment candidate rejected fail-closed as non-authoritative descriptive evidence only; deployment, release, production, public-use, and readiness authority remain disabled".to_string(),
        )
    }

    fn from_parts(
        status: LocalDeploymentCandidateValidationStatus,
        reasons: Vec<LocalDeploymentCandidateReason>,
        candidate_identifier: String,
        local_only: bool,
        manual_review_required: bool,
        summary: String,
    ) -> Self {
        let reason_codes = reasons
            .iter()
            .map(LocalDeploymentCandidateReason::code)
            .collect();
        let denied = LocalDeploymentCandidateAuthorityDenialSnapshot::denied();
        Self {
            status,
            reasons,
            reason_codes,
            candidate_identifier,
            local_only,
            candidate_evidence_only: true,
            non_authoritative: true,
            manual_review_required,
            deterministic: true,
            fail_closed: true,
            mutates_filesystem: false,
            mutates_permissions: false,
            opens_network_socket: false,
            starts_service: false,
            launches_process: false,
            deployment_automation_enabled: false,
            release_artifact_created: false,
            installer_enabled: false,
            update_channel_enabled: false,
            signing_enabled: false,
            publishing_enabled: false,
            github_release_created: false,
            release_tag_created: false,
            production_deployment_enabled: false,
            public_release_enabled: false,
            public_use_approved: false,
            production_human_use_approved: false,
            provider_trust_granted: false,
            provider_output_promoted: false,
            replay_repaired: false,
            recovery_promoted: false,
            action_executed: false,
            readiness_approved: false,
            production_candidate_approved: false,
            release_candidate_approved: false,
            authority_denial_snapshot: denied,
            summary,
        }
    }
}

pub fn validate_local_deployment_candidate_boundary(
    boundary: &LocalDeploymentCandidateBoundary,
) -> LocalDeploymentCandidateValidationReport {
    let mut reasons = BTreeSet::new();
    validate_candidate_identity(boundary, &mut reasons);
    validate_evidence_references(&boundary.evidence_references, &mut reasons);
    validate_residual_risk(&boundary.residual_risk_acknowledgement, &mut reasons);
    validate_local_review_posture(boundary, &mut reasons);
    validate_candidate_authority_denials(&boundary.authority_denials, &mut reasons);
    validate_side_effect_denials(boundary, &mut reasons);

    if reasons.is_empty() {
        LocalDeploymentCandidateValidationReport::accepted(boundary.candidate_identifier.clone())
    } else {
        LocalDeploymentCandidateValidationReport::rejected(
            reasons,
            boundary.candidate_identifier.clone(),
        )
    }
}

pub fn local_deployment_candidate_grants_authority(
    report: &LocalDeploymentCandidateValidationReport,
) -> bool {
    report.deployment_automation_enabled
        || report.release_artifact_created
        || report.installer_enabled
        || report.update_channel_enabled
        || report.signing_enabled
        || report.publishing_enabled
        || report.github_release_created
        || report.release_tag_created
        || report.production_deployment_enabled
        || report.public_release_enabled
        || report.public_use_approved
        || report.production_human_use_approved
        || report.provider_trust_granted
        || report.provider_output_promoted
        || report.replay_repaired
        || report.recovery_promoted
        || report.action_executed
        || report.readiness_approved
        || report.production_candidate_approved
        || report.release_candidate_approved
        || report.mutates_filesystem
        || report.mutates_permissions
        || report.opens_network_socket
        || report.starts_service
        || report.launches_process
}

pub fn parse_local_deployment_candidate_payload(
    payload: &str,
) -> LocalDeploymentCandidateValidationReport {
    if payload.len() > MAX_LOCAL_DEPLOYMENT_CANDIDATE_PAYLOAD_BYTES {
        let mut reasons = BTreeSet::new();
        reasons.insert(LocalDeploymentCandidateReason::OversizedCandidate);
        return LocalDeploymentCandidateValidationReport::rejected(reasons, String::new());
    }

    match parse_local_deployment_candidate_block(payload) {
        Ok(boundary) => validate_local_deployment_candidate_boundary(&boundary),
        Err((reasons, candidate_identifier)) => {
            LocalDeploymentCandidateValidationReport::rejected(reasons, candidate_identifier)
        }
    }
}

fn validate_candidate_identity(
    boundary: &LocalDeploymentCandidateBoundary,
    reasons: &mut BTreeSet<LocalDeploymentCandidateReason>,
) {
    if boundary.candidate_identifier.trim().is_empty() {
        reasons.insert(LocalDeploymentCandidateReason::MissingCandidateIdentifier);
    }
    if boundary.candidate_mode.trim().is_empty() {
        reasons.insert(LocalDeploymentCandidateReason::MissingLocalOnlyMode);
    }
    if boundary.candidate_mode != "local_only" || !boundary.local_only {
        reasons.insert(LocalDeploymentCandidateReason::NonLocalCandidateModeRejected);
    }
}

fn validate_evidence_references(
    evidence: &LocalDeploymentCandidateEvidenceReferences,
    reasons: &mut BTreeSet<LocalDeploymentCandidateReason>,
) {
    if evidence
        .phase_113_deployment_configuration_evidence
        .trim()
        .is_empty()
    {
        reasons.insert(
            LocalDeploymentCandidateReason::MissingPhase113DeploymentConfigurationEvidenceReference,
        );
    }
    if evidence
        .phase_114_policy_governance_evidence
        .trim()
        .is_empty()
    {
        reasons.insert(
            LocalDeploymentCandidateReason::MissingPhase114PolicyGovernanceEvidenceReference,
        );
    }
    if evidence.phase_115_security_audit_evidence.trim().is_empty() {
        reasons
            .insert(LocalDeploymentCandidateReason::MissingPhase115SecurityAuditEvidenceReference);
    }
    if evidence.storage_configuration_reference.trim().is_empty() {
        reasons.insert(LocalDeploymentCandidateReason::MissingStorageConfigurationReference);
    }
    if evidence.recovery_handoff_reference.trim().is_empty() {
        reasons.insert(LocalDeploymentCandidateReason::MissingRecoveryHandoffReference);
    }
}

fn validate_residual_risk(
    residual_risk: &LocalDeploymentCandidateResidualRiskAcknowledgement,
    reasons: &mut BTreeSet<LocalDeploymentCandidateReason>,
) {
    if !residual_risk.acknowledged
        || residual_risk.acknowledgement_reference.trim().is_empty()
        || residual_risk.residual_risks_reviewed.is_empty()
    {
        reasons.insert(LocalDeploymentCandidateReason::MissingResidualRiskAcknowledgement);
    }
}

fn validate_local_review_posture(
    boundary: &LocalDeploymentCandidateBoundary,
    reasons: &mut BTreeSet<LocalDeploymentCandidateReason>,
) {
    if !boundary.manual_review_required {
        reasons.insert(LocalDeploymentCandidateReason::MissingManualReviewPosture);
    }
    if boundary.supported_local_validation_commands.is_empty() {
        reasons.insert(LocalDeploymentCandidateReason::MissingRequiredField);
    }
    if boundary
        .unsupported_public_production_release_declarations
        .is_empty()
    {
        reasons.insert(
            LocalDeploymentCandidateReason::MissingUnsupportedPublicProductionReleaseDeclaration,
        );
    }
    if boundary.public_availability_claimed {
        reasons.insert(LocalDeploymentCandidateReason::PublicAvailabilityClaimRejected);
    }
    if boundary.production_readiness_claimed {
        reasons.insert(LocalDeploymentCandidateReason::ProductionReadinessClaimRejected);
    }
}

fn validate_candidate_authority_denials(
    denial: &LocalDeploymentCandidateAuthorityDenialSnapshot,
    reasons: &mut BTreeSet<LocalDeploymentCandidateReason>,
) {
    if denial.deployment_automation_enabled {
        reasons.insert(LocalDeploymentCandidateReason::DeploymentAutomationRejected);
    }
    if denial.release_artifact_created {
        reasons.insert(LocalDeploymentCandidateReason::ReleaseArtifactCreationRejected);
    }
    if denial.installer_enabled {
        reasons.insert(LocalDeploymentCandidateReason::InstallerRejected);
    }
    if denial.update_channel_enabled {
        reasons.insert(LocalDeploymentCandidateReason::UpdateChannelRejected);
    }
    if denial.signing_enabled {
        reasons.insert(LocalDeploymentCandidateReason::SigningRejected);
    }
    if denial.publishing_enabled {
        reasons.insert(LocalDeploymentCandidateReason::PublishingRejected);
    }
    if denial.github_release_created {
        reasons.insert(LocalDeploymentCandidateReason::GithubReleaseRejected);
    }
    if denial.release_tag_created {
        reasons.insert(LocalDeploymentCandidateReason::ReleaseTagRejected);
    }
    if denial.production_deployment_enabled {
        reasons.insert(LocalDeploymentCandidateReason::ProductionDeploymentRejected);
    }
    if denial.public_release_enabled {
        reasons.insert(LocalDeploymentCandidateReason::PublicReleaseRejected);
    }
    if denial.public_use_approved {
        reasons.insert(LocalDeploymentCandidateReason::PublicUseApprovalRejected);
    }
    if denial.production_human_use_approved {
        reasons.insert(LocalDeploymentCandidateReason::ProductionHumanUseApprovalRejected);
    }
    if denial.production_candidate_approved {
        reasons.insert(LocalDeploymentCandidateReason::ProductionCandidateApprovalRejected);
    }
    if denial.release_candidate_approved {
        reasons.insert(LocalDeploymentCandidateReason::ReleaseCandidateApprovalRejected);
    }
    if denial.readiness_approved {
        reasons.insert(LocalDeploymentCandidateReason::ReadinessApprovalRejected);
    }
    if denial.provider_trust_granted {
        reasons.insert(LocalDeploymentCandidateReason::ProviderTrustRejected);
    }
    if denial.provider_output_promoted {
        reasons.insert(LocalDeploymentCandidateReason::ProviderOutputPromotionRejected);
    }
    if denial.replay_repaired {
        reasons.insert(LocalDeploymentCandidateReason::ReplayRepairRejected);
    }
    if denial.recovery_promoted {
        reasons.insert(LocalDeploymentCandidateReason::RecoveryPromotionRejected);
    }
    if denial.action_executed {
        reasons.insert(LocalDeploymentCandidateReason::ActionExecutionRejected);
    }
}

fn validate_side_effect_denials(
    boundary: &LocalDeploymentCandidateBoundary,
    reasons: &mut BTreeSet<LocalDeploymentCandidateReason>,
) {
    if boundary.mutates_filesystem {
        reasons.insert(LocalDeploymentCandidateReason::FilesystemMutationRejected);
    }
    if boundary.mutates_permissions {
        reasons.insert(LocalDeploymentCandidateReason::PermissionMutationRejected);
    }
    if boundary.opens_network_socket {
        reasons.insert(LocalDeploymentCandidateReason::NetworkSocketRejected);
    }
    if boundary.starts_service {
        reasons.insert(LocalDeploymentCandidateReason::ServiceStartRejected);
    }
    if boundary.launches_process {
        reasons.insert(LocalDeploymentCandidateReason::ProcessLaunchRejected);
    }
}

fn parse_local_deployment_candidate_block(
    payload: &str,
) -> Result<LocalDeploymentCandidateBoundary, (BTreeSet<LocalDeploymentCandidateReason>, String)> {
    let mut reasons = BTreeSet::new();
    let mut block = BTreeMap::new();
    let mut saw_header = false;

    for raw_line in payload.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "local_deployment_candidate" {
            if saw_header {
                reasons.insert(LocalDeploymentCandidateReason::MalformedCandidate);
            }
            saw_header = true;
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            reasons.insert(LocalDeploymentCandidateReason::MalformedCandidate);
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() || !saw_header {
            reasons.insert(LocalDeploymentCandidateReason::MalformedCandidate);
            continue;
        }
        if !local_deployment_candidate_key_is_allowed(key) {
            reasons.insert(LocalDeploymentCandidateReason::MalformedCandidate);
        }
        if authority_bearing_candidate_key_or_value(key, value) {
            reasons.insert(LocalDeploymentCandidateReason::AuthorityBearingCandidateRejected);
        }
        if block.insert(key.to_string(), value.to_string()).is_some() {
            reasons.insert(LocalDeploymentCandidateReason::MalformedCandidate);
        }
    }

    if !saw_header {
        reasons.insert(LocalDeploymentCandidateReason::MissingRequiredField);
    }
    let candidate_identifier = block
        .get("candidate_identifier")
        .cloned()
        .unwrap_or_default();
    if !reasons.is_empty() {
        return Err((reasons, candidate_identifier));
    }

    boundary_from_block(&block).map_err(|reasons| (reasons, candidate_identifier))
}

fn boundary_from_block(
    block: &BTreeMap<String, String>,
) -> Result<LocalDeploymentCandidateBoundary, BTreeSet<LocalDeploymentCandidateReason>> {
    let mut reasons = BTreeSet::new();
    let candidate_identifier = required_value(block, "candidate_identifier", &mut reasons)
        .unwrap_or_default()
        .to_string();
    let candidate_mode = required_value(block, "candidate_mode", &mut reasons)
        .unwrap_or_default()
        .to_string();
    let local_only = parse_required_bool(block, "local_only", &mut reasons);
    let evidence_references = LocalDeploymentCandidateEvidenceReferences {
        phase_113_deployment_configuration_evidence: required_value(
            block,
            "phase_113_deployment_configuration_evidence",
            &mut reasons,
        )
        .unwrap_or_default()
        .to_string(),
        phase_114_policy_governance_evidence: required_value(
            block,
            "phase_114_policy_governance_evidence",
            &mut reasons,
        )
        .unwrap_or_default()
        .to_string(),
        phase_115_security_audit_evidence: required_value(
            block,
            "phase_115_security_audit_evidence",
            &mut reasons,
        )
        .unwrap_or_default()
        .to_string(),
        storage_configuration_reference: required_value(
            block,
            "storage_configuration_reference",
            &mut reasons,
        )
        .unwrap_or_default()
        .to_string(),
        recovery_handoff_reference: required_value(
            block,
            "recovery_handoff_reference",
            &mut reasons,
        )
        .unwrap_or_default()
        .to_string(),
    };
    let residual_risk_acknowledgement = LocalDeploymentCandidateResidualRiskAcknowledgement {
        acknowledged: parse_required_bool(block, "residual_risk_acknowledged", &mut reasons),
        acknowledgement_reference: required_value(
            block,
            "residual_risk_acknowledgement_reference",
            &mut reasons,
        )
        .unwrap_or_default()
        .to_string(),
        residual_risks_reviewed: parse_string_list(block, "residual_risks_reviewed", &mut reasons),
    };
    let authority_denials = LocalDeploymentCandidateAuthorityDenialSnapshot {
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
        github_release_created: parse_required_bool(block, "github_release_created", &mut reasons),
        release_tag_created: parse_required_bool(block, "release_tag_created", &mut reasons),
        production_deployment_enabled: parse_required_bool(
            block,
            "production_deployment_enabled",
            &mut reasons,
        ),
        public_release_enabled: parse_required_bool(block, "public_release_enabled", &mut reasons),
        public_use_approved: parse_required_bool(block, "public_use_approved", &mut reasons),
        production_human_use_approved: parse_required_bool(
            block,
            "production_human_use_approved",
            &mut reasons,
        ),
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
    };

    let manual_review_required = parse_bool_value(block, "manual_review_required", &mut reasons);
    let supported_local_validation_commands =
        parse_string_list(block, "supported_local_validation_commands", &mut reasons);
    let unsupported_public_production_release_declarations = parse_string_list(
        block,
        "unsupported_public_production_release_declarations",
        &mut reasons,
    );
    let mutates_filesystem = parse_bool_value(block, "mutates_filesystem", &mut reasons);
    let mutates_permissions = parse_bool_value(block, "mutates_permissions", &mut reasons);
    let opens_network_socket = parse_bool_value(block, "opens_network_socket", &mut reasons);
    let starts_service = parse_bool_value(block, "starts_service", &mut reasons);
    let launches_process = parse_bool_value(block, "launches_process", &mut reasons);
    let public_availability_claimed =
        parse_bool_value(block, "public_availability_claimed", &mut reasons);
    let production_readiness_claimed =
        parse_bool_value(block, "production_readiness_claimed", &mut reasons);

    if !reasons.is_empty() {
        return Err(reasons);
    }

    Ok(LocalDeploymentCandidateBoundary {
        candidate_identifier,
        candidate_mode,
        local_only,
        evidence_references,
        residual_risk_acknowledgement,
        manual_review_required,
        supported_local_validation_commands,
        unsupported_public_production_release_declarations,
        authority_denials,
        mutates_filesystem,
        mutates_permissions,
        opens_network_socket,
        starts_service,
        launches_process,
        public_availability_claimed,
        production_readiness_claimed,
    })
}

fn required_value<'a>(
    block: &'a BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<LocalDeploymentCandidateReason>,
) -> Option<&'a str> {
    match block.get(key) {
        Some(value) => Some(value.as_str()),
        None => {
            reasons.insert(LocalDeploymentCandidateReason::MissingRequiredField);
            None
        }
    }
}

fn parse_required_bool(
    block: &BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<LocalDeploymentCandidateReason>,
) -> bool {
    match required_value(block, key, reasons) {
        Some(value) => parse_bool_literal(value, reasons),
        None => false,
    }
}

fn parse_bool_value(
    block: &BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<LocalDeploymentCandidateReason>,
) -> bool {
    match block.get(key) {
        Some(value) => parse_bool_literal(value, reasons),
        None => {
            reasons.insert(LocalDeploymentCandidateReason::MissingRequiredField);
            false
        }
    }
}

fn parse_bool_literal(value: &str, reasons: &mut BTreeSet<LocalDeploymentCandidateReason>) -> bool {
    match value {
        "true" => true,
        "false" => false,
        _ => {
            reasons.insert(LocalDeploymentCandidateReason::MalformedCandidate);
            false
        }
    }
}

fn parse_string_list(
    block: &BTreeMap<String, String>,
    key: &str,
    reasons: &mut BTreeSet<LocalDeploymentCandidateReason>,
) -> Vec<String> {
    let Some(value) = required_value(block, key, reasons) else {
        return Vec::new();
    };
    let mut values = BTreeSet::new();
    for item in value.split(',').map(str::trim) {
        if item.is_empty() {
            reasons.insert(LocalDeploymentCandidateReason::MalformedCandidate);
        } else {
            values.insert(item.to_string());
        }
    }
    values.into_iter().collect()
}

fn local_deployment_candidate_key_is_allowed(key: &str) -> bool {
    matches!(
        key,
        "candidate_identifier"
            | "candidate_mode"
            | "local_only"
            | "phase_113_deployment_configuration_evidence"
            | "phase_114_policy_governance_evidence"
            | "phase_115_security_audit_evidence"
            | "storage_configuration_reference"
            | "recovery_handoff_reference"
            | "residual_risk_acknowledged"
            | "residual_risk_acknowledgement_reference"
            | "residual_risks_reviewed"
            | "manual_review_required"
            | "supported_local_validation_commands"
            | "unsupported_public_production_release_declarations"
            | "deployment_automation_enabled"
            | "release_artifact_created"
            | "installer_enabled"
            | "update_channel_enabled"
            | "signing_enabled"
            | "publishing_enabled"
            | "github_release_created"
            | "release_tag_created"
            | "production_deployment_enabled"
            | "public_release_enabled"
            | "public_use_approved"
            | "production_human_use_approved"
            | "provider_trust_granted"
            | "provider_output_promoted"
            | "replay_repaired"
            | "recovery_promoted"
            | "action_executed"
            | "readiness_approved"
            | "production_candidate_approved"
            | "release_candidate_approved"
            | "mutates_filesystem"
            | "mutates_permissions"
            | "opens_network_socket"
            | "starts_service"
            | "launches_process"
            | "public_availability_claimed"
            | "production_readiness_claimed"
    )
}

fn authority_bearing_candidate_key_or_value(key: &str, value: &str) -> bool {
    let lowered = format!("{}={}", key, value).to_ascii_lowercase();
    [
        "public_download",
        "process_exec",
        "network_exec",
        "filesystem_mutation",
        "start_daemon",
        "background_service",
        "production_ready=true",
        "release_ready=true",
    ]
    .iter()
    .any(|needle| lowered.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_boundary() -> LocalDeploymentCandidateBoundary {
        LocalDeploymentCandidateBoundary {
            candidate_identifier: "phase-116-local-candidate".to_string(),
            candidate_mode: "local_only".to_string(),
            local_only: true,
            evidence_references: LocalDeploymentCandidateEvidenceReferences {
                phase_113_deployment_configuration_evidence:
                    "docs/operations/deployment-configuration-contract-phase-113.md".to_string(),
                phase_114_policy_governance_evidence:
                    "docs/operations/policy-governance-versioning-phase-114.md".to_string(),
                phase_115_security_audit_evidence:
                    "docs/operations/security-threat-model-abuse-case-audit-phase-115.md"
                        .to_string(),
                storage_configuration_reference: "phase-113-storage-configuration".to_string(),
                recovery_handoff_reference: "phase-112-recovery-handoff".to_string(),
            },
            residual_risk_acknowledgement: LocalDeploymentCandidateResidualRiskAcknowledgement {
                acknowledged: true,
                acknowledgement_reference: "phase-115-residual-risks".to_string(),
                residual_risks_reviewed: vec!["local-only-testing-risk".to_string()],
            },
            manual_review_required: true,
            supported_local_validation_commands: vec!["./scripts/check.sh".to_string()],
            unsupported_public_production_release_declarations: vec![
                "no-public-release".to_string(),
                "no-production-deployment".to_string(),
            ],
            authority_denials: LocalDeploymentCandidateAuthorityDenialSnapshot::denied(),
            mutates_filesystem: false,
            mutates_permissions: false,
            opens_network_socket: false,
            starts_service: false,
            launches_process: false,
            public_availability_claimed: false,
            production_readiness_claimed: false,
        }
    }

    fn valid_payload() -> String {
        [
            "local_deployment_candidate",
            "candidate_identifier=phase-116-local-candidate",
            "candidate_mode=local_only",
            "local_only=true",
            "phase_113_deployment_configuration_evidence=docs/operations/deployment-configuration-contract-phase-113.md",
            "phase_114_policy_governance_evidence=docs/operations/policy-governance-versioning-phase-114.md",
            "phase_115_security_audit_evidence=docs/operations/security-threat-model-abuse-case-audit-phase-115.md",
            "storage_configuration_reference=phase-113-storage-configuration",
            "recovery_handoff_reference=phase-112-recovery-handoff",
            "residual_risk_acknowledged=true",
            "residual_risk_acknowledgement_reference=phase-115-residual-risks",
            "residual_risks_reviewed=local-only-testing-risk",
            "manual_review_required=true",
            "supported_local_validation_commands=./scripts/check.sh,cargo test --manifest-path core/Cargo.toml phase_116 --all-targets",
            "unsupported_public_production_release_declarations=no-public-release,no-production-deployment,no-installer,no-update-channel,no-signing,no-publishing",
            "deployment_automation_enabled=false",
            "release_artifact_created=false",
            "installer_enabled=false",
            "update_channel_enabled=false",
            "signing_enabled=false",
            "publishing_enabled=false",
            "github_release_created=false",
            "release_tag_created=false",
            "production_deployment_enabled=false",
            "public_release_enabled=false",
            "public_use_approved=false",
            "production_human_use_approved=false",
            "provider_trust_granted=false",
            "provider_output_promoted=false",
            "replay_repaired=false",
            "recovery_promoted=false",
            "action_executed=false",
            "readiness_approved=false",
            "production_candidate_approved=false",
            "release_candidate_approved=false",
            "mutates_filesystem=false",
            "mutates_permissions=false",
            "opens_network_socket=false",
            "starts_service=false",
            "launches_process=false",
            "public_availability_claimed=false",
            "production_readiness_claimed=false",
        ]
        .join("\n")
    }

    #[test]
    fn phase_116_valid_local_deployment_candidate_validates_as_local_only_contract_evidence() {
        let report = validate_local_deployment_candidate_boundary(&valid_boundary());
        assert_eq!(
            report.status,
            LocalDeploymentCandidateValidationStatus::Accepted
        );
        assert!(report.local_only);
        assert!(report.candidate_evidence_only);
        assert!(report.non_authoritative);
        assert!(report.manual_review_required);
        assert!(!local_deployment_candidate_grants_authority(&report));
        assert_eq!(
            report.reasons,
            vec![LocalDeploymentCandidateReason::LocalOnlyContractEvidenceAccepted]
        );
    }

    #[test]
    fn phase_116_required_identity_evidence_risk_and_review_fields_reject() {
        let mut boundary = valid_boundary();
        boundary.candidate_identifier.clear();
        assert!(validate_local_deployment_candidate_boundary(&boundary)
            .reasons
            .contains(&LocalDeploymentCandidateReason::MissingCandidateIdentifier));

        let mut boundary = valid_boundary();
        boundary.candidate_mode.clear();
        boundary.local_only = false;
        let report = validate_local_deployment_candidate_boundary(&boundary);
        assert!(report
            .reasons
            .contains(&LocalDeploymentCandidateReason::MissingLocalOnlyMode));
        assert!(report
            .reasons
            .contains(&LocalDeploymentCandidateReason::NonLocalCandidateModeRejected));

        let mut boundary = valid_boundary();
        boundary
            .evidence_references
            .phase_113_deployment_configuration_evidence
            .clear();
        assert!(validate_local_deployment_candidate_boundary(&boundary).reasons.contains(
            &LocalDeploymentCandidateReason::MissingPhase113DeploymentConfigurationEvidenceReference
        ));

        let mut boundary = valid_boundary();
        boundary
            .evidence_references
            .phase_114_policy_governance_evidence
            .clear();
        assert!(validate_local_deployment_candidate_boundary(&boundary)
            .reasons
            .contains(
                &LocalDeploymentCandidateReason::MissingPhase114PolicyGovernanceEvidenceReference
            ));

        let mut boundary = valid_boundary();
        boundary
            .evidence_references
            .phase_115_security_audit_evidence
            .clear();
        assert!(validate_local_deployment_candidate_boundary(&boundary)
            .reasons
            .contains(
                &LocalDeploymentCandidateReason::MissingPhase115SecurityAuditEvidenceReference
            ));

        let mut boundary = valid_boundary();
        boundary.residual_risk_acknowledgement.acknowledged = false;
        assert!(validate_local_deployment_candidate_boundary(&boundary)
            .reasons
            .contains(&LocalDeploymentCandidateReason::MissingResidualRiskAcknowledgement));

        let mut boundary = valid_boundary();
        boundary.manual_review_required = false;
        assert!(validate_local_deployment_candidate_boundary(&boundary)
            .reasons
            .contains(&LocalDeploymentCandidateReason::MissingManualReviewPosture));
    }

    #[test]
    fn phase_116_authority_and_release_claims_reject() {
        let cases: Vec<(&str, LocalDeploymentCandidateReason)> = vec![
            (
                "deployment_automation_enabled",
                LocalDeploymentCandidateReason::DeploymentAutomationRejected,
            ),
            (
                "release_artifact_created",
                LocalDeploymentCandidateReason::ReleaseArtifactCreationRejected,
            ),
            (
                "installer_enabled",
                LocalDeploymentCandidateReason::InstallerRejected,
            ),
            (
                "update_channel_enabled",
                LocalDeploymentCandidateReason::UpdateChannelRejected,
            ),
            (
                "signing_enabled",
                LocalDeploymentCandidateReason::SigningRejected,
            ),
            (
                "publishing_enabled",
                LocalDeploymentCandidateReason::PublishingRejected,
            ),
            (
                "github_release_created",
                LocalDeploymentCandidateReason::GithubReleaseRejected,
            ),
            (
                "release_tag_created",
                LocalDeploymentCandidateReason::ReleaseTagRejected,
            ),
            (
                "production_deployment_enabled",
                LocalDeploymentCandidateReason::ProductionDeploymentRejected,
            ),
            (
                "public_release_enabled",
                LocalDeploymentCandidateReason::PublicReleaseRejected,
            ),
            (
                "public_use_approved",
                LocalDeploymentCandidateReason::PublicUseApprovalRejected,
            ),
            (
                "production_human_use_approved",
                LocalDeploymentCandidateReason::ProductionHumanUseApprovalRejected,
            ),
            (
                "production_candidate_approved",
                LocalDeploymentCandidateReason::ProductionCandidateApprovalRejected,
            ),
            (
                "release_candidate_approved",
                LocalDeploymentCandidateReason::ReleaseCandidateApprovalRejected,
            ),
            (
                "readiness_approved",
                LocalDeploymentCandidateReason::ReadinessApprovalRejected,
            ),
            (
                "provider_trust_granted",
                LocalDeploymentCandidateReason::ProviderTrustRejected,
            ),
            (
                "provider_output_promoted",
                LocalDeploymentCandidateReason::ProviderOutputPromotionRejected,
            ),
            (
                "replay_repaired",
                LocalDeploymentCandidateReason::ReplayRepairRejected,
            ),
            (
                "recovery_promoted",
                LocalDeploymentCandidateReason::RecoveryPromotionRejected,
            ),
            (
                "action_executed",
                LocalDeploymentCandidateReason::ActionExecutionRejected,
            ),
        ];

        for (field, reason) in cases {
            let mut boundary = valid_boundary();
            match field {
                "deployment_automation_enabled" => {
                    boundary.authority_denials.deployment_automation_enabled = true
                }
                "release_artifact_created" => {
                    boundary.authority_denials.release_artifact_created = true
                }
                "installer_enabled" => boundary.authority_denials.installer_enabled = true,
                "update_channel_enabled" => {
                    boundary.authority_denials.update_channel_enabled = true
                }
                "signing_enabled" => boundary.authority_denials.signing_enabled = true,
                "publishing_enabled" => boundary.authority_denials.publishing_enabled = true,
                "github_release_created" => {
                    boundary.authority_denials.github_release_created = true
                }
                "release_tag_created" => boundary.authority_denials.release_tag_created = true,
                "production_deployment_enabled" => {
                    boundary.authority_denials.production_deployment_enabled = true
                }
                "public_release_enabled" => {
                    boundary.authority_denials.public_release_enabled = true
                }
                "public_use_approved" => boundary.authority_denials.public_use_approved = true,
                "production_human_use_approved" => {
                    boundary.authority_denials.production_human_use_approved = true
                }
                "production_candidate_approved" => {
                    boundary.authority_denials.production_candidate_approved = true
                }
                "release_candidate_approved" => {
                    boundary.authority_denials.release_candidate_approved = true
                }
                "readiness_approved" => boundary.authority_denials.readiness_approved = true,
                "provider_trust_granted" => {
                    boundary.authority_denials.provider_trust_granted = true
                }
                "provider_output_promoted" => {
                    boundary.authority_denials.provider_output_promoted = true
                }
                "replay_repaired" => boundary.authority_denials.replay_repaired = true,
                "recovery_promoted" => boundary.authority_denials.recovery_promoted = true,
                "action_executed" => boundary.authority_denials.action_executed = true,
                _ => unreachable!(),
            }
            let report = validate_local_deployment_candidate_boundary(&boundary);
            assert_eq!(
                report.status,
                LocalDeploymentCandidateValidationStatus::Rejected
            );
            assert!(report.reasons.contains(&reason), "{field}");
            assert!(!local_deployment_candidate_grants_authority(&report));
        }
    }

    #[test]
    fn phase_116_side_effect_shaped_candidate_rejects_without_authority() {
        let cases: Vec<(&str, LocalDeploymentCandidateReason)> = vec![
            (
                "mutates_filesystem",
                LocalDeploymentCandidateReason::FilesystemMutationRejected,
            ),
            (
                "mutates_permissions",
                LocalDeploymentCandidateReason::PermissionMutationRejected,
            ),
            (
                "opens_network_socket",
                LocalDeploymentCandidateReason::NetworkSocketRejected,
            ),
            (
                "starts_service",
                LocalDeploymentCandidateReason::ServiceStartRejected,
            ),
            (
                "launches_process",
                LocalDeploymentCandidateReason::ProcessLaunchRejected,
            ),
        ];

        for (field, reason) in cases {
            let mut boundary = valid_boundary();
            match field {
                "mutates_filesystem" => boundary.mutates_filesystem = true,
                "mutates_permissions" => boundary.mutates_permissions = true,
                "opens_network_socket" => boundary.opens_network_socket = true,
                "starts_service" => boundary.starts_service = true,
                "launches_process" => boundary.launches_process = true,
                _ => unreachable!(),
            }
            let report = validate_local_deployment_candidate_boundary(&boundary);
            assert_eq!(
                report.status,
                LocalDeploymentCandidateValidationStatus::Rejected
            );
            assert!(report.reasons.contains(&reason), "{field}");
            assert!(!report.mutates_filesystem);
            assert!(!report.mutates_permissions);
            assert!(!report.opens_network_socket);
            assert!(!report.starts_service);
            assert!(!report.launches_process);
        }
    }

    #[test]
    fn phase_116_validation_is_deterministic_across_equivalent_input() {
        let report_one = parse_local_deployment_candidate_payload(&valid_payload());
        let report_two = parse_local_deployment_candidate_payload(&valid_payload());
        assert_eq!(report_one, report_two);
        assert!(report_one.deterministic);
        assert!(report_one.fail_closed);
    }

    #[test]
    fn phase_116_payload_rejects_malformed_and_authority_noise() {
        let report = parse_local_deployment_candidate_payload(
            "local_deployment_candidate\nunknown=value\nprocess_exec=true\n",
        );
        assert_eq!(
            report.status,
            LocalDeploymentCandidateValidationStatus::Rejected
        );
        assert!(report
            .reasons
            .contains(&LocalDeploymentCandidateReason::MalformedCandidate));
        assert!(report
            .reasons
            .contains(&LocalDeploymentCandidateReason::AuthorityBearingCandidateRejected));
    }
}
