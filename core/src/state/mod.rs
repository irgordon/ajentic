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
    PromotionAuthorizationRequired,
}

impl LifecycleError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidTransition => "invalid_transition",
            Self::TerminalState => "terminal_state",
            Self::UnknownCannotPass => "unknown_cannot_pass",
            Self::PromotionAuthorizationRequired => "promotion_authorization_required",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromotionAuthorization {
    binding: crate::authority::AuthorityBinding,
    validation_receipt_digest: crate::integrity::Digest,
    policy_receipt_digest: crate::integrity::Digest,
    replay_receipt_digest: crate::integrity::Digest,
    authorization_digest: crate::integrity::Digest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionAuthorizationError {
    BindingMismatch,
    ValidationNotPassed,
    PolicyNotAllowed,
    ReplayNotReady,
    LifecycleNotPassed,
    RevisionMismatch,
}

impl PromotionAuthorizationError {
    pub fn code(self) -> &'static str {
        match self {
            Self::BindingMismatch => "binding_mismatch",
            Self::ValidationNotPassed => "validation_not_passed",
            Self::PolicyNotAllowed => "policy_not_allowed",
            Self::ReplayNotReady => "replay_not_ready",
            Self::LifecycleNotPassed => "lifecycle_not_passed",
            Self::RevisionMismatch => "revision_mismatch",
        }
    }
}

impl PromotionAuthorization {
    pub fn binding(&self) -> &crate::authority::AuthorityBinding {
        &self.binding
    }

    pub fn validation_receipt_digest(&self) -> &crate::integrity::Digest {
        &self.validation_receipt_digest
    }

    pub fn policy_receipt_digest(&self) -> &crate::integrity::Digest {
        &self.policy_receipt_digest
    }

    pub fn replay_receipt_digest(&self) -> &crate::integrity::Digest {
        &self.replay_receipt_digest
    }

    pub fn digest(&self) -> &crate::integrity::Digest {
        &self.authorization_digest
    }
}

pub fn authorize_promotion(
    binding: crate::authority::AuthorityBinding,
    validation: &crate::validation::ValidationReceipt,
    policy: &crate::policy::PolicyReceipt,
    replay: &crate::replay::ReplayReceipt,
) -> Result<PromotionAuthorization, PromotionAuthorizationError> {
    validate_promotion_receipts(&binding, validation, policy, replay)?;
    Ok(issue_promotion_authorization(
        binding, validation, policy, replay,
    ))
}

fn validate_promotion_receipts(
    binding: &crate::authority::AuthorityBinding,
    validation: &crate::validation::ValidationReceipt,
    policy: &crate::policy::PolicyReceipt,
    replay: &crate::replay::ReplayReceipt,
) -> Result<(), PromotionAuthorizationError> {
    validate_receipt_bindings(binding, validation, policy, replay)?;
    validate_positive_receipts(validation, policy, replay)
}

fn validate_receipt_bindings(
    binding: &crate::authority::AuthorityBinding,
    validation: &crate::validation::ValidationReceipt,
    policy: &crate::policy::PolicyReceipt,
    replay: &crate::replay::ReplayReceipt,
) -> Result<(), PromotionAuthorizationError> {
    if validation.binding() == binding
        && policy.binding() == binding
        && replay.binding() == binding
        && policy.validation_receipt_digest() == validation.digest()
    {
        return Ok(());
    }
    Err(PromotionAuthorizationError::BindingMismatch)
}

fn validate_positive_receipts(
    validation: &crate::validation::ValidationReceipt,
    policy: &crate::policy::PolicyReceipt,
    replay: &crate::replay::ReplayReceipt,
) -> Result<(), PromotionAuthorizationError> {
    if !validation.passed() {
        return Err(PromotionAuthorizationError::ValidationNotPassed);
    }
    if !policy.allowed() {
        return Err(PromotionAuthorizationError::PolicyNotAllowed);
    }
    validate_replay_receipt(replay)
}

fn validate_replay_receipt(
    replay: &crate::replay::ReplayReceipt,
) -> Result<(), PromotionAuthorizationError> {
    if !replay.ready() {
        return Err(PromotionAuthorizationError::ReplayNotReady);
    }
    if replay.final_state() == LifecycleState::Passed {
        return Ok(());
    }
    Err(PromotionAuthorizationError::LifecycleNotPassed)
}

