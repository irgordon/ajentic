//! Release-candidate preparation contract projection.
//!
//! This module classifies committed local evidence for later release-candidate
//! work. It does not create artifacts and it does not approve readiness.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidatePreparationStatus {
    NotPrepared,
    PreparationProjected,
    PreparationValidated,
    PreparationBlocked,
    PreparationRejected,
    InvalidPreparationInput,
}

impl ReleaseCandidatePreparationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotPrepared => "not_prepared",
            Self::PreparationProjected => "preparation_projected",
            Self::PreparationValidated => "preparation_validated",
            Self::PreparationBlocked => "preparation_blocked",
            Self::PreparationRejected => "preparation_rejected",
            Self::InvalidPreparationInput => "invalid_preparation_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidatePreparationEvidenceCategory {
    LocalBetaWorkflow,
    ControlledInternalTrialExecution,
    TrialObservability,
    TrialErrorReporting,
    TrialEvidenceReview,
    UserFacingHelp,
    LocalHtmlHelpPages,
    VisibleHelpEntryPoint,
    ProviderConfiguration,
    ProviderAdapterContract,
    ProviderAdapterDryRun,
    ConstrainedProviderInvocation,
    ProviderOutputPipeline,
    ProviderOutputValidation,
    ProviderOutputReview,
    StagedProposal,
    StagedProposalValidation,
    CandidateReview,
    OperatorDecision,
    LocalCandidateMaterialization,
    ReplayStatus,
    EvidenceExport,
    LocalSessionPackage,
    SessionRestoreHistory,
    ControlledInternalTrialPackage,
    TrialOperatorRunbook,
    TrialFailureDrill,
    TrialSessionEvidence,
    ReplayRestoreVerification,
    DeterministicValidation,
}

impl ReleaseCandidatePreparationEvidenceCategory {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalBetaWorkflow => "local_beta_workflow",
            Self::ControlledInternalTrialExecution => "controlled_internal_trial_execution",
            Self::TrialObservability => "trial_observability",
            Self::TrialErrorReporting => "trial_error_reporting",
            Self::TrialEvidenceReview => "trial_evidence_review",
            Self::UserFacingHelp => "user_facing_help",
            Self::LocalHtmlHelpPages => "local_html_help_pages",
            Self::VisibleHelpEntryPoint => "visible_help_entry_point",
            Self::ProviderConfiguration => "provider_configuration",
            Self::ProviderAdapterContract => "provider_adapter_contract",
            Self::ProviderAdapterDryRun => "provider_adapter_dry_run",
            Self::ConstrainedProviderInvocation => "constrained_provider_invocation",
            Self::ProviderOutputPipeline => "provider_output_pipeline",
            Self::ProviderOutputValidation => "provider_output_validation",
            Self::ProviderOutputReview => "provider_output_review",
            Self::StagedProposal => "staged_proposal",
            Self::StagedProposalValidation => "staged_proposal_validation",
            Self::CandidateReview => "candidate_review",
            Self::OperatorDecision => "operator_decision",
            Self::LocalCandidateMaterialization => "local_candidate_materialization",
            Self::ReplayStatus => "replay_status",
            Self::EvidenceExport => "evidence_export",
            Self::LocalSessionPackage => "local_session_package",
            Self::SessionRestoreHistory => "session_restore_history",
            Self::ControlledInternalTrialPackage => "controlled_internal_trial_package",
            Self::TrialOperatorRunbook => "trial_operator_runbook",
            Self::TrialFailureDrill => "trial_failure_drill",
            Self::TrialSessionEvidence => "trial_session_evidence",
            Self::ReplayRestoreVerification => "replay_restore_verification",
            Self::DeterministicValidation => "deterministic_validation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidatePreparationEvidenceCategoryStatus {
    Present,
    Missing,
    Blocked,
    Rejected,
    Deferred,
    NotApplicable,
}

impl ReleaseCandidatePreparationEvidenceCategoryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Missing => "missing",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
            Self::Deferred => "deferred",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidatePreparationValidationStatus {
    NotValidated,
    Valid,
    Invalid,
}

impl ReleaseCandidatePreparationValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotValidated => "not_validated",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidatePreparationValidationError {
    MissingLocalBetaWorkflow,
    MissingTrialExecution,
    MissingTrialObservability,
    MissingTrialEvidenceReview,
    MissingUserHelp,
    MissingHelpIndex,
    MissingVisibleHelpEntryPoint,
    MissingCandidateMaterialization,
    MissingReplayStatus,
    MissingSessionPackage,
    MissingRestoreHistory,
    MissingTrialPackage,
    MissingTrialSessionEvidence,
    MissingReplayRestoreVerification,
    EvidenceCategoryBlocked,
    EvidenceCategoryRejected,
    ReadinessClaimDetected,
    ReleaseClaimDetected,
    DeploymentClaimDetected,
    PublicUseClaimDetected,
    ProductionUseClaimDetected,
    SigningClaimDetected,
    PublishingClaimDetected,
    InstallerClaimDetected,
    UpdateChannelClaimDetected,
    ProviderTrustClaimDetected,
    ActionAuthorizationClaimDetected,
    ReplayRepairClaimDetected,
    RecoveryPromotionClaimDetected,
}

impl ReleaseCandidatePreparationValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingLocalBetaWorkflow => "missing_local_beta_workflow",
            Self::MissingTrialExecution => "missing_trial_execution",
            Self::MissingTrialObservability => "missing_trial_observability",
            Self::MissingTrialEvidenceReview => "missing_trial_evidence_review",
            Self::MissingUserHelp => "missing_user_help",
            Self::MissingHelpIndex => "missing_help_index",
            Self::MissingVisibleHelpEntryPoint => "missing_visible_help_entry_point",
            Self::MissingCandidateMaterialization => "missing_candidate_materialization",
            Self::MissingReplayStatus => "missing_replay_status",
            Self::MissingSessionPackage => "missing_session_package",
            Self::MissingRestoreHistory => "missing_restore_history",
            Self::MissingTrialPackage => "missing_trial_package",
            Self::MissingTrialSessionEvidence => "missing_trial_session_evidence",
            Self::MissingReplayRestoreVerification => "missing_replay_restore_verification",
            Self::EvidenceCategoryBlocked => "evidence_category_blocked",
            Self::EvidenceCategoryRejected => "evidence_category_rejected",
            Self::ReadinessClaimDetected => "readiness_claim_detected",
            Self::ReleaseClaimDetected => "release_claim_detected",
            Self::DeploymentClaimDetected => "deployment_claim_detected",
            Self::PublicUseClaimDetected => "public_use_claim_detected",
            Self::ProductionUseClaimDetected => "production_use_claim_detected",
            Self::SigningClaimDetected => "signing_claim_detected",
            Self::PublishingClaimDetected => "publishing_claim_detected",
            Self::InstallerClaimDetected => "installer_claim_detected",
            Self::UpdateChannelClaimDetected => "update_channel_claim_detected",
            Self::ProviderTrustClaimDetected => "provider_trust_claim_detected",
            Self::ActionAuthorizationClaimDetected => "action_authorization_claim_detected",
            Self::ReplayRepairClaimDetected => "replay_repair_claim_detected",
            Self::RecoveryPromotionClaimDetected => "recovery_promotion_claim_detected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidatePreparationBoundaryStatus {
    ReleaseCandidatePreparationOnly,
    ReleaseReadinessNotApproved,
    ReleaseCandidateStatusNotApproved,
    ProductionStatusNotApproved,
    NoReleaseArtifact,
    NoDeploymentArtifact,
    NoPublicDistribution,
    NoSigning,
    NoPublishing,
    NoInstallerActivation,
    NoUpdateChannelActivation,
    NoProviderTrust,
    NoActionAuthorization,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl ReleaseCandidatePreparationBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ReleaseCandidatePreparationOnly => "release_candidate_preparation_only",
            Self::ReleaseReadinessNotApproved => "release_readiness_not_approved",
            Self::ReleaseCandidateStatusNotApproved => "release_candidate_status_not_approved",
            Self::ProductionStatusNotApproved => "production_status_not_approved",
            Self::NoReleaseArtifact => "no_release_artifact",
            Self::NoDeploymentArtifact => "no_deployment_artifact",
            Self::NoPublicDistribution => "no_public_distribution",
            Self::NoSigning => "no_signing",
            Self::NoPublishing => "no_publishing",
            Self::NoInstallerActivation => "no_installer_activation",
            Self::NoUpdateChannelActivation => "no_update_channel_activation",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoActionAuthorization => "no_action_authorization",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidatePreparationClassification {
    ReleaseCandidatePreparationOnly,
    NonProduction,
    ReleaseNotApproved,
    NoPublicDistribution,
}

