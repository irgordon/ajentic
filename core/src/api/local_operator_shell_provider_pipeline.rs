//! Provider output pipeline projections for the local operator shell.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputPipelineSourceKind {
    DeterministicStubProviderExecution,
    ConstrainedLocalProviderInvocation,
}

impl LocalProviderOutputPipelineSourceKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DeterministicStubProviderExecution => "deterministic_stub_provider_execution",
            Self::ConstrainedLocalProviderInvocation => "constrained_local_provider_invocation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalProviderOutputPipelineStage {
    InvocationOutputProjected,
    ProviderExecutionResultProjected,
    ProviderOutputValidationRequired,
    ProviderOutputValidationProjected,
    ProviderOutputReviewRequired,
    ProviderOutputReviewProjected,
    StagedProposalRequired,
    StagedProposalProjected,
    StagedProposalValidationRequired,
    StagedProposalValidationProjected,
    CandidateReviewRequired,
    CandidateReviewProjected,
    OperatorDecisionRequired,
    OperatorDecisionProjected,
}

impl LocalProviderOutputPipelineStage {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvocationOutputProjected => "invocation_output_projected",
            Self::ProviderExecutionResultProjected => "provider_execution_result_projected",
            Self::ProviderOutputValidationRequired => "provider_output_validation_required",
            Self::ProviderOutputValidationProjected => "provider_output_validation_projected",
            Self::ProviderOutputReviewRequired => "provider_output_review_required",
            Self::ProviderOutputReviewProjected => "provider_output_review_projected",
            Self::StagedProposalRequired => "staged_conversion_required",
            Self::StagedProposalProjected => "staged_proposal_projected",
            Self::StagedProposalValidationRequired => "staged_validation_required",
            Self::StagedProposalValidationProjected => "staged_proposal_validation_projected",
            Self::CandidateReviewRequired => "candidate_review_required",
            Self::CandidateReviewProjected => "candidate_review_projected",
            Self::OperatorDecisionRequired => "operator_decision_required",
            Self::OperatorDecisionProjected => "operator_decision_projected",
        }
    }
}

