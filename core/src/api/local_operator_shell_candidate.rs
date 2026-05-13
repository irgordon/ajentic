//! Local candidate materialization boundary and projection helpers.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalCandidateMaterializationStatus {
    NotMaterialized,
    LocalCandidateMaterialized,
    MaterializationRejected,
    MaterializationPreconditionMissing,
    InvalidMaterializationRequest,
}

impl LocalCandidateMaterializationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotMaterialized => "not_materialized",
            Self::LocalCandidateMaterialized => "local_candidate_materialized",
            Self::MaterializationRejected => "materialization_rejected",
            Self::MaterializationPreconditionMissing => "materialization_precondition_missing",
            Self::InvalidMaterializationRequest => "invalid_materialization_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalCandidateMaterializationError {
    ProviderOutputValidationMissing,
    ProviderOutputValidationNotReviewableUntrusted,
    ProviderOutputReviewMissing,
    StagedProposalMissing,
    StagedProposalValidationMissing,
    StagedProposalValidationNotValid,
    CandidateReviewMissing,
    OperatorDecisionMissing,
    OperatorDecisionRejected,
    OperatorDecisionNotApproved,
    ProviderPipelineIncomplete,
    ProviderPipelineRejected,
    SourceLinkageMismatch,
    InvocationResultIdMismatch,
    ProviderExecutionResultIdMismatch,
    StagedProposalIdMismatch,
    OperatorDecisionIdMismatch,
    TrustClaimRejected,
    SafetyClaimRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    ProviderOutputApprovalClaimRejected,
    ActionClaimRejected,
    PersistenceClaimRejected,
    ExecutionClaimRejected,
    SigningClaimRejected,
    PublishingClaimRejected,
}

impl LocalCandidateMaterializationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ProviderOutputValidationMissing => "provider_output_validation_missing",
            Self::ProviderOutputValidationNotReviewableUntrusted => {
                "provider_output_validation_not_reviewable_untrusted"
            }
            Self::ProviderOutputReviewMissing => "provider_output_review_missing",
            Self::StagedProposalMissing => "staged_proposal_missing",
            Self::StagedProposalValidationMissing => "staged_proposal_validation_missing",
            Self::StagedProposalValidationNotValid => "staged_proposal_validation_not_valid",
            Self::CandidateReviewMissing => "candidate_review_missing",
            Self::OperatorDecisionMissing => "operator_decision_missing",
            Self::OperatorDecisionRejected => "operator_decision_rejected",
            Self::OperatorDecisionNotApproved => "operator_decision_not_approved",
            Self::ProviderPipelineIncomplete => "provider_pipeline_incomplete",
            Self::ProviderPipelineRejected => "provider_pipeline_rejected",
            Self::SourceLinkageMismatch => "source_linkage_mismatch",
            Self::InvocationResultIdMismatch => "invocation_result_id_mismatch",
            Self::ProviderExecutionResultIdMismatch => "provider_execution_result_id_mismatch",
            Self::StagedProposalIdMismatch => "staged_proposal_id_mismatch",
            Self::OperatorDecisionIdMismatch => "operator_decision_id_mismatch",
            Self::TrustClaimRejected => "trust_claim_rejected",
            Self::SafetyClaimRejected => "safety_claim_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::ProviderOutputApprovalClaimRejected => "provider_output_approval_claim_rejected",
            Self::ActionClaimRejected => "action_claim_rejected",
            Self::PersistenceClaimRejected => "persistence_claim_rejected",
            Self::ExecutionClaimRejected => "execution_claim_rejected",
            Self::SigningClaimRejected => "signing_claim_rejected",
            Self::PublishingClaimRejected => "publishing_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalCandidateMaterializationBoundaryStatus {
    LocalCandidateOutputOnly,
    NonProductionCandidate,
    ProviderOutputRemainsUntrusted,
    NoProviderTrust,
    NoProductionApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoActionExecution,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl LocalCandidateMaterializationBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalCandidateOutputOnly => "local_candidate_output_only",
            Self::NonProductionCandidate => "non_production_candidate",
            Self::ProviderOutputRemainsUntrusted => "provider_output_remains_untrusted",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalCandidateMaterializationEffectStatus {
    NoProviderTrust,
    NoProductionApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoActionExecution,
    NoReplayRepair,
    NoRecoveryPromotion,
    NoFileWrite,
    NoNetworkSocket,
    NoProcessSpawn,
    NoSecretRead,
    NoProviderExecution,
    NoProviderConfigurationMutation,
    NoProviderExecutionMutation,
    NoProviderOutputValidationMutation,
    NoStagedProposalMutation,
    NoOperatorDecisionMutation,
    NoExportPromotion,
}

impl LocalCandidateMaterializationEffectStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
            Self::NoFileWrite => "no_file_write",
            Self::NoNetworkSocket => "no_network_socket",
            Self::NoProcessSpawn => "no_process_spawn",
            Self::NoSecretRead => "no_secret_read",
            Self::NoProviderExecution => "no_provider_execution",
            Self::NoProviderConfigurationMutation => "no_provider_configuration_mutation",
            Self::NoProviderExecutionMutation => "no_provider_execution_mutation",
            Self::NoProviderOutputValidationMutation => "no_provider_output_validation_mutation",
            Self::NoStagedProposalMutation => "no_staged_proposal_mutation",
            Self::NoOperatorDecisionMutation => "no_operator_decision_mutation",
            Self::NoExportPromotion => "no_export_promotion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCandidateMaterializationRequest {
    pub staged_proposal_id: String,
    pub operator_decision_id: String,
    pub provider_execution_result_id: String,
    pub source_invocation_result_id: String,
    pub claims_trust: bool,
    pub claims_safety: bool,
    pub claims_readiness: bool,
    pub claims_release: bool,
    pub claims_deployment: bool,
    pub claims_public_use: bool,
    pub claims_provider_output_approval: bool,
    pub claims_action: bool,
    pub claims_persistence: bool,
    pub claims_execution: bool,
    pub claims_signing: bool,
    pub claims_publishing: bool,
}

