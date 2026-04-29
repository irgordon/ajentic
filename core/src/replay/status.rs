//! Replay classification status types.
//!
//! These enums are descriptive-only replay outputs derived from ledger facts.
//! They do not grant authority or alter lifecycle, governance, or promotion behavior.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayReadinessStatus {
    Ready,
    MissingCandidate,
    MissingEvaluation,
    MissingGovernance,
    MissingPromotion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayCompletionStatus {
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayFinalStatus {
    Unknown,
    Failed,
    Blocked,
    PromotedTier1,
    Denied,
}
