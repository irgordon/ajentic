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
        LedgerEntry::ReuseApplied(record) => {
            if record.reuse_event_id.is_empty() {
                return Err(LedgerAppendError::MissingReuseEventId);
            }
            if record.reused_candidate_id.is_empty() {
                return Err(LedgerAppendError::MissingReusedCandidateId);
            }
            if record.target_candidate_id.is_empty() {
                return Err(LedgerAppendError::MissingReuseTargetCandidateId);
            }
            if record.reuse_reason.is_empty() {
                return Err(LedgerAppendError::MissingReuseReason);
            }
            if record.triggering_actor.is_empty() {
                return Err(LedgerAppendError::MissingReuseTriggeringActor);
            }
            if record.timestamp_reference.is_empty() {
                return Err(LedgerAppendError::MissingReuseTimestampReference);
            }
        }
    }
    Ok(())
}

pub fn validate_order(
    existing: &[LedgerEntry],
    entry: &LedgerEntry,
) -> Result<(), LedgerAppendError> {
    match entry {
        LedgerEntry::CandidateCreated(_) => Ok(()),
        LedgerEntry::EvaluationRecorded(record) => {
            if !candidate_exists(existing, &record.candidate_id) {
                return Err(LedgerAppendError::CandidateEntryMissing {
                    candidate_id: record.candidate_id.clone(),
                });
            }
            Ok(())
        }
        LedgerEntry::GovernanceReviewed(record) => {
            if !candidate_exists(existing, &record.candidate_id) {
                return Err(LedgerAppendError::CandidateEntryMissing {
                    candidate_id: record.candidate_id.clone(),
                });
            }
            Ok(())
        }
        LedgerEntry::PromotionDecided(record) => {
            if !candidate_exists(existing, &record.candidate_id) {
                return Err(LedgerAppendError::CandidateEntryMissing {
                    candidate_id: record.candidate_id.clone(),
                });
            }
            if !governance_exists(existing, &record.candidate_id) {
                return Err(LedgerAppendError::GovernanceEntryMissing {
                    candidate_id: record.candidate_id.clone(),
                });
            }
            if record.promotion_status == PromotionStatus::Approved {
                if !evaluation_exists(existing, &record.candidate_id) {
                    return Err(LedgerAppendError::EvaluationEntryMissing {
                        candidate_id: record.candidate_id.clone(),
                    });
                }
                if !passing_governance_exists(existing, &record.candidate_id) {
                    return Err(LedgerAppendError::GovernanceNotPassed {
                        candidate_id: record.candidate_id.clone(),
                    });
                }
            }
            Ok(())
        }
        LedgerEntry::ReuseApplied(record) => {
            if !candidate_exists(existing, &record.reused_candidate_id) {
                return Err(LedgerAppendError::ReuseSourceCandidateEntryMissing {
                    candidate_id: record.reused_candidate_id.clone(),
                });
            }
            if !candidate_exists(existing, &record.target_candidate_id) {
                return Err(LedgerAppendError::ReuseTargetCandidateEntryMissing {
                    candidate_id: record.target_candidate_id.clone(),
                });
            }
            Ok(())
        }
    }
}

fn candidate_exists(existing: &[LedgerEntry], candidate_id: &str) -> bool {
    existing.iter().any(
        |entry| matches!(entry, LedgerEntry::CandidateCreated(record) if record.candidate_id == candidate_id),
    )
}

fn evaluation_exists(existing: &[LedgerEntry], candidate_id: &str) -> bool {
    existing.iter().any(
        |entry| matches!(entry, LedgerEntry::EvaluationRecorded(record) if record.candidate_id == candidate_id),
    )
}

fn governance_exists(existing: &[LedgerEntry], candidate_id: &str) -> bool {
    existing.iter().any(
        |entry| matches!(entry, LedgerEntry::GovernanceReviewed(record) if record.candidate_id == candidate_id),
    )
}

fn passing_governance_exists(existing: &[LedgerEntry], candidate_id: &str) -> bool {
    existing.iter().any(
        |entry| matches!(entry, LedgerEntry::GovernanceReviewed(record)
            if record.candidate_id == candidate_id && record.status == GovernanceStatus::Pass),
    )
}