impl ReleaseCandidatePreparationClassification {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ReleaseCandidatePreparationOnly => "release_candidate_preparation_only",
            Self::NonProduction => "non_production",
            Self::ReleaseNotApproved => "release_not_approved",
            Self::NoPublicDistribution => "no_public_distribution",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidatePreparationSourceLinkage {
    pub category: ReleaseCandidatePreparationEvidenceCategory,
    pub source_surface: String,
    pub source_status: String,
    pub source_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidatePreparationEvidenceItem {
    pub category: ReleaseCandidatePreparationEvidenceCategory,
    pub status: ReleaseCandidatePreparationEvidenceCategoryStatus,
    pub reason: String,
    pub source_linkage: ReleaseCandidatePreparationSourceLinkage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidatePreparationMissingEvidence {
    pub category: ReleaseCandidatePreparationEvidenceCategory,
    pub reason: String,
    pub source_surface: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidatePreparationBlocker {
    pub category: ReleaseCandidatePreparationEvidenceCategory,
    pub reason: String,
    pub source_surface: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidatePreparationCapabilitySurface {
    pub creates_release_artifacts: bool,
    pub creates_deployment_artifacts: bool,
    pub creates_public_downloads: bool,
    pub creates_release_tags: bool,
    pub creates_github_releases: bool,
    pub signs_artifacts: bool,
    pub publishes_artifacts: bool,
    pub creates_installers: bool,
    pub activates_update_channels: bool,
    pub approves_release_candidate_status: bool,
    pub approves_release_readiness: bool,
    pub approves_production_status: bool,
    pub approves_production_readiness: bool,
    pub approves_deployment: bool,
    pub approves_public_use: bool,
    pub approves_production_use: bool,
    pub trusts_provider_output: bool,
    pub authorizes_actions: bool,
    pub repairs_replay: bool,
    pub promotes_recovery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidatePreparationInputSnapshot {
    pub evidence_items: Vec<ReleaseCandidatePreparationEvidenceItem>,
    pub claims_readiness: bool,
    pub claims_release: bool,
    pub claims_deployment: bool,
    pub claims_public_use: bool,
    pub claims_production_use: bool,
    pub claims_signing: bool,
    pub claims_publishing: bool,
    pub claims_installer: bool,
    pub claims_update_channel: bool,
    pub claims_provider_trust: bool,
    pub claims_action_authorization: bool,
    pub claims_replay_repair: bool,
    pub claims_recovery_promotion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidatePreparationContract {
    pub preparation_id: String,
    pub status: ReleaseCandidatePreparationStatus,
    pub validation_status: ReleaseCandidatePreparationValidationStatus,
    pub classification: Vec<ReleaseCandidatePreparationClassification>,
    pub evidence_items: Vec<ReleaseCandidatePreparationEvidenceItem>,
    pub missing_evidence: Vec<ReleaseCandidatePreparationMissingEvidence>,
    pub blockers: Vec<ReleaseCandidatePreparationBlocker>,
    pub validation_errors: Vec<ReleaseCandidatePreparationValidationError>,
    pub boundary_statuses: Vec<ReleaseCandidatePreparationBoundaryStatus>,
    pub capability_surface: ReleaseCandidatePreparationCapabilitySurface,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidatePreparationProjection {
    pub preparation_id: String,
    pub status: ReleaseCandidatePreparationStatus,
    pub validation_status: ReleaseCandidatePreparationValidationStatus,
    pub classification: Vec<ReleaseCandidatePreparationClassification>,
    pub evidence_items: Vec<ReleaseCandidatePreparationEvidenceItem>,
    pub missing_evidence: Vec<ReleaseCandidatePreparationMissingEvidence>,
    pub blockers: Vec<ReleaseCandidatePreparationBlocker>,
    pub validation_errors: Vec<ReleaseCandidatePreparationValidationError>,
    pub boundary_statuses: Vec<ReleaseCandidatePreparationBoundaryStatus>,
    pub capability_surface: ReleaseCandidatePreparationCapabilitySurface,
    pub category_count: usize,
    pub present_evidence_count: usize,
    pub missing_evidence_count: usize,
    pub blocked_evidence_count: usize,
    pub rejected_evidence_count: usize,
    pub no_release_readiness_note: String,
    pub no_release_artifact_note: String,
    pub no_release_candidate_status_note: String,
    pub no_production_deployment_public_signing_publishing_installer_update_note: String,
    pub no_provider_trust_note: String,
    pub no_action_authorization_note: String,
}

pub fn release_candidate_preparation_evidence_categories(
) -> Vec<ReleaseCandidatePreparationEvidenceCategory> {
    vec![
        ReleaseCandidatePreparationEvidenceCategory::LocalBetaWorkflow,
        ReleaseCandidatePreparationEvidenceCategory::ControlledInternalTrialExecution,
        ReleaseCandidatePreparationEvidenceCategory::TrialObservability,
        ReleaseCandidatePreparationEvidenceCategory::TrialErrorReporting,
        ReleaseCandidatePreparationEvidenceCategory::TrialEvidenceReview,
        ReleaseCandidatePreparationEvidenceCategory::UserFacingHelp,
        ReleaseCandidatePreparationEvidenceCategory::LocalHtmlHelpPages,
        ReleaseCandidatePreparationEvidenceCategory::VisibleHelpEntryPoint,
        ReleaseCandidatePreparationEvidenceCategory::ProviderConfiguration,
        ReleaseCandidatePreparationEvidenceCategory::ProviderAdapterContract,
        ReleaseCandidatePreparationEvidenceCategory::ProviderAdapterDryRun,
        ReleaseCandidatePreparationEvidenceCategory::ConstrainedProviderInvocation,
        ReleaseCandidatePreparationEvidenceCategory::ProviderOutputPipeline,
        ReleaseCandidatePreparationEvidenceCategory::ProviderOutputValidation,
        ReleaseCandidatePreparationEvidenceCategory::ProviderOutputReview,
        ReleaseCandidatePreparationEvidenceCategory::StagedProposal,
        ReleaseCandidatePreparationEvidenceCategory::StagedProposalValidation,
        ReleaseCandidatePreparationEvidenceCategory::CandidateReview,
        ReleaseCandidatePreparationEvidenceCategory::OperatorDecision,
        ReleaseCandidatePreparationEvidenceCategory::LocalCandidateMaterialization,
        ReleaseCandidatePreparationEvidenceCategory::ReplayStatus,
        ReleaseCandidatePreparationEvidenceCategory::EvidenceExport,
        ReleaseCandidatePreparationEvidenceCategory::LocalSessionPackage,
        ReleaseCandidatePreparationEvidenceCategory::SessionRestoreHistory,
        ReleaseCandidatePreparationEvidenceCategory::ControlledInternalTrialPackage,
        ReleaseCandidatePreparationEvidenceCategory::TrialOperatorRunbook,
        ReleaseCandidatePreparationEvidenceCategory::TrialFailureDrill,
        ReleaseCandidatePreparationEvidenceCategory::TrialSessionEvidence,
        ReleaseCandidatePreparationEvidenceCategory::ReplayRestoreVerification,
        ReleaseCandidatePreparationEvidenceCategory::DeterministicValidation,
    ]
}

pub fn release_candidate_preparation_boundary_statuses(
) -> Vec<ReleaseCandidatePreparationBoundaryStatus> {
    vec![
        ReleaseCandidatePreparationBoundaryStatus::ReleaseCandidatePreparationOnly,
        ReleaseCandidatePreparationBoundaryStatus::ReleaseReadinessNotApproved,
        ReleaseCandidatePreparationBoundaryStatus::ReleaseCandidateStatusNotApproved,
        ReleaseCandidatePreparationBoundaryStatus::ProductionStatusNotApproved,
        ReleaseCandidatePreparationBoundaryStatus::NoReleaseArtifact,
        ReleaseCandidatePreparationBoundaryStatus::NoDeploymentArtifact,
        ReleaseCandidatePreparationBoundaryStatus::NoPublicDistribution,
        ReleaseCandidatePreparationBoundaryStatus::NoSigning,
        ReleaseCandidatePreparationBoundaryStatus::NoPublishing,
        ReleaseCandidatePreparationBoundaryStatus::NoInstallerActivation,
        ReleaseCandidatePreparationBoundaryStatus::NoUpdateChannelActivation,
        ReleaseCandidatePreparationBoundaryStatus::NoProviderTrust,
        ReleaseCandidatePreparationBoundaryStatus::NoActionAuthorization,
        ReleaseCandidatePreparationBoundaryStatus::NoReplayRepair,
        ReleaseCandidatePreparationBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub fn release_candidate_preparation_capability_surface(
) -> ReleaseCandidatePreparationCapabilitySurface {
    ReleaseCandidatePreparationCapabilitySurface {
        creates_release_artifacts: false,
        creates_deployment_artifacts: false,
        creates_public_downloads: false,
        creates_release_tags: false,
        creates_github_releases: false,
        signs_artifacts: false,
        publishes_artifacts: false,
        creates_installers: false,
        activates_update_channels: false,
        approves_release_candidate_status: false,
        approves_release_readiness: false,
        approves_production_status: false,
        approves_production_readiness: false,
        approves_deployment: false,
        approves_public_use: false,
        approves_production_use: false,
        trusts_provider_output: false,
        authorizes_actions: false,
        repairs_replay: false,
        promotes_recovery: false,
    }
}

pub fn initial_release_candidate_preparation_projection() -> ReleaseCandidatePreparationProjection {
    derive_release_candidate_preparation_projection(
        &empty_release_candidate_preparation_input_snapshot(),
    )
}

pub fn empty_release_candidate_preparation_input_snapshot(
) -> ReleaseCandidatePreparationInputSnapshot {
    ReleaseCandidatePreparationInputSnapshot {
        evidence_items: release_candidate_preparation_evidence_categories()
            .into_iter()
            .map(|category| {
                evidence_item(
                    category,
                    ReleaseCandidatePreparationEvidenceCategoryStatus::Missing,
                    "committed local shell evidence has not been projected",
                    category.code(),
                    "missing",
                )
            })
            .collect(),
        claims_readiness: false,
        claims_release: false,
        claims_deployment: false,
        claims_public_use: false,
        claims_production_use: false,
        claims_signing: false,
        claims_publishing: false,
        claims_installer: false,
        claims_update_channel: false,
        claims_provider_trust: false,
        claims_action_authorization: false,
        claims_replay_repair: false,
        claims_recovery_promotion: false,
    }
}

pub fn complete_release_candidate_preparation_input_snapshot(
) -> ReleaseCandidatePreparationInputSnapshot {
    ReleaseCandidatePreparationInputSnapshot {
        evidence_items: release_candidate_preparation_evidence_categories()
            .into_iter()
            .map(|category| {
                evidence_item(
                    category,
                    ReleaseCandidatePreparationEvidenceCategoryStatus::Present,
                    "committed preparation evidence is present",
                    category.code(),
                    "present",
                )
            })
            .collect(),
        ..empty_release_candidate_preparation_input_snapshot()
    }
}

pub fn derive_release_candidate_preparation_input_snapshot(
    state: &LocalOperatorShellState,
) -> ReleaseCandidatePreparationInputSnapshot {
    use ReleaseCandidatePreparationEvidenceCategory as Category;
    use ReleaseCandidatePreparationEvidenceCategoryStatus as Status;

    let mut items = Vec::new();
    items.push(item_from_status(
        Category::LocalBetaWorkflow,
        map_workflow_status(state.complete_local_operator_workflow.status),
        "complete_local_operator_workflow",
        state.complete_local_operator_workflow.status.code(),
    ));
    items.push(item_from_status(
        Category::ControlledInternalTrialExecution,
        map_trial_execution_status(state.controlled_internal_trial_execution.status),
        "controlled_internal_trial_execution",
        state.controlled_internal_trial_execution.status.code(),
    ));
    items.push(item_from_status(
        Category::TrialObservability,
        map_trial_observability_status(state.trial_observability.status),
        "trial_observability",
        state.trial_observability.status.code(),
    ));
    items.push(item_from_status(
        Category::TrialErrorReporting,
        if state.trial_error_report.status == TrialErrorReportStatus::InvalidObservabilityInput {
            Status::Rejected
        } else {
            Status::Present
        },
        "trial_error_report",
        state.trial_error_report.status.code(),
    ));
    items.push(item_from_status(
        Category::TrialEvidenceReview,
        map_trial_review_status(state.trial_evidence_review.status),
        "trial_evidence_review",
        state.trial_evidence_review.status.code(),
    ));
    items.push(evidence_item(
        Category::UserFacingHelp,
        Status::Present,
        "committed browser help entry text is available",
        "ui_help_surface",
        "present",
    ));
    items.push(evidence_item(
        Category::LocalHtmlHelpPages,
        Status::Present,
        "local HTML help pages are committed in the browser help surface",
        "ui_help_surface",
        "present",
    ));
    items.push(evidence_item(
        Category::VisibleHelpEntryPoint,
        Status::Present,
        "visible local help entry point is rendered by the browser shell",
        "ui_help_entry_point",
        "present",
    ));
    items.push(item_from_status(
        Category::ProviderConfiguration,
        match state.provider_configuration.status {
            LocalProviderConfigurationStatus::Accepted => Status::Present,
            LocalProviderConfigurationStatus::Rejected
            | LocalProviderConfigurationStatus::Unsupported => Status::Rejected,
            LocalProviderConfigurationStatus::NotConfigured => Status::Missing,
        },
        "provider_configuration",
        state.provider_configuration.status.code(),
    ));
    items.push(item_from_status(
        Category::ProviderAdapterContract,
        map_adapter_contract_status(&state.local_provider_adapter_registry),
        "local_provider_adapter_registry",
        &format!(
            "declarations={}",
            state.local_provider_adapter_registry.declarations.len()
        ),
    ));
    items.push(item_from_status(
        Category::ProviderAdapterDryRun,
        match state.local_provider_adapter_dry_run.status {
            LocalProviderAdapterDryRunStatus::DryRunExecuted => Status::Present,
            LocalProviderAdapterDryRunStatus::DryRunRejected
            | LocalProviderAdapterDryRunStatus::UnsupportedAdapter
            | LocalProviderAdapterDryRunStatus::InvalidDryRunRequest => Status::Rejected,
            _ => Status::Missing,
        },
        "local_provider_adapter_dry_run",
        state.local_provider_adapter_dry_run.status.code(),
    ));
    items.push(item_from_status(
        Category::ConstrainedProviderInvocation,
        match state.constrained_local_provider_invocation.status {
            ConstrainedLocalProviderInvocationStatus::InvocationExecuted => Status::Present,
            ConstrainedLocalProviderInvocationStatus::InvocationRejected
            | ConstrainedLocalProviderInvocationStatus::UnsupportedProvider
            | ConstrainedLocalProviderInvocationStatus::InvalidInvocationRequest => {
                Status::Rejected
            }
            ConstrainedLocalProviderInvocationStatus::AllowlistedProviderRequired => {
                Status::Blocked
            }
            ConstrainedLocalProviderInvocationStatus::NotInvoked => Status::Missing,
        },
        "constrained_local_provider_invocation",
        state.constrained_local_provider_invocation.status.code(),
    ));
    items.push(item_from_status(
        Category::ProviderOutputPipeline,
        map_pipeline_status(state.local_provider_output_pipeline.status),
        "local_provider_output_pipeline",
        state.local_provider_output_pipeline.status.code(),
    ));
    items.push(item_from_status(
        Category::ProviderOutputValidation,
        map_output_validation_status(state.provider_output_validation.status),
        "provider_output_validation",
        state.provider_output_validation.status.code(),
    ));
    items.push(item_from_status(
        Category::ProviderOutputReview,
        map_reviewability_status(state.provider_output_validation.reviewability_status),
        "provider_output_validation.reviewability",
        state.provider_output_validation.reviewability_status.code(),
    ));
    items.push(item_from_status(
        Category::StagedProposal,
        map_staged_proposal_status(state.staged_candidate_conversion_proposal.status),
        "staged_candidate_conversion_proposal",
        state.staged_candidate_conversion_proposal.status.code(),
    ));
    items.push(item_from_status(
        Category::StagedProposalValidation,
        map_staged_validation_status(state.staged_candidate_conversion_validation.status),
        "staged_candidate_conversion_validation",
        state.staged_candidate_conversion_validation.status.code(),
    ));
    items.push(evidence_item(
        Category::CandidateReview,
        if state.local_provider_output_pipeline.candidate_review_status == "display_only" {
            Status::Present
        } else {
            Status::Missing
        },
        "candidate review display surface is required",
        "local_provider_output_pipeline.candidate_review_status",
        &state.local_provider_output_pipeline.candidate_review_status,
    ));
    items.push(item_from_status(
        Category::OperatorDecision,
        map_operator_decision_status(state.operator_candidate_decision.status),
        "operator_candidate_decision",
        state.operator_candidate_decision.status.code(),
    ));
    items.push(item_from_status(
        Category::LocalCandidateMaterialization,
        map_candidate_status(state.local_candidate_output.status),
        "local_candidate_output",
        state.local_candidate_output.status.code(),
    ));
    items.push(item_from_status(
        Category::ReplayStatus,
        if state.run.replay_status.is_empty() {
            Status::Missing
        } else {
            Status::Present
        },
        "run.replay_status",
        &state.run.replay_status,
    ));
    items.push(item_from_status(
        Category::EvidenceExport,
        if state.local_session_evidence_export.export_id.is_empty() {
            Status::Missing
        } else {
            Status::Present
        },
        "local_session_evidence_export",
        state.local_session_evidence_export.export_status.code(),
    ));
    items.push(item_from_status(
        Category::LocalSessionPackage,
        map_session_package_status(state.local_session_package_projection.status),
        "local_session_package_projection",
        state.local_session_package_projection.status.code(),
    ));
    items.push(item_from_status(
        Category::SessionRestoreHistory,
        map_restore_history_status(state),
        "local_session_restore_history",
        state.local_session_restore_projection.status.code(),
    ));
    items.push(item_from_status(
        Category::ControlledInternalTrialPackage,
        map_trial_package_status(state.controlled_internal_trial_package_projection.status),
        "controlled_internal_trial_package_projection",
        state
            .controlled_internal_trial_package_projection
            .status
            .code(),
    ));
    items.push(item_from_status(
        Category::TrialOperatorRunbook,
        map_runbook_status(state.trial_operator_runbook.status),
        "trial_operator_runbook",
        state.trial_operator_runbook.status.code(),
    ));
    items.push(item_from_status(
        Category::TrialFailureDrill,
        Status::Present,
        "trial_failure_drill",
        state.trial_failure_drill.status.code(),
    ));
    items.push(item_from_status(
        Category::TrialSessionEvidence,
        map_trial_session_evidence_status(state.trial_session_evidence_projection.status),
        "trial_session_evidence_projection",
        state.trial_session_evidence_projection.status.code(),
    ));
    items.push(item_from_status(
        Category::ReplayRestoreVerification,
        map_replay_restore_verification_status(state.trial_replay_restore_verification.status),
        "trial_replay_restore_verification",
        state.trial_replay_restore_verification.status.code(),
    ));
    items.push(item_from_status(
        Category::DeterministicValidation,
        if state.run.validation.is_some() {
            Status::Present
        } else {
            Status::Missing
        },
        "run.validation",
        state
            .run
            .validation
            .as_ref()
            .map(|v| v.validation_status.as_str())
            .unwrap_or("missing"),
    ));

    ReleaseCandidatePreparationInputSnapshot {
        evidence_items: items,
        ..empty_release_candidate_preparation_input_snapshot()
    }
}

fn map_adapter_contract_status(
    registry: &LocalProviderAdapterRegistry,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use ReleaseCandidatePreparationEvidenceCategoryStatus as Status;
    if registry
        .declarations
        .iter()
        .any(|d| d.status == LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting)
    {
        Status::Present
    } else if registry.declarations.iter().any(|d| {
        matches!(
            d.status,
            LocalProviderAdapterValidationStatus::AdapterRejected
                | LocalProviderAdapterValidationStatus::UnsupportedAdapter
                | LocalProviderAdapterValidationStatus::InvalidAdapterDeclaration
        )
    }) {
        Status::Rejected
    } else {
        Status::Missing
    }
}
fn map_workflow_status(
    status: CompleteLocalOperatorWorkflowStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use CompleteLocalOperatorWorkflowStatus as S;
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    match status {
        S::CompleteLocalWorkflowProjected | S::LocalCandidateMaterialized => R::Present,
        S::Blocked => R::Blocked,
        S::Rejected => R::Rejected,
        _ => R::Missing,
    }
}
fn map_trial_execution_status(
    status: ControlledInternalTrialRunStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use ControlledInternalTrialRunStatus as S;
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    match status {
        S::TrialRunCompletedLocally => R::Present,
        S::TrialRunBlocked => R::Blocked,
        S::TrialRunRejected | S::InvalidTrialRunRequest => R::Rejected,
        S::PreconditionsMissing => R::Blocked,
        _ => R::Missing,
    }
}
fn map_trial_observability_status(
    status: TrialObservabilityStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    use TrialObservabilityStatus as S;
    match status {
        S::TrialRunObserved
        | S::BlockedStateObserved
        | S::RejectedStateObserved
        | S::StopConditionObserved
        | S::VerificationMismatchObserved
        | S::ErrorReportProjected => R::Present,
        S::ObservabilityProjected => R::Deferred,
        S::NotObserved => R::Missing,
    }
}
fn map_trial_review_status(
    status: TrialEvidenceReviewStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    use TrialEvidenceReviewStatus as S;
    match status {
        S::ReviewProjected | S::ReviewWithFindings | S::HardeningCandidatesProjected => R::Present,
        S::ReviewBlocked => R::Blocked,
        S::ReviewRejected | S::InvalidReviewInput => R::Rejected,
        S::NotReviewed => R::Missing,
    }
}
fn map_pipeline_status(
    status: LocalProviderOutputPipelineValidationStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use LocalProviderOutputPipelineValidationStatus as S;
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    match status {
        S::Valid => R::Present,
        S::Blocked => R::Blocked,
        S::Rejected => R::Rejected,
        S::NotStarted => R::Missing,
    }
}
fn map_output_validation_status(
    status: LocalProviderOutputValidationStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use LocalProviderOutputValidationStatus as S;
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    match status {
        S::ReviewableUntrusted => R::Present,
        S::Rejected | S::InvalidValidationInput => R::Rejected,
        _ => R::Missing,
    }
}
fn map_reviewability_status(
    status: LocalProviderOutputReviewabilityStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use LocalProviderOutputReviewabilityStatus as S;
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    match status {
        S::ReviewableUntrusted => R::Present,
        S::RejectedBeforeReview => R::Rejected,
        S::NotReviewable => R::Missing,
    }
}
fn map_staged_proposal_status(
    status: StagedCandidateConversionProposalStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    use StagedCandidateConversionProposalStatus as S;
    match status {
        S::StagedProposalCreated => R::Present,
        S::SourceNotReviewableUntrusted => R::Blocked,
        S::RejectedSourceNotEligible | S::InvalidProposalRequest => R::Rejected,
        S::NoProposal => R::Missing,
    }
}
fn map_staged_validation_status(
    status: StagedCandidateConversionValidationStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    use StagedCandidateConversionValidationStatus as S;
    match status {
        S::StagedProposalShapeValid => R::Present,
        S::RejectedStagedProposal | S::InvalidValidationInput => R::Rejected,
        S::NotValidated => R::Missing,
    }
}
fn map_operator_decision_status(
    status: OperatorCandidateDecisionStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use OperatorCandidateDecisionStatus as S;
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    match status {
        S::ApprovedValidatedStagedProposal => R::Present,
        S::RejectedValidatedStagedProposal
        | S::RejectedOperatorDecisionRequest
        | S::InvalidOperatorDecisionInput => R::Rejected,
        S::NoOperatorDecision => R::Missing,
    }
}
fn map_candidate_status(
    status: LocalCandidateMaterializationStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use LocalCandidateMaterializationStatus as S;
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    match status {
        S::LocalCandidateMaterialized => R::Present,
        S::MaterializationPreconditionMissing => R::Blocked,
        S::MaterializationRejected | S::InvalidMaterializationRequest => R::Rejected,
        S::NotMaterialized => R::Missing,
    }
}
fn map_session_package_status(
    status: LocalSessionPackageStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use LocalSessionPackageStatus as S;
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    match status {
        S::PackageProjected | S::PackageWritten | S::PackageReadBackValidated => R::Present,
        S::PackageRejected | S::InvalidPackageInput => R::Rejected,
        S::NotPackaged => R::Missing,
    }
}
fn map_restore_history_status(
    state: &LocalOperatorShellState,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    if state.local_session_history_projection.status
        == LocalSessionHistoryStatus::SessionHistoryProjected
        && !matches!(
            state.local_session_restore_projection.status,
            LocalSessionRestoreStatus::RestoreRejected
                | LocalSessionRestoreStatus::InvalidRestoreInput
        )
    {
        R::Present
    } else if matches!(
        state.local_session_restore_projection.status,
        LocalSessionRestoreStatus::RestoreRejected | LocalSessionRestoreStatus::InvalidRestoreInput
    ) {
        R::Rejected
    } else {
        R::Missing
    }
}
fn map_trial_package_status(
    status: ControlledInternalTrialPackageStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use ControlledInternalTrialPackageStatus as S;
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    match status {
        S::PackageProjected
        | S::PackageValidated
        | S::PackageWritten
        | S::PackageReadBackValidated => R::Present,
        S::PackageRejected | S::InvalidPackageInput => R::Rejected,
        S::NotPackaged => R::Missing,
    }
}
fn map_runbook_status(
    status: TrialOperatorRunbookStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    use TrialOperatorRunbookStatus as S;
    match status {
        S::ReadyForManualTrialPreparation | S::TrialOperatorRunbookProjected => R::Present,
        S::Blocked => R::Blocked,
        S::TrialPackageRequired | S::FailureDrillRequired => R::Blocked,
        S::NotAvailable => R::Missing,
    }
}
fn map_trial_session_evidence_status(
    status: TrialSessionEvidenceStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    use TrialSessionEvidenceStatus as S;
    match status {
        S::EvidenceProjected
        | S::EvidenceValidated
        | S::EvidenceWritten
        | S::EvidenceReadBackValidated => R::Present,
        S::EvidenceRejected | S::InvalidEvidenceInput => R::Rejected,
        S::NotCaptured => R::Missing,
    }
}
fn map_replay_restore_verification_status(
    status: TrialReplayRestoreVerificationStatus,
) -> ReleaseCandidatePreparationEvidenceCategoryStatus {
    use ReleaseCandidatePreparationEvidenceCategoryStatus as R;
    use TrialReplayRestoreVerificationStatus as S;
    match status {
        S::VerificationProjected | S::VerificationPassed => R::Present,
        S::VerificationInputMissing => R::Blocked,
        S::VerificationRejected | S::InvalidVerificationInput => R::Rejected,
        S::NotVerified => R::Missing,
    }
}

fn item_from_status(
    category: ReleaseCandidatePreparationEvidenceCategory,
    status: ReleaseCandidatePreparationEvidenceCategoryStatus,
    source_surface: &str,
    source_status: &str,
) -> ReleaseCandidatePreparationEvidenceItem {
    evidence_item(
        category,
        status,
        status_reason(category, status),
        source_surface,
        source_status,
    )
}

fn evidence_item(
    category: ReleaseCandidatePreparationEvidenceCategory,
    status: ReleaseCandidatePreparationEvidenceCategoryStatus,
    reason: &str,
    source_surface: &str,
    source_status: &str,
) -> ReleaseCandidatePreparationEvidenceItem {
    ReleaseCandidatePreparationEvidenceItem {
        category,
        status,
        reason: reason.to_string(),
        source_linkage: ReleaseCandidatePreparationSourceLinkage {
            category,
            source_surface: source_surface.to_string(),
            source_status: source_status.to_string(),
            source_summary: format!("{}:{}", source_surface, source_status),
        },
    }
}

fn status_reason(
    category: ReleaseCandidatePreparationEvidenceCategory,
    status: ReleaseCandidatePreparationEvidenceCategoryStatus,
) -> &'static str {
    match status {
        ReleaseCandidatePreparationEvidenceCategoryStatus::Present => {
            "committed local shell evidence is present"
        }
        ReleaseCandidatePreparationEvidenceCategoryStatus::Missing => match category {
            ReleaseCandidatePreparationEvidenceCategory::UserFacingHelp => {
                "user-facing help evidence is missing"
            }
            ReleaseCandidatePreparationEvidenceCategory::LocalHtmlHelpPages => {
                "local HTML help page evidence is missing"
            }
            ReleaseCandidatePreparationEvidenceCategory::VisibleHelpEntryPoint => {
                "visible help entry point evidence is missing"
            }
            _ => "required committed evidence is missing",
        },
        ReleaseCandidatePreparationEvidenceCategoryStatus::Blocked => {
            "required committed evidence is blocked"
        }
        ReleaseCandidatePreparationEvidenceCategoryStatus::Rejected => {
            "required committed evidence is rejected"
        }
        ReleaseCandidatePreparationEvidenceCategoryStatus::Deferred => {
            "required committed evidence is deferred"
        }
        ReleaseCandidatePreparationEvidenceCategoryStatus::NotApplicable => {
            "evidence category is not applicable"
        }
    }
}

pub fn derive_release_candidate_preparation_contract(
    snapshot: &ReleaseCandidatePreparationInputSnapshot,
) -> ReleaseCandidatePreparationContract {
    let evidence_items = normalized_evidence_items(snapshot);
    let (missing_evidence, blockers) = missing_and_blocked(&evidence_items);
    let validation_errors =
        validate_release_candidate_preparation_contract(snapshot, &evidence_items);
    let validation_status = if validation_errors.is_empty() {
        ReleaseCandidatePreparationValidationStatus::Valid
    } else {
        ReleaseCandidatePreparationValidationStatus::Invalid
    };
    let status = if validation_errors.iter().any(|e| {
        matches!(
            e,
            ReleaseCandidatePreparationValidationError::EvidenceCategoryRejected
                | ReleaseCandidatePreparationValidationError::ReadinessClaimDetected
                | ReleaseCandidatePreparationValidationError::ReleaseClaimDetected
                | ReleaseCandidatePreparationValidationError::DeploymentClaimDetected
                | ReleaseCandidatePreparationValidationError::PublicUseClaimDetected
                | ReleaseCandidatePreparationValidationError::ProductionUseClaimDetected
                | ReleaseCandidatePreparationValidationError::SigningClaimDetected
                | ReleaseCandidatePreparationValidationError::PublishingClaimDetected
                | ReleaseCandidatePreparationValidationError::InstallerClaimDetected
                | ReleaseCandidatePreparationValidationError::UpdateChannelClaimDetected
                | ReleaseCandidatePreparationValidationError::ProviderTrustClaimDetected
                | ReleaseCandidatePreparationValidationError::ActionAuthorizationClaimDetected
                | ReleaseCandidatePreparationValidationError::ReplayRepairClaimDetected
                | ReleaseCandidatePreparationValidationError::RecoveryPromotionClaimDetected
        )
    }) {
        ReleaseCandidatePreparationStatus::PreparationRejected
    } else if validation_errors
        .contains(&ReleaseCandidatePreparationValidationError::EvidenceCategoryBlocked)
    {
        ReleaseCandidatePreparationStatus::PreparationBlocked
    } else if validation_errors.is_empty() {
        ReleaseCandidatePreparationStatus::PreparationValidated
    } else {
        ReleaseCandidatePreparationStatus::InvalidPreparationInput
    };
    ReleaseCandidatePreparationContract {
        preparation_id: deterministic_preparation_id(&evidence_items),
        status,
        validation_status,
        classification: vec![
            ReleaseCandidatePreparationClassification::ReleaseCandidatePreparationOnly,
            ReleaseCandidatePreparationClassification::NonProduction,
            ReleaseCandidatePreparationClassification::ReleaseNotApproved,
            ReleaseCandidatePreparationClassification::NoPublicDistribution,
        ],
        evidence_items,
        missing_evidence,
        blockers,
        validation_errors,
        boundary_statuses: release_candidate_preparation_boundary_statuses(),
        capability_surface: release_candidate_preparation_capability_surface(),
    }
}

pub fn validate_release_candidate_preparation_contract(
    snapshot: &ReleaseCandidatePreparationInputSnapshot,
    evidence_items: &[ReleaseCandidatePreparationEvidenceItem],
) -> Vec<ReleaseCandidatePreparationValidationError> {
    use ReleaseCandidatePreparationEvidenceCategory as Category;
    use ReleaseCandidatePreparationEvidenceCategoryStatus as Status;
    let mut errors = Vec::new();
    for item in evidence_items {
        if item.status == Status::Blocked {
            errors.push(ReleaseCandidatePreparationValidationError::EvidenceCategoryBlocked);
        }
        if item.status == Status::Rejected {
            errors.push(ReleaseCandidatePreparationValidationError::EvidenceCategoryRejected);
        }
    }
    for category in release_candidate_preparation_evidence_categories() {
        let status = evidence_items
            .iter()
            .find(|item| item.category == category)
            .map(|item| item.status)
            .unwrap_or(Status::Missing);
        if status == Status::Missing || status == Status::Deferred {
            errors.push(match category {
                Category::LocalBetaWorkflow => {
                    ReleaseCandidatePreparationValidationError::MissingLocalBetaWorkflow
                }
                Category::ControlledInternalTrialExecution => {
                    ReleaseCandidatePreparationValidationError::MissingTrialExecution
                }
                Category::TrialObservability => {
                    ReleaseCandidatePreparationValidationError::MissingTrialObservability
                }
                Category::TrialEvidenceReview => {
                    ReleaseCandidatePreparationValidationError::MissingTrialEvidenceReview
                }
                Category::UserFacingHelp => {
                    ReleaseCandidatePreparationValidationError::MissingUserHelp
                }
                Category::LocalHtmlHelpPages => {
                    ReleaseCandidatePreparationValidationError::MissingHelpIndex
                }
                Category::VisibleHelpEntryPoint => {
                    ReleaseCandidatePreparationValidationError::MissingVisibleHelpEntryPoint
                }
                Category::LocalCandidateMaterialization => {
                    ReleaseCandidatePreparationValidationError::MissingCandidateMaterialization
                }
                Category::ReplayStatus => {
                    ReleaseCandidatePreparationValidationError::MissingReplayStatus
                }
                Category::LocalSessionPackage => {
                    ReleaseCandidatePreparationValidationError::MissingSessionPackage
                }
                Category::SessionRestoreHistory => {
                    ReleaseCandidatePreparationValidationError::MissingRestoreHistory
                }
                Category::ControlledInternalTrialPackage => {
                    ReleaseCandidatePreparationValidationError::MissingTrialPackage
                }
                Category::TrialSessionEvidence => {
                    ReleaseCandidatePreparationValidationError::MissingTrialSessionEvidence
                }
                Category::ReplayRestoreVerification => {
                    ReleaseCandidatePreparationValidationError::MissingReplayRestoreVerification
                }
                _ => ReleaseCandidatePreparationValidationError::MissingLocalBetaWorkflow,
            });
        }
    }
    if snapshot.claims_readiness {
        errors.push(ReleaseCandidatePreparationValidationError::ReadinessClaimDetected);
    }
    if snapshot.claims_release {
        errors.push(ReleaseCandidatePreparationValidationError::ReleaseClaimDetected);
    }
    if snapshot.claims_deployment {
        errors.push(ReleaseCandidatePreparationValidationError::DeploymentClaimDetected);
    }
    if snapshot.claims_public_use {
        errors.push(ReleaseCandidatePreparationValidationError::PublicUseClaimDetected);
    }
    if snapshot.claims_production_use {
        errors.push(ReleaseCandidatePreparationValidationError::ProductionUseClaimDetected);
    }
    if snapshot.claims_signing {
        errors.push(ReleaseCandidatePreparationValidationError::SigningClaimDetected);
    }
    if snapshot.claims_publishing {
        errors.push(ReleaseCandidatePreparationValidationError::PublishingClaimDetected);
    }
    if snapshot.claims_installer {
        errors.push(ReleaseCandidatePreparationValidationError::InstallerClaimDetected);
    }
    if snapshot.claims_update_channel {
        errors.push(ReleaseCandidatePreparationValidationError::UpdateChannelClaimDetected);
    }
    if snapshot.claims_provider_trust {
        errors.push(ReleaseCandidatePreparationValidationError::ProviderTrustClaimDetected);
    }
    if snapshot.claims_action_authorization {
        errors.push(ReleaseCandidatePreparationValidationError::ActionAuthorizationClaimDetected);
    }
    if snapshot.claims_replay_repair {
        errors.push(ReleaseCandidatePreparationValidationError::ReplayRepairClaimDetected);
    }
    if snapshot.claims_recovery_promotion {
        errors.push(ReleaseCandidatePreparationValidationError::RecoveryPromotionClaimDetected);
    }
    errors.sort();
    errors.dedup();
    errors
}

pub fn derive_release_candidate_preparation_projection(
    snapshot: &ReleaseCandidatePreparationInputSnapshot,
) -> ReleaseCandidatePreparationProjection {
    let contract = derive_release_candidate_preparation_contract(snapshot);
    let category_count = contract.evidence_items.len();
    let present_evidence_count = contract
        .evidence_items
        .iter()
        .filter(|item| item.status == ReleaseCandidatePreparationEvidenceCategoryStatus::Present)
        .count();
    let missing_evidence_count = contract.missing_evidence.len();
    let blocked_evidence_count = contract
        .evidence_items
        .iter()
        .filter(|item| item.status == ReleaseCandidatePreparationEvidenceCategoryStatus::Blocked)
        .count();
    let rejected_evidence_count = contract
        .evidence_items
        .iter()
        .filter(|item| item.status == ReleaseCandidatePreparationEvidenceCategoryStatus::Rejected)
        .count();
    ReleaseCandidatePreparationProjection {
        preparation_id: contract.preparation_id,
        status: contract.status,
        validation_status: contract.validation_status,
        classification: contract.classification,
        evidence_items: contract.evidence_items,
        missing_evidence: contract.missing_evidence,
        blockers: contract.blockers,
        validation_errors: contract.validation_errors,
        boundary_statuses: contract.boundary_statuses,
        capability_surface: contract.capability_surface,
        category_count,
        present_evidence_count,
        missing_evidence_count,
        blocked_evidence_count,
        rejected_evidence_count,
        no_release_readiness_note: "Release-candidate preparation is not release readiness.".to_string(),
        no_release_artifact_note: "This contract does not create release artifacts.".to_string(),
        no_release_candidate_status_note: "This contract does not approve Release Candidate status.".to_string(),
        no_production_deployment_public_signing_publishing_installer_update_note: "This contract does not approve production, deployment, public use, signing, publishing, installer, or update-channel behavior.".to_string(),
        no_provider_trust_note: "Provider output remains untrusted.".to_string(),
        no_action_authorization_note: "No action authorization is granted.".to_string(),
    }
}

fn normalized_evidence_items(
    snapshot: &ReleaseCandidatePreparationInputSnapshot,
) -> Vec<ReleaseCandidatePreparationEvidenceItem> {
    let mut items = Vec::new();
    for category in release_candidate_preparation_evidence_categories() {
        if let Some(item) = snapshot
            .evidence_items
            .iter()
            .find(|item| item.category == category)
        {
            items.push(item.clone());
        } else {
            items.push(evidence_item(
                category,
                ReleaseCandidatePreparationEvidenceCategoryStatus::Missing,
                "closed evidence category is absent from input snapshot",
                category.code(),
                "missing",
            ));
        }
    }
    items
}

fn missing_and_blocked(
    evidence_items: &[ReleaseCandidatePreparationEvidenceItem],
) -> (
    Vec<ReleaseCandidatePreparationMissingEvidence>,
    Vec<ReleaseCandidatePreparationBlocker>,
) {
    let mut missing = Vec::new();
    let mut blockers = Vec::new();
    for item in evidence_items {
        if matches!(
            item.status,
            ReleaseCandidatePreparationEvidenceCategoryStatus::Missing
                | ReleaseCandidatePreparationEvidenceCategoryStatus::Deferred
        ) {
            missing.push(ReleaseCandidatePreparationMissingEvidence {
                category: item.category,
                reason: item.reason.clone(),
                source_surface: item.source_linkage.source_surface.clone(),
            });
        }
        if matches!(
            item.status,
            ReleaseCandidatePreparationEvidenceCategoryStatus::Blocked
                | ReleaseCandidatePreparationEvidenceCategoryStatus::Rejected
        ) {
            blockers.push(ReleaseCandidatePreparationBlocker {
                category: item.category,
                reason: item.reason.clone(),
                source_surface: item.source_linkage.source_surface.clone(),
            });
        }
    }
    (missing, blockers)
}

fn deterministic_preparation_id(
    evidence_items: &[ReleaseCandidatePreparationEvidenceItem],
) -> String {
    let mut hash = 0x811c9dc5_u32;
    for item in evidence_items {
        for byte in format!(
            "{}:{}:{};",
            item.category.code(),
            item.status.code(),
            item.source_linkage.source_status
        )
        .bytes()
        {
            hash ^= u32::from(byte);
            hash = hash.wrapping_mul(0x01000193);
        }
    }
    format!("release-candidate-preparation-{hash:08x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(
        category: ReleaseCandidatePreparationEvidenceCategory,
        status: ReleaseCandidatePreparationEvidenceCategoryStatus,
    ) -> ReleaseCandidatePreparationEvidenceItem {
        evidence_item(category, status, "test", category.code(), status.code())
    }

    #[test]
    fn evidence_categories_are_closed_and_deterministic() {
        let first = release_candidate_preparation_evidence_categories();
        let second = release_candidate_preparation_evidence_categories();
        assert_eq!(first, second);
        assert_eq!(first.len(), 30);
        assert_eq!(first[0].code(), "local_beta_workflow");
        assert_eq!(first[29].code(), "deterministic_validation");
    }

    #[test]
    fn complete_evidence_maps_into_valid_preparation_contract() {
        let projection = derive_release_candidate_preparation_projection(
            &complete_release_candidate_preparation_input_snapshot(),
        );
        assert_eq!(
            projection.status,
            ReleaseCandidatePreparationStatus::PreparationValidated
        );
        assert_eq!(
            projection.validation_status,
            ReleaseCandidatePreparationValidationStatus::Valid
        );
        assert_eq!(projection.present_evidence_count, 30);
        assert!(projection.missing_evidence.is_empty());
        assert!(projection.blockers.is_empty());
        assert!(projection
            .classification
            .contains(&ReleaseCandidatePreparationClassification::ReleaseCandidatePreparationOnly));
        assert!(projection
            .classification
            .contains(&ReleaseCandidatePreparationClassification::NonProduction));
        assert!(projection
            .classification
            .contains(&ReleaseCandidatePreparationClassification::ReleaseNotApproved));
        assert!(projection
            .classification
            .contains(&ReleaseCandidatePreparationClassification::NoPublicDistribution));
    }

    #[test]
    fn missing_evidence_fails_closed_and_orders_missing_items() {
        let mut snapshot = complete_release_candidate_preparation_input_snapshot();
        snapshot.evidence_items = vec![item(
            ReleaseCandidatePreparationEvidenceCategory::VisibleHelpEntryPoint,
            ReleaseCandidatePreparationEvidenceCategoryStatus::Missing,
        )];
        let projection = derive_release_candidate_preparation_projection(&snapshot);
        assert_eq!(
            projection.validation_status,
            ReleaseCandidatePreparationValidationStatus::Invalid
        );
        assert!(projection
            .validation_errors
            .contains(&ReleaseCandidatePreparationValidationError::MissingVisibleHelpEntryPoint));
        assert_eq!(
            projection.missing_evidence[0].category,
            ReleaseCandidatePreparationEvidenceCategory::LocalBetaWorkflow
        );
    }

    #[test]
    fn blocked_and_rejected_evidence_fail_closed() {
        let mut blocked = complete_release_candidate_preparation_input_snapshot();
        blocked.evidence_items[0] = item(
            ReleaseCandidatePreparationEvidenceCategory::LocalBetaWorkflow,
            ReleaseCandidatePreparationEvidenceCategoryStatus::Blocked,
        );
        let blocked_projection = derive_release_candidate_preparation_projection(&blocked);
        assert_eq!(
            blocked_projection.status,
            ReleaseCandidatePreparationStatus::PreparationBlocked
        );
        assert!(blocked_projection
            .validation_errors
            .contains(&ReleaseCandidatePreparationValidationError::EvidenceCategoryBlocked));

        let mut rejected = complete_release_candidate_preparation_input_snapshot();
        rejected.evidence_items[1] = item(
            ReleaseCandidatePreparationEvidenceCategory::ControlledInternalTrialExecution,
            ReleaseCandidatePreparationEvidenceCategoryStatus::Rejected,
        );
        let rejected_projection = derive_release_candidate_preparation_projection(&rejected);
        assert_eq!(
            rejected_projection.status,
            ReleaseCandidatePreparationStatus::PreparationRejected
        );
        assert!(rejected_projection
            .validation_errors
            .contains(&ReleaseCandidatePreparationValidationError::EvidenceCategoryRejected));
    }

    #[test]
    fn no_authority_boundary_markers_and_capabilities_stay_disabled() {
        let projection = derive_release_candidate_preparation_projection(
            &complete_release_candidate_preparation_input_snapshot(),
        );
        for marker in [
            ReleaseCandidatePreparationBoundaryStatus::ReleaseCandidatePreparationOnly,
            ReleaseCandidatePreparationBoundaryStatus::ReleaseReadinessNotApproved,
            ReleaseCandidatePreparationBoundaryStatus::ReleaseCandidateStatusNotApproved,
            ReleaseCandidatePreparationBoundaryStatus::NoReleaseArtifact,
            ReleaseCandidatePreparationBoundaryStatus::NoSigning,
            ReleaseCandidatePreparationBoundaryStatus::NoPublishing,
            ReleaseCandidatePreparationBoundaryStatus::NoInstallerActivation,
            ReleaseCandidatePreparationBoundaryStatus::NoUpdateChannelActivation,
            ReleaseCandidatePreparationBoundaryStatus::NoProviderTrust,
            ReleaseCandidatePreparationBoundaryStatus::NoActionAuthorization,
            ReleaseCandidatePreparationBoundaryStatus::NoReplayRepair,
            ReleaseCandidatePreparationBoundaryStatus::NoRecoveryPromotion,
        ] {
            assert!(projection.boundary_statuses.contains(&marker));
        }
        assert!(!projection.capability_surface.creates_release_artifacts);
        assert!(!projection.capability_surface.signs_artifacts);
        assert!(!projection.capability_surface.publishes_artifacts);
        assert!(!projection.capability_surface.trusts_provider_output);
        assert!(!projection.capability_surface.authorizes_actions);
    }

    #[test]
    fn authority_bearing_claims_are_rejected() {
        let mut snapshot = complete_release_candidate_preparation_input_snapshot();
        snapshot.claims_readiness = true;
        snapshot.claims_release = true;
        snapshot.claims_deployment = true;
        snapshot.claims_public_use = true;
        snapshot.claims_production_use = true;
        snapshot.claims_signing = true;
        snapshot.claims_publishing = true;
        snapshot.claims_installer = true;
        snapshot.claims_update_channel = true;
        snapshot.claims_provider_trust = true;
        snapshot.claims_action_authorization = true;
        snapshot.claims_replay_repair = true;
        snapshot.claims_recovery_promotion = true;
        let projection = derive_release_candidate_preparation_projection(&snapshot);
        assert_eq!(
            projection.status,
            ReleaseCandidatePreparationStatus::PreparationRejected
        );
        assert!(projection
            .validation_errors
            .contains(&ReleaseCandidatePreparationValidationError::ReadinessClaimDetected));
        assert!(projection
            .validation_errors
            .contains(&ReleaseCandidatePreparationValidationError::RecoveryPromotionClaimDetected));
    }

    #[test]
    fn projection_and_id_are_deterministic_for_identical_shell_state() {
        let state = initial_local_operator_shell_state();
        let before = state.clone();
        let first = derive_release_candidate_preparation_projection(
            &derive_release_candidate_preparation_input_snapshot(&state),
        );
        let second = derive_release_candidate_preparation_projection(
            &derive_release_candidate_preparation_input_snapshot(&state),
        );
        assert_eq!(first, second);
        assert_eq!(state, before);
    }
}
