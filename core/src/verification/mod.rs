use crate::authority::{AuthorityBinding, EvidenceManifest};
use crate::execution::{ProviderOutput, ProviderOutputStatus, ProviderOutputTrust};
use crate::integrity::Digest;

const VERIFIER_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationCheckKind {
    EvidenceManifestWellFormed,
    CandidateShapeSatisfied,
    CandidateEvidenceBound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyCheckKind {
    RequiredContextBound,
    RequiredOperatorIntentBound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifierStatus {
    Satisfied,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifierReason {
    Verified,
    MissingStructuredInput,
    ManifestBindingMismatch,
    CandidateShapeInvalid,
    CandidateDigestMismatch,
    ContextDigestMismatch,
    OperatorIntentDigestMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationVerifierReceipt {
    kind: ValidationCheckKind,
    binding: AuthorityBinding,
    source_digests: Vec<Digest>,
    verifier_id: &'static str,
    verifier_version: &'static str,
    status: VerifierStatus,
    reason: VerifierReason,
    receipt_digest: Digest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyVerifierReceipt {
    kind: PolicyCheckKind,
    binding: AuthorityBinding,
    source_digests: Vec<Digest>,
    verifier_id: &'static str,
    verifier_version: &'static str,
    status: VerifierStatus,
    reason: VerifierReason,
    receipt_digest: Digest,
}

impl ValidationVerifierReceipt {
    pub fn kind(&self) -> ValidationCheckKind {
        self.kind
    }

    pub fn binding(&self) -> &AuthorityBinding {
        &self.binding
    }

    pub fn source_digests(&self) -> &[Digest] {
        &self.source_digests
    }

    pub fn status(&self) -> VerifierStatus {
        self.status
    }

    pub fn reason(&self) -> VerifierReason {
        self.reason
    }

    pub fn digest(&self) -> &Digest {
        &self.receipt_digest
    }

    pub fn is_internally_valid(&self) -> bool {
        self.receipt_digest == validation_receipt_digest(self)
    }
}

impl PolicyVerifierReceipt {
    pub fn kind(&self) -> PolicyCheckKind {
        self.kind
    }

    pub fn binding(&self) -> &AuthorityBinding {
        &self.binding
    }

    pub fn source_digests(&self) -> &[Digest] {
        &self.source_digests
    }

    pub fn status(&self) -> VerifierStatus {
        self.status
    }

    pub fn reason(&self) -> VerifierReason {
        self.reason
    }

    pub fn digest(&self) -> &Digest {
        &self.receipt_digest
    }

    pub fn is_internally_valid(&self) -> bool {
        self.receipt_digest == policy_receipt_digest(self)
    }
}

pub fn verify_evidence_manifest(
    binding: &AuthorityBinding,
    manifest: Option<&EvidenceManifest>,
) -> ValidationVerifierReceipt {
    let sources = manifest.map(manifest_source_digests).unwrap_or_default();
    let outcome = verify_manifest_outcome(binding, manifest);
    issue_validation_receipt(
        ValidationCheckKind::EvidenceManifestWellFormed,
        binding,
        sources,
        "evidence-manifest-verifier",
        outcome,
    )
}

pub fn verify_candidate_shape(
    binding: &AuthorityBinding,
    output: Option<&ProviderOutput>,
) -> ValidationVerifierReceipt {
    let sources = output.map(candidate_source_digests).unwrap_or_default();
    let outcome = verify_candidate_shape_outcome(binding, output);
    issue_validation_receipt(
        ValidationCheckKind::CandidateShapeSatisfied,
        binding,
        sources,
        "candidate-shape-verifier",
        outcome,
    )
}

pub fn verify_candidate_evidence_binding(
    binding: &AuthorityBinding,
    output: Option<&ProviderOutput>,
    manifest: Option<&EvidenceManifest>,
) -> ValidationVerifierReceipt {
    let sources = candidate_evidence_source_digests(output, manifest);
    let outcome = verify_candidate_evidence_outcome(binding, output, manifest);
    issue_validation_receipt(
        ValidationCheckKind::CandidateEvidenceBound,
        binding,
        sources,
        "candidate-evidence-binding-verifier",
        outcome,
    )
}

pub fn verify_required_context(
    binding: &AuthorityBinding,
    context_packet_id: Option<&str>,
) -> PolicyVerifierReceipt {
    let sources = text_source_digests(context_packet_id);
    let outcome = verify_context_outcome(binding, context_packet_id);
    issue_policy_receipt(
        PolicyCheckKind::RequiredContextBound,
        binding,
        sources,
        "required-context-verifier",
        outcome,
    )
}

pub fn verify_required_operator_intent(
    binding: &AuthorityBinding,
    operator_intent_source: Option<&str>,
) -> PolicyVerifierReceipt {
    let sources = text_source_digests(operator_intent_source);
    let outcome = verify_operator_intent_outcome(binding, operator_intent_source);
    issue_policy_receipt(
        PolicyCheckKind::RequiredOperatorIntentBound,
        binding,
        sources,
        "required-operator-intent-verifier",
        outcome,
    )
}

fn verify_manifest_outcome(
    binding: &AuthorityBinding,
    manifest: Option<&EvidenceManifest>,
) -> (VerifierStatus, VerifierReason) {
    let Some(manifest) = manifest else {
        return (
            VerifierStatus::Unknown,
            VerifierReason::MissingStructuredInput,
        );
    };
    if manifest.verify_binding(binding).is_ok() {
        return (VerifierStatus::Satisfied, VerifierReason::Verified);
    }
    (
        VerifierStatus::Failed,
        VerifierReason::ManifestBindingMismatch,
    )
}

fn verify_candidate_shape_outcome(
    _binding: &AuthorityBinding,
    output: Option<&ProviderOutput>,
) -> (VerifierStatus, VerifierReason) {
    let Some(output) = output else {
        return (
            VerifierStatus::Unknown,
            VerifierReason::MissingStructuredInput,
        );
    };
    if !candidate_shape_is_valid(output) {
        return (
            VerifierStatus::Failed,
            VerifierReason::CandidateShapeInvalid,
        );
    }
    (VerifierStatus::Satisfied, VerifierReason::Verified)
}

fn verify_candidate_evidence_outcome(
    binding: &AuthorityBinding,
    output: Option<&ProviderOutput>,
    manifest: Option<&EvidenceManifest>,
) -> (VerifierStatus, VerifierReason) {
    let (Some(output), Some(manifest)) = (output, manifest) else {
        return (
            VerifierStatus::Unknown,
            VerifierReason::MissingStructuredInput,
        );
    };
    if manifest.verify_binding(binding).is_err() {
        return (
            VerifierStatus::Failed,
            VerifierReason::ManifestBindingMismatch,
        );
    }
    verify_candidate_digest(binding, output)
}

fn verify_context_outcome(
    binding: &AuthorityBinding,
    context_packet_id: Option<&str>,
) -> (VerifierStatus, VerifierReason) {
    let Some(context_packet_id) = nonempty_source(context_packet_id) else {
        return (
            VerifierStatus::Unknown,
            VerifierReason::MissingStructuredInput,
        );
    };
    if &Digest::of_text(context_packet_id) == binding.context_packet_digest() {
        return (VerifierStatus::Satisfied, VerifierReason::Verified);
    }
    (
        VerifierStatus::Failed,
        VerifierReason::ContextDigestMismatch,
    )
}

fn verify_operator_intent_outcome(
    binding: &AuthorityBinding,
    operator_intent_source: Option<&str>,
) -> (VerifierStatus, VerifierReason) {
    let Some(source) = nonempty_source(operator_intent_source) else {
        return (
            VerifierStatus::Unknown,
            VerifierReason::MissingStructuredInput,
        );
    };
    if &Digest::of_text(source) == binding.operator_intent_digest() {
        return (VerifierStatus::Satisfied, VerifierReason::Verified);
    }
    (
        VerifierStatus::Failed,
        VerifierReason::OperatorIntentDigestMismatch,
    )
}

fn verify_candidate_digest(
    binding: &AuthorityBinding,
    output: &ProviderOutput,
) -> (VerifierStatus, VerifierReason) {
    if &Digest::of_text(&output.content) == binding.candidate_digest() {
        return (VerifierStatus::Satisfied, VerifierReason::Verified);
    }
    (
        VerifierStatus::Failed,
        VerifierReason::CandidateDigestMismatch,
    )
}

fn candidate_shape_is_valid(output: &ProviderOutput) -> bool {
    !output.id.trim().is_empty()
        && !output.request_id.trim().is_empty()
        && !output.content.trim().is_empty()
        && output.status == ProviderOutputStatus::Received
        && output.trust == ProviderOutputTrust::Untrusted
}

fn nonempty_source(source: Option<&str>) -> Option<&str> {
    source.filter(|value| !value.trim().is_empty())
}

fn issue_validation_receipt(
    kind: ValidationCheckKind,
    binding: &AuthorityBinding,
    source_digests: Vec<Digest>,
    verifier_id: &'static str,
    outcome: (VerifierStatus, VerifierReason),
) -> ValidationVerifierReceipt {
    let mut receipt = ValidationVerifierReceipt {
        kind,
        binding: binding.clone(),
        source_digests,
        verifier_id,
        verifier_version: VERIFIER_VERSION,
        status: outcome.0,
        reason: outcome.1,
        receipt_digest: Digest::of_text("pending"),
    };
    receipt.receipt_digest = validation_receipt_digest(&receipt);
    receipt
}

fn issue_policy_receipt(
    kind: PolicyCheckKind,
    binding: &AuthorityBinding,
    source_digests: Vec<Digest>,
    verifier_id: &'static str,
    outcome: (VerifierStatus, VerifierReason),
) -> PolicyVerifierReceipt {
    let mut receipt = PolicyVerifierReceipt {
        kind,
        binding: binding.clone(),
        source_digests,
        verifier_id,
        verifier_version: VERIFIER_VERSION,
        status: outcome.0,
        reason: outcome.1,
        receipt_digest: Digest::of_text("pending"),
    };
    receipt.receipt_digest = policy_receipt_digest(&receipt);
    receipt
}

fn validation_receipt_digest(receipt: &ValidationVerifierReceipt) -> Digest {
    verifier_receipt_digest(
        &format!("validation:{:?}", receipt.kind),
        &receipt.binding,
        &receipt.source_digests,
        receipt.verifier_id,
        receipt.verifier_version,
        receipt.status,
        receipt.reason,
    )
}

fn policy_receipt_digest(receipt: &PolicyVerifierReceipt) -> Digest {
    verifier_receipt_digest(
        &format!("policy:{:?}", receipt.kind),
        &receipt.binding,
        &receipt.source_digests,
        receipt.verifier_id,
        receipt.verifier_version,
        receipt.status,
        receipt.reason,
    )
}

fn verifier_receipt_digest(
    kind: &str,
    binding: &AuthorityBinding,
    source_digests: &[Digest],
    verifier_id: &str,
    verifier_version: &str,
    status: VerifierStatus,
    reason: VerifierReason,
) -> Digest {
    Digest::of_text(&format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:?}|{:?}",
        kind,
        binding.run_id(),
        binding.task_digest().as_str(),
        binding.operator_intent_digest().as_str(),
        binding.context_packet_digest().as_str(),
        binding.candidate_digest().as_str(),
        binding.policy_bundle_digest().as_str(),
        binding.evidence_manifest_digest().as_str(),
        binding.valid_through_revision(),
        source_digest_string(source_digests),
        verifier_id,
        verifier_version,
        status,
        reason
    ))
}

fn source_digest_string(source_digests: &[Digest]) -> String {
    source_digests
        .iter()
        .map(Digest::as_str)
        .collect::<Vec<_>>()
        .join(",")
}

fn manifest_source_digests(manifest: &EvidenceManifest) -> Vec<Digest> {
    vec![manifest.digest().clone()]
}

fn candidate_source_digests(output: &ProviderOutput) -> Vec<Digest> {
    vec![
        candidate_shape_digest(output),
        Digest::of_text(&output.content),
    ]
}

fn candidate_evidence_source_digests(
    output: Option<&ProviderOutput>,
    manifest: Option<&EvidenceManifest>,
) -> Vec<Digest> {
    let mut sources = output.map(candidate_source_digests).unwrap_or_default();
    sources.extend(manifest.map(manifest_source_digests).unwrap_or_default());
    sources
}

fn text_source_digests(source: Option<&str>) -> Vec<Digest> {
    nonempty_source(source)
        .map(|value| vec![Digest::of_text(value)])
        .unwrap_or_default()
}

fn candidate_shape_digest(output: &ProviderOutput) -> Digest {
    Digest::of_text(&format!(
        "{}|{}|{:?}|{:?}|{}",
        output.id,
        output.request_id,
        output.status,
        output.trust,
        output.content.len()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authority::{AuthorityBindingInput, EvidenceReference};
    use crate::execution::ProviderKind;

    fn manifest() -> EvidenceManifest {
        EvidenceManifest::new(vec![EvidenceReference::new(
            "evidence-1",
            Digest::of_text("facts"),
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
            verifier_id: "authority-verifier".into(),
            verifier_version: "1.0.0".into(),
            valid_through_revision: 2,
        })
        .unwrap()
    }

    #[test]
    fn fixed_inputs_produce_identical_validation_receipts() {
        let manifest = manifest();
        let binding = binding(&manifest, "candidate");
        let first = verify_evidence_manifest(&binding, Some(&manifest));
        let second = verify_evidence_manifest(&binding, Some(&manifest));
        assert_eq!(first, second);
        assert!(first.is_internally_valid());
    }

    #[test]
    fn missing_candidate_is_unknown() {
        let manifest = manifest();
        let receipt = verify_candidate_shape(&binding(&manifest, "candidate"), None);
        assert_eq!(receipt.status(), VerifierStatus::Unknown);
    }

    #[test]
    fn candidate_digest_mismatch_fails() {
        let manifest = manifest();
        let receipt = verify_candidate_evidence_binding(
            &binding(&manifest, "expected"),
            Some(&output("actual")),
            Some(&manifest),
        );
        assert_eq!(receipt.status(), VerifierStatus::Failed);
        assert_eq!(receipt.reason(), VerifierReason::CandidateDigestMismatch);
    }

    #[test]
    fn context_and_intent_are_derived_from_source_text() {
        let manifest = manifest();
        let binding = binding(&manifest, "candidate");
        assert_eq!(
            verify_required_context(&binding, Some("context")).status(),
            VerifierStatus::Satisfied
        );
        assert_eq!(
            verify_required_operator_intent(&binding, Some("intent")).status(),
            VerifierStatus::Satisfied
        );
    }
}
