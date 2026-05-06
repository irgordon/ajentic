use ajentic_core::api::{
    accept_recovery_candidate_for_in_memory_use, encode_audit_export_snapshot,
    observability_snapshot_from_supplied_evidence, observability_snapshot_mutates_authority,
    provider_evidence_snapshot_from_harness_report, recovery_acceptance_mutates_authority,
    run_end_to_end_local_harness, verify_provider_evidence_replay, ApplicationRecoveryCandidate,
    AuditExportEncodingLimits, EndToEndLocalHarnessRequest, EndToEndLocalHarnessStatus,
    ObservedDiagnosticSummary, ProviderEvidenceReplayMode, ProviderEvidenceReplayStatus,
    RecoveryAcceptanceRequest, RecoveryAcceptanceStatus,
};

#[test]
fn root_integration_harness_report_is_bounded_and_non_authoritative() {
    let report = run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
        run_id: "phase-82-5-run".to_string(),
        provider_prompt: "deterministic local harness prompt".to_string(),
        operator_id: "operator-1".to_string(),
        target_id: "target-1".to_string(),
        reason: "phase-82-5-baseline".to_string(),
    });

    assert_eq!(report.status, EndToEndLocalHarnessStatus::Completed);
    assert!(!report.provider_output_trusted);
    assert!(!report.provider_output_authoritative);
    assert!(!report.retry_scheduled);
    assert!(report.recovery_candidate_only);
    assert!(!report.recovered_state_promoted);
    assert!(!report.ui_transport_live);
    assert!(!report.ui_submission_executes_action);
    assert_eq!(report.action_kind, "RecordExecutionDecision");
    assert!(!report.action_real_world_effect);
}

#[test]
fn root_integration_provider_replay_is_distinguishable_from_live_run() {
    let harness = run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
        run_id: "phase-82-5-run-replay".to_string(),
        provider_prompt: "deterministic local harness prompt".to_string(),
        operator_id: "operator-1".to_string(),
        target_id: "target-1".to_string(),
        reason: "phase-82-5-replay".to_string(),
    });

    let snapshot = provider_evidence_snapshot_from_harness_report("phase-82-5-evidence", &harness);
    let replay =
        verify_provider_evidence_replay("phase-82-5-replay", "phase-82-5-run-replay", snapshot);

    assert_eq!(replay.status, ProviderEvidenceReplayStatus::Verified);
    assert_eq!(replay.mode, ProviderEvidenceReplayMode::Replay);
    assert!(!replay.live_execution_performed);
    assert!(!replay.new_authorization_created);
    assert!(!replay.new_audit_record_created);
    assert!(!replay.new_action_executed);
    assert!(!replay.new_ledger_fact_created);
    assert!(!replay.persisted);
    assert!(!replay.repaired_replay);
    assert!(!replay.mutated_application_state);
}

#[test]
fn root_integration_recovery_candidate_acceptance_is_in_memory_only() {
    let candidate = ApplicationRecoveryCandidate {
        recovery_id: "recovery-84".into(),
        ledger_record_id: "ledger-84".into(),
        revision: 2,
        payload_len: 3,
        checksum: "abc".into(),
        candidate_bytes: b"abc".to_vec(),
    };
    let report = accept_recovery_candidate_for_in_memory_use(RecoveryAcceptanceRequest {
        acceptance_id: "acceptance-84".into(),
        expected_recovery_id: "recovery-84".into(),
        expected_ledger_record_id: "ledger-84".into(),
        expected_revision: Some(2),
        candidate,
    });
    assert_eq!(report.status, RecoveryAcceptanceStatus::Accepted);
    assert!(report.accepted_for_in_memory_use);
    assert!(!report.replaced_global_state);
    assert!(!report.persisted);
    assert!(!report.appended_ledger);
    assert!(!report.appended_audit);
}

