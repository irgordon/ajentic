//! Phase 7 policy check result model.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyCheckStatus {
    Pass,
    Fail,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyCheckResult {
    pub id: String,
    pub policy_check_id: String,
    pub status: PolicyCheckStatus,
    pub evidence_ref: String,
    pub failure_reasons: Vec<String>,
}
