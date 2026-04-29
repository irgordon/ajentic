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
