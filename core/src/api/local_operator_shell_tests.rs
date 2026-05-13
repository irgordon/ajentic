use super::*;

fn phase_151_package_state_with_available_sections() -> LocalOperatorShellState {
    let configured = apply_local_provider_configuration_candidate(
        &initial_local_operator_shell_state(),
        LocalProviderConfigurationCandidate::deterministic_stub(),
    )
    .unwrap();
    let executed = apply_local_provider_execution(
        &configured,
        LocalProviderExecutionRequest::deterministic_stub("phase 151 package input"),
    )
    .unwrap();
    let proposal = create_staged_candidate_conversion_proposal(
        &executed,
        StagedCandidateConversionProposalRequest::staging_only("phase 151 package proposal"),
    )
    .unwrap();
    let validated = validate_staged_candidate_conversion_proposal_for_phase_147(
        &proposal,
        StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    let proposal_id = validated
        .staged_candidate_conversion_proposal
        .proposal
        .as_ref()
        .unwrap()
        .proposal_id
        .clone();
    let result_id = validated
        .provider_execution
        .result
        .as_ref()
        .unwrap()
        .result_id
        .clone();
    let decided = submit_operator_candidate_decision(
        &validated,
        OperatorCandidateDecisionRequest::approve(proposal_id, result_id),
    )
    .unwrap();
    let mut run = start_deterministic_stub_run(&decided);
    run.run.decision_replay =
        derive_local_decision_replay_projection(&run.run, &run.decision_ledger);
    run.run.replay_status = run.run.decision_replay.replay_status.code().to_string();
    attach_local_session_evidence_export(run)
}

#[test]
fn phase_151_initial_shell_state_exposes_not_packaged_projection() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.local_session_package_projection.status,
        LocalSessionPackageStatus::NotPackaged
    );
    assert_eq!(state.local_session_package_projection.package_id, None);
    assert_eq!(
        state
            .local_session_package_projection
            .package_classification,
        "local_session_package_only"
    );
    assert_eq!(
        state
            .local_session_package_projection
            .production_classification,
        "non_production"
    );
}

#[test]
fn phase_151_package_derivation_serialization_and_id_are_deterministic() {
    let state = phase_151_package_state_with_available_sections();
    let before = state.clone();
    let first = derive_local_session_package(&state);
    let second = derive_local_session_package(&state);
    assert_eq!(first, second);
    assert_eq!(first.metadata.package_id, second.metadata.package_id);
    assert_eq!(
        serialize_local_session_package(&first).unwrap(),
        serialize_local_session_package(&second).unwrap()
    );
    assert_eq!(state, before);
    validate_local_session_package(&first).unwrap();
}

#[test]
fn phase_151_package_includes_available_sections_and_absence_markers() {
    let package = derive_local_session_package(&phase_151_package_state_with_available_sections());
    assert!(package
        .payload
        .provider_configuration_projection
        .contains("DeterministicStub"));
    assert!(package
        .payload
        .provider_execution_result_projection
        .contains("LocalProviderExecutionResult"));
    assert!(package
        .payload
        .provider_output_validation_projection
        .contains("ReviewableUntrusted"));
    assert!(package
        .payload
        .staged_candidate_conversion_proposal_projection
        .contains("proposal"));
    assert!(package
        .payload
        .staged_candidate_conversion_validation_projection
        .contains("StagedProposalShapeValid"));
    assert!(package
        .payload
        .operator_candidate_decision_projection
        .contains("ApprovedValidatedStagedProposal"));
    assert!(package
        .payload
        .local_decision_ledger_projection
        .contains("records"));
    assert!(package
        .payload
        .replay_status_projection
        .contains("LocalDecisionReplayProjection"));
    assert!(package
        .payload
        .local_session_evidence_export_projection
        .contains("LocalSessionEvidenceExport"));
    assert!(package
        .payload
        .phase_150_handoff_context_projection
        .contains("phase_150"));
    assert!(package.absence_markers.release_artifact_absent);
    assert!(package.absence_markers.deployment_artifact_absent);
    assert!(package.absence_markers.readiness_evidence_absent);
    assert!(package.absence_markers.action_execution_absent);
}

#[test]
fn phase_151_validation_rejects_missing_invalid_and_claim_bearing_package_content() {
    let package = derive_local_session_package(&initial_local_operator_shell_state());
    let mut missing_id = package.clone();
    missing_id.metadata.package_id.clear();
    assert!(validate_local_session_package(&missing_id)
        .unwrap_err()
        .contains(&LocalSessionPackageValidationError::MissingPackageId));
    let mut missing_version = package.clone();
    missing_version.metadata.package_version.clear();
    assert!(validate_local_session_package(&missing_version)
        .unwrap_err()
        .contains(&LocalSessionPackageValidationError::MissingPackageVersion));
    let mut invalid_classification = package.clone();
    invalid_classification.metadata.package_classification = "release_artifact".to_string();
    assert!(validate_local_session_package(&invalid_classification)
        .unwrap_err()
        .contains(&LocalSessionPackageValidationError::InvalidPackageClassification));
    let mut invalid_production = package.clone();
    invalid_production.metadata.production_classification = "production".to_string();
    assert!(validate_local_session_package(&invalid_production)
        .unwrap_err()
        .contains(&LocalSessionPackageValidationError::InvalidProductionClassification));
    let mut missing_marker = package.clone();
    missing_marker.absence_markers.update_channel_absent = false;
    assert!(validate_local_session_package(&missing_marker)
        .unwrap_err()
        .contains(&LocalSessionPackageValidationError::MissingAbsenceMarker));
    let mut drifted = package.clone();
    drifted
        .payload
        .provider_configuration_projection
        .push_str("drift");
    assert!(validate_local_session_package(&drifted)
        .unwrap_err()
        .contains(&LocalSessionPackageValidationError::DeterministicContentMismatch));
    for (claim, expected) in [
        (
            " claim:production_ready ",
            LocalSessionPackageValidationError::ForbiddenReleaseReadinessOrDeploymentClaim,
        ),
        (
            " claim:signing_enabled ",
            LocalSessionPackageValidationError::ForbiddenSigningPublishingInstallerOrUpdateClaim,
        ),
        (
            " claim:provider_trusted ",
            LocalSessionPackageValidationError::ForbiddenProviderTrustClaim,
        ),
        (
            " claim:candidate_approved ",
            LocalSessionPackageValidationError::ForbiddenCandidateApprovalClaim,
        ),
        (
            " claim:action_executed ",
            LocalSessionPackageValidationError::ForbiddenActionExecutionClaim,
        ),
        (
            " claim:durable persistence authority ",
            LocalSessionPackageValidationError::ForbiddenPersistenceAuthorityClaim,
        ),
    ] {
        let mut claimed = package.clone();
        claimed
            .payload
            .provider_configuration_projection
            .push_str(claim);
        assert!(validate_local_session_package(&claimed)
            .unwrap_err()
            .contains(&expected));
    }
}

#[test]
fn phase_152_initial_history_and_restore_projection_are_local_only() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.local_session_history_projection.status,
        LocalSessionHistoryStatus::NoSessionHistory
    );
    assert!(state.local_session_history_projection.entries.is_empty());
    assert_eq!(
        state.local_session_restore_projection.status,
        LocalSessionRestoreStatus::RestoreNotRequested
    );
    assert!(state
        .local_session_restore_projection
        .boundary_status
        .contains(&LocalSessionRestoreBoundaryStatus::LocalRestoreProjectionOnly));
    assert!(state
        .local_session_restore_projection
        .boundary_status
        .contains(&LocalSessionRestoreBoundaryStatus::NoRecoveryPromotion));
    assert!(state
        .local_session_restore_projection
        .boundary_status
        .contains(&LocalSessionRestoreBoundaryStatus::NoReplayRepair));
    assert!(state
        .local_session_restore_projection
        .local_only_note
        .contains("local-only and non-production"));
}

#[test]
fn phase_152_history_projection_is_deterministic_from_explicit_packages() {
    let package = derive_local_session_package(&initial_local_operator_shell_state());
    let first = project_local_session_history(std::slice::from_ref(&package));
    let second = project_local_session_history(std::slice::from_ref(&package));

    assert_eq!(first, second);
    assert_eq!(
        first.status,
        LocalSessionHistoryStatus::SessionHistoryProjected
    );
    assert_eq!(first.entries.len(), 1);
    assert_eq!(first.entries[0].package_id, package.metadata.package_id);
    assert_eq!(
        first.entries[0].package_classification,
        "local_session_package_only"
    );
    assert_eq!(first.entries[0].production_classification, "non_production");
}

#[test]
fn phase_152_valid_package_produces_deterministic_restore_preview_and_projection() {
    let state = initial_local_operator_shell_state();
    let before = state.clone();
    let package = derive_local_session_package(&state);
    let serialized = serialize_local_session_package(&package).unwrap();
    let candidate = create_local_session_restore_candidate(
        LocalSessionRestoreRequest::ExplicitPackagePayload(serialized.clone()),
    );
    validate_local_session_restore_candidate(&candidate).unwrap();
    assert_eq!(
        candidate.read_back_status,
        LocalSessionRestoreReadBackStatus::PackageReadBackValidated
    );

    let first = project_local_session_restore_from_payload(&serialized);
    let second = project_local_session_restore_from_payload(&serialized);
    assert_eq!(first, second);
    assert_eq!(
        first.status,
        LocalSessionRestoreStatus::RestorePreviewProjected
    );
    assert_eq!(first.package_id, Some(package.metadata.package_id.clone()));
    assert_eq!(
        first.package_classification,
        Some("local_session_package_only".to_string())
    );
    assert_eq!(
        first.production_classification,
        Some("non_production".to_string())
    );
    assert!(first
        .boundary_status
        .contains(&LocalSessionRestoreBoundaryStatus::NoRecoveryPromotion));
    assert!(first
        .boundary_status
        .contains(&LocalSessionRestoreBoundaryStatus::NoReplayRepair));
    let projection = derive_local_session_restore_projection(&package).unwrap();
    assert_eq!(
        projection.status,
        LocalSessionRestoreStatus::RestoreProjected
    );
    assert_eq!(state, before);
}

#[test]
fn phase_152_restore_rejects_malformed_and_missing_required_sections() {
    let malformed = project_local_session_restore_from_payload("not a local session package");
    assert_eq!(
        malformed.status,
        LocalSessionRestoreStatus::InvalidRestoreInput
    );
    assert!(malformed
        .errors
        .contains(&LocalSessionRestoreError::MissingRequiredPackageSection));

    let package = derive_local_session_package(&initial_local_operator_shell_state());
    let serialized = serialize_local_session_package(&package).unwrap();
    let missing_section = serialized
        .lines()
        .filter(|line| !line.starts_with("absence_markers="))
        .collect::<Vec<_>>()
        .join(
            "
",
        );
    let rejected = project_local_session_restore_from_payload(&missing_section);
    assert_eq!(
        rejected.status,
        LocalSessionRestoreStatus::InvalidRestoreInput
    );
    assert!(rejected
        .errors
        .contains(&LocalSessionRestoreError::MissingRequiredPackageSection));
}

#[test]
fn phase_152_restore_rejects_invalid_classification_and_authority_claims() {
    let package = derive_local_session_package(&initial_local_operator_shell_state());
    let mut missing_id = package.clone();
    missing_id.metadata.package_id.clear();
    assert!(derive_local_session_restore_preview(&missing_id)
        .unwrap_err()
        .contains(&LocalSessionRestoreError::PackageValidationFailed));
    let mut missing_version = package.clone();
    missing_version.metadata.package_version.clear();
    assert!(derive_local_session_restore_preview(&missing_version)
        .unwrap_err()
        .contains(&LocalSessionRestoreError::PackageValidationFailed));
    let mut invalid_classification = package.clone();
    invalid_classification.metadata.package_classification = "release_artifact".to_string();
    assert!(
        derive_local_session_restore_preview(&invalid_classification)
            .unwrap_err()
            .contains(&LocalSessionRestoreError::InvalidPackageClassification)
    );
    let mut invalid_production = package.clone();
    invalid_production.metadata.production_classification = "production".to_string();
    assert!(derive_local_session_restore_preview(&invalid_production)
        .unwrap_err()
        .contains(&LocalSessionRestoreError::InvalidProductionClassification));
    let mut missing_marker = package.clone();
    missing_marker.absence_markers.remote_sync_absent = false;
    assert!(derive_local_session_restore_preview(&missing_marker)
        .unwrap_err()
        .contains(&LocalSessionRestoreError::MissingAbsenceMarker));

    for (claim, expected) in [
        (
            " claim:readiness_approved ",
            LocalSessionRestoreError::ReadinessClaimDetected,
        ),
        (
            " claim:release_candidate_approved ",
            LocalSessionRestoreError::ReleaseClaimDetected,
        ),
        (
            " claim:deployment_enabled ",
            LocalSessionRestoreError::DeploymentClaimDetected,
        ),
        (
            " claim:public_use_approved ",
            LocalSessionRestoreError::PublicUseClaimDetected,
        ),
        (
            " claim:provider_trusted ",
            LocalSessionRestoreError::ProviderTrustClaimDetected,
        ),
        (
            " claim:candidate_approved ",
            LocalSessionRestoreError::CandidateApprovalClaimDetected,
        ),
        (
            " claim:action_executed ",
            LocalSessionRestoreError::ActionExecutionClaimDetected,
        ),
        (
            " claim:replay_repaired ",
            LocalSessionRestoreError::ReplayRepairClaimDetected,
        ),
        (
            " claim:recovery_promoted ",
            LocalSessionRestoreError::RecoveryPromotionClaimDetected,
        ),
    ] {
        let mut claimed = package.clone();
        claimed
            .payload
            .provider_configuration_projection
            .push_str(claim);
        assert!(derive_local_session_restore_preview(&claimed)
            .unwrap_err()
            .contains(&expected));
    }
}

#[test]
fn transport_initial_state_returns_idle_non_production_projection() {
    let mut transport = LocalOperatorShellTransport::new();
    let response = transport.step(LocalOperatorShellRequest::GetInitialState);

    assert_eq!(response.status, LocalOperatorShellTransportStatus::Accepted);
    assert_eq!(response.reason, "initial_shell_state_returned");
    assert!(response.state.non_production);
    assert_eq!(response.state.run.status, LocalRunStatus::Idle);
    assert!(response.capabilities.local_only);
    assert!(!response.capabilities.provider_execution_enabled);
    assert!(response.state.decision_ledger.records.is_empty());
    assert!(response.state.run.decision_timeline.is_empty());
}

#[test]
fn transport_stub_run_returns_deterministic_output() {
    let state = initial_local_operator_shell_state();
    let first = local_operator_shell_transport_step(
        &state,
        LocalOperatorShellRequest::StartDeterministicStubRun,
    );
    let second = local_operator_shell_transport_step(
        &state,
        LocalOperatorShellRequest::StartDeterministicStubRun,
    );

    assert_eq!(first, second);
    assert_eq!(first.status, LocalOperatorShellTransportStatus::Accepted);
    assert_eq!(first.state.run.status, LocalRunStatus::StubCompleted);
    assert_eq!(
        first.state.run.candidate.as_ref().unwrap().candidate_id,
        "candidate-local-stub-133"
    );
    assert!(first.state.decision_ledger.records.is_empty());
    assert!(first.state.run.decision_timeline.records.is_empty());
}

#[test]
fn transport_records_approve_and_reject_intents() {
    for kind in [
        LocalOperatorIntentKind::Approve,
        LocalOperatorIntentKind::Reject,
    ] {
        let mut transport = LocalOperatorShellTransport::new();
        let started = start_local_operator_shell_stub_run(&mut transport);
        let response = submit_local_operator_shell_intent(
            &mut transport,
            LocalOperatorIntent::new(
                kind,
                "operator-local",
                &started.state.run.run_id,
                "reviewed locally",
            ),
        );

        assert_eq!(response.status, LocalOperatorShellTransportStatus::Accepted);
        assert_eq!(response.state.run.selected_intent, Some(kind));
        assert_eq!(response.state.run.decision_timeline.records.len(), 1);
        let record = &response.state.run.decision_timeline.records[0];
        assert_eq!(record.intent_kind, kind.into());
        assert_eq!(record.decision_status, LocalDecisionRecordStatus::Recorded);
        assert_eq!(record.recorded_sequence, 1);
        assert_eq!(record.decision_id, "local-decision-0001");
        assert!(response.state.run.timeline.iter().any(|entry| entry
            == &format!(
                "operator intent recorded: {} by operator-local as decision local-decision-0001",
                kind.code()
            )));
    }
}

#[test]
fn transport_rejects_invalid_target_run_and_candidate() {
    let mut transport = LocalOperatorShellTransport::new();
    let started = start_local_operator_shell_stub_run(&mut transport);

    let invalid_run = submit_local_operator_shell_intent(
        &mut transport,
        LocalOperatorIntent::new(
            LocalOperatorIntentKind::Approve,
            "operator-local",
            "wrong-run",
            "reviewed locally",
        ),
    );
    assert_eq!(
        invalid_run.status,
        LocalOperatorShellTransportStatus::Rejected
    );
    assert_eq!(invalid_run.reason, "target_mismatch");

    let mut invalid_candidate = LocalOperatorIntent::new(
        LocalOperatorIntentKind::Approve,
        "operator-local",
        &started.state.run.run_id,
        "reviewed locally",
    );
    invalid_candidate.target_candidate_id = "wrong-candidate".to_string();
    let response = submit_local_operator_shell_intent(&mut transport, invalid_candidate);
    assert_eq!(response.status, LocalOperatorShellTransportStatus::Rejected);
    assert_eq!(response.reason, "candidate_target_mismatch");
    assert!(response.state.decision_ledger.records.is_empty());
    assert!(transport.current_state().decision_ledger.records.is_empty());
}

#[test]
fn duplicate_decision_for_same_run_candidate_fails_closed() {
    let mut transport = LocalOperatorShellTransport::new();
    let started = start_local_operator_shell_stub_run(&mut transport);
    let first = submit_local_operator_shell_intent(
        &mut transport,
        LocalOperatorIntent::new(
            LocalOperatorIntentKind::Approve,
            "operator-local",
            &started.state.run.run_id,
            "reviewed locally",
        ),
    );
    assert_eq!(first.status, LocalOperatorShellTransportStatus::Accepted);
    assert_eq!(first.state.decision_ledger.records.len(), 1);

    let duplicate = submit_local_operator_shell_intent(
        &mut transport,
        LocalOperatorIntent::new(
            LocalOperatorIntentKind::Reject,
            "operator-local",
            &started.state.run.run_id,
            "duplicate local decision",
        ),
    );
    assert_eq!(
        duplicate.status,
        LocalOperatorShellTransportStatus::Rejected
    );
    assert_eq!(duplicate.reason, "duplicate_decision_rejected");
    assert_eq!(duplicate.state.decision_ledger.records.len(), 1);
    assert_eq!(transport.current_state().decision_ledger.records.len(), 1);
}

#[test]
fn forbidden_transport_requests_fail_closed() {
    let state = start_deterministic_stub_run(&initial_local_operator_shell_state());
    for request in [
        LocalOperatorShellForbiddenRequest::AuthorityGrant,
        LocalOperatorShellForbiddenRequest::ProviderExecution,
        LocalOperatorShellForbiddenRequest::ReadinessClaim,
        LocalOperatorShellForbiddenRequest::ProductionStateMutation,
        LocalOperatorShellForbiddenRequest::ReleaseArtifactCreation,
        LocalOperatorShellForbiddenRequest::DeploymentEnablement,
        LocalOperatorShellForbiddenRequest::SigningEnablement,
        LocalOperatorShellForbiddenRequest::PublishingEnablement,
    ] {
        let response = local_operator_shell_transport_step(
            &state,
            LocalOperatorShellRequest::Forbidden(request),
        );
        assert_eq!(response.status, LocalOperatorShellTransportStatus::Rejected);
        assert_eq!(response.reason, request.rejection_code());
        assert_eq!(response.state, state);
    }
}

#[test]
fn replay_projection_exposes_no_decision_then_approved_and_rejected_states() {
    let initial = initial_local_operator_shell_state();
    assert_eq!(
        initial.run.decision_replay.replay_status,
        LocalDecisionReplayStatus::NoDecisionRecorded
    );
    assert_eq!(initial.run.decision_replay.source_decision_count, 0);

    let started = start_deterministic_stub_run(&initial);
    assert_eq!(
        started.run.decision_replay.replay_status,
        LocalDecisionReplayStatus::NoDecisionRecorded
    );

    for (kind, expected_status) in [
        (
            LocalOperatorIntentKind::Approve,
            LocalDecisionReplayStatus::ApprovedDecisionReplayed,
        ),
        (
            LocalOperatorIntentKind::Reject,
            LocalDecisionReplayStatus::RejectedDecisionReplayed,
        ),
    ] {
        let state = start_deterministic_stub_run(&initial_local_operator_shell_state());
        let updated = apply_local_operator_intent(
            &state,
            LocalOperatorIntent::new(
                kind,
                "operator-local",
                &state.run.run_id,
                "reviewed locally",
            ),
        )
        .expect("valid local decision should record");
        assert_eq!(updated.run.decision_replay.replay_status, expected_status);
        assert_eq!(updated.run.decision_replay.source_decision_count, 1);
        assert_eq!(
            updated.run.decision_replay.latest_decision_id.as_deref(),
            Some("local-decision-0001")
        );
        assert_eq!(
            updated.run.decision_replay.latest_operator_id.as_deref(),
            Some("operator-local")
        );
        assert_eq!(
            updated.run.decision_replay.latest_decision_kind,
            Some(kind.into())
        );
        assert_eq!(
            updated.run.decision_replay.integrity_status,
            LocalDecisionReplayIntegrityStatus::Consistent
        );
    }
}

#[test]
fn replay_projection_is_deterministic_and_does_not_mutate_inputs() {
    let state = start_deterministic_stub_run(&initial_local_operator_shell_state());
    let updated = apply_local_operator_intent(
        &state,
        LocalOperatorIntent::new(
            LocalOperatorIntentKind::Approve,
            "operator-local",
            &state.run.run_id,
            "reviewed locally",
        ),
    )
    .expect("valid local decision should record");
    let run_before = updated.run.clone();
    let ledger_before = updated.decision_ledger.clone();

    let first = derive_local_decision_replay_projection(&updated.run, &updated.decision_ledger);
    let second = derive_local_decision_replay_projection(&updated.run, &updated.decision_ledger);

    assert_eq!(first, second);
    assert_eq!(updated.run, run_before);
    assert_eq!(updated.decision_ledger, ledger_before);
}

#[test]
fn replay_projection_fails_closed_for_malformed_ledger() {
    let state = start_deterministic_stub_run(&initial_local_operator_shell_state());
    let mut ledger = state.decision_ledger.clone();
    ledger.records.push(LocalDecisionRecord {
        decision_id: "local-decision-0002".to_string(),
        run_id: state.run.run_id.clone(),
        candidate_id: state.run.candidate.as_ref().unwrap().candidate_id.clone(),
        operator_id: "operator-local".to_string(),
        intent_kind: LocalDecisionRecordKind::Approve,
        decision_status: LocalDecisionRecordStatus::Recorded,
        validation_status: "accepted_by_rust_local_validation".to_string(),
        recorded_sequence: 2,
        recorded_at_label: "local-sequence-0002".to_string(),
        reason: "malformed sequence".to_string(),
    });

    let projection = derive_local_decision_replay_projection(&state.run, &ledger);

    assert_eq!(
        projection.replay_status,
        LocalDecisionReplayStatus::InconsistentDecisionLedger
    );
    assert_eq!(
        projection.integrity_status,
        LocalDecisionReplayIntegrityStatus::Inconsistent
    );
    assert_eq!(
        projection.error,
        Some(LocalDecisionReplayError::SequenceMismatch)
    );
}

#[test]
fn invalid_intent_leaves_replay_projection_unchanged() {
    let mut transport = LocalOperatorShellTransport::new();
    let started = start_local_operator_shell_stub_run(&mut transport);
    let before = started.state.run.decision_replay.clone();
    let rejected = submit_local_operator_shell_intent(
        &mut transport,
        LocalOperatorIntent::new(
            LocalOperatorIntentKind::Approve,
            "",
            &started.state.run.run_id,
            "reviewed locally",
        ),
    );

    assert_eq!(rejected.status, LocalOperatorShellTransportStatus::Rejected);
    assert_eq!(rejected.state.run.decision_replay, before);
    assert_eq!(transport.current_state().run.decision_replay, before);
}

