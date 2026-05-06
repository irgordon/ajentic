use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationStateError {
    EmptyStateId,
    EmptyProjectionId,
    EmptyRunId,
    EmptyContextPacketId,
    EmptyMemorySnapshotId,
    UnsafeRuntimeConfig,
    ProjectionFailed,
}

impl ApplicationStateError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyStateId => "empty_state_id",
            Self::EmptyProjectionId => "empty_projection_id",
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyContextPacketId => "empty_context_packet_id",
            Self::EmptyMemorySnapshotId => "empty_memory_snapshot_id",
            Self::UnsafeRuntimeConfig => "unsafe_runtime_config",
            Self::ProjectionFailed => "projection_failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationContextMetadata {
    pub packet_id: String,
    pub slice_count: usize,
    pub source_count: usize,
    pub budget_used: usize,
    pub budget_max: usize,
}

impl ApplicationContextMetadata {
    pub fn new(
        packet_id: impl Into<String>,
        slice_count: usize,
        source_count: usize,
        budget_used: usize,
        budget_max: usize,
    ) -> Result<Self, ApplicationStateError> {
        let packet_id = packet_id.into();
        if packet_id.is_empty() {
            return Err(ApplicationStateError::EmptyContextPacketId);
        }
        Ok(Self {
            packet_id,
            slice_count,
            source_count,
            budget_used,
            budget_max,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationMemoryMetadata {
    pub snapshot_id: String,
    pub active_entries: usize,
    pub disabled_entries: usize,
    pub rejected_entries: usize,
}

impl ApplicationMemoryMetadata {
    pub fn new(
        snapshot_id: impl Into<String>,
        active_entries: usize,
        disabled_entries: usize,
        rejected_entries: usize,
    ) -> Result<Self, ApplicationStateError> {
        let snapshot_id = snapshot_id.into();
        if snapshot_id.is_empty() {
            return Err(ApplicationStateError::EmptyMemorySnapshotId);
        }
        Ok(Self {
            snapshot_id,
            active_entries,
            disabled_entries,
            rejected_entries,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalApplicationState {
    pub state_id: String,
    pub projection_id: String,
    pub run_id: String,
    pub runtime_config: LocalRuntimeConfig,
    pub harness_state: crate::state::HarnessState,
    pub controlled_run: crate::execution::ControlledRunResult,
    pub provider_output: crate::execution::ProviderOutput,
    pub integration_output: IntegrationOutput,
    pub ledger: crate::ledger::Ledger,
    pub replay_report: crate::replay::ReplayReport,
    pub audit_projections: Vec<crate::audit::AuditProjection>,
    pub context: ApplicationContextMetadata,
    pub memory: ApplicationMemoryMetadata,
}

impl LocalApplicationState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        state_id: impl Into<String>,
        projection_id: impl Into<String>,
        run_id: impl Into<String>,
        runtime_config: LocalRuntimeConfig,
        harness_state: crate::state::HarnessState,
        controlled_run: crate::execution::ControlledRunResult,
        provider_output: crate::execution::ProviderOutput,
        integration_output: IntegrationOutput,
        ledger: crate::ledger::Ledger,
        replay_report: crate::replay::ReplayReport,
        audit_projections: Vec<crate::audit::AuditProjection>,
        context: ApplicationContextMetadata,
        memory: ApplicationMemoryMetadata,
    ) -> Result<Self, ApplicationStateError> {
        let state_id = state_id.into();
        if state_id.is_empty() {
            return Err(ApplicationStateError::EmptyStateId);
        }
        let projection_id = projection_id.into();
        if projection_id.is_empty() {
            return Err(ApplicationStateError::EmptyProjectionId);
        }
        let run_id = run_id.into();
        if run_id.is_empty() {
            return Err(ApplicationStateError::EmptyRunId);
        }
        if local_runtime_config_allows_authority_bypass(&runtime_config) {
            return Err(ApplicationStateError::UnsafeRuntimeConfig);
        }
        Ok(Self {
            state_id,
            projection_id,
            run_id,
            runtime_config,
            harness_state,
            controlled_run,
            provider_output,
            integration_output,
            ledger,
            replay_report,
            audit_projections,
            context,
            memory,
        })
    }

    pub fn state_id(&self) -> &str {
        &self.state_id
    }
    pub fn run_id(&self) -> &str {
        &self.run_id
    }
    pub fn ledger_event_count(&self) -> usize {
        self.ledger.events().len()
    }
    pub fn last_ledger_revision(&self) -> Option<u64> {
        self.ledger.last_revision()
    }

    pub fn derive_read_projection(
        &self,
    ) -> Result<ApplicationReadProjection, ApplicationStateError> {
        ApplicationReadProjection::new(
            self.projection_id.clone(),
            self.run_id.clone(),
            &self.runtime_config,
            &self.harness_state,
            &self.controlled_run,
            &self.provider_output,
            &self.integration_output,
            &self.ledger,
            &self.replay_report,
            &self.audit_projections,
            self.context.packet_id.clone(),
            self.context.slice_count,
            self.context.source_count,
            self.context.budget_used,
            self.context.budget_max,
            self.memory.snapshot_id.clone(),
            self.memory.active_entries,
            self.memory.disabled_entries,
            self.memory.rejected_entries,
        )
        .map_err(|error| match error {
            ReadProjectionError::UnsafeRuntimeConfig => ApplicationStateError::UnsafeRuntimeConfig,
            _ => ApplicationStateError::ProjectionFailed,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationRecoveryStatus {
    CandidatePrepared,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationRecoveryReason {
    CandidatePreparedFromVerifiedLedgerBytes,
    EmptyRecoveryId,
    EmptyLedgerRecordId,
    EmptyLedgerBytes,
    LedgerVerificationFailed,
    LedgerChecksumMismatch,
    LedgerMalformed,
    LedgerStaleRevision,
    LedgerUnknownPayloadKind,
    NonLedgerPayloadKind,
    StateReplacementNotAllowed,
    ReplayRepairNotAllowed,
    PromotionNotAllowed,
}

impl ApplicationRecoveryReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::CandidatePreparedFromVerifiedLedgerBytes => {
                "candidate_prepared_from_verified_ledger_bytes"
            }
            Self::EmptyRecoveryId => "empty_recovery_id",
            Self::EmptyLedgerRecordId => "empty_ledger_record_id",
            Self::EmptyLedgerBytes => "empty_ledger_bytes",
            Self::LedgerVerificationFailed => "ledger_verification_failed",
            Self::LedgerChecksumMismatch => "ledger_checksum_mismatch",
            Self::LedgerMalformed => "ledger_malformed",
            Self::LedgerStaleRevision => "ledger_stale_revision",
            Self::LedgerUnknownPayloadKind => "ledger_unknown_payload_kind",
            Self::NonLedgerPayloadKind => "non_ledger_payload_kind",
            Self::StateReplacementNotAllowed => "state_replacement_not_allowed",
            Self::ReplayRepairNotAllowed => "replay_repair_not_allowed",
            Self::PromotionNotAllowed => "promotion_not_allowed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationRecoveryRequest {
    pub recovery_id: String,
    pub ledger_record_id: String,
    pub expected_revision: Option<u64>,
    pub ledger_bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationRecoveryCandidate {
    pub recovery_id: String,
    pub ledger_record_id: String,
    pub revision: u64,
    pub payload_len: usize,
    pub checksum: String,
    pub candidate_bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationRecoveryReport {
    pub status: ApplicationRecoveryStatus,
    pub reason: ApplicationRecoveryReason,
    pub recovery_id: String,
    pub ledger_record_id: String,
    pub revision: Option<u64>,
    pub payload_len: Option<usize>,
    pub checksum: Option<String>,
    pub candidate: Option<ApplicationRecoveryCandidate>,
    pub recovers_application_state: bool,
    pub promotes_recovered_state: bool,
    pub repairs_replay: bool,
    pub mutates_ledger: bool,
    pub summary: String,
}

pub fn prepare_application_recovery_candidate(
    request: ApplicationRecoveryRequest,
) -> ApplicationRecoveryReport {
    let mut report = ApplicationRecoveryReport {
        status: ApplicationRecoveryStatus::Rejected,
        reason: ApplicationRecoveryReason::LedgerVerificationFailed,
        recovery_id: request.recovery_id.clone(),
        ledger_record_id: request.ledger_record_id.clone(),
        revision: None,
        payload_len: None,
        checksum: None,
        candidate: None,
        recovers_application_state: false,
        promotes_recovered_state: false,
        repairs_replay: false,
        mutates_ledger: false,
        summary: "recovery candidate rejected; verified ledger bytes are required and do not replace local application state".to_string(),
    };

    if request.recovery_id.is_empty() {
        report.reason = ApplicationRecoveryReason::EmptyRecoveryId;
        return report;
    }
    if request.ledger_record_id.is_empty() {
        report.reason = ApplicationRecoveryReason::EmptyLedgerRecordId;
        return report;
    }
    if request.ledger_bytes.is_empty() {
        report.reason = ApplicationRecoveryReason::EmptyLedgerBytes;
        return report;
    }

    let envelope = match decode_persisted_record_envelope(&request.ledger_bytes) {
        Ok(envelope) => envelope,
        Err(error) => {
            report.reason = match error {
                PersistedRecordEnvelopeError::ChecksumMismatch => {
                    ApplicationRecoveryReason::LedgerChecksumMismatch
                }
                PersistedRecordEnvelopeError::UnknownPayloadKind => {
                    ApplicationRecoveryReason::LedgerUnknownPayloadKind
                }
                PersistedRecordEnvelopeError::PayloadLengthMismatch
                | PersistedRecordEnvelopeError::InvalidPayloadHex
                | PersistedRecordEnvelopeError::MalformedRecord => {
                    ApplicationRecoveryReason::LedgerMalformed
                }
                _ => ApplicationRecoveryReason::LedgerVerificationFailed,
            };
            return report;
        }
    };

    if envelope.payload_kind != LocalPersistencePayloadKind::LedgerSnapshot {
        report.reason = ApplicationRecoveryReason::NonLedgerPayloadKind;
        return report;
    }

    let verification = verify_persisted_record_envelope(&envelope, request.expected_revision);
    if verification.status != PersistedRecordVerificationStatus::Valid {
        report.reason = match verification.status {
            PersistedRecordVerificationStatus::ChecksumMismatch => {
                ApplicationRecoveryReason::LedgerChecksumMismatch
            }
            PersistedRecordVerificationStatus::UnknownPayloadKind => {
                ApplicationRecoveryReason::LedgerUnknownPayloadKind
            }
            PersistedRecordVerificationStatus::StaleRevision => {
                ApplicationRecoveryReason::LedgerStaleRevision
            }
            PersistedRecordVerificationStatus::PayloadLengthMismatch
            | PersistedRecordVerificationStatus::InvalidPayloadHex
            | PersistedRecordVerificationStatus::MalformedRecord => {
                ApplicationRecoveryReason::LedgerMalformed
            }
            _ => ApplicationRecoveryReason::LedgerVerificationFailed,
        };
        return report;
    }

    report.status = ApplicationRecoveryStatus::CandidatePrepared;
    report.reason = ApplicationRecoveryReason::CandidatePreparedFromVerifiedLedgerBytes;
    report.revision = Some(envelope.revision);
    report.payload_len = Some(envelope.payload_len);
    report.checksum = Some(envelope.checksum.clone());
    report.candidate = Some(ApplicationRecoveryCandidate {
        recovery_id: request.recovery_id,
        ledger_record_id: request.ledger_record_id,
        revision: envelope.revision,
        payload_len: envelope.payload_len,
        checksum: envelope.checksum,
        candidate_bytes: envelope.payload,
    });
    report.summary = "recovery candidate prepared from verified ledger bytes; candidate does not replace local application state".to_string();
    report
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAcceptanceStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAcceptanceReason {
    AcceptedForInMemoryUse,
    EmptyAcceptanceId,
    EmptyRecoveryId,
    EmptyLedgerRecordId,
    EmptyCandidateBytes,
    RevisionMismatch,
    CandidateRequired,
    CandidateNotVerified,
    GlobalStateReplacementNotAllowed,
    PersistenceNotAllowed,
    LedgerAppendNotAllowed,
    AuditAppendNotAllowed,
    ReplayRepairNotAllowed,
    ProviderTrustNotAllowed,
    ActionExecutionNotAllowed,
}

impl RecoveryAcceptanceReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AcceptedForInMemoryUse => "accepted_for_in_memory_use",
            Self::EmptyAcceptanceId => "empty_acceptance_id",
            Self::EmptyRecoveryId => "empty_recovery_id",
            Self::EmptyLedgerRecordId => "empty_ledger_record_id",
            Self::EmptyCandidateBytes => "empty_candidate_bytes",
            Self::RevisionMismatch => "revision_mismatch",
            Self::CandidateRequired => "candidate_required",
            Self::CandidateNotVerified => "candidate_not_verified",
            Self::GlobalStateReplacementNotAllowed => "global_state_replacement_not_allowed",
            Self::PersistenceNotAllowed => "persistence_not_allowed",
            Self::LedgerAppendNotAllowed => "ledger_append_not_allowed",
            Self::AuditAppendNotAllowed => "audit_append_not_allowed",
            Self::ReplayRepairNotAllowed => "replay_repair_not_allowed",
            Self::ProviderTrustNotAllowed => "provider_trust_not_allowed",
            Self::ActionExecutionNotAllowed => "action_execution_not_allowed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryAcceptanceRequest {
    pub acceptance_id: String,
    pub expected_recovery_id: String,
    pub expected_ledger_record_id: String,
    pub expected_revision: Option<u64>,
    pub candidate: ApplicationRecoveryCandidate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryAcceptanceReport {
    pub status: RecoveryAcceptanceStatus,
    pub reason: RecoveryAcceptanceReason,
    pub acceptance_id: String,
    pub recovery_id: String,
    pub ledger_record_id: String,
    pub revision: Option<u64>,
    pub payload_len: Option<usize>,
    pub checksum: Option<String>,
    pub accepted_for_in_memory_use: bool,
    pub replaced_global_state: bool,
    pub persisted: bool,
    pub appended_ledger: bool,
    pub appended_audit: bool,
    pub repaired_replay: bool,
    pub promoted_provider_output: bool,
    pub executed_action: bool,
    pub summary: String,
}

pub fn accept_recovery_candidate_for_in_memory_use(
    request: RecoveryAcceptanceRequest,
) -> RecoveryAcceptanceReport {
    let mut report = RecoveryAcceptanceReport {
        status: RecoveryAcceptanceStatus::Rejected,
        reason: RecoveryAcceptanceReason::CandidateRequired,
        acceptance_id: request.acceptance_id.clone(),
        recovery_id: request.candidate.recovery_id.clone(),
        ledger_record_id: request.candidate.ledger_record_id.clone(),
        revision: Some(request.candidate.revision),
        payload_len: Some(request.candidate.payload_len),
        checksum: Some(request.candidate.checksum.clone()),
        accepted_for_in_memory_use: false,
        replaced_global_state: false,
        persisted: false,
        appended_ledger: false,
        appended_audit: false,
        repaired_replay: false,
        promoted_provider_output: false,
        executed_action: false,
        summary: "recovery acceptance rejected; acceptance must be explicit, in-memory only, and does not replace global application state".to_string(),
    };

    if request.acceptance_id.is_empty() {
        report.reason = RecoveryAcceptanceReason::EmptyAcceptanceId;
        return report;
    }
    if request.expected_recovery_id.is_empty() {
        report.reason = RecoveryAcceptanceReason::EmptyRecoveryId;
        return report;
    }
    if request.expected_ledger_record_id.is_empty() {
        report.reason = RecoveryAcceptanceReason::EmptyLedgerRecordId;
        return report;
    }
    if request.candidate.candidate_bytes.is_empty() {
        report.reason = RecoveryAcceptanceReason::EmptyCandidateBytes;
        return report;
    }
    if request.candidate.recovery_id != request.expected_recovery_id {
        report.reason = RecoveryAcceptanceReason::CandidateNotVerified;
        return report;
    }
    if request.candidate.ledger_record_id != request.expected_ledger_record_id {
        report.reason = RecoveryAcceptanceReason::CandidateNotVerified;
        return report;
    }
    if let Some(expected_revision) = request.expected_revision {
        if request.candidate.revision != expected_revision {
            report.reason = RecoveryAcceptanceReason::RevisionMismatch;
            return report;
        }
    }

    report.status = RecoveryAcceptanceStatus::Accepted;
    report.reason = RecoveryAcceptanceReason::AcceptedForInMemoryUse;
    report.accepted_for_in_memory_use = true;
    report.summary = "recovery acceptance is explicit, in-memory only, and does not replace global application state".to_string();
    report
}

pub fn recovery_acceptance_replaces_global_state(report: &RecoveryAcceptanceReport) -> bool {
    report.replaced_global_state
}

pub fn recovery_acceptance_mutates_authority(report: &RecoveryAcceptanceReport) -> bool {
    report.replaced_global_state
        || report.persisted
        || report.appended_ledger
        || report.appended_audit
        || report.repaired_replay
        || report.promoted_provider_output
        || report.executed_action
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsafe_runtime_config_cannot_initialize_state() {
        let mut defaults = RuntimeSafetyDefaults::strict();
        defaults.allow_ui_mutation = true;
        let cfg = LocalRuntimeConfig::new(
            "cfg",
            LocalRuntimeMode::DryRun,
            LocalProviderMode::Stub,
            RuntimeSafetyLevel::Strict,
            LocalWorkspaceMetadata::new("ws", "opaque://ws", "op").expect("valid"),
            defaults,
        );
        assert!(cfg.is_err());
    }

    #[test]
    fn application_recovery_reason_codes_are_stable() {
        assert_eq!(
            ApplicationRecoveryReason::CandidatePreparedFromVerifiedLedgerBytes.code(),
            "candidate_prepared_from_verified_ledger_bytes"
        );
        assert_eq!(
            ApplicationRecoveryReason::EmptyRecoveryId.code(),
            "empty_recovery_id"
        );
        assert_eq!(
            ApplicationRecoveryReason::EmptyLedgerRecordId.code(),
            "empty_ledger_record_id"
        );
        assert_eq!(
            ApplicationRecoveryReason::EmptyLedgerBytes.code(),
            "empty_ledger_bytes"
        );
        assert_eq!(
            ApplicationRecoveryReason::LedgerVerificationFailed.code(),
            "ledger_verification_failed"
        );
        assert_eq!(
            ApplicationRecoveryReason::LedgerChecksumMismatch.code(),
            "ledger_checksum_mismatch"
        );
        assert_eq!(
            ApplicationRecoveryReason::LedgerMalformed.code(),
            "ledger_malformed"
        );
        assert_eq!(
            ApplicationRecoveryReason::LedgerStaleRevision.code(),
            "ledger_stale_revision"
        );
        assert_eq!(
            ApplicationRecoveryReason::LedgerUnknownPayloadKind.code(),
            "ledger_unknown_payload_kind"
        );
        assert_eq!(
            ApplicationRecoveryReason::NonLedgerPayloadKind.code(),
            "non_ledger_payload_kind"
        );
        assert_eq!(
            ApplicationRecoveryReason::StateReplacementNotAllowed.code(),
            "state_replacement_not_allowed"
        );
        assert_eq!(
            ApplicationRecoveryReason::ReplayRepairNotAllowed.code(),
            "replay_repair_not_allowed"
        );
        assert_eq!(
            ApplicationRecoveryReason::PromotionNotAllowed.code(),
            "promotion_not_allowed"
        );
    }

    fn recovery_request(bytes: Vec<u8>) -> ApplicationRecoveryRequest {
        ApplicationRecoveryRequest {
            recovery_id: "recovery-1".into(),
            ledger_record_id: "ledger-1".into(),
            expected_revision: Some(2),
            ledger_bytes: bytes,
        }
    }

    fn valid_ledger_bytes(
        kind: LocalPersistencePayloadKind,
        revision: u64,
        payload: Vec<u8>,
    ) -> Vec<u8> {
        let envelope = PersistedRecordEnvelope::new("ledger-1", kind, revision, payload).unwrap();
        encode_persisted_record_envelope(&envelope)
    }

    #[test]
    fn recovery_request_requires_recovery_id() {
        let mut request = recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"ok".to_vec(),
        ));
        request.recovery_id.clear();
        let report = prepare_application_recovery_candidate(request);
        assert_eq!(report.reason, ApplicationRecoveryReason::EmptyRecoveryId);
    }
    #[test]
    fn recovery_request_requires_ledger_record_id() {
        let mut request = recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"ok".to_vec(),
        ));
        request.ledger_record_id.clear();
        let report = prepare_application_recovery_candidate(request);
        assert_eq!(
            report.reason,
            ApplicationRecoveryReason::EmptyLedgerRecordId
        );
    }
    #[test]
    fn recovery_request_requires_ledger_bytes() {
        let report = prepare_application_recovery_candidate(recovery_request(vec![]));
        assert_eq!(report.reason, ApplicationRecoveryReason::EmptyLedgerBytes);
    }
    #[test]
    fn recovery_rejects_malformed_ledger_bytes() {
        let report =
            prepare_application_recovery_candidate(recovery_request(b"not-envelope".to_vec()));
        assert_eq!(report.reason, ApplicationRecoveryReason::LedgerMalformed);
    }
    #[test]
    fn recovery_rejects_invalid_payload_hex() {
        let bytes = b"AJENTIC_RECORD_V1
record_id:ledger-1
payload_kind:ledger_snapshot
revision:2
payload_len:1
checksum:af63dc4c8601ec8c
payload_hex:gg
"
        .to_vec();
        let report = prepare_application_recovery_candidate(recovery_request(bytes));
        assert_eq!(report.reason, ApplicationRecoveryReason::LedgerMalformed);
    }
    #[test]
    fn recovery_rejects_checksum_mismatch() {
        let mut text = String::from_utf8(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"abc".to_vec(),
        ))
        .unwrap();
        text = text.replace("payload_hex:616263", "payload_hex:646566");
        let report = prepare_application_recovery_candidate(recovery_request(text.into_bytes()));
        assert_eq!(
            report.reason,
            ApplicationRecoveryReason::LedgerChecksumMismatch
        );
    }
    #[test]
    fn recovery_rejects_stale_revision() {
        let mut request = recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            1,
            b"abc".to_vec(),
        ));
        request.expected_revision = Some(2);
        let report = prepare_application_recovery_candidate(request);
        assert_eq!(
            report.reason,
            ApplicationRecoveryReason::LedgerStaleRevision
        );
    }
    #[test]
    fn recovery_rejects_unknown_payload_kind() {
        let bytes = b"AJENTIC_RECORD_V1
record_id:ledger-1
payload_kind:unknown
revision:2
payload_len:1
checksum:af63dc4c8601ec8c
payload_hex:61
"
        .to_vec();
        let report = prepare_application_recovery_candidate(recovery_request(bytes));
        assert_eq!(
            report.reason,
            ApplicationRecoveryReason::LedgerUnknownPayloadKind
        );
    }
    #[test]
    fn recovery_rejects_non_ledger_payload_kind() {
        let report = prepare_application_recovery_candidate(recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::RunRecord,
            2,
            b"abc".to_vec(),
        )));
        assert_eq!(
            report.reason,
            ApplicationRecoveryReason::NonLedgerPayloadKind
        );
    }
    #[test]
    fn recovery_prepares_candidate_from_verified_ledger_bytes() {
        let report = prepare_application_recovery_candidate(recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"abc".to_vec(),
        )));
        assert_eq!(report.status, ApplicationRecoveryStatus::CandidatePrepared);
        assert_eq!(
            report.reason,
            ApplicationRecoveryReason::CandidatePreparedFromVerifiedLedgerBytes
        );
        assert!(report
            .summary
            .contains("does not replace local application state"));
    }
    #[test]
    fn recovery_candidate_preserves_revision_payload_len_checksum_and_bytes() {
        let payload = b"ledger-state".to_vec();
        let report = prepare_application_recovery_candidate(recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            9,
            payload.clone(),
        )));
        let candidate = report.candidate.unwrap();
        assert_eq!(candidate.revision, 9);
        assert_eq!(candidate.payload_len, payload.len());
        assert_eq!(
            candidate.checksum,
            calculate_persisted_record_checksum(&payload)
        );
        assert_eq!(candidate.candidate_bytes, payload);
    }
    #[test]
    fn recovery_candidate_does_not_replace_local_application_state() {
        let report = prepare_application_recovery_candidate(recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"abc".to_vec(),
        )));
        assert!(!report.recovers_application_state);
    }
    #[test]
    fn recovery_candidate_does_not_promote_recovered_state() {
        let report = prepare_application_recovery_candidate(recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"abc".to_vec(),
        )));
        assert!(!report.promotes_recovered_state);
    }
    #[test]
    fn recovery_candidate_does_not_repair_replay() {
        let report = prepare_application_recovery_candidate(recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"abc".to_vec(),
        )));
        assert!(!report.repairs_replay);
    }
    #[test]
    fn recovery_candidate_does_not_mutate_ledger() {
        let report = prepare_application_recovery_candidate(recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"abc".to_vec(),
        )));
        assert!(!report.mutates_ledger);
    }
    #[test]
    fn recovery_does_not_call_persistence_write_boundary() {
        let report = prepare_application_recovery_candidate(recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"abc".to_vec(),
        )));
        assert!(!report.summary.contains("write"));
    }
    #[test]
    fn recovery_does_not_execute_provider_output() {
        let report = prepare_application_recovery_candidate(recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"abc".to_vec(),
        )));
        assert!(!report.summary.contains("provider execution"));
    }
    #[test]
    fn recovery_does_not_record_provider_retry() {
        let report = prepare_application_recovery_candidate(recovery_request(valid_ledger_bytes(
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"abc".to_vec(),
        )));
        assert!(!report.summary.contains("retry"));
    }
    #[test]
    fn dry_run_does_not_prepare_application_recovery_candidate() {
        let report = prepare_application_recovery_candidate(recovery_request(vec![]));
        assert_eq!(report.status, ApplicationRecoveryStatus::Rejected);
    }

    fn acceptance_request(candidate: ApplicationRecoveryCandidate) -> RecoveryAcceptanceRequest {
        RecoveryAcceptanceRequest {
            acceptance_id: "acceptance-1".into(),
            expected_recovery_id: "recovery-1".into(),
            expected_ledger_record_id: "ledger-1".into(),
            expected_revision: Some(3),
            candidate,
        }
    }
    fn acceptance_candidate(bytes: Vec<u8>) -> ApplicationRecoveryCandidate {
        ApplicationRecoveryCandidate {
            recovery_id: "recovery-1".into(),
            ledger_record_id: "ledger-1".into(),
            revision: 3,
            payload_len: bytes.len(),
            checksum: calculate_persisted_record_checksum(&bytes),
            candidate_bytes: bytes,
        }
    }

    #[test]
    fn recovery_acceptance_rejects_candidate_recovery_id_mismatch() {
        let mut request = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        request.candidate.recovery_id = "mismatched-recovery".into();
        let report = accept_recovery_candidate_for_in_memory_use(request);
        assert_eq!(report.status, RecoveryAcceptanceStatus::Rejected);
        assert_eq!(
            report.reason,
            RecoveryAcceptanceReason::CandidateNotVerified
        );
        assert!(!report.accepted_for_in_memory_use);
    }

    #[test]
    fn recovery_acceptance_rejects_candidate_ledger_record_id_mismatch() {
        let mut request = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        request.candidate.ledger_record_id = "mismatched-ledger".into();
        let report = accept_recovery_candidate_for_in_memory_use(request);
        assert_eq!(report.status, RecoveryAcceptanceStatus::Rejected);
        assert_eq!(
            report.reason,
            RecoveryAcceptanceReason::CandidateNotVerified
        );
        assert!(!report.accepted_for_in_memory_use);
    }

    #[test]
    fn recovery_acceptance_rejects_candidate_revision_mismatch() {
        let mut request = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        request.candidate.revision = 99;
        let report = accept_recovery_candidate_for_in_memory_use(request);
        assert_eq!(report.status, RecoveryAcceptanceStatus::Rejected);
        assert_eq!(report.reason, RecoveryAcceptanceReason::RevisionMismatch);
        assert!(!report.accepted_for_in_memory_use);
    }

    #[test]
    fn recovery_acceptance_rejects_empty_candidate_bytes() {
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(
            acceptance_candidate(Vec::new()),
        ));
        assert_eq!(report.status, RecoveryAcceptanceStatus::Rejected);
        assert_eq!(report.reason, RecoveryAcceptanceReason::EmptyCandidateBytes);
        assert!(!report.accepted_for_in_memory_use);
    }

    #[test]
    fn recovery_candidate_mismatch_does_not_replace_global_state() {
        let mut request = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        request.candidate.recovery_id = "mismatched-recovery".into();
        let report = accept_recovery_candidate_for_in_memory_use(request);
        assert!(!report.replaced_global_state);
        assert!(!recovery_acceptance_replaces_global_state(&report));
    }

    #[test]
    fn recovery_candidate_mismatch_does_not_persist_or_append() {
        let mut request = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        request.candidate.ledger_record_id = "mismatched-ledger".into();
        let report = accept_recovery_candidate_for_in_memory_use(request);
        assert!(!report.persisted);
        assert!(!report.appended_ledger);
        assert!(!report.appended_audit);
        assert!(!recovery_acceptance_mutates_authority(&report));
    }

    #[test]
    fn recovery_candidate_mismatch_does_not_repair_replay() {
        let mut request = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        request.candidate.revision = 99;
        let report = accept_recovery_candidate_for_in_memory_use(request);
        assert!(!report.repaired_replay);
    }

    #[test]
    fn recovery_acceptance_reason_codes_are_stable() {
        assert_eq!(
            RecoveryAcceptanceReason::AcceptedForInMemoryUse.code(),
            "accepted_for_in_memory_use"
        );
        assert_eq!(
            RecoveryAcceptanceReason::EmptyAcceptanceId.code(),
            "empty_acceptance_id"
        );
        assert_eq!(
            RecoveryAcceptanceReason::EmptyRecoveryId.code(),
            "empty_recovery_id"
        );
        assert_eq!(
            RecoveryAcceptanceReason::EmptyLedgerRecordId.code(),
            "empty_ledger_record_id"
        );
        assert_eq!(
            RecoveryAcceptanceReason::EmptyCandidateBytes.code(),
            "empty_candidate_bytes"
        );
        assert_eq!(
            RecoveryAcceptanceReason::RevisionMismatch.code(),
            "revision_mismatch"
        );
        assert_eq!(
            RecoveryAcceptanceReason::CandidateRequired.code(),
            "candidate_required"
        );
        assert_eq!(
            RecoveryAcceptanceReason::CandidateNotVerified.code(),
            "candidate_not_verified"
        );
        assert_eq!(
            RecoveryAcceptanceReason::GlobalStateReplacementNotAllowed.code(),
            "global_state_replacement_not_allowed"
        );
        assert_eq!(
            RecoveryAcceptanceReason::PersistenceNotAllowed.code(),
            "persistence_not_allowed"
        );
        assert_eq!(
            RecoveryAcceptanceReason::LedgerAppendNotAllowed.code(),
            "ledger_append_not_allowed"
        );
        assert_eq!(
            RecoveryAcceptanceReason::AuditAppendNotAllowed.code(),
            "audit_append_not_allowed"
        );
        assert_eq!(
            RecoveryAcceptanceReason::ReplayRepairNotAllowed.code(),
            "replay_repair_not_allowed"
        );
        assert_eq!(
            RecoveryAcceptanceReason::ProviderTrustNotAllowed.code(),
            "provider_trust_not_allowed"
        );
        assert_eq!(
            RecoveryAcceptanceReason::ActionExecutionNotAllowed.code(),
            "action_execution_not_allowed"
        );
    }
    #[test]
    fn recovery_acceptance_requires_acceptance_id() {
        let mut r = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        r.acceptance_id.clear();
        let report = accept_recovery_candidate_for_in_memory_use(r);
        assert_eq!(report.reason, RecoveryAcceptanceReason::EmptyAcceptanceId);
    }
    #[test]
    fn recovery_acceptance_requires_recovery_id() {
        let mut r = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        r.expected_recovery_id.clear();
        let report = accept_recovery_candidate_for_in_memory_use(r);
        assert_eq!(report.reason, RecoveryAcceptanceReason::EmptyRecoveryId);
    }
    #[test]
    fn recovery_acceptance_requires_ledger_record_id() {
        let mut r = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        r.expected_ledger_record_id.clear();
        let report = accept_recovery_candidate_for_in_memory_use(r);
        assert_eq!(report.reason, RecoveryAcceptanceReason::EmptyLedgerRecordId);
    }
    #[test]
    fn recovery_acceptance_requires_candidate_bytes() {
        let r = acceptance_request(acceptance_candidate(vec![]));
        let report = accept_recovery_candidate_for_in_memory_use(r);
        assert_eq!(report.reason, RecoveryAcceptanceReason::EmptyCandidateBytes);
    }
    #[test]
    fn recovery_acceptance_rejects_recovery_id_mismatch() {
        let mut r = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        r.expected_recovery_id = "other".into();
        let report = accept_recovery_candidate_for_in_memory_use(r);
        assert_eq!(
            report.reason,
            RecoveryAcceptanceReason::CandidateNotVerified
        );
    }
    #[test]
    fn recovery_acceptance_rejects_ledger_record_id_mismatch() {
        let mut r = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        r.expected_ledger_record_id = "other".into();
        let report = accept_recovery_candidate_for_in_memory_use(r);
        assert_eq!(
            report.reason,
            RecoveryAcceptanceReason::CandidateNotVerified
        );
    }
    #[test]
    fn recovery_acceptance_rejects_revision_mismatch() {
        let mut r = acceptance_request(acceptance_candidate(b"abc".to_vec()));
        r.expected_revision = Some(9);
        let report = accept_recovery_candidate_for_in_memory_use(r);
        assert_eq!(report.reason, RecoveryAcceptanceReason::RevisionMismatch);
    }
    #[test]
    fn recovery_acceptance_accepts_verified_candidate_for_in_memory_use() {
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(
            acceptance_candidate(b"abc".to_vec()),
        ));
        assert_eq!(report.status, RecoveryAcceptanceStatus::Accepted);
        assert!(report.accepted_for_in_memory_use);
    }
    #[test]
    fn recovery_acceptance_preserves_candidate_revision_payload_len_checksum() {
        let bytes = b"ledger-state".to_vec();
        let c = acceptance_candidate(bytes.clone());
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(c.clone()));
        assert_eq!(report.revision, Some(c.revision));
        assert_eq!(report.payload_len, Some(bytes.len()));
        assert_eq!(report.checksum, Some(c.checksum));
    }
    #[test]
    fn recovery_acceptance_does_not_replace_global_state() {
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(
            acceptance_candidate(b"abc".to_vec()),
        ));
        assert!(!recovery_acceptance_replaces_global_state(&report));
    }
    #[test]
    fn recovery_acceptance_does_not_persist() {
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(
            acceptance_candidate(b"abc".to_vec()),
        ));
        assert!(!report.persisted);
    }
    #[test]
    fn recovery_acceptance_does_not_append_ledger() {
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(
            acceptance_candidate(b"abc".to_vec()),
        ));
        assert!(!report.appended_ledger);
    }
    #[test]
    fn recovery_acceptance_does_not_append_audit() {
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(
            acceptance_candidate(b"abc".to_vec()),
        ));
        assert!(!report.appended_audit);
    }
    #[test]
    fn recovery_acceptance_does_not_repair_replay() {
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(
            acceptance_candidate(b"abc".to_vec()),
        ));
        assert!(!report.repaired_replay);
    }
    #[test]
    fn recovery_acceptance_does_not_promote_provider_output() {
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(
            acceptance_candidate(b"abc".to_vec()),
        ));
        assert!(!report.promoted_provider_output);
    }
    #[test]
    fn recovery_acceptance_does_not_execute_action() {
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(
            acceptance_candidate(b"abc".to_vec()),
        ));
        assert!(!report.executed_action);
    }
    #[test]
    fn recovery_acceptance_mutates_authority_returns_false() {
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(
            acceptance_candidate(b"abc".to_vec()),
        ));
        assert!(!recovery_acceptance_mutates_authority(&report));
    }
    #[test]
    fn risky_candidate_bytes_cannot_trigger_authority() {
        let bytes=b"accepted trusted promote recover global state repair replay append ledger execute action".to_vec();
        let report = accept_recovery_candidate_for_in_memory_use(acceptance_request(
            acceptance_candidate(bytes),
        ));
        assert!(!recovery_acceptance_mutates_authority(&report));
    }
    #[test]
    fn dry_run_does_not_accept_recovery_candidate() {
        let mut r = acceptance_request(acceptance_candidate(vec![]));
        r.acceptance_id = "".into();
        let report = accept_recovery_candidate_for_in_memory_use(r);
        assert_eq!(report.status, RecoveryAcceptanceStatus::Rejected);
    }
}

#[cfg(test)]
mod diagnostic_tests {
    use super::*;

    #[test]
    fn application_state_error_diagnostic_preserves_code() {
        let error = ApplicationStateError::EmptyStateId;
        let diagnostic = crate::api::application_state_error_diagnostic(error);
        assert_eq!(diagnostic.code, error.code());
    }
}
