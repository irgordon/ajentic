//! Phase 1 contract shapes for the Governance Result and Promotion Decision types.
//!
//! This file defines the contract shape only. Validation behavior will be
//! added in later phases. No parsing, serialization, or lifecycle logic
//! is implemented here.

use crate::evaluation::contract::ContractStatus;

/// Category of a reuse event.
///
/// These are shape definitions only. Reuse logic is not implemented
/// in Phase 1.
pub enum ReuseType {
    Strategy,
    Structure,
    EvaluatorOrder,
    PromptPattern,
    RepairPattern,
}

/// Promotion outcome.
///
/// These are shape definitions only. Promotion logic is not implemented
/// in Phase 1.
pub enum PromotionStatus {
    Approved,
    Denied,
}

/// Contract shape for a governance result.
///
/// Field names align with `schemas/governance_result.schema.json`.
pub struct GovernanceResultContract {
    /// Unique identifier for this governance result.
    pub id: String,
    /// Candidate solution this governance result applies to.
    pub candidate_solution_id: String,
    /// Governance status.
    pub status: ContractStatus,
    /// Policy check identifiers that were applied.
    pub policy_checks: Vec<String>,
    /// Reasons the candidate was blocked, if applicable.
    pub blocked_reasons: Vec<String>,
    /// References to evidence artifacts supporting this governance result.
    pub evidence_refs: Vec<String>,
}

/// Contract shape for a promotion decision.
///
/// Field names align with `schemas/promotion_decision.schema.json`.
pub struct PromotionDecisionContract {
    /// Unique identifier for this promotion decision.
    pub id: String,
    /// Candidate solution this decision applies to.
    pub candidate_solution_id: String,
    /// Lifecycle state before this decision.
    pub from_state: String,
    /// Lifecycle state after this decision.
    pub to_state: String,
    /// Promotion outcome.
    pub promotion_status: PromotionStatus,
    /// Whether all required checks passed before this decision.
    pub required_checks_passed: bool,
    /// References to evidence artifacts supporting this decision.
    pub evidence_refs: Vec<String>,
    /// ISO 8601 timestamp of when this decision was recorded.
    pub decision_timestamp: String,
}
