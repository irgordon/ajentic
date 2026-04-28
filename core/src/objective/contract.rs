//! Phase 1 objective contract shapes.
//! These definitions reserve the boundary contract only.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectiveContract {
    pub id: String,
    pub domain_id: String,
    pub objective_type: String,
    pub description: String,
    pub success_criteria: Vec<String>,
    pub constraints_ref: String,
    pub evaluation_profile_ref: String,
}
