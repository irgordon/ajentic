//! Phase 1 audit contract shapes.
//! These definitions reserve the boundary contract only.

use crate::evaluation::contract::ContractStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReuseType {
    Strategy,
    Structure,
    EvaluatorOrder,
    PromptPattern,
    RepairPattern,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReuseEventContract {
    pub id: String,
    pub source_run_id: String,
    pub source_pattern_ref: String,
    pub target_run_id: String,
    pub target_domain_id: String,
    pub reuse_type: ReuseType,
    pub validation_status: ContractStatus,
    pub mismatch_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditRecordContract {
    pub id: String,
    pub run_id: String,
    pub objective_id: String,
    pub constraints_id: String,
    pub domain_id: String,
    pub candidate_solution_ids: Vec<String>,
    pub evaluation_result_ids: Vec<String>,
    pub governance_result_ids: Vec<String>,
    pub promotion_decision_ids: Vec<String>,
    pub final_status: ContractStatus,
}
