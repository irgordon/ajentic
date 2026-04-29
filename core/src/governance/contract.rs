//! Phase 1 governance contract shapes.
//! These definitions reserve the boundary contract only.
//! Runtime governance and promotion behavior lives under governance runtime modules.

use crate::evaluation::contract::ContractStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromotionStatus {
    Approved,
    Denied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceResultContract {
    pub id: String,
    pub candidate_solution_id: String,
    pub status: ContractStatus,
    pub policy_checks: Vec<String>,
    pub blocked_reasons: Vec<String>,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromotionDecisionContract {
    pub id: String,
    pub candidate_solution_id: String,
    pub from_state: String,
    pub to_state: String,
    pub promotion_status: PromotionStatus,
    pub required_checks_passed: bool,
    pub evidence_refs: Vec<String>,
    pub decision_timestamp: String,
}
