//! Phase 6 evaluation result ingestion.
//! Evaluation satisfaction is not promotion eligibility.
//! Governance promotion consumes evaluation results as one input only.

use crate::candidate::contract::CandidateRecord;
use crate::errors::EvaluationIngestionError;

use super::result::{EvaluationResultRecord, EvaluationResultSet, EvaluationStatus};

pub fn ingest_evaluation_result(
    candidate: &CandidateRecord,
    result: EvaluationResultRecord,
    result_set: &mut EvaluationResultSet,
) -> Result<(), EvaluationIngestionError> {
    if candidate.id.is_empty() {
        return Err(EvaluationIngestionError::MissingCandidateId);
    }
    if result.id.is_empty() {
        return Err(EvaluationIngestionError::MissingResultId);
    }
    if result.candidate_solution_id.is_empty() {
        return Err(EvaluationIngestionError::MissingResultCandidateId);
    }
    if result.candidate_solution_id != candidate.id {
        return Err(EvaluationIngestionError::CandidateIdMismatch {
            expected: candidate.id.clone(),
            found: result.candidate_solution_id,
        });
    }
    if result.evaluator_id.is_empty() {
        return Err(EvaluationIngestionError::MissingEvaluatorId);
    }
    if result.evidence_ref.is_empty() {
        return Err(EvaluationIngestionError::MissingEvidenceRef);
    }
    if result_set.candidate_id != candidate.id {
        return Err(EvaluationIngestionError::ResultSetCandidateMismatch {
            expected: candidate.id.clone(),
            found: result_set.candidate_id.clone(),
        });
    }
    if result_set.results.iter().any(|item| item.id == result.id) {
        return Err(EvaluationIngestionError::DuplicateResultId {
            id: result.id.clone(),
        });
    }
    if result_set
        .results
        .iter()
        .any(|item| item.evaluator_id == result.evaluator_id)
    {
        return Err(EvaluationIngestionError::DuplicateEvaluatorId {
            evaluator_id: result.evaluator_id.clone(),
        });
    }

    result_set.results.push(result);
    Ok(())
}

pub fn required_evaluators_recorded(
    result_set: &EvaluationResultSet,
    required_evaluator_ids: &[String],
) -> bool {
    missing_required_evaluators(result_set, required_evaluator_ids).is_empty()
}

pub fn missing_required_evaluators(
    result_set: &EvaluationResultSet,
    required_evaluator_ids: &[String],
) -> Vec<String> {
    required_evaluator_ids
        .iter()
        .filter(|required_id| {
            !result_set
                .results
                .iter()
                .any(|result| &result.evaluator_id == *required_id)
        })
        .cloned()
        .collect()
}

pub fn required_evaluators_satisfied(
    result_set: &EvaluationResultSet,
    required_evaluator_ids: &[String],
) -> bool {
    !required_evaluator_ids.is_empty()
        && required_evaluator_ids.iter().all(|required_id| {
            result_set.results.iter().any(|result| {
                result.evaluator_id == *required_id
                    && matches!(result.status, EvaluationStatus::Pass)
            })
        })
}

#[cfg(test)]
mod tests {
    use crate::candidate::lifecycle::CandidateLifecycleState;
    use crate::execution::adapter_protocol::AdapterStatus;

    use super::*;

    fn candidate(id: &str) -> CandidateRecord {
        CandidateRecord {
            id: id.to_string(),
            run_id: "run-1".to_string(),
            domain_id: "domain-1".to_string(),
            objective_id: "objective-1".to_string(),
            constraints_id: "constraints-1".to_string(),
            content_ref: "content-ref".to_string(),
            generation_metadata_ref: "meta-ref".to_string(),
            adapter_name: "mock".to_string(),
            adapter_version: "v1".to_string(),
            adapter_status: AdapterStatus::Completed,
            raw_output_ref: "raw-ref".to_string(),
            structured_output_ref: "structured-ref".to_string(),
            output_text: "text".to_string(),
            lifecycle_state: CandidateLifecycleState::Created,
        }
    }

    fn result(
        id: &str,
        candidate_id: &str,
        evaluator_id: &str,
        status: EvaluationStatus,
    ) -> EvaluationResultRecord {
        EvaluationResultRecord {
            id: id.to_string(),
            candidate_solution_id: candidate_id.to_string(),
            evaluator_id: evaluator_id.to_string(),
            status,
            checked_constraints: vec!["constraints://safety".to_string()],
            threshold_results: vec!["threshold://length:ok".to_string()],
            evidence_ref: "evidence://local/file".to_string(),
            failure_reasons: vec!["none".to_string()],
        }
    }

    #[test]
    fn records_all_status_variants() {
        let c = candidate("candidate-1");
        let mut set = EvaluationResultSet::new(c.id.clone());
        assert!(ingest_evaluation_result(
            &c,
            result("r1", &c.id, "eval-1", EvaluationStatus::Pass),
            &mut set
        )
        .is_ok());
        assert!(ingest_evaluation_result(
            &c,
            result("r2", &c.id, "eval-2", EvaluationStatus::Fail),
            &mut set
        )
        .is_ok());
        assert!(ingest_evaluation_result(
            &c,
            result("r3", &c.id, "eval-3", EvaluationStatus::Blocked),
            &mut set
        )
        .is_ok());
        assert!(ingest_evaluation_result(
            &c,
            result("r4", &c.id, "eval-4", EvaluationStatus::Unknown),
            &mut set
        )
        .is_ok());
        assert_eq!(set.results.len(), 4);
    }

