//! Deterministic early production boundary validation for controlled review workflows.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarlyProductionBoundaryStatus {
    BoundarySatisfied,
    BoundaryNotSatisfied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EarlyProductionBoundaryError {
    MissingStaticValidation,
    MissingAdapterBoundary,
    MissingProviderParity,
    MissingCandidateGenerationPath,
    MissingStructuredEvaluation,
    MissingRustGovernanceAuthority,
    MissingLedgerBoundary,
    MissingReplayFromFacts,
    MissingAuditTraceability,
    MissingUiNonAuthority,
    MissingMultiDomainStability,
    MissingReuseBoundary,
    MissingNegativePathCoverage,
    MissingOperationalReadiness,
    ProductionApprovalClaimDetected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EarlyProductionBoundaryEvidence {
    pub static_validation_present: bool,
    pub adapter_boundary_present: bool,
    pub provider_parity_validated: bool,
    pub candidate_generation_path_present: bool,
    pub structured_evaluation_present: bool,
    pub rust_governance_authority_present: bool,
    pub ledger_boundary_present: bool,
    pub replay_from_facts_present: bool,
    pub audit_traceability_present: bool,
    pub ui_non_authority_preserved: bool,
    pub multi_domain_stability_validated: bool,
    pub reuse_boundary_present: bool,
    pub negative_path_coverage_present: bool,
    pub operational_readiness_validated: bool,
    pub production_approval_claim_absent: bool,
}

pub fn validate_early_production_boundary(
    evidence: &EarlyProductionBoundaryEvidence,
) -> Result<EarlyProductionBoundaryStatus, EarlyProductionBoundaryError> {
    if !evidence.static_validation_present {
        return Err(EarlyProductionBoundaryError::MissingStaticValidation);
    }

    if !evidence.adapter_boundary_present {
        return Err(EarlyProductionBoundaryError::MissingAdapterBoundary);
    }

    if !evidence.provider_parity_validated {
        return Err(EarlyProductionBoundaryError::MissingProviderParity);
    }

    if !evidence.candidate_generation_path_present {
        return Err(EarlyProductionBoundaryError::MissingCandidateGenerationPath);
    }

    if !evidence.structured_evaluation_present {
        return Err(EarlyProductionBoundaryError::MissingStructuredEvaluation);
    }

    if !evidence.rust_governance_authority_present {
        return Err(EarlyProductionBoundaryError::MissingRustGovernanceAuthority);
    }

    if !evidence.ledger_boundary_present {
        return Err(EarlyProductionBoundaryError::MissingLedgerBoundary);
    }

    if !evidence.replay_from_facts_present {
        return Err(EarlyProductionBoundaryError::MissingReplayFromFacts);
    }

    if !evidence.audit_traceability_present {
        return Err(EarlyProductionBoundaryError::MissingAuditTraceability);
    }

    if !evidence.ui_non_authority_preserved {
        return Err(EarlyProductionBoundaryError::MissingUiNonAuthority);
    }

    if !evidence.multi_domain_stability_validated {
        return Err(EarlyProductionBoundaryError::MissingMultiDomainStability);
    }

    if !evidence.reuse_boundary_present {
        return Err(EarlyProductionBoundaryError::MissingReuseBoundary);
    }

    if !evidence.negative_path_coverage_present {
        return Err(EarlyProductionBoundaryError::MissingNegativePathCoverage);
    }

    if !evidence.operational_readiness_validated {
        return Err(EarlyProductionBoundaryError::MissingOperationalReadiness);
    }

    if !evidence.production_approval_claim_absent {
        return Err(EarlyProductionBoundaryError::ProductionApprovalClaimDetected);
    }

    Ok(EarlyProductionBoundaryStatus::BoundarySatisfied)
}

#[cfg(test)]
mod tests {
    use super::{
        validate_early_production_boundary, EarlyProductionBoundaryError,
        EarlyProductionBoundaryEvidence, EarlyProductionBoundaryStatus,
    };

    fn complete_evidence() -> EarlyProductionBoundaryEvidence {
        EarlyProductionBoundaryEvidence {
            static_validation_present: true,
            adapter_boundary_present: true,
            provider_parity_validated: true,
            candidate_generation_path_present: true,
            structured_evaluation_present: true,
            rust_governance_authority_present: true,
            ledger_boundary_present: true,
            replay_from_facts_present: true,
            audit_traceability_present: true,
            ui_non_authority_preserved: true,
            multi_domain_stability_validated: true,
            reuse_boundary_present: true,
            negative_path_coverage_present: true,
            operational_readiness_validated: true,
            production_approval_claim_absent: true,
        }
    }

    #[test]
    fn all_early_production_boundary_evidence_is_accepted() {
        let evidence = complete_evidence();

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(result, Ok(EarlyProductionBoundaryStatus::BoundarySatisfied));
    }

    #[test]
    fn missing_static_validation_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.static_validation_present = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingStaticValidation)
        );
    }

    #[test]
    fn missing_adapter_boundary_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.adapter_boundary_present = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingAdapterBoundary)
        );
    }

    #[test]
    fn missing_provider_parity_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.provider_parity_validated = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingProviderParity)
        );
    }

    #[test]
    fn missing_candidate_generation_path_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.candidate_generation_path_present = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingCandidateGenerationPath)
        );
    }

    #[test]
    fn missing_structured_evaluation_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.structured_evaluation_present = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingStructuredEvaluation)
        );
    }

    #[test]
    fn missing_rust_governance_authority_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.rust_governance_authority_present = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingRustGovernanceAuthority)
        );
    }

    #[test]
    fn missing_ledger_boundary_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.ledger_boundary_present = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingLedgerBoundary)
        );
    }

    #[test]
    fn missing_replay_from_facts_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.replay_from_facts_present = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingReplayFromFacts)
        );
    }

    #[test]
    fn missing_audit_traceability_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.audit_traceability_present = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingAuditTraceability)
        );
    }

    #[test]
    fn missing_ui_non_authority_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.ui_non_authority_preserved = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingUiNonAuthority)
        );
    }

    #[test]
    fn missing_multi_domain_stability_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.multi_domain_stability_validated = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingMultiDomainStability)
        );
    }

    #[test]
    fn missing_reuse_boundary_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.reuse_boundary_present = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingReuseBoundary)
        );
    }

    #[test]
    fn missing_negative_path_coverage_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.negative_path_coverage_present = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingNegativePathCoverage)
        );
    }

    #[test]
    fn missing_operational_readiness_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.operational_readiness_validated = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::MissingOperationalReadiness)
        );
    }

    #[test]
    fn production_approval_claim_is_rejected() {
        let mut evidence = complete_evidence();
        evidence.production_approval_claim_absent = false;

        let result = validate_early_production_boundary(&evidence);

        assert_eq!(
            result,
            Err(EarlyProductionBoundaryError::ProductionApprovalClaimDetected)
        );
    }

    #[test]
    fn validation_is_deterministic() {
        let evidence = complete_evidence();

        let first = validate_early_production_boundary(&evidence);
        let second = validate_early_production_boundary(&evidence);

        assert_eq!(first, second);
        assert_eq!(first, Ok(EarlyProductionBoundaryStatus::BoundarySatisfied));
    }
}
