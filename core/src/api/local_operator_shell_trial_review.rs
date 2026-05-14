//! Trial evidence review projections and hardening candidate derivation.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrialEvidenceReviewStatus {
    NotReviewed,
    ReviewProjected,
    ReviewWithFindings,
    ReviewBlocked,
    ReviewRejected,
    HardeningCandidatesProjected,
    InvalidReviewInput,
}

impl TrialEvidenceReviewStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotReviewed => "not_reviewed",
            Self::ReviewProjected => "review_projected",
            Self::ReviewWithFindings => "review_with_findings",
            Self::ReviewBlocked => "review_blocked",
            Self::ReviewRejected => "review_rejected",
            Self::HardeningCandidatesProjected => "hardening_candidates_projected",
            Self::InvalidReviewInput => "invalid_review_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrialEvidenceReviewFindingCategory {
    TrialPackage,
    TrialExecution,
    TrialSessionEvidence,
    ReplayRestoreVerification,
    TrialObservability,
    ErrorReporting,
    StopCondition,
    EscalationGuidance,
    WorkflowBlocker,
    ProviderPipeline,
    CandidateMaterialization,
    RestoreHistory,
    ReplayStatus,
    LocalSessionPackage,
    OperatorUsability,
    DocumentationNeeded,
    UserHelpNeeded,
    LocalBetaHardening,
}

