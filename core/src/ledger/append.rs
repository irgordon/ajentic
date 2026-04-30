use crate::errors::LedgerAppendError;
use crate::ledger::entry::LedgerEntry;
use crate::ledger::integrity::{validate_entry_fields, validate_order};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryLedger {
    entries: Vec<LedgerEntry>,
}

impl InMemoryLedger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn append(&mut self, entry: LedgerEntry) -> Result<(), LedgerAppendError> {
        validate_entry_fields(&entry)?;
        validate_order(&self.entries, &entry)?;
        self.entries.push(entry);
        Ok(())
    }

    pub fn entries(&self) -> &[LedgerEntry] {
        &self.entries
    }

    #[cfg(test)]
    /// Test-only corruption hook for replay and ledger negative-path tests.
    /// This must not be exposed in non-test builds.
    pub(crate) fn entries_mut_for_tests(&mut self) -> &mut Vec<LedgerEntry> {
        &mut self.entries
    }
}

impl Default for InMemoryLedger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::candidate::lifecycle::CandidateLifecycleState;
    use crate::evaluation::result::EvaluationStatus;
    use crate::governance::promotion::PromotionStatus;
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

    fn created_with_id(candidate_id: &str) -> LedgerEntry {
        LedgerEntry::CandidateCreated(CandidateCreatedLedgerRecord {
            candidate_id: candidate_id.into(),
            run_id: "run-1".into(),
            objective_id: "obj-1".into(),
            constraints_id: "con-1".into(),
            domain_id: "dom-1".into(),
        })
    }

    fn reuse_with_ids(source_candidate_id: &str, target_candidate_id: &str) -> LedgerEntry {
        LedgerEntry::ReuseApplied(ReuseAppliedLedgerRecord {
            reuse_event_id: "reuse-1".into(),
            reused_candidate_id: source_candidate_id.into(),
            target_candidate_id: target_candidate_id.into(),
            reuse_reason: "same objective type".into(),
            triggering_actor: "owner".into(),
            timestamp_reference: "manual-ref-1".into(),
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
            blocked_reasons: vec![],
            failure_reasons: vec![],
        })
    }

    #[test]
    fn preserves_order() {
        let mut l = InMemoryLedger::new();
        l.append(created()).unwrap();
        l.append(eval()).unwrap();
        assert!(matches!(l.entries()[0], LedgerEntry::CandidateCreated(_)));
    }
    #[test]
    fn rejects_eval_before_candidate() {
        let mut l = InMemoryLedger::new();
        assert!(matches!(
            l.append(eval()),
            Err(LedgerAppendError::CandidateEntryMissing { .. })
        ));
    }
    #[test]
    fn rejects_gov_before_candidate() {
        let mut l = InMemoryLedger::new();
        assert!(matches!(
            l.append(gov(GovernanceStatus::Pass)),
            Err(LedgerAppendError::CandidateEntryMissing { .. })
        ));
    }
    #[test]
    fn rejects_reuse_when_source_candidate_is_missing() {
        let mut l = InMemoryLedger::new();
        l.append(created_with_id("cand-2")).unwrap();

        assert_eq!(
            l.append(reuse_with_ids("cand-1", "cand-2")),
            Err(LedgerAppendError::ReuseSourceCandidateEntryMissing {
                candidate_id: "cand-1".into(),
            })
        );
    }

    #[test]
    fn rejects_reuse_when_target_candidate_is_missing() {
        let mut l = InMemoryLedger::new();
        l.append(created_with_id("cand-1")).unwrap();

        assert_eq!(
            l.append(reuse_with_ids("cand-1", "cand-2")),
            Err(LedgerAppendError::ReuseTargetCandidateEntryMissing {
                candidate_id: "cand-2".into(),
            })
        );
    }

    #[test]
    fn rejects_candidate_created_with_missing_candidate_id() {
        let mut l = InMemoryLedger::new();
        let entry = created_with_id("");

        assert_eq!(l.append(entry), Err(LedgerAppendError::MissingCandidateId));
    }

    #[test]
    fn rejects_approved_promotion_without_evaluation_fact() {
        let mut l = InMemoryLedger::new();
        l.append(created()).unwrap();
        l.append(gov(GovernanceStatus::Pass)).unwrap();

        let promotion = LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
            promotion_decision_id: "p1".into(),
            candidate_id: "cand-1".into(),
            promotion_status: PromotionStatus::Approved,
            from_state: CandidateLifecycleState::Passed,
            to_state: CandidateLifecycleState::PromotedTier1,
            required_checks_passed: true,
            evidence_refs: vec!["ev://p1".into()],
            denial_reasons: vec![],
        });

        assert_eq!(
            l.append(promotion),
            Err(LedgerAppendError::EvaluationEntryMissing {
                candidate_id: "cand-1".into(),
            })
        );
    }

    #[test]
    fn rejects_approved_promotion_without_passing_governance_fact() {
        let mut l = InMemoryLedger::new();
        l.append(created()).unwrap();
        l.append(eval()).unwrap();
        l.append(gov(GovernanceStatus::Fail)).unwrap();

        let promotion = LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
            promotion_decision_id: "p1".into(),
            candidate_id: "cand-1".into(),
            promotion_status: PromotionStatus::Approved,
            from_state: CandidateLifecycleState::Passed,
            to_state: CandidateLifecycleState::PromotedTier1,
            required_checks_passed: true,
            evidence_refs: vec!["ev://p1".into()],
            denial_reasons: vec![],
        });

        assert_eq!(
            l.append(promotion),
            Err(LedgerAppendError::GovernanceNotPassed {
                candidate_id: "cand-1".into(),
            })
        );
    }

    #[test]
    fn rejects_approved_promotion_without_evidence_refs() {
        let mut l = InMemoryLedger::new();
        l.append(created()).unwrap();
        l.append(eval()).unwrap();
        l.append(gov(GovernanceStatus::Pass)).unwrap();

        let promotion = LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
            promotion_decision_id: "p1".into(),
            candidate_id: "cand-1".into(),
            promotion_status: PromotionStatus::Approved,
            from_state: CandidateLifecycleState::Passed,
            to_state: CandidateLifecycleState::PromotedTier1,
            required_checks_passed: true,
            evidence_refs: vec![],
            denial_reasons: vec![],
        });

        assert_eq!(
            l.append(promotion),
            Err(LedgerAppendError::ApprovedPromotionMissingEvidence)
        );
    }

    #[test]
    fn approves_only_after_eval_and_pass_governance() {
        let mut l = InMemoryLedger::new();
        l.append(created()).unwrap();
        l.append(eval()).unwrap();
        l.append(gov(GovernanceStatus::Pass)).unwrap();
        let p = LedgerEntry::PromotionDecided(PromotionDecidedLedgerRecord {
            promotion_decision_id: "p1".into(),
            candidate_id: "cand-1".into(),
            promotion_status: PromotionStatus::Approved,
            from_state: CandidateLifecycleState::Passed,
            to_state: CandidateLifecycleState::PromotedTier1,
            required_checks_passed: true,
            evidence_refs: vec!["ev://p1".into()],
            denial_reasons: vec![],
        });
        l.append(p).unwrap();
    }
}
