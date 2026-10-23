mod common;

use ajentic_core::execution::{
    append_promotion_record, build_authorized_promotion_record, verify_authorized_promotion_replay,
    PromotionReplayEvidence, PromotionReplayVerificationStatus,
};
use ajentic_core::ledger::{Ledger, LedgerActor, LedgerActorType};
use ajentic_core::replay::{verify_replay_receipt, ReplayReceiptError};
use ajentic_core::state::authorize_promotion;

#[test]
fn contiguous_unsealed_history_is_not_replay_proof() {
    let manifest = common::evidence_manifest();
    let binding = common::authority_binding("run-1", "candidate", &manifest, 2);
    let result = verify_replay_receipt(binding, &common::raw_passed_ledger(), &manifest);
    assert_eq!(result, Err(ReplayReceiptError::IntegrityMismatch));
}

#[test]
fn tampered_event_hash_is_rejected_before_append() {
    let bundle = common::receipt_bundle("run-1", "candidate");
    let mut event = bundle.ledger.events()[0].clone();
    event.integrity.as_mut().unwrap().event_hash =
        ajentic_core::integrity::Digest::of_text("tampered");
    assert!(Ledger::empty().append(event).is_err());
}

#[test]
fn promotion_replay_requires_matching_authorization() {
    let bundle = common::receipt_bundle("run-1", "candidate");
    let authorization = authorize_promotion(
        bundle.binding.clone(),
        &bundle.validation,
        &bundle.policy,
        &bundle.replay,
    )
    .unwrap();
    let record = build_authorized_promotion_record(
        &authorization,
        &bundle.manifest,
        LedgerActor::new(LedgerActorType::System, "system").unwrap(),
        &bundle.ledger,
    )
    .unwrap();
    let ledger = append_promotion_record(&bundle.ledger, record).unwrap();
    let report = verify_authorized_promotion_replay(
        &ledger,
        &authorization,
        &bundle.manifest,
        PromotionReplayEvidence {
            evaluations: &bundle.evaluation_evidence,
            validation: &bundle.validation,
            policy: &bundle.policy,
            replay: &bundle.replay,
            authorization_digest: authorization.digest(),
        },
    );
    assert_eq!(report.status, PromotionReplayVerificationStatus::Verified);
}
