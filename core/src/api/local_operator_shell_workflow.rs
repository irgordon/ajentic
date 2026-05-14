//! Complete local operator workflow projection types and helpers.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteLocalOperatorWorkflowStatus {
    NotStarted,
    InProgress,
    Blocked,
    Rejected,
    LocalCandidateMaterialized,
    CompleteLocalWorkflowProjected,
}

impl CompleteLocalOperatorWorkflowStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::InProgress => "in_progress",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
            Self::LocalCandidateMaterialized => "local_candidate_materialized",
            Self::CompleteLocalWorkflowProjected => "complete_local_workflow_projected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteLocalOperatorWorkflowStepStatus {
    NotStarted,
    Available,
    Completed,
    Blocked,
    Rejected,
    NotApplicable,
}

impl CompleteLocalOperatorWorkflowStepStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Available => "available",
            Self::Completed => "completed",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteLocalOperatorWorkflowStepKind {
    ProviderAdapterConfigured,
    AdapterDryRunAvailable,
    ConstrainedInvocationCompleted,
    ProviderOutputPipelineProjected,
    ProviderOutputValidated,
    ProviderOutputReviewed,
    StagedProposalCreated,
    StagedProposalValidated,
    CandidateReviewProjected,
    OperatorDecisionRecorded,
    LocalCandidateMaterialized,
    ReplayStatusProjected,
    LocalEvidenceExportProjected,
    SessionPackageProjected,
    RestoreStatusProjected,
}

impl CompleteLocalOperatorWorkflowStepKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ProviderAdapterConfigured => "provider_adapter_configured",
            Self::AdapterDryRunAvailable => "adapter_dry_run_available",
            Self::ConstrainedInvocationCompleted => "constrained_invocation_completed",
            Self::ProviderOutputPipelineProjected => "provider_output_pipeline_projected",
            Self::ProviderOutputValidated => "provider_output_validated",
            Self::ProviderOutputReviewed => "provider_output_reviewed",
            Self::StagedProposalCreated => "staged_proposal_created",
            Self::StagedProposalValidated => "staged_proposal_validated",
            Self::CandidateReviewProjected => "candidate_review_projected",
            Self::OperatorDecisionRecorded => "operator_decision_recorded",
            Self::LocalCandidateMaterialized => "local_candidate_materialized",
            Self::ReplayStatusProjected => "replay_status_projected",
            Self::LocalEvidenceExportProjected => "local_evidence_export_projected",
            Self::SessionPackageProjected => "session_package_projected",
            Self::RestoreStatusProjected => "restore_status_projected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteLocalOperatorWorkflowError {
    AdapterNotConfigured,
    InvocationMissing,
    InvocationRejected,
    ProviderPipelineBlocked,
    ProviderOutputValidationMissing,
    ProviderOutputValidationRejected,
    ProviderOutputReviewMissing,
    StagedProposalMissing,
    StagedProposalValidationMissing,
    StagedProposalValidationRejected,
    CandidateReviewMissing,
    OperatorDecisionMissing,
    OperatorDecisionRejected,
    LocalCandidateNotMaterialized,
    ReplayStatusMissing,
    EvidenceExportMissing,
    SessionPackageMissing,
    RestoreStatusMissing,
}