#[test]
fn transport_exposes_no_production_or_provider_authority() {
    let response = local_operator_shell_transport_step(
        &initial_local_operator_shell_state(),
        LocalOperatorShellRequest::GetCurrentState,
    );
    let capabilities = response.capabilities;

    assert!(capabilities.local_only);
    assert!(capabilities.non_production);
    assert!(!capabilities.provider_execution_enabled);
    assert!(!capabilities.cloud_model_calls_enabled);
    assert!(!capabilities.broad_command_execution_enabled);
    assert!(!capabilities.production_persistence_enabled);
    assert!(!capabilities.release_artifact_creation_enabled);
    assert!(!capabilities.deployment_enabled);
    assert!(!capabilities.signing_authority_available);
    assert!(!capabilities.publishing_authority_available);
    assert!(!capabilities.readiness_approval_enabled);
}

#[test]
fn deterministic_stub_run_produces_identical_projection() {
    let state = initial_local_operator_shell_state();
    let first = start_deterministic_stub_run(&state);
    let second = start_deterministic_stub_run(&state);
    assert_eq!(first, second);
    assert_eq!(first.run.status, LocalRunStatus::StubCompleted);
    assert!(
        !first
            .run
            .candidate
            .as_ref()
            .unwrap()
            .provider_execution_enabled
    );
}

#[test]
fn approve_and_reject_intents_are_typed_and_recorded() {
    for kind in [
        LocalOperatorIntentKind::Approve,
        LocalOperatorIntentKind::Reject,
    ] {
        let state = start_deterministic_stub_run(&initial_local_operator_shell_state());
        let intent = LocalOperatorIntent::new(
            kind,
            "operator-local",
            &state.run.run_id,
            "reviewed locally",
        );
        let updated =
            apply_local_operator_intent(&state, intent).expect("typed intent should record");
        assert_eq!(updated.run.status, LocalRunStatus::IntentRecorded);
        assert_eq!(updated.run.selected_intent, Some(kind));
        assert_eq!(updated.decision_ledger.records.len(), 1);
        assert_eq!(updated.run.decision_timeline.records.len(), 1);
    }
}

#[test]
fn invalid_operator_intent_fails_closed() {
    let state = start_deterministic_stub_run(&initial_local_operator_shell_state());
    let err = apply_local_operator_intent(
        &state,
        LocalOperatorIntent::new(
            LocalOperatorIntentKind::Approve,
            "",
            &state.run.run_id,
            "reviewed locally",
        ),
    )
    .expect_err("empty operator should fail");
    assert_eq!(err, LocalOperatorShellError::EmptyOperatorId);
}

#[test]
fn ui_boundary_cannot_grant_authority_or_readiness_or_provider_execution() {
    let state = start_deterministic_stub_run(&initial_local_operator_shell_state());

    let mut authority_intent = LocalOperatorIntent::new(
        LocalOperatorIntentKind::Approve,
        "operator-local",
        &state.run.run_id,
        "reviewed locally",
    );
    authority_intent.requests_authority_grant = true;
    assert_eq!(
        apply_local_operator_intent(&state, authority_intent).expect_err("authority grant fails"),
        LocalOperatorShellError::AuthorityClaimRejected
    );

    let mut provider_intent = LocalOperatorIntent::new(
        LocalOperatorIntentKind::Approve,
        "operator-local",
        &state.run.run_id,
        "reviewed locally",
    );
    provider_intent.requests_provider_execution = true;
    assert_eq!(
        apply_local_operator_intent(&state, provider_intent).expect_err("provider execution fails"),
        LocalOperatorShellError::ProviderExecutionRejected
    );

    let mut readiness_intent = LocalOperatorIntent::new(
        LocalOperatorIntentKind::Approve,
        "operator-local",
        &state.run.run_id,
        "reviewed locally",
    );
    readiness_intent.claims_readiness = true;
    assert_eq!(
        apply_local_operator_intent(&state, readiness_intent).expect_err("readiness claim fails"),
        LocalOperatorShellError::ReadinessClaimRejected
    );
}

#[test]
fn session_evidence_export_initial_stub_and_decision_states_are_complete() {
    let initial = initial_local_operator_shell_state();
    assert_eq!(
        initial.local_session_evidence_export.export_status,
        LocalSessionEvidenceExportStatus::NoCompletedRunEvidence
    );
    assert_eq!(
        initial.local_session_evidence_export.export_classification,
        "local_session_evidence_only"
    );
    assert_eq!(
        initial
            .local_session_evidence_export
            .production_classification,
        "non-production"
    );
    assert_eq!(
        validate_local_session_evidence_export(&initial.local_session_evidence_export, true),
        Ok(())
    );

    let started = start_deterministic_stub_run(&initial);
    let export = &started.local_session_evidence_export;
    assert_eq!(
        export.export_status,
        LocalSessionEvidenceExportStatus::RunEvidenceProjected
    );
    assert_eq!(export.run_id, "local-stub-run-133");
    assert_eq!(export.run_status, LocalRunStatus::StubCompleted);
    assert_eq!(export.candidate_id, "candidate-local-stub-133");
    assert_eq!(export.validation_status, "pass_for_local_stub_review");
    assert_eq!(export.policy_status, "pass_for_local_stub_review");
    assert_eq!(export.decision_count, 0);
    assert_eq!(
        export.replay_status,
        LocalDecisionReplayStatus::NoDecisionRecorded
    );
    assert_eq!(
        export.export_validation_status,
        LocalSessionEvidenceExportValidationStatus::Complete
    );

    let decided = apply_local_operator_intent(
        &started,
        LocalOperatorIntent::new(
            LocalOperatorIntentKind::Approve,
            "operator-local",
            &started.run.run_id,
            "reviewed locally",
        ),
    )
    .expect("valid decision should record");
    let export = &decided.local_session_evidence_export;
    assert_eq!(
        export.export_status,
        LocalSessionEvidenceExportStatus::DecisionEvidenceProjected
    );
    assert_eq!(export.decision_count, 1);
    assert_eq!(
        export.decision_records[0].decision_id,
        "local-decision-0001"
    );
    assert_eq!(
        export.replay_status,
        LocalDecisionReplayStatus::ApprovedDecisionReplayed
    );
    assert_eq!(
        export.replay_integrity_status,
        LocalDecisionReplayIntegrityStatus::Consistent
    );
    assert!(export.absence_markers.provider_execution_absent);
    assert!(export.absence_markers.persistence_absent);
    assert!(export.absence_markers.release_absent);
    assert!(export.absence_markers.deployment_absent);
    assert!(export.absence_markers.signing_absent);
    assert!(export.absence_markers.publishing_absent);
    assert!(export.absence_markers.installer_absent);
    assert!(export.absence_markers.update_channel_absent);
    assert!(export.absence_markers.public_use_absent);
    assert!(export.absence_markers.readiness_approval_absent);
    assert!(export
        .absence_markers
        .marker_summary
        .contains(&"provider execution absent".to_string()));
    assert!(export
        .absence_markers
        .marker_summary
        .contains(&"release absent".to_string()));
    assert!(export
        .absence_markers
        .marker_summary
        .contains(&"deployment absent".to_string()));
    assert!(export
        .absence_markers
        .marker_summary
        .contains(&"readiness absent".to_string()));
}

#[test]
fn session_evidence_export_is_deterministic_and_non_mutating() {
    let started = start_deterministic_stub_run(&initial_local_operator_shell_state());
    let decided = apply_local_operator_intent(
        &started,
        LocalOperatorIntent::new(
            LocalOperatorIntentKind::Reject,
            "operator-local",
            &started.run.run_id,
            "reviewed locally",
        ),
    )
    .expect("valid decision should record");
    let before = decided.clone();

    let first = project_local_session_evidence_export(&decided);
    let second = project_local_session_evidence_export(&decided);

    assert_eq!(first, second);
    assert_eq!(decided, before);
    assert_eq!(decided.decision_ledger.records.len(), 1);
    assert_eq!(first.decision_records, decided.decision_ledger.records);
}

#[test]
fn session_evidence_export_validation_fails_closed_for_bad_classification_and_markers() {
    let state = start_deterministic_stub_run(&initial_local_operator_shell_state());
    let mut export = state.local_session_evidence_export.clone();

    export.export_classification = "release_evidence".to_string();
    assert_eq!(
        validate_local_session_evidence_export(&export, true),
        Err(LocalSessionEvidenceExportError::InvalidExportClassification)
    );

    let mut export = state.local_session_evidence_export.clone();
    export.production_classification = "production".to_string();
    assert_eq!(
        validate_local_session_evidence_export(&export, true),
        Err(LocalSessionEvidenceExportError::InvalidProductionClassification)
    );

    let mut export = state.local_session_evidence_export.clone();
    export.absence_markers.provider_execution_absent = false;
    assert_eq!(
        validate_local_session_evidence_export(&export, true),
        Err(LocalSessionEvidenceExportError::MissingAbsenceMarker)
    );

    let mut export = state.local_session_evidence_export.clone();
    export.candidate_id = "not_applicable_until_stub_run".to_string();
    assert_eq!(
        validate_local_session_evidence_export(&export, true),
        Err(LocalSessionEvidenceExportError::MissingRunEvidence)
    );
}

#[test]
fn rejected_or_forbidden_requests_do_not_promote_export_evidence() {
    let mut transport = LocalOperatorShellTransport::new();
    let started = start_local_operator_shell_stub_run(&mut transport);
    let before = started.state.local_session_evidence_export.clone();
    let mut forbidden_intent = LocalOperatorIntent::new(
        LocalOperatorIntentKind::Approve,
        "operator-local",
        &started.state.run.run_id,
        "request provider execution",
    );
    forbidden_intent.requests_provider_execution = true;

    let rejected = submit_local_operator_shell_intent(&mut transport, forbidden_intent);
    assert_eq!(rejected.status, LocalOperatorShellTransportStatus::Rejected);
    assert_eq!(rejected.state.local_session_evidence_export, before);
    assert_eq!(
        transport.current_state().local_session_evidence_export,
        before
    );

    let forbidden = transport.step(LocalOperatorShellRequest::Forbidden(
        LocalOperatorShellForbiddenRequest::ReleaseArtifactCreation,
    ));
    assert_eq!(
        forbidden.status,
        LocalOperatorShellTransportStatus::Rejected
    );
    assert_eq!(forbidden.state.local_session_evidence_export, before);
}

#[test]
fn phase_139_initial_provider_configuration_is_visible_and_non_executable() {
    let state = initial_local_operator_shell_state();
    let projection = project_local_provider_configuration(&state.provider_configuration);
    assert_eq!(projection.configured_provider_kind, "none");
    assert_eq!(
        projection.status,
        LocalProviderConfigurationStatus::NotConfigured
    );
    assert_eq!(projection.execution_status, "disabled_not_executed");
    assert!(projection.capability_surface.configuration_only);
    assert!(!projection.capability_surface.provider_execution_enabled);
    assert!(!projection.capability_surface.cloud_calls_enabled);
    assert!(!projection.capability_surface.network_enabled);
    assert!(!projection.capability_surface.shell_commands_enabled);
    assert!(!projection.capability_surface.filesystem_enabled);
    assert!(!projection.capability_surface.secrets_allowed);
}

#[test]
fn phase_139_accepts_valid_deterministic_stub_configuration_only() {
    let mut transport = LocalOperatorShellTransport::new();
    let before = transport.current_state();
    let response = submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate::deterministic_stub(),
    );

    assert_eq!(response.status, LocalOperatorShellTransportStatus::Accepted);
    assert_eq!(response.reason, "local_provider_configuration_accepted");
    assert_eq!(
        response.state.provider_configuration.status,
        LocalProviderConfigurationStatus::Accepted
    );
    assert_eq!(
        response
            .state
            .provider_configuration
            .configured_provider_kind,
        Some(LocalProviderKind::DeterministicStub)
    );
    assert_eq!(response.state.run.status, before.run.status);
    assert_eq!(response.state.run.candidate, before.run.candidate);
    assert_eq!(response.state.decision_ledger, before.decision_ledger);
    assert_eq!(
        response.state.run.decision_replay,
        before.run.decision_replay
    );
    assert_eq!(
        response.state.local_session_evidence_export.decision_count,
        0
    );
    assert!(
        response
            .state
            .local_session_evidence_export
            .absence_markers
            .provider_execution_absent
    );
}

#[test]
fn phase_139_rejected_provider_configuration_preserves_prior_accepted_state() {
    let mut transport = LocalOperatorShellTransport::new();
    let accepted = submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate::deterministic_stub(),
    );
    assert_eq!(accepted.status, LocalOperatorShellTransportStatus::Accepted);

    let rejected = submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate {
            provider_kind: Some("deterministic_stub".to_string()),
            fields: vec![("endpoint".to_string(), "http://localhost:11434".to_string())],
        },
    );
    assert_eq!(rejected.status, LocalOperatorShellTransportStatus::Rejected);
    assert_eq!(
        rejected.state.provider_configuration,
        accepted.state.provider_configuration
    );
    assert_eq!(
        transport.current_state().provider_configuration,
        accepted.state.provider_configuration
    );
}

#[test]
fn phase_139_provider_validation_is_deterministic_and_non_mutating() {
    let candidate = LocalProviderConfigurationCandidate {
        provider_kind: Some("deterministic_stub".to_string()),
        fields: vec![("provider_execution_enabled".to_string(), "true".to_string())],
    };
    let state = initial_local_operator_shell_state();
    let first = validate_local_provider_configuration(&candidate);
    let second = validate_local_provider_configuration(&candidate);
    assert_eq!(first, second);
    assert_eq!(state, initial_local_operator_shell_state());
}

#[test]
fn phase_139_provider_kind_edge_cases_fail_closed() {
    for candidate in [
        LocalProviderConfigurationCandidate {
            provider_kind: None,
            fields: Vec::new(),
        },
        LocalProviderConfigurationCandidate {
            provider_kind: Some("".to_string()),
            fields: Vec::new(),
        },
        LocalProviderConfigurationCandidate {
            provider_kind: Some("   ".to_string()),
            fields: Vec::new(),
        },
        LocalProviderConfigurationCandidate {
            provider_kind: Some("Deterministic_Stub".to_string()),
            fields: Vec::new(),
        },
        LocalProviderConfigurationCandidate {
            provider_kind: Some("mystery".to_string()),
            fields: Vec::new(),
        },
        LocalProviderConfigurationCandidate {
            provider_kind: Some("local_model".to_string()),
            fields: Vec::new(),
        },
        LocalProviderConfigurationCandidate {
            provider_kind: Some("cloud_model".to_string()),
            fields: Vec::new(),
        },
        LocalProviderConfigurationCandidate {
            provider_kind: Some("external_http".to_string()),
            fields: Vec::new(),
        },
        LocalProviderConfigurationCandidate {
            provider_kind: Some("shell_command".to_string()),
            fields: Vec::new(),
        },
        LocalProviderConfigurationCandidate {
            provider_kind: Some("filesystem_provider".to_string()),
            fields: Vec::new(),
        },
        LocalProviderConfigurationCandidate {
            provider_kind: Some("unknown".to_string()),
            fields: Vec::new(),
        },
    ] {
        let validation = validate_local_provider_configuration(&candidate);
        assert_ne!(
            validation.status,
            LocalProviderConfigurationStatus::Accepted
        );
    }
}

#[test]
fn phase_139_forbidden_provider_configuration_fields_fail_closed() {
    let cases = [
        (
            "endpoint",
            "http://localhost",
            LocalProviderConfigurationError::ForbiddenEndpointField,
        ),
        (
            "url",
            "https://example.invalid",
            LocalProviderConfigurationError::ForbiddenEndpointField,
        ),
        (
            "host",
            "localhost",
            LocalProviderConfigurationError::ForbiddenEndpointField,
        ),
        (
            "port",
            "11434",
            LocalProviderConfigurationError::ForbiddenEndpointField,
        ),
        (
            "command",
            "ollama run",
            LocalProviderConfigurationError::ForbiddenCommandField,
        ),
        (
            "args",
            "--serve",
            LocalProviderConfigurationError::ForbiddenCommandField,
        ),
        (
            "shell",
            "bash",
            LocalProviderConfigurationError::ForbiddenCommandField,
        ),
        (
            "process",
            "provider",
            LocalProviderConfigurationError::ForbiddenCommandField,
        ),
        (
            "path",
            "/tmp/model",
            LocalProviderConfigurationError::ForbiddenPathField,
        ),
        (
            "file",
            "model.bin",
            LocalProviderConfigurationError::ForbiddenPathField,
        ),
        (
            "directory",
            "/models",
            LocalProviderConfigurationError::ForbiddenPathField,
        ),
        (
            "secret",
            "value",
            LocalProviderConfigurationError::ForbiddenSecretField,
        ),
        (
            "token",
            "value",
            LocalProviderConfigurationError::ForbiddenSecretField,
        ),
        (
            "api_key",
            "value",
            LocalProviderConfigurationError::ForbiddenSecretField,
        ),
        (
            "credential",
            "value",
            LocalProviderConfigurationError::ForbiddenSecretField,
        ),
        (
            "provider_execution_enabled",
            "true",
            LocalProviderConfigurationError::ProviderExecutionRejected,
        ),
        (
            "trust_granted",
            "true",
            LocalProviderConfigurationError::TrustGrantRejected,
        ),
        (
            "readiness_approved",
            "true",
            LocalProviderConfigurationError::ReadinessClaimRejected,
        ),
        (
            "release_candidate_approved",
            "true",
            LocalProviderConfigurationError::ReleaseClaimRejected,
        ),
        (
            "deployment_enabled",
            "true",
            LocalProviderConfigurationError::DeploymentClaimRejected,
        ),
        (
            "public_use_approved",
            "true",
            LocalProviderConfigurationError::PublicUseClaimRejected,
        ),
        (
            "signing_enabled",
            "true",
            LocalProviderConfigurationError::SigningClaimRejected,
        ),
        (
            "publishing_enabled",
            "true",
            LocalProviderConfigurationError::PublishingClaimRejected,
        ),
        (
            "surprise",
            "field",
            LocalProviderConfigurationError::UnknownFieldRejected,
        ),
    ];
    for (key, value, expected_error) in cases {
        let validation =
            validate_local_provider_configuration(&LocalProviderConfigurationCandidate {
                provider_kind: Some("deterministic_stub".to_string()),
                fields: vec![(key.to_string(), value.to_string())],
            });
        assert_eq!(
            validation.status,
            LocalProviderConfigurationStatus::Rejected
        );
        assert!(
            validation.error_codes.contains(&expected_error),
            "missing {expected_error:?} for {key}"
        );
    }
}

#[test]
fn phase_153_initial_adapter_registry_projection_is_deterministic() {
    let state = initial_local_operator_shell_state();
    let first = project_local_provider_adapter_registry(&state.local_provider_adapter_registry);
    let second = project_local_provider_adapter_registry(&state.local_provider_adapter_registry);

    assert_eq!(first, second);
    assert_eq!(
        first.registry_status,
        LocalProviderAdapterValidationStatus::RegistryProjected
    );
    assert!(first
        .supported_adapter_kinds
        .contains(&"deterministic_fake_adapter".to_string()));
    assert!(first
        .supported_adapter_kinds
        .contains(&"local_model_adapter_contract".to_string()));
    assert!(first
        .rejected_adapter_kinds
        .contains(&"unsupported_cloud_model".to_string()));
    assert!(first.capability_surface.contract_only);
    assert!(first.capability_surface.no_execution);
    assert!(first.capability_surface.no_provider_trust);
    assert!(first.capability_surface.no_network);
    assert!(first.capability_surface.no_shell);
    assert!(first.capability_surface.no_secrets);
    assert!(first
        .boundary_statuses
        .contains(&"contract_only".to_string()));
    assert!(first
        .boundary_statuses
        .contains(&"no_execution".to_string()));
    assert_eq!(
        first.execution_status,
        "execution_not_available_in_phase_153"
    );
    assert_eq!(first.trust_status, "no_provider_trust");
}

#[test]
fn phase_153_accepts_non_executing_adapter_declarations() {
    for candidate in [
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
        LocalProviderAdapterConfigurationCandidate::local_model_adapter_contract(),
    ] {
        let validation = validate_local_provider_adapter_declaration(&candidate);
        assert_eq!(
            validation.status,
            LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting
        );
        let mut transport = LocalOperatorShellTransport::new();
        let before = transport.current_state();
        let response = submit_local_provider_adapter_declaration(&mut transport, candidate);
        assert_eq!(response.status, LocalOperatorShellTransportStatus::Accepted);
        assert_eq!(
            response.reason,
            "local_provider_adapter_declaration_accepted"
        );
        let projection = project_local_provider_adapter_registry(
            &response.state.local_provider_adapter_registry,
        );
        assert_eq!(projection.declarations.len(), 1);
        assert_eq!(
            projection.declarations[0].status,
            LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting
        );
        assert_eq!(
            projection.declarations[0].contract.execution_status,
            LocalProviderAdapterExecutionStatus::ExecutionNotAvailableInPhase153
        );
        assert_eq!(
            projection.declarations[0].contract.trust_status,
            LocalProviderAdapterTrustStatus::NoProviderTrust
        );
        assert!(projection.declarations[0]
            .contract
            .boundary_statuses
            .contains(&LocalProviderAdapterBoundaryStatus::ContractOnly));
        assert!(projection.declarations[0]
            .contract
            .boundary_statuses
            .contains(&LocalProviderAdapterBoundaryStatus::NoExecution));
        assert_eq!(response.state.run, before.run);
        assert_eq!(response.state.provider_execution, before.provider_execution);
        assert_eq!(
            response.state.provider_output_validation,
            before.provider_output_validation
        );
        assert_eq!(
            response.state.staged_candidate_conversion_proposal,
            before.staged_candidate_conversion_proposal
        );
        assert_eq!(
            response.state.staged_candidate_conversion_validation,
            before.staged_candidate_conversion_validation
        );
        assert_eq!(
            response.state.operator_candidate_decision,
            before.operator_candidate_decision
        );
        assert_eq!(
            response.state.local_session_package_projection,
            before.local_session_package_projection
        );
        assert_eq!(
            response.state.local_session_history_projection,
            before.local_session_history_projection
        );
        assert_eq!(
            response.state.local_session_restore_projection,
            before.local_session_restore_projection
        );
    }
}

#[test]
fn phase_153_unsupported_and_unsafe_adapter_declarations_fail_closed() {
    let cases = [
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: None,
                declaration_id: None,
                fields: Vec::new(),
            },
            LocalProviderAdapterValidationError::MissingAdapterKind,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("unknown".to_string()),
                declaration_id: None,
                fields: Vec::new(),
            },
            LocalProviderAdapterValidationError::UnsupportedAdapter,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("mystery".to_string()),
                declaration_id: None,
                fields: Vec::new(),
            },
            LocalProviderAdapterValidationError::UnsupportedAdapter,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("unsupported_cloud_model".to_string()),
                declaration_id: None,
                fields: Vec::new(),
            },
            LocalProviderAdapterValidationError::CloudOrNetworkAdapterRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("unsupported_network_adapter".to_string()),
                declaration_id: None,
                fields: Vec::new(),
            },
            LocalProviderAdapterValidationError::CloudOrNetworkAdapterRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("unsupported_shell_adapter".to_string()),
                declaration_id: None,
                fields: Vec::new(),
            },
            LocalProviderAdapterValidationError::ShellAdapterRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("unsupported_filesystem_adapter".to_string()),
                declaration_id: None,
                fields: Vec::new(),
            },
            LocalProviderAdapterValidationError::FilesystemAdapterRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("executable_path".to_string(), "/bin/model".to_string())],
            },
            LocalProviderAdapterValidationError::ExecutablePathRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("endpoint".to_string(), "http://localhost".to_string())],
            },
            LocalProviderAdapterValidationError::EndpointFieldRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("command".to_string(), "run model".to_string())],
            },
            LocalProviderAdapterValidationError::CommandFieldRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("path".to_string(), "/tmp/model".to_string())],
            },
            LocalProviderAdapterValidationError::PathFieldRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("api_key".to_string(), "secret".to_string())],
            },
            LocalProviderAdapterValidationError::SecretFieldRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("execution_requested".to_string(), "true".to_string())],
            },
            LocalProviderAdapterValidationError::ExecutionFlagRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("provider_trust_claimed".to_string(), "true".to_string())],
            },
            LocalProviderAdapterValidationError::ProviderTrustFlagRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("readiness_claim".to_string(), "true".to_string())],
            },
            LocalProviderAdapterValidationError::ReadinessClaimRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("release_claim".to_string(), "true".to_string())],
            },
            LocalProviderAdapterValidationError::ReleaseClaimRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("deployment_claim".to_string(), "true".to_string())],
            },
            LocalProviderAdapterValidationError::DeploymentClaimRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("public_use_claim".to_string(), "true".to_string())],
            },
            LocalProviderAdapterValidationError::PublicUseClaimRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("signing_claim".to_string(), "true".to_string())],
            },
            LocalProviderAdapterValidationError::SigningClaimRejected,
        ),
        (
            LocalProviderAdapterConfigurationCandidate {
                adapter_kind: Some("deterministic_fake_adapter".to_string()),
                declaration_id: None,
                fields: vec![("publishing_claim".to_string(), "true".to_string())],
            },
            LocalProviderAdapterValidationError::PublishingClaimRejected,
        ),
    ];

    for (candidate, expected_error) in cases {
        let validation = validate_local_provider_adapter_declaration(&candidate);
        assert_ne!(
            validation.status,
            LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting
        );
        assert!(
            validation.error_codes.contains(&expected_error),
            "missing {expected_error:?} in {:?}",
            validation.error_codes
        );
    }
}

