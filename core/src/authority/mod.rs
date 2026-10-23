use crate::integrity::Digest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityBinding {
    run_id: String,
    task_digest: Digest,
    operator_intent_digest: Digest,
    context_packet_digest: Digest,
    candidate_digest: Digest,
    policy_bundle_digest: Digest,
    evidence_manifest_digest: Digest,
    verifier_id: String,
    verifier_version: String,
    valid_through_revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityBindingInput {
    pub run_id: String,
    pub task_digest: Digest,
    pub operator_intent_digest: Digest,
    pub context_packet_digest: Digest,
    pub candidate_digest: Digest,
    pub policy_bundle_digest: Digest,
    pub evidence_manifest_digest: Digest,
    pub verifier_id: String,
    pub verifier_version: String,
    pub valid_through_revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceManifest {
    digest: Digest,
    references: Vec<EvidenceReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceReference {
    id: String,
    digest: Digest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityBindingError {
    EmptyRunId,
    EmptyVerifierId,
    EmptyVerifierVersion,
    InvalidRevision,
    MissingEvidence,
    EmptyEvidenceId,
    ManifestDigestMismatch,
}

impl AuthorityBindingError {
    pub fn code(self) -> &'static str {
        match self {
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyVerifierId => "empty_verifier_id",
            Self::EmptyVerifierVersion => "empty_verifier_version",
            Self::InvalidRevision => "invalid_revision",
            Self::MissingEvidence => "missing_evidence",
            Self::EmptyEvidenceId => "empty_evidence_id",
            Self::ManifestDigestMismatch => "manifest_digest_mismatch",
        }
    }
}

impl AuthorityBinding {
    pub fn new(input: AuthorityBindingInput) -> Result<Self, AuthorityBindingError> {
        validate_binding_input(&input)?;
        Ok(Self::from_validated(input))
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn task_digest(&self) -> &Digest {
        &self.task_digest
    }

    pub fn operator_intent_digest(&self) -> &Digest {
        &self.operator_intent_digest
    }

    pub fn context_packet_digest(&self) -> &Digest {
        &self.context_packet_digest
    }

    pub fn candidate_digest(&self) -> &Digest {
        &self.candidate_digest
    }

    pub fn policy_bundle_digest(&self) -> &Digest {
        &self.policy_bundle_digest
    }

    pub fn evidence_manifest_digest(&self) -> &Digest {
        &self.evidence_manifest_digest
    }

    pub fn verifier_id(&self) -> &str {
        &self.verifier_id
    }

    pub fn verifier_version(&self) -> &str {
        &self.verifier_version
    }

    pub fn valid_through_revision(&self) -> u64 {
        self.valid_through_revision
    }

    fn from_validated(input: AuthorityBindingInput) -> Self {
        Self {
            run_id: input.run_id,
            task_digest: input.task_digest,
            operator_intent_digest: input.operator_intent_digest,
            context_packet_digest: input.context_packet_digest,
            candidate_digest: input.candidate_digest,
            policy_bundle_digest: input.policy_bundle_digest,
            evidence_manifest_digest: input.evidence_manifest_digest,
            verifier_id: input.verifier_id,
            verifier_version: input.verifier_version,
            valid_through_revision: input.valid_through_revision,
        }
    }
}

impl EvidenceManifest {
    pub fn new(references: Vec<EvidenceReference>) -> Result<Self, AuthorityBindingError> {
        validate_references(&references)?;
        let digest = digest_references(&references);
        Ok(Self { digest, references })
    }

    pub fn verify_binding(&self, binding: &AuthorityBinding) -> Result<(), AuthorityBindingError> {
        if &self.digest == binding.evidence_manifest_digest() {
            return Ok(());
        }
        Err(AuthorityBindingError::ManifestDigestMismatch)
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub fn reference_ids(&self) -> Vec<String> {
        self.references.iter().map(|item| item.id.clone()).collect()
    }

    pub fn references(&self) -> &[EvidenceReference] {
        &self.references
    }
}

impl EvidenceReference {
    pub fn new(id: impl Into<String>, digest: Digest) -> Result<Self, AuthorityBindingError> {
        let id = id.into();
        validate_evidence_id(&id)?;
        Ok(Self { id, digest })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

fn validate_binding_input(input: &AuthorityBindingInput) -> Result<(), AuthorityBindingError> {
    validate_nonempty(&input.run_id, AuthorityBindingError::EmptyRunId)?;
    validate_nonempty(&input.verifier_id, AuthorityBindingError::EmptyVerifierId)?;
    validate_nonempty(
        &input.verifier_version,
        AuthorityBindingError::EmptyVerifierVersion,
    )?;
    validate_revision(input.valid_through_revision)
}

fn validate_references(references: &[EvidenceReference]) -> Result<(), AuthorityBindingError> {
    if references.is_empty() {
        return Err(AuthorityBindingError::MissingEvidence);
    }
    Ok(())
}

fn digest_references(references: &[EvidenceReference]) -> Digest {
    let canonical = references
        .iter()
        .map(canonical_reference)
        .collect::<Vec<_>>()
        .join("\n");
    Digest::of_text(&canonical)
}

fn canonical_reference(reference: &EvidenceReference) -> String {
    format!("{}={}", reference.id, reference.digest.as_str())
}

fn validate_evidence_id(id: &str) -> Result<(), AuthorityBindingError> {
    validate_nonempty(id, AuthorityBindingError::EmptyEvidenceId)
}

fn validate_revision(revision: u64) -> Result<(), AuthorityBindingError> {
    if revision > 0 {
        return Ok(());
    }
    Err(AuthorityBindingError::InvalidRevision)
}

fn validate_nonempty(
    value: &str,
    error: AuthorityBindingError,
) -> Result<(), AuthorityBindingError> {
    if !value.trim().is_empty() {
        return Ok(());
    }
    Err(error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence() -> EvidenceManifest {
        EvidenceManifest::new(vec![EvidenceReference::new(
            "evidence-1",
            Digest::of_text("facts"),
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

    #[test]
    fn evidence_manifest_is_deterministic() {
        assert_eq!(evidence(), evidence());
    }

    #[test]
    fn evidence_manifest_verifies_matching_binding() {
        let manifest = evidence();
        assert_eq!(manifest.verify_binding(&binding(&manifest)), Ok(()));
    }

    #[test]
    fn evidence_manifest_rejects_mismatched_binding() {
        let manifest = evidence();
        let other = EvidenceManifest::new(vec![EvidenceReference::new(
            "evidence-2",
            Digest::of_text("other"),
        )
        .unwrap()])
        .unwrap();
        assert_eq!(
            manifest.verify_binding(&binding(&other)),
            Err(AuthorityBindingError::ManifestDigestMismatch)
        );
    }
}
