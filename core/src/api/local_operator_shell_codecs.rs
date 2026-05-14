//! Trial package and trial session evidence codecs for the local operator shell.

use super::*;

pub(crate) fn controlled_internal_trial_package_content_basis(
    payload: &ControlledInternalTrialPackagePayload,
    markers: &ControlledInternalTrialPackageAbsenceMarkers,
) -> String {
    format!(
        "version={}|classification=controlled_internal_trial_package_only|production=non_production|distribution=local_only_non_public|payload={:?}|absence={:?}",
        CONTROLLED_INTERNAL_TRIAL_PACKAGE_VERSION, payload, markers
    )
}

pub(crate) fn stable_controlled_internal_trial_package_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub fn serialize_controlled_internal_trial_package(
    package: &ControlledInternalTrialPackage,
) -> Result<String, Vec<ControlledInternalTrialPackageValidationError>> {
    validate_controlled_internal_trial_package(package)?;
    let scope =
        package.payload.trial_scope.as_ref().ok_or_else(|| {
            vec![ControlledInternalTrialPackageValidationError::MissingTrialScope]
        })?;
    let lines = vec![
        "ajentic_controlled_internal_trial_package=v1".to_string(),
        format!("package_id={}", package.metadata.package_id),
        format!("package_version={}", package.metadata.package_version),
        format!("package_classification={}", package.metadata.package_classification),
        format!("production_classification={}", package.metadata.production_classification),
        format!("distribution_classification={}", package.metadata.distribution_classification),
        format!("package_status={}", package.metadata.package_status.code()),
        format!("content_digest={}", package.metadata.content_digest),
        format!("scope_id={}", hex_encode(&scope.scope_id)),
        format!("scope_summary={}", hex_encode(&scope.scope_summary)),
        format!("scope_local_beta_workflow={}", hex_encode(&scope.local_beta_workflow_scope)),
        format!("operators={}", hex_encode(&package.payload.named_internal_operators.iter().map(|operator| format!("{}|{}|{}", operator.operator_id, operator.display_label, operator.role)).collect::<Vec<_>>().join(";;"))),
        format!("participants={}", hex_encode(&package.payload.trial_participants.iter().map(|participant| format!("{}|{}|{}", participant.participant_id, participant.display_label, participant.role)).collect::<Vec<_>>().join(";;"))),
        format!("stop_conditions={}", package.payload.stop_conditions.iter().map(|condition| condition.code()).collect::<Vec<_>>().join(",")),
        format!("included_evidence={}", hex_encode(&[package.payload.included_evidence.local_beta_workflow_status.as_str(), package.payload.included_evidence.provider_output_pipeline_status.as_str(), package.payload.included_evidence.local_candidate_materialization_status.as_str(), package.payload.included_evidence.operator_decision_status.as_str(), package.payload.included_evidence.replay_status_summary.as_str(), package.payload.included_evidence.local_evidence_export_summary.as_str(), package.payload.included_evidence.local_session_package_summary.as_str(), package.payload.included_evidence.restore_history_summary.as_str(), package.payload.included_evidence.phase_160_gate_decision_context.as_str()].join(";;"))),
        format!("absence_markers={}", hex_encode(&format!("{:?}", package.absence_markers))),
        "local_only_non_public_note=Controlled internal trial package is local-only and non-public.".to_string(),
        "release_boundary_note=This package is not a release artifact.".to_string(),
        "deployment_readiness_boundary_note=This package is not deployment or readiness evidence.".to_string(),
        "public_production_boundary_note=This package does not approve public/general use or production use.".to_string(),
        "stop_condition_note=Stop conditions are trial metadata; they do not automate enforcement in Phase 161.".to_string(),
    ];
    Ok(format!("{}\n", lines.join("\n")))
}