impl TrialEvidenceReviewFindingCategory {
    pub fn code(&self) -> &'static str {
        match self {
            Self::TrialPackage => "trial_package",
            Self::TrialExecution => "trial_execution",
            Self::TrialSessionEvidence => "trial_session_evidence",
            Self::ReplayRestoreVerification => "replay_restore_verification",
            Self::TrialObservability => "trial_observability",
            Self::ErrorReporting => "error_reporting",
            Self::StopCondition => "stop_condition",
            Self::EscalationGuidance => "escalation_guidance",
            Self::WorkflowBlocker => "workflow_blocker",
            Self::ProviderPipeline => "provider_pipeline",
            Self::CandidateMaterialization => "candidate_materialization",
            Self::RestoreHistory => "restore_history",
            Self::ReplayStatus => "replay_status",
            Self::LocalSessionPackage => "local_session_package",
            Self::OperatorUsability => "operator_usability",
            Self::DocumentationNeeded => "documentation_needed",
            Self::UserHelpNeeded => "user_help_needed",
            Self::LocalBetaHardening => "local_beta_hardening",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrialEvidenceReviewFindingSeverity {
    Info,
    Warning,
    Blocking,
    StopCondition,
    HardeningRequired,
    InvalidInput,
}

impl TrialEvidenceReviewFindingSeverity {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Blocking => "blocking",
            Self::StopCondition => "stop_condition",
            Self::HardeningRequired => "hardening_required",
            Self::InvalidInput => "invalid_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrialEvidenceReviewFindingDisposition {
    Unresolved,
    RequiresPhase169Hardening,
    RequiresOperatorReview,
    RequiresSecurityReviewerReview,
    RequiresReleaseStewardReview,
    RequiresTrialCoordinatorReview,
    Deferred,
    NotApplicable,
}

impl TrialEvidenceReviewFindingDisposition {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Unresolved => "unresolved",
            Self::RequiresPhase169Hardening => "requires_phase_169_hardening",
            Self::RequiresOperatorReview => "requires_operator_review",
            Self::RequiresSecurityReviewerReview => "requires_security_reviewer_review",
            Self::RequiresReleaseStewardReview => "requires_release_steward_review",
            Self::RequiresTrialCoordinatorReview => "requires_trial_coordinator_review",
            Self::Deferred => "deferred",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrialEvidenceReviewSource {
    ControlledInternalTrialPackage,
    TrialOperatorRunbook,
    TrialFailureDrill,
    TrialSessionEvidence,
    ReplayRestoreVerification,
    ControlledTrialExecutionHarness,
    TrialObservability,
    TrialErrorReport,
    CompleteLocalWorkflow,
    LocalCandidateMaterialization,
    SessionPackage,
    RestoreHistory,
}

impl TrialEvidenceReviewSource {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ControlledInternalTrialPackage => "controlled_internal_trial_package",
            Self::TrialOperatorRunbook => "trial_operator_runbook",
            Self::TrialFailureDrill => "trial_failure_drill",
            Self::TrialSessionEvidence => "trial_session_evidence",
            Self::ReplayRestoreVerification => "replay_restore_verification",
            Self::ControlledTrialExecutionHarness => "controlled_trial_execution_harness",
            Self::TrialObservability => "trial_observability",
            Self::TrialErrorReport => "trial_error_report",
            Self::CompleteLocalWorkflow => "complete_local_workflow",
            Self::LocalCandidateMaterialization => "local_candidate_materialization",
            Self::SessionPackage => "session_package",
            Self::RestoreHistory => "restore_history",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialEvidenceReviewEvidenceLinkage {
    pub source: TrialEvidenceReviewSource,
    pub surface: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialEvidenceReviewFinding {
    pub finding_id: String,
    pub category: TrialEvidenceReviewFindingCategory,
    pub severity: TrialEvidenceReviewFindingSeverity,
    pub disposition: TrialEvidenceReviewFindingDisposition,
    pub source: TrialEvidenceReviewSource,
    pub summary: String,
    pub evidence_linkage: TrialEvidenceReviewEvidenceLinkage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialEvidenceReviewHardeningCandidate {
    pub candidate_id: String,
    pub source_finding_id: String,
    pub target_surface: String,
    pub local_beta_scope: String,
    pub summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialEvidenceReviewBoundaryStatus {
    LocalEvidenceReviewOnly,
    NonPublicReview,
    ReviewNotAuthority,
    NoControlledHumanUseApproval,
    NoReadinessApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoProductionApproval,
    NoProviderTrust,
    NoActionExecution,
    NoAutomatedRemediation,
    NoAutomatedEscalation,
    NoStopConditionAutomation,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl TrialEvidenceReviewBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalEvidenceReviewOnly => "local_evidence_review_only",
            Self::NonPublicReview => "non_public_review",
            Self::ReviewNotAuthority => "review_not_authority",
            Self::NoControlledHumanUseApproval => "no_controlled_human_use_approval",
            Self::NoReadinessApproval => "no_readiness_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoActionExecution => "no_action_execution",
            Self::NoAutomatedRemediation => "no_automated_remediation",
            Self::NoAutomatedEscalation => "no_automated_escalation",
            Self::NoStopConditionAutomation => "no_stop_condition_automation",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialEvidenceReviewCapabilitySurface {
    pub local_only: bool,
    pub non_public: bool,
    pub authoritative: bool,
    pub approves_controlled_human_use: bool,
    pub approves_readiness: bool,
    pub approves_release: bool,
    pub approves_deployment: bool,
    pub approves_public_use: bool,
    pub approves_production: bool,
    pub trusts_provider_output: bool,
    pub executes_actions: bool,
    pub automates_remediation: bool,
    pub automates_escalation: bool,
    pub automates_stop_conditions: bool,
    pub repairs_replay: bool,
    pub promotes_recovery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialEvidenceReviewBlockerSummary {
    pub current_blocker: String,
    pub unresolved_blocker_count: usize,
    pub blockers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialEvidenceReviewMismatchSummary {
    pub mismatch_count: usize,
    pub error_category_count: usize,
    pub replay_status_comparison: String,
    pub restore_history_comparison: String,
    pub package_evidence_read_back_failures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialEvidenceReviewProjection {
    pub status: TrialEvidenceReviewStatus,
    pub review_id: String,
    pub controlled_trial_package_status: String,
    pub trial_execution_status: String,
    pub trial_session_evidence_status: String,
    pub replay_restore_verification_status: String,
    pub observability_status: String,
    pub error_report_status: String,
    pub stop_condition_observation: String,
    pub escalation_guidance_summary: Vec<String>,
    pub evidence_linkage: Vec<TrialEvidenceReviewEvidenceLinkage>,
    pub findings: Vec<TrialEvidenceReviewFinding>,
    pub unresolved_blockers: TrialEvidenceReviewBlockerSummary,
    pub mismatch_error_summary: TrialEvidenceReviewMismatchSummary,
    pub unresolved_finding_count: usize,
    pub hardening_candidates: Vec<TrialEvidenceReviewHardeningCandidate>,
    pub boundary_statuses: Vec<TrialEvidenceReviewBoundaryStatus>,
    pub capability_surface: TrialEvidenceReviewCapabilitySurface,
    pub local_only_non_public_note: String,
    pub no_authority_note: String,
    pub no_automation_note: String,
    pub no_replay_repair_note: String,
    pub hardening_candidate_note: String,
}

pub fn trial_evidence_review_boundary_statuses() -> Vec<TrialEvidenceReviewBoundaryStatus> {
    vec![
        TrialEvidenceReviewBoundaryStatus::LocalEvidenceReviewOnly,
        TrialEvidenceReviewBoundaryStatus::NonPublicReview,
        TrialEvidenceReviewBoundaryStatus::ReviewNotAuthority,
        TrialEvidenceReviewBoundaryStatus::NoControlledHumanUseApproval,
        TrialEvidenceReviewBoundaryStatus::NoReadinessApproval,
        TrialEvidenceReviewBoundaryStatus::NoReleaseApproval,
        TrialEvidenceReviewBoundaryStatus::NoDeploymentApproval,
        TrialEvidenceReviewBoundaryStatus::NoPublicUseApproval,
        TrialEvidenceReviewBoundaryStatus::NoProductionApproval,
        TrialEvidenceReviewBoundaryStatus::NoProviderTrust,
        TrialEvidenceReviewBoundaryStatus::NoActionExecution,
        TrialEvidenceReviewBoundaryStatus::NoAutomatedRemediation,
        TrialEvidenceReviewBoundaryStatus::NoAutomatedEscalation,
        TrialEvidenceReviewBoundaryStatus::NoStopConditionAutomation,
        TrialEvidenceReviewBoundaryStatus::NoReplayRepair,
        TrialEvidenceReviewBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub fn trial_evidence_review_capability_surface() -> TrialEvidenceReviewCapabilitySurface {
    TrialEvidenceReviewCapabilitySurface {
        local_only: true,
        non_public: true,
        authoritative: false,
        approves_controlled_human_use: false,
        approves_readiness: false,
        approves_release: false,
        approves_deployment: false,
        approves_public_use: false,
        approves_production: false,
        trusts_provider_output: false,
        executes_actions: false,
        automates_remediation: false,
        automates_escalation: false,
        automates_stop_conditions: false,
        repairs_replay: false,
        promotes_recovery: false,
    }
}

pub fn initial_trial_evidence_review_projection() -> TrialEvidenceReviewProjection {
    TrialEvidenceReviewProjection {
        status: TrialEvidenceReviewStatus::NotReviewed,
        review_id: "trial-evidence-review-initial".to_string(),
        controlled_trial_package_status: "not_packaged".to_string(),
        trial_execution_status: "not_started".to_string(),
        trial_session_evidence_status: "not_captured".to_string(),
        replay_restore_verification_status: "not_verified".to_string(),
        observability_status: "not_observed".to_string(),
        error_report_status: "no_errors_observed".to_string(),
        stop_condition_observation: "not_observed".to_string(),
        escalation_guidance_summary: Vec::new(),
        evidence_linkage: Vec::new(),
        findings: Vec::new(),
        unresolved_blockers: TrialEvidenceReviewBlockerSummary {
            current_blocker: "not_reviewed".to_string(),
            unresolved_blocker_count: 0,
            blockers: Vec::new(),
        },
        mismatch_error_summary: TrialEvidenceReviewMismatchSummary {
            mismatch_count: 0,
            error_category_count: 0,
            replay_status_comparison: "not_reviewed".to_string(),
            restore_history_comparison: "not_reviewed".to_string(),
            package_evidence_read_back_failures: Vec::new(),
        },
        unresolved_finding_count: 0,
        hardening_candidates: Vec::new(),
        boundary_statuses: trial_evidence_review_boundary_statuses(),
        capability_surface: trial_evidence_review_capability_surface(),
        local_only_non_public_note: "Trial evidence review is local-only and non-public.".to_string(),
        no_authority_note: "Review findings are evidence, not approval. Review does not approve controlled human use, readiness, release, deployment, public use, or production use.".to_string(),
        no_automation_note: "Review does not automate remediation, escalation, or stop-condition enforcement.".to_string(),
        no_replay_repair_note: "Review does not repair replay or promote recovery.".to_string(),
        hardening_candidate_note: "Hardening candidates are inputs for Phase 169 code work, not approvals.".to_string(),
    }
}

fn trial_review_link(
    source: TrialEvidenceReviewSource,
    surface: &str,
    summary: String,
) -> TrialEvidenceReviewEvidenceLinkage {
    TrialEvidenceReviewEvidenceLinkage {
        source,
        surface: surface.to_string(),
        summary,
    }
}

fn trial_review_finding(
    category: TrialEvidenceReviewFindingCategory,
    severity: TrialEvidenceReviewFindingSeverity,
    disposition: TrialEvidenceReviewFindingDisposition,
    source: TrialEvidenceReviewSource,
    summary: String,
    surface: &str,
    linkage_summary: String,
) -> TrialEvidenceReviewFinding {
    TrialEvidenceReviewFinding {
        finding_id: String::new(),
        category,
        severity,
        disposition,
        source,
        summary,
        evidence_linkage: trial_review_link(source, surface, linkage_summary),
    }
}

fn ordered_trial_review_findings(
    mut findings: Vec<TrialEvidenceReviewFinding>,
) -> Vec<TrialEvidenceReviewFinding> {
    findings.sort_by(|a, b| {
        (
            a.severity,
            a.category,
            a.source,
            a.summary.as_str(),
            a.evidence_linkage.surface.as_str(),
        )
            .cmp(&(
                b.severity,
                b.category,
                b.source,
                b.summary.as_str(),
                b.evidence_linkage.surface.as_str(),
            ))
    });
    for (index, finding) in findings.iter_mut().enumerate() {
        finding.finding_id = format!("trial-evidence-review-finding-{:04}", index + 1);
    }
    findings
}

pub fn derive_trial_evidence_review_findings(
    state: &LocalOperatorShellState,
) -> Vec<TrialEvidenceReviewFinding> {
    let mut findings = Vec::new();
    let package = &state.controlled_internal_trial_package_projection;
    if package.status == ControlledInternalTrialPackageStatus::NotPackaged {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::TrialPackage,
            TrialEvidenceReviewFindingSeverity::Blocking,
            TrialEvidenceReviewFindingDisposition::RequiresTrialCoordinatorReview,
            TrialEvidenceReviewSource::ControlledInternalTrialPackage,
            "Controlled internal trial package is missing.".to_string(),
            "controlled_internal_trial_package",
            format!("package_status={}", package.status.code()),
        ));
    }
    if package.validation_status == ControlledInternalTrialPackageValidationStatus::Invalid
        || matches!(package.read_back_validation_status, Some(status) if status != ControlledInternalTrialPackageValidationStatus::Valid)
    {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::TrialPackage,
            TrialEvidenceReviewFindingSeverity::Blocking,
            TrialEvidenceReviewFindingDisposition::RequiresOperatorReview,
            TrialEvidenceReviewSource::ControlledInternalTrialPackage,
            "Controlled internal trial package validation or read-back failed.".to_string(),
            "controlled_internal_trial_package",
            format!(
                "validation={} read_back={}",
                package.validation_status.code(),
                package
                    .read_back_validation_status
                    .map(|s| s.code())
                    .unwrap_or("not_read")
            ),
        ));
    }

    let execution = &state.controlled_internal_trial_execution;
    if execution.status == ControlledInternalTrialRunStatus::NotStarted {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::TrialExecution,
            TrialEvidenceReviewFindingSeverity::Warning,
            TrialEvidenceReviewFindingDisposition::RequiresOperatorReview,
            TrialEvidenceReviewSource::ControlledTrialExecutionHarness,
            "Controlled internal trial execution harness has no trial run.".to_string(),
            "controlled_trial_execution_harness",
            format!("trial_execution_status={}", execution.status.code()),
        ));
    }
    if matches!(
        execution.status,
        ControlledInternalTrialRunStatus::TrialRunRejected
            | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
    ) {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::TrialExecution,
            TrialEvidenceReviewFindingSeverity::Blocking,
            TrialEvidenceReviewFindingDisposition::RequiresTrialCoordinatorReview,
            TrialEvidenceReviewSource::ControlledTrialExecutionHarness,
            format!(
                "Controlled trial run rejected: {}.",
                execution
                    .rejection_reasons
                    .iter()
                    .map(|reason| reason.code())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            "controlled_trial_execution_harness",
            format!("trial_execution_status={}", execution.status.code()),
        ));
    }
    if execution.status == ControlledInternalTrialRunStatus::TrialRunBlocked {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::WorkflowBlocker,
            TrialEvidenceReviewFindingSeverity::Blocking,
            TrialEvidenceReviewFindingDisposition::RequiresOperatorReview,
            TrialEvidenceReviewSource::ControlledTrialExecutionHarness,
            format!(
                "Controlled trial run blocked at {}.",
                execution
                    .current_blocker
                    .map(|blocker| blocker.code())
                    .unwrap_or("unknown")
            ),
            "controlled_trial_execution_harness",
            format!(
                "current_blocker={}",
                execution
                    .current_blocker
                    .map(|blocker| blocker.code())
                    .unwrap_or("none")
            ),
        ));
    }

    let evidence = &state.trial_session_evidence_projection;
    if evidence.status == TrialSessionEvidenceStatus::NotCaptured {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::TrialSessionEvidence,
            TrialEvidenceReviewFindingSeverity::Blocking,
            TrialEvidenceReviewFindingDisposition::RequiresOperatorReview,
            TrialEvidenceReviewSource::TrialSessionEvidence,
            "Trial session evidence is missing.".to_string(),
            "trial_session_evidence",
            format!("evidence_status={}", evidence.status.code()),
        ));
    }
    if evidence.validation_status == TrialSessionEvidenceValidationStatus::Invalid
        || matches!(evidence.read_back_validation_status, Some(status) if status != TrialSessionEvidenceValidationStatus::Valid)
    {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::TrialSessionEvidence,
            TrialEvidenceReviewFindingSeverity::Blocking,
            TrialEvidenceReviewFindingDisposition::RequiresOperatorReview,
            TrialEvidenceReviewSource::TrialSessionEvidence,
            "Trial session evidence validation or read-back failed.".to_string(),
            "trial_session_evidence",
            format!(
                "validation={} read_back={}",
                evidence.validation_status.code(),
                evidence
                    .read_back_validation_status
                    .map(|s| s.code())
                    .unwrap_or("not_read")
            ),
        ));
    }

