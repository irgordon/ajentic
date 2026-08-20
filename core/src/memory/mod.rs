#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryStatus {
    Proposed,
    Quarantined,
    Verified,
    Active,
    Superseded,
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    ProjectFact,
    Convention,
    OperatorNote,
    RunObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryProposerKind {
    Model,
    Human,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryTrustClassification {
    Untrusted,
    VerifiedEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryProvenance {
    pub source: String,
    pub created_by: String,
    pub created_at: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryIntegrityMetadata {
    pub content_digest: crate::integrity::Digest,
    pub source_digest: crate::integrity::Digest,
    pub evidence_refs: Vec<String>,
    pub source_trust: MemoryTrustClassification,
    pub proposer_kind: MemoryProposerKind,
    pub proposed_by: String,
    pub expiry_or_review_date: String,
    pub conflicts: Vec<String>,
    pub supersedes: Option<String>,
    pub protected_key: bool,
    pub snapshot_id: String,
    pub rollback_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryEntry {
    id: String,
    memory_type: MemoryType,
    content: String,
    provenance: MemoryProvenance,
    status: MemoryStatus,
    integrity: MemoryIntegrityMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedMemory {
    entry: MemoryEntry,
    receipt: MemoryVerificationReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryVerificationReceipt {
    memory_id: String,
    content_digest: crate::integrity::Digest,
    verified_by: String,
    verifier_version: String,
    evidence_manifest_digest: crate::integrity::Digest,
    receipt_digest: crate::integrity::Digest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemorySnapshot {
    pub id: String,
    pub created_at: String,
    pub entries: Vec<MemoryEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryError {
    EmptyMemoryId,
    EmptyContent,
    EmptyProvenanceSource,
    EmptyProvenanceCreatedBy,
    EmptyProvenanceCreatedAt,
    EmptyProvenanceReason,
    EmptyProposedBy,
    MissingEvidenceRefs,
    EmptyReviewDate,
    EmptySnapshotId,
    EmptyRollbackRef,
    EmptySnapshotCreatedAt,
    ContentDigestMismatch,
    EvidenceManifestMismatch,
    InvalidVerificationState,
    SelfVerificationNotAllowed,
    EmptyVerifier,
    VerificationReceiptMismatch,
}

impl MemoryError {
    pub fn code(self) -> &'static str {
        match self {
            Self::EmptyMemoryId => "empty_memory_id",
            Self::EmptyContent => "empty_content",
            Self::EmptyProvenanceSource => "empty_provenance_source",
            Self::EmptyProvenanceCreatedBy => "empty_provenance_created_by",
            Self::EmptyProvenanceCreatedAt => "empty_provenance_created_at",
            Self::EmptyProvenanceReason => "empty_provenance_reason",
            Self::EmptyProposedBy => "empty_proposed_by",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::EmptyReviewDate => "empty_review_date",
            Self::EmptySnapshotId => "empty_snapshot_id",
            Self::EmptyRollbackRef => "empty_rollback_ref",
            Self::EmptySnapshotCreatedAt => "empty_snapshot_created_at",
            Self::ContentDigestMismatch => "content_digest_mismatch",
            Self::EvidenceManifestMismatch => "evidence_manifest_mismatch",
            Self::InvalidVerificationState => "invalid_verification_state",
            Self::SelfVerificationNotAllowed => "self_verification_not_allowed",
            Self::EmptyVerifier => "empty_verifier",
            Self::VerificationReceiptMismatch => "verification_receipt_mismatch",
        }
    }
}

impl MemoryProvenance {
    pub fn new(
        source: impl Into<String>,
        created_by: impl Into<String>,
        created_at: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<Self, MemoryError> {
        let values = (
            source.into(),
            created_by.into(),
            created_at.into(),
            reason.into(),
        );
        validate_provenance(&values)?;
        Ok(Self {
            source: values.0,
            created_by: values.1,
            created_at: values.2,
            reason: values.3,
        })
    }
}

impl MemoryEntry {
    pub fn propose(
        id: impl Into<String>,
        memory_type: MemoryType,
        content: impl Into<String>,
        provenance: MemoryProvenance,
        integrity: MemoryIntegrityMetadata,
    ) -> Result<Self, MemoryError> {
        let values = (id.into(), content.into());
        validate_memory_proposal(&values, &integrity)?;
        Ok(Self {
            id: values.0,
            memory_type,
            content: values.1,
            provenance,
            status: MemoryStatus::Proposed,
            integrity,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn memory_type(&self) -> MemoryType {
        self.memory_type
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn provenance(&self) -> &MemoryProvenance {
        &self.provenance
    }

    pub fn status(&self) -> MemoryStatus {
        self.status
    }

    pub fn integrity(&self) -> &MemoryIntegrityMetadata {
        &self.integrity
    }

    pub fn is_active(&self) -> bool {
        self.status == MemoryStatus::Active
    }
}

impl MemoryVerificationReceipt {
    pub fn verified_by(&self) -> &str {
        &self.verified_by
    }

    pub fn digest(&self) -> &crate::integrity::Digest {
        &self.receipt_digest
    }
}

impl MemorySnapshot {
    pub fn new(
        id: impl Into<String>,
        created_at: impl Into<String>,
        entries: Vec<MemoryEntry>,
    ) -> Result<Self, MemoryError> {
        let values = (id.into(), created_at.into());
        validate_snapshot_identity(&values)?;
        Ok(Self {
            id: values.0,
            created_at: values.1,
            entries,
        })
    }

    pub fn active_entries(&self) -> Vec<&MemoryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.is_active())
            .collect()
    }
}

pub fn verify_memory_entry(
    mut entry: MemoryEntry,
    verified_by: impl Into<String>,
    verifier_version: impl Into<String>,
    manifest: &crate::authority::EvidenceManifest,
) -> Result<VerifiedMemory, MemoryError> {
    let values = (verified_by.into(), verifier_version.into());
    validate_memory_verification(&entry, &values, manifest)?;
    let receipt = issue_memory_verification_receipt(&entry, values, manifest);
    entry.status = MemoryStatus::Verified;
    entry.integrity.source_trust = MemoryTrustClassification::VerifiedEvidence;
    Ok(VerifiedMemory { entry, receipt })
}

pub fn activate_verified_memory(verified: VerifiedMemory) -> Result<MemoryEntry, MemoryError> {
    validate_verification_receipt(&verified)?;
    let mut entry = verified.entry;
    entry.status = MemoryStatus::Active;
    Ok(entry)
}

fn validate_provenance(values: &(String, String, String, String)) -> Result<(), MemoryError> {
    validate_memory_text(&values.0, MemoryError::EmptyProvenanceSource)?;
    validate_memory_text(&values.1, MemoryError::EmptyProvenanceCreatedBy)?;
    validate_memory_text(&values.2, MemoryError::EmptyProvenanceCreatedAt)?;
    validate_memory_text(&values.3, MemoryError::EmptyProvenanceReason)
}

fn validate_memory_proposal(
    values: &(String, String),
    integrity: &MemoryIntegrityMetadata,
) -> Result<(), MemoryError> {
    validate_memory_text(&values.0, MemoryError::EmptyMemoryId)?;
    validate_memory_text(&values.1, MemoryError::EmptyContent)?;
    validate_memory_integrity(values, integrity)
}

fn validate_memory_integrity(
    values: &(String, String),
    integrity: &MemoryIntegrityMetadata,
) -> Result<(), MemoryError> {
    validate_memory_text(&integrity.proposed_by, MemoryError::EmptyProposedBy)?;
    validate_memory_text(
        &integrity.expiry_or_review_date,
        MemoryError::EmptyReviewDate,
    )?;
    validate_memory_text(&integrity.snapshot_id, MemoryError::EmptySnapshotId)?;
    validate_memory_text(&integrity.rollback_ref, MemoryError::EmptyRollbackRef)?;
    if integrity.evidence_refs.is_empty() {
        return Err(MemoryError::MissingEvidenceRefs);
    }
    if integrity.content_digest != crate::integrity::Digest::of_text(&values.1) {
        return Err(MemoryError::ContentDigestMismatch);
    }
    Ok(())
}

fn validate_snapshot_identity(values: &(String, String)) -> Result<(), MemoryError> {
    validate_memory_text(&values.0, MemoryError::EmptySnapshotId)?;
    validate_memory_text(&values.1, MemoryError::EmptySnapshotCreatedAt)
}

fn validate_memory_verification(
    entry: &MemoryEntry,
    verifier: &(String, String),
    manifest: &crate::authority::EvidenceManifest,
) -> Result<(), MemoryError> {
    if entry.status != MemoryStatus::Proposed && entry.status != MemoryStatus::Quarantined {
        return Err(MemoryError::InvalidVerificationState);
    }
    validate_memory_text(&verifier.0, MemoryError::EmptyVerifier)?;
    validate_memory_text(&verifier.1, MemoryError::EmptyVerifier)?;
    validate_independent_verifier(entry, &verifier.0)?;
    validate_memory_manifest(entry, manifest)
}

fn validate_independent_verifier(
    entry: &MemoryEntry,
    verified_by: &str,
) -> Result<(), MemoryError> {
    if entry.integrity.proposer_kind == MemoryProposerKind::Model
        && entry.integrity.proposed_by == verified_by
    {
        return Err(MemoryError::SelfVerificationNotAllowed);
    }
    Ok(())
}

fn validate_memory_manifest(
    entry: &MemoryEntry,
    manifest: &crate::authority::EvidenceManifest,
) -> Result<(), MemoryError> {
    let manifest_ids = manifest.reference_ids();
    if entry
        .integrity
        .evidence_refs
        .iter()
        .all(|item| manifest_ids.contains(item))
    {
        return Ok(());
    }
    Err(MemoryError::EvidenceManifestMismatch)
}

fn issue_memory_verification_receipt(
    entry: &MemoryEntry,
    verifier: (String, String),
    manifest: &crate::authority::EvidenceManifest,
) -> MemoryVerificationReceipt {
    let receipt_digest = memory_verification_digest(entry, &verifier, manifest);
    MemoryVerificationReceipt {
        memory_id: entry.id.clone(),
        content_digest: entry.integrity.content_digest.clone(),
        verified_by: verifier.0,
        verifier_version: verifier.1,
        evidence_manifest_digest: manifest.digest().clone(),
        receipt_digest,
    }
}

fn memory_verification_digest(
    entry: &MemoryEntry,
    verifier: &(String, String),
    manifest: &crate::authority::EvidenceManifest,
) -> crate::integrity::Digest {
    crate::integrity::Digest::of_text(&format!(
        "memory|{}|{}|{}|{}|{}",
        entry.id,
        entry.integrity.content_digest.as_str(),
        verifier.0,
        verifier.1,
        manifest.digest().as_str()
    ))
}

fn validate_verification_receipt(verified: &VerifiedMemory) -> Result<(), MemoryError> {
    let entry = &verified.entry;
    let receipt = &verified.receipt;
    if entry.status == MemoryStatus::Verified
        && entry.id == receipt.memory_id
        && entry.integrity.content_digest == receipt.content_digest
        && !receipt.verified_by.is_empty()
        && !receipt.verifier_version.is_empty()
        && receipt
            .evidence_manifest_digest
            .as_str()
            .starts_with("sha256:")
    {
        return Ok(());
    }
    Err(MemoryError::VerificationReceiptMismatch)
}

fn validate_memory_text(value: &str, error: MemoryError) -> Result<(), MemoryError> {
    if !value.trim().is_empty() {
        return Ok(());
    }
    Err(error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provenance() -> MemoryProvenance {
        MemoryProvenance::new("source", "model", "2026-08-19T00:00:00Z", "proposal").unwrap()
    }

    fn manifest() -> crate::authority::EvidenceManifest {
        crate::authority::EvidenceManifest::new(vec![crate::authority::EvidenceReference::new(
            "evidence-1",
            crate::integrity::Digest::of_text("facts"),
        )
        .unwrap()])
        .unwrap()
    }

    fn proposal() -> MemoryEntry {
        let content = "project fact";
        MemoryEntry::propose(
            "memory-1",
            MemoryType::ProjectFact,
            content,
            provenance(),
            MemoryIntegrityMetadata {
                content_digest: crate::integrity::Digest::of_text(content),
                source_digest: crate::integrity::Digest::of_text("source"),
                evidence_refs: vec!["evidence-1".into()],
                source_trust: MemoryTrustClassification::Untrusted,
                proposer_kind: MemoryProposerKind::Model,
                proposed_by: "model".into(),
                expiry_or_review_date: "2026-09-19".into(),
                conflicts: Vec::new(),
                supersedes: None,
                protected_key: false,
                snapshot_id: "snapshot-1".into(),
                rollback_ref: "rollback-1".into(),
            },
        )
        .unwrap()
    }

    #[test]
    fn model_memory_starts_proposed() {
        assert_eq!(proposal().status(), MemoryStatus::Proposed);
        assert!(!proposal().is_active());
    }

    #[test]
    fn model_cannot_verify_its_own_memory() {
        let result = verify_memory_entry(proposal(), "model", "1", &manifest());
        assert_eq!(result, Err(MemoryError::SelfVerificationNotAllowed));
    }

    #[test]
    fn independent_verification_allows_activation() {
        let verified = verify_memory_entry(proposal(), "operator", "1", &manifest()).unwrap();
        let active = activate_verified_memory(verified).unwrap();
        assert_eq!(active.status(), MemoryStatus::Active);
    }

    #[test]
    fn snapshot_filters_active_memory() {
        let verified = verify_memory_entry(proposal(), "operator", "1", &manifest()).unwrap();
        let active = activate_verified_memory(verified).unwrap();
        let snapshot = MemorySnapshot::new("snapshot-1", "2026-08-19", vec![active]).unwrap();
        assert_eq!(snapshot.active_entries().len(), 1);
    }
}
