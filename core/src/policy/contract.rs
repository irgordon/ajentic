//! Phase 1 contract shape for the Policy type.
//!
//! This file defines the contract shape only. Validation behavior will be
//! added in later phases. No parsing, serialization, or lifecycle logic
//! is implemented here.

/// Contract shape for a policy document.
///
/// Field names align with `schemas/policy.schema.json`.
pub struct PolicyContract {
    /// Unique identifier for this policy document.
    pub id: String,
    /// Adapter identifiers permitted under this policy.
    pub allowed_adapters: Vec<String>,
    /// Evaluator identifiers that must run before promotion is eligible.
    pub required_evaluators: Vec<String>,
    /// Resource limit identifiers enforced by this policy.
    pub resource_limits: Vec<String>,
    /// Promotion threshold identifiers that must be satisfied.
    pub promotion_thresholds: Vec<String>,
    /// Bypass flags that are prohibited under this policy.
    pub prohibited_bypass_flags: Vec<String>,
}
