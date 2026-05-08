use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceAuthorityDecisionStatus {
    Phase110NarrowActivationCandidatePermitted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposedPersistenceScopeClassification {
    DecisionEvidenceOnly,
    Phase110NarrowRustValidatedAppendCandidate,
    ProhibitedAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowedPersistenceCandidateCategory {
    Phase110RustValidatedDecisionEvidenceAppend,
}

impl AllowedPersistenceCandidateCategory {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Phase110RustValidatedDecisionEvidenceAppend => {
                "phase_110_rust_validated_decision_evidence_append"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProhibitedPersistenceCategory {
    ProviderOutputAuthority,
    WorkflowCompletionAuthority,
    ReplayRepairAuthority,
    RecoveryPromotionAuthority,
    ActionExecutionAuthority,
    UiAuthorizedPersistence,
    TransportAuthorizedPersistence,
    ImplicitTrustPromotion,
    ImplicitReadinessPromotion,
}

impl ProhibitedPersistenceCategory {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ProviderOutputAuthority => "provider_output_authority",
            Self::WorkflowCompletionAuthority => "workflow_completion_authority",
            Self::ReplayRepairAuthority => "replay_repair_authority",
            Self::RecoveryPromotionAuthority => "recovery_promotion_authority",
            Self::ActionExecutionAuthority => "action_execution_authority",
            Self::UiAuthorizedPersistence => "ui_authorized_persistence",
            Self::TransportAuthorizedPersistence => "transport_authorized_persistence",
            Self::ImplicitTrustPromotion => "implicit_trust_promotion",
            Self::ImplicitReadinessPromotion => "implicit_readiness_promotion",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PersistenceAuthorityDecisionReasonCode {
    DecisionEvidenceOnly,
    DescriptiveOnlyEvidence,
    NoPhase109PersistenceActivation,
    Phase110NarrowCandidatePermitted,
    Phase110RequiresRustValidatedBoundary,
    Phase110RequiresCommittedEvidenceReferences,
    ProhibitedPersistenceCategoryRejected,
    ProviderOutputNotAuthority,
    WorkflowCompletionNotAuthority,
    SandboxSuccessNotAuthority,
    ReplayRepairProhibited,
    RecoveryPromotionProhibited,
    ActionExecutionProhibited,
    UiAuthorityProhibited,
    TransportAuthorityProhibited,
    TrustPromotionProhibited,
    ReadinessPromotionProhibited,
    MalformedEvidenceRejected,
}

impl PersistenceAuthorityDecisionReasonCode {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DecisionEvidenceOnly => "decision_evidence_only",
            Self::DescriptiveOnlyEvidence => "descriptive_only_evidence",
            Self::NoPhase109PersistenceActivation => "no_phase_109_persistence_activation",
            Self::Phase110NarrowCandidatePermitted => "phase_110_narrow_candidate_permitted",
            Self::Phase110RequiresRustValidatedBoundary => {
                "phase_110_requires_rust_validated_boundary"
            }
            Self::Phase110RequiresCommittedEvidenceReferences => {
                "phase_110_requires_committed_evidence_references"
            }
            Self::ProhibitedPersistenceCategoryRejected => {
                "prohibited_persistence_category_rejected"
            }
            Self::ProviderOutputNotAuthority => "provider_output_not_authority",
            Self::WorkflowCompletionNotAuthority => "workflow_completion_not_authority",
            Self::SandboxSuccessNotAuthority => "sandbox_success_not_authority",
            Self::ReplayRepairProhibited => "replay_repair_prohibited",
            Self::RecoveryPromotionProhibited => "recovery_promotion_prohibited",
            Self::ActionExecutionProhibited => "action_execution_prohibited",
            Self::UiAuthorityProhibited => "ui_authority_prohibited",
            Self::TransportAuthorityProhibited => "transport_authority_prohibited",
            Self::TrustPromotionProhibited => "trust_promotion_prohibited",
            Self::ReadinessPromotionProhibited => "readiness_promotion_prohibited",
            Self::MalformedEvidenceRejected => "malformed_evidence_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProposedPersistenceBoundary {
    pub boundary_id: String,
    pub scope: ProposedPersistenceScopeClassification,
    pub allowed_candidate_categories: Vec<AllowedPersistenceCandidateCategory>,
    pub prohibited_categories: Vec<ProhibitedPersistenceCategory>,
    pub workflow_evidence_refs: Vec<String>,
    pub sandbox_evidence_refs: Vec<String>,
    pub governance_evidence_refs: Vec<String>,
    pub provider_execution_succeeded: bool,
    pub workflow_completed: bool,
    pub provider_output_present: bool,
    pub requested_by_ui: bool,
    pub requested_by_transport: bool,
    pub malformed_authority_request: bool,
}

impl ProposedPersistenceBoundary {
    pub fn phase_110_narrow_candidate(boundary_id: impl Into<String>) -> Self {
        Self {
            boundary_id: boundary_id.into(),
            scope:
                ProposedPersistenceScopeClassification::Phase110NarrowRustValidatedAppendCandidate,
            allowed_candidate_categories: vec![
                AllowedPersistenceCandidateCategory::Phase110RustValidatedDecisionEvidenceAppend,
            ],
            prohibited_categories: Vec::new(),
            workflow_evidence_refs: vec!["checklists/current-phase.md".to_string()],
            sandbox_evidence_refs: vec![
                "docs/operations/provider-timeout-resource-boundary-phase-108.md".to_string(),
            ],
            governance_evidence_refs: vec!["docs/roadmap/phases.md".to_string()],
            provider_execution_succeeded: false,
            workflow_completed: false,
            provider_output_present: false,
            requested_by_ui: false,
            requested_by_transport: false,
            malformed_authority_request: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeAuthorityEvidence {
    pub descriptive_only: bool,
    pub grants_provider_trust: bool,
    pub grants_readiness: bool,
    pub grants_workflow_completion_authority: bool,
    pub grants_provider_output_authority: bool,
    pub activates_persistence: bool,
    pub durable_append_enabled: bool,
    pub replay_repair_enabled: bool,
    pub recovery_promotion_enabled: bool,
    pub action_execution_enabled: bool,
    pub ui_authority_enabled: bool,
    pub transport_authority_enabled: bool,
    pub no_promotion: bool,
    pub no_replay_repair: bool,
    pub no_recovery_promotion: bool,
    pub no_action_execution: bool,
}

impl NegativeAuthorityEvidence {
    fn phase_109() -> Self {
        Self {
            descriptive_only: true,
            grants_provider_trust: false,
            grants_readiness: false,
            grants_workflow_completion_authority: false,
            grants_provider_output_authority: false,
            activates_persistence: false,
            durable_append_enabled: false,
            replay_repair_enabled: false,
            recovery_promotion_enabled: false,
            action_execution_enabled: false,
            ui_authority_enabled: false,
            transport_authority_enabled: false,
            no_promotion: true,
            no_replay_repair: true,
            no_recovery_promotion: true,
            no_action_execution: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceAuthorityConstraintSet {
    pub phase_110_only: bool,
    pub rust_boundary_required: bool,
    pub committed_evidence_required: bool,
    pub descriptive_provider_evidence_only: bool,
    pub provider_output_authority_prohibited: bool,
    pub workflow_completion_authority_prohibited: bool,
    pub replay_repair_prohibited: bool,
    pub recovery_promotion_prohibited: bool,
    pub action_execution_prohibited: bool,
    pub ui_authority_prohibited: bool,
    pub transport_authority_prohibited: bool,
    pub trust_promotion_prohibited: bool,
    pub readiness_promotion_prohibited: bool,
}

impl PersistenceAuthorityConstraintSet {
    fn phase_110_constraints() -> Self {
        Self {
            phase_110_only: true,
            rust_boundary_required: true,
            committed_evidence_required: true,
            descriptive_provider_evidence_only: true,
            provider_output_authority_prohibited: true,
            workflow_completion_authority_prohibited: true,
            replay_repair_prohibited: true,
            recovery_promotion_prohibited: true,
            action_execution_prohibited: true,
            ui_authority_prohibited: true,
            transport_authority_prohibited: true,
            trust_promotion_prohibited: true,
            readiness_promotion_prohibited: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurablePersistenceAuthorityDecisionEvidence {
    pub decision_id: String,
    pub status: PersistenceAuthorityDecisionStatus,
    pub proposed_boundary: ProposedPersistenceBoundary,
    pub negative_authority: NegativeAuthorityEvidence,
    pub constraint_set: PersistenceAuthorityConstraintSet,
    pub prohibited_categories: Vec<ProhibitedPersistenceCategory>,
    pub permitted_phase_110_candidate_categories: Vec<AllowedPersistenceCandidateCategory>,
    pub reason_codes: Vec<PersistenceAuthorityDecisionReasonCode>,
    pub reason_code_strings: Vec<&'static str>,
    pub descriptive_only: bool,
    pub non_authoritative: bool,
    pub non_self_activating: bool,
    pub persistence_activated: bool,
    pub durable_append_activated: bool,
    pub replay_repair_activated: bool,
    pub recovery_promotion_activated: bool,
    pub action_execution_activated: bool,
    pub readiness_approved: bool,
    pub production_candidate_approved: bool,
    pub release_candidate_approved: bool,
    pub public_usability_approved: bool,
    pub production_human_use_approved: bool,
}

pub fn evaluate_durable_persistence_authority_decision(
    proposed_boundary: ProposedPersistenceBoundary,
) -> DurablePersistenceAuthorityDecisionEvidence {
    let mut reason_codes = BTreeSet::from([
        PersistenceAuthorityDecisionReasonCode::DecisionEvidenceOnly,
        PersistenceAuthorityDecisionReasonCode::DescriptiveOnlyEvidence,
        PersistenceAuthorityDecisionReasonCode::NoPhase109PersistenceActivation,
        PersistenceAuthorityDecisionReasonCode::Phase110RequiresRustValidatedBoundary,
        PersistenceAuthorityDecisionReasonCode::Phase110RequiresCommittedEvidenceReferences,
    ]);

    let mut prohibited_categories: BTreeSet<ProhibitedPersistenceCategory> = proposed_boundary
        .prohibited_categories
        .iter()
        .copied()
        .collect();

    if proposed_boundary.provider_output_present {
        reason_codes.insert(PersistenceAuthorityDecisionReasonCode::ProviderOutputNotAuthority);
    }
    if proposed_boundary.workflow_completed {
        reason_codes.insert(PersistenceAuthorityDecisionReasonCode::WorkflowCompletionNotAuthority);
    }
    if proposed_boundary.provider_execution_succeeded {
        reason_codes.insert(PersistenceAuthorityDecisionReasonCode::SandboxSuccessNotAuthority);
    }
    if proposed_boundary.requested_by_ui {
        prohibited_categories.insert(ProhibitedPersistenceCategory::UiAuthorizedPersistence);
    }
    if proposed_boundary.requested_by_transport {
        prohibited_categories.insert(ProhibitedPersistenceCategory::TransportAuthorizedPersistence);
    }

    for category in &prohibited_categories {
        reason_codes
            .insert(PersistenceAuthorityDecisionReasonCode::ProhibitedPersistenceCategoryRejected);
        reason_codes.insert(prohibited_reason_code(*category));
    }

    let malformed = proposed_boundary.malformed_authority_request
        || proposed_boundary.boundary_id.trim().is_empty()
        || proposed_boundary.workflow_evidence_refs.is_empty()
        || proposed_boundary.sandbox_evidence_refs.is_empty()
        || proposed_boundary.governance_evidence_refs.is_empty();
    if malformed {
        reason_codes.insert(PersistenceAuthorityDecisionReasonCode::MalformedEvidenceRejected);
    }

    let can_permit_phase_110_candidate = !malformed
        && prohibited_categories.is_empty()
        && proposed_boundary.scope
            == ProposedPersistenceScopeClassification::Phase110NarrowRustValidatedAppendCandidate
        && proposed_boundary.allowed_candidate_categories
            == [AllowedPersistenceCandidateCategory::Phase110RustValidatedDecisionEvidenceAppend];

    if can_permit_phase_110_candidate {
        reason_codes
            .insert(PersistenceAuthorityDecisionReasonCode::Phase110NarrowCandidatePermitted);
    }

    let reason_codes: Vec<_> = reason_codes.into_iter().collect();
    let reason_code_strings = reason_codes.iter().map(|reason| reason.code()).collect();

    DurablePersistenceAuthorityDecisionEvidence {
        decision_id: format!("phase-109-decision:{}", proposed_boundary.boundary_id),
        status: if can_permit_phase_110_candidate {
            PersistenceAuthorityDecisionStatus::Phase110NarrowActivationCandidatePermitted
        } else {
            PersistenceAuthorityDecisionStatus::Rejected
        },
        proposed_boundary,
        negative_authority: NegativeAuthorityEvidence::phase_109(),
        constraint_set: PersistenceAuthorityConstraintSet::phase_110_constraints(),
        prohibited_categories: prohibited_categories.into_iter().collect(),
        permitted_phase_110_candidate_categories: if can_permit_phase_110_candidate {
            vec![AllowedPersistenceCandidateCategory::Phase110RustValidatedDecisionEvidenceAppend]
        } else {
            Vec::new()
        },
        reason_codes,
        reason_code_strings,
        descriptive_only: true,
        non_authoritative: true,
        non_self_activating: true,
        persistence_activated: false,
        durable_append_activated: false,
        replay_repair_activated: false,
        recovery_promotion_activated: false,
        action_execution_activated: false,
        readiness_approved: false,
        production_candidate_approved: false,
        release_candidate_approved: false,
        public_usability_approved: false,
        production_human_use_approved: false,
    }
}

pub fn durable_persistence_decision_activates_authority(
    evidence: &DurablePersistenceAuthorityDecisionEvidence,
) -> bool {
    evidence.persistence_activated
        || evidence.durable_append_activated
        || evidence.replay_repair_activated
        || evidence.recovery_promotion_activated
        || evidence.action_execution_activated
        || evidence.readiness_approved
        || evidence.production_candidate_approved
        || evidence.release_candidate_approved
        || evidence.public_usability_approved
        || evidence.production_human_use_approved
        || !evidence.negative_authority.descriptive_only
        || evidence.negative_authority.grants_provider_trust
        || evidence.negative_authority.grants_readiness
        || evidence.negative_authority.activates_persistence
}

fn prohibited_reason_code(
    category: ProhibitedPersistenceCategory,
) -> PersistenceAuthorityDecisionReasonCode {
    match category {
        ProhibitedPersistenceCategory::ProviderOutputAuthority => {
            PersistenceAuthorityDecisionReasonCode::ProviderOutputNotAuthority
        }
        ProhibitedPersistenceCategory::WorkflowCompletionAuthority => {
            PersistenceAuthorityDecisionReasonCode::WorkflowCompletionNotAuthority
        }
        ProhibitedPersistenceCategory::ReplayRepairAuthority => {
            PersistenceAuthorityDecisionReasonCode::ReplayRepairProhibited
        }
        ProhibitedPersistenceCategory::RecoveryPromotionAuthority => {
            PersistenceAuthorityDecisionReasonCode::RecoveryPromotionProhibited
        }
        ProhibitedPersistenceCategory::ActionExecutionAuthority => {
            PersistenceAuthorityDecisionReasonCode::ActionExecutionProhibited
        }
        ProhibitedPersistenceCategory::UiAuthorizedPersistence => {
            PersistenceAuthorityDecisionReasonCode::UiAuthorityProhibited
        }
        ProhibitedPersistenceCategory::TransportAuthorizedPersistence => {
            PersistenceAuthorityDecisionReasonCode::TransportAuthorityProhibited
        }
        ProhibitedPersistenceCategory::ImplicitTrustPromotion => {
            PersistenceAuthorityDecisionReasonCode::TrustPromotionProhibited
        }
        ProhibitedPersistenceCategory::ImplicitReadinessPromotion => {
            PersistenceAuthorityDecisionReasonCode::ReadinessPromotionProhibited
        }
    }
}
