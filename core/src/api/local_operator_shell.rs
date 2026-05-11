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
    pub output_trust_status: String,
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
    pub configured_provider_kind: String,
    pub sandbox_status: LocalProviderExecutionSandboxStatus,
    pub result: Option<LocalProviderExecutionResult>,
    pub validation_status: String,
    pub validation_error_codes: Vec<String>,
    pub validation_reason: String,
    pub capability_surface: LocalProviderExecutionCapabilitySurface,
    pub note: String,
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
    LocalProviderExecutionProjection {
        status: LocalProviderExecutionStatus::NotExecuted,
        configured_provider_kind: "none".to_string(),
        sandbox_status: LocalProviderExecutionSandboxStatus::NotEntered,
        result: None,
        validation_status: "not_executed".to_string(),
        validation_error_codes: Vec::new(),
        validation_reason: "deterministic_stub execution has not been requested".to_string(),
        capability_surface: local_provider_execution_capability_surface(),
        note: "Provider execution output is untrusted/descriptive and is not candidate output, trust approval, readiness evidence, release evidence, deployment evidence, or a decision.".to_string(),
    }
}

pub fn project_local_provider_execution(
    state: &LocalOperatorShellState,
) -> LocalProviderExecutionProjection {
    LocalProviderExecutionProjection {
        configured_provider_kind: state
            .provider_configuration
            .configured_provider_kind
            .map(|kind| kind.code().to_string())
            .unwrap_or_else(|| "none".to_string()),
        ..state.provider_execution.clone()
    }
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
        output_trust_status: "untrusted/descriptive".to_string(),
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
    next.provider_execution = LocalProviderExecutionProjection {
        status: LocalProviderExecutionStatus::Executed,
        configured_provider_kind: "deterministic_stub".to_string(),
        sandbox_status: LocalProviderExecutionSandboxStatus::SandboxedDeterministicNoExternalEffects,
        result: Some(result),
        validation_status: validation.status.code().to_string(),
        validation_error_codes: Vec::new(),
        validation_reason: validation.reason,
        capability_surface: local_provider_execution_capability_surface(),
        note: "Provider execution output is untrusted/descriptive and is not candidate output, trust approval, readiness evidence, release evidence, deployment evidence, or a decision.".to_string(),
    };
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
        assert_eq!(result.output_trust_status, "untrusted/descriptive");
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
}
