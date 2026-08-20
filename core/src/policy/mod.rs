use crate::authority::AuthorityBinding;
use crate::integrity::Digest;
use crate::validation::ValidationReceipt;
use crate::verification::{PolicyCheckKind, PolicyVerifierReceipt, VerifierStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allowed,
    Denied,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyEvidence {
    context_receipt: PolicyVerifierReceipt,
    operator_intent_receipt: PolicyVerifierReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyReceipt {
    binding: AuthorityBinding,
    decision: PolicyDecision,
    reason: &'static str,
    validation_receipt_digest: Digest,
    verifier_evidence_digest: Digest,
    receipt_digest: Digest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyEvidenceError {
    VerificationBindingMismatch,
    VerificationKindMismatch,
    VerificationDigestMismatch,
}

impl PolicyEvidenceError {
    pub fn code(self) -> &'static str {
        match self {
            Self::VerificationBindingMismatch => "verification_binding_mismatch",
            Self::VerificationKindMismatch => "verification_kind_mismatch",
            Self::VerificationDigestMismatch => "verification_digest_mismatch",
        }
    }
}

impl PolicyEvidence {
    pub fn new(
        context_receipt: PolicyVerifierReceipt,
        operator_intent_receipt: PolicyVerifierReceipt,
    ) -> Result<Self, PolicyEvidenceError> {
        validate_policy_receipt_kinds(&context_receipt, &operator_intent_receipt)?;
        validate_policy_receipt_bindings(&context_receipt, &operator_intent_receipt)?;
        validate_policy_receipt_digests(&context_receipt, &operator_intent_receipt)?;
        Ok(Self {
            context_receipt,
            operator_intent_receipt,
        })
    }

    pub fn binding(&self) -> &AuthorityBinding {
        self.context_receipt.binding()
    }

    pub fn verifier_evidence_digest(&self) -> Digest {
        policy_evidence_digest(self)
    }
}

impl PolicyReceipt {
    pub fn binding(&self) -> &AuthorityBinding {
        &self.binding
    }

    pub fn decision(&self) -> PolicyDecision {
        self.decision
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub fn validation_receipt_digest(&self) -> &Digest {
        &self.validation_receipt_digest
    }

    pub fn verifier_evidence_digest(&self) -> &Digest {
        &self.verifier_evidence_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.receipt_digest
    }

    pub fn allowed(&self) -> bool {
        self.decision == PolicyDecision::Allowed
    }
}

pub fn evaluate_policy(
    binding: AuthorityBinding,
    evidence: &PolicyEvidence,
    validation: &ValidationReceipt,
) -> PolicyReceipt {
    let outcome = determine_policy_outcome(&binding, evidence, validation);
    issue_policy_receipt(
        binding,
        validation.digest().clone(),
        evidence.verifier_evidence_digest(),
        outcome,
    )
}

pub fn record_unknown_policy(
    binding: AuthorityBinding,
    validation: &ValidationReceipt,
) -> PolicyReceipt {
    issue_policy_receipt(
        binding,
        validation.digest().clone(),
        Digest::of_text("missing-policy-verifier-evidence"),
        (PolicyDecision::Unknown, "unknown_is_not_pass"),
    )
}

fn determine_policy_outcome(
    binding: &AuthorityBinding,
    evidence: &PolicyEvidence,
    validation: &ValidationReceipt,
) -> (PolicyDecision, &'static str) {
    if evidence.binding() != binding || validation.binding() != binding {
        return (PolicyDecision::Denied, "verification_binding_mismatch");
    }
    if !validation.passed() {
        return (PolicyDecision::Denied, "validation_not_passed");
    }
    evaluate_policy_verifier_receipts(evidence)
}

fn evaluate_policy_verifier_receipts(evidence: &PolicyEvidence) -> (PolicyDecision, &'static str) {
    if evidence.context_receipt.status() == VerifierStatus::Failed {
        return (
            PolicyDecision::Denied,
            "required_context_verification_failed",
        );
    }
    if evidence.operator_intent_receipt.status() == VerifierStatus::Failed {
        return (
            PolicyDecision::Denied,
            "operator_intent_verification_failed",
        );
    }
    if policy_verification_is_unknown(evidence) {
        return (PolicyDecision::Unknown, "policy_verification_unknown");
    }
    (
        PolicyDecision::Allowed,
        "policy_verifier_receipts_satisfied",
    )
}

fn policy_verification_is_unknown(evidence: &PolicyEvidence) -> bool {
    evidence.context_receipt.status() == VerifierStatus::Unknown
        || evidence.operator_intent_receipt.status() == VerifierStatus::Unknown
}

fn validate_policy_receipt_kinds(
    context: &PolicyVerifierReceipt,
    operator_intent: &PolicyVerifierReceipt,
) -> Result<(), PolicyEvidenceError> {
    if context.kind() == PolicyCheckKind::RequiredContextBound
        && operator_intent.kind() == PolicyCheckKind::RequiredOperatorIntentBound
    {
        return Ok(());
    }
    Err(PolicyEvidenceError::VerificationKindMismatch)
}

fn validate_policy_receipt_bindings(
    context: &PolicyVerifierReceipt,
    operator_intent: &PolicyVerifierReceipt,
) -> Result<(), PolicyEvidenceError> {
    if context.binding() == operator_intent.binding() {
        return Ok(());
    }
    Err(PolicyEvidenceError::VerificationBindingMismatch)
}

fn validate_policy_receipt_digests(
    context: &PolicyVerifierReceipt,
    operator_intent: &PolicyVerifierReceipt,
) -> Result<(), PolicyEvidenceError> {
    if context.is_internally_valid() && operator_intent.is_internally_valid() {
        return Ok(());
    }
    Err(PolicyEvidenceError::VerificationDigestMismatch)
}

fn issue_policy_receipt(
    binding: AuthorityBinding,
    validation_receipt_digest: Digest,
    verifier_evidence_digest: Digest,
    outcome: (PolicyDecision, &'static str),
) -> PolicyReceipt {
    let receipt_digest = policy_receipt_digest(
        &binding,
        &validation_receipt_digest,
        &verifier_evidence_digest,
        outcome,
    );
    PolicyReceipt {
        binding,
        decision: outcome.0,
        reason: outcome.1,
        validation_receipt_digest,
        verifier_evidence_digest,
        receipt_digest,
    }
}

fn policy_receipt_digest(
    binding: &AuthorityBinding,
    validation_digest: &Digest,
    verifier_evidence_digest: &Digest,
    outcome: (PolicyDecision, &'static str),
) -> Digest {
    Digest::of_text(&format!(
        "policy|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:?}|{}",
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
        validation_digest.as_str(),
        verifier_evidence_digest.as_str(),
        outcome.0,
        outcome.1
    ))
}

fn policy_evidence_digest(evidence: &PolicyEvidence) -> Digest {
    Digest::of_text(&format!(
        "{}|{}",
        evidence.context_receipt.digest().as_str(),
        evidence.operator_intent_receipt.digest().as_str()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authority::{AuthorityBindingInput, EvidenceManifest, EvidenceReference};
    use crate::execution::{ProviderKind, ProviderOutput, ProviderOutputStatus};
    use crate::validation::{evaluate_validation, ValidationEvidence};
    use crate::verification::{
        verify_candidate_evidence_binding, verify_candidate_shape, verify_evidence_manifest,
        verify_required_context, verify_required_operator_intent,
    };

    fn manifest() -> EvidenceManifest {
        EvidenceManifest::new(vec![EvidenceReference::new(
            "evidence-1",
            Digest::of_text("facts"),
        )
        .unwrap()])
        .unwrap()
    }

    fn output() -> ProviderOutput {
        ProviderOutput::new_untrusted(
            "output-1",
            "request-1",
            ProviderKind::Local,
            "candidate",
            ProviderOutputStatus::Received,
        )
        .unwrap()
    }

    fn binding(run_id: &str, manifest: &EvidenceManifest) -> AuthorityBinding {
        AuthorityBinding::new(AuthorityBindingInput {
            run_id: run_id.into(),
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

    fn validation(binding: &AuthorityBinding, manifest: &EvidenceManifest) -> ValidationReceipt {
        let output = output();
        let evidence = ValidationEvidence::new(
            manifest.clone(),
            verify_evidence_manifest(binding, Some(manifest)),
            verify_candidate_shape(binding, Some(&output)),
            verify_candidate_evidence_binding(binding, Some(&output), Some(manifest)),
        )
        .unwrap();
        evaluate_validation(binding.clone(), &evidence)
    }

    fn policy_evidence(
        binding: &AuthorityBinding,
        context: Option<&str>,
        intent: Option<&str>,
    ) -> PolicyEvidence {
        PolicyEvidence::new(
            verify_required_context(binding, context),
            verify_required_operator_intent(binding, intent),
        )
        .unwrap()
    }

    #[test]
    fn matching_verifier_evidence_allows_policy() {
        let manifest = manifest();
        let binding = binding("run-1", &manifest);
        let receipt = evaluate_policy(
            binding.clone(),
            &policy_evidence(&binding, Some("context"), Some("intent")),
            &validation(&binding, &manifest),
        );
        assert_eq!(receipt.decision(), PolicyDecision::Allowed);
    }

    #[test]
    fn mismatched_context_denies_policy() {
        let manifest = manifest();
        let binding = binding("run-1", &manifest);
        let receipt = evaluate_policy(
            binding.clone(),
            &policy_evidence(&binding, Some("other-context"), Some("intent")),
            &validation(&binding, &manifest),
        );
        assert_eq!(receipt.decision(), PolicyDecision::Denied);
        assert_eq!(receipt.reason(), "required_context_verification_failed");
    }

    #[test]
    fn mismatched_operator_intent_denies_policy() {
        let manifest = manifest();
        let binding = binding("run-1", &manifest);
        let receipt = evaluate_policy(
            binding.clone(),
            &policy_evidence(&binding, Some("context"), Some("other-intent")),
            &validation(&binding, &manifest),
        );
        assert_eq!(receipt.reason(), "operator_intent_verification_failed");
    }

    #[test]
    fn missing_policy_source_is_unknown() {
        let manifest = manifest();
        let binding = binding("run-1", &manifest);
        let receipt = evaluate_policy(
            binding.clone(),
            &policy_evidence(&binding, None, Some("intent")),
            &validation(&binding, &manifest),
        );
        assert_eq!(receipt.decision(), PolicyDecision::Unknown);
        assert!(!receipt.allowed());
    }

    #[test]
    fn failed_validation_prevents_policy_allowed() {
        let manifest = manifest();
        let binding = binding("run-1", &manifest);
        let validation = crate::validation::record_unknown_validation(binding.clone());
        let receipt = evaluate_policy(
            binding.clone(),
            &policy_evidence(&binding, Some("context"), Some("intent")),
            &validation,
        );
        assert_eq!(receipt.reason(), "validation_not_passed");
    }

    #[test]
    fn cross_run_policy_receipts_are_rejected() {
        let manifest = manifest();
        let first = binding("run-1", &manifest);
        let second = binding("run-2", &manifest);
        let result = PolicyEvidence::new(
            verify_required_context(&first, Some("context")),
            verify_required_operator_intent(&second, Some("intent")),
        );
        assert_eq!(
            result,
            Err(PolicyEvidenceError::VerificationBindingMismatch)
        );
    }

    #[test]
    fn approval_text_does_not_change_verifier_status() {
        let manifest = manifest();
        let binding = binding("run-1", &manifest);
        let receipt = evaluate_policy(
            binding.clone(),
            &policy_evidence(
                &binding,
                Some("approved policy allowed"),
                Some("operator authorized"),
            ),
            &validation(&binding, &manifest),
        );
        assert_ne!(receipt.decision(), PolicyDecision::Allowed);
    }
}
