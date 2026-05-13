//! Trial replay/restore verification projections and boundary checks.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialReplayRestoreVerificationStatus {
    NotVerified,
    VerificationProjected,
    VerificationPassed,
    VerificationRejected,
    VerificationInputMissing,
    InvalidVerificationInput,
}

impl TrialReplayRestoreVerificationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotVerified => "not_verified",
            Self::VerificationProjected => "trial_replay_restore_verification_projected",
            Self::VerificationPassed => "verification_passed",
            Self::VerificationRejected => "verification_rejected",
            Self::VerificationInputMissing => "verification_input_missing",
            Self::InvalidVerificationInput => "invalid_verification_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrialReplayRestoreVerificationMismatch {
    MissingTrialPackageReadBack,
    MissingTrialSessionEvidenceReadBack,
    TrialPackageReadBackInvalid,
    TrialSessionEvidenceReadBackInvalid,
    PackageIdMismatch,
    PackageVersionMismatch,
    PackageClassificationMismatch,
    ProductionClassificationMismatch,
    DistributionClassificationMismatch,
    AuthorityClassificationMismatch,
    WorkflowStatusMismatch,
    LocalCandidateMaterializationStatusMismatch,
    ProviderOutputPipelineStatusMismatch,
    OperatorDecisionStatusMismatch,
    ReplayStatusSnapshotMismatch,
    RestoreHistorySnapshotMismatch,
    StopConditionSnapshotMismatch,
    EscalationSnapshotMismatch,
    FailureCategorySnapshotMismatch,
    ReadinessClaimDetected,
    ReleaseClaimDetected,
    DeploymentClaimDetected,
    PublicUseClaimDetected,
    ProductionUseClaimDetected,
    ProviderTrustClaimDetected,
    ActionAuthorizationClaimDetected,
    ReplayRepairClaimDetected,
    RecoveryPromotionClaimDetected,
}

