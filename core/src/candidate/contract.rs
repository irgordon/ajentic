//! Candidate contract shapes.
//! These definitions reserve the boundary contract only.

use super::lifecycle::CandidateLifecycleState;

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
