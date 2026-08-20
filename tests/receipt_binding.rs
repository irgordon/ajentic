mod common;

use ajentic_core::execution::{
    ControlledRunError, ControlledRunRequest, ProviderKind, ProviderOutput, ProviderOutputStatus,
};
use ajentic_core::ledger::{LedgerActor, LedgerActorType};
use ajentic_core::policy::{evaluate_policy, PolicyDecision, PolicyEvidence};

#[test]
fn validation_receipt_from_run_one_cannot_allow_run_two_policy() {
    let first = common::receipt_bundle("run-1", "candidate");
    let second = common::receipt_bundle("run-2", "candidate");
    let receipt = evaluate_policy(
        second.binding,
        &PolicyEvidence::new(true, true, false),
        &first.validation,
    );
    assert_eq!(receipt.decision(), PolicyDecision::Denied);
    assert_eq!(receipt.reason(), "validation_binding_mismatch");
}

#[test]
fn candidate_digest_mismatch_rejects_controlled_request() {
    let bundle = common::receipt_bundle("run-1", "candidate-a");
    let output = ProviderOutput::new_untrusted(
        "output-1",
        "request-1",
        ProviderKind::Local,
        "candidate-b",
        ProviderOutputStatus::Received,
    )
    .unwrap();
    let result = ControlledRunRequest::new(
        "context",
        output,
        bundle.policy,
        bundle.validation,
        bundle.replay,
        bundle.ledger,
        LedgerActor::new(LedgerActorType::System, "system").unwrap(),
        bundle.manifest,
        bundle.evaluation_evidence,
    );
    assert_eq!(result, Err(ControlledRunError::CandidateDigestMismatch));
}