#[test]
fn phase_153_rejected_adapter_declaration_preserves_prior_registry() {
    let mut transport = LocalOperatorShellTransport::new();
    let accepted = submit_local_provider_adapter_declaration(
        &mut transport,
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
    );
    assert_eq!(accepted.status, LocalOperatorShellTransportStatus::Accepted);
    let rejected = submit_local_provider_adapter_declaration(
        &mut transport,
        LocalProviderAdapterConfigurationCandidate {
            adapter_kind: Some("deterministic_fake_adapter".to_string()),
            declaration_id: Some("unsafe-adapter".to_string()),
            fields: vec![("token".to_string(), "secret".to_string())],
        },
    );
    assert_eq!(rejected.status, LocalOperatorShellTransportStatus::Rejected);
    assert_eq!(
        rejected.state.local_provider_adapter_registry,
        accepted.state.local_provider_adapter_registry
    );
    assert_eq!(
        transport.current_state().local_provider_adapter_registry,
        accepted.state.local_provider_adapter_registry
    );
}

#[test]
fn phase_153_adapter_validation_is_deterministic() {
    let candidate = LocalProviderAdapterConfigurationCandidate {
        adapter_kind: Some("deterministic_fake_adapter".to_string()),
        declaration_id: Some("deterministic".to_string()),
        fields: vec![("command".to_string(), "run model".to_string())],
    };
    let first = validate_local_provider_adapter_declaration(&candidate);
    let second = validate_local_provider_adapter_declaration(&candidate);
    assert_eq!(first, second);
}

#[test]
fn phase_139_duplicate_deterministic_stub_submission_is_stable() {
    let mut transport = LocalOperatorShellTransport::new();
    let first = submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate::deterministic_stub(),
    );
    let second = submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate::deterministic_stub(),
    );
    assert_eq!(first.status, LocalOperatorShellTransportStatus::Accepted);
    assert_eq!(second.status, LocalOperatorShellTransportStatus::Accepted);
    assert_eq!(
        first.state.provider_configuration,
        second.state.provider_configuration
    );
    assert_eq!(second.state.decision_ledger.records.len(), 0);
}

#[test]
fn phase_141_initial_execution_projection_is_not_executed() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.provider_execution.status,
        LocalProviderExecutionStatus::NotExecuted
    );
    assert_eq!(
        state.provider_execution.sandbox_status,
        LocalProviderExecutionSandboxStatus::NotEntered
    );
    assert_eq!(state.provider_execution.configured_provider_kind, "none");
    assert!(state.provider_execution.result.is_none());
    assert!(
        state
            .provider_execution
            .capability_surface
            .deterministic_stub_execution_supported
    );
    assert_eq!(
        state
            .provider_execution
            .capability_surface
            .supported_provider_kind,
        "deterministic_stub"
    );
}

#[test]
fn phase_141_executes_accepted_deterministic_stub_inside_sandbox() {
    let mut transport = LocalOperatorShellTransport::new();
    submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate::deterministic_stub(),
    );
    let before = transport.current_state();
    let response = execute_local_provider(
        &mut transport,
        LocalProviderExecutionRequest::deterministic_stub("local deterministic provider input"),
    );

    assert_eq!(response.status, LocalOperatorShellTransportStatus::Accepted);
    assert_eq!(response.reason, "local_provider_execution_accepted");
    assert_eq!(
        response.state.provider_execution.status,
        LocalProviderExecutionStatus::Executed
    );
    let result = response.state.provider_execution.result.as_ref().unwrap();
    assert_eq!(result.provider_kind, LocalProviderKind::DeterministicStub);
    assert_eq!(
        result.sandbox_status,
        LocalProviderExecutionSandboxStatus::SandboxedDeterministicNoExternalEffects
    );
    assert_eq!(
        result.output_trust_status,
        LocalProviderOutputTrustStatus::UntrustedDescriptive
    );
    assert!(result.descriptive_only);
    assert!(!result.provider_output_trusted);
    assert!(!result.candidate_output_promoted);
    assert!(!result.decision_appended);
    assert!(!result.replay_repaired);
    assert!(!result.release_or_deployment_evidence_created);
    assert_eq!(response.state.decision_ledger, before.decision_ledger);
    assert_eq!(
        response.state.run.decision_replay,
        before.run.decision_replay
    );
    assert_eq!(
        response.state.local_session_evidence_export,
        before.local_session_evidence_export
    );
}

#[test]
fn phase_141_deterministic_stub_execution_repeats_identically() {
    let mut transport = LocalOperatorShellTransport::new();
    submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate::deterministic_stub(),
    );
    let first = execute_local_provider(
        &mut transport,
        LocalProviderExecutionRequest::deterministic_stub("same deterministic input"),
    );
    let second = execute_local_provider(
        &mut transport,
        LocalProviderExecutionRequest::deterministic_stub("same deterministic input"),
    );

    assert_eq!(first.status, LocalOperatorShellTransportStatus::Accepted);
    assert_eq!(second.status, LocalOperatorShellTransportStatus::Accepted);
    assert_eq!(
        first.state.provider_execution,
        second.state.provider_execution
    );
    assert_eq!(second.state.decision_ledger.records.len(), 0);
}

#[test]
fn phase_142_projection_validation_linkage_and_non_promotion_are_explicit() {
    let mut transport = LocalOperatorShellTransport::new();
    submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate::deterministic_stub(),
    );
    let before = transport.current_state();
    let first = execute_local_provider(
        &mut transport,
        LocalProviderExecutionRequest::deterministic_stub("phase 142 projection input"),
    );
    let second = execute_local_provider(
        &mut transport,
        LocalProviderExecutionRequest::deterministic_stub("phase 142 projection input"),
    );

    let projection = project_local_provider_execution(&first.state);
    assert_eq!(first.status, LocalOperatorShellTransportStatus::Accepted);
    assert_eq!(
        projection.projection_status,
        LocalProviderExecutionResultProjectionStatus::ExecutionProjected
    );
    assert_eq!(
        projection.output_trust_status,
        LocalProviderOutputTrustStatus::UntrustedDescriptive
    );
    assert_eq!(
        projection.output_materialization_status,
        LocalProviderOutputMaterializationStatus::NotCandidateMaterial
    );
    assert_eq!(
        projection.output_promotion_status,
        LocalProviderOutputPromotionStatus::NotPromoted
    );
    assert_eq!(
        projection.promotion_availability_status,
        LocalProviderOutputPromotionStatus::PromotionNotAvailableInPhase142
    );
    assert_eq!(
        projection.projection_validation.status,
        LocalProviderExecutionResultProjectionValidationStatus::Valid
    );
    assert_eq!(projection.linkage.run_id, "local-stub-run-133");
    assert_eq!(
        projection.linkage.provider_configuration_kind,
        "deterministic_stub"
    );
    assert_eq!(projection.linkage.provider_configuration_status, "accepted");
    assert_eq!(
        projection.linkage.source_boundary,
        "sandboxed_deterministic_provider_execution"
    );
    assert!(projection.absence_markers.no_process_spawned);
    assert!(projection.absence_markers.no_network_socket_opened);
    assert!(projection.absence_markers.no_filesystem_persistence);
    assert!(
        projection
            .absence_markers
            .provider_output_not_candidate_material
    );
    assert_eq!(
        first.state.provider_execution,
        second.state.provider_execution
    );
    assert_eq!(first.state.decision_ledger, before.decision_ledger);
    assert_eq!(first.state.run.candidate, before.run.candidate);
}

#[test]
fn phase_142_projection_validation_fails_closed_for_invalid_boundaries() {
    let mut projection = initial_local_provider_execution_projection();
    assert_eq!(
        validate_local_provider_execution_result_projection(&projection).status,
        LocalProviderExecutionResultProjectionValidationStatus::Valid
    );

    projection.output_trust_status = LocalProviderOutputTrustStatus::TrustedOutput;
    assert!(
        validate_local_provider_execution_result_projection(&projection)
            .error_codes
            .contains(&"invalid_trust_status".to_string())
    );
    projection.output_trust_status = LocalProviderOutputTrustStatus::UntrustedDescriptive;

    projection.output_materialization_status =
        LocalProviderOutputMaterializationStatus::CandidateMaterial;
    assert!(
        validate_local_provider_execution_result_projection(&projection)
            .error_codes
            .contains(&"invalid_materialization_status".to_string())
    );
    projection.output_materialization_status =
        LocalProviderOutputMaterializationStatus::NotCandidateMaterial;

    projection.output_promotion_status = LocalProviderOutputPromotionStatus::Promoted;
    assert!(
        validate_local_provider_execution_result_projection(&projection)
            .error_codes
            .contains(&"invalid_promotion_status".to_string())
    );
    projection.output_promotion_status = LocalProviderOutputPromotionStatus::NotPromoted;

    projection.absence_markers.no_process_spawned = false;
    let first = validate_local_provider_execution_result_projection(&projection);
    let second = validate_local_provider_execution_result_projection(&projection);
    assert_eq!(first, second);
    assert!(first
        .error_codes
        .contains(&"missing_absence_marker".to_string()));
}

#[test]
fn phase_141_execution_requires_accepted_deterministic_stub_configuration() {
    let mut transport = LocalOperatorShellTransport::new();
    let missing = execute_local_provider(
        &mut transport,
        LocalProviderExecutionRequest::deterministic_stub("input"),
    );
    assert_eq!(missing.status, LocalOperatorShellTransportStatus::Rejected);
    assert_eq!(
        missing.state.provider_execution.status,
        LocalProviderExecutionStatus::NotExecuted
    );

    let rejected_config = submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate {
            provider_kind: Some("cloud_model".to_string()),
            fields: Vec::new(),
        },
    );
    assert_eq!(
        rejected_config.status,
        LocalOperatorShellTransportStatus::Rejected
    );
    let after_rejected = execute_local_provider(
        &mut transport,
        LocalProviderExecutionRequest::deterministic_stub("input"),
    );
    assert_eq!(
        after_rejected.status,
        LocalOperatorShellTransportStatus::Rejected
    );
    assert_eq!(
        transport.current_state().provider_execution.status,
        LocalProviderExecutionStatus::NotExecuted
    );
}

#[test]
fn phase_141_unsupported_provider_execution_fails_closed() {
    let mut transport = LocalOperatorShellTransport::new();
    submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate::deterministic_stub(),
    );
    for provider_kind in [
        None,
        Some(""),
        Some(" deterministic_stub"),
        Some("Deterministic_Stub"),
        Some("local_model"),
        Some("cloud_model"),
        Some("external_http"),
        Some("shell_command"),
        Some("filesystem_provider"),
        Some("unknown"),
        Some("surprise_provider"),
    ] {
        let before = transport.current_state();
        let response = execute_local_provider(
            &mut transport,
            LocalProviderExecutionRequest {
                provider_kind: provider_kind.map(str::to_string),
                input_summary: "input".to_string(),
                fields: Vec::new(),
            },
        );
        assert_eq!(response.status, LocalOperatorShellTransportStatus::Rejected);
        assert_eq!(response.state, before);
        assert_eq!(transport.current_state(), before);
    }
}

#[test]
fn phase_141_forbidden_execution_fields_fail_closed_and_preserve_state() {
    let mut transport = LocalOperatorShellTransport::new();
    submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate::deterministic_stub(),
    );
    let accepted = execute_local_provider(
        &mut transport,
        LocalProviderExecutionRequest::deterministic_stub("safe input"),
    );
    assert_eq!(accepted.status, LocalOperatorShellTransportStatus::Accepted);

    let cases = [
        (
            "endpoint",
            "http://localhost",
            LocalProviderExecutionError::ForbiddenEndpointField,
        ),
        (
            "url",
            "https://example.invalid",
            LocalProviderExecutionError::ForbiddenEndpointField,
        ),
        (
            "host",
            "localhost",
            LocalProviderExecutionError::ForbiddenEndpointField,
        ),
        (
            "port",
            "11434",
            LocalProviderExecutionError::ForbiddenEndpointField,
        ),
        (
            "command",
            "ollama run",
            LocalProviderExecutionError::ForbiddenCommandField,
        ),
        (
            "args",
            "--serve",
            LocalProviderExecutionError::ForbiddenCommandField,
        ),
        (
            "shell",
            "bash",
            LocalProviderExecutionError::ForbiddenCommandField,
        ),
        (
            "process",
            "provider",
            LocalProviderExecutionError::ForbiddenCommandField,
        ),
        (
            "path",
            "/tmp/model",
            LocalProviderExecutionError::ForbiddenPathField,
        ),
        (
            "file",
            "model.bin",
            LocalProviderExecutionError::ForbiddenPathField,
        ),
        (
            "directory",
            "/models",
            LocalProviderExecutionError::ForbiddenPathField,
        ),
        (
            "secret",
            "value",
            LocalProviderExecutionError::ForbiddenSecretField,
        ),
        (
            "token",
            "value",
            LocalProviderExecutionError::ForbiddenSecretField,
        ),
        (
            "api_key",
            "value",
            LocalProviderExecutionError::ForbiddenSecretField,
        ),
        (
            "credential",
            "value",
            LocalProviderExecutionError::ForbiddenSecretField,
        ),
        (
            "provider_execution_enabled",
            "true",
            LocalProviderExecutionError::ProviderExecutionFlagRejected,
        ),
        (
            "trust_granted",
            "true",
            LocalProviderExecutionError::TrustGrantRejected,
        ),
        (
            "readiness_approved",
            "true",
            LocalProviderExecutionError::ReadinessClaimRejected,
        ),
        (
            "release_candidate_approved",
            "true",
            LocalProviderExecutionError::ReleaseClaimRejected,
        ),
        (
            "deployment_enabled",
            "true",
            LocalProviderExecutionError::DeploymentClaimRejected,
        ),
        (
            "public_use_approved",
            "true",
            LocalProviderExecutionError::PublicUseClaimRejected,
        ),
        (
            "signing_enabled",
            "true",
            LocalProviderExecutionError::SigningClaimRejected,
        ),
        (
            "publishing_enabled",
            "true",
            LocalProviderExecutionError::PublishingClaimRejected,
        ),
        (
            "extra",
            "field",
            LocalProviderExecutionError::UnknownFieldRejected,
        ),
    ];

    for (key, value, expected_error) in cases {
        let before = transport.current_state();
        let request = LocalProviderExecutionRequest {
            provider_kind: Some("deterministic_stub".to_string()),
            input_summary: "safe input".to_string(),
            fields: vec![(key.to_string(), value.to_string())],
        };
        let validation =
            validate_local_provider_execution_request(&before.provider_configuration, &request);
        assert!(
            validation.error_codes.contains(&expected_error),
            "missing {expected_error:?} for {key}"
        );
        let response = execute_local_provider(&mut transport, request);
        assert_eq!(response.status, LocalOperatorShellTransportStatus::Rejected);
        assert_eq!(response.state, before);
        assert_eq!(transport.current_state(), before);
    }
}

fn phase_143_reviewable_provider_output_state() -> LocalOperatorShellState {
    let mut transport = LocalOperatorShellTransport::new();
    submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate::deterministic_stub(),
    );
    execute_local_provider(
        &mut transport,
        LocalProviderExecutionRequest::deterministic_stub("phase 143 validation input"),
    )
    .state
}

#[test]
fn phase_143_initial_provider_output_validation_is_not_validated() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.provider_output_validation.status,
        LocalProviderOutputValidationStatus::NotValidated
    );
    assert_eq!(
        state.provider_output_validation.reviewability_status,
        LocalProviderOutputReviewabilityStatus::NotReviewable
    );
    assert!(state
        .provider_output_validation
        .reasons
        .contains(&LocalProviderOutputValidationReason::NoProviderExecutionResult));
    assert!(validate_local_provider_output_validation_projection(
        &state.provider_output_validation
    )
    .is_ok());
}

#[test]
fn phase_143_valid_deterministic_stub_output_is_reviewable_untrusted_only() {
    let state = phase_143_reviewable_provider_output_state();
    let projection = project_local_provider_output_validation(&state);
    assert_eq!(
        projection.status,
        LocalProviderOutputValidationStatus::ReviewableUntrusted
    );
    assert_eq!(
        projection.reviewability_status,
        LocalProviderOutputReviewabilityStatus::ReviewableUntrusted
    );
    assert_eq!(
        projection.candidate_boundary_status,
        LocalProviderOutputCandidateBoundaryStatus::NotCandidateMaterial
    );
    assert!(projection
        .candidate_boundary_statuses
        .contains(&LocalProviderOutputCandidateBoundaryStatus::CandidateConversionNotPerformed));
    assert!(projection.candidate_boundary_statuses.contains(
        &LocalProviderOutputCandidateBoundaryStatus::CandidateConversionRequiresFuturePhase
    ));
    assert!(projection
        .reasons
        .contains(&LocalProviderOutputValidationReason::DeterministicStubOutputShapeValid));
    assert!(projection
        .reasons
        .contains(&LocalProviderOutputValidationReason::CandidateConversionNotAvailableInPhase143));
    assert_eq!(
        projection.output_trust_status,
        LocalProviderOutputTrustStatus::UntrustedDescriptive
    );
    assert_eq!(
        projection.output_promotion_status,
        LocalProviderOutputPromotionStatus::NotPromoted
    );
    assert_eq!(
        projection.trust_effect,
        LocalProviderOutputValidationEffect::None
    );
    assert_eq!(
        projection.candidate_effect,
        LocalProviderOutputValidationEffect::None
    );
    assert_eq!(
        projection.decision_ledger_effect,
        LocalProviderOutputValidationEffect::None
    );
    assert_eq!(
        projection.replay_effect,
        LocalProviderOutputValidationEffect::None
    );
    assert_eq!(
        projection.export_effect,
        LocalProviderOutputValidationEffect::None
    );
    assert_eq!(
        projection.action_effect,
        LocalProviderOutputValidationEffect::None
    );
    assert_eq!(
        projection.readiness_effect,
        LocalProviderOutputValidationEffect::None
    );
    assert_eq!(
        projection.release_effect,
        LocalProviderOutputValidationEffect::None
    );
    assert_eq!(
        projection.deployment_effect,
        LocalProviderOutputValidationEffect::None
    );
    assert!(validate_local_provider_output_validation_projection(&projection).is_ok());
}

#[test]
fn phase_143_rejects_malformed_and_claim_bearing_provider_output() {
    let state = phase_143_reviewable_provider_output_state();
    let cases = [
        ("", LocalProviderOutputValidationReason::EmptyOutput),
        (
            "not deterministic",
            LocalProviderOutputValidationReason::MalformedOutput,
        ),
        (
            &"x".repeat(1025),
            LocalProviderOutputValidationReason::OutputTooLarge,
        ),
        (
            "deterministic_stub descriptive output secret token",
            LocalProviderOutputValidationReason::ContainsForbiddenSecretMarker,
        ),
        (
            "deterministic_stub descriptive output execute shell command",
            LocalProviderOutputValidationReason::ContainsExecutionInstruction,
        ),
        (
            "deterministic_stub descriptive output https://example.invalid",
            LocalProviderOutputValidationReason::ContainsNetworkInstruction,
        ),
        (
            "deterministic_stub descriptive output write filesystem path",
            LocalProviderOutputValidationReason::ContainsFilesystemInstruction,
        ),
        (
            "deterministic_stub descriptive output release readiness production ready",
            LocalProviderOutputValidationReason::ContainsReadinessOrReleaseClaim,
        ),
        (
            "deterministic_stub descriptive output trusted_output approval granted",
            LocalProviderOutputValidationReason::ContainsTrustOrApprovalClaim,
        ),
        (
            "deterministic_stub descriptive output authorize action",
            LocalProviderOutputValidationReason::ContainsActionInstruction,
        ),
    ];
    for (output, reason) in cases {
        let mut execution = state.provider_execution.clone();
        execution
            .result
            .as_mut()
            .expect("provider result")
            .output_summary = output.to_string();
        let projection = validate_local_provider_output(&execution);
        assert_eq!(
            projection.status,
            LocalProviderOutputValidationStatus::Rejected,
            "{output}"
        );
        assert_eq!(
            projection.reviewability_status,
            LocalProviderOutputReviewabilityStatus::RejectedBeforeReview
        );
        assert!(
            projection.reasons.contains(&reason),
            "missing {reason:?} for {output}"
        );
        assert_eq!(
            projection.candidate_effect,
            LocalProviderOutputValidationEffect::None
        );
    }
}

#[test]
fn phase_143_validation_is_deterministic_and_non_promoting() {
    let state = phase_143_reviewable_provider_output_state();
    let before_ledger = state.decision_ledger.clone();
    let before_replay = state.run.decision_replay.clone();
    let before_export = state.local_session_evidence_export.clone();
    let first = project_local_provider_output_validation(&state);
    let second = project_local_provider_output_validation(&state);
    assert_eq!(first, second);
    assert_eq!(state.decision_ledger, before_ledger);
    assert_eq!(state.run.decision_replay, before_replay);
    assert_eq!(state.local_session_evidence_export, before_export);
    assert_eq!(
        state
            .run
            .candidate
            .as_ref()
            .map(|candidate| candidate.candidate_id.as_str()),
        None
    );
}

#[test]
fn phase_143_validation_projection_fails_closed_on_boundary_drift() {
    let state = phase_143_reviewable_provider_output_state();
    let mut projection = project_local_provider_output_validation(&state);
    projection.output_trust_status = LocalProviderOutputTrustStatus::TrustedOutput;
    assert_eq!(
        validate_local_provider_output_validation_projection(&projection),
        Err(LocalProviderOutputValidationError::InvalidReviewableTrustStatus)
    );

    projection = project_local_provider_output_validation(&state);
    projection.candidate_boundary_statuses =
        vec![LocalProviderOutputCandidateBoundaryStatus::NotCandidateMaterial];
    assert_eq!(
        validate_local_provider_output_validation_projection(&projection),
        Err(LocalProviderOutputValidationError::InvalidCandidateBoundaryStatus)
    );

    projection = project_local_provider_output_validation(&state);
    projection.output_promotion_status = LocalProviderOutputPromotionStatus::Promoted;
    assert_eq!(
        validate_local_provider_output_validation_projection(&projection),
        Err(LocalProviderOutputValidationError::InvalidPromotionStatus)
    );

    for drift in 0..9 {
        projection = project_local_provider_output_validation(&state);
        match drift {
            0 => projection.trust_effect = LocalProviderOutputValidationEffect::EffectDetected,
            1 => projection.candidate_effect = LocalProviderOutputValidationEffect::EffectDetected,
            2 => {
                projection.decision_ledger_effect =
                    LocalProviderOutputValidationEffect::EffectDetected
            }
            3 => projection.replay_effect = LocalProviderOutputValidationEffect::EffectDetected,
            4 => projection.export_effect = LocalProviderOutputValidationEffect::EffectDetected,
            5 => projection.action_effect = LocalProviderOutputValidationEffect::EffectDetected,
            6 => projection.readiness_effect = LocalProviderOutputValidationEffect::EffectDetected,
            7 => projection.release_effect = LocalProviderOutputValidationEffect::EffectDetected,
            _ => projection.deployment_effect = LocalProviderOutputValidationEffect::EffectDetected,
        }
        assert_eq!(
            validate_local_provider_output_validation_projection(&projection),
            Err(LocalProviderOutputValidationError::InvalidNoEffectBoundary)
        );
    }
}

#[test]
fn phase_146_initial_state_exposes_no_staged_candidate_conversion_proposal() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.staged_candidate_conversion_proposal.status,
        StagedCandidateConversionProposalStatus::NoProposal
    );
    assert!(state
        .staged_candidate_conversion_proposal
        .proposal
        .is_none());
    assert_eq!(state.decision_ledger.records.len(), 0);
}

