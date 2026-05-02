#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    Pass,
    Fail,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub status: ValidationStatus,
    pub message: &'static str,
}

impl ValidationResult {
    pub fn unknown() -> Self {
        Self {
            status: ValidationStatus::Unknown,
            message: "unknown_is_not_pass",
        }
    }
}