impl CompleteLocalOperatorWorkflowError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AdapterNotConfigured => "adapter_not_configured",
            Self::InvocationMissing => "invocation_missing",
            Self::InvocationRejected => "invocation_rejected",
            Self::ProviderPipelineBlocked => "provider_pipeline_blocked",
            Self::ProviderOutputValidationMissing => "provider_output_validation_missing",
            Self::ProviderOutputValidationRejected => "provider_output_validation_rejected",
            Self::ProviderOutputReviewMissing => "provider_output_review_missing",
            Self::StagedProposalMissing => "staged_proposal_missing",
            Self::StagedProposalValidationMissing => "staged_proposal_validation_missing",
            Self::StagedProposalValidationRejected => "staged_proposal_validation_rejected",
            Self::CandidateReviewMissing => "candidate_review_missing",
            Self::OperatorDecisionMissing => "operator_decision_missing",
            Self::OperatorDecisionRejected => "operator_decision_rejected",
            Self::LocalCandidateNotMaterialized => "local_candidate_not_materialized",
            Self::ReplayStatusMissing => "replay_status_missing",
            Self::EvidenceExportMissing => "evidence_export_missing",
            Self::SessionPackageMissing => "session_package_missing",
            Self::RestoreStatusMissing => "restore_status_missing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteLocalOperatorWorkflowBoundaryStatus {
    LocalBetaWorkflowOnly,
    NoProviderTrust,
    NoProductionApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoActionExecution,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl CompleteLocalOperatorWorkflowBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalBetaWorkflowOnly => "local_beta_workflow_only",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteLocalOperatorWorkflowStep {
    pub step: CompleteLocalOperatorWorkflowStepKind,
    pub status: CompleteLocalOperatorWorkflowStepStatus,
    pub error: Option<CompleteLocalOperatorWorkflowError>,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteLocalOperatorWorkflowEvidenceSummary {
    pub provider_output_pipeline_status: String,
    pub local_candidate_materialization_status: String,
    pub replay_status: String,
    pub local_evidence_export_status: String,
    pub session_package_status: String,
    pub session_history_status: String,
    pub restore_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteLocalOperatorWorkflowCapabilitySurface {
    pub local_only: bool,
    pub non_production: bool,
    pub provider_trust_granted: bool,
    pub action_execution_authorized: bool,
    pub readiness_approved: bool,
    pub release_approved: bool,
    pub deployment_approved: bool,
    pub public_use_approved: bool,
    pub replay_repair_performed: bool,
    pub recovery_promotion_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteLocalOperatorWorkflowProjection {
    pub status: CompleteLocalOperatorWorkflowStatus,
    pub classification: String,
    pub current_step: Option<CompleteLocalOperatorWorkflowStepKind>,
    pub next_required_step: Option<CompleteLocalOperatorWorkflowStepKind>,
    pub current_blocking_step: Option<CompleteLocalOperatorWorkflowStepKind>,
    pub current_error: Option<CompleteLocalOperatorWorkflowError>,
    pub steps: Vec<CompleteLocalOperatorWorkflowStep>,
    pub rejection_reasons: Vec<String>,
    pub evidence_summary: CompleteLocalOperatorWorkflowEvidenceSummary,
    pub boundary_statuses: Vec<CompleteLocalOperatorWorkflowBoundaryStatus>,
    pub capability_surface: CompleteLocalOperatorWorkflowCapabilitySurface,
    pub local_only_note: String,
    pub no_authority_note: String,
}

pub fn complete_local_operator_workflow_step_order() -> Vec<CompleteLocalOperatorWorkflowStepKind> {
    vec![
        CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured,
        CompleteLocalOperatorWorkflowStepKind::AdapterDryRunAvailable,
        CompleteLocalOperatorWorkflowStepKind::ConstrainedInvocationCompleted,
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputPipelineProjected,
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputValidated,
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputReviewed,
        CompleteLocalOperatorWorkflowStepKind::StagedProposalCreated,
        CompleteLocalOperatorWorkflowStepKind::StagedProposalValidated,
        CompleteLocalOperatorWorkflowStepKind::CandidateReviewProjected,
        CompleteLocalOperatorWorkflowStepKind::OperatorDecisionRecorded,
        CompleteLocalOperatorWorkflowStepKind::LocalCandidateMaterialized,
        CompleteLocalOperatorWorkflowStepKind::ReplayStatusProjected,
        CompleteLocalOperatorWorkflowStepKind::LocalEvidenceExportProjected,
        CompleteLocalOperatorWorkflowStepKind::SessionPackageProjected,
        CompleteLocalOperatorWorkflowStepKind::RestoreStatusProjected,
    ]
}

pub fn initial_complete_local_operator_workflow_projection(
) -> CompleteLocalOperatorWorkflowProjection {
    let steps = complete_local_operator_workflow_step_order()
        .into_iter()
        .map(|step| {
            let (status, error, summary) = if step == CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured {
                (
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::AdapterNotConfigured),
                    "Provider adapter configuration is required before the local workflow can continue.".to_string(),
                )
            } else {
                (
                    CompleteLocalOperatorWorkflowStepStatus::NotStarted,
                    None,
                    "Waiting for earlier local workflow steps.".to_string(),
                )
            };
            CompleteLocalOperatorWorkflowStep { step, status, error, summary }
        })
        .collect();
    CompleteLocalOperatorWorkflowProjection {
        status: CompleteLocalOperatorWorkflowStatus::Blocked,
        classification: "local_beta_workflow_only".to_string(),
        current_step: Some(CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured),
        next_required_step: Some(CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured),
        current_blocking_step: Some(CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured),
        current_error: Some(CompleteLocalOperatorWorkflowError::AdapterNotConfigured),
        steps,
        rejection_reasons: Vec::new(),
        evidence_summary: CompleteLocalOperatorWorkflowEvidenceSummary {
            provider_output_pipeline_status: "not_started".to_string(),
            local_candidate_materialization_status: "not_materialized".to_string(),
            replay_status: "no_decision_recorded".to_string(),
            local_evidence_export_status: "no_completed_run_evidence".to_string(),
            session_package_status: "not_packaged".to_string(),
            session_history_status: "no_session_history".to_string(),
            restore_status: "restore_not_requested".to_string(),
        },
        boundary_statuses: complete_local_operator_workflow_boundary_statuses(),
        capability_surface: CompleteLocalOperatorWorkflowCapabilitySurface {
            local_only: true,
            non_production: true,
            provider_trust_granted: false,
            action_execution_authorized: false,
            readiness_approved: false,
            release_approved: false,
            deployment_approved: false,
            public_use_approved: false,
            replay_repair_performed: false,
            recovery_promotion_performed: false,
        },
        local_only_note: "Complete local workflow is local-only and non-production.".to_string(),
        no_authority_note: "Workflow completion does not approve readiness, release, deployment, public use, or production use. Provider output remains untrusted unless a later bounded phase explicitly changes that. Workflow status does not authorize actions. Replay is not repaired and recovery is not promoted.".to_string(),
    }
}

fn complete_local_operator_workflow_evidence_summary(
    state: &LocalOperatorShellState,
) -> CompleteLocalOperatorWorkflowEvidenceSummary {
    CompleteLocalOperatorWorkflowEvidenceSummary {
        provider_output_pipeline_status: state
            .local_provider_output_pipeline
            .status
            .code()
            .to_string(),
        local_candidate_materialization_status: state
            .local_candidate_output
            .status
            .code()
            .to_string(),
        replay_status: state.run.decision_replay.replay_status.code().to_string(),
        local_evidence_export_status: state
            .local_session_evidence_export
            .export_status
            .code()
            .to_string(),
        session_package_status: state
            .local_session_package_projection
            .status
            .code()
            .to_string(),
        session_history_status: state
            .local_session_history_projection
            .status
            .code()
            .to_string(),
        restore_status: state
            .local_session_restore_projection
            .status
            .code()
            .to_string(),
    }
}

fn workflow_step(
    step: CompleteLocalOperatorWorkflowStepKind,
    status: CompleteLocalOperatorWorkflowStepStatus,
    error: Option<CompleteLocalOperatorWorkflowError>,
    summary: impl Into<String>,
) -> CompleteLocalOperatorWorkflowStep {
    CompleteLocalOperatorWorkflowStep {
        step,
        status,
        error,
        summary: summary.into(),
    }
}

pub fn classify_complete_local_operator_workflow_step(
    state: &LocalOperatorShellState,
    step: CompleteLocalOperatorWorkflowStepKind,
) -> CompleteLocalOperatorWorkflowStep {
    match step {
        CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured => {
            match state.local_provider_adapter_registry.last_validation.status {
                LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Provider adapter declaration is accepted.",
                ),
                LocalProviderAdapterValidationStatus::AdapterRejected
                | LocalProviderAdapterValidationStatus::UnsupportedAdapter
                | LocalProviderAdapterValidationStatus::InvalidAdapterDeclaration => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::AdapterNotConfigured),
                    "Provider adapter declaration is rejected or invalid.",
                ),
                LocalProviderAdapterValidationStatus::RegistryProjected => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::AdapterNotConfigured),
                    "Provider adapter declaration is missing.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::AdapterDryRunAvailable => {
            match state.local_provider_adapter_dry_run.status {
                LocalProviderAdapterDryRunStatus::DryRunExecuted => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Controlled adapter dry run has executed.",
                ),
                LocalProviderAdapterDryRunStatus::DryRunRejected
                | LocalProviderAdapterDryRunStatus::UnsupportedAdapter
                | LocalProviderAdapterDryRunStatus::InvalidDryRunRequest => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::InvocationRejected),
                    format!(
                        "Controlled adapter dry run rejected: {}.",
                        state
                            .local_provider_adapter_dry_run
                            .error_codes
                            .iter()
                            .map(|error| error.code())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                ),
                _ if state.local_provider_adapter_registry.last_validation.status
                    == LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting =>
                {
                    workflow_step(
                        step,
                        CompleteLocalOperatorWorkflowStepStatus::Available,
                        None,
                        "Controlled adapter dry run is available.",
                    )
                }
                _ => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::NotStarted,
                    None,
                    "Controlled adapter dry run waits for adapter configuration.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::ConstrainedInvocationCompleted => match state
            .constrained_local_provider_invocation
            .status
        {
            ConstrainedLocalProviderInvocationStatus::InvocationExecuted => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Completed,
                None,
                "Constrained local provider invocation has executed.",
            ),
            ConstrainedLocalProviderInvocationStatus::InvocationRejected
            | ConstrainedLocalProviderInvocationStatus::UnsupportedProvider
            | ConstrainedLocalProviderInvocationStatus::InvalidInvocationRequest => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Rejected,
                Some(CompleteLocalOperatorWorkflowError::InvocationRejected),
                format!(
                    "Constrained local provider invocation rejected: {}.",
                    state
                        .constrained_local_provider_invocation
                        .error_codes
                        .iter()
                        .map(|error| error.code())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ),
            _ if state.local_provider_adapter_registry.last_validation.status
                == LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting =>
            {
                workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::InvocationMissing),
                    "Constrained local provider invocation is missing.",
                )
            }
            _ => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::NotStarted,
                None,
                "Invocation waits for provider adapter configuration.",
            ),
        },
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputPipelineProjected => {
            match state.local_provider_output_pipeline.status {
                LocalProviderOutputPipelineValidationStatus::Valid => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Provider output pipeline is valid.",
                ),
                LocalProviderOutputPipelineValidationStatus::Rejected => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::ProviderPipelineBlocked),
                    format!(
                        "Provider output pipeline blocked: {}.",
                        state
                            .local_provider_output_pipeline
                            .errors
                            .iter()
                            .map(|error| error.code())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                ),
                _ => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::ProviderPipelineBlocked),
                    "Provider output pipeline projection is missing or incomplete.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputValidated => {
            match state.provider_output_validation.status {
                LocalProviderOutputValidationStatus::ReviewableUntrusted => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Provider output validation is reviewable and untrusted.",
                ),
                LocalProviderOutputValidationStatus::Rejected
                | LocalProviderOutputValidationStatus::InvalidValidationInput
                | LocalProviderOutputValidationStatus::ValidationNotApplicable => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::ProviderOutputValidationRejected),
                    format!(
                        "Provider output validation rejected: {}.",
                        state
                            .provider_output_validation
                            .reasons
                            .iter()
                            .map(|reason| reason.code())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                ),
                LocalProviderOutputValidationStatus::NotValidated => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::ProviderOutputValidationMissing),
                    "Provider output validation is missing.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputReviewed => {
            match state.provider_output_validation.reviewability_status {
                LocalProviderOutputReviewabilityStatus::ReviewableUntrusted => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Provider output review surface is projected.",
                ),
                LocalProviderOutputReviewabilityStatus::RejectedBeforeReview => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::ProviderOutputValidationRejected),
                    "Provider output was rejected before review.",
                ),
                LocalProviderOutputReviewabilityStatus::NotReviewable => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::ProviderOutputReviewMissing),
                    "Provider output review is missing.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::StagedProposalCreated => {
            match state.staged_candidate_conversion_proposal.status {
                StagedCandidateConversionProposalStatus::StagedProposalCreated => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Staged candidate-conversion proposal exists.",
                ),
                StagedCandidateConversionProposalStatus::RejectedSourceNotEligible
                | StagedCandidateConversionProposalStatus::InvalidProposalRequest => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::StagedProposalMissing),
                    "Staged proposal creation was rejected.",
                ),
                _ => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::StagedProposalMissing),
                    "Staged candidate-conversion proposal is missing.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::StagedProposalValidated => match state
            .staged_candidate_conversion_validation
            .status
        {
            StagedCandidateConversionValidationStatus::StagedProposalShapeValid => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Completed,
                None,
                "Staged proposal shape and linkage are valid.",
            ),
            StagedCandidateConversionValidationStatus::RejectedStagedProposal
            | StagedCandidateConversionValidationStatus::InvalidValidationInput => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Rejected,
                Some(CompleteLocalOperatorWorkflowError::StagedProposalValidationRejected),
                format!(
                    "Staged proposal validation rejected: {}.",
                    state
                        .staged_candidate_conversion_validation
                        .reasons
                        .iter()
                        .map(|reason| reason.code())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ),
            StagedCandidateConversionValidationStatus::NotValidated => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Blocked,
                Some(CompleteLocalOperatorWorkflowError::StagedProposalValidationMissing),
                "Staged proposal validation is missing.",
            ),
        },
        CompleteLocalOperatorWorkflowStepKind::CandidateReviewProjected => {
            if state.local_provider_output_pipeline.candidate_review_status == "display_only" {
                workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Candidate review surface is projected as display-only.",
                )
            } else {
                workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::CandidateReviewMissing),
                    "Candidate review projection is missing.",
                )
            }
        }
        CompleteLocalOperatorWorkflowStepKind::OperatorDecisionRecorded => {
            match state.operator_candidate_decision.status {
                OperatorCandidateDecisionStatus::ApprovedValidatedStagedProposal => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Operator decision on validated staged proposal is recorded.",
                ),
                OperatorCandidateDecisionStatus::RejectedValidatedStagedProposal
                | OperatorCandidateDecisionStatus::RejectedOperatorDecisionRequest
                | OperatorCandidateDecisionStatus::InvalidOperatorDecisionInput => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::OperatorDecisionRejected),
                    "Operator decision is rejected or invalid.",
                ),
                OperatorCandidateDecisionStatus::NoOperatorDecision => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::OperatorDecisionMissing),
                    "Operator decision is missing.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::LocalCandidateMaterialized => match state
            .local_candidate_output
            .status
        {
            LocalCandidateMaterializationStatus::LocalCandidateMaterialized => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Completed,
                None,
                "Local candidate output is materialized.",
            ),
            LocalCandidateMaterializationStatus::MaterializationRejected
            | LocalCandidateMaterializationStatus::MaterializationPreconditionMissing
            | LocalCandidateMaterializationStatus::InvalidMaterializationRequest => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Rejected,
                Some(CompleteLocalOperatorWorkflowError::LocalCandidateNotMaterialized),
                format!(
                    "Local candidate materialization rejected: {}.",
                    state
                        .local_candidate_output
                        .error
                        .as_ref()
                        .map(|error| error.code())
                        .unwrap_or("unknown")
                ),
            ),
            LocalCandidateMaterializationStatus::NotMaterialized => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Blocked,
                Some(CompleteLocalOperatorWorkflowError::LocalCandidateNotMaterialized),
                "Local candidate output is not materialized.",
            ),
        },
        CompleteLocalOperatorWorkflowStepKind::ReplayStatusProjected => workflow_step(
            step,
            CompleteLocalOperatorWorkflowStepStatus::Completed,
            None,
            format!(
                "Replay/status projection is {}.",
                state.run.decision_replay.replay_status.code()
            ),
        ),
        CompleteLocalOperatorWorkflowStepKind::LocalEvidenceExportProjected => workflow_step(
            step,
            CompleteLocalOperatorWorkflowStepStatus::Completed,
            None,
            format!(
                "Local evidence export is {}.",
                state.local_session_evidence_export.export_status.code()
            ),
        ),
        CompleteLocalOperatorWorkflowStepKind::SessionPackageProjected => workflow_step(
            step,
            CompleteLocalOperatorWorkflowStepStatus::Completed,
            None,
            format!(
                "Local session package status is {}.",
                state.local_session_package_projection.status.code()
            ),
        ),
        CompleteLocalOperatorWorkflowStepKind::RestoreStatusProjected => workflow_step(
            step,
            CompleteLocalOperatorWorkflowStepStatus::Completed,
            None,
            format!(
                "Restore/history status is {} / {}.",
                state.local_session_history_projection.status.code(),
                state.local_session_restore_projection.status.code()
            ),
        ),
    }
}

