mod common;

use ajentic_core::integrity::Digest;
use ajentic_core::memory::{
    activate_verified_memory, verify_memory_entry, MemoryEntry, MemoryError,
    MemoryIntegrityMetadata, MemoryProposerKind, MemoryProvenance, MemoryStatus,
    MemoryTrustClassification, MemoryType,
};

fn proposal() -> MemoryEntry {
    let content = "model supplied fact";
    MemoryEntry::propose(
        "memory-1",
        MemoryType::ProjectFact,
        content,
        MemoryProvenance::new("model", "model", "2026-08-19", "proposal").unwrap(),
        MemoryIntegrityMetadata {
            content_digest: Digest::of_text(content),
            source_digest: Digest::of_text("model-source"),
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
fn model_generated_memory_cannot_activate_itself() {
    assert_eq!(
        verify_memory_entry(proposal(), "model", "1", &common::evidence_manifest()),
        Err(MemoryError::SelfVerificationNotAllowed)
    );
}

#[test]
fn independent_verification_is_required_for_active_state() {
    let verified =
        verify_memory_entry(proposal(), "operator", "1", &common::evidence_manifest()).unwrap();
    let active = activate_verified_memory(verified).unwrap();
    assert_eq!(active.status(), MemoryStatus::Active);
}
