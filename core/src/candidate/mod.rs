//! Candidate solution module.
//!
//! This phase defines the candidate lifecycle transition surface.

pub mod contract;
pub mod lifecycle;
pub mod validate;

pub use lifecycle::{transition, CandidateLifecycleState};
pub use validate::can_transition;
