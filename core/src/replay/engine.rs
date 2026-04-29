use crate::candidate::lifecycle::CandidateLifecycleState;
use crate::errors::ReplayError;
use crate::governance::promotion::PromotionStatus;
use crate::governance::runtime::GovernanceStatus;
use crate::ledger::entry::LedgerEntry;
use crate::ledger::InMemoryLedger;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayFinalStatus {
    Promoted,
    Denied,
    Failed,
    Blocked,
    Unknown,
}

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
            _ => {}
        }
    }

    let candidate = candidate.ok_or_else(|| ReplayError::CandidateEntryMissing {
        candidate_id: candidate_id.to_string(),
    })?;

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

    // Phase 9 promotion decisions do not carry an explicit governance-result reference.
    // Denied replay status is therefore mapped from the last governance status available
    // for this candidate in the ledger.
    let final_status = match last_promotion_status {
        Some(PromotionStatus::Approved) => ReplayFinalStatus::Promoted,
        Some(PromotionStatus::Denied) => match last_governance_status {
            Some(GovernanceStatus::Fail) => ReplayFinalStatus::Failed,
            Some(GovernanceStatus::Blocked) => ReplayFinalStatus::Blocked,
            Some(GovernanceStatus::Unknown) => ReplayFinalStatus::Unknown,
            _ => ReplayFinalStatus::Denied,
        },
        None => ReplayFinalStatus::Unknown,
    };

    if final_status == ReplayFinalStatus::Promoted {
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

    fn created() -> LedgerEntry {
        LedgerEntry::CandidateCreated(CandidateCreatedLedgerRecord {
            candidate_id: "cand-1".into(),
            run_id: "run-1".into(),
            objective_id: "obj-1".into(),
            constraints_id: "con-1".into(),
            domain_id: "dom-1".into(),
        })
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

    #[test]
    fn replays_approved_promotion() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();
        ledger.append(approved()).unwrap();

        let result = replay_candidate("cand-1", &ledger).unwrap();

        assert_eq!(result.final_status, ReplayFinalStatus::Promoted);
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
    fn keeps_all_promotion_ids_and_uses_last_status() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(eval()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();
        ledger.append(denied_with_id("prom-1")).unwrap();
        ledger.append(approved_with_id("prom-2")).unwrap();

        let result = replay_candidate("cand-1", &ledger).unwrap();

        assert_eq!(result.final_status, ReplayFinalStatus::Promoted);
        assert_eq!(result.promotion_decision_ids, vec!["prom-1", "prom-2"]);
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
    fn missing_candidate_entry_fails() {
        let ledger = InMemoryLedger::new();

        assert_eq!(
            replay_candidate("cand-1", &ledger),
            Err(ReplayError::CandidateEntryMissing {
                candidate_id: "cand-1".into(),
            })
        );
    }

    #[test]
    fn promotion_before_candidate_fails() {
        let mut ledger = InMemoryLedger::new();
        ledger
            .append(LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
                promotion_decision_id: "prom-1".into(),
                candidate_id: "cand-1".into(),
                promotion_status: PromotionStatus::Denied,
                from_state: CandidateLifecycleState::Passed,
                to_state: CandidateLifecycleState::Passed,
                required_checks_passed: true,
                evidence_refs: vec!["ev://p1".into()],
                denial_reasons: vec!["deny".into()],
            }))
            .unwrap_err();

        assert_eq!(
            replay_candidate("cand-1", &ledger),
            Err(ReplayError::CandidateEntryMissing {
                candidate_id: "cand-1".into(),
            })
        );
    }

    #[test]
    fn promoted_replay_without_evaluation_fails() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created()).unwrap();
        ledger.append(gov(GovernanceStatus::Pass)).unwrap();

        // Bypass append validation is intentionally impossible through the public ledger API,
        // so this test verifies replay behavior up to the missing promotion requirement.
        assert_eq!(
            replay_candidate("cand-1", &ledger),
            Err(ReplayError::PromotionEntryMissing {
                candidate_id: "cand-1".into(),
            })
        );
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
}
