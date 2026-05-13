//! Trial observability projection and local error-reporting helpers.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialObservabilityStatus {
    NotObserved,
    ObservabilityProjected,
    TrialRunObserved,
    BlockedStateObserved,
    RejectedStateObserved,
    StopConditionObserved,
    VerificationMismatchObserved,
    ErrorReportProjected,
}

impl TrialObservabilityStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotObserved => "not_observed",
            Self::ObservabilityProjected => "observability_projected",
            Self::TrialRunObserved => "trial_run_observed",
            Self::BlockedStateObserved => "blocked_state_observed",
            Self::RejectedStateObserved => "rejected_state_observed",
            Self::StopConditionObserved => "stop_condition_observed",
            Self::VerificationMismatchObserved => "verification_mismatch_observed",
            Self::ErrorReportProjected => "error_report_projected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialErrorReportStatus {
    NoErrorsObserved,
    ErrorsObserved,
    BlockedErrorsObserved,
    RejectedErrorsObserved,
    InvalidObservabilityInput,
}

impl TrialErrorReportStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoErrorsObserved => "no_errors_observed",
            Self::ErrorsObserved => "errors_observed",
            Self::BlockedErrorsObserved => "blocked_errors_observed",
            Self::RejectedErrorsObserved => "rejected_errors_observed",
            Self::InvalidObservabilityInput => "invalid_observability_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialErrorCategory {
    NoTrialRun,
    TrialPreconditionMissing,
    TrialRunBlocked,
    TrialRunRejected,
    StopConditionObserved,
    ManualOperatorStepRequired,
    PackageMissing,
    PackageValidationFailed,
    PackageReadBackFailed,
    EvidenceMissing,
    EvidenceValidationFailed,
    EvidenceReadBackFailed,
    ReplayRestoreVerificationMissing,
    ReplayRestoreVerificationRejected,
    ReplayStatusMismatch,
    RestoreHistoryMismatch,
    ProviderPipelineBlocked,
    ProviderOutputValidationRejected,
    WorkflowBlocked,
    WorkflowRejected,
    MaterializationMissing,
    AuthorityClaimDetected,
    ReadinessClaimDetected,
    ReleaseOrDeploymentClaimDetected,
    PublicOrProductionUseClaimDetected,
    ActionAuthorizationClaimDetected,
    ReplayRepairOrRecoveryPromotionClaimDetected,
}

