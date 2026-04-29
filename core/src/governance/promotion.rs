//! Phase 7 Rust-only promotion decisions.

use crate::candidate::contract::CandidateRecord;
use crate::candidate::lifecycle::{transition, CandidateLifecycleState};
use crate::errors::PromotionError;
use crate::policy::engine::PolicyCheckStatus;

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

    if governance
        .policy_checks
        .iter()
        .any(|check| check.status != PolicyCheckStatus::Pass)
    {
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
    use crate::policy::engine::PolicyCheckResult;

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

    fn policy_check(status: PolicyCheckStatus) -> PolicyCheckResult {
        PolicyCheckResult {
            id: "policy::cand-1::policy-1".into(),
            policy_check_id: "policy-1".into(),
            status,
            evidence_ref: "ev://policy-1".into(),
            failure_reasons: Vec::new(),
        }
    }

    fn gov(status: GovernanceStatus) -> GovernanceResultRecord {
        GovernanceResultRecord {
            id: "governance::cand-1".into(),
            candidate_id: "cand-1".into(),
            status,
            required_evaluators_satisfied: true,
            policy_checks: vec![policy_check(PolicyCheckStatus::Pass)],
            evidence_refs: vec!["ev://1".into()],
            blocked_reasons: vec![],
            failure_reasons: vec![],
        }
    }

    #[test]
    fn passed_candidate_with_passing_governance_promotes() {
        let (candidate, decision) = promote_candidate(
            &candidate(CandidateLifecycleState::Passed),
            &gov(GovernanceStatus::Pass),
        )
        .unwrap();

        assert_eq!(
            candidate.lifecycle_state,
            CandidateLifecycleState::PromotedTier1
        );
        assert_eq!(decision.promotion_status, PromotionStatus::Approved);
        assert_eq!(decision.from_state, CandidateLifecycleState::Passed);
        assert_eq!(
            decision.to_state,
            CandidateLifecycleState::PromotedTier1
        );
        assert_eq!(decision.evidence_refs, vec!["ev://1"]);
    }

    #[test]
    fn created_candidate_cannot_promote() {
        assert_eq!(
            promote_candidate(
                &candidate(CandidateLifecycleState::Created),
                &gov(GovernanceStatus::Pass),
            ),
            Err(PromotionError::CandidateNotPassed {
                state: CandidateLifecycleState::Created,
            })
        );
    }

    #[test]
    fn evaluating_candidate_cannot_promote() {
        assert_eq!(
            promote_candidate(
                &candidate(CandidateLifecycleState::Evaluating),
                &gov(GovernanceStatus::Pass),
            ),
            Err(PromotionError::CandidateNotPassed {
                state: CandidateLifecycleState::Evaluating,
            })
        );
    }

    #[test]
    fn failed_candidate_cannot_promote() {
        assert_eq!(
            promote_candidate(
                &candidate(CandidateLifecycleState::Failed),
                &gov(GovernanceStatus::Pass),
            ),
            Err(PromotionError::CandidateNotPassed {
                state: CandidateLifecycleState::Failed,
            })
        );
    }

    #[test]
    fn blocked_candidate_cannot_promote() {
        assert_eq!(
            promote_candidate(
                &candidate(CandidateLifecycleState::Blocked),
                &gov(GovernanceStatus::Pass),
            ),
            Err(PromotionError::CandidateNotPassed {
                state: CandidateLifecycleState::Blocked,
            })
        );
    }

    #[test]
    fn rejected_candidate_cannot_promote() {
        assert_eq!(
            promote_candidate(
                &candidate(CandidateLifecycleState::Rejected),
                &gov(GovernanceStatus::Pass),
            ),
            Err(PromotionError::CandidateNotPassed {
                state: CandidateLifecycleState::Rejected,
            })
        );
    }

    #[test]
    fn unknown_candidate_cannot_promote() {
        assert_eq!(
            promote_candidate(
                &candidate(CandidateLifecycleState::Unknown),
                &gov(GovernanceStatus::Pass),
            ),
            Err(PromotionError::CandidateNotPassed {
                state: CandidateLifecycleState::Unknown,
            })
        );
    }

    #[test]
    fn governance_fail_denies_promotion() {
        assert_eq!(
            promote_candidate(
                &candidate(CandidateLifecycleState::Passed),
                &gov(GovernanceStatus::Fail),
            ),
            Err(PromotionError::GovernanceNotPassed {
                status: GovernanceStatus::Fail,
            })
        );
    }

    #[test]
    fn governance_blocked_denies_promotion() {
        assert_eq!(
            promote_candidate(
                &candidate(CandidateLifecycleState::Passed),
                &gov(GovernanceStatus::Blocked),
            ),
            Err(PromotionError::GovernanceNotPassed {
                status: GovernanceStatus::Blocked,
            })
        );
    }

    #[test]
    fn governance_unknown_denies_promotion() {
        assert_eq!(
            promote_candidate(
                &candidate(CandidateLifecycleState::Passed),
                &gov(GovernanceStatus::Unknown),
            ),
            Err(PromotionError::GovernanceNotPassed {
                status: GovernanceStatus::Unknown,
            })
        );
    }

    #[test]
    fn unsatisfied_required_evaluators_deny_promotion() {
        let mut governance = gov(GovernanceStatus::Pass);
        governance.required_evaluators_satisfied = false;

        assert_eq!(
            promote_candidate(&candidate(CandidateLifecycleState::Passed), &governance),
            Err(PromotionError::RequiredChecksNotPassed)
        );
    }

    #[test]
    fn missing_evidence_refs_deny_promotion() {
        let mut governance = gov(GovernanceStatus::Pass);
        governance.evidence_refs.clear();

        assert_eq!(
            promote_candidate(&candidate(CandidateLifecycleState::Passed), &governance),
            Err(PromotionError::MissingEvidenceRefs)
        );
    }

    #[test]
    fn candidate_id_mismatch_denies_promotion() {
        let mut governance = gov(GovernanceStatus::Pass);
        governance.candidate_id = "other-candidate".into();

        assert_eq!(
            promote_candidate(&candidate(CandidateLifecycleState::Passed), &governance),
            Err(PromotionError::CandidateIdMismatch {
                expected: "cand-1".into(),
                found: "other-candidate".into(),
            })
        );
    }

    #[test]
    fn failed_policy_check_denies_promotion() {
        let mut governance = gov(GovernanceStatus::Pass);
        governance.policy_checks = vec![policy_check(PolicyCheckStatus::Fail)];

        assert_eq!(
            promote_candidate(&candidate(CandidateLifecycleState::Passed), &governance),
            Err(PromotionError::RequiredChecksNotPassed)
        );
    }

    #[test]
    fn blocked_policy_check_denies_promotion() {
        let mut governance = gov(GovernanceStatus::Pass);
        governance.policy_checks = vec![policy_check(PolicyCheckStatus::Blocked)];

        assert_eq!(
            promote_candidate(&candidate(CandidateLifecycleState::Passed), &governance),
            Err(PromotionError::RequiredChecksNotPassed)
        );
    }

    #[test]
    fn unknown_policy_check_denies_promotion() {
        let mut governance = gov(GovernanceStatus::Pass);
        governance.policy_checks = vec![policy_check(PolicyCheckStatus::Unknown)];

        assert_eq!(
            promote_candidate(&candidate(CandidateLifecycleState::Passed), &governance),
            Err(PromotionError::RequiredChecksNotPassed)
        );
    }
}
