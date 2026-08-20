#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerEventType {
    StateTransition,
    MemoryWrite,
    MemoryDelete,
    ExecutionStart,
    ExecutionEnd,
    ValidationPass,
    ValidationFail,
    PolicyPass,
    PolicyFail,
    ContextCreated,
    CandidateReceived,
    IntentAccepted,
    IntentRejected,
    ReplayRequested,
    AuditExported,
}

impl LedgerEventType {
    pub fn code(&self) -> &'static str {
        match self {
            Self::StateTransition => "state_transition",
            Self::MemoryWrite => "memory_write",
            Self::MemoryDelete => "memory_delete",
            Self::ExecutionStart => "execution_start",
            Self::ExecutionEnd => "execution_end",
            Self::ValidationPass => "validation_pass",
            Self::ValidationFail => "validation_fail",
            Self::PolicyPass => "policy_pass",
            Self::PolicyFail => "policy_fail",
            Self::ContextCreated => "context_created",
            Self::CandidateReceived => "candidate_received",
            Self::IntentAccepted => "intent_accepted",
            Self::IntentRejected => "intent_rejected",
            Self::ReplayRequested => "replay_requested",
            Self::AuditExported => "audit_exported",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerActorType {
    System,
    Human,
    Ui,
    Cli,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerActor {
    pub actor_type: LedgerActorType,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerPayload {
    pub summary: String,
    pub lifecycle_transition: Option<crate::state::LifecycleState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEvent {
    pub id: String,
    pub revision: u64,
    pub event_type: LedgerEventType,
    pub actor: LedgerActor,
    pub evidence_refs: Vec<String>,
    pub payload: LedgerPayload,
    pub integrity: Option<LedgerEventIntegrity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEventIntegrity {
    pub run_id: String,
    pub task_digest: crate::integrity::Digest,
    pub candidate_digest: crate::integrity::Digest,
    pub context_digest: crate::integrity::Digest,
    pub policy_bundle_digest: crate::integrity::Digest,
    pub previous_event_hash: crate::integrity::Digest,
    pub event_hash: crate::integrity::Digest,
    pub payload_hash: crate::integrity::Digest,
    pub causal_parent_refs: Vec<String>,
    pub actor_authorization_ref: String,
    pub validation_receipt_ref: String,
    pub policy_receipt_ref: String,
    pub schema_version: String,
    pub verifier_version: String,
    pub evidence_manifest_digest: crate::integrity::Digest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerSeal {
    pub binding: crate::authority::AuthorityBinding,
    pub actor_authorization_ref: String,
    pub validation_receipt_ref: String,
    pub policy_receipt_ref: String,
    pub schema_version: String,
    pub verifier_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ledger {
    events: Vec<LedgerEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    EmptyEventId,
    InvalidRevision,
    EmptyActorId,
    EmptyEvidenceRef,
    MissingEvidenceRefs,
    EmptyPayloadSummary,
    InvalidRevisionSequence,
    MixedIntegrityChain,
    IntegrityFieldMissing,
    IntegrityBindingMismatch,
    PreviousEventHashMismatch,
    PayloadHashMismatch,
    EventHashMismatch,
    CausalParentMismatch,
    LedgerAlreadySealed,
}

impl LedgerError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyEventId => "empty_event_id",
            Self::InvalidRevision => "invalid_revision",
            Self::EmptyActorId => "empty_actor_id",
            Self::EmptyEvidenceRef => "empty_evidence_ref",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::EmptyPayloadSummary => "empty_payload_summary",
            Self::InvalidRevisionSequence => "invalid_revision_sequence",
            Self::MixedIntegrityChain => "mixed_integrity_chain",
            Self::IntegrityFieldMissing => "integrity_field_missing",
            Self::IntegrityBindingMismatch => "integrity_binding_mismatch",
            Self::PreviousEventHashMismatch => "previous_event_hash_mismatch",
            Self::PayloadHashMismatch => "payload_hash_mismatch",
            Self::EventHashMismatch => "event_hash_mismatch",
            Self::CausalParentMismatch => "causal_parent_mismatch",
            Self::LedgerAlreadySealed => "ledger_already_sealed",
        }
    }
}

impl LedgerActor {
    pub fn new(actor_type: LedgerActorType, id: impl Into<String>) -> Result<Self, LedgerError> {
        let id = id.into();
        if id.is_empty() {
            return Err(LedgerError::EmptyActorId);
        }

        Ok(Self { actor_type, id })
    }
}

impl LedgerPayload {
    pub fn new(summary: impl Into<String>) -> Result<Self, LedgerError> {
        let summary = summary.into();
        if summary.is_empty() {
            return Err(LedgerError::EmptyPayloadSummary);
        }

        Ok(Self {
            summary,
            lifecycle_transition: None,
        })
    }

    pub fn with_lifecycle_transition(
        summary: impl Into<String>,
        next_state: crate::state::LifecycleState,
    ) -> Result<Self, LedgerError> {
        let mut payload = Self::new(summary)?;
        payload.lifecycle_transition = Some(next_state);
        Ok(payload)
    }
}

impl LedgerEvent {
    pub fn new(
        id: impl Into<String>,
        revision: u64,
        event_type: LedgerEventType,
        actor: LedgerActor,
        evidence_refs: Vec<String>,
        payload: LedgerPayload,
    ) -> Result<Self, LedgerError> {
        let id = id.into();
        if id.is_empty() {
            return Err(LedgerError::EmptyEventId);
        }
        if revision == 0 {
            return Err(LedgerError::InvalidRevision);
        }
        if evidence_refs.is_empty() {
            return Err(LedgerError::MissingEvidenceRefs);
        }
        if evidence_refs
            .iter()
            .any(|evidence_ref| evidence_ref.is_empty())
        {
            return Err(LedgerError::EmptyEvidenceRef);
        }

        Ok(Self {
            id,
            revision,
            event_type,
            actor,
            evidence_refs,
            payload,
            integrity: None,
        })
    }

    pub fn new_bound(
        event: LedgerEvent,
        seal: &LedgerSeal,
        previous_event_hash: crate::integrity::Digest,
        causal_parent_refs: Vec<String>,
    ) -> Result<Self, LedgerError> {
        validate_unsealed_event(&event)?;
        validate_seal(seal)?;
        let integrity =
            build_event_integrity(&event, seal, previous_event_hash, causal_parent_refs);
        Ok(Self {
            integrity: Some(integrity),
            ..event
        })
    }
}

impl Ledger {
    pub fn empty() -> Self {
        Self { events: Vec::new() }
    }

    pub fn append(&self, event: LedgerEvent) -> Result<Self, LedgerError> {
        let expected_revision = match self.last_revision() {
            Some(last_revision) => last_revision + 1,
            None => 1,
        };

        if event.revision != expected_revision {
            return Err(LedgerError::InvalidRevisionSequence);
        }

        validate_append_integrity(self, &event)?;

        let mut events = self.events.clone();
        events.push(event);
        Ok(Self { events })
    }

    pub fn events(&self) -> &[LedgerEvent] {
        &self.events
    }

    pub fn last_revision(&self) -> Option<u64> {
        self.events.last().map(|event| event.revision)
    }

    pub fn last_event_hash(&self) -> Option<&crate::integrity::Digest> {
        self.events
            .last()
            .and_then(|event| event.integrity.as_ref())
            .map(|integrity| &integrity.event_hash)
    }

    pub fn seal(&self, seal: &LedgerSeal) -> Result<Self, LedgerError> {
        validate_sealable_ledger(self)?;
        seal_ledger_events(self, seal)
    }

    pub fn verify_integrity(
        &self,
        binding: &crate::authority::AuthorityBinding,
        manifest: &crate::authority::EvidenceManifest,
    ) -> Result<(), LedgerError> {
        manifest
            .verify_binding(binding)
            .map_err(|_| LedgerError::IntegrityBindingMismatch)?;
        verify_integrity_chain(self, binding)
    }
}

fn validate_unsealed_event(event: &LedgerEvent) -> Result<(), LedgerError> {
    if event.integrity.is_none() {
        return Ok(());
    }
    Err(LedgerError::LedgerAlreadySealed)
}

fn validate_seal(seal: &LedgerSeal) -> Result<(), LedgerError> {
    if seal.actor_authorization_ref.trim().is_empty()
        || seal.validation_receipt_ref.trim().is_empty()
        || seal.policy_receipt_ref.trim().is_empty()
        || seal.schema_version.trim().is_empty()
        || seal.verifier_version.trim().is_empty()
    {
        return Err(LedgerError::IntegrityFieldMissing);
    }
    Ok(())
}

fn build_event_integrity(
    event: &LedgerEvent,
    seal: &LedgerSeal,
    previous_event_hash: crate::integrity::Digest,
    causal_parent_refs: Vec<String>,
) -> LedgerEventIntegrity {
    let payload_hash = ledger_payload_hash(&event.payload);
    let mut integrity =
        integrity_without_event_hash(seal, previous_event_hash, payload_hash, causal_parent_refs);
    integrity.event_hash = ledger_event_hash(event, &integrity);
    integrity
}

fn integrity_without_event_hash(
    seal: &LedgerSeal,
    previous_event_hash: crate::integrity::Digest,
    payload_hash: crate::integrity::Digest,
    causal_parent_refs: Vec<String>,
) -> LedgerEventIntegrity {
    LedgerEventIntegrity {
        run_id: seal.binding.run_id().to_string(),
        task_digest: seal.binding.task_digest().clone(),
        candidate_digest: seal.binding.candidate_digest().clone(),
        context_digest: seal.binding.context_packet_digest().clone(),
        policy_bundle_digest: seal.binding.policy_bundle_digest().clone(),
        previous_event_hash,
        event_hash: crate::integrity::Digest::of_text("pending"),
        payload_hash,
        causal_parent_refs,
        actor_authorization_ref: seal.actor_authorization_ref.clone(),
        validation_receipt_ref: seal.validation_receipt_ref.clone(),
        policy_receipt_ref: seal.policy_receipt_ref.clone(),
        schema_version: seal.schema_version.clone(),
        verifier_version: seal.verifier_version.clone(),
        evidence_manifest_digest: seal.binding.evidence_manifest_digest().clone(),
    }
}

fn validate_append_integrity(ledger: &Ledger, event: &LedgerEvent) -> Result<(), LedgerError> {
    match (ledger.last_event_hash(), event.integrity.as_ref()) {
        (None, None) if ledger.events.is_empty() => Ok(()),
        (None, None) => Ok(()),
        (Some(_), None) | (None, Some(_)) if !ledger.events.is_empty() => {
            Err(LedgerError::MixedIntegrityChain)
        }
        (Some(_), None) => Err(LedgerError::MixedIntegrityChain),
        (None, Some(integrity)) => validate_genesis_integrity(event, integrity),
        (Some(previous), Some(integrity)) => {
            validate_linked_integrity(ledger, event, previous, integrity)
        }
    }
}

fn validate_genesis_integrity(
    event: &LedgerEvent,
    integrity: &LedgerEventIntegrity,
) -> Result<(), LedgerError> {
    if integrity.previous_event_hash != genesis_event_hash() {
        return Err(LedgerError::PreviousEventHashMismatch);
    }
    if !integrity.causal_parent_refs.is_empty() {
        return Err(LedgerError::CausalParentMismatch);
    }
    validate_event_hashes(event, integrity)
}

fn validate_linked_integrity(
    ledger: &Ledger,
    event: &LedgerEvent,
    previous: &crate::integrity::Digest,
    integrity: &LedgerEventIntegrity,
) -> Result<(), LedgerError> {
    if &integrity.previous_event_hash != previous {
        return Err(LedgerError::PreviousEventHashMismatch);
    }
    validate_causal_parent(ledger, integrity)?;
    validate_event_hashes(event, integrity)
}

fn validate_causal_parent(
    ledger: &Ledger,
    integrity: &LedgerEventIntegrity,
) -> Result<(), LedgerError> {
    let expected = ledger.events.last().map(|event| event.id.as_str());
    if expected.is_some() && integrity.causal_parent_refs.last().map(String::as_str) == expected {
        return Ok(());
    }
    Err(LedgerError::CausalParentMismatch)
}

fn validate_event_hashes(
    event: &LedgerEvent,
    integrity: &LedgerEventIntegrity,
) -> Result<(), LedgerError> {
    if integrity.payload_hash != ledger_payload_hash(&event.payload) {
        return Err(LedgerError::PayloadHashMismatch);
    }
    if integrity.event_hash != ledger_event_hash(event, integrity) {
        return Err(LedgerError::EventHashMismatch);
    }
    Ok(())
}

fn validate_sealable_ledger(ledger: &Ledger) -> Result<(), LedgerError> {
    if ledger.events.iter().any(|event| event.integrity.is_some()) {
        return Err(LedgerError::LedgerAlreadySealed);
    }
    Ok(())
}

fn seal_ledger_events(ledger: &Ledger, seal: &LedgerSeal) -> Result<Ledger, LedgerError> {
    let mut sealed = Ledger::empty();
    for event in &ledger.events {
        let previous = sealed
            .last_event_hash()
            .cloned()
            .unwrap_or_else(genesis_event_hash);
        let parents = sealed
            .events
            .last()
            .map(|event| vec![event.id.clone()])
            .unwrap_or_default();
        let bound = LedgerEvent::new_bound(event.clone(), seal, previous, parents)?;
        sealed = sealed.append(bound)?;
    }
    Ok(sealed)
}

fn verify_integrity_chain(
    ledger: &Ledger,
    binding: &crate::authority::AuthorityBinding,
) -> Result<(), LedgerError> {
    let mut previous = genesis_event_hash();
    for (index, event) in ledger.events.iter().enumerate() {
        let integrity = event
            .integrity
            .as_ref()
            .ok_or(LedgerError::MixedIntegrityChain)?;
        verify_event_binding(integrity, binding)?;
        verify_chain_position(ledger, index, event, integrity, &previous)?;
        previous = integrity.event_hash.clone();
    }
    Ok(())
}

fn verify_event_binding(
    integrity: &LedgerEventIntegrity,
    binding: &crate::authority::AuthorityBinding,
) -> Result<(), LedgerError> {
    if integrity.run_id == binding.run_id()
        && &integrity.task_digest == binding.task_digest()
        && &integrity.candidate_digest == binding.candidate_digest()
        && &integrity.context_digest == binding.context_packet_digest()
        && &integrity.policy_bundle_digest == binding.policy_bundle_digest()
        && &integrity.evidence_manifest_digest == binding.evidence_manifest_digest()
    {
        return Ok(());
    }
    Err(LedgerError::IntegrityBindingMismatch)
}

fn verify_chain_position(
    ledger: &Ledger,
    index: usize,
    event: &LedgerEvent,
    integrity: &LedgerEventIntegrity,
    previous: &crate::integrity::Digest,
) -> Result<(), LedgerError> {
    if &integrity.previous_event_hash != previous {
        return Err(LedgerError::PreviousEventHashMismatch);
    }
    verify_parent_at_index(ledger, index, integrity)?;
    validate_event_hashes(event, integrity)
}

fn verify_parent_at_index(
    ledger: &Ledger,
    index: usize,
    integrity: &LedgerEventIntegrity,
) -> Result<(), LedgerError> {
    if index == 0 && integrity.causal_parent_refs.is_empty() {
        return Ok(());
    }
    let expected = ledger
        .events
        .get(index.wrapping_sub(1))
        .map(|event| event.id.as_str());
    if integrity.causal_parent_refs.last().map(String::as_str) == expected {
        return Ok(());
    }
    Err(LedgerError::CausalParentMismatch)
}

fn ledger_payload_hash(payload: &LedgerPayload) -> crate::integrity::Digest {
    crate::integrity::Digest::of_text(&format!(
        "{}|{:?}",
        payload.summary, payload.lifecycle_transition
    ))
}

fn ledger_event_hash(
    event: &LedgerEvent,
    integrity: &LedgerEventIntegrity,
) -> crate::integrity::Digest {
    crate::integrity::Digest::of_text(&format!(
        "{}|{}|{}|{:?}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        event.id,
        event.revision,
        event.event_type.code(),
        event.actor.actor_type,
        event.actor.id,
        event.evidence_refs.join(","),
        integrity.run_id,
        integrity.task_digest.as_str(),
        integrity.candidate_digest.as_str(),
        integrity.context_digest.as_str(),
        integrity.policy_bundle_digest.as_str(),
        integrity.previous_event_hash.as_str(),
        integrity.payload_hash.as_str(),
        integrity.causal_parent_refs.join(","),
        integrity.actor_authorization_ref,
        integrity.validation_receipt_ref,
        integrity.policy_receipt_ref,
        integrity.evidence_manifest_digest.as_str()
    ))
}

fn genesis_event_hash() -> crate::integrity::Digest {
    crate::integrity::Digest::of_text("AJENTIC_LEDGER_GENESIS")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn actor() -> LedgerActor {
        LedgerActor::new(LedgerActorType::System, "actor-1").expect("actor should be valid")
    }

    fn payload() -> LedgerPayload {
        LedgerPayload::new("summary").expect("payload should be valid")
    }

    fn event(id: &str, revision: u64) -> LedgerEvent {
        LedgerEvent::new(
            id,
            revision,
            LedgerEventType::StateTransition,
            actor(),
            vec!["evidence-1".to_string()],
            payload(),
        )
        .expect("event should be valid")
    }

    #[test]
    fn ledger_actor_requires_id() {
        let result = LedgerActor::new(LedgerActorType::System, "");
        assert_eq!(result, Err(LedgerError::EmptyActorId));
    }

    #[test]
    fn ledger_payload_requires_summary() {
        let result = LedgerPayload::new("");
        assert_eq!(result, Err(LedgerError::EmptyPayloadSummary));
    }

    #[test]
    fn ledger_payload_new_has_no_lifecycle_transition() {
        let payload = LedgerPayload::new("summary").expect("payload should be valid");
        assert_eq!(payload.lifecycle_transition, None);
    }

    #[test]
    fn ledger_payload_with_lifecycle_transition_sets_next_state() {
        let payload = LedgerPayload::with_lifecycle_transition(
            "summary",
            crate::state::LifecycleState::Evaluating,
        )
        .expect("payload should be valid");

        assert_eq!(
            payload.lifecycle_transition,
            Some(crate::state::LifecycleState::Evaluating)
        );
    }

    #[test]
    fn ledger_event_requires_id() {
        let result = LedgerEvent::new(
            "",
            1,
            LedgerEventType::StateTransition,
            actor(),
            vec!["evidence-1".to_string()],
            payload(),
        );
        assert_eq!(result, Err(LedgerError::EmptyEventId));
    }

    #[test]
    fn ledger_event_requires_nonzero_revision() {
        let result = LedgerEvent::new(
            "event-1",
            0,
            LedgerEventType::StateTransition,
            actor(),
            vec!["evidence-1".to_string()],
            payload(),
        );
        assert_eq!(result, Err(LedgerError::InvalidRevision));
    }

    #[test]
    fn stable_code_methods_return_expected_values() {
        assert_eq!(LedgerEventType::StateTransition.code(), "state_transition");
        assert_eq!(LedgerEventType::MemoryWrite.code(), "memory_write");
        assert_eq!(LedgerEventType::MemoryDelete.code(), "memory_delete");
        assert_eq!(LedgerEventType::ExecutionStart.code(), "execution_start");
        assert_eq!(LedgerEventType::ExecutionEnd.code(), "execution_end");
        assert_eq!(LedgerEventType::ValidationPass.code(), "validation_pass");
        assert_eq!(LedgerEventType::ValidationFail.code(), "validation_fail");
        assert_eq!(LedgerEventType::PolicyPass.code(), "policy_pass");
        assert_eq!(LedgerEventType::PolicyFail.code(), "policy_fail");
        assert_eq!(LedgerEventType::ContextCreated.code(), "context_created");
        assert_eq!(
            LedgerEventType::CandidateReceived.code(),
            "candidate_received"
        );
        assert_eq!(LedgerEventType::IntentAccepted.code(), "intent_accepted");
        assert_eq!(LedgerEventType::IntentRejected.code(), "intent_rejected");
        assert_eq!(LedgerEventType::ReplayRequested.code(), "replay_requested");
        assert_eq!(LedgerEventType::AuditExported.code(), "audit_exported");
    }

    #[test]
    fn ledger_event_requires_evidence_refs() {
        let result = LedgerEvent::new(
            "event-1",
            1,
            LedgerEventType::StateTransition,
            actor(),
            Vec::new(),
            payload(),
        );
        assert_eq!(result, Err(LedgerError::MissingEvidenceRefs));
    }

    #[test]
    fn ledger_event_rejects_empty_evidence_ref() {
        let result = LedgerEvent::new(
            "event-1",
            1,
            LedgerEventType::StateTransition,
            actor(),
            vec![String::new()],
            payload(),
        );
        assert_eq!(result, Err(LedgerError::EmptyEvidenceRef));
    }

    #[test]
    fn ledger_empty_has_no_events() {
        assert!(Ledger::empty().events().is_empty());
    }

    #[test]
    fn ledger_empty_has_no_last_revision() {
        assert_eq!(Ledger::empty().last_revision(), None);
    }

    #[test]
    fn ledger_first_append_requires_revision_one() {
        let ledger = Ledger::empty();
        let result = ledger.append(event("event-1", 2));
        assert_eq!(result, Err(LedgerError::InvalidRevisionSequence));
    }

    #[test]
    fn ledger_append_accepts_revision_one_on_empty_ledger() {
        let ledger = Ledger::empty();
        let next = ledger
            .append(event("event-1", 1))
            .expect("append should succeed");
        assert_eq!(next.events().len(), 1);
    }

    #[test]
    fn ledger_append_requires_next_revision() {
        let ledger = Ledger::empty()
            .append(event("event-1", 1))
            .expect("first append should succeed");
        let result = ledger.append(event("event-2", 3));
        assert_eq!(result, Err(LedgerError::InvalidRevisionSequence));
    }

    #[test]
    fn ledger_append_failure_does_not_mutate_ledger() {
        let ledger = Ledger::empty()
            .append(event("event-1", 1))
            .expect("first append should succeed");
        let failed = ledger.append(event("event-2", 3));

        assert_eq!(failed, Err(LedgerError::InvalidRevisionSequence));
        assert_eq!(ledger.events().len(), 1);
        assert_eq!(ledger.last_revision(), Some(1));
    }

    #[test]
    fn ledger_append_preserves_event_order() {
        let ledger = Ledger::empty()
            .append(event("event-1", 1))
            .expect("first append should succeed")
            .append(event("event-2", 2))
            .expect("second append should succeed");

        let ids: Vec<&str> = ledger
            .events()
            .iter()
            .map(|event| event.id.as_str())
            .collect();
        assert_eq!(ids, vec!["event-1", "event-2"]);
    }

    #[test]
    fn ledger_events_returns_immutable_event_slice() {
        let ledger = Ledger::empty()
            .append(event("event-1", 1))
            .expect("append should succeed");

        let events = ledger.events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, "event-1");
    }

    #[test]
    fn ledger_last_revision_returns_last_revision() {
        let ledger = Ledger::empty()
            .append(event("event-1", 1))
            .expect("first append should succeed")
            .append(event("event-2", 2))
            .expect("second append should succeed");

        assert_eq!(ledger.last_revision(), Some(2));
    }

    #[test]
    fn ledger_error_codes_are_stable() {
        assert_eq!(LedgerError::EmptyEventId.code(), "empty_event_id");
        assert_eq!(LedgerError::InvalidRevision.code(), "invalid_revision");
        assert_eq!(LedgerError::EmptyActorId.code(), "empty_actor_id");
        assert_eq!(LedgerError::EmptyEvidenceRef.code(), "empty_evidence_ref");
        assert_eq!(
            LedgerError::MissingEvidenceRefs.code(),
            "missing_evidence_refs"
        );
        assert_eq!(
            LedgerError::EmptyPayloadSummary.code(),
            "empty_payload_summary"
        );
        assert_eq!(
            LedgerError::InvalidRevisionSequence.code(),
            "invalid_revision_sequence"
        );
    }
}
