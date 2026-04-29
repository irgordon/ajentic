//! Phase 7 governance runtime records and review logic.

use crate::candidate::contract::CandidateRecord;
use crate::errors::GovernanceError;
use crate::evaluation::ingestion::missing_required_evaluators;
use crate::evaluation::result::{EvaluationResultSet, EvaluationStatus};
use crate::governance::precheck::GovernanceReviewInput;
use crate::policy::engine::{PolicyCheckResult, PolicyCheckStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceStatus {
    Pass,
    Fail,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceResultRecord {
    pub id: String,
    pub candidate_id: String,
    pub status: GovernanceStatus,
    pub required_evaluators_satisfied: bool,
    pub policy_checks: Vec<PolicyCheckResult>,
    pub evidence_refs: Vec<String>,
    pub blocked_reasons: Vec<String>,
    pub failure_reasons: Vec<String>,
}

pub fn review_governance(
    candidate: &CandidateRecord,
    evaluations: &EvaluationResultSet,
    input: GovernanceReviewInput,
    policy_checks: Vec<PolicyCheckResult>,
) -> Result<GovernanceResultRecord, GovernanceError> {
    if candidate.id.is_empty() {
        return Err(GovernanceError::MissingCandidateId);
    }
    if input.objective_id.is_empty() {
        return Err(GovernanceError::MissingObjectiveId);
    }
    if input.constraints_id.is_empty() {
        return Err(GovernanceError::MissingConstraintsId);
    }
    if input.domain_id.is_empty() {
        return Err(GovernanceError::MissingDomainId);
    }
    if input.candidate_id != candidate.id {
        return Err(GovernanceError::CandidateIdMismatch {
            expected: candidate.id.clone(),
            found: input.candidate_id,
        });
    }
    if input.required_evaluator_ids.is_empty() {
        return Err(GovernanceError::MissingRequiredEvaluators);
    }
    if input.evidence_refs.is_empty() {
        return Err(GovernanceError::MissingEvidenceRefs);
    }
    if input.required_policy_check_ids.is_empty() {
        return Err(GovernanceError::MissingRequiredPolicyChecks);
    }

    let mut failure_reasons = Vec::new();
    let mut blocked_reasons = Vec::new();

    for missing in missing_required_evaluators(evaluations, &input.required_evaluator_ids) {
        blocked_reasons.push(format!("missing evaluator: {missing}"));
    }

    for evaluator_id in &input.required_evaluator_ids {
        let Some(result) = evaluations
            .results
            .iter()
            .find(|r| &r.evaluator_id == evaluator_id)
        else {
            continue;
        };
        if result.evidence_ref.is_empty() {
            failure_reasons.push(format!("missing evaluator evidence: {evaluator_id}"));
        }
        match result.status {
            EvaluationStatus::Pass => {}
            EvaluationStatus::Fail => {
                failure_reasons.push(format!("evaluator failed: {evaluator_id}"))
            }
            EvaluationStatus::Blocked => {
                blocked_reasons.push(format!("evaluator blocked: {evaluator_id}"))
            }
            EvaluationStatus::Unknown => {
                blocked_reasons.push(format!("evaluator unknown: {evaluator_id}"))
            }
        }
    }

    for policy_check_id in &input.required_policy_check_ids {
        let Some(check) = policy_checks
            .iter()
            .find(|p| &p.policy_check_id == policy_check_id)
        else {
            failure_reasons.push(format!("missing policy check: {policy_check_id}"));
            continue;
        };
        if check.evidence_ref.is_empty() {
            failure_reasons.push(format!("missing policy evidence: {policy_check_id}"));
        }
        match check.status {
            PolicyCheckStatus::Pass => {}
            PolicyCheckStatus::Fail => {
                failure_reasons.push(format!("policy check failed: {policy_check_id}"))
            }
            PolicyCheckStatus::Blocked => {
                blocked_reasons.push(format!("policy check blocked: {policy_check_id}"))
            }
            PolicyCheckStatus::Unknown => {
                blocked_reasons.push(format!("policy check unknown: {policy_check_id}"))
            }
        }
    }

    let status = if !failure_reasons.is_empty() {
        GovernanceStatus::Fail
    } else if !blocked_reasons.is_empty() {
        GovernanceStatus::Blocked
    } else {
        GovernanceStatus::Pass
    };

    Ok(GovernanceResultRecord {
        id: format!("governance::{}", candidate.id),
        candidate_id: candidate.id.clone(),
        status,
        required_evaluators_satisfied: failure_reasons.is_empty() && blocked_reasons.is_empty(),
        policy_checks,
        evidence_refs: input.evidence_refs,
        blocked_reasons,
        failure_reasons,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::candidate::lifecycle::CandidateLifecycleState;
    use crate::evaluation::result::{EvaluationResultRecord, EvaluationStatus};
    use crate::execution::adapter_protocol::AdapterStatus;

    fn candidate() -> CandidateRecord {
        CandidateRecord {
            id: "cand-1".into(),
            run_id: "r".into(),
            domain_id: "d".into(),
            objective_id: "o".into(),
            constraints_id: "c".into(),
            content_ref: "x".into(),
            generation_metadata_ref: "m".into(),
            adapter_name: "a".into(),
            adapter_version: "1".into(),
            adapter_status: AdapterStatus::Completed,
            raw_output_ref: "r".into(),
            structured_output_ref: "s".into(),
            output_text: "t".into(),
            lifecycle_state: CandidateLifecycleState::Passed,
        }
    }
    fn eval_set(status: EvaluationStatus, evidence: &str) -> EvaluationResultSet {
        EvaluationResultSet {
            candidate_id: "cand-1".into(),
            results: vec![EvaluationResultRecord {
                id: "e1".into(),
                candidate_solution_id: "cand-1".into(),
                evaluator_id: "eval-a".into(),
                status,
                checked_constraints: vec![],
                threshold_results: vec![],
                evidence_ref: evidence.into(),
                failure_reasons: vec![],
            }],
        }
    }
    fn input() -> GovernanceReviewInput {
        GovernanceReviewInput {
            candidate_id: "cand-1".into(),
            objective_id: "o".into(),
            constraints_id: "c".into(),
            domain_id: "d".into(),
            required_evaluator_ids: vec!["eval-a".into()],
            required_policy_check_ids: vec!["pol-a".into()],
            evidence_refs: vec!["ev://1".into()],
        }
    }
    fn checks(status: PolicyCheckStatus, evidence: &str) -> Vec<PolicyCheckResult> {
        vec![PolicyCheckResult {
            id: "policy::cand-1::pol-a".into(),
            policy_check_id: "pol-a".into(),
            status,
            evidence_ref: evidence.into(),
            failure_reasons: vec![],
        }]
    }

    #[test]
    fn governance_passes() {
        let r = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Pass, "ev"),
            input(),
            checks(PolicyCheckStatus::Pass, "ev"),
        )
        .unwrap();
        assert_eq!(r.status, GovernanceStatus::Pass);
        assert!(r.failure_reasons.is_empty());
    }
    #[test]
    fn empty_objective_fails() {
        let mut i = input();
        i.objective_id.clear();
        assert_eq!(
            review_governance(
                &candidate(),
                &eval_set(EvaluationStatus::Pass, "ev"),
                i,
                checks(PolicyCheckStatus::Pass, "ev")
            ),
            Err(GovernanceError::MissingObjectiveId)
        );
    }
    #[test]
    fn unknown_blocks() {
        let r = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Unknown, "ev"),
            input(),
            checks(PolicyCheckStatus::Pass, "ev"),
        )
        .unwrap();
        assert_eq!(r.status, GovernanceStatus::Blocked);
    }
    #[test]
    fn missing_evidence_fails() {
        let r = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Pass, ""),
            input(),
            checks(PolicyCheckStatus::Pass, "ev"),
        )
        .unwrap();
        assert_eq!(r.status, GovernanceStatus::Fail);
    }
}