pub fn local_provider_output_pipeline_stage_order() -> Vec<LocalProviderOutputPipelineStage> {
    vec![
        LocalProviderOutputPipelineStage::InvocationOutputProjected,
        LocalProviderOutputPipelineStage::ProviderExecutionResultProjected,
        LocalProviderOutputPipelineStage::ProviderOutputValidationRequired,
        LocalProviderOutputPipelineStage::ProviderOutputValidationProjected,
        LocalProviderOutputPipelineStage::ProviderOutputReviewRequired,
        LocalProviderOutputPipelineStage::ProviderOutputReviewProjected,
        LocalProviderOutputPipelineStage::StagedProposalRequired,
        LocalProviderOutputPipelineStage::StagedProposalProjected,
        LocalProviderOutputPipelineStage::StagedProposalValidationRequired,
        LocalProviderOutputPipelineStage::StagedProposalValidationProjected,
        LocalProviderOutputPipelineStage::CandidateReviewRequired,
        LocalProviderOutputPipelineStage::CandidateReviewProjected,
        LocalProviderOutputPipelineStage::OperatorDecisionRequired,
        LocalProviderOutputPipelineStage::OperatorDecisionProjected,
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputPipelineStageStatus {
    NotStarted,
    Available,
    Blocked,
    Completed,
    Rejected,
    NotApplicable,
}

impl LocalProviderOutputPipelineStageStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Available => "available",
            Self::Blocked => "blocked",
            Self::Completed => "completed",
            Self::Rejected => "rejected",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputPipelineValidationStatus {
    NotStarted,
    Valid,
    Blocked,
    Rejected,
}

impl LocalProviderOutputPipelineValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Valid => "valid",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalProviderOutputPipelineError {
    NoInvocationOutput,
    InvocationOutputRejected,
    InvocationOutputNotUntrustedDescriptive,
    InvocationResultIdMismatch,
    InvocationOutputSummaryMismatch,
    ProviderOutputValidationMissing,
    ProviderOutputValidationRejected,
    ProviderOutputReviewMissing,
    StagedProposalMissing,
    StagedProposalValidationMissing,
    StagedProposalValidationRejected,
    CandidateReviewMissing,
    OperatorDecisionMissing,
    PipelineSkipAttemptRejected,
    TrustClaimRejected,
    ApprovalClaimRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    CandidateCreationClaimRejected,
    CandidateMaterializationClaimRejected,
    ActionClaimRejected,
    PersistenceClaimRejected,
}

impl LocalProviderOutputPipelineError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoInvocationOutput => "no_invocation_output",
            Self::InvocationOutputRejected => "invocation_output_rejected",
            Self::InvocationOutputNotUntrustedDescriptive => {
                "invocation_output_not_untrusted_descriptive"
            }
            Self::InvocationResultIdMismatch => "invocation_result_id_mismatch",
            Self::InvocationOutputSummaryMismatch => "invocation_output_summary_mismatch",
            Self::ProviderOutputValidationMissing => "provider_output_validation_missing",
            Self::ProviderOutputValidationRejected => "provider_output_validation_rejected",
            Self::ProviderOutputReviewMissing => "provider_output_review_missing",
            Self::StagedProposalMissing => "staged_proposal_missing",
            Self::StagedProposalValidationMissing => "staged_proposal_validation_missing",
            Self::StagedProposalValidationRejected => "staged_proposal_validation_rejected",
            Self::CandidateReviewMissing => "candidate_review_missing",
            Self::OperatorDecisionMissing => "operator_decision_missing",
            Self::PipelineSkipAttemptRejected => "pipeline_skip_attempt_rejected",
            Self::TrustClaimRejected => "trust_claim_rejected",
            Self::ApprovalClaimRejected => "approval_claim_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::CandidateCreationClaimRejected => "candidate_creation_claim_rejected",
            Self::CandidateMaterializationClaimRejected => {
                "candidate_materialization_claim_rejected"
            }
            Self::ActionClaimRejected => "action_claim_rejected",
            Self::PersistenceClaimRejected => "persistence_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputPipelineBoundaryStatus {
    PipelineIntegrationOnly,
    UntrustedDescriptiveSource,
    ValidationRequired,
    ReviewRequired,
    StagedConversionRequired,
    StagedValidationRequired,
    CandidateReviewRequired,
    OperatorDecisionRequired,
    NoCandidateMaterialization,
    NoProviderTrust,
    NoActionExecution,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl LocalProviderOutputPipelineBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::PipelineIntegrationOnly => "pipeline_integration_only",
            Self::UntrustedDescriptiveSource => "untrusted_descriptive_source",
            Self::ValidationRequired => "validation_required",
            Self::ReviewRequired => "review_required",
            Self::StagedConversionRequired => "staged_conversion_required",
            Self::StagedValidationRequired => "staged_validation_required",
            Self::CandidateReviewRequired => "candidate_review_required",
            Self::OperatorDecisionRequired => "operator_decision_required",
            Self::NoCandidateMaterialization => "no_candidate_materialization",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NoPublicUseEffect => "no_public_use_effect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputPipelineEffectStatus {
    NoProviderExecution,
    NoCandidateCreation,
    NoCandidateMaterialization,
    NoDecisionLedgerAppend,
    NoReplayMutation,
    NoExportPromotion,
    NoProviderTrust,
    NoFileWrite,
    NoNetworkSocket,
    NoProcessSpawn,
    NoSecretRead,
    NoActionExecution,
}

impl LocalProviderOutputPipelineEffectStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoProviderExecution => "no_provider_execution",
            Self::NoCandidateCreation => "no_candidate_creation",
            Self::NoCandidateMaterialization => "no_candidate_materialization",
            Self::NoDecisionLedgerAppend => "no_decision_ledger_append",
            Self::NoReplayMutation => "no_replay_mutation",
            Self::NoExportPromotion => "no_export_promotion",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoFileWrite => "no_file_write",
            Self::NoNetworkSocket => "no_network_socket",
            Self::NoProcessSpawn => "no_process_spawn",
            Self::NoSecretRead => "no_secret_read",
            Self::NoActionExecution => "no_action_execution",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderOutputPipelineStageProjection {
    pub stage: LocalProviderOutputPipelineStage,
    pub status: LocalProviderOutputPipelineStageStatus,
    pub reason: Option<LocalProviderOutputPipelineError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvocationProviderOutputBridgeProjection {
    pub source_kind: LocalProviderOutputPipelineSourceKind,
    pub invocation_result_id: String,
    pub provider_execution_result_id: String,
    pub output_summary: String,
    pub output_trust_status: LocalProviderOutputTrustStatus,
    pub output_materialization_status: LocalProviderOutputMaterializationStatus,
    pub output_promotion_status: LocalProviderOutputPromotionStatus,
    pub descriptive_only: bool,
    pub not_candidate_material: bool,
    pub not_promoted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderOutputPipelineProjection {
    pub status: LocalProviderOutputPipelineValidationStatus,
    pub source_kind: Option<LocalProviderOutputPipelineSourceKind>,
    pub source_invocation_result_id: Option<String>,
    pub provider_execution_result_id: Option<String>,
    pub current_stage: Option<LocalProviderOutputPipelineStage>,
    pub next_required_stage: Option<LocalProviderOutputPipelineStage>,
    pub stages: Vec<LocalProviderOutputPipelineStageProjection>,
    pub provider_output_validation_status: LocalProviderOutputValidationStatus,
    pub provider_output_review_status: LocalProviderOutputReviewabilityStatus,
    pub staged_proposal_status: StagedCandidateConversionProposalStatus,
    pub staged_proposal_validation_status: StagedCandidateConversionValidationStatus,
    pub candidate_review_status: String,
    pub operator_decision_status: OperatorCandidateDecisionStatus,
    pub boundary_statuses: Vec<LocalProviderOutputPipelineBoundaryStatus>,
    pub effect_statuses: Vec<LocalProviderOutputPipelineEffectStatus>,
    pub errors: Vec<LocalProviderOutputPipelineError>,
    pub bridge: Option<InvocationProviderOutputBridgeProjection>,
    pub note: String,
}

pub fn local_provider_output_pipeline_boundary_statuses(
) -> Vec<LocalProviderOutputPipelineBoundaryStatus> {
    vec![
        LocalProviderOutputPipelineBoundaryStatus::PipelineIntegrationOnly,
        LocalProviderOutputPipelineBoundaryStatus::UntrustedDescriptiveSource,
        LocalProviderOutputPipelineBoundaryStatus::ValidationRequired,
        LocalProviderOutputPipelineBoundaryStatus::ReviewRequired,
        LocalProviderOutputPipelineBoundaryStatus::StagedConversionRequired,
        LocalProviderOutputPipelineBoundaryStatus::StagedValidationRequired,
        LocalProviderOutputPipelineBoundaryStatus::CandidateReviewRequired,
        LocalProviderOutputPipelineBoundaryStatus::OperatorDecisionRequired,
        LocalProviderOutputPipelineBoundaryStatus::NoCandidateMaterialization,
        LocalProviderOutputPipelineBoundaryStatus::NoProviderTrust,
        LocalProviderOutputPipelineBoundaryStatus::NoActionExecution,
        LocalProviderOutputPipelineBoundaryStatus::NoReadinessEffect,
        LocalProviderOutputPipelineBoundaryStatus::NoReleaseEffect,
        LocalProviderOutputPipelineBoundaryStatus::NoDeploymentEffect,
        LocalProviderOutputPipelineBoundaryStatus::NoPublicUseEffect,
    ]
}

pub fn local_provider_output_pipeline_effect_statuses(
) -> Vec<LocalProviderOutputPipelineEffectStatus> {
    vec![
        LocalProviderOutputPipelineEffectStatus::NoProviderExecution,
        LocalProviderOutputPipelineEffectStatus::NoCandidateCreation,
        LocalProviderOutputPipelineEffectStatus::NoCandidateMaterialization,
        LocalProviderOutputPipelineEffectStatus::NoDecisionLedgerAppend,
        LocalProviderOutputPipelineEffectStatus::NoReplayMutation,
        LocalProviderOutputPipelineEffectStatus::NoExportPromotion,
        LocalProviderOutputPipelineEffectStatus::NoProviderTrust,
        LocalProviderOutputPipelineEffectStatus::NoFileWrite,
        LocalProviderOutputPipelineEffectStatus::NoNetworkSocket,
        LocalProviderOutputPipelineEffectStatus::NoProcessSpawn,
        LocalProviderOutputPipelineEffectStatus::NoSecretRead,
        LocalProviderOutputPipelineEffectStatus::NoActionExecution,
    ]
}

pub fn initial_local_provider_output_pipeline_projection() -> LocalProviderOutputPipelineProjection
{
    LocalProviderOutputPipelineProjection {
        status: LocalProviderOutputPipelineValidationStatus::NotStarted,
        source_kind: None,
        source_invocation_result_id: None,
        provider_execution_result_id: None,
        current_stage: None,
        next_required_stage: Some(LocalProviderOutputPipelineStage::InvocationOutputProjected),
        stages: local_provider_output_pipeline_stage_order()
            .into_iter()
            .map(|stage| LocalProviderOutputPipelineStageProjection {
                stage,
                status: LocalProviderOutputPipelineStageStatus::NotStarted,
                reason: None,
            })
            .collect(),
        provider_output_validation_status: LocalProviderOutputValidationStatus::NotValidated,
        provider_output_review_status: LocalProviderOutputReviewabilityStatus::NotReviewable,
        staged_proposal_status: StagedCandidateConversionProposalStatus::NoProposal,
        staged_proposal_validation_status: StagedCandidateConversionValidationStatus::NotValidated,
        candidate_review_status: "not_available".to_string(),
        operator_decision_status: OperatorCandidateDecisionStatus::NoOperatorDecision,
        boundary_statuses: local_provider_output_pipeline_boundary_statuses(),
        effect_statuses: local_provider_output_pipeline_effect_statuses(),
        errors: Vec::new(),
        bridge: None,
        note: "Provider output pipeline integration has not started; invocation output remains untrusted and descriptive.".to_string(),
    }
}

fn invocation_output_pipeline_claim_errors(text: &str) -> Vec<LocalProviderOutputPipelineError> {
    let lower = text.to_ascii_lowercase();
    let mut errors = std::collections::BTreeSet::new();
    if [
        "trust",
        "trusted",
        "provider_output_approval",
        "approved output",
        "approval granted",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::TrustClaimRejected);
        errors.insert(LocalProviderOutputPipelineError::ApprovalClaimRejected);
    }
    if ["readiness", "ready for"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::ReadinessClaimRejected);
    }
    if lower.contains("release") {
        errors.insert(LocalProviderOutputPipelineError::ReleaseClaimRejected);
    }
    if ["deployment", "deploy"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::DeploymentClaimRejected);
    }
    if ["public_use", "public-use", "public use"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::PublicUseClaimRejected);
    }
    if ["candidate creation", "candidate_output"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::CandidateCreationClaimRejected);
    }
    if ["candidate", "materialization"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::CandidateMaterializationClaimRejected);
    }
    if lower.contains("action") {
        errors.insert(LocalProviderOutputPipelineError::ActionClaimRejected);
    }
    if ["persist", "persistence"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::PersistenceClaimRejected);
    }
    errors.into_iter().collect()
}

