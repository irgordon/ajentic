mod common;

use ajentic_core::api::{
    authorize_exact_action, authorize_operator_intent, submit_operator_intent,
    verify_exact_action_approval, ActionReversibility, ActionRiskClass, ExactActionApprovalError,
    ExactActionApprovalRequest, OperatorAuthorizationRequest, OperatorIdentity, OperatorIntent,
    OperatorIntentSubmission, OperatorIntentTargetKind, OperatorIntentType, OperatorSafetyContext,
    OperatorTargetContext,
};
use ajentic_core::integrity::Digest;

fn authorization_decision() -> ajentic_core::api::OperatorAuthorizationDecision {
    let submission = OperatorIntentSubmission::new(
        "submission-1",
        "operator-1",
        OperatorIntent::new(OperatorIntentType::Approve, "approve exact action"),
        OperatorIntentTargetKind::Run,
        "run-1",
    )
    .unwrap();
    authorize_operator_intent(
        OperatorAuthorizationRequest::new(
            "authorization-1",
            submission.clone(),
            submit_operator_intent(submission),
            OperatorIdentity::new("operator-1", "session-1", "human").unwrap(),
            OperatorSafetyContext::new("config-1", "strict", false, false, false).unwrap(),
            OperatorTargetContext::new(OperatorIntentTargetKind::Run, "run-1", "run-1").unwrap(),
        )
        .unwrap(),
    )
}

fn action_request() -> ExactActionApprovalRequest {
    ExactActionApprovalRequest {
        binding: common::receipt_bundle("run-1", "candidate").binding,
        tool: "send_email".into(),
        argument_digest: Digest::of_text("to=A;body=X"),
        target: "mailbox".into(),
        recipient: Some("A".into()),
        disclosed_data_digest: Digest::of_text("body=X"),
        risk_class: ActionRiskClass::ExternalCommunication,
        reversibility: ActionReversibility::Irreversible,
        expected_cost_microunits: 1,
        operator_id: "operator-1".into(),
        expires_after_revision: 2,
        nonce: "nonce-1".into(),
        previous_approval_id: None,
    }
}

#[test]
fn changed_recipient_invalidates_prior_approval() {
    let request = action_request();
    let receipt = authorize_exact_action(&authorization_decision(), request.clone(), 2).unwrap();
    let mut changed = request;
    changed.recipient = Some("B".into());
    assert_eq!(
        verify_exact_action_approval(&receipt, &changed, 2, &std::collections::HashSet::new(),),
        Err(ExactActionApprovalError::RecipientMismatch)
    );
}

#[test]
fn consumed_nonce_cannot_be_reused() {
    let request = action_request();
    let receipt = authorize_exact_action(&authorization_decision(), request.clone(), 2).unwrap();
    let consumed = std::collections::HashSet::from([receipt.nonce().to_string()]);
    assert_eq!(
        verify_exact_action_approval(&receipt, &request, 2, &consumed),
        Err(ExactActionApprovalError::NonceAlreadyConsumed)
    );
}
