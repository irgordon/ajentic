use super::{
    DurableAppendReport, EndToEndLocalHarnessReport, OperatorActionExecutionReport,
    ProviderEvidenceReplayReport, RecoveryAcceptanceReport,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObservabilitySnapshotMode {
    SuppliedEvidenceSnapshot,
    CurrentInMemorySnapshotUnsupported,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObservabilitySnapshotStatus {
    Built,
    Rejected,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
pub struct ObservedActionSummary {
    pub action_kind: String,
    pub action_reason_code: String,
    pub action_real_world_effect: bool,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ObservedDiagnosticSummary {
    pub family: String,
    pub code: String,
    pub key: String,
    pub summary: String,
}

#[derive(Clone, PartialEq, Eq)]
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
            status_code: harness_status_code(report.status).to_string(),
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
            status_code: durable_append_status_code(report.status).to_string(),
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
            status_code: recovery_status_code(report.status).to_string(),
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
            status_code: replay_status_code(report.status).to_string(),
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
            action_kind: operator_action_kind_code(report.action_kind).to_string(),
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

fn snapshot_mode_code(mode: ObservabilitySnapshotMode) -> &'static str {
    match mode {
        ObservabilitySnapshotMode::SuppliedEvidenceSnapshot => "supplied_evidence_snapshot",
        ObservabilitySnapshotMode::CurrentInMemorySnapshotUnsupported => {
            "current_in_memory_snapshot_unsupported"
        }
    }
}

fn snapshot_status_code(status: ObservabilitySnapshotStatus) -> &'static str {
    match status {
        ObservabilitySnapshotStatus::Built => "built",
        ObservabilitySnapshotStatus::Rejected => "rejected",
    }
}

fn harness_status_code(status: super::EndToEndLocalHarnessStatus) -> &'static str {
    match status {
        super::EndToEndLocalHarnessStatus::Completed => "completed",
        super::EndToEndLocalHarnessStatus::Rejected => "rejected",
    }
}

fn durable_append_status_code(status: super::DurableAppendStatus) -> &'static str {
    match status {
        super::DurableAppendStatus::Prepared => "prepared",
        super::DurableAppendStatus::Written => "written",
        super::DurableAppendStatus::Verified => "verified",
        super::DurableAppendStatus::Rejected => "rejected",
    }
}

fn recovery_status_code(status: super::RecoveryAcceptanceStatus) -> &'static str {
    match status {
        super::RecoveryAcceptanceStatus::Accepted => "accepted",
        super::RecoveryAcceptanceStatus::Rejected => "rejected",
    }
}

fn replay_status_code(status: super::ProviderEvidenceReplayStatus) -> &'static str {
    match status {
        super::ProviderEvidenceReplayStatus::Verified => "verified",
        super::ProviderEvidenceReplayStatus::Rejected => "rejected",
        super::ProviderEvidenceReplayStatus::Mismatch => "mismatch",
    }
}

fn operator_action_kind_code(kind: super::OperatorActionKind) -> &'static str {
    match kind {
        super::OperatorActionKind::RecordExecutionDecision => "record_execution_decision",
        super::OperatorActionKind::PersistLedgerRecord => "persist_ledger_record",
        super::OperatorActionKind::ExecuteProvider => "execute_provider",
        super::OperatorActionKind::RepairReplay => "repair_replay",
        super::OperatorActionKind::MutateApplicationState => "mutate_application_state",
        super::OperatorActionKind::Unknown => "unknown",
    }
}

