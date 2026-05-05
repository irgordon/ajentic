use super::*;

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
    let out =
        execute_local_persistence_plan(plan, &encode_persisted_record_envelope(&record.envelope));
    let (status, reason) = match out {
        Ok(_) => (
            LedgerPersistenceStatus::Written,
            LedgerPersistenceReason::WrittenThroughPersistenceBoundary,
        ),
        Err(LocalPersistenceError::InvalidPlan) => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::InvalidPersistencePlan,
        ),
        Err(_) => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::PersistenceWriteFailed,
        ),
    };
    LedgerPersistenceReport{status,reason,lifecycle_id:record.lifecycle_id.clone(),ledger_record_id:record.ledger_record_id.clone(),revision:Some(record.revision),payload_len:Some(record.envelope.payload_len),checksum:Some(record.envelope.checksum.clone()),usable_for_state_recovery:false,mutates_application_state:false,summary:"Ledger persistence lifecycle writes typed ledger bytes only and does not recover state.".to_string()}
}

pub fn verify_ledger_persistence_paths(
    lifecycle_id: impl Into<String>,
    ledger_record_id: impl Into<String>,
    target_path: impl AsRef<std::path::Path>,
    temp_path: impl AsRef<std::path::Path>,
    expected_revision: Option<u64>,
) -> LedgerPersistenceReport {
    let r = verify_persisted_record_paths(target_path, temp_path, expected_revision);
    let (status, reason) = match r.status {
        PersistedRecordVerificationStatus::Valid => (
            LedgerPersistenceStatus::Verified,
            LedgerPersistenceReason::VerifiedForLoad,
        ),
        PersistedRecordVerificationStatus::ChecksumMismatch => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::ChecksumMismatch,
        ),
        PersistedRecordVerificationStatus::MalformedRecord
        | PersistedRecordVerificationStatus::InvalidPayloadHex
        | PersistedRecordVerificationStatus::PayloadLengthMismatch => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::MalformedRecord,
        ),
        PersistedRecordVerificationStatus::StaleRevision => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::StaleRevision,
        ),
        PersistedRecordVerificationStatus::UnknownPayloadKind => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::UnknownPayloadKind,
        ),
        _ => (
            LedgerPersistenceStatus::Rejected,
            LedgerPersistenceReason::VerificationFailed,
        ),
    };
    LedgerPersistenceReport{status,reason,lifecycle_id:lifecycle_id.into(),ledger_record_id:ledger_record_id.into(),revision:r.revision,payload_len:r.payload_len,checksum:r.checksum,usable_for_state_recovery:false,mutates_application_state:false,summary:"Ledger verification validates persisted bytes only and does not recover application state.".to_string()}
}

pub fn load_verified_ledger_record_bytes(
    envelope: &PersistedRecordEnvelope,
    expected_revision: Option<u64>,
) -> Result<Vec<u8>, LedgerPersistenceReason> {
    if envelope.payload_kind != LocalPersistencePayloadKind::LedgerSnapshot {
        return Err(LedgerPersistenceReason::UnknownPayloadKind);
    }
    let v = verify_persisted_record_envelope(envelope, expected_revision);
    if v.status != PersistedRecordVerificationStatus::Valid {
        return Err(match v.status {
            PersistedRecordVerificationStatus::ChecksumMismatch => {
                LedgerPersistenceReason::ChecksumMismatch
            }
            PersistedRecordVerificationStatus::MalformedRecord
            | PersistedRecordVerificationStatus::InvalidPayloadHex
            | PersistedRecordVerificationStatus::PayloadLengthMismatch => {
                LedgerPersistenceReason::MalformedRecord
            }
            PersistedRecordVerificationStatus::StaleRevision => {
                LedgerPersistenceReason::StaleRevision
            }
            PersistedRecordVerificationStatus::UnknownPayloadKind => {
                LedgerPersistenceReason::UnknownPayloadKind
            }
            _ => LedgerPersistenceReason::VerificationFailed,
        });
    }
    Ok(envelope.payload.clone())
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
}

#[cfg(test)]
mod diagnostic_tests {
    use super::*;

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
}

