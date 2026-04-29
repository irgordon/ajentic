use crate::errors::AuditError;
use crate::governance::promotion::PromotionStatus;
use crate::governance::runtime::GovernanceStatus;
use crate::ledger::append::InMemoryLedger;
use crate::ledger::entry::LedgerEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditFinalStatus {
    Promoted,
    Denied,
    Failed,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditRecord {
    pub candidate_id: String,
    pub run_id: String,
    pub objective_id: String,
    pub constraints_id: String,
    pub domain_id: String,
    pub evaluation_result_ids: Vec<String>,
    pub governance_result_ids: Vec<String>,
    pub promotion_decision_ids: Vec<String>,
    pub final_status: AuditFinalStatus,
    pub evidence_refs: Vec<String>,
    pub blocked_reasons: Vec<String>,
    pub failure_reasons: Vec<String>,
}

pub fn build_audit_record(
    candidate_id: &str,
    ledger: &InMemoryLedger,
) -> Result<AuditRecord, AuditError> {
    if candidate_id.is_empty() {
        return Err(AuditError::MissingCandidateId);
    }

    let candidate = ledger
        .entries()
        .iter()
        .find_map(|entry| match entry {
            LedgerEntry::CandidateCreated(record) if record.candidate_id == candidate_id => {
                Some(record)
            }
            _ => None,
        })
        .ok_or_else(|| AuditError::CandidateEntryMissing {
            candidate_id: candidate_id.to_string(),
        })?;

    if candidate.run_id.is_empty() {
        return Err(AuditError::MissingRunId {
            candidate_id: candidate_id.to_string(),
        });
    }
    if candidate.objective_id.is_empty() {
        return Err(AuditError::MissingObjectiveId {
            candidate_id: candidate_id.to_string(),
        });
    }
    if candidate.constraints_id.is_empty() {
        return Err(AuditError::MissingConstraintsId {
            candidate_id: candidate_id.to_string(),
        });
    }
    if candidate.domain_id.is_empty() {
        return Err(AuditError::MissingDomainId {
            candidate_id: candidate_id.to_string(),
        });
    }

    let mut evaluation_result_ids = Vec::new();
    let mut governance_result_ids = Vec::new();
    let mut promotion_decision_ids = Vec::new();
    let mut evidence_refs = Vec::new();
    let mut blocked_reasons = Vec::new();
    let mut failure_reasons = Vec::new();
    let mut last_governance_status = None;
    let mut last_promotion_status = None;
    let mut last_promotion_evidence_count = 0usize;

    for entry in ledger.entries() {
        match entry {
            LedgerEntry::EvaluationRecorded(record) if record.candidate_id == candidate_id => {
                evaluation_result_ids.push(record.evaluation_result_id.clone());
                evidence_refs.push(record.evidence_ref.clone());
            }
            LedgerEntry::GovernanceReviewed(record) if record.candidate_id == candidate_id => {
                governance_result_ids.push(record.governance_result_id.clone());
                evidence_refs.extend(record.evidence_refs.clone());
                blocked_reasons.extend(record.blocked_reasons.clone());
                failure_reasons.extend(record.failure_reasons.clone());
                last_governance_status = Some(record.status.clone());
            }
            LedgerEntry::PromotionDecided(record) if record.candidate_id == candidate_id => {
                promotion_decision_ids.push(record.promotion_decision_id.clone());
                evidence_refs.extend(record.evidence_refs.clone());
                blocked_reasons.extend(record.denial_reasons.clone());
                failure_reasons.extend(record.denial_reasons.clone());
                last_promotion_status = Some(record.promotion_status.clone());
                last_promotion_evidence_count = record.evidence_refs.len();
            }
            _ => {}
        }
    }

    if governance_result_ids.is_empty() {
        return Err(AuditError::GovernanceEntryMissing {
            candidate_id: candidate_id.to_string(),
        });
    }
    if promotion_decision_ids.is_empty() {
        return Err(AuditError::PromotionEntryMissing {
            candidate_id: candidate_id.to_string(),
        });
    }

    let final_status = match last_promotion_status {
        Some(PromotionStatus::Approved) => AuditFinalStatus::Promoted,
        Some(PromotionStatus::Denied) => match last_governance_status {
            Some(GovernanceStatus::Fail) => AuditFinalStatus::Failed,
            Some(GovernanceStatus::Blocked) => AuditFinalStatus::Blocked,
            Some(GovernanceStatus::Unknown) => AuditFinalStatus::Unknown,
            _ => AuditFinalStatus::Denied,
        },
        None => AuditFinalStatus::Unknown,
    };

    if final_status == AuditFinalStatus::Promoted {
        if evaluation_result_ids.is_empty() {
            return Err(AuditError::EvaluationEntryMissing {
                candidate_id: candidate_id.to_string(),
            });
        }
        if governance_result_ids.is_empty() {
            return Err(AuditError::MissingGovernanceEvidence {
                candidate_id: candidate_id.to_string(),
            });
        }
        if promotion_decision_ids.is_empty() {
            return Err(AuditError::MissingPromotionEvidence {
                candidate_id: candidate_id.to_string(),
            });
        }
        if last_promotion_evidence_count == 0 {
            return Err(AuditError::MissingPromotionEvidence {
                candidate_id: candidate_id.to_string(),
            });
        }
    }

    Ok(AuditRecord {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::candidate::lifecycle::CandidateLifecycleState;
    use crate::evaluation::result::EvaluationStatus;
    use crate::governance::promotion::PromotionStatus;
    use crate::governance::runtime::GovernanceStatus;
    use crate::ledger::append::InMemoryLedger;
    use crate::ledger::entry::*;

    fn promoted_ledger() -> InMemoryLedger {
        let mut l = InMemoryLedger::new();
        l.append(LedgerEntry::CandidateCreated(
            CandidateCreatedLedgerRecord {
                candidate_id: "cand-1".into(),
                run_id: "run-1".into(),
                objective_id: "obj-1".into(),
                constraints_id: "con-1".into(),
                domain_id: "dom-1".into(),
            },
        ))
        .unwrap();
        l.append(LedgerEntry::EvaluationRecorded(
            EvaluationRecordedLedgerRecord {
                evaluation_result_id: "eval-1".into(),
                candidate_id: "cand-1".into(),
                evaluator_id: "e-1".into(),
                status: EvaluationStatus::Pass,
                evidence_ref: "ev://e1".into(),
            },
        ))
        .unwrap();
        l.append(LedgerEntry::GovernanceReviewed(
            GovernanceReviewedLedgerRecord {
                governance_result_id: "gov-1".into(),
                candidate_id: "cand-1".into(),
                status: GovernanceStatus::Pass,
                required_evaluators_satisfied: true,
                evidence_refs: vec!["ev://g1".into()],
                blocked_reasons: vec![],
                failure_reasons: vec![],
            },
        ))
        .unwrap();
        l.append(LedgerEntry::PromotionDecided(
            PromotionDecidedLedgerRecord {
                promotion_decision_id: "prom-1".into(),
                candidate_id: "cand-1".into(),
                promotion_status: PromotionStatus::Approved,
                from_state: CandidateLifecycleState::Passed,
                to_state: CandidateLifecycleState::PromotedTier1,
                required_checks_passed: true,
                evidence_refs: vec!["ev://p1".into()],
                denial_reasons: vec![],
            },
        ))
        .unwrap();
        l
    }

    #[test]
    fn builds_promoted_audit() {
        let r = build_audit_record("cand-1", &promoted_ledger()).unwrap();
        assert_eq!(r.final_status, AuditFinalStatus::Promoted);
        assert!(!r.evidence_refs.is_empty());
    }
    #[test]
    fn missing_candidate_id_fails() {
        assert_eq!(
            build_audit_record("", &promoted_ledger()),
            Err(AuditError::MissingCandidateId)
        );
    }
}
