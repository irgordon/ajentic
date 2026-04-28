//! Candidate lifecycle types and transition API.

use crate::errors::CandidateLifecycleError;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CandidateLifecycleState {
    Created,
    Evaluating,
    Failed,
    Blocked,
    Passed,
    PromotedTier1,
    Rejected,
    Unknown,
}

pub fn transition(
    from: CandidateLifecycleState,
    to: CandidateLifecycleState,
) -> Result<CandidateLifecycleState, CandidateLifecycleError> {
    if super::validate::can_transition(from, to) {
        Ok(to)
    } else {
        Err(CandidateLifecycleError::InvalidTransition { from, to })
    }
}

#[cfg(test)]
mod tests {
    use super::{transition, CandidateLifecycleState};
    use crate::candidate::validate::can_transition;
    use crate::errors::CandidateLifecycleError;

    use CandidateLifecycleState::*;

    #[test]
    fn valid_transitions_are_allowed() {
        let valid = [
            (Created, Evaluating),
            (Created, Failed),
            (Created, Blocked),
            (Created, Rejected),
            (Evaluating, Passed),
            (Evaluating, Failed),
            (Evaluating, Blocked),
            (Evaluating, Unknown),
            (Unknown, Evaluating),
            (Unknown, Failed),
            (Unknown, Blocked),
            (Unknown, Rejected),
            (Passed, PromotedTier1),
            (Passed, Rejected),
            (Failed, Rejected),
            (Blocked, Rejected),
        ];

        for (from, to) in valid {
            assert!(can_transition(from, to));
            assert_eq!(transition(from, to), Ok(to));
        }
    }

    #[test]
    fn invalid_transitions_are_rejected() {
        let invalid = [
            (Created, PromotedTier1),
            (Unknown, Passed),
            (Unknown, PromotedTier1),
            (Failed, Passed),
            (Blocked, Passed),
            (Rejected, Created),
            (Rejected, Evaluating),
            (PromotedTier1, Created),
            (PromotedTier1, Rejected),
            (Created, Created),
            (Evaluating, Evaluating),
            (Passed, Passed),
        ];

        for (from, to) in invalid {
            assert!(!can_transition(from, to));
            assert!(matches!(
                transition(from, to),
                Err(CandidateLifecycleError::InvalidTransition { .. })
            ));
        }
    }

    #[test]
    fn invalid_transition_error_shape_is_typed() {
        let result = transition(Created, PromotedTier1);
        assert_eq!(
            result,
            Err(CandidateLifecycleError::InvalidTransition {
                from: Created,
                to: PromotedTier1,
            })
        );
    }

    #[test]
    fn terminal_states_remain_terminal() {
        let all_targets = [
            Created,
            Evaluating,
            Failed,
            Blocked,
            Passed,
            PromotedTier1,
            Rejected,
            Unknown,
        ];

        for to in all_targets {
            assert!(!can_transition(PromotedTier1, to));
            assert!(!can_transition(Rejected, to));
            assert!(transition(PromotedTier1, to).is_err());
            assert!(transition(Rejected, to).is_err());
        }
    }

    #[test]
    fn no_invalid_transition_returns_ok() {
        let states = [
            Created,
            Evaluating,
            Failed,
            Blocked,
            Passed,
            PromotedTier1,
            Rejected,
            Unknown,
        ];

        for from in states {
            for to in states {
                if !can_transition(from, to) {
                    assert!(transition(from, to).is_err());
                }
            }
        }
    }
}
