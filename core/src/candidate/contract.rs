//! Phase 1 candidate contract shapes.
//! These definitions reserve the boundary contract only.

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateSolutionContract {
    pub id: String,
    pub run_id: String,
    pub domain_id: String,
    pub objective_id: String,
    pub constraints_id: String,
    pub content_ref: String,
    pub generation_metadata_ref: String,
    pub lifecycle_state: CandidateLifecycleState,
    pub evaluation_results_ref: Vec<String>,
    pub governance_results_ref: Vec<String>,
}