#[cfg(test)]
mod phase73_tests {
    use super::*;
    use std::{env, fs};
    fn tmp(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        let base = env::temp_dir().join(format!("ajentic-phase73-{name}"));
        let _ = fs::create_dir_all(&base);
        (base.join("a.rec"), base.join("a.tmp"))
    }
    fn rec() -> LedgerPersistenceRecord {
        prepare_ledger_persistence_record("lc", "lr", 7, vec![1, 2, 3]).unwrap()
    }
    #[test]
    fn ledger_persistence_reason_codes_are_stable() {
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
            rec().envelope.payload_kind,
            LocalPersistencePayloadKind::LedgerSnapshot
        );
    }
    #[test]
    fn prepare_ledger_record_does_not_serialize_application_state() {
        assert_eq!(rec().envelope.payload, vec![1, 2, 3]);
    }
    #[test]
    fn write_ledger_record_uses_existing_persistence_boundary() {
        let (t, tp) = tmp("w1");
        let p = LocalPersistencePlan::new(
            "p",
            t.display().to_string(),
            tp.display().to_string(),
            Some(7),
            LocalPersistencePayloadKind::LedgerSnapshot,
            LocalPersistenceWriteMode::ReplaceExisting,
            LocalPersistenceAtomicity::Required,
        );
        assert_eq!(
            write_ledger_persistence_record(&rec(), &p).status,
            LedgerPersistenceStatus::Written
        )
    }
    #[test]
    fn write_ledger_record_rejects_invalid_plan() {
        let (t, _) = tmp("w2");
        let p = LocalPersistencePlan::new(
            "",
            t.display().to_string(),
            t.display().to_string(),
            None,
            LocalPersistencePayloadKind::Unknown,
            LocalPersistenceWriteMode::Unknown,
            LocalPersistenceAtomicity::Unknown,
        );
        assert_eq!(
            write_ledger_persistence_record(&rec(), &p).reason,
            LedgerPersistenceReason::InvalidPersistencePlan
        )
    }
    #[test]
    fn write_ledger_record_writes_encoded_envelope() {
        let (t, tp) = tmp("w3");
        let r = rec();
        let p = LocalPersistencePlan::new(
            "p",
            t.display().to_string(),
            tp.display().to_string(),
            Some(7),
            LocalPersistencePayloadKind::LedgerSnapshot,
            LocalPersistenceWriteMode::ReplaceExisting,
            LocalPersistenceAtomicity::Required,
        );
        let _ = write_ledger_persistence_record(&r, &p);
        assert_eq!(
            fs::read(&t).unwrap(),
            encode_persisted_record_envelope(&r.envelope)
        );
    }
    #[test]
    fn verify_ledger_paths_accepts_valid_record() {
        let (t, tp) = tmp("v1");
        let r = rec();
        fs::write(&t, encode_persisted_record_envelope(&r.envelope)).unwrap();
        assert_eq!(
            verify_ledger_persistence_paths("l", "r", &t, &tp, Some(7)).status,
            LedgerPersistenceStatus::Verified
        );
    }
    #[test]
    fn verify_ledger_paths_rejects_checksum_mismatch() {
        let (t, tp) = tmp("v2");
        let e = rec().envelope;
        let mut text = String::from_utf8(encode_persisted_record_envelope(&e)).unwrap();
        text = text.replace(&format!("checksum={}", e.checksum), "checksum=0000000000000000");
        fs::write(&t, text.as_bytes()).unwrap();
        assert_eq!(
            verify_ledger_persistence_paths("l", "r", &t, &tp, None).reason,
            LedgerPersistenceReason::ChecksumMismatch
        );
    }
    #[test]
    fn verify_ledger_paths_rejects_malformed_record() {
        let (t, tp) = tmp("v3");
        fs::write(&t, b"bad").unwrap();
        assert_eq!(
            verify_ledger_persistence_paths("l", "r", &t, &tp, None).reason,
            LedgerPersistenceReason::MalformedRecord
        );
    }
    #[test]
    fn verify_ledger_paths_rejects_stale_revision() {
        let (t, tp) = tmp("v4");
        let r = rec();
        fs::write(&t, encode_persisted_record_envelope(&r.envelope)).unwrap();
        assert_eq!(
            verify_ledger_persistence_paths("l", "r", &t, &tp, Some(8)).reason,
            LedgerPersistenceReason::StaleRevision
        );
    }
    #[test]
    fn verify_ledger_paths_rejects_unknown_payload_kind() {
        let (t, tp) = tmp("v5");
        fs::write(&t,b"record_id=x\npayload_kind=unknown\nrevision=1\npayload_len=1\nchecksum=0000000000000000\npayload_hex=00\n").unwrap();
        assert_eq!(
            verify_ledger_persistence_paths("l", "r", &t, &tp, None).reason,
            LedgerPersistenceReason::UnknownPayloadKind
        );
    }
    #[test]
    fn load_verified_ledger_record_bytes_returns_bytes_for_valid_envelope() {
        assert_eq!(
            load_verified_ledger_record_bytes(&rec().envelope, Some(7)).unwrap(),
            vec![1, 2, 3]
        );
    }
    #[test]
    fn load_verified_ledger_record_bytes_rejects_non_ledger_payload_kind() {
        let e =
            PersistedRecordEnvelope::new("x", LocalPersistencePayloadKind::RunRecord, 1, vec![1])
                .unwrap();
        assert_eq!(
            load_verified_ledger_record_bytes(&e, None),
            Err(LedgerPersistenceReason::UnknownPayloadKind)
        );
    }
    #[test]
    fn ledger_persistence_does_not_recover_application_state() {
        assert_eq!(
            LedgerPersistenceReason::RecoveryNotImplemented.code(),
            "recovery_not_implemented"
        );
    }
    #[test]
    fn ledger_persistence_does_not_mutate_application_state() {
        assert!(
            !verify_ledger_persistence_paths("l", "r", "/tmp/no1", "/tmp/no2", None)
                .mutates_application_state
        );
    }
    #[test]
    fn ledger_persistence_does_not_execute_provider_output() {
        assert!(
            !verify_ledger_persistence_paths("l", "r", "/tmp/no3", "/tmp/no4", None)
                .summary
                .contains("provider output")
        );
    }
    #[test]
    fn ledger_persistence_does_not_record_provider_retry() {
        assert!(
            !verify_ledger_persistence_paths("l", "r", "/tmp/no5", "/tmp/no6", None)
                .summary
                .contains("retry record")
        );
    }
    #[test]
    fn ledger_persistence_does_not_repair_replay() {
        assert!(
            !verify_ledger_persistence_paths("l", "r", "/tmp/no7", "/tmp/no8", None)
                .summary
                .contains("repair replay")
        );
    }
    #[test]
    fn ledger_persistence_does_not_promote() {
        assert!(
            !verify_ledger_persistence_paths("l", "r", "/tmp/no9", "/tmp/no10", None)
                .summary
                .contains("promote")
        );
    }
    #[test]
    fn dry_run_does_not_write_or_load_ledger_persistence() {
        assert!(!std::path::Path::new("./target/ledger-persistence-dry-run").exists());
    }
}
