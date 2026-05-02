#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditProjectionType {
    Timeline,
    ValidationSummary,
    PolicySummary,
    MemorySummary,
    ContextSummary,
    ReplaySummary,
    OutputSummary,
    ExportSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditSourceType {
    LedgerEvent,
    ReplayReport,
    ReplayReconstruction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditSourceRef {
    pub source_type: AuditSourceType,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditProjection {
    pub projection_type: AuditProjectionType,
    pub source_refs: Vec<AuditSourceRef>,
    pub summary: String,
    pub details: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditError {
    MissingSourceRefs,
    EmptySourceRefId,
    EmptySummary,
    EmptyDetail,
}

impl AuditError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingSourceRefs => "missing_source_refs",
            Self::EmptySourceRefId => "empty_source_ref_id",
            Self::EmptySummary => "empty_summary",
            Self::EmptyDetail => "empty_detail",
        }
    }
}

impl AuditSourceRef {
    pub fn new(source_type: AuditSourceType, id: impl Into<String>) -> Result<Self, AuditError> {
        let id = id.into();
        if id.is_empty() {
            return Err(AuditError::EmptySourceRefId);
        }

        Ok(Self { source_type, id })
    }
}

impl AuditProjection {
    pub fn new(
        projection_type: AuditProjectionType,
        source_refs: Vec<AuditSourceRef>,
        summary: impl Into<String>,
        details: Vec<String>,
    ) -> Result<Self, AuditError> {
        if source_refs.is_empty() {
            return Err(AuditError::MissingSourceRefs);
        }
        if source_refs
            .iter()
            .any(|source_ref| source_ref.id.is_empty())
        {
            return Err(AuditError::EmptySourceRefId);
        }

        let summary = summary.into();
        if summary.is_empty() {
            return Err(AuditError::EmptySummary);
        }

        if details.iter().any(|detail| detail.is_empty()) {
            return Err(AuditError::EmptyDetail);
        }

        Ok(Self {
            projection_type,
            source_refs,
            summary,
            details,
        })
    }
}

pub fn project_ledger_timeline(events: &[crate::ledger::LedgerEvent]) -> Vec<AuditProjection> {
    events
        .iter()
        .map(|event| {
            AuditProjection::new(
                AuditProjectionType::Timeline,
                vec![
                    AuditSourceRef::new(AuditSourceType::LedgerEvent, event.id.clone())
                        .expect("ledger event id should be valid"),
                ],
                format!(
                    "ledger_event:{}:{}",
                    event.revision,
                    event.event_type.code()
                ),
                vec![
                    format!("revision:{}", event.revision),
                    format!("event_id:{}", event.id),
                    format!("evidence_refs:{}", event.evidence_refs.len()),
                ],
            )
            .expect("projection should be valid")
        })
        .collect()
}

pub fn project_replay_summary(report: &crate::replay::ReplayReport) -> AuditProjection {
    AuditProjection::new(
        AuditProjectionType::ReplaySummary,
        vec![
            AuditSourceRef::new(AuditSourceType::ReplayReport, "replay_report")
                .expect("source id should be valid"),
        ],
        format!(
            "replay:{}:{}:{}",
            report.status.code(),
            report.integrity.code(),
            report.readiness.code()
        ),
        vec![
            format!("events_replayed:{}", report.events_replayed),
            format!("reason:{}", report.reason),
        ],
    )
    .expect("projection should be valid")
}

pub fn project_reconstruction_summary(
    reconstruction: &crate::replay::ReplayReconstruction,
) -> AuditProjection {
    AuditProjection::new(
        AuditProjectionType::OutputSummary,
        vec![AuditSourceRef::new(
            AuditSourceType::ReplayReconstruction,
            "replay_reconstruction",
        )
        .expect("source id should be valid")],
        format!(
            "reconstruction:final_revision:{}",
            reconstruction.final_state.revision
        ),
        vec![
            format!(
                "final_lifecycle:{}",
                lifecycle_state_code(reconstruction.final_state.lifecycle)
            ),
            format!("events_seen:{}", reconstruction.events_seen),
            format!(
                "state_transitions_applied:{}",
                reconstruction.state_transitions_applied
            ),
        ],
    )
    .expect("projection should be valid")
}

