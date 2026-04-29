//! Phase 6 structured evaluation result records.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationStatus {
    Pass,
    Fail,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationResultRecord {
    pub id: String,
    pub candidate_solution_id: String,
    pub evaluator_id: String,
    pub status: EvaluationStatus,
    pub checked_constraints: Vec<String>,
    pub threshold_results: Vec<String>,
    pub evidence_ref: String,
    pub failure_reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationResultSet {
    pub candidate_id: String,
    pub results: Vec<EvaluationResultRecord>,
}

impl EvaluationResultSet {
    pub fn new(candidate_id: String) -> Self {
        Self {
            candidate_id,
            results: Vec::new(),
        }
    }
}