impl TrialReplayRestoreVerificationMismatch {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingTrialPackageReadBack => "missing_trial_package_read_back",
            Self::MissingTrialSessionEvidenceReadBack => "missing_trial_session_evidence_read_back",
            Self::TrialPackageReadBackInvalid => "trial_package_read_back_invalid",
            Self::TrialSessionEvidenceReadBackInvalid => "trial_session_evidence_read_back_invalid",
            Self::PackageIdMismatch => "package_id_mismatch",
            Self::PackageVersionMismatch => "package_version_mismatch",
            Self::PackageClassificationMismatch => "package_classification_mismatch",
            Self::ProductionClassificationMismatch => "production_classification_mismatch",
            Self::DistributionClassificationMismatch => "distribution_classification_mismatch",
            Self::AuthorityClassificationMismatch => "authority_classification_mismatch",
            Self::WorkflowStatusMismatch => "workflow_status_mismatch",
            Self::LocalCandidateMaterializationStatusMismatch => {
                "local_candidate_materialization_status_mismatch"
            }
            Self::ProviderOutputPipelineStatusMismatch => {
                "provider_output_pipeline_status_mismatch"
            }
            Self::OperatorDecisionStatusMismatch => "operator_decision_status_mismatch",
            Self::ReplayStatusSnapshotMismatch => "replay_status_snapshot_mismatch",
            Self::RestoreHistorySnapshotMismatch => "restore_history_snapshot_mismatch",
            Self::StopConditionSnapshotMismatch => "stop_condition_snapshot_mismatch",
            Self::EscalationSnapshotMismatch => "escalation_snapshot_mismatch",
            Self::FailureCategorySnapshotMismatch => "failure_category_snapshot_mismatch",
            Self::ReadinessClaimDetected => "readiness_claim_detected",
            Self::ReleaseClaimDetected => "release_claim_detected",
            Self::DeploymentClaimDetected => "deployment_claim_detected",
            Self::PublicUseClaimDetected => "public_use_claim_detected",
            Self::ProductionUseClaimDetected => "production_use_claim_detected",
            Self::ProviderTrustClaimDetected => "provider_trust_claim_detected",
            Self::ActionAuthorizationClaimDetected => "action_authorization_claim_detected",
            Self::ReplayRepairClaimDetected => "replay_repair_claim_detected",
            Self::RecoveryPromotionClaimDetected => "recovery_promotion_claim_detected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialReplayRestoreVerificationBoundaryStatus {
    LocalVerificationOnly,
    NonPublicVerification,
    VerificationNotAuthority,
    NoTrialExecution,
    NoControlledHumanUseApproval,
    NoReadinessApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoProductionApproval,
    NoProviderTrust,
    NoActionAuthorization,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl TrialReplayRestoreVerificationBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalVerificationOnly => "local_verification_only",
            Self::NonPublicVerification => "non_public_verification",
            Self::VerificationNotAuthority => "verification_not_authority",
            Self::NoTrialExecution => "no_trial_execution",
            Self::NoControlledHumanUseApproval => "no_controlled_human_use_approval",
            Self::NoReadinessApproval => "no_readiness_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoActionAuthorization => "no_action_authorization",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialReplayRestoreVerificationComparisonSummary {
    pub package_read_back_status: String,
    pub evidence_read_back_status: String,
    pub package_evidence_linkage_status: String,
    pub workflow_linkage_status: String,
    pub replay_status_comparison: String,
    pub restore_history_comparison: String,
    pub stop_condition_comparison: String,
    pub escalation_failure_comparison: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialReplayRestoreVerificationProjection {
    pub status: TrialReplayRestoreVerificationStatus,
    pub verification_id: Option<String>,
    pub comparison_summary: TrialReplayRestoreVerificationComparisonSummary,
    pub matched_fields: Vec<String>,
    pub mismatches: Vec<TrialReplayRestoreVerificationMismatch>,
    pub missing_inputs: Vec<String>,
    pub boundary_status: Vec<TrialReplayRestoreVerificationBoundaryStatus>,
    pub local_only_non_authority_note: String,
    pub comparison_scope_note: String,
    pub no_replay_repair_note: String,
    pub no_recovery_promotion_note: String,
    pub no_approval_note: String,
    pub no_action_execution_note: String,
}

pub fn trial_replay_restore_verification_boundary_statuses(
) -> Vec<TrialReplayRestoreVerificationBoundaryStatus> {
    vec![
        TrialReplayRestoreVerificationBoundaryStatus::LocalVerificationOnly,
        TrialReplayRestoreVerificationBoundaryStatus::NonPublicVerification,
        TrialReplayRestoreVerificationBoundaryStatus::VerificationNotAuthority,
        TrialReplayRestoreVerificationBoundaryStatus::NoTrialExecution,
        TrialReplayRestoreVerificationBoundaryStatus::NoControlledHumanUseApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoReadinessApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoReleaseApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoDeploymentApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoPublicUseApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoProductionApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoProviderTrust,
        TrialReplayRestoreVerificationBoundaryStatus::NoActionAuthorization,
        TrialReplayRestoreVerificationBoundaryStatus::NoReplayRepair,
        TrialReplayRestoreVerificationBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub fn initial_trial_replay_restore_verification_projection(
) -> TrialReplayRestoreVerificationProjection {
    TrialReplayRestoreVerificationProjection {
        status: TrialReplayRestoreVerificationStatus::NotVerified,
        verification_id: None,
        comparison_summary: TrialReplayRestoreVerificationComparisonSummary {
            package_read_back_status: "not_verified".to_string(),
            evidence_read_back_status: "not_verified".to_string(),
            package_evidence_linkage_status: "not_verified".to_string(),
            workflow_linkage_status: "not_verified".to_string(),
            replay_status_comparison: "not_verified".to_string(),
            restore_history_comparison: "not_verified".to_string(),
            stop_condition_comparison: "not_verified".to_string(),
            escalation_failure_comparison: "not_verified".to_string(),
        },
        matched_fields: Vec::new(),
        mismatches: Vec::new(),
        missing_inputs: Vec::new(),
        boundary_status: trial_replay_restore_verification_boundary_statuses(),
        local_only_non_authority_note: "Trial replay and restore verification is local-only, non-public, and non-authoritative.".to_string(),
        comparison_scope_note: "Verification compares trial artifacts and restore/replay projections.".to_string(),
        no_replay_repair_note: "Verification does not repair replay.".to_string(),
        no_recovery_promotion_note: "Verification does not promote recovery.".to_string(),
        no_approval_note: "Verification does not approve controlled human use, readiness, release, deployment, public use, or production use.".to_string(),
        no_action_execution_note: "Verification does not execute actions.".to_string(),
    }
}

fn stable_trial_replay_restore_verification_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("trial-replay-restore-verification-{hash:016x}")
}

fn add_verification_mismatch(
    mismatches: &mut Vec<TrialReplayRestoreVerificationMismatch>,
    mismatch: TrialReplayRestoreVerificationMismatch,
) {
    if !mismatches.contains(&mismatch) {
        mismatches.push(mismatch);
    }
}

fn compare_verification_field(
    matched_fields: &mut Vec<String>,
    mismatches: &mut Vec<TrialReplayRestoreVerificationMismatch>,
    field: &str,
    left: &str,
    right: &str,
    mismatch: TrialReplayRestoreVerificationMismatch,
) {
    if left == right {
        matched_fields.push(format!("{field}={left}"));
    } else {
        add_verification_mismatch(mismatches, mismatch);
    }
}

fn current_trial_verification_restore_history_snapshot(state: &LocalOperatorShellState) -> String {
    format!(
        "{} / {}",
        state.local_session_restore_projection.status.code(),
        state.local_session_history_projection.status.code()
    )
}

fn current_trial_verification_escalation_snapshot(state: &LocalOperatorShellState) -> Vec<String> {
    state
        .trial_failure_drill
        .escalation_guidance
        .iter()
        .map(|entry| format!("{}:{}", entry.role.code(), entry.guidance))
        .collect()
}

fn current_trial_verification_failure_snapshot(state: &LocalOperatorShellState) -> Vec<String> {
    state
        .trial_failure_drill
        .categories
        .iter()
        .map(|entry| format!("{}={}", entry.category.code(), entry.severity.code()))
        .collect()
}

fn collect_trial_verification_claim_mismatches(
    text: &str,
    mismatches: &mut Vec<TrialReplayRestoreVerificationMismatch>,
) {
    let checks = [
        (
            "claim:readiness_approved",
            TrialReplayRestoreVerificationMismatch::ReadinessClaimDetected,
        ),
        (
            "claim:release_candidate_approved",
            TrialReplayRestoreVerificationMismatch::ReleaseClaimDetected,
        ),
        (
            "claim:deployment_enabled",
            TrialReplayRestoreVerificationMismatch::DeploymentClaimDetected,
        ),
        (
            "claim:public_use_approved",
            TrialReplayRestoreVerificationMismatch::PublicUseClaimDetected,
        ),
        (
            "claim:production_use_approved",
            TrialReplayRestoreVerificationMismatch::ProductionUseClaimDetected,
        ),
        (
            "claim:production_persistence_enabled",
            TrialReplayRestoreVerificationMismatch::ProductionUseClaimDetected,
        ),
        (
            "claim:provider_trusted",
            TrialReplayRestoreVerificationMismatch::ProviderTrustClaimDetected,
        ),
        (
            "claim:action_authorized",
            TrialReplayRestoreVerificationMismatch::ActionAuthorizationClaimDetected,
        ),
        (
            "claim:action_executed",
            TrialReplayRestoreVerificationMismatch::ActionAuthorizationClaimDetected,
        ),
        (
            "claim:replay_repaired",
            TrialReplayRestoreVerificationMismatch::ReplayRepairClaimDetected,
        ),
        (
            "claim:recovery_promoted",
            TrialReplayRestoreVerificationMismatch::RecoveryPromotionClaimDetected,
        ),
    ];
    let lowered = text.to_ascii_lowercase();
    for (needle, mismatch) in checks {
        if lowered.contains(needle) {
            add_verification_mismatch(mismatches, mismatch);
        }
    }
}

pub fn derive_trial_replay_restore_verification_projection(
    state: &LocalOperatorShellState,
    package_read_back: Option<&ControlledInternalTrialPackage>,
    evidence_read_back: Option<&TrialSessionEvidenceRecord>,
) -> TrialReplayRestoreVerificationProjection {
    let mut projection = initial_trial_replay_restore_verification_projection();
    projection.status = TrialReplayRestoreVerificationStatus::VerificationProjected;
    let mut matched_fields = Vec::new();
    let mut mismatches = Vec::new();
    let mut missing_inputs = Vec::new();

    let Some(package) = package_read_back else {
        missing_inputs.push("trial_package_read_back".to_string());
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::MissingTrialPackageReadBack,
        );
        projection.status = TrialReplayRestoreVerificationStatus::VerificationInputMissing;
        projection.missing_inputs = missing_inputs;
        projection.mismatches = mismatches;
        projection.comparison_summary.package_read_back_status =
            "missing_trial_package_read_back".to_string();
        return projection;
    };
    let Some(evidence) = evidence_read_back else {
        missing_inputs.push("trial_session_evidence_read_back".to_string());
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::MissingTrialSessionEvidenceReadBack,
        );
        projection.status = TrialReplayRestoreVerificationStatus::VerificationInputMissing;
        projection.missing_inputs = missing_inputs;
        projection.mismatches = mismatches;
        projection.comparison_summary.package_read_back_status =
            "package_read_back_present".to_string();
        projection.comparison_summary.evidence_read_back_status =
            "missing_trial_session_evidence_read_back".to_string();
        return projection;
    };