fn parse_controlled_internal_trial_package_status(
    code: &str,
) -> Result<ControlledInternalTrialPackageStatus, ControlledInternalTrialPackageValidationError> {
    match code {
        "package_projected" => Ok(ControlledInternalTrialPackageStatus::PackageProjected),
        "package_validated" => Ok(ControlledInternalTrialPackageStatus::PackageValidated),
        "package_written" => Ok(ControlledInternalTrialPackageStatus::PackageWritten),
        "package_read_back_validated" => {
            Ok(ControlledInternalTrialPackageStatus::PackageReadBackValidated)
        }
        "package_rejected" => Ok(ControlledInternalTrialPackageStatus::PackageRejected),
        "invalid_package_input" => Ok(ControlledInternalTrialPackageStatus::InvalidPackageInput),
        "not_packaged" => Ok(ControlledInternalTrialPackageStatus::NotPackaged),
        _ => Err(ControlledInternalTrialPackageValidationError::MalformedPackageInput),
    }
}

fn parse_stop_condition(
    code: &str,
) -> Result<ControlledInternalTrialStopCondition, ControlledInternalTrialPackageValidationError> {
    match code {
        "stop_on_provider_trust_claim" => {
            Ok(ControlledInternalTrialStopCondition::StopOnProviderTrustClaim)
        }
        "stop_on_readiness_claim" => Ok(ControlledInternalTrialStopCondition::StopOnReadinessClaim),
        "stop_on_release_or_deployment_claim" => {
            Ok(ControlledInternalTrialStopCondition::StopOnReleaseOrDeploymentClaim)
        }
        "stop_on_public_use_claim" => {
            Ok(ControlledInternalTrialStopCondition::StopOnPublicUseClaim)
        }
        "stop_on_action_authorization_claim" => {
            Ok(ControlledInternalTrialStopCondition::StopOnActionAuthorizationClaim)
        }
        "stop_on_replay_repair_or_recovery_promotion_claim" => {
            Ok(ControlledInternalTrialStopCondition::StopOnReplayRepairOrRecoveryPromotionClaim)
        }
        "stop_on_package_validation_failure" => {
            Ok(ControlledInternalTrialStopCondition::StopOnPackageValidationFailure)
        }
        "stop_on_restore_validation_failure" => {
            Ok(ControlledInternalTrialStopCondition::StopOnRestoreValidationFailure)
        }
        "stop_on_workflow_rejection" => {
            Ok(ControlledInternalTrialStopCondition::StopOnWorkflowRejection)
        }
        "stop_on_operator_escalation" => {
            Ok(ControlledInternalTrialStopCondition::StopOnOperatorEscalation)
        }
        _ => Err(ControlledInternalTrialPackageValidationError::MalformedPackageInput),
    }
}