pub fn complete_local_operator_workflow_current_blocker(
    steps: &[CompleteLocalOperatorWorkflowStep],
) -> Option<CompleteLocalOperatorWorkflowStepKind> {
    steps
        .iter()
        .find(|step| {
            matches!(
                step.status,
                CompleteLocalOperatorWorkflowStepStatus::Blocked
                    | CompleteLocalOperatorWorkflowStepStatus::Rejected
            )
        })
        .map(|step| step.step)
}

pub fn derive_complete_local_operator_workflow_projection(
    state: &LocalOperatorShellState,
) -> CompleteLocalOperatorWorkflowProjection {
    let steps = complete_local_operator_workflow_step_order()
        .into_iter()
        .map(|step| classify_complete_local_operator_workflow_step(state, step))
        .collect::<Vec<_>>();
    let current_blocking_step = complete_local_operator_workflow_current_blocker(&steps);
    let current_error = current_blocking_step.and_then(|blocked| {
        steps
            .iter()
            .find(|step| step.step == blocked)
            .and_then(|step| step.error)
    });
    let rejection_reasons = steps
        .iter()
        .filter(|step| step.status == CompleteLocalOperatorWorkflowStepStatus::Rejected)
        .map(|step| {
            format!(
                "{}: {}",
                step.step.code(),
                step.error.map(|error| error.code()).unwrap_or("rejected")
            )
        })
        .collect::<Vec<_>>();
    let primary_status = if !rejection_reasons.is_empty() {
        CompleteLocalOperatorWorkflowStatus::Rejected
    } else if current_blocking_step.is_some() {
        CompleteLocalOperatorWorkflowStatus::Blocked
    } else if state.local_candidate_output.status
        == LocalCandidateMaterializationStatus::LocalCandidateMaterialized
    {
        CompleteLocalOperatorWorkflowStatus::CompleteLocalWorkflowProjected
    } else if steps
        .iter()
        .any(|step| step.status == CompleteLocalOperatorWorkflowStepStatus::Completed)
    {
        CompleteLocalOperatorWorkflowStatus::InProgress
    } else {
        CompleteLocalOperatorWorkflowStatus::NotStarted
    };
    let current_step = current_blocking_step.or_else(|| {
        steps
            .iter()
            .find(|step| step.status != CompleteLocalOperatorWorkflowStepStatus::Completed)
            .map(|step| step.step)
    });
    CompleteLocalOperatorWorkflowProjection {
        status: primary_status,
        classification: "local_beta_workflow_only".to_string(),
        current_step,
        next_required_step: current_step,
        current_blocking_step,
        current_error,
        steps,
        rejection_reasons,
        evidence_summary: complete_local_operator_workflow_evidence_summary(state),
        boundary_statuses: complete_local_operator_workflow_boundary_statuses(),
        capability_surface: CompleteLocalOperatorWorkflowCapabilitySurface {
            local_only: true,
            non_production: true,
            provider_trust_granted: false,
            action_execution_authorized: false,
            readiness_approved: false,
            release_approved: false,
            deployment_approved: false,
            public_use_approved: false,
            replay_repair_performed: false,
            recovery_promotion_performed: false,
        },
        local_only_note: "Complete local workflow is local-only and non-production.".to_string(),
        no_authority_note: "Workflow completion does not approve readiness, release, deployment, public use, or production use. Provider output remains untrusted unless a later bounded phase explicitly changes that. Workflow status does not authorize actions. Replay is not repaired and recovery is not promoted.".to_string(),
    }
}
