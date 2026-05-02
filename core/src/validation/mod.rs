#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    Pass,
    Fail,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationEvidence {
    pub schema_valid: bool,
    pub evidence_present: bool,
    pub evidence_well_formed: bool,
    pub deterministic_check_passed: bool,
    pub model_output_claims_valid: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub status: ValidationStatus,
    pub message: &'static str,
}

impl ValidationResult {
    pub fn unknown() -> Self {
        Self {
            status: ValidationStatus::Unknown,
            message: "unknown_is_not_pass",
        }
    }

    pub fn pass(message: &'static str) -> Self {
        Self {
            status: ValidationStatus::Pass,
            message,
        }
    }

    pub fn fail(message: &'static str) -> Self {
        Self {
            status: ValidationStatus::Fail,
            message,
        }
    }

    pub fn blocked(message: &'static str) -> Self {
        Self {
            status: ValidationStatus::Blocked,
            message,
        }
    }
}

pub fn evaluate_validation(evidence: &ValidationEvidence) -> ValidationResult {
    if !evidence.evidence_present {
        return ValidationResult::fail("missing_evidence");
    }

    if !evidence.evidence_well_formed {
        return ValidationResult::fail("malformed_evidence");
    }

    if !evidence.schema_valid {
        return ValidationResult::fail("schema_invalid");
    }

    if !evidence.deterministic_check_passed {
        return ValidationResult::fail("deterministic_check_failed");
    }

    ValidationResult::pass("validation_evidence_passed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_unknown_is_not_pass() {
        let result = ValidationResult::unknown();

        assert_eq!(result.status, ValidationStatus::Unknown);
        assert_eq!(result.message, "unknown_is_not_pass");
    }

    #[test]
    fn validation_missing_evidence_fails() {
        let result = evaluate_validation(&ValidationEvidence {
            schema_valid: true,
            evidence_present: false,
            evidence_well_formed: true,
            deterministic_check_passed: true,
            model_output_claims_valid: false,
        });

        assert_eq!(result.status, ValidationStatus::Fail);
        assert_eq!(result.message, "missing_evidence");
    }

    #[test]
    fn validation_malformed_evidence_fails() {
        let result = evaluate_validation(&ValidationEvidence {
            schema_valid: true,
            evidence_present: true,
            evidence_well_formed: false,
            deterministic_check_passed: true,
            model_output_claims_valid: false,
        });

        assert_eq!(result.status, ValidationStatus::Fail);
        assert_eq!(result.message, "malformed_evidence");
    }

    #[test]
    fn validation_schema_invalid_fails() {
        let result = evaluate_validation(&ValidationEvidence {
            schema_valid: false,
            evidence_present: true,
            evidence_well_formed: true,
            deterministic_check_passed: true,
            model_output_claims_valid: false,
        });

        assert_eq!(result.status, ValidationStatus::Fail);
        assert_eq!(result.message, "schema_invalid");
    }

    #[test]
    fn validation_deterministic_check_failed_fails() {
        let result = evaluate_validation(&ValidationEvidence {
            schema_valid: true,
            evidence_present: true,
            evidence_well_formed: true,
            deterministic_check_passed: false,
            model_output_claims_valid: false,
        });

        assert_eq!(result.status, ValidationStatus::Fail);
        assert_eq!(result.message, "deterministic_check_failed");
    }

    #[test]
    fn validation_required_evidence_passes() {
        let result = evaluate_validation(&ValidationEvidence {
            schema_valid: true,
            evidence_present: true,
            evidence_well_formed: true,
            deterministic_check_passed: true,
            model_output_claims_valid: false,
        });

        assert_eq!(result.status, ValidationStatus::Pass);
        assert_eq!(result.message, "validation_evidence_passed");
    }

    #[test]
    fn validation_model_valid_claim_alone_does_not_pass() {
        let result = evaluate_validation(&ValidationEvidence {
            schema_valid: false,
            evidence_present: false,
            evidence_well_formed: false,
            deterministic_check_passed: false,
            model_output_claims_valid: true,
        });

        assert_eq!(result.status, ValidationStatus::Fail);
        assert_ne!(result.status, ValidationStatus::Pass);
    }

    #[test]
    fn validation_model_valid_claim_does_not_override_malformed_evidence() {
        let result = evaluate_validation(&ValidationEvidence {
            schema_valid: true,
            evidence_present: true,
            evidence_well_formed: false,
            deterministic_check_passed: true,
            model_output_claims_valid: true,
        });

        assert_eq!(result.status, ValidationStatus::Fail);
        assert_eq!(result.message, "malformed_evidence");
    }

    #[test]
    fn validation_messages_are_stable() {
        let all_failures = evaluate_validation(&ValidationEvidence {
            schema_valid: false,
            evidence_present: false,
            evidence_well_formed: false,
            deterministic_check_passed: false,
            model_output_claims_valid: true,
        });
        assert_eq!(all_failures.message, "missing_evidence");

        let malformed_and_schema_invalid = evaluate_validation(&ValidationEvidence {
            schema_valid: false,
            evidence_present: true,
            evidence_well_formed: false,
            deterministic_check_passed: false,
            model_output_claims_valid: true,
        });
        assert_eq!(malformed_and_schema_invalid.message, "malformed_evidence");

        let deterministic_fail = evaluate_validation(&ValidationEvidence {
            schema_valid: true,
            evidence_present: true,
            evidence_well_formed: true,
            deterministic_check_passed: false,
            model_output_claims_valid: true,
        });
        assert_eq!(deterministic_fail.message, "deterministic_check_failed");

        let pass = evaluate_validation(&ValidationEvidence {
            schema_valid: true,
            evidence_present: true,
            evidence_well_formed: true,
            deterministic_check_passed: true,
            model_output_claims_valid: true,
        });
        assert_eq!(pass.message, "validation_evidence_passed");
    }
}
