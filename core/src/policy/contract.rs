//! Phase 1 policy contract shapes.
//! These definitions reserve the boundary contract only.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyContract {
    pub id: String,
    pub allowed_adapters: Vec<String>,
    pub required_evaluators: Vec<String>,
    pub resource_limits: Vec<String>,
    pub promotion_thresholds: Vec<String>,
    pub prohibited_bypass_flags: Vec<String>,
}
