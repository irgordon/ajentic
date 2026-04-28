//! Phase 1 contract shapes for the Evaluation Result type.
//!
//! This file defines the contract shape only. Validation behavior will be
//! added in later phases. No parsing, serialization, or lifecycle logic
//! is implemented here.

/// Status of a contract check.
///
/// UNKNOWN is not PASS. Any result that cannot be positively confirmed
/// as passing is treated as failing. These are shape definitions only.
pub enum ContractStatus {
    Pass,
    Fail,
    Blocked,
    Unknown,
}

/// Contract shape for an evaluation result.
///
/// Field names align with `schemas/evaluation_result.schema.json`.
pub struct EvaluationResultContract {
    /// Unique identifier for this evaluation result.
    pub id: String,
    /// Candidate solution this result evaluates.
    pub candidate_solution_id: String,
    /// Identifier of the evaluator that produced this result.
    pub evaluator_id: String,
    /// Evaluation status.
    pub status: ContractStatus,
    /// Constraint identifiers that were checked during this evaluation.
    pub checked_constraints: Vec<String>,
    /// Threshold check result identifiers from this evaluation.
    pub threshold_results: Vec<String>,
    /// Reference to the evidence artifact for this evaluation result.
    pub evidence_ref: String,
    /// Reasons for failure if status is Fail or Blocked.
    pub failure_reasons: Vec<String>,
}
