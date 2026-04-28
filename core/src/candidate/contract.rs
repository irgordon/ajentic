//! Phase 1 contract shapes for the Candidate Solution type.
//!
//! This file defines the contract shape only. Validation behavior will be
//! added in later phases. No parsing, serialization, or lifecycle logic
//! is implemented here.

/// Lifecycle state of a candidate solution.
///
/// These are shape definitions only. State transition rules are not
/// implemented in Phase 1.
pub enum CandidateLifecycleState {
    Created,
    Evaluating,
    Failed,
    Blocked,
    Passed,
    PromotedTier1,
    Rejected,
    Unknown,
}

/// Contract shape for a candidate solution in a governed run.
///
/// Field names align with `schemas/candidate_solution.schema.json`.
pub struct CandidateSolutionContract {
    /// Unique identifier for this candidate solution.
    pub id: String,
    /// Identifier of the run that generated this candidate.
    pub run_id: String,
    /// Domain this candidate was generated for.
    pub domain_id: String,
    /// Objective this candidate was generated to satisfy.
    pub objective_id: String,
    /// Constraints document applied to this candidate.
    pub constraints_id: String,
    /// Reference to the candidate content artifact.
    pub content_ref: String,
    /// Reference to generation metadata (adapter, model, prompt).
    pub generation_metadata_ref: String,
    /// Current lifecycle state of this candidate.
    pub lifecycle_state: CandidateLifecycleState,
    /// References to evaluation result records for this candidate.
    pub evaluation_results_ref: Vec<String>,
    /// References to governance result records for this candidate.
    pub governance_results_ref: Vec<String>,
}
