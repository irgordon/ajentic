use crate::domain::registry::{code_patch_review_domain, security_analysis_domain};
use crate::evaluation::result::EvaluationStatus;
use crate::governance::runtime::GovernanceStatus;
use crate::ledger::entry::{
    CandidateCreatedLedgerRecord, EvaluationRecordedLedgerRecord, GovernanceReviewedLedgerRecord,
    LedgerEntry,
};
use crate::ledger::InMemoryLedger;
use crate::replay::replay_candidate;
use crate::verification::state_snapshot::collect_verification_state_snapshot;
use crate::verification::{verify_architecture_alignment, AlignmentEvidence};

fn alignment() -> crate::verification::ArchitectureAlignmentVerification {
    verify_architecture_alignment(&AlignmentEvidence {
        architecture_rules_present: true,
        boundary_rules_present: true,
        deterministic_replay_behavior_confirmed: true,
        fail_closed_behavior_confirmed: true,
        documentation_matches_implementation: true,
        unauthorized_non_rust_lifecycle_mutation: false,
        unauthorized_non_rust_promotion_authorization: false,
        unauthorized_state_transition_path: false,
        missing_required_facts_detected: false,
        malformed_inputs_detected: false,
        unknown_inputs_detected: false,
    })
}

#[test]
fn verification_classification_is_domain_independent() {
    let mut code_ledger = InMemoryLedger::new();
    let mut security_ledger = InMemoryLedger::new();

    for (ledger, domain_id, candidate_id) in [
        (&mut code_ledger, code_patch_review_domain().id, "cand-code"),
        (
            &mut security_ledger,
            security_analysis_domain().id,
            "cand-security",
        ),
    ] {
        ledger
            .append(LedgerEntry::CandidateCreated(
                CandidateCreatedLedgerRecord {
                    candidate_id: candidate_id.into(),
                    run_id: "run-1".into(),
                    objective_id: "obj-1".into(),
                    constraints_id: "con-1".into(),
                    domain_id,
                },
            ))
            .unwrap();
        ledger
            .append(LedgerEntry::EvaluationRecorded(
                EvaluationRecordedLedgerRecord {
                    evaluation_result_id: format!("eval-{candidate_id}"),
                    candidate_id: candidate_id.into(),
                    evaluator_id: "eval-1".into(),
                    status: EvaluationStatus::Pass,
                    evidence_ref: "ev://eval".into(),
                },
            ))
            .unwrap();
        ledger
            .append(LedgerEntry::GovernanceReviewed(
                GovernanceReviewedLedgerRecord {
                    governance_result_id: format!("gov-{candidate_id}"),
                    candidate_id: candidate_id.into(),
                    status: GovernanceStatus::Pass,
                    required_evaluators_satisfied: true,
                    evidence_refs: vec!["ev://gov".into()],
                    blocked_reasons: vec![],
                    failure_reasons: vec![],
                },
            ))
            .unwrap();
    }

    let code_replay = replay_candidate("cand-code", &code_ledger).unwrap();
    let security_replay = replay_candidate("cand-security", &security_ledger).unwrap();

    let code_snapshot = collect_verification_state_snapshot(&alignment(), &code_replay);
    let security_snapshot = collect_verification_state_snapshot(&alignment(), &security_replay);

    assert_eq!(
        code_replay.readiness_status,
        security_replay.readiness_status
    );
    assert_eq!(
        code_replay.completion_status,
        security_replay.completion_status
    );
    assert_eq!(code_replay.final_status, security_replay.final_status);
    assert_eq!(code_snapshot, security_snapshot);
}
