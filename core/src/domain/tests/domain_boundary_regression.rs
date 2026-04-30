use crate::candidate::contract::CandidateRecord;
use crate::candidate::lifecycle::{transition, CandidateLifecycleState};
use crate::domain::registry::{code_patch_review_domain, security_analysis_domain};
use crate::evaluation::result::EvaluationStatus;
use crate::execution::adapter_protocol::AdapterStatus;
use crate::governance::promotion::promote_candidate;
use crate::governance::runtime::{GovernanceResultRecord, GovernanceStatus};
use crate::ledger::entry::{
    CandidateCreatedLedgerRecord, EvaluationRecordedLedgerRecord, GovernanceReviewedLedgerRecord,
    LedgerEntry,
};
use crate::ledger::InMemoryLedger;
use crate::policy::engine::{PolicyCheckResult, PolicyCheckStatus};
use crate::replay::replay_candidate;

fn candidate_for_domain(domain_id: &str) -> CandidateRecord {
    CandidateRecord {
        id: format!("cand-{domain_id}"),
        run_id: "run-1".into(),
        domain_id: domain_id.into(),
        objective_id: "obj-1".into(),
        constraints_id: "con-1".into(),
        content_ref: "content://1".into(),
        generation_metadata_ref: "meta://1".into(),
        adapter_name: "mock".into(),
        adapter_version: "1".into(),
        adapter_status: AdapterStatus::Completed,
        raw_output_ref: "raw://1".into(),
        structured_output_ref: "structured://1".into(),
        output_text: "ok".into(),
        lifecycle_state: CandidateLifecycleState::Passed,
    }
}

fn passing_governance(candidate_id: &str) -> GovernanceResultRecord {
    GovernanceResultRecord {
        id: format!("gov::{candidate_id}"),
        candidate_id: candidate_id.into(),
        status: GovernanceStatus::Pass,
        required_evaluators_satisfied: true,
        policy_checks: vec![PolicyCheckResult {
            id: format!("policy::{candidate_id}::p1"),
            policy_check_id: "p1".into(),
            status: PolicyCheckStatus::Pass,
            evidence_ref: "ev://policy".into(),
            failure_reasons: vec![],
        }],
        evidence_refs: vec!["ev://gov".into()],
        blocked_reasons: vec![],
        failure_reasons: vec![],
    }
}

#[test]
fn promotion_logic_is_identical_across_domains() {
    let code = candidate_for_domain(&code_patch_review_domain().id);
    let security = candidate_for_domain(&security_analysis_domain().id);

    let code_result = promote_candidate(&code, &passing_governance(&code.id)).unwrap();
    let security_result = promote_candidate(&security, &passing_governance(&security.id)).unwrap();

    assert_eq!(
        code_result.0.lifecycle_state,
        security_result.0.lifecycle_state
    );
    assert_eq!(
        code_result.1.promotion_status,
        security_result.1.promotion_status
    );
    assert_eq!(
        code_result.1.required_checks_passed,
        security_result.1.required_checks_passed
    );
}

#[test]
fn replay_classification_is_identical_across_domains() {
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

    let code = replay_candidate("cand-code", &code_ledger).unwrap();
    let security = replay_candidate("cand-security", &security_ledger).unwrap();

    assert_eq!(code.readiness_status, security.readiness_status);
    assert_eq!(code.completion_status, security.completion_status);
    assert_eq!(code.final_status, security.final_status);
}

#[test]
fn lifecycle_transitions_are_domain_independent() {
    let from = CandidateLifecycleState::Passed;
    let to = CandidateLifecycleState::PromotedTier1;
    let code = transition(from, to).unwrap();
    let security = transition(from, to).unwrap();

    assert_eq!(code, security);
}

#[test]
fn domain_configuration_cannot_authorize_promotion() {
    let mut candidate = candidate_for_domain(&code_patch_review_domain().id);
    candidate.lifecycle_state = CandidateLifecycleState::Created;

    let denial = promote_candidate(&candidate, &passing_governance(&candidate.id));
    assert!(denial.is_err());
}
