use crate::candidate::lifecycle::CandidateLifecycleState;
use crate::errors::LedgerAppendError;
use crate::governance::promotion::PromotionStatus;
use crate::governance::runtime::GovernanceStatus;
use crate::ledger::entry::LedgerEntry;

pub fn validate_entry_fields(entry: &LedgerEntry) -> Result<(), LedgerAppendError> {
    match entry {
        LedgerEntry::CandidateCreated(record) => {
            if record.candidate_id.is_empty() {
                return Err(LedgerAppendError::MissingCandidateId);
            }
            if record.run_id.is_empty() {
                return Err(LedgerAppendError::MissingRunId);
            }
            if record.objective_id.is_empty() {
                return Err(LedgerAppendError::MissingObjectiveId);
            }
            if record.constraints_id.is_empty() {
                return Err(LedgerAppendError::MissingConstraintsId);
            }
            if record.domain_id.is_empty() {
                return Err(LedgerAppendError::MissingDomainId);
            }
        }
        LedgerEntry::EvaluationRecorded(record) => {
            if record.evaluation_result_id.is_empty() {
                return Err(LedgerAppendError::MissingEvaluationResultId);
            }
            if record.candidate_id.is_empty() {
                return Err(LedgerAppendError::MissingCandidateId);
            }
            if record.evaluator_id.is_empty() {
                return Err(LedgerAppendError::MissingEvaluatorId);
            }
            if record.evidence_ref.is_empty() {
                return Err(LedgerAppendError::MissingEvidenceRef);
            }
        }
        LedgerEntry::GovernanceReviewed(record) => {
            if record.governance_result_id.is_empty() {
                return Err(LedgerAppendError::MissingGovernanceResultId);
            }
            if record.candidate_id.is_empty() {
                return Err(LedgerAppendError::MissingCandidateId);
            }
            if record.evidence_refs.is_empty() {
                return Err(LedgerAppendError::MissingEvidenceRef);
            }
        }
        LedgerEntry::PromotionDecided(record) => {
            if record.promotion_decision_id.is_empty() {
                return Err(LedgerAppendError::MissingPromotionDecisionId);
            }
            if record.candidate_id.is_empty() {
                return Err(LedgerAppendError::MissingCandidateId);
            }
            if record.promotion_status == PromotionStatus::Approved {
                if record.evidence_refs.is_empty() {
                    return Err(LedgerAppendError::ApprovedPromotionMissingEvidence);
                }
                if !record.required_checks_passed {
                    return Err(LedgerAppendError::ApprovedPromotionMissingRequiredChecks);
                }
                if record.from_state != CandidateLifecycleState::Passed
                    || record.to_state != CandidateLifecycleState::PromotedTier1
                {
                    return Err(LedgerAppendError::InvalidApprovedPromotionTransition);
                }
            }
        }
    }
    Ok(())
}

pub fn validate_order(
    existing: &[LedgerEntry],
    entry: &LedgerEntry,
) -> Result<(), LedgerAppendError> {
    let candidate_id = match entry {
        LedgerEntry::CandidateCreated(r) => &r.candidate_id,
        LedgerEntry::EvaluationRecorded(r) => &r.candidate_id,
        LedgerEntry::GovernanceReviewed(r) => &r.candidate_id,
        LedgerEntry::PromotionDecided(r) => &r.candidate_id,
    };

    let has_candidate = existing
        .iter()
        .any(|e| matches!(e, LedgerEntry::CandidateCreated(r) if r.candidate_id == *candidate_id));
    let has_evaluation = existing.iter().any(
        |e| matches!(e, LedgerEntry::EvaluationRecorded(r) if r.candidate_id == *candidate_id),
    );
    let has_governance = existing.iter().any(
        |e| matches!(e, LedgerEntry::GovernanceReviewed(r) if r.candidate_id == *candidate_id),
    );
    let has_passing_governance = existing.iter().any(|e| matches!(e, LedgerEntry::GovernanceReviewed(r) if r.candidate_id == *candidate_id && r.status == GovernanceStatus::Pass));

    match entry {
        LedgerEntry::CandidateCreated(_) => Ok(()),
        LedgerEntry::EvaluationRecorded(_) => {
            if !has_candidate {
                return Err(LedgerAppendError::CandidateEntryMissing {
                    candidate_id: candidate_id.clone(),
                });
            }
            Ok(())
        }
        LedgerEntry::GovernanceReviewed(_) => {
            if !has_candidate {
                return Err(LedgerAppendError::CandidateEntryMissing {
                    candidate_id: candidate_id.clone(),
                });
            }
            Ok(())
        }
        LedgerEntry::PromotionDecided(record) => {
            if !has_candidate {
                return Err(LedgerAppendError::CandidateEntryMissing {
                    candidate_id: candidate_id.clone(),
                });
            }
            if !has_governance {
                return Err(LedgerAppendError::GovernanceEntryMissing {
                    candidate_id: candidate_id.clone(),
                });
            }
            if record.promotion_status == PromotionStatus::Approved {
                if !has_evaluation {
                    return Err(LedgerAppendError::EvaluationEntryMissing {
                        candidate_id: candidate_id.clone(),
                    });
                }
                if !has_passing_governance {
                    return Err(LedgerAppendError::GovernanceNotPassed {
                        candidate_id: candidate_id.clone(),
                    });
                }
            }
            Ok(())
        }
    }
}