pub fn parse_controlled_internal_trial_package(
    content: &str,
) -> Result<ControlledInternalTrialPackage, Vec<ControlledInternalTrialPackageValidationError>> {
    let mut values = std::collections::BTreeMap::new();
    for line in content.lines() {
        let Some((key, value)) = line.split_once('=') else {
            return Err(vec![
                ControlledInternalTrialPackageValidationError::MalformedPackageInput,
            ]);
        };
        values.insert(key.to_string(), value.to_string());
    }
    if values
        .get("ajentic_controlled_internal_trial_package")
        .map(String::as_str)
        != Some("v1")
    {
        return Err(vec![
            ControlledInternalTrialPackageValidationError::MalformedPackageInput,
        ]);
    }
    let get = |key: &str| {
        values
            .get(key)
            .cloned()
            .ok_or(ControlledInternalTrialPackageValidationError::MalformedPackageInput)
    };
    let decode = |key: &str| {
        get(key).and_then(|value| {
            hex_decode(&value)
                .map_err(|_| ControlledInternalTrialPackageValidationError::MalformedPackageInput)
        })
    };
    let stop_conditions = get("stop_conditions")
        .map_err(|e| vec![e])?
        .split(',')
        .map(parse_stop_condition)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| vec![e])?;
    let operators_text = decode("operators").map_err(|e| vec![e])?;
    let named_internal_operators = operators_text
        .split(";;")
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let parts = entry.split('|').collect::<Vec<_>>();
            if parts.len() != 3 {
                return Err(ControlledInternalTrialPackageValidationError::MalformedPackageInput);
            }
            Ok(ControlledInternalTrialOperator {
                operator_id: parts[0].to_string(),
                display_label: parts[1].to_string(),
                role: parts[2].to_string(),
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| vec![e])?;
    let participants_text = decode("participants").map_err(|e| vec![e])?;
    let trial_participants = participants_text
        .split(";;")
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let parts = entry.split('|').collect::<Vec<_>>();
            if parts.len() != 3 {
                return Err(ControlledInternalTrialPackageValidationError::MalformedPackageInput);
            }
            Ok(ControlledInternalTrialParticipant {
                participant_id: parts[0].to_string(),
                display_label: parts[1].to_string(),
                role: parts[2].to_string(),
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| vec![e])?;
    let evidence_text = decode("included_evidence").map_err(|e| vec![e])?;
    let evidence_parts = evidence_text.split(";;").collect::<Vec<_>>();
    if evidence_parts.len() != 9 {
        return Err(vec![
            ControlledInternalTrialPackageValidationError::MalformedPackageInput,
        ]);
    }
    let included_evidence = ControlledInternalTrialIncludedEvidence {
        local_beta_workflow_status: evidence_parts[0].to_string(),
        provider_output_pipeline_status: evidence_parts[1].to_string(),
        local_candidate_materialization_status: evidence_parts[2].to_string(),
        operator_decision_status: evidence_parts[3].to_string(),
        replay_status_summary: evidence_parts[4].to_string(),
        local_evidence_export_summary: evidence_parts[5].to_string(),
        local_session_package_summary: evidence_parts[6].to_string(),
        restore_history_summary: evidence_parts[7].to_string(),
        phase_160_gate_decision_context: evidence_parts[8].to_string(),
    };
    let package = ControlledInternalTrialPackage {
        metadata: ControlledInternalTrialPackageMetadata {
            package_id: get("package_id").map_err(|e| vec![e])?,
            package_version: get("package_version").map_err(|e| vec![e])?,
            package_classification: get("package_classification").map_err(|e| vec![e])?,
            production_classification: get("production_classification").map_err(|e| vec![e])?,
            distribution_classification: get("distribution_classification").map_err(|e| vec![e])?,
            package_status: parse_controlled_internal_trial_package_status(
                &get("package_status").map_err(|e| vec![e])?,
            )
            .map_err(|e| vec![e])?,
            validation_status: ControlledInternalTrialPackageValidationStatus::Valid,
            content_digest: get("content_digest").map_err(|e| vec![e])?,
        },
        payload: ControlledInternalTrialPackagePayload {
            trial_scope: Some(ControlledInternalTrialScope {
                scope_id: decode("scope_id").map_err(|e| vec![e])?,
                scope_summary: decode("scope_summary").map_err(|e| vec![e])?,
                local_beta_workflow_scope: decode("scope_local_beta_workflow")
                    .map_err(|e| vec![e])?,
            }),
            named_internal_operators,
            trial_participants,
            stop_conditions,
            included_evidence,
            boundary_status: controlled_internal_trial_package_boundary_statuses(),
            no_release_marker: "not a release artifact".to_string(),
            no_deployment_marker: "not deployment evidence".to_string(),
            no_readiness_marker: "not readiness evidence".to_string(),
            no_public_use_marker: "does not approve public/general use".to_string(),
            no_production_use_marker: "does not approve production use".to_string(),
            no_provider_trust_marker: "no provider trust".to_string(),
            no_action_authorization_marker: "no action authorization".to_string(),
            no_replay_repair_marker: "no replay repair".to_string(),
            no_recovery_promotion_marker: "no recovery promotion".to_string(),
        },
        absence_markers: controlled_internal_trial_package_absence_markers(),
    };
    validate_controlled_internal_trial_package(&package)?;
    Ok(package)
}

pub fn write_controlled_internal_trial_package(
    package: &ControlledInternalTrialPackage,
    caller_provided_path: &std::path::Path,
) -> Result<
    ControlledInternalTrialPackageWriteResult,
    Vec<ControlledInternalTrialPackageValidationError>,
> {
    let content = serialize_controlled_internal_trial_package(package)?;
    let write_package = trial_standard::fs::/* local trial package file boundary */write;
    write_package(caller_provided_path, content.as_bytes())
        .map_err(|_| vec![ControlledInternalTrialPackageValidationError::MalformedPackageInput])?;
    let mut written = package.clone();
    written.metadata.package_status = ControlledInternalTrialPackageStatus::PackageWritten;
    Ok(ControlledInternalTrialPackageWriteResult {
        status: ControlledInternalTrialPackageStatus::PackageWritten,
        path: caller_provided_path.display().to_string(),
        bytes_written: content.len(),
        projection: project_controlled_internal_trial_package_status(Some(&written), None),
    })
}

pub fn read_controlled_internal_trial_package(
    caller_provided_path: &std::path::Path,
) -> Result<
    ControlledInternalTrialPackageReadResult,
    Vec<ControlledInternalTrialPackageValidationError>,
> {
    let read_package = trial_standard::fs::/* local trial package file boundary */read;
    let content =
        String::from_utf8(read_package(caller_provided_path).map_err(|_| {
            vec![ControlledInternalTrialPackageValidationError::MalformedPackageInput]
        })?)
        .map_err(|_| vec![ControlledInternalTrialPackageValidationError::MalformedPackageInput])?;
    let package = parse_controlled_internal_trial_package(&content)?;
    let projection = validate_controlled_internal_trial_package_read_back(&package);
    Ok(ControlledInternalTrialPackageReadResult {
        status: ControlledInternalTrialPackageStatus::PackageReadBackValidated,
        path: caller_provided_path.display().to_string(),
        package: Some(package),
        projection,
    })
}

pub fn validate_controlled_internal_trial_package_read_back(
    package: &ControlledInternalTrialPackage,
) -> ControlledInternalTrialPackageProjection {
    project_controlled_internal_trial_package_status(
        Some(package),
        Some(ControlledInternalTrialPackageValidationStatus::Valid),
    )
}

pub(crate) fn stable_trial_session_evidence_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub(crate) fn trial_session_evidence_payload_basis(
    payload: &TrialSessionEvidencePayload,
    markers: &TrialSessionEvidenceAbsenceMarkers,
) -> String {
    format!(
        "version={}|classification=trial_session_evidence_only|production=non_production|distribution=local_only_non_public|authority=non_authoritative_evidence|payload={:?}|absence={:?}",
        TRIAL_SESSION_EVIDENCE_VERSION, payload, markers
    )
}

fn trial_session_evidence_decode(
    input: &str,
) -> Result<String, TrialSessionEvidenceValidationError> {
    if !input.len().is_multiple_of(2) {
        return Err(TrialSessionEvidenceValidationError::MalformedEvidenceInput);
    }
    let mut bytes = Vec::with_capacity(input.len() / 2);
    for index in (0..input.len()).step_by(2) {
        let byte = u8::from_str_radix(&input[index..index + 2], 16)
            .map_err(|_| TrialSessionEvidenceValidationError::MalformedEvidenceInput)?;
        bytes.push(byte);
    }
    String::from_utf8(bytes)
        .map_err(|_| TrialSessionEvidenceValidationError::MalformedEvidenceInput)
}

fn split_encoded_list(input: String) -> Vec<String> {
    input
        .split(";;")
        .filter(|entry| !entry.is_empty())
        .map(str::to_string)
        .collect()
}

pub fn serialize_trial_session_evidence_record(
    record: &TrialSessionEvidenceRecord,
) -> Result<String, Vec<TrialSessionEvidenceValidationError>> {
    validate_trial_session_evidence_record(record)?;
    let lines = vec![
        "ajentic_trial_session_evidence=v1".to_string(),
        format!("evidence_id={}", record.metadata.evidence_id),
        format!("evidence_version={}", record.metadata.evidence_version),
        format!("evidence_classification={}", record.metadata.evidence_classification),
        format!("production_classification={}", record.metadata.production_classification),
        format!("distribution_classification={}", record.metadata.distribution_classification),
        format!("authority_classification={}", record.metadata.authority_classification),
        format!("evidence_status={}", record.metadata.evidence_status.code()),
        format!("content_digest={}", record.metadata.content_digest),
        format!("trial_package_id={}", hex_encode(&record.payload.trial_package_id)),
        format!("trial_package_version={}", hex_encode(&record.payload.trial_package_version)),
        format!("trial_package_status={}", record.payload.trial_package_status),
        format!("trial_package_validation_status={}", record.payload.trial_package_validation_status),
        format!("trial_package_read_back_status={}", record.payload.trial_package_read_back_status),
        format!("runbook_status={}", record.payload.runbook_status),
        format!("runbook_current_step={}", record.payload.runbook_current_step),
        format!("runbook_current_blocker={}", hex_encode(&record.payload.runbook_current_blocker)),
        format!("failure_drill_status={}", record.payload.failure_drill_status),
        format!("failure_drill_highest_severity={}", record.payload.failure_drill_highest_severity),
        format!("workflow_status_snapshot={}", record.payload.workflow_status_snapshot),
        format!("local_candidate_materialization_snapshot={}", record.payload.local_candidate_materialization_snapshot),
        format!("provider_output_pipeline_snapshot={}", record.payload.provider_output_pipeline_snapshot),
        format!("operator_decision_snapshot={}", record.payload.operator_decision_snapshot),
        format!("replay_status_snapshot={}", hex_encode(&record.payload.replay_status_snapshot)),
        format!("local_evidence_export_snapshot={}", hex_encode(&record.payload.local_evidence_export_snapshot)),
        format!("local_session_package_snapshot={}", record.payload.local_session_package_snapshot),
        format!("restore_history_snapshot={}", hex_encode(&record.payload.restore_history_snapshot)),
        format!("stop_condition_snapshot={}", hex_encode(&record.payload.stop_condition_snapshot.join(";;"))),
        format!("escalation_guidance_snapshot={}", hex_encode(&record.payload.escalation_guidance_snapshot.join(";;"))),
        format!("failure_category_snapshot={}", hex_encode(&record.payload.failure_category_snapshot.join(";;"))),
        format!("current_blocker_snapshot={}", hex_encode(&record.payload.current_blocker_snapshot)),
        format!("boundary_status={}", record.payload.boundary_status.iter().map(|status| status.code()).collect::<Vec<_>>().join(",")),
        format!("no_approval_no_authority_markers={}", record.payload.no_approval_no_authority_markers.join(",")),
        format!("absence_markers={}", hex_encode(&format!("{:?}", record.absence_markers))),
        "local_only_non_authoritative_note=Trial session evidence is local-only, non-public, and non-authoritative.".to_string(),
        "local_preparation_only_note=Evidence capture records local trial-preparation state only.".to_string(),
        "no_trial_approval_note=Evidence capture does not start or approve a controlled trial.".to_string(),
        "no_release_deployment_readiness_note=This evidence is not release, deployment, readiness, public-use, or production-use evidence.".to_string(),
        "read_back_validation_note=Read-back validation checks evidence structure only.".to_string(),
    ];
    Ok(format!("{}\n", lines.join("\n")))
}

fn parse_trial_session_evidence_status(
    code: &str,
) -> Result<TrialSessionEvidenceStatus, TrialSessionEvidenceValidationError> {
    match code {
        "evidence_projected" => Ok(TrialSessionEvidenceStatus::EvidenceProjected),
        "evidence_validated" => Ok(TrialSessionEvidenceStatus::EvidenceValidated),
        "evidence_written" => Ok(TrialSessionEvidenceStatus::EvidenceWritten),
        "evidence_read_back_validated" => Ok(TrialSessionEvidenceStatus::EvidenceReadBackValidated),
        "evidence_rejected" => Ok(TrialSessionEvidenceStatus::EvidenceRejected),
        "invalid_evidence_input" => Ok(TrialSessionEvidenceStatus::InvalidEvidenceInput),
        "not_captured" => Ok(TrialSessionEvidenceStatus::NotCaptured),
        _ => Err(TrialSessionEvidenceValidationError::MalformedEvidenceInput),
    }
}

pub fn parse_trial_session_evidence_record(
    content: &str,
) -> Result<TrialSessionEvidenceRecord, Vec<TrialSessionEvidenceValidationError>> {
    let mut values = std::collections::BTreeMap::new();
    for line in content.lines() {
        let Some((key, value)) = line.split_once('=') else {
            return Err(vec![
                TrialSessionEvidenceValidationError::MalformedEvidenceInput,
            ]);
        };
        values.insert(key.to_string(), value.to_string());
    }
    if values
        .get("ajentic_trial_session_evidence")
        .map(String::as_str)
        != Some("v1")
    {
        return Err(vec![
            TrialSessionEvidenceValidationError::MalformedEvidenceInput,
        ]);
    }
    let get = |key: &str| {
        values
            .get(key)
            .cloned()
            .ok_or(TrialSessionEvidenceValidationError::MalformedEvidenceInput)
    };
    let decode = |key: &str| get(key).and_then(|value| trial_session_evidence_decode(&value));
    let record = TrialSessionEvidenceRecord {
        metadata: TrialSessionEvidenceMetadata {
            evidence_id: get("evidence_id").map_err(|e| vec![e])?,
            evidence_version: get("evidence_version").map_err(|e| vec![e])?,
            evidence_classification: get("evidence_classification").map_err(|e| vec![e])?,
            production_classification: get("production_classification").map_err(|e| vec![e])?,
            distribution_classification: get("distribution_classification").map_err(|e| vec![e])?,
            authority_classification: get("authority_classification").map_err(|e| vec![e])?,
            evidence_status: parse_trial_session_evidence_status(
                &get("evidence_status").map_err(|e| vec![e])?,
            )
            .map_err(|e| vec![e])?,
            validation_status: TrialSessionEvidenceValidationStatus::Valid,
            content_digest: get("content_digest").map_err(|e| vec![e])?,
        },
        payload: TrialSessionEvidencePayload {
            trial_package_id: decode("trial_package_id").map_err(|e| vec![e])?,
            trial_package_version: decode("trial_package_version").map_err(|e| vec![e])?,
            trial_package_status: get("trial_package_status").map_err(|e| vec![e])?,
            trial_package_validation_status: get("trial_package_validation_status")
                .map_err(|e| vec![e])?,
            trial_package_read_back_status: get("trial_package_read_back_status")
                .map_err(|e| vec![e])?,
            runbook_status: get("runbook_status").map_err(|e| vec![e])?,
            runbook_current_step: get("runbook_current_step").map_err(|e| vec![e])?,
            runbook_current_blocker: decode("runbook_current_blocker").map_err(|e| vec![e])?,
            failure_drill_status: get("failure_drill_status").map_err(|e| vec![e])?,
            failure_drill_highest_severity: get("failure_drill_highest_severity")
                .map_err(|e| vec![e])?,
            workflow_status_snapshot: get("workflow_status_snapshot").map_err(|e| vec![e])?,
            local_candidate_materialization_snapshot: get(
                "local_candidate_materialization_snapshot",
            )
            .map_err(|e| vec![e])?,
            provider_output_pipeline_snapshot: get("provider_output_pipeline_snapshot")
                .map_err(|e| vec![e])?,
            operator_decision_snapshot: get("operator_decision_snapshot").map_err(|e| vec![e])?,
            replay_status_snapshot: decode("replay_status_snapshot").map_err(|e| vec![e])?,
            local_evidence_export_snapshot: decode("local_evidence_export_snapshot")
                .map_err(|e| vec![e])?,
            local_session_package_snapshot: get("local_session_package_snapshot")
                .map_err(|e| vec![e])?,
            restore_history_snapshot: decode("restore_history_snapshot").map_err(|e| vec![e])?,
            stop_condition_snapshot: split_encoded_list(
                decode("stop_condition_snapshot").map_err(|e| vec![e])?,
            ),
            escalation_guidance_snapshot: split_encoded_list(
                decode("escalation_guidance_snapshot").map_err(|e| vec![e])?,
            ),
            failure_category_snapshot: split_encoded_list(
                decode("failure_category_snapshot").map_err(|e| vec![e])?,
            ),
            current_blocker_snapshot: decode("current_blocker_snapshot").map_err(|e| vec![e])?,
            boundary_status: trial_session_evidence_boundary_statuses(),
            no_approval_no_authority_markers: get("no_approval_no_authority_markers")
                .map_err(|e| vec![e])?
                .split(',')
                .filter(|entry| !entry.is_empty())
                .map(str::to_string)
                .collect(),
        },
        absence_markers: trial_session_evidence_absence_markers(),
    };
    validate_trial_session_evidence_record(&record)?;
    Ok(record)
}

pub fn validate_trial_session_evidence_read_back(
    record: &TrialSessionEvidenceRecord,
) -> TrialSessionEvidenceProjection {
    project_trial_session_evidence_status(
        Some(record),
        Some(TrialSessionEvidenceValidationStatus::Valid),
    )
}