impl TrialErrorCategory {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoTrialRun => "no_trial_run",
            Self::TrialPreconditionMissing => "trial_precondition_missing",
            Self::TrialRunBlocked => "trial_run_blocked",
            Self::TrialRunRejected => "trial_run_rejected",
            Self::StopConditionObserved => "stop_condition_observed",
            Self::ManualOperatorStepRequired => "manual_operator_step_required",
            Self::PackageMissing => "package_missing",
            Self::PackageValidationFailed => "package_validation_failed",
            Self::PackageReadBackFailed => "package_read_back_failed",
            Self::EvidenceMissing => "evidence_missing",
            Self::EvidenceValidationFailed => "evidence_validation_failed",
            Self::EvidenceReadBackFailed => "evidence_read_back_failed",
            Self::ReplayRestoreVerificationMissing => "replay_restore_verification_missing",
            Self::ReplayRestoreVerificationRejected => "replay_restore_verification_rejected",
            Self::ReplayStatusMismatch => "replay_status_mismatch",
            Self::RestoreHistoryMismatch => "restore_history_mismatch",
            Self::ProviderPipelineBlocked => "provider_pipeline_blocked",
            Self::ProviderOutputValidationRejected => "provider_output_validation_rejected",
            Self::WorkflowBlocked => "workflow_blocked",
            Self::WorkflowRejected => "workflow_rejected",
            Self::MaterializationMissing => "materialization_missing",
            Self::AuthorityClaimDetected => "authority_claim_detected",
            Self::ReadinessClaimDetected => "readiness_claim_detected",
            Self::ReleaseOrDeploymentClaimDetected => "release_or_deployment_claim_detected",
            Self::PublicOrProductionUseClaimDetected => "public_or_production_use_claim_detected",
            Self::ActionAuthorizationClaimDetected => "action_authorization_claim_detected",
            Self::ReplayRepairOrRecoveryPromotionClaimDetected => {
                "replay_repair_or_recovery_promotion_claim_detected"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialErrorSeverity {
    Info,
    Warning,
    Blocking,
    StopCondition,
    InvalidInput,
}

impl TrialErrorSeverity {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Blocking => "blocking",
            Self::StopCondition => "stop_condition",
            Self::InvalidInput => "invalid_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialErrorSource {
    ControlledTrialExecutionHarness,
    ControlledInternalTrialPackage,
    TrialSessionEvidence,
    ReplayRestoreVerification,
    TrialRunbook,
    FailureDrill,
    CompleteLocalWorkflow,
    ProviderOutputPipeline,
    LocalCandidateMaterialization,
    SessionRestore,
}

impl TrialErrorSource {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ControlledTrialExecutionHarness => "controlled_trial_execution_harness",
            Self::ControlledInternalTrialPackage => "controlled_internal_trial_package",
            Self::TrialSessionEvidence => "trial_session_evidence",
            Self::ReplayRestoreVerification => "replay_restore_verification",
            Self::TrialRunbook => "trial_runbook",
            Self::FailureDrill => "failure_drill",
            Self::CompleteLocalWorkflow => "complete_local_workflow",
            Self::ProviderOutputPipeline => "provider_output_pipeline",
            Self::LocalCandidateMaterialization => "local_candidate_materialization",
            Self::SessionRestore => "session_restore",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialObservabilityBoundaryStatus {
    LocalObservabilityOnly,
    NonPublicObservability,
    NoProductionMonitoring,
    NoRemoteTelemetry,
    NoNetworkReporting,
    NoBackgroundService,
    NoAutomatedRemediation,
    NoAutomatedEscalation,
    NoStopConditionAutomation,
    NoActionExecution,
    NoReplayRepair,
    NoRecoveryPromotion,
    NoReadinessApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoProductionApproval,
}

impl TrialObservabilityBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalObservabilityOnly => "local_observability_only",
            Self::NonPublicObservability => "non_public_observability",
            Self::NoProductionMonitoring => "no_production_monitoring",
            Self::NoRemoteTelemetry => "no_remote_telemetry",
            Self::NoNetworkReporting => "no_network_reporting",
            Self::NoBackgroundService => "no_background_service",
            Self::NoAutomatedRemediation => "no_automated_remediation",
            Self::NoAutomatedEscalation => "no_automated_escalation",
            Self::NoStopConditionAutomation => "no_stop_condition_automation",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
            Self::NoReadinessApproval => "no_readiness_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoProductionApproval => "no_production_approval",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialObservabilityCapabilitySurface {
    pub local_only: bool,
    pub non_public: bool,
    pub production_monitoring: bool,
    pub remote_telemetry: bool,
    pub network_reporting: bool,
    pub background_service: bool,
    pub remediates: bool,
    pub escalates: bool,
    pub automates_stop_conditions: bool,
    pub executes_actions: bool,
    pub repairs_replay: bool,
    pub promotes_recovery: bool,
    pub approves_readiness: bool,
    pub approves_release: bool,
    pub approves_deployment: bool,
    pub approves_public_use: bool,
    pub approves_production: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialBlockedStateSummary {
    pub status: String,
    pub current_blocker: String,
    pub rejection_reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialVerificationMismatchSummary {
    pub verification_status: String,
    pub mismatches: Vec<String>,
    pub replay_status_comparison: String,
    pub restore_history_comparison: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialStopConditionObservationSummary {
    pub status: String,
    pub observed: bool,
    pub markers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialErrorEvidenceLinkage {
    pub source: TrialErrorSource,
    pub linkage: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialErrorDetail {
    pub category: TrialErrorCategory,
    pub severity: TrialErrorSeverity,
    pub source: TrialErrorSource,
    pub summary: String,
    pub operator_guidance: String,
    pub evidence_linkage: TrialErrorEvidenceLinkage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialErrorReportProjection {
    pub status: TrialErrorReportStatus,
    pub error_count: usize,
    pub highest_severity: TrialErrorSeverity,
    pub details: Vec<TrialErrorDetail>,
    pub category_summary: Vec<String>,
    pub evidence_linkage_summary: Vec<String>,
    pub local_descriptive_only_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialObservabilityProjection {
    pub status: TrialObservabilityStatus,
    pub trial_run_status: String,
    pub current_trial_step: String,
    pub current_blocker: String,
    pub stop_condition_observation: TrialStopConditionObservationSummary,
    pub manual_operator_step_status: String,
    pub package_status: String,
    pub evidence_status: String,
    pub package_read_back_status: String,
    pub evidence_read_back_status: String,
    pub replay_restore_verification_status: String,
    pub mismatch_summary: TrialVerificationMismatchSummary,
    pub package_evidence_read_back_failure_summary: Vec<String>,
    pub replay_status_comparison_summary: String,
    pub restore_history_comparison_summary: String,
    pub runbook_failure_drill_category_summary: Vec<String>,
    pub evidence_linkage_summary: Vec<String>,
    pub blocked_state_summary: TrialBlockedStateSummary,
    pub boundary_statuses: Vec<TrialObservabilityBoundaryStatus>,
    pub capability_surface: TrialObservabilityCapabilitySurface,
    pub local_only_non_public_note: String,
    pub no_production_monitoring_note: String,
    pub no_remote_telemetry_note: String,
    pub no_background_service_note: String,
    pub no_remediation_escalation_stop_automation_note: String,
    pub no_approval_note: String,
}

pub fn trial_observability_boundary_statuses() -> Vec<TrialObservabilityBoundaryStatus> {
    vec![
        TrialObservabilityBoundaryStatus::LocalObservabilityOnly,
        TrialObservabilityBoundaryStatus::NonPublicObservability,
        TrialObservabilityBoundaryStatus::NoProductionMonitoring,
        TrialObservabilityBoundaryStatus::NoRemoteTelemetry,
        TrialObservabilityBoundaryStatus::NoNetworkReporting,
        TrialObservabilityBoundaryStatus::NoBackgroundService,
        TrialObservabilityBoundaryStatus::NoAutomatedRemediation,
        TrialObservabilityBoundaryStatus::NoAutomatedEscalation,
        TrialObservabilityBoundaryStatus::NoStopConditionAutomation,
        TrialObservabilityBoundaryStatus::NoActionExecution,
        TrialObservabilityBoundaryStatus::NoReplayRepair,
        TrialObservabilityBoundaryStatus::NoRecoveryPromotion,
        TrialObservabilityBoundaryStatus::NoReadinessApproval,
        TrialObservabilityBoundaryStatus::NoReleaseApproval,
        TrialObservabilityBoundaryStatus::NoDeploymentApproval,
        TrialObservabilityBoundaryStatus::NoPublicUseApproval,
        TrialObservabilityBoundaryStatus::NoProductionApproval,
    ]
}

pub fn trial_observability_capability_surface() -> TrialObservabilityCapabilitySurface {
    TrialObservabilityCapabilitySurface {
        local_only: true,
        non_public: true,
        production_monitoring: false,
        remote_telemetry: false,
        network_reporting: false,
        background_service: false,
        remediates: false,
        escalates: false,
        automates_stop_conditions: false,
        executes_actions: false,
        repairs_replay: false,
        promotes_recovery: false,
        approves_readiness: false,
        approves_release: false,
        approves_deployment: false,
        approves_public_use: false,
        approves_production: false,
    }
}

pub(crate) fn initial_trial_error_report_projection() -> TrialErrorReportProjection {
    TrialErrorReportProjection {
        status: TrialErrorReportStatus::ErrorsObserved,
        error_count: 1,
        highest_severity: TrialErrorSeverity::Warning,
        details: vec![TrialErrorDetail {
            category: TrialErrorCategory::NoTrialRun,
            severity: TrialErrorSeverity::Warning,
            source: TrialErrorSource::ControlledTrialExecutionHarness,
            summary: "No controlled internal trial run has been observed.".to_string(),
            operator_guidance:
                "Inspect local trial preconditions before starting a bounded local trial run."
                    .to_string(),
            evidence_linkage: TrialErrorEvidenceLinkage {
                source: TrialErrorSource::ControlledTrialExecutionHarness,
                linkage: "controlled_internal_trial_execution=not_started".to_string(),
            },
        }],
        category_summary: vec!["no_trial_run".to_string()],
        evidence_linkage_summary: vec![
            "controlled_internal_trial_execution=not_started".to_string()
        ],
        local_descriptive_only_note: "Error reporting is local and descriptive only.".to_string(),
    }
}

pub fn initial_trial_observability_projection() -> TrialObservabilityProjection {
    TrialObservabilityProjection {
        status: TrialObservabilityStatus::NotObserved,
        trial_run_status: "not_started".to_string(),
        current_trial_step: "none".to_string(),
        current_blocker: "none".to_string(),
        stop_condition_observation: TrialStopConditionObservationSummary {
            status: "not_started".to_string(),
            observed: false,
            markers: Vec::new(),
        },
        manual_operator_step_status: "not_started".to_string(),
        package_status: "not_packaged".to_string(),
        evidence_status: "not_captured".to_string(),
        package_read_back_status: "not_read".to_string(),
        evidence_read_back_status: "not_read".to_string(),
        replay_restore_verification_status: "not_verified".to_string(),
        mismatch_summary: TrialVerificationMismatchSummary {
            verification_status: "not_verified".to_string(),
            mismatches: Vec::new(),
            replay_status_comparison: "not_verified".to_string(),
            restore_history_comparison: "not_verified".to_string(),
        },
        package_evidence_read_back_failure_summary: Vec::new(),
        replay_status_comparison_summary: "not_verified".to_string(),
        restore_history_comparison_summary: "not_verified".to_string(),
        runbook_failure_drill_category_summary: Vec::new(),
        evidence_linkage_summary: Vec::new(),
        blocked_state_summary: TrialBlockedStateSummary {
            status: "not_observed".to_string(),
            current_blocker: "none".to_string(),
            rejection_reasons: Vec::new(),
        },
        boundary_statuses: trial_observability_boundary_statuses(),
        capability_surface: trial_observability_capability_surface(),
        local_only_non_public_note: "Trial observability is local-only and non-public.".to_string(),
        no_production_monitoring_note: "No production monitoring is active.".to_string(),
        no_remote_telemetry_note: "No remote telemetry is sent.".to_string(),
        no_background_service_note: "No background service is active.".to_string(),
        no_remediation_escalation_stop_automation_note: "No remediation, escalation, or stop-condition enforcement is automated.".to_string(),
        no_approval_note: "Observability does not approve controlled human use, readiness, release, deployment, public use, or production use.".to_string(),
    }
}

fn push_trial_error_detail(
    details: &mut Vec<TrialErrorDetail>,
    category: TrialErrorCategory,
    source: TrialErrorSource,
    summary: impl Into<String>,
    linkage: impl Into<String>,
) {
    let severity = classify_trial_error_severity(category);
    details.push(TrialErrorDetail {
        category,
        severity,
        source,
        summary: summary.into(),
        operator_guidance: trial_error_guidance(category).to_string(),
        evidence_linkage: TrialErrorEvidenceLinkage {
            source,
            linkage: linkage.into(),
        },
    });
}

fn classify_trial_error_severity(category: TrialErrorCategory) -> TrialErrorSeverity {
    match category {
        TrialErrorCategory::StopConditionObserved => TrialErrorSeverity::StopCondition,
        TrialErrorCategory::AuthorityClaimDetected
        | TrialErrorCategory::ReadinessClaimDetected
        | TrialErrorCategory::ReleaseOrDeploymentClaimDetected
        | TrialErrorCategory::PublicOrProductionUseClaimDetected
        | TrialErrorCategory::ActionAuthorizationClaimDetected
        | TrialErrorCategory::ReplayRepairOrRecoveryPromotionClaimDetected => {
            TrialErrorSeverity::InvalidInput
        }
        TrialErrorCategory::NoTrialRun
        | TrialErrorCategory::ManualOperatorStepRequired
        | TrialErrorCategory::TrialPreconditionMissing => TrialErrorSeverity::Warning,
        _ => TrialErrorSeverity::Blocking,
    }
}

fn trial_error_guidance(category: TrialErrorCategory) -> &'static str {
    match category {
        TrialErrorCategory::NoTrialRun => {
            "Inspect local trial preconditions before starting a bounded local trial run."
        }
        TrialErrorCategory::ManualOperatorStepRequired => {
            "Record the manual operator step locally before closing the trial run."
        }
        TrialErrorCategory::StopConditionObserved => {
            "Review the observed stop-condition marker locally; no enforcement is automated."
        }
        _ => "Review the linked local trial surface and preserve the diagnostic evidence.",
    }
}

pub fn derive_trial_error_report_projection(
    state: &LocalOperatorShellState,
) -> TrialErrorReportProjection {
    let mut details = Vec::new();
    let execution = &state.controlled_internal_trial_execution;
    let displayed_run = execution
        .active_run
        .as_ref()
        .or(execution.last_rejected_run.as_ref());

    if displayed_run.is_none() {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::NoTrialRun,
            TrialErrorSource::ControlledTrialExecutionHarness,
            "No controlled internal trial run has been observed.",
            "controlled_internal_trial_execution=not_started",
        );
    }
    if execution.status == ControlledInternalTrialRunStatus::PreconditionsMissing {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::TrialPreconditionMissing,
            TrialErrorSource::ControlledTrialExecutionHarness,
            "Trial preconditions are missing.",
            "controlled_internal_trial_execution=preconditions_missing",
        );
    }
    if execution.status == ControlledInternalTrialRunStatus::TrialRunBlocked {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::TrialRunBlocked,
            TrialErrorSource::ControlledTrialExecutionHarness,
            "Controlled internal trial run is blocked.",
            "controlled_internal_trial_execution=trial_run_blocked",
        );
    }
    if matches!(
        execution.status,
        ControlledInternalTrialRunStatus::TrialRunRejected
            | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
    ) {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::TrialRunRejected,
            TrialErrorSource::ControlledTrialExecutionHarness,
            "Controlled internal trial run is rejected.",
            "controlled_internal_trial_execution=trial_run_rejected",
        );
    }
    if let Some(run) = displayed_run {
        if run.stop_condition_observation.observed {
            push_trial_error_detail(
                &mut details,
                TrialErrorCategory::StopConditionObserved,
                TrialErrorSource::ControlledTrialExecutionHarness,
                "Stop condition was observed in the local trial run.",
                "stop_condition_observation=observed",
            );
        }
        if run.manual_operator_step_status
            == ControlledInternalTrialManualStepStatus::ManualActionRequired
            || run.manual_operator_step_status == ControlledInternalTrialManualStepStatus::Missing
        {
            push_trial_error_detail(
                &mut details,
                TrialErrorCategory::ManualOperatorStepRequired,
                TrialErrorSource::ControlledTrialExecutionHarness,
                "Manual operator step is required or missing.",
                "manual_operator_step_status=required",
            );
        }
    }

    if state.controlled_internal_trial_package_projection.status
        == ControlledInternalTrialPackageStatus::NotPackaged
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::PackageMissing,
            TrialErrorSource::ControlledInternalTrialPackage,
            "Controlled internal trial package is missing.",
            "trial_package=missing",
        );
    }
    if state
        .controlled_internal_trial_package_projection
        .validation_status
        != ControlledInternalTrialPackageValidationStatus::Valid
        && state.controlled_internal_trial_package_projection.status
            != ControlledInternalTrialPackageStatus::NotPackaged
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::PackageValidationFailed,
            TrialErrorSource::ControlledInternalTrialPackage,
            "Controlled internal trial package validation failed.",
            "trial_package_validation=failed",
        );
    }
    if matches!(state.controlled_internal_trial_package_projection.read_back_validation_status, Some(status) if status != ControlledInternalTrialPackageValidationStatus::Valid)
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::PackageReadBackFailed,
            TrialErrorSource::ControlledInternalTrialPackage,
            "Controlled internal trial package read-back failed.",
            "trial_package_read_back=failed",
        );
    }
    if state.trial_session_evidence_projection.status == TrialSessionEvidenceStatus::NotCaptured {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::EvidenceMissing,
            TrialErrorSource::TrialSessionEvidence,
            "Trial session evidence is missing.",
            "trial_session_evidence=missing",
        );
    }
    if state.trial_session_evidence_projection.validation_status
        != TrialSessionEvidenceValidationStatus::Valid
        && state.trial_session_evidence_projection.status != TrialSessionEvidenceStatus::NotCaptured
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::EvidenceValidationFailed,
            TrialErrorSource::TrialSessionEvidence,
            "Trial session evidence validation failed.",
            "trial_session_evidence_validation=failed",
        );
    }
    if matches!(state.trial_session_evidence_projection.read_back_validation_status, Some(status) if status != TrialSessionEvidenceValidationStatus::Valid)
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::EvidenceReadBackFailed,
            TrialErrorSource::TrialSessionEvidence,
            "Trial session evidence read-back failed.",
            "trial_session_evidence_read_back=failed",
        );
    }

    match state.trial_replay_restore_verification.status {
        TrialReplayRestoreVerificationStatus::NotVerified
        | TrialReplayRestoreVerificationStatus::VerificationInputMissing => {
            push_trial_error_detail(
                &mut details,
                TrialErrorCategory::ReplayRestoreVerificationMissing,
                TrialErrorSource::ReplayRestoreVerification,
                "Replay/restore verification is missing.",
                "replay_restore_verification=missing",
            )
        }
        TrialReplayRestoreVerificationStatus::VerificationRejected
        | TrialReplayRestoreVerificationStatus::InvalidVerificationInput => {
            push_trial_error_detail(
                &mut details,
                TrialErrorCategory::ReplayRestoreVerificationRejected,
                TrialErrorSource::ReplayRestoreVerification,
                "Replay/restore verification is rejected.",
                "replay_restore_verification=rejected",
            )
        }
        _ => {}
    }
    if state
        .trial_replay_restore_verification
        .mismatches
        .contains(&TrialReplayRestoreVerificationMismatch::ReplayStatusSnapshotMismatch)
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::ReplayStatusMismatch,
            TrialErrorSource::ReplayRestoreVerification,
            "Replay/status comparison mismatch observed.",
            "replay_status_comparison=mismatch",
        );
    }
    if state
        .trial_replay_restore_verification
        .mismatches
        .contains(&TrialReplayRestoreVerificationMismatch::RestoreHistorySnapshotMismatch)
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::RestoreHistoryMismatch,
            TrialErrorSource::ReplayRestoreVerification,
            "Restore/history comparison mismatch observed.",
            "restore_history_comparison=mismatch",
        );
    }
    if state.local_provider_output_pipeline.status
        == LocalProviderOutputPipelineValidationStatus::Rejected
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::ProviderPipelineBlocked,
            TrialErrorSource::ProviderOutputPipeline,
            "Provider output pipeline is blocked or rejected.",
            "provider_output_pipeline=rejected",
        );
    }
    if state.provider_output_validation.status == LocalProviderOutputValidationStatus::Rejected {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::ProviderOutputValidationRejected,
            TrialErrorSource::ProviderOutputPipeline,
            "Provider output validation is rejected.",
            "provider_output_validation=rejected",
        );
    }
    if state.complete_local_operator_workflow.status == CompleteLocalOperatorWorkflowStatus::Blocked
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::WorkflowBlocked,
            TrialErrorSource::CompleteLocalWorkflow,
            "Complete local workflow is blocked.",
            "complete_local_workflow=blocked",
        );
    }
    if state.complete_local_operator_workflow.status
        == CompleteLocalOperatorWorkflowStatus::Rejected
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::WorkflowRejected,
            TrialErrorSource::CompleteLocalWorkflow,
            "Complete local workflow is rejected.",
            "complete_local_workflow=rejected",
        );
    }
    if state.local_candidate_output.status == LocalCandidateMaterializationStatus::NotMaterialized {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::MaterializationMissing,
            TrialErrorSource::LocalCandidateMaterialization,
            "Local candidate materialization is missing.",
            "local_candidate_materialization=missing",
        );
    }

    for reason in &execution.rejection_reasons {
        match reason {
            ControlledInternalTrialRunError::ReadinessClaimRejected => push_trial_error_detail(
                &mut details,
                TrialErrorCategory::ReadinessClaimDetected,
                TrialErrorSource::ControlledTrialExecutionHarness,
                "Readiness claim detected and rejected.",
                "claim=readiness",
            ),
            ControlledInternalTrialRunError::ReleaseClaimRejected
            | ControlledInternalTrialRunError::DeploymentClaimRejected
            | ControlledInternalTrialRunError::SigningClaimRejected
            | ControlledInternalTrialRunError::PublishingClaimRejected
            | ControlledInternalTrialRunError::ReleaseArtifactClaimRejected => {
                push_trial_error_detail(
                    &mut details,
                    TrialErrorCategory::ReleaseOrDeploymentClaimDetected,
                    TrialErrorSource::ControlledTrialExecutionHarness,
                    "Release or deployment claim detected and rejected.",
                    "claim=release_or_deployment",
                )
            }
            ControlledInternalTrialRunError::PublicUseClaimRejected
            | ControlledInternalTrialRunError::ProductionUseClaimRejected => {
                push_trial_error_detail(
                    &mut details,
                    TrialErrorCategory::PublicOrProductionUseClaimDetected,
                    TrialErrorSource::ControlledTrialExecutionHarness,
                    "Public or production use claim detected and rejected.",
                    "claim=public_or_production_use",
                )
            }
            ControlledInternalTrialRunError::ProviderTrustClaimRejected => push_trial_error_detail(
                &mut details,
                TrialErrorCategory::AuthorityClaimDetected,
                TrialErrorSource::ControlledTrialExecutionHarness,
                "Provider trust claim detected and rejected.",
                "claim=provider_trust",
            ),
            ControlledInternalTrialRunError::ActionAuthorizationClaimRejected => {
                push_trial_error_detail(
                    &mut details,
                    TrialErrorCategory::ActionAuthorizationClaimDetected,
                    TrialErrorSource::ControlledTrialExecutionHarness,
                    "Action authorization claim detected and rejected.",
                    "claim=action_authorization",
                )
            }
            ControlledInternalTrialRunError::ReplayRepairClaimRejected
            | ControlledInternalTrialRunError::RecoveryPromotionClaimRejected => {
                push_trial_error_detail(
                    &mut details,
                    TrialErrorCategory::ReplayRepairOrRecoveryPromotionClaimDetected,
                    TrialErrorSource::ControlledTrialExecutionHarness,
                    "Replay repair or recovery promotion claim detected and rejected.",
                    "claim=replay_repair_or_recovery_promotion",
                )
            }
            _ => {}
        }
    }

    details.sort_by_key(|detail| detail.category.code());
    let highest_severity = if details
        .iter()
        .any(|detail| detail.severity == TrialErrorSeverity::InvalidInput)
    {
        TrialErrorSeverity::InvalidInput
    } else if details
        .iter()
        .any(|detail| detail.severity == TrialErrorSeverity::StopCondition)
    {
        TrialErrorSeverity::StopCondition
    } else if details
        .iter()
        .any(|detail| detail.severity == TrialErrorSeverity::Blocking)
    {
        TrialErrorSeverity::Blocking
    } else if details
        .iter()
        .any(|detail| detail.severity == TrialErrorSeverity::Warning)
    {
        TrialErrorSeverity::Warning
    } else {
        TrialErrorSeverity::Info
    };
    let status = if details.is_empty() {
        TrialErrorReportStatus::NoErrorsObserved
    } else if execution.status == ControlledInternalTrialRunStatus::TrialRunBlocked {
        TrialErrorReportStatus::BlockedErrorsObserved
    } else if matches!(
        execution.status,
        ControlledInternalTrialRunStatus::TrialRunRejected
            | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
    ) {
        TrialErrorReportStatus::RejectedErrorsObserved
    } else if highest_severity == TrialErrorSeverity::InvalidInput {
        TrialErrorReportStatus::InvalidObservabilityInput
    } else {
        TrialErrorReportStatus::ErrorsObserved
    };
    TrialErrorReportProjection {
        status,
        error_count: details.len(),
        highest_severity,
        category_summary: details
            .iter()
            .map(|detail| detail.category.code().to_string())
            .collect(),
        evidence_linkage_summary: details
            .iter()
            .map(|detail| detail.evidence_linkage.linkage.clone())
            .collect(),
        details,
        local_descriptive_only_note: "Error reporting is local and descriptive only.".to_string(),
    }
}

