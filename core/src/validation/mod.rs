use crate::authority::{AuthorityBinding, EvidenceManifest};
use crate::integrity::Digest;
use crate::verification::{ValidationCheckKind, ValidationVerifierReceipt, VerifierStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    Pass,
    Fail,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationEvidence {
    manifest: EvidenceManifest,
    manifest_receipt: ValidationVerifierReceipt,
    shape_receipt: ValidationVerifierReceipt,
    deterministic_receipt: ValidationVerifierReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReceipt {
    binding: AuthorityBinding,
    status: ValidationStatus,
    message: &'static str,
    verifier_evidence_digest: Digest,
    receipt_digest: Digest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationEvidenceError {
    VerificationBindingMismatch,
    VerificationKindMismatch,
    VerificationSourceMismatch,
    VerificationDigestMismatch,
}

impl ValidationEvidenceError {
    pub fn code(self) -> &'static str {
        match self {
            Self::VerificationBindingMismatch => "verification_binding_mismatch",
            Self::VerificationKindMismatch => "verification_kind_mismatch",
            Self::VerificationSourceMismatch => "verification_source_mismatch",
            Self::VerificationDigestMismatch => "verification_digest_mismatch",
        }
    }
}

impl ValidationEvidence {
    pub fn new(
        manifest: EvidenceManifest,
        manifest_receipt: ValidationVerifierReceipt,
        shape_receipt: ValidationVerifierReceipt,
        deterministic_receipt: ValidationVerifierReceipt,
    ) -> Result<Self, ValidationEvidenceError> {
        validate_receipt_kinds(&manifest_receipt, &shape_receipt, &deterministic_receipt)?;
        validate_receipt_bindings(&manifest_receipt, &shape_receipt, &deterministic_receipt)?;
        validate_receipt_digests(&manifest_receipt, &shape_receipt, &deterministic_receipt)?;
        validate_receipt_sources(
            &manifest,
            &manifest_receipt,
            &shape_receipt,
            &deterministic_receipt,
        )?;
        Ok(Self {
            manifest,
            manifest_receipt,
            shape_receipt,
            deterministic_receipt,
        })
    }

    pub fn manifest(&self) -> &EvidenceManifest {
        &self.manifest
    }

    pub fn binding(&self) -> &AuthorityBinding {
        self.manifest_receipt.binding()
    }

    pub fn verifier_evidence_digest(&self) -> Digest {
        verifier_evidence_digest(self)
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

    pub fn verifier_evidence_digest(&self) -> &Digest {
        &self.verifier_evidence_digest
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
    issue_validation_receipt(binding, outcome, evidence.verifier_evidence_digest())
}

pub fn record_unknown_validation(binding: AuthorityBinding) -> ValidationReceipt {
    issue_validation_receipt(
        binding,
        (ValidationStatus::Unknown, "unknown_is_not_pass"),
        Digest::of_text("missing-verifier-evidence"),
    )
}

fn determine_validation_outcome(
    binding: &AuthorityBinding,
    evidence: &ValidationEvidence,
) -> (ValidationStatus, &'static str) {
    if evidence.binding() != binding || evidence.manifest.verify_binding(binding).is_err() {
        return (ValidationStatus::Fail, "evidence_manifest_mismatch");
    }
    evaluate_verifier_receipts(evidence)
}

fn evaluate_verifier_receipts(evidence: &ValidationEvidence) -> (ValidationStatus, &'static str) {
    if evidence.manifest_receipt.status() == VerifierStatus::Failed {
        return (
            ValidationStatus::Fail,
            "evidence_manifest_verification_failed",
        );
    }
    if evidence.shape_receipt.status() == VerifierStatus::Failed {
        return (ValidationStatus::Fail, "schema_shape_verification_failed");
    }
    if evidence.deterministic_receipt.status() == VerifierStatus::Failed {
        return (ValidationStatus::Fail, "deterministic_check_failed");
    }
    if validation_verification_is_unknown(evidence) {
        return (ValidationStatus::Unknown, "validation_verification_unknown");
    }
    (
        ValidationStatus::Pass,
        "validation_verifier_receipts_satisfied",
    )
}

fn validation_verification_is_unknown(evidence: &ValidationEvidence) -> bool {
    evidence.manifest_receipt.status() == VerifierStatus::Unknown
        || evidence.shape_receipt.status() == VerifierStatus::Unknown
        || evidence.deterministic_receipt.status() == VerifierStatus::Unknown
}

fn validate_receipt_kinds(
    manifest: &ValidationVerifierReceipt,
    shape: &ValidationVerifierReceipt,
    deterministic: &ValidationVerifierReceipt,
) -> Result<(), ValidationEvidenceError> {
    if manifest.kind() == ValidationCheckKind::EvidenceManifestWellFormed
        && shape.kind() == ValidationCheckKind::CandidateShapeSatisfied
        && deterministic.kind() == ValidationCheckKind::CandidateEvidenceBound
    {
        return Ok(());
    }
    Err(ValidationEvidenceError::VerificationKindMismatch)
}

fn validate_receipt_bindings(
    manifest: &ValidationVerifierReceipt,
    shape: &ValidationVerifierReceipt,
    deterministic: &ValidationVerifierReceipt,
) -> Result<(), ValidationEvidenceError> {
    if manifest.binding() == shape.binding() && shape.binding() == deterministic.binding() {
        return Ok(());
    }
    Err(ValidationEvidenceError::VerificationBindingMismatch)
}

fn validate_receipt_digests(
    manifest: &ValidationVerifierReceipt,
    shape: &ValidationVerifierReceipt,
    deterministic: &ValidationVerifierReceipt,
) -> Result<(), ValidationEvidenceError> {
    if manifest.is_internally_valid()
        && shape.is_internally_valid()
        && deterministic.is_internally_valid()
    {
        return Ok(());
    }
    Err(ValidationEvidenceError::VerificationDigestMismatch)
}

fn validate_receipt_sources(
    manifest: &EvidenceManifest,
    manifest_receipt: &ValidationVerifierReceipt,
    shape: &ValidationVerifierReceipt,
    deterministic: &ValidationVerifierReceipt,
) -> Result<(), ValidationEvidenceError> {
    validate_manifest_source(manifest, manifest_receipt, deterministic)?;
    validate_candidate_source(shape, deterministic)
}

fn validate_manifest_source(
    manifest: &EvidenceManifest,
    manifest_receipt: &ValidationVerifierReceipt,
    deterministic: &ValidationVerifierReceipt,
) -> Result<(), ValidationEvidenceError> {
    if manifest_receipt
        .source_digests()
        .contains(manifest.digest())
        && deterministic_source_is_unknown_or_contains(deterministic, manifest.digest())
    {
        return Ok(());
    }
    Err(ValidationEvidenceError::VerificationSourceMismatch)
}

fn deterministic_source_is_unknown_or_contains(
    receipt: &ValidationVerifierReceipt,
    digest: &Digest,
) -> bool {
    receipt.status() == VerifierStatus::Unknown || receipt.source_digests().contains(digest)
}

fn validate_candidate_source(
    shape: &ValidationVerifierReceipt,
    deterministic: &ValidationVerifierReceipt,
) -> Result<(), ValidationEvidenceError> {
    if shape.status() == VerifierStatus::Unknown
        || deterministic.status() == VerifierStatus::Unknown
    {
        return Ok(());
    }
    let candidate_digest = shape.source_digests().get(1);
    if candidate_digest.is_some() && deterministic.source_digests().get(1) == candidate_digest {
        return Ok(());
    }
    Err(ValidationEvidenceError::VerificationSourceMismatch)
}

fn issue_validation_receipt(
    binding: AuthorityBinding,
    outcome: (ValidationStatus, &'static str),
    verifier_evidence_digest: Digest,
) -> ValidationReceipt {
    let receipt_digest = validation_receipt_digest(&binding, outcome, &verifier_evidence_digest);
    ValidationReceipt {
        binding,
        status: outcome.0,
        message: outcome.1,
        verifier_evidence_digest,
        receipt_digest,
    }
}

fn validation_receipt_digest(
    binding: &AuthorityBinding,
    outcome: (ValidationStatus, &'static str),
    verifier_evidence_digest: &Digest,
) -> Digest {
    Digest::of_text(&format!(
        "validation|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:?}|{}",
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
        verifier_evidence_digest.as_str(),
        outcome.0,
        outcome.1
    ))
}

fn verifier_evidence_digest(evidence: &ValidationEvidence) -> Digest {
    Digest::of_text(&format!(
        "{}|{}|{}|{}",
        evidence.manifest.digest().as_str(),
        evidence.manifest_receipt.digest().as_str(),
        evidence.shape_receipt.digest().as_str(),
        evidence.deterministic_receipt.digest().as_str()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authority::{AuthorityBindingInput, EvidenceReference};
    use crate::execution::{ProviderKind, ProviderOutput, ProviderOutputStatus};
    use crate::verification::{
        verify_candidate_evidence_binding, verify_candidate_shape, verify_evidence_manifest,
    };

    fn manifest(label: &str) -> EvidenceManifest {
        EvidenceManifest::new(vec![EvidenceReference::new(
            "evidence-1",
            Digest::of_text(label),
        )
        .unwrap()])
        .unwrap()
    }

    fn output(content: &str) -> ProviderOutput {
        ProviderOutput::new_untrusted(
            "output-1",
            "request-1",
            ProviderKind::Local,
            content,
            ProviderOutputStatus::Received,
        )
        .unwrap()
    }

    fn binding(manifest: &EvidenceManifest, content: &str) -> AuthorityBinding {
        AuthorityBinding::new(AuthorityBindingInput {
            run_id: "run-1".into(),
            task_digest: Digest::of_text("task"),
            operator_intent_digest: Digest::of_text("intent"),
            context_packet_digest: Digest::of_text("context"),
            candidate_digest: Digest::of_text(content),
            policy_bundle_digest: Digest::of_text("policy"),
            evidence_manifest_digest: manifest.digest().clone(),
            verifier_id: "validator".into(),
            verifier_version: "1.0.0".into(),
            valid_through_revision: 2,
        })
        .unwrap()
    }

    fn evidence(
        binding: &AuthorityBinding,
        manifest: EvidenceManifest,
        output: Option<&ProviderOutput>,
    ) -> ValidationEvidence {
        ValidationEvidence::new(
            manifest.clone(),
            verify_evidence_manifest(binding, Some(&manifest)),
            verify_candidate_shape(binding, output),
            verify_candidate_evidence_binding(binding, output, Some(&manifest)),
        )
        .unwrap()
    }

    #[test]
    fn matching_verifier_evidence_passes() {
        let manifest = manifest("facts");
        let output = output("candidate");
        let binding = binding(&manifest, &output.content);
        let receipt = evaluate_validation(
            binding.clone(),
            &evidence(&binding, manifest, Some(&output)),
        );
        assert_eq!(receipt.status(), ValidationStatus::Pass);
    }

    #[test]
    fn candidate_shape_failure_prevents_pass() {
        let manifest = manifest("facts");
        let mut output = output("candidate");
        let binding = binding(&manifest, &output.content);
        output.status = ProviderOutputStatus::Rejected;
        let receipt = evaluate_validation(
            binding.clone(),
            &evidence(&binding, manifest, Some(&output)),
        );
        assert_eq!(receipt.status(), ValidationStatus::Fail);
        assert_eq!(receipt.message(), "schema_shape_verification_failed");
    }

    #[test]
    fn candidate_digest_mismatch_prevents_pass() {
        let manifest = manifest("facts");
        let output = output("actual");
        let binding = binding(&manifest, "expected");
        let receipt = evaluate_validation(
            binding.clone(),
            &evidence(&binding, manifest, Some(&output)),
        );
        assert_eq!(receipt.status(), ValidationStatus::Fail);
        assert_eq!(receipt.message(), "deterministic_check_failed");
    }

    #[test]
    fn missing_candidate_verification_is_unknown() {
        let manifest = manifest("facts");
        let binding = binding(&manifest, "candidate");
        let receipt = evaluate_validation(binding.clone(), &evidence(&binding, manifest, None));
        assert_eq!(receipt.status(), ValidationStatus::Unknown);
        assert!(!receipt.passed());
    }

    #[test]
    fn cross_binding_receipt_bundle_is_rejected() {
        let manifest = manifest("facts");
        let output = output("candidate");
        let first = binding(&manifest, "candidate");
        let second_input = first.clone();
        assert_eq!(second_input.run_id(), "run-1");
        let second = AuthorityBinding::new(AuthorityBindingInput {
            run_id: "run-2".into(),
            task_digest: first.task_digest().clone(),
            operator_intent_digest: first.operator_intent_digest().clone(),
            context_packet_digest: first.context_packet_digest().clone(),
            candidate_digest: first.candidate_digest().clone(),
            policy_bundle_digest: first.policy_bundle_digest().clone(),
            evidence_manifest_digest: first.evidence_manifest_digest().clone(),
            verifier_id: first.verifier_id().into(),
            verifier_version: first.verifier_version().into(),
            valid_through_revision: first.valid_through_revision(),
        })
        .unwrap();
        let result = ValidationEvidence::new(
            manifest.clone(),
            verify_evidence_manifest(&first, Some(&manifest)),
            verify_candidate_shape(&second, Some(&output)),
            verify_candidate_evidence_binding(&first, Some(&output), Some(&manifest)),
        );
        assert_eq!(
            result,
            Err(ValidationEvidenceError::VerificationBindingMismatch)
        );
    }

    #[test]
    fn model_validity_text_cannot_repair_failed_shape() {
        let manifest = manifest("facts");
        let mut output = output("valid approved schema valid evidence complete");
        let binding = binding(&manifest, &output.content);
        output.status = ProviderOutputStatus::Rejected;
        let receipt = evaluate_validation(
            binding.clone(),
            &evidence(&binding, manifest, Some(&output)),
        );
        assert_eq!(receipt.status(), ValidationStatus::Fail);
    }
}