impl LocalCandidateMaterializationRequest {
    pub fn from_validated_state(state: &LocalOperatorShellState) -> Option<Self> {
        let proposal = state
            .staged_candidate_conversion_proposal
            .proposal
            .as_ref()?;
        let decision = state.operator_candidate_decision.record.as_ref()?;
        Some(Self {
            staged_proposal_id: proposal.proposal_id.clone(),
            operator_decision_id: decision.decision_id.clone(),
            provider_execution_result_id: proposal.source_execution_result_id.clone(),
            source_invocation_result_id: state
                .local_provider_output_pipeline
                .source_invocation_result_id
                .clone()
                .unwrap_or_default(),
            claims_trust: false,
            claims_safety: false,
            claims_readiness: false,
            claims_release: false,
            claims_deployment: false,
            claims_public_use: false,
            claims_provider_output_approval: false,
            claims_action: false,
            claims_persistence: false,
            claims_execution: false,
            claims_signing: false,
            claims_publishing: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCandidateOutputSourceLinkage {
    pub source_invocation_result_id: String,
    pub provider_execution_result_id: String,
    pub provider_output_validation_status: LocalProviderOutputValidationStatus,
    pub provider_output_review_status: LocalProviderOutputReviewabilityStatus,
    pub staged_proposal_id: String,
    pub staged_proposal_validation_status: StagedCandidateConversionValidationStatus,
    pub operator_decision_id: String,
    pub operator_decision_status: OperatorCandidateDecisionStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCandidateMaterializationCapabilitySurface {
    pub local_only: bool,
    pub non_production: bool,
    pub materializes_candidate_output: bool,
    pub provider_trust_enabled: bool,
    pub action_execution_enabled: bool,
    pub production_approval_enabled: bool,
    pub release_approval_enabled: bool,
    pub deployment_approval_enabled: bool,
    pub public_use_approval_enabled: bool,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCandidateOutputProjection {
    pub status: LocalCandidateMaterializationStatus,
    pub candidate_id: Option<String>,
    pub content_summary: Option<String>,
    pub output_classification: String,
    pub production_classification: String,
    pub source_linkage: Option<LocalCandidateOutputSourceLinkage>,
    pub provider_output_trust_carry_forward: String,
    pub boundary_statuses: Vec<LocalCandidateMaterializationBoundaryStatus>,
    pub effect_statuses: Vec<LocalCandidateMaterializationEffectStatus>,
    pub error: Option<LocalCandidateMaterializationError>,
    pub capability_surface: LocalCandidateMaterializationCapabilitySurface,
    pub note: String,
}

pub fn local_candidate_materialization_boundary_statuses(
) -> Vec<LocalCandidateMaterializationBoundaryStatus> {
    vec![
        LocalCandidateMaterializationBoundaryStatus::LocalCandidateOutputOnly,
        LocalCandidateMaterializationBoundaryStatus::NonProductionCandidate,
        LocalCandidateMaterializationBoundaryStatus::ProviderOutputRemainsUntrusted,
        LocalCandidateMaterializationBoundaryStatus::NoProviderTrust,
        LocalCandidateMaterializationBoundaryStatus::NoProductionApproval,
        LocalCandidateMaterializationBoundaryStatus::NoReleaseApproval,
        LocalCandidateMaterializationBoundaryStatus::NoDeploymentApproval,
        LocalCandidateMaterializationBoundaryStatus::NoPublicUseApproval,
        LocalCandidateMaterializationBoundaryStatus::NoActionExecution,
        LocalCandidateMaterializationBoundaryStatus::NoReplayRepair,
        LocalCandidateMaterializationBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub fn local_candidate_materialization_effect_statuses(
) -> Vec<LocalCandidateMaterializationEffectStatus> {
    vec![
        LocalCandidateMaterializationEffectStatus::NoProviderTrust,
        LocalCandidateMaterializationEffectStatus::NoProductionApproval,
        LocalCandidateMaterializationEffectStatus::NoReleaseApproval,
        LocalCandidateMaterializationEffectStatus::NoDeploymentApproval,
        LocalCandidateMaterializationEffectStatus::NoPublicUseApproval,
        LocalCandidateMaterializationEffectStatus::NoActionExecution,
        LocalCandidateMaterializationEffectStatus::NoReplayRepair,
        LocalCandidateMaterializationEffectStatus::NoRecoveryPromotion,
        LocalCandidateMaterializationEffectStatus::NoFileWrite,
        LocalCandidateMaterializationEffectStatus::NoNetworkSocket,
        LocalCandidateMaterializationEffectStatus::NoProcessSpawn,
        LocalCandidateMaterializationEffectStatus::NoSecretRead,
        LocalCandidateMaterializationEffectStatus::NoProviderExecution,
        LocalCandidateMaterializationEffectStatus::NoProviderConfigurationMutation,
        LocalCandidateMaterializationEffectStatus::NoProviderExecutionMutation,
        LocalCandidateMaterializationEffectStatus::NoProviderOutputValidationMutation,
        LocalCandidateMaterializationEffectStatus::NoStagedProposalMutation,
        LocalCandidateMaterializationEffectStatus::NoOperatorDecisionMutation,
        LocalCandidateMaterializationEffectStatus::NoExportPromotion,
    ]
}

pub fn local_candidate_materialization_capability_surface(
) -> LocalCandidateMaterializationCapabilitySurface {
    LocalCandidateMaterializationCapabilitySurface {
        local_only: true,
        non_production: true,
        materializes_candidate_output: true,
        provider_trust_enabled: false,
        action_execution_enabled: false,
        production_approval_enabled: false,
        release_approval_enabled: false,
        deployment_approval_enabled: false,
        public_use_approval_enabled: false,
        summary: "Local candidate output materialization only; non-production; provider output remains untrusted; no provider trust, action, production, readiness, release, deployment, or public-use approval.".to_string(),
    }
}

pub fn initial_local_candidate_output_projection() -> LocalCandidateOutputProjection {
    LocalCandidateOutputProjection {
        status: LocalCandidateMaterializationStatus::NotMaterialized,
        candidate_id: None,
        content_summary: None,
        output_classification: "local_candidate_output_only".to_string(),
        production_classification: "non_production_candidate".to_string(),
        source_linkage: None,
        provider_output_trust_carry_forward: "provider_output_remains_untrusted".to_string(),
        boundary_statuses: local_candidate_materialization_boundary_statuses(),
        effect_statuses: local_candidate_materialization_effect_statuses(),
        error: None,
        capability_surface: local_candidate_materialization_capability_surface(),
        note: "Local candidate output only. This candidate output is non-production. Provider output remains untrusted. Candidate materialization does not approve readiness, release, deployment, or public use. Candidate materialization does not authorize actions. Production approval remains unavailable.".to_string(),
    }
}

fn stable_local_candidate_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub fn derive_local_candidate_output_content(
    state: &LocalOperatorShellState,
    linkage: &LocalCandidateOutputSourceLinkage,
) -> String {
    let proposal = state
        .staged_candidate_conversion_proposal
        .proposal
        .as_ref()
        .expect("validated materialization requires staged proposal");
    let output_summary = state
        .provider_execution
        .result
        .as_ref()
        .map(|result| result.output_summary.as_str())
        .unwrap_or("none");
    format!(
        "Local candidate output only. staged_proposal={} operator_decision={} provider_execution={} invocation={} validation={} review={} source_summary={}",
        linkage.staged_proposal_id,
        linkage.operator_decision_id,
        linkage.provider_execution_result_id,
        linkage.source_invocation_result_id,
        linkage.provider_output_validation_status.code(),
        linkage.provider_output_review_status.code(),
        output_summary
    ) + &format!(" proposal_boundary={}", proposal.proposal_boundary)
}

pub fn validate_local_candidate_materialization_request(
    state: &LocalOperatorShellState,
    request: &LocalCandidateMaterializationRequest,
) -> Result<LocalCandidateOutputSourceLinkage, LocalCandidateMaterializationError> {
    if request.claims_trust {
        return Err(LocalCandidateMaterializationError::TrustClaimRejected);
    }
    if request.claims_safety {
        return Err(LocalCandidateMaterializationError::SafetyClaimRejected);
    }
    if request.claims_readiness {
        return Err(LocalCandidateMaterializationError::ReadinessClaimRejected);
    }
    if request.claims_release {
        return Err(LocalCandidateMaterializationError::ReleaseClaimRejected);
    }
    if request.claims_deployment {
        return Err(LocalCandidateMaterializationError::DeploymentClaimRejected);
    }
    if request.claims_public_use {
        return Err(LocalCandidateMaterializationError::PublicUseClaimRejected);
    }
    if request.claims_provider_output_approval {
        return Err(LocalCandidateMaterializationError::ProviderOutputApprovalClaimRejected);
    }
    if request.claims_action {
        return Err(LocalCandidateMaterializationError::ActionClaimRejected);
    }
    if request.claims_persistence {
        return Err(LocalCandidateMaterializationError::PersistenceClaimRejected);
    }
    if request.claims_execution {
        return Err(LocalCandidateMaterializationError::ExecutionClaimRejected);
    }
    if request.claims_signing {
        return Err(LocalCandidateMaterializationError::SigningClaimRejected);
    }
    if request.claims_publishing {
        return Err(LocalCandidateMaterializationError::PublishingClaimRejected);
    }
    if state.local_provider_output_pipeline.status
        == LocalProviderOutputPipelineValidationStatus::Rejected
    {
        return Err(LocalCandidateMaterializationError::ProviderPipelineRejected);
    }
    if state.local_provider_output_pipeline.status
        != LocalProviderOutputPipelineValidationStatus::Valid
    {
        return Err(LocalCandidateMaterializationError::ProviderPipelineIncomplete);
    }
    validate_provider_output_pipeline_stage_order(&state.local_provider_output_pipeline)
        .map_err(|_| LocalCandidateMaterializationError::ProviderPipelineIncomplete)?;
    if state.provider_output_validation.status == LocalProviderOutputValidationStatus::NotValidated
    {
        return Err(LocalCandidateMaterializationError::ProviderOutputValidationMissing);
    }
    if state.provider_output_validation.status
        != LocalProviderOutputValidationStatus::ReviewableUntrusted
    {
        return Err(
            LocalCandidateMaterializationError::ProviderOutputValidationNotReviewableUntrusted,
        );
    }
    if state.provider_output_validation.reviewability_status
        != LocalProviderOutputReviewabilityStatus::ReviewableUntrusted
    {
        return Err(LocalCandidateMaterializationError::ProviderOutputReviewMissing);
    }
    let Some(proposal) = state.staged_candidate_conversion_proposal.proposal.as_ref() else {
        return Err(LocalCandidateMaterializationError::StagedProposalMissing);
    };
    if state.staged_candidate_conversion_proposal.status
        != StagedCandidateConversionProposalStatus::StagedProposalCreated
    {
        return Err(LocalCandidateMaterializationError::StagedProposalMissing);
    }
    match state.staged_candidate_conversion_validation.status {
        StagedCandidateConversionValidationStatus::NotValidated => {
            return Err(LocalCandidateMaterializationError::StagedProposalValidationMissing)
        }
        StagedCandidateConversionValidationStatus::StagedProposalShapeValid => {}
        StagedCandidateConversionValidationStatus::RejectedStagedProposal
        | StagedCandidateConversionValidationStatus::InvalidValidationInput => {
            return Err(LocalCandidateMaterializationError::StagedProposalValidationNotValid)
        }
    }
    if state.local_provider_output_pipeline.candidate_review_status != "display_only" {
        return Err(LocalCandidateMaterializationError::CandidateReviewMissing);
    }
    match state.operator_candidate_decision.status {
        OperatorCandidateDecisionStatus::NoOperatorDecision => {
            return Err(LocalCandidateMaterializationError::OperatorDecisionMissing)
        }
        OperatorCandidateDecisionStatus::RejectedValidatedStagedProposal => {
            return Err(LocalCandidateMaterializationError::OperatorDecisionRejected)
        }
        OperatorCandidateDecisionStatus::ApprovedValidatedStagedProposal => {}
        OperatorCandidateDecisionStatus::RejectedOperatorDecisionRequest
        | OperatorCandidateDecisionStatus::InvalidOperatorDecisionInput => {
            return Err(LocalCandidateMaterializationError::OperatorDecisionNotApproved)
        }
    }
    let Some(decision) = state.operator_candidate_decision.record.as_ref() else {
        return Err(LocalCandidateMaterializationError::OperatorDecisionMissing);
    };
    if request.staged_proposal_id != proposal.proposal_id
        || request.staged_proposal_id != decision.staged_proposal_id
        || Some(&request.staged_proposal_id)
            != state
                .staged_candidate_conversion_validation
                .proposal_id
                .as_ref()
    {
        return Err(LocalCandidateMaterializationError::StagedProposalIdMismatch);
    }
    if request.operator_decision_id != decision.decision_id {
        return Err(LocalCandidateMaterializationError::OperatorDecisionIdMismatch);
    }
    let provider_result_id = state
        .provider_execution
        .result
        .as_ref()
        .map(|result| result.result_id.as_str())
        .ok_or(LocalCandidateMaterializationError::ProviderExecutionResultIdMismatch)?;
    if request.provider_execution_result_id != provider_result_id
        || request.provider_execution_result_id != proposal.source_execution_result_id
        || request.provider_execution_result_id != decision.provider_execution_result_id
        || Some(&request.provider_execution_result_id)
            != state
                .provider_output_validation
                .provider_execution_result_id
                .as_ref()
        || Some(&request.provider_execution_result_id)
            != state
                .staged_candidate_conversion_validation
                .source_execution_result_id
                .as_ref()
        || Some(&request.provider_execution_result_id)
            != state
                .local_provider_output_pipeline
                .provider_execution_result_id
                .as_ref()
    {
        return Err(LocalCandidateMaterializationError::ProviderExecutionResultIdMismatch);
    }
    let invocation_result_id = state
        .constrained_local_provider_invocation
        .result
        .as_ref()
        .map(|result| result.result_id.as_str())
        .ok_or(LocalCandidateMaterializationError::InvocationResultIdMismatch)?;
    if request.source_invocation_result_id != invocation_result_id
        || Some(&request.source_invocation_result_id)
            != state
                .local_provider_output_pipeline
                .source_invocation_result_id
                .as_ref()
    {
        return Err(LocalCandidateMaterializationError::InvocationResultIdMismatch);
    }
    let reprojected_validation = project_staged_candidate_conversion_validation(
        state,
        &StagedCandidateConversionValidationRequest {
            proposal_id: Some(proposal.proposal_id.clone()),
        },
    );
    if reprojected_validation != state.staged_candidate_conversion_validation {
        return Err(LocalCandidateMaterializationError::SourceLinkageMismatch);
    }
    Ok(LocalCandidateOutputSourceLinkage {
        source_invocation_result_id: request.source_invocation_result_id.clone(),
        provider_execution_result_id: request.provider_execution_result_id.clone(),
        provider_output_validation_status: state.provider_output_validation.status,
        provider_output_review_status: state.provider_output_validation.reviewability_status,
        staged_proposal_id: request.staged_proposal_id.clone(),
        staged_proposal_validation_status: state.staged_candidate_conversion_validation.status,
        operator_decision_id: request.operator_decision_id.clone(),
        operator_decision_status: state.operator_candidate_decision.status,
    })
}

pub fn reject_local_candidate_materialization(
    previous: &LocalCandidateOutputProjection,
    error: LocalCandidateMaterializationError,
) -> LocalCandidateOutputProjection {
    let mut projection = previous.clone();
    projection.status = match error {
        LocalCandidateMaterializationError::TrustClaimRejected
        | LocalCandidateMaterializationError::SafetyClaimRejected
        | LocalCandidateMaterializationError::ReadinessClaimRejected
        | LocalCandidateMaterializationError::ReleaseClaimRejected
        | LocalCandidateMaterializationError::DeploymentClaimRejected
        | LocalCandidateMaterializationError::PublicUseClaimRejected
        | LocalCandidateMaterializationError::ProviderOutputApprovalClaimRejected
        | LocalCandidateMaterializationError::ActionClaimRejected
        | LocalCandidateMaterializationError::PersistenceClaimRejected
        | LocalCandidateMaterializationError::ExecutionClaimRejected
        | LocalCandidateMaterializationError::SigningClaimRejected
        | LocalCandidateMaterializationError::PublishingClaimRejected => {
            LocalCandidateMaterializationStatus::InvalidMaterializationRequest
        }
        _ => LocalCandidateMaterializationStatus::MaterializationRejected,
    };
    if matches!(
        error,
        LocalCandidateMaterializationError::OperatorDecisionMissing
            | LocalCandidateMaterializationError::ProviderOutputValidationMissing
            | LocalCandidateMaterializationError::ProviderOutputReviewMissing
            | LocalCandidateMaterializationError::StagedProposalMissing
            | LocalCandidateMaterializationError::StagedProposalValidationMissing
            | LocalCandidateMaterializationError::CandidateReviewMissing
            | LocalCandidateMaterializationError::ProviderPipelineIncomplete
    ) {
        projection.status = LocalCandidateMaterializationStatus::MaterializationPreconditionMissing;
    }
    projection.error = Some(error);
    projection.note = format!(
        "Local candidate materialization rejected: {}. Local candidate output only; provider output remains untrusted; no production, release, deployment, public-use, or action authorization effect occurred.",
        error.code()
    );
    projection
}

pub fn project_local_candidate_output(
    state: &LocalOperatorShellState,
    request: &LocalCandidateMaterializationRequest,
) -> Result<LocalCandidateOutputProjection, LocalCandidateMaterializationError> {
    let linkage = validate_local_candidate_materialization_request(state, request)?;
    let content_summary = derive_local_candidate_output_content(state, &linkage);
    let digest = stable_local_candidate_digest(&format!(
        "phase_158|{}|{}|{}|{}|{}",
        linkage.source_invocation_result_id,
        linkage.provider_execution_result_id,
        linkage.staged_proposal_id,
        linkage.operator_decision_id,
        content_summary
    ));
    Ok(LocalCandidateOutputProjection {
        status: LocalCandidateMaterializationStatus::LocalCandidateMaterialized,
        candidate_id: Some(format!("local-candidate-output-{digest}")),
        content_summary: Some(content_summary),
        output_classification: "local_candidate_output_only".to_string(),
        production_classification: "non_production_candidate".to_string(),
        source_linkage: Some(linkage),
        provider_output_trust_carry_forward: "provider_output_remains_untrusted".to_string(),
        boundary_statuses: local_candidate_materialization_boundary_statuses(),
        effect_statuses: local_candidate_materialization_effect_statuses(),
        error: None,
        capability_surface: local_candidate_materialization_capability_surface(),
        note: "Local candidate output only. This candidate output is non-production. Provider output remains untrusted. Candidate materialization does not approve readiness, release, deployment, or public use. Candidate materialization does not authorize actions. Production approval remains unavailable.".to_string(),
    })
}

pub fn materialize_local_candidate_output(
    state: &LocalOperatorShellState,
    request: LocalCandidateMaterializationRequest,
) -> Result<LocalOperatorShellState, LocalCandidateMaterializationError> {
    let projection = project_local_candidate_output(state, &request)?;
    let mut next = state.clone();
    next.local_candidate_output = projection;
    Ok(attach_local_session_evidence_export(next))
}