    #[test]
    fn preserves_structured_fields() {
        let c = candidate("candidate-1");
        let mut set = EvaluationResultSet::new(c.id.clone());
        let expected = result("r1", &c.id, "eval-1", EvaluationStatus::Fail);
        ingest_evaluation_result(&c, expected.clone(), &mut set).unwrap();
        assert_eq!(set.results[0], expected);
    }

    #[test]
    fn required_evaluator_recorded_and_satisfied_checks() {
        let c = candidate("candidate-1");
        let mut set = EvaluationResultSet::new(c.id.clone());
        ingest_evaluation_result(
            &c,
            result("r1", &c.id, "eval-a", EvaluationStatus::Pass),
            &mut set,
        )
        .unwrap();
        ingest_evaluation_result(
            &c,
            result("r2", &c.id, "eval-b", EvaluationStatus::Unknown),
            &mut set,
        )
        .unwrap();

        let required = vec!["eval-a".to_string(), "eval-b".to_string()];
        assert!(required_evaluators_recorded(&set, &required));
        assert!(!required_evaluators_satisfied(&set, &required));
        assert!(missing_required_evaluators(&set, &required).is_empty());
    }

    #[test]
    fn empty_required_evaluator_list_is_not_satisfied() {
        let set = EvaluationResultSet::new("candidate-1".to_string());
        let required: Vec<String> = Vec::new();

        assert!(!required_evaluators_satisfied(&set, &required));
    }

    #[test]
    fn rejects_invalid_ingestion_and_keeps_candidate_state() {
        let mut c = candidate("candidate-1");
        let initial_state = c.lifecycle_state;
        let mut set = EvaluationResultSet::new(c.id.clone());

        c.id = "".to_string();
        assert_eq!(
            ingest_evaluation_result(
                &c,
                result("r1", "candidate-1", "eval-1", EvaluationStatus::Pass),
                &mut set
            ),
            Err(EvaluationIngestionError::MissingCandidateId)
        );
        assert_eq!(set.results.len(), 0);
        assert_eq!(initial_state, CandidateLifecycleState::Created);
    }

    #[test]
    fn rejects_all_required_failure_cases() {
        let c = candidate("candidate-1");

        let mut set = EvaluationResultSet::new(c.id.clone());
        assert_eq!(
            ingest_evaluation_result(
                &c,
                result("", &c.id, "eval-1", EvaluationStatus::Pass),
                &mut set
            ),
            Err(EvaluationIngestionError::MissingResultId)
        );

        let mut set = EvaluationResultSet::new(c.id.clone());
        assert_eq!(
            ingest_evaluation_result(
                &c,
                result("r1", "", "eval-1", EvaluationStatus::Pass),
                &mut set
            ),
            Err(EvaluationIngestionError::MissingResultCandidateId)
        );

        let mut set = EvaluationResultSet::new(c.id.clone());
        assert_eq!(
            ingest_evaluation_result(
                &c,
                result("r1", "wrong", "eval-1", EvaluationStatus::Pass),
                &mut set
            ),
            Err(EvaluationIngestionError::CandidateIdMismatch {
                expected: c.id.clone(),
                found: "wrong".to_string()
            })
        );

        let mut missing_eval = result("r1", &c.id, "eval-1", EvaluationStatus::Pass);
        missing_eval.evaluator_id = "".to_string();
        let mut set = EvaluationResultSet::new(c.id.clone());
        assert_eq!(
            ingest_evaluation_result(&c, missing_eval, &mut set),
            Err(EvaluationIngestionError::MissingEvaluatorId)
        );

        let mut missing_evidence = result("r1", &c.id, "eval-1", EvaluationStatus::Pass);
        missing_evidence.evidence_ref = "".to_string();
        let mut set = EvaluationResultSet::new(c.id.clone());
        assert_eq!(
            ingest_evaluation_result(&c, missing_evidence, &mut set),
            Err(EvaluationIngestionError::MissingEvidenceRef)
        );

        let mut wrong_set = EvaluationResultSet::new("different-candidate".to_string());
        assert_eq!(
            ingest_evaluation_result(
                &c,
                result("r1", &c.id, "eval-1", EvaluationStatus::Pass),
                &mut wrong_set
            ),
            Err(EvaluationIngestionError::ResultSetCandidateMismatch {
                expected: c.id.clone(),
                found: "different-candidate".to_string()
            })
        );

        let mut set = EvaluationResultSet::new(c.id.clone());
        ingest_evaluation_result(
            &c,
            result("r1", &c.id, "eval-1", EvaluationStatus::Pass),
            &mut set,
        )
        .unwrap();
        assert_eq!(
            ingest_evaluation_result(
                &c,
                result("r1", &c.id, "eval-2", EvaluationStatus::Pass),
                &mut set
            ),
            Err(EvaluationIngestionError::DuplicateResultId {
                id: "r1".to_string()
            })
        );
        assert_eq!(
            ingest_evaluation_result(
                &c,
                result("r2", &c.id, "eval-1", EvaluationStatus::Pass),
                &mut set
            ),
            Err(EvaluationIngestionError::DuplicateEvaluatorId {
                evaluator_id: "eval-1".to_string()
            })
        );

        let required = vec!["eval-1".to_string(), "eval-2".to_string()];
        assert!(!required_evaluators_satisfied(&set, &required));
    }
}
