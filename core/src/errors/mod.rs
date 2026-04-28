//! Errors module.

use crate::candidate::lifecycle::CandidateLifecycleState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CandidateLifecycleError {
    InvalidTransition {
        from: CandidateLifecycleState,
        to: CandidateLifecycleState,
    },
}