pub fn project_invocation_output_into_provider_pipeline(
    state: &LocalOperatorShellState,
) -> Result<InvocationProviderOutputBridgeProjection, Vec<LocalProviderOutputPipelineError>> {
    let invocation = &state.constrained_local_provider_invocation;
    if invocation.status == ConstrainedLocalProviderInvocationStatus::NotInvoked {
        return Err(vec![LocalProviderOutputPipelineError::NoInvocationOutput]);
    }
    if invocation.status != ConstrainedLocalProviderInvocationStatus::InvocationExecuted {
        return Err(vec![
            LocalProviderOutputPipelineError::InvocationOutputRejected,
        ]);
    }
    let Some(result) = invocation.result.as_ref() else {
        return Err(vec![LocalProviderOutputPipelineError::NoInvocationOutput]);
    };
    let mut errors = std::collections::BTreeSet::new();
    if result.output_trust_status
        != ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive
        || invocation.output_trust_status
            != ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive
    {
        errors.insert(LocalProviderOutputPipelineError::InvocationOutputNotUntrustedDescriptive);
    }
    if !result
        .result_id
        .starts_with("constrained-local-provider-invocation-")
        || result.result_id.len() <= "constrained-local-provider-invocation-".len()
    {
        errors.insert(LocalProviderOutputPipelineError::InvocationResultIdMismatch);
    }
    let checksum = result
        .result_id
        .strip_prefix("constrained-local-provider-invocation-")
        .unwrap_or("");
    if !result
        .output_summary
        .starts_with("allowlisted_local_deterministic_provider descriptive output for input_bytes=")
        || !result.output_summary.contains(" checksum=")
        || !result.output_summary.ends_with(checksum)
    {
        errors.insert(LocalProviderOutputPipelineError::InvocationOutputSummaryMismatch);
    }
    for error in invocation_output_pipeline_claim_errors(&result.output_summary) {
        errors.insert(error);
    }
    for required in constrained_local_provider_invocation_boundary_statuses() {
        if !result.boundary_statuses.contains(&required)
            || !invocation.boundary_statuses.contains(&required)
        {
            errors
                .insert(LocalProviderOutputPipelineError::InvocationOutputNotUntrustedDescriptive);
        }
    }
    for required in constrained_local_provider_invocation_effect_statuses() {
        if !result.effect_statuses.contains(&required)
            || !invocation.effect_statuses.contains(&required)
        {
            errors.insert(LocalProviderOutputPipelineError::CandidateMaterializationClaimRejected);
        }
    }
    if !errors.is_empty() {
        return Err(errors.into_iter().collect());
    }
    Ok(InvocationProviderOutputBridgeProjection {
        source_kind: LocalProviderOutputPipelineSourceKind::ConstrainedLocalProviderInvocation,
        invocation_result_id: result.result_id.clone(),
        provider_execution_result_id: result.result_id.clone(),
        output_summary: result.output_summary.clone(),
        output_trust_status: LocalProviderOutputTrustStatus::UntrustedDescriptive,
        output_materialization_status:
            LocalProviderOutputMaterializationStatus::ProjectedAsUntrustedOutput,
        output_promotion_status: LocalProviderOutputPromotionStatus::NotPromoted,
        descriptive_only: true,
        not_candidate_material: true,
        not_promoted: true,
    })
}

