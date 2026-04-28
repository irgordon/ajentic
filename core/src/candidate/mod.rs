//! Candidate solution module.
//!
//! Phase 5 creates Rust-owned candidate records from validated adapter responses.

pub mod contract;
pub mod creation;
pub mod lifecycle;
pub mod validate;

pub use creation::{
    create_candidate_from_adapter_response, derive_candidate_id, CandidateCreationInput,
};
pub use lifecycle::{transition, CandidateLifecycleState};
pub use validate::can_transition;
