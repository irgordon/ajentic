//! Phase 1 contract shape for the Objective type.
//!
//! This file defines the contract shape only. Validation behavior will be
//! added in later phases. No parsing, serialization, or lifecycle logic
//! is implemented here.

/// The type category of an objective within its domain.
pub type ObjectiveType = String;

/// Contract shape for an objective in a governed run.
///
/// Field names align with `schemas/objective.schema.json`.
pub struct ObjectiveContract {
    /// Unique identifier for this objective.
    pub id: String,
    /// Reference to the domain this objective belongs to.
    pub domain_id: String,
    /// The category or type of this objective within its domain.
    pub objective_type: ObjectiveType,
    /// Human-readable description of what this objective requires.
    pub description: String,
    /// List of criteria that define a successful outcome.
    pub success_criteria: Vec<String>,
    /// Reference to the constraints document that applies to this objective.
    pub constraints_ref: String,
    /// Reference to the evaluation profile used to assess candidates.
    pub evaluation_profile_ref: String,
}