pub const AUDIT_EXPORT_FORMAT_VERSION: &str = "audit-export-v1";
pub const AUDIT_EXPORT_RECORD_KIND: &str = "observability-snapshot";
const NONE_FIELD: &str = "none";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AuditExportEncodingStatus {
    Encoded,
    Rejected,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AuditExportEncodingReason {
    EncodedCanonicalSnapshot,
    EmptyExportId,
    UnsupportedSnapshotMode,
    SnapshotNotBuilt,
    TooManyDiagnostics,
    FieldTooLarge,
    SummaryTooLarge,
    EncodedSizeLimitExceeded,
    RawPayloadRejected,
    SecretMaterialRejected,
    NonCanonicalInputRejected,
    ExportWriteNotAllowed,
    PersistenceNotAllowed,
    AuthorityMutationNotAllowed,
}

impl AuditExportEncodingReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EncodedCanonicalSnapshot => "encoded_canonical_snapshot",
            Self::EmptyExportId => "empty_export_id",
            Self::UnsupportedSnapshotMode => "unsupported_snapshot_mode",
            Self::SnapshotNotBuilt => "snapshot_not_built",
            Self::TooManyDiagnostics => "too_many_diagnostics",
            Self::FieldTooLarge => "field_too_large",
            Self::SummaryTooLarge => "summary_too_large",
            Self::EncodedSizeLimitExceeded => "encoded_size_limit_exceeded",
            Self::RawPayloadRejected => "raw_payload_rejected",
            Self::SecretMaterialRejected => "secret_material_rejected",
            Self::NonCanonicalInputRejected => "non_canonical_input_rejected",
            Self::ExportWriteNotAllowed => "export_write_not_allowed",
            Self::PersistenceNotAllowed => "persistence_not_allowed",
            Self::AuthorityMutationNotAllowed => "authority_mutation_not_allowed",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AuditExportEncodingLimits {
    pub max_diagnostics: usize,
    pub max_field_len: usize,
    pub max_summary_len: usize,
    pub max_total_bytes: usize,
}

impl AuditExportEncodingLimits {
    pub fn strict_defaults() -> Self {
        Self {
            max_diagnostics: 16,
            max_field_len: 256,
            max_summary_len: 512,
            max_total_bytes: 8192,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct AuditExportEnvelope {
    pub export_id: String,
    pub format_version: String,
    pub record_kind: String,
    pub encoded_bytes: Vec<u8>,
    pub byte_len: usize,
    pub writes_files: bool,
    pub reads_persistence: bool,
    pub writes_persistence: bool,
    pub mutates_authority: bool,
    pub summary: String,
}

#[derive(Clone, PartialEq, Eq)]
pub struct AuditExportEncodingReport {
    pub status: AuditExportEncodingStatus,
    pub reason: AuditExportEncodingReason,
    pub export_id: String,
    pub format_version: String,
    pub record_kind: String,
    pub byte_len: Option<usize>,
    pub writes_files: bool,
    pub reads_persistence: bool,
    pub writes_persistence: bool,
    pub mutates_authority: bool,
    pub summary: String,
}

pub fn encode_audit_export_snapshot(
    export_id: impl Into<String>,
    snapshot: &ObservabilitySnapshot,
    limits: AuditExportEncodingLimits,
) -> Result<AuditExportEnvelope, AuditExportEncodingReport> {
    let export_id = export_id.into();
    validate_audit_export_request(&export_id, snapshot, &limits)?;
    encode_audit_export_snapshot_bytes(export_id.clone(), snapshot, limits)
        .map_err(|reason| rejection_report(&export_id, reason, None))
}

fn encode_audit_export_snapshot_bytes(
    export_id: String,
    snapshot: &ObservabilitySnapshot,
    limits: AuditExportEncodingLimits,
) -> Result<AuditExportEnvelope, AuditExportEncodingReason> {
    let mut buffer = Vec::new();
    append_line(
        &mut buffer,
        "format_version",
        AUDIT_EXPORT_FORMAT_VERSION,
        &limits,
    )?;
    append_line(
        &mut buffer,
        "record_kind",
        AUDIT_EXPORT_RECORD_KIND,
        &limits,
    )?;
    append_line(&mut buffer, "export_id", &export_id, &limits)?;
    append_line(&mut buffer, "snapshot_id", &snapshot.snapshot_id, &limits)?;
    append_line(
        &mut buffer,
        "snapshot_mode",
        snapshot_mode_code(snapshot.mode),
        &limits,
    )?;
    append_line(
        &mut buffer,
        "snapshot_status",
        snapshot_status_code(snapshot.status),
        &limits,
    )?;
    append_line(
        &mut buffer,
        "snapshot_reason",
        snapshot.reason.code(),
        &limits,
    )?;
    append_harness(&mut buffer, snapshot.harness.as_ref(), &limits)?;
    append_durable_append(&mut buffer, snapshot.durable_append.as_ref(), &limits)?;
    append_recovery(&mut buffer, snapshot.recovery.as_ref(), &limits)?;
    append_replay(&mut buffer, snapshot.replay.as_ref(), &limits)?;
    append_action(&mut buffer, snapshot.action.as_ref(), &limits)?;
    append_usize(
        &mut buffer,
        "diagnostics.count",
        snapshot.diagnostics.len(),
        &limits,
    )?;
    for (index, diagnostic) in snapshot.diagnostics.iter().enumerate() {
        let prefix = diagnostic_prefix(index);
        append_line(
            &mut buffer,
            &format_key(&prefix, "family"),
            &diagnostic.family,
            &limits,
        )?;
        append_line(
            &mut buffer,
            &format_key(&prefix, "code"),
            &diagnostic.code,
            &limits,
        )?;
        append_line(
            &mut buffer,
            &format_key(&prefix, "key"),
            &diagnostic.key,
            &limits,
        )?;
        append_line(
            &mut buffer,
            &format_key(&prefix, "summary"),
            &diagnostic.summary,
            &limits,
        )?;
    }
    append_bool(
        &mut buffer,
        "contains_raw_provider_payload",
        snapshot.contains_raw_provider_payload,
        &limits,
    )?;
    append_bool(
        &mut buffer,
        "contains_secret_material",
        snapshot.contains_secret_material,
        &limits,
    )?;
    append_bool(
        &mut buffer,
        "mutates_application_state",
        snapshot.mutates_application_state,
        &limits,
    )?;
    append_bool(
        &mut buffer,
        "reads_persistence",
        snapshot.reads_persistence,
        &limits,
    )?;
    append_bool(
        &mut buffer,
        "writes_persistence",
        snapshot.writes_persistence,
        &limits,
    )?;
    append_bool(
        &mut buffer,
        "recomputes_authority",
        snapshot.recomputes_authority,
        &limits,
    )?;
    append_bool(
        &mut buffer,
        "repairs_state",
        snapshot.repairs_state,
        &limits,
    )?;
    append_bool(&mut buffer, "exports_data", snapshot.exports_data, &limits)?;
    append_line(&mut buffer, "summary", &snapshot.summary, &limits)?;

    let byte_len = buffer.len();
    Ok(AuditExportEnvelope {
        export_id,
        format_version: AUDIT_EXPORT_FORMAT_VERSION.to_string(),
        record_kind: AUDIT_EXPORT_RECORD_KIND.to_string(),
        encoded_bytes: buffer,
        byte_len,
        writes_files: false,
        reads_persistence: false,
        writes_persistence: false,
        mutates_authority: false,
        summary: "canonical audit export encoding for observability snapshot only; no file write, persistence access, or authority mutation".to_string(),
    })
}

fn validate_audit_export_request(
    export_id: &str,
    snapshot: &ObservabilitySnapshot,
    limits: &AuditExportEncodingLimits,
) -> Result<(), AuditExportEncodingReport> {
    if export_id.is_empty() {
        return Err(rejection_report(
            export_id,
            AuditExportEncodingReason::EmptyExportId,
            None,
        ));
    }
    if snapshot.mode != ObservabilitySnapshotMode::SuppliedEvidenceSnapshot {
        return Err(rejection_report(
            export_id,
            AuditExportEncodingReason::UnsupportedSnapshotMode,
            None,
        ));
    }
    if snapshot.status != ObservabilitySnapshotStatus::Built {
        return Err(rejection_report(
            export_id,
            AuditExportEncodingReason::SnapshotNotBuilt,
            None,
        ));
    }
    if snapshot.contains_raw_provider_payload {
        return Err(rejection_report(
            export_id,
            AuditExportEncodingReason::RawPayloadRejected,
            None,
        ));
    }
    if snapshot.contains_secret_material {
        return Err(rejection_report(
            export_id,
            AuditExportEncodingReason::SecretMaterialRejected,
            None,
        ));
    }
    if observability_snapshot_mutates_authority(snapshot) {
        return Err(rejection_report(
            export_id,
            AuditExportEncodingReason::AuthorityMutationNotAllowed,
            None,
        ));
    }
    if snapshot.reads_persistence || snapshot.writes_persistence {
        return Err(rejection_report(
            export_id,
            AuditExportEncodingReason::PersistenceNotAllowed,
            None,
        ));
    }
    if snapshot.diagnostics.len() > limits.max_diagnostics {
        return Err(rejection_report(
            export_id,
            AuditExportEncodingReason::TooManyDiagnostics,
            None,
        ));
    }
    if snapshot.summary.len() > limits.max_summary_len {
        return Err(rejection_report(
            export_id,
            AuditExportEncodingReason::SummaryTooLarge,
            None,
        ));
    }
    Ok(())
}

fn rejection_report(
    export_id: &str,
    reason: AuditExportEncodingReason,
    byte_len: Option<usize>,
) -> AuditExportEncodingReport {
    AuditExportEncodingReport {
        status: AuditExportEncodingStatus::Rejected,
        reason,
        export_id: export_id.to_string(),
        format_version: AUDIT_EXPORT_FORMAT_VERSION.to_string(),
        record_kind: AUDIT_EXPORT_RECORD_KIND.to_string(),
        byte_len,
        writes_files: false,
        reads_persistence: false,
        writes_persistence: false,
        mutates_authority: false,
        summary: format_rejection_summary(reason),
    }
}

fn format_rejection_summary(reason: AuditExportEncodingReason) -> String {
    let mut summary = String::from("audit export encoding rejected: ");
    summary.push_str(reason.code());
    summary
}

pub fn append_line(
    buffer: &mut Vec<u8>,
    key: &str,
    value: &str,
    limits: &AuditExportEncodingLimits,
) -> Result<(), AuditExportEncodingReason> {
    if !is_canonical_fragment(key) || !is_canonical_fragment(value) {
        return Err(AuditExportEncodingReason::NonCanonicalInputRejected);
    }
    if value.len() > limits.max_field_len {
        return Err(AuditExportEncodingReason::FieldTooLarge);
    }
    let new_len = buffer
        .len()
        .checked_add(key.len())
        .and_then(|len| len.checked_add(1))
        .and_then(|len| len.checked_add(value.len()))
        .and_then(|len| len.checked_add(1))
        .ok_or(AuditExportEncodingReason::EncodedSizeLimitExceeded)?;
    if new_len > limits.max_total_bytes {
        return Err(AuditExportEncodingReason::EncodedSizeLimitExceeded);
    }
    buffer.extend_from_slice(key.as_bytes());
    buffer.push(b'=');
    buffer.extend_from_slice(value.as_bytes());
    buffer.push(b'\n');
    Ok(())
}

pub fn append_bool(
    buffer: &mut Vec<u8>,
    key: &str,
    value: bool,
    limits: &AuditExportEncodingLimits,
) -> Result<(), AuditExportEncodingReason> {
    append_line(buffer, key, if value { "true" } else { "false" }, limits)
}

pub fn append_usize(
    buffer: &mut Vec<u8>,
    key: &str,
    value: usize,
    limits: &AuditExportEncodingLimits,
) -> Result<(), AuditExportEncodingReason> {
    append_line(buffer, key, &value.to_string(), limits)
}

pub fn append_optional_string(
    buffer: &mut Vec<u8>,
    key: &str,
    value: Option<&str>,
    limits: &AuditExportEncodingLimits,
) -> Result<(), AuditExportEncodingReason> {
    append_line(buffer, key, value.unwrap_or(NONE_FIELD), limits)
}

fn append_harness(
    buffer: &mut Vec<u8>,
    harness: Option<&ObservedHarnessSummary>,
    limits: &AuditExportEncodingLimits,
) -> Result<(), AuditExportEncodingReason> {
    append_bool(buffer, "harness.present", harness.is_some(), limits)?;
    if let Some(harness) = harness {
        append_line(buffer, "harness.run_id", &harness.run_id, limits)?;
        append_line(buffer, "harness.status_code", &harness.status_code, limits)?;
        append_line(buffer, "harness.reason_code", &harness.reason_code, limits)?;
        append_bool(
            buffer,
            "harness.provider_output_trusted",
            harness.provider_output_trusted,
            limits,
        )?;
        append_bool(
            buffer,
            "harness.provider_output_authoritative",
            harness.provider_output_authoritative,
            limits,
        )?;
        append_bool(
            buffer,
            "harness.retry_scheduled",
            harness.retry_scheduled,
            limits,
        )?;
        append_bool(
            buffer,
            "harness.recovery_candidate_only",
            harness.recovery_candidate_only,
            limits,
        )?;
        append_bool(
            buffer,
            "harness.recovered_state_promoted",
            harness.recovered_state_promoted,
            limits,
        )?;
        append_bool(
            buffer,
            "harness.ui_transport_live",
            harness.ui_transport_live,
            limits,
        )?;
        append_bool(
            buffer,
            "harness.ui_submission_executes_action",
            harness.ui_submission_executes_action,
            limits,
        )?;
        append_line(buffer, "harness.action_kind", &harness.action_kind, limits)?;
        append_bool(
            buffer,
            "harness.action_real_world_effect",
            harness.action_real_world_effect,
            limits,
        )
    } else {
        append_absent_harness(buffer, limits)
    }
}

fn append_absent_harness(
    buffer: &mut Vec<u8>,
    limits: &AuditExportEncodingLimits,
) -> Result<(), AuditExportEncodingReason> {
    append_optional_string(buffer, "harness.run_id", None, limits)?;
    append_optional_string(buffer, "harness.status_code", None, limits)?;
    append_optional_string(buffer, "harness.reason_code", None, limits)?;
    append_bool(buffer, "harness.provider_output_trusted", false, limits)?;
    append_bool(
        buffer,
        "harness.provider_output_authoritative",
        false,
        limits,
    )?;
    append_bool(buffer, "harness.retry_scheduled", false, limits)?;
    append_bool(buffer, "harness.recovery_candidate_only", false, limits)?;
    append_bool(buffer, "harness.recovered_state_promoted", false, limits)?;
    append_bool(buffer, "harness.ui_transport_live", false, limits)?;
    append_bool(
        buffer,
        "harness.ui_submission_executes_action",
        false,
        limits,
    )?;
    append_optional_string(buffer, "harness.action_kind", None, limits)?;
    append_bool(buffer, "harness.action_real_world_effect", false, limits)
}

fn append_durable_append(
    buffer: &mut Vec<u8>,
    durable_append: Option<&ObservedDurableAppendSummary>,
    limits: &AuditExportEncodingLimits,
) -> Result<(), AuditExportEncodingReason> {
    append_bool(
        buffer,
        "durable_append.present",
        durable_append.is_some(),
        limits,
    )?;
    if let Some(durable_append) = durable_append {
        append_line(
            buffer,
            "durable_append.append_transaction_id",
            &durable_append.append_transaction_id,
            limits,
        )?;
        append_line(
            buffer,
            "durable_append.status_code",
            &durable_append.status_code,
            limits,
        )?;
        append_line(
            buffer,
            "durable_append.reason_code",
            &durable_append.reason_code,
            limits,
        )?;
        append_bool(
            buffer,
            "durable_append.committed",
            durable_append.committed,
            limits,
        )?;
        append_bool(
            buffer,
            "durable_append.promoted",
            durable_append.promoted,
            limits,
        )?;
        append_bool(
            buffer,
            "durable_append.recovered_state",
            durable_append.recovered_state,
            limits,
        )?;
        append_bool(
            buffer,
            "durable_append.repaired_replay",
            durable_append.repaired_replay,
            limits,
        )?;
        append_bool(
            buffer,
            "durable_append.trusted_provider_output",
            durable_append.trusted_provider_output,
            limits,
        )?;
        append_bool(
            buffer,
            "durable_append.executed_action",
            durable_append.executed_action,
            limits,
        )?;
        append_bool(
            buffer,
            "durable_append.mutated_application_state",
            durable_append.mutated_application_state,
            limits,
        )
    } else {
        append_optional_string(buffer, "durable_append.append_transaction_id", None, limits)?;
        append_optional_string(buffer, "durable_append.status_code", None, limits)?;
        append_optional_string(buffer, "durable_append.reason_code", None, limits)?;
        append_bool(buffer, "durable_append.committed", false, limits)?;
        append_bool(buffer, "durable_append.promoted", false, limits)?;
        append_bool(buffer, "durable_append.recovered_state", false, limits)?;
        append_bool(buffer, "durable_append.repaired_replay", false, limits)?;
        append_bool(
            buffer,
            "durable_append.trusted_provider_output",
            false,
            limits,
        )?;
        append_bool(buffer, "durable_append.executed_action", false, limits)?;
        append_bool(
            buffer,
            "durable_append.mutated_application_state",
            false,
            limits,
        )
    }
}

fn append_recovery(
    buffer: &mut Vec<u8>,
    recovery: Option<&ObservedRecoverySummary>,
    limits: &AuditExportEncodingLimits,
) -> Result<(), AuditExportEncodingReason> {
    append_bool(buffer, "recovery.present", recovery.is_some(), limits)?;
    if let Some(recovery) = recovery {
        append_line(
            buffer,
            "recovery.acceptance_id",
            &recovery.acceptance_id,
            limits,
        )?;
        append_line(
            buffer,
            "recovery.status_code",
            &recovery.status_code,
            limits,
        )?;
        append_line(
            buffer,
            "recovery.reason_code",
            &recovery.reason_code,
            limits,
        )?;
        append_bool(
            buffer,
            "recovery.accepted_for_in_memory_use",
            recovery.accepted_for_in_memory_use,
            limits,
        )?;
        append_bool(
            buffer,
            "recovery.replaced_global_state",
            recovery.replaced_global_state,
            limits,
        )?;
        append_bool(buffer, "recovery.persisted", recovery.persisted, limits)?;
        append_bool(
            buffer,
            "recovery.appended_ledger",
            recovery.appended_ledger,
            limits,
        )?;
        append_bool(
            buffer,
            "recovery.appended_audit",
            recovery.appended_audit,
            limits,
        )?;
        append_bool(
            buffer,
            "recovery.repaired_replay",
            recovery.repaired_replay,
            limits,
        )?;
        append_bool(
            buffer,
            "recovery.promoted_provider_output",
            recovery.promoted_provider_output,
            limits,
        )?;
        append_bool(
            buffer,
            "recovery.executed_action",
            recovery.executed_action,
            limits,
        )
    } else {
        append_optional_string(buffer, "recovery.acceptance_id", None, limits)?;
        append_optional_string(buffer, "recovery.status_code", None, limits)?;
        append_optional_string(buffer, "recovery.reason_code", None, limits)?;
        append_bool(buffer, "recovery.accepted_for_in_memory_use", false, limits)?;
        append_bool(buffer, "recovery.replaced_global_state", false, limits)?;
        append_bool(buffer, "recovery.persisted", false, limits)?;
        append_bool(buffer, "recovery.appended_ledger", false, limits)?;
        append_bool(buffer, "recovery.appended_audit", false, limits)?;
        append_bool(buffer, "recovery.repaired_replay", false, limits)?;
        append_bool(buffer, "recovery.promoted_provider_output", false, limits)?;
        append_bool(buffer, "recovery.executed_action", false, limits)
    }
}

fn append_replay(
    buffer: &mut Vec<u8>,
    replay: Option<&ObservedReplaySummary>,
    limits: &AuditExportEncodingLimits,
) -> Result<(), AuditExportEncodingReason> {
    append_bool(buffer, "replay.present", replay.is_some(), limits)?;
    if let Some(replay) = replay {
        append_line(buffer, "replay.replay_id", &replay.replay_id, limits)?;
        append_line(
            buffer,
            "replay.source_run_id",
            &replay.source_run_id,
            limits,
        )?;
        append_line(buffer, "replay.evidence_id", &replay.evidence_id, limits)?;
        append_line(buffer, "replay.status_code", &replay.status_code, limits)?;
        append_line(buffer, "replay.reason_code", &replay.reason_code, limits)?;
        append_bool(
            buffer,
            "replay.replayed_from_evidence",
            replay.replayed_from_evidence,
            limits,
        )?;
        append_bool(
            buffer,
            "replay.live_execution_performed",
            replay.live_execution_performed,
            limits,
        )?;
        append_bool(
            buffer,
            "replay.new_authorization_created",
            replay.new_authorization_created,
            limits,
        )?;
        append_bool(
            buffer,
            "replay.new_audit_record_created",
            replay.new_audit_record_created,
            limits,
        )?;
        append_bool(
            buffer,
            "replay.new_action_executed",
            replay.new_action_executed,
            limits,
        )?;
        append_bool(
            buffer,
            "replay.new_ledger_fact_created",
            replay.new_ledger_fact_created,
            limits,
        )?;
        append_bool(buffer, "replay.persisted", replay.persisted, limits)?;
        append_bool(
            buffer,
            "replay.repaired_replay",
            replay.repaired_replay,
            limits,
        )?;
        append_bool(
            buffer,
            "replay.mutated_application_state",
            replay.mutated_application_state,
            limits,
        )
    } else {
        append_optional_string(buffer, "replay.replay_id", None, limits)?;
        append_optional_string(buffer, "replay.source_run_id", None, limits)?;
        append_optional_string(buffer, "replay.evidence_id", None, limits)?;
        append_optional_string(buffer, "replay.status_code", None, limits)?;
        append_optional_string(buffer, "replay.reason_code", None, limits)?;
        append_bool(buffer, "replay.replayed_from_evidence", false, limits)?;
        append_bool(buffer, "replay.live_execution_performed", false, limits)?;
        append_bool(buffer, "replay.new_authorization_created", false, limits)?;
        append_bool(buffer, "replay.new_audit_record_created", false, limits)?;
        append_bool(buffer, "replay.new_action_executed", false, limits)?;
        append_bool(buffer, "replay.new_ledger_fact_created", false, limits)?;
        append_bool(buffer, "replay.persisted", false, limits)?;
        append_bool(buffer, "replay.repaired_replay", false, limits)?;
        append_bool(buffer, "replay.mutated_application_state", false, limits)
    }
}

fn append_action(
    buffer: &mut Vec<u8>,
    action: Option<&ObservedActionSummary>,
    limits: &AuditExportEncodingLimits,
) -> Result<(), AuditExportEncodingReason> {
    append_bool(buffer, "action.present", action.is_some(), limits)?;
    if let Some(action) = action {
        append_line(buffer, "action.action_kind", &action.action_kind, limits)?;
        append_line(
            buffer,
            "action.action_reason_code",
            &action.action_reason_code,
            limits,
        )?;
        append_bool(
            buffer,
            "action.action_real_world_effect",
            action.action_real_world_effect,
            limits,
        )
    } else {
        append_optional_string(buffer, "action.action_kind", None, limits)?;
        append_optional_string(buffer, "action.action_reason_code", None, limits)?;
        append_bool(buffer, "action.action_real_world_effect", false, limits)
    }
}

fn diagnostic_prefix(index: usize) -> String {
    let mut prefix = String::from("diagnostics.");
    prefix.push_str(&index.to_string());
    prefix
}

fn format_key(prefix: &str, suffix: &str) -> String {
    let mut key = String::from(prefix);
    key.push('.');
    key.push_str(suffix);
    key
}

fn is_canonical_fragment(value: &str) -> bool {
    !value.is_empty() && !value.contains('\n') && !value.contains('\r')
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
        assert!(
            observability_snapshot_from_supplied_evidence("", None, None, None, None, None, vec![])
                .reason
                == ObservabilitySnapshotReason::EmptySnapshotId
        );
    }
    #[test]
    fn observability_snapshot_rejects_empty_supplied_evidence() {
        assert!(
            observability_snapshot_from_supplied_evidence(
                "s",
                None,
                None,
                None,
                None,
                None,
                vec![]
            )
            .reason
                == ObservabilitySnapshotReason::NoEvidenceSupplied
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
        assert!(r.status == ObservabilitySnapshotStatus::Built);
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
        assert!(snapshot.status == ObservabilitySnapshotStatus::Built);
    }
    #[test]
    fn current_in_memory_snapshot_mode_is_unsupported() {
        assert!(
            observability_snapshot_from_current_in_memory_state("s").reason
                == ObservabilitySnapshotReason::CurrentInMemorySnapshotUnsupported
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
        assert!(s.status == ObservabilitySnapshotStatus::Rejected);
    }

    fn minimal_audit_snapshot() -> ObservabilitySnapshot {
        observability_snapshot_from_supplied_evidence(
            "snapshot-1",
            None,
            None,
            None,
            None,
            None,
            vec![ObservedDiagnosticSummary {
                family: "diag".into(),
                code: "ok".into(),
                key: "key".into(),
                summary: "minimal".into(),
            }],
        )
    }

    fn encode_minimal_snapshot() -> AuditExportEnvelope {
        match encode_audit_export_snapshot(
            "export-1",
            &minimal_audit_snapshot(),
            AuditExportEncodingLimits::strict_defaults(),
        ) {
            Ok(envelope) => envelope,
            Err(_) => panic!("minimal audit export encoding should succeed"),
        }
    }

    fn rejection_reason(
        result: Result<AuditExportEnvelope, AuditExportEncodingReport>,
    ) -> AuditExportEncodingReason {
        match result {
            Ok(_) => panic!("audit export encoding should reject this input"),
            Err(report) => report.reason,
        }
    }

    fn golden_minimal_snapshot() -> &'static [u8] {
        b"format_version=audit-export-v1\nrecord_kind=observability-snapshot\nexport_id=export-1\nsnapshot_id=snapshot-1\nsnapshot_mode=supplied_evidence_snapshot\nsnapshot_status=built\nsnapshot_reason=built_from_supplied_evidence\nharness.present=false\nharness.run_id=none\nharness.status_code=none\nharness.reason_code=none\nharness.provider_output_trusted=false\nharness.provider_output_authoritative=false\nharness.retry_scheduled=false\nharness.recovery_candidate_only=false\nharness.recovered_state_promoted=false\nharness.ui_transport_live=false\nharness.ui_submission_executes_action=false\nharness.action_kind=none\nharness.action_real_world_effect=false\ndurable_append.present=false\ndurable_append.append_transaction_id=none\ndurable_append.status_code=none\ndurable_append.reason_code=none\ndurable_append.committed=false\ndurable_append.promoted=false\ndurable_append.recovered_state=false\ndurable_append.repaired_replay=false\ndurable_append.trusted_provider_output=false\ndurable_append.executed_action=false\ndurable_append.mutated_application_state=false\nrecovery.present=false\nrecovery.acceptance_id=none\nrecovery.status_code=none\nrecovery.reason_code=none\nrecovery.accepted_for_in_memory_use=false\nrecovery.replaced_global_state=false\nrecovery.persisted=false\nrecovery.appended_ledger=false\nrecovery.appended_audit=false\nrecovery.repaired_replay=false\nrecovery.promoted_provider_output=false\nrecovery.executed_action=false\nreplay.present=false\nreplay.replay_id=none\nreplay.source_run_id=none\nreplay.evidence_id=none\nreplay.status_code=none\nreplay.reason_code=none\nreplay.replayed_from_evidence=false\nreplay.live_execution_performed=false\nreplay.new_authorization_created=false\nreplay.new_audit_record_created=false\nreplay.new_action_executed=false\nreplay.new_ledger_fact_created=false\nreplay.persisted=false\nreplay.repaired_replay=false\nreplay.mutated_application_state=false\naction.present=false\naction.action_kind=none\naction.action_reason_code=none\naction.action_real_world_effect=false\ndiagnostics.count=1\ndiagnostics.0.family=diag\ndiagnostics.0.code=ok\ndiagnostics.0.key=key\ndiagnostics.0.summary=minimal\ncontains_raw_provider_payload=false\ncontains_secret_material=false\nmutates_application_state=false\nreads_persistence=false\nwrites_persistence=false\nrecomputes_authority=false\nrepairs_state=false\nexports_data=false\nsummary=read-only observability snapshot built from caller-supplied evidence only; snapshot is non-authoritative and does not mutate authority\n"
    }

    #[test]
    fn audit_export_reason_codes_are_stable() {
        assert!(
            AuditExportEncodingReason::EncodedCanonicalSnapshot.code()
                == "encoded_canonical_snapshot"
        );
        assert!(AuditExportEncodingReason::EmptyExportId.code() == "empty_export_id");
        assert!(
            AuditExportEncodingReason::UnsupportedSnapshotMode.code()
                == "unsupported_snapshot_mode"
        );
        assert!(AuditExportEncodingReason::SnapshotNotBuilt.code() == "snapshot_not_built");
        assert!(AuditExportEncodingReason::TooManyDiagnostics.code() == "too_many_diagnostics");
        assert!(AuditExportEncodingReason::FieldTooLarge.code() == "field_too_large");
        assert!(AuditExportEncodingReason::SummaryTooLarge.code() == "summary_too_large");
        assert!(
            AuditExportEncodingReason::EncodedSizeLimitExceeded.code()
                == "encoded_size_limit_exceeded"
        );
        assert!(AuditExportEncodingReason::RawPayloadRejected.code() == "raw_payload_rejected");
        assert!(
            AuditExportEncodingReason::SecretMaterialRejected.code() == "secret_material_rejected"
        );
        assert!(
            AuditExportEncodingReason::NonCanonicalInputRejected.code()
                == "non_canonical_input_rejected"
        );
        assert!(
            AuditExportEncodingReason::ExportWriteNotAllowed.code() == "export_write_not_allowed"
        );
        assert!(
            AuditExportEncodingReason::PersistenceNotAllowed.code() == "persistence_not_allowed"
        );
        assert!(
            AuditExportEncodingReason::AuthorityMutationNotAllowed.code()
                == "authority_mutation_not_allowed"
        );
    }

    #[test]
    fn audit_export_limits_are_strict_and_stable() {
        let limits = AuditExportEncodingLimits::strict_defaults();
        assert!(limits.max_diagnostics == 16);
        assert!(limits.max_field_len == 256);
        assert!(limits.max_summary_len == 512);
        assert!(limits.max_total_bytes == 8192);
    }

    #[test]
    fn audit_export_rejects_empty_export_id() {
        assert!(
            rejection_reason(encode_audit_export_snapshot(
                "",
                &minimal_audit_snapshot(),
                AuditExportEncodingLimits::strict_defaults(),
            )) == AuditExportEncodingReason::EmptyExportId
        );
    }

    #[test]
    fn audit_export_rejects_unsupported_snapshot_mode() {
        assert!(
            rejection_reason(encode_audit_export_snapshot(
                "export-1",
                &observability_snapshot_from_current_in_memory_state("snapshot-1"),
                AuditExportEncodingLimits::strict_defaults(),
            )) == AuditExportEncodingReason::UnsupportedSnapshotMode
        );
    }

    #[test]
    fn audit_export_rejects_snapshot_not_built() {
        let snapshot = observability_snapshot_from_supplied_evidence(
            "snapshot-1",
            None,
            None,
            None,
            None,
            None,
            vec![],
        );
        assert!(
            rejection_reason(encode_audit_export_snapshot(
                "export-1",
                &snapshot,
                AuditExportEncodingLimits::strict_defaults(),
            )) == AuditExportEncodingReason::SnapshotNotBuilt
        );
    }

    #[test]
    fn audit_export_rejects_raw_payload_flag() {
        let mut snapshot = minimal_audit_snapshot();
        snapshot.contains_raw_provider_payload = true;
        assert!(
            rejection_reason(encode_audit_export_snapshot(
                "export-1",
                &snapshot,
                AuditExportEncodingLimits::strict_defaults(),
            )) == AuditExportEncodingReason::RawPayloadRejected
        );
    }

    #[test]
    fn audit_export_rejects_secret_material_flag() {
        let mut snapshot = minimal_audit_snapshot();
        snapshot.contains_secret_material = true;
        assert!(
            rejection_reason(encode_audit_export_snapshot(
                "export-1",
                &snapshot,
                AuditExportEncodingLimits::strict_defaults(),
            )) == AuditExportEncodingReason::SecretMaterialRejected
        );
    }

    #[test]
    fn audit_export_rejects_authority_mutating_snapshot() {
        let mut snapshot = minimal_audit_snapshot();
        snapshot.exports_data = true;
        assert!(
            rejection_reason(encode_audit_export_snapshot(
                "export-1",
                &snapshot,
                AuditExportEncodingLimits::strict_defaults(),
            )) == AuditExportEncodingReason::AuthorityMutationNotAllowed
        );
    }

    #[test]
    fn audit_export_rejects_too_many_diagnostics() {
        let diagnostics = (0..17)
            .map(|index| ObservedDiagnosticSummary {
                family: "diag".into(),
                code: "ok".into(),
                key: index.to_string(),
                summary: "bounded".into(),
            })
            .collect();
        let snapshot = observability_snapshot_from_supplied_evidence(
            "snapshot-1",
            None,
            None,
            None,
            None,
            None,
            diagnostics,
        );
        assert!(
            rejection_reason(encode_audit_export_snapshot(
                "export-1",
                &snapshot,
                AuditExportEncodingLimits::strict_defaults(),
            )) == AuditExportEncodingReason::TooManyDiagnostics
        );
    }

    #[test]
    fn audit_export_rejects_oversized_field() {
        let mut snapshot = minimal_audit_snapshot();
        snapshot.snapshot_id = "x".repeat(257);
        assert!(
            rejection_reason(encode_audit_export_snapshot(
                "export-1",
                &snapshot,
                AuditExportEncodingLimits::strict_defaults(),
            )) == AuditExportEncodingReason::FieldTooLarge
        );
    }

    #[test]
    fn audit_export_rejects_oversized_summary() {
        let mut snapshot = minimal_audit_snapshot();
        snapshot.summary = "x".repeat(513);
        assert!(
            rejection_reason(encode_audit_export_snapshot(
                "export-1",
                &snapshot,
                AuditExportEncodingLimits::strict_defaults(),
            )) == AuditExportEncodingReason::SummaryTooLarge
        );
    }

    #[test]
    fn audit_export_rejects_total_size_limit() {
        let limits = AuditExportEncodingLimits {
            max_total_bytes: 32,
            ..AuditExportEncodingLimits::strict_defaults()
        };
        assert!(
            rejection_reason(encode_audit_export_snapshot(
                "export-1",
                &minimal_audit_snapshot(),
                limits
            )) == AuditExportEncodingReason::EncodedSizeLimitExceeded
        );
    }

    #[test]
    fn audit_export_encodes_version_and_record_kind() {
        let envelope = encode_minimal_snapshot();
        assert!(envelope.format_version == AUDIT_EXPORT_FORMAT_VERSION);
        assert!(envelope.record_kind == AUDIT_EXPORT_RECORD_KIND);
        assert!(envelope
            .encoded_bytes
            .starts_with(b"format_version=audit-export-v1\nrecord_kind=observability-snapshot\n"));
    }

    #[test]
    fn audit_export_encoding_is_deterministic_for_same_snapshot() {
        let first = encode_minimal_snapshot().encoded_bytes;
        let second = encode_minimal_snapshot().encoded_bytes;
        assert!(first == second);
    }

    #[test]
    fn audit_export_encoding_uses_lf_line_endings() {
        let encoded = encode_minimal_snapshot().encoded_bytes;
        assert!(encoded.contains(&b'\n'));
        assert!(!encoded.contains(&b'\r'));
    }

    #[test]
    fn audit_export_field_order_is_stable() {
        let encoded = encode_minimal_snapshot().encoded_bytes;
        let text = match core::str::from_utf8(&encoded) {
            Ok(text) => text,
            Err(_) => panic!("encoded audit export should be utf-8"),
        };
        let format_index = match text.find("format_version=") {
            Some(index) => index,
            None => panic!("format_version field missing"),
        };
        let record_index = match text.find("record_kind=") {
            Some(index) => index,
            None => panic!("record_kind field missing"),
        };
        let export_index = match text.find("export_id=") {
            Some(index) => index,
            None => panic!("export_id field missing"),
        };
        let diagnostics_index = match text.find("diagnostics.count=") {
            Some(index) => index,
            None => panic!("diagnostics count field missing"),
        };
        let summary_index = match text.find("summary=") {
            Some(index) => index,
            None => panic!("summary field missing"),
        };
        assert!(format_index < record_index);
        assert!(record_index < export_index);
        assert!(export_index < diagnostics_index);
        assert!(diagnostics_index < summary_index);
    }

    #[test]
    fn audit_export_encodes_absent_optional_sections_as_none() {
        let encoded = encode_minimal_snapshot().encoded_bytes;
        let text = match core::str::from_utf8(&encoded) {
            Ok(text) => text,
            Err(_) => panic!("encoded audit export should be utf-8"),
        };
        assert!(text.contains("harness.present=false\n"));
        assert!(text.contains("harness.run_id=none\n"));
        assert!(text.contains("durable_append.append_transaction_id=none\n"));
        assert!(text.contains("recovery.acceptance_id=none\n"));
        assert!(text.contains("replay.replay_id=none\n"));
        assert!(text.contains("action.action_kind=none\n"));
    }

    #[test]
    fn audit_export_encodes_booleans_as_true_false() {
        let encoded = encode_minimal_snapshot().encoded_bytes;
        let text = match core::str::from_utf8(&encoded) {
            Ok(text) => text,
            Err(_) => panic!("encoded audit export should be utf-8"),
        };
        assert!(text.contains("contains_raw_provider_payload=false\n"));
        assert!(text.contains("harness.present=false\n"));
        assert!(!text.contains("=True\n"));
        assert!(!text.contains("=False\n"));
    }

    #[test]
    fn audit_export_does_not_use_debug_or_platform_paths() {
        let encoded = encode_minimal_snapshot().encoded_bytes;
        let text = match core::str::from_utf8(&encoded) {
            Ok(text) => text,
            Err(_) => panic!("encoded audit export should be utf-8"),
        };
        assert!(text.contains("snapshot_status=built\n"));
        assert!(!text.contains("Built"));
        assert!(!text.contains("Rejected"));
        assert!(!text.contains("/"));
        assert!(!text.contains("\\"));
    }

    #[test]
    fn audit_export_does_not_write_files() {
        assert!(!encode_minimal_snapshot().writes_files);
    }

    #[test]
    fn audit_export_does_not_read_or_write_persistence() {
        let envelope = encode_minimal_snapshot();
        assert!(!envelope.reads_persistence);
        assert!(!envelope.writes_persistence);
    }

    #[test]
    fn audit_export_does_not_mutate_authority() {
        assert!(!encode_minimal_snapshot().mutates_authority);
    }

    #[test]
    fn audit_export_does_not_include_raw_provider_payload() {
        let encoded = encode_minimal_snapshot().encoded_bytes;
        let text = match core::str::from_utf8(&encoded) {
            Ok(text) => text,
            Err(_) => panic!("encoded audit export should be utf-8"),
        };
        assert!(text.contains("contains_raw_provider_payload=false\n"));
        assert!(!text.contains("provider_prompt"));
    }

    #[test]
    fn audit_export_does_not_include_secret_material() {
        let encoded = encode_minimal_snapshot().encoded_bytes;
        let text = match core::str::from_utf8(&encoded) {
            Ok(text) => text,
            Err(_) => panic!("encoded audit export should be utf-8"),
        };
        assert!(text.contains("contains_secret_material=false\n"));
        assert!(!text.contains("password"));
        assert!(!text.contains("token"));
    }

    #[test]
    fn audit_export_golden_minimal_snapshot_matches_expected_bytes() {
        assert!(encode_minimal_snapshot().encoded_bytes == golden_minimal_snapshot());
    }

    #[test]
    fn risky_snapshot_summary_cannot_enable_export_write_or_authority() {
        let mut snapshot = minimal_audit_snapshot();
        snapshot.summary =
            "write files read persistence mutate authority append ledger export data".to_string();
        let envelope = match encode_audit_export_snapshot(
            "export-1",
            &snapshot,
            AuditExportEncodingLimits::strict_defaults(),
        ) {
            Ok(envelope) => envelope,
            Err(_) => panic!("risky summary text should remain inert data"),
        };
        assert!(!envelope.writes_files);
        assert!(!envelope.reads_persistence);
        assert!(!envelope.writes_persistence);
        assert!(!envelope.mutates_authority);
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
