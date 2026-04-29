//! Phase 7 Rust-only promotion decisions.

use crate::candidate::contract::CandidateRecord;
use crate::candidate::lifecycle::{transition, CandidateLifecycleState};
use crate::errors::PromotionError;

use super::runtime::{GovernanceResultRecord, GovernanceStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromotionStatus {
    Approved,
    Denied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromotionDecisionRecord {
    pub id: String,
    pub candidate_id: String,
    pub from_state: CandidateLifecycleState,
    pub to_state: CandidateLifecycleState,
    pub promotion_status: PromotionStatus,
    pub required_checks_passed: bool,
    pub evidence_refs: Vec<String>,
    pub denial_reasons: Vec<String>,
}

pub fn decide_promotion(
    candidate: &CandidateRecord,
    governance: &GovernanceResultRecord,
) -> Result<PromotionDecisionRecord, PromotionError> {
    if governance.candidate_id != candidate.id {
        return Err(PromotionError::CandidateIdMismatch {
            expected: candidate.id.clone(),
            found: governance.candidate_id.clone(),
        });
    }
    if governance.evidence_refs.is_empty() {
        return Err(PromotionError::MissingEvidenceRefs);
    }
    if candidate.lifecycle_state != CandidateLifecycleState::Passed {
        return Err(PromotionError::CandidateNotPassed {
            state: candidate.lifecycle_state,
        });
    }
    if governance.status != GovernanceStatus::Pass {
        return Err(PromotionError::GovernanceNotPassed {
            status: governance.status.clone(),
        });
    }
    if !governance.required_evaluators_satisfied {
        return Err(PromotionError::RequiredChecksNotPassed);
    }

    Ok(PromotionDecisionRecord {
        id: format!("promotion::{}", candidate.id),
        candidate_id: candidate.id.clone(),
        from_state: CandidateLifecycleState::Passed,
        to_state: CandidateLifecycleState::PromotedTier1,
        promotion_status: PromotionStatus::Approved,
        required_checks_passed: true,
        evidence_refs: governance.evidence_refs.clone(),
        denial_reasons: Vec::new(),
    })
}

pub fn promote_candidate(
    candidate: &CandidateRecord,
    governance: &GovernanceResultRecord,
) -> Result<(CandidateRecord, PromotionDecisionRecord), PromotionError> {
    let decision = decide_promotion(candidate, governance)?;
    let new_state = transition(
        candidate.lifecycle_state,
        CandidateLifecycleState::PromotedTier1,
    )
    .map_err(|_| PromotionError::InvalidPromotionTransition)?;
    let mut updated = candidate.clone();
    updated.lifecycle_state = new_state;
    Ok((updated, decision))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::candidate::lifecycle::CandidateLifecycleState;
    use crate::execution::adapter_protocol::AdapterStatus;

    fn candidate(state: CandidateLifecycleState) -> CandidateRecord {
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
            lifecycle_state: state,
        }
    }
    fn gov(status: GovernanceStatus) -> GovernanceResultRecord {
        GovernanceResultRecord {
            id: "governance::cand-1".into(),
            candidate_id: "cand-1".into(),
            status,
            required_evaluators_satisfied: true,
            policy_checks: vec![],
            evidence_refs: vec!["ev://1".into()],
            blocked_reasons: vec![],
            failure_reasons: vec![],
        }
    }

    #[test]
    fn passed_promotes() {
        let (c, d) = promote_candidate(
            &candidate(CandidateLifecycleState::Passed),
            &gov(GovernanceStatus::Pass),
        )
        .unwrap();
        assert_eq!(c.lifecycle_state, CandidateLifecycleState::PromotedTier1);
        assert_eq!(d.promotion_status, PromotionStatus::Approved);
    }
    #[test]
    fn created_cannot_promote() {
        assert_eq!(
            promote_candidate(
                &candidate(CandidateLifecycleState::Created),
                &gov(GovernanceStatus::Pass)
            ),
            Err(PromotionError::CandidateNotPassed {
                state: CandidateLifecycleState::Created
            })
        );
    }
    #[test]
    fn governance_fail_denies() {
        assert_eq!(
            promote_candidate(
                &candidate(CandidateLifecycleState::Passed),
                &gov(GovernanceStatus::Fail)
            ),
            Err(PromotionError::GovernanceNotPassed {
                status: GovernanceStatus::Fail
            })
        );
    }
}
