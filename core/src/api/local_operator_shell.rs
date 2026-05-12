#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalRunStatus {
    Idle,
    StubCompleted,
    IntentRecorded,
}

impl LocalRunStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::StubCompleted => "stub_completed",
            Self::IntentRecorded => "intent_recorded",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalOperatorIntentKind {
    Approve,
    Reject,
}

impl LocalOperatorIntentKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Approve => "approve",
            Self::Reject => "reject",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalOperatorShellError {
    RunNotStarted,
    EmptyOperatorId,
    EmptyReason,
    TargetMismatch,
    CandidateTargetMismatch,
    DuplicateDecisionRejected,
    AuthorityClaimRejected,
    ProviderExecutionRejected,
    ReadinessClaimRejected,
}

impl LocalOperatorShellError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::RunNotStarted => "run_not_started",
            Self::EmptyOperatorId => "empty_operator_id",
            Self::EmptyReason => "empty_reason",
            Self::TargetMismatch => "target_mismatch",
            Self::CandidateTargetMismatch => "candidate_target_mismatch",
            Self::DuplicateDecisionRejected => "duplicate_decision_rejected",
            Self::AuthorityClaimRejected => "authority_claim_rejected",
            Self::ProviderExecutionRejected => "provider_execution_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalDecisionRecordKind {
    Approve,
    Reject,
}

impl LocalDecisionRecordKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Approve => "approve",
            Self::Reject => "reject",
        }
    }
}

