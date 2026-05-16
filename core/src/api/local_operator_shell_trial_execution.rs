//! Controlled internal trial execution projections and helpers.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialRunStatus {
    NotStarted,
    PreconditionsMissing,
    ReadyForBoundedLocalTrialRun,
    TrialRunStarted,
    TrialRunInProgress,
    TrialRunBlocked,
    TrialRunCompletedLocally,
    TrialRunRejected,
    InvalidTrialRunRequest,
}

impl ControlledInternalTrialRunStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::PreconditionsMissing => "preconditions_missing",
            Self::ReadyForBoundedLocalTrialRun => "ready_for_bounded_local_trial_run",
            Self::TrialRunStarted => "trial_run_started",
            Self::TrialRunInProgress => "trial_run_in_progress",
            Self::TrialRunBlocked => "trial_run_blocked",
            Self::TrialRunCompletedLocally => "trial_run_completed_locally",
            Self::TrialRunRejected => "trial_run_rejected",
            Self::InvalidTrialRunRequest => "invalid_trial_run_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialRunStep {
    VerifyTrialPackage,
    VerifyRunbook,
    VerifyFailureDrill,
    VerifyTrialSessionEvidence,
    VerifyReplayRestore,
    VerifyCompleteLocalWorkflow,
    ObserveStopConditions,
    RecordManualOperatorStep,
    ProjectTrialRunSummary,
    ProjectTrialEvidenceLinkage,
    CloseLocalTrialRun,
}

impl ControlledInternalTrialRunStep {
    pub fn code(&self) -> &'static str {
        match self {
            Self::VerifyTrialPackage => "verify_trial_package",
            Self::VerifyRunbook => "verify_runbook",
            Self::VerifyFailureDrill => "verify_failure_drill",
            Self::VerifyTrialSessionEvidence => "verify_trial_session_evidence",
            Self::VerifyReplayRestore => "verify_replay_restore",
            Self::VerifyCompleteLocalWorkflow => "verify_complete_local_workflow",
            Self::ObserveStopConditions => "observe_stop_conditions",
            Self::RecordManualOperatorStep => "record_manual_operator_step",
            Self::ProjectTrialRunSummary => "project_trial_run_summary",
            Self::ProjectTrialEvidenceLinkage => "project_trial_evidence_linkage",
            Self::CloseLocalTrialRun => "close_local_trial_run",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialRunStepStatus {
    NotStarted,
    Available,
    Completed,
    Blocked,
    Rejected,
    ManualActionRequired,
    NotApplicable,
}

impl ControlledInternalTrialRunStepStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Available => "available",
            Self::Completed => "completed",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
            Self::ManualActionRequired => "manual_action_required",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialRunError {
    TrialPackageMissing,
    TrialPackageInvalid,
    RunbookMissing,
    RunbookBlocked,
    FailureDrillMissing,
    TrialSessionEvidenceMissing,
    TrialSessionEvidenceInvalid,
    ReplayRestoreVerificationMissing,
    ReplayRestoreVerificationRejected,
    ReplayRestoreVerificationNotPassed,
    CompleteLocalWorkflowMissing,
    CompleteLocalWorkflowBlocked,
    CompleteLocalWorkflowRejected,
    StopConditionObserved,
    ManualOperatorStepMissing,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    ProductionUseClaimRejected,
    ProviderTrustClaimRejected,
    ActionAuthorizationClaimRejected,
    ReplayRepairClaimRejected,
    RecoveryPromotionClaimRejected,
    SigningClaimRejected,
    PublishingClaimRejected,
    ReleaseArtifactClaimRejected,
}

impl ControlledInternalTrialRunError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::TrialPackageMissing => "trial_package_missing",
            Self::TrialPackageInvalid => "trial_package_invalid",
            Self::RunbookMissing => "runbook_missing",
            Self::RunbookBlocked => "runbook_blocked",
            Self::FailureDrillMissing => "failure_drill_missing",
            Self::TrialSessionEvidenceMissing => "trial_session_evidence_missing",
            Self::TrialSessionEvidenceInvalid => "trial_session_evidence_invalid",
            Self::ReplayRestoreVerificationMissing => "replay_restore_verification_missing",
            Self::ReplayRestoreVerificationRejected => "replay_restore_verification_rejected",
            Self::ReplayRestoreVerificationNotPassed => "replay_restore_verification_not_passed",
            Self::CompleteLocalWorkflowMissing => "complete_local_workflow_missing",
            Self::CompleteLocalWorkflowBlocked => "complete_local_workflow_blocked",
            Self::CompleteLocalWorkflowRejected => "complete_local_workflow_rejected",
            Self::StopConditionObserved => "stop_condition_observed",
            Self::ManualOperatorStepMissing => "manual_operator_step_missing",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::ProductionUseClaimRejected => "production_use_claim_rejected",
            Self::ProviderTrustClaimRejected => "provider_trust_claim_rejected",
            Self::ActionAuthorizationClaimRejected => "action_authorization_claim_rejected",
            Self::ReplayRepairClaimRejected => "replay_repair_claim_rejected",
            Self::RecoveryPromotionClaimRejected => "recovery_promotion_claim_rejected",
            Self::SigningClaimRejected => "signing_claim_rejected",
            Self::PublishingClaimRejected => "publishing_claim_rejected",
            Self::ReleaseArtifactClaimRejected => "release_artifact_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialManualStepStatus {
    NotStarted,
    ManualActionRequired,
    Recorded,
    Missing,
}

impl ControlledInternalTrialManualStepStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::ManualActionRequired => "manual_action_required",
            Self::Recorded => "recorded",
            Self::Missing => "manual_operator_step_missing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialExecutionBoundaryStatus {
    ControlledInternalTrialHarnessOnly,
    LocalTrialExecutionOnly,
    NonPublicTrialExecution,
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
    NoStopConditionAutomation,
    NoAutomatedEscalation,
}

impl ControlledInternalTrialExecutionBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ControlledInternalTrialHarnessOnly => "controlled_internal_trial_harness_only",
            Self::LocalTrialExecutionOnly => "local_trial_execution_only",
            Self::NonPublicTrialExecution => "non_public_trial_execution",
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
            Self::NoStopConditionAutomation => "no_stop_condition_automation",
            Self::NoAutomatedEscalation => "no_automated_escalation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialExecutionRequest {
    pub operator_step_recorded: bool,
    pub stop_condition_observed: bool,
    pub claims_readiness: bool,
    pub claims_release: bool,
    pub claims_deployment: bool,
    pub claims_public_use: bool,
    pub claims_production_use: bool,
    pub claims_provider_trust: bool,
    pub claims_action_authorization: bool,
    pub claims_replay_repair: bool,
    pub claims_recovery_promotion: bool,
    pub claims_signing: bool,
    pub claims_publishing: bool,
    pub claims_release_artifact: bool,
}

impl ControlledInternalTrialExecutionRequest {
    pub fn bounded_local_trial_run() -> Self {
        Self {
            operator_step_recorded: false,
            stop_condition_observed: false,
            claims_readiness: false,
            claims_release: false,
            claims_deployment: false,
            claims_public_use: false,
            claims_production_use: false,
            claims_provider_trust: false,
            claims_action_authorization: false,
            claims_replay_repair: false,
            claims_recovery_promotion: false,
            claims_signing: false,
            claims_publishing: false,
            claims_release_artifact: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialRunStepProjection {
    pub step: ControlledInternalTrialRunStep,
    pub status: ControlledInternalTrialRunStepStatus,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialStopObservation {
    pub status: String,
    pub observed: bool,
    pub markers: Vec<String>,
    pub enforcement_automated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialEvidenceLinkage {
    pub trial_package: String,
    pub runbook: String,
    pub failure_drill: String,
    pub trial_session_evidence: String,
    pub replay_restore_verification: String,
    pub local_workflow: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialExecutionCapabilitySurface {
    pub local_only: bool,
    pub non_public: bool,
    pub approves_controlled_human_use: bool,
    pub approves_readiness: bool,
    pub approves_release: bool,
    pub approves_deployment: bool,
    pub approves_public_use: bool,
    pub approves_production: bool,
    pub trusts_provider_output: bool,
    pub authorizes_actions: bool,
    pub repairs_replay: bool,
    pub promotes_recovery: bool,
    pub automates_stop_conditions: bool,
    pub automates_escalation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialRunProjection {
    pub run_id: String,
    pub status: ControlledInternalTrialRunStatus,
    pub current_step: Option<ControlledInternalTrialRunStep>,
    pub next_step: Option<ControlledInternalTrialRunStep>,
    pub steps: Vec<ControlledInternalTrialRunStepProjection>,
    pub current_blocker: Option<ControlledInternalTrialRunError>,
    pub rejection_reasons: Vec<ControlledInternalTrialRunError>,
    pub stop_condition_observation: ControlledInternalTrialStopObservation,
    pub manual_operator_step_status: ControlledInternalTrialManualStepStatus,
    pub evidence_linkage: ControlledInternalTrialEvidenceLinkage,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialExecutionProjection {
    pub status: ControlledInternalTrialRunStatus,
    pub active_run: Option<ControlledInternalTrialRunProjection>,
    pub last_rejected_run: Option<ControlledInternalTrialRunProjection>,
    pub precondition_status: Vec<String>,
    pub current_blocker: Option<ControlledInternalTrialRunError>,
    pub rejection_reasons: Vec<ControlledInternalTrialRunError>,
    pub evidence_linkage: ControlledInternalTrialEvidenceLinkage,
    pub boundary_statuses: Vec<ControlledInternalTrialExecutionBoundaryStatus>,
    pub capability_surface: ControlledInternalTrialExecutionCapabilitySurface,
    pub local_only_non_public_note: String,
    pub no_controlled_human_use_note: String,
    pub no_readiness_release_deployment_public_production_note: String,
    pub stop_condition_note: String,
    pub escalation_note: String,
    pub no_action_authorization_note: String,
}

fn empty_controlled_internal_trial_execution_linkage() -> ControlledInternalTrialEvidenceLinkage {
    ControlledInternalTrialEvidenceLinkage {
        trial_package: "trial_package=missing".to_string(),
        runbook: "runbook=missing".to_string(),
        failure_drill: "failure_drill=missing".to_string(),
        trial_session_evidence: "trial_session_evidence=missing".to_string(),
        replay_restore_verification: "replay_restore_verification=missing".to_string(),
        local_workflow: "local_workflow=missing".to_string(),
    }
}

pub fn initial_controlled_internal_trial_execution_projection(
) -> ControlledInternalTrialExecutionProjection {
    ControlledInternalTrialExecutionProjection {
        status: ControlledInternalTrialRunStatus::NotStarted,
        active_run: None,
        last_rejected_run: None,
        precondition_status: Vec::new(),
        current_blocker: None,
        rejection_reasons: Vec::new(),
        evidence_linkage: empty_controlled_internal_trial_execution_linkage(),
        boundary_statuses: controlled_internal_trial_execution_boundary_statuses(),
        capability_surface: controlled_internal_trial_execution_capability_surface(),
        local_only_non_public_note: "Controlled internal trial execution harness is local-only and non-public.".to_string(),
        no_controlled_human_use_note: "The harness does not approve controlled human use.".to_string(),
        no_readiness_release_deployment_public_production_note: "The harness does not approve readiness, release, deployment, public use, or production use.".to_string(),
        stop_condition_note: "Stop conditions are observed only; enforcement is not automated in Phase 166.".to_string(),
        escalation_note: "Escalation is not automated.".to_string(),
        no_action_authorization_note: "No action authorization is granted.".to_string(),
    }
}

fn controlled_internal_trial_execution_evidence_linkage(
    state: &LocalOperatorShellState,
) -> ControlledInternalTrialEvidenceLinkage {
    ControlledInternalTrialEvidenceLinkage {
        trial_package: format!(
            "package={} status={} validation={}",
            state
                .controlled_internal_trial_package_projection
                .package_id
                .as_deref()
                .unwrap_or("none"),
            state
                .controlled_internal_trial_package_projection
                .status
                .code(),
            state
                .controlled_internal_trial_package_projection
                .validation_status
                .code()
        ),
        runbook: format!(
            "runbook_status={}",
            state.trial_operator_runbook.status.code()
        ),
        failure_drill: format!(
            "failure_drill_status={} highest_severity={}",
            state.trial_failure_drill.status.code(),
            state.trial_failure_drill.highest_severity.code()
        ),
        trial_session_evidence: format!(
            "evidence={} status={} validation={}",
            state
                .trial_session_evidence_projection
                .evidence_id
                .as_deref()
                .unwrap_or("none"),
            state.trial_session_evidence_projection.status.code(),
            state
                .trial_session_evidence_projection
                .validation_status
                .code()
        ),
        replay_restore_verification: format!(
            "verification={} status={}",
            state
                .trial_replay_restore_verification
                .verification_id
                .as_deref()
                .unwrap_or("none"),
            state.trial_replay_restore_verification.status.code()
        ),
        local_workflow: format!(
            "workflow_status={}",
            state.complete_local_operator_workflow.status.code()
        ),
    }
}

fn controlled_internal_trial_execution_claim_errors(
    request: &ControlledInternalTrialExecutionRequest,
) -> Vec<ControlledInternalTrialRunError> {
    let mut errors = Vec::new();
    if request.claims_readiness {
        errors.push(ControlledInternalTrialRunError::ReadinessClaimRejected);
    }
    if request.claims_release {
        errors.push(ControlledInternalTrialRunError::ReleaseClaimRejected);
    }
    if request.claims_deployment {
        errors.push(ControlledInternalTrialRunError::DeploymentClaimRejected);
    }
    if request.claims_public_use {
        errors.push(ControlledInternalTrialRunError::PublicUseClaimRejected);
    }
    if request.claims_production_use {
        errors.push(ControlledInternalTrialRunError::ProductionUseClaimRejected);
    }
    if request.claims_provider_trust {
        errors.push(ControlledInternalTrialRunError::ProviderTrustClaimRejected);
    }
    if request.claims_action_authorization {
        errors.push(ControlledInternalTrialRunError::ActionAuthorizationClaimRejected);
    }
    if request.claims_replay_repair {
        errors.push(ControlledInternalTrialRunError::ReplayRepairClaimRejected);
    }
    if request.claims_recovery_promotion {
        errors.push(ControlledInternalTrialRunError::RecoveryPromotionClaimRejected);
    }
    if request.claims_signing {
        errors.push(ControlledInternalTrialRunError::SigningClaimRejected);
    }
    if request.claims_publishing {
        errors.push(ControlledInternalTrialRunError::PublishingClaimRejected);
    }
    if request.claims_release_artifact {
        errors.push(ControlledInternalTrialRunError::ReleaseArtifactClaimRejected);
    }
    errors
}

pub fn validate_controlled_internal_trial_execution_preconditions(
    state: &LocalOperatorShellState,
) -> Vec<ControlledInternalTrialRunError> {
    let mut errors = Vec::new();
    let package = &state.controlled_internal_trial_package_projection;
    if package.status == ControlledInternalTrialPackageStatus::NotPackaged {
        errors.push(ControlledInternalTrialRunError::TrialPackageMissing);
    } else if trial_package_is_invalid(package)
        || package.validation_status != ControlledInternalTrialPackageValidationStatus::Valid
    {
        errors.push(ControlledInternalTrialRunError::TrialPackageInvalid);
    }

    match state.trial_operator_runbook.status {
        TrialOperatorRunbookStatus::NotAvailable => {
            errors.push(ControlledInternalTrialRunError::RunbookMissing)
        }
        TrialOperatorRunbookStatus::TrialPackageRequired | TrialOperatorRunbookStatus::Blocked => {
            errors.push(ControlledInternalTrialRunError::RunbookBlocked)
        }
        _ => {}
    }

    if state.trial_failure_drill.stop_condition_drills.is_empty()
        && state.trial_failure_drill.categories.is_empty()
    {
        errors.push(ControlledInternalTrialRunError::FailureDrillMissing);
    }

    match state.trial_session_evidence_projection.status {
        TrialSessionEvidenceStatus::NotCaptured => {
            errors.push(ControlledInternalTrialRunError::TrialSessionEvidenceMissing)
        }
        TrialSessionEvidenceStatus::EvidenceRejected
        | TrialSessionEvidenceStatus::InvalidEvidenceInput => {
            errors.push(ControlledInternalTrialRunError::TrialSessionEvidenceInvalid)
        }
        _ => {}
    }
    if state.trial_session_evidence_projection.validation_status
        != TrialSessionEvidenceValidationStatus::Valid
        && state
            .trial_session_evidence_projection
            .read_back_validation_status
            != Some(TrialSessionEvidenceValidationStatus::Valid)
        && !errors.contains(&ControlledInternalTrialRunError::TrialSessionEvidenceMissing)
    {
        errors.push(ControlledInternalTrialRunError::TrialSessionEvidenceInvalid);
    }

    match state.trial_replay_restore_verification.status {
        TrialReplayRestoreVerificationStatus::NotVerified => {
            errors.push(ControlledInternalTrialRunError::ReplayRestoreVerificationMissing)
        }
        TrialReplayRestoreVerificationStatus::VerificationRejected
        | TrialReplayRestoreVerificationStatus::InvalidVerificationInput => {
            errors.push(ControlledInternalTrialRunError::ReplayRestoreVerificationRejected)
        }
        TrialReplayRestoreVerificationStatus::VerificationInputMissing => {
            errors.push(ControlledInternalTrialRunError::ReplayRestoreVerificationMissing)
        }
        TrialReplayRestoreVerificationStatus::VerificationPassed => {}
        TrialReplayRestoreVerificationStatus::VerificationProjected => {
            errors.push(ControlledInternalTrialRunError::ReplayRestoreVerificationNotPassed)
        }
    }

    match state.complete_local_operator_workflow.status {
        CompleteLocalOperatorWorkflowStatus::NotStarted => {
            errors.push(ControlledInternalTrialRunError::CompleteLocalWorkflowMissing)
        }
        CompleteLocalOperatorWorkflowStatus::Blocked => {
            errors.push(ControlledInternalTrialRunError::CompleteLocalWorkflowBlocked)
        }
        CompleteLocalOperatorWorkflowStatus::Rejected => {
            errors.push(ControlledInternalTrialRunError::CompleteLocalWorkflowRejected)
        }
        CompleteLocalOperatorWorkflowStatus::InProgress => {
            errors.push(ControlledInternalTrialRunError::CompleteLocalWorkflowMissing)
        }
        CompleteLocalOperatorWorkflowStatus::CompleteLocalWorkflowProjected
        | CompleteLocalOperatorWorkflowStatus::LocalCandidateMaterialized => {}
    }
    errors
}

fn controlled_internal_trial_execution_precondition_status(
    state: &LocalOperatorShellState,
    errors: &[ControlledInternalTrialRunError],
) -> Vec<String> {
    vec![
        format!(
            "trial_package={}",
            state
                .controlled_internal_trial_package_projection
                .status
                .code()
        ),
        format!("runbook={}", state.trial_operator_runbook.status.code()),
        format!("failure_drill={}", state.trial_failure_drill.status.code()),
        format!(
            "trial_session_evidence={}",
            state.trial_session_evidence_projection.status.code()
        ),
        format!(
            "replay_restore_verification={}",
            state.trial_replay_restore_verification.status.code()
        ),
        format!(
            "local_workflow={}",
            state.complete_local_operator_workflow.status.code()
        ),
        format!(
            "precondition_errors={}",
            errors
                .iter()
                .map(|e| e.code())
                .collect::<Vec<_>>()
                .join(",")
        ),
    ]
}

fn controlled_internal_trial_step_order() -> Vec<ControlledInternalTrialRunStep> {
    vec![
        ControlledInternalTrialRunStep::VerifyTrialPackage,
        ControlledInternalTrialRunStep::VerifyRunbook,
        ControlledInternalTrialRunStep::VerifyFailureDrill,
        ControlledInternalTrialRunStep::VerifyTrialSessionEvidence,
        ControlledInternalTrialRunStep::VerifyReplayRestore,
        ControlledInternalTrialRunStep::VerifyCompleteLocalWorkflow,
        ControlledInternalTrialRunStep::ObserveStopConditions,
        ControlledInternalTrialRunStep::RecordManualOperatorStep,
        ControlledInternalTrialRunStep::ProjectTrialRunSummary,
        ControlledInternalTrialRunStep::ProjectTrialEvidenceLinkage,
        ControlledInternalTrialRunStep::CloseLocalTrialRun,
    ]
}

fn deterministic_controlled_internal_trial_run_id(
    state: &LocalOperatorShellState,
    request: &ControlledInternalTrialExecutionRequest,
) -> String {
    let basis = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        state
            .controlled_internal_trial_package_projection
            .package_id
            .as_deref()
            .unwrap_or("none"),
        state.trial_operator_runbook.status.code(),
        state
            .trial_session_evidence_projection
            .evidence_id
            .as_deref()
            .unwrap_or("none"),
        state
            .trial_replay_restore_verification
            .verification_id
            .as_deref()
            .unwrap_or("none"),
        state.complete_local_operator_workflow.status.code(),
        request.operator_step_recorded,
        request.stop_condition_observed,
        state.trial_failure_drill.highest_severity.code(),
    );
    format!(
        "controlled-internal-trial-run-{}",
        stable_trial_session_evidence_digest(&basis)
    )
}

pub fn derive_controlled_internal_trial_execution_projection(
    state: &LocalOperatorShellState,
) -> ControlledInternalTrialExecutionProjection {
    let mut projection = initial_controlled_internal_trial_execution_projection();
    let errors = validate_controlled_internal_trial_execution_preconditions(state);
    projection.status = if errors.is_empty() {
        ControlledInternalTrialRunStatus::ReadyForBoundedLocalTrialRun
    } else {
        ControlledInternalTrialRunStatus::PreconditionsMissing
    };
    projection.current_blocker = errors.first().copied();
    projection.rejection_reasons = errors.clone();
    projection.precondition_status =
        controlled_internal_trial_execution_precondition_status(state, &errors);
    projection.evidence_linkage = controlled_internal_trial_execution_evidence_linkage(state);
    projection
}

fn controlled_internal_trial_run_projection(
    state: &LocalOperatorShellState,
    request: &ControlledInternalTrialExecutionRequest,
    errors: Vec<ControlledInternalTrialRunError>,
) -> ControlledInternalTrialRunProjection {
    let blocked = request.stop_condition_observed || !errors.is_empty();
    let manual_status = if request.operator_step_recorded {
        ControlledInternalTrialManualStepStatus::Recorded
    } else if blocked {
        ControlledInternalTrialManualStepStatus::Missing
    } else {
        ControlledInternalTrialManualStepStatus::ManualActionRequired
    };
    let blocker = if request.stop_condition_observed {
        Some(ControlledInternalTrialRunError::StopConditionObserved)
    } else if !request.operator_step_recorded && errors.is_empty() {
        Some(ControlledInternalTrialRunError::ManualOperatorStepMissing)
    } else {
        errors.first().copied()
    };
    let status = if !errors.is_empty() {
        ControlledInternalTrialRunStatus::TrialRunRejected
    } else if request.stop_condition_observed {
        ControlledInternalTrialRunStatus::TrialRunBlocked
    } else if request.operator_step_recorded {
        ControlledInternalTrialRunStatus::TrialRunCompletedLocally
    } else {
        ControlledInternalTrialRunStatus::TrialRunInProgress
    };
    let steps = controlled_internal_trial_step_order()
        .into_iter()
        .map(|step| {
            let status = if !errors.is_empty() {
                ControlledInternalTrialRunStepStatus::Rejected
            } else if request.stop_condition_observed
                && matches!(
                    step,
                    ControlledInternalTrialRunStep::ObserveStopConditions
                        | ControlledInternalTrialRunStep::RecordManualOperatorStep
                        | ControlledInternalTrialRunStep::ProjectTrialRunSummary
                        | ControlledInternalTrialRunStep::ProjectTrialEvidenceLinkage
                        | ControlledInternalTrialRunStep::CloseLocalTrialRun
                )
            {
                ControlledInternalTrialRunStepStatus::Blocked
            } else if step == ControlledInternalTrialRunStep::RecordManualOperatorStep
                && !request.operator_step_recorded
            {
                ControlledInternalTrialRunStepStatus::ManualActionRequired
            } else if request.operator_step_recorded
                || step != ControlledInternalTrialRunStep::CloseLocalTrialRun
            {
                ControlledInternalTrialRunStepStatus::Completed
            } else {
                ControlledInternalTrialRunStepStatus::Available
            };
            ControlledInternalTrialRunStepProjection {
                step,
                status,
                summary: format!("{} => {}", step.code(), status.code()),
            }
        })
        .collect::<Vec<_>>();
    ControlledInternalTrialRunProjection {
        run_id: deterministic_controlled_internal_trial_run_id(state, request),
        status,
        current_step: steps
            .iter()
            .find(|step| step.status != ControlledInternalTrialRunStepStatus::Completed)
            .map(|step| step.step),
        next_step: steps
            .iter()
            .find(|step| {
                matches!(
                    step.status,
                    ControlledInternalTrialRunStepStatus::Available
                        | ControlledInternalTrialRunStepStatus::ManualActionRequired
                )
            })
            .map(|step| step.step),
        steps,
        current_blocker: blocker,
        rejection_reasons: errors,
        stop_condition_observation: ControlledInternalTrialStopObservation {
            status: if request.stop_condition_observed {
                "stop_condition_observed"
            } else {
                "no_stop_condition_observed"
            }
            .to_string(),
            observed: request.stop_condition_observed,
            markers: state
                .controlled_internal_trial_package_projection
                .stop_condition_summary
                .clone(),
            enforcement_automated: false,
        },
        manual_operator_step_status: manual_status,
        evidence_linkage: controlled_internal_trial_execution_evidence_linkage(state),
        summary: format!(
            "Bounded local controlled internal trial run status: {}.",
            status.code()
        ),
    }
}

pub fn start_controlled_internal_trial_execution(
    state: &LocalOperatorShellState,
    request: ControlledInternalTrialExecutionRequest,
) -> LocalOperatorShellState {
    let mut next = state.clone();
    let mut errors = validate_controlled_internal_trial_execution_preconditions(state);
    errors.extend(controlled_internal_trial_execution_claim_errors(&request));
    let run = controlled_internal_trial_run_projection(state, &request, errors.clone());
    let mut projection = derive_controlled_internal_trial_execution_projection(state);
    if errors.is_empty() && !request.stop_condition_observed {
        projection.status = run.status;
        projection.active_run = Some(run);
        projection.current_blocker = projection
            .active_run
            .as_ref()
            .and_then(|r| r.current_blocker);
    } else {
        projection.status = if controlled_internal_trial_execution_claim_errors(&request).is_empty()
        {
            run.status
        } else {
            ControlledInternalTrialRunStatus::InvalidTrialRunRequest
        };
        projection.active_run = state.controlled_internal_trial_execution.active_run.clone();
        projection.last_rejected_run = Some(run);
        projection.current_blocker = projection
            .last_rejected_run
            .as_ref()
            .and_then(|r| r.current_blocker);
        projection.rejection_reasons = errors;
    }
    next.controlled_internal_trial_execution = projection;
    refresh_trial_evidence_review_state(refresh_trial_observability_state(next))
}

pub fn step_controlled_internal_trial_execution(
    state: &LocalOperatorShellState,
    mut request: ControlledInternalTrialExecutionRequest,
) -> LocalOperatorShellState {
    request.operator_step_recorded = true;
    start_controlled_internal_trial_execution(state, request)
}
