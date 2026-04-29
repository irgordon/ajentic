//! Replay module.
//!
//! Replay reconstructs candidate status and derived classifications from ledger facts.
//! Replay is read-only and must not call adapters, providers, scripts, UI code,
//! or regenerate candidate output.

pub mod engine;
pub mod integrity;
pub mod status;

pub use engine::{replay_candidate, replay_status_from_ledger, ReplayResult};
pub use integrity::{
    ReplayCompletenessStatus, ReplayIdempotenceStatus, ReplayIntegrityStatus,
    ReplayOrderingStabilityStatus,
};
pub use status::{ReplayCompletionStatus, ReplayFinalStatus, ReplayReadinessStatus};