#[test]
fn phase_146_creates_staged_proposal_from_reviewable_untrusted_output_only() {
    let state = phase_143_reviewable_provider_output_state();
    let before_candidate = state.run.candidate.clone();
    let before_ledger = state.decision_ledger.clone();
    let before_replay = state.run.decision_replay.clone();
    let before_export = state.local_session_evidence_export.clone();
    let before_configuration = state.provider_configuration.clone();
    let before_execution = state.provider_execution.clone();

    let next = create_staged_candidate_conversion_proposal(
        &state,
        StagedCandidateConversionProposalRequest::staging_only("phase 146 local staging"),
    )
    .expect("reviewable_untrusted source can create staged proposal");
    let projection = &next.staged_candidate_conversion_proposal;
    let proposal = projection.proposal.as_ref().unwrap();

    assert_eq!(
        projection.status,
        StagedCandidateConversionProposalStatus::StagedProposalCreated
    );
    assert_eq!(
        proposal.source_validation_status,
        LocalProviderOutputValidationStatus::ReviewableUntrusted
    );
    assert_eq!(
        proposal.source_reviewability_status,
        LocalProviderOutputReviewabilityStatus::ReviewableUntrusted
    );
    assert_eq!(
        proposal.source_candidate_boundary_status,
        LocalProviderOutputCandidateBoundaryStatus::NotCandidateMaterial
    );
    assert_eq!(
        proposal.source_boundary,
        "provider_output_validation_phase_143"
    );
    assert_eq!(
        proposal.proposal_boundary,
        "staged_candidate_conversion_phase_146"
    );
    assert!(proposal
        .boundary_statuses
        .contains(&StagedCandidateConversionBoundaryStatus::StagingOnlyNotCandidateMaterial));
    assert!(proposal
        .boundary_statuses
        .contains(&StagedCandidateConversionBoundaryStatus::CandidateConversionNotPerformed));
    assert!(proposal
        .boundary_statuses
        .contains(&StagedCandidateConversionBoundaryStatus::ValidationRequiredInFuturePhase));
    assert!(proposal
        .boundary_statuses
        .contains(&StagedCandidateConversionBoundaryStatus::ApprovalNotAvailableInPhase146));
    assert!(proposal
        .trust_statuses
        .contains(&StagedCandidateConversionTrustStatus::UntrustedSource));
    assert!(proposal
        .trust_statuses
        .contains(&StagedCandidateConversionTrustStatus::NotTrusted));
    assert!(proposal
        .trust_statuses
        .contains(&StagedCandidateConversionTrustStatus::NotApproved));
    for effect in staged_candidate_conversion_no_effects() {
        assert!(proposal.effect_statuses.contains(&effect));
    }
    assert!(validate_staged_candidate_conversion_proposal(projection).is_ok());
    assert_eq!(next.run.candidate, before_candidate);
    assert_eq!(next.decision_ledger, before_ledger);
    assert_eq!(next.run.decision_replay, before_replay);
    assert_eq!(next.local_session_evidence_export, before_export);
    assert_eq!(next.provider_configuration, before_configuration);
    assert_eq!(next.provider_execution, before_execution);
}

#[test]
fn phase_146_proposal_identity_and_linkage_are_deterministic() {
    let state = phase_143_reviewable_provider_output_state();
    let first = create_staged_candidate_conversion_proposal(
        &state,
        StagedCandidateConversionProposalRequest::staging_only("same source"),
    )
    .unwrap();
    let second = create_staged_candidate_conversion_proposal(
        &state,
        StagedCandidateConversionProposalRequest::staging_only("same source"),
    )
    .unwrap();
    let first_proposal = first.staged_candidate_conversion_proposal.proposal.unwrap();
    let second_proposal = second
        .staged_candidate_conversion_proposal
        .proposal
        .unwrap();
    assert_eq!(first_proposal.proposal_id, second_proposal.proposal_id);
    assert_eq!(
        first_proposal.source_execution_result_id,
        second_proposal.source_execution_result_id
    );
    assert_eq!(first_proposal.source_provider_kind, "deterministic_stub");
}

#[test]
fn phase_146_rejects_ineligible_sources_and_preserves_transport_state() {
    let mut transport = LocalOperatorShellTransport::new();
    let missing = create_local_staged_candidate_conversion_proposal(
        &mut transport,
        StagedCandidateConversionProposalRequest::staging_only("missing source"),
    );
    assert_eq!(missing.status, LocalOperatorShellTransportStatus::Rejected);
    assert_eq!(missing.reason, "missing_provider_execution_result");
    assert_eq!(
        missing.state.staged_candidate_conversion_proposal.status,
        StagedCandidateConversionProposalStatus::NoProposal
    );

    submit_local_provider_configuration(
        &mut transport,
        LocalProviderConfigurationCandidate::deterministic_stub(),
    );
    let accepted = execute_local_provider(
        &mut transport,
        LocalProviderExecutionRequest::deterministic_stub("phase 146 rejected source"),
    );
    let mut rejected_state = accepted.state.clone();
    rejected_state
        .provider_execution
        .result
        .as_mut()
        .unwrap()
        .output_summary = "deterministic_stub descriptive output authorize action".to_string();
    rejected_state.provider_output_validation =
        validate_local_provider_output(&rejected_state.provider_execution);
    assert_eq!(
        rejected_state.provider_output_validation.status,
        LocalProviderOutputValidationStatus::Rejected
    );
    assert_eq!(
        create_staged_candidate_conversion_proposal(
            &rejected_state,
            StagedCandidateConversionProposalRequest::staging_only("rejected source"),
        ),
        Err(StagedCandidateConversionProposalError::RejectedSourceNotEligible)
    );
}

#[test]
fn phase_146_rejects_not_applicable_invalid_and_inconsistent_validation() {
    let mut state = phase_143_reviewable_provider_output_state();
    state.provider_output_validation.status = LocalProviderOutputValidationStatus::NotValidated;
    assert_eq!(
        create_staged_candidate_conversion_proposal(
            &state,
            StagedCandidateConversionProposalRequest::staging_only("not validated"),
        ),
        Err(StagedCandidateConversionProposalError::MissingOrInconsistentValidationProjection)
    );

    state = phase_143_reviewable_provider_output_state();
    state.provider_output_validation.status =
        LocalProviderOutputValidationStatus::ValidationNotApplicable;
    state.provider_output_validation.reviewability_status =
        LocalProviderOutputReviewabilityStatus::NotReviewable;
    assert_eq!(
        create_staged_candidate_conversion_proposal(
            &state,
            StagedCandidateConversionProposalRequest::staging_only("not applicable"),
        ),
        Err(StagedCandidateConversionProposalError::MissingOrInconsistentValidationProjection)
    );

    state = phase_143_reviewable_provider_output_state();
    state.provider_output_validation.status =
        LocalProviderOutputValidationStatus::InvalidValidationInput;
    assert_eq!(
        create_staged_candidate_conversion_proposal(
            &state,
            StagedCandidateConversionProposalRequest::staging_only("invalid input"),
        ),
        Err(StagedCandidateConversionProposalError::MissingOrInconsistentValidationProjection)
    );

    state = phase_143_reviewable_provider_output_state();
    state.provider_output_validation.reasons.clear();
    assert_eq!(
        create_staged_candidate_conversion_proposal(
            &state,
            StagedCandidateConversionProposalRequest::staging_only("inconsistent"),
        ),
        Err(StagedCandidateConversionProposalError::MissingOrInconsistentValidationProjection)
    );
}

#[test]
fn phase_146_request_claims_fail_closed() {
    let state = phase_143_reviewable_provider_output_state();
    for (key, value) in [
        ("trust", "true"),
        ("approval", "true"),
        ("safe", "true"),
        ("readiness", "true"),
        ("release", "true"),
        ("deployment", "true"),
        ("public_use", "true"),
        ("action", "run"),
        ("execution", "run"),
        ("persistence", "true"),
        ("candidate_creation", "true"),
    ] {
        let request = StagedCandidateConversionProposalRequest {
            operator_note: "forbidden shortcut".to_string(),
            claims: vec![(key.to_string(), value.to_string())],
        };
        assert_eq!(
            create_staged_candidate_conversion_proposal(&state, request),
            Err(StagedCandidateConversionProposalError::InvalidProposalRequest),
            "{key} should fail closed"
        );
    }
}

#[test]
fn phase_146_projection_validation_rejects_boundary_drift() {
    let state = phase_143_reviewable_provider_output_state();
    let next = create_staged_candidate_conversion_proposal(
        &state,
        StagedCandidateConversionProposalRequest::staging_only("valid"),
    )
    .unwrap();
    let mut projection = next.staged_candidate_conversion_proposal.clone();
    assert!(validate_staged_candidate_conversion_proposal(&projection).is_ok());

    projection.proposal.as_mut().unwrap().boundary_statuses =
        vec![StagedCandidateConversionBoundaryStatus::StagingOnlyNotCandidateMaterial];
    assert_eq!(
        validate_staged_candidate_conversion_proposal(&projection),
        Err(StagedCandidateConversionProposalError::InvalidProposalBoundary)
    );

    projection = next.staged_candidate_conversion_proposal.clone();
    projection.proposal.as_mut().unwrap().trust_statuses =
        vec![StagedCandidateConversionTrustStatus::UntrustedSource];
    assert_eq!(
        validate_staged_candidate_conversion_proposal(&projection),
        Err(StagedCandidateConversionProposalError::InvalidProposalBoundary)
    );

    projection = next.staged_candidate_conversion_proposal.clone();
    projection.proposal.as_mut().unwrap().effect_statuses =
        vec![StagedCandidateConversionEffectStatus::NoDecisionLedgerEffect];
    assert_eq!(
        validate_staged_candidate_conversion_proposal(&projection),
        Err(StagedCandidateConversionProposalError::InvalidProposalBoundary)
    );
}

fn phase_147_validated_state() -> LocalOperatorShellState {
    let state = phase_143_reviewable_provider_output_state();
    create_staged_candidate_conversion_proposal(
        &state,
        StagedCandidateConversionProposalRequest::staging_only("phase 147 validation"),
    )
    .unwrap()
}

fn phase_149_validated_decision_state() -> LocalOperatorShellState {
    let state = phase_147_validated_state();
    validate_staged_candidate_conversion_proposal_for_phase_147(
        &state,
        StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    )
}

#[test]
fn phase_147_initial_validation_projection_is_not_validated() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.staged_candidate_conversion_validation.status,
        StagedCandidateConversionValidationStatus::NotValidated
    );
    assert!(state
        .staged_candidate_conversion_validation
        .reasons
        .is_empty());
    assert!(state
        .staged_candidate_conversion_validation
        .materialization_statuses
        .contains(
            &StagedCandidateConversionMaterializationStatus::MaterializationNotAvailableInPhase147
        ));
}

