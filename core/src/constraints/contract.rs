//! Phase 1 constraints contract shapes.
//! These definitions reserve the boundary contract only.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintsContract {
    pub id: String,
    pub performance: Vec<String>,
    pub safety: Vec<String>,
    pub policy: Vec<String>,
    pub compliance: Vec<String>,
    pub logic: Vec<String>,
    pub resource_limits: Vec<String>,
    pub prohibited_outputs: Vec<String>,
}
