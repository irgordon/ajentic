use crate::candidate::lifecycle::CandidateLifecycleState;
use crate::errors::ReplayError;
use crate::governance::promotion::PromotionStatus;
use crate::governance::runtime::GovernanceStatus;
use crate::ledger::entry::LedgerEntry;
use crate::ledger::InMemoryLedger;
use crate::replay::status::{ReplayCompletionStatus, ReplayFinalStatus, ReplayReadinessStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayResult {
    pub candidate_id: String,
    pub run_id: String,
    pub objective_id: String,
    pub constraints_id: String,
    pub domain_id: String,
    pub evaluation_result_ids: Vec<String>,
    pub governance_result_ids: Vec<String>,
    pub promotion_decision_ids: Vec<String>,
    pub reuse_event_ids: Vec<String>,
    pub readiness_status: ReplayReadinessStatus,
    pub completion_status: ReplayCompletionStatus,
    pub final_status: ReplayFinalStatus,
    pub evidence_refs: Vec<String>,
    pub blocked_reasons: Vec<String>,
    pub failure_reasons: Vec<String>,
}

pub fn replay_candidate(
    candidate_id: &str,
    ledger: &InMemoryLedger,
) -> Result<ReplayResult, ReplayError> {
    if candidate_id.is_empty() {
        return Err(ReplayError::MissingCandidateId);
    }

    let mut candidate = None;
    let mut evaluation_result_ids = Vec::new();
    let mut governance_result_ids = Vec::new();
    let mut promotion_decision_ids = Vec::new();
    let mut reuse_event_ids = Vec::new();
    let mut evidence_refs = Vec::new();
    let mut blocked_reasons = Vec::new();
    let mut failure_reasons = Vec::new();
    let mut saw_passing_governance = false;
    let mut last_governance_status = None;
    let mut last_promotion_status = None;

    for entry in ledger.entries() {
        match entry {
            LedgerEntry::CandidateCreated(record) if record.candidate_id == candidate_id => {
                candidate = Some(record);
            }
            LedgerEntry::EvaluationRecorded(record) if record.candidate_id == candidate_id => {
                if candidate.is_none() {
                    return Err(ReplayError::InvalidEntryOrder {
                        candidate_id: candidate_id.to_string(),
                    });
                }

                evaluation_result_ids.push(record.evaluation_result_id.clone());
                evidence_refs.push(record.evidence_ref.clone());
            }
            LedgerEntry::GovernanceReviewed(record) if record.candidate_id == candidate_id => {
                if candidate.is_none() {
                    return Err(ReplayError::InvalidEntryOrder {
                        candidate_id: candidate_id.to_string(),
                    });
                }

                governance_result_ids.push(record.governance_result_id.clone());
                evidence_refs.extend(record.evidence_refs.clone());
                blocked_reasons.extend(record.blocked_reasons.clone());
                failure_reasons.extend(record.failure_reasons.clone());

                if record.status == GovernanceStatus::Pass {
                    saw_passing_governance = true;
                }

                last_governance_status = Some(record.status.clone());
            }
            LedgerEntry::PromotionDecided(record) if record.candidate_id == candidate_id => {
                if candidate.is_none() {
                    return Err(ReplayError::InvalidEntryOrder {
                        candidate_id: candidate_id.to_string(),
                    });
                }

                if record.promotion_status == PromotionStatus::Approved {
                    if evaluation_result_ids.is_empty() || !saw_passing_governance {
                        return Err(ReplayError::InvalidEntryOrder {
                            candidate_id: candidate_id.to_string(),
                        });
                    }

                    if record.evidence_refs.is_empty() {
                        return Err(ReplayError::MissingEvidence {
                            candidate_id: candidate_id.to_string(),
                        });
                    }

                    if !record.required_checks_passed
                        || record.from_state != CandidateLifecycleState::Passed
                        || record.to_state != CandidateLifecycleState::PromotedTier1
                    {
                        return Err(ReplayError::UnsupportedPromotion {
                            candidate_id: candidate_id.to_string(),
                        });
                    }
                }

                promotion_decision_ids.push(record.promotion_decision_id.clone());
                evidence_refs.extend(record.evidence_refs.clone());

                // Promotion denial reasons are not typed in Phase 9, so replay preserves them
                // in both reason collections until a later phase adds a narrower record shape.
                blocked_reasons.extend(record.denial_reasons.clone());
                failure_reasons.extend(record.denial_reasons.clone());

                last_promotion_status = Some(record.promotion_status.clone());
            }
            LedgerEntry::ReuseApplied(record)
                if record.reused_candidate_id == candidate_id
                    || record.target_candidate_id == candidate_id =>
            {
                if candidate.is_none() {
                    return Err(ReplayError::InvalidEntryOrder {
                        candidate_id: candidate_id.to_string(),
                    });
                }

                reuse_event_ids.push(record.reuse_event_id.clone());
            }
            _ => {}
        }
    }

    let readiness_status = if candidate.is_none() {
        ReplayReadinessStatus::MissingCandidate
    } else if evaluation_result_ids.is_empty() {
        ReplayReadinessStatus::MissingEvaluation
    } else if governance_result_ids.is_empty() {
        ReplayReadinessStatus::MissingGovernance
    } else if promotion_decision_ids.is_empty() {
        ReplayReadinessStatus::MissingPromotion
    } else {
        ReplayReadinessStatus::Ready
    };

    let completion_status = if readiness_status == ReplayReadinessStatus::Ready {
        ReplayCompletionStatus::Complete
    } else {
        ReplayCompletionStatus::Incomplete
    };

    let candidate = match candidate {
        Some(candidate) => candidate,
        None => {
            return Ok(ReplayResult {
                candidate_id: candidate_id.to_string(),
                run_id: String::new(),
                objective_id: String::new(),
                constraints_id: String::new(),
                domain_id: String::new(),
                evaluation_result_ids,
                governance_result_ids,
                promotion_decision_ids,
                reuse_event_ids,
                readiness_status,
                completion_status,
                final_status: ReplayFinalStatus::Unknown,
                evidence_refs,
                blocked_reasons,
                failure_reasons,
            })
        }
    };

    if candidate.run_id.is_empty() {
        return Err(ReplayError::MissingRunId {
            candidate_id: candidate_id.to_string(),
        });
    }
    if candidate.objective_id.is_empty() {
        return Err(ReplayError::MissingObjectiveId {
            candidate_id: candidate_id.to_string(),
        });
    }
    if candidate.constraints_id.is_empty() {
        return Err(ReplayError::MissingConstraintsId {
            candidate_id: candidate_id.to_string(),
        });
    }
    if candidate.domain_id.is_empty() {
        return Err(ReplayError::MissingDomainId {
            candidate_id: candidate_id.to_string(),
        });
    }
    // Phase 9 promotion decisions do not carry an explicit governance-result reference.
    // Denied replay status is therefore mapped from the last governance status available
    // for this candidate in the ledger.
    let final_status = match last_promotion_status {
        Some(PromotionStatus::Approved) => ReplayFinalStatus::PromotedTier1,
        Some(PromotionStatus::Denied) => match last_governance_status {
            Some(GovernanceStatus::Fail) => ReplayFinalStatus::Failed,
            Some(GovernanceStatus::Blocked) => ReplayFinalStatus::Blocked,
            Some(GovernanceStatus::Unknown) => ReplayFinalStatus::Unknown,
            _ => ReplayFinalStatus::Denied,
        },
        None => ReplayFinalStatus::Unknown,
    };

    if final_status == ReplayFinalStatus::PromotedTier1 {
        if evaluation_result_ids.is_empty() {
            return Err(ReplayError::EvaluationEntryMissing {
                candidate_id: candidate_id.to_string(),
            });
        }
        if governance_result_ids.is_empty() {
            return Err(ReplayError::GovernanceEntryMissing {
                candidate_id: candidate_id.to_string(),
            });
        }
        if promotion_decision_ids.is_empty() {
            return Err(ReplayError::PromotionEntryMissing {
                candidate_id: candidate_id.to_string(),
            });
        }
        if evidence_refs.is_empty() {
            return Err(ReplayError::MissingEvidence {
                candidate_id: candidate_id.to_string(),
            });
        }
    }

    Ok(ReplayResult {
        candidate_id: candidate_id.to_string(),
        run_id: candidate.run_id.clone(),
        objective_id: candidate.objective_id.clone(),
        constraints_id: candidate.constraints_id.clone(),
        domain_id: candidate.domain_id.clone(),
        evaluation_result_ids,
        governance_result_ids,
        promotion_decision_ids,
        reuse_event_ids,
        readiness_status,
        completion_status,
        final_status,
        evidence_refs,
        blocked_reasons,
        failure_reasons,
    })
}

pub fn replay_status_from_ledger(
    candidate_id: &str,
    ledger: &InMemoryLedger,
) -> Result<ReplayFinalStatus, ReplayError> {
    replay_candidate(candidate_id, ledger).map(|result| result.final_status)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluation::result::EvaluationStatus;
    use crate::governance::runtime::GovernanceStatus;
    use crate::ledger::entry::*;

    fn created_with_id(candidate_id: &str) -> LedgerEntry {
        LedgerEntry::CandidateCreated(CandidateCreatedLedgerRecord {
            candidate_id: candidate_id.into(),
            run_id: "run-1".into(),
            objective_id: "obj-1".into(),
            constraints_id: "con-1".into(),
            domain_id: "dom-1".into(),
        })
    }

    fn created() -> LedgerEntry {
        created_with_id("cand-1")
    }

    fn eval() -> LedgerEntry {
        LedgerEntry::EvaluationRecorded(EvaluationRecordedLedgerRecord {
            evaluation_result_id: "eval-1".into(),
            candidate_id: "cand-1".into(),
            evaluator_id: "e-1".into(),
            status: EvaluationStatus::Pass,
            evidence_ref: "ev://e1".into(),
        })
    }

    fn eval_for(candidate_id: &str, result_id: &str) -> LedgerEntry {
        LedgerEntry::EvaluationRecorded(EvaluationRecordedLedgerRecord {
            evaluation_result_id: result_id.into(),
            candidate_id: candidate_id.into(),
            evaluator_id: "e-1".into(),
            status: EvaluationStatus::Pass,
            evidence_ref: "ev://e1".into(),
        })
    }

    fn gov(status: GovernanceStatus) -> LedgerEntry {
        LedgerEntry::GovernanceReviewed(GovernanceReviewedLedgerRecord {
            governance_result_id: "gov-1".into(),
            candidate_id: "cand-1".into(),
            status,
            required_evaluators_satisfied: true,
            evidence_refs: vec!["ev://g1".into()],
            blocked_reasons: vec!["blocked".into()],
            failure_reasons: vec!["failed".into()],
        })
    }

    fn gov_for(candidate_id: &str, result_id: &str, status: GovernanceStatus) -> LedgerEntry {
        LedgerEntry::GovernanceReviewed(GovernanceReviewedLedgerRecord {
            governance_result_id: result_id.into(),
            candidate_id: candidate_id.into(),
            status,
            required_evaluators_satisfied: true,
            evidence_refs: vec!["ev://g1".into()],
            blocked_reasons: vec!["blocked".into()],
            failure_reasons: vec!["failed".into()],
        })
    }

    fn approved_with_id(id: &str) -> LedgerEntry {
        LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
            promotion_decision_id: id.into(),
            candidate_id: "cand-1".into(),
            promotion_status: PromotionStatus::Approved,
            from_state: CandidateLifecycleState::Passed,
            to_state: CandidateLifecycleState::PromotedTier1,
            required_checks_passed: true,
            evidence_refs: vec!["ev://p1".into()],
            denial_reasons: vec![],
        })
    }

    fn approved() -> LedgerEntry {
        approved_with_id("prom-1")
    }

    fn denied_with_id(id: &str) -> LedgerEntry {
        LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
            promotion_decision_id: id.into(),
            candidate_id: "cand-1".into(),
            promotion_status: PromotionStatus::Denied,
            from_state: CandidateLifecycleState::Passed,
            to_state: CandidateLifecycleState::Passed,
            required_checks_passed: true,
            evidence_refs: vec!["ev://p1".into()],
            denial_reasons: vec!["deny".into()],
        })
    }

    fn denied_for(candidate_id: &str, promotion_id: &str) -> LedgerEntry {
        LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
            promotion_decision_id: promotion_id.into(),
            candidate_id: candidate_id.into(),
            promotion_status: PromotionStatus::Denied,
            from_state: CandidateLifecycleState::Passed,
            to_state: CandidateLifecycleState::Passed,
            required_checks_passed: true,
            evidence_refs: vec!["ev://p1".into()],
            denial_reasons: vec!["deny".into()],
        })
    }

    fn reuse() -> LedgerEntry {
        LedgerEntry::ReuseApplied(ReuseAppliedLedgerRecord {
            reuse_event_id: "reuse-1".into(),
            reused_candidate_id: "cand-1".into(),
            target_candidate_id: "cand-2".into(),
            reuse_reason: "same objective type".into(),
            triggering_actor: "owner".into(),
            timestamp_reference: "manual-ref-1".into(),
        })
    }

    #[test]
    fn replays_approved_promotion() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();
        ledger.append(approved()).unwrap();

        let result = replay_candidate("cand-1", &ledger).unwrap();

        assert_eq!(result.final_status, ReplayFinalStatus::PromotedTier1);
        assert_eq!(result.run_id, "run-1");
        assert_eq!(result.objective_id, "obj-1");
        assert_eq!(result.constraints_id, "con-1");
        assert_eq!(result.domain_id, "dom-1");
        assert_eq!(result.evaluation_result_ids, vec!["eval-1"]);
        assert_eq!(result.governance_result_ids, vec!["gov-1"]);
        assert_eq!(result.promotion_decision_ids, vec!["prom-1"]);
        assert!(!result.evidence_refs.is_empty());
    }

    #[test]
    fn denied_promotion_after_blocked_governance_replays_blocked() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Blocked)).unwrap();
        ledger.append(denied_with_id("prom-1")).unwrap();

        let result = replay_candidate("cand-1", &ledger).unwrap();

        assert_eq!(result.final_status, ReplayFinalStatus::Blocked);
        assert_eq!(result.promotion_decision_ids, vec!["prom-1"]);
    }

    #[test]
    fn keeps_all_promotion_ids_and_uses_last_status() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();
        ledger.append(denied_with_id("prom-1")).unwrap();
        ledger.append(approved_with_id("prom-2")).unwrap();

        let result = replay_candidate("cand-1", &ledger).unwrap();

        assert_eq!(result.final_status, ReplayFinalStatus::PromotedTier1);
        assert_eq!(result.promotion_decision_ids, vec!["prom-1", "prom-2"]);
    }

    #[test]
    fn reuse_event_replays_for_source_and_target_candidates_without_affecting_status_or_facts() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created_with_id("cand-1")).unwrap();
        ledger.append(eval_for("cand-1", "eval-1")).unwrap();
        ledger
            .append(gov_for("cand-1", "gov-1", GovernanceStatus::Unknown))
            .unwrap();
        ledger.append(denied_for("cand-1", "prom-1")).unwrap();

        ledger.append(created_with_id("cand-2")).unwrap();
        ledger.append(eval_for("cand-2", "eval-2")).unwrap();
        ledger
            .append(gov_for("cand-2", "gov-2", GovernanceStatus::Blocked))
            .unwrap();
        ledger.append(denied_for("cand-2", "prom-2")).unwrap();

        ledger.append(reuse()).unwrap();

        let source_result = replay_candidate("cand-1", &ledger).unwrap();
        let target_result = replay_candidate("cand-2", &ledger).unwrap();

        assert_eq!(source_result.reuse_event_ids, vec!["reuse-1"]);
        assert_eq!(target_result.reuse_event_ids, vec!["reuse-1"]);

        assert_eq!(source_result.final_status, ReplayFinalStatus::Unknown);
        assert_eq!(target_result.final_status, ReplayFinalStatus::Blocked);

        assert_eq!(source_result.evaluation_result_ids, vec!["eval-1"]);
        assert_eq!(target_result.evaluation_result_ids, vec!["eval-2"]);
        assert_eq!(source_result.governance_result_ids, vec!["gov-1"]);
        assert_eq!(target_result.governance_result_ids, vec!["gov-2"]);
        assert_eq!(source_result.promotion_decision_ids, vec!["prom-1"]);
        assert_eq!(target_result.promotion_decision_ids, vec!["prom-2"]);
    }

    #[test]
    fn empty_candidate_id_fails() {
        let ledger = InMemoryLedger::new();

        assert_eq!(
            replay_candidate("", &ledger),
            Err(ReplayError::MissingCandidateId)
        );
    }

    #[test]
    fn missing_candidate_entry_is_classified_not_ready() {
        let ledger = InMemoryLedger::new();

        let result = replay_candidate("cand-1", &ledger).unwrap();
        assert_eq!(
            result.readiness_status,
            ReplayReadinessStatus::MissingCandidate
        );
        assert_eq!(result.completion_status, ReplayCompletionStatus::Incomplete);
        assert_eq!(result.final_status, ReplayFinalStatus::Unknown);
    }

    #[test]
    fn ledger_rejects_promotion_before_candidate() {
        let mut ledger = InMemoryLedger::new();

        assert!(ledger
            .append(LedgerEntry::PromotionDecided(
                PromotionDecidedLedgerRecord {
                    promotion_decision_id: "prom-1".into(),
                    candidate_id: "cand-1".into(),
                    promotion_status: PromotionStatus::Denied,
                    from_state: CandidateLifecycleState::Passed,
                    to_state: CandidateLifecycleState::Passed,
                    required_checks_passed: true,
                    evidence_refs: vec!["ev://p1".into()],
                    denial_reasons: vec!["deny".into()],
                },
            ))
            .is_err());
    }

    #[test]
    fn replay_without_promotion_entry_is_incomplete() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();

        let result = replay_candidate("cand-1", &ledger).unwrap();
        assert_eq!(
            result.readiness_status,
            ReplayReadinessStatus::MissingPromotion
        );
        assert_eq!(result.completion_status, ReplayCompletionStatus::Incomplete);
    }

    #[test]
    fn approved_promotion_before_evaluation_is_rejected_by_ledger() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();

        assert!(ledger.append(approved()).is_err());
    }

    #[test]
    fn approved_promotion_before_passing_governance_is_rejected_by_ledger() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Fail)).unwrap();

        assert!(ledger.append(approved()).is_err());
    }

    #[test]
    fn approved_promotion_with_empty_evidence_is_rejected_by_ledger() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();

        let entry = LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
            promotion_decision_id: "prom-1".into(),
            candidate_id: "cand-1".into(),
            promotion_status: PromotionStatus::Approved,
            from_state: CandidateLifecycleState::Passed,
            to_state: CandidateLifecycleState::PromotedTier1,
            required_checks_passed: true,
            evidence_refs: vec![],
            denial_reasons: vec![],
        });

        assert!(ledger.append(entry).is_err());
    }

    #[test]
    fn approved_promotion_with_required_checks_false_is_rejected_by_ledger() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();

        let entry = LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
            promotion_decision_id: "prom-1".into(),
            candidate_id: "cand-1".into(),
            promotion_status: PromotionStatus::Approved,
            from_state: CandidateLifecycleState::Passed,
            to_state: CandidateLifecycleState::PromotedTier1,
            required_checks_passed: false,
            evidence_refs: vec!["ev://p1".into()],
            denial_reasons: vec![],
        });

        assert!(ledger.append(entry).is_err());
    }

    #[test]
    fn approved_promotion_from_non_passed_state_is_rejected_by_ledger() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();

        let entry = LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
            promotion_decision_id: "prom-1".into(),
            candidate_id: "cand-1".into(),
            promotion_status: PromotionStatus::Approved,
            from_state: CandidateLifecycleState::Created,
            to_state: CandidateLifecycleState::PromotedTier1,
            required_checks_passed: true,
            evidence_refs: vec!["ev://p1".into()],
            denial_reasons: vec![],
        });

        assert!(ledger.append(entry).is_err());
    }

    #[test]
    fn approved_promotion_to_non_promoted_state_is_rejected_by_ledger() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();

        let entry = LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
            promotion_decision_id: "prom-1".into(),
            candidate_id: "cand-1".into(),
            promotion_status: PromotionStatus::Approved,
            from_state: CandidateLifecycleState::Passed,
            to_state: CandidateLifecycleState::Passed,
            required_checks_passed: true,
            evidence_refs: vec!["ev://p1".into()],
            denial_reasons: vec![],
        });

        assert!(ledger.append(entry).is_err());
    }

    #[test]
    fn replay_readiness_is_deterministic() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();

        let first = replay_candidate("cand-1", &ledger).unwrap();
        let second = replay_candidate("cand-1", &ledger).unwrap();

        assert_eq!(first.readiness_status, second.readiness_status);
        assert_eq!(first.completion_status, second.completion_status);
    }

    #[test]
    fn replay_missing_fact_states_are_classified_not_ready() {
        let mut missing_evaluation = InMemoryLedger::new();
        missing_evaluation.append(created()).unwrap();
        let result = replay_candidate("cand-1", &missing_evaluation).unwrap();
        assert_eq!(
            result.readiness_status,
            ReplayReadinessStatus::MissingEvaluation
        );

        let mut missing_governance = InMemoryLedger::new();
        missing_governance.append(created()).unwrap();
        missing_governance.append(eval()).unwrap();
        let result = replay_candidate("cand-1", &missing_governance).unwrap();
        assert_eq!(
            result.readiness_status,
            ReplayReadinessStatus::MissingGovernance
        );
    }

    #[test]
    fn replay_output_is_deterministic_for_identical_input() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Blocked)).unwrap();
        ledger.append(denied_with_id("prom-1")).unwrap();

        let first = replay_candidate("cand-1", &ledger).unwrap();
        let second = replay_candidate("cand-1", &ledger).unwrap();
        assert_eq!(first, second);
    }
}