pub fn provider_execution_projection_from_invocation_bridge(
    state: &LocalOperatorShellState,
    bridge: &InvocationProviderOutputBridgeProjection,
) -> LocalProviderExecutionProjection {
    with_provider_execution_projection_validation(LocalProviderExecutionProjection {
        status: LocalProviderExecutionStatus::Executed,
        projection_status: LocalProviderExecutionResultProjectionStatus::ExecutionProjected,
        configured_provider_kind: "allowlisted_local_deterministic_provider".to_string(),
        sandbox_status: LocalProviderExecutionSandboxStatus::SandboxedDeterministicNoExternalEffects,
        output_trust_status: LocalProviderOutputTrustStatus::UntrustedDescriptive,
        output_materialization_status: LocalProviderOutputMaterializationStatus::NotCandidateMaterial,
        output_promotion_status: LocalProviderOutputPromotionStatus::NotPromoted,
        promotion_availability_status: LocalProviderOutputPromotionStatus::PromotionNotAvailableInPhase142,
        linkage: LocalProviderExecutionResultLinkage {
            execution_result_id: bridge.provider_execution_result_id.clone(),
            source_boundary: "constrained_local_provider_invocation_pipeline_bridge".to_string(),
            ..local_provider_execution_linkage(state, &bridge.provider_execution_result_id)
        },
        absence_markers: local_provider_execution_result_absence_markers(),
        projection_validation: LocalProviderExecutionResultProjectionValidation {
            status: LocalProviderExecutionResultProjectionValidationStatus::Invalid,
            error_codes: Vec::new(),
            reason: "projection validation pending".to_string(),
        },
        result: Some(LocalProviderExecutionResult {
            result_id: bridge.provider_execution_result_id.clone(),
            provider_kind: LocalProviderKind::DeterministicStub,
            sandbox_status: LocalProviderExecutionSandboxStatus::SandboxedDeterministicNoExternalEffects,
            output_summary: bridge.output_summary.clone(),
            output_trust_status: LocalProviderOutputTrustStatus::UntrustedDescriptive,
            output_materialization_status: LocalProviderOutputMaterializationStatus::ProjectedAsUntrustedOutput,
            output_promotion_status: LocalProviderOutputPromotionStatus::NotPromoted,
            promotion_availability_status: LocalProviderOutputPromotionStatus::PromotionNotAvailableInPhase142,
            descriptive_only: true,
            provider_output_trusted: false,
            candidate_output_promoted: false,
            decision_appended: false,
            replay_repaired: false,
            release_or_deployment_evidence_created: false,
        }),
        validation_status: "invocation_output_projected".to_string(),
        validation_error_codes: Vec::new(),
        validation_reason: "constrained invocation output projected into provider execution/result surface without changing trust boundary".to_string(),
        capability_surface: local_provider_execution_capability_surface(),
        note: "Constrained invocation output is projected as untrusted_descriptive provider output only; it is not candidate material, not promoted, and not approved.".to_string(),
    })
}

