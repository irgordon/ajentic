//! Phase 7 governance review input surface.
//! This type is a data carrier only.
//! Validation and promotion authority live in governance runtime and promotion modules.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceReviewInput {
    pub candidate_id: String,
    pub objective_id: String,
    pub constraints_id: String,
    pub domain_id: String,
    pub required_evaluator_ids: Vec<String>,
    pub required_policy_check_ids: Vec<String>,
    pub evidence_refs: Vec<String>,
}