pub fn derive_trial_observability_projection(
    state: &LocalOperatorShellState,
) -> TrialObservabilityProjection {
    let mut projection = initial_trial_observability_projection();
    let execution = &state.controlled_internal_trial_execution;
    let displayed_run = execution
        .active_run
        .as_ref()
        .or(execution.last_rejected_run.as_ref());
    projection.trial_run_status = displayed_run
        .map(|run| run.status.code())
        .unwrap_or_else(|| execution.status.code())
        .to_string();
    projection.current_trial_step = displayed_run
        .and_then(|run| run.current_step)
        .map(|step| step.code())
        .unwrap_or("none")
        .to_string();
    projection.current_blocker = displayed_run
        .and_then(|run| run.current_blocker)
        .or(execution.current_blocker)
        .map(|blocker| blocker.code())
        .unwrap_or("none")
        .to_string();
    projection.manual_operator_step_status = displayed_run
        .map(|run| run.manual_operator_step_status.code())
        .unwrap_or("not_started")
        .to_string();
    projection.package_status = state
        .controlled_internal_trial_package_projection
        .status
        .code()
        .to_string();
    projection.evidence_status = state
        .trial_session_evidence_projection
        .status
        .code()
        .to_string();
    projection.package_read_back_status = state
        .controlled_internal_trial_package_projection
        .read_back_validation_status
        .map(|status| status.code())
        .unwrap_or("not_read")
        .to_string();
    projection.evidence_read_back_status = state
        .trial_session_evidence_projection
        .read_back_validation_status
        .map(|status| status.code())
        .unwrap_or("not_read")
        .to_string();
    projection.replay_restore_verification_status = state
        .trial_replay_restore_verification
        .status
        .code()
        .to_string();
    if let Some(run) = displayed_run {
        projection.stop_condition_observation = TrialStopConditionObservationSummary {
            status: run.stop_condition_observation.status.clone(),
            observed: run.stop_condition_observation.observed,
            markers: run.stop_condition_observation.markers.clone(),
        };
        projection.evidence_linkage_summary = vec![
            run.evidence_linkage.trial_package.clone(),
            run.evidence_linkage.runbook.clone(),
            run.evidence_linkage.failure_drill.clone(),
            run.evidence_linkage.trial_session_evidence.clone(),
            run.evidence_linkage.replay_restore_verification.clone(),
            run.evidence_linkage.local_workflow.clone(),
        ];
    } else {
        projection.evidence_linkage_summary = vec![
            execution.evidence_linkage.trial_package.clone(),
            execution.evidence_linkage.runbook.clone(),
            execution.evidence_linkage.failure_drill.clone(),
            execution.evidence_linkage.trial_session_evidence.clone(),
            execution
                .evidence_linkage
                .replay_restore_verification
                .clone(),
            execution.evidence_linkage.local_workflow.clone(),
        ];
    }
    projection.mismatch_summary = TrialVerificationMismatchSummary {
        verification_status: state
            .trial_replay_restore_verification
            .status
            .code()
            .to_string(),
        mismatches: state
            .trial_replay_restore_verification
            .mismatches
            .iter()
            .map(|mismatch| mismatch.code().to_string())
            .collect(),
        replay_status_comparison: state
            .trial_replay_restore_verification
            .comparison_summary
            .replay_status_comparison
            .clone(),
        restore_history_comparison: state
            .trial_replay_restore_verification
            .comparison_summary
            .restore_history_comparison
            .clone(),
    };
    projection.package_evidence_read_back_failure_summary = state
        .trial_replay_restore_verification
        .mismatches
        .iter()
        .filter_map(|mismatch| match mismatch {
            TrialReplayRestoreVerificationMismatch::MissingTrialPackageReadBack
            | TrialReplayRestoreVerificationMismatch::TrialPackageReadBackInvalid
            | TrialReplayRestoreVerificationMismatch::MissingTrialSessionEvidenceReadBack
            | TrialReplayRestoreVerificationMismatch::TrialSessionEvidenceReadBackInvalid => {
                Some(mismatch.code().to_string())
            }
            _ => None,
        })
        .collect();
    projection.replay_status_comparison_summary = state
        .trial_replay_restore_verification
        .comparison_summary
        .replay_status_comparison
        .clone();
    projection.restore_history_comparison_summary = state
        .trial_replay_restore_verification
        .comparison_summary
        .restore_history_comparison
        .clone();
    projection.runbook_failure_drill_category_summary = state
        .trial_failure_drill
        .categories
        .iter()
        .map(|category| format!("{}={}", category.category.code(), category.severity.code()))
        .collect();
    projection.blocked_state_summary = TrialBlockedStateSummary {
        status: if matches!(
            execution.status,
            ControlledInternalTrialRunStatus::TrialRunBlocked
                | ControlledInternalTrialRunStatus::TrialRunRejected
                | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
        ) {
            "observed"
        } else {
            "not_observed"
        }
        .to_string(),
        current_blocker: projection.current_blocker.clone(),
        rejection_reasons: execution
            .rejection_reasons
            .iter()
            .map(|reason| reason.code().to_string())
            .collect(),
    };
    projection.status = if projection.stop_condition_observation.observed {
        TrialObservabilityStatus::StopConditionObserved
    } else if execution.status == ControlledInternalTrialRunStatus::TrialRunBlocked {
        TrialObservabilityStatus::BlockedStateObserved
    } else if matches!(
        execution.status,
        ControlledInternalTrialRunStatus::TrialRunRejected
            | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
    ) {
        TrialObservabilityStatus::RejectedStateObserved
    } else if !projection.mismatch_summary.mismatches.is_empty() {
        TrialObservabilityStatus::VerificationMismatchObserved
    } else if displayed_run.is_some() {
        TrialObservabilityStatus::TrialRunObserved
    } else {
        TrialObservabilityStatus::ObservabilityProjected
    };
    projection
}

pub fn refresh_trial_observability_state(
    mut state: LocalOperatorShellState,
) -> LocalOperatorShellState {
    state.trial_observability = derive_trial_observability_projection(&state);
    state.trial_error_report = derive_trial_error_report_projection(&state);
    state
}
