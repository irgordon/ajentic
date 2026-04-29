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

    if evaluations.candidate_id != candidate.id {
        return Err(GovernanceError::CandidateIdMismatch {
            expected: candidate.id.clone(),
            found: evaluations.candidate_id.clone(),
        });
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

    let mut evaluator_failure_found = false;
    let mut evaluator_block_found = false;

    for missing in missing_required_evaluators(evaluations, &input.required_evaluator_ids) {
        evaluator_block_found = true;
        blocked_reasons.push(format!("missing evaluator: {missing}"));
    }

    for evaluator_id in &input.required_evaluator_ids {
        let Some(result) = evaluations
            .results
            .iter()
            .find(|result| &result.evaluator_id == evaluator_id)
        else {
            continue;
        };

        if result.evidence_ref.is_empty() {
            evaluator_failure_found = true;
            failure_reasons.push(format!("missing evaluator evidence: {evaluator_id}"));
        }

        match result.status {
            EvaluationStatus::Pass => {}
            EvaluationStatus::Fail => {
                evaluator_failure_found = true;
                failure_reasons.push(format!("evaluator failed: {evaluator_id}"));
            }
            EvaluationStatus::Blocked => {
                evaluator_block_found = true;
                blocked_reasons.push(format!("evaluator blocked: {evaluator_id}"));
            }
            EvaluationStatus::Unknown => {
                evaluator_block_found = true;
                blocked_reasons.push(format!("evaluator unknown: {evaluator_id}"));
            }
        }
    }

    let required_evaluators_satisfied = !evaluator_failure_found && !evaluator_block_found;

    for policy_check_id in &input.required_policy_check_ids {
        let Some(check) = policy_checks
            .iter()
            .find(|policy_check| &policy_check.policy_check_id == policy_check_id)
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
                failure_reasons.push(format!("policy check failed: {policy_check_id}"));
            }
            PolicyCheckStatus::Blocked => {
                blocked_reasons.push(format!("policy check blocked: {policy_check_id}"));
            }
            PolicyCheckStatus::Unknown => {
                blocked_reasons.push(format!("policy check unknown: {policy_check_id}"));
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
        required_evaluators_satisfied,
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
        let result = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Pass, "ev"),
            input(),
            checks(PolicyCheckStatus::Pass, "ev"),
        )
        .unwrap();

        assert_eq!(result.status, GovernanceStatus::Pass);
        assert!(result.required_evaluators_satisfied);
        assert!(result.failure_reasons.is_empty());
        assert!(result.blocked_reasons.is_empty());
    }

    #[test]
    fn empty_objective_fails() {
        let mut input = input();
        input.objective_id.clear();

        assert_eq!(
            review_governance(
                &candidate(),
                &eval_set(EvaluationStatus::Pass, "ev"),
                input,
                checks(PolicyCheckStatus::Pass, "ev"),
            ),
            Err(GovernanceError::MissingObjectiveId)
        );
    }

    #[test]
    fn evaluation_set_candidate_mismatch_fails() {
        let mut evaluations = eval_set(EvaluationStatus::Pass, "ev");
        evaluations.candidate_id = "other-candidate".into();

        assert_eq!(
            review_governance(
                &candidate(),
                &evaluations,
                input(),
                checks(PolicyCheckStatus::Pass, "ev"),
            ),
            Err(GovernanceError::CandidateIdMismatch {
                expected: "cand-1".into(),
                found: "other-candidate".into(),
            })
        );
    }

    #[test]
    fn input_candidate_mismatch_fails() {
        let mut input = input();
        input.candidate_id = "other-candidate".into();

        assert_eq!(
            review_governance(
                &candidate(),
                &eval_set(EvaluationStatus::Pass, "ev"),
                input,
                checks(PolicyCheckStatus::Pass, "ev"),
            ),
            Err(GovernanceError::CandidateIdMismatch {
                expected: "cand-1".into(),
                found: "other-candidate".into(),
            })
        );
    }

    #[test]
    fn unknown_evaluator_blocks() {
        let result = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Unknown, "ev"),
            input(),
            checks(PolicyCheckStatus::Pass, "ev"),
        )
        .unwrap();

        assert_eq!(result.status, GovernanceStatus::Blocked);
        assert!(!result.required_evaluators_satisfied);
    }

    #[test]
    fn failed_evaluator_fails() {
        let result = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Fail, "ev"),
            input(),
            checks(PolicyCheckStatus::Pass, "ev"),
        )
        .unwrap();

        assert_eq!(result.status, GovernanceStatus::Fail);
        assert!(!result.required_evaluators_satisfied);
    }

    #[test]
    fn blocked_evaluator_blocks() {
        let result = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Blocked, "ev"),
            input(),
            checks(PolicyCheckStatus::Pass, "ev"),
        )
        .unwrap();

        assert_eq!(result.status, GovernanceStatus::Blocked);
        assert!(!result.required_evaluators_satisfied);
    }

    #[test]
    fn missing_evaluator_evidence_fails() {
        let result = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Pass, ""),
            input(),
            checks(PolicyCheckStatus::Pass, "ev"),
        )
        .unwrap();

        assert_eq!(result.status, GovernanceStatus::Fail);
        assert!(!result.required_evaluators_satisfied);
    }

    #[test]
    fn missing_policy_check_fails_without_changing_evaluator_satisfaction() {
        let result = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Pass, "ev"),
            input(),
            Vec::new(),
        )
        .unwrap();

        assert_eq!(result.status, GovernanceStatus::Fail);
        assert!(result.required_evaluators_satisfied);
    }

    #[test]
    fn failed_policy_check_fails_without_changing_evaluator_satisfaction() {
        let result = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Pass, "ev"),
            input(),
            checks(PolicyCheckStatus::Fail, "ev"),
        )
        .unwrap();

        assert_eq!(result.status, GovernanceStatus::Fail);
        assert!(result.required_evaluators_satisfied);
    }

    #[test]
    fn blocked_policy_check_blocks_without_changing_evaluator_satisfaction() {
        let result = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Pass, "ev"),
            input(),
            checks(PolicyCheckStatus::Blocked, "ev"),
        )
        .unwrap();

        assert_eq!(result.status, GovernanceStatus::Blocked);
        assert!(result.required_evaluators_satisfied);
    }

    #[test]
    fn unknown_policy_check_blocks_without_changing_evaluator_satisfaction() {
        let result = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Pass, "ev"),
            input(),
            checks(PolicyCheckStatus::Unknown, "ev"),
        )
        .unwrap();

        assert_eq!(result.status, GovernanceStatus::Blocked);
        assert!(result.required_evaluators_satisfied);
    }

    #[test]
    fn missing_policy_evidence_fails_without_changing_evaluator_satisfaction() {
        let result = review_governance(
            &candidate(),
            &eval_set(EvaluationStatus::Pass, "ev"),
            input(),
            checks(PolicyCheckStatus::Pass, ""),
        )
        .unwrap();

        assert_eq!(result.status, GovernanceStatus::Fail);
        assert!(result.required_evaluators_satisfied);
    }
}
