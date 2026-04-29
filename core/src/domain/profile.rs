//! Rust-owned domain compatibility profile for Phase 13.
//! Domain profiles configure compatibility expectations only.
//! They do not define lifecycle, promotion, ledger, audit, replay, or adapter authority.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainProfile {
    pub id: String,
    pub name: String,
    pub objective_types: Vec<String>,
    pub constraint_types: Vec<String>,
    pub required_evaluators: Vec<String>,
    pub optional_evaluators: Vec<String>,
    pub known_failure_modes: Vec<String>,
    pub promotion_thresholds: Vec<String>,
    pub evidence_requirements: Vec<String>,
}
