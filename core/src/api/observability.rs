use super::{
    DurableAppendReport, EndToEndLocalHarnessReport, OperatorActionExecutionReport,
    ProviderEvidenceReplayReport, RecoveryAcceptanceReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservabilitySnapshotMode {
    SuppliedEvidenceSnapshot,
    CurrentInMemorySnapshotUnsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservabilitySnapshotStatus {
    Built,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservabilitySnapshotReason {
    BuiltFromSuppliedEvidence,
    EmptySnapshotId,
    NoEvidenceSupplied,
    CurrentInMemorySnapshotUnsupported,
    RawPayloadNotAllowed,
    MutableStateObservationNotAllowed,
    PersistenceReadNotAllowed,
    PersistenceWriteNotAllowed,
    ExportNotAllowed,
    RecomputeNotAllowed,
    AuthorityMutationNotAllowed,
}

impl ObservabilitySnapshotReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::BuiltFromSuppliedEvidence => "built_from_supplied_evidence",
            Self::EmptySnapshotId => "empty_snapshot_id",
            Self::NoEvidenceSupplied => "no_evidence_supplied",
            Self::CurrentInMemorySnapshotUnsupported => "current_in_memory_snapshot_unsupported",
            Self::RawPayloadNotAllowed => "raw_payload_not_allowed",
            Self::MutableStateObservationNotAllowed => "mutable_state_observation_not_allowed",
            Self::PersistenceReadNotAllowed => "persistence_read_not_allowed",
            Self::PersistenceWriteNotAllowed => "persistence_write_not_allowed",
            Self::ExportNotAllowed => "export_not_allowed",
            Self::RecomputeNotAllowed => "recompute_not_allowed",
            Self::AuthorityMutationNotAllowed => "authority_mutation_not_allowed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedHarnessSummary {
    pub run_id: String,
    pub status_code: String,
    pub reason_code: String,
    pub provider_output_trusted: bool,
    pub provider_output_authoritative: bool,
    pub retry_scheduled: bool,
    pub recovery_candidate_only: bool,
    pub recovered_state_promoted: bool,
    pub ui_transport_live: bool,
    pub ui_submission_executes_action: bool,
    pub action_kind: String,
    pub action_real_world_effect: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedDurableAppendSummary {
    pub append_transaction_id: String,
    pub status_code: String,
    pub reason_code: String,
    pub committed: bool,
    pub promoted: bool,
    pub recovered_state: bool,
    pub repaired_replay: bool,
    pub trusted_provider_output: bool,
    pub executed_action: bool,
    pub mutated_application_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedRecoverySummary {
    pub acceptance_id: String,
    pub status_code: String,
    pub reason_code: String,
    pub accepted_for_in_memory_use: bool,
    pub replaced_global_state: bool,
    pub persisted: bool,
    pub appended_ledger: bool,
    pub appended_audit: bool,
    pub repaired_replay: bool,
    pub promoted_provider_output: bool,
    pub executed_action: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedReplaySummary {
    pub replay_id: String,
    pub source_run_id: String,
    pub evidence_id: String,
    pub status_code: String,
    pub reason_code: String,
    pub replayed_from_evidence: bool,
    pub live_execution_performed: bool,
    pub new_authorization_created: bool,
    pub new_audit_record_created: bool,
    pub new_action_executed: bool,
    pub new_ledger_fact_created: bool,
    pub persisted: bool,
    pub repaired_replay: bool,
    pub mutated_application_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedActionSummary {
    pub action_kind: String,
    pub action_reason_code: String,
    pub action_real_world_effect: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedDiagnosticSummary {
    pub family: String,
    pub code: String,
    pub key: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservabilitySnapshot {
    pub snapshot_id: String,
    pub mode: ObservabilitySnapshotMode,
    pub status: ObservabilitySnapshotStatus,
    pub reason: ObservabilitySnapshotReason,
    pub harness: Option<ObservedHarnessSummary>,
    pub durable_append: Option<ObservedDurableAppendSummary>,
    pub recovery: Option<ObservedRecoverySummary>,
    pub replay: Option<ObservedReplaySummary>,
    pub action: Option<ObservedActionSummary>,
    pub diagnostics: Vec<ObservedDiagnosticSummary>,
    pub contains_raw_provider_payload: bool,
    pub contains_secret_material: bool,
    pub mutates_application_state: bool,
    pub reads_persistence: bool,
    pub writes_persistence: bool,
    pub recomputes_authority: bool,
    pub repairs_state: bool,
    pub exports_data: bool,
    pub summary: String,
}

#[allow(clippy::too_many_arguments)]
pub fn observability_snapshot_from_supplied_evidence(
    snapshot_id: impl Into<String>,
    harness: Option<&EndToEndLocalHarnessReport>,
    durable_append: Option<&DurableAppendReport>,
    recovery: Option<&RecoveryAcceptanceReport>,
    replay: Option<&ProviderEvidenceReplayReport>,
    action: Option<&OperatorActionExecutionReport>,
    diagnostics: Vec<ObservedDiagnosticSummary>,
) -> ObservabilitySnapshot {
    let snapshot_id = snapshot_id.into();
    let has_evidence = harness.is_some()
        || durable_append.is_some()
        || recovery.is_some()
        || replay.is_some()
        || action.is_some()
        || !diagnostics.is_empty();
    let (status, reason) = if snapshot_id.is_empty() {
        (
            ObservabilitySnapshotStatus::Rejected,
            ObservabilitySnapshotReason::EmptySnapshotId,
        )
    } else if !has_evidence {
        (
            ObservabilitySnapshotStatus::Rejected,
            ObservabilitySnapshotReason::NoEvidenceSupplied,
        )
    } else {
        (
            ObservabilitySnapshotStatus::Built,
            ObservabilitySnapshotReason::BuiltFromSuppliedEvidence,
        )
    };

    ObservabilitySnapshot {
        snapshot_id,
        mode: ObservabilitySnapshotMode::SuppliedEvidenceSnapshot,
        status,
        reason,
        harness: harness.map(|report| ObservedHarnessSummary {
            run_id: report.run_id.clone(),
            status_code: format!("{:?}", report.status).to_lowercase(),
            reason_code: report.reason.code().to_string(),
            provider_output_trusted: report.provider_output_trusted,
            provider_output_authoritative: report.provider_output_authoritative,
            retry_scheduled: report.retry_scheduled,
            recovery_candidate_only: report.recovery_candidate_only,
            recovered_state_promoted: report.recovered_state_promoted,
            ui_transport_live: report.ui_transport_live,
            ui_submission_executes_action: report.ui_submission_executes_action,
            action_kind: report.action_kind.clone(),
            action_real_world_effect: report.action_real_world_effect,
        }),
        durable_append: durable_append.map(|report| ObservedDurableAppendSummary {
            append_transaction_id: report.append_transaction_id.clone(),
            status_code: format!("{:?}", report.status).to_lowercase(),
            reason_code: report.reason.code().to_string(),
            committed: report.committed,
            promoted: report.promoted,
            recovered_state: report.recovered_state,
            repaired_replay: report.repaired_replay,
            trusted_provider_output: report.trusted_provider_output,
            executed_action: report.executed_action,
            mutated_application_state: report.mutated_application_state,
        }),
        recovery: recovery.map(|report| ObservedRecoverySummary {
            acceptance_id: report.acceptance_id.clone(),
            status_code: format!("{:?}", report.status).to_lowercase(),
            reason_code: report.reason.code().to_string(),
            accepted_for_in_memory_use: report.accepted_for_in_memory_use,
            replaced_global_state: report.replaced_global_state,
            persisted: report.persisted,
            appended_ledger: report.appended_ledger,
            appended_audit: report.appended_audit,
            repaired_replay: report.repaired_replay,
            promoted_provider_output: report.promoted_provider_output,
            executed_action: report.executed_action,
        }),
        replay: replay.map(|report| ObservedReplaySummary {
            replay_id: report.replay_id.clone(),
            source_run_id: report.source_run_id.clone(),
            evidence_id: report.evidence_id.clone(),
            status_code: format!("{:?}", report.status).to_lowercase(),
            reason_code: report.reason.code().to_string(),
            replayed_from_evidence: report.replayed_from_evidence,
            live_execution_performed: report.live_execution_performed,
            new_authorization_created: report.new_authorization_created,
            new_audit_record_created: report.new_audit_record_created,
            new_action_executed: report.new_action_executed,
            new_ledger_fact_created: report.new_ledger_fact_created,
            persisted: report.persisted,
            repaired_replay: report.repaired_replay,
            mutated_application_state: report.mutated_application_state,
        }),
        action: action.map(|report| ObservedActionSummary {
            action_kind: format!("{:?}", report.action_kind),
            action_reason_code: report.reason.code().to_string(),
            action_real_world_effect: report.executed_real_world_effect,
        }),
        diagnostics,
        contains_raw_provider_payload: false,
        contains_secret_material: false,
        mutates_application_state: false,
        reads_persistence: false,
        writes_persistence: false,
        recomputes_authority: false,
        repairs_state: false,
        exports_data: false,
        summary: "read-only observability snapshot built from caller-supplied evidence only; snapshot is non-authoritative and does not mutate authority".to_string(),
    }
}

pub fn observability_snapshot_from_current_in_memory_state(
    snapshot_id: impl Into<String>,
) -> ObservabilitySnapshot {
    ObservabilitySnapshot {
        snapshot_id: snapshot_id.into(),
        mode: ObservabilitySnapshotMode::CurrentInMemorySnapshotUnsupported,
        status: ObservabilitySnapshotStatus::Rejected,
        reason: ObservabilitySnapshotReason::CurrentInMemorySnapshotUnsupported,
        harness: None,
        durable_append: None,
        recovery: None,
        replay: None,
        action: None,
        diagnostics: vec![],
        contains_raw_provider_payload: false,
        contains_secret_material: false,
        mutates_application_state: false,
        reads_persistence: false,
        writes_persistence: false,
        recomputes_authority: false,
        repairs_state: false,
        exports_data: false,
        summary: "current in-memory snapshot mode is unsupported; observability remains read-only and non-authoritative".to_string(),
    }
}

pub fn observability_snapshot_mutates_authority(snapshot: &ObservabilitySnapshot) -> bool {
    snapshot.mutates_application_state
        || snapshot.writes_persistence
        || snapshot.recomputes_authority
        || snapshot.repairs_state
        || snapshot.exports_data
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::*;

    fn harness_report() -> EndToEndLocalHarnessReport {
        run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
            run_id: "run-1".into(),
            provider_prompt: "p".into(),
            operator_id: "o".into(),
            target_id: "t".into(),
            reason: "r".into(),
        })
    }

    #[test]
    fn observability_reason_codes_are_stable() {
        assert_eq!(
            ObservabilitySnapshotReason::BuiltFromSuppliedEvidence.code(),
            "built_from_supplied_evidence"
        );
    }
    #[test]
    fn observability_snapshot_rejects_empty_snapshot_id() {
        assert_eq!(
            observability_snapshot_from_supplied_evidence("", None, None, None, None, None, vec![])
                .reason,
            ObservabilitySnapshotReason::EmptySnapshotId
        );
    }
    #[test]
    fn observability_snapshot_rejects_empty_supplied_evidence() {
        assert_eq!(
            observability_snapshot_from_supplied_evidence(
                "s",
                None,
                None,
                None,
                None,
                None,
                vec![]
            )
            .reason,
            ObservabilitySnapshotReason::NoEvidenceSupplied
        );
    }
    #[test]
    fn observability_snapshot_builds_from_harness_report() {
        assert!(observability_snapshot_from_supplied_evidence(
            "s",
            Some(&harness_report()),
            None,
            None,
            None,
            None,
            vec![]
        )
        .harness
        .is_some());
    }
    #[test]
    fn observability_snapshot_builds_from_durable_append_report() {
        let r = append_report();
        assert!(observability_snapshot_from_supplied_evidence(
            "s",
            None,
            Some(&r),
            None,
            None,
            None,
            vec![]
        )
        .durable_append
        .is_some());
    }
    #[test]
    fn observability_snapshot_builds_from_recovery_acceptance_report() {
        let r = recovery_report();
        assert!(observability_snapshot_from_supplied_evidence(
            "s",
            None,
            None,
            Some(&r),
            None,
            None,
            vec![]
        )
        .recovery
        .is_some());
    }
    #[test]
    fn observability_snapshot_builds_from_replay_report() {
        let r = replay_report();
        assert!(observability_snapshot_from_supplied_evidence(
            "s",
            None,
            None,
            None,
            Some(&r),
            None,
            vec![]
        )
        .replay
        .is_some());
    }
    #[test]
    fn observability_snapshot_builds_from_action_report() {
        let r = action_report();
        assert!(observability_snapshot_from_supplied_evidence(
            "s",
            None,
            None,
            None,
            None,
            Some(&r),
            vec![]
        )
        .action
        .is_some());
    }
    #[test]
    fn observability_snapshot_builds_from_multiple_reports() {
        let r = observability_snapshot_from_supplied_evidence(
            "s",
            Some(&harness_report()),
            Some(&append_report()),
            Some(&recovery_report()),
            Some(&replay_report()),
            None,
            vec![ObservedDiagnosticSummary {
                family: "f".into(),
                code: "c".into(),
                key: "k".into(),
                summary: "s".into(),
            }],
        );
        assert_eq!(r.status, ObservabilitySnapshotStatus::Built);
    }
    #[test]
    fn observability_snapshot_copies_owned_fields_without_references() {
        let run_id = harness_report().run_id.clone();
        let snapshot = observability_snapshot_from_supplied_evidence(
            "s",
            Some(&harness_report()),
            None,
            None,
            None,
            None,
            vec![],
        );
        assert_eq!(snapshot.harness.unwrap().run_id, run_id);
    }
    #[test]
    fn observability_snapshot_does_not_mutate_source_reports() {
        let source = harness_report();
        let copy = source.clone();
        let _ = observability_snapshot_from_supplied_evidence(
            "s",
            Some(&source),
            None,
            None,
            None,
            None,
            vec![],
        );
        assert_eq!(source, copy);
    }
    #[test]
    fn observability_snapshot_does_not_require_local_application_state() {
        let snapshot = observability_snapshot_from_supplied_evidence(
            "s",
            None,
            None,
            None,
            None,
            None,
            vec![ObservedDiagnosticSummary {
                family: "f".into(),
                code: "c".into(),
                key: "k".into(),
                summary: "s".into(),
            }],
        );
        assert_eq!(snapshot.status, ObservabilitySnapshotStatus::Built);
    }
    #[test]
    fn current_in_memory_snapshot_mode_is_unsupported() {
        assert_eq!(
            observability_snapshot_from_current_in_memory_state("s").reason,
            ObservabilitySnapshotReason::CurrentInMemorySnapshotUnsupported
        );
    }
    #[test]
    fn observability_snapshot_does_not_read_persistence() {
        assert!(!observability_snapshot_from_current_in_memory_state("s").reads_persistence);
    }
    #[test]
    fn observability_snapshot_does_not_write_persistence() {
        assert!(!observability_snapshot_from_current_in_memory_state("s").writes_persistence);
    }
    #[test]
    fn observability_snapshot_does_not_export_data() {
        assert!(!observability_snapshot_from_current_in_memory_state("s").exports_data);
    }
    #[test]
    fn observability_snapshot_does_not_recompute_authority() {
        assert!(!observability_snapshot_from_current_in_memory_state("s").recomputes_authority);
    }
    #[test]
    fn observability_snapshot_does_not_repair_state() {
        assert!(!observability_snapshot_from_current_in_memory_state("s").repairs_state);
    }
    #[test]
    fn observability_snapshot_does_not_include_raw_provider_payload() {
        assert!(
            !observability_snapshot_from_current_in_memory_state("s").contains_raw_provider_payload
        );
    }
    #[test]
    fn observability_snapshot_does_not_include_secret_material() {
        assert!(!observability_snapshot_from_current_in_memory_state("s").contains_secret_material);
    }
    #[test]
    fn durable_append_committed_is_observed_as_append_evidence_not_recovery_or_promotion() {
        let r = append_report();
        let s = observability_snapshot_from_supplied_evidence(
            "s",
            None,
            Some(&r),
            None,
            None,
            None,
            vec![],
        );
        let d = s.durable_append.unwrap();
        assert!(d.committed);
        assert!(!d.promoted);
        assert!(!d.recovered_state);
    }
    #[test]
    fn risky_summary_text_cannot_create_authority() {
        let s = observability_snapshot_from_supplied_evidence(
            "s",
            None,
            None,
            None,
            None,
            None,
            vec![ObservedDiagnosticSummary {
                family: "secret".into(),
                code: "trusted provider payload".into(),
                key: "fix state".into(),
                summary: "recover global state export now append ledger execute action".into(),
            }],
        );
        assert!(!observability_snapshot_mutates_authority(&s));
    }
    #[test]
    fn observability_snapshot_mutates_authority_returns_false() {
        assert!(!observability_snapshot_mutates_authority(
            &observability_snapshot_from_current_in_memory_state("s")
        ));
    }
    #[test]
    fn dry_run_does_not_build_observability_snapshot() {
        let s =
            observability_snapshot_from_supplied_evidence("", None, None, None, None, None, vec![]);
        assert_eq!(s.status, ObservabilitySnapshotStatus::Rejected);
    }

    fn append_report() -> DurableAppendReport {
        DurableAppendReport {
            status: DurableAppendStatus::Verified,
            reason: DurableAppendReason::PreparedForAppend,
            append_transaction_id: "tx".into(),
            audit_record_id: "a".into(),
            ledger_record_id: "l".into(),
            prior_revision: Some(1),
            next_revision: Some(2),
            audit_payload_len: Some(5),
            ledger_payload_len: Some(6),
            checksum: Some("c".into()),
            committed: true,
            promoted: false,
            recovered_state: false,
            repaired_replay: false,
            trusted_provider_output: false,
            executed_action: false,
            mutated_application_state: false,
            summary: "s".into(),
        }
    }
    fn recovery_report() -> RecoveryAcceptanceReport {
        accept_recovery_candidate_for_in_memory_use(RecoveryAcceptanceRequest {
            acceptance_id: "a".into(),
            expected_recovery_id: "r".into(),
            expected_ledger_record_id: "l".into(),
            expected_revision: Some(1),
            candidate: ApplicationRecoveryCandidate {
                recovery_id: "r".into(),
                ledger_record_id: "l".into(),
                revision: 1,
                payload_len: 3,
                checksum: "abc".into(),
                candidate_bytes: b"abc".to_vec(),
            },
        })
    }
    fn action_report() -> OperatorActionExecutionReport {
        OperatorActionExecutionReport {
            status: OperatorActionExecutionStatus::Executed,
            reason: OperatorActionExecutionReason::ExecutionDecisionRecorded,
            execution_id: "exec".into(),
            action_kind: OperatorActionKind::RecordExecutionDecision,
            submission_id: "sub".into(),
            authorization_id: "auth".into(),
            audit_record_id: "audit".into(),
            operator_id: "op".into(),
            target_id: "t".into(),
            executed_real_world_effect: false,
            persisted: false,
            appended_ledger: false,
            appended_audit: false,
            called_provider: false,
            repaired_replay: false,
            mutated_application_state: false,
            summary: "s".into(),
        }
    }
    fn replay_report() -> ProviderEvidenceReplayReport {
        let h = harness_report();
        verify_provider_evidence_replay(
            "replay",
            "run-1",
            provider_evidence_snapshot_from_harness_report("e", &h),
        )
    }
}
