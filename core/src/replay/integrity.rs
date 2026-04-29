//! Replay integrity verification classifications.
//! These statuses are descriptive outputs only.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayOrderingStabilityStatus {
    Stable,
    OrderDependent,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayIdempotenceStatus {
    Idempotent,
    NonIdempotent,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayCompletenessStatus {
    Complete,
    Incomplete,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayIntegrityStatus {
    Verified,
    Violation,
    Unknown,
}