#[test]
fn phase_147_valid_staged_proposal_validates_shape_and_linkage_only() {
    let state = phase_147_validated_state();
    let before = state.clone();
    let response = validate_staged_candidate_conversion_proposal_for_phase_147(
        &state,
        StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    let validation = response.staged_candidate_conversion_validation;
    assert_eq!(
        validation.status,
        StagedCandidateConversionValidationStatus::StagedProposalShapeValid
    );
    assert!(validation
        .reasons
        .contains(&StagedCandidateConversionValidationReason::SourceLinkageValidated));
    assert!(validation.reasons.contains(
        &StagedCandidateConversionValidationReason::CandidateMaterializationNotPerformed
    ));
    assert!(validation
        .reasons
        .contains(&StagedCandidateConversionValidationReason::FutureReviewBoundaryRequired));
    assert!(validation.reasons.contains(
        &StagedCandidateConversionValidationReason::OperatorDecisionNotAvailableInPhase147
    ));
    assert!(validation
        .trust_statuses
        .contains(&StagedCandidateConversionTrustStatus::UntrustedSource));
    assert!(validation
        .trust_statuses
        .contains(&StagedCandidateConversionTrustStatus::NotTrusted));
    assert!(validation
        .trust_statuses
        .contains(&StagedCandidateConversionTrustStatus::NotApproved));
    assert_eq!(response.run.candidate, before.run.candidate);
    assert_eq!(response.decision_ledger, before.decision_ledger);
    assert_eq!(response.run.decision_replay, before.run.decision_replay);
    assert_eq!(
        response.local_session_evidence_export,
        before.local_session_evidence_export
    );
    assert_eq!(
        response.provider_configuration,
        before.provider_configuration
    );
    assert_eq!(response.provider_execution, before.provider_execution);
}

#[test]
fn phase_147_validation_transport_is_deterministic_and_rejects_missing_proposal() {
    let mut transport = LocalOperatorShellTransport::new();
    let missing = validate_local_staged_candidate_conversion_proposal(
        &mut transport,
        StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    assert_eq!(missing.status, LocalOperatorShellTransportStatus::Rejected);
    assert_eq!(
        missing.state.staged_candidate_conversion_validation.status,
        StagedCandidateConversionValidationStatus::InvalidValidationInput
    );
    assert!(missing
        .state
        .staged_candidate_conversion_validation
        .reasons
        .contains(&StagedCandidateConversionValidationReason::NoStagedProposal));
    assert_eq!(
        transport
            .current_state()
            .staged_candidate_conversion_validation
            .status,
        StagedCandidateConversionValidationStatus::NotValidated
    );

    let state = phase_147_validated_state();
    let first = project_staged_candidate_conversion_validation(
        &state,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    let second = project_staged_candidate_conversion_validation(
        &state,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    assert_eq!(first, second);
}

#[test]
fn phase_147_validation_rejects_linkage_and_source_drift() {
    let state = phase_147_validated_state();
    let mut drifted = state.clone();
    drifted
        .staged_candidate_conversion_proposal
        .proposal
        .as_mut()
        .unwrap()
        .proposal_id = "wrong-proposal-id".to_string();
    let projection = project_staged_candidate_conversion_validation(
        &drifted,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    assert_eq!(
        projection.status,
        StagedCandidateConversionValidationStatus::RejectedStagedProposal
    );
    assert!(projection
        .reasons
        .contains(&StagedCandidateConversionValidationReason::DeterministicProposalIdMismatch));

    drifted = state.clone();
    drifted
        .provider_execution
        .result
        .as_mut()
        .unwrap()
        .result_id = "wrong-result".to_string();
    let projection = project_staged_candidate_conversion_validation(
        &drifted,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    assert!(projection
        .reasons
        .contains(&StagedCandidateConversionValidationReason::ExecutionResultIdMismatch));

    drifted = state.clone();
    drifted
        .staged_candidate_conversion_proposal
        .proposal
        .as_mut()
        .unwrap()
        .source_validation_status = LocalProviderOutputValidationStatus::Rejected;
    let projection = project_staged_candidate_conversion_validation(
        &drifted,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    assert!(projection
        .reasons
        .contains(&StagedCandidateConversionValidationReason::SourceValidationStatusMismatch));

    drifted = state.clone();
    drifted
        .staged_candidate_conversion_proposal
        .proposal
        .as_mut()
        .unwrap()
        .source_reviewability_status = LocalProviderOutputReviewabilityStatus::NotReviewable;
    let projection = project_staged_candidate_conversion_validation(
        &drifted,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    assert!(projection
        .reasons
        .contains(&StagedCandidateConversionValidationReason::SourceReviewabilityStatusMismatch));

    drifted = state.clone();
    drifted
        .staged_candidate_conversion_proposal
        .proposal
        .as_mut()
        .unwrap()
        .source_candidate_boundary_status =
        LocalProviderOutputCandidateBoundaryStatus::CandidateConversionNotPerformed;
    let projection = project_staged_candidate_conversion_validation(
        &drifted,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    assert!(projection.reasons.contains(
        &StagedCandidateConversionValidationReason::SourceCandidateBoundaryStatusMismatch
    ));
}

#[test]
fn phase_147_validation_rejects_missing_inconsistent_malformed_and_boundary_drift() {
    let state = phase_147_validated_state();
    let mut drifted = state.clone();
    drifted.provider_execution.result = None;
    assert!(project_staged_candidate_conversion_validation(
        &drifted,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    )
    .reasons
    .contains(&StagedCandidateConversionValidationReason::ProviderExecutionResultMissing));

    drifted = state.clone();
    drifted.provider_execution.projection_status =
        LocalProviderExecutionResultProjectionStatus::ExecutionRejected;
    assert!(project_staged_candidate_conversion_validation(
        &drifted,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    )
    .reasons
    .contains(&StagedCandidateConversionValidationReason::ProviderExecutionResultMalformed));

    drifted = state.clone();
    drifted.provider_output_validation.reasons.clear();
    assert!(project_staged_candidate_conversion_validation(
        &drifted,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    )
    .reasons
    .contains(&StagedCandidateConversionValidationReason::ProviderOutputValidationMissing));

    drifted = state.clone();
    drifted.provider_output_validation.output_promotion_status =
        LocalProviderOutputPromotionStatus::Promoted;
    assert!(project_staged_candidate_conversion_validation(
        &drifted,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    )
    .reasons
    .contains(&StagedCandidateConversionValidationReason::ProviderOutputValidationInconsistent));

    drifted = state.clone();
    drifted
        .staged_candidate_conversion_proposal
        .proposal
        .as_mut()
        .unwrap()
        .boundary_statuses
        .pop();
    let projection = project_staged_candidate_conversion_validation(
        &drifted,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    assert!(projection
        .reasons
        .contains(&StagedCandidateConversionValidationReason::BoundaryFlagMissing));
    assert!(projection
        .reasons
        .contains(&StagedCandidateConversionValidationReason::BoundaryFlagDrift));
    assert!(projection
        .reasons
        .contains(&StagedCandidateConversionValidationReason::FuturePhaseMarkerMissing));

    drifted = state.clone();
    drifted
        .staged_candidate_conversion_proposal
        .proposal
        .as_mut()
        .unwrap()
        .effect_statuses
        .pop();
    let projection = project_staged_candidate_conversion_validation(
        &drifted,
        &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    assert!(projection
        .reasons
        .contains(&StagedCandidateConversionValidationReason::NoEffectFieldMissing));
    assert!(projection
        .reasons
        .contains(&StagedCandidateConversionValidationReason::NoEffectFieldDrift));
}

#[test]
fn phase_147_validation_rejects_claim_bearing_proposals() {
    let base = phase_147_validated_state();
    let cases = [
        (
            "trust granted",
            StagedCandidateConversionValidationReason::ContainsTrustClaim,
        ),
        (
            "approval granted",
            StagedCandidateConversionValidationReason::ContainsApprovalClaim,
        ),
        (
            "is safe",
            StagedCandidateConversionValidationReason::ContainsSafetyClaim,
        ),
        (
            "readiness",
            StagedCandidateConversionValidationReason::ContainsReadinessClaim,
        ),
        (
            "release claim",
            StagedCandidateConversionValidationReason::ContainsReleaseClaim,
        ),
        (
            "deployment claim",
            StagedCandidateConversionValidationReason::ContainsDeploymentClaim,
        ),
        (
            "public use",
            StagedCandidateConversionValidationReason::ContainsPublicUseClaim,
        ),
        (
            "action claim",
            StagedCandidateConversionValidationReason::ContainsActionClaim,
        ),
        (
            "persistence claim",
            StagedCandidateConversionValidationReason::ContainsPersistenceClaim,
        ),
        (
            "execution claim",
            StagedCandidateConversionValidationReason::ContainsExecutionClaim,
        ),
        (
            "candidate creation",
            StagedCandidateConversionValidationReason::ContainsCandidateCreationClaim,
        ),
        (
            "candidate materialization",
            StagedCandidateConversionValidationReason::ContainsCandidateMaterializationClaim,
        ),
    ];
    for (claim, reason) in cases {
        let mut state = base.clone();
        state
            .staged_candidate_conversion_proposal
            .proposal
            .as_mut()
            .unwrap()
            .note = claim.to_string();
        let projection = project_staged_candidate_conversion_validation(
            &state,
            &StagedCandidateConversionValidationRequest::existing_staged_proposal(),
        );
        assert_eq!(
            projection.status,
            StagedCandidateConversionValidationStatus::RejectedStagedProposal
        );
        assert!(projection.reasons.contains(&reason), "missing {reason:?}");
    }
}

#[test]
fn phase_149_initial_decision_and_handoff_are_projected() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.operator_candidate_decision.status,
        OperatorCandidateDecisionStatus::NoOperatorDecision
    );
    assert!(state
        .phase_150_code_production_handoff
        .implemented_capability_evidence
        .iter()
        .any(|item| item.contains("operator decision boundary: no_operator_decision")));
    assert!(state
        .phase_150_code_production_handoff
        .remaining_production_grade_gaps
        .contains(&"local session persistence".to_string()));
}

#[test]
fn phase_149_accepts_approve_and_reject_for_validated_staged_proposal() {
    let state = phase_149_validated_decision_state();
    let proposal = state
        .staged_candidate_conversion_proposal
        .proposal
        .as_ref()
        .unwrap();
    for (request, expected) in [
        (
            OperatorCandidateDecisionRequest::approve(
                &proposal.proposal_id,
                &proposal.source_execution_result_id,
            ),
            OperatorCandidateDecisionStatus::ApprovedValidatedStagedProposal,
        ),
        (
            OperatorCandidateDecisionRequest::reject(
                &proposal.proposal_id,
                &proposal.source_execution_result_id,
            ),
            OperatorCandidateDecisionStatus::RejectedValidatedStagedProposal,
        ),
    ] {
        let next = submit_operator_candidate_decision(&state, request).unwrap();
        let record = next.operator_candidate_decision.record.as_ref().unwrap();
        assert_eq!(next.operator_candidate_decision.status, expected);
        assert_eq!(
            record.decision_scope,
            "decision_scope_validated_staged_proposal_only"
        );
        assert_eq!(
            record.materialization_status,
            "candidate_materialization_not_performed"
        );
        assert_eq!(record.trust_status, "provider_output_remains_untrusted");
        assert_eq!(record.readiness_status, "no_readiness_effect");
        assert_eq!(record.release_status, "no_release_effect");
        assert_eq!(record.deployment_status, "no_deployment_effect");
        assert_eq!(record.public_use_status, "no_public_use_effect");
        assert_eq!(record.action_status, "no_action_effect");
        assert_eq!(record.persistence_status, "no_persistence_effect");
        assert_eq!(record.replay_repair_status, "no_replay_repair_effect");
        assert_eq!(
            record.recovery_promotion_status,
            "no_recovery_promotion_effect"
        );
        assert_eq!(next.run.candidate, state.run.candidate);
        assert_eq!(next.provider_configuration, state.provider_configuration);
        assert_eq!(next.provider_execution, state.provider_execution);
        assert_eq!(
            next.provider_output_validation,
            state.provider_output_validation
        );
        assert_eq!(
            next.staged_candidate_conversion_proposal,
            state.staged_candidate_conversion_proposal
        );
        assert_eq!(
            next.staged_candidate_conversion_validation,
            state.staged_candidate_conversion_validation
        );
        assert_eq!(
            next.decision_ledger.records.len(),
            state.decision_ledger.records.len()
        );
    }
}

#[test]
fn phase_149_rejects_invalid_decision_preconditions_and_claims() {
    let state = phase_149_validated_decision_state();
    let proposal = state
        .staged_candidate_conversion_proposal
        .proposal
        .as_ref()
        .unwrap();
    assert_eq!(
        validate_operator_candidate_decision_request(
            &initial_local_operator_shell_state(),
            &OperatorCandidateDecisionRequest::approve("missing", "missing")
        ),
        Err(OperatorCandidateDecisionError::NoStagedProposal)
    );
    let mut not_validated = state.clone();
    not_validated.staged_candidate_conversion_validation =
        initial_staged_candidate_conversion_validation_projection();
    assert_eq!(
        validate_operator_candidate_decision_request(
            &not_validated,
            &OperatorCandidateDecisionRequest::approve(
                &proposal.proposal_id,
                &proposal.source_execution_result_id
            )
        ),
        Err(OperatorCandidateDecisionError::StagedProposalNotValidated)
    );
    let mut rejected = state.clone();
    rejected.staged_candidate_conversion_validation.status =
        StagedCandidateConversionValidationStatus::RejectedStagedProposal;
    assert_eq!(
        validate_operator_candidate_decision_request(
            &rejected,
            &OperatorCandidateDecisionRequest::approve(
                &proposal.proposal_id,
                &proposal.source_execution_result_id
            )
        ),
        Err(OperatorCandidateDecisionError::StagedProposalValidationRejected)
    );
    let mut invalid = state.clone();
    invalid.staged_candidate_conversion_validation.status =
        StagedCandidateConversionValidationStatus::InvalidValidationInput;
    assert_eq!(
        validate_operator_candidate_decision_request(
            &invalid,
            &OperatorCandidateDecisionRequest::approve(
                &proposal.proposal_id,
                &proposal.source_execution_result_id
            )
        ),
        Err(OperatorCandidateDecisionError::InvalidValidationInput)
    );
    assert_eq!(
        validate_operator_candidate_decision_request(
            &state,
            &OperatorCandidateDecisionRequest::approve(
                "drift",
                &proposal.source_execution_result_id
            )
        ),
        Err(OperatorCandidateDecisionError::SourceLinkageInconsistent)
    );
    let claim_cases = [
        ("trust", OperatorCandidateDecisionError::TrustClaimRejected),
        (
            "provider",
            OperatorCandidateDecisionError::ProviderOutputApprovalClaimRejected,
        ),
        (
            "readiness",
            OperatorCandidateDecisionError::ReadinessClaimRejected,
        ),
        (
            "release",
            OperatorCandidateDecisionError::ReleaseClaimRejected,
        ),
        (
            "deployment",
            OperatorCandidateDecisionError::DeploymentClaimRejected,
        ),
        (
            "public",
            OperatorCandidateDecisionError::PublicUseClaimRejected,
        ),
        (
            "action",
            OperatorCandidateDecisionError::ActionClaimRejected,
        ),
        (
            "execution",
            OperatorCandidateDecisionError::ExecutionClaimRejected,
        ),
        (
            "persistence",
            OperatorCandidateDecisionError::PersistenceClaimRejected,
        ),
        (
            "creation",
            OperatorCandidateDecisionError::CandidateCreationClaimRejected,
        ),
        (
            "materialization",
            OperatorCandidateDecisionError::CandidateMaterializationClaimRejected,
        ),
    ];
    for (claim, expected) in claim_cases {
        let mut request = OperatorCandidateDecisionRequest::approve(
            &proposal.proposal_id,
            &proposal.source_execution_result_id,
        );
        match claim {
            "trust" => request.claims_trust = true,
            "provider" => request.claims_provider_output_approval = true,
            "readiness" => request.claims_readiness = true,
            "release" => request.claims_release = true,
            "deployment" => request.claims_deployment = true,
            "public" => request.claims_public_use = true,
            "action" => request.claims_action = true,
            "execution" => request.claims_execution = true,
            "persistence" => request.claims_persistence = true,
            "creation" => request.claims_candidate_creation = true,
            "materialization" => request.claims_candidate_materialization = true,
            _ => unreachable!(),
        }
        assert_eq!(
            validate_operator_candidate_decision_request(&state, &request),
            Err(expected)
        );
    }
}

#[test]
fn phase_149_decision_and_handoff_are_deterministic_and_non_mutating() {
    let state = phase_149_validated_decision_state();
    let proposal = state
        .staged_candidate_conversion_proposal
        .proposal
        .as_ref()
        .unwrap();
    let request = OperatorCandidateDecisionRequest::approve(
        &proposal.proposal_id,
        &proposal.source_execution_result_id,
    );
    let first = project_operator_candidate_decision(&request);
    let second = project_operator_candidate_decision(&request);
    assert_eq!(first, second);
    let handoff_first = derive_phase_150_code_production_handoff(&state);
    let handoff_second = derive_phase_150_code_production_handoff(&state);
    assert_eq!(handoff_first, handoff_second);
    assert_eq!(state, phase_149_validated_decision_state());
    assert!(handoff_first
        .implemented_capability_evidence
        .iter()
        .any(|item| item.contains("operator decision boundary")));
    assert!(handoff_first
        .remaining_production_grade_gaps
        .contains(&"candidate materialization".to_string()));
}

#[test]
fn phase_154_initial_adapter_dry_run_projection_is_not_run() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.local_provider_adapter_dry_run.status,
        LocalProviderAdapterDryRunStatus::NotRun
    );
    assert!(state
        .local_provider_adapter_dry_run
        .boundary_statuses
        .contains(&LocalProviderAdapterDryRunBoundaryStatus::ControlledDryRunOnly));
    assert!(state
        .local_provider_adapter_dry_run
        .boundary_statuses
        .contains(&LocalProviderAdapterDryRunBoundaryStatus::NoRealModelExecution));
    assert_eq!(
        state.local_provider_adapter_dry_run.output_trust_status,
        LocalProviderAdapterDryRunTrustStatus::UntrustedDescriptive
    );
}

#[test]
fn phase_154_accepted_deterministic_fake_adapter_dry_run_is_deterministic() {
    let state = apply_local_provider_adapter_declaration(
        &initial_local_operator_shell_state(),
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
    )
    .unwrap();
    let request = LocalProviderAdapterDryRunRequest::deterministic_default();
    let first = apply_local_provider_adapter_dry_run(&state, request.clone()).unwrap();
    let second = apply_local_provider_adapter_dry_run(&state, request).unwrap();
    let first_result = first
        .local_provider_adapter_dry_run
        .result
        .as_ref()
        .unwrap();
    let second_result = second
        .local_provider_adapter_dry_run
        .result
        .as_ref()
        .unwrap();
    assert_eq!(
        first.local_provider_adapter_dry_run.status,
        LocalProviderAdapterDryRunStatus::DryRunExecuted
    );
    assert_eq!(first_result.result_id, second_result.result_id);
    assert_eq!(first_result.output_summary, second_result.output_summary);
    assert_eq!(
        first_result.output_trust_status,
        LocalProviderAdapterDryRunTrustStatus::UntrustedDescriptive
    );
    assert!(first
        .local_provider_adapter_dry_run
        .boundary_statuses
        .contains(&LocalProviderAdapterDryRunBoundaryStatus::DeterministicFakeAdapterOnly));
    assert!(first
        .local_provider_adapter_dry_run
        .boundary_statuses
        .contains(&LocalProviderAdapterDryRunBoundaryStatus::NoProcessSpawn));
    assert!(first
        .local_provider_adapter_dry_run
        .boundary_statuses
        .contains(&LocalProviderAdapterDryRunBoundaryStatus::NoNetwork));
    assert!(first
        .local_provider_adapter_dry_run
        .boundary_statuses
        .contains(&LocalProviderAdapterDryRunBoundaryStatus::NoShell));
    assert!(first
        .local_provider_adapter_dry_run
        .boundary_statuses
        .contains(&LocalProviderAdapterDryRunBoundaryStatus::NoSecrets));
    assert!(first
        .local_provider_adapter_dry_run
        .effect_statuses
        .contains(&LocalProviderAdapterDryRunEffectStatus::NoCandidateMaterialization));
    assert!(first
        .local_provider_adapter_dry_run
        .effect_statuses
        .contains(&LocalProviderAdapterDryRunEffectStatus::NoActionExecution));
}

#[test]
fn phase_154_adapter_dry_run_rejects_preconditions_and_forbidden_fields() {
    let initial = initial_local_operator_shell_state();
    let missing = apply_local_provider_adapter_dry_run(
        &initial,
        LocalProviderAdapterDryRunRequest::deterministic_default(),
    )
    .unwrap_err();
    assert_eq!(
        missing.status,
        LocalProviderAdapterDryRunStatus::AdapterRequired
    );
    assert_eq!(
        missing.error_codes,
        vec![LocalProviderAdapterDryRunError::NoAdapterDeclared]
    );

    let local_model = apply_local_provider_adapter_declaration(
        &initial,
        LocalProviderAdapterConfigurationCandidate::local_model_adapter_contract(),
    )
    .unwrap();
    let rejected = apply_local_provider_adapter_dry_run(
        &local_model,
        LocalProviderAdapterDryRunRequest::deterministic_default(),
    )
    .unwrap_err();
    assert!(rejected
        .error_codes
        .contains(&LocalProviderAdapterDryRunError::LocalModelAdapterNotExecutableInPhase154));

    let accepted = apply_local_provider_adapter_declaration(
        &initial,
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
    )
    .unwrap();
    let cases = [
        (
            ("endpoint", "https://example.invalid"),
            LocalProviderAdapterDryRunError::EndpointFieldRejected,
        ),
        (
            ("command", "run process"),
            LocalProviderAdapterDryRunError::CommandFieldRejected,
        ),
        (
            ("model_path", "/tmp/model"),
            LocalProviderAdapterDryRunError::PathFieldRejected,
        ),
        (
            ("api_key", "secret-token"),
            LocalProviderAdapterDryRunError::SecretFieldRejected,
        ),
        (
            ("execution", "true"),
            LocalProviderAdapterDryRunError::ExecutionClaimRejected,
        ),
        (
            ("trust", "true"),
            LocalProviderAdapterDryRunError::TrustClaimRejected,
        ),
        (
            ("readiness", "true"),
            LocalProviderAdapterDryRunError::ReadinessClaimRejected,
        ),
        (
            ("release", "true"),
            LocalProviderAdapterDryRunError::ReleaseClaimRejected,
        ),
        (
            ("deployment", "true"),
            LocalProviderAdapterDryRunError::DeploymentClaimRejected,
        ),
        (
            ("public_use", "true"),
            LocalProviderAdapterDryRunError::PublicUseClaimRejected,
        ),
        (
            ("signing", "true"),
            LocalProviderAdapterDryRunError::SigningClaimRejected,
        ),
        (
            ("publishing", "true"),
            LocalProviderAdapterDryRunError::PublishingClaimRejected,
        ),
        (
            ("action", "true"),
            LocalProviderAdapterDryRunError::ActionClaimRejected,
        ),
        (
            ("persistence", "true"),
            LocalProviderAdapterDryRunError::PersistenceClaimRejected,
        ),
    ];
    for ((key, value), expected) in cases {
        let projection = apply_local_provider_adapter_dry_run(
            &accepted,
            LocalProviderAdapterDryRunRequest {
                input_summary: "deterministic dry run".to_string(),
                fields: vec![(key.to_string(), value.to_string())],
            },
        )
        .unwrap_err();
        assert!(
            projection.error_codes.contains(&expected),
            "missing {expected:?}"
        );
    }
}

#[test]
fn phase_154_adapter_dry_run_has_no_decision_replay_or_candidate_effects() {
    let state = apply_local_provider_adapter_declaration(
        &initial_local_operator_shell_state(),
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
    )
    .unwrap();
    let before_ledger = state.decision_ledger.clone();
    let before_replay = state.run.decision_replay.clone();
    let before_run_candidate = state.run.candidate.clone();
    let before_export_status = state.local_session_evidence_export.export_status;
    let after = apply_local_provider_adapter_dry_run(
        &state,
        LocalProviderAdapterDryRunRequest::deterministic_default(),
    )
    .unwrap();
    assert_eq!(after.decision_ledger, before_ledger);
    assert_eq!(after.run.decision_replay, before_replay);
    assert_eq!(after.run.candidate, before_run_candidate);
    assert_eq!(
        after.local_session_evidence_export.export_status,
        before_export_status
    );
    assert_eq!(
        after.local_session_package_projection,
        state.local_session_package_projection
    );
    assert_eq!(
        after.local_session_restore_projection,
        state.local_session_restore_projection
    );
}

fn phase_157_invoked_state() -> LocalOperatorShellState {
    let configured = apply_local_provider_adapter_declaration(
        &initial_local_operator_shell_state(),
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
    )
    .unwrap();
    execute_constrained_local_provider_invocation(
        &configured,
        ConstrainedLocalProviderInvocationRequest::allowlisted_default(),
    )
    .unwrap()
}

#[test]
fn phase_157_initial_and_valid_invocation_pipeline_projection() {
    let initial = initial_local_operator_shell_state();
    assert_eq!(
        initial.local_provider_output_pipeline.status,
        LocalProviderOutputPipelineValidationStatus::NotStarted
    );
    assert_eq!(
        initial.local_provider_output_pipeline.next_required_stage,
        Some(LocalProviderOutputPipelineStage::InvocationOutputProjected)
    );

    let invoked = phase_157_invoked_state();
    let pipeline = &invoked.local_provider_output_pipeline;
    assert_eq!(
        pipeline.source_kind,
        Some(LocalProviderOutputPipelineSourceKind::ConstrainedLocalProviderInvocation)
    );
    assert_eq!(
        pipeline.source_invocation_result_id,
        invoked
            .constrained_local_provider_invocation
            .result
            .as_ref()
            .map(|result| result.result_id.clone())
    );
    assert_eq!(
        pipeline.provider_output_validation_status,
        LocalProviderOutputValidationStatus::ReviewableUntrusted
    );
    assert_eq!(
        pipeline.provider_output_review_status,
        LocalProviderOutputReviewabilityStatus::ReviewableUntrusted
    );
    assert_eq!(
        pipeline.next_required_stage,
        Some(LocalProviderOutputPipelineStage::StagedProposalProjected)
    );
    assert!(pipeline
        .boundary_statuses
        .contains(&LocalProviderOutputPipelineBoundaryStatus::UntrustedDescriptiveSource));
    assert!(pipeline
        .boundary_statuses
        .contains(&LocalProviderOutputPipelineBoundaryStatus::NoCandidateMaterialization));
    assert!(pipeline
        .effect_statuses
        .contains(&LocalProviderOutputPipelineEffectStatus::NoProviderExecution));
    assert!(
        !invoked
            .provider_execution
            .result
            .as_ref()
            .unwrap()
            .provider_output_trusted
    );
    assert!(
        !invoked
            .provider_execution
            .result
            .as_ref()
            .unwrap()
            .candidate_output_promoted
    );
    validate_provider_output_pipeline_stage_order(pipeline).unwrap();
}

#[test]
fn phase_157_pipeline_rejects_missing_rejected_and_drifted_invocation_output() {
    let missing = initial_local_operator_shell_state();
    assert_eq!(
        project_invocation_output_into_provider_pipeline(&missing).unwrap_err(),
        vec![LocalProviderOutputPipelineError::NoInvocationOutput]
    );

    let mut rejected = initial_local_operator_shell_state();
    rejected.constrained_local_provider_invocation = reject_constrained_local_provider_invocation(
        ConstrainedLocalProviderInvocationStatus::InvocationRejected,
        None,
        None,
        None,
        0,
        vec![ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted],
    );
    let projection = derive_local_provider_output_pipeline_projection(&rejected);
    assert_eq!(
        projection.status,
        LocalProviderOutputPipelineValidationStatus::Rejected
    );
    assert!(projection
        .errors
        .contains(&LocalProviderOutputPipelineError::InvocationOutputRejected));

    let mut drifted = phase_157_invoked_state();
    drifted
        .constrained_local_provider_invocation
        .result
        .as_mut()
        .unwrap()
        .output_summary =
        "allowlisted_local_deterministic_provider descriptive output drift".to_string();
    assert!(project_invocation_output_into_provider_pipeline(&drifted)
        .unwrap_err()
        .contains(&LocalProviderOutputPipelineError::InvocationOutputSummaryMismatch));

    let mut claim = phase_157_invoked_state();
    claim
        .constrained_local_provider_invocation
        .result
        .as_mut()
        .unwrap()
        .output_summary
        .push_str(
            " trust release deployment public-use candidate materialization action persistence",
        );
    let errors = project_invocation_output_into_provider_pipeline(&claim).unwrap_err();
    for expected in [
        LocalProviderOutputPipelineError::TrustClaimRejected,
        LocalProviderOutputPipelineError::ApprovalClaimRejected,
        LocalProviderOutputPipelineError::ReleaseClaimRejected,
        LocalProviderOutputPipelineError::DeploymentClaimRejected,
        LocalProviderOutputPipelineError::PublicUseClaimRejected,
        LocalProviderOutputPipelineError::CandidateMaterializationClaimRejected,
        LocalProviderOutputPipelineError::ActionClaimRejected,
        LocalProviderOutputPipelineError::PersistenceClaimRejected,
    ] {
        assert!(errors.contains(&expected), "missing {expected:?}");
    }
}

#[test]
fn phase_157_pipeline_stage_order_no_skip_and_determinism() {
    let invoked = phase_157_invoked_state();
    let first = derive_local_provider_output_pipeline_projection(&invoked);
    let second = derive_local_provider_output_pipeline_projection(&invoked);
    assert_eq!(first, second);
    validate_provider_output_pipeline_stage_order(&first).unwrap();

    let mut skipped = first.clone();
    skipped.stages.swap(2, 3);
    assert_eq!(
        validate_provider_output_pipeline_stage_order(&skipped),
        Err(LocalProviderOutputPipelineError::PipelineSkipAttemptRejected)
    );

    let mut completed_after_block = first.clone();
    completed_after_block.stages[8].status = LocalProviderOutputPipelineStageStatus::Completed;
    assert_eq!(
        validate_provider_output_pipeline_stage_order(&completed_after_block),
        Err(LocalProviderOutputPipelineError::PipelineSkipAttemptRejected)
    );
}

#[test]
fn phase_157_pipeline_tracks_downstream_boundaries_without_effects() {
    let invoked = phase_157_invoked_state();
    let proposed = create_staged_candidate_conversion_proposal(
        &invoked,
        StagedCandidateConversionProposalRequest::staging_only("phase 157 pipeline staging"),
    )
    .unwrap();
    assert_eq!(
        proposed.local_provider_output_pipeline.next_required_stage,
        Some(LocalProviderOutputPipelineStage::StagedProposalValidationProjected)
    );
    let mut validated = validate_staged_candidate_conversion_proposal_for_phase_147(
        &proposed,
        StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    validated.local_provider_output_pipeline =
        derive_local_provider_output_pipeline_projection(&validated);
    let pipeline = validated.local_provider_output_pipeline.clone();
    assert_eq!(
        validated.staged_candidate_conversion_validation.status,
        StagedCandidateConversionValidationStatus::StagedProposalShapeValid
    );
    assert_eq!(
        pipeline.next_required_stage,
        Some(LocalProviderOutputPipelineStage::OperatorDecisionProjected)
    );
    assert_eq!(pipeline.candidate_review_status, "display_only");
    assert_eq!(validated.run.candidate, None);
    assert!(validated.decision_ledger.records.is_empty());
    assert!(pipeline
        .effect_statuses
        .contains(&LocalProviderOutputPipelineEffectStatus::NoDecisionLedgerAppend));
    assert!(pipeline
        .effect_statuses
        .contains(&LocalProviderOutputPipelineEffectStatus::NoFileWrite));
    assert!(pipeline
        .effect_statuses
        .contains(&LocalProviderOutputPipelineEffectStatus::NoNetworkSocket));
    assert!(pipeline
        .effect_statuses
        .contains(&LocalProviderOutputPipelineEffectStatus::NoProcessSpawn));
    assert!(pipeline
        .effect_statuses
        .contains(&LocalProviderOutputPipelineEffectStatus::NoSecretRead));
}

fn phase_158_approved_decision_state() -> LocalOperatorShellState {
    let invoked = phase_157_invoked_state();
    let proposed = create_staged_candidate_conversion_proposal(
        &invoked,
        StagedCandidateConversionProposalRequest::staging_only("phase 158 materialization"),
    )
    .unwrap();
    let validated = validate_staged_candidate_conversion_proposal_for_phase_147(
        &proposed,
        StagedCandidateConversionValidationRequest::existing_staged_proposal(),
    );
    let proposal = validated
        .staged_candidate_conversion_proposal
        .proposal
        .as_ref()
        .unwrap();
    submit_operator_candidate_decision(
        &validated,
        OperatorCandidateDecisionRequest::approve(
            &proposal.proposal_id,
            &proposal.source_execution_result_id,
        ),
    )
    .unwrap()
}

#[test]
fn phase_158_initial_and_valid_materialization_are_deterministic_and_linked() {
    let initial = initial_local_operator_shell_state();
    assert_eq!(
        initial.local_candidate_output.status,
        LocalCandidateMaterializationStatus::NotMaterialized
    );
    let state = phase_158_approved_decision_state();
    let request = LocalCandidateMaterializationRequest::from_validated_state(&state).unwrap();
    let first = project_local_candidate_output(&state, &request).unwrap();
    let second = project_local_candidate_output(&state, &request).unwrap();
    assert_eq!(first.candidate_id, second.candidate_id);
    assert_eq!(first.content_summary, second.content_summary);
    assert_eq!(
        first.status,
        LocalCandidateMaterializationStatus::LocalCandidateMaterialized
    );
    assert_eq!(first.output_classification, "local_candidate_output_only");
    assert_eq!(first.production_classification, "non_production_candidate");
    assert_eq!(
        first.provider_output_trust_carry_forward,
        "provider_output_remains_untrusted"
    );
    let linkage = first.source_linkage.as_ref().unwrap();
    assert_eq!(linkage.staged_proposal_id, request.staged_proposal_id);
    assert_eq!(linkage.operator_decision_id, request.operator_decision_id);
    assert_eq!(
        linkage.provider_output_validation_status,
        LocalProviderOutputValidationStatus::ReviewableUntrusted
    );
    assert_eq!(
        linkage.staged_proposal_validation_status,
        StagedCandidateConversionValidationStatus::StagedProposalShapeValid
    );
    for required in [
        LocalCandidateMaterializationBoundaryStatus::LocalCandidateOutputOnly,
        LocalCandidateMaterializationBoundaryStatus::NonProductionCandidate,
        LocalCandidateMaterializationBoundaryStatus::ProviderOutputRemainsUntrusted,
        LocalCandidateMaterializationBoundaryStatus::NoProviderTrust,
        LocalCandidateMaterializationBoundaryStatus::NoProductionApproval,
        LocalCandidateMaterializationBoundaryStatus::NoReleaseApproval,
        LocalCandidateMaterializationBoundaryStatus::NoDeploymentApproval,
        LocalCandidateMaterializationBoundaryStatus::NoPublicUseApproval,
        LocalCandidateMaterializationBoundaryStatus::NoActionExecution,
        LocalCandidateMaterializationBoundaryStatus::NoReplayRepair,
        LocalCandidateMaterializationBoundaryStatus::NoRecoveryPromotion,
    ] {
        assert!(
            first.boundary_statuses.contains(&required),
            "missing {required:?}"
        );
    }
}

#[test]
fn phase_158_materialization_rejects_preconditions_claims_and_drift() {
    let state = phase_158_approved_decision_state();
    let request = LocalCandidateMaterializationRequest::from_validated_state(&state).unwrap();
    let mut missing_decision = state.clone();
    missing_decision.operator_candidate_decision = initial_operator_candidate_decision_projection();
    assert_eq!(
        validate_local_candidate_materialization_request(&missing_decision, &request),
        Err(LocalCandidateMaterializationError::OperatorDecisionMissing)
    );
    let mut rejected_decision_state = state.clone();
    let proposal = rejected_decision_state
        .staged_candidate_conversion_proposal
        .proposal
        .as_ref()
        .unwrap();
    rejected_decision_state.operator_candidate_decision =
        project_operator_candidate_decision(&OperatorCandidateDecisionRequest::reject(
            &proposal.proposal_id,
            &proposal.source_execution_result_id,
        ));
    rejected_decision_state.local_provider_output_pipeline =
        derive_local_provider_output_pipeline_projection(&rejected_decision_state);
    assert_eq!(
        validate_local_candidate_materialization_request(&rejected_decision_state, &request),
        Err(LocalCandidateMaterializationError::OperatorDecisionRejected)
    );
    let mut validation_rejected = state.clone();
    validation_rejected.provider_output_validation.status =
        LocalProviderOutputValidationStatus::Rejected;
    validation_rejected.local_provider_output_pipeline =
        derive_local_provider_output_pipeline_projection(&validation_rejected);
    assert_eq!(
        validate_local_candidate_materialization_request(&validation_rejected, &request),
        Err(LocalCandidateMaterializationError::ProviderPipelineRejected)
    );
    let mut proposal_drift = request.clone();
    proposal_drift.staged_proposal_id = "drift".to_string();
    assert_eq!(
        validate_local_candidate_materialization_request(&state, &proposal_drift),
        Err(LocalCandidateMaterializationError::StagedProposalIdMismatch)
    );
    let mut invocation_drift = request.clone();
    invocation_drift.source_invocation_result_id = "drift".to_string();
    assert_eq!(
        validate_local_candidate_materialization_request(&state, &invocation_drift),
        Err(LocalCandidateMaterializationError::InvocationResultIdMismatch)
    );
    let mut execution_drift = request.clone();
    execution_drift.provider_execution_result_id = "drift".to_string();
    assert_eq!(
        validate_local_candidate_materialization_request(&state, &execution_drift),
        Err(LocalCandidateMaterializationError::ProviderExecutionResultIdMismatch)
    );
    let claim_cases = [
        (
            "trust",
            LocalCandidateMaterializationError::TrustClaimRejected,
        ),
        (
            "safety",
            LocalCandidateMaterializationError::SafetyClaimRejected,
        ),
        (
            "readiness",
            LocalCandidateMaterializationError::ReadinessClaimRejected,
        ),
        (
            "release",
            LocalCandidateMaterializationError::ReleaseClaimRejected,
        ),
        (
            "deployment",
            LocalCandidateMaterializationError::DeploymentClaimRejected,
        ),
        (
            "public",
            LocalCandidateMaterializationError::PublicUseClaimRejected,
        ),
        (
            "provider",
            LocalCandidateMaterializationError::ProviderOutputApprovalClaimRejected,
        ),
        (
            "action",
            LocalCandidateMaterializationError::ActionClaimRejected,
        ),
        (
            "persistence",
            LocalCandidateMaterializationError::PersistenceClaimRejected,
        ),
        (
            "execution",
            LocalCandidateMaterializationError::ExecutionClaimRejected,
        ),
        (
            "signing",
            LocalCandidateMaterializationError::SigningClaimRejected,
        ),
        (
            "publishing",
            LocalCandidateMaterializationError::PublishingClaimRejected,
        ),
    ];
    for (claim, expected) in claim_cases {
        let mut claim_request = request.clone();
        match claim {
            "trust" => claim_request.claims_trust = true,
            "safety" => claim_request.claims_safety = true,
            "readiness" => claim_request.claims_readiness = true,
            "release" => claim_request.claims_release = true,
            "deployment" => claim_request.claims_deployment = true,
            "public" => claim_request.claims_public_use = true,
            "provider" => claim_request.claims_provider_output_approval = true,
            "action" => claim_request.claims_action = true,
            "persistence" => claim_request.claims_persistence = true,
            "execution" => claim_request.claims_execution = true,
            "signing" => claim_request.claims_signing = true,
            "publishing" => claim_request.claims_publishing = true,
            _ => unreachable!(),
        }
        assert_eq!(
            validate_local_candidate_materialization_request(&state, &claim_request),
            Err(expected)
        );
    }
}

#[test]
fn phase_158_materialization_preserves_no_effect_boundaries() {
    let state = phase_158_approved_decision_state();
    let before = state.clone();
    let request = LocalCandidateMaterializationRequest::from_validated_state(&state).unwrap();
    let next = materialize_local_candidate_output(&state, request).unwrap();
    assert_eq!(next.provider_configuration, before.provider_configuration);
    assert_eq!(next.provider_execution, before.provider_execution);
    assert_eq!(
        next.provider_output_validation,
        before.provider_output_validation
    );
    assert_eq!(
        next.staged_candidate_conversion_proposal,
        before.staged_candidate_conversion_proposal
    );
    assert_eq!(
        next.staged_candidate_conversion_validation,
        before.staged_candidate_conversion_validation
    );
    assert_eq!(
        next.operator_candidate_decision,
        before.operator_candidate_decision
    );
    assert_eq!(
        next.local_session_restore_projection,
        before.local_session_restore_projection
    );
    assert_eq!(
        next.local_session_evidence_export,
        before.local_session_evidence_export
    );
    for required in [
        LocalCandidateMaterializationEffectStatus::NoProviderExecution,
        LocalCandidateMaterializationEffectStatus::NoFileWrite,
        LocalCandidateMaterializationEffectStatus::NoNetworkSocket,
        LocalCandidateMaterializationEffectStatus::NoProcessSpawn,
        LocalCandidateMaterializationEffectStatus::NoSecretRead,
        LocalCandidateMaterializationEffectStatus::NoActionExecution,
        LocalCandidateMaterializationEffectStatus::NoReplayRepair,
        LocalCandidateMaterializationEffectStatus::NoRecoveryPromotion,
        LocalCandidateMaterializationEffectStatus::NoExportPromotion,
    ] {
        assert!(next
            .local_candidate_output
            .effect_statuses
            .contains(&required));
    }
}

#[test]
fn phase_156_initial_projection_and_allowlisted_invocation_are_deterministic() {
    let initial = initial_local_operator_shell_state();
    assert_eq!(
        initial.constrained_local_provider_invocation.status,
        ConstrainedLocalProviderInvocationStatus::NotInvoked
    );
    assert!(initial
        .constrained_local_provider_invocation
        .boundary_statuses
        .contains(
            &ConstrainedLocalProviderInvocationBoundaryStatus::ConstrainedLocalInvocationOnly
        ));
    assert!(initial
        .constrained_local_provider_invocation
        .boundary_statuses
        .contains(&ConstrainedLocalProviderInvocationBoundaryStatus::AllowlistedProviderOnly));
    assert!(initial
        .constrained_local_provider_invocation
        .boundary_statuses
        .contains(&ConstrainedLocalProviderInvocationBoundaryStatus::NoArbitraryCommand));

    let state = apply_local_provider_adapter_declaration(
        &initial,
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
    )
    .unwrap();
    let request = ConstrainedLocalProviderInvocationRequest::allowlisted_default();
    let first = execute_constrained_local_provider_invocation(&state, request.clone()).unwrap();
    let second = execute_constrained_local_provider_invocation(&state, request).unwrap();
    let first_result = first
        .constrained_local_provider_invocation
        .result
        .as_ref()
        .unwrap();
    let second_result = second
        .constrained_local_provider_invocation
        .result
        .as_ref()
        .unwrap();

    assert_eq!(
        first.constrained_local_provider_invocation.status,
        ConstrainedLocalProviderInvocationStatus::InvocationExecuted
    );
    assert_eq!(first_result.result_id, second_result.result_id);
    assert_eq!(first_result.output_summary, second_result.output_summary);
    assert!(first_result
        .result_id
        .starts_with("constrained-local-provider-invocation-"));
    assert!(first_result
        .output_summary
        .starts_with("allowlisted_local_deterministic_provider descriptive output"));
    assert_eq!(
        first
            .constrained_local_provider_invocation
            .output_trust_status,
        ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive
    );
    assert!(first
        .constrained_local_provider_invocation
        .boundary_statuses
        .contains(&ConstrainedLocalProviderInvocationBoundaryStatus::NoShell));
    assert!(first
        .constrained_local_provider_invocation
        .boundary_statuses
        .contains(&ConstrainedLocalProviderInvocationBoundaryStatus::NoNetwork));
    assert!(first
        .constrained_local_provider_invocation
        .boundary_statuses
        .contains(&ConstrainedLocalProviderInvocationBoundaryStatus::NoCloud));
    assert!(first
        .constrained_local_provider_invocation
        .boundary_statuses
        .contains(&ConstrainedLocalProviderInvocationBoundaryStatus::NoSecrets));
}

#[test]
fn phase_156_invocation_rejects_preconditions_and_forbidden_fields() {
    let initial = initial_local_operator_shell_state();
    let missing = execute_constrained_local_provider_invocation(
        &initial,
        ConstrainedLocalProviderInvocationRequest::allowlisted_default(),
    )
    .unwrap_err();
    assert_eq!(
        missing.status,
        ConstrainedLocalProviderInvocationStatus::AllowlistedProviderRequired
    );
    assert!(missing
        .error_codes
        .contains(&ConstrainedLocalProviderInvocationError::NoAdapterDeclared));

    let rejected_adapter = apply_local_provider_adapter_declaration(
        &initial,
        LocalProviderAdapterConfigurationCandidate::local_model_adapter_contract(),
    )
    .unwrap();
    let adapter_rejected = execute_constrained_local_provider_invocation(
        &rejected_adapter,
        ConstrainedLocalProviderInvocationRequest::allowlisted_default(),
    )
    .unwrap_err();
    assert!(adapter_rejected
        .error_codes
        .contains(&ConstrainedLocalProviderInvocationError::AdapterNotAccepted));

    let accepted = apply_local_provider_adapter_declaration(
        &initial,
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
    )
    .unwrap();
    let unsupported = execute_constrained_local_provider_invocation(
        &accepted,
        ConstrainedLocalProviderInvocationRequest {
            provider_kind: AllowlistedLocalProviderKind::UnsupportedCloudProvider,
            input_summary: "cloud provider rejected".to_string(),
            fields: Vec::new(),
        },
    )
    .unwrap_err();
    assert!(unsupported
        .error_codes
        .contains(&ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted));
    assert!(unsupported
        .error_codes
        .contains(&ConstrainedLocalProviderInvocationError::NetworkFieldRejected));

    let cases = [
        (
            ("command", "run"),
            ConstrainedLocalProviderInvocationError::ArbitraryCommandRejected,
        ),
        (
            ("shell", "true"),
            ConstrainedLocalProviderInvocationError::ShellFieldRejected,
        ),
        (
            ("process", "spawn"),
            ConstrainedLocalProviderInvocationError::ProcessFieldRejected,
        ),
        (
            ("args", "--flag"),
            ConstrainedLocalProviderInvocationError::ArgsFieldRejected,
        ),
        (
            ("endpoint", "https://example.invalid"),
            ConstrainedLocalProviderInvocationError::EndpointFieldRejected,
        ),
        (
            ("network", "true"),
            ConstrainedLocalProviderInvocationError::NetworkFieldRejected,
        ),
        (
            ("api_key", "secret"),
            ConstrainedLocalProviderInvocationError::SecretFieldRejected,
        ),
        (
            ("model_path", "/tmp/model"),
            ConstrainedLocalProviderInvocationError::PathFieldRejected,
        ),
        (
            ("trust", "true"),
            ConstrainedLocalProviderInvocationError::TrustClaimRejected,
        ),
        (
            ("approved_output", "true"),
            ConstrainedLocalProviderInvocationError::ProviderOutputApprovalClaimRejected,
        ),
        (
            ("readiness", "true"),
            ConstrainedLocalProviderInvocationError::ReadinessClaimRejected,
        ),
        (
            ("release", "true"),
            ConstrainedLocalProviderInvocationError::ReleaseClaimRejected,
        ),
        (
            ("deployment", "true"),
            ConstrainedLocalProviderInvocationError::DeploymentClaimRejected,
        ),
        (
            ("public_use", "true"),
            ConstrainedLocalProviderInvocationError::PublicUseClaimRejected,
        ),
        (
            ("candidate", "create"),
            ConstrainedLocalProviderInvocationError::CandidateMaterializationClaimRejected,
        ),
        (
            ("action", "true"),
            ConstrainedLocalProviderInvocationError::ActionClaimRejected,
        ),
        (
            ("persistence", "true"),
            ConstrainedLocalProviderInvocationError::PersistenceClaimRejected,
        ),
    ];
    for ((key, value), expected) in cases {
        let projection = execute_constrained_local_provider_invocation(
            &accepted,
            ConstrainedLocalProviderInvocationRequest {
                provider_kind: AllowlistedLocalProviderKind::AllowlistedLocalDeterministicProvider,
                input_summary: "phase 156 constrained local invocation".to_string(),
                fields: vec![(key.to_string(), value.to_string())],
            },
        )
        .unwrap_err();
        assert!(
            projection.error_codes.contains(&expected),
            "missing {expected:?}"
        );
    }
}

#[test]
fn phase_156_invocation_has_no_decision_replay_candidate_or_restore_effects() {
    let state = apply_local_provider_adapter_declaration(
        &initial_local_operator_shell_state(),
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
    )
    .unwrap();
    let before_ledger = state.decision_ledger.clone();
    let before_replay = state.run.decision_replay.clone();
    let before_run_candidate = state.run.candidate.clone();
    let before_export_status = state.local_session_evidence_export.export_status;
    let after = execute_constrained_local_provider_invocation(
        &state,
        ConstrainedLocalProviderInvocationRequest::allowlisted_default(),
    )
    .unwrap();
    assert_eq!(after.decision_ledger, before_ledger);
    assert_eq!(after.run.decision_replay, before_replay);
    assert_eq!(after.run.candidate, before_run_candidate);
    assert_eq!(
        after.local_session_evidence_export.export_status,
        before_export_status
    );
    assert_eq!(
        after.local_session_package_projection,
        state.local_session_package_projection
    );
    assert_eq!(
        after.local_session_restore_projection,
        state.local_session_restore_projection
    );
}

#[test]
fn phase_156_rejected_invocation_preserves_prior_accepted_projection_in_transport() {
    let mut transport = LocalOperatorShellTransport::new();
    submit_local_provider_adapter_declaration(
        &mut transport,
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
    );
    let accepted = invoke_constrained_local_provider(
        &mut transport,
        ConstrainedLocalProviderInvocationRequest::allowlisted_default(),
    );
    let accepted_projection = accepted.state.constrained_local_provider_invocation.clone();
    let rejected = invoke_constrained_local_provider(
        &mut transport,
        ConstrainedLocalProviderInvocationRequest {
            provider_kind: AllowlistedLocalProviderKind::AllowlistedLocalDeterministicProvider,
            input_summary: "phase 156 rejected command".to_string(),
            fields: vec![("command".to_string(), "run".to_string())],
        },
    );
    assert_eq!(rejected.status, LocalOperatorShellTransportStatus::Rejected);
    assert_eq!(
        rejected.state.constrained_local_provider_invocation,
        accepted_projection
    );
}
#[test]
fn phase_159_initial_complete_workflow_projection_is_blocked_local_beta() {
    let state = initial_local_operator_shell_state();
    let workflow = &state.complete_local_operator_workflow;
    assert_eq!(
        workflow.status,
        CompleteLocalOperatorWorkflowStatus::Blocked
    );
    assert_eq!(workflow.classification, "local_beta_workflow_only");
    assert_eq!(
        workflow.current_blocking_step,
        Some(CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured)
    );
    assert_eq!(
        workflow.current_error,
        Some(CompleteLocalOperatorWorkflowError::AdapterNotConfigured)
    );
    assert_eq!(
        workflow.steps.len(),
        complete_local_operator_workflow_step_order().len()
    );
    assert!(workflow
        .boundary_statuses
        .contains(&CompleteLocalOperatorWorkflowBoundaryStatus::LocalBetaWorkflowOnly));
    assert!(workflow
        .boundary_statuses
        .contains(&CompleteLocalOperatorWorkflowBoundaryStatus::NoProviderTrust));
    assert!(workflow.no_authority_note.contains(
            "Workflow completion does not approve readiness, release, deployment, public use, or production use"
        ));
}

#[test]
fn phase_159_complete_workflow_projection_is_deterministic_and_non_mutating() {
    let state = phase_158_approved_decision_state();
    let before = state.clone();
    let first = derive_complete_local_operator_workflow_projection(&state);
    let second = derive_complete_local_operator_workflow_projection(&state);
    assert_eq!(first, second);
    assert_eq!(state, before);
    assert!(!first.capability_surface.provider_trust_granted);
    assert!(!first.capability_surface.action_execution_authorized);
    assert!(!first.capability_surface.readiness_approved);
    assert!(!first.capability_surface.release_approved);
    assert!(!first.capability_surface.deployment_approved);
    assert!(!first.capability_surface.public_use_approved);
    assert!(!first.capability_surface.replay_repair_performed);
    assert!(!first.capability_surface.recovery_promotion_performed);
}

#[test]
fn phase_159_complete_workflow_reaches_projected_after_materialization() {
    let state = phase_158_approved_decision_state();
    let request = LocalCandidateMaterializationRequest::from_validated_state(&state).unwrap();
    let materialized = materialize_local_candidate_output(&state, request).unwrap();
    let workflow = &materialized.complete_local_operator_workflow;
    assert_eq!(
        workflow.status,
        CompleteLocalOperatorWorkflowStatus::CompleteLocalWorkflowProjected
    );
    assert_eq!(workflow.current_blocking_step, None);
    assert_eq!(workflow.current_error, None);
    assert!(workflow.steps.iter().any(|step| {
        step.step == CompleteLocalOperatorWorkflowStepKind::LocalCandidateMaterialized
            && step.status == CompleteLocalOperatorWorkflowStepStatus::Completed
    }));
    assert_eq!(
        workflow
            .evidence_summary
            .local_candidate_materialization_status,
        "local_candidate_materialized"
    );
    assert_eq!(
        workflow.evidence_summary.provider_output_pipeline_status,
        "valid"
    );
    assert_eq!(
        workflow.evidence_summary.local_evidence_export_status,
        materialized
            .local_session_evidence_export
            .export_status
            .code()
    );
    assert_eq!(
        workflow.evidence_summary.session_package_status,
        materialized.local_session_package_projection.status.code()
    );
    assert_eq!(
        workflow.evidence_summary.restore_status,
        materialized.local_session_restore_projection.status.code()
    );
}

#[test]
fn phase_159_complete_workflow_detects_rejected_invocation_and_decision() {
    let configured = apply_local_provider_adapter_declaration(
        &initial_local_operator_shell_state(),
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
    )
    .unwrap();
    let mut rejected_invocation = configured.clone();
    rejected_invocation.constrained_local_provider_invocation =
        reject_constrained_local_provider_invocation(
            ConstrainedLocalProviderInvocationStatus::InvocationRejected,
            None,
            None,
            None,
            rejected_invocation
                .local_provider_adapter_registry
                .declarations
                .len(),
            vec![ConstrainedLocalProviderInvocationError::SecretFieldRejected],
        );
    rejected_invocation = attach_local_session_evidence_export(rejected_invocation);
    let workflow = &rejected_invocation.complete_local_operator_workflow;
    assert_eq!(
        workflow.status,
        CompleteLocalOperatorWorkflowStatus::Rejected
    );
    assert_eq!(
        workflow.current_blocking_step,
        Some(CompleteLocalOperatorWorkflowStepKind::ConstrainedInvocationCompleted)
    );
    assert!(workflow
        .rejection_reasons
        .iter()
        .any(|reason| reason.contains("invocation_rejected")));

    let mut rejected_decision = phase_158_approved_decision_state();
    rejected_decision.operator_candidate_decision = rejected_operator_candidate_decision_projection(
        OperatorCandidateDecisionError::PublicUseClaimRejected,
    );
    let decision_workflow = derive_complete_local_operator_workflow_projection(&rejected_decision);
    assert_eq!(
        decision_workflow.current_blocking_step,
        Some(CompleteLocalOperatorWorkflowStepKind::OperatorDecisionRecorded)
    );
    assert_eq!(
        decision_workflow.current_error,
        Some(CompleteLocalOperatorWorkflowError::OperatorDecisionRejected)
    );
}

#[test]
fn phase_159_transport_response_carries_complete_workflow_projection() {
    let mut transport = LocalOperatorShellTransport::new();
    let response = local_operator_shell_transport_step(
        &transport.current_state(),
        LocalOperatorShellRequest::GetInitialState,
    );
    assert_eq!(
        response.state.complete_local_operator_workflow,
        derive_complete_local_operator_workflow_projection(&response.state)
    );
    submit_local_provider_adapter_declaration(
        &mut transport,
        LocalProviderAdapterConfigurationCandidate::deterministic_fake_adapter(),
    );
    let invoked = invoke_constrained_local_provider(
        &mut transport,
        ConstrainedLocalProviderInvocationRequest::allowlisted_default(),
    );
    assert_eq!(
        invoked.state.complete_local_operator_workflow,
        derive_complete_local_operator_workflow_projection(&invoked.state)
    );
    assert!(invoked
        .state
        .complete_local_operator_workflow
        .steps
        .iter()
        .any(
            |step| step.step == CompleteLocalOperatorWorkflowStepKind::ProviderOutputValidated
                && step.status == CompleteLocalOperatorWorkflowStepStatus::Completed
        ));
}

fn phase_161_trial_scope() -> ControlledInternalTrialScope {
    ControlledInternalTrialScope {
        scope_id: "phase-161-internal-trial-scope".to_string(),
        scope_summary: "Controlled internal trial preparation from local beta workflow evidence"
            .to_string(),
        local_beta_workflow_scope: "provider setup through restore/history projection".to_string(),
    }
}

fn phase_161_trial_operator() -> ControlledInternalTrialOperator {
    ControlledInternalTrialOperator {
        operator_id: "internal-operator-alpha".to_string(),
        display_label: "Internal Operator Alpha".to_string(),
        role: "internal_trial_operator".to_string(),
    }
}

fn phase_161_trial_participant() -> ControlledInternalTrialParticipant {
    ControlledInternalTrialParticipant {
        participant_id: "internal-participant-beta".to_string(),
        display_label: "Internal Participant Beta".to_string(),
        role: "internal_trial_participant".to_string(),
    }
}

fn phase_161_trial_package() -> ControlledInternalTrialPackage {
    derive_controlled_internal_trial_package(
        &initial_local_operator_shell_state(),
        phase_161_trial_scope(),
        vec![phase_161_trial_operator()],
        vec![phase_161_trial_participant()],
        required_controlled_internal_trial_stop_conditions(),
    )
}

#[test]
fn phase_161_initial_state_exposes_not_packaged_projection() {
    let state = initial_local_operator_shell_state();
    let projection = state.controlled_internal_trial_package_projection;
    assert_eq!(
        projection.status,
        ControlledInternalTrialPackageStatus::NotPackaged
    );
    assert_eq!(projection.package_id, None);
    assert_eq!(
        projection.package_classification,
        "controlled_internal_trial_package_only"
    );
    assert_eq!(projection.production_classification, "non_production");
    assert_eq!(
        projection.distribution_classification,
        "local_only_non_public"
    );
    assert!(projection
        .local_only_non_public_note
        .contains("local-only and non-public"));
    assert!(projection
        .release_boundary_note
        .contains("not a release artifact"));
    assert!(projection
        .deployment_readiness_boundary_note
        .contains("not deployment or readiness evidence"));
    assert!(projection
        .public_production_boundary_note
        .contains("does not approve public/general use or production use"));
}

#[test]
fn phase_161_package_derivation_and_serialization_are_deterministic() {
    let first = phase_161_trial_package();
    let second = phase_161_trial_package();
    assert_eq!(first, second);
    assert_eq!(first.metadata.package_id, second.metadata.package_id);
    assert_eq!(
        first.metadata.content_digest,
        second.metadata.content_digest
    );
    assert_eq!(
        serialize_controlled_internal_trial_package(&first).unwrap(),
        serialize_controlled_internal_trial_package(&second).unwrap()
    );
    assert_eq!(
        first.metadata.package_classification,
        "controlled_internal_trial_package_only"
    );
    assert_eq!(first.metadata.production_classification, "non_production");
    assert_eq!(
        first.metadata.distribution_classification,
        "local_only_non_public"
    );
}

#[test]
fn phase_161_valid_package_includes_trial_metadata_stop_conditions_and_evidence() {
    let package = phase_161_trial_package();
    validate_controlled_internal_trial_package(&package).unwrap();
    assert!(package.payload.trial_scope.is_some());
    assert_eq!(package.payload.named_internal_operators.len(), 1);
    assert_eq!(package.payload.trial_participants.len(), 1);
    assert_eq!(
        package.payload.stop_conditions,
        required_controlled_internal_trial_stop_conditions()
    );
    let summary =
        controlled_internal_trial_included_evidence_summary(&package.payload.included_evidence);
    assert!(summary
        .iter()
        .any(|line| line.contains("local beta workflow status")));
    assert!(summary
        .iter()
        .any(|line| line.contains("local candidate materialization status")));
    assert!(summary
        .iter()
        .any(|line| line.contains("replay/status summary")));
    assert!(summary
        .iter()
        .any(|line| line.contains("local evidence export summary")));
    assert!(summary
        .iter()
        .any(|line| line.contains("local session package summary")));
    assert!(summary
        .iter()
        .any(|line| line.contains("restore/history summary")));
    assert!(summary
        .iter()
        .any(|line| line.contains("Phase 160 gate decision")));
}

#[test]
fn phase_161_validation_rejects_missing_metadata_scope_operator_stop_and_markers() {
    let mut package = phase_161_trial_package();
    package.metadata.package_id.clear();
    assert!(validate_controlled_internal_trial_package(&package)
        .unwrap_err()
        .contains(&ControlledInternalTrialPackageValidationError::MissingPackageId));

    let mut package = phase_161_trial_package();
    package.metadata.package_version.clear();
    assert!(validate_controlled_internal_trial_package(&package)
        .unwrap_err()
        .contains(&ControlledInternalTrialPackageValidationError::MissingPackageVersion));

    let mut package = phase_161_trial_package();
    package.metadata.package_classification = "local_session_package_only".to_string();
    assert!(validate_controlled_internal_trial_package(&package)
        .unwrap_err()
        .contains(&ControlledInternalTrialPackageValidationError::InvalidPackageClassification));

    let mut package = phase_161_trial_package();
    package.metadata.production_classification = "production".to_string();
    assert!(validate_controlled_internal_trial_package(&package)
        .unwrap_err()
        .contains(&ControlledInternalTrialPackageValidationError::InvalidProductionClassification));

    let mut package = phase_161_trial_package();
    package.metadata.distribution_classification = "public".to_string();
    assert!(validate_controlled_internal_trial_package(&package)
        .unwrap_err()
        .contains(
            &ControlledInternalTrialPackageValidationError::InvalidDistributionClassification
        ));

    let mut package = phase_161_trial_package();
    package.payload.trial_scope = None;
    assert!(validate_controlled_internal_trial_package(&package)
        .unwrap_err()
        .contains(&ControlledInternalTrialPackageValidationError::MissingTrialScope));

    let mut package = phase_161_trial_package();
    package.payload.named_internal_operators.clear();
    assert!(validate_controlled_internal_trial_package(&package)
        .unwrap_err()
        .contains(
            &ControlledInternalTrialPackageValidationError::MissingNamedOperatorOrParticipant
        ));

    let mut package = phase_161_trial_package();
    package.payload.trial_participants.clear();
    assert!(validate_controlled_internal_trial_package(&package)
        .unwrap_err()
        .contains(
            &ControlledInternalTrialPackageValidationError::MissingNamedOperatorOrParticipant
        ));

    let mut package = phase_161_trial_package();
    package.payload.stop_conditions.clear();
    assert!(validate_controlled_internal_trial_package(&package)
        .unwrap_err()
        .contains(&ControlledInternalTrialPackageValidationError::MissingStopConditionMarkers));

    let mut package = phase_161_trial_package();
    package.absence_markers.release_artifact_absent = false;
    assert!(validate_controlled_internal_trial_package(&package)
        .unwrap_err()
        .contains(&ControlledInternalTrialPackageValidationError::MissingAbsenceMarker));
}

#[test]
fn phase_161_validation_rejects_no_authority_boundary_claims() {
    let release_claims = [
        "claim:readiness_approved",
        "claim:release_candidate_approved",
        "claim:deployment_enabled",
        "claim:public_use_approved",
        "claim:production_use_approved",
    ];
    for claim in release_claims {
        let mut package = phase_161_trial_package();
        package
            .payload
            .included_evidence
            .phase_160_gate_decision_context = claim.to_string();
        assert!(validate_controlled_internal_trial_package(&package).unwrap_err().contains(&ControlledInternalTrialPackageValidationError::ForbiddenReleaseDeploymentReadinessPublicOrProductionUseClaim));
    }
    let execution_claims = [
        "claim:provider_trusted",
        "claim:action_authorized",
        "claim:replay_repaired",
        "claim:recovery_promoted",
    ];
    for claim in execution_claims {
        let mut package = phase_161_trial_package();
        package
            .payload
            .included_evidence
            .phase_160_gate_decision_context = claim.to_string();
        assert!(validate_controlled_internal_trial_package(&package).unwrap_err().contains(&ControlledInternalTrialPackageValidationError::ForbiddenProviderTrustActionReplayRepairOrRecoveryPromotionClaim));
    }
}

#[test]
fn phase_161_explicit_write_read_and_read_back_validation_use_temp_path() {
    let state_before = initial_local_operator_shell_state();
    let package = phase_161_trial_package();
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::path::PathBuf::from(format!(
        "/tmp/ajentic-phase-161-{unique}-{}.trialpkg",
        package.metadata.content_digest
    ));
    let write = write_controlled_internal_trial_package(&package, &path).unwrap();
    assert_eq!(
        write.status,
        ControlledInternalTrialPackageStatus::PackageWritten
    );
    assert!(write.bytes_written > 0);
    let read = read_controlled_internal_trial_package(&path).unwrap();
    assert_eq!(
        read.status,
        ControlledInternalTrialPackageStatus::PackageReadBackValidated
    );
    assert_eq!(
        read.projection.read_back_validation_status,
        Some(ControlledInternalTrialPackageValidationStatus::Valid)
    );
    assert_eq!(initial_local_operator_shell_state(), state_before);
}

#[test]
fn phase_161_read_back_rejects_malformed_content() {
    assert!(
        parse_controlled_internal_trial_package("not a controlled internal trial package")
            .unwrap_err()
            .contains(&ControlledInternalTrialPackageValidationError::MalformedPackageInput)
    );
}

fn phase_162_state_with_package(
    mut package: ControlledInternalTrialPackage,
) -> LocalOperatorShellState {
    package.metadata.package_status = ControlledInternalTrialPackageStatus::PackageValidated;
    let mut state = initial_local_operator_shell_state();
    state.controlled_internal_trial_package_projection =
        project_controlled_internal_trial_package_status(
            Some(&package),
            Some(ControlledInternalTrialPackageValidationStatus::Valid),
        );
    attach_local_session_evidence_export(state)
}

#[test]
fn phase_162_initial_runbook_and_failure_drill_are_projected_and_deterministic() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.trial_operator_runbook,
        derive_trial_operator_runbook_projection(&state)
    );
    assert_eq!(
        state.trial_failure_drill,
        derive_trial_failure_drill_projection(&state)
    );
    assert_eq!(
        derive_trial_operator_runbook_projection(&state),
        derive_trial_operator_runbook_projection(&state)
    );
    assert_eq!(
        derive_trial_failure_drill_projection(&state),
        derive_trial_failure_drill_projection(&state)
    );
    assert_eq!(
        state.trial_operator_runbook.status,
        TrialOperatorRunbookStatus::TrialPackageRequired
    );
    assert!(state
        .trial_failure_drill
        .categories
        .iter()
        .any(|entry| entry.category == TrialFailureCategory::NoTrialPackage));
    assert!(state
        .trial_operator_runbook
        .boundary_statuses
        .contains(&TrialRunbookBoundaryStatus::LocalTrialGuidanceOnly));
    assert!(state
        .trial_operator_runbook
        .boundary_statuses
        .contains(&TrialRunbookBoundaryStatus::NoTrialExecution));
    assert!(
        !state
            .trial_operator_runbook
            .capability_surface
            .executes_trial
    );
    assert!(
        !state
            .trial_operator_runbook
            .capability_surface
            .activates_authority
    );
}

#[test]
fn phase_162_valid_package_exposes_package_state_steps_and_stop_drill() {
    let state = phase_162_state_with_package(phase_161_trial_package());
    let runbook = &state.trial_operator_runbook;
    assert_eq!(
        runbook.trial_package_id,
        state
            .controlled_internal_trial_package_projection
            .package_id
    );
    assert_eq!(
        runbook.trial_package_validation_status,
        ControlledInternalTrialPackageValidationStatus::Valid
    );
    assert_eq!(
        runbook.trial_scope_status,
        TrialOperatorRunbookStepStatus::Completed
    );
    assert_eq!(
        runbook.named_operator_status,
        TrialOperatorRunbookStepStatus::Completed
    );
    assert_eq!(
        runbook.named_participant_status,
        TrialOperatorRunbookStepStatus::Completed
    );
    assert_eq!(
        runbook.stop_condition_summary,
        required_controlled_internal_trial_stop_conditions()
            .into_iter()
            .map(|condition| condition.code().to_string())
            .collect::<Vec<_>>()
    );
    assert!(runbook
        .steps
        .iter()
        .any(|step| step.step == TrialOperatorRunbookStepKind::ReviewStopConditions));
    assert!(state
        .trial_failure_drill
        .stop_condition_drills
        .iter()
        .all(|drill| !drill.enforcement_automated));
}

#[test]
fn phase_162_missing_or_rejected_package_metadata_blocks_runbook() {
    let mut missing_scope = phase_161_trial_package();
    missing_scope.payload.trial_scope = None;
    let state = phase_162_state_with_package(missing_scope);
    assert_eq!(
        state.trial_operator_runbook.trial_scope_status,
        TrialOperatorRunbookStepStatus::Blocked
    );
    assert!(state
        .trial_failure_drill
        .categories
        .iter()
        .any(|entry| entry.category == TrialFailureCategory::MissingTrialScope));

    let mut missing_operator = phase_161_trial_package();
    missing_operator.payload.named_internal_operators.clear();
    let state = phase_162_state_with_package(missing_operator);
    assert_eq!(
        state.trial_operator_runbook.named_operator_status,
        TrialOperatorRunbookStepStatus::Blocked
    );
    assert!(state
        .trial_failure_drill
        .categories
        .iter()
        .any(|entry| entry.category == TrialFailureCategory::MissingNamedOperator));

    let mut missing_participant = phase_161_trial_package();
    missing_participant.payload.trial_participants.clear();
    let state = phase_162_state_with_package(missing_participant);
    assert_eq!(
        state.trial_operator_runbook.named_participant_status,
        TrialOperatorRunbookStepStatus::Blocked
    );
    assert!(state
        .trial_failure_drill
        .categories
        .iter()
        .any(|entry| entry.category == TrialFailureCategory::MissingNamedParticipant));

    let mut missing_stop = phase_161_trial_package();
    missing_stop.payload.stop_conditions.clear();
    let state = phase_162_state_with_package(missing_stop);
    assert!(state
        .trial_failure_drill
        .categories
        .iter()
        .any(|entry| entry.category == TrialFailureCategory::MissingStopConditions));
}

#[test]
fn phase_162_failure_drill_classifies_workflow_provider_restore_and_read_back() {
    let mut state = phase_162_state_with_package(phase_161_trial_package());
    state.local_provider_output_pipeline.status =
        LocalProviderOutputPipelineValidationStatus::Rejected;
    state.provider_output_validation.status = LocalProviderOutputValidationStatus::Rejected;
    state.local_session_restore_projection.status = LocalSessionRestoreStatus::RestoreRejected;
    state.local_session_restore_projection.validation_status =
        LocalSessionRestoreValidationStatus::Invalid;
    state
        .controlled_internal_trial_package_projection
        .read_back_validation_status =
        Some(ControlledInternalTrialPackageValidationStatus::Invalid);
    state.complete_local_operator_workflow =
        derive_complete_local_operator_workflow_projection(&state);
    state.trial_failure_drill = derive_trial_failure_drill_projection(&state);
    for expected in [
        TrialFailureCategory::ProviderPipelineBlocked,
        TrialFailureCategory::ProviderOutputValidationRejected,
        TrialFailureCategory::SecurityEscalationRequired,
        TrialFailureCategory::RestoreProjectionRejected,
        TrialFailureCategory::TrialPackageReadBackFailure,
    ] {
        assert!(
            state
                .trial_failure_drill
                .categories
                .iter()
                .any(|entry| entry.category == expected),
            "missing {expected:?}"
        );
    }
    assert!(state
        .trial_failure_drill
        .escalation_guidance
        .iter()
        .any(|guidance| guidance.role == TrialEscalationRole::SecurityReviewer));
}

#[test]
fn phase_162_release_claim_failure_gets_release_steward_guidance() {
    let mut state = phase_162_state_with_package(phase_161_trial_package());
    state
        .controlled_internal_trial_package_projection
        .validation_errors
        .push("release_deployment_readiness_public_production_claim_rejected".to_string());
    state.trial_failure_drill = derive_trial_failure_drill_projection(&state);
    assert!(state
        .trial_failure_drill
        .categories
        .iter()
        .any(|entry| entry.category == TrialFailureCategory::ReleaseStewardReviewRequired));
    assert!(state
        .trial_failure_drill
        .escalation_guidance
        .iter()
        .any(|guidance| guidance.role == TrialEscalationRole::ReleaseSteward));
}

#[test]
fn phase_162_projection_does_not_mutate_shell_state_or_enable_authority() {
    let state = phase_162_state_with_package(phase_161_trial_package());
    let before = state.clone();
    let runbook = derive_trial_operator_runbook_projection(&state);
    let drill = derive_trial_failure_drill_projection(&state);
    assert_eq!(state, before);
    assert!(!runbook.capability_surface.executes_trial);
    assert!(!runbook.capability_surface.approves_readiness);
    assert!(!runbook.capability_surface.approves_release);
    assert!(!runbook.capability_surface.approves_deployment);
    assert!(!runbook.capability_surface.approves_public_use);
    assert!(!runbook.capability_surface.approves_production);
    assert!(!runbook.capability_surface.executes_actions);
    assert!(!runbook.capability_surface.repairs_replay);
    assert!(!runbook.capability_surface.promotes_recovery);
    assert!(drill
        .escalation_guidance
        .iter()
        .all(|guidance| guidance.descriptive_only));
}

fn phase_163_state_with_valid_package() -> LocalOperatorShellState {
    phase_162_state_with_package(phase_161_trial_package())
}

fn phase_163_evidence_record() -> TrialSessionEvidenceRecord {
    derive_trial_session_evidence_record(&phase_163_state_with_valid_package())
}

#[test]
fn phase_163_initial_state_exposes_not_captured_projection() {
    let state = initial_local_operator_shell_state();
    let projection = state.trial_session_evidence_projection;
    assert_eq!(projection.status, TrialSessionEvidenceStatus::NotCaptured);
    assert_eq!(projection.evidence_id, None);
    assert_eq!(
        projection.evidence_classification,
        "trial_session_evidence_only"
    );
    assert_eq!(projection.production_classification, "non_production");
    assert_eq!(
        projection.distribution_classification,
        "local_only_non_public"
    );
    assert_eq!(
        projection.authority_classification,
        "non_authoritative_evidence"
    );
    assert!(projection
        .local_only_non_authoritative_note
        .contains("local-only, non-public, and non-authoritative"));
    assert!(projection
        .no_trial_approval_note
        .contains("does not start or approve a controlled trial"));
}

#[test]
fn phase_163_evidence_derivation_id_and_serialization_are_deterministic() {
    let state = phase_163_state_with_valid_package();
    let before = state.clone();
    let first = derive_trial_session_evidence_record(&state);
    let second = derive_trial_session_evidence_record(&state);
    assert_eq!(state, before);
    assert_eq!(first, second);
    assert_eq!(first.metadata.evidence_id, second.metadata.evidence_id);
    assert_eq!(
        first.metadata.content_digest,
        second.metadata.content_digest
    );
    assert_eq!(
        serialize_trial_session_evidence_record(&first).unwrap(),
        serialize_trial_session_evidence_record(&second).unwrap()
    );
    assert_eq!(
        first.metadata.evidence_classification,
        "trial_session_evidence_only"
    );
    assert_eq!(first.metadata.production_classification, "non_production");
    assert_eq!(
        first.metadata.distribution_classification,
        "local_only_non_public"
    );
    assert_eq!(
        first.metadata.authority_classification,
        "non_authoritative_evidence"
    );
}

#[test]
fn phase_163_valid_evidence_includes_linkages_snapshots_and_boundaries() {
    let record = phase_163_evidence_record();
    validate_trial_session_evidence_record(&record).unwrap();
    assert!(record
        .payload
        .trial_package_id
        .starts_with("controlled-internal-trial-package-"));
    assert!(!record.payload.runbook_status.is_empty());
    assert!(!record.payload.failure_drill_status.is_empty());
    assert!(!record.payload.workflow_status_snapshot.is_empty());
    assert!(!record
        .payload
        .local_candidate_materialization_snapshot
        .is_empty());
    assert!(!record.payload.provider_output_pipeline_snapshot.is_empty());
    assert!(!record.payload.operator_decision_snapshot.is_empty());
    assert!(!record.payload.replay_status_snapshot.is_empty());
    assert!(!record.payload.local_evidence_export_snapshot.is_empty());
    assert!(!record.payload.local_session_package_snapshot.is_empty());
    assert!(!record.payload.restore_history_snapshot.is_empty());
    assert!(!record.payload.stop_condition_snapshot.is_empty());
    assert!(!record.payload.escalation_guidance_snapshot.is_empty());
    assert!(!record.payload.failure_category_snapshot.is_empty());
    assert!(!record.payload.current_blocker_snapshot.is_empty());
    assert!(record
        .payload
        .boundary_status
        .contains(&TrialSessionEvidenceBoundaryStatus::NoControlledHumanUseApproval));
    assert!(record
        .payload
        .boundary_status
        .contains(&TrialSessionEvidenceBoundaryStatus::NoPublicUseApproval));
    assert!(record
        .payload
        .boundary_status
        .contains(&TrialSessionEvidenceBoundaryStatus::NoProductionApproval));
    assert!(record.absence_markers.release_artifact_absent);
    assert!(record.absence_markers.default_persistence_absent);
}

#[test]
fn phase_163_validation_rejects_missing_required_fields_and_linkages() {
    let mut record = phase_163_evidence_record();
    record.metadata.evidence_id.clear();
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::MissingEvidenceId));

    let mut record = phase_163_evidence_record();
    record.metadata.evidence_version.clear();
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::MissingEvidenceVersion));

    let mut record = phase_163_evidence_record();
    record.metadata.evidence_classification = "local_session_package_only".to_string();
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::InvalidEvidenceClassification));

    let mut record = phase_163_evidence_record();
    record.metadata.production_classification = "production".to_string();
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::InvalidProductionClassification));

    let mut record = phase_163_evidence_record();
    record.metadata.distribution_classification = "public".to_string();
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::InvalidDistributionClassification));

    let mut record = phase_163_evidence_record();
    record.metadata.authority_classification = "authoritative".to_string();
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::InvalidAuthorityClassification));

    let mut record = phase_163_evidence_record();
    record.payload.trial_package_id.clear();
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::MissingTrialPackageLinkage));

    let mut record = phase_163_evidence_record();
    record.payload.runbook_status.clear();
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::MissingRunbookLinkage));

    let mut record = phase_163_evidence_record();
    record.payload.failure_drill_status.clear();
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::MissingFailureDrillLinkage));

    let mut record = phase_163_evidence_record();
    record.payload.stop_condition_snapshot.clear();
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::MissingStopConditionSnapshot));

    let mut record = phase_163_evidence_record();
    record.payload.escalation_guidance_snapshot.clear();
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::MissingEscalationSnapshot));

    let mut record = phase_163_evidence_record();
    record.absence_markers.release_artifact_absent = false;
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::MissingAbsenceMarker));
}