impl From<LocalOperatorIntentKind> for LocalDecisionRecordKind {
    fn from(kind: LocalOperatorIntentKind) -> Self {
        match kind {
            LocalOperatorIntentKind::Approve => Self::Approve,
            LocalOperatorIntentKind::Reject => Self::Reject,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalDecisionRecordStatus {
    Recorded,
}

impl LocalDecisionRecordStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Recorded => "recorded",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDecisionRecord {
    pub decision_id: String,
    pub run_id: String,
    pub candidate_id: String,
    pub operator_id: String,
    pub intent_kind: LocalDecisionRecordKind,
    pub decision_status: LocalDecisionRecordStatus,
    pub validation_status: String,
    pub recorded_sequence: u64,
    pub recorded_at_label: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDecisionTimelineProjection {
    pub records: Vec<LocalDecisionRecord>,
    pub empty_message: String,
}

impl LocalDecisionTimelineProjection {
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDecisionLedger {
    pub records: Vec<LocalDecisionRecord>,
}

pub fn initial_local_decision_ledger() -> LocalDecisionLedger {
    LocalDecisionLedger {
        records: Vec::new(),
    }
}

pub fn project_local_decision_timeline(
    ledger: &LocalDecisionLedger,
) -> LocalDecisionTimelineProjection {
    LocalDecisionTimelineProjection {
        records: ledger.records.clone(),
        empty_message: "No recorded local operator decisions".to_string(),
    }
}

pub fn next_local_decision_sequence(ledger: &LocalDecisionLedger) -> u64 {
    ledger.records.len() as u64 + 1
}

fn append_local_decision(
    ledger: &LocalDecisionLedger,
    intent: &LocalOperatorIntent,
    candidate_id: &str,
) -> Result<LocalDecisionLedger, LocalOperatorShellError> {
    if ledger
        .records
        .iter()
        .any(|record| record.run_id == intent.target_run_id && record.candidate_id == candidate_id)
    {
        return Err(LocalOperatorShellError::DuplicateDecisionRejected);
    }

    let recorded_sequence = next_local_decision_sequence(ledger);
    let mut next = ledger.clone();
    next.records.push(LocalDecisionRecord {
        decision_id: format!("local-decision-{recorded_sequence:04}"),
        run_id: intent.target_run_id.clone(),
        candidate_id: candidate_id.to_string(),
        operator_id: intent.operator_id.clone(),
        intent_kind: intent.kind.into(),
        decision_status: LocalDecisionRecordStatus::Recorded,
        validation_status: "accepted_by_rust_local_validation".to_string(),
        recorded_sequence,
        recorded_at_label: format!("local-sequence-{recorded_sequence:04}"),
        reason: intent.reason.clone(),
    });
    Ok(next)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalDecisionReplayStatus {
    NoDecisionRecorded,
    ApprovedDecisionReplayed,
    RejectedDecisionReplayed,
    InconsistentDecisionLedger,
    ReplayNotApplicable,
}

impl LocalDecisionReplayStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoDecisionRecorded => "no_decision_recorded",
            Self::ApprovedDecisionReplayed => "approved_decision_replayed",
            Self::RejectedDecisionReplayed => "rejected_decision_replayed",
            Self::InconsistentDecisionLedger => "inconsistent_decision_ledger",
            Self::ReplayNotApplicable => "replay_not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalDecisionReplayIntegrityStatus {
    Consistent,
    Inconsistent,
}

impl LocalDecisionReplayIntegrityStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Consistent => "consistent",
            Self::Inconsistent => "inconsistent",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalDecisionReplayError {
    EmptyRecordField,
    SequenceMismatch,
    DecisionIdMismatch,
    RunMismatch,
    CandidateMismatch,
    UnsupportedDecisionStatus,
}

impl LocalDecisionReplayError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyRecordField => "empty_record_field",
            Self::SequenceMismatch => "sequence_mismatch",
            Self::DecisionIdMismatch => "decision_id_mismatch",
            Self::RunMismatch => "run_mismatch",
            Self::CandidateMismatch => "candidate_mismatch",
            Self::UnsupportedDecisionStatus => "unsupported_decision_status",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDecisionReplayEntry {
    pub replay_sequence: String,
    pub decision_id: String,
    pub run_id: String,
    pub candidate_id: String,
    pub operator_id: String,
    pub decision_kind: LocalDecisionRecordKind,
    pub decision_status: LocalDecisionRecordStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDecisionReplayProjection {
    pub replay_status: LocalDecisionReplayStatus,
    pub replay_sequence: String,
    pub source_decision_count: usize,
    pub latest_decision_id: Option<String>,
    pub latest_run_id: Option<String>,
    pub latest_candidate_id: Option<String>,
    pub latest_operator_id: Option<String>,
    pub latest_decision_kind: Option<LocalDecisionRecordKind>,
    pub latest_decision_status: Option<LocalDecisionRecordStatus>,
    pub integrity_status: LocalDecisionReplayIntegrityStatus,
    pub error: Option<LocalDecisionReplayError>,
    pub entries: Vec<LocalDecisionReplayEntry>,
    pub summary: String,
}

pub fn initial_local_decision_replay_projection() -> LocalDecisionReplayProjection {
    LocalDecisionReplayProjection {
        replay_status: LocalDecisionReplayStatus::NoDecisionRecorded,
        replay_sequence: "local-replay-sequence-0000".to_string(),
        source_decision_count: 0,
        latest_decision_id: None,
        latest_run_id: None,
        latest_candidate_id: None,
        latest_operator_id: None,
        latest_decision_kind: None,
        latest_decision_status: None,
        integrity_status: LocalDecisionReplayIntegrityStatus::Consistent,
        error: None,
        entries: Vec::new(),
        summary: "No local operator decision has been recorded for replay projection.".to_string(),
    }
}

fn inconsistent_local_decision_replay_projection(
    ledger: &LocalDecisionLedger,
    error: LocalDecisionReplayError,
) -> LocalDecisionReplayProjection {
    LocalDecisionReplayProjection {
        replay_status: LocalDecisionReplayStatus::InconsistentDecisionLedger,
        replay_sequence: format!("local-replay-sequence-{:04}", ledger.records.len()),
        source_decision_count: ledger.records.len(),
        latest_decision_id: ledger
            .records
            .last()
            .map(|record| record.decision_id.clone()),
        latest_run_id: ledger.records.last().map(|record| record.run_id.clone()),
        latest_candidate_id: ledger
            .records
            .last()
            .map(|record| record.candidate_id.clone()),
        latest_operator_id: ledger
            .records
            .last()
            .map(|record| record.operator_id.clone()),
        latest_decision_kind: ledger.records.last().map(|record| record.intent_kind),
        latest_decision_status: ledger.records.last().map(|record| record.decision_status),
        integrity_status: LocalDecisionReplayIntegrityStatus::Inconsistent,
        error: Some(error),
        entries: Vec::new(),
        summary: format!("Local decision ledger is inconsistent: {}.", error.code()),
    }
}

pub fn validate_local_decision_replay_input(
    run: &LocalRunProjection,
    ledger: &LocalDecisionLedger,
) -> Result<(), LocalDecisionReplayError> {
    for (index, record) in ledger.records.iter().enumerate() {
        let expected_sequence = index as u64 + 1;
        if record.decision_id.is_empty()
            || record.run_id.is_empty()
            || record.candidate_id.is_empty()
            || record.operator_id.is_empty()
        {
            return Err(LocalDecisionReplayError::EmptyRecordField);
        }
        if record.recorded_sequence != expected_sequence {
            return Err(LocalDecisionReplayError::SequenceMismatch);
        }
        if record.decision_id != format!("local-decision-{expected_sequence:04}") {
            return Err(LocalDecisionReplayError::DecisionIdMismatch);
        }
        if record.decision_status != LocalDecisionRecordStatus::Recorded {
            return Err(LocalDecisionReplayError::UnsupportedDecisionStatus);
        }
    }

    if let Some(latest) = ledger.records.last() {
        if latest.run_id != run.run_id {
            return Err(LocalDecisionReplayError::RunMismatch);
        }
        if let Some(candidate) = run.candidate.as_ref() {
            if latest.candidate_id != candidate.candidate_id {
                return Err(LocalDecisionReplayError::CandidateMismatch);
            }
        }
    }

    Ok(())
}

pub fn derive_local_decision_replay_projection(
    run: &LocalRunProjection,
    ledger: &LocalDecisionLedger,
) -> LocalDecisionReplayProjection {
    if ledger.records.is_empty() {
        return initial_local_decision_replay_projection();
    }

    if let Err(error) = validate_local_decision_replay_input(run, ledger) {
        return inconsistent_local_decision_replay_projection(ledger, error);
    }

    let entries = ledger
        .records
        .iter()
        .map(|record| LocalDecisionReplayEntry {
            replay_sequence: format!("local-replay-entry-{:04}", record.recorded_sequence),
            decision_id: record.decision_id.clone(),
            run_id: record.run_id.clone(),
            candidate_id: record.candidate_id.clone(),
            operator_id: record.operator_id.clone(),
            decision_kind: record.intent_kind,
            decision_status: record.decision_status,
        })
        .collect::<Vec<_>>();
    let latest = ledger
        .records
        .last()
        .expect("non-empty ledger has latest decision");
    let replay_status = match latest.intent_kind {
        LocalDecisionRecordKind::Approve => LocalDecisionReplayStatus::ApprovedDecisionReplayed,
        LocalDecisionRecordKind::Reject => LocalDecisionReplayStatus::RejectedDecisionReplayed,
    };

    LocalDecisionReplayProjection {
        replay_status,
        replay_sequence: format!("local-replay-sequence-{:04}", ledger.records.len()),
        source_decision_count: ledger.records.len(),
        latest_decision_id: Some(latest.decision_id.clone()),
        latest_run_id: Some(latest.run_id.clone()),
        latest_candidate_id: Some(latest.candidate_id.clone()),
        latest_operator_id: Some(latest.operator_id.clone()),
        latest_decision_kind: Some(latest.intent_kind),
        latest_decision_status: Some(latest.decision_status),
        integrity_status: LocalDecisionReplayIntegrityStatus::Consistent,
        error: None,
        entries,
        summary: format!(
            "Local decision replay projection derived {} recorded decision(s); latest decision {} is {}.",
            ledger.records.len(),
            latest.decision_id,
            replay_status.code()
        ),
    }
}

pub fn project_local_decision_replay(
    state: &LocalOperatorShellState,
) -> LocalDecisionReplayProjection {
    derive_local_decision_replay_projection(&state.run, &state.decision_ledger)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionEvidenceExportStatus {
    NoCompletedRunEvidence,
    RunEvidenceProjected,
    DecisionEvidenceProjected,
}

impl LocalSessionEvidenceExportStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoCompletedRunEvidence => "no_completed_run_evidence",
            Self::RunEvidenceProjected => "run_evidence_projected",
            Self::DecisionEvidenceProjected => "decision_evidence_projected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionEvidenceExportValidationStatus {
    Complete,
    Incomplete,
}

impl LocalSessionEvidenceExportValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionEvidenceExportError {
    MissingRequiredField,
    InvalidExportClassification,
    InvalidProductionClassification,
    MissingAbsenceMarker,
    MissingRunEvidence,
    MissingDecisionReplayEvidence,
}

impl LocalSessionEvidenceExportError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingRequiredField => "missing_required_field",
            Self::InvalidExportClassification => "invalid_export_classification",
            Self::InvalidProductionClassification => "invalid_production_classification",
            Self::MissingAbsenceMarker => "missing_absence_marker",
            Self::MissingRunEvidence => "missing_run_evidence",
            Self::MissingDecisionReplayEvidence => "missing_decision_replay_evidence",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionEvidenceExportAbsenceMarkers {
    pub provider_execution_absent: bool,
    pub persistence_absent: bool,
    pub release_absent: bool,
    pub deployment_absent: bool,
    pub signing_absent: bool,
    pub publishing_absent: bool,
    pub installer_absent: bool,
    pub update_channel_absent: bool,
    pub public_use_absent: bool,
    pub readiness_approval_absent: bool,
    pub marker_summary: Vec<String>,
}

pub fn local_session_evidence_export_absence_markers() -> LocalSessionEvidenceExportAbsenceMarkers {
    LocalSessionEvidenceExportAbsenceMarkers {
        provider_execution_absent: true,
        persistence_absent: true,
        release_absent: true,
        deployment_absent: true,
        signing_absent: true,
        publishing_absent: true,
        installer_absent: true,
        update_channel_absent: true,
        public_use_absent: true,
        readiness_approval_absent: true,
        marker_summary: vec![
            "provider execution absent".to_string(),
            "persistence absent".to_string(),
            "release absent".to_string(),
            "deployment absent".to_string(),
            "signing absent".to_string(),
            "publishing absent".to_string(),
            "installer absent".to_string(),
            "update-channel absent".to_string(),
            "public-use absent".to_string(),
            "readiness absent".to_string(),
        ],
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionEvidenceExport {
    pub export_id: String,
    pub export_status: LocalSessionEvidenceExportStatus,
    pub export_classification: String,
    pub production_classification: String,
    pub shell_status: String,
    pub run_id: String,
    pub run_status: LocalRunStatus,
    pub bounded_context_summary: Vec<String>,
    pub candidate_id: String,
    pub candidate_output_summary: String,
    pub validation_status: String,
    pub policy_status: String,
    pub decision_count: usize,
    pub decision_records: Vec<LocalDecisionRecord>,
    pub replay_status: LocalDecisionReplayStatus,
    pub replay_integrity_status: LocalDecisionReplayIntegrityStatus,
    pub absence_markers: LocalSessionEvidenceExportAbsenceMarkers,
    pub export_validation_status: LocalSessionEvidenceExportValidationStatus,
    pub summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderKind {
    DeterministicStub,
    LocalModel,
    CloudModel,
    ExternalHttp,
    ShellCommand,
    FilesystemProvider,
    Unknown,
}

impl LocalProviderKind {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "deterministic_stub" => Some(Self::DeterministicStub),
            "local_model" => Some(Self::LocalModel),
            "cloud_model" => Some(Self::CloudModel),
            "external_http" => Some(Self::ExternalHttp),
            "shell_command" => Some(Self::ShellCommand),
            "filesystem_provider" => Some(Self::FilesystemProvider),
            "unknown" => Some(Self::Unknown),
            _ => None,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::DeterministicStub => "deterministic_stub",
            Self::LocalModel => "local_model",
            Self::CloudModel => "cloud_model",
            Self::ExternalHttp => "external_http",
            Self::ShellCommand => "shell_command",
            Self::FilesystemProvider => "filesystem_provider",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderConfigurationStatus {
    NotConfigured,
    Accepted,
    Rejected,
    Unsupported,
}

impl LocalProviderConfigurationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotConfigured => "not_configured",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalProviderConfigurationError {
    MissingProviderKind,
    MalformedProviderKind,
    UnsupportedProviderKind,
    ForbiddenEndpointField,
    ForbiddenCommandField,
    ForbiddenPathField,
    ForbiddenSecretField,
    ProviderExecutionRejected,
    TrustGrantRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    SigningClaimRejected,
    PublishingClaimRejected,
    UnknownFieldRejected,
}

impl LocalProviderConfigurationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingProviderKind => "missing_provider_kind",
            Self::MalformedProviderKind => "malformed_provider_kind",
            Self::UnsupportedProviderKind => "unsupported_provider_kind",
            Self::ForbiddenEndpointField => "forbidden_endpoint_field",
            Self::ForbiddenCommandField => "forbidden_command_field",
            Self::ForbiddenPathField => "forbidden_path_field",
            Self::ForbiddenSecretField => "forbidden_secret_field",
            Self::ProviderExecutionRejected => "provider_execution_rejected",
            Self::TrustGrantRejected => "trust_grant_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::SigningClaimRejected => "signing_claim_rejected",
            Self::PublishingClaimRejected => "publishing_claim_rejected",
            Self::UnknownFieldRejected => "unknown_field_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderConfigurationCapabilitySurface {
    pub configuration_only: bool,
    pub provider_execution_enabled: bool,
    pub cloud_calls_enabled: bool,
    pub network_enabled: bool,
    pub shell_commands_enabled: bool,
    pub filesystem_enabled: bool,
    pub secrets_allowed: bool,
    pub trust_granted: bool,
    pub readiness_approved: bool,
    pub release_approved: bool,
    pub deployment_enabled: bool,
    pub summary: String,
}

impl LocalProviderConfigurationCapabilitySurface {
    pub fn phase_139_configuration_only() -> Self {
        Self {
            configuration_only: true,
            provider_execution_enabled: false,
            cloud_calls_enabled: false,
            network_enabled: false,
            shell_commands_enabled: false,
            filesystem_enabled: false,
            secrets_allowed: false,
            trust_granted: false,
            readiness_approved: false,
            release_approved: false,
            deployment_enabled: false,
            summary: "deterministic_stub configuration-only surface; no execution, cloud, network, shell, filesystem, secrets, trust, readiness, release, or deployment capability".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderConfigurationValidation {
    pub status: LocalProviderConfigurationStatus,
    pub provider_kind: Option<LocalProviderKind>,
    pub error_codes: Vec<LocalProviderConfigurationError>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderConfigurationProjection {
    pub configured_provider_kind: String,
    pub status: LocalProviderConfigurationStatus,
    pub validation_status: String,
    pub validation_reason: String,
    pub validation_error_codes: Vec<String>,
    pub execution_status: String,
    pub capability_surface: LocalProviderConfigurationCapabilitySurface,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderConfiguration {
    pub configured_provider_kind: Option<LocalProviderKind>,
    pub status: LocalProviderConfigurationStatus,
    pub last_validation: LocalProviderConfigurationValidation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderConfigurationCandidate {
    pub provider_kind: Option<String>,
    pub fields: Vec<(String, String)>,
}

impl LocalProviderConfigurationCandidate {
    pub fn deterministic_stub() -> Self {
        Self {
            provider_kind: Some("deterministic_stub".to_string()),
            fields: Vec::new(),
        }
    }
}

pub fn initial_local_provider_configuration() -> LocalProviderConfiguration {
    LocalProviderConfiguration {
        configured_provider_kind: None,
        status: LocalProviderConfigurationStatus::NotConfigured,
        last_validation: LocalProviderConfigurationValidation {
            status: LocalProviderConfigurationStatus::NotConfigured,
            provider_kind: None,
            error_codes: Vec::new(),
            reason: "no executable provider configured; deterministic_stub is available for configuration-only validation".to_string(),
        },
    }
}

pub fn local_provider_configuration_capability_surface(
) -> LocalProviderConfigurationCapabilitySurface {
    LocalProviderConfigurationCapabilitySurface::phase_139_configuration_only()
}

pub fn project_local_provider_configuration(
    configuration: &LocalProviderConfiguration,
) -> LocalProviderConfigurationProjection {
    LocalProviderConfigurationProjection {
        configured_provider_kind: configuration
            .configured_provider_kind
            .map(|kind| kind.code().to_string())
            .unwrap_or_else(|| "none".to_string()),
        status: configuration.status,
        validation_status: configuration.last_validation.status.code().to_string(),
        validation_reason: configuration.last_validation.reason.clone(),
        validation_error_codes: configuration
            .last_validation
            .error_codes
            .iter()
            .map(|error| error.code().to_string())
            .collect(),
        execution_status: "disabled_not_executed".to_string(),
        capability_surface: local_provider_configuration_capability_surface(),
        note: "Phase 139 deterministic_stub is configuration-only; validation does not execute providers or approve trust/readiness/release/deployment.".to_string(),
    }
}

pub fn validate_local_provider_configuration(
    candidate: &LocalProviderConfigurationCandidate,
) -> LocalProviderConfigurationValidation {
    let mut errors = std::collections::BTreeSet::new();
    let parsed_kind = match candidate.provider_kind.as_deref() {
        None => {
            errors.insert(LocalProviderConfigurationError::MissingProviderKind);
            None
        }
        Some(kind) if kind.trim().is_empty() => {
            errors.insert(LocalProviderConfigurationError::MissingProviderKind);
            None
        }
        Some(kind) if kind.trim() != kind => {
            errors.insert(LocalProviderConfigurationError::MalformedProviderKind);
            None
        }
        Some(kind) => match LocalProviderKind::parse(kind) {
            Some(LocalProviderKind::DeterministicStub) => {
                Some(LocalProviderKind::DeterministicStub)
            }
            Some(other) => {
                errors.insert(LocalProviderConfigurationError::UnsupportedProviderKind);
                Some(other)
            }
            None => {
                errors.insert(LocalProviderConfigurationError::UnsupportedProviderKind);
                None
            }
        },
    };

    for (key, value) in &candidate.fields {
        reject_forbidden_provider_configuration(key, value, &mut errors);
    }

    if errors.is_empty() && parsed_kind == Some(LocalProviderKind::DeterministicStub) {
        LocalProviderConfigurationValidation {
            status: LocalProviderConfigurationStatus::Accepted,
            provider_kind: parsed_kind,
            error_codes: Vec::new(),
            reason: "deterministic_stub configuration accepted as local-session configuration-only state; provider execution remains disabled".to_string(),
        }
    } else {
        let status = if errors.contains(&LocalProviderConfigurationError::UnsupportedProviderKind) {
            LocalProviderConfigurationStatus::Unsupported
        } else {
            LocalProviderConfigurationStatus::Rejected
        };
        LocalProviderConfigurationValidation {
            status,
            provider_kind: parsed_kind,
            error_codes: errors.into_iter().collect(),
            reason: "provider configuration rejected fail-closed; prior configuration remains unchanged and no provider execution occurs".to_string(),
        }
    }
}

pub fn reject_forbidden_provider_configuration(
    key: &str,
    value: &str,
    errors: &mut std::collections::BTreeSet<LocalProviderConfigurationError>,
) {
    let key = key.to_ascii_lowercase();
    let value = value.to_ascii_lowercase();
    let combined = format!("{key}={value}");
    if matches!(key.as_str(), "label" | "description") {
        return;
    }
    if ["endpoint", "url", "host", "port", "http", "network"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderConfigurationError::ForbiddenEndpointField);
    } else if ["command", "args", "shell", "process"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderConfigurationError::ForbiddenCommandField);
    } else if ["path", "file", "directory"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderConfigurationError::ForbiddenPathField);
    } else if ["secret", "token", "api_key", "apikey", "credential"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderConfigurationError::ForbiddenSecretField);
    } else if key == "provider_execution_enabled" && value == "true" {
        errors.insert(LocalProviderConfigurationError::ProviderExecutionRejected);
    } else if key == "trust_granted" && value == "true" {
        errors.insert(LocalProviderConfigurationError::TrustGrantRejected);
    } else if key == "readiness_approved" && value == "true" {
        errors.insert(LocalProviderConfigurationError::ReadinessClaimRejected);
    } else if key == "release_candidate_approved" && value == "true" {
        errors.insert(LocalProviderConfigurationError::ReleaseClaimRejected);
    } else if key == "deployment_enabled" && value == "true" {
        errors.insert(LocalProviderConfigurationError::DeploymentClaimRejected);
    } else if key == "public_use_approved" && value == "true" {
        errors.insert(LocalProviderConfigurationError::PublicUseClaimRejected);
    } else if key == "signing_enabled" && value == "true" {
        errors.insert(LocalProviderConfigurationError::SigningClaimRejected);
    } else if key == "publishing_enabled" && value == "true" {
        errors.insert(LocalProviderConfigurationError::PublishingClaimRejected);
    } else {
        errors.insert(LocalProviderConfigurationError::UnknownFieldRejected);
    }
}

pub fn apply_local_provider_configuration_candidate(
    state: &LocalOperatorShellState,
    candidate: LocalProviderConfigurationCandidate,
) -> Result<LocalOperatorShellState, LocalProviderConfigurationValidation> {
    let validation = validate_local_provider_configuration(&candidate);
    if validation.status != LocalProviderConfigurationStatus::Accepted {
        return Err(validation);
    }

    let mut next = state.clone();
    next.provider_configuration = LocalProviderConfiguration {
        configured_provider_kind: validation.provider_kind,
        status: LocalProviderConfigurationStatus::Accepted,
        last_validation: validation,
    };
    Ok(attach_local_session_evidence_export(next))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderExecutionStatus {
    NotExecuted,
    Executed,
    Rejected,
    UnsupportedProvider,
    ConfigurationRequired,
    InvalidRequest,
}

impl LocalProviderExecutionStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotExecuted => "not_executed",
            Self::Executed => "executed",
            Self::Rejected => "rejected",
            Self::UnsupportedProvider => "unsupported_provider",
            Self::ConfigurationRequired => "configuration_required",
            Self::InvalidRequest => "invalid_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderExecutionSandboxStatus {
    NotEntered,
    SandboxedDeterministicNoExternalEffects,
    RejectedBeforeSandbox,
}

impl LocalProviderExecutionSandboxStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotEntered => "not_entered",
            Self::SandboxedDeterministicNoExternalEffects => {
                "sandboxed_deterministic_no_external_effects"
            }
            Self::RejectedBeforeSandbox => "rejected_before_sandbox",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalProviderExecutionError {
    MissingProviderConfiguration,
    RejectedProviderConfiguration,
    MissingProviderKind,
    MalformedProviderKind,
    UnsupportedProviderKind,
    ForbiddenEndpointField,
    ForbiddenCommandField,
    ForbiddenPathField,
    ForbiddenSecretField,
    ProviderExecutionFlagRejected,
    TrustGrantRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    SigningClaimRejected,
    PublishingClaimRejected,
    UnknownFieldRejected,
}

impl LocalProviderExecutionError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingProviderConfiguration => "missing_provider_configuration",
            Self::RejectedProviderConfiguration => "rejected_provider_configuration",
            Self::MissingProviderKind => "missing_provider_kind",
            Self::MalformedProviderKind => "malformed_provider_kind",
            Self::UnsupportedProviderKind => "unsupported_provider_kind",
            Self::ForbiddenEndpointField => "forbidden_endpoint_field",
            Self::ForbiddenCommandField => "forbidden_command_field",
            Self::ForbiddenPathField => "forbidden_path_field",
            Self::ForbiddenSecretField => "forbidden_secret_field",
            Self::ProviderExecutionFlagRejected => "provider_execution_flag_rejected",
            Self::TrustGrantRejected => "trust_grant_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::SigningClaimRejected => "signing_claim_rejected",
            Self::PublishingClaimRejected => "publishing_claim_rejected",
            Self::UnknownFieldRejected => "unknown_field_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderExecutionResultProjectionStatus {
    NotExecuted,
    ExecutionProjected,
    ExecutionRejected,
    UnsupportedProvider,
    InvalidExecutionRequest,
}

impl LocalProviderExecutionResultProjectionStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotExecuted => "not_executed",
            Self::ExecutionProjected => "execution_projected",
            Self::ExecutionRejected => "execution_rejected",
            Self::UnsupportedProvider => "unsupported_provider",
            Self::InvalidExecutionRequest => "invalid_execution_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputMaterializationStatus {
    NotMaterialized,
    ProjectedAsUntrustedOutput,
    NotCandidateMaterial,
    CandidateMaterial,
}

impl LocalProviderOutputMaterializationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotMaterialized => "not_materialized",
            Self::ProjectedAsUntrustedOutput => "projected_as_untrusted_output",
            Self::NotCandidateMaterial => "not_candidate_material",
            Self::CandidateMaterial => "candidate_material",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputTrustStatus {
    UntrustedDescriptive,
    TrustedOutput,
}

impl LocalProviderOutputTrustStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UntrustedDescriptive => "untrusted_descriptive",
            Self::TrustedOutput => "trusted_output",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputPromotionStatus {
    NotPromoted,
    PromotionNotAvailableInPhase142,
    Promoted,
}

impl LocalProviderOutputPromotionStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotPromoted => "not_promoted",
            Self::PromotionNotAvailableInPhase142 => "promotion_not_available_in_phase_142",
            Self::Promoted => "promoted",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputValidationStatus {
    NotValidated,
    ReviewableUntrusted,
    Rejected,
    ValidationNotApplicable,
    InvalidValidationInput,
}

impl LocalProviderOutputValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotValidated => "not_validated",
            Self::ReviewableUntrusted => "reviewable_untrusted",
            Self::Rejected => "rejected",
            Self::ValidationNotApplicable => "validation_not_applicable",
            Self::InvalidValidationInput => "invalid_validation_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputReviewabilityStatus {
    NotReviewable,
    ReviewableUntrusted,
    RejectedBeforeReview,
}

impl LocalProviderOutputReviewabilityStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotReviewable => "not_reviewable",
            Self::ReviewableUntrusted => "reviewable_untrusted",
            Self::RejectedBeforeReview => "rejected_before_review",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputCandidateBoundaryStatus {
    NotCandidateMaterial,
    CandidateConversionNotPerformed,
    CandidateConversionRequiresFuturePhase,
}

impl LocalProviderOutputCandidateBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotCandidateMaterial => "not_candidate_material",
            Self::CandidateConversionNotPerformed => "candidate_conversion_not_performed",
            Self::CandidateConversionRequiresFuturePhase => {
                "candidate_conversion_requires_future_phase"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalProviderOutputValidationReason {
    NoProviderExecutionResult,
    ProviderExecutionNotProjected,
    DeterministicStubOutputShapeValid,
    MissingExecutionResult,
    UnsupportedProviderKind,
    EmptyOutput,
    MalformedOutput,
    OutputTooLarge,
    ContainsForbiddenSecretMarker,
    ContainsExecutionInstruction,
    ContainsNetworkInstruction,
    ContainsFilesystemInstruction,
    ContainsReadinessOrReleaseClaim,
    ContainsTrustOrApprovalClaim,
    ContainsActionInstruction,
    CandidateConversionNotAvailableInPhase143,
}

impl LocalProviderOutputValidationReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoProviderExecutionResult => "no_provider_execution_result",
            Self::ProviderExecutionNotProjected => "provider_execution_not_projected",
            Self::DeterministicStubOutputShapeValid => "deterministic_stub_output_shape_valid",
            Self::MissingExecutionResult => "missing_execution_result",
            Self::UnsupportedProviderKind => "unsupported_provider_kind",
            Self::EmptyOutput => "empty_output",
            Self::MalformedOutput => "malformed_output",
            Self::OutputTooLarge => "output_too_large",
            Self::ContainsForbiddenSecretMarker => "contains_forbidden_secret_marker",
            Self::ContainsExecutionInstruction => "contains_execution_instruction",
            Self::ContainsNetworkInstruction => "contains_network_instruction",
            Self::ContainsFilesystemInstruction => "contains_filesystem_instruction",
            Self::ContainsReadinessOrReleaseClaim => "contains_readiness_or_release_claim",
            Self::ContainsTrustOrApprovalClaim => "contains_trust_or_approval_claim",
            Self::ContainsActionInstruction => "contains_action_instruction",
            Self::CandidateConversionNotAvailableInPhase143 => {
                "candidate_conversion_not_available_in_phase_143"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputValidationEffect {
    None,
    EffectDetected,
}

impl LocalProviderOutputValidationEffect {
    pub fn code(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::EffectDetected => "effect_detected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputValidationError {
    InvalidReviewableTrustStatus,
    InvalidCandidateBoundaryStatus,
    InvalidPromotionStatus,
    InvalidNoEffectBoundary,
    MissingValidationReason,
}

impl LocalProviderOutputValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidReviewableTrustStatus => "invalid_reviewable_trust_status",
            Self::InvalidCandidateBoundaryStatus => "invalid_candidate_boundary_status",
            Self::InvalidPromotionStatus => "invalid_promotion_status",
            Self::InvalidNoEffectBoundary => "invalid_no_effect_boundary",
            Self::MissingValidationReason => "missing_validation_reason",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderOutputValidationProjection {
    pub status: LocalProviderOutputValidationStatus,
    pub reviewability_status: LocalProviderOutputReviewabilityStatus,
    pub candidate_boundary_status: LocalProviderOutputCandidateBoundaryStatus,
    pub candidate_boundary_statuses: Vec<LocalProviderOutputCandidateBoundaryStatus>,
    pub reasons: Vec<LocalProviderOutputValidationReason>,
    pub provider_execution_result_id: Option<String>,
    pub provider_kind: String,
    pub output_trust_status: LocalProviderOutputTrustStatus,
    pub output_promotion_status: LocalProviderOutputPromotionStatus,
    pub trust_effect: LocalProviderOutputValidationEffect,
    pub candidate_effect: LocalProviderOutputValidationEffect,
    pub decision_ledger_effect: LocalProviderOutputValidationEffect,
    pub replay_effect: LocalProviderOutputValidationEffect,
    pub export_effect: LocalProviderOutputValidationEffect,
    pub action_effect: LocalProviderOutputValidationEffect,
    pub readiness_effect: LocalProviderOutputValidationEffect,
    pub release_effect: LocalProviderOutputValidationEffect,
    pub deployment_effect: LocalProviderOutputValidationEffect,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderExecutionResultLinkage {
    pub shell_state_label: String,
    pub run_id: String,
    pub provider_configuration_kind: String,
    pub provider_configuration_status: String,
    pub execution_result_id: String,
    pub candidate_id: String,
    pub source_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderExecutionResultAbsenceMarkers {
    pub no_process_spawned: bool,
    pub no_network_socket_opened: bool,
    pub no_filesystem_persistence: bool,
    pub no_secrets_read: bool,
    pub no_release_created: bool,
    pub no_deployment_created: bool,
    pub no_signing_performed: bool,
    pub no_publishing_performed: bool,
    pub no_public_use_approved: bool,
    pub no_readiness_approved: bool,
    pub no_replay_repair: bool,
    pub no_recovery_promotion: bool,
    pub no_action_execution: bool,
    pub provider_output_not_candidate_material: bool,
    pub marker_summary: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderExecutionResultProjectionValidationStatus {
    Valid,
    Invalid,
}

impl LocalProviderExecutionResultProjectionValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::Invalid => "invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderExecutionResultProjectionError {
    InvalidTrustStatus,
    InvalidMaterializationStatus,
    InvalidPromotionStatus,
    MissingAbsenceMarker,
    MissingLinkage,
}

impl LocalProviderExecutionResultProjectionError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidTrustStatus => "invalid_trust_status",
            Self::InvalidMaterializationStatus => "invalid_materialization_status",
            Self::InvalidPromotionStatus => "invalid_promotion_status",
            Self::MissingAbsenceMarker => "missing_absence_marker",
            Self::MissingLinkage => "missing_linkage",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderExecutionResultProjectionValidation {
    pub status: LocalProviderExecutionResultProjectionValidationStatus,
    pub error_codes: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderExecutionRequest {
    pub provider_kind: Option<String>,
    pub input_summary: String,
    pub fields: Vec<(String, String)>,
}

impl LocalProviderExecutionRequest {
    pub fn deterministic_stub(input_summary: impl Into<String>) -> Self {
        Self {
            provider_kind: Some("deterministic_stub".to_string()),
            input_summary: input_summary.into(),
            fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderExecutionCapabilitySurface {
    pub deterministic_stub_execution_supported: bool,
    pub supported_provider_kind: String,
    pub cloud_calls_enabled: bool,
    pub network_enabled: bool,
    pub shell_commands_enabled: bool,
    pub filesystem_enabled: bool,
    pub secrets_allowed: bool,
    pub trust_granted: bool,
    pub readiness_approved: bool,
    pub release_approved: bool,
    pub deployment_enabled: bool,
    pub signing_enabled: bool,
    pub publishing_enabled: bool,
    pub public_use_enabled: bool,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderExecutionResult {
    pub result_id: String,
    pub provider_kind: LocalProviderKind,
    pub sandbox_status: LocalProviderExecutionSandboxStatus,
    pub output_summary: String,
    pub output_trust_status: LocalProviderOutputTrustStatus,
    pub output_materialization_status: LocalProviderOutputMaterializationStatus,
    pub output_promotion_status: LocalProviderOutputPromotionStatus,
    pub promotion_availability_status: LocalProviderOutputPromotionStatus,
    pub descriptive_only: bool,
    pub provider_output_trusted: bool,
    pub candidate_output_promoted: bool,
    pub decision_appended: bool,
    pub replay_repaired: bool,
    pub release_or_deployment_evidence_created: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderExecutionValidation {
    pub status: LocalProviderExecutionStatus,
    pub provider_kind: Option<LocalProviderKind>,
    pub error_codes: Vec<LocalProviderExecutionError>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderExecutionProjection {
    pub status: LocalProviderExecutionStatus,
    pub projection_status: LocalProviderExecutionResultProjectionStatus,
    pub configured_provider_kind: String,
    pub sandbox_status: LocalProviderExecutionSandboxStatus,
    pub result: Option<LocalProviderExecutionResult>,
    pub output_trust_status: LocalProviderOutputTrustStatus,
    pub output_materialization_status: LocalProviderOutputMaterializationStatus,
    pub output_promotion_status: LocalProviderOutputPromotionStatus,
    pub promotion_availability_status: LocalProviderOutputPromotionStatus,
    pub linkage: LocalProviderExecutionResultLinkage,
    pub absence_markers: LocalProviderExecutionResultAbsenceMarkers,
    pub projection_validation: LocalProviderExecutionResultProjectionValidation,
    pub validation_status: String,
    pub validation_error_codes: Vec<String>,
    pub validation_reason: String,
    pub capability_surface: LocalProviderExecutionCapabilitySurface,
    pub note: String,
}

pub fn local_provider_output_validation_no_effects() -> LocalProviderOutputValidationEffect {
    LocalProviderOutputValidationEffect::None
}

fn local_provider_output_candidate_boundary_statuses(
) -> Vec<LocalProviderOutputCandidateBoundaryStatus> {
    vec![
        LocalProviderOutputCandidateBoundaryStatus::NotCandidateMaterial,
        LocalProviderOutputCandidateBoundaryStatus::CandidateConversionNotPerformed,
        LocalProviderOutputCandidateBoundaryStatus::CandidateConversionRequiresFuturePhase,
    ]
}

pub fn initial_local_provider_output_validation_projection(
) -> LocalProviderOutputValidationProjection {
    LocalProviderOutputValidationProjection {
        status: LocalProviderOutputValidationStatus::NotValidated,
        reviewability_status: LocalProviderOutputReviewabilityStatus::NotReviewable,
        candidate_boundary_status: LocalProviderOutputCandidateBoundaryStatus::NotCandidateMaterial,
        candidate_boundary_statuses: local_provider_output_candidate_boundary_statuses(),
        reasons: vec![
            LocalProviderOutputValidationReason::NoProviderExecutionResult,
            LocalProviderOutputValidationReason::MissingExecutionResult,
            LocalProviderOutputValidationReason::CandidateConversionNotAvailableInPhase143,
        ],
        provider_execution_result_id: None,
        provider_kind: "none".to_string(),
        output_trust_status: LocalProviderOutputTrustStatus::UntrustedDescriptive,
        output_promotion_status: LocalProviderOutputPromotionStatus::NotPromoted,
        trust_effect: local_provider_output_validation_no_effects(),
        candidate_effect: local_provider_output_validation_no_effects(),
        decision_ledger_effect: local_provider_output_validation_no_effects(),
        replay_effect: local_provider_output_validation_no_effects(),
        export_effect: local_provider_output_validation_no_effects(),
        action_effect: local_provider_output_validation_no_effects(),
        readiness_effect: local_provider_output_validation_no_effects(),
        release_effect: local_provider_output_validation_no_effects(),
        deployment_effect: local_provider_output_validation_no_effects(),
        note: "Provider output validation has not run; provider output is not candidate material and cannot be approved in Phase 143.".to_string(),
    }
}

pub fn local_provider_output_validation_reasons(
    execution: &LocalProviderExecutionProjection,
) -> Vec<LocalProviderOutputValidationReason> {
    let mut reasons = std::collections::BTreeSet::new();
    if execution.projection_status
        != LocalProviderExecutionResultProjectionStatus::ExecutionProjected
    {
        reasons.insert(LocalProviderOutputValidationReason::ProviderExecutionNotProjected);
    }

    let Some(result) = execution.result.as_ref() else {
        reasons.insert(LocalProviderOutputValidationReason::NoProviderExecutionResult);
        reasons.insert(LocalProviderOutputValidationReason::MissingExecutionResult);
        reasons
            .insert(LocalProviderOutputValidationReason::CandidateConversionNotAvailableInPhase143);
        return reasons.into_iter().collect();
    };

    if result.provider_kind != LocalProviderKind::DeterministicStub {
        reasons.insert(LocalProviderOutputValidationReason::UnsupportedProviderKind);
    }
    let output = result.output_summary.trim();
    if output.is_empty() {
        reasons.insert(LocalProviderOutputValidationReason::EmptyOutput);
    }
    if result.output_summary.len() > 1024 {
        reasons.insert(LocalProviderOutputValidationReason::OutputTooLarge);
    }
    if !result
        .output_summary
        .starts_with("deterministic_stub descriptive output for input_bytes=")
        || !result.output_summary.contains(" checksum=")
        || result.sandbox_status
            != LocalProviderExecutionSandboxStatus::SandboxedDeterministicNoExternalEffects
        || result.output_trust_status != LocalProviderOutputTrustStatus::UntrustedDescriptive
        || result.output_materialization_status
            != LocalProviderOutputMaterializationStatus::ProjectedAsUntrustedOutput
        || result.output_promotion_status != LocalProviderOutputPromotionStatus::NotPromoted
        || result.promotion_availability_status
            != LocalProviderOutputPromotionStatus::PromotionNotAvailableInPhase142
        || !result.descriptive_only
        || result.provider_output_trusted
        || result.candidate_output_promoted
        || result.decision_appended
        || result.replay_repaired
        || result.release_or_deployment_evidence_created
    {
        reasons.insert(LocalProviderOutputValidationReason::MalformedOutput);
    }

    let lower = result.output_summary.to_ascii_lowercase();
    if ["secret", "token", "api_key", "apikey", "credential"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.insert(LocalProviderOutputValidationReason::ContainsForbiddenSecretMarker);
    }
    if ["command", "shell", "process", "execute ", "run "]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.insert(LocalProviderOutputValidationReason::ContainsExecutionInstruction);
    }
    if ["http://", "https://", "network", "socket", "fetch("]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.insert(LocalProviderOutputValidationReason::ContainsNetworkInstruction);
    }
    if [
        "filesystem",
        "file ",
        "write ",
        "path",
        "directory",
        "fs::write",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        reasons.insert(LocalProviderOutputValidationReason::ContainsFilesystemInstruction);
    }
    if [
        "readiness",
        "release",
        "deployment",
        "public-use",
        "public use",
        "production ready",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        reasons.insert(LocalProviderOutputValidationReason::ContainsReadinessOrReleaseClaim);
    }
    if [
        "trusted_output",
        "trusted output",
        "approved_output",
        "approved output",
        "approval granted",
        "trust_granted",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        reasons.insert(LocalProviderOutputValidationReason::ContainsTrustOrApprovalClaim);
    }
    if [
        "action_executed",
        "authorize action",
        "action authorization",
        "perform action",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        reasons.insert(LocalProviderOutputValidationReason::ContainsActionInstruction);
    }

    if reasons.is_empty() {
        reasons.insert(LocalProviderOutputValidationReason::DeterministicStubOutputShapeValid);
    }
    reasons.insert(LocalProviderOutputValidationReason::CandidateConversionNotAvailableInPhase143);
    reasons.into_iter().collect()
}

pub fn validate_local_provider_output(
    execution: &LocalProviderExecutionProjection,
) -> LocalProviderOutputValidationProjection {
    let reasons = local_provider_output_validation_reasons(execution);
    let result = execution.result.as_ref();
    let provider_kind = result
        .map(|result| result.provider_kind.code().to_string())
        .unwrap_or_else(|| execution.configured_provider_kind.clone());
    let provider_execution_result_id = result.map(|result| result.result_id.clone());
    let output_trust_status = result
        .map(|result| result.output_trust_status)
        .unwrap_or(LocalProviderOutputTrustStatus::UntrustedDescriptive);
    let output_promotion_status = result
        .map(|result| result.output_promotion_status)
        .unwrap_or(LocalProviderOutputPromotionStatus::NotPromoted);

    let status = if result.is_none() {
        LocalProviderOutputValidationStatus::NotValidated
    } else if execution.projection_status
        != LocalProviderExecutionResultProjectionStatus::ExecutionProjected
    {
        LocalProviderOutputValidationStatus::ValidationNotApplicable
    } else if reasons.iter().all(|reason| {
        matches!(
            reason,
            LocalProviderOutputValidationReason::DeterministicStubOutputShapeValid
                | LocalProviderOutputValidationReason::CandidateConversionNotAvailableInPhase143
        )
    }) {
        LocalProviderOutputValidationStatus::ReviewableUntrusted
    } else {
        LocalProviderOutputValidationStatus::Rejected
    };
    let reviewability_status = match status {
        LocalProviderOutputValidationStatus::ReviewableUntrusted => {
            LocalProviderOutputReviewabilityStatus::ReviewableUntrusted
        }
        LocalProviderOutputValidationStatus::Rejected => {
            LocalProviderOutputReviewabilityStatus::RejectedBeforeReview
        }
        _ => LocalProviderOutputReviewabilityStatus::NotReviewable,
    };

    LocalProviderOutputValidationProjection {
        status,
        reviewability_status,
        candidate_boundary_status: LocalProviderOutputCandidateBoundaryStatus::NotCandidateMaterial,
        candidate_boundary_statuses: local_provider_output_candidate_boundary_statuses(),
        reasons,
        provider_execution_result_id,
        provider_kind,
        output_trust_status,
        output_promotion_status,
        trust_effect: LocalProviderOutputValidationEffect::None,
        candidate_effect: LocalProviderOutputValidationEffect::None,
        decision_ledger_effect: LocalProviderOutputValidationEffect::None,
        replay_effect: LocalProviderOutputValidationEffect::None,
        export_effect: LocalProviderOutputValidationEffect::None,
        action_effect: LocalProviderOutputValidationEffect::None,
        readiness_effect: LocalProviderOutputValidationEffect::None,
        release_effect: LocalProviderOutputValidationEffect::None,
        deployment_effect: LocalProviderOutputValidationEffect::None,
        note: "reviewable_untrusted is not candidate material and cannot be approved in Phase 143; provider output is not promoted.".to_string(),
    }
}

pub fn project_local_provider_output_validation(
    state: &LocalOperatorShellState,
) -> LocalProviderOutputValidationProjection {
    validate_local_provider_output(&project_local_provider_execution(state))
}

pub fn validate_local_provider_output_validation_projection(
    projection: &LocalProviderOutputValidationProjection,
) -> Result<(), LocalProviderOutputValidationError> {
    if projection.reasons.is_empty() {
        return Err(LocalProviderOutputValidationError::MissingValidationReason);
    }
    if projection.status == LocalProviderOutputValidationStatus::ReviewableUntrusted
        && (projection.output_trust_status != LocalProviderOutputTrustStatus::UntrustedDescriptive)
    {
        return Err(LocalProviderOutputValidationError::InvalidReviewableTrustStatus);
    }
    if projection.candidate_boundary_status
        != LocalProviderOutputCandidateBoundaryStatus::NotCandidateMaterial
        || !projection
            .candidate_boundary_statuses
            .contains(&LocalProviderOutputCandidateBoundaryStatus::NotCandidateMaterial)
        || !projection
            .candidate_boundary_statuses
            .contains(&LocalProviderOutputCandidateBoundaryStatus::CandidateConversionNotPerformed)
        || !projection.candidate_boundary_statuses.contains(
            &LocalProviderOutputCandidateBoundaryStatus::CandidateConversionRequiresFuturePhase,
        )
    {
        return Err(LocalProviderOutputValidationError::InvalidCandidateBoundaryStatus);
    }
    if projection.output_promotion_status != LocalProviderOutputPromotionStatus::NotPromoted {
        return Err(LocalProviderOutputValidationError::InvalidPromotionStatus);
    }
    let effects = [
        projection.trust_effect,
        projection.candidate_effect,
        projection.decision_ledger_effect,
        projection.replay_effect,
        projection.export_effect,
        projection.action_effect,
        projection.readiness_effect,
        projection.release_effect,
        projection.deployment_effect,
    ];
    if effects
        .iter()
        .any(|effect| *effect != LocalProviderOutputValidationEffect::None)
    {
        return Err(LocalProviderOutputValidationError::InvalidNoEffectBoundary);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedCandidateConversionProposalStatus {
    NoProposal,
    StagedProposalCreated,
    SourceNotReviewableUntrusted,
    RejectedSourceNotEligible,
    InvalidProposalRequest,
}

impl StagedCandidateConversionProposalStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoProposal => "no_proposal",
            Self::StagedProposalCreated => "staged_proposal_created",
            Self::SourceNotReviewableUntrusted => "source_not_reviewable_untrusted",
            Self::RejectedSourceNotEligible => "rejected_source_not_eligible",
            Self::InvalidProposalRequest => "invalid_proposal_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedCandidateConversionBoundaryStatus {
    StagingOnlyNotCandidateMaterial,
    CandidateConversionNotPerformed,
    ValidationRequiredInFuturePhase,
    ApprovalNotAvailableInPhase146,
}

impl StagedCandidateConversionBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::StagingOnlyNotCandidateMaterial => "staging_only_not_candidate_material",
            Self::CandidateConversionNotPerformed => "candidate_conversion_not_performed",
            Self::ValidationRequiredInFuturePhase => "validation_required_in_future_phase",
            Self::ApprovalNotAvailableInPhase146 => "approval_not_available_in_phase_146",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedCandidateConversionTrustStatus {
    UntrustedSource,
    NotTrusted,
    NotApproved,
}

impl StagedCandidateConversionTrustStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UntrustedSource => "untrusted_source",
            Self::NotTrusted => "not_trusted",
            Self::NotApproved => "not_approved",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedCandidateConversionEffectStatus {
    NoDecisionLedgerEffect,
    NoReplayEffect,
    NoExportEffect,
    NoProviderConfigurationEffect,
    NoProviderExecutionEffect,
    NoActionEffect,
    NoPersistenceEffect,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NotExecutable,
    NotPersistent,
}

impl StagedCandidateConversionEffectStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoDecisionLedgerEffect => "no_decision_ledger_effect",
            Self::NoReplayEffect => "no_replay_effect",
            Self::NoExportEffect => "no_export_effect",
            Self::NoProviderConfigurationEffect => "no_provider_configuration_effect",
            Self::NoProviderExecutionEffect => "no_provider_execution_effect",
            Self::NoActionEffect => "no_action_effect",
            Self::NoPersistenceEffect => "no_persistence_effect",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NotExecutable => "not_executable",
            Self::NotPersistent => "not_persistent",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedCandidateConversionSourceEligibilityStatus {
    EligibleReviewableUntrusted,
    MissingProviderExecutionResult,
    SourceNotReviewableUntrusted,
    RejectedSourceNotEligible,
    ValidationNotApplicableSourceNotEligible,
    InvalidValidationInputSourceNotEligible,
    MissingOrInconsistentValidationProjection,
}

impl StagedCandidateConversionSourceEligibilityStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EligibleReviewableUntrusted => "eligible_reviewable_untrusted",
            Self::MissingProviderExecutionResult => "missing_provider_execution_result",
            Self::SourceNotReviewableUntrusted => "source_not_reviewable_untrusted",
            Self::RejectedSourceNotEligible => "rejected_source_not_eligible",
            Self::ValidationNotApplicableSourceNotEligible => {
                "validation_not_applicable_source_not_eligible"
            }
            Self::InvalidValidationInputSourceNotEligible => {
                "invalid_validation_input_source_not_eligible"
            }
            Self::MissingOrInconsistentValidationProjection => {
                "missing_or_inconsistent_validation_projection"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedCandidateConversionProposalError {
    MissingProviderExecutionResult,
    SourceNotReviewableUntrusted,
    RejectedSourceNotEligible,
    ValidationNotApplicableSourceNotEligible,
    InvalidValidationInputSourceNotEligible,
    MissingOrInconsistentValidationProjection,
    InvalidProposalRequest,
    InvalidProposalBoundary,
}

impl StagedCandidateConversionProposalError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingProviderExecutionResult => "missing_provider_execution_result",
            Self::SourceNotReviewableUntrusted => "source_not_reviewable_untrusted",
            Self::RejectedSourceNotEligible => "rejected_source_not_eligible",
            Self::ValidationNotApplicableSourceNotEligible => {
                "validation_not_applicable_source_not_eligible"
            }
            Self::InvalidValidationInputSourceNotEligible => {
                "invalid_validation_input_source_not_eligible"
            }
            Self::MissingOrInconsistentValidationProjection => {
                "missing_or_inconsistent_validation_projection"
            }
            Self::InvalidProposalRequest => "invalid_proposal_request",
            Self::InvalidProposalBoundary => "invalid_proposal_boundary",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedCandidateConversionProposalRequest {
    pub operator_note: String,
    pub claims: Vec<(String, String)>,
}

impl StagedCandidateConversionProposalRequest {
    pub fn staging_only(operator_note: impl Into<String>) -> Self {
        Self {
            operator_note: operator_note.into(),
            claims: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedCandidateConversionProposal {
    pub proposal_id: String,
    pub source_provider_kind: String,
    pub source_execution_result_id: String,
    pub source_validation_status: LocalProviderOutputValidationStatus,
    pub source_reviewability_status: LocalProviderOutputReviewabilityStatus,
    pub source_candidate_boundary_status: LocalProviderOutputCandidateBoundaryStatus,
    pub source_boundary: String,
    pub proposal_boundary: String,
    pub boundary_statuses: Vec<StagedCandidateConversionBoundaryStatus>,
    pub trust_statuses: Vec<StagedCandidateConversionTrustStatus>,
    pub effect_statuses: Vec<StagedCandidateConversionEffectStatus>,
    pub source_eligibility_status: StagedCandidateConversionSourceEligibilityStatus,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedCandidateConversionProposalProjection {
    pub status: StagedCandidateConversionProposalStatus,
    pub proposal: Option<StagedCandidateConversionProposal>,
    pub source_eligibility_status: StagedCandidateConversionSourceEligibilityStatus,
    pub error: Option<StagedCandidateConversionProposalError>,
    pub note: String,
}

pub fn staged_candidate_conversion_no_effects() -> Vec<StagedCandidateConversionEffectStatus> {
    vec![
        StagedCandidateConversionEffectStatus::NoDecisionLedgerEffect,
        StagedCandidateConversionEffectStatus::NoReplayEffect,
        StagedCandidateConversionEffectStatus::NoExportEffect,
        StagedCandidateConversionEffectStatus::NoProviderConfigurationEffect,
        StagedCandidateConversionEffectStatus::NoProviderExecutionEffect,
        StagedCandidateConversionEffectStatus::NoActionEffect,
        StagedCandidateConversionEffectStatus::NoPersistenceEffect,
        StagedCandidateConversionEffectStatus::NoReadinessEffect,
        StagedCandidateConversionEffectStatus::NoReleaseEffect,
        StagedCandidateConversionEffectStatus::NoDeploymentEffect,
        StagedCandidateConversionEffectStatus::NotExecutable,
        StagedCandidateConversionEffectStatus::NotPersistent,
    ]
}

fn staged_candidate_conversion_boundary_statuses() -> Vec<StagedCandidateConversionBoundaryStatus> {
    vec![
        StagedCandidateConversionBoundaryStatus::StagingOnlyNotCandidateMaterial,
        StagedCandidateConversionBoundaryStatus::CandidateConversionNotPerformed,
        StagedCandidateConversionBoundaryStatus::ValidationRequiredInFuturePhase,
        StagedCandidateConversionBoundaryStatus::ApprovalNotAvailableInPhase146,
    ]
}

fn staged_candidate_conversion_trust_statuses() -> Vec<StagedCandidateConversionTrustStatus> {
    vec![
        StagedCandidateConversionTrustStatus::UntrustedSource,
        StagedCandidateConversionTrustStatus::NotTrusted,
        StagedCandidateConversionTrustStatus::NotApproved,
    ]
}

pub fn initial_staged_candidate_conversion_proposal_projection(
) -> StagedCandidateConversionProposalProjection {
    StagedCandidateConversionProposalProjection {
        status: StagedCandidateConversionProposalStatus::NoProposal,
        proposal: None,
        source_eligibility_status: StagedCandidateConversionSourceEligibilityStatus::MissingProviderExecutionResult,
        error: None,
        note: "No staged candidate-conversion proposal exists; Phase 146 staging is proposal only and not candidate material.".to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedCandidateConversionValidationStatus {
    NotValidated,
    StagedProposalShapeValid,
    RejectedStagedProposal,
    InvalidValidationInput,
}

impl StagedCandidateConversionValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotValidated => "not_validated",
            Self::StagedProposalShapeValid => "staged_proposal_shape_valid",
            Self::RejectedStagedProposal => "rejected_staged_proposal",
            Self::InvalidValidationInput => "invalid_validation_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StagedCandidateConversionValidationReason {
    NoStagedProposal,
    SourceLinkageValidated,
    StagedProposalShapeValid,
    SourceNotReviewableUntrusted,
    ProviderOutputValidationMissing,
    ProviderOutputValidationInconsistent,
    ProviderExecutionResultMissing,
    ProviderExecutionResultMalformed,
    DeterministicProposalIdMismatch,
    ExecutionResultIdMismatch,
    SourceValidationStatusMismatch,
    SourceReviewabilityStatusMismatch,
    SourceCandidateBoundaryStatusMismatch,
    BoundaryFlagMissing,
    BoundaryFlagDrift,
    NoEffectFieldMissing,
    NoEffectFieldDrift,
    FuturePhaseMarkerMissing,
    ContainsTrustClaim,
    ContainsApprovalClaim,
    ContainsSafetyClaim,
    ContainsReadinessClaim,
    ContainsReleaseClaim,
    ContainsDeploymentClaim,
    ContainsPublicUseClaim,
    ContainsActionClaim,
    ContainsPersistenceClaim,
    ContainsExecutionClaim,
    ContainsCandidateCreationClaim,
    ContainsCandidateMaterializationClaim,
    CandidateMaterializationNotPerformed,
    FutureReviewBoundaryRequired,
    OperatorDecisionNotAvailableInPhase147,
}

impl StagedCandidateConversionValidationReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoStagedProposal => "no_staged_proposal",
            Self::SourceLinkageValidated => "source_linkage_validated",
            Self::StagedProposalShapeValid => "staged_proposal_shape_valid",
            Self::SourceNotReviewableUntrusted => "source_not_reviewable_untrusted",
            Self::ProviderOutputValidationMissing => "provider_output_validation_missing",
            Self::ProviderOutputValidationInconsistent => "provider_output_validation_inconsistent",
            Self::ProviderExecutionResultMissing => "provider_execution_result_missing",
            Self::ProviderExecutionResultMalformed => "provider_execution_result_malformed",
            Self::DeterministicProposalIdMismatch => "deterministic_proposal_id_mismatch",
            Self::ExecutionResultIdMismatch => "execution_result_id_mismatch",
            Self::SourceValidationStatusMismatch => "source_validation_status_mismatch",
            Self::SourceReviewabilityStatusMismatch => "source_reviewability_status_mismatch",
            Self::SourceCandidateBoundaryStatusMismatch => {
                "source_candidate_boundary_status_mismatch"
            }
            Self::BoundaryFlagMissing => "boundary_flag_missing",
            Self::BoundaryFlagDrift => "boundary_flag_drift",
            Self::NoEffectFieldMissing => "no_effect_field_missing",
            Self::NoEffectFieldDrift => "no_effect_field_drift",
            Self::FuturePhaseMarkerMissing => "future_phase_marker_missing",
            Self::ContainsTrustClaim => "contains_trust_claim",
            Self::ContainsApprovalClaim => "contains_approval_claim",
            Self::ContainsSafetyClaim => "contains_safety_claim",
            Self::ContainsReadinessClaim => "contains_readiness_claim",
            Self::ContainsReleaseClaim => "contains_release_claim",
            Self::ContainsDeploymentClaim => "contains_deployment_claim",
            Self::ContainsPublicUseClaim => "contains_public_use_claim",
            Self::ContainsActionClaim => "contains_action_claim",
            Self::ContainsPersistenceClaim => "contains_persistence_claim",
            Self::ContainsExecutionClaim => "contains_execution_claim",
            Self::ContainsCandidateCreationClaim => "contains_candidate_creation_claim",
            Self::ContainsCandidateMaterializationClaim => {
                "contains_candidate_materialization_claim"
            }
            Self::CandidateMaterializationNotPerformed => "candidate_materialization_not_performed",
            Self::FutureReviewBoundaryRequired => "future_review_boundary_required",
            Self::OperatorDecisionNotAvailableInPhase147 => {
                "operator_decision_not_available_in_phase_147"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedCandidateConversionMaterializationStatus {
    NotMaterialized,
    MaterializationNotAvailableInPhase147,
    MaterializationRequiresFuturePhase,
}

impl StagedCandidateConversionMaterializationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotMaterialized => "not_materialized",
            Self::MaterializationNotAvailableInPhase147 => {
                "materialization_not_available_in_phase_147"
            }
            Self::MaterializationRequiresFuturePhase => "materialization_requires_future_phase",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedCandidateConversionOperatorDecisionStatus {
    NotAvailableInPhase147,
}

impl StagedCandidateConversionOperatorDecisionStatus {
    pub fn code(&self) -> &'static str {
        "not_available_in_phase_147"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedCandidateConversionValidationBoundaryStatus {
    ValidationShapeOnly,
    CandidateMaterializationNotPerformed,
    FutureReviewBoundaryRequired,
    OperatorDecisionNotAvailableInPhase147,
}

impl StagedCandidateConversionValidationBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ValidationShapeOnly => "validation_shape_only",
            Self::CandidateMaterializationNotPerformed => "candidate_materialization_not_performed",
            Self::FutureReviewBoundaryRequired => "future_review_boundary_required",
            Self::OperatorDecisionNotAvailableInPhase147 => {
                "operator_decision_not_available_in_phase_147"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedCandidateConversionValidationRequest {
    pub proposal_id: Option<String>,
}

impl StagedCandidateConversionValidationRequest {
    pub fn existing_staged_proposal() -> Self {
        Self { proposal_id: None }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedCandidateConversionValidationProjection {
    pub status: StagedCandidateConversionValidationStatus,
    pub reasons: Vec<StagedCandidateConversionValidationReason>,
    pub proposal_id: Option<String>,
    pub source_provider_kind: String,
    pub source_execution_result_id: Option<String>,
    pub source_validation_status: String,
    pub source_reviewability_status: String,
    pub source_candidate_boundary_status: String,
    pub deterministic_linkage_status: String,
    pub materialization_statuses: Vec<StagedCandidateConversionMaterializationStatus>,
    pub future_review_boundary_status: StagedCandidateConversionValidationBoundaryStatus,
    pub operator_decision_status: StagedCandidateConversionOperatorDecisionStatus,
    pub trust_statuses: Vec<StagedCandidateConversionTrustStatus>,
    pub boundary_statuses: Vec<StagedCandidateConversionValidationBoundaryStatus>,
    pub no_effect_summary: Vec<StagedCandidateConversionEffectStatus>,
    pub note: String,
}

pub fn initial_staged_candidate_conversion_validation_projection(
) -> StagedCandidateConversionValidationProjection {
    StagedCandidateConversionValidationProjection {
        status: StagedCandidateConversionValidationStatus::NotValidated,
        reasons: Vec::new(),
        proposal_id: None,
        source_provider_kind: "none".to_string(),
        source_execution_result_id: None,
        source_validation_status: "not_validated".to_string(),
        source_reviewability_status: "not_reviewable".to_string(),
        source_candidate_boundary_status: "not_candidate_material".to_string(),
        deterministic_linkage_status: "not_validated".to_string(),
        materialization_statuses: vec![
            StagedCandidateConversionMaterializationStatus::NotMaterialized,
            StagedCandidateConversionMaterializationStatus::MaterializationNotAvailableInPhase147,
            StagedCandidateConversionMaterializationStatus::MaterializationRequiresFuturePhase,
        ],
        future_review_boundary_status:
            StagedCandidateConversionValidationBoundaryStatus::FutureReviewBoundaryRequired,
        operator_decision_status:
            StagedCandidateConversionOperatorDecisionStatus::NotAvailableInPhase147,
        trust_statuses: staged_candidate_conversion_trust_statuses(),
        boundary_statuses: staged_candidate_conversion_validation_boundary_statuses(),
        no_effect_summary: staged_candidate_conversion_no_effects(),
        note: "Validation checks staged proposal shape and source linkage only. Validated staged proposal is not candidate output. Candidate materialization was not performed in Phase 147. Future review boundary is required before any operator decision. Operator decision is not available in Phase 147. Provider output remains untrusted and not approved.".to_string(),
    }
}

fn staged_candidate_conversion_validation_boundary_statuses(
) -> Vec<StagedCandidateConversionValidationBoundaryStatus> {
    vec![
        StagedCandidateConversionValidationBoundaryStatus::ValidationShapeOnly,
        StagedCandidateConversionValidationBoundaryStatus::CandidateMaterializationNotPerformed,
        StagedCandidateConversionValidationBoundaryStatus::FutureReviewBoundaryRequired,
        StagedCandidateConversionValidationBoundaryStatus::OperatorDecisionNotAvailableInPhase147,
    ]
}

fn proposal_note_claim_reasons(
    proposal: &StagedCandidateConversionProposal,
) -> Vec<StagedCandidateConversionValidationReason> {
    let lower = proposal.note.to_ascii_lowercase();
    let mut reasons = Vec::new();
    if ["trust granted", "is trusted", "mark trusted"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.push(StagedCandidateConversionValidationReason::ContainsTrustClaim);
    }
    if ["approval granted", "is approved", "mark approved"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.push(StagedCandidateConversionValidationReason::ContainsApprovalClaim);
    }
    if ["is safe", "safe output", "safe for"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.push(StagedCandidateConversionValidationReason::ContainsSafetyClaim);
    }
    if ["readiness", "ready for"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.push(StagedCandidateConversionValidationReason::ContainsReadinessClaim);
    }
    if ["release claim", "release evidence"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.push(StagedCandidateConversionValidationReason::ContainsReleaseClaim);
    }
    if ["deployment claim", "deployment evidence"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.push(StagedCandidateConversionValidationReason::ContainsDeploymentClaim);
    }
    if ["public use", "public-use"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.push(StagedCandidateConversionValidationReason::ContainsPublicUseClaim);
    }
    if ["action claim", "action effect"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.push(StagedCandidateConversionValidationReason::ContainsActionClaim);
    }
    if ["persistence claim", "persisted"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.push(StagedCandidateConversionValidationReason::ContainsPersistenceClaim);
    }
    if ["execution claim", "executed proposal"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.push(StagedCandidateConversionValidationReason::ContainsExecutionClaim);
    }
    if ["candidate creation", "candidate output created"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons.push(StagedCandidateConversionValidationReason::ContainsCandidateCreationClaim);
    }
    if ["candidate materialization", "materialized candidate"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        reasons
            .push(StagedCandidateConversionValidationReason::ContainsCandidateMaterializationClaim);
    }
    reasons
}

fn same_set<T: Eq + Clone>(left: &[T], right: &[T]) -> bool {
    left.len() == right.len() && right.iter().all(|item| left.contains(item))
}

pub fn staged_candidate_conversion_validation_reasons(
    state: &LocalOperatorShellState,
    request: &StagedCandidateConversionValidationRequest,
) -> Vec<StagedCandidateConversionValidationReason> {
    let mut reasons = std::collections::BTreeSet::new();
    let Some(proposal) = state.staged_candidate_conversion_proposal.proposal.as_ref() else {
        reasons.insert(StagedCandidateConversionValidationReason::NoStagedProposal);
        return reasons.into_iter().collect();
    };
    if let Some(requested_id) = request.proposal_id.as_ref() {
        if requested_id != &proposal.proposal_id {
            reasons
                .insert(StagedCandidateConversionValidationReason::DeterministicProposalIdMismatch);
        }
    }
    let Some(result) = state.provider_execution.result.as_ref() else {
        reasons.insert(StagedCandidateConversionValidationReason::ProviderExecutionResultMissing);
        return reasons.into_iter().collect();
    };
    if state.provider_execution.projection_validation.status
        != LocalProviderExecutionResultProjectionValidationStatus::Valid
        || state.provider_execution.projection_status
            != LocalProviderExecutionResultProjectionStatus::ExecutionProjected
    {
        reasons.insert(StagedCandidateConversionValidationReason::ProviderExecutionResultMalformed);
    }
    if state.provider_output_validation.reasons.is_empty()
        || state
            .provider_output_validation
            .provider_execution_result_id
            .is_none()
    {
        reasons.insert(StagedCandidateConversionValidationReason::ProviderOutputValidationMissing);
    } else if validate_local_provider_output_validation_projection(
        &state.provider_output_validation,
    )
    .is_err()
        || project_local_provider_output_validation(state) != state.provider_output_validation
    {
        reasons.insert(
            StagedCandidateConversionValidationReason::ProviderOutputValidationInconsistent,
        );
    }
    if state.provider_output_validation.status
        != LocalProviderOutputValidationStatus::ReviewableUntrusted
    {
        reasons.insert(StagedCandidateConversionValidationReason::SourceNotReviewableUntrusted);
    }
    let Some(validation_execution_id) = state
        .provider_output_validation
        .provider_execution_result_id
        .as_ref()
    else {
        reasons.insert(StagedCandidateConversionValidationReason::ProviderOutputValidationMissing);
        return reasons.into_iter().collect();
    };
    if result.result_id != *validation_execution_id
        || proposal.source_execution_result_id != *validation_execution_id
    {
        reasons.insert(StagedCandidateConversionValidationReason::ExecutionResultIdMismatch);
    }
    let expected_id = deterministic_staged_candidate_conversion_proposal_id(
        validation_execution_id,
        &state.provider_output_validation,
    );
    if proposal.proposal_id != expected_id {
        reasons.insert(StagedCandidateConversionValidationReason::DeterministicProposalIdMismatch);
    }
    if proposal.source_validation_status != state.provider_output_validation.status
        || proposal.source_validation_status
            != LocalProviderOutputValidationStatus::ReviewableUntrusted
    {
        reasons.insert(StagedCandidateConversionValidationReason::SourceValidationStatusMismatch);
    }
    if proposal.source_reviewability_status != state.provider_output_validation.reviewability_status
        || proposal.source_reviewability_status
            != LocalProviderOutputReviewabilityStatus::ReviewableUntrusted
    {
        reasons
            .insert(StagedCandidateConversionValidationReason::SourceReviewabilityStatusMismatch);
    }
    if proposal.source_candidate_boundary_status
        != state.provider_output_validation.candidate_boundary_status
        || proposal.source_candidate_boundary_status
            != LocalProviderOutputCandidateBoundaryStatus::NotCandidateMaterial
    {
        reasons.insert(
            StagedCandidateConversionValidationReason::SourceCandidateBoundaryStatusMismatch,
        );
    }
    if !staged_candidate_conversion_boundary_statuses()
        .iter()
        .all(|status| proposal.boundary_statuses.contains(status))
    {
        reasons.insert(StagedCandidateConversionValidationReason::BoundaryFlagMissing);
    }
    if !same_set(
        &proposal.boundary_statuses,
        &staged_candidate_conversion_boundary_statuses(),
    ) {
        reasons.insert(StagedCandidateConversionValidationReason::BoundaryFlagDrift);
    }
    if !proposal
        .boundary_statuses
        .contains(&StagedCandidateConversionBoundaryStatus::ValidationRequiredInFuturePhase)
        || !proposal
            .boundary_statuses
            .contains(&StagedCandidateConversionBoundaryStatus::ApprovalNotAvailableInPhase146)
    {
        reasons.insert(StagedCandidateConversionValidationReason::FuturePhaseMarkerMissing);
    }
    if !staged_candidate_conversion_trust_statuses()
        .iter()
        .all(|status| proposal.trust_statuses.contains(status))
    {
        reasons.insert(StagedCandidateConversionValidationReason::BoundaryFlagMissing);
    }
    if !same_set(
        &proposal.trust_statuses,
        &staged_candidate_conversion_trust_statuses(),
    ) {
        reasons.insert(StagedCandidateConversionValidationReason::BoundaryFlagDrift);
    }
    if !staged_candidate_conversion_no_effects()
        .iter()
        .all(|status| proposal.effect_statuses.contains(status))
    {
        reasons.insert(StagedCandidateConversionValidationReason::NoEffectFieldMissing);
    }
    if !same_set(
        &proposal.effect_statuses,
        &staged_candidate_conversion_no_effects(),
    ) {
        reasons.insert(StagedCandidateConversionValidationReason::NoEffectFieldDrift);
    }
    if proposal.source_boundary != "provider_output_validation_phase_143"
        || proposal.proposal_boundary != "staged_candidate_conversion_phase_146"
        || proposal.source_eligibility_status
            != StagedCandidateConversionSourceEligibilityStatus::EligibleReviewableUntrusted
    {
        reasons.insert(StagedCandidateConversionValidationReason::BoundaryFlagDrift);
    }
    for reason in proposal_note_claim_reasons(proposal) {
        reasons.insert(reason);
    }
    if reasons.is_empty() {
        reasons.insert(StagedCandidateConversionValidationReason::StagedProposalShapeValid);
        reasons.insert(StagedCandidateConversionValidationReason::SourceLinkageValidated);
        reasons.insert(
            StagedCandidateConversionValidationReason::CandidateMaterializationNotPerformed,
        );
        reasons.insert(StagedCandidateConversionValidationReason::FutureReviewBoundaryRequired);
        reasons.insert(
            StagedCandidateConversionValidationReason::OperatorDecisionNotAvailableInPhase147,
        );
    }
    reasons.into_iter().collect()
}

pub fn project_staged_candidate_conversion_validation(
    state: &LocalOperatorShellState,
    request: &StagedCandidateConversionValidationRequest,
) -> StagedCandidateConversionValidationProjection {
    let reasons = staged_candidate_conversion_validation_reasons(state, request);
    let proposal = state.staged_candidate_conversion_proposal.proposal.as_ref();
    let status = if reasons.contains(&StagedCandidateConversionValidationReason::NoStagedProposal) {
        StagedCandidateConversionValidationStatus::InvalidValidationInput
    } else if reasons.iter().all(|reason| {
        matches!(
            reason,
            StagedCandidateConversionValidationReason::StagedProposalShapeValid
                | StagedCandidateConversionValidationReason::SourceLinkageValidated
                | StagedCandidateConversionValidationReason::CandidateMaterializationNotPerformed
                | StagedCandidateConversionValidationReason::FutureReviewBoundaryRequired
                | StagedCandidateConversionValidationReason::OperatorDecisionNotAvailableInPhase147
        )
    }) {
        StagedCandidateConversionValidationStatus::StagedProposalShapeValid
    } else {
        StagedCandidateConversionValidationStatus::RejectedStagedProposal
    };
    StagedCandidateConversionValidationProjection {
        status,
        reasons,
        proposal_id: proposal.map(|proposal| proposal.proposal_id.clone()),
        source_provider_kind: proposal
            .map(|proposal| proposal.source_provider_kind.clone())
            .unwrap_or_else(|| "none".to_string()),
        source_execution_result_id: proposal.map(|proposal| proposal.source_execution_result_id.clone()),
        source_validation_status: proposal
            .map(|proposal| proposal.source_validation_status.code().to_string())
            .unwrap_or_else(|| state.provider_output_validation.status.code().to_string()),
        source_reviewability_status: proposal
            .map(|proposal| proposal.source_reviewability_status.code().to_string())
            .unwrap_or_else(|| state.provider_output_validation.reviewability_status.code().to_string()),
        source_candidate_boundary_status: proposal
            .map(|proposal| proposal.source_candidate_boundary_status.code().to_string())
            .unwrap_or_else(|| state.provider_output_validation.candidate_boundary_status.code().to_string()),
        deterministic_linkage_status: if status
            == StagedCandidateConversionValidationStatus::StagedProposalShapeValid
        {
            "source_linkage_validated".to_string()
        } else {
            "not_validated".to_string()
        },
        materialization_statuses: vec![
            StagedCandidateConversionMaterializationStatus::NotMaterialized,
            StagedCandidateConversionMaterializationStatus::MaterializationNotAvailableInPhase147,
            StagedCandidateConversionMaterializationStatus::MaterializationRequiresFuturePhase,
        ],
        future_review_boundary_status:
            StagedCandidateConversionValidationBoundaryStatus::FutureReviewBoundaryRequired,
        operator_decision_status:
            StagedCandidateConversionOperatorDecisionStatus::NotAvailableInPhase147,
        trust_statuses: staged_candidate_conversion_trust_statuses(),
        boundary_statuses: staged_candidate_conversion_validation_boundary_statuses(),
        no_effect_summary: staged_candidate_conversion_no_effects(),
        note: "Validation checks staged proposal shape and source linkage only. Validated staged proposal is not candidate output. Candidate materialization was not performed in Phase 147. Future review boundary is required before any operator decision. Operator decision is not available in Phase 147. Provider output remains untrusted and not approved.".to_string(),
    }
}

pub fn validate_staged_candidate_conversion_proposal_for_phase_147(
    state: &LocalOperatorShellState,
    request: StagedCandidateConversionValidationRequest,
) -> LocalOperatorShellState {
    let mut next = state.clone();
    next.staged_candidate_conversion_validation =
        project_staged_candidate_conversion_validation(state, &request);
    next
}

fn deterministic_staged_candidate_conversion_proposal_id(
    execution_result_id: &str,
    validation: &LocalProviderOutputValidationProjection,
) -> String {
    let input = format!(
        "{}|{}|{}|{}|{}|phase_146",
        validation.provider_kind,
        execution_result_id,
        validation.status.code(),
        validation.reviewability_status.code(),
        validation.candidate_boundary_status.code()
    );
    let mut accumulator: u32 = 2_166_136_261;
    for byte in input.as_bytes() {
        accumulator ^= u32::from(*byte);
        accumulator = accumulator.wrapping_mul(16_777_619);
    }
    format!("staged-candidate-conversion-proposal-{accumulator:08x}")
}

fn proposal_request_contains_forbidden_claim(
    request: &StagedCandidateConversionProposalRequest,
) -> bool {
    request.claims.iter().any(|(key, value)| {
        let text = format!("{} {}", key, value).to_ascii_lowercase();
        [
            "trust",
            "approval",
            "approved",
            "safe",
            "readiness",
            "ready",
            "release",
            "deployment",
            "public-use",
            "public_use",
            "execute",
            "execution",
            "persistence",
            "persistent",
            "action",
            "candidate_creation",
            "candidate_output",
            "candidate_material",
            "conversion_performed",
        ]
        .iter()
        .any(|needle| text.contains(needle))
    })
}

pub fn validate_staged_candidate_conversion_source(
    state: &LocalOperatorShellState,
) -> StagedCandidateConversionSourceEligibilityStatus {
    if state.provider_execution.result.is_none() {
        return StagedCandidateConversionSourceEligibilityStatus::MissingProviderExecutionResult;
    }
    if validate_local_provider_output_validation_projection(&state.provider_output_validation)
        .is_err()
    {
        return StagedCandidateConversionSourceEligibilityStatus::MissingOrInconsistentValidationProjection;
    }
    let projected = project_local_provider_output_validation(state);
    if projected != state.provider_output_validation {
        return StagedCandidateConversionSourceEligibilityStatus::MissingOrInconsistentValidationProjection;
    }
    match state.provider_output_validation.status {
        LocalProviderOutputValidationStatus::Rejected => {
            return StagedCandidateConversionSourceEligibilityStatus::RejectedSourceNotEligible;
        }
        LocalProviderOutputValidationStatus::NotValidated => {
            return StagedCandidateConversionSourceEligibilityStatus::SourceNotReviewableUntrusted;
        }
        LocalProviderOutputValidationStatus::ValidationNotApplicable => {
            return StagedCandidateConversionSourceEligibilityStatus::ValidationNotApplicableSourceNotEligible;
        }
        LocalProviderOutputValidationStatus::InvalidValidationInput => {
            return StagedCandidateConversionSourceEligibilityStatus::InvalidValidationInputSourceNotEligible;
        }
        LocalProviderOutputValidationStatus::ReviewableUntrusted => {}
    }
    if state.provider_output_validation.reviewability_status
        != LocalProviderOutputReviewabilityStatus::ReviewableUntrusted
        || state.provider_output_validation.candidate_boundary_status
            != LocalProviderOutputCandidateBoundaryStatus::NotCandidateMaterial
        || !state
            .provider_output_validation
            .candidate_boundary_statuses
            .contains(&LocalProviderOutputCandidateBoundaryStatus::CandidateConversionNotPerformed)
        || !state
            .provider_output_validation
            .candidate_boundary_statuses
            .contains(
                &LocalProviderOutputCandidateBoundaryStatus::CandidateConversionRequiresFuturePhase,
            )
    {
        return StagedCandidateConversionSourceEligibilityStatus::SourceNotReviewableUntrusted;
    }
    StagedCandidateConversionSourceEligibilityStatus::EligibleReviewableUntrusted
}

fn source_eligibility_error(
    status: StagedCandidateConversionSourceEligibilityStatus,
) -> Option<StagedCandidateConversionProposalError> {
    match status {
        StagedCandidateConversionSourceEligibilityStatus::EligibleReviewableUntrusted => None,
        StagedCandidateConversionSourceEligibilityStatus::MissingProviderExecutionResult => {
            Some(StagedCandidateConversionProposalError::MissingProviderExecutionResult)
        }
        StagedCandidateConversionSourceEligibilityStatus::SourceNotReviewableUntrusted => {
            Some(StagedCandidateConversionProposalError::SourceNotReviewableUntrusted)
        }
        StagedCandidateConversionSourceEligibilityStatus::RejectedSourceNotEligible => {
            Some(StagedCandidateConversionProposalError::RejectedSourceNotEligible)
        }
        StagedCandidateConversionSourceEligibilityStatus::ValidationNotApplicableSourceNotEligible => {
            Some(StagedCandidateConversionProposalError::ValidationNotApplicableSourceNotEligible)
        }
        StagedCandidateConversionSourceEligibilityStatus::InvalidValidationInputSourceNotEligible => {
            Some(StagedCandidateConversionProposalError::InvalidValidationInputSourceNotEligible)
        }
        StagedCandidateConversionSourceEligibilityStatus::MissingOrInconsistentValidationProjection => {
            Some(StagedCandidateConversionProposalError::MissingOrInconsistentValidationProjection)
        }
    }
}

pub fn validate_staged_candidate_conversion_proposal(
    projection: &StagedCandidateConversionProposalProjection,
) -> Result<(), StagedCandidateConversionProposalError> {
    if projection.status == StagedCandidateConversionProposalStatus::NoProposal {
        return Ok(());
    }
    let Some(proposal) = projection.proposal.as_ref() else {
        return Err(StagedCandidateConversionProposalError::InvalidProposalBoundary);
    };
    for required in staged_candidate_conversion_boundary_statuses() {
        if !proposal.boundary_statuses.contains(&required) {
            return Err(StagedCandidateConversionProposalError::InvalidProposalBoundary);
        }
    }
    for required in staged_candidate_conversion_trust_statuses() {
        if !proposal.trust_statuses.contains(&required) {
            return Err(StagedCandidateConversionProposalError::InvalidProposalBoundary);
        }
    }
    for required in staged_candidate_conversion_no_effects() {
        if !proposal.effect_statuses.contains(&required) {
            return Err(StagedCandidateConversionProposalError::InvalidProposalBoundary);
        }
    }
    if proposal.source_validation_status != LocalProviderOutputValidationStatus::ReviewableUntrusted
        || proposal.source_reviewability_status
            != LocalProviderOutputReviewabilityStatus::ReviewableUntrusted
        || proposal.source_candidate_boundary_status
            != LocalProviderOutputCandidateBoundaryStatus::NotCandidateMaterial
        || proposal.source_boundary != "provider_output_validation_phase_143"
        || proposal.proposal_boundary != "staged_candidate_conversion_phase_146"
        || proposal.source_eligibility_status
            != StagedCandidateConversionSourceEligibilityStatus::EligibleReviewableUntrusted
    {
        return Err(StagedCandidateConversionProposalError::InvalidProposalBoundary);
    }
    Ok(())
}

pub fn project_staged_candidate_conversion_proposal(
    proposal: StagedCandidateConversionProposal,
) -> StagedCandidateConversionProposalProjection {
    let projection = StagedCandidateConversionProposalProjection {
        status: StagedCandidateConversionProposalStatus::StagedProposalCreated,
        proposal: Some(proposal),
        source_eligibility_status:
            StagedCandidateConversionSourceEligibilityStatus::EligibleReviewableUntrusted,
        error: None,
        note: "This is a staged conversion proposal only. It is not candidate output.".to_string(),
    };
    match validate_staged_candidate_conversion_proposal(&projection) {
        Ok(()) => projection,
        Err(error) => StagedCandidateConversionProposalProjection {
            status: StagedCandidateConversionProposalStatus::InvalidProposalRequest,
            proposal: None,
            source_eligibility_status: StagedCandidateConversionSourceEligibilityStatus::MissingOrInconsistentValidationProjection,
            error: Some(error),
            note: "Staged candidate-conversion proposal rejected fail-closed; no candidate conversion was performed.".to_string(),
        },
    }
}

pub fn create_staged_candidate_conversion_proposal(
    state: &LocalOperatorShellState,
    request: StagedCandidateConversionProposalRequest,
) -> Result<LocalOperatorShellState, StagedCandidateConversionProposalError> {
    if proposal_request_contains_forbidden_claim(&request) {
        return Err(StagedCandidateConversionProposalError::InvalidProposalRequest);
    }
    let eligibility = validate_staged_candidate_conversion_source(state);
    if let Some(error) = source_eligibility_error(eligibility) {
        return Err(error);
    }
    let validation = &state.provider_output_validation;
    let execution_result_id = validation
        .provider_execution_result_id
        .clone()
        .ok_or(StagedCandidateConversionProposalError::MissingProviderExecutionResult)?;
    let proposal = StagedCandidateConversionProposal {
        proposal_id: deterministic_staged_candidate_conversion_proposal_id(
            &execution_result_id,
            validation,
        ),
        source_provider_kind: validation.provider_kind.clone(),
        source_execution_result_id: execution_result_id,
        source_validation_status: validation.status,
        source_reviewability_status: validation.reviewability_status,
        source_candidate_boundary_status: validation.candidate_boundary_status,
        source_boundary: "provider_output_validation_phase_143".to_string(),
        proposal_boundary: "staged_candidate_conversion_phase_146".to_string(),
        boundary_statuses: staged_candidate_conversion_boundary_statuses(),
        trust_statuses: staged_candidate_conversion_trust_statuses(),
        effect_statuses: staged_candidate_conversion_no_effects(),
        source_eligibility_status: StagedCandidateConversionSourceEligibilityStatus::EligibleReviewableUntrusted,
        note: format!(
            "{} This proposal is not persistent, not executable, not approved, and not candidate material.",
            request.operator_note
        ),
    };
    let projection = project_staged_candidate_conversion_proposal(proposal);
    validate_staged_candidate_conversion_proposal(&projection)?;
    let mut next = state.clone();
    next.staged_candidate_conversion_proposal = projection;
    Ok(next)
}

pub fn local_provider_execution_result_absence_markers(
) -> LocalProviderExecutionResultAbsenceMarkers {
    LocalProviderExecutionResultAbsenceMarkers {
        no_process_spawned: true,
        no_network_socket_opened: true,
        no_filesystem_persistence: true,
        no_secrets_read: true,
        no_release_created: true,
        no_deployment_created: true,
        no_signing_performed: true,
        no_publishing_performed: true,
        no_public_use_approved: true,
        no_readiness_approved: true,
        no_replay_repair: true,
        no_recovery_promotion: true,
        no_action_execution: true,
        provider_output_not_candidate_material: true,
        marker_summary: vec![
            "no process".to_string(),
            "no network".to_string(),
            "no filesystem persistence".to_string(),
            "no secrets".to_string(),
            "no release/deployment/signing/publishing/public-use/readiness".to_string(),
            "no replay repair/recovery promotion/action execution".to_string(),
            "provider output is not candidate material".to_string(),
        ],
    }
}

fn initial_local_provider_execution_linkage() -> LocalProviderExecutionResultLinkage {
    LocalProviderExecutionResultLinkage {
        shell_state_label: "idle_local_harness".to_string(),
        run_id: "local-stub-run-133".to_string(),
        provider_configuration_kind: "none".to_string(),
        provider_configuration_status: "not_configured".to_string(),
        execution_result_id: "none".to_string(),
        candidate_id: "not_candidate_material".to_string(),
        source_boundary: "sandboxed_deterministic_provider_execution".to_string(),
    }
}

fn local_provider_execution_linkage(
    state: &LocalOperatorShellState,
    execution_result_id: &str,
) -> LocalProviderExecutionResultLinkage {
    LocalProviderExecutionResultLinkage {
        shell_state_label: state.harness_status.clone(),
        run_id: state.run.run_id.clone(),
        provider_configuration_kind: state
            .provider_configuration
            .configured_provider_kind
            .map(|kind| kind.code().to_string())
            .unwrap_or_else(|| "none".to_string()),
        provider_configuration_status: state.provider_configuration.status.code().to_string(),
        execution_result_id: execution_result_id.to_string(),
        candidate_id: state
            .run
            .candidate
            .as_ref()
            .map(|candidate| candidate.candidate_id.clone())
            .unwrap_or_else(|| "not_candidate_material".to_string()),
        source_boundary: "sandboxed_deterministic_provider_execution".to_string(),
    }
}

pub fn validate_local_provider_execution_result_projection(
    projection: &LocalProviderExecutionProjection,
) -> LocalProviderExecutionResultProjectionValidation {
    let mut errors = Vec::new();
    if projection.output_trust_status != LocalProviderOutputTrustStatus::UntrustedDescriptive {
        errors.push(
            LocalProviderExecutionResultProjectionError::InvalidTrustStatus
                .code()
                .to_string(),
        );
    }
    if projection.output_materialization_status
        == LocalProviderOutputMaterializationStatus::CandidateMaterial
    {
        errors.push(
            LocalProviderExecutionResultProjectionError::InvalidMaterializationStatus
                .code()
                .to_string(),
        );
    }
    if projection.output_promotion_status != LocalProviderOutputPromotionStatus::NotPromoted
        || projection.promotion_availability_status
            != LocalProviderOutputPromotionStatus::PromotionNotAvailableInPhase142
    {
        errors.push(
            LocalProviderExecutionResultProjectionError::InvalidPromotionStatus
                .code()
                .to_string(),
        );
    }
    let markers = &projection.absence_markers;
    if !markers.no_process_spawned
        || !markers.no_network_socket_opened
        || !markers.no_filesystem_persistence
        || !markers.no_secrets_read
        || !markers.no_release_created
        || !markers.no_deployment_created
        || !markers.no_signing_performed
        || !markers.no_publishing_performed
        || !markers.no_public_use_approved
        || !markers.no_readiness_approved
        || !markers.no_replay_repair
        || !markers.no_recovery_promotion
        || !markers.no_action_execution
        || !markers.provider_output_not_candidate_material
        || markers.marker_summary.is_empty()
    {
        errors.push(
            LocalProviderExecutionResultProjectionError::MissingAbsenceMarker
                .code()
                .to_string(),
        );
    }
    if projection.linkage.run_id.is_empty()
        || projection.linkage.provider_configuration_kind.is_empty()
        || projection.linkage.provider_configuration_status.is_empty()
        || projection.linkage.execution_result_id.is_empty()
        || projection.linkage.source_boundary != "sandboxed_deterministic_provider_execution"
    {
        errors.push(
            LocalProviderExecutionResultProjectionError::MissingLinkage
                .code()
                .to_string(),
        );
    }
    if errors.is_empty() {
        LocalProviderExecutionResultProjectionValidation {
            status: LocalProviderExecutionResultProjectionValidationStatus::Valid,
            error_codes: Vec::new(),
            reason: "provider execution result projection is valid; output remains untrusted_descriptive, not_candidate_material, and not_promoted".to_string(),
        }
    } else {
        LocalProviderExecutionResultProjectionValidation {
            status: LocalProviderExecutionResultProjectionValidationStatus::Invalid,
            error_codes: errors,
            reason: "provider execution result projection rejected fail-closed".to_string(),
        }
    }
}

fn with_provider_execution_projection_validation(
    mut projection: LocalProviderExecutionProjection,
) -> LocalProviderExecutionProjection {
    projection.projection_validation =
        validate_local_provider_execution_result_projection(&projection);
    projection
}

pub fn local_provider_execution_capability_surface() -> LocalProviderExecutionCapabilitySurface {
    LocalProviderExecutionCapabilitySurface {
        deterministic_stub_execution_supported: true,
        supported_provider_kind: "deterministic_stub".to_string(),
        cloud_calls_enabled: false,
        network_enabled: false,
        shell_commands_enabled: false,
        filesystem_enabled: false,
        secrets_allowed: false,
        trust_granted: false,
        readiness_approved: false,
        release_approved: false,
        deployment_enabled: false,
        signing_enabled: false,
        publishing_enabled: false,
        public_use_enabled: false,
        summary: "sandboxed deterministic provider execution supports deterministic_stub only; no cloud, network, shell, filesystem, secrets, trust, readiness, release, deployment, signing, publishing, or public-use capability".to_string(),
    }
}

pub fn initial_local_provider_execution_projection() -> LocalProviderExecutionProjection {
    with_provider_execution_projection_validation(LocalProviderExecutionProjection {
        status: LocalProviderExecutionStatus::NotExecuted,
        projection_status: LocalProviderExecutionResultProjectionStatus::NotExecuted,
        configured_provider_kind: "none".to_string(),
        sandbox_status: LocalProviderExecutionSandboxStatus::NotEntered,
        result: None,
        output_trust_status: LocalProviderOutputTrustStatus::UntrustedDescriptive,
        output_materialization_status: LocalProviderOutputMaterializationStatus::NotMaterialized,
        output_promotion_status: LocalProviderOutputPromotionStatus::NotPromoted,
        promotion_availability_status: LocalProviderOutputPromotionStatus::PromotionNotAvailableInPhase142,
        linkage: initial_local_provider_execution_linkage(),
        absence_markers: local_provider_execution_result_absence_markers(),
        projection_validation: LocalProviderExecutionResultProjectionValidation {
            status: LocalProviderExecutionResultProjectionValidationStatus::Invalid,
            error_codes: Vec::new(),
            reason: "projection validation pending".to_string(),
        },
        validation_status: "not_executed".to_string(),
        validation_error_codes: Vec::new(),
        validation_reason: "deterministic_stub execution has not been requested".to_string(),
        capability_surface: local_provider_execution_capability_surface(),
        note: "Provider execution result projection is projection_only evidence; provider output is untrusted_descriptive, not_candidate_material, not_promoted, and not eligible for approve/reject in Phase 142.".to_string(),
    })
}

pub fn project_local_provider_execution(
    state: &LocalOperatorShellState,
) -> LocalProviderExecutionProjection {
    let execution_result_id = state
        .provider_execution
        .result
        .as_ref()
        .map(|result| result.result_id.as_str())
        .unwrap_or("none");
    with_provider_execution_projection_validation(LocalProviderExecutionProjection {
        configured_provider_kind: state
            .provider_configuration
            .configured_provider_kind
            .map(|kind| kind.code().to_string())
            .unwrap_or_else(|| "none".to_string()),
        linkage: local_provider_execution_linkage(state, execution_result_id),
        ..state.provider_execution.clone()
    })
}

pub fn validate_local_provider_execution_request(
    configuration: &LocalProviderConfiguration,
    request: &LocalProviderExecutionRequest,
) -> LocalProviderExecutionValidation {
    let mut errors = std::collections::BTreeSet::new();
    if configuration.status != LocalProviderConfigurationStatus::Accepted
        || configuration.configured_provider_kind != Some(LocalProviderKind::DeterministicStub)
    {
        if configuration.status == LocalProviderConfigurationStatus::NotConfigured {
            errors.insert(LocalProviderExecutionError::MissingProviderConfiguration);
        } else {
            errors.insert(LocalProviderExecutionError::RejectedProviderConfiguration);
        }
    }

    let parsed_kind = match request.provider_kind.as_deref() {
        None => {
            errors.insert(LocalProviderExecutionError::MissingProviderKind);
            None
        }
        Some(kind) if kind.trim().is_empty() => {
            errors.insert(LocalProviderExecutionError::MissingProviderKind);
            None
        }
        Some(kind) if kind.trim() != kind => {
            errors.insert(LocalProviderExecutionError::MalformedProviderKind);
            None
        }
        Some(kind) => match LocalProviderKind::parse(kind) {
            Some(LocalProviderKind::DeterministicStub) => {
                Some(LocalProviderKind::DeterministicStub)
            }
            Some(other) => {
                errors.insert(LocalProviderExecutionError::UnsupportedProviderKind);
                Some(other)
            }
            None => {
                errors.insert(LocalProviderExecutionError::UnsupportedProviderKind);
                None
            }
        },
    };

    if request.input_summary.trim().is_empty() || request.input_summary.len() > 4096 {
        errors.insert(LocalProviderExecutionError::MalformedProviderKind);
    }

    for (key, value) in &request.fields {
        reject_forbidden_provider_execution_field(key, value, &mut errors);
    }

    if errors.is_empty() && parsed_kind == Some(LocalProviderKind::DeterministicStub) {
        LocalProviderExecutionValidation {
            status: LocalProviderExecutionStatus::Executed,
            provider_kind: parsed_kind,
            error_codes: Vec::new(),
            reason: "deterministic_stub execution accepted inside Rust-owned sandboxed deterministic boundary".to_string(),
        }
    } else {
        let status = if errors.contains(&LocalProviderExecutionError::UnsupportedProviderKind) {
            LocalProviderExecutionStatus::UnsupportedProvider
        } else if errors.contains(&LocalProviderExecutionError::MissingProviderConfiguration)
            || errors.contains(&LocalProviderExecutionError::RejectedProviderConfiguration)
        {
            LocalProviderExecutionStatus::ConfigurationRequired
        } else {
            LocalProviderExecutionStatus::InvalidRequest
        };
        LocalProviderExecutionValidation {
            status,
            provider_kind: parsed_kind,
            error_codes: errors.into_iter().collect(),
            reason: "provider execution rejected fail-closed; previous shell state is preserved"
                .to_string(),
        }
    }
}

fn reject_forbidden_provider_execution_field(
    key: &str,
    value: &str,
    errors: &mut std::collections::BTreeSet<LocalProviderExecutionError>,
) {
    let key = key.to_ascii_lowercase();
    let value = value.to_ascii_lowercase();
    let combined = format!("{key}={value}");
    if ["endpoint", "url", "host", "port", "http", "network"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderExecutionError::ForbiddenEndpointField);
    } else if ["command", "args", "shell", "process"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderExecutionError::ForbiddenCommandField);
    } else if ["path", "file", "directory"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderExecutionError::ForbiddenPathField);
    } else if ["secret", "token", "api_key", "apikey", "credential"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderExecutionError::ForbiddenSecretField);
    } else if key == "provider_execution_enabled" {
        errors.insert(LocalProviderExecutionError::ProviderExecutionFlagRejected);
    } else if key == "trust_granted" {
        errors.insert(LocalProviderExecutionError::TrustGrantRejected);
    } else if key == "readiness_approved" {
        errors.insert(LocalProviderExecutionError::ReadinessClaimRejected);
    } else if key == "release_candidate_approved" {
        errors.insert(LocalProviderExecutionError::ReleaseClaimRejected);
    } else if key == "deployment_enabled" {
        errors.insert(LocalProviderExecutionError::DeploymentClaimRejected);
    } else if key == "public_use_approved" {
        errors.insert(LocalProviderExecutionError::PublicUseClaimRejected);
    } else if key == "signing_enabled" {
        errors.insert(LocalProviderExecutionError::SigningClaimRejected);
    } else if key == "publishing_enabled" {
        errors.insert(LocalProviderExecutionError::PublishingClaimRejected);
    } else {
        errors.insert(LocalProviderExecutionError::UnknownFieldRejected);
    }
}

pub fn execute_sandboxed_deterministic_provider(
    request: &LocalProviderExecutionRequest,
) -> LocalProviderExecutionResult {
    let checksum = deterministic_provider_input_checksum(&request.input_summary);
    LocalProviderExecutionResult {
        result_id: format!("local-provider-execution-deterministic_stub-{checksum:08x}"),
        provider_kind: LocalProviderKind::DeterministicStub,
        sandbox_status:
            LocalProviderExecutionSandboxStatus::SandboxedDeterministicNoExternalEffects,
        output_summary: format!(
            "deterministic_stub descriptive output for input_bytes={} checksum={checksum:08x}",
            request.input_summary.len()
        ),
        output_trust_status: LocalProviderOutputTrustStatus::UntrustedDescriptive,
        output_materialization_status:
            LocalProviderOutputMaterializationStatus::ProjectedAsUntrustedOutput,
        output_promotion_status: LocalProviderOutputPromotionStatus::NotPromoted,
        promotion_availability_status:
            LocalProviderOutputPromotionStatus::PromotionNotAvailableInPhase142,
        descriptive_only: true,
        provider_output_trusted: false,
        candidate_output_promoted: false,
        decision_appended: false,
        replay_repaired: false,
        release_or_deployment_evidence_created: false,
    }
}

fn deterministic_provider_input_checksum(input: &str) -> u32 {
    input.bytes().fold(0x141u32, |accumulator, byte| {
        accumulator.wrapping_mul(33).wrapping_add(u32::from(byte))
    })
}

pub fn apply_local_provider_execution(
    state: &LocalOperatorShellState,
    request: LocalProviderExecutionRequest,
) -> Result<LocalOperatorShellState, LocalProviderExecutionValidation> {
    let validation =
        validate_local_provider_execution_request(&state.provider_configuration, &request);
    if validation.status != LocalProviderExecutionStatus::Executed {
        return Err(validation);
    }

    let result = execute_sandboxed_deterministic_provider(&request);
    let mut next = state.clone();
    let linkage = local_provider_execution_linkage(state, &result.result_id);
    next.provider_execution = with_provider_execution_projection_validation(LocalProviderExecutionProjection {
        status: LocalProviderExecutionStatus::Executed,
        projection_status: LocalProviderExecutionResultProjectionStatus::ExecutionProjected,
        configured_provider_kind: "deterministic_stub".to_string(),
        sandbox_status: LocalProviderExecutionSandboxStatus::SandboxedDeterministicNoExternalEffects,
        output_trust_status: result.output_trust_status,
        output_materialization_status: LocalProviderOutputMaterializationStatus::NotCandidateMaterial,
        output_promotion_status: result.output_promotion_status,
        promotion_availability_status: result.promotion_availability_status,
        linkage,
        absence_markers: local_provider_execution_result_absence_markers(),
        projection_validation: LocalProviderExecutionResultProjectionValidation {
            status: LocalProviderExecutionResultProjectionValidationStatus::Invalid,
            error_codes: Vec::new(),
            reason: "projection validation pending".to_string(),
        },
        result: Some(result),
        validation_status: validation.status.code().to_string(),
        validation_error_codes: Vec::new(),
        validation_reason: validation.reason,
        capability_surface: local_provider_execution_capability_surface(),
        note: "Provider execution result projection is projection_only evidence; provider output is untrusted_descriptive, not_candidate_material, not_promoted, promotion_not_available_in_phase_142, and not eligible for approve/reject in Phase 142.".to_string(),
    });
    next.provider_output_validation = validate_local_provider_output(&next.provider_execution);
    Ok(next)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCandidateOutput {
    pub candidate_id: String,
    pub title: String,
    pub body: String,
    pub provider_kind: String,
    pub provider_output_trusted: bool,
    pub provider_execution_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalValidationProjection {
    pub validation_id: String,
    pub policy_status: String,
    pub validation_status: String,
    pub reason: String,
    pub authority: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalRunProjection {
    pub run_id: String,
    pub status: LocalRunStatus,
    pub bounded_context: Vec<String>,
    pub candidate: Option<LocalCandidateOutput>,
    pub validation: Option<LocalValidationProjection>,
    pub selected_intent: Option<LocalOperatorIntentKind>,
    pub timeline: Vec<String>,
    pub replay_status: String,
    pub decision_timeline: LocalDecisionTimelineProjection,
    pub decision_replay: LocalDecisionReplayProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorShellState {
    pub harness_status: String,
    pub non_production: bool,
    pub run: LocalRunProjection,
    pub decision_ledger: LocalDecisionLedger,
    pub local_session_evidence_export: LocalSessionEvidenceExport,
    pub provider_configuration: LocalProviderConfiguration,
    pub provider_execution: LocalProviderExecutionProjection,
    pub provider_output_validation: LocalProviderOutputValidationProjection,
    pub staged_candidate_conversion_proposal: StagedCandidateConversionProposalProjection,
    pub staged_candidate_conversion_validation: StagedCandidateConversionValidationProjection,
}

pub fn derive_local_session_evidence_export(
    harness_status: &str,
    non_production: bool,
    run: &LocalRunProjection,
    ledger: &LocalDecisionLedger,
) -> LocalSessionEvidenceExport {
    let replay = derive_local_decision_replay_projection(run, ledger);
    let export_status = match (run.status, ledger.records.is_empty()) {
        (LocalRunStatus::Idle, _) => LocalSessionEvidenceExportStatus::NoCompletedRunEvidence,
        (_, true) => LocalSessionEvidenceExportStatus::RunEvidenceProjected,
        (_, false) => LocalSessionEvidenceExportStatus::DecisionEvidenceProjected,
    };
    let candidate_id = run
        .candidate
        .as_ref()
        .map(|candidate| candidate.candidate_id.clone())
        .unwrap_or_else(|| "not_applicable_until_stub_run".to_string());
    let candidate_output_summary = run
        .candidate
        .as_ref()
        .map(|candidate| format!("{}: {}", candidate.title, candidate.body))
        .unwrap_or_else(|| "no completed deterministic stub run candidate evidence".to_string());
    let validation_status = run
        .validation
        .as_ref()
        .map(|validation| validation.validation_status.clone())
        .unwrap_or_else(|| "not_applicable_until_stub_run".to_string());
    let policy_status = run
        .validation
        .as_ref()
        .map(|validation| validation.policy_status.clone())
        .unwrap_or_else(|| "not_applicable_until_stub_run".to_string());
    let mut export = LocalSessionEvidenceExport {
        export_id: format!(
            "local-session-evidence-export-{}-decisions-{:04}",
            run.run_id,
            ledger.records.len()
        ),
        export_status,
        export_classification: "local_session_evidence_only".to_string(),
        production_classification: "non-production".to_string(),
        shell_status: harness_status.to_string(),
        run_id: run.run_id.clone(),
        run_status: run.status,
        bounded_context_summary: run.bounded_context.clone(),
        candidate_id,
        candidate_output_summary,
        validation_status,
        policy_status,
        decision_count: ledger.records.len(),
        decision_records: ledger.records.clone(),
        replay_status: replay.replay_status,
        replay_integrity_status: replay.integrity_status,
        absence_markers: local_session_evidence_export_absence_markers(),
        export_validation_status: LocalSessionEvidenceExportValidationStatus::Incomplete,
        summary: format!(
            "Local session evidence export preview for run {} is {}; local only, non-production, and non-mutating.",
            run.run_id,
            export_status.code()
        ),
    };
    export.export_validation_status =
        match validate_local_session_evidence_export(&export, non_production) {
            Ok(()) => LocalSessionEvidenceExportValidationStatus::Complete,
            Err(_) => LocalSessionEvidenceExportValidationStatus::Incomplete,
        };
    export
}

pub fn project_local_session_evidence_export(
    state: &LocalOperatorShellState,
) -> LocalSessionEvidenceExport {
    derive_local_session_evidence_export(
        &state.harness_status,
        state.non_production,
        &state.run,
        &state.decision_ledger,
    )
}

pub fn validate_local_session_evidence_export(
    export: &LocalSessionEvidenceExport,
    non_production: bool,
) -> Result<(), LocalSessionEvidenceExportError> {
    if export.export_id.is_empty() || export.shell_status.is_empty() || export.run_id.is_empty() {
        return Err(LocalSessionEvidenceExportError::MissingRequiredField);
    }
    if export.export_classification != "local_session_evidence_only" {
        return Err(LocalSessionEvidenceExportError::InvalidExportClassification);
    }
    if !non_production || export.production_classification != "non-production" {
        return Err(LocalSessionEvidenceExportError::InvalidProductionClassification);
    }
    let markers = &export.absence_markers;
    if !markers.provider_execution_absent
        || !markers.persistence_absent
        || !markers.release_absent
        || !markers.deployment_absent
        || !markers.signing_absent
        || !markers.publishing_absent
        || !markers.installer_absent
        || !markers.update_channel_absent
        || !markers.public_use_absent
        || !markers.readiness_approval_absent
    {
        return Err(LocalSessionEvidenceExportError::MissingAbsenceMarker);
    }
    if matches!(
        export.export_status,
        LocalSessionEvidenceExportStatus::RunEvidenceProjected
            | LocalSessionEvidenceExportStatus::DecisionEvidenceProjected
    ) && (export.bounded_context_summary.is_empty()
        || export.candidate_id == "not_applicable_until_stub_run"
        || export.validation_status == "not_applicable_until_stub_run"
        || export.policy_status == "not_applicable_until_stub_run")
    {
        return Err(LocalSessionEvidenceExportError::MissingRunEvidence);
    }
    if export.export_status == LocalSessionEvidenceExportStatus::DecisionEvidenceProjected
        && (export.decision_count == 0
            || export.decision_records.is_empty()
            || export.replay_status == LocalDecisionReplayStatus::NoDecisionRecorded)
    {
        return Err(LocalSessionEvidenceExportError::MissingDecisionReplayEvidence);
    }
    Ok(())
}

pub fn initial_local_session_evidence_export() -> LocalSessionEvidenceExport {
    let ledger = initial_local_decision_ledger();
    let replay = initial_local_decision_replay_projection();
    let run = LocalRunProjection {
        run_id: "local-stub-run-133".to_string(),
        status: LocalRunStatus::Idle,
        bounded_context: Vec::new(),
        candidate: None,
        validation: None,
        selected_intent: None,
        timeline: vec!["idle local harness initialized".to_string()],
        replay_status: replay.replay_status.code().to_string(),
        decision_timeline: project_local_decision_timeline(&ledger),
        decision_replay: replay,
    };
    derive_local_session_evidence_export("idle_local_harness", true, &run, &ledger)
}

fn attach_local_session_evidence_export(
    mut state: LocalOperatorShellState,
) -> LocalOperatorShellState {
    state.local_session_evidence_export = project_local_session_evidence_export(&state);
    state
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorIntent {
    pub kind: LocalOperatorIntentKind,
    pub operator_id: String,
    pub target_run_id: String,
    pub target_candidate_id: String,
    pub reason: String,
    pub requests_authority_grant: bool,
    pub requests_provider_execution: bool,
    pub claims_readiness: bool,
}

impl LocalOperatorIntent {
    pub fn new(
        kind: LocalOperatorIntentKind,
        operator_id: impl Into<String>,
        target_run_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            operator_id: operator_id.into(),
            target_run_id: target_run_id.into(),
            target_candidate_id: "candidate-local-stub-133".to_string(),
            reason: reason.into(),
            requests_authority_grant: false,
            requests_provider_execution: false,
            claims_readiness: false,
        }
    }
}

pub fn initial_local_operator_shell_state() -> LocalOperatorShellState {
    let ledger = initial_local_decision_ledger();
    let replay = initial_local_decision_replay_projection();
    let run = LocalRunProjection {
        run_id: "local-stub-run-133".to_string(),
        status: LocalRunStatus::Idle,
        bounded_context: Vec::new(),
        candidate: None,
        validation: None,
        selected_intent: None,
        timeline: vec!["idle local harness initialized".to_string()],
        replay_status: replay.replay_status.code().to_string(),
        decision_timeline: project_local_decision_timeline(&ledger),
        decision_replay: replay,
    };
    let export = derive_local_session_evidence_export("idle_local_harness", true, &run, &ledger);
    LocalOperatorShellState {
        harness_status: "idle_local_harness".to_string(),
        non_production: true,
        run,
        decision_ledger: ledger,
        local_session_evidence_export: export,
        provider_configuration: initial_local_provider_configuration(),
        provider_execution: initial_local_provider_execution_projection(),
        provider_output_validation: initial_local_provider_output_validation_projection(),
        staged_candidate_conversion_proposal:
            initial_staged_candidate_conversion_proposal_projection(),
        staged_candidate_conversion_validation:
            initial_staged_candidate_conversion_validation_projection(),
    }
}

pub fn start_deterministic_stub_run(state: &LocalOperatorShellState) -> LocalOperatorShellState {
    let mut next = state.clone();
    next.harness_status = "deterministic_stub_completed".to_string();
    next.run.status = LocalRunStatus::StubCompleted;
    next.run.bounded_context = vec![
        "phase=133".to_string(),
        "scope=local operator UI shell".to_string(),
        "provider=deterministic stub only".to_string(),
        "network=disabled".to_string(),
    ];
    next.run.candidate = Some(LocalCandidateOutput {
        candidate_id: "candidate-local-stub-133".to_string(),
        title: "Deterministic local stub candidate".to_string(),
        body: "AJENTIC local shell rendered a deterministic candidate from a Rust-owned projection fixture.".to_string(),
        provider_kind: "deterministic_stub".to_string(),
        provider_output_trusted: false,
        provider_execution_enabled: false,
    });
    next.run.validation = Some(LocalValidationProjection {
        validation_id: "validation-local-stub-133".to_string(),
        policy_status: "pass_for_local_stub_review".to_string(),
        validation_status: "pass_for_local_stub_review".to_string(),
        reason: "deterministic fixture satisfies local shell display checks only".to_string(),
        authority: "rust".to_string(),
    });
    next.run.timeline = vec![
        "idle local harness initialized".to_string(),
        "deterministic stub run started".to_string(),
        "candidate output projected".to_string(),
        "validation and policy projection completed".to_string(),
    ];
    next.run.decision_timeline = project_local_decision_timeline(&next.decision_ledger);
    next.run.decision_replay =
        derive_local_decision_replay_projection(&next.run, &next.decision_ledger);
    next.run.replay_status = next.run.decision_replay.replay_status.code().to_string();
    attach_local_session_evidence_export(next)
}

pub fn apply_local_operator_intent(
    state: &LocalOperatorShellState,
    intent: LocalOperatorIntent,
) -> Result<LocalOperatorShellState, LocalOperatorShellError> {
    if state.run.status == LocalRunStatus::Idle {
        return Err(LocalOperatorShellError::RunNotStarted);
    }
    if intent.operator_id.is_empty() {
        return Err(LocalOperatorShellError::EmptyOperatorId);
    }
    if intent.reason.is_empty() {
        return Err(LocalOperatorShellError::EmptyReason);
    }
    if intent.target_run_id != state.run.run_id {
        return Err(LocalOperatorShellError::TargetMismatch);
    }
    let Some(candidate) = state.run.candidate.as_ref() else {
        return Err(LocalOperatorShellError::RunNotStarted);
    };
    if intent.target_candidate_id != candidate.candidate_id {
        return Err(LocalOperatorShellError::CandidateTargetMismatch);
    }
    if intent.requests_authority_grant {
        return Err(LocalOperatorShellError::AuthorityClaimRejected);
    }
    if intent.requests_provider_execution {
        return Err(LocalOperatorShellError::ProviderExecutionRejected);
    }
    if intent.claims_readiness {
        return Err(LocalOperatorShellError::ReadinessClaimRejected);
    }

    let next_ledger =
        append_local_decision(&state.decision_ledger, &intent, &candidate.candidate_id)?;

    let mut next = state.clone();
    next.decision_ledger = next_ledger;
    next.run.status = LocalRunStatus::IntentRecorded;
    next.run.selected_intent = Some(intent.kind);
    next.run.decision_timeline = project_local_decision_timeline(&next.decision_ledger);
    next.run.decision_replay =
        derive_local_decision_replay_projection(&next.run, &next.decision_ledger);
    next.run.replay_status = next.run.decision_replay.replay_status.code().to_string();
    next.run.timeline.push(format!(
        "operator intent recorded: {} by {} as decision {}",
        intent.kind.code(),
        intent.operator_id,
        next.run
            .decision_timeline
            .records
            .last()
            .map(|record| record.decision_id.as_str())
            .unwrap_or("local-decision-missing")
    ));
    Ok(attach_local_session_evidence_export(next))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalOperatorShellTransportStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalOperatorShellForbiddenRequest {
    AuthorityGrant,
    ProviderExecution,
    ReadinessClaim,
    ProductionStateMutation,
    ReleaseArtifactCreation,
    DeploymentEnablement,
    SigningEnablement,
    PublishingEnablement,
}

impl LocalOperatorShellForbiddenRequest {
    pub fn rejection_code(&self) -> &'static str {
        match self {
            Self::AuthorityGrant => "authority_grant_rejected",
            Self::ProviderExecution => "provider_execution_rejected",
            Self::ReadinessClaim => "readiness_claim_rejected",
            Self::ProductionStateMutation => "production_state_mutation_rejected",
            Self::ReleaseArtifactCreation => "release_artifact_creation_rejected",
            Self::DeploymentEnablement => "deployment_enablement_rejected",
            Self::SigningEnablement => "signing_enablement_rejected",
            Self::PublishingEnablement => "publishing_enablement_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalOperatorShellRequest {
    GetInitialState,
    GetCurrentState,
    StartDeterministicStubRun,
    SubmitOperatorIntent(LocalOperatorIntent),
    SubmitProviderConfiguration(LocalProviderConfigurationCandidate),
    ExecuteProvider(LocalProviderExecutionRequest),
    CreateStagedCandidateConversionProposal(StagedCandidateConversionProposalRequest),
    ValidateStagedCandidateConversionProposal(StagedCandidateConversionValidationRequest),
    Forbidden(LocalOperatorShellForbiddenRequest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorShellCapabilities {
    pub local_only: bool,
    pub non_production: bool,
    pub provider_execution_enabled: bool,
    pub cloud_model_calls_enabled: bool,
    pub broad_command_execution_enabled: bool,
    pub production_persistence_enabled: bool,
    pub release_artifact_creation_enabled: bool,
    pub deployment_enabled: bool,
    pub signing_authority_available: bool,
    pub publishing_authority_available: bool,
    pub readiness_approval_enabled: bool,
}

impl LocalOperatorShellCapabilities {
    pub fn local_stub_only() -> Self {
        Self {
            local_only: true,
            non_production: true,
            provider_execution_enabled: false,
            cloud_model_calls_enabled: false,
            broad_command_execution_enabled: false,
            production_persistence_enabled: false,
            release_artifact_creation_enabled: false,
            deployment_enabled: false,
            signing_authority_available: false,
            publishing_authority_available: false,
            readiness_approval_enabled: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorShellResponse {
    pub status: LocalOperatorShellTransportStatus,
    pub reason: String,
    pub state: LocalOperatorShellState,
    pub local_session_evidence_export: LocalSessionEvidenceExport,
    pub capabilities: LocalOperatorShellCapabilities,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorShellTransport {
    state: LocalOperatorShellState,
}

impl LocalOperatorShellTransport {
    pub fn new() -> Self {
        Self {
            state: initial_local_operator_shell_state(),
        }
    }

    pub fn current_state(&self) -> LocalOperatorShellState {
        self.state.clone()
    }

    pub fn step(&mut self, request: LocalOperatorShellRequest) -> LocalOperatorShellResponse {
        let response = local_operator_shell_transport_step(&self.state, request);
        if response.status == LocalOperatorShellTransportStatus::Accepted {
            self.state = response.state.clone();
        }
        response
    }
}

impl Default for LocalOperatorShellTransport {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_local_operator_shell_state(
    transport: &LocalOperatorShellTransport,
) -> LocalOperatorShellState {
    transport.current_state()
}

pub fn start_local_operator_shell_stub_run(
    transport: &mut LocalOperatorShellTransport,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::StartDeterministicStubRun)
}

pub fn submit_local_operator_shell_intent(
    transport: &mut LocalOperatorShellTransport,
    intent: LocalOperatorIntent,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::SubmitOperatorIntent(intent))
}

pub fn submit_local_provider_configuration(
    transport: &mut LocalOperatorShellTransport,
    candidate: LocalProviderConfigurationCandidate,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::SubmitProviderConfiguration(
        candidate,
    ))
}

pub fn execute_local_provider(
    transport: &mut LocalOperatorShellTransport,
    request: LocalProviderExecutionRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::ExecuteProvider(request))
}

pub fn create_local_staged_candidate_conversion_proposal(
    transport: &mut LocalOperatorShellTransport,
    request: StagedCandidateConversionProposalRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::CreateStagedCandidateConversionProposal(request))
}

pub fn validate_local_staged_candidate_conversion_proposal(
    transport: &mut LocalOperatorShellTransport,
    request: StagedCandidateConversionValidationRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::ValidateStagedCandidateConversionProposal(request))
}

pub fn local_operator_shell_transport_step(
    state: &LocalOperatorShellState,
    request: LocalOperatorShellRequest,
) -> LocalOperatorShellResponse {
    match request {
        LocalOperatorShellRequest::GetInitialState => accepted(
            "initial_shell_state_returned",
            initial_local_operator_shell_state(),
        ),
        LocalOperatorShellRequest::GetCurrentState => {
            accepted("current_shell_state_returned", state.clone())
        }
        LocalOperatorShellRequest::StartDeterministicStubRun => accepted(
            "deterministic_stub_run_completed",
            start_deterministic_stub_run(state),
        ),
        LocalOperatorShellRequest::SubmitOperatorIntent(intent) => {
            match apply_local_operator_intent(state, intent) {
                Ok(next) => accepted("local_operator_intent_recorded", next),
                Err(err) => rejected(err.code(), state.clone()),
            }
        }
        LocalOperatorShellRequest::SubmitProviderConfiguration(candidate) => {
            match apply_local_provider_configuration_candidate(state, candidate) {
                Ok(next) => accepted("local_provider_configuration_accepted", next),
                Err(validation) => rejected(validation.reason, state.clone()),
            }
        }
        LocalOperatorShellRequest::ExecuteProvider(request) => {
            match apply_local_provider_execution(state, request) {
                Ok(next) => accepted("local_provider_execution_accepted", next),
                Err(validation) => rejected(validation.reason, state.clone()),
            }
        }
        LocalOperatorShellRequest::CreateStagedCandidateConversionProposal(request) => {
            match create_staged_candidate_conversion_proposal(state, request) {
                Ok(next) => accepted("staged_candidate_conversion_proposal_created", next),
                Err(error) => rejected(error.code(), state.clone()),
            }
        }
        LocalOperatorShellRequest::ValidateStagedCandidateConversionProposal(request) => {
            let next = validate_staged_candidate_conversion_proposal_for_phase_147(state, request);
            if next.staged_candidate_conversion_validation.status
                == StagedCandidateConversionValidationStatus::StagedProposalShapeValid
            {
                accepted("staged_candidate_conversion_validation_completed", next)
            } else {
                rejected("staged_candidate_conversion_validation_rejected", next)
            }
        }
        LocalOperatorShellRequest::Forbidden(forbidden) => {
            rejected(forbidden.rejection_code(), state.clone())
        }
    }
}

fn accepted(
    reason: impl Into<String>,
    state: LocalOperatorShellState,
) -> LocalOperatorShellResponse {
    LocalOperatorShellResponse {
        status: LocalOperatorShellTransportStatus::Accepted,
        reason: reason.into(),
        local_session_evidence_export: state.local_session_evidence_export.clone(),
        state,
        capabilities: LocalOperatorShellCapabilities::local_stub_only(),
    }
}

fn rejected(
    reason: impl Into<String>,
    state: LocalOperatorShellState,
) -> LocalOperatorShellResponse {
    LocalOperatorShellResponse {
        status: LocalOperatorShellTransportStatus::Rejected,
        reason: reason.into(),
        local_session_evidence_export: state.local_session_evidence_export.clone(),
        state,
        capabilities: LocalOperatorShellCapabilities::local_stub_only(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let second =
            derive_local_decision_replay_projection(&updated.run, &updated.decision_ledger);

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
            apply_local_operator_intent(&state, authority_intent)
                .expect_err("authority grant fails"),
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
            apply_local_operator_intent(&state, provider_intent)
                .expect_err("provider execution fails"),
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
            apply_local_operator_intent(&state, readiness_intent)
                .expect_err("readiness claim fails"),
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
        assert!(projection.candidate_boundary_statuses.contains(
            &LocalProviderOutputCandidateBoundaryStatus::CandidateConversionNotPerformed
        ));
        assert!(projection.candidate_boundary_statuses.contains(
            &LocalProviderOutputCandidateBoundaryStatus::CandidateConversionRequiresFuturePhase
        ));
        assert!(projection
            .reasons
            .contains(&LocalProviderOutputValidationReason::DeterministicStubOutputShapeValid));
        assert!(projection.reasons.contains(
            &LocalProviderOutputValidationReason::CandidateConversionNotAvailableInPhase143
        ));
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
                1 => {
                    projection.candidate_effect =
                        LocalProviderOutputValidationEffect::EffectDetected
                }
                2 => {
                    projection.decision_ledger_effect =
                        LocalProviderOutputValidationEffect::EffectDetected
                }
                3 => projection.replay_effect = LocalProviderOutputValidationEffect::EffectDetected,
                4 => projection.export_effect = LocalProviderOutputValidationEffect::EffectDetected,
                5 => projection.action_effect = LocalProviderOutputValidationEffect::EffectDetected,
                6 => {
                    projection.readiness_effect =
                        LocalProviderOutputValidationEffect::EffectDetected
                }
                7 => {
                    projection.release_effect = LocalProviderOutputValidationEffect::EffectDetected
                }
                _ => {
                    projection.deployment_effect =
                        LocalProviderOutputValidationEffect::EffectDetected
                }
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
            .contains(&StagedCandidateConversionMaterializationStatus::MaterializationNotAvailableInPhase147));
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
        assert!(projection.reasons.contains(
            &StagedCandidateConversionValidationReason::SourceReviewabilityStatusMismatch
        ));

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
        .contains(
            &StagedCandidateConversionValidationReason::ProviderOutputValidationInconsistent
        ));

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
}