    let package_valid = validate_controlled_internal_trial_package(package).is_ok()
        && package.metadata.package_status
            == ControlledInternalTrialPackageStatus::PackageReadBackValidated;
    if package_valid {
        matched_fields.push("trial_package_read_back=valid".to_string());
    } else {
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::TrialPackageReadBackInvalid,
        );
    }
    let evidence_valid = validate_trial_session_evidence_record(evidence).is_ok()
        && evidence.metadata.evidence_status
            == TrialSessionEvidenceStatus::EvidenceReadBackValidated;
    if evidence_valid {
        matched_fields.push("trial_session_evidence_read_back=valid".to_string());
    } else {
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::TrialSessionEvidenceReadBackInvalid,
        );
    }

    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "package_id",
        &package.metadata.package_id,
        &evidence.payload.trial_package_id,
        TrialReplayRestoreVerificationMismatch::PackageIdMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "package_version",
        &package.metadata.package_version,
        &evidence.payload.trial_package_version,
        TrialReplayRestoreVerificationMismatch::PackageVersionMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "package_classification",
        &package.metadata.package_classification,
        ControlledInternalTrialPackageClassification::ControlledInternalTrialPackageOnly.code(),
        TrialReplayRestoreVerificationMismatch::PackageClassificationMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "production_classification",
        &package.metadata.production_classification,
        TrialSessionEvidenceProductionClassification::NonProduction.code(),
        TrialReplayRestoreVerificationMismatch::ProductionClassificationMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "distribution_classification",
        &package.metadata.distribution_classification,
        TrialSessionEvidenceDistributionClassification::LocalOnlyNonPublic.code(),
        TrialReplayRestoreVerificationMismatch::DistributionClassificationMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "authority_classification",
        &evidence.metadata.authority_classification,
        TrialSessionEvidenceAuthorityClassification::NonAuthoritativeEvidence.code(),
        TrialReplayRestoreVerificationMismatch::AuthorityClassificationMismatch,
    );

    let included = &package.payload.included_evidence;
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "workflow_status",
        &included.local_beta_workflow_status,
        &evidence.payload.workflow_status_snapshot,
        TrialReplayRestoreVerificationMismatch::WorkflowStatusMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "local_candidate_materialization_status",
        &included.local_candidate_materialization_status,
        &evidence.payload.local_candidate_materialization_snapshot,
        TrialReplayRestoreVerificationMismatch::LocalCandidateMaterializationStatusMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "provider_output_pipeline_status",
        &included.provider_output_pipeline_status,
        &evidence.payload.provider_output_pipeline_snapshot,
        TrialReplayRestoreVerificationMismatch::ProviderOutputPipelineStatusMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "operator_decision_status",
        &included.operator_decision_status,
        &evidence.payload.operator_decision_snapshot,
        TrialReplayRestoreVerificationMismatch::OperatorDecisionStatusMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "replay_status_snapshot",
        &state.run.decision_replay.summary,
        &evidence.payload.replay_status_snapshot,
        TrialReplayRestoreVerificationMismatch::ReplayStatusSnapshotMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "restore_history_snapshot",
        &current_trial_verification_restore_history_snapshot(state),
        &evidence.payload.restore_history_snapshot,
        TrialReplayRestoreVerificationMismatch::RestoreHistorySnapshotMismatch,
    );

    let package_stop_conditions = package
        .payload
        .stop_conditions
        .iter()
        .map(|condition| condition.code().to_string())
        .collect::<Vec<_>>();
    if package_stop_conditions == evidence.payload.stop_condition_snapshot {
        matched_fields.push("stop_condition_snapshot=matched".to_string());
    } else {
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::StopConditionSnapshotMismatch,
        );
    }
    if current_trial_verification_escalation_snapshot(state)
        == evidence.payload.escalation_guidance_snapshot
    {
        matched_fields.push("escalation_snapshot=matched".to_string());
    } else {
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::EscalationSnapshotMismatch,
        );
    }
    if current_trial_verification_failure_snapshot(state)
        == evidence.payload.failure_category_snapshot
    {
        matched_fields.push("failure_category_snapshot=matched".to_string());
    } else {
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::FailureCategorySnapshotMismatch,
        );
    }

    collect_trial_verification_claim_mismatches(
        &format!("{:?} {:?}", package, evidence),
        &mut mismatches,
    );
    mismatches.sort();

    let invalid_input = mismatches.iter().any(|mismatch| {
        matches!(
            mismatch,
            TrialReplayRestoreVerificationMismatch::TrialPackageReadBackInvalid
                | TrialReplayRestoreVerificationMismatch::TrialSessionEvidenceReadBackInvalid
        )
    });
    let passed = mismatches.is_empty();
    projection.status = if passed {
        TrialReplayRestoreVerificationStatus::VerificationPassed
    } else if invalid_input {
        TrialReplayRestoreVerificationStatus::InvalidVerificationInput
    } else {
        TrialReplayRestoreVerificationStatus::VerificationRejected
    };
    projection.verification_id = Some(stable_trial_replay_restore_verification_digest(&format!(
        "package={:?}|evidence={:?}|matched={:?}|mismatches={:?}",
        package.metadata, evidence.metadata, matched_fields, mismatches
    )));
    projection.comparison_summary = TrialReplayRestoreVerificationComparisonSummary {
        package_read_back_status: if package_valid { "package_read_back_valid" } else { "package_read_back_invalid" }.to_string(),
        evidence_read_back_status: if evidence_valid { "evidence_read_back_valid" } else { "evidence_read_back_invalid" }.to_string(),
        package_evidence_linkage_status: if passed { "package_evidence_linkage_matched" } else { "package_evidence_linkage_rejected" }.to_string(),
        workflow_linkage_status: if mismatches.iter().any(|m| matches!(m, TrialReplayRestoreVerificationMismatch::WorkflowStatusMismatch | TrialReplayRestoreVerificationMismatch::LocalCandidateMaterializationStatusMismatch | TrialReplayRestoreVerificationMismatch::ProviderOutputPipelineStatusMismatch | TrialReplayRestoreVerificationMismatch::OperatorDecisionStatusMismatch)) { "workflow_linkage_rejected" } else { "workflow_linkage_matched" }.to_string(),
        replay_status_comparison: if mismatches.contains(&TrialReplayRestoreVerificationMismatch::ReplayStatusSnapshotMismatch) { "replay/status comparison rejected" } else { "replay/status comparison matched" }.to_string(),
        restore_history_comparison: if mismatches.contains(&TrialReplayRestoreVerificationMismatch::RestoreHistorySnapshotMismatch) { "restore/history comparison rejected" } else { "restore/history comparison matched" }.to_string(),
        stop_condition_comparison: if mismatches.contains(&TrialReplayRestoreVerificationMismatch::StopConditionSnapshotMismatch) { "stop_condition_snapshot_rejected" } else { "stop_condition_snapshot_matched" }.to_string(),
        escalation_failure_comparison: if mismatches.iter().any(|m| matches!(m, TrialReplayRestoreVerificationMismatch::EscalationSnapshotMismatch | TrialReplayRestoreVerificationMismatch::FailureCategorySnapshotMismatch)) { "escalation_failure_snapshot_rejected" } else { "escalation_failure_snapshot_matched" }.to_string(),
    };
    projection.matched_fields = matched_fields;
    projection.mismatches = mismatches;
    projection.missing_inputs = missing_inputs;
    projection
}