fn pipeline_stage(
    stage: LocalProviderOutputPipelineStage,
    status: LocalProviderOutputPipelineStageStatus,
    reason: Option<LocalProviderOutputPipelineError>,
) -> LocalProviderOutputPipelineStageProjection {
    LocalProviderOutputPipelineStageProjection {
        stage,
        status,
        reason,
    }
}

pub fn derive_local_provider_output_pipeline_projection(
    state: &LocalOperatorShellState,
) -> LocalProviderOutputPipelineProjection {
    let bridge_result = project_invocation_output_into_provider_pipeline(state);
    if state.constrained_local_provider_invocation.status
        == ConstrainedLocalProviderInvocationStatus::NotInvoked
    {
        return initial_local_provider_output_pipeline_projection();
    }
    if let Err(errors) = bridge_result.clone() {
        let reason = errors
            .first()
            .copied()
            .unwrap_or(LocalProviderOutputPipelineError::NoInvocationOutput);
        return LocalProviderOutputPipelineProjection {
            status: LocalProviderOutputPipelineValidationStatus::Rejected,
            source_kind: Some(LocalProviderOutputPipelineSourceKind::ConstrainedLocalProviderInvocation),
            source_invocation_result_id: state.constrained_local_provider_invocation.result.as_ref().map(|result| result.result_id.clone()),
            provider_execution_result_id: None,
            current_stage: Some(LocalProviderOutputPipelineStage::InvocationOutputProjected),
            next_required_stage: Some(LocalProviderOutputPipelineStage::InvocationOutputProjected),
            stages: local_provider_output_pipeline_stage_order()
                .into_iter()
                .map(|stage| {
                    let status = if stage == LocalProviderOutputPipelineStage::InvocationOutputProjected {
                        LocalProviderOutputPipelineStageStatus::Rejected
                    } else {
                        LocalProviderOutputPipelineStageStatus::Blocked
                    };
                    pipeline_stage(stage, status, Some(reason))
                })
                .collect(),
            provider_output_validation_status: state.provider_output_validation.status,
            provider_output_review_status: state.provider_output_validation.reviewability_status,
            staged_proposal_status: state.staged_candidate_conversion_proposal.status,
            staged_proposal_validation_status: state.staged_candidate_conversion_validation.status,
            candidate_review_status: "blocked".to_string(),
            operator_decision_status: state.operator_candidate_decision.status,
            boundary_statuses: local_provider_output_pipeline_boundary_statuses(),
            effect_statuses: local_provider_output_pipeline_effect_statuses(),
            errors,
            bridge: None,
            note: "Provider output pipeline integration rejected fail-closed; no candidate output is created.".to_string(),
        };
    }
    let bridge = bridge_result.unwrap();
    let mut errors = Vec::new();
    let validation_completed = state.provider_output_validation.status
        == LocalProviderOutputValidationStatus::ReviewableUntrusted;
    let validation_rejected = matches!(
        state.provider_output_validation.status,
        LocalProviderOutputValidationStatus::Rejected
            | LocalProviderOutputValidationStatus::ValidationNotApplicable
            | LocalProviderOutputValidationStatus::InvalidValidationInput
    );
    let review_completed = state.provider_output_validation.reviewability_status
        == LocalProviderOutputReviewabilityStatus::ReviewableUntrusted;
    let proposal_completed = state.staged_candidate_conversion_proposal.status
        == StagedCandidateConversionProposalStatus::StagedProposalCreated;
    let staged_validation_completed = state.staged_candidate_conversion_validation.status
        == StagedCandidateConversionValidationStatus::StagedProposalShapeValid;
    let staged_validation_rejected = matches!(
        state.staged_candidate_conversion_validation.status,
        StagedCandidateConversionValidationStatus::RejectedStagedProposal
            | StagedCandidateConversionValidationStatus::InvalidValidationInput
    );
    let candidate_review_completed = staged_validation_completed;
    let operator_decision_completed = matches!(
        state.operator_candidate_decision.status,
        OperatorCandidateDecisionStatus::ApprovedValidatedStagedProposal
            | OperatorCandidateDecisionStatus::RejectedValidatedStagedProposal
    );

    let mut next_required_stage = None;
    let mut stages = Vec::new();
    stages.push(pipeline_stage(
        LocalProviderOutputPipelineStage::InvocationOutputProjected,
        LocalProviderOutputPipelineStageStatus::Completed,
        None,
    ));
    stages.push(pipeline_stage(
        LocalProviderOutputPipelineStage::ProviderExecutionResultProjected,
        LocalProviderOutputPipelineStageStatus::Completed,
        None,
    ));
    stages.push(pipeline_stage(
        LocalProviderOutputPipelineStage::ProviderOutputValidationRequired,
        LocalProviderOutputPipelineStageStatus::Completed,
        None,
    ));
    if validation_rejected {
        errors.push(LocalProviderOutputPipelineError::ProviderOutputValidationRejected);
        stages.push(pipeline_stage(
            LocalProviderOutputPipelineStage::ProviderOutputValidationProjected,
            LocalProviderOutputPipelineStageStatus::Rejected,
            Some(LocalProviderOutputPipelineError::ProviderOutputValidationRejected),
        ));
        for stage in local_provider_output_pipeline_stage_order()
            .into_iter()
            .skip(4)
        {
            stages.push(pipeline_stage(
                stage,
                LocalProviderOutputPipelineStageStatus::Blocked,
                Some(LocalProviderOutputPipelineError::ProviderOutputValidationRejected),
            ));
        }
        next_required_stage =
            Some(LocalProviderOutputPipelineStage::ProviderOutputValidationProjected);
    } else if !validation_completed {
        errors.push(LocalProviderOutputPipelineError::ProviderOutputValidationMissing);
        stages.push(pipeline_stage(
            LocalProviderOutputPipelineStage::ProviderOutputValidationProjected,
            LocalProviderOutputPipelineStageStatus::Available,
            Some(LocalProviderOutputPipelineError::ProviderOutputValidationMissing),
        ));
        for stage in local_provider_output_pipeline_stage_order()
            .into_iter()
            .skip(4)
        {
            stages.push(pipeline_stage(
                stage,
                LocalProviderOutputPipelineStageStatus::Blocked,
                Some(LocalProviderOutputPipelineError::ProviderOutputValidationMissing),
            ));
        }
        next_required_stage =
            Some(LocalProviderOutputPipelineStage::ProviderOutputValidationProjected);
    } else {
        stages.push(pipeline_stage(
            LocalProviderOutputPipelineStage::ProviderOutputValidationProjected,
            LocalProviderOutputPipelineStageStatus::Completed,
            None,
        ));
        stages.push(pipeline_stage(
            LocalProviderOutputPipelineStage::ProviderOutputReviewRequired,
            LocalProviderOutputPipelineStageStatus::Completed,
            None,
        ));
        if !review_completed {
            errors.push(LocalProviderOutputPipelineError::ProviderOutputReviewMissing);
            stages.push(pipeline_stage(
                LocalProviderOutputPipelineStage::ProviderOutputReviewProjected,
                LocalProviderOutputPipelineStageStatus::Available,
                Some(LocalProviderOutputPipelineError::ProviderOutputReviewMissing),
            ));
            for stage in local_provider_output_pipeline_stage_order()
                .into_iter()
                .skip(6)
            {
                stages.push(pipeline_stage(
                    stage,
                    LocalProviderOutputPipelineStageStatus::Blocked,
                    Some(LocalProviderOutputPipelineError::ProviderOutputReviewMissing),
                ));
            }
            next_required_stage =
                Some(LocalProviderOutputPipelineStage::ProviderOutputReviewProjected);
        } else {
            stages.push(pipeline_stage(
                LocalProviderOutputPipelineStage::ProviderOutputReviewProjected,
                LocalProviderOutputPipelineStageStatus::Completed,
                None,
            ));
            stages.push(pipeline_stage(
                LocalProviderOutputPipelineStage::StagedProposalRequired,
                LocalProviderOutputPipelineStageStatus::Completed,
                None,
            ));
            if !proposal_completed {
                errors.push(LocalProviderOutputPipelineError::StagedProposalMissing);
                stages.push(pipeline_stage(
                    LocalProviderOutputPipelineStage::StagedProposalProjected,
                    LocalProviderOutputPipelineStageStatus::Available,
                    Some(LocalProviderOutputPipelineError::StagedProposalMissing),
                ));
                for stage in local_provider_output_pipeline_stage_order()
                    .into_iter()
                    .skip(8)
                {
                    stages.push(pipeline_stage(
                        stage,
                        LocalProviderOutputPipelineStageStatus::Blocked,
                        Some(LocalProviderOutputPipelineError::StagedProposalMissing),
                    ));
                }
                next_required_stage =
                    Some(LocalProviderOutputPipelineStage::StagedProposalProjected);
            } else {
                stages.push(pipeline_stage(
                    LocalProviderOutputPipelineStage::StagedProposalProjected,
                    LocalProviderOutputPipelineStageStatus::Completed,
                    None,
                ));
                stages.push(pipeline_stage(
                    LocalProviderOutputPipelineStage::StagedProposalValidationRequired,
                    LocalProviderOutputPipelineStageStatus::Completed,
                    None,
                ));
                if staged_validation_rejected {
                    errors.push(LocalProviderOutputPipelineError::StagedProposalValidationRejected);
                    stages.push(pipeline_stage(
                        LocalProviderOutputPipelineStage::StagedProposalValidationProjected,
                        LocalProviderOutputPipelineStageStatus::Rejected,
                        Some(LocalProviderOutputPipelineError::StagedProposalValidationRejected),
                    ));
                    for stage in local_provider_output_pipeline_stage_order()
                        .into_iter()
                        .skip(10)
                    {
                        stages.push(pipeline_stage(
                            stage,
                            LocalProviderOutputPipelineStageStatus::Blocked,
                            Some(
                                LocalProviderOutputPipelineError::StagedProposalValidationRejected,
                            ),
                        ));
                    }
                    next_required_stage =
                        Some(LocalProviderOutputPipelineStage::StagedProposalValidationProjected);
                } else if !staged_validation_completed {
                    errors.push(LocalProviderOutputPipelineError::StagedProposalValidationMissing);
                    stages.push(pipeline_stage(
                        LocalProviderOutputPipelineStage::StagedProposalValidationProjected,
                        LocalProviderOutputPipelineStageStatus::Available,
                        Some(LocalProviderOutputPipelineError::StagedProposalValidationMissing),
                    ));
                    for stage in local_provider_output_pipeline_stage_order()
                        .into_iter()
                        .skip(10)
                    {
                        stages.push(pipeline_stage(
                            stage,
                            LocalProviderOutputPipelineStageStatus::Blocked,
                            Some(LocalProviderOutputPipelineError::StagedProposalValidationMissing),
                        ));
                    }
                    next_required_stage =
                        Some(LocalProviderOutputPipelineStage::StagedProposalValidationProjected);
                } else {
                    stages.push(pipeline_stage(
                        LocalProviderOutputPipelineStage::StagedProposalValidationProjected,
                        LocalProviderOutputPipelineStageStatus::Completed,
                        None,
                    ));
                    stages.push(pipeline_stage(
                        LocalProviderOutputPipelineStage::CandidateReviewRequired,
                        LocalProviderOutputPipelineStageStatus::Completed,
                        None,
                    ));
                    if !candidate_review_completed {
                        errors.push(LocalProviderOutputPipelineError::CandidateReviewMissing);
                        stages.push(pipeline_stage(
                            LocalProviderOutputPipelineStage::CandidateReviewProjected,
                            LocalProviderOutputPipelineStageStatus::Available,
                            Some(LocalProviderOutputPipelineError::CandidateReviewMissing),
                        ));
                    } else {
                        stages.push(pipeline_stage(
                            LocalProviderOutputPipelineStage::CandidateReviewProjected,
                            LocalProviderOutputPipelineStageStatus::Completed,
                            None,
                        ));
                    }
                    stages.push(pipeline_stage(
                        LocalProviderOutputPipelineStage::OperatorDecisionRequired,
                        LocalProviderOutputPipelineStageStatus::Completed,
                        None,
                    ));
                    if !operator_decision_completed {
                        errors.push(LocalProviderOutputPipelineError::OperatorDecisionMissing);
                        stages.push(pipeline_stage(
                            LocalProviderOutputPipelineStage::OperatorDecisionProjected,
                            LocalProviderOutputPipelineStageStatus::Available,
                            Some(LocalProviderOutputPipelineError::OperatorDecisionMissing),
                        ));
                        next_required_stage =
                            Some(LocalProviderOutputPipelineStage::OperatorDecisionProjected);
                    } else {
                        stages.push(pipeline_stage(
                            LocalProviderOutputPipelineStage::OperatorDecisionProjected,
                            LocalProviderOutputPipelineStageStatus::Completed,
                            None,
                        ));
                    }
                }
            }
        }
    }

    let current_stage = stages
        .iter()
        .rev()
        .find(|stage| stage.status == LocalProviderOutputPipelineStageStatus::Completed)
        .map(|stage| stage.stage);
    if next_required_stage.is_none() {
        next_required_stage = stages
            .iter()
            .find(|stage| {
                matches!(
                    stage.status,
                    LocalProviderOutputPipelineStageStatus::Available
                        | LocalProviderOutputPipelineStageStatus::Rejected
                        | LocalProviderOutputPipelineStageStatus::Blocked
                )
            })
            .map(|stage| stage.stage);
    }
    let status = if errors.iter().any(|error| {
        matches!(
            error,
            LocalProviderOutputPipelineError::ProviderOutputValidationRejected
                | LocalProviderOutputPipelineError::StagedProposalValidationRejected
        )
    }) {
        LocalProviderOutputPipelineValidationStatus::Rejected
    } else if errors.is_empty() {
        LocalProviderOutputPipelineValidationStatus::Valid
    } else {
        LocalProviderOutputPipelineValidationStatus::Blocked
    };

    LocalProviderOutputPipelineProjection {
        status,
        source_kind: Some(LocalProviderOutputPipelineSourceKind::ConstrainedLocalProviderInvocation),
        source_invocation_result_id: Some(bridge.invocation_result_id.clone()),
        provider_execution_result_id: Some(bridge.provider_execution_result_id.clone()),
        current_stage,
        next_required_stage,
        stages,
        provider_output_validation_status: state.provider_output_validation.status,
        provider_output_review_status: state.provider_output_validation.reviewability_status,
        staged_proposal_status: state.staged_candidate_conversion_proposal.status,
        staged_proposal_validation_status: state.staged_candidate_conversion_validation.status,
        candidate_review_status: if staged_validation_completed { "display_only".to_string() } else { "required".to_string() },
        operator_decision_status: state.operator_candidate_decision.status,
        boundary_statuses: local_provider_output_pipeline_boundary_statuses(),
        effect_statuses: local_provider_output_pipeline_effect_statuses(),
        errors,
        bridge: Some(bridge),
        note: "Invocation output remains untrusted and descriptive. Pipeline integration does not create candidate output. Validation, review, staging, staged validation, candidate review, and operator decision boundaries cannot be skipped. Candidate materialization remains a later bounded step. Provider trust, readiness, release, deployment, and public-use approval are not granted.".to_string(),
    }
}

