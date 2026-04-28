//! Candidate contract shapes.
//! These definitions reserve the boundary contract only.

use super::lifecycle::CandidateLifecycleState;
use crate::execution::adapter_protocol::AdapterStatus;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateRecord {
    pub id: String,
    pub run_id: String,
    pub domain_id: String,
    pub objective_id: String,
    pub constraints_id: String,
    pub content_ref: String,
    pub generation_metadata_ref: String,
    pub adapter_name: String,
    pub adapter_version: String,
    pub adapter_status: AdapterStatus,
    pub raw_output_ref: String,
    pub structured_output_ref: String,
    pub output_text: String,
    pub lifecycle_state: CandidateLifecycleState,
}
