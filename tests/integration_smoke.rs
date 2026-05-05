use ajentic_core::api::{
    provider_evidence_snapshot_from_harness_report, run_end_to_end_local_harness,
    verify_provider_evidence_replay, EndToEndLocalHarnessRequest, EndToEndLocalHarnessStatus,
    ProviderEvidenceReplayMode, ProviderEvidenceReplayStatus,
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
