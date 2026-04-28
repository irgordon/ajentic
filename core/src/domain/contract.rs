//! Phase 1 domain contract shapes.
//! These definitions reserve the boundary contract only.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainContract {
    pub id: String,
    pub name: String,
    pub objective_types: Vec<String>,
    pub constraint_types: Vec<String>,
    pub evaluators: Vec<String>,
    pub known_failure_modes: Vec<String>,
    pub promotion_thresholds: Vec<String>,
}
