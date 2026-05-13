use super::*;

pub fn write_local_session_package(
    package: &LocalSessionPackage,
    caller_provided_path: &std::path::Path,
) -> Result<LocalSessionPackageWriteResult, Vec<LocalSessionPackageValidationError>> {
    let serialized = serialize_local_session_package(package)?;
    std::fs::write(caller_provided_path, serialized.as_bytes())
        .map_err(|_| vec![LocalSessionPackageValidationError::MalformedPackageInput])?;
    let mut written_package = package.clone();
    written_package.metadata.package_status = LocalSessionPackageStatus::PackageWritten;
    Ok(LocalSessionPackageWriteResult {
        status: LocalSessionPackageStatus::PackageWritten,
        path: caller_provided_path.display().to_string(),
        bytes_written: serialized.len(),
        projection: project_local_session_package_status(Some(&written_package), None),
    })
}

pub fn read_local_session_package(
    caller_provided_path: &std::path::Path,
) -> Result<LocalSessionPackageReadResult, Vec<LocalSessionPackageValidationError>> {
    let content = std::fs::read_to_string(caller_provided_path)
        .map_err(|_| vec![LocalSessionPackageValidationError::MalformedPackageInput])?;
    let package = parse_local_session_package(&content)?;
    let projection = validate_local_session_package_read_back(&package);
    Ok(LocalSessionPackageReadResult {
        status: LocalSessionPackageStatus::PackageReadBackValidated,
        path: caller_provided_path.display().to_string(),
        package: Some(package),
        projection,
    })
}

pub fn read_local_session_restore_projection(
    caller_provided_path: &std::path::Path,
) -> LocalSessionRestoreProjection {
    match std::fs::read_to_string(caller_provided_path) {
        Ok(content) => project_local_session_restore_from_payload(&content),
        Err(_) => reject_local_session_restore_request(LocalSessionRestoreError::PackageReadFailed),
    }
}

pub fn write_trial_session_evidence_record(
    record: &TrialSessionEvidenceRecord,
    caller_provided_path: &std::path::Path,
) -> Result<TrialSessionEvidenceWriteResult, Vec<TrialSessionEvidenceValidationError>> {
    let content = serialize_trial_session_evidence_record(record)?;
    std::fs::write(caller_provided_path, content.as_bytes())
        .map_err(|_| vec![TrialSessionEvidenceValidationError::MalformedEvidenceInput])?;
    let mut written = record.clone();
    written.metadata.evidence_status = TrialSessionEvidenceStatus::EvidenceWritten;
    Ok(TrialSessionEvidenceWriteResult {
        status: TrialSessionEvidenceStatus::EvidenceWritten,
        path: caller_provided_path.display().to_string(),
        bytes_written: content.len(),
        projection: project_trial_session_evidence_status(Some(&written), None),
    })
}

pub fn read_trial_session_evidence_record(
    caller_provided_path: &std::path::Path,
) -> Result<TrialSessionEvidenceReadResult, Vec<TrialSessionEvidenceValidationError>> {
    let content = std::fs::read_to_string(caller_provided_path)
        .map_err(|_| vec![TrialSessionEvidenceValidationError::MalformedEvidenceInput])?;
    let record = parse_trial_session_evidence_record(&content)?;
    let projection = validate_trial_session_evidence_read_back(&record);
    Ok(TrialSessionEvidenceReadResult {
        status: TrialSessionEvidenceStatus::EvidenceReadBackValidated,
        path: caller_provided_path.display().to_string(),
        record: Some(record),
        projection,
    })
}

#[cfg(test)]
mod phase_151_local_session_package_persistence_tests {
    use super::*;

    fn phase_151_package_with_available_sections() -> LocalSessionPackage {
        let configured = apply_local_provider_configuration_candidate(
            &initial_local_operator_shell_state(),
            LocalProviderConfigurationCandidate::deterministic_stub(),
        )
        .unwrap();
        let executed = apply_local_provider_execution(
            &configured,
            LocalProviderExecutionRequest::deterministic_stub(
                "phase 151 package persistence input",
            ),
        )
        .unwrap();
        derive_local_session_package(&executed)
    }

