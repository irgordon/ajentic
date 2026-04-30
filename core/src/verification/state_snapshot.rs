use crate::replay::ReplayResult;
use crate::replay::status::ReplayCompletionStatus;
use crate::replay::{
    ReplayCompletenessStatus, ReplayFinalStatus, ReplayIdempotenceStatus, ReplayIntegrityStatus,
    ReplayOrderingStabilityStatus, ReplayReadinessStatus,
};
use crate::verification::ArchitectureAlignmentVerificationResult;
use crate::verification::architecture_alignment::{
    ArchitectureAlignmentStatus, AuthorityBoundaryStatus, DeterminismVerificationStatus,
    FailClosedVerificationStatus,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerificationStateSnapshot {
    pub architecture_alignment_status: ArchitectureAlignmentStatus,
    pub authority_boundary_status: AuthorityBoundaryStatus,
    pub determinism_status: DeterminismVerificationStatus,
    pub fail_closed_status: FailClosedVerificationStatus,
    pub replay_ordering_stability_status: ReplayOrderingStabilityStatus,
    pub replay_idempotence_status: ReplayIdempotenceStatus,
    pub replay_completeness_status: ReplayCompletenessStatus,
    pub replay_integrity_status: ReplayIntegrityStatus,
    pub replay_readiness_status: ReplayReadinessStatus,
    pub replay_completion_status: ReplayCompletionStatus,
    pub replay_final_status: ReplayFinalStatus,
}

pub fn collect_verification_state_snapshot(
    architecture_alignment: &ArchitectureAlignmentVerificationResult,
    replay_result: &ReplayResult,
) -> VerificationStateSnapshot {
    VerificationStateSnapshot {
        architecture_alignment_status: architecture_alignment.architecture_alignment,
        authority_boundary_status: architecture_alignment.authority_boundary,
        determinism_status: architecture_alignment.determinism_verification,
        fail_closed_status: architecture_alignment.fail_closed_verification,
        replay_ordering_stability_status: replay_result.ordering_stability_status,
        replay_idempotence_status: replay_result.idempotence_status,
        replay_completeness_status: replay_result.completeness_status,
        replay_integrity_status: replay_result.integrity_status,
        replay_readiness_status: replay_result.readiness_status,
        replay_completion_status: replay_result.completion_status,
        replay_final_status: replay_result.final_status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::replay::{
        ReplayCompletenessStatus, ReplayIdempotenceStatus, ReplayOrderingStabilityStatus,
    };
    use crate::verification::{
        AlignmentEvidence, ArchitectureAlignmentVerification, verify_architecture_alignment,
    };

    fn baseline_alignment() -> ArchitectureAlignmentVerification {
        verify_architecture_alignment(&AlignmentEvidence {
            architecture_rules_present: true,
            boundary_rules_present: true,
            deterministic_replay_behavior_confirmed: true,
            fail_closed_behavior_confirmed: true,
            documentation_matches_implementation: true,
            unauthorized_non_rust_lifecycle_mutation: false,
            unauthorized_non_rust_promotion_authorization: false,
            unauthorized_state_transition_path: false,
            missing_required_facts_detected: false,
            malformed_inputs_detected: false,
            unknown_inputs_detected: false,
        })
    }

    fn baseline_replay_result() -> ReplayResult {
        ReplayResult {
            candidate_id: "c1".to_string(),
            run_id: "r1".to_string(),
            objective_id: "o1".to_string(),
            constraints_id: "k1".to_string(),
            domain_id: "d1".to_string(),
            evaluation_result_ids: vec!["e1".to_string()],
            governance_result_ids: vec!["g1".to_string()],
            promotion_decision_ids: vec!["p1".to_string()],
            reuse_event_ids: vec![],
            readiness_status: ReplayReadinessStatus::Ready,
            completion_status: ReplayCompletionStatus::Complete,
            final_status: ReplayFinalStatus::PromotedTier1,
            evidence_refs: vec!["ev1".to_string()],
            blocked_reasons: vec![],
            failure_reasons: vec![],
            ordering_stability_status: ReplayOrderingStabilityStatus::Stable,
            idempotence_status: ReplayIdempotenceStatus::Idempotent,
            completeness_status: ReplayCompletenessStatus::Complete,
            integrity_status: ReplayIntegrityStatus::Verified,
        }
    }

    #[test]
    fn snapshot_is_deterministic_for_identical_inputs() {
        let alignment = baseline_alignment();
        let replay_result = baseline_replay_result();

        let first = collect_verification_state_snapshot(&alignment, &replay_result);
        let second = collect_verification_state_snapshot(&alignment, &replay_result);

        assert_eq!(first, second);
    }

    #[test]
    fn snapshot_populates_all_required_fields() {
        let alignment = baseline_alignment();
        let replay_result = baseline_replay_result();

        let snapshot = collect_verification_state_snapshot(&alignment, &replay_result);

        assert_eq!(
            snapshot.architecture_alignment_status,
            alignment.architecture_alignment
        );
        assert_eq!(
            snapshot.authority_boundary_status,
            alignment.authority_boundary
        );
        assert_eq!(
            snapshot.determinism_status,
            alignment.determinism_verification
        );
        assert_eq!(
            snapshot.fail_closed_status,
            alignment.fail_closed_verification
        );
        assert_eq!(
            snapshot.replay_ordering_stability_status,
            replay_result.ordering_stability_status
        );
        assert_eq!(
            snapshot.replay_idempotence_status,
            replay_result.idempotence_status
        );
        assert_eq!(
            snapshot.replay_completeness_status,
            replay_result.completeness_status
        );
        assert_eq!(
            snapshot.replay_integrity_status,
            replay_result.integrity_status
        );
        assert_eq!(
            snapshot.replay_readiness_status,
            replay_result.readiness_status
        );
        assert_eq!(
            snapshot.replay_completion_status,
            replay_result.completion_status
        );
        assert_eq!(snapshot.replay_final_status, replay_result.final_status);
    }

    #[test]
    fn snapshot_creation_is_non_authoritative_and_read_only() {
        let alignment = baseline_alignment();
        let replay_result = baseline_replay_result();

        let alignment_before = alignment;
        let replay_before = replay_result.clone();

        let _snapshot = collect_verification_state_snapshot(&alignment, &replay_result);

        assert_eq!(alignment, alignment_before);
        assert_eq!(replay_result, replay_before);
    }

    #[test]
    fn snapshots_from_identical_inputs_are_equal() {
        let alignment = baseline_alignment();
        let replay_result = baseline_replay_result();

        let first_snapshot = collect_verification_state_snapshot(&alignment, &replay_result);
        let second_snapshot = collect_verification_state_snapshot(&alignment, &replay_result);

        assert_eq!(first_snapshot, second_snapshot);
    }
}