    let verification = &state.trial_replay_restore_verification;
    if verification.status == TrialReplayRestoreVerificationStatus::NotVerified {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::ReplayRestoreVerification,
            TrialEvidenceReviewFindingSeverity::Blocking,
            TrialEvidenceReviewFindingDisposition::RequiresOperatorReview,
            TrialEvidenceReviewSource::ReplayRestoreVerification,
            "Replay/restore verification is missing.".to_string(),
            "replay_restore_verification",
            format!("verification_status={}", verification.status.code()),
        ));
    }
    if matches!(
        verification.status,
        TrialReplayRestoreVerificationStatus::VerificationRejected
    ) {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::ReplayRestoreVerification,
            TrialEvidenceReviewFindingSeverity::Blocking,
            TrialEvidenceReviewFindingDisposition::RequiresOperatorReview,
            TrialEvidenceReviewSource::ReplayRestoreVerification,
            "Replay/restore verification did not pass.".to_string(),
            "replay_restore_verification",
            format!(
                "verification_status={} mismatches={}",
                verification.status.code(),
                verification
                    .mismatches
                    .iter()
                    .map(|m| m.code())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        ));
    }
    for mismatch in &verification.mismatches {
        let category = match mismatch {
            TrialReplayRestoreVerificationMismatch::ReplayStatusSnapshotMismatch => {
                TrialEvidenceReviewFindingCategory::ReplayStatus
            }
            TrialReplayRestoreVerificationMismatch::RestoreHistorySnapshotMismatch => {
                TrialEvidenceReviewFindingCategory::RestoreHistory
            }
            TrialReplayRestoreVerificationMismatch::TrialPackageReadBackInvalid => {
                TrialEvidenceReviewFindingCategory::TrialPackage
            }
            TrialReplayRestoreVerificationMismatch::TrialSessionEvidenceReadBackInvalid => {
                TrialEvidenceReviewFindingCategory::TrialSessionEvidence
            }
            _ => TrialEvidenceReviewFindingCategory::ReplayRestoreVerification,
        };
        findings.push(trial_review_finding(
            category,
            TrialEvidenceReviewFindingSeverity::Blocking,
            TrialEvidenceReviewFindingDisposition::RequiresOperatorReview,
            TrialEvidenceReviewSource::ReplayRestoreVerification,
            format!(
                "Replay/restore verification mismatch observed: {}.",
                mismatch.code()
            ),
            "replay_restore_verification",
            format!("mismatch={}", mismatch.code()),
        ));
    }

    if state
        .trial_observability
        .stop_condition_observation
        .observed
    {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::StopCondition,
            TrialEvidenceReviewFindingSeverity::StopCondition,
            TrialEvidenceReviewFindingDisposition::RequiresSecurityReviewerReview,
            TrialEvidenceReviewSource::TrialObservability,
            "Stop-condition observation is present for operator review.".to_string(),
            "trial_observability",
            state
                .trial_observability
                .stop_condition_observation
                .status
                .clone(),
        ));
    }
    for detail in &state.trial_error_report.details {
        let category = match detail.category {
            TrialErrorCategory::StopConditionObserved => {
                TrialEvidenceReviewFindingCategory::StopCondition
            }
            TrialErrorCategory::ReplayStatusMismatch => {
                TrialEvidenceReviewFindingCategory::ReplayStatus
            }
            TrialErrorCategory::RestoreHistoryMismatch => {
                TrialEvidenceReviewFindingCategory::RestoreHistory
            }
            TrialErrorCategory::MaterializationMissing => {
                TrialEvidenceReviewFindingCategory::CandidateMaterialization
            }
            TrialErrorCategory::ProviderPipelineBlocked
            | TrialErrorCategory::ProviderOutputValidationRejected => {
                TrialEvidenceReviewFindingCategory::ProviderPipeline
            }
            TrialErrorCategory::WorkflowBlocked | TrialErrorCategory::WorkflowRejected => {
                TrialEvidenceReviewFindingCategory::WorkflowBlocker
            }
            TrialErrorCategory::PackageMissing
            | TrialErrorCategory::PackageValidationFailed
            | TrialErrorCategory::PackageReadBackFailed => {
                TrialEvidenceReviewFindingCategory::TrialPackage
            }
            TrialErrorCategory::EvidenceMissing
            | TrialErrorCategory::EvidenceValidationFailed
            | TrialErrorCategory::EvidenceReadBackFailed => {
                TrialEvidenceReviewFindingCategory::TrialSessionEvidence
            }
            _ => TrialEvidenceReviewFindingCategory::ErrorReporting,
        };
        let severity = match detail.severity {
            TrialErrorSeverity::Info => TrialEvidenceReviewFindingSeverity::Info,
            TrialErrorSeverity::Warning => TrialEvidenceReviewFindingSeverity::Warning,
            TrialErrorSeverity::Blocking => TrialEvidenceReviewFindingSeverity::Blocking,
            TrialErrorSeverity::StopCondition => TrialEvidenceReviewFindingSeverity::StopCondition,
            TrialErrorSeverity::InvalidInput => TrialEvidenceReviewFindingSeverity::InvalidInput,
        };
        findings.push(trial_review_finding(
            category,
            severity,
            TrialEvidenceReviewFindingDisposition::RequiresOperatorReview,
            TrialEvidenceReviewSource::TrialErrorReport,
            detail.summary.clone(),
            "trial_error_report",
            detail.evidence_linkage.linkage.clone(),
        ));
    }
    if state
        .complete_local_operator_workflow
        .current_blocking_step
        .is_some()
    {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::WorkflowBlocker,
            TrialEvidenceReviewFindingSeverity::Blocking,
            TrialEvidenceReviewFindingDisposition::RequiresOperatorReview,
            TrialEvidenceReviewSource::CompleteLocalWorkflow,
            format!(
                "Complete local workflow blocker: {}.",
                state
                    .complete_local_operator_workflow
                    .current_blocking_step
                    .map(|step| step.code())
                    .unwrap_or("unknown")
            ),
            "complete_local_workflow",
            format!(
                "workflow_status={}",
                state.complete_local_operator_workflow.status.code()
            ),
        ));
    }
    if state.local_candidate_output.status == LocalCandidateMaterializationStatus::NotMaterialized {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::CandidateMaterialization,
            TrialEvidenceReviewFindingSeverity::Warning,
            TrialEvidenceReviewFindingDisposition::RequiresPhase169Hardening,
            TrialEvidenceReviewSource::LocalCandidateMaterialization,
            "Local candidate materialization is missing from the trial evidence chain.".to_string(),
            "local_candidate_materialization",
            format!(
                "materialization_status={}",
                state.local_candidate_output.status.code()
            ),
        ));
    }
    findings.push(trial_review_finding(
        TrialEvidenceReviewFindingCategory::UserHelpNeeded,
        TrialEvidenceReviewFindingSeverity::HardeningRequired,
        TrialEvidenceReviewFindingDisposition::RequiresPhase169Hardening,
        TrialEvidenceReviewSource::TrialOperatorRunbook,
        "User-facing local help documentation is not yet represented in the trial review surface."
            .to_string(),
        "trial_operator_runbook",
        "Phase 169 owns user-facing local HTML help if still missing.".to_string(),
    ));
    for guidance in &state.trial_failure_drill.escalation_guidance {
        findings.push(trial_review_finding(
            TrialEvidenceReviewFindingCategory::EscalationGuidance,
            TrialEvidenceReviewFindingSeverity::Info,
            TrialEvidenceReviewFindingDisposition::RequiresTrialCoordinatorReview,
            TrialEvidenceReviewSource::TrialFailureDrill,
            format!(
                "Escalation guidance remains descriptive for role {}.",
                guidance.role.code()
            ),
            "trial_failure_drill",
            guidance
                .categories
                .iter()
                .map(|category| category.code())
                .collect::<Vec<_>>()
                .join(","),
        ));
    }
    ordered_trial_review_findings(findings)
}