pub fn validate_provider_output_pipeline_stage_order(
    projection: &LocalProviderOutputPipelineProjection,
) -> Result<(), LocalProviderOutputPipelineError> {
    let expected = local_provider_output_pipeline_stage_order();
    let actual: Vec<_> = projection.stages.iter().map(|stage| stage.stage).collect();
    if actual != expected {
        return Err(LocalProviderOutputPipelineError::PipelineSkipAttemptRejected);
    }
    let mut seen_incomplete = false;
    for stage in &projection.stages {
        if seen_incomplete && stage.status == LocalProviderOutputPipelineStageStatus::Completed {
            return Err(LocalProviderOutputPipelineError::PipelineSkipAttemptRejected);
        }
        if stage.status != LocalProviderOutputPipelineStageStatus::Completed {
            seen_incomplete = true;
        }
    }
    Ok(())
}

pub fn reject_provider_output_pipeline_integration(
    error: LocalProviderOutputPipelineError,
) -> LocalProviderOutputPipelineProjection {
    let mut projection = initial_local_provider_output_pipeline_projection();
    projection.status = LocalProviderOutputPipelineValidationStatus::Rejected;
    projection.errors = vec![error];
    projection.stages = local_provider_output_pipeline_stage_order()
        .into_iter()
        .map(|stage| {
            pipeline_stage(
                stage,
                LocalProviderOutputPipelineStageStatus::Blocked,
                Some(error),
            )
        })
        .collect();
    projection.note = "Provider output pipeline integration rejected fail-closed.".to_string();
    projection
}
