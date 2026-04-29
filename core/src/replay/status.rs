//! Replay classification status types.
//!
//! These enums are descriptive-only replay outputs derived from ledger facts.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayReadinessStatus {
    Ready,
    MissingCandidate,
    MissingEvaluation,
    MissingGovernance,
    MissingPromotion,
    MissingRequiredFacts,
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
