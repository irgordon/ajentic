//! Replay integrity verification classifications.
//!
//! These statuses are deterministic, closed-set, descriptive outputs only.
//! They must not be used as lifecycle, governance, promotion, replay,
//! ledger, or persistence authority.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayOrderingStabilityStatus {
    Stable,
    OrderDependent,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayIdempotenceStatus {
    Idempotent,
    NonIdempotent,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayCompletenessStatus {
    Complete,
    Incomplete,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayIntegrityStatus {
    Verified,
    Violation,
    Unknown,
}
