mod common;

use ajentic_core::state::{
    authorize_promotion, HarnessState, LifecycleError, LifecycleState, PromotionAuthorizationError,
};

#[test]
fn generic_passed_state_cannot_promote() {
    assert_eq!(
        LifecycleState::Passed.transition_to(LifecycleState::PromotedTier1),
        Err(LifecycleError::PromotionAuthorizationRequired)
    );
}

#[test]
fn governance_authorization_promotes_matching_state() {
    let bundle = common::receipt_bundle("run-1", "candidate-a");
    let authorization = authorize_promotion(
        bundle.binding.clone(),
        &bundle.validation,
        &bundle.policy,
        &bundle.replay,
    )
    .unwrap();
    let state = HarnessState {
        revision: 2,
        lifecycle: LifecycleState::Passed,
    };
    let promoted = state.promote(&bundle.binding, &authorization).unwrap();
    assert_eq!(promoted.lifecycle, LifecycleState::PromotedTier1);
}

#[test]
fn authorization_for_candidate_a_cannot_promote_candidate_b() {
    let first = common::receipt_bundle("run-1", "candidate-a");
    let second = common::receipt_bundle("run-1", "candidate-b");
    let authorization = authorize_promotion(
        first.binding.clone(),
        &first.validation,
        &first.policy,
        &first.replay,
    )
    .unwrap();
    let state = HarnessState {
        revision: 2,
        lifecycle: LifecycleState::Passed,
    };
    assert_eq!(
        state.promote(&second.binding, &authorization),
        Err(PromotionAuthorizationError::BindingMismatch)
    );
}