pub fn derive_trial_hardening_candidates(
    findings: &[TrialEvidenceReviewFinding],
) -> Vec<TrialEvidenceReviewHardeningCandidate> {
    let mut candidates = findings
        .iter()
        .filter(|finding| {
            matches!(
                finding.disposition,
                TrialEvidenceReviewFindingDisposition::RequiresPhase169Hardening
            ) || matches!(
                finding.severity,
                TrialEvidenceReviewFindingSeverity::Blocking
                    | TrialEvidenceReviewFindingSeverity::HardeningRequired
            )
        })
        .map(|finding| TrialEvidenceReviewHardeningCandidate {
            candidate_id: String::new(),
            source_finding_id: finding.finding_id.clone(),
            target_surface: finding.evidence_linkage.surface.clone(),
            local_beta_scope: "local_beta_only_phase_169_input".to_string(),
            summary: format!(
                "Phase 169 local-beta hardening candidate from {}: {}",
                finding.category.code(),
                finding.summary
            ),
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|a, b| {
        (
            a.target_surface.as_str(),
            a.source_finding_id.as_str(),
            a.summary.as_str(),
        )
            .cmp(&(
                b.target_surface.as_str(),
                b.source_finding_id.as_str(),
                b.summary.as_str(),
            ))
    });
    for (index, candidate) in candidates.iter_mut().enumerate() {
        candidate.candidate_id = format!("trial-hardening-candidate-{:04}", index + 1);
    }
    candidates
}

pub fn derive_trial_evidence_review_projection(
    state: &LocalOperatorShellState,
) -> TrialEvidenceReviewProjection {
    let findings = derive_trial_evidence_review_findings(state);
    let hardening_candidates = derive_trial_hardening_candidates(&findings);
    let blockers = findings
        .iter()
        .filter(|finding| {
            matches!(
                finding.severity,
                TrialEvidenceReviewFindingSeverity::Blocking
                    | TrialEvidenceReviewFindingSeverity::StopCondition
                    | TrialEvidenceReviewFindingSeverity::InvalidInput
            )
        })
        .map(|finding| format!("{}:{}", finding.finding_id, finding.summary))
        .collect::<Vec<_>>();
    let evidence_linkage = vec![
        trial_review_link(
            TrialEvidenceReviewSource::ControlledInternalTrialPackage,
            "controlled_internal_trial_package",
            format!(
                "package_status={} package_id={}",
                state
                    .controlled_internal_trial_package_projection
                    .status
                    .code(),
                state
                    .controlled_internal_trial_package_projection
                    .package_id
                    .as_deref()
                    .unwrap_or("none")
            ),
        ),
        trial_review_link(
            TrialEvidenceReviewSource::ControlledTrialExecutionHarness,
            "controlled_trial_execution_harness",
            format!(
                "trial_execution_status={}",
                state.controlled_internal_trial_execution.status.code()
            ),
        ),
        trial_review_link(
            TrialEvidenceReviewSource::TrialSessionEvidence,
            "trial_session_evidence",
            format!(
                "evidence_status={} evidence_id={}",
                state.trial_session_evidence_projection.status.code(),
                state
                    .trial_session_evidence_projection
                    .evidence_id
                    .as_deref()
                    .unwrap_or("none")
            ),
        ),
        trial_review_link(
            TrialEvidenceReviewSource::ReplayRestoreVerification,
            "replay_restore_verification",
            format!(
                "verification_status={}",
                state.trial_replay_restore_verification.status.code()
            ),
        ),
        trial_review_link(
            TrialEvidenceReviewSource::TrialObservability,
            "trial_observability",
            format!(
                "observability_status={}",
                state.trial_observability.status.code()
            ),
        ),
        trial_review_link(
            TrialEvidenceReviewSource::TrialErrorReport,
            "trial_error_report",
            format!(
                "error_report_status={} categories={}",
                state.trial_error_report.status.code(),
                state.trial_error_report.category_summary.len()
            ),
        ),
    ];
    let status = if findings
        .iter()
        .any(|f| f.severity == TrialEvidenceReviewFindingSeverity::InvalidInput)
    {
        TrialEvidenceReviewStatus::InvalidReviewInput
    } else if findings
        .iter()
        .any(|f| f.severity == TrialEvidenceReviewFindingSeverity::StopCondition)
    {
        TrialEvidenceReviewStatus::ReviewBlocked
    } else if matches!(
        state.controlled_internal_trial_execution.status,
        ControlledInternalTrialRunStatus::TrialRunRejected
            | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
    ) {
        TrialEvidenceReviewStatus::ReviewRejected
    } else if !hardening_candidates.is_empty() {
        TrialEvidenceReviewStatus::HardeningCandidatesProjected
    } else if !findings.is_empty() {
        TrialEvidenceReviewStatus::ReviewWithFindings
    } else {
        TrialEvidenceReviewStatus::ReviewProjected
    };
    TrialEvidenceReviewProjection {
        status,
        review_id: format!("trial-evidence-review-findings-{:04}-hardening-{:04}", findings.len(), hardening_candidates.len()),
        controlled_trial_package_status: state.controlled_internal_trial_package_projection.status.code().to_string(),
        trial_execution_status: state.controlled_internal_trial_execution.status.code().to_string(),
        trial_session_evidence_status: state.trial_session_evidence_projection.status.code().to_string(),
        replay_restore_verification_status: state.trial_replay_restore_verification.status.code().to_string(),
        observability_status: state.trial_observability.status.code().to_string(),
        error_report_status: state.trial_error_report.status.code().to_string(),
        stop_condition_observation: state.trial_observability.stop_condition_observation.status.clone(),
        escalation_guidance_summary: state.trial_failure_drill.escalation_guidance.iter().map(|guidance| format!("{}:{}", guidance.role.code(), guidance.categories.iter().map(|category| category.code()).collect::<Vec<_>>().join("/"))).collect(),
        evidence_linkage,
        unresolved_finding_count: findings.iter().filter(|finding| finding.disposition != TrialEvidenceReviewFindingDisposition::NotApplicable).count(),
        unresolved_blockers: TrialEvidenceReviewBlockerSummary {
            current_blocker: state.trial_observability.current_blocker.clone(),
            unresolved_blocker_count: blockers.len(),
            blockers,
        },
        mismatch_error_summary: TrialEvidenceReviewMismatchSummary {
            mismatch_count: state.trial_observability.mismatch_summary.mismatches.len(),
            error_category_count: state.trial_error_report.category_summary.len(),
            replay_status_comparison: state.trial_observability.replay_status_comparison_summary.clone(),
            restore_history_comparison: state.trial_observability.restore_history_comparison_summary.clone(),
            package_evidence_read_back_failures: state.trial_observability.package_evidence_read_back_failure_summary.clone(),
        },
        findings,
        hardening_candidates,
        boundary_statuses: trial_evidence_review_boundary_statuses(),
        capability_surface: trial_evidence_review_capability_surface(),
        local_only_non_public_note: "Trial evidence review is local-only and non-public.".to_string(),
        no_authority_note: "Review findings are evidence, not approval. Review does not approve controlled human use, readiness, release, deployment, public use, or production use.".to_string(),
        no_automation_note: "Review does not automate remediation, escalation, or stop-condition enforcement.".to_string(),
        no_replay_repair_note: "Review does not repair replay or promote recovery.".to_string(),
        hardening_candidate_note: "Hardening candidates are inputs for Phase 169 code work, not approvals.".to_string(),
    }
}

pub fn refresh_trial_evidence_review_state(
    mut state: LocalOperatorShellState,
) -> LocalOperatorShellState {
    state.trial_evidence_review = derive_trial_evidence_review_projection(&state);
    state
}
