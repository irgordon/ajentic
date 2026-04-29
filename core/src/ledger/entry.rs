use crate::candidate::lifecycle::CandidateLifecycleState;
use crate::evaluation::result::EvaluationStatus;
use crate::governance::promotion::PromotionStatus;
use crate::governance::runtime::GovernanceStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerEntryKind {
    CandidateCreated,
    EvaluationRecorded,
    GovernanceReviewed,
    PromotionDecided,
    ReuseApplied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateCreatedLedgerRecord {
    pub candidate_id: String,
    pub run_id: String,
    pub objective_id: String,
    pub constraints_id: String,
    pub domain_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationRecordedLedgerRecord {
    pub evaluation_result_id: String,
    pub candidate_id: String,
    pub evaluator_id: String,
    pub status: EvaluationStatus,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceReviewedLedgerRecord {
    pub governance_result_id: String,
    pub candidate_id: String,
    pub status: GovernanceStatus,
    pub required_evaluators_satisfied: bool,
    pub evidence_refs: Vec<String>,
    pub blocked_reasons: Vec<String>,
    pub failure_reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromotionDecidedLedgerRecord {
    pub promotion_decision_id: String,
    pub candidate_id: String,
    pub promotion_status: PromotionStatus,
    pub from_state: CandidateLifecycleState,
    pub to_state: CandidateLifecycleState,
    pub required_checks_passed: bool,
    pub evidence_refs: Vec<String>,
    pub denial_reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReuseAppliedLedgerRecord {
    pub reuse_event_id: String,
    pub reused_candidate_id: String,
    pub new_run_id: String,
    pub reuse_reason: String,
    pub triggering_actor: String,
    pub timestamp_reference: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerEntry {
    CandidateCreated(CandidateCreatedLedgerRecord),
    EvaluationRecorded(EvaluationRecordedLedgerRecord),
    GovernanceReviewed(GovernanceReviewedLedgerRecord),
    PromotionDecided(PromotionDecidedLedgerRecord),
    ReuseApplied(ReuseAppliedLedgerRecord),
}

impl LedgerEntry {
    pub fn kind(&self) -> LedgerEntryKind {
        match self {
            Self::CandidateCreated(_) => LedgerEntryKind::CandidateCreated,
            Self::EvaluationRecorded(_) => LedgerEntryKind::EvaluationRecorded,
            Self::GovernanceReviewed(_) => LedgerEntryKind::GovernanceReviewed,
            Self::PromotionDecided(_) => LedgerEntryKind::PromotionDecided,
            Self::ReuseApplied(_) => LedgerEntryKind::ReuseApplied,
        }
    }
}
