//! Phase 1 contract shape for the Constraints type.
//!
//! This file defines the contract shape only. Validation behavior will be
//! added in later phases. No parsing, serialization, or lifecycle logic
//! is implemented here.

/// Contract shape for a constraints document in a governed run.
///
/// Field names align with `schemas/constraints.schema.json`.
pub struct ConstraintsContract {
    /// Unique identifier for this constraints document.
    pub id: String,
    /// Performance constraints the candidate must satisfy.
    pub performance: Vec<String>,
    /// Safety constraints the candidate must satisfy.
    pub safety: Vec<String>,
    /// Policy constraints the candidate must satisfy.
    pub policy: Vec<String>,
    /// Compliance constraints the candidate must satisfy.
    pub compliance: Vec<String>,
    /// Logic constraints the candidate must satisfy.
    pub logic: Vec<String>,
    /// Resource limits the candidate must not exceed.
    pub resource_limits: Vec<String>,
    /// Output patterns or content categories that are prohibited.
    pub prohibited_outputs: Vec<String>,
}
