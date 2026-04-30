//! Candidate lifecycle transition validation.

use super::lifecycle::CandidateLifecycleState;

pub fn can_transition(from: CandidateLifecycleState, to: CandidateLifecycleState) -> bool {
    use CandidateLifecycleState::*;

    matches!(
        (from, to),
        (Created, Evaluating | Failed | Blocked | Rejected)
            | (Evaluating, Passed | Failed | Blocked | Unknown)
            | (Unknown, Evaluating | Failed | Blocked | Rejected)
            | (Passed, PromotedTier1 | Rejected)
            | (Failed, Rejected)
            | (Blocked, Rejected)
    )
}
