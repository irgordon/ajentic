//! Phase 1 contract shape for the Audit Record type.
//!
//! This file defines the contract shape only. Validation behavior will be
//! added in later phases. No parsing, serialization, or lifecycle logic
//! is implemented here.

use crate::evaluation::contract::ContractStatus;

/// Contract shape for an audit record.
///
/// Field names align with `schemas/audit_record.schema.json`.
pub struct AuditRecordContract {
    /// Unique identifier for this audit record.
    pub id: String,
    /// Run identifier this audit record belongs to.
    pub run_id: String,
    /// Objective identifier for this run.
    pub objective_id: String,
    /// Constraints document identifier for this run.
    pub constraints_id: String,
    /// Domain identifier for this run.
    pub domain_id: String,
    /// Candidate solution identifiers included in this audit record.
    pub candidate_solution_ids: Vec<String>,
    /// Evaluation result identifiers included in this audit record.
    pub evaluation_result_ids: Vec<String>,
    /// Governance result identifiers included in this audit record.
    pub governance_result_ids: Vec<String>,
    /// Promotion decision identifiers included in this audit record.
    pub promotion_decision_ids: Vec<String>,
    /// Final run status.
    pub final_status: ContractStatus,
}
