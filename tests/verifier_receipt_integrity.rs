mod common;

use ajentic_core::authority::{AuthorityBinding, AuthorityBindingInput};
use ajentic_core::execution::{ControlledRunError, ControlledRunRequest};
use ajentic_core::ledger::{LedgerActor, LedgerActorType};
use ajentic_core::policy::evaluate_policy;
use ajentic_core::state::{authorize_promotion, PromotionAuthorizationError};
use ajentic_core::validation::{
    evaluate_validation, ValidationEvidence, ValidationEvidenceError, ValidationStatus,
};
use ajentic_core::verification::{
    verify_candidate_evidence_binding, verify_candidate_shape, verify_evidence_manifest,
    verify_required_context, verify_required_operator_intent,
};

fn rebound(binding: &AuthorityBinding, run_id: &str, revision: u64) -> AuthorityBinding {
    AuthorityBinding::new(AuthorityBindingInput {
        run_id: run_id.into(),
        task_digest: binding.task_digest().clone(),
        operator_intent_digest: binding.operator_intent_digest().clone(),
        context_packet_digest: binding.context_packet_digest().clone(),
        candidate_digest: binding.candidate_digest().clone(),
        policy_bundle_digest: binding.policy_bundle_digest().clone(),
        evidence_manifest_digest: binding.evidence_manifest_digest().clone(),
        verifier_id: binding.verifier_id().into(),
        verifier_version: binding.verifier_version().into(),
        valid_through_revision: revision,
    })
    .unwrap()
}

#[test]
fn cross_candidate_validation_verifier_receipt_fails() {
    let first = common::receipt_bundle("run-1", "candidate-a");
    let second = common::receipt_bundle("run-1", "candidate-b");
    let output = common::provider_output("candidate-a");
    let result = ValidationEvidence::new(
        first.manifest.clone(),
        verify_evidence_manifest(&first.binding, Some(&first.manifest)),
        verify_candidate_shape(&second.binding, Some(&output)),
        verify_candidate_evidence_binding(&first.binding, Some(&output), Some(&first.manifest)),
    );
    assert_eq!(
        result,
        Err(ValidationEvidenceError::VerificationBindingMismatch)
    );
}

#[test]
fn cross_revision_validation_verifier_receipt_fails() {
    let bundle = common::receipt_bundle("run-1", "candidate");
    let later = rebound(&bundle.binding, "run-1", 3);
    let output = common::provider_output("candidate");
    let result = ValidationEvidence::new(
        bundle.manifest.clone(),
        verify_evidence_manifest(&bundle.binding, Some(&bundle.manifest)),
        verify_candidate_shape(&later, Some(&output)),
        verify_candidate_evidence_binding(&bundle.binding, Some(&output), Some(&bundle.manifest)),
    );
    assert_eq!(
        result,
        Err(ValidationEvidenceError::VerificationBindingMismatch)
    );
}

#[test]
fn missing_manifest_verifier_proof_fails_closed() {
    let bundle = common::receipt_bundle("run-1", "candidate");
    let output = common::provider_output("candidate");
    let result = ValidationEvidence::new(
        bundle.manifest.clone(),
        verify_evidence_manifest(&bundle.binding, None),
        verify_candidate_shape(&bundle.binding, Some(&output)),
        verify_candidate_evidence_binding(&bundle.binding, Some(&output), Some(&bundle.manifest)),
    );
    assert_eq!(
        result,
        Err(ValidationEvidenceError::VerificationSourceMismatch)
    );
}

#[test]
fn malformed_evidence_manifest_cannot_produce_validation_proof() {
    assert_eq!(
        ajentic_core::authority::EvidenceManifest::new(Vec::new()),
        Err(ajentic_core::authority::AuthorityBindingError::MissingEvidence)
    );
}

#[test]
fn evidence_manifest_mismatch_prevents_validation_pass() {
    let expected = common::receipt_bundle("run-1", "candidate");
    let other_manifest = ajentic_core::authority::EvidenceManifest::new(vec![
        ajentic_core::authority::EvidenceReference::new(
            "other-evidence",
            ajentic_core::integrity::Digest::of_text("other"),
        )
        .unwrap(),
    ])
    .unwrap();
    let output = common::provider_output("candidate");
    let evidence = ValidationEvidence::new(
        other_manifest.clone(),
        verify_evidence_manifest(&expected.binding, Some(&other_manifest)),
        verify_candidate_shape(&expected.binding, Some(&output)),
        verify_candidate_evidence_binding(&expected.binding, Some(&output), Some(&other_manifest)),
    )
    .unwrap();
    let receipt = evaluate_validation(expected.binding, &evidence);
    assert_ne!(receipt.status(), ValidationStatus::Pass);
}

#[test]
fn controlled_run_rejects_substituted_verifier_evidence() {
    let first = common::receipt_bundle("run-1", "candidate");
    let second = common::receipt_bundle("run-2", "candidate");
    let result = ControlledRunRequest::new(
        "context",
        common::provider_output("candidate"),
        first.policy,
        first.validation,
        first.replay,
        first.ledger,
        LedgerActor::new(LedgerActorType::System, "system").unwrap(),
        first.manifest,
        second.evaluation_evidence,
    );
    assert_eq!(result, Err(ControlledRunError::ReceiptDerivationMismatch));
}

#[test]
fn unknown_validation_verifier_evidence_blocks_promotion() {
    let bundle = common::receipt_bundle("run-1", "candidate");
    let evidence = ValidationEvidence::new(
        bundle.manifest.clone(),
        verify_evidence_manifest(&bundle.binding, Some(&bundle.manifest)),
        verify_candidate_shape(&bundle.binding, None),
        verify_candidate_evidence_binding(&bundle.binding, None, Some(&bundle.manifest)),
    )
    .unwrap();
    let validation = evaluate_validation(bundle.binding.clone(), &evidence);
    let policy_evidence = ajentic_core::policy::PolicyEvidence::new(
        verify_required_context(&bundle.binding, Some("context")),
        verify_required_operator_intent(&bundle.binding, Some("intent")),
    )
    .unwrap();
    let policy = evaluate_policy(bundle.binding.clone(), &policy_evidence, &validation);
    assert_eq!(
        authorize_promotion(bundle.binding, &validation, &policy, &bundle.replay),
        Err(PromotionAuthorizationError::ValidationNotPassed)
    );
}
