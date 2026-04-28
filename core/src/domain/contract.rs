//! Phase 1 contract shape for the Domain type.
//!
//! This file defines the contract shape only. Validation behavior will be
//! added in later phases. No parsing, serialization, or lifecycle logic
//! is implemented here.

/// Contract shape for a domain definition.
///
/// Field names align with `schemas/domain.schema.json`.
pub struct DomainContract {
    /// Unique identifier for this domain.
    pub id: String,
    /// Human-readable name for this domain.
    pub name: String,
    /// Objective type identifiers supported in this domain.
    pub objective_types: Vec<String>,
    /// Constraint category identifiers applicable in this domain.
    pub constraint_types: Vec<String>,
    /// Evaluator identifiers configured for this domain.
    pub evaluators: Vec<String>,
    /// Known failure mode identifiers for this domain.
    pub known_failure_modes: Vec<String>,
    /// Promotion threshold identifiers configured for this domain.
    pub promotion_thresholds: Vec<String>,
}
