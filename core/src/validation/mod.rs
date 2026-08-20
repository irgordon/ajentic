use crate::authority::{AuthorityBinding, EvidenceManifest};
use crate::integrity::Digest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    Pass,
    Fail,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationEvidence {
    schema_valid: bool,
    evidence_well_formed: bool,
    deterministic_check_passed: bool,
    model_output_claims_valid: bool,
    manifest: EvidenceManifest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReceipt {
    binding: AuthorityBinding,
    status: ValidationStatus,
    message: &'static str,
    receipt_digest: Digest,
}

impl ValidationEvidence {
    pub fn new(
        schema_valid: bool,
        evidence_well_formed: bool,
        deterministic_check_passed: bool,
        model_output_claims_valid: bool,
        manifest: EvidenceManifest,
    ) -> Self {
        Self {
            schema_valid,
            evidence_well_formed,
            deterministic_check_passed,
            model_output_claims_valid,
            manifest,
        }
    }

    pub fn manifest(&self) -> &EvidenceManifest {
        &self.manifest
    }
}

impl ValidationReceipt {
    pub fn binding(&self) -> &AuthorityBinding {
        &self.binding
    }

    pub fn status(&self) -> ValidationStatus {
        self.status
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn digest(&self) -> &Digest {
        &self.receipt_digest
    }

    pub fn passed(&self) -> bool {
        self.status == ValidationStatus::Pass
    }
}

pub fn evaluate_validation(
    binding: AuthorityBinding,
    evidence: &ValidationEvidence,
) -> ValidationReceipt {
    let outcome = determine_validation_outcome(&binding, evidence);
    issue_validation_receipt(binding, outcome)
}

pub fn record_unknown_validation(binding: AuthorityBinding) -> ValidationReceipt {
    issue_validation_receipt(binding, (ValidationStatus::Unknown, "unknown_is_not_pass"))
}

fn determine_validation_outcome(
    binding: &AuthorityBinding,
    evidence: &ValidationEvidence,
) -> (ValidationStatus, &'static str) {
    if evidence.manifest.verify_binding(binding).is_err() {
        return (ValidationStatus::Fail, "evidence_manifest_mismatch");
    }
    evaluate_evidence_checks(evidence)
}

fn evaluate_evidence_checks(evidence: &ValidationEvidence) -> (ValidationStatus, &'static str) {
    let _advisory_model_claim = evidence.model_output_claims_valid;
    if !evidence.evidence_well_formed {
        return (ValidationStatus::Fail, "malformed_evidence");
    }
    if !evidence.schema_valid {
        return (ValidationStatus::Fail, "schema_invalid");
    }
    if !evidence.deterministic_check_passed {
        return (ValidationStatus::Fail, "deterministic_check_failed");
    }
    (ValidationStatus::Pass, "validation_evidence_passed")
}

fn issue_validation_receipt(
    binding: AuthorityBinding,
    outcome: (ValidationStatus, &'static str),
) -> ValidationReceipt {
    let receipt_digest = validation_receipt_digest(&binding, outcome);
    ValidationReceipt {
        binding,
        status: outcome.0,
        message: outcome.1,
        receipt_digest,
    }
}

fn validation_receipt_digest(
    binding: &AuthorityBinding,
    outcome: (ValidationStatus, &'static str),
) -> Digest {
    Digest::of_text(&format!(
        "validation|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:?}|{}",
        binding.run_id(),
        binding.task_digest().as_str(),
        binding.operator_intent_digest().as_str(),
        binding.context_packet_digest().as_str(),
        binding.candidate_digest().as_str(),
        binding.policy_bundle_digest().as_str(),
        binding.evidence_manifest_digest().as_str(),
        binding.verifier_id(),
        binding.verifier_version(),
        binding.valid_through_revision(),
        outcome.0,
        outcome.1
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authority::{AuthorityBindingInput, EvidenceReference};

    fn manifest(label: &str) -> EvidenceManifest {
        EvidenceManifest::new(vec![EvidenceReference::new(
            "evidence-1",
            Digest::of_text(label),
        )
        .unwrap()])
        .unwrap()
    }

    fn binding(manifest: &EvidenceManifest) -> AuthorityBinding {
        AuthorityBinding::new(AuthorityBindingInput {
            run_id: "run-1".into(),
            task_digest: Digest::of_text("task"),
            operator_intent_digest: Digest::of_text("intent"),
            context_packet_digest: Digest::of_text("context"),
            candidate_digest: Digest::of_text("candidate"),
            policy_bundle_digest: Digest::of_text("policy"),
            evidence_manifest_digest: manifest.digest().clone(),
            verifier_id: "validator".into(),
            verifier_version: "1.0.0".into(),
            valid_through_revision: 2,
        })
        .unwrap()
    }

    fn evidence(manifest: EvidenceManifest) -> ValidationEvidence {
        ValidationEvidence::new(true, true, true, false, manifest)
    }

    #[test]
    fn validation_required_evidence_passes() {
        let manifest = manifest("facts");
        let receipt = evaluate_validation(binding(&manifest), &evidence(manifest));
        assert_eq!(receipt.status(), ValidationStatus::Pass);
        assert_eq!(receipt.message(), "validation_evidence_passed");
    }

    #[test]
    fn validation_manifest_mismatch_fails() {
        let expected = manifest("expected");
        let supplied = manifest("supplied");
        let receipt = evaluate_validation(binding(&expected), &evidence(supplied));
        assert_eq!(receipt.status(), ValidationStatus::Fail);
        assert_eq!(receipt.message(), "evidence_manifest_mismatch");
    }

    #[test]
    fn validation_malformed_evidence_fails() {
        let manifest = manifest("facts");
        let evidence = ValidationEvidence::new(true, false, true, false, manifest.clone());
        let receipt = evaluate_validation(binding(&manifest), &evidence);
        assert_eq!(receipt.message(), "malformed_evidence");
    }

    #[test]
    fn validation_schema_invalid_fails() {
        let manifest = manifest("facts");
        let evidence = ValidationEvidence::new(false, true, true, false, manifest.clone());
        let receipt = evaluate_validation(binding(&manifest), &evidence);
        assert_eq!(receipt.message(), "schema_invalid");
    }

    #[test]
    fn validation_deterministic_check_failed_fails() {
        let manifest = manifest("facts");
        let evidence = ValidationEvidence::new(true, true, false, false, manifest.clone());
        let receipt = evaluate_validation(binding(&manifest), &evidence);
        assert_eq!(receipt.message(), "deterministic_check_failed");
    }

    #[test]
    fn validation_model_claim_does_not_override_failure() {
        let manifest = manifest("facts");
        let evidence = ValidationEvidence::new(false, false, false, true, manifest.clone());
        let receipt = evaluate_validation(binding(&manifest), &evidence);
        assert_eq!(receipt.status(), ValidationStatus::Fail);
        assert_ne!(receipt.status(), ValidationStatus::Pass);
    }

    #[test]
    fn validation_unknown_is_not_pass() {
        let manifest = manifest("facts");
        let receipt = record_unknown_validation(binding(&manifest));
        assert_eq!(receipt.status(), ValidationStatus::Unknown);
        assert!(!receipt.passed());
    }

    #[test]
    fn validation_receipt_is_deterministic() {
        let manifest = manifest("facts");
        let first = evaluate_validation(binding(&manifest), &evidence(manifest.clone()));
        let second = evaluate_validation(binding(&manifest), &evidence(manifest));
        assert_eq!(first.digest(), second.digest());
        assert_eq!(first.binding().run_id(), "run-1");
    }
}