    #[test]
    fn phase_151_explicit_write_read_and_malformed_read_back_behavior() {
        let package = phase_151_package_with_available_sections();
        let path = std::env::temp_dir().join(format!(
            "ajentic-phase-151-{}.session",
            package.metadata.content_digest
        ));
        let _ = std::fs::remove_file(&path);
        let write = write_local_session_package(&package, &path).unwrap();
        assert_eq!(write.status, LocalSessionPackageStatus::PackageWritten);
        assert!(write.bytes_written > 0);
        let read = read_local_session_package(&path).unwrap();
        assert_eq!(
            read.status,
            LocalSessionPackageStatus::PackageReadBackValidated
        );
        assert_eq!(
            read.projection.read_back_validation_status,
            Some(LocalSessionPackageValidationStatus::Valid)
        );
        assert_eq!(
            read.projection.restore_status,
            LocalSessionPackageRestoreStatus::ReadBackValidated
        );
        std::fs::remove_file(&path).unwrap();

        let malformed_path = std::env::temp_dir().join("ajentic-phase-151-malformed.session");
        std::fs::write(&malformed_path, "not a local session package").unwrap();
        assert!(read_local_session_package(&malformed_path)
            .unwrap_err()
            .contains(&LocalSessionPackageValidationError::MalformedPackageInput));
        std::fs::remove_file(&malformed_path).unwrap();

        let missing_path = std::env::temp_dir().join("ajentic-phase-152-missing.session");
        let _ = std::fs::remove_file(&missing_path);
        let restore = read_local_session_restore_projection(&missing_path);
        assert_eq!(restore.status, LocalSessionRestoreStatus::RestoreRejected);
        assert!(restore
            .errors
            .contains(&LocalSessionRestoreError::PackageReadFailed));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPersistencePayloadKind {
    LedgerSnapshot,
    RunRecord,
    AuditProjection,
    MemorySnapshot,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPersistenceWriteMode {
    CreateNew,
    ReplaceExisting,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPersistenceAtomicity {
    Required,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalPersistencePlan {
    pub plan_id: String,
    pub target_path: String,
    pub temp_path: String,
    pub expected_revision: Option<u64>,
    pub payload_kind: LocalPersistencePayloadKind,
    pub write_mode: LocalPersistenceWriteMode,
    pub atomicity: LocalPersistenceAtomicity,
}

impl LocalPersistencePlan {
    pub fn new(
        plan_id: impl Into<String>,
        target_path: impl Into<String>,
        temp_path: impl Into<String>,
        expected_revision: Option<u64>,
        payload_kind: LocalPersistencePayloadKind,
        write_mode: LocalPersistenceWriteMode,
        atomicity: LocalPersistenceAtomicity,
    ) -> Self {
        Self {
            plan_id: plan_id.into(),
            target_path: target_path.into(),
            temp_path: temp_path.into(),
            expected_revision,
            payload_kind,
            write_mode,
            atomicity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalPersistenceValidation {
    pub plan_id: String,
    pub valid: bool,
    pub reason: LocalPersistenceValidationReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPersistenceValidationReason {
    Valid,
    EmptyPlanId,
    EmptyTargetPath,
    EmptyTempPath,
    TargetAndTempPathSame,
    MissingExpectedRevision,
    UnknownPayloadKind,
    UnknownWriteMode,
    AtomicityNotRequired,
    PathLooksLikeSecret,
}

impl LocalPersistenceValidationReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::EmptyPlanId => "empty_plan_id",
            Self::EmptyTargetPath => "empty_target_path",
            Self::EmptyTempPath => "empty_temp_path",
            Self::TargetAndTempPathSame => "target_and_temp_path_same",
            Self::MissingExpectedRevision => "missing_expected_revision",
            Self::UnknownPayloadKind => "unknown_payload_kind",
            Self::UnknownWriteMode => "unknown_write_mode",
            Self::AtomicityNotRequired => "atomicity_not_required",
            Self::PathLooksLikeSecret => "path_looks_like_secret",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPersistenceError {
    InvalidPlan,
    PhysicalWriteNotImplemented,
    EmptyPayload,
    TargetAlreadyExists,
    TempWriteFailed,
    TempSyncFailed,
    AtomicRenameFailed,
}

impl LocalPersistenceError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPlan => "invalid_plan",
            Self::PhysicalWriteNotImplemented => "physical_write_not_implemented",
            Self::EmptyPayload => "empty_payload",
            Self::TargetAlreadyExists => "target_already_exists",
            Self::TempWriteFailed => "temp_write_failed",
            Self::TempSyncFailed => "temp_sync_failed",
            Self::AtomicRenameFailed => "atomic_rename_failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedRecordEnvelope {
    pub record_id: String,
    pub payload_kind: LocalPersistencePayloadKind,
    pub revision: u64,
    pub payload_len: usize,
    pub checksum: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistedRecordEnvelopeError {
    EmptyRecordId,
    UnknownPayloadKind,
    EmptyPayload,
    PayloadLengthMismatch,
    ChecksumMismatch,
    MalformedRecord,
    InvalidRevision,
    InvalidPayloadHex,
    InvalidChecksum,
}

impl PersistedRecordEnvelopeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyRecordId => "empty_record_id",
            Self::UnknownPayloadKind => "unknown_payload_kind",
            Self::EmptyPayload => "empty_payload",
            Self::PayloadLengthMismatch => "payload_length_mismatch",
            Self::ChecksumMismatch => "checksum_mismatch",
            Self::MalformedRecord => "malformed_record",
            Self::InvalidRevision => "invalid_revision",
            Self::InvalidPayloadHex => "invalid_payload_hex",
            Self::InvalidChecksum => "invalid_checksum",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistedRecordVerificationStatus {
    Valid,
    MissingTarget,
    OrphanedTemp,
    MalformedRecord,
    InvalidPayloadHex,
    ChecksumMismatch,
    PayloadLengthMismatch,
    StaleRevision,
    UnknownPayloadKind,
    ReadFailed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistedRecordRecoveryAction {
    None,
    ManualReviewRequired,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedRecordVerificationReport {
    pub status: PersistedRecordVerificationStatus,
    pub recovery_action: PersistedRecordRecoveryAction,
    pub record_id: Option<String>,
    pub payload_kind: Option<LocalPersistencePayloadKind>,
    pub revision: Option<u64>,
    pub checksum: Option<String>,
    pub payload_len: Option<usize>,
    pub summary: String,
}

impl PersistedRecordEnvelope {
    pub fn new(
        record_id: impl Into<String>,
        payload_kind: LocalPersistencePayloadKind,
        revision: u64,
        payload: Vec<u8>,
    ) -> Result<Self, PersistedRecordEnvelopeError> {
        let record_id = record_id.into();
        if record_id.is_empty() {
            return Err(PersistedRecordEnvelopeError::EmptyRecordId);
        }
        if payload_kind == LocalPersistencePayloadKind::Unknown {
            return Err(PersistedRecordEnvelopeError::UnknownPayloadKind);
        }
        if payload.is_empty() {
            return Err(PersistedRecordEnvelopeError::EmptyPayload);
        }
        let payload_len = payload.len();
        let checksum = calculate_persisted_record_checksum(&payload);
        Ok(Self {
            record_id,
            payload_kind,
            revision,
            payload_len,
            checksum,
            payload,
        })
    }
}

pub fn calculate_persisted_record_checksum(payload: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in payload {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn is_valid_checksum(checksum: &str) -> bool {
    checksum.len() == 16
        && checksum
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}
fn payload_kind_code(kind: LocalPersistencePayloadKind) -> &'static str {
    match kind {
        LocalPersistencePayloadKind::LedgerSnapshot => "ledger_snapshot",
        LocalPersistencePayloadKind::RunRecord => "run_record",
        LocalPersistencePayloadKind::AuditProjection => "audit_projection",
        LocalPersistencePayloadKind::MemorySnapshot => "memory_snapshot",
        LocalPersistencePayloadKind::Unknown => "unknown",
    }
}
fn parse_payload_kind(kind: &str) -> Option<LocalPersistencePayloadKind> {
    Some(match kind {
        "ledger_snapshot" => LocalPersistencePayloadKind::LedgerSnapshot,
        "run_record" => LocalPersistencePayloadKind::RunRecord,
        "audit_projection" => LocalPersistencePayloadKind::AuditProjection,
        "memory_snapshot" => LocalPersistencePayloadKind::MemorySnapshot,
        "unknown" => LocalPersistencePayloadKind::Unknown,
        _ => return None,
    })
}
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(hex: &str) -> Result<Vec<u8>, PersistedRecordEnvelopeError> {
    if !hex.len().is_multiple_of(2) {
        return Err(PersistedRecordEnvelopeError::InvalidPayloadHex);
    }
    let mut out = Vec::with_capacity(hex.len() / 2);
    let chars: Vec<char> = hex.chars().collect();
    for i in (0..chars.len()).step_by(2) {
        let s: String = [chars[i], chars[i + 1]].iter().collect();
        out.push(
            u8::from_str_radix(&s, 16)
                .map_err(|_| PersistedRecordEnvelopeError::InvalidPayloadHex)?,
        );
    }
    Ok(out)
}

pub fn encode_persisted_record_envelope(envelope: &PersistedRecordEnvelope) -> Vec<u8> {
    format!("AJENTIC_RECORD_V1\nrecord_id:{}\npayload_kind:{}\nrevision:{}\npayload_len:{}\nchecksum:{}\npayload_hex:{}\n", envelope.record_id, payload_kind_code(envelope.payload_kind), envelope.revision, envelope.payload_len, envelope.checksum, hex_encode(&envelope.payload)).into_bytes()
}

pub fn decode_persisted_record_envelope(
    bytes: &[u8],
) -> Result<PersistedRecordEnvelope, PersistedRecordEnvelopeError> {
    let text =
        std::str::from_utf8(bytes).map_err(|_| PersistedRecordEnvelopeError::MalformedRecord)?;
    let mut lines = text.lines();
    if lines.next() != Some("AJENTIC_RECORD_V1") {
        return Err(PersistedRecordEnvelopeError::MalformedRecord);
    }
    let mut record_id = None;
    let mut payload_kind = None;
    let mut revision = None;
    let mut payload_len = None;
    let mut checksum = None;
    let mut payload_hex = None;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (k, v) = line
            .split_once(':')
            .ok_or(PersistedRecordEnvelopeError::MalformedRecord)?;
        let slot = match k {
            "record_id" => &mut record_id,
            "payload_kind" => &mut payload_kind,
            "revision" => &mut revision,
            "payload_len" => &mut payload_len,
            "checksum" => &mut checksum,
            "payload_hex" => &mut payload_hex,
            _ => return Err(PersistedRecordEnvelopeError::MalformedRecord),
        };
        if slot.is_some() {
            return Err(PersistedRecordEnvelopeError::MalformedRecord);
        }
        *slot = Some(v.to_string());
    }
    let record_id = record_id.ok_or(PersistedRecordEnvelopeError::MalformedRecord)?;
    if record_id.is_empty() {
        return Err(PersistedRecordEnvelopeError::EmptyRecordId);
    }
    if record_id.contains('\n') {
        return Err(PersistedRecordEnvelopeError::MalformedRecord);
    }
    let payload_kind =
        parse_payload_kind(&payload_kind.ok_or(PersistedRecordEnvelopeError::MalformedRecord)?)
            .ok_or(PersistedRecordEnvelopeError::UnknownPayloadKind)?;
    if payload_kind == LocalPersistencePayloadKind::Unknown {
        return Err(PersistedRecordEnvelopeError::UnknownPayloadKind);
    }
    let revision = revision
        .ok_or(PersistedRecordEnvelopeError::MalformedRecord)?
        .parse::<u64>()
        .map_err(|_| PersistedRecordEnvelopeError::InvalidRevision)?;
    let payload_len = payload_len
        .ok_or(PersistedRecordEnvelopeError::MalformedRecord)?
        .parse::<usize>()
        .map_err(|_| PersistedRecordEnvelopeError::MalformedRecord)?;
    let checksum = checksum.ok_or(PersistedRecordEnvelopeError::MalformedRecord)?;
    if !is_valid_checksum(&checksum) {
        return Err(PersistedRecordEnvelopeError::InvalidChecksum);
    }
    let payload = hex_decode(&payload_hex.ok_or(PersistedRecordEnvelopeError::MalformedRecord)?)?;
    if payload_len != payload.len() {
        return Err(PersistedRecordEnvelopeError::PayloadLengthMismatch);
    }
    let actual = calculate_persisted_record_checksum(&payload);
    if checksum != actual {
        return Err(PersistedRecordEnvelopeError::ChecksumMismatch);
    }
    if payload.is_empty() {
        return Err(PersistedRecordEnvelopeError::EmptyPayload);
    }
    Ok(PersistedRecordEnvelope {
        record_id,
        payload_kind,
        revision,
        payload_len,
        checksum,
        payload,
    })
}

pub fn verify_persisted_record_envelope(
    envelope: &PersistedRecordEnvelope,
    expected_revision: Option<u64>,
) -> PersistedRecordVerificationReport {
    let mut r = PersistedRecordVerificationReport {
        status: PersistedRecordVerificationStatus::Valid,
        recovery_action: PersistedRecordRecoveryAction::None,
        record_id: Some(envelope.record_id.clone()),
        payload_kind: Some(envelope.payload_kind),
        revision: Some(envelope.revision),
        checksum: Some(envelope.checksum.clone()),
        payload_len: Some(envelope.payload_len),
        summary: "valid persisted record".into(),
    };
    if envelope.payload_kind == LocalPersistencePayloadKind::Unknown {
        r.status = PersistedRecordVerificationStatus::UnknownPayloadKind;
        r.recovery_action = PersistedRecordRecoveryAction::ManualReviewRequired;
        r.summary = "unknown payload kind".into();
    } else if envelope.payload.is_empty() || envelope.record_id.is_empty() {
        r.status = PersistedRecordVerificationStatus::MalformedRecord;
        r.recovery_action = PersistedRecordRecoveryAction::ManualReviewRequired;
        r.summary = "malformed record".into();
    } else if envelope.payload_len != envelope.payload.len() {
        r.status = PersistedRecordVerificationStatus::PayloadLengthMismatch;
        r.recovery_action = PersistedRecordRecoveryAction::ManualReviewRequired;
        r.summary = "payload length mismatch".into();
    } else if !is_valid_checksum(&envelope.checksum) {
        r.status = PersistedRecordVerificationStatus::MalformedRecord;
        r.recovery_action = PersistedRecordRecoveryAction::ManualReviewRequired;
        r.summary = "malformed checksum".into();
    } else if envelope.checksum != calculate_persisted_record_checksum(&envelope.payload) {
        r.status = PersistedRecordVerificationStatus::ChecksumMismatch;
        r.recovery_action = PersistedRecordRecoveryAction::ManualReviewRequired;
        r.summary = "checksum mismatch".into();
    } else if let Some(exp) = expected_revision {
        if envelope.revision < exp {
            r.status = PersistedRecordVerificationStatus::StaleRevision;
            r.recovery_action = PersistedRecordRecoveryAction::ManualReviewRequired;
            r.summary = "stale revision".into();
        }
    }
    r
}

pub fn verify_persisted_record_paths(
    target_path: impl AsRef<std::path::Path>,
    temp_path: impl AsRef<std::path::Path>,
    expected_revision: Option<u64>,
) -> PersistedRecordVerificationReport {
    let target_path = target_path.as_ref();
    let temp_path = temp_path.as_ref();
    let target_exists = target_path.exists();
    let temp_exists = temp_path.exists();
    if !target_exists && !temp_exists {
        return PersistedRecordVerificationReport {
            status: PersistedRecordVerificationStatus::MissingTarget,
            recovery_action: PersistedRecordRecoveryAction::ManualReviewRequired,
            record_id: None,
            payload_kind: None,
            revision: None,
            checksum: None,
            payload_len: None,
            summary: "target missing".into(),
        };
    }
    if temp_exists {
        return PersistedRecordVerificationReport {
            status: PersistedRecordVerificationStatus::OrphanedTemp,
            recovery_action: PersistedRecordRecoveryAction::ManualReviewRequired,
            record_id: None,
            payload_kind: None,
            revision: None,
            checksum: None,
            payload_len: None,
            summary: "orphaned temp".into(),
        };
    }
    let bytes = match std::fs::read(target_path) {
        Ok(v) => v,
        Err(_) => {
            return PersistedRecordVerificationReport {
                status: PersistedRecordVerificationStatus::ReadFailed,
                recovery_action: PersistedRecordRecoveryAction::ManualReviewRequired,
                record_id: None,
                payload_kind: None,
                revision: None,
                checksum: None,
                payload_len: None,
                summary: "read failed".into(),
            }
        }
    };
    match decode_persisted_record_envelope(&bytes) {
        Ok(env) => verify_persisted_record_envelope(&env, expected_revision),
        Err(err) => {
            let status = match err {
                PersistedRecordEnvelopeError::UnknownPayloadKind => {
                    PersistedRecordVerificationStatus::UnknownPayloadKind
                }
                PersistedRecordEnvelopeError::PayloadLengthMismatch => {
                    PersistedRecordVerificationStatus::PayloadLengthMismatch
                }
                PersistedRecordEnvelopeError::ChecksumMismatch => {
                    PersistedRecordVerificationStatus::ChecksumMismatch
                }
                PersistedRecordEnvelopeError::InvalidPayloadHex => {
                    PersistedRecordVerificationStatus::InvalidPayloadHex
                }
                _ => PersistedRecordVerificationStatus::MalformedRecord,
            };
            PersistedRecordVerificationReport {
                status,
                recovery_action: PersistedRecordRecoveryAction::ManualReviewRequired,
                record_id: None,
                payload_kind: None,
                revision: None,
                checksum: None,
                payload_len: None,
                summary: format!("decode failed: {}", err.code()),
            }
        }
    }
}

pub fn validate_local_persistence_plan(plan: &LocalPersistencePlan) -> LocalPersistenceValidation {
    let reason = if plan.plan_id.is_empty() {
        LocalPersistenceValidationReason::EmptyPlanId
    } else if plan.target_path.is_empty() {
        LocalPersistenceValidationReason::EmptyTargetPath
    } else if plan.temp_path.is_empty() {
        LocalPersistenceValidationReason::EmptyTempPath
    } else if plan.target_path == plan.temp_path {
        LocalPersistenceValidationReason::TargetAndTempPathSame
    } else if plan.expected_revision.is_none() {
        LocalPersistenceValidationReason::MissingExpectedRevision
    } else if plan.payload_kind == LocalPersistencePayloadKind::Unknown {
        LocalPersistenceValidationReason::UnknownPayloadKind
    } else if plan.write_mode == LocalPersistenceWriteMode::Unknown {
        LocalPersistenceValidationReason::UnknownWriteMode
    } else if plan.atomicity != LocalPersistenceAtomicity::Required {
        LocalPersistenceValidationReason::AtomicityNotRequired
    } else if contains_secret_marker(&plan.target_path) || contains_secret_marker(&plan.temp_path) {
        LocalPersistenceValidationReason::PathLooksLikeSecret
    } else {
        LocalPersistenceValidationReason::Valid
    };
    LocalPersistenceValidation {
        plan_id: plan.plan_id.clone(),
        valid: reason == LocalPersistenceValidationReason::Valid,
        reason,
    }
}

pub fn local_persistence_plan_allows_hidden_write(plan: &LocalPersistencePlan) -> bool {
    !validate_local_persistence_plan(plan).valid
}

pub fn execute_local_persistence_plan(
    plan: &LocalPersistencePlan,
    payload_bytes: &[u8],
) -> Result<(), LocalPersistenceError> {
    let validation = validate_local_persistence_plan(plan);
    if !validation.valid {
        return Err(LocalPersistenceError::InvalidPlan);
    }
    if payload_bytes.is_empty() {
        return Err(LocalPersistenceError::EmptyPayload);
    }
    if plan.write_mode == LocalPersistenceWriteMode::CreateNew
        && std::path::Path::new(&plan.target_path).exists()
    {
        return Err(LocalPersistenceError::TargetAlreadyExists);
    }
    let mut temp_file = std::fs::File::create(&plan.temp_path)
        .map_err(|_| LocalPersistenceError::TempWriteFailed)?;
    use std::io::Write;
    temp_file
        .write_all(payload_bytes)
        .map_err(|_| LocalPersistenceError::TempWriteFailed)?;
    temp_file
        .flush()
        .map_err(|_| LocalPersistenceError::TempSyncFailed)?;
    temp_file
        .sync_all()
        .map_err(|_| LocalPersistenceError::TempSyncFailed)?;
    drop(temp_file);
    std::fs::rename(&plan.temp_path, &plan.target_path)
        .map_err(|_| LocalPersistenceError::AtomicRenameFailed)?;
    Ok(())
}

pub fn write_phase_111_decision_evidence_append_bytes(
    plan: &LocalPersistencePlan,
    payload_bytes: &[u8],
) -> Result<(), LocalPersistenceError> {
    match execute_local_persistence_plan(plan, payload_bytes) {
        Ok(()) => Ok(()),
        Err(err) => {
            let _ = std::fs::remove_file(&plan.temp_path);
            Err(err)
        }
    }
}

pub fn create_local_persistence_dir(
    path: impl AsRef<std::path::Path>,
) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(path)
}

pub fn local_persistence_path_exists(path: impl AsRef<std::path::Path>) -> bool {
    path.as_ref().exists()
}

pub fn read_local_persistence_text(
    path: impl AsRef<std::path::Path>,
) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

pub fn remove_local_persistence_tree(
    path: impl AsRef<std::path::Path>,
) -> Result<(), std::io::Error> {
    std::fs::remove_dir_all(path)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalExportPersistenceWriteError {
    TargetAlreadyExists,
    TargetIsSymlink,
    EmptyPayload,
    CreateFailed,
    WriteFailed,
    SyncFailed,
    ReadbackFailed,
    VerificationFailed,
}

pub fn create_new_verified_local_export_file(
    target_path: &std::path::Path,
    payload_bytes: &[u8],
) -> Result<usize, LocalExportPersistenceWriteError> {
    if payload_bytes.is_empty() {
        return Err(LocalExportPersistenceWriteError::EmptyPayload);
    }
    if target_path.exists() {
        return Err(LocalExportPersistenceWriteError::TargetAlreadyExists);
    }
    if target_path
        .symlink_metadata()
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
    {
        return Err(LocalExportPersistenceWriteError::TargetIsSymlink);
    }

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(target_path)
        .map_err(|_| {
            if target_path.exists() {
                LocalExportPersistenceWriteError::TargetAlreadyExists
            } else {
                LocalExportPersistenceWriteError::CreateFailed
            }
        })?;
    use std::io::Write;
    file.write_all(payload_bytes)
        .map_err(|_| LocalExportPersistenceWriteError::WriteFailed)?;
    file.flush()
        .map_err(|_| LocalExportPersistenceWriteError::SyncFailed)?;
    file.sync_all()
        .map_err(|_| LocalExportPersistenceWriteError::SyncFailed)?;
    drop(file);

    let written_bytes =
        std::fs::read(target_path).map_err(|_| LocalExportPersistenceWriteError::ReadbackFailed)?;
    if written_bytes != payload_bytes {
        return Err(LocalExportPersistenceWriteError::VerificationFailed);
    }
    Ok(written_bytes.len())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerPersistenceStatus {
    Prepared,
    Written,
    Verified,
    Loaded,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerPersistenceReason {
    PreparedForPersistence,
    WrittenThroughPersistenceBoundary,
    VerifiedForLoad,
    LoadedVerifiedBytes,
    EmptyLifecycleId,
    EmptyLedgerRecordId,
    EmptyLedgerBytes,
    InvalidPersistencePlan,
    PersistenceWriteFailed,
    VerificationFailed,
    ChecksumMismatch,
    MalformedRecord,
    StaleRevision,
    UnknownPayloadKind,
    RecoveryNotImplemented,
    StateRecoveryNotAllowed,
}

impl LedgerPersistenceReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::PreparedForPersistence => "prepared_for_persistence",
            Self::WrittenThroughPersistenceBoundary => "written_through_persistence_boundary",
            Self::VerifiedForLoad => "verified_for_load",
            Self::LoadedVerifiedBytes => "loaded_verified_bytes",
            Self::EmptyLifecycleId => "empty_lifecycle_id",
            Self::EmptyLedgerRecordId => "empty_ledger_record_id",
            Self::EmptyLedgerBytes => "empty_ledger_bytes",
            Self::InvalidPersistencePlan => "invalid_persistence_plan",
            Self::PersistenceWriteFailed => "persistence_write_failed",
            Self::VerificationFailed => "verification_failed",
            Self::ChecksumMismatch => "checksum_mismatch",
            Self::MalformedRecord => "malformed_record",
            Self::StaleRevision => "stale_revision",
            Self::UnknownPayloadKind => "unknown_payload_kind",
            Self::RecoveryNotImplemented => "recovery_not_implemented",
            Self::StateRecoveryNotAllowed => "state_recovery_not_allowed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerPersistenceRecord {
    pub lifecycle_id: String,
    pub ledger_record_id: String,
    pub revision: u64,
    pub envelope: PersistedRecordEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerPersistenceReport {
    pub status: LedgerPersistenceStatus,
    pub reason: LedgerPersistenceReason,
    pub lifecycle_id: String,
    pub ledger_record_id: String,
    pub revision: Option<u64>,
    pub payload_len: Option<usize>,
    pub checksum: Option<String>,
    pub usable_for_state_recovery: bool,
    pub mutates_application_state: bool,
    pub summary: String,
}

pub fn prepare_ledger_persistence_record(
    lifecycle_id: impl Into<String>,
    ledger_record_id: impl Into<String>,
    revision: u64,
    ledger_bytes: Vec<u8>,
) -> Result<LedgerPersistenceRecord, LedgerPersistenceReason> {
    let lifecycle_id = lifecycle_id.into();
    if lifecycle_id.is_empty() {
        return Err(LedgerPersistenceReason::EmptyLifecycleId);
    }
    let ledger_record_id = ledger_record_id.into();
    if ledger_record_id.is_empty() {
        return Err(LedgerPersistenceReason::EmptyLedgerRecordId);
    }
    if ledger_bytes.is_empty() {
        return Err(LedgerPersistenceReason::EmptyLedgerBytes);
    }
    let envelope = PersistedRecordEnvelope::new(
        ledger_record_id.clone(),
        LocalPersistencePayloadKind::LedgerSnapshot,
        revision,
        ledger_bytes,
    )
    .map_err(|_| LedgerPersistenceReason::MalformedRecord)?;
    Ok(LedgerPersistenceRecord {
        lifecycle_id,
        ledger_record_id,
        revision,
        envelope,
    })
}

pub fn write_ledger_persistence_record(
    record: &LedgerPersistenceRecord,
    plan: &LocalPersistencePlan,
) -> LedgerPersistenceReport {
    let validation = validate_local_persistence_plan(plan);
    if !validation.valid {
        return ledger_rejected_report(
            record,
            LedgerPersistenceReason::InvalidPersistencePlan,
            "invalid persistence plan",
        );
    }
    let encoded = encode_persisted_record_envelope(&record.envelope);
    match execute_local_persistence_plan(plan, &encoded) {
        Ok(()) => LedgerPersistenceReport {
            status: LedgerPersistenceStatus::Written,
            reason: LedgerPersistenceReason::WrittenThroughPersistenceBoundary,
            lifecycle_id: record.lifecycle_id.clone(),
            ledger_record_id: record.ledger_record_id.clone(),
            revision: Some(record.revision),
            payload_len: Some(record.envelope.payload_len),
            checksum: Some(record.envelope.checksum.clone()),
            usable_for_state_recovery: false,
            mutates_application_state: false,
            summary: "ledger record written through persistence boundary".to_string(),
        },
        Err(_) => ledger_rejected_report(
            record,
            LedgerPersistenceReason::PersistenceWriteFailed,
            "persistence write failed",
        ),
    }
}

pub fn verify_ledger_persistence_paths(
    lifecycle_id: impl Into<String>,
    ledger_record_id: impl Into<String>,
    target_path: impl AsRef<std::path::Path>,
    temp_path: impl AsRef<std::path::Path>,
    expected_revision: Option<u64>,
) -> LedgerPersistenceReport {
    let lifecycle_id = lifecycle_id.into();
    let ledger_record_id = ledger_record_id.into();
    let verification = verify_persisted_record_paths(target_path, temp_path, expected_revision);
    let (status, reason) = match verification.status {
        PersistedRecordVerificationStatus::Valid => (
            LedgerPersistenceStatus::Verified,
            LedgerPersistenceReason::VerifiedForLoad,
        ),
        PersistedRecordVerificationStatus::ChecksumMismatch => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::ChecksumMismatch,
        ),
        PersistedRecordVerificationStatus::UnknownPayloadKind => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::UnknownPayloadKind,
        ),
        PersistedRecordVerificationStatus::StaleRevision => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::StaleRevision,
        ),
        PersistedRecordVerificationStatus::MalformedRecord
        | PersistedRecordVerificationStatus::InvalidPayloadHex
        | PersistedRecordVerificationStatus::PayloadLengthMismatch => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::MalformedRecord,
        ),
        _ => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::VerificationFailed,
        ),
    };
    LedgerPersistenceReport {
        status,
        reason,
        lifecycle_id,
        ledger_record_id,
        revision: verification.revision,
        payload_len: verification.payload_len,
        checksum: verification.checksum,
        usable_for_state_recovery: false,
        mutates_application_state: false,
        summary: verification.summary,
    }
}

pub fn load_verified_ledger_record_bytes(
    envelope: &PersistedRecordEnvelope,
    expected_revision: Option<u64>,
) -> Result<Vec<u8>, LedgerPersistenceReason> {
    let report = verify_persisted_record_envelope(envelope, expected_revision);
    match report.status {
        PersistedRecordVerificationStatus::Valid => {
            if envelope.payload_kind == LocalPersistencePayloadKind::LedgerSnapshot {
                Ok(envelope.payload.clone())
            } else {
                Err(LedgerPersistenceReason::UnknownPayloadKind)
            }
        }
        PersistedRecordVerificationStatus::ChecksumMismatch => {
            Err(LedgerPersistenceReason::ChecksumMismatch)
        }
        PersistedRecordVerificationStatus::UnknownPayloadKind => {
            Err(LedgerPersistenceReason::UnknownPayloadKind)
        }
        PersistedRecordVerificationStatus::StaleRevision => {
            Err(LedgerPersistenceReason::StaleRevision)
        }
        PersistedRecordVerificationStatus::PayloadLengthMismatch
        | PersistedRecordVerificationStatus::MalformedRecord => {
            Err(LedgerPersistenceReason::MalformedRecord)
        }
        _ => Err(LedgerPersistenceReason::VerificationFailed),
    }
}

fn ledger_rejected_report(
    record: &LedgerPersistenceRecord,
    reason: LedgerPersistenceReason,
    summary: &str,
) -> LedgerPersistenceReport {
    LedgerPersistenceReport {
        status: LedgerPersistenceStatus::Rejected,
        reason,
        lifecycle_id: record.lifecycle_id.clone(),
        ledger_record_id: record.ledger_record_id.clone(),
        revision: Some(record.revision),
        payload_len: Some(record.envelope.payload_len),
        checksum: Some(record.envelope.checksum.clone()),
        usable_for_state_recovery: false,
        mutates_application_state: false,
        summary: summary.to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurableAppendStatus {
    Prepared,
    Written,
    Verified,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurableAppendReason {
    PreparedForAppend,
    WrittenThroughPersistenceBoundary,
    VerifiedAppendTransaction,
    EmptyAppendTransactionId,
    EmptyAuditRecordId,
    EmptyLedgerRecordId,
    EmptyAuditPayload,
    EmptyLedgerPayload,
    InvalidRevisionChain,
    TransactionIdMismatch,
    AuditOnlyAppendRejected,
    LedgerOnlyAppendRejected,
    InvalidPersistencePlan,
    AppendWriteFailed,
    AppendVerificationFailed,
    AppendChecksumMismatch,
    MalformedAppendTransaction,
    PromotionNotAllowed,
    RecoveryNotAllowed,
    ReplayRepairNotAllowed,
    ActionExecutionNotAllowed,
    ApplicationStateMutationNotAllowed,
}
impl DurableAppendReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::PreparedForAppend => "prepared_for_append",
            Self::WrittenThroughPersistenceBoundary => "written_through_persistence_boundary",
            Self::VerifiedAppendTransaction => "verified_append_transaction",
            Self::EmptyAppendTransactionId => "empty_append_transaction_id",
            Self::EmptyAuditRecordId => "empty_audit_record_id",
            Self::EmptyLedgerRecordId => "empty_ledger_record_id",
            Self::EmptyAuditPayload => "empty_audit_payload",
            Self::EmptyLedgerPayload => "empty_ledger_payload",
            Self::InvalidRevisionChain => "invalid_revision_chain",
            Self::TransactionIdMismatch => "transaction_id_mismatch",
            Self::AuditOnlyAppendRejected => "audit_only_append_rejected",
            Self::LedgerOnlyAppendRejected => "ledger_only_append_rejected",
            Self::InvalidPersistencePlan => "invalid_persistence_plan",
            Self::AppendWriteFailed => "append_write_failed",
            Self::AppendVerificationFailed => "append_verification_failed",
            Self::AppendChecksumMismatch => "append_checksum_mismatch",
            Self::MalformedAppendTransaction => "malformed_append_transaction",
            Self::PromotionNotAllowed => "promotion_not_allowed",
            Self::RecoveryNotAllowed => "recovery_not_allowed",
            Self::ReplayRepairNotAllowed => "replay_repair_not_allowed",
            Self::ActionExecutionNotAllowed => "action_execution_not_allowed",
            Self::ApplicationStateMutationNotAllowed => "application_state_mutation_not_allowed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableAppendTransaction {
    pub append_transaction_id: String,
    pub audit_record_id: String,
    pub ledger_record_id: String,
    pub prior_revision: u64,
    pub next_revision: u64,
    pub audit_payload: Vec<u8>,
    pub ledger_payload: Vec<u8>,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableAppendReport {
    pub status: DurableAppendStatus,
    pub reason: DurableAppendReason,
    pub append_transaction_id: String,
    pub audit_record_id: String,
    pub ledger_record_id: String,
    pub prior_revision: Option<u64>,
    pub next_revision: Option<u64>,
    pub audit_payload_len: Option<usize>,
    pub ledger_payload_len: Option<usize>,
    pub checksum: Option<String>,
    pub committed: bool,
    pub promoted: bool,
    pub recovered_state: bool,
    pub repaired_replay: bool,
    pub trusted_provider_output: bool,
    pub executed_action: bool,
    pub mutated_application_state: bool,
    pub summary: String,
}

fn compute_durable_append_checksum(content: &[u8]) -> String {
    calculate_persisted_record_checksum(content)
}

pub fn prepare_durable_append_transaction(
    append_transaction_id: impl Into<String>,
    audit_record_id: impl Into<String>,
    ledger_record_id: impl Into<String>,
    prior_revision: u64,
    next_revision: u64,
    audit_payload: Vec<u8>,
    ledger_payload: Vec<u8>,
) -> Result<DurableAppendTransaction, DurableAppendReason> {
    let append_transaction_id = append_transaction_id.into();
    if append_transaction_id.is_empty() {
        return Err(DurableAppendReason::EmptyAppendTransactionId);
    }
    let audit_record_id = audit_record_id.into();
    if audit_record_id.is_empty() {
        return Err(DurableAppendReason::EmptyAuditRecordId);
    }
    let ledger_record_id = ledger_record_id.into();
    if ledger_record_id.is_empty() {
        return Err(DurableAppendReason::EmptyLedgerRecordId);
    }
    if audit_payload.is_empty() {
        return Err(DurableAppendReason::EmptyAuditPayload);
    }
    if ledger_payload.is_empty() {
        return Err(DurableAppendReason::EmptyLedgerPayload);
    }
    if next_revision != prior_revision + 1 {
        return Err(DurableAppendReason::InvalidRevisionChain);
    }
    let mut c = Vec::new();
    c.extend_from_slice(
        format!(
            "{}
{}
{}
{}
{}
{}
{}",
            append_transaction_id,
            audit_record_id,
            ledger_record_id,
            prior_revision,
            next_revision,
            hex_encode(&audit_payload),
            hex_encode(&ledger_payload)
        )
        .as_bytes(),
    );
    let checksum = compute_durable_append_checksum(&c);
    Ok(DurableAppendTransaction {
        append_transaction_id,
        audit_record_id,
        ledger_record_id,
        prior_revision,
        next_revision,
        audit_payload,
        ledger_payload,
        checksum,
    })
}

pub fn encode_durable_append_transaction(transaction: &DurableAppendTransaction) -> Vec<u8> {
    format!(
        "append_transaction_id={}
audit_record_id={}
ledger_record_id={}
prior_revision={}
next_revision={}
audit_payload_hex={}
ledger_payload_hex={}
checksum={}
",
        transaction.append_transaction_id,
        transaction.audit_record_id,
        transaction.ledger_record_id,
        transaction.prior_revision,
        transaction.next_revision,
        hex_encode(&transaction.audit_payload),
        hex_encode(&transaction.ledger_payload),
        transaction.checksum
    )
    .into_bytes()
}

pub fn decode_durable_append_transaction(
    bytes: &[u8],
) -> Result<DurableAppendTransaction, DurableAppendReason> {
    let text =
        std::str::from_utf8(bytes).map_err(|_| DurableAppendReason::MalformedAppendTransaction)?;
    let mut tid = None;
    let mut aid = None;
    let mut lid = None;
    let mut pr = None;
    let mut nr = None;
    let mut ahex = None;
    let mut lhex = None;
    let mut checksum = None;
    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        let (k, v) = line
            .split_once('=')
            .ok_or(DurableAppendReason::MalformedAppendTransaction)?;
        match k {
            "append_transaction_id" => {
                if tid.is_some() {
                    return Err(DurableAppendReason::MalformedAppendTransaction);
                }
                tid = Some(v.to_string());
            }
            "audit_record_id" => {
                if aid.is_some() {
                    return Err(DurableAppendReason::MalformedAppendTransaction);
                }
                aid = Some(v.to_string());
            }
            "ledger_record_id" => {
                if lid.is_some() {
                    return Err(DurableAppendReason::MalformedAppendTransaction);
                }
                lid = Some(v.to_string());
            }
            "prior_revision" => {
                if pr.is_some() {
                    return Err(DurableAppendReason::MalformedAppendTransaction);
                }
                pr = Some(
                    v.parse::<u64>()
                        .map_err(|_| DurableAppendReason::MalformedAppendTransaction)?,
                )
            }
            "next_revision" => {
                if nr.is_some() {
                    return Err(DurableAppendReason::MalformedAppendTransaction);
                }
                nr = Some(
                    v.parse::<u64>()
                        .map_err(|_| DurableAppendReason::MalformedAppendTransaction)?,
                )
            }
            "audit_payload_hex" => {
                if ahex.is_some() {
                    return Err(DurableAppendReason::MalformedAppendTransaction);
                }
                ahex = Some(v.to_string());
            }
            "ledger_payload_hex" => {
                if lhex.is_some() {
                    return Err(DurableAppendReason::MalformedAppendTransaction);
                }
                lhex = Some(v.to_string());
            }
            "checksum" => {
                if checksum.is_some() {
                    return Err(DurableAppendReason::MalformedAppendTransaction);
                }
                checksum = Some(v.to_string());
            }
            _ => return Err(DurableAppendReason::MalformedAppendTransaction),
        }
    }
    let (tid, aid, lid, pr, nr, ahex, lhex, checksum) = (
        tid.ok_or(DurableAppendReason::MalformedAppendTransaction)?,
        aid.ok_or(DurableAppendReason::MalformedAppendTransaction)?,
        lid.ok_or(DurableAppendReason::MalformedAppendTransaction)?,
        pr.ok_or(DurableAppendReason::MalformedAppendTransaction)?,
        nr.ok_or(DurableAppendReason::MalformedAppendTransaction)?,
        ahex.ok_or(DurableAppendReason::MalformedAppendTransaction)?,
        lhex.ok_or(DurableAppendReason::MalformedAppendTransaction)?,
        checksum.ok_or(DurableAppendReason::MalformedAppendTransaction)?,
    );
    if ahex.is_empty() {
        return Err(DurableAppendReason::AuditOnlyAppendRejected);
    }
    if lhex.is_empty() {
        return Err(DurableAppendReason::LedgerOnlyAppendRejected);
    }
    let ap = hex_decode(&ahex).map_err(|_| DurableAppendReason::MalformedAppendTransaction)?;
    let lp = hex_decode(&lhex).map_err(|_| DurableAppendReason::MalformedAppendTransaction)?;
    let tx = prepare_durable_append_transaction(tid, aid, lid, pr, nr, ap, lp)?;
    if tx.checksum != checksum {
        return Err(DurableAppendReason::AppendChecksumMismatch);
    }
    Ok(DurableAppendTransaction { checksum, ..tx })
}

pub fn verify_durable_append_transaction_bytes(
    bytes: &[u8],
    expected_append_transaction_id: impl Into<String>,
) -> DurableAppendReport {
    let expected = expected_append_transaction_id.into();
    match decode_durable_append_transaction(bytes) {
        Ok(tx) => {
            if tx.append_transaction_id != expected {
                return durable_append_report_from_tx(
                    &tx,
                    DurableAppendStatus::Rejected,
                    DurableAppendReason::TransactionIdMismatch,
                    false,
                    "append transaction id mismatch",
                );
            }
            durable_append_report_from_tx(
                &tx,
                DurableAppendStatus::Verified,
                DurableAppendReason::VerifiedAppendTransaction,
                false,
                "verified append transaction bytes",
            )
        }
        Err(reason) => DurableAppendReport {
            status: DurableAppendStatus::Rejected,
            reason,
            append_transaction_id: expected,
            audit_record_id: String::new(),
            ledger_record_id: String::new(),
            prior_revision: None,
            next_revision: None,
            audit_payload_len: None,
            ledger_payload_len: None,
            checksum: None,
            committed: false,
            promoted: false,
            recovered_state: false,
            repaired_replay: false,
            trusted_provider_output: false,
            executed_action: false,
            mutated_application_state: false,
            summary: "append transaction verification failed".to_string(),
        },
    }
}

pub fn write_durable_append_transaction(
    transaction: &DurableAppendTransaction,
    plan: &LocalPersistencePlan,
) -> DurableAppendReport {
    let validation = validate_local_persistence_plan(plan);
    if !validation.valid {
        return durable_append_report_from_tx(
            transaction,
            DurableAppendStatus::Rejected,
            DurableAppendReason::InvalidPersistencePlan,
            false,
            "invalid persistence plan",
        );
    }
    let envelope = match PersistedRecordEnvelope::new(
        transaction.append_transaction_id.clone(),
        LocalPersistencePayloadKind::LedgerSnapshot,
        transaction.next_revision,
        encode_durable_append_transaction(transaction),
    ) {
        Ok(e) => e,
        Err(_) => {
            return durable_append_report_from_tx(
                transaction,
                DurableAppendStatus::Rejected,
                DurableAppendReason::MalformedAppendTransaction,
                false,
                "malformed append envelope",
            )
        }
    };
    let encoded = encode_persisted_record_envelope(&envelope);
    if execute_local_persistence_plan(plan, &encoded).is_err() {
        return durable_append_report_from_tx(
            transaction,
            DurableAppendStatus::Rejected,
            DurableAppendReason::AppendWriteFailed,
            false,
            "append write failed",
        );
    }
    let verification = verify_persisted_record_paths(
        &plan.target_path,
        &plan.temp_path,
        Some(transaction.next_revision),
    );
    if verification.status != PersistedRecordVerificationStatus::Valid {
        return durable_append_report_from_tx(
            transaction,
            DurableAppendStatus::Rejected,
            DurableAppendReason::AppendVerificationFailed,
            false,
            "append verification failed",
        );
    }
    let read = std::fs::read(&plan.target_path).ok();
    if let Some(bytes) = read {
        if let Ok(env) = decode_persisted_record_envelope(&bytes) {
            match verify_durable_append_transaction_bytes(
                &env.payload,
                transaction.append_transaction_id.clone(),
            )
            .reason
            {
                DurableAppendReason::VerifiedAppendTransaction => {
                    return durable_append_report_from_tx(
                        transaction,
                        DurableAppendStatus::Verified,
                        DurableAppendReason::VerifiedAppendTransaction,
                        true,
                        "combined append transaction written and verified",
                    )
                }
                DurableAppendReason::AppendChecksumMismatch => {
                    return durable_append_report_from_tx(
                        transaction,
                        DurableAppendStatus::Rejected,
                        DurableAppendReason::AppendChecksumMismatch,
                        false,
                        "append checksum mismatch",
                    )
                }
                _ => {}
            }
        }
    }
    durable_append_report_from_tx(
        transaction,
        DurableAppendStatus::Rejected,
        DurableAppendReason::AppendVerificationFailed,
        false,
        "append verification failed",
    )
}

fn durable_append_report_from_tx(
    transaction: &DurableAppendTransaction,
    status: DurableAppendStatus,
    reason: DurableAppendReason,
    committed: bool,
    summary: &str,
) -> DurableAppendReport {
    DurableAppendReport {
        status,
        reason,
        append_transaction_id: transaction.append_transaction_id.clone(),
        audit_record_id: transaction.audit_record_id.clone(),
        ledger_record_id: transaction.ledger_record_id.clone(),
        prior_revision: Some(transaction.prior_revision),
        next_revision: Some(transaction.next_revision),
        audit_payload_len: Some(transaction.audit_payload.len()),
        ledger_payload_len: Some(transaction.ledger_payload.len()),
        checksum: Some(transaction.checksum.clone()),
        committed,
        promoted: false,
        recovered_state: false,
        repaired_replay: false,
        trusted_provider_output: false,
        executed_action: false,
        mutated_application_state: false,
        summary: summary.to_string(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    fn temp(name: &str) -> (PathBuf, PathBuf, PathBuf) {
        let mut d = std::env::temp_dir();
        d.push(format!("aj62_{}_{}", name, std::process::id()));
        let _ = fs::create_dir_all(&d);
        (d.clone(), d.join("target.rec"), d.join("temp.rec"))
    }
    fn env() -> PersistedRecordEnvelope {
        PersistedRecordEnvelope::new(
            "r1",
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"abc".to_vec(),
        )
        .unwrap()
    }
    #[test]
    fn persisted_record_checksum_is_deterministic() {
        assert_eq!(
            calculate_persisted_record_checksum(b"abc"),
            calculate_persisted_record_checksum(b"abc")
        );
    }
    #[test]
    fn persisted_record_envelope_error_codes_are_stable() {
        assert_eq!(
            PersistedRecordEnvelopeError::InvalidPayloadHex.code(),
            "invalid_payload_hex"
        );
    }
    #[test]
    fn persisted_record_verification_valid_envelope() {
        assert_eq!(
            verify_persisted_record_envelope(&env(), None).status,
            PersistedRecordVerificationStatus::Valid
        );
    }
    #[test]
    fn persisted_record_envelope_requires_record_id() {
        assert!(matches!(
            PersistedRecordEnvelope::new(
                "",
                LocalPersistencePayloadKind::LedgerSnapshot,
                1,
                vec![1]
            ),
            Err(PersistedRecordEnvelopeError::EmptyRecordId)
        ));
    }
    #[test]
    fn persisted_record_envelope_rejects_unknown_payload_kind() {
        assert!(matches!(
            PersistedRecordEnvelope::new("a", LocalPersistencePayloadKind::Unknown, 1, vec![1]),
            Err(PersistedRecordEnvelopeError::UnknownPayloadKind)
        ));
    }
    #[test]
    fn persisted_record_envelope_rejects_empty_payload() {
        assert!(matches!(
            PersistedRecordEnvelope::new(
                "a",
                LocalPersistencePayloadKind::LedgerSnapshot,
                1,
                vec![]
            ),
            Err(PersistedRecordEnvelopeError::EmptyPayload)
        ));
    }
    #[test]
    fn persisted_record_verification_detects_payload_length_mismatch() {
        let mut e = env();
        e.payload_len = 9;
        assert_eq!(
            verify_persisted_record_envelope(&e, None).status,
            PersistedRecordVerificationStatus::PayloadLengthMismatch
        );
    }
    #[test]
    fn persisted_record_verification_detects_checksum_mismatch() {
        let mut e = env();
        e.checksum = "0000000000000000".into();
        assert_eq!(
            verify_persisted_record_envelope(&e, None).status,
            PersistedRecordVerificationStatus::ChecksumMismatch
        );
    }
    #[test]
    fn persisted_record_verification_detects_stale_revision() {
        assert_eq!(
            verify_persisted_record_envelope(&env(), Some(3)).status,
            PersistedRecordVerificationStatus::StaleRevision
        );
    }
    #[test]
    fn persisted_record_verification_does_not_mutate_envelope() {
        let e = env();
        let c = e.clone();
        let _ = verify_persisted_record_envelope(&e, None);
        assert_eq!(e, c);
    }
    #[test]
    fn persisted_record_encode_decode_round_trip() {
        let e = env();
        let d = decode_persisted_record_envelope(&encode_persisted_record_envelope(&e)).unwrap();
        assert_eq!(e, d);
    }
    #[test]
    fn persisted_record_decode_rejects_malformed_record() {
        assert!(matches!(
            decode_persisted_record_envelope(b"bad"),
            Err(PersistedRecordEnvelopeError::MalformedRecord)
        ));
    }
    #[test]
    fn persisted_record_decode_rejects_invalid_payload_hex() {
        let s="AJENTIC_RECORD_V1\nrecord_id:r\npayload_kind:ledger_snapshot\nrevision:1\npayload_len:1\nchecksum:af63bc4c8601b62c\npayload_hex:zz\n";
        assert!(matches!(
            decode_persisted_record_envelope(s.as_bytes()),
            Err(PersistedRecordEnvelopeError::InvalidPayloadHex)
        ));
    }
    #[test]
    fn persisted_record_decode_rejects_payload_length_mismatch() {
        let e = env();
        let mut s = String::from_utf8(encode_persisted_record_envelope(&e)).unwrap();
        s = s.replace("payload_len:3", "payload_len:4");
        assert!(matches!(
            decode_persisted_record_envelope(s.as_bytes()),
            Err(PersistedRecordEnvelopeError::PayloadLengthMismatch)
        ));
    }
    #[test]
    fn persisted_record_decode_rejects_checksum_mismatch() {
        let e = env();
        let mut s = String::from_utf8(encode_persisted_record_envelope(&e)).unwrap();
        s = s.replace(
            &format!("checksum:{}", e.checksum),
            "checksum:0000000000000000",
        );
        assert!(matches!(
            decode_persisted_record_envelope(s.as_bytes()),
            Err(PersistedRecordEnvelopeError::ChecksumMismatch)
        ));
    }
    #[test]
    fn verify_persisted_record_paths_reports_missing_target_when_no_files() {
        let (d, t, tmp) = temp("missing");
        let _ = fs::remove_file(&t);
        let _ = fs::remove_file(&tmp);
        assert_eq!(
            verify_persisted_record_paths(&t, &tmp, None).status,
            PersistedRecordVerificationStatus::MissingTarget
        );
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_persisted_record_paths_reports_orphaned_temp_when_temp_exists_without_target() {
        let (d, t, tmp) = temp("orphan1");
        fs::write(&tmp, b"x").unwrap();
        assert_eq!(
            verify_persisted_record_paths(&t, &tmp, None).status,
            PersistedRecordVerificationStatus::OrphanedTemp
        );
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_persisted_record_paths_reports_orphaned_temp_when_target_and_temp_exist() {
        let (d, t, tmp) = temp("orphan2");
        fs::write(&t, b"x").unwrap();
        fs::write(&tmp, b"x").unwrap();
        assert_eq!(
            verify_persisted_record_paths(&t, &tmp, None).status,
            PersistedRecordVerificationStatus::OrphanedTemp
        );
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_persisted_record_paths_reports_valid_for_encoded_target_and_no_temp() {
        let (d, t, tmp) = temp("valid");
        fs::write(&t, encode_persisted_record_envelope(&env())).unwrap();
        assert_eq!(
            verify_persisted_record_paths(&t, &tmp, None).status,
            PersistedRecordVerificationStatus::Valid
        );
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_persisted_record_paths_reports_malformed_for_arbitrary_target_bytes() {
        let (d, t, tmp) = temp("mal");
        fs::write(&t, b"abc").unwrap();
        assert_eq!(
            verify_persisted_record_paths(&t, &tmp, None).status,
            PersistedRecordVerificationStatus::MalformedRecord
        );
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_persisted_record_paths_reports_invalid_payload_hex_for_bad_payload_hex() {
        let (d, t, tmp) = temp("badhex");
        let s="AJENTIC_RECORD_V1\nrecord_id:r\npayload_kind:ledger_snapshot\nrevision:1\npayload_len:1\nchecksum:af63bc4c8601b62c\npayload_hex:gg\n";
        fs::write(&t, s).unwrap();
        assert_eq!(
            verify_persisted_record_paths(&t, &tmp, None).status,
            PersistedRecordVerificationStatus::InvalidPayloadHex
        );
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_persisted_record_paths_reports_checksum_mismatch_for_tampered_target() {
        let (d, t, tmp) = temp("tamper");
        let e = env();
        let mut s = String::from_utf8(encode_persisted_record_envelope(&e)).unwrap();
        s = s.replace("payload_hex:616263", "payload_hex:646566");
        fs::write(&t, s).unwrap();
        assert_eq!(
            verify_persisted_record_paths(&t, &tmp, None).status,
            PersistedRecordVerificationStatus::ChecksumMismatch
        );
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_persisted_record_paths_does_not_delete_or_repair_orphaned_temp() {
        let (d, t, tmp) = temp("nodel");
        fs::write(&tmp, b"x").unwrap();
        let _ = verify_persisted_record_paths(&t, &tmp, None);
        assert!(tmp.exists());
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_persisted_record_paths_does_not_rewrite_target() {
        let (d, t, tmp) = temp("norewrite");
        let bytes = encode_persisted_record_envelope(&env());
        fs::write(&t, &bytes).unwrap();
        let _ = verify_persisted_record_paths(&t, &tmp, None);
        assert_eq!(fs::read(&t).unwrap(), bytes);
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn recovery_action_is_manual_review_for_corruption() {
        let mut e = env();
        e.payload_len = 0;
        assert_eq!(
            verify_persisted_record_envelope(&e, None).recovery_action,
            PersistedRecordRecoveryAction::ManualReviewRequired
        );
    }
    #[test]
    fn recovery_action_is_none_for_valid_record() {
        assert_eq!(
            verify_persisted_record_envelope(&env(), None).recovery_action,
            PersistedRecordRecoveryAction::None
        );
    }

    fn ledger_record_fixture() -> LedgerPersistenceRecord {
        prepare_ledger_persistence_record("lifecycle-1", "ledger-1", 3, b"ledger".to_vec()).unwrap()
    }
    #[test]
    fn ledger_persistence_reason_codes_are_stable() {
        assert_eq!(
            LedgerPersistenceReason::PreparedForPersistence.code(),
            "prepared_for_persistence"
        );
        assert_eq!(
            LedgerPersistenceReason::StateRecoveryNotAllowed.code(),
            "state_recovery_not_allowed"
        );
    }
    #[test]
    fn prepare_ledger_record_requires_lifecycle_id() {
        assert_eq!(
            prepare_ledger_persistence_record("", "a", 1, vec![1]),
            Err(LedgerPersistenceReason::EmptyLifecycleId)
        );
    }
    #[test]
    fn prepare_ledger_record_requires_ledger_record_id() {
        assert_eq!(
            prepare_ledger_persistence_record("a", "", 1, vec![1]),
            Err(LedgerPersistenceReason::EmptyLedgerRecordId)
        );
    }
    #[test]
    fn prepare_ledger_record_requires_ledger_bytes() {
        assert_eq!(
            prepare_ledger_persistence_record("a", "b", 1, vec![]),
            Err(LedgerPersistenceReason::EmptyLedgerBytes)
        );
    }
    #[test]
    fn prepare_ledger_record_uses_ledger_snapshot_payload_kind() {
        assert_eq!(
            ledger_record_fixture().envelope.payload_kind,
            LocalPersistencePayloadKind::LedgerSnapshot
        );
    }
    #[test]
    fn prepare_ledger_record_does_not_serialize_application_state() {
        assert!(!String::from_utf8(encode_persisted_record_envelope(
            &ledger_record_fixture().envelope
        ))
        .unwrap()
        .contains("local_application_state"));
    }
    #[test]
    fn write_ledger_record_uses_existing_persistence_boundary() {
        let (d, t, tmp) = temp("ledger_write");
        let record = ledger_record_fixture();
        let plan = LocalPersistencePlan::new(
            "p",
            t.to_string_lossy(),
            tmp.to_string_lossy(),
            Some(record.revision),
            LocalPersistencePayloadKind::LedgerSnapshot,
            LocalPersistenceWriteMode::ReplaceExisting,
            LocalPersistenceAtomicity::Required,
        );
        let report = write_ledger_persistence_record(&record, &plan);
        assert_eq!(
            report.reason,
            LedgerPersistenceReason::WrittenThroughPersistenceBoundary
        );
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn write_ledger_record_rejects_invalid_plan() {
        let record = ledger_record_fixture();
        let plan = LocalPersistencePlan::new(
            "",
            "a",
            "b",
            Some(1),
            LocalPersistencePayloadKind::LedgerSnapshot,
            LocalPersistenceWriteMode::ReplaceExisting,
            LocalPersistenceAtomicity::Required,
        );
        let report = write_ledger_persistence_record(&record, &plan);
        assert_eq!(
            report.reason,
            LedgerPersistenceReason::InvalidPersistencePlan
        );
    }
    #[test]
    fn write_ledger_record_writes_encoded_envelope() {
        let (d, t, tmp) = temp("ledger_encoded");
        let record = ledger_record_fixture();
        let plan = LocalPersistencePlan::new(
            "p2",
            t.to_string_lossy(),
            tmp.to_string_lossy(),
            Some(record.revision),
            LocalPersistencePayloadKind::LedgerSnapshot,
            LocalPersistenceWriteMode::ReplaceExisting,
            LocalPersistenceAtomicity::Required,
        );
        let _ = write_ledger_persistence_record(&record, &plan);
        let on_disk = fs::read(&t).unwrap();
        assert_eq!(on_disk, encode_persisted_record_envelope(&record.envelope));
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_ledger_paths_accepts_valid_record() {
        let (d, t, tmp) = temp("ledger_verify_valid");
        let record = ledger_record_fixture();
        fs::write(&t, encode_persisted_record_envelope(&record.envelope)).unwrap();
        let report = verify_ledger_persistence_paths("l", "r", &t, &tmp, Some(record.revision));
        assert_eq!(report.reason, LedgerPersistenceReason::VerifiedForLoad);
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_ledger_paths_rejects_checksum_mismatch() {
        let (d, t, tmp) = temp("ledger_verify_checksum");
        let record = ledger_record_fixture();
        let mut s = String::from_utf8(encode_persisted_record_envelope(&record.envelope)).unwrap();
        s = s.replace("payload_hex:6c6564676572", "payload_hex:616263646566");
        fs::write(&t, s).unwrap();
        let report = verify_ledger_persistence_paths("l", "r", &t, &tmp, None);
        assert_eq!(report.reason, LedgerPersistenceReason::ChecksumMismatch);
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_ledger_paths_rejects_unknown_payload_kind() {
        let (d, t, tmp) = temp("ledger_verify_unknown");
        let s="AJENTIC_RECORD_V1\nrecord_id:r\npayload_kind:unknown\nrevision:1\npayload_len:1\nchecksum:af63bc4c8601b62c\npayload_hex:61\n";
        fs::write(&t, s).unwrap();
        let report = verify_ledger_persistence_paths("l", "r", &t, &tmp, None);
        assert_eq!(report.reason, LedgerPersistenceReason::UnknownPayloadKind);
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_ledger_paths_rejects_malformed_record() {
        let (d, t, tmp) = temp("ledger_verify_malformed");
        fs::write(&t, b"not-record").unwrap();
        let report = verify_ledger_persistence_paths("l", "r", &t, &tmp, None);
        assert_eq!(report.reason, LedgerPersistenceReason::MalformedRecord);
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_ledger_paths_rejects_stale_revision() {
        let (d, t, tmp) = temp("ledger_verify_stale");
        let record = ledger_record_fixture();
        fs::write(&t, encode_persisted_record_envelope(&record.envelope)).unwrap();
        let report = verify_ledger_persistence_paths("l", "r", &t, &tmp, Some(record.revision + 1));
        assert_eq!(report.reason, LedgerPersistenceReason::StaleRevision);
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn verify_ledger_paths_rejects_missing_or_orphaned_records_as_verification_failed() {
        let (d, t, tmp) = temp("ledger_verify_missing");
        let report_missing = verify_ledger_persistence_paths("l", "r", &t, &tmp, None);
        assert_eq!(
            report_missing.reason,
            LedgerPersistenceReason::VerificationFailed
        );
        fs::write(&tmp, b"x").unwrap();
        let report_orphan = verify_ledger_persistence_paths("l", "r", &t, &tmp, None);
        assert_eq!(
            report_orphan.reason,
            LedgerPersistenceReason::VerificationFailed
        );
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn load_verified_ledger_record_bytes_returns_bytes_for_valid_envelope() {
        let record = ledger_record_fixture();
        let bytes =
            load_verified_ledger_record_bytes(&record.envelope, Some(record.revision)).unwrap();
        assert_eq!(bytes, b"ledger".to_vec());
    }
    #[test]
    fn load_verified_ledger_record_bytes_rejects_non_ledger_payload_kind() {
        let envelope = PersistedRecordEnvelope::new(
            "r",
            LocalPersistencePayloadKind::RunRecord,
            1,
            b"x".to_vec(),
        )
        .unwrap();
        assert_eq!(
            load_verified_ledger_record_bytes(&envelope, None),
            Err(LedgerPersistenceReason::UnknownPayloadKind)
        );
    }
    #[test]
    fn load_verified_ledger_record_bytes_rejects_checksum_mismatch() {
        let mut envelope = ledger_record_fixture().envelope;
        envelope.checksum = "0000000000000000".into();
        assert_eq!(
            load_verified_ledger_record_bytes(&envelope, None),
            Err(LedgerPersistenceReason::ChecksumMismatch)
        );
    }
    #[test]
    fn ledger_persistence_does_not_recover_application_state() {
        let report =
            verify_ledger_persistence_paths("l", "r", "/tmp/missing-a", "/tmp/missing-b", None);
        assert!(!report.usable_for_state_recovery);
    }
    #[test]
    fn ledger_persistence_does_not_mutate_application_state() {
        let report =
            verify_ledger_persistence_paths("l", "r", "/tmp/missing-c", "/tmp/missing-d", None);
        assert!(!report.mutates_application_state);
    }
    #[test]
    fn ledger_persistence_does_not_execute_provider_output() {
        let report =
            verify_ledger_persistence_paths("l", "r", "/tmp/missing-e", "/tmp/missing-f", None);
        assert!(!report.summary.contains("provider execution"));
    }
    #[test]
    fn ledger_persistence_does_not_record_provider_retry() {
        let report =
            verify_ledger_persistence_paths("l", "r", "/tmp/missing-g", "/tmp/missing-h", None);
        assert!(!report.summary.contains("retry"));
    }
    #[test]
    fn ledger_persistence_does_not_repair_replay() {
        let report =
            verify_ledger_persistence_paths("l", "r", "/tmp/missing-i", "/tmp/missing-j", None);
        assert!(!report.summary.contains("repair"));
    }
    #[test]
    fn ledger_persistence_does_not_promote() {
        let report =
            verify_ledger_persistence_paths("l", "r", "/tmp/missing-k", "/tmp/missing-l", None);
        assert!(!report.summary.contains("promote"));
    }
    #[test]
    fn dry_run_does_not_write_or_load_ledger_persistence() {
        let report = verify_ledger_persistence_paths(
            "dry-run",
            "record",
            "/tmp/missing-m",
            "/tmp/missing-n",
            None,
        );
        assert_eq!(report.status, LedgerPersistenceStatus::Rejected);
    }
}

#[cfg(test)]
mod diagnostic_tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn local_persistence_validation_reason_diagnostic_preserves_code() {
        let reason = LocalPersistenceValidationReason::EmptyPlanId;
        let diagnostic = crate::api::local_persistence_validation_reason_diagnostic(reason);
        assert_eq!(diagnostic.code, reason.code());
    }

    #[test]
    fn local_persistence_error_diagnostic_preserves_code() {
        let error = LocalPersistenceError::PhysicalWriteNotImplemented;
        let diagnostic = crate::api::local_persistence_error_diagnostic(error);
        assert_eq!(diagnostic.code, error.code());
    }

    #[test]
    fn persisted_record_envelope_error_diagnostic_preserves_code() {
        let error = PersistedRecordEnvelopeError::ChecksumMismatch;
        let diagnostic = crate::api::persisted_record_envelope_error_diagnostic(error.clone());
        assert_eq!(diagnostic.code, error.code());
    }

    fn diagnostic_temp(name: &str) -> (PathBuf, PathBuf, PathBuf) {
        let mut directory = std::env::temp_dir();
        directory.push(format!("aj93_{name}"));
        let _ = fs::create_dir_all(&directory);
        (
            directory.clone(),
            directory.join("target.rec"),
            directory.join("temp.rec"),
        )
    }

    fn persisted_env() -> PersistedRecordEnvelope {
        PersistedRecordEnvelope::new(
            "r1",
            LocalPersistencePayloadKind::LedgerSnapshot,
            2,
            b"abc".to_vec(),
        )
        .unwrap()
    }

    fn append_tx() -> DurableAppendTransaction {
        prepare_durable_append_transaction(
            "append-93",
            "audit-93",
            "ledger-93",
            7,
            8,
            b"audit".to_vec(),
            b"ledger".to_vec(),
        )
        .unwrap()
    }

    fn append_plan(name: &str) -> (PathBuf, LocalPersistencePlan) {
        let (d, target, temp) = diagnostic_temp(name);
        let plan = LocalPersistencePlan::new(
            format!("plan-{name}"),
            target.to_string_lossy().to_string(),
            temp.to_string_lossy().to_string(),
            Some(8),
            LocalPersistencePayloadKind::LedgerSnapshot,
            LocalPersistenceWriteMode::CreateNew,
            LocalPersistenceAtomicity::Required,
        );
        (d, plan)
    }

    fn encoded_append_text() -> String {
        String::from_utf8(encode_durable_append_transaction(&append_tx())).unwrap()
    }

    #[test]
    fn persisted_record_rejects_payload_length_mismatch() {
        let e = persisted_env();
        let text = String::from_utf8(encode_persisted_record_envelope(&e))
            .unwrap()
            .replace("payload_len:3", "payload_len:4");
        assert_eq!(
            decode_persisted_record_envelope(text.as_bytes()),
            Err(PersistedRecordEnvelopeError::PayloadLengthMismatch)
        );
    }

    #[test]
    fn persisted_record_rejects_invalid_payload_hex() {
        let text = "AJENTIC_RECORD_V1
record_id:r
payload_kind:ledger_snapshot
revision:1
payload_len:1
checksum:af63bc4c8601b62c
payload_hex:zz
";
        assert_eq!(
            decode_persisted_record_envelope(text.as_bytes()),
            Err(PersistedRecordEnvelopeError::InvalidPayloadHex)
        );
    }

    #[test]
    fn persisted_record_rejects_checksum_mismatch() {
        let e = persisted_env();
        let text = String::from_utf8(encode_persisted_record_envelope(&e))
            .unwrap()
            .replace(
                &format!("checksum:{}", e.checksum),
                "checksum:0000000000000000",
            );
        assert_eq!(
            decode_persisted_record_envelope(text.as_bytes()),
            Err(PersistedRecordEnvelopeError::ChecksumMismatch)
        );
    }

    #[test]
    fn durable_append_decode_rejects_checksum_drift() {
        let text = encoded_append_text().replace("checksum=", "checksum=0");
        assert_eq!(
            decode_durable_append_transaction(text.as_bytes()),
            Err(DurableAppendReason::AppendChecksumMismatch)
        );
    }

    #[test]
    fn durable_append_decode_rejects_missing_audit_payload() {
        let text =
            encoded_append_text().replace("audit_payload_hex=6175646974", "audit_payload_hex=");
        assert_eq!(
            decode_durable_append_transaction(text.as_bytes()),
            Err(DurableAppendReason::AuditOnlyAppendRejected)
        );
    }

    #[test]
    fn durable_append_decode_rejects_missing_ledger_payload() {
        let text =
            encoded_append_text().replace("ledger_payload_hex=6c6564676572", "ledger_payload_hex=");
        assert_eq!(
            decode_durable_append_transaction(text.as_bytes()),
            Err(DurableAppendReason::LedgerOnlyAppendRejected)
        );
    }

    #[test]
    fn durable_append_decode_rejects_malformed_revision() {
        let text =
            encoded_append_text().replace("prior_revision=7", "prior_revision=not-a-revision");
        assert_eq!(
            decode_durable_append_transaction(text.as_bytes()),
            Err(DurableAppendReason::MalformedAppendTransaction)
        );
    }

    #[test]
    fn durable_append_decode_rejects_stale_revision_chain() {
        let text = encoded_append_text().replace("next_revision=8", "next_revision=9");
        assert_eq!(
            decode_durable_append_transaction(text.as_bytes()),
            Err(DurableAppendReason::InvalidRevisionChain)
        );
    }

    #[test]
    fn durable_append_verify_rejects_transaction_id_mismatch() {
        let report = verify_durable_append_transaction_bytes(
            &encode_durable_append_transaction(&append_tx()),
            "other-append",
        );
        assert_eq!(report.status, DurableAppendStatus::Rejected);
        assert_eq!(report.reason, DurableAppendReason::TransactionIdMismatch);
        assert!(!report.committed);
    }

    #[test]
    fn durable_append_tampered_bytes_are_not_committed() {
        let text = encoded_append_text().replace(
            "audit_payload_hex=6175646974",
            "audit_payload_hex=6175646975",
        );
        let report = verify_durable_append_transaction_bytes(text.as_bytes(), "append-93");
        assert_eq!(report.reason, DurableAppendReason::AppendChecksumMismatch);
        assert!(!report.committed);
    }

    #[test]
    fn durable_append_partial_write_is_not_committed_if_simulatable() {
        let (d, target, temp_path) = diagnostic_temp("append_partial_write");
        fs::write(&temp_path, b"partial append bytes").unwrap();
        let verification = verify_persisted_record_paths(&target, &temp_path, Some(8));
        assert_eq!(
            verification.status,
            PersistedRecordVerificationStatus::OrphanedTemp
        );
        assert!(!target.exists());
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn durable_append_failed_write_is_not_committed() {
        let tx = append_tx();
        let plan = LocalPersistencePlan::new(
            "bad-plan",
            "",
            "",
            Some(8),
            LocalPersistencePayloadKind::LedgerSnapshot,
            LocalPersistenceWriteMode::CreateNew,
            LocalPersistenceAtomicity::Required,
        );
        let report = write_durable_append_transaction(&tx, &plan);
        assert_eq!(report.reason, DurableAppendReason::InvalidPersistencePlan);
        assert!(!report.committed);
    }

    #[test]
    fn durable_append_verification_failure_is_not_committed() {
        let report =
            verify_durable_append_transaction_bytes(b"not an append transaction", "append-93");
        assert_eq!(report.status, DurableAppendStatus::Rejected);
        assert_eq!(
            report.reason,
            DurableAppendReason::MalformedAppendTransaction
        );
        assert!(!report.committed);
    }

    #[test]
    fn durable_append_success_path_still_requires_complete_audit_and_ledger_payloads() {
        let tx = append_tx();
        assert!(!tx.audit_payload.is_empty());
        assert!(!tx.ledger_payload.is_empty());
        let (d, plan) = append_plan("append_success_complete");
        let report = write_durable_append_transaction(&tx, &plan);
        assert_eq!(report.status, DurableAppendStatus::Verified);
        assert_eq!(
            report.reason,
            DurableAppendReason::VerifiedAppendTransaction
        );
        assert!(report.committed);
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn append_revision_must_advance_by_one() {
        assert_eq!(
            prepare_durable_append_transaction("t", "a", "l", 4, 6, vec![1], vec![2]),
            Err(DurableAppendReason::InvalidRevisionChain)
        );
    }

    #[test]
    fn append_prior_revision_mismatch_rejects() {
        let text = encoded_append_text().replace("prior_revision=7", "prior_revision=6");
        assert_eq!(
            decode_durable_append_transaction(text.as_bytes()),
            Err(DurableAppendReason::InvalidRevisionChain)
        );
    }

    #[test]
    fn append_next_revision_mismatch_rejects() {
        let text = encoded_append_text().replace("next_revision=8", "next_revision=10");
        assert_eq!(
            decode_durable_append_transaction(text.as_bytes()),
            Err(DurableAppendReason::InvalidRevisionChain)
        );
    }

    #[test]
    fn append_audit_payload_change_after_checksum_rejects() {
        let text = encoded_append_text().replace(
            "audit_payload_hex=6175646974",
            "audit_payload_hex=6175646975",
        );
        assert_eq!(
            decode_durable_append_transaction(text.as_bytes()),
            Err(DurableAppendReason::AppendChecksumMismatch)
        );
    }

    #[test]
    fn append_ledger_payload_change_after_checksum_rejects() {
        let text = encoded_append_text().replace(
            "ledger_payload_hex=6c6564676572",
            "ledger_payload_hex=6c6564676573",
        );
        assert_eq!(
            decode_durable_append_transaction(text.as_bytes()),
            Err(DurableAppendReason::AppendChecksumMismatch)
        );
    }

    #[test]
    fn append_audit_only_transaction_never_verifies() {
        let text =
            encoded_append_text().replace("ledger_payload_hex=6c6564676572", "ledger_payload_hex=");
        let report = verify_durable_append_transaction_bytes(text.as_bytes(), "append-93");
        assert_eq!(report.status, DurableAppendStatus::Rejected);
        assert_eq!(report.reason, DurableAppendReason::LedgerOnlyAppendRejected);
        assert!(!report.committed);
    }

    #[test]
    fn append_ledger_only_transaction_never_verifies() {
        let text =
            encoded_append_text().replace("audit_payload_hex=6175646974", "audit_payload_hex=");
        let report = verify_durable_append_transaction_bytes(text.as_bytes(), "append-93");
        assert_eq!(report.status, DurableAppendStatus::Rejected);
        assert_eq!(report.reason, DurableAppendReason::AuditOnlyAppendRejected);
        assert!(!report.committed);
    }

    #[test]
    fn append_report_does_not_promote_recover_or_repair() {
        let report = verify_durable_append_transaction_bytes(
            &encode_durable_append_transaction(&append_tx()),
            "append-93",
        );
        assert_eq!(report.status, DurableAppendStatus::Verified);
        assert!(!report.promoted);
        assert!(!report.recovered_state);
        assert!(!report.repaired_replay);
    }

    #[test]
    fn durable_append_reason_codes_are_stable() {
        assert_eq!(
            DurableAppendReason::PreparedForAppend.code(),
            "prepared_for_append"
        );
    }
    #[test]
    fn prepare_append_requires_transaction_id() {
        assert_eq!(
            prepare_durable_append_transaction("", "a", "l", 1, 2, vec![1], vec![1]),
            Err(DurableAppendReason::EmptyAppendTransactionId)
        );
    }
    #[test]
    fn prepare_append_requires_audit_record_id() {
        assert_eq!(
            prepare_durable_append_transaction("t", "", "l", 1, 2, vec![1], vec![1]),
            Err(DurableAppendReason::EmptyAuditRecordId)
        );
    }
    #[test]
    fn prepare_append_requires_ledger_record_id() {
        assert_eq!(
            prepare_durable_append_transaction("t", "a", "", 1, 2, vec![1], vec![1]),
            Err(DurableAppendReason::EmptyLedgerRecordId)
        );
    }
    #[test]
    fn prepare_append_requires_audit_payload() {
        assert_eq!(
            prepare_durable_append_transaction("t", "a", "l", 1, 2, vec![], vec![1]),
            Err(DurableAppendReason::EmptyAuditPayload)
        );
    }
    #[test]
    fn prepare_append_requires_ledger_payload() {
        assert_eq!(
            prepare_durable_append_transaction("t", "a", "l", 1, 2, vec![1], vec![]),
            Err(DurableAppendReason::EmptyLedgerPayload)
        );
    }
    #[test]
    fn prepare_append_rejects_invalid_revision_chain() {
        assert_eq!(
            prepare_durable_append_transaction("t", "a", "l", 1, 3, vec![1], vec![1]),
            Err(DurableAppendReason::InvalidRevisionChain)
        );
    }
    #[test]
    fn prepare_append_computes_deterministic_checksum() {
        let a = prepare_durable_append_transaction("t", "a", "l", 1, 2, vec![1], vec![2]).unwrap();
        let b = prepare_durable_append_transaction("t", "a", "l", 1, 2, vec![1], vec![2]).unwrap();
        assert_eq!(a.checksum, b.checksum);
    }
    #[test]
    fn encode_append_transaction_is_deterministic() {
        let t = prepare_durable_append_transaction("t", "a", "l", 1, 2, vec![1], vec![2]).unwrap();
        assert_eq!(
            encode_durable_append_transaction(&t),
            encode_durable_append_transaction(&t)
        );
    }
    #[test]
    fn decode_append_transaction_round_trips() {
        let t = prepare_durable_append_transaction("t", "a", "l", 1, 2, vec![1], vec![2]).unwrap();
        let d = decode_durable_append_transaction(&encode_durable_append_transaction(&t)).unwrap();
        assert_eq!(t, d);
    }
    #[test]
    fn persisted_record_rejects_unknown_payload_kind_or_documents_no_version_field() {
        let text = "AJENTIC_RECORD_V1\nrecord_id:r\npayload_kind:future_payload\nrevision:1\npayload_len:1\nchecksum:af63bc4c8601b62c\npayload_hex:61\n";
        assert_eq!(
            decode_persisted_record_envelope(text.as_bytes()),
            Err(PersistedRecordEnvelopeError::UnknownPayloadKind)
        );
        let versioned_text = text.replace("AJENTIC_RECORD_V1", "AJENTIC_RECORD_V2");
        assert_eq!(
            decode_persisted_record_envelope(versioned_text.as_bytes()),
            Err(PersistedRecordEnvelopeError::MalformedRecord)
        );
    }

    #[test]
    fn durable_append_rejects_unsupported_transaction_kind_if_representable() {
        let text = format!("transaction_kind=future_append\n{}", encoded_append_text());
        assert_eq!(
            decode_durable_append_transaction(text.as_bytes()),
            Err(DurableAppendReason::MalformedAppendTransaction)
        );
    }

    #[test]
    fn durable_append_audit_only_remains_unsupported() {
        let text =
            encoded_append_text().replace("ledger_payload_hex=6c6564676572", "ledger_payload_hex=");
        let report = verify_durable_append_transaction_bytes(text.as_bytes(), "append-93");
        assert_eq!(report.status, DurableAppendStatus::Rejected);
        assert_eq!(report.reason, DurableAppendReason::LedgerOnlyAppendRejected);
        assert!(!report.committed);
    }

    #[test]
    fn durable_append_ledger_only_remains_unsupported() {
        let text =
            encoded_append_text().replace("audit_payload_hex=6175646974", "audit_payload_hex=");
        let report = verify_durable_append_transaction_bytes(text.as_bytes(), "append-93");
        assert_eq!(report.status, DurableAppendStatus::Rejected);
        assert_eq!(report.reason, DurableAppendReason::AuditOnlyAppendRejected);
        assert!(!report.committed);
    }

    #[test]
    fn durable_append_out_of_order_revision_is_drift() {
        let text = encoded_append_text().replace("next_revision=8", "next_revision=9");
        let report = verify_durable_append_transaction_bytes(text.as_bytes(), "append-93");
        assert_eq!(report.status, DurableAppendStatus::Rejected);
        assert_eq!(report.reason, DurableAppendReason::InvalidRevisionChain);
        assert!(!report.committed);
    }

    #[test]
    fn durable_append_success_does_not_claim_continuous_integrity() {
        let tx = append_tx();
        let (directory, plan) = append_plan("write_time_not_continuous");
        let report = write_durable_append_transaction(&tx, &plan);
        assert_eq!(report.status, DurableAppendStatus::Verified);
        assert!(report.committed);

        fs::write(
            &plan.target_path,
            b"external tampering after write-time verification",
        )
        .unwrap();
        let later = verify_persisted_record_paths(&plan.target_path, &plan.temp_path, Some(8));
        assert_eq!(
            later.status,
            PersistedRecordVerificationStatus::MalformedRecord
        );
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn corrupted_persisted_record_decode_does_not_silently_skip() {
        let text = "AJENTIC_RECORD_V1\nrecord_id:r\npayload_kind:ledger_snapshot\nrevision:1\npayload_len:1\nchecksum:af63bc4c8601b62c\npayload_hex:61\ncorrupted read line without delimiter\n";
        assert_eq!(
            decode_persisted_record_envelope(text.as_bytes()),
            Err(PersistedRecordEnvelopeError::MalformedRecord)
        );
    }

    #[test]
    fn export_bundle_bytes_cannot_verify_as_durable_append_transaction() {
        let export_bundle_bytes = b"audit_export.format_version=audit-export-v1\naudit_export.record_kind=observability-snapshot\n";
        let report = verify_durable_append_transaction_bytes(export_bundle_bytes, "export-bundle");
        assert_eq!(report.status, DurableAppendStatus::Rejected);
        assert!(!report.committed);
    }

    #[test]
    fn paired_audit_ledger_append_model_is_preserved() {
        let tx = append_tx();
        assert!(!tx.audit_payload.is_empty());
        assert!(!tx.ledger_payload.is_empty());
        let report = verify_durable_append_transaction_bytes(
            &encode_durable_append_transaction(&tx),
            "append-93",
        );
        assert_eq!(
            report.reason,
            DurableAppendReason::VerifiedAppendTransaction
        );
    }

    #[test]
    fn single_writer_revision_assumption_is_documented() {
        assert_eq!(
            prepare_durable_append_transaction(
                "stale_revision",
                "audit",
                "ledger",
                2,
                2,
                vec![1],
                vec![2]
            ),
            Err(DurableAppendReason::InvalidRevisionChain)
        );
    }

    #[test]
    fn concurrent_writer_support_is_not_implemented() {
        let first = prepare_durable_append_transaction(
            "writer-a",
            "audit-a",
            "ledger-a",
            4,
            5,
            vec![1],
            vec![2],
        )
        .unwrap();
        let second = prepare_durable_append_transaction(
            "writer-b",
            "audit-b",
            "ledger-b",
            4,
            5,
            vec![3],
            vec![4],
        )
        .unwrap();
        assert_eq!(first.prior_revision, second.prior_revision);
        assert_eq!(first.next_revision, second.next_revision);
        assert_ne!(first.append_transaction_id, second.append_transaction_id);
    }

    #[test]
    fn replay_verification_does_not_repair_persistence_drift() {
        let report = verify_durable_append_transaction_bytes(
            b"replay verification evidence only",
            "append-93",
        );
        assert_eq!(report.status, DurableAppendStatus::Rejected);
        assert!(!report.repaired_replay);
        assert!(!report.mutated_application_state);
    }

    #[test]
    fn non_repair_posture_is_preserved() {
        let mut envelope = persisted_env();
        envelope.checksum = "0000000000000000".into();
        let before = envelope.clone();
        let report = verify_persisted_record_envelope(&envelope, None);
        assert_eq!(
            report.status,
            PersistedRecordVerificationStatus::ChecksumMismatch
        );
        assert_eq!(envelope, before);
    }
}
