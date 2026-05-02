#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allowed,
    Denied,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyResult {
    pub decision: PolicyDecision,
    pub reason: &'static str,
}

impl PolicyResult {
    pub fn unknown() -> Self {
        Self {
            decision: PolicyDecision::Unknown,
            reason: "unknown_is_not_pass",
        }
    }
}
