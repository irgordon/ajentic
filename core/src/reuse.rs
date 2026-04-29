use crate::errors::ReuseError;
use crate::ledger::entry::{LedgerEntry, ReuseAppliedLedgerRecord};
use crate::ledger::InMemoryLedger;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReuseCandidate {
    pub candidate_id: String,
    pub domain_id: String,
    pub objective_type: String,
    pub constraint_types: Vec<String>,
    pub evaluator_summary: Vec<String>,
    pub evidence_summary: Vec<String>,
    pub timestamp_reference: String,
    pub reuse_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReuseCriteria {
    pub domain_id: String,
    pub objective_type: String,
    pub required_constraint_types: Vec<String>,
}

pub fn discover_reusable_candidates(
    criteria: &ReuseCriteria,
    ledger: &InMemoryLedger,
) -> Result<Vec<ReuseCandidate>, ReuseError> {
    if criteria.domain_id.is_empty() {
        return Err(ReuseError::MissingDomainId);
    }
    if criteria.objective_type.is_empty() {
        return Err(ReuseError::MissingObjectiveType);
    }
    if criteria.required_constraint_types.is_empty()
        || criteria
            .required_constraint_types
            .iter()
            .any(|value| value.is_empty())
    {
        return Err(ReuseError::InvalidConstraintCriteria);
    }

    let mut out = Vec::new();

    for (index, entry) in ledger.entries().iter().enumerate() {
        if let LedgerEntry::CandidateCreated(record) = entry {
            if record.domain_id != criteria.domain_id || record.objective_id != criteria.objective_type
            {
                continue;
            }
            if !criteria
                .required_constraint_types
                .iter()
                .any(|value| value == &record.constraints_id)
            {
                continue;
            }

            out.push(ReuseCandidate {
                candidate_id: record.candidate_id.clone(),
                domain_id: record.domain_id.clone(),
                objective_type: record.objective_id.clone(),
                constraint_types: vec![record.constraints_id.clone()],
                evaluator_summary: Vec::new(),
                evidence_summary: Vec::new(),
                timestamp_reference: format!("ledger_index:{index}"),
                reuse_reason: "matches_explicit_criteria".into(),
            });
        }
    }

    Ok(out)
}

pub fn build_reuse_applied_record(
    reuse_event_id: &str,
    reused_candidate_id: &str,
    target_candidate_id: &str,
    reuse_reason: &str,
    triggering_actor: &str,
    timestamp_reference: &str,
) -> Result<ReuseAppliedLedgerRecord, ReuseError> {
    if reused_candidate_id.is_empty() || target_candidate_id.is_empty() {
        return Err(ReuseError::MissingCandidateId);
    }
    if reuse_reason.is_empty() || triggering_actor.is_empty() || timestamp_reference.is_empty() {
        return Err(ReuseError::InvalidConstraintCriteria);
    }

    Ok(ReuseAppliedLedgerRecord {
        reuse_event_id: reuse_event_id.to_string(),
        reused_candidate_id: reused_candidate_id.to_string(),
        target_candidate_id: target_candidate_id.to_string(),
        reuse_reason: reuse_reason.to_string(),
        triggering_actor: triggering_actor.to_string(),
        timestamp_reference: timestamp_reference.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::candidate::lifecycle::CandidateLifecycleState;
    use crate::evaluation::result::EvaluationStatus;
    use crate::governance::promotion::PromotionStatus;
    use crate::governance::runtime::GovernanceStatus;
    use crate::ledger::entry::*;

    fn created(candidate_id: &str, objective_id: &str) -> LedgerEntry {
        LedgerEntry::CandidateCreated(CandidateCreatedLedgerRecord {
            candidate_id: candidate_id.into(),
            run_id: format!("run-{candidate_id}"),
            objective_id: objective_id.into(),
            constraints_id: "constraint-a".into(),
            domain_id: "domain-1".into(),
        })
    }

    #[test]
    fn deterministic_discovery_stable_ordering() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created("cand-1", "objective-a")).unwrap();
        ledger.append(created("cand-2", "objective-a")).unwrap();

        let criteria = ReuseCriteria {
            domain_id: "domain-1".into(),
            objective_type: "objective-a".into(),
            required_constraint_types: vec!["constraint-a".into()],
        };

        let first = discover_reusable_candidates(&criteria, &ledger).unwrap();
        let second = discover_reusable_candidates(&criteria, &ledger).unwrap();

        assert_eq!(first, second);
        assert_eq!(first[0].candidate_id, "cand-1");
        assert_eq!(first[1].candidate_id, "cand-2");
    }

    #[test]
    fn invalid_criteria_rejected() {
        let ledger = InMemoryLedger::new();
        let criteria = ReuseCriteria {
            domain_id: "".into(),
            objective_type: "objective-a".into(),
            required_constraint_types: vec!["constraint-a".into()],
        };

        assert_eq!(
            discover_reusable_candidates(&criteria, &ledger),
            Err(ReuseError::MissingDomainId)
        );
    }

    #[test]
    fn reuse_applied_record_uses_explicit_target_candidate() {
        let record = build_reuse_applied_record(
            "reuse-1",
            "cand-1",
            "cand-2",
            "manual",
            "operator",
            "ts-1",
        )
        .unwrap();

        assert_eq!(record.reuse_event_id, "reuse-1");
        assert_eq!(record.reused_candidate_id, "cand-1");
        assert_eq!(record.target_candidate_id, "cand-2");
    }

    #[test]
    fn reuse_does_not_change_lifecycle_promotion_or_governance() {
        let mut ledger = InMemoryLedger::new();
        ledger.append(created("cand-1", "objective-a")).unwrap();
        ledger.append(created("cand-2", "objective-a")).unwrap();
        ledger
            .append(LedgerEntry::EvaluationRecorded(
                EvaluationRecordedLedgerRecord {
                    evaluation_result_id: "eval-1".into(),
                    candidate_id: "cand-1".into(),
                    evaluator_id: "e-1".into(),
                    status: EvaluationStatus::Unknown,
                    evidence_ref: "ev://e1".into(),
                },
            ))
            .unwrap();
        ledger
            .append(LedgerEntry::GovernanceReviewed(
                GovernanceReviewedLedgerRecord {
                    governance_result_id: "gov-1".into(),
                    candidate_id: "cand-1".into(),
                    status: GovernanceStatus::Unknown,
                    required_evaluators_satisfied: false,
                    evidence_refs: vec!["ev://g1".into()],
                    blocked_reasons: vec![],
                    failure_reasons: vec![],
                },
            ))
            .unwrap();
        ledger
            .append(LedgerEntry::PromotionDecided(
                PromotionDecidedLedgerRecord {
                    promotion_decision_id: "prom-1".into(),
                    candidate_id: "cand-1".into(),
                    promotion_status: PromotionStatus::Denied,
                    from_state: CandidateLifecycleState::Passed,
                    to_state: CandidateLifecycleState::Passed,
                    required_checks_passed: true,
                    evidence_refs: vec!["ev://p1".into()],
                    denial_reasons: vec!["unknown".into()],
                },
            ))
            .unwrap();

        let record =
            build_reuse_applied_record("reuse-1", "cand-1", "cand-2", "manual", "operator", "ts-1")
                .unwrap();
        ledger.append(LedgerEntry::ReuseApplied(record)).unwrap();

        let replay = crate::replay::replay_candidate("cand-1", &ledger).unwrap();
        assert_eq!(
            replay.final_status,
            crate::replay::ReplayFinalStatus::Unknown
        );
        assert_eq!(replay.promotion_decision_ids, vec!["prom-1"]);
        assert_eq!(replay.reuse_event_ids, vec!["reuse-1"]);
    }
}
