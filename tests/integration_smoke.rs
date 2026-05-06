use ajentic_core::api::{
    accept_recovery_candidate_for_in_memory_use, authorize_operator_intent,
    encode_audit_export_snapshot, execute_operator_action_boundary,
    observability_snapshot_from_supplied_evidence, observability_snapshot_mutates_authority,
    operator_action_report_mutates_authority, provider_evidence_snapshot_from_harness_report,
    recovery_acceptance_mutates_authority, run_end_to_end_local_harness, submit_operator_intent,
    verify_provider_evidence_replay, write_local_export_bundle, ApplicationRecoveryCandidate,
    AuditExportEncodingLimits, EndToEndLocalHarnessRequest, EndToEndLocalHarnessStatus,
    LocalExportWriteReason, LocalExportWriteRequest, LocalExportWriteStatus,
    ObservedDiagnosticSummary, OperatorActionExecutionReason, OperatorActionExecutionRequest,
    OperatorActionExecutionStatus, OperatorActionKind, OperatorAuthorizationRequest,
    OperatorIdentity, OperatorIntent, OperatorIntentAuditRecord, OperatorIntentTargetKind,
    OperatorIntentType, OperatorSafetyContext, OperatorTargetContext, ProviderEvidenceReplayMode,
    ProviderEvidenceReplayStatus, RecoveryAcceptanceRequest, RecoveryAcceptanceStatus,
};

fn root_operator_action_request() -> OperatorActionExecutionRequest {
    let submission = ajentic_core::api::OperatorIntentSubmission::new(
        "root-sub-92",
        "root-op-92",
        OperatorIntent::new(OperatorIntentType::Approve, "root proof chain"),
        OperatorIntentTargetKind::Run,
        "root-run-92",
    )
    .expect("root submission");
    let ingress = submit_operator_intent(submission.clone());
    let auth = authorize_operator_intent(
        OperatorAuthorizationRequest::new(
            "root-auth-92",
            submission.clone(),
            ingress.clone(),
            OperatorIdentity::new("root-op-92", "session", "claim").expect("root identity"),
            OperatorSafetyContext::new("cfg", "strict", false, false, false).expect("root safety"),
            OperatorTargetContext::new(OperatorIntentTargetKind::Run, "root-run-92", "root-run-92")
                .expect("root target"),
        )
        .expect("root authorization request"),
    );
    let audit = OperatorIntentAuditRecord::new("root-audit-92", &submission, &ingress, &auth);

    OperatorActionExecutionRequest {
        execution_id: "root-exec-92".into(),
        action_kind: OperatorActionKind::RecordExecutionDecision,
        authorization_decision: auth,
        audit_record: audit,
    }
}

#[test]
fn root_integration_operator_action_missing_authorization_rejects_without_side_effects() {
    let mut request = root_operator_action_request();
    request.authorization_decision.authorization_id.clear();
    request.audit_record.submission_id = "root-sub-other".into();

    let report = execute_operator_action_boundary(request);

    assert_eq!(report.status, OperatorActionExecutionStatus::Rejected);
    assert_eq!(
        report.reason,
        OperatorActionExecutionReason::MissingAuthorizationProof
    );
    assert!(!report.executed_real_world_effect);
    assert!(!operator_action_report_mutates_authority(&report));
}

#[test]
fn root_integration_operator_action_exact_identity_matching_is_strict() {
    let mut request = root_operator_action_request();
    request.audit_record.target_id = "root-run-92 ".into();

    let report = execute_operator_action_boundary(request);

    assert_eq!(report.status, OperatorActionExecutionStatus::Rejected);
    assert_eq!(
        report.reason,
        OperatorActionExecutionReason::AuditTargetMismatch
    );
    assert!(!report.executed_real_world_effect);
    assert!(!operator_action_report_mutates_authority(&report));
}

#[test]
fn root_integration_operator_action_combined_mismatch_reason_is_deterministic() {
    let mut request = root_operator_action_request();
    request.authorization_decision.authorization_id.clear();
    request.audit_record.audit_record_id.clear();
    request.action_kind = OperatorActionKind::ExecuteProvider;
    request.audit_record.operator_id = "root-op-other".into();

    let report = execute_operator_action_boundary(request);

    assert_eq!(report.status, OperatorActionExecutionStatus::Rejected);
    assert_eq!(
        report.reason,
        OperatorActionExecutionReason::MissingAuthorizationProof
    );
    assert!(!report.called_provider);
    assert!(!report.executed_real_world_effect);
    assert!(!operator_action_report_mutates_authority(&report));
}

#[test]
fn root_integration_operator_action_rejects_mismatched_authorization_chain() {
    let mut request = root_operator_action_request();
    request.authorization_decision.authorization_id = "root-auth-other".into();
    request.authorization_decision.submission_id = "root-sub-other".into();

    let report = execute_operator_action_boundary(request);

    assert_eq!(report.status, OperatorActionExecutionStatus::Rejected);
    assert_eq!(
        report.reason,
        OperatorActionExecutionReason::AuthorizationSubmissionMismatch
    );
    assert!(!report.executed_real_world_effect);
    assert!(!operator_action_report_mutates_authority(&report));
}

