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
