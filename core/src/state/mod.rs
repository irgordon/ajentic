#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Created,
    Evaluating,
    Failed,
    Blocked,
    Passed,
    PromotedTier1,
    Rejected,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleError {
    InvalidTransition,
    TerminalState,
    UnknownCannotPass,
}

impl LifecycleError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidTransition => "invalid_transition",
            Self::TerminalState => "terminal_state",
            Self::UnknownCannotPass => "unknown_cannot_pass",
        }
    }
}

impl LifecycleState {
    pub fn transition_to(self, next: LifecycleState) -> Result<LifecycleState, LifecycleError> {
        if self == next {
            return Ok(self);
        }

        match self {
            Self::PromotedTier1 | Self::Rejected => return Err(LifecycleError::TerminalState),
            Self::Unknown if next == Self::Passed => {
                return Err(LifecycleError::UnknownCannotPass);
            }
            _ => {}
        }

        let allowed = match self {
            Self::Created => matches!(
                next,
                Self::Evaluating | Self::Blocked | Self::Rejected | Self::Unknown
            ),
            Self::Evaluating => matches!(
                next,
                Self::Passed | Self::Failed | Self::Blocked | Self::Rejected | Self::Unknown
            ),
            Self::Failed => matches!(next, Self::Evaluating | Self::Blocked | Self::Rejected),
            Self::Blocked => matches!(next, Self::Evaluating | Self::Rejected | Self::Unknown),
            Self::Passed => matches!(next, Self::PromotedTier1 | Self::Rejected),
            Self::Unknown => matches!(next, Self::Evaluating | Self::Blocked | Self::Rejected),
            Self::PromotedTier1 | Self::Rejected => false,
        };

        if allowed {
            Ok(next)
        } else {
            Err(LifecycleError::InvalidTransition)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessState {
    pub revision: u64,
    pub lifecycle: LifecycleState,
}

impl HarnessState {
    pub fn genesis() -> Self {
        Self {
            revision: 0,
            lifecycle: LifecycleState::Created,
        }
    }

    pub fn transition_to(&self, next: LifecycleState) -> Result<Self, LifecycleError> {
        let lifecycle = self.lifecycle.transition_to(next)?;

        let revision = if lifecycle == self.lifecycle {
            self.revision
        } else {
            self.revision + 1
        };

        Ok(Self {
            revision,
            lifecycle,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{HarnessState, LifecycleError, LifecycleState};

    #[test]
    fn valid_created_to_evaluating_passes() {
        let next = LifecycleState::Created.transition_to(LifecycleState::Evaluating);
        assert_eq!(next, Ok(LifecycleState::Evaluating));
    }

    #[test]
    fn valid_evaluating_to_passed_passes() {
        let next = LifecycleState::Evaluating.transition_to(LifecycleState::Passed);
        assert_eq!(next, Ok(LifecycleState::Passed));
    }

    #[test]
    fn valid_passed_to_promoted_tier_1_passes() {
        let next = LifecycleState::Passed.transition_to(LifecycleState::PromotedTier1);
        assert_eq!(next, Ok(LifecycleState::PromotedTier1));
    }

    #[test]
    fn created_to_promoted_tier_1_fails() {
        let error = LifecycleState::Created
            .transition_to(LifecycleState::PromotedTier1)
            .expect_err("created to promoted tier 1 must fail");
        assert_eq!(error, LifecycleError::InvalidTransition);
    }

    #[test]
    fn unknown_to_passed_fails() {
        let error = LifecycleState::Unknown
            .transition_to(LifecycleState::Passed)
            .expect_err("unknown to passed must fail");
        assert_eq!(error, LifecycleError::UnknownCannotPass);
    }

    #[test]
    fn promoted_tier_1_terminal_blocks_change() {
        let error = LifecycleState::PromotedTier1
            .transition_to(LifecycleState::Evaluating)
            .expect_err("promoted tier 1 must be terminal");
        assert_eq!(error, LifecycleError::TerminalState);
    }

    #[test]
    fn rejected_terminal_blocks_change() {
        let error = LifecycleState::Rejected
            .transition_to(LifecycleState::Evaluating)
            .expect_err("rejected must be terminal");
        assert_eq!(error, LifecycleError::TerminalState);
    }

    #[test]
    fn promoted_tier_1_self_transition_keeps_state() {
        let next = LifecycleState::PromotedTier1
            .transition_to(LifecycleState::PromotedTier1)
            .expect("terminal self transition should be idempotent");

        assert_eq!(next, LifecycleState::PromotedTier1);
    }

    #[test]
    fn rejected_self_transition_keeps_state() {
        let next = LifecycleState::Rejected
            .transition_to(LifecycleState::Rejected)
            .expect("terminal self transition should be idempotent");

        assert_eq!(next, LifecycleState::Rejected);
    }

    #[test]
    fn lifecycle_error_codes_are_stable() {
        assert_eq!(
            LifecycleError::InvalidTransition.code(),
            "invalid_transition"
        );
        assert_eq!(LifecycleError::TerminalState.code(), "terminal_state");
        assert_eq!(
            LifecycleError::UnknownCannotPass.code(),
            "unknown_cannot_pass"
        );
    }

    #[test]
    fn harness_state_successful_transition_increments_revision() {
        let state = HarnessState::genesis();
        let next = state
            .transition_to(LifecycleState::Evaluating)
            .expect("created to evaluating should succeed");

        assert_eq!(next.revision, 1);
        assert_eq!(next.lifecycle, LifecycleState::Evaluating);
    }

    #[test]
    fn harness_state_failed_transition_does_not_return_new_state() {
        let state = HarnessState::genesis();
        let error = state
            .transition_to(LifecycleState::PromotedTier1)
            .expect_err("created to promoted tier 1 must fail");

        assert_eq!(error, LifecycleError::InvalidTransition);
        assert_eq!(state.revision, 0);
        assert_eq!(state.lifecycle, LifecycleState::Created);
    }

    #[test]
    fn harness_state_self_transition_keeps_revision() {
        let state = HarnessState::genesis();
        let next = state
            .transition_to(LifecycleState::Created)
            .expect("self transition should be idempotent");

        assert_eq!(next.revision, 0);
        assert_eq!(next.lifecycle, LifecycleState::Created);
    }
}