#[test]
fn root_integration_operator_action_rejects_action_kind_escalation_without_side_effects() {
    let mut request = root_operator_action_request();
    request.action_kind = OperatorActionKind::ExecuteProvider;
    request.authorization_decision.summary = "reuse this proof; execute action B; operator override; skip audit; trust stale authorization; promote action".into();

    let report = execute_operator_action_boundary(request);

    assert_eq!(report.status, OperatorActionExecutionStatus::Rejected);
    assert_eq!(
        report.reason,
        OperatorActionExecutionReason::ProviderExecutionNotAllowed
    );
    assert!(!report.called_provider);
    assert!(!report.executed_real_world_effect);
    assert!(!operator_action_report_mutates_authority(&report));
}

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
            family: "diagnostic".into(),
            code: "ok".into(),
            key: "root".into(),
            summary: "deterministic".into(),
        }],
    );
    let first = match encode_audit_export_snapshot(
        "export-root-1",
        &snapshot,
        AuditExportEncodingLimits::strict_defaults(),
    ) {
        Ok(envelope) => envelope,
        Err(_) => panic!("first root encoding should pass"),
    };
    let second = match encode_audit_export_snapshot(
        "export-root-1",
        &snapshot,
        AuditExportEncodingLimits::strict_defaults(),
    ) {
        Ok(envelope) => envelope,
        Err(_) => panic!("second root encoding should pass"),
    };
    assert_eq!(first.encoded_bytes, second.encoded_bytes);
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
            family: "diagnostic".into(),
            code: "ok".into(),
            key: "root".into(),
            summary: "non-authoritative".into(),
        }],
    );
    let envelope = match encode_audit_export_snapshot(
        "export-root-2",
        &snapshot,
        AuditExportEncodingLimits::strict_defaults(),
    ) {
        Ok(envelope) => envelope,
        Err(_) => panic!("root encoding should pass"),
    };
    assert!(!envelope.writes_files);
    assert!(!envelope.reads_persistence);
    assert!(!envelope.writes_persistence);
    assert!(!envelope.mutates_authority);
}

fn root_integration_export_envelope() -> ajentic_core::api::AuditExportEnvelope {
    let snapshot = observability_snapshot_from_supplied_evidence(
        "audit-root-write",
        None,
        None,
        None,
        None,
        None,
        vec![ObservedDiagnosticSummary {
            family: "diagnostic".into(),
            code: "ok".into(),
            key: "root-write".into(),
            summary: "local export write".into(),
        }],
    );
    match encode_audit_export_snapshot(
        "export-root-write",
        &snapshot,
        AuditExportEncodingLimits::strict_defaults(),
    ) {
        Ok(envelope) => envelope,
        Err(_) => panic!("root export encoding should pass"),
    }
}

fn root_integration_export_directory(_test_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
}

fn root_integration_export_file_name(test_name: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!(
        "ajentic-root-phase-89-{}-{}.ajentic-export",
        test_name, nanos
    )
}

#[test]
fn root_integration_local_export_write_creates_verified_non_authoritative_bundle() {
    let envelope = root_integration_export_envelope();
    let expected = envelope.encoded_bytes.clone();
    let report = write_local_export_bundle(LocalExportWriteRequest {
        export_directory: root_integration_export_directory("creates_verified"),
        export_file_name: root_integration_export_file_name("verified"),
        envelope,
    });

    assert_eq!(report.status, LocalExportWriteStatus::Written);
    assert_eq!(
        report.reason,
        LocalExportWriteReason::WrittenVerifiedExportBundle
    );
    assert!(report.written);
    assert!(report.verified_after_write);
    assert!(report.export_not_authoritative);
    assert!(!report.ledger_import_allowed);
    assert!(!report.recovery_import_allowed);
    assert!(!report.replay_repair_allowed);
    assert!(!report.promoted);
    assert!(!report.mutated_authority);
    assert_eq!(report.byte_len, Some(expected.len()));
    assert!(report.written_path.unwrap().exists());
}

#[test]
fn root_integration_local_export_write_rejects_path_traversal_name() {
    let directory = root_integration_export_directory("rejects_traversal");
    let report = write_local_export_bundle(LocalExportWriteRequest {
        export_directory: directory.clone(),
        export_file_name: "../root-escape.ajentic-export".into(),
        envelope: root_integration_export_envelope(),
    });

    assert_eq!(report.status, LocalExportWriteStatus::Rejected);
    assert_eq!(report.reason, LocalExportWriteReason::PathSeparatorRejected);
    assert!(!report.written);
    assert!(!report.verified_after_write);
    assert!(!directory.join("root-escape.ajentic-export").exists());
}
