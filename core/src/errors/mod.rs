#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AjenticError {
    InvalidState,
    InvalidPolicy,
    InvalidValidation,
    InvalidContext,
    InvalidMemory,
    InvalidExecution,
    InvalidLedger,
    InvalidReplay,
    InvalidAudit,
    InvalidApi,
    UnsupportedCommand,
}

impl AjenticError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidState => "invalid_state",
            Self::InvalidPolicy => "invalid_policy",
            Self::InvalidValidation => "invalid_validation",
            Self::InvalidContext => "invalid_context",
            Self::InvalidMemory => "invalid_memory",
            Self::InvalidExecution => "invalid_execution",
            Self::InvalidLedger => "invalid_ledger",
            Self::InvalidReplay => "invalid_replay",
            Self::InvalidAudit => "invalid_audit",
            Self::InvalidApi => "invalid_api",
            Self::UnsupportedCommand => "unsupported_command",
        }
    }
}
