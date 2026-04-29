//! Deterministic architecture alignment verification surface.
//!
//! This module is read-only and classification-only. It does not mutate
//! lifecycle, governance, promotion, replay, or ledger authority.

pub mod architecture_alignment;

use architecture_alignment::{
    ArchitectureAlignmentStatus, AuthorityBoundaryStatus, DeterminismVerificationStatus,
    DocumentationAlignmentStatus, FailClosedVerificationStatus, OverallAlignmentStatus,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlignmentEvidence {
    pub architecture_rules_present: bool,
    pub boundary_rules_present: bool,
    pub deterministic_replay_behavior_confirmed: bool,
    pub fail_closed_behavior_confirmed: bool,
    pub documentation_matches_implementation: bool,
    pub unauthorized_non_rust_lifecycle_mutation: bool,
    pub unauthorized_non_rust_promotion_authorization: bool,
    pub unauthorized_state_transition_path: bool,
    pub missing_required_facts_detected: bool,
    pub malformed_inputs_detected: bool,
    pub unknown_inputs_detected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchitectureAlignmentVerification {
    pub architecture_alignment: ArchitectureAlignmentStatus,
    pub authority_boundary: AuthorityBoundaryStatus,
    pub determinism_verification: DeterminismVerificationStatus,
    pub fail_closed_verification: FailClosedVerificationStatus,
    pub documentation_alignment: DocumentationAlignmentStatus,
    pub overall: OverallAlignmentStatus,
}

pub fn verify_architecture_alignment(
    evidence: &AlignmentEvidence,
) -> ArchitectureAlignmentVerification {
    let architecture_alignment = evaluate_architecture_alignment(evidence);
    let authority_boundary = evaluate_authority_boundary(evidence);
    let determinism_verification = evaluate_determinism(evidence);
    let fail_closed_verification = evaluate_fail_closed(evidence);
    let documentation_alignment = evaluate_documentation_alignment(evidence);
    let overall = evaluate_overall(
        &architecture_alignment,
        &authority_boundary,
        &determinism_verification,
        &fail_closed_verification,
        &documentation_alignment,
    );

    ArchitectureAlignmentVerification {
        architecture_alignment,
        authority_boundary,
        determinism_verification,
        fail_closed_verification,
        documentation_alignment,
        overall,
    }
}

fn evaluate_architecture_alignment(evidence: &AlignmentEvidence) -> ArchitectureAlignmentStatus {
    if !evidence.architecture_rules_present {
        return ArchitectureAlignmentStatus::Unknown;
    }

    if evidence.unauthorized_non_rust_lifecycle_mutation
        || evidence.unauthorized_non_rust_promotion_authorization
        || evidence.unauthorized_state_transition_path
    {
        return ArchitectureAlignmentStatus::DriftDetected;
    }

    ArchitectureAlignmentStatus::Aligned
}

fn evaluate_authority_boundary(evidence: &AlignmentEvidence) -> AuthorityBoundaryStatus {
    if !evidence.boundary_rules_present {
        return AuthorityBoundaryStatus::Unknown;
    }

    if evidence.unauthorized_non_rust_lifecycle_mutation
        || evidence.unauthorized_non_rust_promotion_authorization
        || evidence.unauthorized_state_transition_path
    {
        return AuthorityBoundaryStatus::ViolationDetected;
    }

    AuthorityBoundaryStatus::Preserved
}

fn evaluate_determinism(evidence: &AlignmentEvidence) -> DeterminismVerificationStatus {
    if !evidence.architecture_rules_present {
        return DeterminismVerificationStatus::Unknown;
    }

    if evidence.deterministic_replay_behavior_confirmed {
        return DeterminismVerificationStatus::Deterministic;
    }

    DeterminismVerificationStatus::NonDeterministic
}

fn evaluate_fail_closed(evidence: &AlignmentEvidence) -> FailClosedVerificationStatus {
    if !evidence.boundary_rules_present {
        return FailClosedVerificationStatus::Unknown;
    }

    if !evidence.fail_closed_behavior_confirmed {
        return FailClosedVerificationStatus::ViolationDetected;
    }

    if evidence.missing_required_facts_detected
        || evidence.malformed_inputs_detected
        || evidence.unknown_inputs_detected
    {
        return FailClosedVerificationStatus::Enforced;
    }

    FailClosedVerificationStatus::Enforced
}

fn evaluate_documentation_alignment(evidence: &AlignmentEvidence) -> DocumentationAlignmentStatus {
    if !evidence.architecture_rules_present || !evidence.boundary_rules_present {
        return DocumentationAlignmentStatus::Unknown;
    }

    if evidence.documentation_matches_implementation {
        return DocumentationAlignmentStatus::Consistent;
    }

    DocumentationAlignmentStatus::MismatchDetected
}

fn evaluate_overall(
    architecture_alignment: &ArchitectureAlignmentStatus,
    authority_boundary: &AuthorityBoundaryStatus,
    determinism_verification: &DeterminismVerificationStatus,
    fail_closed_verification: &FailClosedVerificationStatus,
    documentation_alignment: &DocumentationAlignmentStatus,
) -> OverallAlignmentStatus {
    if matches!(architecture_alignment, ArchitectureAlignmentStatus::Unknown)
        || matches!(authority_boundary, AuthorityBoundaryStatus::Unknown)
        || matches!(
            determinism_verification,
            DeterminismVerificationStatus::Unknown
        )
        || matches!(
            fail_closed_verification,
            FailClosedVerificationStatus::Unknown
        )
        || matches!(
            documentation_alignment,
            DocumentationAlignmentStatus::Unknown
        )
    {
        return OverallAlignmentStatus::Unknown;
    }

    if matches!(
        architecture_alignment,
        ArchitectureAlignmentStatus::DriftDetected
    ) || matches!(
        authority_boundary,
        AuthorityBoundaryStatus::ViolationDetected
    ) || matches!(
        determinism_verification,
        DeterminismVerificationStatus::NonDeterministic
    ) || matches!(
        fail_closed_verification,
        FailClosedVerificationStatus::ViolationDetected
    ) || matches!(
        documentation_alignment,
        DocumentationAlignmentStatus::MismatchDetected
    ) {
        return OverallAlignmentStatus::Violation;
    }

    OverallAlignmentStatus::Verified
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline_evidence() -> AlignmentEvidence {
        AlignmentEvidence {
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
        }
    }

    #[test]
    fn architecture_alignment_verification_is_deterministic() {
        let evidence = baseline_evidence();

        let first = verify_architecture_alignment(&evidence);
        let second = verify_architecture_alignment(&evidence);

        assert_eq!(first, second);
    }

    #[test]
    fn authority_boundary_violations_are_detected() {
        let mut evidence = baseline_evidence();
        evidence.unauthorized_non_rust_lifecycle_mutation = true;

        let result = verify_architecture_alignment(&evidence);

        assert_eq!(
            result.authority_boundary,
            AuthorityBoundaryStatus::ViolationDetected
        );
        assert_eq!(result.overall, OverallAlignmentStatus::Violation);
    }

    #[test]
    fn determinism_verification_remains_stable() {
        let evidence = baseline_evidence();

        let result = verify_architecture_alignment(&evidence);

        assert_eq!(
            result.determinism_verification,
            DeterminismVerificationStatus::Deterministic
        );
    }

    #[test]
    fn fail_closed_behavior_remains_enforced() {
        let mut evidence = baseline_evidence();
        evidence.missing_required_facts_detected = true;
        evidence.malformed_inputs_detected = true;
        evidence.unknown_inputs_detected = true;

        let result = verify_architecture_alignment(&evidence);

        assert_eq!(
            result.fail_closed_verification,
            FailClosedVerificationStatus::Enforced
        );
    }

    #[test]
    fn documentation_alignment_detects_mismatch() {
        let mut evidence = baseline_evidence();
        evidence.documentation_matches_implementation = false;

        let result = verify_architecture_alignment(&evidence);

        assert_eq!(
            result.documentation_alignment,
            DocumentationAlignmentStatus::MismatchDetected
        );
    }

    #[test]
    fn verification_remains_non_authoritative() {
        let evidence = baseline_evidence();
        let before = evidence.clone();

        let _ = verify_architecture_alignment(&evidence);

        assert_eq!(before, evidence);
    }

    #[test]
    fn verification_output_is_byte_equivalent_for_identical_input() {
        let evidence = baseline_evidence();

        let first = format!("{:?}", verify_architecture_alignment(&evidence));
        let second = format!("{:?}", verify_architecture_alignment(&evidence));

        assert_eq!(first.as_bytes(), second.as_bytes());
    }
}