fn lifecycle_state_code(state: crate::state::LifecycleState) -> &'static str {
    match state {
        crate::state::LifecycleState::Created => "created",
        crate::state::LifecycleState::Evaluating => "evaluating",
        crate::state::LifecycleState::Failed => "failed",
        crate::state::LifecycleState::Blocked => "blocked",
        crate::state::LifecycleState::Passed => "passed",
        crate::state::LifecycleState::PromotedTier1 => "promoted_tier_1",
        crate::state::LifecycleState::Rejected => "rejected",
        crate::state::LifecycleState::Unknown => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::{
        LedgerActor, LedgerActorType, LedgerEvent, LedgerEventType, LedgerPayload,
    };
    use crate::replay::{ReplayIntegrity, ReplayReadiness, ReplayReport, ReplayStatus};
    use crate::state::{HarnessState, LifecycleState};

    fn actor() -> LedgerActor {
        LedgerActor::new(LedgerActorType::System, "actor-1").expect("actor should be valid")
    }

    fn payload() -> LedgerPayload {
        LedgerPayload::new("summary").expect("payload should be valid")
    }

    fn event(id: &str, revision: u64, event_type: LedgerEventType) -> LedgerEvent {
        LedgerEvent::new(
            id,
            revision,
            event_type,
            actor(),
            vec!["evidence-1".to_string(), "evidence-2".to_string()],
            payload(),
        )
        .expect("event should be valid")
    }

    #[test]
    fn audit_source_ref_requires_id() {
        assert_eq!(
            AuditSourceRef::new(AuditSourceType::LedgerEvent, ""),
            Err(AuditError::EmptySourceRefId)
        );
    }

    #[test]
    fn audit_projection_requires_source_refs() {
        assert_eq!(
            AuditProjection::new(AuditProjectionType::Timeline, vec![], "ok", vec![]),
            Err(AuditError::MissingSourceRefs)
        );
    }

    #[test]
    fn audit_projection_requires_summary() {
        let source_ref =
            AuditSourceRef::new(AuditSourceType::LedgerEvent, "event-1").expect("valid");

        assert_eq!(
            AuditProjection::new(AuditProjectionType::Timeline, vec![source_ref], "", vec![]),
            Err(AuditError::EmptySummary)
        );
    }

    #[test]
    fn audit_projection_rejects_empty_detail() {
        let source_ref =
            AuditSourceRef::new(AuditSourceType::LedgerEvent, "event-1").expect("valid");

        assert_eq!(
            AuditProjection::new(
                AuditProjectionType::Timeline,
                vec![source_ref],
                "ok",
                vec!["".to_string()],
            ),
            Err(AuditError::EmptyDetail)
        );
    }

    #[test]
    fn audit_error_codes_are_stable() {
        assert_eq!(AuditError::MissingSourceRefs.code(), "missing_source_refs");
        assert_eq!(AuditError::EmptySourceRefId.code(), "empty_source_ref_id");
        assert_eq!(AuditError::EmptySummary.code(), "empty_summary");
        assert_eq!(AuditError::EmptyDetail.code(), "empty_detail");
    }

    #[test]
    fn project_ledger_timeline_empty_input_returns_empty_vec() {
        assert!(project_ledger_timeline(&[]).is_empty());
    }

    #[test]
    fn project_ledger_timeline_creates_one_projection_per_event() {
        let events = vec![
            event("event-1", 1, LedgerEventType::ExecutionStart),
            event("event-2", 2, LedgerEventType::ExecutionEnd),
        ];

        let projections = project_ledger_timeline(&events);
        assert_eq!(projections.len(), 2);
    }

    #[test]
    fn project_ledger_timeline_preserves_event_order() {
        let events = vec![
            event("event-a", 1, LedgerEventType::ExecutionStart),
            event("event-b", 2, LedgerEventType::ExecutionEnd),
        ];

        let projections = project_ledger_timeline(&events);
        assert_eq!(projections[0].source_refs[0].id, "event-a");
        assert_eq!(projections[1].source_refs[0].id, "event-b");
    }

    #[test]
    fn project_ledger_timeline_uses_stable_summary() {
        let events = vec![event("event-1", 7, LedgerEventType::PolicyPass)];

        let projections = project_ledger_timeline(&events);
        assert_eq!(projections[0].summary, "ledger_event:7:policy_pass");
    }

    #[test]
    fn project_ledger_timeline_includes_required_details() {
        let events = vec![event("event-1", 3, LedgerEventType::MemoryWrite)];

        let projections = project_ledger_timeline(&events);
        assert_eq!(
            projections[0].details,
            vec![
                "revision:3".to_string(),
                "event_id:event-1".to_string(),
                "evidence_refs:2".to_string(),
            ]
        );
    }

    #[test]
    fn project_replay_summary_uses_stable_summary() {
        let report = ReplayReport {
            status: ReplayStatus::Replayable,
            integrity: ReplayIntegrity::Valid,
            readiness: ReplayReadiness::Ready,
            events_replayed: 4,
            reason: "ledger_sequence_valid",
        };

        let projection = project_replay_summary(&report);
        assert_eq!(projection.summary, "replay:replayable:valid:ready");
    }

    #[test]
    fn project_replay_summary_includes_required_details() {
        let report = ReplayReport {
            status: ReplayStatus::NotReplayable,
            integrity: ReplayIntegrity::Incomplete,
            readiness: ReplayReadiness::NotReady,
            events_replayed: 0,
            reason: "revision_gap",
        };

        let projection = project_replay_summary(&report);
        assert_eq!(
            projection.details,
            vec![
                "events_replayed:0".to_string(),
                "reason:revision_gap".to_string(),
            ]
        );
    }

    #[test]
    fn project_reconstruction_summary_uses_final_revision() {
        let reconstruction = crate::replay::ReplayReconstruction {
            final_state: HarnessState {
                revision: 9,
                lifecycle: LifecycleState::Passed,
            },
            events_seen: 5,
            state_transitions_applied: 2,
        };

        let projection = project_reconstruction_summary(&reconstruction);
        assert_eq!(projection.summary, "reconstruction:final_revision:9");
    }

    #[test]
    fn project_reconstruction_summary_includes_lifecycle_and_counts() {
        let reconstruction = crate::replay::ReplayReconstruction {
            final_state: HarnessState {
                revision: 2,
                lifecycle: LifecycleState::PromotedTier1,
            },
            events_seen: 6,
            state_transitions_applied: 3,
        };

        let projection = project_reconstruction_summary(&reconstruction);
        assert_eq!(
            projection.details,
            vec![
                "final_lifecycle:promoted_tier_1".to_string(),
                "events_seen:6".to_string(),
                "state_transitions_applied:3".to_string(),
            ]
        );
    }
}