#[test]
fn root_integration_recovery_acceptance_does_not_mutate_authority() {
    let report = accept_recovery_candidate_for_in_memory_use(RecoveryAcceptanceRequest {
        acceptance_id: "acceptance-84-b".into(),
        expected_recovery_id: "recovery-84-b".into(),
        expected_ledger_record_id: "ledger-84-b".into(),
        expected_revision: None,
        candidate: ApplicationRecoveryCandidate {
            recovery_id: "recovery-84-b".into(),
            ledger_record_id: "ledger-84-b".into(),
            revision: 7,
            payload_len: 4,
            checksum: "seed".into(),
            candidate_bytes: b"seed".to_vec(),
        },
    });
    assert!(!recovery_acceptance_mutates_authority(&report));
}

#[test]
fn root_integration_observability_snapshot_is_read_only() {
    let snapshot = observability_snapshot_from_supplied_evidence(
        "obs-root-1",
        None,
        None,
        None,
        None,
        None,
        vec![ObservedDiagnosticSummary {
            family: "diag".into(),
            code: "ok".into(),
            key: "k".into(),
            summary: "read-only".into(),
        }],
    );
    assert!(!snapshot.reads_persistence);
    assert!(!snapshot.writes_persistence);
    assert!(!snapshot.exports_data);
    assert!(!observability_snapshot_mutates_authority(&snapshot));
}

#[test]
fn root_integration_observability_snapshot_observes_harness_and_replay_without_authority() {
    let harness = run_end_to_end_local_harness(EndToEndLocalHarnessRequest {
        run_id: "obs-root-run".to_string(),
        provider_prompt: "deterministic local harness prompt".to_string(),
        operator_id: "operator-1".to_string(),
        target_id: "target-1".to_string(),
        reason: "obs".to_string(),
    });
    let replay = verify_provider_evidence_replay(
        "obs-replay",
        "obs-root-run",
        provider_evidence_snapshot_from_harness_report("obs-evidence", &harness),
    );
    let snapshot = observability_snapshot_from_supplied_evidence(
        "obs-root-2",
        Some(&harness),
        None,
        None,
        Some(&replay),
        None,
        vec![],
    );
    assert!(snapshot.harness.is_some());
    assert!(snapshot.replay.is_some());
    assert!(!observability_snapshot_mutates_authority(&snapshot));
}

#[test]
fn root_integration_audit_export_encoding_is_deterministic() {
    let snapshot = observability_snapshot_from_supplied_evidence(
        "audit-root-1",
        None,
        None,
        None,
        None,
        None,
        vec![ObservedDiagnosticSummary {
            family: "diag".into(),
            code: "ok".into(),
            key: "root".into(),
            summary: "deterministic".into(),
        }],
    );
    let first = match encode_audit_export_snapshot(
        "audit-export-root-1",
        &snapshot,
        AuditExportEncodingLimits::strict_defaults(),
    ) {
        Ok(envelope) => envelope,
        Err(_) => panic!("root integration audit export encoding should succeed"),
    };
    let second = match encode_audit_export_snapshot(
        "audit-export-root-1",
        &snapshot,
        AuditExportEncodingLimits::strict_defaults(),
    ) {
        Ok(envelope) => envelope,
        Err(_) => panic!("root integration audit export encoding should succeed"),
    };

    assert!(first.encoded_bytes == second.encoded_bytes);
    assert!(first.byte_len == second.byte_len);
}

#[test]
fn root_integration_audit_export_encoding_is_non_authoritative() {
    let snapshot = observability_snapshot_from_supplied_evidence(
        "audit-root-2",
        None,
        None,
        None,
        None,
        None,
        vec![ObservedDiagnosticSummary {
            family: "diag".into(),
            code: "ok".into(),
            key: "root".into(),
            summary: "non-authoritative".into(),
        }],
    );
    let envelope = match encode_audit_export_snapshot(
        "audit-export-root-2",
        &snapshot,
        AuditExportEncodingLimits::strict_defaults(),
    ) {
        Ok(envelope) => envelope,
        Err(_) => panic!("root integration audit export encoding should succeed"),
    };

    assert!(!envelope.writes_files);
    assert!(!envelope.reads_persistence);
    assert!(!envelope.writes_persistence);
    assert!(!envelope.mutates_authority);
}