#[test]
fn phase_163_validation_rejects_authority_and_execution_claims() {
    let claims = [
        (
            "claim:readiness_approved",
            TrialSessionEvidenceValidationError::ReadinessClaimDetected,
        ),
        (
            "claim:release_candidate_approved",
            TrialSessionEvidenceValidationError::ReleaseClaimDetected,
        ),
        (
            "claim:deployment_enabled",
            TrialSessionEvidenceValidationError::DeploymentClaimDetected,
        ),
        (
            "claim:public_use_approved",
            TrialSessionEvidenceValidationError::PublicUseClaimDetected,
        ),
        (
            "claim:production_use_approved",
            TrialSessionEvidenceValidationError::ProductionUseClaimDetected,
        ),
        (
            "claim:provider_trusted",
            TrialSessionEvidenceValidationError::ProviderTrustClaimDetected,
        ),
        (
            "claim:action_authorized",
            TrialSessionEvidenceValidationError::ActionAuthorizationClaimDetected,
        ),
        (
            "claim:replay_repaired",
            TrialSessionEvidenceValidationError::ReplayRepairClaimDetected,
        ),
        (
            "claim:recovery_promoted",
            TrialSessionEvidenceValidationError::RecoveryPromotionClaimDetected,
        ),
    ];
    for (claim, expected) in claims {
        let mut record = phase_163_evidence_record();
        record.payload.current_blocker_snapshot = claim.to_string();
        assert!(validate_trial_session_evidence_record(&record)
            .unwrap_err()
            .contains(&expected));
    }
}

