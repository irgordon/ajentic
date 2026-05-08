use std::collections::{BTreeMap, BTreeSet};

use super::persistence::{
    calculate_persisted_record_checksum, write_phase_111_decision_evidence_append_bytes,
    LocalPersistencePlan,
};

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
    SandboxSuccessAuthority,
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
            Self::SandboxSuccessAuthority => "sandbox_success_authority",
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
    DuplicateOrAmbiguousEvidenceRejected,
    DecisionEvidenceAppendVerified,
    DecisionEvidenceAppendWriteFailed,
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
            Self::DuplicateOrAmbiguousEvidenceRejected => {
                "duplicate_or_ambiguous_evidence_rejected"
            }
            Self::DecisionEvidenceAppendVerified => "decision_evidence_append_verified",
            Self::DecisionEvidenceAppendWriteFailed => "decision_evidence_append_write_failed",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase111DecisionEvidenceAppendStatus {
    Appended,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase111DecisionEvidenceAppendRejection {
    None,
    InvalidDecisionEvidence,
    ProhibitedProviderOutputAuthority,
    ProhibitedWorkflowCompletionAuthority,
    ProhibitedSandboxSuccessAuthority,
    ProhibitedUiAuthorizedPersistence,
    ProhibitedTransportAuthorizedPersistence,
    ProhibitedReplayRepairAuthority,
    ProhibitedRecoveryPromotionAuthority,
    ProhibitedActionExecutionAuthority,
    ProhibitedTrustOrReadinessApproval,
    DuplicateOrAmbiguousDecisionEvidence,
    InvalidPersistencePlan,
    AppendWriteFailed,
}

impl Phase111DecisionEvidenceAppendRejection {
    pub fn code(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::InvalidDecisionEvidence => "invalid_decision_evidence",
            Self::ProhibitedProviderOutputAuthority => "prohibited_provider_output_authority",
            Self::ProhibitedWorkflowCompletionAuthority => {
                "prohibited_workflow_completion_authority"
            }
            Self::ProhibitedSandboxSuccessAuthority => "prohibited_sandbox_success_authority",
            Self::ProhibitedUiAuthorizedPersistence => "prohibited_ui_authorized_persistence",
            Self::ProhibitedTransportAuthorizedPersistence => {
                "prohibited_transport_authorized_persistence"
            }
            Self::ProhibitedReplayRepairAuthority => "prohibited_replay_repair_authority",
            Self::ProhibitedRecoveryPromotionAuthority => "prohibited_recovery_promotion_authority",
            Self::ProhibitedActionExecutionAuthority => "prohibited_action_execution_authority",
            Self::ProhibitedTrustOrReadinessApproval => "prohibited_trust_or_readiness_approval",
            Self::DuplicateOrAmbiguousDecisionEvidence => {
                "duplicate_or_ambiguous_decision_evidence"
            }
            Self::InvalidPersistencePlan => "invalid_persistence_plan",
            Self::AppendWriteFailed => "append_write_failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase111DecisionEvidenceAppendRecord {
    pub record_type: &'static str,
    pub decision_evidence_id: String,
    pub decision_status: PersistenceAuthorityDecisionStatus,
    pub reason_codes: Vec<&'static str>,
    pub proposed_boundary_classification: ProposedPersistenceScopeClassification,
    pub phase_110_constraint_snapshot: PersistenceAuthorityConstraintSet,
    pub negative_authority_snapshot: NegativeAuthorityEvidence,
    pub referenced_sandbox_evidence: Vec<String>,
    pub referenced_workflow_evidence: Vec<String>,
    pub validation_result: &'static str,
    pub provider_output_authority: bool,
    pub workflow_completion_authority: bool,
    pub sandbox_success_authority: bool,
    pub ui_authorized_persistence: bool,
    pub transport_authorized_persistence: bool,
    pub replay_repair_authority: bool,
    pub recovery_promotion_authority: bool,
    pub action_execution_authority: bool,
    pub readiness_approval: bool,
    pub production_candidate_approval: bool,
    pub release_candidate_approval: bool,
    pub public_use_approval: bool,
    pub production_human_use_approval: bool,
    pub provider_output_trusted: bool,
    pub provider_output_promoted: bool,
    pub deterministic_integrity_marker: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase111DecisionEvidenceAppendReport {
    pub status: Phase111DecisionEvidenceAppendStatus,
    pub rejection: Phase111DecisionEvidenceAppendRejection,
    pub accepted_record_type: &'static str,
    pub decision_evidence_id: String,
    pub record_checksum: Option<String>,
    pub bytes_written: usize,
    pub committed: bool,
    pub provider_output_trusted: bool,
    pub provider_output_promoted: bool,
    pub workflow_completion_authority: bool,
    pub sandbox_success_authority: bool,
    pub ui_transport_authority: bool,
    pub replay_repair_authority: bool,
    pub recovery_promotion_authority: bool,
    pub action_execution_authority: bool,
    pub readiness_approved: bool,
    pub production_candidate_approved: bool,
    pub release_candidate_approved: bool,
    pub public_use_approved: bool,
    pub production_human_use_approved: bool,
    pub summary: String,
}

const PHASE_111_RECORD_TYPE: &str = "phase_111_rust_validated_decision_evidence_append";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryLifecycleStatus {
    EvidencePresent,
    ManualReviewRequired,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecoveryLifecycleReason {
    RecoveryEvidencePresent,
    RecoveryEvidenceMissing,
    RecoveryEvidenceMalformed,
    RecoveryChecksumMismatch,
    RecoveryUnsupportedRecordType,
    RecoveryDuplicateEvidence,
    RecoveryConflictingEvidence,
    RecoveryStaleConstraintSnapshot,
    RecoveryMissingNegativeAuthorityEvidence,
    RecoveryManualReviewRequired,
    RecoveryRejected,
    RecoveryClassificationOnly,
}

impl RecoveryLifecycleReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::RecoveryEvidencePresent => "recovery_evidence_present",
            Self::RecoveryEvidenceMissing => "recovery_evidence_missing",
            Self::RecoveryEvidenceMalformed => "recovery_evidence_malformed",
            Self::RecoveryChecksumMismatch => "recovery_checksum_mismatch",
            Self::RecoveryUnsupportedRecordType => "recovery_unsupported_record_type",
            Self::RecoveryDuplicateEvidence => "recovery_duplicate_evidence",
            Self::RecoveryConflictingEvidence => "recovery_conflicting_evidence",
            Self::RecoveryStaleConstraintSnapshot => "recovery_stale_constraint_snapshot",
            Self::RecoveryMissingNegativeAuthorityEvidence => {
                "recovery_missing_negative_authority_evidence"
            }
            Self::RecoveryManualReviewRequired => "recovery_manual_review_required",
            Self::RecoveryRejected => "recovery_rejected",
            Self::RecoveryClassificationOnly => "recovery_classification_only",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryEvidenceInspection {
    pub record_index: usize,
    pub record_type: Option<String>,
    pub decision_evidence_id: Option<String>,
    pub checksum: Option<String>,
    pub reasons: Vec<RecoveryLifecycleReason>,
    pub manual_review_required: bool,
    pub rejected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryManualReviewRequirement {
    pub required: bool,
    pub reasons: Vec<RecoveryLifecycleReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryAuthorityDenialSnapshot {
    pub repaired_replay: bool,
    pub promoted_recovery: bool,
    pub executed_action: bool,
    pub trusted_provider_output: bool,
    pub promoted_provider_output: bool,
    pub accepted_workflow_completion: bool,
    pub accepted_sandbox_success: bool,
    pub accepted_ui_transport_authority: bool,
    pub approved_readiness: bool,
    pub approved_production_candidate: bool,
    pub approved_release_candidate: bool,
    pub approved_public_use: bool,
    pub approved_production_human_use: bool,
}

impl RecoveryAuthorityDenialSnapshot {
    fn all_denied() -> Self {
        Self {
            repaired_replay: false,
            promoted_recovery: false,
            executed_action: false,
            trusted_provider_output: false,
            promoted_provider_output: false,
            accepted_workflow_completion: false,
            accepted_sandbox_success: false,
            accepted_ui_transport_authority: false,
            approved_readiness: false,
            approved_production_candidate: false,
            approved_release_candidate: false,
            approved_public_use: false,
            approved_production_human_use: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryLifecycleReport {
    pub status: RecoveryLifecycleStatus,
    pub reasons: Vec<RecoveryLifecycleReason>,
    pub reason_codes: Vec<&'static str>,
    pub inspections: Vec<RecoveryEvidenceInspection>,
    pub manual_review: RecoveryManualReviewRequirement,
    pub authority_denial: RecoveryAuthorityDenialSnapshot,
    pub repaired_replay: bool,
    pub promoted_recovery: bool,
    pub executed_action: bool,
    pub trusted_provider_output: bool,
    pub promoted_provider_output: bool,
    pub accepted_workflow_completion: bool,
    pub accepted_sandbox_success: bool,
    pub accepted_ui_transport_authority: bool,
    pub approved_readiness: bool,
    pub approved_production_candidate: bool,
    pub approved_release_candidate: bool,
    pub approved_public_use: bool,
    pub approved_production_human_use: bool,
}

pub fn inspect_phase_111_recovery_lifecycle(records: &[Vec<u8>]) -> RecoveryLifecycleReport {
    let mut reasons = BTreeSet::from([RecoveryLifecycleReason::RecoveryClassificationOnly]);
    let mut inspections = Vec::new();
    let mut valid_ids = BTreeSet::new();
    let mut valid_record_fingerprints = BTreeSet::new();

    if records.is_empty() {
        reasons.insert(RecoveryLifecycleReason::RecoveryEvidenceMissing);
        reasons.insert(RecoveryLifecycleReason::RecoveryManualReviewRequired);
        reasons.insert(RecoveryLifecycleReason::RecoveryRejected);
    }

    for (index, bytes) in records.iter().enumerate() {
        let inspection = inspect_one_phase_111_recovery_record(index, bytes);
        if inspection.rejected || inspection.manual_review_required {
            reasons.extend(inspection.reasons.iter().copied());
            reasons.insert(RecoveryLifecycleReason::RecoveryManualReviewRequired);
            reasons.insert(RecoveryLifecycleReason::RecoveryRejected);
        } else if let Some(decision_id) = inspection.decision_evidence_id.as_ref() {
            reasons.insert(RecoveryLifecycleReason::RecoveryEvidencePresent);
            if !valid_ids.insert(decision_id.clone()) {
                reasons.insert(RecoveryLifecycleReason::RecoveryDuplicateEvidence);
                reasons.insert(RecoveryLifecycleReason::RecoveryManualReviewRequired);
                reasons.insert(RecoveryLifecycleReason::RecoveryRejected);
            }
            if let Some(checksum) = inspection.checksum.as_ref() {
                valid_record_fingerprints.insert((decision_id.clone(), checksum.clone()));
            }
        }
        inspections.push(inspection);
    }

    if valid_ids.len() > 1 || valid_record_fingerprints.len() > 1 {
        reasons.insert(RecoveryLifecycleReason::RecoveryConflictingEvidence);
        reasons.insert(RecoveryLifecycleReason::RecoveryManualReviewRequired);
        reasons.insert(RecoveryLifecycleReason::RecoveryRejected);
    }

    let reasons: Vec<_> = reasons.into_iter().collect();
    let manual_review_required =
        reasons.contains(&RecoveryLifecycleReason::RecoveryManualReviewRequired);
    let rejected = reasons.contains(&RecoveryLifecycleReason::RecoveryRejected);
    let status = if rejected {
        RecoveryLifecycleStatus::Rejected
    } else if manual_review_required {
        RecoveryLifecycleStatus::ManualReviewRequired
    } else {
        RecoveryLifecycleStatus::EvidencePresent
    };
    let reason_codes = reasons.iter().map(|reason| reason.code()).collect();
    let authority_denial = RecoveryAuthorityDenialSnapshot::all_denied();
    RecoveryLifecycleReport {
        status,
        reason_codes,
        inspections,
        manual_review: RecoveryManualReviewRequirement {
            required: manual_review_required,
            reasons: reasons.clone(),
        },
        authority_denial: authority_denial.clone(),
        repaired_replay: authority_denial.repaired_replay,
        promoted_recovery: authority_denial.promoted_recovery,
        executed_action: authority_denial.executed_action,
        trusted_provider_output: authority_denial.trusted_provider_output,
        promoted_provider_output: authority_denial.promoted_provider_output,
        accepted_workflow_completion: authority_denial.accepted_workflow_completion,
        accepted_sandbox_success: authority_denial.accepted_sandbox_success,
        accepted_ui_transport_authority: authority_denial.accepted_ui_transport_authority,
        approved_readiness: authority_denial.approved_readiness,
        approved_production_candidate: authority_denial.approved_production_candidate,
        approved_release_candidate: authority_denial.approved_release_candidate,
        approved_public_use: authority_denial.approved_public_use,
        approved_production_human_use: authority_denial.approved_production_human_use,
        reasons,
    }
}

fn inspect_one_phase_111_recovery_record(index: usize, bytes: &[u8]) -> RecoveryEvidenceInspection {
    let mut reasons = BTreeSet::new();
    let parsed = parse_phase_111_append_record(bytes);
    let mut record_type = None;
    let mut decision_evidence_id = None;
    let mut checksum = None;

    match parsed {
        Ok(fields) => {
            record_type = fields.get("record_type").cloned();
            decision_evidence_id = fields.get("decision_evidence_id").cloned();
            checksum = fields.get("deterministic_integrity_marker").cloned();
            classify_phase_111_recovery_fields(&fields, bytes, &mut reasons);
        }
        Err(reason) => {
            reasons.insert(reason);
        }
    }

    let rejected = !reasons.is_empty();
    if rejected {
        reasons.insert(RecoveryLifecycleReason::RecoveryManualReviewRequired);
        reasons.insert(RecoveryLifecycleReason::RecoveryRejected);
    } else {
        reasons.insert(RecoveryLifecycleReason::RecoveryEvidencePresent);
        reasons.insert(RecoveryLifecycleReason::RecoveryClassificationOnly);
    }

    let reasons: Vec<_> = reasons.into_iter().collect();
    RecoveryEvidenceInspection {
        record_index: index,
        record_type,
        decision_evidence_id,
        checksum,
        manual_review_required: reasons
            .contains(&RecoveryLifecycleReason::RecoveryManualReviewRequired),
        rejected: reasons.contains(&RecoveryLifecycleReason::RecoveryRejected),
        reasons,
    }
}

fn parse_phase_111_append_record(
    bytes: &[u8],
) -> Result<BTreeMap<String, String>, RecoveryLifecycleReason> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| RecoveryLifecycleReason::RecoveryEvidenceMalformed)?;
    let mut fields = BTreeMap::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or(RecoveryLifecycleReason::RecoveryEvidenceMalformed)?;
        if key.is_empty() || fields.insert(key.to_string(), value.to_string()).is_some() {
            return Err(RecoveryLifecycleReason::RecoveryEvidenceMalformed);
        }
    }
    if fields.is_empty() {
        return Err(RecoveryLifecycleReason::RecoveryEvidenceMalformed);
    }
    Ok(fields)
}

fn classify_phase_111_recovery_fields(
    fields: &BTreeMap<String, String>,
    bytes: &[u8],
    reasons: &mut BTreeSet<RecoveryLifecycleReason>,
) {
    let required = [
        "record_type",
        "decision_evidence_id",
        "decision_status",
        "reason_codes",
        "proposed_boundary_classification",
        "phase_110_constraints",
        "negative_authority",
        "validation_result",
        "deterministic_integrity_marker",
    ];
    for key in required {
        if !fields.contains_key(key) {
            reasons.insert(RecoveryLifecycleReason::RecoveryEvidenceMalformed);
        }
    }

    if fields.get("record_type").map(String::as_str) != Some(PHASE_111_RECORD_TYPE) {
        reasons.insert(RecoveryLifecycleReason::RecoveryUnsupportedRecordType);
    }
    if fields
        .get("decision_evidence_id")
        .is_none_or(|v| v.trim().is_empty())
    {
        reasons.insert(RecoveryLifecycleReason::RecoveryEvidenceMalformed);
    }
    if fields.get("decision_status").map(String::as_str)
        != Some("Phase110NarrowActivationCandidatePermitted")
    {
        reasons.insert(RecoveryLifecycleReason::RecoveryConflictingEvidence);
    }
    if fields
        .get("proposed_boundary_classification")
        .map(String::as_str)
        != Some("Phase110NarrowRustValidatedAppendCandidate")
    {
        reasons.insert(RecoveryLifecycleReason::RecoveryConflictingEvidence);
    }
    if fields.get("validation_result").map(String::as_str)
        != Some("rust_validated_phase_111_decision_evidence_append")
    {
        reasons.insert(RecoveryLifecycleReason::RecoveryEvidenceMalformed);
    }

    let marker = fields
        .get("deterministic_integrity_marker")
        .map(String::as_str)
        .unwrap_or_default();
    let marker_shape_invalid = marker.len() != 16
        || marker
            .chars()
            .any(|c| !c.is_ascii_hexdigit() || c.is_ascii_uppercase());
    if marker_shape_invalid || calculate_phase_111_recovery_marker(bytes) != marker {
        reasons.insert(RecoveryLifecycleReason::RecoveryChecksumMismatch);
    }

    if fields
        .get("phase_110_constraints")
        .is_none_or(|v| !phase_110_constraints_are_current(v))
    {
        reasons.insert(RecoveryLifecycleReason::RecoveryStaleConstraintSnapshot);
    }
    if fields
        .get("negative_authority")
        .is_none_or(|v| !negative_authority_snapshot_is_complete(v))
    {
        reasons.insert(RecoveryLifecycleReason::RecoveryMissingNegativeAuthorityEvidence);
    }

    for key in [
        "provider_output_authority",
        "workflow_completion_authority",
        "sandbox_success_authority",
        "ui_authorized_persistence",
        "transport_authorized_persistence",
        "replay_repair_authority",
        "recovery_promotion_authority",
        "action_execution_authority",
        "readiness_approval",
        "production_candidate_approval",
        "release_candidate_approval",
        "public_use_approval",
        "production_human_use_approval",
        "provider_output_trusted",
        "provider_output_promoted",
    ] {
        match fields.get(key).map(String::as_str) {
            Some("false") => {}
            Some("true") => {
                reasons.insert(RecoveryLifecycleReason::RecoveryConflictingEvidence);
            }
            _ => {
                reasons.insert(RecoveryLifecycleReason::RecoveryEvidenceMalformed);
            }
        }
    }
}

fn calculate_phase_111_recovery_marker(bytes: &[u8]) -> String {
    let text = std::str::from_utf8(bytes).unwrap_or_default();
    let without_marker: String = text
        .lines()
        .filter(|line| !line.starts_with("deterministic_integrity_marker="))
        .map(|line| format!("{line}\n"))
        .collect();
    calculate_persisted_record_checksum(without_marker.as_bytes())
}

fn phase_110_constraints_are_current(value: &str) -> bool {
    [
        "phase_110_only:true",
        "rust_boundary_required:true",
        "committed_evidence_required:true",
        "descriptive_provider_evidence_only:true",
        "provider_output_authority_prohibited:true",
        "workflow_completion_authority_prohibited:true",
        "replay_repair_prohibited:true",
        "recovery_promotion_prohibited:true",
        "action_execution_prohibited:true",
        "ui_authority_prohibited:true",
        "transport_authority_prohibited:true",
        "trust_promotion_prohibited:true",
        "readiness_promotion_prohibited:true",
    ]
    .iter()
    .all(|required| value.split(';').any(|part| part == *required))
}

fn negative_authority_snapshot_is_complete(value: &str) -> bool {
    [
        "descriptive_only:true",
        "grants_provider_trust:false",
        "grants_readiness:false",
        "grants_workflow_completion_authority:false",
        "grants_provider_output_authority:false",
        "activates_persistence:false",
        "durable_append_enabled:false",
        "replay_repair_enabled:false",
        "recovery_promotion_enabled:false",
        "action_execution_enabled:false",
        "ui_authority_enabled:false",
        "transport_authority_enabled:false",
        "no_promotion:true",
        "no_replay_repair:true",
        "no_recovery_promotion:true",
        "no_action_execution:true",
    ]
    .iter()
    .all(|required| value.split(';').any(|part| part == *required))
}

pub fn build_phase_111_decision_evidence_append_record(
    evidence: &DurablePersistenceAuthorityDecisionEvidence,
) -> Result<Phase111DecisionEvidenceAppendRecord, Phase111DecisionEvidenceAppendRejection> {
    validate_phase_111_decision_evidence_for_append(evidence)?;
    let mut record = Phase111DecisionEvidenceAppendRecord {
        record_type: PHASE_111_RECORD_TYPE,
        decision_evidence_id: evidence.decision_id.clone(),
        decision_status: evidence.status,
        reason_codes: evidence.reason_code_strings.clone(),
        proposed_boundary_classification: evidence.proposed_boundary.scope,
        phase_110_constraint_snapshot: evidence.constraint_set.clone(),
        negative_authority_snapshot: evidence.negative_authority.clone(),
        referenced_sandbox_evidence: evidence.proposed_boundary.sandbox_evidence_refs.clone(),
        referenced_workflow_evidence: evidence.proposed_boundary.workflow_evidence_refs.clone(),
        validation_result: "rust_validated_phase_111_decision_evidence_append",
        provider_output_authority: false,
        workflow_completion_authority: false,
        sandbox_success_authority: false,
        ui_authorized_persistence: false,
        transport_authorized_persistence: false,
        replay_repair_authority: false,
        recovery_promotion_authority: false,
        action_execution_authority: false,
        readiness_approval: false,
        production_candidate_approval: false,
        release_candidate_approval: false,
        public_use_approval: false,
        production_human_use_approval: false,
        provider_output_trusted: false,
        provider_output_promoted: false,
        deterministic_integrity_marker: String::new(),
    };
    let marker = calculate_persisted_record_checksum(
        &encode_phase_111_append_record_without_marker(&record),
    );
    record.deterministic_integrity_marker = marker;
    Ok(record)
}

pub fn encode_phase_111_decision_evidence_append_record(
    record: &Phase111DecisionEvidenceAppendRecord,
) -> Vec<u8> {
    let mut bytes = encode_phase_111_append_record_without_marker(record);
    bytes.extend_from_slice(
        format!(
            "deterministic_integrity_marker={}\n",
            record.deterministic_integrity_marker
        )
        .as_bytes(),
    );
    bytes
}

pub fn append_phase_111_decision_evidence(
    evidence: &DurablePersistenceAuthorityDecisionEvidence,
    plan: &LocalPersistencePlan,
) -> Phase111DecisionEvidenceAppendReport {
    let record = match build_phase_111_decision_evidence_append_record(evidence) {
        Ok(record) => record,
        Err(rejection) => {
            return rejected_phase_111_append_report(
                evidence,
                rejection,
                "validation failed before append",
            )
        }
    };

    let validation = super::persistence::validate_local_persistence_plan(plan);
    if !validation.valid {
        return rejected_phase_111_append_report(
            evidence,
            Phase111DecisionEvidenceAppendRejection::InvalidPersistencePlan,
            "invalid persistence plan rejected before append",
        );
    }

    let encoded = encode_phase_111_decision_evidence_append_record(&record);
    let checksum = calculate_persisted_record_checksum(&encoded);
    match write_phase_111_decision_evidence_append_bytes(plan, &encoded) {
        Ok(()) => Phase111DecisionEvidenceAppendReport {
            status: Phase111DecisionEvidenceAppendStatus::Appended,
            rejection: Phase111DecisionEvidenceAppendRejection::None,
            accepted_record_type: PHASE_111_RECORD_TYPE,
            decision_evidence_id: evidence.decision_id.clone(),
            record_checksum: Some(checksum),
            bytes_written: encoded.len(),
            committed: true,
            provider_output_trusted: false,
            provider_output_promoted: false,
            workflow_completion_authority: false,
            sandbox_success_authority: false,
            ui_transport_authority: false,
            replay_repair_authority: false,
            recovery_promotion_authority: false,
            action_execution_authority: false,
            readiness_approved: false,
            production_candidate_approved: false,
            release_candidate_approved: false,
            public_use_approved: false,
            production_human_use_approved: false,
            summary: "Phase 111 decision-evidence record appended after Rust validation only"
                .to_string(),
        },
        Err(_) => rejected_phase_111_append_report(
            evidence,
            Phase111DecisionEvidenceAppendRejection::AppendWriteFailed,
            "append failed without authority mutation",
        ),
    }
}

pub fn validate_phase_111_decision_evidence_for_append(
    evidence: &DurablePersistenceAuthorityDecisionEvidence,
) -> Result<(), Phase111DecisionEvidenceAppendRejection> {
    if evidence.decision_id.trim().is_empty()
        || evidence.decision_id.matches(":").count() != 1
        || evidence.status
            != PersistenceAuthorityDecisionStatus::Phase110NarrowActivationCandidatePermitted
        || evidence.proposed_boundary.scope
            != ProposedPersistenceScopeClassification::Phase110NarrowRustValidatedAppendCandidate
        || evidence.permitted_phase_110_candidate_categories
            != [AllowedPersistenceCandidateCategory::Phase110RustValidatedDecisionEvidenceAppend]
        || evidence.proposed_boundary.allowed_candidate_categories
            != [AllowedPersistenceCandidateCategory::Phase110RustValidatedDecisionEvidenceAppend]
        || !evidence.descriptive_only
        || !evidence.non_authoritative
        || !evidence.non_self_activating
    {
        return Err(Phase111DecisionEvidenceAppendRejection::InvalidDecisionEvidence);
    }
    if evidence.reason_code_strings.len() != evidence.reason_codes.len()
        || evidence.reason_codes.is_empty()
        || has_duplicate_reason_codes(&evidence.reason_codes)
    {
        return Err(Phase111DecisionEvidenceAppendRejection::DuplicateOrAmbiguousDecisionEvidence);
    }
    if evidence.proposed_boundary.provider_output_present
        || evidence
            .prohibited_categories
            .contains(&ProhibitedPersistenceCategory::ProviderOutputAuthority)
    {
        return Err(Phase111DecisionEvidenceAppendRejection::ProhibitedProviderOutputAuthority);
    }
    if evidence.proposed_boundary.workflow_completed
        || evidence
            .prohibited_categories
            .contains(&ProhibitedPersistenceCategory::WorkflowCompletionAuthority)
    {
        return Err(Phase111DecisionEvidenceAppendRejection::ProhibitedWorkflowCompletionAuthority);
    }
    if evidence.proposed_boundary.provider_execution_succeeded
        || evidence
            .prohibited_categories
            .contains(&ProhibitedPersistenceCategory::SandboxSuccessAuthority)
    {
        return Err(Phase111DecisionEvidenceAppendRejection::ProhibitedSandboxSuccessAuthority);
    }
    if evidence.proposed_boundary.requested_by_ui
        || evidence
            .prohibited_categories
            .contains(&ProhibitedPersistenceCategory::UiAuthorizedPersistence)
    {
        return Err(Phase111DecisionEvidenceAppendRejection::ProhibitedUiAuthorizedPersistence);
    }
    if evidence.proposed_boundary.requested_by_transport
        || evidence
            .prohibited_categories
            .contains(&ProhibitedPersistenceCategory::TransportAuthorizedPersistence)
    {
        return Err(
            Phase111DecisionEvidenceAppendRejection::ProhibitedTransportAuthorizedPersistence,
        );
    }
    if evidence
        .prohibited_categories
        .contains(&ProhibitedPersistenceCategory::ReplayRepairAuthority)
    {
        return Err(Phase111DecisionEvidenceAppendRejection::ProhibitedReplayRepairAuthority);
    }
    if evidence
        .prohibited_categories
        .contains(&ProhibitedPersistenceCategory::RecoveryPromotionAuthority)
    {
        return Err(Phase111DecisionEvidenceAppendRejection::ProhibitedRecoveryPromotionAuthority);
    }
    if evidence
        .prohibited_categories
        .contains(&ProhibitedPersistenceCategory::ActionExecutionAuthority)
    {
        return Err(Phase111DecisionEvidenceAppendRejection::ProhibitedActionExecutionAuthority);
    }
    if durable_persistence_decision_activates_authority(evidence)
        || evidence
            .prohibited_categories
            .contains(&ProhibitedPersistenceCategory::ImplicitTrustPromotion)
        || evidence
            .prohibited_categories
            .contains(&ProhibitedPersistenceCategory::ImplicitReadinessPromotion)
    {
        return Err(Phase111DecisionEvidenceAppendRejection::ProhibitedTrustOrReadinessApproval);
    }
    Ok(())
}

fn encode_phase_111_append_record_without_marker(
    record: &Phase111DecisionEvidenceAppendRecord,
) -> Vec<u8> {
    format!(
        "record_type={}\ndecision_evidence_id={}\ndecision_status={:?}\nreason_codes={}\nproposed_boundary_classification={:?}\nphase_110_constraints={}\nnegative_authority={}\nreferenced_sandbox_evidence={}\nreferenced_workflow_evidence={}\nvalidation_result={}\nprovider_output_authority={}\nworkflow_completion_authority={}\nsandbox_success_authority={}\nui_authorized_persistence={}\ntransport_authorized_persistence={}\nreplay_repair_authority={}\nrecovery_promotion_authority={}\naction_execution_authority={}\nreadiness_approval={}\nproduction_candidate_approval={}\nrelease_candidate_approval={}\npublic_use_approval={}\nproduction_human_use_approval={}\nprovider_output_trusted={}\nprovider_output_promoted={}\n",
        record.record_type,
        record.decision_evidence_id,
        record.decision_status,
        record.reason_codes.join(","),
        record.proposed_boundary_classification,
        encode_phase_110_constraints(&record.phase_110_constraint_snapshot),
        encode_negative_authority(&record.negative_authority_snapshot),
        record.referenced_sandbox_evidence.join(","),
        record.referenced_workflow_evidence.join(","),
        record.validation_result,
        record.provider_output_authority,
        record.workflow_completion_authority,
        record.sandbox_success_authority,
        record.ui_authorized_persistence,
        record.transport_authorized_persistence,
        record.replay_repair_authority,
        record.recovery_promotion_authority,
        record.action_execution_authority,
        record.readiness_approval,
        record.production_candidate_approval,
        record.release_candidate_approval,
        record.public_use_approval,
        record.production_human_use_approval,
        record.provider_output_trusted,
        record.provider_output_promoted,
    )
    .into_bytes()
}

fn encode_phase_110_constraints(constraints: &PersistenceAuthorityConstraintSet) -> String {
    format!(
        "phase_110_only:{};rust_boundary_required:{};committed_evidence_required:{};descriptive_provider_evidence_only:{};provider_output_authority_prohibited:{};workflow_completion_authority_prohibited:{};replay_repair_prohibited:{};recovery_promotion_prohibited:{};action_execution_prohibited:{};ui_authority_prohibited:{};transport_authority_prohibited:{};trust_promotion_prohibited:{};readiness_promotion_prohibited:{}",
        constraints.phase_110_only,
        constraints.rust_boundary_required,
        constraints.committed_evidence_required,
        constraints.descriptive_provider_evidence_only,
        constraints.provider_output_authority_prohibited,
        constraints.workflow_completion_authority_prohibited,
        constraints.replay_repair_prohibited,
        constraints.recovery_promotion_prohibited,
        constraints.action_execution_prohibited,
        constraints.ui_authority_prohibited,
        constraints.transport_authority_prohibited,
        constraints.trust_promotion_prohibited,
        constraints.readiness_promotion_prohibited,
    )
}

fn encode_negative_authority(negative: &NegativeAuthorityEvidence) -> String {
    format!(
        "descriptive_only:{};grants_provider_trust:{};grants_readiness:{};grants_workflow_completion_authority:{};grants_provider_output_authority:{};activates_persistence:{};durable_append_enabled:{};replay_repair_enabled:{};recovery_promotion_enabled:{};action_execution_enabled:{};ui_authority_enabled:{};transport_authority_enabled:{};no_promotion:{};no_replay_repair:{};no_recovery_promotion:{};no_action_execution:{}",
        negative.descriptive_only,
        negative.grants_provider_trust,
        negative.grants_readiness,
        negative.grants_workflow_completion_authority,
        negative.grants_provider_output_authority,
        negative.activates_persistence,
        negative.durable_append_enabled,
        negative.replay_repair_enabled,
        negative.recovery_promotion_enabled,
        negative.action_execution_enabled,
        negative.ui_authority_enabled,
        negative.transport_authority_enabled,
        negative.no_promotion,
        negative.no_replay_repair,
        negative.no_recovery_promotion,
        negative.no_action_execution,
    )
}

fn has_duplicate_reason_codes(codes: &[PersistenceAuthorityDecisionReasonCode]) -> bool {
    let mut seen = BTreeSet::new();
    codes.iter().any(|code| !seen.insert(*code))
}

fn rejected_phase_111_append_report(
    evidence: &DurablePersistenceAuthorityDecisionEvidence,
    rejection: Phase111DecisionEvidenceAppendRejection,
    summary: &str,
) -> Phase111DecisionEvidenceAppendReport {
    Phase111DecisionEvidenceAppendReport {
        status: Phase111DecisionEvidenceAppendStatus::Rejected,
        rejection,
        accepted_record_type: PHASE_111_RECORD_TYPE,
        decision_evidence_id: evidence.decision_id.clone(),
        record_checksum: None,
        bytes_written: 0,
        committed: false,
        provider_output_trusted: false,
        provider_output_promoted: false,
        workflow_completion_authority: false,
        sandbox_success_authority: false,
        ui_transport_authority: false,
        replay_repair_authority: false,
        recovery_promotion_authority: false,
        action_execution_authority: false,
        readiness_approved: false,
        production_candidate_approved: false,
        release_candidate_approved: false,
        public_use_approved: false,
        production_human_use_approved: false,
        summary: summary.to_string(),
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
        ProhibitedPersistenceCategory::SandboxSuccessAuthority => {
            PersistenceAuthorityDecisionReasonCode::SandboxSuccessNotAuthority
        }
        ProhibitedPersistenceCategory::ImplicitTrustPromotion => {
            PersistenceAuthorityDecisionReasonCode::TrustPromotionProhibited
        }
        ProhibitedPersistenceCategory::ImplicitReadinessPromotion => {
            PersistenceAuthorityDecisionReasonCode::ReadinessPromotionProhibited
        }
    }
}
