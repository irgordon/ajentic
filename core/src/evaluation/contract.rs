//! Phase 1 evaluation contract shapes.
//! These definitions reserve the boundary contract only.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractStatus {
    Pass,
    Fail,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationResultContract {
    pub id: String,
    pub candidate_solution_id: String,
    pub evaluator_id: String,
    pub status: ContractStatus,
    pub checked_constraints: Vec<String>,
    pub threshold_results: Vec<String>,
    pub evidence_ref: String,
    pub failure_reasons: Vec<String>,
}