#[test]
fn phase_163_rejects_missing_package_linkage_from_initial_state() {
    let record = derive_trial_session_evidence_record(&initial_local_operator_shell_state());
    assert!(validate_trial_session_evidence_record(&record)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::MissingTrialPackageLinkage));
}

#[test]
fn phase_163_explicit_write_read_and_read_back_validation_use_temp_path() {
    let state_before = phase_163_state_with_valid_package();
    let record = derive_trial_session_evidence_record(&state_before);
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::path::PathBuf::from(format!(
        "/tmp/ajentic-phase-163-{unique}-{}.trialevidence",
        record.metadata.content_digest
    ));
    let write = crate::api::write_trial_session_evidence_record(&record, &path).unwrap();
    assert_eq!(write.status, TrialSessionEvidenceStatus::EvidenceWritten);
    assert!(write.bytes_written > 0);
    let read = crate::api::read_trial_session_evidence_record(&path).unwrap();
    assert_eq!(
        read.status,
        TrialSessionEvidenceStatus::EvidenceReadBackValidated
    );
    assert_eq!(
        read.projection.read_back_validation_status,
        Some(TrialSessionEvidenceValidationStatus::Valid)
    );
    assert_eq!(phase_163_state_with_valid_package(), state_before);
}

#[test]
fn phase_163_read_back_rejects_malformed_content_and_digest_drift() {
    assert!(
        parse_trial_session_evidence_record("not trial session evidence")
            .unwrap_err()
            .contains(&TrialSessionEvidenceValidationError::MalformedEvidenceInput)
    );
    let record = phase_163_evidence_record();
    let mut content = serialize_trial_session_evidence_record(&record).unwrap();
    content = content.replace(
        &format!("content_digest={}", record.metadata.content_digest),
        "content_digest=0000000000000000",
    );
    assert!(parse_trial_session_evidence_record(&content)
        .unwrap_err()
        .contains(&TrialSessionEvidenceValidationError::DeterministicDigestMismatch));
}

fn phase_164_verification_inputs() -> (
    LocalOperatorShellState,
    ControlledInternalTrialPackage,
    TrialSessionEvidenceRecord,
) {
    let mut state = phase_163_state_with_valid_package();
    let mut package = phase_161_trial_package();
    package.metadata.package_status =
        ControlledInternalTrialPackageStatus::PackageReadBackValidated;
    state.controlled_internal_trial_package_projection =
        validate_controlled_internal_trial_package_read_back(&package);
    state.trial_failure_drill = derive_trial_failure_drill_projection(&state);
    state.trial_operator_runbook = derive_trial_operator_runbook_projection(&state);
    let mut evidence = derive_trial_session_evidence_record(&state);
    evidence.metadata.evidence_status = TrialSessionEvidenceStatus::EvidenceReadBackValidated;
    state.trial_session_evidence_projection = validate_trial_session_evidence_read_back(&evidence);
    (state, package, evidence)
}

#[test]
fn phase_164_initial_verification_projection_is_not_verified_and_non_authoritative() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.trial_replay_restore_verification.status,
        TrialReplayRestoreVerificationStatus::NotVerified
    );
    assert!(state
        .trial_replay_restore_verification
        .boundary_status
        .contains(&TrialReplayRestoreVerificationBoundaryStatus::LocalVerificationOnly));
    assert!(state
        .trial_replay_restore_verification
        .boundary_status
        .contains(&TrialReplayRestoreVerificationBoundaryStatus::NoReplayRepair));
    assert!(state
        .trial_replay_restore_verification
        .boundary_status
        .contains(&TrialReplayRestoreVerificationBoundaryStatus::NoRecoveryPromotion));
}

#[test]
fn phase_164_valid_package_evidence_restore_and_replay_verify_deterministically() {
    let (state, package, evidence) = phase_164_verification_inputs();
    let before = state.clone();
    let first = derive_trial_replay_restore_verification_projection(
        &state,
        Some(&package),
        Some(&evidence),
    );
    let second = derive_trial_replay_restore_verification_projection(
        &state,
        Some(&package),
        Some(&evidence),
    );

    assert_eq!(first, second);
    assert_eq!(state, before);
    assert_eq!(
        first.status,
        TrialReplayRestoreVerificationStatus::VerificationPassed
    );
    assert!(first.verification_id.is_some());
    assert!(first.mismatches.is_empty());
    assert_eq!(
        first.comparison_summary.replay_status_comparison,
        "replay/status comparison matched"
    );
    assert_eq!(
        first.comparison_summary.restore_history_comparison,
        "restore/history comparison matched"
    );
}

#[test]
fn phase_164_missing_and_malformed_read_back_inputs_fail_closed() {
    let (state, package, evidence) = phase_164_verification_inputs();
    let missing_package =
        derive_trial_replay_restore_verification_projection(&state, None, Some(&evidence));
    assert_eq!(
        missing_package.status,
        TrialReplayRestoreVerificationStatus::VerificationInputMissing
    );
    assert!(missing_package
        .mismatches
        .contains(&TrialReplayRestoreVerificationMismatch::MissingTrialPackageReadBack));

    let missing_evidence =
        derive_trial_replay_restore_verification_projection(&state, Some(&package), None);
    assert!(missing_evidence
        .mismatches
        .contains(&TrialReplayRestoreVerificationMismatch::MissingTrialSessionEvidenceReadBack));

    let mut malformed_package = package.clone();
    malformed_package.metadata.package_id.clear();
    let rejected = derive_trial_replay_restore_verification_projection(
        &state,
        Some(&malformed_package),
        Some(&evidence),
    );
    assert_eq!(
        rejected.status,
        TrialReplayRestoreVerificationStatus::InvalidVerificationInput
    );
    assert!(rejected
        .mismatches
        .contains(&TrialReplayRestoreVerificationMismatch::TrialPackageReadBackInvalid));

    let mut malformed_evidence = evidence.clone();
    malformed_evidence.metadata.evidence_id.clear();
    let rejected = derive_trial_replay_restore_verification_projection(
        &state,
        Some(&package),
        Some(&malformed_evidence),
    );
    assert!(rejected
        .mismatches
        .contains(&TrialReplayRestoreVerificationMismatch::TrialSessionEvidenceReadBackInvalid));
}

