use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayStatus {
    Replayable,
    NotReplayable,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayIntegrity {
    Valid,
    Invalid,
    Incomplete,
    Conflicting,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayReadiness {
    Ready,
    NotReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayError {
    EmptyLedger,
    InvalidFirstRevision,
    RevisionGap,
    DuplicateRevision,
    ConflictingEventId,
}

impl ReplayError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyLedger => "empty_ledger",
            Self::InvalidFirstRevision => "invalid_first_revision",
            Self::RevisionGap => "revision_gap",
            Self::DuplicateRevision => "duplicate_revision",
            Self::ConflictingEventId => "conflicting_event_id",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReport {
    pub status: ReplayStatus,
    pub integrity: ReplayIntegrity,
    pub readiness: ReplayReadiness,
    pub events_replayed: u64,
    pub reason: &'static str,
}

impl ReplayReport {
    pub fn unknown() -> Self {
        Self {
            status: ReplayStatus::Unknown,
            integrity: ReplayIntegrity::Unknown,
            readiness: ReplayReadiness::NotReady,
            events_replayed: 0,
            reason: "unknown_is_not_pass",
        }
    }

    pub fn not_replayable(integrity: ReplayIntegrity, reason: &'static str) -> Self {
        Self {
            status: ReplayStatus::NotReplayable,
            integrity,
            readiness: ReplayReadiness::NotReady,
            events_replayed: 0,
            reason,
        }
    }

    pub fn replayable(events_replayed: u64) -> Self {
        Self {
            status: ReplayStatus::Replayable,
            integrity: ReplayIntegrity::Valid,
            readiness: ReplayReadiness::Ready,
            events_replayed,
            reason: "ledger_sequence_valid",
        }
    }
}

pub fn classify_replay_readiness(
    events: &[crate::ledger::LedgerEvent],
) -> Result<ReplayReport, ReplayError> {
    if events.is_empty() {
        return Err(ReplayError::EmptyLedger);
    }

    if events[0].revision != 1 {
        return Err(ReplayError::InvalidFirstRevision);
    }

    let mut seen_event_ids: HashSet<&str> = HashSet::new();

    for (expected_revision, event) in (1_u64..).zip(events.iter()) {
        if !seen_event_ids.insert(event.id.as_str()) {
            return Err(ReplayError::ConflictingEventId);
        }

        if event.revision < expected_revision {
            return Err(ReplayError::DuplicateRevision);
        }

        if event.revision > expected_revision {
            return Err(ReplayError::RevisionGap);
        }
    }

    Ok(ReplayReport::replayable(events.len() as u64))
}

pub fn report_from_error(error: ReplayError) -> ReplayReport {
    match error {
        ReplayError::EmptyLedger => {
            ReplayReport::not_replayable(ReplayIntegrity::Incomplete, "empty_ledger")
        }
        ReplayError::InvalidFirstRevision => {
            ReplayReport::not_replayable(ReplayIntegrity::Invalid, "invalid_first_revision")
        }
        ReplayError::RevisionGap => {
            ReplayReport::not_replayable(ReplayIntegrity::Incomplete, "revision_gap")
        }
        ReplayError::DuplicateRevision => {
            ReplayReport::not_replayable(ReplayIntegrity::Conflicting, "duplicate_revision")
        }
        ReplayError::ConflictingEventId => {
            ReplayReport::not_replayable(ReplayIntegrity::Conflicting, "conflicting_event_id")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::{
        LedgerActor, LedgerActorType, LedgerEvent, LedgerEventType, LedgerPayload,
    };

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
    fn replay_unknown_is_not_ready() {
        let report = ReplayReport::unknown();
        assert_eq!(report.status, ReplayStatus::Unknown);
        assert_eq!(report.integrity, ReplayIntegrity::Unknown);
        assert_eq!(report.readiness, ReplayReadiness::NotReady);
        assert_eq!(report.events_replayed, 0);
        assert_eq!(report.reason, "unknown_is_not_pass");
    }

    #[test]
    fn replay_error_codes_are_stable() {
        assert_eq!(ReplayError::EmptyLedger.code(), "empty_ledger");
        assert_eq!(
            ReplayError::InvalidFirstRevision.code(),
            "invalid_first_revision"
        );
        assert_eq!(ReplayError::RevisionGap.code(), "revision_gap");
        assert_eq!(ReplayError::DuplicateRevision.code(), "duplicate_revision");
        assert_eq!(
            ReplayError::ConflictingEventId.code(),
            "conflicting_event_id"
        );
    }

    #[test]
    fn replay_empty_ledger_errors() {
        let result = classify_replay_readiness(&[]);
        assert_eq!(result, Err(ReplayError::EmptyLedger));
    }

    #[test]
    fn replay_first_revision_must_be_one() {
        let events = vec![event("event-1", 2)];
        let result = classify_replay_readiness(&events);
        assert_eq!(result, Err(ReplayError::InvalidFirstRevision));
    }

    #[test]
    fn replay_contiguous_events_are_replayable() {
        let events = vec![event("event-1", 1), event("event-2", 2)];
        let report = classify_replay_readiness(&events).expect("report should be replayable");

        assert_eq!(report.status, ReplayStatus::Replayable);
        assert_eq!(report.integrity, ReplayIntegrity::Valid);
        assert_eq!(report.readiness, ReplayReadiness::Ready);
        assert_eq!(report.events_replayed, 2);
        assert_eq!(report.reason, "ledger_sequence_valid");
    }

    #[test]
    fn replay_counts_events_without_applying_them() {
        let events = vec![
            event("event-1", 1),
            event("event-2", 2),
            event("event-3", 3),
        ];
        let report = classify_replay_readiness(&events).expect("report should be replayable");
        assert_eq!(report.events_replayed, events.len() as u64);
    }

    #[test]
    fn replay_revision_gap_errors() {
        let events = vec![event("event-1", 1), event("event-2", 3)];
        let result = classify_replay_readiness(&events);
        assert_eq!(result, Err(ReplayError::RevisionGap));
    }

    #[test]
    fn replay_duplicate_revision_errors() {
        let events = vec![event("event-1", 1), event("event-2", 1)];
        let result = classify_replay_readiness(&events);
        assert_eq!(result, Err(ReplayError::DuplicateRevision));
    }

    #[test]
    fn replay_conflicting_event_id_errors_same_revision() {
        let events = vec![event("event-1", 1), event("event-1", 2)];
        let result = classify_replay_readiness(&events);
        assert_eq!(result, Err(ReplayError::ConflictingEventId));
    }

    #[test]
    fn replay_conflicting_event_id_errors_different_revision() {
        let events = vec![
            event("event-1", 1),
            event("event-2", 2),
            event("event-1", 3),
        ];
        let result = classify_replay_readiness(&events);
        assert_eq!(result, Err(ReplayError::ConflictingEventId));
    }

    #[test]
    fn replay_preserves_input_order_by_not_sorting() {
        let events = vec![event("event-1", 2), event("event-2", 1)];
        let result = classify_replay_readiness(&events);
        assert_eq!(result, Err(ReplayError::InvalidFirstRevision));
    }

    #[test]
    fn replay_report_from_empty_ledger_error() {
        let report = report_from_error(ReplayError::EmptyLedger);
        assert_eq!(
            report,
            ReplayReport::not_replayable(ReplayIntegrity::Incomplete, "empty_ledger")
        );
    }

    #[test]
    fn replay_report_from_invalid_first_revision_error() {
        let report = report_from_error(ReplayError::InvalidFirstRevision);
        assert_eq!(
            report,
            ReplayReport::not_replayable(ReplayIntegrity::Invalid, "invalid_first_revision")
        );
    }

    #[test]
    fn replay_report_from_revision_gap_error() {
        let report = report_from_error(ReplayError::RevisionGap);
        assert_eq!(
            report,
            ReplayReport::not_replayable(ReplayIntegrity::Incomplete, "revision_gap")
        );
    }

    #[test]
    fn replay_report_from_duplicate_revision_error() {
        let report = report_from_error(ReplayError::DuplicateRevision);
        assert_eq!(
            report,
            ReplayReport::not_replayable(ReplayIntegrity::Conflicting, "duplicate_revision")
        );
    }

    #[test]
    fn replay_report_from_conflicting_event_id_error() {
        let report = report_from_error(ReplayError::ConflictingEventId);
        assert_eq!(
            report,
            ReplayReport::not_replayable(ReplayIntegrity::Conflicting, "conflicting_event_id")
        );
    }
}
