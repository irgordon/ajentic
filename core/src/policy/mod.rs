use crate::authority::AuthorityBinding;
use crate::integrity::Digest;
use crate::validation::ValidationReceipt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allowed,
    Denied,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyEvidence {
    has_required_context: bool,
    has_required_operator_intent: bool,
    model_output_claims_success: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyReceipt {
    binding: AuthorityBinding,
    decision: PolicyDecision,
    reason: &'static str,
    validation_receipt_digest: Digest,
    receipt_digest: Digest,
}

impl PolicyEvidence {
    pub fn new(
        has_required_context: bool,
        has_required_operator_intent: bool,
        model_output_claims_success: bool,
    ) -> Self {
        Self {
            has_required_context,
            has_required_operator_intent,
            model_output_claims_success,
        }
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
    issue_policy_receipt(binding, validation.digest().clone(), outcome)
}

pub fn record_unknown_policy(
    binding: AuthorityBinding,
    validation: &ValidationReceipt,
) -> PolicyReceipt {
    issue_policy_receipt(
        binding,
        validation.digest().clone(),
        (PolicyDecision::Unknown, "unknown_is_not_pass"),
    )
}

fn determine_policy_outcome(
    binding: &AuthorityBinding,
    evidence: &PolicyEvidence,
    validation: &ValidationReceipt,
) -> (PolicyDecision, &'static str) {
    if validation.binding() != binding {
        return (PolicyDecision::Denied, "validation_binding_mismatch");
    }
    if !validation.passed() {
        return (PolicyDecision::Denied, "validation_not_passed");
    }
    evaluate_policy_evidence(evidence)
}

fn evaluate_policy_evidence(evidence: &PolicyEvidence) -> (PolicyDecision, &'static str) {
    let _advisory_model_claim = evidence.model_output_claims_success;
    if !evidence.has_required_context {
        return (PolicyDecision::Denied, "missing_required_context");
    }
    if !evidence.has_required_operator_intent {
        return (PolicyDecision::Denied, "missing_required_operator_intent");
    }
    (PolicyDecision::Allowed, "required_policy_evidence_present")
}

fn issue_policy_receipt(
    binding: AuthorityBinding,
    validation_receipt_digest: Digest,
    outcome: (PolicyDecision, &'static str),
) -> PolicyReceipt {
    let receipt_digest = policy_receipt_digest(&binding, &validation_receipt_digest, outcome);
    PolicyReceipt {
        binding,
        decision: outcome.0,
        reason: outcome.1,
        validation_receipt_digest,
        receipt_digest,
    }
}

fn policy_receipt_digest(
    binding: &AuthorityBinding,
    validation_digest: &Digest,
    outcome: (PolicyDecision, &'static str),
) -> Digest {
    Digest::of_text(&format!(
        "policy|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:?}|{}",
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
        outcome.0,
        outcome.1
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authority::{AuthorityBindingInput, EvidenceManifest, EvidenceReference};
    use crate::validation::{evaluate_validation, ValidationEvidence};

    fn manifest(label: &str) -> EvidenceManifest {
        EvidenceManifest::new(vec![EvidenceReference::new(
            "evidence-1",
            Digest::of_text(label),
        )
        .unwrap()])
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

    fn passing_validation(
        binding: AuthorityBinding,
        manifest: EvidenceManifest,
    ) -> ValidationReceipt {
        evaluate_validation(
            binding,
            &ValidationEvidence::new(true, true, true, false, manifest),
        )
    }

    fn evidence() -> PolicyEvidence {
        PolicyEvidence::new(true, true, false)
    }

    #[test]
    fn policy_required_evidence_allows() {
        let manifest = manifest("facts");
        let binding = binding("run-1", &manifest);
        let validation = passing_validation(binding.clone(), manifest);
        let receipt = evaluate_policy(binding, &evidence(), &validation);
        assert_eq!(receipt.decision(), PolicyDecision::Allowed);
    }

    #[test]
    fn policy_rejects_cross_run_validation_receipt() {
        let manifest = manifest("facts");
        let validation_binding = binding("run-1", &manifest);
        let policy_binding = binding("run-2", &manifest);
        let validation = passing_validation(validation_binding, manifest);
        let receipt = evaluate_policy(policy_binding, &evidence(), &validation);
        assert_eq!(receipt.reason(), "validation_binding_mismatch");
    }

    #[test]
    fn policy_rejects_failed_validation_receipt() {
        let manifest = manifest("facts");
        let binding = binding("run-1", &manifest);
        let validation = evaluate_validation(
            binding.clone(),
            &ValidationEvidence::new(false, true, true, true, manifest),
        );
        let receipt = evaluate_policy(binding, &evidence(), &validation);
        assert_eq!(receipt.reason(), "validation_not_passed");
    }

    #[test]
    fn policy_missing_context_denies() {
        let manifest = manifest("facts");
        let binding = binding("run-1", &manifest);
        let validation = passing_validation(binding.clone(), manifest);
        let receipt = evaluate_policy(
            binding,
            &PolicyEvidence::new(false, true, false),
            &validation,
        );
        assert_eq!(receipt.reason(), "missing_required_context");
    }

    #[test]
    fn policy_model_claim_does_not_override_missing_intent() {
        let manifest = manifest("facts");
        let binding = binding("run-1", &manifest);
        let validation = passing_validation(binding.clone(), manifest);
        let receipt = evaluate_policy(
            binding,
            &PolicyEvidence::new(true, false, true),
            &validation,
        );
        assert_eq!(receipt.decision(), PolicyDecision::Denied);
        assert_eq!(receipt.reason(), "missing_required_operator_intent");
    }

    #[test]
    fn policy_unknown_is_not_allowed() {
        let manifest = manifest("facts");
        let binding = binding("run-1", &manifest);
        let validation = passing_validation(binding.clone(), manifest);
        let receipt = record_unknown_policy(binding, &validation);
        assert!(!receipt.allowed());
        assert_eq!(receipt.decision(), PolicyDecision::Unknown);
    }
}