#[test]
fn phase_164_linkage_replay_restore_and_claim_mismatches_reject_in_order() {
    let (state, package, mut evidence) = phase_164_verification_inputs();
    evidence.payload.trial_package_id = "different-package".to_string();
    evidence.payload.trial_package_version = "different-version".to_string();
    evidence.payload.workflow_status_snapshot = "different-workflow".to_string();
    evidence.payload.provider_output_pipeline_snapshot = "different-provider-pipeline".to_string();
    evidence.payload.local_candidate_materialization_snapshot = "different-candidate".to_string();
    evidence.payload.operator_decision_snapshot = "different-decision".to_string();
    evidence.payload.replay_status_snapshot = "different-replay".to_string();
    evidence.payload.restore_history_snapshot = "different-restore".to_string();
    evidence.payload.stop_condition_snapshot = vec!["different-stop".to_string()];
    evidence.payload.escalation_guidance_snapshot = vec!["different-escalation".to_string()];
    evidence.payload.failure_category_snapshot = vec!["different-failure".to_string()];
    evidence.payload.current_blocker_snapshot = "claim:readiness_approved claim:provider_trusted claim:action_authorized claim:replay_repaired claim:recovery_promoted claim:release_candidate_approved claim:deployment_enabled claim:public_use_approved claim:production_use_approved".to_string();

    let projection = derive_trial_replay_restore_verification_projection(
        &state,
        Some(&package),
        Some(&evidence),
    );
    assert_eq!(
        projection.status,
        TrialReplayRestoreVerificationStatus::InvalidVerificationInput
    );
    for expected in [
        TrialReplayRestoreVerificationMismatch::PackageIdMismatch,
        TrialReplayRestoreVerificationMismatch::PackageVersionMismatch,
        TrialReplayRestoreVerificationMismatch::WorkflowStatusMismatch,
        TrialReplayRestoreVerificationMismatch::ProviderOutputPipelineStatusMismatch,
        TrialReplayRestoreVerificationMismatch::LocalCandidateMaterializationStatusMismatch,
        TrialReplayRestoreVerificationMismatch::OperatorDecisionStatusMismatch,
        TrialReplayRestoreVerificationMismatch::ReplayStatusSnapshotMismatch,
        TrialReplayRestoreVerificationMismatch::RestoreHistorySnapshotMismatch,
        TrialReplayRestoreVerificationMismatch::StopConditionSnapshotMismatch,
        TrialReplayRestoreVerificationMismatch::EscalationSnapshotMismatch,
        TrialReplayRestoreVerificationMismatch::FailureCategorySnapshotMismatch,
        TrialReplayRestoreVerificationMismatch::ReadinessClaimDetected,
        TrialReplayRestoreVerificationMismatch::ReleaseClaimDetected,
        TrialReplayRestoreVerificationMismatch::DeploymentClaimDetected,
        TrialReplayRestoreVerificationMismatch::PublicUseClaimDetected,
        TrialReplayRestoreVerificationMismatch::ProductionUseClaimDetected,
        TrialReplayRestoreVerificationMismatch::ProviderTrustClaimDetected,
        TrialReplayRestoreVerificationMismatch::ActionAuthorizationClaimDetected,
        TrialReplayRestoreVerificationMismatch::ReplayRepairClaimDetected,
        TrialReplayRestoreVerificationMismatch::RecoveryPromotionClaimDetected,
    ] {
        assert!(
            projection.mismatches.contains(&expected),
            "missing {expected:?}"
        );
    }
    let mut sorted = projection.mismatches.clone();
    sorted.sort();
    assert_eq!(projection.mismatches, sorted);
}

fn phase_166_valid_execution_state() -> LocalOperatorShellState {
    let state = phase_158_approved_decision_state();
    let request = LocalCandidateMaterializationRequest::from_validated_state(&state).unwrap();
    let mut state = materialize_local_candidate_output(&state, request).unwrap();
    let mut package = derive_controlled_internal_trial_package(
        &state,
        phase_161_trial_scope(),
        vec![phase_161_trial_operator()],
        vec![phase_161_trial_participant()],
        required_controlled_internal_trial_stop_conditions(),
    );
    package.metadata.package_status =
        ControlledInternalTrialPackageStatus::PackageReadBackValidated;
    state.controlled_internal_trial_package_projection =
        validate_controlled_internal_trial_package_read_back(&package);
    state.trial_failure_drill = derive_trial_failure_drill_projection(&state);
    state.trial_operator_runbook = derive_trial_operator_runbook_projection(&state);
    let mut evidence = derive_trial_session_evidence_record(&state);
    evidence.metadata.evidence_status = TrialSessionEvidenceStatus::EvidenceReadBackValidated;
    state.trial_session_evidence_projection = validate_trial_session_evidence_read_back(&evidence);
    state.trial_replay_restore_verification = derive_trial_replay_restore_verification_projection(
        &state,
        Some(&package),
        Some(&evidence),
    );
    state.controlled_internal_trial_execution =
        derive_controlled_internal_trial_execution_projection(&state);
    state
}

#[test]
fn phase_166_initial_harness_projection_is_not_started() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.controlled_internal_trial_execution.status,
        ControlledInternalTrialRunStatus::NotStarted
    );
    let initial = initial_controlled_internal_trial_execution_projection();
    assert_eq!(initial.status, ControlledInternalTrialRunStatus::NotStarted);
    assert!(initial.boundary_statuses.contains(
        &ControlledInternalTrialExecutionBoundaryStatus::ControlledInternalTrialHarnessOnly
    ));
    assert!(!initial.capability_surface.approves_controlled_human_use);
    assert!(!initial.capability_surface.authorizes_actions);
}

#[test]
fn phase_166_valid_preconditions_allow_bounded_trial_run_and_step_completion() {
    let state = phase_166_valid_execution_state();
    assert_eq!(
        state.controlled_internal_trial_execution.status,
        ControlledInternalTrialRunStatus::ReadyForBoundedLocalTrialRun
    );
    let next = start_controlled_internal_trial_execution(
        &state,
        ControlledInternalTrialExecutionRequest::bounded_local_trial_run(),
    );
    let run = next
        .controlled_internal_trial_execution
        .active_run
        .as_ref()
        .unwrap();
    assert_eq!(
        run.status,
        ControlledInternalTrialRunStatus::TrialRunInProgress
    );
    assert_eq!(
        run.manual_operator_step_status,
        ControlledInternalTrialManualStepStatus::ManualActionRequired
    );
    assert_eq!(
        run.stop_condition_observation.status,
        "no_stop_condition_observed"
    );
    assert!(run.evidence_linkage.trial_package.contains("package="));

    let completed = step_controlled_internal_trial_execution(
        &state,
        ControlledInternalTrialExecutionRequest::bounded_local_trial_run(),
    );
    let run = completed
        .controlled_internal_trial_execution
        .active_run
        .as_ref()
        .unwrap();
    assert_eq!(
        run.status,
        ControlledInternalTrialRunStatus::TrialRunCompletedLocally
    );
    assert_eq!(
        run.manual_operator_step_status,
        ControlledInternalTrialManualStepStatus::Recorded
    );
    assert!(
        !completed
            .controlled_internal_trial_execution
            .capability_surface
            .approves_production
    );
    assert!(
        !completed
            .controlled_internal_trial_execution
            .capability_surface
            .repairs_replay
    );
}

#[test]
fn phase_166_missing_preconditions_and_stop_condition_fail_closed() {
    let state = initial_local_operator_shell_state();
    let rejected = start_controlled_internal_trial_execution(
        &state,
        ControlledInternalTrialExecutionRequest::bounded_local_trial_run(),
    );
    assert_eq!(
        rejected.controlled_internal_trial_execution.status,
        ControlledInternalTrialRunStatus::TrialRunRejected
    );
    assert!(rejected
        .controlled_internal_trial_execution
        .rejection_reasons
        .contains(&ControlledInternalTrialRunError::TrialPackageMissing));
    assert!(rejected
        .controlled_internal_trial_execution
        .last_rejected_run
        .is_some());

    let state = phase_166_valid_execution_state();
    let mut request = ControlledInternalTrialExecutionRequest::bounded_local_trial_run();
    request.stop_condition_observed = true;
    let blocked = start_controlled_internal_trial_execution(&state, request);
    let run = blocked
        .controlled_internal_trial_execution
        .last_rejected_run
        .as_ref()
        .unwrap();
    assert_eq!(
        run.status,
        ControlledInternalTrialRunStatus::TrialRunBlocked
    );
    assert_eq!(
        run.current_blocker,
        Some(ControlledInternalTrialRunError::StopConditionObserved)
    );
    assert!(!run.stop_condition_observation.enforcement_automated);
}

#[test]
fn phase_166_trial_run_projection_and_id_are_deterministic() {
    let state = phase_166_valid_execution_state();
    let request = ControlledInternalTrialExecutionRequest::bounded_local_trial_run();
    let first = start_controlled_internal_trial_execution(&state, request.clone());
    let second = start_controlled_internal_trial_execution(&state, request);
    assert_eq!(
        first.controlled_internal_trial_execution,
        second.controlled_internal_trial_execution
    );
    assert_eq!(
        first
            .controlled_internal_trial_execution
            .active_run
            .as_ref()
            .unwrap()
            .run_id,
        second
            .controlled_internal_trial_execution
            .active_run
            .as_ref()
            .unwrap()
            .run_id
    );
    assert_eq!(state, phase_166_valid_execution_state());
}

#[test]
fn phase_166_authority_bearing_claims_are_rejected_without_losing_prior_run() {
    let state = phase_166_valid_execution_state();
    let accepted = step_controlled_internal_trial_execution(
        &state,
        ControlledInternalTrialExecutionRequest::bounded_local_trial_run(),
    );
    let mut request = ControlledInternalTrialExecutionRequest::bounded_local_trial_run();
    request.claims_readiness = true;
    request.claims_release = true;
    request.claims_deployment = true;
    request.claims_public_use = true;
    request.claims_production_use = true;
    request.claims_provider_trust = true;
    request.claims_action_authorization = true;
    request.claims_replay_repair = true;
    request.claims_recovery_promotion = true;
    request.claims_signing = true;
    request.claims_publishing = true;
    request.claims_release_artifact = true;
    let rejected = start_controlled_internal_trial_execution(&accepted, request);
    assert_eq!(
        rejected.controlled_internal_trial_execution.status,
        ControlledInternalTrialRunStatus::InvalidTrialRunRequest
    );
    assert!(rejected
        .controlled_internal_trial_execution
        .rejection_reasons
        .contains(&ControlledInternalTrialRunError::ReadinessClaimRejected));
    assert!(rejected
        .controlled_internal_trial_execution
        .rejection_reasons
        .contains(&ControlledInternalTrialRunError::ActionAuthorizationClaimRejected));
    assert!(rejected
        .controlled_internal_trial_execution
        .rejection_reasons
        .contains(&ControlledInternalTrialRunError::PublishingClaimRejected));
    assert!(rejected
        .controlled_internal_trial_execution
        .active_run
        .is_some());
    assert!(
        !rejected
            .controlled_internal_trial_execution
            .capability_surface
            .approves_release
    );
    assert!(
        !rejected
            .controlled_internal_trial_execution
            .capability_surface
            .automates_escalation
    );
}

#[test]
fn phase_167_initial_trial_observability_and_error_report_are_projected() {
    let state = initial_local_operator_shell_state();
    assert_eq!(
        state.trial_observability.status,
        TrialObservabilityStatus::ObservabilityProjected
    );
    assert!(state
        .trial_observability
        .boundary_statuses
        .contains(&TrialObservabilityBoundaryStatus::LocalObservabilityOnly));
    assert!(state
        .trial_observability
        .boundary_statuses
        .contains(&TrialObservabilityBoundaryStatus::NoProductionMonitoring));
    assert!(state
        .trial_observability
        .boundary_statuses
        .contains(&TrialObservabilityBoundaryStatus::NoRemoteTelemetry));
    assert!(state
        .trial_error_report
        .category_summary
        .contains(&TrialErrorCategory::NoTrialRun.code().to_string()));
    assert_eq!(
        state.trial_error_report.highest_severity,
        TrialErrorSeverity::Blocking
    );
}

#[test]
fn phase_167_happy_path_trial_observability_is_deterministic() {
    let ready = phase_166_valid_execution_state();
    let completed = step_controlled_internal_trial_execution(
        &ready,
        ControlledInternalTrialExecutionRequest::bounded_local_trial_run(),
    );
    assert_eq!(
        completed.trial_observability.status,
        TrialObservabilityStatus::TrialRunObserved
    );
    assert_eq!(
        completed.trial_observability.trial_run_status,
        "trial_run_completed_locally"
    );
    assert_eq!(
        completed.trial_error_report.status,
        TrialErrorReportStatus::NoErrorsObserved
    );
    assert_eq!(
        derive_trial_observability_projection(&completed),
        derive_trial_observability_projection(&completed)
    );
    assert_eq!(
        derive_trial_error_report_projection(&completed),
        derive_trial_error_report_projection(&completed)
    );
}

#[test]
fn phase_167_blocked_rejected_and_stop_condition_states_are_reported() {
    let ready = phase_166_valid_execution_state();
    let mut stop_request = ControlledInternalTrialExecutionRequest::bounded_local_trial_run();
    stop_request.stop_condition_observed = true;
    let blocked = start_controlled_internal_trial_execution(&ready, stop_request);
    assert_eq!(
        blocked.trial_observability.status,
        TrialObservabilityStatus::StopConditionObserved
    );
    assert!(blocked
        .trial_error_report
        .category_summary
        .contains(&TrialErrorCategory::StopConditionObserved.code().to_string()));
    assert!(blocked.trial_error_report.category_summary.contains(
        &TrialErrorCategory::ManualOperatorStepRequired
            .code()
            .to_string()
    ));

    let mut rejected_request = ControlledInternalTrialExecutionRequest::bounded_local_trial_run();
    rejected_request.claims_readiness = true;
    rejected_request.claims_action_authorization = true;
    let rejected = start_controlled_internal_trial_execution(&ready, rejected_request);
    assert_eq!(
        rejected.trial_observability.status,
        TrialObservabilityStatus::RejectedStateObserved
    );
    assert!(rejected.trial_error_report.category_summary.contains(
        &TrialErrorCategory::ReadinessClaimDetected
            .code()
            .to_string()
    ));
    assert!(rejected.trial_error_report.category_summary.contains(
        &TrialErrorCategory::ActionAuthorizationClaimDetected
            .code()
            .to_string()
    ));
}

#[test]
fn phase_167_verification_and_read_back_failures_are_reported() {
    let mut state = phase_166_valid_execution_state();
    state
        .controlled_internal_trial_package_projection
        .read_back_validation_status =
        Some(ControlledInternalTrialPackageValidationStatus::Invalid);
    state
        .trial_session_evidence_projection
        .read_back_validation_status = Some(TrialSessionEvidenceValidationStatus::Invalid);
    state.trial_replay_restore_verification.status =
        TrialReplayRestoreVerificationStatus::VerificationRejected;
    state.trial_replay_restore_verification.mismatches = vec![
        TrialReplayRestoreVerificationMismatch::TrialPackageReadBackInvalid,
        TrialReplayRestoreVerificationMismatch::TrialSessionEvidenceReadBackInvalid,
        TrialReplayRestoreVerificationMismatch::ReplayStatusSnapshotMismatch,
        TrialReplayRestoreVerificationMismatch::RestoreHistorySnapshotMismatch,
    ];
    state
        .trial_replay_restore_verification
        .comparison_summary
        .replay_status_comparison = "replay/status comparison rejected".to_string();
    state
        .trial_replay_restore_verification
        .comparison_summary
        .restore_history_comparison = "restore/history comparison rejected".to_string();
    let projected = refresh_trial_observability_state(state);
    assert_eq!(
        projected.trial_observability.status,
        TrialObservabilityStatus::VerificationMismatchObserved
    );
    assert!(projected
        .trial_observability
        .package_evidence_read_back_failure_summary
        .contains(&"trial_package_read_back_invalid".to_string()));
    assert!(projected
        .trial_error_report
        .category_summary
        .contains(&TrialErrorCategory::PackageReadBackFailed.code().to_string()));
    assert!(projected.trial_error_report.category_summary.contains(
        &TrialErrorCategory::EvidenceReadBackFailed
            .code()
            .to_string()
    ));
    assert!(projected
        .trial_error_report
        .category_summary
        .contains(&TrialErrorCategory::ReplayStatusMismatch.code().to_string()));
    assert!(projected.trial_error_report.category_summary.contains(
        &TrialErrorCategory::RestoreHistoryMismatch
            .code()
            .to_string()
    ));
}

#[test]
fn phase_167_observability_capability_surface_has_no_authority() {
    let state = initial_local_operator_shell_state();
    let surface = &state.trial_observability.capability_surface;
    assert!(!surface.production_monitoring);
    assert!(!surface.remote_telemetry);
    assert!(!surface.network_reporting);
    assert!(!surface.background_service);
    assert!(!surface.remediates);
    assert!(!surface.escalates);
    assert!(!surface.automates_stop_conditions);
    assert!(!surface.executes_actions);
    assert!(!surface.repairs_replay);
    assert!(!surface.promotes_recovery);
    assert!(!surface.approves_readiness);
    assert!(!surface.approves_release);
    assert!(!surface.approves_deployment);
    assert!(!surface.approves_public_use);
    assert!(!surface.approves_production);
}

#[test]
fn phase_168_initial_review_projection_has_findings_and_boundaries() {
    let state = initial_local_operator_shell_state();
    let review = &state.trial_evidence_review;
    assert_eq!(
        review.status,
        TrialEvidenceReviewStatus::HardeningCandidatesProjected
    );
    assert!(review
        .findings
        .iter()
        .any(|finding| finding.category == TrialEvidenceReviewFindingCategory::TrialPackage));
    assert!(review.findings.iter().any(
        |finding| finding.category == TrialEvidenceReviewFindingCategory::TrialSessionEvidence
    ));
    assert!(review
        .findings
        .iter()
        .any(|finding| finding.category
            == TrialEvidenceReviewFindingCategory::ReplayRestoreVerification));
    assert!(review
        .findings
        .iter()
        .any(|finding| finding.category == TrialEvidenceReviewFindingCategory::TrialExecution));
    assert!(review
        .findings
        .iter()
        .any(|finding| finding.category == TrialEvidenceReviewFindingCategory::UserHelpNeeded));
    assert!(!review.hardening_candidates.is_empty());
    for required in [
        TrialEvidenceReviewBoundaryStatus::LocalEvidenceReviewOnly,
        TrialEvidenceReviewBoundaryStatus::NonPublicReview,
        TrialEvidenceReviewBoundaryStatus::ReviewNotAuthority,
        TrialEvidenceReviewBoundaryStatus::NoControlledHumanUseApproval,
        TrialEvidenceReviewBoundaryStatus::NoReadinessApproval,
        TrialEvidenceReviewBoundaryStatus::NoReleaseApproval,
        TrialEvidenceReviewBoundaryStatus::NoDeploymentApproval,
        TrialEvidenceReviewBoundaryStatus::NoPublicUseApproval,
        TrialEvidenceReviewBoundaryStatus::NoProductionApproval,
        TrialEvidenceReviewBoundaryStatus::NoProviderTrust,
        TrialEvidenceReviewBoundaryStatus::NoActionExecution,
        TrialEvidenceReviewBoundaryStatus::NoAutomatedRemediation,
        TrialEvidenceReviewBoundaryStatus::NoAutomatedEscalation,
        TrialEvidenceReviewBoundaryStatus::NoStopConditionAutomation,
        TrialEvidenceReviewBoundaryStatus::NoReplayRepair,
        TrialEvidenceReviewBoundaryStatus::NoRecoveryPromotion,
    ] {
        assert!(
            review.boundary_statuses.contains(&required),
            "missing {required:?}"
        );
    }
}

#[test]
fn phase_168_review_derives_blocked_rejected_mismatch_and_readback_findings() {
    let mut state = initial_local_operator_shell_state();
    state.controlled_internal_trial_execution.status =
        ControlledInternalTrialRunStatus::TrialRunRejected;
    state.controlled_internal_trial_execution.rejection_reasons =
        vec![ControlledInternalTrialRunError::StopConditionObserved];
    state
        .controlled_internal_trial_package_projection
        .read_back_validation_status =
        Some(ControlledInternalTrialPackageValidationStatus::Invalid);
    state
        .trial_session_evidence_projection
        .read_back_validation_status = Some(TrialSessionEvidenceValidationStatus::Invalid);
    state.trial_replay_restore_verification.status =
        TrialReplayRestoreVerificationStatus::VerificationRejected;
    state.trial_replay_restore_verification.mismatches = vec![
        TrialReplayRestoreVerificationMismatch::TrialPackageReadBackInvalid,
        TrialReplayRestoreVerificationMismatch::TrialSessionEvidenceReadBackInvalid,
        TrialReplayRestoreVerificationMismatch::ReplayStatusSnapshotMismatch,
        TrialReplayRestoreVerificationMismatch::RestoreHistorySnapshotMismatch,
    ];
    state
        .trial_replay_restore_verification
        .comparison_summary
        .replay_status_comparison = "replay/status comparison rejected".to_string();
    state
        .trial_replay_restore_verification
        .comparison_summary
        .restore_history_comparison = "restore/history comparison rejected".to_string();
    let observed = refresh_trial_observability_state(state);
    let review = derive_trial_evidence_review_projection(&observed);
    assert_eq!(review.status, TrialEvidenceReviewStatus::ReviewRejected);
    assert!(review
        .findings
        .iter()
        .any(|finding| finding.category == TrialEvidenceReviewFindingCategory::ReplayStatus));
    assert!(review
        .findings
        .iter()
        .any(|finding| finding.category == TrialEvidenceReviewFindingCategory::RestoreHistory));
    assert!(review
        .findings
        .iter()
        .any(|finding| finding.summary.contains("read-back failed")));
    assert!(review.mismatch_error_summary.mismatch_count >= 4);
    assert!(!review.unresolved_blockers.blockers.is_empty());
}

#[test]
fn phase_168_review_stop_condition_and_hardening_candidates_are_deterministic() {
    let state = initial_local_operator_shell_state();
    let stopped = start_controlled_internal_trial_execution(
        &state,
        ControlledInternalTrialExecutionRequest {
            stop_condition_observed: true,
            ..ControlledInternalTrialExecutionRequest::bounded_local_trial_run()
        },
    );
    let first = derive_trial_evidence_review_projection(&stopped);
    let second = derive_trial_evidence_review_projection(&stopped);
    assert_eq!(first, second);
    assert!(first
        .findings
        .iter()
        .any(|finding| finding.category == TrialEvidenceReviewFindingCategory::StopCondition));
    let finding_ids = first
        .findings
        .iter()
        .map(|finding| finding.finding_id.clone())
        .collect::<Vec<_>>();
    let mut sorted_finding_ids = finding_ids.clone();
    sorted_finding_ids.sort();
    assert_eq!(finding_ids, sorted_finding_ids);
    let candidate_ids = first
        .hardening_candidates
        .iter()
        .map(|candidate| candidate.candidate_id.clone())
        .collect::<Vec<_>>();
    let mut sorted_candidate_ids = candidate_ids.clone();
    sorted_candidate_ids.sort();
    assert_eq!(candidate_ids, sorted_candidate_ids);
}

#[test]
fn phase_168_review_projection_does_not_mutate_state_or_grant_authority() {
    let state = initial_local_operator_shell_state();
    let before = state.clone();
    let review = derive_trial_evidence_review_projection(&state);
    assert_eq!(state, before);
    let surface = review.capability_surface;
    assert!(surface.local_only);
    assert!(surface.non_public);
    assert!(!surface.authoritative);
    assert!(!surface.approves_controlled_human_use);
    assert!(!surface.approves_readiness);
    assert!(!surface.approves_release);
    assert!(!surface.approves_deployment);
    assert!(!surface.approves_public_use);
    assert!(!surface.approves_production);
    assert!(!surface.trusts_provider_output);
    assert!(!surface.executes_actions);
    assert!(!surface.automates_remediation);
    assert!(!surface.automates_escalation);
    assert!(!surface.automates_stop_conditions);
    assert!(!surface.repairs_replay);
    assert!(!surface.promotes_recovery);
}