fn issue_promotion_authorization(
    binding: crate::authority::AuthorityBinding,
    validation: &crate::validation::ValidationReceipt,
    policy: &crate::policy::PolicyReceipt,
    replay: &crate::replay::ReplayReceipt,
) -> PromotionAuthorization {
    let authorization_digest = promotion_authorization_digest(
        &binding,
        validation.digest(),
        policy.digest(),
        replay.digest(),
    );
    PromotionAuthorization {
        binding,
        validation_receipt_digest: validation.digest().clone(),
        policy_receipt_digest: policy.digest().clone(),
        replay_receipt_digest: replay.digest().clone(),
        authorization_digest,
    }
}

fn promotion_authorization_digest(
    binding: &crate::authority::AuthorityBinding,
    validation: &crate::integrity::Digest,
    policy: &crate::integrity::Digest,
    replay: &crate::integrity::Digest,
) -> crate::integrity::Digest {
    crate::integrity::Digest::of_text(&format!(
        "promotion|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        binding.run_id(),
        binding.task_digest().as_str(),
        binding.operator_intent_digest().as_str(),
        binding.context_packet_digest().as_str(),
        binding.candidate_digest().as_str(),
        binding.policy_bundle_digest().as_str(),
        binding.evidence_manifest_digest().as_str(),
        binding.verifier_id(),
        binding.verifier_version(),
        binding.valid_through_revision(),
        validation.as_str(),
        policy.as_str(),
        replay.as_str()
    ))
}

impl LifecycleState {
    pub fn transition_to(self, next: LifecycleState) -> Result<LifecycleState, LifecycleError> {
        if self == next {
            return Ok(self);
        }

        if self == Self::Passed && next == Self::PromotedTier1 {
            return Err(LifecycleError::PromotionAuthorizationRequired);
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
            Self::Passed => matches!(next, Self::Rejected),
            Self::Unknown => matches!(next, Self::Evaluating | Self::Blocked | Self::Rejected),
            Self::PromotedTier1 | Self::Rejected => false,
        };

        if allowed {
            Ok(next)
        } else {
            Err(LifecycleError::InvalidTransition)
        }
    }

    fn replay_transition_to(self, next: LifecycleState) -> Result<LifecycleState, LifecycleError> {
        if self == Self::Passed && next == Self::PromotedTier1 {
            return Ok(next);
        }
        self.transition_to(next)
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

    pub fn promote(
        &self,
        expected_binding: &crate::authority::AuthorityBinding,
        authorization: &PromotionAuthorization,
    ) -> Result<Self, PromotionAuthorizationError> {
        validate_promotion_state(self, expected_binding, authorization)?;
        Ok(Self {
            revision: self.revision + 1,
            lifecycle: LifecycleState::PromotedTier1,
        })
    }

    pub(crate) fn replay_transition_to(
        &self,
        next: LifecycleState,
    ) -> Result<Self, LifecycleError> {
        let lifecycle = self.lifecycle.replay_transition_to(next)?;
        let revision = next_revision(self, lifecycle);
        Ok(Self {
            revision,
            lifecycle,
        })
    }
}

fn validate_promotion_state(
    state: &HarnessState,
    expected_binding: &crate::authority::AuthorityBinding,
    authorization: &PromotionAuthorization,
) -> Result<(), PromotionAuthorizationError> {
    if authorization.binding() != expected_binding {
        return Err(PromotionAuthorizationError::BindingMismatch);
    }
    if state.lifecycle != LifecycleState::Passed {
        return Err(PromotionAuthorizationError::LifecycleNotPassed);
    }
    if state.revision == authorization.binding.valid_through_revision() {
        return Ok(());
    }
    Err(PromotionAuthorizationError::RevisionMismatch)
}

fn next_revision(state: &HarnessState, lifecycle: LifecycleState) -> u64 {
    if lifecycle == state.lifecycle {
        return state.revision;
    }
    state.revision + 1
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
    fn passed_to_promoted_tier_1_requires_authorization() {
        let next = LifecycleState::Passed.transition_to(LifecycleState::PromotedTier1);
        assert_eq!(next, Err(LifecycleError::PromotionAuthorizationRequired));
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
        assert_eq!(
            LifecycleError::PromotionAuthorizationRequired.code(),
            "promotion_authorization_required"
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
