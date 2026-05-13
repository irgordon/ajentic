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
pub enum LocalProviderAdapterKind {
    DeterministicFakeAdapter,
    LocalModelAdapterContract,
    UnsupportedLocalModel,
    UnsupportedCloudModel,
    UnsupportedNetworkAdapter,
    UnsupportedShellAdapter,
    UnsupportedFilesystemAdapter,
    Unknown,
}

impl LocalProviderAdapterKind {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "deterministic_fake_adapter" => Some(Self::DeterministicFakeAdapter),
            "local_model_adapter_contract" => Some(Self::LocalModelAdapterContract),
            "unsupported_local_model" => Some(Self::UnsupportedLocalModel),
            "unsupported_cloud_model" => Some(Self::UnsupportedCloudModel),
            "unsupported_network_adapter" => Some(Self::UnsupportedNetworkAdapter),
            "unsupported_shell_adapter" => Some(Self::UnsupportedShellAdapter),
            "unsupported_filesystem_adapter" => Some(Self::UnsupportedFilesystemAdapter),
            "unknown" => Some(Self::Unknown),
            _ => None,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::DeterministicFakeAdapter => "deterministic_fake_adapter",
            Self::LocalModelAdapterContract => "local_model_adapter_contract",
            Self::UnsupportedLocalModel => "unsupported_local_model",
            Self::UnsupportedCloudModel => "unsupported_cloud_model",
            Self::UnsupportedNetworkAdapter => "unsupported_network_adapter",
            Self::UnsupportedShellAdapter => "unsupported_shell_adapter",
            Self::UnsupportedFilesystemAdapter => "unsupported_filesystem_adapter",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterValidationStatus {
    RegistryProjected,
    AdapterDeclaredNonExecuting,
    AdapterRejected,
    UnsupportedAdapter,
    InvalidAdapterDeclaration,
}

impl LocalProviderAdapterValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::RegistryProjected => "registry_projected",
            Self::AdapterDeclaredNonExecuting => "adapter_declared_non_executing",
            Self::AdapterRejected => "adapter_rejected",
            Self::UnsupportedAdapter => "unsupported_adapter",
            Self::InvalidAdapterDeclaration => "invalid_adapter_declaration",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalProviderAdapterValidationError {
    MissingAdapterKind,
    MalformedAdapterKind,
    UnsupportedAdapter,
    CloudOrNetworkAdapterRejected,
    ShellAdapterRejected,
    FilesystemAdapterRejected,
    ExecutablePathRejected,
    EndpointFieldRejected,
    CommandFieldRejected,
    PathFieldRejected,
    SecretFieldRejected,
    ExecutionFlagRejected,
    ProviderTrustFlagRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    SigningClaimRejected,
    PublishingClaimRejected,
    UnknownFieldRejected,
}

impl LocalProviderAdapterValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingAdapterKind => "missing_adapter_kind",
            Self::MalformedAdapterKind => "malformed_adapter_kind",
            Self::UnsupportedAdapter => "unsupported_adapter",
            Self::CloudOrNetworkAdapterRejected => "cloud_or_network_adapter_rejected",
            Self::ShellAdapterRejected => "shell_adapter_rejected",
            Self::FilesystemAdapterRejected => "filesystem_adapter_rejected",
            Self::ExecutablePathRejected => "executable_path_rejected",
            Self::EndpointFieldRejected => "endpoint_field_rejected",
            Self::CommandFieldRejected => "command_field_rejected",
            Self::PathFieldRejected => "path_field_rejected",
            Self::SecretFieldRejected => "secret_field_rejected",
            Self::ExecutionFlagRejected => "execution_flag_rejected",
            Self::ProviderTrustFlagRejected => "provider_trust_flag_rejected",
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
pub enum LocalProviderAdapterExecutionStatus {
    ExecutionNotAvailableInPhase153,
}

impl LocalProviderAdapterExecutionStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ExecutionNotAvailableInPhase153 => "execution_not_available_in_phase_153",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterTrustStatus {
    NoProviderTrust,
}

impl LocalProviderAdapterTrustStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoProviderTrust => "no_provider_trust",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterBoundaryStatus {
    ContractOnly,
    NoExecution,
    NoProviderTrust,
    NoNetwork,
    NoShell,
    NoSecrets,
    NoProductionPersistence,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl LocalProviderAdapterBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ContractOnly => "contract_only",
            Self::NoExecution => "no_execution",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoNetwork => "no_network",
            Self::NoShell => "no_shell",
            Self::NoSecrets => "no_secrets",
            Self::NoProductionPersistence => "no_production_persistence",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NoPublicUseEffect => "no_public_use_effect",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterCapabilitySurface {
    pub contract_only: bool,
    pub no_execution: bool,
    pub no_provider_trust: bool,
    pub no_network: bool,
    pub no_shell: bool,
    pub no_secrets: bool,
    pub no_production_persistence: bool,
    pub no_readiness_effect: bool,
    pub no_release_effect: bool,
    pub no_deployment_effect: bool,
    pub no_public_use_effect: bool,
    pub summary: String,
}

pub fn local_provider_adapter_capability_surface() -> LocalProviderAdapterCapabilitySurface {
    LocalProviderAdapterCapabilitySurface {
        contract_only: true,
        no_execution: true,
        no_provider_trust: true,
        no_network: true,
        no_shell: true,
        no_secrets: true,
        no_production_persistence: true,
        no_readiness_effect: true,
        no_release_effect: true,
        no_deployment_effect: true,
        no_public_use_effect: true,
        summary: "Adapter contract only; no model execution is available in Phase 153. No network, shell, secret, or production persistence capability is enabled.".to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterContract {
    pub adapter_kind: LocalProviderAdapterKind,
    pub capability_surface: LocalProviderAdapterCapabilitySurface,
    pub execution_status: LocalProviderAdapterExecutionStatus,
    pub trust_status: LocalProviderAdapterTrustStatus,
    pub boundary_statuses: Vec<LocalProviderAdapterBoundaryStatus>,
}

pub fn local_provider_adapter_contract(
    adapter_kind: LocalProviderAdapterKind,
) -> LocalProviderAdapterContract {
    LocalProviderAdapterContract {
        adapter_kind,
        capability_surface: local_provider_adapter_capability_surface(),
        execution_status: LocalProviderAdapterExecutionStatus::ExecutionNotAvailableInPhase153,
        trust_status: LocalProviderAdapterTrustStatus::NoProviderTrust,
        boundary_statuses: vec![
            LocalProviderAdapterBoundaryStatus::ContractOnly,
            LocalProviderAdapterBoundaryStatus::NoExecution,
            LocalProviderAdapterBoundaryStatus::NoProviderTrust,
            LocalProviderAdapterBoundaryStatus::NoNetwork,
            LocalProviderAdapterBoundaryStatus::NoShell,
            LocalProviderAdapterBoundaryStatus::NoSecrets,
            LocalProviderAdapterBoundaryStatus::NoProductionPersistence,
            LocalProviderAdapterBoundaryStatus::NoReadinessEffect,
            LocalProviderAdapterBoundaryStatus::NoReleaseEffect,
            LocalProviderAdapterBoundaryStatus::NoDeploymentEffect,
            LocalProviderAdapterBoundaryStatus::NoPublicUseEffect,
        ],
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterDeclaration {
    pub adapter_kind: LocalProviderAdapterKind,
    pub declaration_id: String,
    pub status: LocalProviderAdapterValidationStatus,
    pub contract: LocalProviderAdapterContract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterConfigurationCandidate {
    pub adapter_kind: Option<String>,
    pub declaration_id: Option<String>,
    pub fields: Vec<(String, String)>,
}

impl LocalProviderAdapterConfigurationCandidate {
    pub fn deterministic_fake_adapter() -> Self {
        Self {
            adapter_kind: Some("deterministic_fake_adapter".to_string()),
            declaration_id: Some("local-adapter-declaration-deterministic-fake".to_string()),
            fields: Vec::new(),
        }
    }

    pub fn local_model_adapter_contract() -> Self {
        Self {
            adapter_kind: Some("local_model_adapter_contract".to_string()),
            declaration_id: Some("local-adapter-declaration-local-model-contract".to_string()),
            fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterValidation {
    pub status: LocalProviderAdapterValidationStatus,
    pub adapter_kind: Option<LocalProviderAdapterKind>,
    pub declaration_id: Option<String>,
    pub error_codes: Vec<LocalProviderAdapterValidationError>,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterRegistryProjection {
    pub registry_status: LocalProviderAdapterValidationStatus,
    pub supported_adapter_kinds: Vec<String>,
    pub rejected_adapter_kinds: Vec<String>,
    pub declarations: Vec<LocalProviderAdapterDeclaration>,
    pub last_validation: LocalProviderAdapterValidation,
    pub capability_surface: LocalProviderAdapterCapabilitySurface,
    pub execution_status: String,
    pub trust_status: String,
    pub boundary_statuses: Vec<String>,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterRegistry {
    pub declarations: Vec<LocalProviderAdapterDeclaration>,
    pub last_validation: LocalProviderAdapterValidation,
}

pub fn initial_local_provider_adapter_registry() -> LocalProviderAdapterRegistry {
    LocalProviderAdapterRegistry {
        declarations: Vec::new(),
        last_validation: LocalProviderAdapterValidation {
            status: LocalProviderAdapterValidationStatus::RegistryProjected,
            adapter_kind: None,
            declaration_id: None,
            error_codes: Vec::new(),
            reason: "initial local provider adapter registry projected; no adapter declarations execute in Phase 153".to_string(),
        },
    }
}

pub fn project_local_provider_adapter_registry(
    registry: &LocalProviderAdapterRegistry,
) -> LocalProviderAdapterRegistryProjection {
    let boundary_statuses =
        local_provider_adapter_contract(LocalProviderAdapterKind::DeterministicFakeAdapter)
            .boundary_statuses
            .iter()
            .map(|status| status.code().to_string())
            .collect();
    LocalProviderAdapterRegistryProjection {
        registry_status: LocalProviderAdapterValidationStatus::RegistryProjected,
        supported_adapter_kinds: vec![
            "deterministic_fake_adapter".to_string(),
            "local_model_adapter_contract".to_string(),
        ],
        rejected_adapter_kinds: vec![
            "unsupported_local_model".to_string(),
            "unsupported_cloud_model".to_string(),
            "unsupported_network_adapter".to_string(),
            "unsupported_shell_adapter".to_string(),
            "unsupported_filesystem_adapter".to_string(),
            "unknown".to_string(),
        ],
        declarations: registry.declarations.clone(),
        last_validation: registry.last_validation.clone(),
        capability_surface: local_provider_adapter_capability_surface(),
        execution_status: LocalProviderAdapterExecutionStatus::ExecutionNotAvailableInPhase153
            .code()
            .to_string(),
        trust_status: LocalProviderAdapterTrustStatus::NoProviderTrust
            .code()
            .to_string(),
        boundary_statuses,
        note: "Adapter contract only; no model execution is available in Phase 153. Accepted adapter declarations are non-executing. Adapter declaration does not grant provider trust. No network, shell, secret, or production persistence capability is enabled.".to_string(),
    }
}

pub fn validate_local_provider_adapter_declaration(
    candidate: &LocalProviderAdapterConfigurationCandidate,
) -> LocalProviderAdapterValidation {
    let mut errors = std::collections::BTreeSet::new();
    let parsed_kind = match candidate.adapter_kind.as_deref() {
        None => {
            errors.insert(LocalProviderAdapterValidationError::MissingAdapterKind);
            None
        }
        Some(kind) if kind.trim().is_empty() => {
            errors.insert(LocalProviderAdapterValidationError::MissingAdapterKind);
            None
        }
        Some(kind) if kind.trim() != kind => {
            errors.insert(LocalProviderAdapterValidationError::MalformedAdapterKind);
            None
        }
        Some(kind) => match LocalProviderAdapterKind::parse(kind) {
            Some(LocalProviderAdapterKind::DeterministicFakeAdapter) => {
                Some(LocalProviderAdapterKind::DeterministicFakeAdapter)
            }
            Some(LocalProviderAdapterKind::LocalModelAdapterContract) => {
                Some(LocalProviderAdapterKind::LocalModelAdapterContract)
            }
            Some(LocalProviderAdapterKind::UnsupportedCloudModel)
            | Some(LocalProviderAdapterKind::UnsupportedNetworkAdapter) => {
                errors.insert(LocalProviderAdapterValidationError::CloudOrNetworkAdapterRejected);
                LocalProviderAdapterKind::parse(kind)
            }
            Some(LocalProviderAdapterKind::UnsupportedShellAdapter) => {
                errors.insert(LocalProviderAdapterValidationError::ShellAdapterRejected);
                Some(LocalProviderAdapterKind::UnsupportedShellAdapter)
            }
            Some(LocalProviderAdapterKind::UnsupportedFilesystemAdapter) => {
                errors.insert(LocalProviderAdapterValidationError::FilesystemAdapterRejected);
                Some(LocalProviderAdapterKind::UnsupportedFilesystemAdapter)
            }
            Some(other) => {
                errors.insert(LocalProviderAdapterValidationError::UnsupportedAdapter);
                Some(other)
            }
            None => {
                errors.insert(LocalProviderAdapterValidationError::UnsupportedAdapter);
                None
            }
        },
    };

    for (key, value) in &candidate.fields {
        reject_forbidden_provider_adapter_declaration_field(key, value, &mut errors);
    }

    let error_codes: Vec<_> = errors.into_iter().collect();
    if error_codes.is_empty()
        && matches!(
            parsed_kind,
            Some(LocalProviderAdapterKind::DeterministicFakeAdapter)
                | Some(LocalProviderAdapterKind::LocalModelAdapterContract)
        )
    {
        return LocalProviderAdapterValidation {
            status: LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting,
            adapter_kind: parsed_kind,
            declaration_id: candidate.declaration_id.clone(),
            error_codes,
            reason: "adapter declaration accepted as contract-only local projection; no provider execution, trust, network, shell, secrets, or production persistence is enabled".to_string(),
        };
    }

    LocalProviderAdapterValidation {
        status: if error_codes.contains(&LocalProviderAdapterValidationError::UnsupportedAdapter) {
            LocalProviderAdapterValidationStatus::UnsupportedAdapter
        } else {
            LocalProviderAdapterValidationStatus::InvalidAdapterDeclaration
        },
        adapter_kind: parsed_kind,
        declaration_id: candidate.declaration_id.clone(),
        error_codes,
        reason: "adapter declaration rejected fail-closed; prior registry projection remains unchanged and no provider execution occurs".to_string(),
    }
}

pub fn reject_forbidden_provider_adapter_declaration_field(
    key: &str,
    value: &str,
    errors: &mut std::collections::BTreeSet<LocalProviderAdapterValidationError>,
) {
    let lowered_key = key.to_ascii_lowercase();
    let combined = format!("{}={}", lowered_key, value.to_ascii_lowercase());
    if lowered_key == "label" || lowered_key == "description" {
        return;
    }
    if ["endpoint", "url", "host", "port", "http", "network"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::EndpointFieldRejected);
    } else if ["command", "args", "shell", "process"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::CommandFieldRejected);
    } else if ["executable", "binary"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::ExecutablePathRejected);
    } else if ["path", "file", "directory"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::PathFieldRejected);
    } else if ["secret", "token", "api_key", "apikey", "credential"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::SecretFieldRejected);
    } else if [
        "provider_execution",
        "execution_requested",
        "execution_flag",
    ]
    .iter()
    .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::ExecutionFlagRejected);
    } else if ["trust_granted", "provider_trust", "trust_claimed"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::ProviderTrustFlagRejected);
    } else if ["readiness", "ready"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::ReadinessClaimRejected);
    } else if ["release"].iter().any(|needle| combined.contains(needle)) {
        errors.insert(LocalProviderAdapterValidationError::ReleaseClaimRejected);
    } else if ["deployment", "deploy"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::DeploymentClaimRejected);
    } else if ["public_use", "public-use"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::PublicUseClaimRejected);
    } else if ["signing"].iter().any(|needle| combined.contains(needle)) {
        errors.insert(LocalProviderAdapterValidationError::SigningClaimRejected);
    } else if ["publishing", "publish"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterValidationError::PublishingClaimRejected);
    } else {
        errors.insert(LocalProviderAdapterValidationError::UnknownFieldRejected);
    }
}

pub fn apply_local_provider_adapter_declaration(
    state: &LocalOperatorShellState,
    candidate: LocalProviderAdapterConfigurationCandidate,
) -> Result<LocalOperatorShellState, LocalProviderAdapterValidation> {
    let validation = validate_local_provider_adapter_declaration(&candidate);
    if validation.status != LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting {
        return Err(validation);
    }
    let adapter_kind = validation
        .adapter_kind
        .expect("accepted adapter declaration includes adapter kind");
    let declaration_id = validation
        .declaration_id
        .clone()
        .unwrap_or_else(|| format!("local-adapter-declaration-{}", adapter_kind.code()));
    let declaration = LocalProviderAdapterDeclaration {
        adapter_kind,
        declaration_id,
        status: validation.status,
        contract: local_provider_adapter_contract(adapter_kind),
    };
    let mut next = state.clone();
    next.local_provider_adapter_registry = LocalProviderAdapterRegistry {
        declarations: vec![declaration],
        last_validation: validation,
    };
    Ok(attach_local_session_evidence_export(next))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterDryRunStatus {
    NotRun,
    DryRunExecuted,
    DryRunRejected,
    AdapterRequired,
    UnsupportedAdapter,
    InvalidDryRunRequest,
}

impl LocalProviderAdapterDryRunStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotRun => "not_run",
            Self::DryRunExecuted => "dry_run_executed",
            Self::DryRunRejected => "dry_run_rejected",
            Self::AdapterRequired => "adapter_required",
            Self::UnsupportedAdapter => "unsupported_adapter",
            Self::InvalidDryRunRequest => "invalid_dry_run_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalProviderAdapterDryRunError {
    NoAdapterDeclared,
    AdapterNotAccepted,
    UnsupportedAdapterKind,
    LocalModelAdapterNotExecutableInPhase154,
    CloudAdapterRejected,
    NetworkAdapterRejected,
    ShellAdapterRejected,
    FilesystemAdapterRejected,
    ExecutablePathRejected,
    EndpointFieldRejected,
    CommandFieldRejected,
    PathFieldRejected,
    SecretFieldRejected,
    ExecutionClaimRejected,
    TrustClaimRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    SigningClaimRejected,
    PublishingClaimRejected,
    ActionClaimRejected,
    PersistenceClaimRejected,
}

impl LocalProviderAdapterDryRunError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoAdapterDeclared => "no_adapter_declared",
            Self::AdapterNotAccepted => "adapter_not_accepted",
            Self::UnsupportedAdapterKind => "unsupported_adapter_kind",
            Self::LocalModelAdapterNotExecutableInPhase154 => {
                "local_model_adapter_not_executable_in_phase_154"
            }
            Self::CloudAdapterRejected => "cloud_adapter_rejected",
            Self::NetworkAdapterRejected => "network_adapter_rejected",
            Self::ShellAdapterRejected => "shell_adapter_rejected",
            Self::FilesystemAdapterRejected => "filesystem_adapter_rejected",
            Self::ExecutablePathRejected => "executable_path_rejected",
            Self::EndpointFieldRejected => "endpoint_field_rejected",
            Self::CommandFieldRejected => "command_field_rejected",
            Self::PathFieldRejected => "path_field_rejected",
            Self::SecretFieldRejected => "secret_field_rejected",
            Self::ExecutionClaimRejected => "execution_claim_rejected",
            Self::TrustClaimRejected => "trust_claim_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::SigningClaimRejected => "signing_claim_rejected",
            Self::PublishingClaimRejected => "publishing_claim_rejected",
            Self::ActionClaimRejected => "action_claim_rejected",
            Self::PersistenceClaimRejected => "persistence_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterDryRunBoundaryStatus {
    ControlledDryRunOnly,
    DeterministicFakeAdapterOnly,
    NoRealModelExecution,
    NoProcessSpawn,
    NoNetwork,
    NoShell,
    NoSecrets,
    NoProviderTrust,
    NoCandidateMaterialization,
    NoActionExecution,
    NoProductionPersistence,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl LocalProviderAdapterDryRunBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ControlledDryRunOnly => "controlled_dry_run_only",
            Self::DeterministicFakeAdapterOnly => "deterministic_fake_adapter_only",
            Self::NoRealModelExecution => "no_real_model_execution",
            Self::NoProcessSpawn => "no_process_spawn",
            Self::NoNetwork => "no_network",
            Self::NoShell => "no_shell",
            Self::NoSecrets => "no_secrets",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoCandidateMaterialization => "no_candidate_materialization",
            Self::NoActionExecution => "no_action_execution",
            Self::NoProductionPersistence => "no_production_persistence",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NoPublicUseEffect => "no_public_use_effect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterDryRunTrustStatus {
    UntrustedDescriptive,
}

impl LocalProviderAdapterDryRunTrustStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UntrustedDescriptive => "untrusted_descriptive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderAdapterDryRunEffectStatus {
    NoProviderTrust,
    NoCandidateMaterialization,
    NoActionExecution,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl LocalProviderAdapterDryRunEffectStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoCandidateMaterialization => "no_candidate_materialization",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NoPublicUseEffect => "no_public_use_effect",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterDryRunCapabilitySurface {
    pub controlled_dry_run_only: bool,
    pub deterministic_fake_adapter_only: bool,
    pub no_real_model_execution: bool,
    pub no_process_spawn: bool,
    pub no_network: bool,
    pub no_shell: bool,
    pub no_secrets: bool,
    pub no_provider_trust: bool,
    pub no_candidate_materialization: bool,
    pub no_action_execution: bool,
    pub no_production_persistence: bool,
    pub no_readiness_effect: bool,
    pub no_release_effect: bool,
    pub no_deployment_effect: bool,
    pub no_public_use_effect: bool,
    pub summary: String,
}

pub fn local_provider_adapter_dry_run_capability_surface(
) -> LocalProviderAdapterDryRunCapabilitySurface {
    LocalProviderAdapterDryRunCapabilitySurface {
        controlled_dry_run_only: true,
        deterministic_fake_adapter_only: true,
        no_real_model_execution: true,
        no_process_spawn: true,
        no_network: true,
        no_shell: true,
        no_secrets: true,
        no_provider_trust: true,
        no_candidate_materialization: true,
        no_action_execution: true,
        no_production_persistence: true,
        no_readiness_effect: true,
        no_release_effect: true,
        no_deployment_effect: true,
        no_public_use_effect: true,
        summary: "Controlled adapter dry run only; only deterministic_fake_adapter can execute in Phase 154. No real model execution, process, network, shell, secrets, provider trust, candidate materialization, action, readiness, release, deployment, public-use, or production persistence effect is enabled.".to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterDryRunRequest {
    pub input_summary: String,
    pub fields: Vec<(String, String)>,
}

impl LocalProviderAdapterDryRunRequest {
    pub fn deterministic_default() -> Self {
        Self {
            input_summary: "phase 154 deterministic fake adapter dry-run input".to_string(),
            fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterDryRunResult {
    pub result_id: String,
    pub adapter_kind: LocalProviderAdapterKind,
    pub adapter_declaration_id: String,
    pub output_summary: String,
    pub output_trust_status: LocalProviderAdapterDryRunTrustStatus,
    pub boundary_statuses: Vec<LocalProviderAdapterDryRunBoundaryStatus>,
    pub effect_statuses: Vec<LocalProviderAdapterDryRunEffectStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderAdapterDryRunProjection {
    pub status: LocalProviderAdapterDryRunStatus,
    pub adapter_kind: Option<LocalProviderAdapterKind>,
    pub adapter_declaration_id: Option<String>,
    pub result: Option<LocalProviderAdapterDryRunResult>,
    pub error_codes: Vec<LocalProviderAdapterDryRunError>,
    pub boundary_statuses: Vec<LocalProviderAdapterDryRunBoundaryStatus>,
    pub output_trust_status: LocalProviderAdapterDryRunTrustStatus,
    pub effect_statuses: Vec<LocalProviderAdapterDryRunEffectStatus>,
    pub capability_surface: LocalProviderAdapterDryRunCapabilitySurface,
    pub registry_declaration_count: usize,
    pub reason: String,
}

pub fn local_provider_adapter_dry_run_boundary_statuses(
) -> Vec<LocalProviderAdapterDryRunBoundaryStatus> {
    vec![
        LocalProviderAdapterDryRunBoundaryStatus::ControlledDryRunOnly,
        LocalProviderAdapterDryRunBoundaryStatus::DeterministicFakeAdapterOnly,
        LocalProviderAdapterDryRunBoundaryStatus::NoRealModelExecution,
        LocalProviderAdapterDryRunBoundaryStatus::NoProcessSpawn,
        LocalProviderAdapterDryRunBoundaryStatus::NoNetwork,
        LocalProviderAdapterDryRunBoundaryStatus::NoShell,
        LocalProviderAdapterDryRunBoundaryStatus::NoSecrets,
        LocalProviderAdapterDryRunBoundaryStatus::NoProviderTrust,
        LocalProviderAdapterDryRunBoundaryStatus::NoCandidateMaterialization,
        LocalProviderAdapterDryRunBoundaryStatus::NoActionExecution,
        LocalProviderAdapterDryRunBoundaryStatus::NoProductionPersistence,
        LocalProviderAdapterDryRunBoundaryStatus::NoReadinessEffect,
        LocalProviderAdapterDryRunBoundaryStatus::NoReleaseEffect,
        LocalProviderAdapterDryRunBoundaryStatus::NoDeploymentEffect,
        LocalProviderAdapterDryRunBoundaryStatus::NoPublicUseEffect,
    ]
}

pub fn local_provider_adapter_dry_run_effect_statuses(
) -> Vec<LocalProviderAdapterDryRunEffectStatus> {
    vec![
        LocalProviderAdapterDryRunEffectStatus::NoProviderTrust,
        LocalProviderAdapterDryRunEffectStatus::NoCandidateMaterialization,
        LocalProviderAdapterDryRunEffectStatus::NoActionExecution,
        LocalProviderAdapterDryRunEffectStatus::NoReadinessEffect,
        LocalProviderAdapterDryRunEffectStatus::NoReleaseEffect,
        LocalProviderAdapterDryRunEffectStatus::NoDeploymentEffect,
        LocalProviderAdapterDryRunEffectStatus::NoPublicUseEffect,
    ]
}

pub fn initial_local_provider_adapter_dry_run_projection() -> LocalProviderAdapterDryRunProjection {
    LocalProviderAdapterDryRunProjection {
        status: LocalProviderAdapterDryRunStatus::NotRun,
        adapter_kind: None,
        adapter_declaration_id: None,
        result: None,
        error_codes: Vec::new(),
        boundary_statuses: local_provider_adapter_dry_run_boundary_statuses(),
        output_trust_status: LocalProviderAdapterDryRunTrustStatus::UntrustedDescriptive,
        effect_statuses: local_provider_adapter_dry_run_effect_statuses(),
        capability_surface: local_provider_adapter_dry_run_capability_surface(),
        registry_declaration_count: 0,
        reason: "adapter dry-run not_run; deterministic_fake_adapter declaration is required before Phase 154 dry run".to_string(),
    }
}

fn reject_local_provider_adapter_dry_run(
    status: LocalProviderAdapterDryRunStatus,
    adapter_kind: Option<LocalProviderAdapterKind>,
    adapter_declaration_id: Option<String>,
    registry_declaration_count: usize,
    errors: Vec<LocalProviderAdapterDryRunError>,
) -> LocalProviderAdapterDryRunProjection {
    LocalProviderAdapterDryRunProjection {
        status,
        adapter_kind,
        adapter_declaration_id,
        result: None,
        error_codes: errors,
        boundary_statuses: local_provider_adapter_dry_run_boundary_statuses(),
        output_trust_status: LocalProviderAdapterDryRunTrustStatus::UntrustedDescriptive,
        effect_statuses: local_provider_adapter_dry_run_effect_statuses(),
        capability_surface: local_provider_adapter_dry_run_capability_surface(),
        registry_declaration_count,
        reason: "adapter dry-run rejected fail-closed; prior accepted shell state is preserved and no provider trust, candidate, action, readiness, release, deployment, public-use, process, network, shell, secret, environment, or persistence effect occurs".to_string(),
    }
}

pub fn validate_local_provider_adapter_dry_run_request(
    registry: &LocalProviderAdapterRegistry,
    request: &LocalProviderAdapterDryRunRequest,
) -> Result<LocalProviderAdapterDeclaration, Box<LocalProviderAdapterDryRunProjection>> {
    let mut errors = std::collections::BTreeSet::new();
    let declaration = match registry.declarations.last() {
        Some(declaration) => declaration,
        None => {
            return Err(Box::new(reject_local_provider_adapter_dry_run(
                LocalProviderAdapterDryRunStatus::AdapterRequired,
                None,
                None,
                registry.declarations.len(),
                vec![LocalProviderAdapterDryRunError::NoAdapterDeclared],
            )));
        }
    };

    if declaration.status != LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting {
        errors.insert(LocalProviderAdapterDryRunError::AdapterNotAccepted);
    }
    match declaration.adapter_kind {
        LocalProviderAdapterKind::DeterministicFakeAdapter => {}
        LocalProviderAdapterKind::LocalModelAdapterContract => {
            errors
                .insert(LocalProviderAdapterDryRunError::LocalModelAdapterNotExecutableInPhase154);
        }
        LocalProviderAdapterKind::UnsupportedCloudModel => {
            errors.insert(LocalProviderAdapterDryRunError::CloudAdapterRejected);
        }
        LocalProviderAdapterKind::UnsupportedNetworkAdapter => {
            errors.insert(LocalProviderAdapterDryRunError::NetworkAdapterRejected);
        }
        LocalProviderAdapterKind::UnsupportedShellAdapter => {
            errors.insert(LocalProviderAdapterDryRunError::ShellAdapterRejected);
        }
        LocalProviderAdapterKind::UnsupportedFilesystemAdapter => {
            errors.insert(LocalProviderAdapterDryRunError::FilesystemAdapterRejected);
        }
        LocalProviderAdapterKind::UnsupportedLocalModel | LocalProviderAdapterKind::Unknown => {
            errors.insert(LocalProviderAdapterDryRunError::UnsupportedAdapterKind);
        }
    }

    for (key, value) in &request.fields {
        reject_forbidden_provider_adapter_dry_run_field(key, value, &mut errors);
    }
    if !request.input_summary.trim().is_empty() {
        reject_forbidden_provider_adapter_dry_run_field(
            "input_summary",
            &request.input_summary,
            &mut errors,
        );
    }

    let error_codes: Vec<_> = errors.into_iter().collect();
    if error_codes.is_empty() {
        Ok(declaration.clone())
    } else {
        Err(Box::new(reject_local_provider_adapter_dry_run(
            if error_codes.iter().any(|error| {
                matches!(
                    error,
                    LocalProviderAdapterDryRunError::LocalModelAdapterNotExecutableInPhase154
                        | LocalProviderAdapterDryRunError::CloudAdapterRejected
                        | LocalProviderAdapterDryRunError::NetworkAdapterRejected
                        | LocalProviderAdapterDryRunError::ShellAdapterRejected
                        | LocalProviderAdapterDryRunError::FilesystemAdapterRejected
                        | LocalProviderAdapterDryRunError::UnsupportedAdapterKind
                )
            }) {
                LocalProviderAdapterDryRunStatus::UnsupportedAdapter
            } else {
                LocalProviderAdapterDryRunStatus::InvalidDryRunRequest
            },
            Some(declaration.adapter_kind),
            Some(declaration.declaration_id.clone()),
            registry.declarations.len(),
            error_codes,
        )))
    }
}

pub fn reject_forbidden_provider_adapter_dry_run_field(
    key: &str,
    value: &str,
    errors: &mut std::collections::BTreeSet<LocalProviderAdapterDryRunError>,
) {
    let lowered_key = key.to_ascii_lowercase();
    let combined = format!("{}={}", lowered_key, value.to_ascii_lowercase());
    if lowered_key == "label"
        || lowered_key == "description"
        || lowered_key == "input_summary"
            && !combined.contains("endpoint")
            && !combined.contains("url")
            && !combined.contains("host")
            && !combined.contains("port")
            && !combined.contains("command")
            && !combined.contains("args")
            && !combined.contains("process")
            && !combined.contains("shell")
            && !combined.contains("path")
            && !combined.contains("file")
            && !combined.contains("directory")
            && !combined.contains("token")
            && !combined.contains("secret")
            && !combined.contains("api_key")
            && !combined.contains("apikey")
            && !combined.contains("credential")
            && !combined.contains("execution")
            && !combined.contains("trust")
            && !combined.contains("readiness")
            && !combined.contains("release")
            && !combined.contains("deployment")
            && !combined.contains("public-use")
            && !combined.contains("public_use")
            && !combined.contains("signing")
            && !combined.contains("publishing")
            && !combined.contains("action")
            && !combined.contains("persistence")
    {
        return;
    }
    if ["endpoint", "url", "host", "port", "http", "network"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::EndpointFieldRejected);
    } else if ["command", "args", "process", "shell"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::CommandFieldRejected);
    } else if ["executable", "binary"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::ExecutablePathRejected);
    } else if ["path", "file", "directory"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::PathFieldRejected);
    } else if ["secret", "token", "api_key", "apikey", "credential"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::SecretFieldRejected);
    } else if ["execution"].iter().any(|needle| combined.contains(needle)) {
        errors.insert(LocalProviderAdapterDryRunError::ExecutionClaimRejected);
    } else if ["trust"].iter().any(|needle| combined.contains(needle)) {
        errors.insert(LocalProviderAdapterDryRunError::TrustClaimRejected);
    } else if ["readiness", "ready"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::ReadinessClaimRejected);
    } else if combined.contains("release") {
        errors.insert(LocalProviderAdapterDryRunError::ReleaseClaimRejected);
    } else if ["deployment", "deploy"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::DeploymentClaimRejected);
    } else if ["public_use", "public-use"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::PublicUseClaimRejected);
    } else if combined.contains("signing") {
        errors.insert(LocalProviderAdapterDryRunError::SigningClaimRejected);
    } else if ["publishing", "publish"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(LocalProviderAdapterDryRunError::PublishingClaimRejected);
    } else if combined.contains("action") {
        errors.insert(LocalProviderAdapterDryRunError::ActionClaimRejected);
    } else if combined.contains("persistence") {
        errors.insert(LocalProviderAdapterDryRunError::PersistenceClaimRejected);
    }
}

fn deterministic_adapter_dry_run_checksum(input: &str) -> u64 {
    input.bytes().fold(154_u64, |acc, byte| {
        acc.wrapping_mul(31).wrapping_add(byte as u64)
    })
}

pub fn execute_deterministic_fake_adapter_dry_run(
    declaration: &LocalProviderAdapterDeclaration,
    request: &LocalProviderAdapterDryRunRequest,
) -> LocalProviderAdapterDryRunResult {
    let checksum = deterministic_adapter_dry_run_checksum(&format!(
        "{}|{}|{}",
        declaration.declaration_id,
        declaration.adapter_kind.code(),
        request.input_summary
    ));
    LocalProviderAdapterDryRunResult {
        result_id: format!("local-provider-adapter-dry-run-{checksum:016x}"),
        adapter_kind: declaration.adapter_kind,
        adapter_declaration_id: declaration.declaration_id.clone(),
        output_summary: format!(
            "deterministic_fake_adapter dry-run descriptive output for input_bytes={} checksum={checksum:016x}",
            request.input_summary.len()
        ),
        output_trust_status: LocalProviderAdapterDryRunTrustStatus::UntrustedDescriptive,
        boundary_statuses: local_provider_adapter_dry_run_boundary_statuses(),
        effect_statuses: local_provider_adapter_dry_run_effect_statuses(),
    }
}

pub fn project_local_provider_adapter_dry_run(
    registry: &LocalProviderAdapterRegistry,
    result: LocalProviderAdapterDryRunResult,
) -> LocalProviderAdapterDryRunProjection {
    LocalProviderAdapterDryRunProjection {
        status: LocalProviderAdapterDryRunStatus::DryRunExecuted,
        adapter_kind: Some(result.adapter_kind),
        adapter_declaration_id: Some(result.adapter_declaration_id.clone()),
        result: Some(result),
        error_codes: Vec::new(),
        boundary_statuses: local_provider_adapter_dry_run_boundary_statuses(),
        output_trust_status: LocalProviderAdapterDryRunTrustStatus::UntrustedDescriptive,
        effect_statuses: local_provider_adapter_dry_run_effect_statuses(),
        capability_surface: local_provider_adapter_dry_run_capability_surface(),
        registry_declaration_count: registry.declarations.len(),
        reason: "controlled adapter dry run executed through deterministic_fake_adapter only; output remains untrusted_descriptive and no provider trust, candidate, action, readiness, release, deployment, public-use, process, network, shell, secret, environment, or persistence effect occurs".to_string(),
    }
}

pub fn apply_local_provider_adapter_dry_run(
    state: &LocalOperatorShellState,
    request: LocalProviderAdapterDryRunRequest,
) -> Result<LocalOperatorShellState, Box<LocalProviderAdapterDryRunProjection>> {
    let declaration = validate_local_provider_adapter_dry_run_request(
        &state.local_provider_adapter_registry,
        &request,
    )?;
    let result = execute_deterministic_fake_adapter_dry_run(&declaration, &request);
    let mut next = state.clone();
    next.local_provider_adapter_dry_run =
        project_local_provider_adapter_dry_run(&state.local_provider_adapter_registry, result);
    Ok(attach_local_session_evidence_export(next))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllowlistedLocalProviderKind {
    AllowlistedLocalDeterministicProvider,
    UnsupportedLocalProvider,
    UnsupportedCloudProvider,
    UnsupportedNetworkProvider,
    UnsupportedShellProvider,
}

impl AllowlistedLocalProviderKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AllowlistedLocalDeterministicProvider => {
                "allowlisted_local_deterministic_provider"
            }
            Self::UnsupportedLocalProvider => "unsupported_local_provider",
            Self::UnsupportedCloudProvider => "unsupported_cloud_provider",
            Self::UnsupportedNetworkProvider => "unsupported_network_provider",
            Self::UnsupportedShellProvider => "unsupported_shell_provider",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstrainedLocalProviderInvocationStatus {
    NotInvoked,
    InvocationExecuted,
    InvocationRejected,
    AllowlistedProviderRequired,
    UnsupportedProvider,
    InvalidInvocationRequest,
}

impl ConstrainedLocalProviderInvocationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotInvoked => "not_invoked",
            Self::InvocationExecuted => "invocation_executed",
            Self::InvocationRejected => "invocation_rejected",
            Self::AllowlistedProviderRequired => "allowlisted_provider_required",
            Self::UnsupportedProvider => "unsupported_provider",
            Self::InvalidInvocationRequest => "invalid_invocation_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstrainedLocalProviderInvocationError {
    NoAdapterDeclared,
    AdapterNotAccepted,
    ProviderNotAllowlisted,
    ArbitraryCommandRejected,
    ShellFieldRejected,
    ProcessFieldRejected,
    ArgsFieldRejected,
    EndpointFieldRejected,
    NetworkFieldRejected,
    SecretFieldRejected,
    PathFieldRejected,
    TrustClaimRejected,
    ProviderOutputApprovalClaimRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    CandidateMaterializationClaimRejected,
    ActionClaimRejected,
    PersistenceClaimRejected,
}

impl ConstrainedLocalProviderInvocationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoAdapterDeclared => "no_adapter_declared",
            Self::AdapterNotAccepted => "adapter_not_accepted",
            Self::ProviderNotAllowlisted => "provider_not_allowlisted",
            Self::ArbitraryCommandRejected => "arbitrary_command_rejected",
            Self::ShellFieldRejected => "shell_field_rejected",
            Self::ProcessFieldRejected => "process_field_rejected",
            Self::ArgsFieldRejected => "args_field_rejected",
            Self::EndpointFieldRejected => "endpoint_field_rejected",
            Self::NetworkFieldRejected => "network_field_rejected",
            Self::SecretFieldRejected => "secret_field_rejected",
            Self::PathFieldRejected => "path_field_rejected",
            Self::TrustClaimRejected => "trust_claim_rejected",
            Self::ProviderOutputApprovalClaimRejected => "provider_output_approval_claim_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::CandidateMaterializationClaimRejected => {
                "candidate_materialization_claim_rejected"
            }
            Self::ActionClaimRejected => "action_claim_rejected",
            Self::PersistenceClaimRejected => "persistence_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstrainedLocalProviderInvocationBoundaryStatus {
    ConstrainedLocalInvocationOnly,
    AllowlistedProviderOnly,
    NoArbitraryCommand,
    NoShell,
    NoNetwork,
    NoCloud,
    NoSecrets,
    NoProviderTrust,
    NoCandidateMaterialization,
    NoActionExecution,
    NoProductionPersistence,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl ConstrainedLocalProviderInvocationBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ConstrainedLocalInvocationOnly => "constrained_local_invocation_only",
            Self::AllowlistedProviderOnly => "allowlisted_provider_only",
            Self::NoArbitraryCommand => "no_arbitrary_command",
            Self::NoShell => "no_shell",
            Self::NoNetwork => "no_network",
            Self::NoCloud => "no_cloud",
            Self::NoSecrets => "no_secrets",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoCandidateMaterialization => "no_candidate_materialization",
            Self::NoActionExecution => "no_action_execution",
            Self::NoProductionPersistence => "no_production_persistence",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NoPublicUseEffect => "no_public_use_effect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstrainedLocalProviderInvocationTrustStatus {
    UntrustedDescriptive,
}

impl ConstrainedLocalProviderInvocationTrustStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UntrustedDescriptive => "untrusted_descriptive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstrainedLocalProviderInvocationEffectStatus {
    NoProviderTrust,
    NoCandidateMaterialization,
    NoActionExecution,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl ConstrainedLocalProviderInvocationEffectStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoCandidateMaterialization => "no_candidate_materialization",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NoPublicUseEffect => "no_public_use_effect",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstrainedLocalProviderInvocationCapabilitySurface {
    pub constrained_local_invocation_only: bool,
    pub allowlisted_provider_only: bool,
    pub allowlisted_provider_kind: AllowlistedLocalProviderKind,
    pub no_arbitrary_command: bool,
    pub no_shell: bool,
    pub no_network: bool,
    pub no_cloud: bool,
    pub no_secrets: bool,
    pub no_provider_trust: bool,
    pub no_candidate_materialization: bool,
    pub no_action_execution: bool,
    pub no_production_persistence: bool,
    pub no_readiness_effect: bool,
    pub no_release_effect: bool,
    pub no_deployment_effect: bool,
    pub no_public_use_effect: bool,
    pub summary: String,
}

pub fn constrained_local_provider_invocation_capability_surface(
) -> ConstrainedLocalProviderInvocationCapabilitySurface {
    ConstrainedLocalProviderInvocationCapabilitySurface {
        constrained_local_invocation_only: true,
        allowlisted_provider_only: true,
        allowlisted_provider_kind: AllowlistedLocalProviderKind::AllowlistedLocalDeterministicProvider,
        no_arbitrary_command: true,
        no_shell: true,
        no_network: true,
        no_cloud: true,
        no_secrets: true,
        no_provider_trust: true,
        no_candidate_materialization: true,
        no_action_execution: true,
        no_production_persistence: true,
        no_readiness_effect: true,
        no_release_effect: true,
        no_deployment_effect: true,
        no_public_use_effect: true,
        summary: "Constrained local provider invocation only; only allowlisted_local_deterministic_provider is enabled in Phase 156. No arbitrary command, shell, network, cloud, secrets, provider trust, candidate materialization, action, readiness, release, deployment, public-use, or production persistence effect is enabled.".to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstrainedLocalProviderInvocationRequest {
    pub provider_kind: AllowlistedLocalProviderKind,
    pub input_summary: String,
    pub fields: Vec<(String, String)>,
}

impl ConstrainedLocalProviderInvocationRequest {
    pub fn allowlisted_default() -> Self {
        Self {
            provider_kind: AllowlistedLocalProviderKind::AllowlistedLocalDeterministicProvider,
            input_summary: "phase 156 constrained local provider invocation input".to_string(),
            fields: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstrainedLocalProviderInvocationResult {
    pub result_id: String,
    pub provider_kind: AllowlistedLocalProviderKind,
    pub adapter_kind: LocalProviderAdapterKind,
    pub adapter_declaration_id: String,
    pub output_summary: String,
    pub output_trust_status: ConstrainedLocalProviderInvocationTrustStatus,
    pub boundary_statuses: Vec<ConstrainedLocalProviderInvocationBoundaryStatus>,
    pub effect_statuses: Vec<ConstrainedLocalProviderInvocationEffectStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstrainedLocalProviderInvocationProjection {
    pub status: ConstrainedLocalProviderInvocationStatus,
    pub provider_kind: Option<AllowlistedLocalProviderKind>,
    pub adapter_kind: Option<LocalProviderAdapterKind>,
    pub adapter_declaration_id: Option<String>,
    pub result: Option<ConstrainedLocalProviderInvocationResult>,
    pub error_codes: Vec<ConstrainedLocalProviderInvocationError>,
    pub boundary_statuses: Vec<ConstrainedLocalProviderInvocationBoundaryStatus>,
    pub output_trust_status: ConstrainedLocalProviderInvocationTrustStatus,
    pub effect_statuses: Vec<ConstrainedLocalProviderInvocationEffectStatus>,
    pub capability_surface: ConstrainedLocalProviderInvocationCapabilitySurface,
    pub registry_declaration_count: usize,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputPipelineSourceKind {
    DeterministicStubProviderExecution,
    ConstrainedLocalProviderInvocation,
}

impl LocalProviderOutputPipelineSourceKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DeterministicStubProviderExecution => "deterministic_stub_provider_execution",
            Self::ConstrainedLocalProviderInvocation => "constrained_local_provider_invocation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalProviderOutputPipelineStage {
    InvocationOutputProjected,
    ProviderExecutionResultProjected,
    ProviderOutputValidationRequired,
    ProviderOutputValidationProjected,
    ProviderOutputReviewRequired,
    ProviderOutputReviewProjected,
    StagedProposalRequired,
    StagedProposalProjected,
    StagedProposalValidationRequired,
    StagedProposalValidationProjected,
    CandidateReviewRequired,
    CandidateReviewProjected,
    OperatorDecisionRequired,
    OperatorDecisionProjected,
}

impl LocalProviderOutputPipelineStage {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvocationOutputProjected => "invocation_output_projected",
            Self::ProviderExecutionResultProjected => "provider_execution_result_projected",
            Self::ProviderOutputValidationRequired => "provider_output_validation_required",
            Self::ProviderOutputValidationProjected => "provider_output_validation_projected",
            Self::ProviderOutputReviewRequired => "provider_output_review_required",
            Self::ProviderOutputReviewProjected => "provider_output_review_projected",
            Self::StagedProposalRequired => "staged_conversion_required",
            Self::StagedProposalProjected => "staged_proposal_projected",
            Self::StagedProposalValidationRequired => "staged_validation_required",
            Self::StagedProposalValidationProjected => "staged_proposal_validation_projected",
            Self::CandidateReviewRequired => "candidate_review_required",
            Self::CandidateReviewProjected => "candidate_review_projected",
            Self::OperatorDecisionRequired => "operator_decision_required",
            Self::OperatorDecisionProjected => "operator_decision_projected",
        }
    }
}

pub fn local_provider_output_pipeline_stage_order() -> Vec<LocalProviderOutputPipelineStage> {
    vec![
        LocalProviderOutputPipelineStage::InvocationOutputProjected,
        LocalProviderOutputPipelineStage::ProviderExecutionResultProjected,
        LocalProviderOutputPipelineStage::ProviderOutputValidationRequired,
        LocalProviderOutputPipelineStage::ProviderOutputValidationProjected,
        LocalProviderOutputPipelineStage::ProviderOutputReviewRequired,
        LocalProviderOutputPipelineStage::ProviderOutputReviewProjected,
        LocalProviderOutputPipelineStage::StagedProposalRequired,
        LocalProviderOutputPipelineStage::StagedProposalProjected,
        LocalProviderOutputPipelineStage::StagedProposalValidationRequired,
        LocalProviderOutputPipelineStage::StagedProposalValidationProjected,
        LocalProviderOutputPipelineStage::CandidateReviewRequired,
        LocalProviderOutputPipelineStage::CandidateReviewProjected,
        LocalProviderOutputPipelineStage::OperatorDecisionRequired,
        LocalProviderOutputPipelineStage::OperatorDecisionProjected,
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputPipelineStageStatus {
    NotStarted,
    Available,
    Blocked,
    Completed,
    Rejected,
    NotApplicable,
}

impl LocalProviderOutputPipelineStageStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Available => "available",
            Self::Blocked => "blocked",
            Self::Completed => "completed",
            Self::Rejected => "rejected",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputPipelineValidationStatus {
    NotStarted,
    Valid,
    Blocked,
    Rejected,
}

impl LocalProviderOutputPipelineValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Valid => "valid",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalProviderOutputPipelineError {
    NoInvocationOutput,
    InvocationOutputRejected,
    InvocationOutputNotUntrustedDescriptive,
    InvocationResultIdMismatch,
    InvocationOutputSummaryMismatch,
    ProviderOutputValidationMissing,
    ProviderOutputValidationRejected,
    ProviderOutputReviewMissing,
    StagedProposalMissing,
    StagedProposalValidationMissing,
    StagedProposalValidationRejected,
    CandidateReviewMissing,
    OperatorDecisionMissing,
    PipelineSkipAttemptRejected,
    TrustClaimRejected,
    ApprovalClaimRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    CandidateCreationClaimRejected,
    CandidateMaterializationClaimRejected,
    ActionClaimRejected,
    PersistenceClaimRejected,
}

impl LocalProviderOutputPipelineError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoInvocationOutput => "no_invocation_output",
            Self::InvocationOutputRejected => "invocation_output_rejected",
            Self::InvocationOutputNotUntrustedDescriptive => {
                "invocation_output_not_untrusted_descriptive"
            }
            Self::InvocationResultIdMismatch => "invocation_result_id_mismatch",
            Self::InvocationOutputSummaryMismatch => "invocation_output_summary_mismatch",
            Self::ProviderOutputValidationMissing => "provider_output_validation_missing",
            Self::ProviderOutputValidationRejected => "provider_output_validation_rejected",
            Self::ProviderOutputReviewMissing => "provider_output_review_missing",
            Self::StagedProposalMissing => "staged_proposal_missing",
            Self::StagedProposalValidationMissing => "staged_proposal_validation_missing",
            Self::StagedProposalValidationRejected => "staged_proposal_validation_rejected",
            Self::CandidateReviewMissing => "candidate_review_missing",
            Self::OperatorDecisionMissing => "operator_decision_missing",
            Self::PipelineSkipAttemptRejected => "pipeline_skip_attempt_rejected",
            Self::TrustClaimRejected => "trust_claim_rejected",
            Self::ApprovalClaimRejected => "approval_claim_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::CandidateCreationClaimRejected => "candidate_creation_claim_rejected",
            Self::CandidateMaterializationClaimRejected => {
                "candidate_materialization_claim_rejected"
            }
            Self::ActionClaimRejected => "action_claim_rejected",
            Self::PersistenceClaimRejected => "persistence_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputPipelineBoundaryStatus {
    PipelineIntegrationOnly,
    UntrustedDescriptiveSource,
    ValidationRequired,
    ReviewRequired,
    StagedConversionRequired,
    StagedValidationRequired,
    CandidateReviewRequired,
    OperatorDecisionRequired,
    NoCandidateMaterialization,
    NoProviderTrust,
    NoActionExecution,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl LocalProviderOutputPipelineBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::PipelineIntegrationOnly => "pipeline_integration_only",
            Self::UntrustedDescriptiveSource => "untrusted_descriptive_source",
            Self::ValidationRequired => "validation_required",
            Self::ReviewRequired => "review_required",
            Self::StagedConversionRequired => "staged_conversion_required",
            Self::StagedValidationRequired => "staged_validation_required",
            Self::CandidateReviewRequired => "candidate_review_required",
            Self::OperatorDecisionRequired => "operator_decision_required",
            Self::NoCandidateMaterialization => "no_candidate_materialization",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NoPublicUseEffect => "no_public_use_effect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalProviderOutputPipelineEffectStatus {
    NoProviderExecution,
    NoCandidateCreation,
    NoCandidateMaterialization,
    NoDecisionLedgerAppend,
    NoReplayMutation,
    NoExportPromotion,
    NoProviderTrust,
    NoFileWrite,
    NoNetworkSocket,
    NoProcessSpawn,
    NoSecretRead,
    NoActionExecution,
}

impl LocalProviderOutputPipelineEffectStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoProviderExecution => "no_provider_execution",
            Self::NoCandidateCreation => "no_candidate_creation",
            Self::NoCandidateMaterialization => "no_candidate_materialization",
            Self::NoDecisionLedgerAppend => "no_decision_ledger_append",
            Self::NoReplayMutation => "no_replay_mutation",
            Self::NoExportPromotion => "no_export_promotion",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoFileWrite => "no_file_write",
            Self::NoNetworkSocket => "no_network_socket",
            Self::NoProcessSpawn => "no_process_spawn",
            Self::NoSecretRead => "no_secret_read",
            Self::NoActionExecution => "no_action_execution",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderOutputPipelineStageProjection {
    pub stage: LocalProviderOutputPipelineStage,
    pub status: LocalProviderOutputPipelineStageStatus,
    pub reason: Option<LocalProviderOutputPipelineError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvocationProviderOutputBridgeProjection {
    pub source_kind: LocalProviderOutputPipelineSourceKind,
    pub invocation_result_id: String,
    pub provider_execution_result_id: String,
    pub output_summary: String,
    pub output_trust_status: LocalProviderOutputTrustStatus,
    pub output_materialization_status: LocalProviderOutputMaterializationStatus,
    pub output_promotion_status: LocalProviderOutputPromotionStatus,
    pub descriptive_only: bool,
    pub not_candidate_material: bool,
    pub not_promoted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalProviderOutputPipelineProjection {
    pub status: LocalProviderOutputPipelineValidationStatus,
    pub source_kind: Option<LocalProviderOutputPipelineSourceKind>,
    pub source_invocation_result_id: Option<String>,
    pub provider_execution_result_id: Option<String>,
    pub current_stage: Option<LocalProviderOutputPipelineStage>,
    pub next_required_stage: Option<LocalProviderOutputPipelineStage>,
    pub stages: Vec<LocalProviderOutputPipelineStageProjection>,
    pub provider_output_validation_status: LocalProviderOutputValidationStatus,
    pub provider_output_review_status: LocalProviderOutputReviewabilityStatus,
    pub staged_proposal_status: StagedCandidateConversionProposalStatus,
    pub staged_proposal_validation_status: StagedCandidateConversionValidationStatus,
    pub candidate_review_status: String,
    pub operator_decision_status: OperatorCandidateDecisionStatus,
    pub boundary_statuses: Vec<LocalProviderOutputPipelineBoundaryStatus>,
    pub effect_statuses: Vec<LocalProviderOutputPipelineEffectStatus>,
    pub errors: Vec<LocalProviderOutputPipelineError>,
    pub bridge: Option<InvocationProviderOutputBridgeProjection>,
    pub note: String,
}

pub fn local_provider_output_pipeline_boundary_statuses(
) -> Vec<LocalProviderOutputPipelineBoundaryStatus> {
    vec![
        LocalProviderOutputPipelineBoundaryStatus::PipelineIntegrationOnly,
        LocalProviderOutputPipelineBoundaryStatus::UntrustedDescriptiveSource,
        LocalProviderOutputPipelineBoundaryStatus::ValidationRequired,
        LocalProviderOutputPipelineBoundaryStatus::ReviewRequired,
        LocalProviderOutputPipelineBoundaryStatus::StagedConversionRequired,
        LocalProviderOutputPipelineBoundaryStatus::StagedValidationRequired,
        LocalProviderOutputPipelineBoundaryStatus::CandidateReviewRequired,
        LocalProviderOutputPipelineBoundaryStatus::OperatorDecisionRequired,
        LocalProviderOutputPipelineBoundaryStatus::NoCandidateMaterialization,
        LocalProviderOutputPipelineBoundaryStatus::NoProviderTrust,
        LocalProviderOutputPipelineBoundaryStatus::NoActionExecution,
        LocalProviderOutputPipelineBoundaryStatus::NoReadinessEffect,
        LocalProviderOutputPipelineBoundaryStatus::NoReleaseEffect,
        LocalProviderOutputPipelineBoundaryStatus::NoDeploymentEffect,
        LocalProviderOutputPipelineBoundaryStatus::NoPublicUseEffect,
    ]
}

pub fn local_provider_output_pipeline_effect_statuses(
) -> Vec<LocalProviderOutputPipelineEffectStatus> {
    vec![
        LocalProviderOutputPipelineEffectStatus::NoProviderExecution,
        LocalProviderOutputPipelineEffectStatus::NoCandidateCreation,
        LocalProviderOutputPipelineEffectStatus::NoCandidateMaterialization,
        LocalProviderOutputPipelineEffectStatus::NoDecisionLedgerAppend,
        LocalProviderOutputPipelineEffectStatus::NoReplayMutation,
        LocalProviderOutputPipelineEffectStatus::NoExportPromotion,
        LocalProviderOutputPipelineEffectStatus::NoProviderTrust,
        LocalProviderOutputPipelineEffectStatus::NoFileWrite,
        LocalProviderOutputPipelineEffectStatus::NoNetworkSocket,
        LocalProviderOutputPipelineEffectStatus::NoProcessSpawn,
        LocalProviderOutputPipelineEffectStatus::NoSecretRead,
        LocalProviderOutputPipelineEffectStatus::NoActionExecution,
    ]
}

pub fn initial_local_provider_output_pipeline_projection() -> LocalProviderOutputPipelineProjection
{
    LocalProviderOutputPipelineProjection {
        status: LocalProviderOutputPipelineValidationStatus::NotStarted,
        source_kind: None,
        source_invocation_result_id: None,
        provider_execution_result_id: None,
        current_stage: None,
        next_required_stage: Some(LocalProviderOutputPipelineStage::InvocationOutputProjected),
        stages: local_provider_output_pipeline_stage_order()
            .into_iter()
            .map(|stage| LocalProviderOutputPipelineStageProjection {
                stage,
                status: LocalProviderOutputPipelineStageStatus::NotStarted,
                reason: None,
            })
            .collect(),
        provider_output_validation_status: LocalProviderOutputValidationStatus::NotValidated,
        provider_output_review_status: LocalProviderOutputReviewabilityStatus::NotReviewable,
        staged_proposal_status: StagedCandidateConversionProposalStatus::NoProposal,
        staged_proposal_validation_status: StagedCandidateConversionValidationStatus::NotValidated,
        candidate_review_status: "not_available".to_string(),
        operator_decision_status: OperatorCandidateDecisionStatus::NoOperatorDecision,
        boundary_statuses: local_provider_output_pipeline_boundary_statuses(),
        effect_statuses: local_provider_output_pipeline_effect_statuses(),
        errors: Vec::new(),
        bridge: None,
        note: "Provider output pipeline integration has not started; invocation output remains untrusted and descriptive.".to_string(),
    }
}

fn invocation_output_pipeline_claim_errors(text: &str) -> Vec<LocalProviderOutputPipelineError> {
    let lower = text.to_ascii_lowercase();
    let mut errors = std::collections::BTreeSet::new();
    if [
        "trust",
        "trusted",
        "provider_output_approval",
        "approved output",
        "approval granted",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::TrustClaimRejected);
        errors.insert(LocalProviderOutputPipelineError::ApprovalClaimRejected);
    }
    if ["readiness", "ready for"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::ReadinessClaimRejected);
    }
    if lower.contains("release") {
        errors.insert(LocalProviderOutputPipelineError::ReleaseClaimRejected);
    }
    if ["deployment", "deploy"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::DeploymentClaimRejected);
    }
    if ["public_use", "public-use", "public use"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::PublicUseClaimRejected);
    }
    if ["candidate creation", "candidate_output"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::CandidateCreationClaimRejected);
    }
    if ["candidate", "materialization"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::CandidateMaterializationClaimRejected);
    }
    if lower.contains("action") {
        errors.insert(LocalProviderOutputPipelineError::ActionClaimRejected);
    }
    if ["persist", "persistence"]
        .iter()
        .any(|needle| lower.contains(needle))
    {
        errors.insert(LocalProviderOutputPipelineError::PersistenceClaimRejected);
    }
    errors.into_iter().collect()
}

pub fn project_invocation_output_into_provider_pipeline(
    state: &LocalOperatorShellState,
) -> Result<InvocationProviderOutputBridgeProjection, Vec<LocalProviderOutputPipelineError>> {
    let invocation = &state.constrained_local_provider_invocation;
    if invocation.status == ConstrainedLocalProviderInvocationStatus::NotInvoked {
        return Err(vec![LocalProviderOutputPipelineError::NoInvocationOutput]);
    }
    if invocation.status != ConstrainedLocalProviderInvocationStatus::InvocationExecuted {
        return Err(vec![
            LocalProviderOutputPipelineError::InvocationOutputRejected,
        ]);
    }
    let Some(result) = invocation.result.as_ref() else {
        return Err(vec![LocalProviderOutputPipelineError::NoInvocationOutput]);
    };
    let mut errors = std::collections::BTreeSet::new();
    if result.output_trust_status
        != ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive
        || invocation.output_trust_status
            != ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive
    {
        errors.insert(LocalProviderOutputPipelineError::InvocationOutputNotUntrustedDescriptive);
    }
    if !result
        .result_id
        .starts_with("constrained-local-provider-invocation-")
        || result.result_id.len() <= "constrained-local-provider-invocation-".len()
    {
        errors.insert(LocalProviderOutputPipelineError::InvocationResultIdMismatch);
    }
    let checksum = result
        .result_id
        .strip_prefix("constrained-local-provider-invocation-")
        .unwrap_or("");
    if !result
        .output_summary
        .starts_with("allowlisted_local_deterministic_provider descriptive output for input_bytes=")
        || !result.output_summary.contains(" checksum=")
        || !result.output_summary.ends_with(checksum)
    {
        errors.insert(LocalProviderOutputPipelineError::InvocationOutputSummaryMismatch);
    }
    for error in invocation_output_pipeline_claim_errors(&result.output_summary) {
        errors.insert(error);
    }
    for required in constrained_local_provider_invocation_boundary_statuses() {
        if !result.boundary_statuses.contains(&required)
            || !invocation.boundary_statuses.contains(&required)
        {
            errors
                .insert(LocalProviderOutputPipelineError::InvocationOutputNotUntrustedDescriptive);
        }
    }
    for required in constrained_local_provider_invocation_effect_statuses() {
        if !result.effect_statuses.contains(&required)
            || !invocation.effect_statuses.contains(&required)
        {
            errors.insert(LocalProviderOutputPipelineError::CandidateMaterializationClaimRejected);
        }
    }
    if !errors.is_empty() {
        return Err(errors.into_iter().collect());
    }
    Ok(InvocationProviderOutputBridgeProjection {
        source_kind: LocalProviderOutputPipelineSourceKind::ConstrainedLocalProviderInvocation,
        invocation_result_id: result.result_id.clone(),
        provider_execution_result_id: result.result_id.clone(),
        output_summary: result.output_summary.clone(),
        output_trust_status: LocalProviderOutputTrustStatus::UntrustedDescriptive,
        output_materialization_status:
            LocalProviderOutputMaterializationStatus::ProjectedAsUntrustedOutput,
        output_promotion_status: LocalProviderOutputPromotionStatus::NotPromoted,
        descriptive_only: true,
        not_candidate_material: true,
        not_promoted: true,
    })
}

pub fn provider_execution_projection_from_invocation_bridge(
    state: &LocalOperatorShellState,
    bridge: &InvocationProviderOutputBridgeProjection,
) -> LocalProviderExecutionProjection {
    with_provider_execution_projection_validation(LocalProviderExecutionProjection {
        status: LocalProviderExecutionStatus::Executed,
        projection_status: LocalProviderExecutionResultProjectionStatus::ExecutionProjected,
        configured_provider_kind: "allowlisted_local_deterministic_provider".to_string(),
        sandbox_status: LocalProviderExecutionSandboxStatus::SandboxedDeterministicNoExternalEffects,
        output_trust_status: LocalProviderOutputTrustStatus::UntrustedDescriptive,
        output_materialization_status: LocalProviderOutputMaterializationStatus::NotCandidateMaterial,
        output_promotion_status: LocalProviderOutputPromotionStatus::NotPromoted,
        promotion_availability_status: LocalProviderOutputPromotionStatus::PromotionNotAvailableInPhase142,
        linkage: LocalProviderExecutionResultLinkage {
            execution_result_id: bridge.provider_execution_result_id.clone(),
            source_boundary: "constrained_local_provider_invocation_pipeline_bridge".to_string(),
            ..local_provider_execution_linkage(state, &bridge.provider_execution_result_id)
        },
        absence_markers: local_provider_execution_result_absence_markers(),
        projection_validation: LocalProviderExecutionResultProjectionValidation {
            status: LocalProviderExecutionResultProjectionValidationStatus::Invalid,
            error_codes: Vec::new(),
            reason: "projection validation pending".to_string(),
        },
        result: Some(LocalProviderExecutionResult {
            result_id: bridge.provider_execution_result_id.clone(),
            provider_kind: LocalProviderKind::DeterministicStub,
            sandbox_status: LocalProviderExecutionSandboxStatus::SandboxedDeterministicNoExternalEffects,
            output_summary: bridge.output_summary.clone(),
            output_trust_status: LocalProviderOutputTrustStatus::UntrustedDescriptive,
            output_materialization_status: LocalProviderOutputMaterializationStatus::ProjectedAsUntrustedOutput,
            output_promotion_status: LocalProviderOutputPromotionStatus::NotPromoted,
            promotion_availability_status: LocalProviderOutputPromotionStatus::PromotionNotAvailableInPhase142,
            descriptive_only: true,
            provider_output_trusted: false,
            candidate_output_promoted: false,
            decision_appended: false,
            replay_repaired: false,
            release_or_deployment_evidence_created: false,
        }),
        validation_status: "invocation_output_projected".to_string(),
        validation_error_codes: Vec::new(),
        validation_reason: "constrained invocation output projected into provider execution/result surface without changing trust boundary".to_string(),
        capability_surface: local_provider_execution_capability_surface(),
        note: "Constrained invocation output is projected as untrusted_descriptive provider output only; it is not candidate material, not promoted, and not approved.".to_string(),
    })
}

fn pipeline_stage(
    stage: LocalProviderOutputPipelineStage,
    status: LocalProviderOutputPipelineStageStatus,
    reason: Option<LocalProviderOutputPipelineError>,
) -> LocalProviderOutputPipelineStageProjection {
    LocalProviderOutputPipelineStageProjection {
        stage,
        status,
        reason,
    }
}

pub fn derive_local_provider_output_pipeline_projection(
    state: &LocalOperatorShellState,
) -> LocalProviderOutputPipelineProjection {
    let bridge_result = project_invocation_output_into_provider_pipeline(state);
    if state.constrained_local_provider_invocation.status
        == ConstrainedLocalProviderInvocationStatus::NotInvoked
    {
        return initial_local_provider_output_pipeline_projection();
    }
    if let Err(errors) = bridge_result.clone() {
        let reason = errors
            .first()
            .copied()
            .unwrap_or(LocalProviderOutputPipelineError::NoInvocationOutput);
        return LocalProviderOutputPipelineProjection {
            status: LocalProviderOutputPipelineValidationStatus::Rejected,
            source_kind: Some(LocalProviderOutputPipelineSourceKind::ConstrainedLocalProviderInvocation),
            source_invocation_result_id: state.constrained_local_provider_invocation.result.as_ref().map(|result| result.result_id.clone()),
            provider_execution_result_id: None,
            current_stage: Some(LocalProviderOutputPipelineStage::InvocationOutputProjected),
            next_required_stage: Some(LocalProviderOutputPipelineStage::InvocationOutputProjected),
            stages: local_provider_output_pipeline_stage_order()
                .into_iter()
                .map(|stage| {
                    let status = if stage == LocalProviderOutputPipelineStage::InvocationOutputProjected {
                        LocalProviderOutputPipelineStageStatus::Rejected
                    } else {
                        LocalProviderOutputPipelineStageStatus::Blocked
                    };
                    pipeline_stage(stage, status, Some(reason))
                })
                .collect(),
            provider_output_validation_status: state.provider_output_validation.status,
            provider_output_review_status: state.provider_output_validation.reviewability_status,
            staged_proposal_status: state.staged_candidate_conversion_proposal.status,
            staged_proposal_validation_status: state.staged_candidate_conversion_validation.status,
            candidate_review_status: "blocked".to_string(),
            operator_decision_status: state.operator_candidate_decision.status,
            boundary_statuses: local_provider_output_pipeline_boundary_statuses(),
            effect_statuses: local_provider_output_pipeline_effect_statuses(),
            errors,
            bridge: None,
            note: "Provider output pipeline integration rejected fail-closed; no candidate output is created.".to_string(),
        };
    }
    let bridge = bridge_result.unwrap();
    let mut errors = Vec::new();
    let validation_completed = state.provider_output_validation.status
        == LocalProviderOutputValidationStatus::ReviewableUntrusted;
    let validation_rejected = matches!(
        state.provider_output_validation.status,
        LocalProviderOutputValidationStatus::Rejected
            | LocalProviderOutputValidationStatus::ValidationNotApplicable
            | LocalProviderOutputValidationStatus::InvalidValidationInput
    );
    let review_completed = state.provider_output_validation.reviewability_status
        == LocalProviderOutputReviewabilityStatus::ReviewableUntrusted;
    let proposal_completed = state.staged_candidate_conversion_proposal.status
        == StagedCandidateConversionProposalStatus::StagedProposalCreated;
    let staged_validation_completed = state.staged_candidate_conversion_validation.status
        == StagedCandidateConversionValidationStatus::StagedProposalShapeValid;
    let staged_validation_rejected = matches!(
        state.staged_candidate_conversion_validation.status,
        StagedCandidateConversionValidationStatus::RejectedStagedProposal
            | StagedCandidateConversionValidationStatus::InvalidValidationInput
    );
    let candidate_review_completed = staged_validation_completed;
    let operator_decision_completed = matches!(
        state.operator_candidate_decision.status,
        OperatorCandidateDecisionStatus::ApprovedValidatedStagedProposal
            | OperatorCandidateDecisionStatus::RejectedValidatedStagedProposal
    );

    let mut next_required_stage = None;
    let mut stages = Vec::new();
    stages.push(pipeline_stage(
        LocalProviderOutputPipelineStage::InvocationOutputProjected,
        LocalProviderOutputPipelineStageStatus::Completed,
        None,
    ));
    stages.push(pipeline_stage(
        LocalProviderOutputPipelineStage::ProviderExecutionResultProjected,
        LocalProviderOutputPipelineStageStatus::Completed,
        None,
    ));
    stages.push(pipeline_stage(
        LocalProviderOutputPipelineStage::ProviderOutputValidationRequired,
        LocalProviderOutputPipelineStageStatus::Completed,
        None,
    ));
    if validation_rejected {
        errors.push(LocalProviderOutputPipelineError::ProviderOutputValidationRejected);
        stages.push(pipeline_stage(
            LocalProviderOutputPipelineStage::ProviderOutputValidationProjected,
            LocalProviderOutputPipelineStageStatus::Rejected,
            Some(LocalProviderOutputPipelineError::ProviderOutputValidationRejected),
        ));
        for stage in local_provider_output_pipeline_stage_order()
            .into_iter()
            .skip(4)
        {
            stages.push(pipeline_stage(
                stage,
                LocalProviderOutputPipelineStageStatus::Blocked,
                Some(LocalProviderOutputPipelineError::ProviderOutputValidationRejected),
            ));
        }
        next_required_stage =
            Some(LocalProviderOutputPipelineStage::ProviderOutputValidationProjected);
    } else if !validation_completed {
        errors.push(LocalProviderOutputPipelineError::ProviderOutputValidationMissing);
        stages.push(pipeline_stage(
            LocalProviderOutputPipelineStage::ProviderOutputValidationProjected,
            LocalProviderOutputPipelineStageStatus::Available,
            Some(LocalProviderOutputPipelineError::ProviderOutputValidationMissing),
        ));
        for stage in local_provider_output_pipeline_stage_order()
            .into_iter()
            .skip(4)
        {
            stages.push(pipeline_stage(
                stage,
                LocalProviderOutputPipelineStageStatus::Blocked,
                Some(LocalProviderOutputPipelineError::ProviderOutputValidationMissing),
            ));
        }
        next_required_stage =
            Some(LocalProviderOutputPipelineStage::ProviderOutputValidationProjected);
    } else {
        stages.push(pipeline_stage(
            LocalProviderOutputPipelineStage::ProviderOutputValidationProjected,
            LocalProviderOutputPipelineStageStatus::Completed,
            None,
        ));
        stages.push(pipeline_stage(
            LocalProviderOutputPipelineStage::ProviderOutputReviewRequired,
            LocalProviderOutputPipelineStageStatus::Completed,
            None,
        ));
        if !review_completed {
            errors.push(LocalProviderOutputPipelineError::ProviderOutputReviewMissing);
            stages.push(pipeline_stage(
                LocalProviderOutputPipelineStage::ProviderOutputReviewProjected,
                LocalProviderOutputPipelineStageStatus::Available,
                Some(LocalProviderOutputPipelineError::ProviderOutputReviewMissing),
            ));
            for stage in local_provider_output_pipeline_stage_order()
                .into_iter()
                .skip(6)
            {
                stages.push(pipeline_stage(
                    stage,
                    LocalProviderOutputPipelineStageStatus::Blocked,
                    Some(LocalProviderOutputPipelineError::ProviderOutputReviewMissing),
                ));
            }
            next_required_stage =
                Some(LocalProviderOutputPipelineStage::ProviderOutputReviewProjected);
        } else {
            stages.push(pipeline_stage(
                LocalProviderOutputPipelineStage::ProviderOutputReviewProjected,
                LocalProviderOutputPipelineStageStatus::Completed,
                None,
            ));
            stages.push(pipeline_stage(
                LocalProviderOutputPipelineStage::StagedProposalRequired,
                LocalProviderOutputPipelineStageStatus::Completed,
                None,
            ));
            if !proposal_completed {
                errors.push(LocalProviderOutputPipelineError::StagedProposalMissing);
                stages.push(pipeline_stage(
                    LocalProviderOutputPipelineStage::StagedProposalProjected,
                    LocalProviderOutputPipelineStageStatus::Available,
                    Some(LocalProviderOutputPipelineError::StagedProposalMissing),
                ));
                for stage in local_provider_output_pipeline_stage_order()
                    .into_iter()
                    .skip(8)
                {
                    stages.push(pipeline_stage(
                        stage,
                        LocalProviderOutputPipelineStageStatus::Blocked,
                        Some(LocalProviderOutputPipelineError::StagedProposalMissing),
                    ));
                }
                next_required_stage =
                    Some(LocalProviderOutputPipelineStage::StagedProposalProjected);
            } else {
                stages.push(pipeline_stage(
                    LocalProviderOutputPipelineStage::StagedProposalProjected,
                    LocalProviderOutputPipelineStageStatus::Completed,
                    None,
                ));
                stages.push(pipeline_stage(
                    LocalProviderOutputPipelineStage::StagedProposalValidationRequired,
                    LocalProviderOutputPipelineStageStatus::Completed,
                    None,
                ));
                if staged_validation_rejected {
                    errors.push(LocalProviderOutputPipelineError::StagedProposalValidationRejected);
                    stages.push(pipeline_stage(
                        LocalProviderOutputPipelineStage::StagedProposalValidationProjected,
                        LocalProviderOutputPipelineStageStatus::Rejected,
                        Some(LocalProviderOutputPipelineError::StagedProposalValidationRejected),
                    ));
                    for stage in local_provider_output_pipeline_stage_order()
                        .into_iter()
                        .skip(10)
                    {
                        stages.push(pipeline_stage(
                            stage,
                            LocalProviderOutputPipelineStageStatus::Blocked,
                            Some(
                                LocalProviderOutputPipelineError::StagedProposalValidationRejected,
                            ),
                        ));
                    }
                    next_required_stage =
                        Some(LocalProviderOutputPipelineStage::StagedProposalValidationProjected);
                } else if !staged_validation_completed {
                    errors.push(LocalProviderOutputPipelineError::StagedProposalValidationMissing);
                    stages.push(pipeline_stage(
                        LocalProviderOutputPipelineStage::StagedProposalValidationProjected,
                        LocalProviderOutputPipelineStageStatus::Available,
                        Some(LocalProviderOutputPipelineError::StagedProposalValidationMissing),
                    ));
                    for stage in local_provider_output_pipeline_stage_order()
                        .into_iter()
                        .skip(10)
                    {
                        stages.push(pipeline_stage(
                            stage,
                            LocalProviderOutputPipelineStageStatus::Blocked,
                            Some(LocalProviderOutputPipelineError::StagedProposalValidationMissing),
                        ));
                    }
                    next_required_stage =
                        Some(LocalProviderOutputPipelineStage::StagedProposalValidationProjected);
                } else {
                    stages.push(pipeline_stage(
                        LocalProviderOutputPipelineStage::StagedProposalValidationProjected,
                        LocalProviderOutputPipelineStageStatus::Completed,
                        None,
                    ));
                    stages.push(pipeline_stage(
                        LocalProviderOutputPipelineStage::CandidateReviewRequired,
                        LocalProviderOutputPipelineStageStatus::Completed,
                        None,
                    ));
                    if !candidate_review_completed {
                        errors.push(LocalProviderOutputPipelineError::CandidateReviewMissing);
                        stages.push(pipeline_stage(
                            LocalProviderOutputPipelineStage::CandidateReviewProjected,
                            LocalProviderOutputPipelineStageStatus::Available,
                            Some(LocalProviderOutputPipelineError::CandidateReviewMissing),
                        ));
                    } else {
                        stages.push(pipeline_stage(
                            LocalProviderOutputPipelineStage::CandidateReviewProjected,
                            LocalProviderOutputPipelineStageStatus::Completed,
                            None,
                        ));
                    }
                    stages.push(pipeline_stage(
                        LocalProviderOutputPipelineStage::OperatorDecisionRequired,
                        LocalProviderOutputPipelineStageStatus::Completed,
                        None,
                    ));
                    if !operator_decision_completed {
                        errors.push(LocalProviderOutputPipelineError::OperatorDecisionMissing);
                        stages.push(pipeline_stage(
                            LocalProviderOutputPipelineStage::OperatorDecisionProjected,
                            LocalProviderOutputPipelineStageStatus::Available,
                            Some(LocalProviderOutputPipelineError::OperatorDecisionMissing),
                        ));
                        next_required_stage =
                            Some(LocalProviderOutputPipelineStage::OperatorDecisionProjected);
                    } else {
                        stages.push(pipeline_stage(
                            LocalProviderOutputPipelineStage::OperatorDecisionProjected,
                            LocalProviderOutputPipelineStageStatus::Completed,
                            None,
                        ));
                    }
                }
            }
        }
    }

    let current_stage = stages
        .iter()
        .rev()
        .find(|stage| stage.status == LocalProviderOutputPipelineStageStatus::Completed)
        .map(|stage| stage.stage);
    if next_required_stage.is_none() {
        next_required_stage = stages
            .iter()
            .find(|stage| {
                matches!(
                    stage.status,
                    LocalProviderOutputPipelineStageStatus::Available
                        | LocalProviderOutputPipelineStageStatus::Rejected
                        | LocalProviderOutputPipelineStageStatus::Blocked
                )
            })
            .map(|stage| stage.stage);
    }
    let status = if errors.iter().any(|error| {
        matches!(
            error,
            LocalProviderOutputPipelineError::ProviderOutputValidationRejected
                | LocalProviderOutputPipelineError::StagedProposalValidationRejected
        )
    }) {
        LocalProviderOutputPipelineValidationStatus::Rejected
    } else if errors.is_empty() {
        LocalProviderOutputPipelineValidationStatus::Valid
    } else {
        LocalProviderOutputPipelineValidationStatus::Blocked
    };

    LocalProviderOutputPipelineProjection {
        status,
        source_kind: Some(LocalProviderOutputPipelineSourceKind::ConstrainedLocalProviderInvocation),
        source_invocation_result_id: Some(bridge.invocation_result_id.clone()),
        provider_execution_result_id: Some(bridge.provider_execution_result_id.clone()),
        current_stage,
        next_required_stage,
        stages,
        provider_output_validation_status: state.provider_output_validation.status,
        provider_output_review_status: state.provider_output_validation.reviewability_status,
        staged_proposal_status: state.staged_candidate_conversion_proposal.status,
        staged_proposal_validation_status: state.staged_candidate_conversion_validation.status,
        candidate_review_status: if staged_validation_completed { "display_only".to_string() } else { "required".to_string() },
        operator_decision_status: state.operator_candidate_decision.status,
        boundary_statuses: local_provider_output_pipeline_boundary_statuses(),
        effect_statuses: local_provider_output_pipeline_effect_statuses(),
        errors,
        bridge: Some(bridge),
        note: "Invocation output remains untrusted and descriptive. Pipeline integration does not create candidate output. Validation, review, staging, staged validation, candidate review, and operator decision boundaries cannot be skipped. Candidate materialization remains a later bounded step. Provider trust, readiness, release, deployment, and public-use approval are not granted.".to_string(),
    }
}

pub fn validate_provider_output_pipeline_stage_order(
    projection: &LocalProviderOutputPipelineProjection,
) -> Result<(), LocalProviderOutputPipelineError> {
    let expected = local_provider_output_pipeline_stage_order();
    let actual: Vec<_> = projection.stages.iter().map(|stage| stage.stage).collect();
    if actual != expected {
        return Err(LocalProviderOutputPipelineError::PipelineSkipAttemptRejected);
    }
    let mut seen_incomplete = false;
    for stage in &projection.stages {
        if seen_incomplete && stage.status == LocalProviderOutputPipelineStageStatus::Completed {
            return Err(LocalProviderOutputPipelineError::PipelineSkipAttemptRejected);
        }
        if stage.status != LocalProviderOutputPipelineStageStatus::Completed {
            seen_incomplete = true;
        }
    }
    Ok(())
}

pub fn reject_provider_output_pipeline_integration(
    error: LocalProviderOutputPipelineError,
) -> LocalProviderOutputPipelineProjection {
    let mut projection = initial_local_provider_output_pipeline_projection();
    projection.status = LocalProviderOutputPipelineValidationStatus::Rejected;
    projection.errors = vec![error];
    projection.stages = local_provider_output_pipeline_stage_order()
        .into_iter()
        .map(|stage| {
            pipeline_stage(
                stage,
                LocalProviderOutputPipelineStageStatus::Blocked,
                Some(error),
            )
        })
        .collect();
    projection.note = "Provider output pipeline integration rejected fail-closed.".to_string();
    projection
}

pub fn constrained_local_provider_invocation_boundary_statuses(
) -> Vec<ConstrainedLocalProviderInvocationBoundaryStatus> {
    vec![
        ConstrainedLocalProviderInvocationBoundaryStatus::ConstrainedLocalInvocationOnly,
        ConstrainedLocalProviderInvocationBoundaryStatus::AllowlistedProviderOnly,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoArbitraryCommand,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoShell,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoNetwork,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoCloud,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoSecrets,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoProviderTrust,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoCandidateMaterialization,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoActionExecution,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoProductionPersistence,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoReadinessEffect,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoReleaseEffect,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoDeploymentEffect,
        ConstrainedLocalProviderInvocationBoundaryStatus::NoPublicUseEffect,
    ]
}

pub fn constrained_local_provider_invocation_effect_statuses(
) -> Vec<ConstrainedLocalProviderInvocationEffectStatus> {
    vec![
        ConstrainedLocalProviderInvocationEffectStatus::NoProviderTrust,
        ConstrainedLocalProviderInvocationEffectStatus::NoCandidateMaterialization,
        ConstrainedLocalProviderInvocationEffectStatus::NoActionExecution,
        ConstrainedLocalProviderInvocationEffectStatus::NoReadinessEffect,
        ConstrainedLocalProviderInvocationEffectStatus::NoReleaseEffect,
        ConstrainedLocalProviderInvocationEffectStatus::NoDeploymentEffect,
        ConstrainedLocalProviderInvocationEffectStatus::NoPublicUseEffect,
    ]
}

pub fn initial_constrained_local_provider_invocation_projection(
) -> ConstrainedLocalProviderInvocationProjection {
    ConstrainedLocalProviderInvocationProjection {
        status: ConstrainedLocalProviderInvocationStatus::NotInvoked,
        provider_kind: None,
        adapter_kind: None,
        adapter_declaration_id: None,
        result: None,
        error_codes: Vec::new(),
        boundary_statuses: constrained_local_provider_invocation_boundary_statuses(),
        output_trust_status: ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive,
        effect_statuses: constrained_local_provider_invocation_effect_statuses(),
        capability_surface: constrained_local_provider_invocation_capability_surface(),
        registry_declaration_count: 0,
        reason: "constrained local provider invocation not_invoked; accepted deterministic_fake_adapter declaration and allowlisted_local_deterministic_provider are required for Phase 156 invocation".to_string(),
    }
}

fn reject_constrained_local_provider_invocation(
    status: ConstrainedLocalProviderInvocationStatus,
    provider_kind: Option<AllowlistedLocalProviderKind>,
    adapter_kind: Option<LocalProviderAdapterKind>,
    adapter_declaration_id: Option<String>,
    registry_declaration_count: usize,
    errors: Vec<ConstrainedLocalProviderInvocationError>,
) -> ConstrainedLocalProviderInvocationProjection {
    ConstrainedLocalProviderInvocationProjection {
        status,
        provider_kind,
        adapter_kind,
        adapter_declaration_id,
        result: None,
        error_codes: errors,
        boundary_statuses: constrained_local_provider_invocation_boundary_statuses(),
        output_trust_status: ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive,
        effect_statuses: constrained_local_provider_invocation_effect_statuses(),
        capability_surface: constrained_local_provider_invocation_capability_surface(),
        registry_declaration_count,
        reason: "constrained local provider invocation rejected fail-closed; prior accepted invocation projection is preserved when present and no provider trust, candidate, action, readiness, release, deployment, public-use, command, shell, network, cloud, secret, environment, or persistence effect occurs".to_string(),
    }
}

pub fn validate_constrained_local_provider_invocation_request(
    registry: &LocalProviderAdapterRegistry,
    request: &ConstrainedLocalProviderInvocationRequest,
) -> Result<LocalProviderAdapterDeclaration, Box<ConstrainedLocalProviderInvocationProjection>> {
    let mut errors = std::collections::BTreeSet::new();
    match request.provider_kind {
        AllowlistedLocalProviderKind::AllowlistedLocalDeterministicProvider => {}
        AllowlistedLocalProviderKind::UnsupportedCloudProvider => {
            errors.insert(ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted);
            errors.insert(ConstrainedLocalProviderInvocationError::NetworkFieldRejected);
        }
        AllowlistedLocalProviderKind::UnsupportedNetworkProvider => {
            errors.insert(ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted);
            errors.insert(ConstrainedLocalProviderInvocationError::NetworkFieldRejected);
        }
        AllowlistedLocalProviderKind::UnsupportedShellProvider => {
            errors.insert(ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted);
            errors.insert(ConstrainedLocalProviderInvocationError::ShellFieldRejected);
        }
        AllowlistedLocalProviderKind::UnsupportedLocalProvider => {
            errors.insert(ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted);
        }
    }

    let declaration = match registry.declarations.last() {
        Some(declaration) => declaration,
        None => {
            errors.insert(ConstrainedLocalProviderInvocationError::NoAdapterDeclared);
            let error_codes: Vec<_> = errors.into_iter().collect();
            return Err(Box::new(reject_constrained_local_provider_invocation(
                ConstrainedLocalProviderInvocationStatus::AllowlistedProviderRequired,
                Some(request.provider_kind),
                None,
                None,
                registry.declarations.len(),
                error_codes,
            )));
        }
    };

    if declaration.status != LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting {
        errors.insert(ConstrainedLocalProviderInvocationError::AdapterNotAccepted);
    }
    if declaration.adapter_kind != LocalProviderAdapterKind::DeterministicFakeAdapter {
        errors.insert(ConstrainedLocalProviderInvocationError::AdapterNotAccepted);
    }

    for (key, value) in &request.fields {
        reject_forbidden_constrained_local_provider_invocation_field(key, value, &mut errors);
    }
    if !request.input_summary.trim().is_empty() {
        reject_forbidden_constrained_local_provider_invocation_field(
            "input_summary",
            &request.input_summary,
            &mut errors,
        );
    }

    let error_codes: Vec<_> = errors.into_iter().collect();
    if error_codes.is_empty() {
        Ok(declaration.clone())
    } else {
        let status = if error_codes.iter().any(|error| {
            matches!(
                error,
                ConstrainedLocalProviderInvocationError::ProviderNotAllowlisted
                    | ConstrainedLocalProviderInvocationError::NetworkFieldRejected
                    | ConstrainedLocalProviderInvocationError::ShellFieldRejected
                    | ConstrainedLocalProviderInvocationError::AdapterNotAccepted
            )
        }) {
            ConstrainedLocalProviderInvocationStatus::UnsupportedProvider
        } else {
            ConstrainedLocalProviderInvocationStatus::InvalidInvocationRequest
        };
        Err(Box::new(reject_constrained_local_provider_invocation(
            status,
            Some(request.provider_kind),
            Some(declaration.adapter_kind),
            Some(declaration.declaration_id.clone()),
            registry.declarations.len(),
            error_codes,
        )))
    }
}

pub fn reject_forbidden_constrained_local_provider_invocation_field(
    key: &str,
    value: &str,
    errors: &mut std::collections::BTreeSet<ConstrainedLocalProviderInvocationError>,
) {
    let lowered_key = key.to_ascii_lowercase();
    let combined = format!("{}={}", lowered_key, value.to_ascii_lowercase());
    let safe_input =
        lowered_key == "label" || lowered_key == "description" || lowered_key == "input_summary";
    let forbidden = [
        "endpoint",
        "url",
        "host",
        "port",
        "http",
        "network",
        "cloud",
        "command",
        "shell",
        "process",
        "args",
        "path",
        "file",
        "directory",
        "secret",
        "token",
        "api_key",
        "apikey",
        "credential",
        "trust",
        "approved_output",
        "provider_output_approval",
        "readiness",
        "ready",
        "release",
        "deployment",
        "deploy",
        "public_use",
        "public-use",
        "candidate",
        "materialization",
        "action",
        "persistence",
    ]
    .iter()
    .any(|needle| combined.contains(needle));
    if safe_input && !forbidden {
        return;
    }
    if combined.contains("command") {
        errors.insert(ConstrainedLocalProviderInvocationError::ArbitraryCommandRejected);
    } else if combined.contains("shell") {
        errors.insert(ConstrainedLocalProviderInvocationError::ShellFieldRejected);
    } else if combined.contains("process") {
        errors.insert(ConstrainedLocalProviderInvocationError::ProcessFieldRejected);
    } else if combined.contains("args") {
        errors.insert(ConstrainedLocalProviderInvocationError::ArgsFieldRejected);
    } else if ["endpoint", "url", "host", "port", "http"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::EndpointFieldRejected);
    } else if ["network", "cloud"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::NetworkFieldRejected);
    } else if ["secret", "token", "api_key", "apikey", "credential"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::SecretFieldRejected);
    } else if ["path", "file", "directory"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::PathFieldRejected);
    } else if combined.contains("approved_output") || combined.contains("provider_output_approval")
    {
        errors.insert(ConstrainedLocalProviderInvocationError::ProviderOutputApprovalClaimRejected);
    } else if combined.contains("trust") {
        errors.insert(ConstrainedLocalProviderInvocationError::TrustClaimRejected);
    } else if ["readiness", "ready"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::ReadinessClaimRejected);
    } else if combined.contains("release") {
        errors.insert(ConstrainedLocalProviderInvocationError::ReleaseClaimRejected);
    } else if ["deployment", "deploy"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::DeploymentClaimRejected);
    } else if ["public_use", "public-use"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors.insert(ConstrainedLocalProviderInvocationError::PublicUseClaimRejected);
    } else if ["candidate", "materialization"]
        .iter()
        .any(|needle| combined.contains(needle))
    {
        errors
            .insert(ConstrainedLocalProviderInvocationError::CandidateMaterializationClaimRejected);
    } else if combined.contains("action") {
        errors.insert(ConstrainedLocalProviderInvocationError::ActionClaimRejected);
    } else if combined.contains("persistence") {
        errors.insert(ConstrainedLocalProviderInvocationError::PersistenceClaimRejected);
    }
}

fn deterministic_constrained_local_provider_invocation_checksum(input: &str) -> u64 {
    input.bytes().fold(156_u64, |acc, byte| {
        acc.wrapping_mul(33).wrapping_add(byte as u64)
    })
}

pub fn execute_allowlisted_local_deterministic_provider_invocation(
    declaration: &LocalProviderAdapterDeclaration,
    request: &ConstrainedLocalProviderInvocationRequest,
) -> ConstrainedLocalProviderInvocationResult {
    let checksum = deterministic_constrained_local_provider_invocation_checksum(&format!(
        "{}|{}|{}|{}",
        declaration.declaration_id,
        declaration.adapter_kind.code(),
        request.provider_kind.code(),
        request.input_summary
    ));
    ConstrainedLocalProviderInvocationResult {
        result_id: format!("constrained-local-provider-invocation-{checksum:016x}"),
        provider_kind: request.provider_kind,
        adapter_kind: declaration.adapter_kind,
        adapter_declaration_id: declaration.declaration_id.clone(),
        output_summary: format!(
            "allowlisted_local_deterministic_provider descriptive output for input_bytes={} checksum={checksum:016x}",
            request.input_summary.len()
        ),
        output_trust_status: ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive,
        boundary_statuses: constrained_local_provider_invocation_boundary_statuses(),
        effect_statuses: constrained_local_provider_invocation_effect_statuses(),
    }
}

pub fn project_constrained_local_provider_invocation(
    registry: &LocalProviderAdapterRegistry,
    result: ConstrainedLocalProviderInvocationResult,
) -> ConstrainedLocalProviderInvocationProjection {
    ConstrainedLocalProviderInvocationProjection {
        status: ConstrainedLocalProviderInvocationStatus::InvocationExecuted,
        provider_kind: Some(result.provider_kind),
        adapter_kind: Some(result.adapter_kind),
        adapter_declaration_id: Some(result.adapter_declaration_id.clone()),
        result: Some(result),
        error_codes: Vec::new(),
        boundary_statuses: constrained_local_provider_invocation_boundary_statuses(),
        output_trust_status: ConstrainedLocalProviderInvocationTrustStatus::UntrustedDescriptive,
        effect_statuses: constrained_local_provider_invocation_effect_statuses(),
        capability_surface: constrained_local_provider_invocation_capability_surface(),
        registry_declaration_count: registry.declarations.len(),
        reason: "constrained local provider invocation executed through exactly one allowlisted local provider path; output remains untrusted_descriptive and no provider trust, candidate, action, readiness, release, deployment, public-use, command, shell, network, cloud, secret, environment, or persistence effect occurs".to_string(),
    }
}

pub fn execute_constrained_local_provider_invocation(
    state: &LocalOperatorShellState,
    request: ConstrainedLocalProviderInvocationRequest,
) -> Result<LocalOperatorShellState, Box<ConstrainedLocalProviderInvocationProjection>> {
    let declaration = validate_constrained_local_provider_invocation_request(
        &state.local_provider_adapter_registry,
        &request,
    )?;
    let result =
        execute_allowlisted_local_deterministic_provider_invocation(&declaration, &request);
    let mut next = state.clone();
    next.constrained_local_provider_invocation = project_constrained_local_provider_invocation(
        &state.local_provider_adapter_registry,
        result,
    );
    let bridge = project_invocation_output_into_provider_pipeline(&next).map_err(|errors| {
        Box::new(reject_constrained_local_provider_invocation(
            ConstrainedLocalProviderInvocationStatus::InvocationRejected,
            next.constrained_local_provider_invocation.provider_kind,
            next.constrained_local_provider_invocation.adapter_kind,
            next.constrained_local_provider_invocation
                .adapter_declaration_id
                .clone(),
            next.local_provider_adapter_registry.declarations.len(),
            errors
                .into_iter()
                .map(|_| {
                    ConstrainedLocalProviderInvocationError::ProviderOutputApprovalClaimRejected
                })
                .collect(),
        ))
    })?;
    next.provider_execution = provider_execution_projection_from_invocation_bridge(&next, &bridge);
    next.provider_output_validation = validate_local_provider_output(&next.provider_execution);
    next.local_provider_output_pipeline = derive_local_provider_output_pipeline_projection(&next);
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
    let valid_output_prefix = result
        .output_summary
        .starts_with("deterministic_stub descriptive output for input_bytes=")
        || result.output_summary.starts_with(
            "allowlisted_local_deterministic_provider descriptive output for input_bytes=",
        );
    if !valid_output_prefix
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
    next.local_provider_output_pipeline = derive_local_provider_output_pipeline_projection(&next);
    next
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorCandidateDecisionKind {
    ApproveValidatedStagedProposal,
    RejectValidatedStagedProposal,
}

impl OperatorCandidateDecisionKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ApproveValidatedStagedProposal => "approve_validated_staged_proposal",
            Self::RejectValidatedStagedProposal => "reject_validated_staged_proposal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorCandidateDecisionStatus {
    NoOperatorDecision,
    ApprovedValidatedStagedProposal,
    RejectedValidatedStagedProposal,
    RejectedOperatorDecisionRequest,
    InvalidOperatorDecisionInput,
}

impl OperatorCandidateDecisionStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoOperatorDecision => "no_operator_decision",
            Self::ApprovedValidatedStagedProposal => "approved_validated_staged_proposal",
            Self::RejectedValidatedStagedProposal => "rejected_validated_staged_proposal",
            Self::RejectedOperatorDecisionRequest => "rejected_operator_decision_request",
            Self::InvalidOperatorDecisionInput => "invalid_operator_decision_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorCandidateDecisionError {
    NoStagedProposal,
    StagedProposalNotValidated,
    StagedProposalValidationRejected,
    InvalidValidationInput,
    SourceLinkageInconsistent,
    TrustClaimRejected,
    ProviderOutputApprovalClaimRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    ActionClaimRejected,
    ExecutionClaimRejected,
    PersistenceClaimRejected,
    CandidateCreationClaimRejected,
    CandidateMaterializationClaimRejected,
}

impl OperatorCandidateDecisionError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoStagedProposal => "no_staged_proposal",
            Self::StagedProposalNotValidated => "staged_proposal_not_validated",
            Self::StagedProposalValidationRejected => "staged_proposal_validation_rejected",
            Self::InvalidValidationInput => "invalid_validation_input",
            Self::SourceLinkageInconsistent => "source_linkage_inconsistent",
            Self::TrustClaimRejected => "trust_claim_rejected",
            Self::ProviderOutputApprovalClaimRejected => "provider_output_approval_claim_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::ActionClaimRejected => "action_claim_rejected",
            Self::ExecutionClaimRejected => "execution_claim_rejected",
            Self::PersistenceClaimRejected => "persistence_claim_rejected",
            Self::CandidateCreationClaimRejected => "candidate_creation_claim_rejected",
            Self::CandidateMaterializationClaimRejected => {
                "candidate_materialization_claim_rejected"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorCandidateDecisionRequest {
    pub kind: OperatorCandidateDecisionKind,
    pub staged_proposal_id: String,
    pub provider_execution_result_id: String,
    pub staged_proposal_validation_status: StagedCandidateConversionValidationStatus,
    pub claims_trust: bool,
    pub claims_provider_output_approval: bool,
    pub claims_readiness: bool,
    pub claims_release: bool,
    pub claims_deployment: bool,
    pub claims_public_use: bool,
    pub claims_action: bool,
    pub claims_execution: bool,
    pub claims_persistence: bool,
    pub claims_candidate_creation: bool,
    pub claims_candidate_materialization: bool,
}

impl OperatorCandidateDecisionRequest {
    pub fn approve(
        staged_proposal_id: impl Into<String>,
        provider_execution_result_id: impl Into<String>,
    ) -> Self {
        Self::new(
            OperatorCandidateDecisionKind::ApproveValidatedStagedProposal,
            staged_proposal_id,
            provider_execution_result_id,
        )
    }

    pub fn reject(
        staged_proposal_id: impl Into<String>,
        provider_execution_result_id: impl Into<String>,
    ) -> Self {
        Self::new(
            OperatorCandidateDecisionKind::RejectValidatedStagedProposal,
            staged_proposal_id,
            provider_execution_result_id,
        )
    }

    fn new(
        kind: OperatorCandidateDecisionKind,
        staged_proposal_id: impl Into<String>,
        provider_execution_result_id: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            staged_proposal_id: staged_proposal_id.into(),
            provider_execution_result_id: provider_execution_result_id.into(),
            staged_proposal_validation_status:
                StagedCandidateConversionValidationStatus::StagedProposalShapeValid,
            claims_trust: false,
            claims_provider_output_approval: false,
            claims_readiness: false,
            claims_release: false,
            claims_deployment: false,
            claims_public_use: false,
            claims_action: false,
            claims_execution: false,
            claims_persistence: false,
            claims_candidate_creation: false,
            claims_candidate_materialization: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorCandidateDecisionRecord {
    pub decision_id: String,
    pub decision_kind: OperatorCandidateDecisionKind,
    pub staged_proposal_id: String,
    pub provider_execution_result_id: String,
    pub staged_proposal_validation_status: StagedCandidateConversionValidationStatus,
    pub decision_scope: String,
    pub materialization_status: String,
    pub trust_status: String,
    pub readiness_status: String,
    pub release_status: String,
    pub deployment_status: String,
    pub public_use_status: String,
    pub action_status: String,
    pub persistence_status: String,
    pub replay_repair_status: String,
    pub recovery_promotion_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorCandidateDecisionProjection {
    pub status: OperatorCandidateDecisionStatus,
    pub record: Option<OperatorCandidateDecisionRecord>,
    pub error: Option<OperatorCandidateDecisionError>,
    pub note: String,
}

pub fn initial_operator_candidate_decision_projection() -> OperatorCandidateDecisionProjection {
    OperatorCandidateDecisionProjection {
        status: OperatorCandidateDecisionStatus::NoOperatorDecision,
        record: None,
        error: None,
        note: "No operator candidate decision has been recorded. Decision applies only to validated staged proposal when present; no candidate output is created in Phase 149.".to_string(),
    }
}

fn deterministic_operator_candidate_decision_id(
    kind: OperatorCandidateDecisionKind,
    staged_proposal_id: &str,
    provider_execution_result_id: &str,
    validation_status: StagedCandidateConversionValidationStatus,
) -> String {
    let input = format!(
        "{}|{}|{}|{}|phase_149",
        kind.code(),
        staged_proposal_id,
        provider_execution_result_id,
        validation_status.code()
    );
    let mut accumulator: u32 = 2_166_136_261;
    for byte in input.as_bytes() {
        accumulator ^= u32::from(*byte);
        accumulator = accumulator.wrapping_mul(16_777_619);
    }
    format!("operator-candidate-decision-{accumulator:08x}")
}

pub fn validate_operator_candidate_decision_request(
    state: &LocalOperatorShellState,
    request: &OperatorCandidateDecisionRequest,
) -> Result<(), OperatorCandidateDecisionError> {
    if request.claims_trust {
        return Err(OperatorCandidateDecisionError::TrustClaimRejected);
    }
    if request.claims_provider_output_approval {
        return Err(OperatorCandidateDecisionError::ProviderOutputApprovalClaimRejected);
    }
    if request.claims_readiness {
        return Err(OperatorCandidateDecisionError::ReadinessClaimRejected);
    }
    if request.claims_release {
        return Err(OperatorCandidateDecisionError::ReleaseClaimRejected);
    }
    if request.claims_deployment {
        return Err(OperatorCandidateDecisionError::DeploymentClaimRejected);
    }
    if request.claims_public_use {
        return Err(OperatorCandidateDecisionError::PublicUseClaimRejected);
    }
    if request.claims_action {
        return Err(OperatorCandidateDecisionError::ActionClaimRejected);
    }
    if request.claims_execution {
        return Err(OperatorCandidateDecisionError::ExecutionClaimRejected);
    }
    if request.claims_persistence {
        return Err(OperatorCandidateDecisionError::PersistenceClaimRejected);
    }
    if request.claims_candidate_creation {
        return Err(OperatorCandidateDecisionError::CandidateCreationClaimRejected);
    }
    if request.claims_candidate_materialization {
        return Err(OperatorCandidateDecisionError::CandidateMaterializationClaimRejected);
    }
    let Some(proposal) = state.staged_candidate_conversion_proposal.proposal.as_ref() else {
        return Err(OperatorCandidateDecisionError::NoStagedProposal);
    };
    match state.staged_candidate_conversion_validation.status {
        StagedCandidateConversionValidationStatus::NotValidated => {
            return Err(OperatorCandidateDecisionError::StagedProposalNotValidated)
        }
        StagedCandidateConversionValidationStatus::RejectedStagedProposal => {
            return Err(OperatorCandidateDecisionError::StagedProposalValidationRejected)
        }
        StagedCandidateConversionValidationStatus::InvalidValidationInput => {
            return Err(OperatorCandidateDecisionError::InvalidValidationInput)
        }
        StagedCandidateConversionValidationStatus::StagedProposalShapeValid => {}
    }
    if request.staged_proposal_validation_status
        != StagedCandidateConversionValidationStatus::StagedProposalShapeValid
        || request.staged_proposal_id != proposal.proposal_id
        || Some(&request.staged_proposal_id)
            != state
                .staged_candidate_conversion_validation
                .proposal_id
                .as_ref()
        || request.provider_execution_result_id != proposal.source_execution_result_id
        || Some(&request.provider_execution_result_id)
            != state
                .staged_candidate_conversion_validation
                .source_execution_result_id
                .as_ref()
        || state
            .staged_candidate_conversion_validation
            .deterministic_linkage_status
            != "source_linkage_validated"
    {
        return Err(OperatorCandidateDecisionError::SourceLinkageInconsistent);
    }
    let reprojected = project_staged_candidate_conversion_validation(
        state,
        &StagedCandidateConversionValidationRequest {
            proposal_id: Some(proposal.proposal_id.clone()),
        },
    );
    if reprojected != state.staged_candidate_conversion_validation {
        return Err(OperatorCandidateDecisionError::SourceLinkageInconsistent);
    }
    Ok(())
}

pub fn project_operator_candidate_decision(
    request: &OperatorCandidateDecisionRequest,
) -> OperatorCandidateDecisionProjection {
    let status = match request.kind {
        OperatorCandidateDecisionKind::ApproveValidatedStagedProposal => {
            OperatorCandidateDecisionStatus::ApprovedValidatedStagedProposal
        }
        OperatorCandidateDecisionKind::RejectValidatedStagedProposal => {
            OperatorCandidateDecisionStatus::RejectedValidatedStagedProposal
        }
    };
    let record = OperatorCandidateDecisionRecord {
        decision_id: deterministic_operator_candidate_decision_id(
            request.kind,
            &request.staged_proposal_id,
            &request.provider_execution_result_id,
            request.staged_proposal_validation_status,
        ),
        decision_kind: request.kind,
        staged_proposal_id: request.staged_proposal_id.clone(),
        provider_execution_result_id: request.provider_execution_result_id.clone(),
        staged_proposal_validation_status: request.staged_proposal_validation_status,
        decision_scope: "decision_scope_validated_staged_proposal_only".to_string(),
        materialization_status: "candidate_materialization_not_performed".to_string(),
        trust_status: "provider_output_remains_untrusted".to_string(),
        readiness_status: "no_readiness_effect".to_string(),
        release_status: "no_release_effect".to_string(),
        deployment_status: "no_deployment_effect".to_string(),
        public_use_status: "no_public_use_effect".to_string(),
        action_status: "no_action_effect".to_string(),
        persistence_status: "no_persistence_effect".to_string(),
        replay_repair_status: "no_replay_repair_effect".to_string(),
        recovery_promotion_status: "no_recovery_promotion_effect".to_string(),
    };
    OperatorCandidateDecisionProjection {
        status,
        record: Some(record),
        error: None,
        note: "This decision applies only to the validated staged proposal. No candidate output is created in Phase 149. Provider output remains untrusted and not approved. This decision does not approve readiness, release, deployment, or public use.".to_string(),
    }
}

pub fn rejected_operator_candidate_decision_projection(
    error: OperatorCandidateDecisionError,
) -> OperatorCandidateDecisionProjection {
    OperatorCandidateDecisionProjection {
        status: OperatorCandidateDecisionStatus::RejectedOperatorDecisionRequest,
        record: None,
        error: Some(error),
        note: "Operator candidate decision request rejected; authoritative shell state is preserved and no candidate materialization is performed.".to_string(),
    }
}

pub fn submit_operator_candidate_decision(
    state: &LocalOperatorShellState,
    request: OperatorCandidateDecisionRequest,
) -> Result<LocalOperatorShellState, OperatorCandidateDecisionError> {
    validate_operator_candidate_decision_request(state, &request)?;
    let mut next = state.clone();
    next.operator_candidate_decision = project_operator_candidate_decision(&request);
    next.local_provider_output_pipeline = derive_local_provider_output_pipeline_projection(&next);
    next.phase_150_code_production_handoff = derive_phase_150_code_production_handoff(&next);
    Ok(next)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase150CodeProductionHandoff {
    pub handoff_id: String,
    pub status: String,
    pub implemented_capability_evidence: Vec<String>,
    pub remaining_production_grade_gaps: Vec<String>,
    pub remap_recommendations: Vec<String>,
    pub phase_149_roadmap_edit_status: String,
}

pub fn phase_150_implemented_capabilities(state: &LocalOperatorShellState) -> Vec<String> {
    vec![
        format!(
            "provider configuration: {}",
            state.provider_configuration.status.code()
        ),
        format!(
            "deterministic provider execution: {}",
            state.provider_execution.projection_status.code()
        ),
        format!(
            "provider execution result projection: {}",
            state
                .provider_execution
                .result
                .as_ref()
                .map(|r| r.result_id.as_str())
                .unwrap_or("none")
        ),
        format!(
            "provider output validation: {}",
            state.provider_output_validation.status.code()
        ),
        format!(
            "provider output review: {}",
            state.provider_output_validation.reviewability_status.code()
        ),
        format!(
            "staged candidate-conversion proposal: {}",
            state.staged_candidate_conversion_proposal.status.code()
        ),
        format!(
            "staged proposal validation: {}",
            state.staged_candidate_conversion_validation.status.code()
        ),
        format!(
            "candidate review surface: {}",
            if state.staged_candidate_conversion_validation.status
                == StagedCandidateConversionValidationStatus::StagedProposalShapeValid
            {
                "validated_staged_proposal_review"
            } else {
                "not_available"
            }
        ),
        format!(
            "operator decision boundary: {}",
            state.operator_candidate_decision.status.code()
        ),
    ]
}

pub fn phase_150_remaining_production_gaps() -> Vec<String> {
    vec![
        "local session persistence".to_string(),
        "session restore".to_string(),
        "real adapter contract".to_string(),
        "real provider invocation".to_string(),
        "candidate materialization".to_string(),
        "complete local operator workflow".to_string(),
        "run history".to_string(),
        "export package".to_string(),
        "controlled trial readiness".to_string(),
        "deployment/package path".to_string(),
    ]
}

pub fn derive_phase_150_code_production_handoff(
    state: &LocalOperatorShellState,
) -> Phase150CodeProductionHandoff {
    let implemented = phase_150_implemented_capabilities(state);
    let gaps = phase_150_remaining_production_gaps();
    Phase150CodeProductionHandoff {
        handoff_id: format!(
            "phase-150-code-production-handoff-{}-{}-{}",
            state.provider_configuration.status.code(),
            state.staged_candidate_conversion_validation.status.code(),
            state.operator_candidate_decision.status.code()
        ),
        status: "phase_150_code_production_handoff".to_string(),
        implemented_capability_evidence: implemented,
        remaining_production_grade_gaps: gaps,
        remap_recommendations: vec![
            "Phase 150 should perform an aggressive production-path remap.".to_string(),
            "Phase 150 should group larger product capability phases.".to_string(),
            "Safety checks remain embedded in implementation phases.".to_string(),
            "Phase 150 is the roadmap/changelog alignment phase.".to_string(),
            "Phase 149 does not edit roadmap files.".to_string(),
        ],
        phase_149_roadmap_edit_status: "phase_149_does_not_edit_roadmap_files".to_string(),
    }
}

pub fn initial_phase_150_code_production_handoff() -> Phase150CodeProductionHandoff {
    Phase150CodeProductionHandoff {
        handoff_id:
            "phase-150-code-production-handoff-not_configured-not_validated-no_operator_decision"
                .to_string(),
        status: "phase_150_code_production_handoff".to_string(),
        implemented_capability_evidence: vec![
            "provider configuration: not_configured".to_string(),
            "deterministic provider execution: disabled_not_executed".to_string(),
            "provider execution result projection: none".to_string(),
            "provider output validation: not_validated".to_string(),
            "provider output review: not_reviewable".to_string(),
            "staged candidate-conversion proposal: no_proposal".to_string(),
            "staged proposal validation: not_validated".to_string(),
            "candidate review surface: not_available".to_string(),
            "operator decision boundary: no_operator_decision".to_string(),
        ],
        remaining_production_grade_gaps: phase_150_remaining_production_gaps(),
        remap_recommendations: vec![
            "Phase 150 should perform an aggressive production-path remap.".to_string(),
            "Phase 150 should group larger product capability phases.".to_string(),
            "Safety checks remain embedded in implementation phases.".to_string(),
            "Phase 150 is the roadmap/changelog alignment phase.".to_string(),
            "Phase 149 does not edit roadmap files.".to_string(),
        ],
        phase_149_roadmap_edit_status: "phase_149_does_not_edit_roadmap_files".to_string(),
    }
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
    next.local_provider_output_pipeline = derive_local_provider_output_pipeline_projection(&next);
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
        || !(projection.linkage.source_boundary == "sandboxed_deterministic_provider_execution"
            || projection.linkage.source_boundary
                == "constrained_local_provider_invocation_pipeline_bridge")
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalCandidateMaterializationStatus {
    NotMaterialized,
    LocalCandidateMaterialized,
    MaterializationRejected,
    MaterializationPreconditionMissing,
    InvalidMaterializationRequest,
}

impl LocalCandidateMaterializationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotMaterialized => "not_materialized",
            Self::LocalCandidateMaterialized => "local_candidate_materialized",
            Self::MaterializationRejected => "materialization_rejected",
            Self::MaterializationPreconditionMissing => "materialization_precondition_missing",
            Self::InvalidMaterializationRequest => "invalid_materialization_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalCandidateMaterializationError {
    ProviderOutputValidationMissing,
    ProviderOutputValidationNotReviewableUntrusted,
    ProviderOutputReviewMissing,
    StagedProposalMissing,
    StagedProposalValidationMissing,
    StagedProposalValidationNotValid,
    CandidateReviewMissing,
    OperatorDecisionMissing,
    OperatorDecisionRejected,
    OperatorDecisionNotApproved,
    ProviderPipelineIncomplete,
    ProviderPipelineRejected,
    SourceLinkageMismatch,
    InvocationResultIdMismatch,
    ProviderExecutionResultIdMismatch,
    StagedProposalIdMismatch,
    OperatorDecisionIdMismatch,
    TrustClaimRejected,
    SafetyClaimRejected,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    ProviderOutputApprovalClaimRejected,
    ActionClaimRejected,
    PersistenceClaimRejected,
    ExecutionClaimRejected,
    SigningClaimRejected,
    PublishingClaimRejected,
}

impl LocalCandidateMaterializationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ProviderOutputValidationMissing => "provider_output_validation_missing",
            Self::ProviderOutputValidationNotReviewableUntrusted => {
                "provider_output_validation_not_reviewable_untrusted"
            }
            Self::ProviderOutputReviewMissing => "provider_output_review_missing",
            Self::StagedProposalMissing => "staged_proposal_missing",
            Self::StagedProposalValidationMissing => "staged_proposal_validation_missing",
            Self::StagedProposalValidationNotValid => "staged_proposal_validation_not_valid",
            Self::CandidateReviewMissing => "candidate_review_missing",
            Self::OperatorDecisionMissing => "operator_decision_missing",
            Self::OperatorDecisionRejected => "operator_decision_rejected",
            Self::OperatorDecisionNotApproved => "operator_decision_not_approved",
            Self::ProviderPipelineIncomplete => "provider_pipeline_incomplete",
            Self::ProviderPipelineRejected => "provider_pipeline_rejected",
            Self::SourceLinkageMismatch => "source_linkage_mismatch",
            Self::InvocationResultIdMismatch => "invocation_result_id_mismatch",
            Self::ProviderExecutionResultIdMismatch => "provider_execution_result_id_mismatch",
            Self::StagedProposalIdMismatch => "staged_proposal_id_mismatch",
            Self::OperatorDecisionIdMismatch => "operator_decision_id_mismatch",
            Self::TrustClaimRejected => "trust_claim_rejected",
            Self::SafetyClaimRejected => "safety_claim_rejected",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::ProviderOutputApprovalClaimRejected => "provider_output_approval_claim_rejected",
            Self::ActionClaimRejected => "action_claim_rejected",
            Self::PersistenceClaimRejected => "persistence_claim_rejected",
            Self::ExecutionClaimRejected => "execution_claim_rejected",
            Self::SigningClaimRejected => "signing_claim_rejected",
            Self::PublishingClaimRejected => "publishing_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalCandidateMaterializationBoundaryStatus {
    LocalCandidateOutputOnly,
    NonProductionCandidate,
    ProviderOutputRemainsUntrusted,
    NoProviderTrust,
    NoProductionApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoActionExecution,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl LocalCandidateMaterializationBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalCandidateOutputOnly => "local_candidate_output_only",
            Self::NonProductionCandidate => "non_production_candidate",
            Self::ProviderOutputRemainsUntrusted => "provider_output_remains_untrusted",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalCandidateMaterializationEffectStatus {
    NoProviderTrust,
    NoProductionApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoActionExecution,
    NoReplayRepair,
    NoRecoveryPromotion,
    NoFileWrite,
    NoNetworkSocket,
    NoProcessSpawn,
    NoSecretRead,
    NoProviderExecution,
    NoProviderConfigurationMutation,
    NoProviderExecutionMutation,
    NoProviderOutputValidationMutation,
    NoStagedProposalMutation,
    NoOperatorDecisionMutation,
    NoExportPromotion,
}

impl LocalCandidateMaterializationEffectStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
            Self::NoFileWrite => "no_file_write",
            Self::NoNetworkSocket => "no_network_socket",
            Self::NoProcessSpawn => "no_process_spawn",
            Self::NoSecretRead => "no_secret_read",
            Self::NoProviderExecution => "no_provider_execution",
            Self::NoProviderConfigurationMutation => "no_provider_configuration_mutation",
            Self::NoProviderExecutionMutation => "no_provider_execution_mutation",
            Self::NoProviderOutputValidationMutation => "no_provider_output_validation_mutation",
            Self::NoStagedProposalMutation => "no_staged_proposal_mutation",
            Self::NoOperatorDecisionMutation => "no_operator_decision_mutation",
            Self::NoExportPromotion => "no_export_promotion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCandidateMaterializationRequest {
    pub staged_proposal_id: String,
    pub operator_decision_id: String,
    pub provider_execution_result_id: String,
    pub source_invocation_result_id: String,
    pub claims_trust: bool,
    pub claims_safety: bool,
    pub claims_readiness: bool,
    pub claims_release: bool,
    pub claims_deployment: bool,
    pub claims_public_use: bool,
    pub claims_provider_output_approval: bool,
    pub claims_action: bool,
    pub claims_persistence: bool,
    pub claims_execution: bool,
    pub claims_signing: bool,
    pub claims_publishing: bool,
}

impl LocalCandidateMaterializationRequest {
    pub fn from_validated_state(state: &LocalOperatorShellState) -> Option<Self> {
        let proposal = state
            .staged_candidate_conversion_proposal
            .proposal
            .as_ref()?;
        let decision = state.operator_candidate_decision.record.as_ref()?;
        Some(Self {
            staged_proposal_id: proposal.proposal_id.clone(),
            operator_decision_id: decision.decision_id.clone(),
            provider_execution_result_id: proposal.source_execution_result_id.clone(),
            source_invocation_result_id: state
                .local_provider_output_pipeline
                .source_invocation_result_id
                .clone()
                .unwrap_or_default(),
            claims_trust: false,
            claims_safety: false,
            claims_readiness: false,
            claims_release: false,
            claims_deployment: false,
            claims_public_use: false,
            claims_provider_output_approval: false,
            claims_action: false,
            claims_persistence: false,
            claims_execution: false,
            claims_signing: false,
            claims_publishing: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCandidateOutputSourceLinkage {
    pub source_invocation_result_id: String,
    pub provider_execution_result_id: String,
    pub provider_output_validation_status: LocalProviderOutputValidationStatus,
    pub provider_output_review_status: LocalProviderOutputReviewabilityStatus,
    pub staged_proposal_id: String,
    pub staged_proposal_validation_status: StagedCandidateConversionValidationStatus,
    pub operator_decision_id: String,
    pub operator_decision_status: OperatorCandidateDecisionStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCandidateMaterializationCapabilitySurface {
    pub local_only: bool,
    pub non_production: bool,
    pub materializes_candidate_output: bool,
    pub provider_trust_enabled: bool,
    pub action_execution_enabled: bool,
    pub production_approval_enabled: bool,
    pub release_approval_enabled: bool,
    pub deployment_approval_enabled: bool,
    pub public_use_approval_enabled: bool,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalCandidateOutputProjection {
    pub status: LocalCandidateMaterializationStatus,
    pub candidate_id: Option<String>,
    pub content_summary: Option<String>,
    pub output_classification: String,
    pub production_classification: String,
    pub source_linkage: Option<LocalCandidateOutputSourceLinkage>,
    pub provider_output_trust_carry_forward: String,
    pub boundary_statuses: Vec<LocalCandidateMaterializationBoundaryStatus>,
    pub effect_statuses: Vec<LocalCandidateMaterializationEffectStatus>,
    pub error: Option<LocalCandidateMaterializationError>,
    pub capability_surface: LocalCandidateMaterializationCapabilitySurface,
    pub note: String,
}

pub fn local_candidate_materialization_boundary_statuses(
) -> Vec<LocalCandidateMaterializationBoundaryStatus> {
    vec![
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
    ]
}

pub fn local_candidate_materialization_effect_statuses(
) -> Vec<LocalCandidateMaterializationEffectStatus> {
    vec![
        LocalCandidateMaterializationEffectStatus::NoProviderTrust,
        LocalCandidateMaterializationEffectStatus::NoProductionApproval,
        LocalCandidateMaterializationEffectStatus::NoReleaseApproval,
        LocalCandidateMaterializationEffectStatus::NoDeploymentApproval,
        LocalCandidateMaterializationEffectStatus::NoPublicUseApproval,
        LocalCandidateMaterializationEffectStatus::NoActionExecution,
        LocalCandidateMaterializationEffectStatus::NoReplayRepair,
        LocalCandidateMaterializationEffectStatus::NoRecoveryPromotion,
        LocalCandidateMaterializationEffectStatus::NoFileWrite,
        LocalCandidateMaterializationEffectStatus::NoNetworkSocket,
        LocalCandidateMaterializationEffectStatus::NoProcessSpawn,
        LocalCandidateMaterializationEffectStatus::NoSecretRead,
        LocalCandidateMaterializationEffectStatus::NoProviderExecution,
        LocalCandidateMaterializationEffectStatus::NoProviderConfigurationMutation,
        LocalCandidateMaterializationEffectStatus::NoProviderExecutionMutation,
        LocalCandidateMaterializationEffectStatus::NoProviderOutputValidationMutation,
        LocalCandidateMaterializationEffectStatus::NoStagedProposalMutation,
        LocalCandidateMaterializationEffectStatus::NoOperatorDecisionMutation,
        LocalCandidateMaterializationEffectStatus::NoExportPromotion,
    ]
}

pub fn local_candidate_materialization_capability_surface(
) -> LocalCandidateMaterializationCapabilitySurface {
    LocalCandidateMaterializationCapabilitySurface {
        local_only: true,
        non_production: true,
        materializes_candidate_output: true,
        provider_trust_enabled: false,
        action_execution_enabled: false,
        production_approval_enabled: false,
        release_approval_enabled: false,
        deployment_approval_enabled: false,
        public_use_approval_enabled: false,
        summary: "Local candidate output materialization only; non-production; provider output remains untrusted; no provider trust, action, production, readiness, release, deployment, or public-use approval.".to_string(),
    }
}

pub fn initial_local_candidate_output_projection() -> LocalCandidateOutputProjection {
    LocalCandidateOutputProjection {
        status: LocalCandidateMaterializationStatus::NotMaterialized,
        candidate_id: None,
        content_summary: None,
        output_classification: "local_candidate_output_only".to_string(),
        production_classification: "non_production_candidate".to_string(),
        source_linkage: None,
        provider_output_trust_carry_forward: "provider_output_remains_untrusted".to_string(),
        boundary_statuses: local_candidate_materialization_boundary_statuses(),
        effect_statuses: local_candidate_materialization_effect_statuses(),
        error: None,
        capability_surface: local_candidate_materialization_capability_surface(),
        note: "Local candidate output only. This candidate output is non-production. Provider output remains untrusted. Candidate materialization does not approve readiness, release, deployment, or public use. Candidate materialization does not authorize actions. Production approval remains unavailable.".to_string(),
    }
}

fn stable_local_candidate_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub fn derive_local_candidate_output_content(
    state: &LocalOperatorShellState,
    linkage: &LocalCandidateOutputSourceLinkage,
) -> String {
    let proposal = state
        .staged_candidate_conversion_proposal
        .proposal
        .as_ref()
        .expect("validated materialization requires staged proposal");
    let output_summary = state
        .provider_execution
        .result
        .as_ref()
        .map(|result| result.output_summary.as_str())
        .unwrap_or("none");
    format!(
        "Local candidate output only. staged_proposal={} operator_decision={} provider_execution={} invocation={} validation={} review={} source_summary={}",
        linkage.staged_proposal_id,
        linkage.operator_decision_id,
        linkage.provider_execution_result_id,
        linkage.source_invocation_result_id,
        linkage.provider_output_validation_status.code(),
        linkage.provider_output_review_status.code(),
        output_summary
    ) + &format!(" proposal_boundary={}", proposal.proposal_boundary)
}

pub fn validate_local_candidate_materialization_request(
    state: &LocalOperatorShellState,
    request: &LocalCandidateMaterializationRequest,
) -> Result<LocalCandidateOutputSourceLinkage, LocalCandidateMaterializationError> {
    if request.claims_trust {
        return Err(LocalCandidateMaterializationError::TrustClaimRejected);
    }
    if request.claims_safety {
        return Err(LocalCandidateMaterializationError::SafetyClaimRejected);
    }
    if request.claims_readiness {
        return Err(LocalCandidateMaterializationError::ReadinessClaimRejected);
    }
    if request.claims_release {
        return Err(LocalCandidateMaterializationError::ReleaseClaimRejected);
    }
    if request.claims_deployment {
        return Err(LocalCandidateMaterializationError::DeploymentClaimRejected);
    }
    if request.claims_public_use {
        return Err(LocalCandidateMaterializationError::PublicUseClaimRejected);
    }
    if request.claims_provider_output_approval {
        return Err(LocalCandidateMaterializationError::ProviderOutputApprovalClaimRejected);
    }
    if request.claims_action {
        return Err(LocalCandidateMaterializationError::ActionClaimRejected);
    }
    if request.claims_persistence {
        return Err(LocalCandidateMaterializationError::PersistenceClaimRejected);
    }
    if request.claims_execution {
        return Err(LocalCandidateMaterializationError::ExecutionClaimRejected);
    }
    if request.claims_signing {
        return Err(LocalCandidateMaterializationError::SigningClaimRejected);
    }
    if request.claims_publishing {
        return Err(LocalCandidateMaterializationError::PublishingClaimRejected);
    }
    if state.local_provider_output_pipeline.status
        == LocalProviderOutputPipelineValidationStatus::Rejected
    {
        return Err(LocalCandidateMaterializationError::ProviderPipelineRejected);
    }
    if state.local_provider_output_pipeline.status
        != LocalProviderOutputPipelineValidationStatus::Valid
    {
        return Err(LocalCandidateMaterializationError::ProviderPipelineIncomplete);
    }
    validate_provider_output_pipeline_stage_order(&state.local_provider_output_pipeline)
        .map_err(|_| LocalCandidateMaterializationError::ProviderPipelineIncomplete)?;
    if state.provider_output_validation.status == LocalProviderOutputValidationStatus::NotValidated
    {
        return Err(LocalCandidateMaterializationError::ProviderOutputValidationMissing);
    }
    if state.provider_output_validation.status
        != LocalProviderOutputValidationStatus::ReviewableUntrusted
    {
        return Err(
            LocalCandidateMaterializationError::ProviderOutputValidationNotReviewableUntrusted,
        );
    }
    if state.provider_output_validation.reviewability_status
        != LocalProviderOutputReviewabilityStatus::ReviewableUntrusted
    {
        return Err(LocalCandidateMaterializationError::ProviderOutputReviewMissing);
    }
    let Some(proposal) = state.staged_candidate_conversion_proposal.proposal.as_ref() else {
        return Err(LocalCandidateMaterializationError::StagedProposalMissing);
    };
    if state.staged_candidate_conversion_proposal.status
        != StagedCandidateConversionProposalStatus::StagedProposalCreated
    {
        return Err(LocalCandidateMaterializationError::StagedProposalMissing);
    }
    match state.staged_candidate_conversion_validation.status {
        StagedCandidateConversionValidationStatus::NotValidated => {
            return Err(LocalCandidateMaterializationError::StagedProposalValidationMissing)
        }
        StagedCandidateConversionValidationStatus::StagedProposalShapeValid => {}
        StagedCandidateConversionValidationStatus::RejectedStagedProposal
        | StagedCandidateConversionValidationStatus::InvalidValidationInput => {
            return Err(LocalCandidateMaterializationError::StagedProposalValidationNotValid)
        }
    }
    if state.local_provider_output_pipeline.candidate_review_status != "display_only" {
        return Err(LocalCandidateMaterializationError::CandidateReviewMissing);
    }
    match state.operator_candidate_decision.status {
        OperatorCandidateDecisionStatus::NoOperatorDecision => {
            return Err(LocalCandidateMaterializationError::OperatorDecisionMissing)
        }
        OperatorCandidateDecisionStatus::RejectedValidatedStagedProposal => {
            return Err(LocalCandidateMaterializationError::OperatorDecisionRejected)
        }
        OperatorCandidateDecisionStatus::ApprovedValidatedStagedProposal => {}
        OperatorCandidateDecisionStatus::RejectedOperatorDecisionRequest
        | OperatorCandidateDecisionStatus::InvalidOperatorDecisionInput => {
            return Err(LocalCandidateMaterializationError::OperatorDecisionNotApproved)
        }
    }
    let Some(decision) = state.operator_candidate_decision.record.as_ref() else {
        return Err(LocalCandidateMaterializationError::OperatorDecisionMissing);
    };
    if request.staged_proposal_id != proposal.proposal_id
        || request.staged_proposal_id != decision.staged_proposal_id
        || Some(&request.staged_proposal_id)
            != state
                .staged_candidate_conversion_validation
                .proposal_id
                .as_ref()
    {
        return Err(LocalCandidateMaterializationError::StagedProposalIdMismatch);
    }
    if request.operator_decision_id != decision.decision_id {
        return Err(LocalCandidateMaterializationError::OperatorDecisionIdMismatch);
    }
    let provider_result_id = state
        .provider_execution
        .result
        .as_ref()
        .map(|result| result.result_id.as_str())
        .ok_or(LocalCandidateMaterializationError::ProviderExecutionResultIdMismatch)?;
    if request.provider_execution_result_id != provider_result_id
        || request.provider_execution_result_id != proposal.source_execution_result_id
        || request.provider_execution_result_id != decision.provider_execution_result_id
        || Some(&request.provider_execution_result_id)
            != state
                .provider_output_validation
                .provider_execution_result_id
                .as_ref()
        || Some(&request.provider_execution_result_id)
            != state
                .staged_candidate_conversion_validation
                .source_execution_result_id
                .as_ref()
        || Some(&request.provider_execution_result_id)
            != state
                .local_provider_output_pipeline
                .provider_execution_result_id
                .as_ref()
    {
        return Err(LocalCandidateMaterializationError::ProviderExecutionResultIdMismatch);
    }
    let invocation_result_id = state
        .constrained_local_provider_invocation
        .result
        .as_ref()
        .map(|result| result.result_id.as_str())
        .ok_or(LocalCandidateMaterializationError::InvocationResultIdMismatch)?;
    if request.source_invocation_result_id != invocation_result_id
        || Some(&request.source_invocation_result_id)
            != state
                .local_provider_output_pipeline
                .source_invocation_result_id
                .as_ref()
    {
        return Err(LocalCandidateMaterializationError::InvocationResultIdMismatch);
    }
    let reprojected_validation = project_staged_candidate_conversion_validation(
        state,
        &StagedCandidateConversionValidationRequest {
            proposal_id: Some(proposal.proposal_id.clone()),
        },
    );
    if reprojected_validation != state.staged_candidate_conversion_validation {
        return Err(LocalCandidateMaterializationError::SourceLinkageMismatch);
    }
    Ok(LocalCandidateOutputSourceLinkage {
        source_invocation_result_id: request.source_invocation_result_id.clone(),
        provider_execution_result_id: request.provider_execution_result_id.clone(),
        provider_output_validation_status: state.provider_output_validation.status,
        provider_output_review_status: state.provider_output_validation.reviewability_status,
        staged_proposal_id: request.staged_proposal_id.clone(),
        staged_proposal_validation_status: state.staged_candidate_conversion_validation.status,
        operator_decision_id: request.operator_decision_id.clone(),
        operator_decision_status: state.operator_candidate_decision.status,
    })
}

pub fn reject_local_candidate_materialization(
    previous: &LocalCandidateOutputProjection,
    error: LocalCandidateMaterializationError,
) -> LocalCandidateOutputProjection {
    let mut projection = previous.clone();
    projection.status = match error {
        LocalCandidateMaterializationError::TrustClaimRejected
        | LocalCandidateMaterializationError::SafetyClaimRejected
        | LocalCandidateMaterializationError::ReadinessClaimRejected
        | LocalCandidateMaterializationError::ReleaseClaimRejected
        | LocalCandidateMaterializationError::DeploymentClaimRejected
        | LocalCandidateMaterializationError::PublicUseClaimRejected
        | LocalCandidateMaterializationError::ProviderOutputApprovalClaimRejected
        | LocalCandidateMaterializationError::ActionClaimRejected
        | LocalCandidateMaterializationError::PersistenceClaimRejected
        | LocalCandidateMaterializationError::ExecutionClaimRejected
        | LocalCandidateMaterializationError::SigningClaimRejected
        | LocalCandidateMaterializationError::PublishingClaimRejected => {
            LocalCandidateMaterializationStatus::InvalidMaterializationRequest
        }
        _ => LocalCandidateMaterializationStatus::MaterializationRejected,
    };
    if matches!(
        error,
        LocalCandidateMaterializationError::OperatorDecisionMissing
            | LocalCandidateMaterializationError::ProviderOutputValidationMissing
            | LocalCandidateMaterializationError::ProviderOutputReviewMissing
            | LocalCandidateMaterializationError::StagedProposalMissing
            | LocalCandidateMaterializationError::StagedProposalValidationMissing
            | LocalCandidateMaterializationError::CandidateReviewMissing
            | LocalCandidateMaterializationError::ProviderPipelineIncomplete
    ) {
        projection.status = LocalCandidateMaterializationStatus::MaterializationPreconditionMissing;
    }
    projection.error = Some(error);
    projection.note = format!(
        "Local candidate materialization rejected: {}. Local candidate output only; provider output remains untrusted; no production, release, deployment, public-use, or action authorization effect occurred.",
        error.code()
    );
    projection
}

pub fn project_local_candidate_output(
    state: &LocalOperatorShellState,
    request: &LocalCandidateMaterializationRequest,
) -> Result<LocalCandidateOutputProjection, LocalCandidateMaterializationError> {
    let linkage = validate_local_candidate_materialization_request(state, request)?;
    let content_summary = derive_local_candidate_output_content(state, &linkage);
    let digest = stable_local_candidate_digest(&format!(
        "phase_158|{}|{}|{}|{}|{}",
        linkage.source_invocation_result_id,
        linkage.provider_execution_result_id,
        linkage.staged_proposal_id,
        linkage.operator_decision_id,
        content_summary
    ));
    Ok(LocalCandidateOutputProjection {
        status: LocalCandidateMaterializationStatus::LocalCandidateMaterialized,
        candidate_id: Some(format!("local-candidate-output-{digest}")),
        content_summary: Some(content_summary),
        output_classification: "local_candidate_output_only".to_string(),
        production_classification: "non_production_candidate".to_string(),
        source_linkage: Some(linkage),
        provider_output_trust_carry_forward: "provider_output_remains_untrusted".to_string(),
        boundary_statuses: local_candidate_materialization_boundary_statuses(),
        effect_statuses: local_candidate_materialization_effect_statuses(),
        error: None,
        capability_surface: local_candidate_materialization_capability_surface(),
        note: "Local candidate output only. This candidate output is non-production. Provider output remains untrusted. Candidate materialization does not approve readiness, release, deployment, or public use. Candidate materialization does not authorize actions. Production approval remains unavailable.".to_string(),
    })
}

pub fn materialize_local_candidate_output(
    state: &LocalOperatorShellState,
    request: LocalCandidateMaterializationRequest,
) -> Result<LocalOperatorShellState, LocalCandidateMaterializationError> {
    let projection = project_local_candidate_output(state, &request)?;
    let mut next = state.clone();
    next.local_candidate_output = projection;
    Ok(attach_local_session_evidence_export(next))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionPackageClassification {
    LocalSessionPackageOnly,
}

impl LocalSessionPackageClassification {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalSessionPackageOnly => "local_session_package_only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionPackageProductionClassification {
    NonProduction,
}

impl LocalSessionPackageProductionClassification {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NonProduction => "non_production",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionPackageStatus {
    NotPackaged,
    PackageProjected,
    PackageWritten,
    PackageReadBackValidated,
    PackageRejected,
    InvalidPackageInput,
}

impl LocalSessionPackageStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotPackaged => "not_packaged",
            Self::PackageProjected => "package_projected",
            Self::PackageWritten => "package_written",
            Self::PackageReadBackValidated => "package_read_back_validated",
            Self::PackageRejected => "package_rejected",
            Self::InvalidPackageInput => "invalid_package_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionPackageValidationStatus {
    NotValidated,
    Valid,
    Invalid,
}

impl LocalSessionPackageValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotValidated => "not_validated",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionPackageRestoreStatus {
    NotRestored,
    ReadBackValidated,
    Rejected,
}

impl LocalSessionPackageRestoreStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotRestored => "not_restored",
            Self::ReadBackValidated => "read_back_validated_structure_only",
            Self::Rejected => "restore_projection_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionPackageValidationError {
    MissingPackageId,
    MissingPackageVersion,
    InvalidPackageClassification,
    InvalidProductionClassification,
    MissingAbsenceMarker,
    ForbiddenReleaseReadinessOrDeploymentClaim,
    ForbiddenSigningPublishingInstallerOrUpdateClaim,
    ForbiddenProviderTrustClaim,
    ForbiddenCandidateApprovalClaim,
    ForbiddenActionExecutionClaim,
    ForbiddenPersistenceAuthorityClaim,
    DeterministicContentMismatch,
    MalformedPackageInput,
}

impl LocalSessionPackageValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingPackageId => "missing_package_id",
            Self::MissingPackageVersion => "missing_package_version",
            Self::InvalidPackageClassification => "invalid_package_classification",
            Self::InvalidProductionClassification => "invalid_production_classification",
            Self::MissingAbsenceMarker => "missing_absence_marker",
            Self::ForbiddenReleaseReadinessOrDeploymentClaim => {
                "forbidden_release_readiness_or_deployment_claim"
            }
            Self::ForbiddenSigningPublishingInstallerOrUpdateClaim => {
                "forbidden_signing_publishing_installer_or_update_claim"
            }
            Self::ForbiddenProviderTrustClaim => "forbidden_provider_trust_claim",
            Self::ForbiddenCandidateApprovalClaim => "forbidden_candidate_approval_claim",
            Self::ForbiddenActionExecutionClaim => "forbidden_action_execution_claim",
            Self::ForbiddenPersistenceAuthorityClaim => "forbidden_persistence_authority_claim",
            Self::DeterministicContentMismatch => "deterministic_content_mismatch",
            Self::MalformedPackageInput => "malformed_package_input",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionPackageMetadata {
    pub package_id: String,
    pub package_version: String,
    pub package_classification: String,
    pub production_classification: String,
    pub package_status: LocalSessionPackageStatus,
    pub validation_status: LocalSessionPackageValidationStatus,
    pub content_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionPackageAbsenceMarkers {
    pub release_artifact_absent: bool,
    pub deployment_artifact_absent: bool,
    pub readiness_evidence_absent: bool,
    pub production_persistence_absent: bool,
    pub installer_absent: bool,
    pub update_channel_absent: bool,
    pub signing_absent: bool,
    pub publishing_absent: bool,
    pub public_use_absent: bool,
    pub provider_trust_absent: bool,
    pub candidate_approval_absent: bool,
    pub action_execution_absent: bool,
    pub automatic_persistence_absent: bool,
    pub background_service_absent: bool,
    pub remote_sync_absent: bool,
    pub marker_summary: Vec<String>,
}

pub fn local_session_package_absence_markers() -> LocalSessionPackageAbsenceMarkers {
    LocalSessionPackageAbsenceMarkers {
        release_artifact_absent: true,
        deployment_artifact_absent: true,
        readiness_evidence_absent: true,
        production_persistence_absent: true,
        installer_absent: true,
        update_channel_absent: true,
        signing_absent: true,
        publishing_absent: true,
        public_use_absent: true,
        provider_trust_absent: true,
        candidate_approval_absent: true,
        action_execution_absent: true,
        automatic_persistence_absent: true,
        background_service_absent: true,
        remote_sync_absent: true,
        marker_summary: vec![
            "release artifact absent".to_string(),
            "deployment artifact absent".to_string(),
            "readiness evidence absent".to_string(),
            "production persistence absent".to_string(),
            "installer absent".to_string(),
            "update-channel absent".to_string(),
            "signing absent".to_string(),
            "publishing absent".to_string(),
            "public-use absent".to_string(),
            "provider trust absent".to_string(),
            "candidate approval absent".to_string(),
            "action execution absent".to_string(),
            "automatic persistence absent".to_string(),
            "background service absent".to_string(),
            "remote sync absent".to_string(),
        ],
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionPackagePayload {
    pub provider_configuration_projection: String,
    pub provider_execution_result_projection: String,
    pub provider_output_validation_projection: String,
    pub provider_output_review_projection: String,
    pub staged_candidate_conversion_proposal_projection: String,
    pub staged_candidate_conversion_validation_projection: String,
    pub candidate_review_surface_projection: String,
    pub operator_candidate_decision_projection: String,
    pub local_decision_ledger_projection: String,
    pub replay_status_projection: String,
    pub local_session_evidence_export_projection: String,
    pub phase_150_handoff_context_projection: String,
    pub no_release_marker: String,
    pub no_deployment_marker: String,
    pub no_readiness_marker: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionPackage {
    pub metadata: LocalSessionPackageMetadata,
    pub payload: LocalSessionPackagePayload,
    pub absence_markers: LocalSessionPackageAbsenceMarkers,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionPackageProjection {
    pub status: LocalSessionPackageStatus,
    pub package_id: Option<String>,
    pub package_version: String,
    pub package_classification: String,
    pub production_classification: String,
    pub validation_status: LocalSessionPackageValidationStatus,
    pub validation_errors: Vec<String>,
    pub read_back_validation_status: Option<LocalSessionPackageValidationStatus>,
    pub restore_status: LocalSessionPackageRestoreStatus,
    pub included_section_summary: Vec<String>,
    pub absence_marker_summary: Vec<String>,
    pub local_only_note: String,
    pub release_boundary_note: String,
    pub deployment_readiness_boundary_note: String,
    pub restore_boundary_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionPackageWriteResult {
    pub status: LocalSessionPackageStatus,
    pub path: String,
    pub bytes_written: usize,
    pub projection: LocalSessionPackageProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionPackageReadResult {
    pub status: LocalSessionPackageStatus,
    pub path: String,
    pub package: Option<LocalSessionPackage>,
    pub projection: LocalSessionPackageProjection,
}

pub const LOCAL_SESSION_PACKAGE_VERSION: &str = "local-session-package-v1";

pub fn initial_local_session_package_projection() -> LocalSessionPackageProjection {
    LocalSessionPackageProjection {
        status: LocalSessionPackageStatus::NotPackaged,
        package_id: None,
        package_version: LOCAL_SESSION_PACKAGE_VERSION.to_string(),
        package_classification: LocalSessionPackageClassification::LocalSessionPackageOnly
            .code()
            .to_string(),
        production_classification: LocalSessionPackageProductionClassification::NonProduction
            .code()
            .to_string(),
        validation_status: LocalSessionPackageValidationStatus::NotValidated,
        validation_errors: Vec::new(),
        read_back_validation_status: None,
        restore_status: LocalSessionPackageRestoreStatus::NotRestored,
        included_section_summary: Vec::new(),
        absence_marker_summary: local_session_package_absence_markers().marker_summary,
        local_only_note: "Local session package is local-only and non-production.".to_string(),
        release_boundary_note: "This package is not a release artifact.".to_string(),
        deployment_readiness_boundary_note: "This package is not deployment or readiness evidence."
            .to_string(),
        restore_boundary_note:
            "Package restore/read-back validates structure only and does not promote recovery."
                .to_string(),
    }
}

fn local_session_package_payload_from_state(
    state: &LocalOperatorShellState,
) -> LocalSessionPackagePayload {
    LocalSessionPackagePayload {
        provider_configuration_projection: format!("{:?}", state.provider_configuration),
        provider_execution_result_projection: format!("{:?}", state.provider_execution),
        provider_output_validation_projection: format!("{:?}", state.provider_output_validation),
        provider_output_review_projection: format!(
            "{:?}",
            project_local_provider_output_validation(state)
        ),
        staged_candidate_conversion_proposal_projection: format!(
            "{:?}",
            state.staged_candidate_conversion_proposal
        ),
        staged_candidate_conversion_validation_projection: format!(
            "{:?}",
            state.staged_candidate_conversion_validation
        ),
        candidate_review_surface_projection: format!("{:?}", state.run.candidate),
        operator_candidate_decision_projection: format!("{:?}", state.operator_candidate_decision),
        local_decision_ledger_projection: format!("{:?}", state.decision_ledger),
        replay_status_projection: format!("{:?}", state.run.decision_replay),
        local_session_evidence_export_projection: format!(
            "{:?}",
            state.local_session_evidence_export
        ),
        phase_150_handoff_context_projection: format!(
            "{:?}",
            state.phase_150_code_production_handoff
        ),
        no_release_marker: "not a release artifact".to_string(),
        no_deployment_marker: "not deployment evidence".to_string(),
        no_readiness_marker: "not readiness evidence".to_string(),
    }
}

fn local_session_package_content_basis(
    payload: &LocalSessionPackagePayload,
    markers: &LocalSessionPackageAbsenceMarkers,
) -> String {
    format!(
        "version={}|classification=local_session_package_only|production=non_production|payload={:?}|absence={:?}",
        LOCAL_SESSION_PACKAGE_VERSION, payload, markers
    )
}

fn stable_local_session_package_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub fn derive_local_session_package(state: &LocalOperatorShellState) -> LocalSessionPackage {
    let payload = local_session_package_payload_from_state(state);
    let absence_markers = local_session_package_absence_markers();
    let content_digest = stable_local_session_package_digest(&local_session_package_content_basis(
        &payload,
        &absence_markers,
    ));
    let package_id = format!("local-session-package-{content_digest}");
    LocalSessionPackage {
        metadata: LocalSessionPackageMetadata {
            package_id,
            package_version: LOCAL_SESSION_PACKAGE_VERSION.to_string(),
            package_classification: LocalSessionPackageClassification::LocalSessionPackageOnly
                .code()
                .to_string(),
            production_classification: LocalSessionPackageProductionClassification::NonProduction
                .code()
                .to_string(),
            package_status: LocalSessionPackageStatus::PackageProjected,
            validation_status: LocalSessionPackageValidationStatus::Valid,
            content_digest,
        },
        payload,
        absence_markers,
    }
}

fn local_session_package_included_sections() -> Vec<String> {
    vec![
        "provider configuration".to_string(),
        "provider execution result".to_string(),
        "provider output validation".to_string(),
        "provider output review projection".to_string(),
        "staged candidate-conversion proposal".to_string(),
        "staged proposal validation".to_string(),
        "candidate review surface".to_string(),
        "operator candidate decision".to_string(),
        "local decision ledger".to_string(),
        "replay/status projection".to_string(),
        "local session evidence export".to_string(),
        "Phase 150 handoff context".to_string(),
    ]
}

fn local_session_package_validation_errors(
    package: &LocalSessionPackage,
) -> Vec<LocalSessionPackageValidationError> {
    let mut errors = Vec::new();
    if package.metadata.package_id.is_empty() {
        errors.push(LocalSessionPackageValidationError::MissingPackageId);
    }
    if package.metadata.package_version.is_empty() {
        errors.push(LocalSessionPackageValidationError::MissingPackageVersion);
    }
    if package.metadata.package_classification
        != LocalSessionPackageClassification::LocalSessionPackageOnly.code()
    {
        errors.push(LocalSessionPackageValidationError::InvalidPackageClassification);
    }
    if package.metadata.production_classification
        != LocalSessionPackageProductionClassification::NonProduction.code()
    {
        errors.push(LocalSessionPackageValidationError::InvalidProductionClassification);
    }
    let markers = &package.absence_markers;
    if !markers.release_artifact_absent
        || !markers.deployment_artifact_absent
        || !markers.readiness_evidence_absent
        || !markers.production_persistence_absent
        || !markers.installer_absent
        || !markers.update_channel_absent
        || !markers.signing_absent
        || !markers.publishing_absent
        || !markers.public_use_absent
        || !markers.provider_trust_absent
        || !markers.candidate_approval_absent
        || !markers.action_execution_absent
        || !markers.automatic_persistence_absent
        || !markers.background_service_absent
        || !markers.remote_sync_absent
    {
        errors.push(LocalSessionPackageValidationError::MissingAbsenceMarker);
    }
    let text = format!("{:?}", package).to_ascii_lowercase();
    if [
        "claim:release_candidate_approved",
        "claim:production candidate status approved",
        "claim:public_use_approved",
        "claim:production_ready",
        "claim:readiness_approved",
        "claim:deployment_enabled",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionPackageValidationError::ForbiddenReleaseReadinessOrDeploymentClaim);
    }
    if [
        "claim:signing_enabled",
        "claim:publishing_enabled",
        "claim:installer_created",
        "claim:update_channel_enabled",
        "claim:github_release_created",
        "claim:release_tag_created",
        "claim:public_download",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(
            LocalSessionPackageValidationError::ForbiddenSigningPublishingInstallerOrUpdateClaim,
        );
    }
    if [
        "claim:provider_trusted",
        "claim:trusted_provider_output",
        "claim:provider trust granted",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionPackageValidationError::ForbiddenProviderTrustClaim);
    }
    if [
        "claim:candidate_approved",
        "claim:candidate approval granted",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionPackageValidationError::ForbiddenCandidateApprovalClaim);
    }
    if [
        "claim:action_executed",
        "claim:execute action",
        "claim:action execution enabled",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionPackageValidationError::ForbiddenActionExecutionClaim);
    }
    if [
        "claim:production_persistence_enabled",
        "claim:durable persistence authority",
        "claim:automatic persistence enabled",
        "claim:background persistence enabled",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionPackageValidationError::ForbiddenPersistenceAuthorityClaim);
    }
    let expected_digest = stable_local_session_package_digest(
        &local_session_package_content_basis(&package.payload, &package.absence_markers),
    );
    let expected_id = format!("local-session-package-{expected_digest}");
    if package.metadata.content_digest != expected_digest
        || (!package.metadata.package_id.is_empty() && package.metadata.package_id != expected_id)
    {
        errors.push(LocalSessionPackageValidationError::DeterministicContentMismatch);
    }
    errors
}

pub fn validate_local_session_package(
    package: &LocalSessionPackage,
) -> Result<(), Vec<LocalSessionPackageValidationError>> {
    let errors = local_session_package_validation_errors(package);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn project_local_session_package_status(
    package: Option<&LocalSessionPackage>,
    read_back_status: Option<LocalSessionPackageValidationStatus>,
) -> LocalSessionPackageProjection {
    match package {
        None => initial_local_session_package_projection(),
        Some(package) => {
            let errors = local_session_package_validation_errors(package);
            let validation_status = if errors.is_empty() {
                LocalSessionPackageValidationStatus::Valid
            } else {
                LocalSessionPackageValidationStatus::Invalid
            };
            LocalSessionPackageProjection {
                status: if errors.is_empty() { package.metadata.package_status } else { LocalSessionPackageStatus::PackageRejected },
                package_id: Some(package.metadata.package_id.clone()),
                package_version: package.metadata.package_version.clone(),
                package_classification: package.metadata.package_classification.clone(),
                production_classification: package.metadata.production_classification.clone(),
                validation_status,
                validation_errors: errors.into_iter().map(|error| error.code().to_string()).collect(),
                read_back_validation_status: read_back_status,
                restore_status: if read_back_status == Some(LocalSessionPackageValidationStatus::Valid) { LocalSessionPackageRestoreStatus::ReadBackValidated } else { LocalSessionPackageRestoreStatus::NotRestored },
                included_section_summary: local_session_package_included_sections(),
                absence_marker_summary: package.absence_markers.marker_summary.clone(),
                local_only_note: "Local session package is local-only and non-production.".to_string(),
                release_boundary_note: "This package is not a release artifact.".to_string(),
                deployment_readiness_boundary_note: "This package is not deployment or readiness evidence.".to_string(),
                restore_boundary_note: "Package restore/read-back validates structure only and does not promote recovery.".to_string(),
            }
        }
    }
}

fn hex_encode(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len() * 2);
    for byte in input.as_bytes() {
        encoded.push_str(&format!("{byte:02x}"));
    }
    encoded
}

fn hex_decode(input: &str) -> Result<String, LocalSessionPackageValidationError> {
    if !input.len().is_multiple_of(2) {
        return Err(LocalSessionPackageValidationError::MalformedPackageInput);
    }
    let mut bytes = Vec::with_capacity(input.len() / 2);
    for index in (0..input.len()).step_by(2) {
        let byte = u8::from_str_radix(&input[index..index + 2], 16)
            .map_err(|_| LocalSessionPackageValidationError::MalformedPackageInput)?;
        bytes.push(byte);
    }
    String::from_utf8(bytes).map_err(|_| LocalSessionPackageValidationError::MalformedPackageInput)
}

pub fn serialize_local_session_package(
    package: &LocalSessionPackage,
) -> Result<String, Vec<LocalSessionPackageValidationError>> {
    validate_local_session_package(package)?;
    let mut lines = vec![
        "ajentic_local_session_package=v1".to_string(),
        format!("package_id={}", package.metadata.package_id),
        format!("package_version={}", package.metadata.package_version),
        format!(
            "package_classification={}",
            package.metadata.package_classification
        ),
        format!(
            "production_classification={}",
            package.metadata.production_classification
        ),
        format!("package_status={}", package.metadata.package_status.code()),
        format!(
            "validation_status={}",
            package.metadata.validation_status.code()
        ),
        format!("content_digest={}", package.metadata.content_digest),
    ];
    let payload_fields = [
        (
            "provider_configuration_projection",
            &package.payload.provider_configuration_projection,
        ),
        (
            "provider_execution_result_projection",
            &package.payload.provider_execution_result_projection,
        ),
        (
            "provider_output_validation_projection",
            &package.payload.provider_output_validation_projection,
        ),
        (
            "provider_output_review_projection",
            &package.payload.provider_output_review_projection,
        ),
        (
            "staged_candidate_conversion_proposal_projection",
            &package
                .payload
                .staged_candidate_conversion_proposal_projection,
        ),
        (
            "staged_candidate_conversion_validation_projection",
            &package
                .payload
                .staged_candidate_conversion_validation_projection,
        ),
        (
            "candidate_review_surface_projection",
            &package.payload.candidate_review_surface_projection,
        ),
        (
            "operator_candidate_decision_projection",
            &package.payload.operator_candidate_decision_projection,
        ),
        (
            "local_decision_ledger_projection",
            &package.payload.local_decision_ledger_projection,
        ),
        (
            "replay_status_projection",
            &package.payload.replay_status_projection,
        ),
        (
            "local_session_evidence_export_projection",
            &package.payload.local_session_evidence_export_projection,
        ),
        (
            "phase_150_handoff_context_projection",
            &package.payload.phase_150_handoff_context_projection,
        ),
        ("no_release_marker", &package.payload.no_release_marker),
        (
            "no_deployment_marker",
            &package.payload.no_deployment_marker,
        ),
        ("no_readiness_marker", &package.payload.no_readiness_marker),
    ];
    for (key, value) in payload_fields {
        lines.push(format!("{key}={}", hex_encode(value)));
    }
    lines.push(format!(
        "absence_markers={}",
        hex_encode(&format!("{:?}", package.absence_markers))
    ));
    lines.push(
        "local_only_note=Local session package is local-only and non-production.".to_string(),
    );
    lines.push("release_boundary_note=This package is not a release artifact.".to_string());
    lines.push(
        "deployment_readiness_boundary_note=This package is not deployment or readiness evidence."
            .to_string(),
    );
    lines.push("restore_boundary_note=Package restore/read-back validates structure only and does not promote recovery.".to_string());
    Ok(format!("{}\n", lines.join("\n")))
}

fn parse_status(
    code: &str,
) -> Result<LocalSessionPackageStatus, LocalSessionPackageValidationError> {
    match code {
        "package_projected" => Ok(LocalSessionPackageStatus::PackageProjected),
        "package_written" => Ok(LocalSessionPackageStatus::PackageWritten),
        "package_read_back_validated" => Ok(LocalSessionPackageStatus::PackageReadBackValidated),
        "package_rejected" => Ok(LocalSessionPackageStatus::PackageRejected),
        "invalid_package_input" => Ok(LocalSessionPackageStatus::InvalidPackageInput),
        "not_packaged" => Ok(LocalSessionPackageStatus::NotPackaged),
        _ => Err(LocalSessionPackageValidationError::MalformedPackageInput),
    }
}

pub fn parse_local_session_package(
    content: &str,
) -> Result<LocalSessionPackage, Vec<LocalSessionPackageValidationError>> {
    let mut values = std::collections::BTreeMap::new();
    for line in content.lines() {
        let Some((key, value)) = line.split_once('=') else {
            return Err(vec![
                LocalSessionPackageValidationError::MalformedPackageInput,
            ]);
        };
        values.insert(key.to_string(), value.to_string());
    }
    if values
        .get("ajentic_local_session_package")
        .map(String::as_str)
        != Some("v1")
    {
        return Err(vec![
            LocalSessionPackageValidationError::MalformedPackageInput,
        ]);
    }
    let get = |key: &str| {
        values
            .get(key)
            .cloned()
            .ok_or(LocalSessionPackageValidationError::MalformedPackageInput)
    };
    let decode = |key: &str| get(key).and_then(|value| hex_decode(&value));
    let package = LocalSessionPackage {
        metadata: LocalSessionPackageMetadata {
            package_id: get("package_id").map_err(|e| vec![e])?,
            package_version: get("package_version").map_err(|e| vec![e])?,
            package_classification: get("package_classification").map_err(|e| vec![e])?,
            production_classification: get("production_classification").map_err(|e| vec![e])?,
            package_status: parse_status(&get("package_status").map_err(|e| vec![e])?)
                .map_err(|e| vec![e])?,
            validation_status: LocalSessionPackageValidationStatus::Valid,
            content_digest: get("content_digest").map_err(|e| vec![e])?,
        },
        payload: LocalSessionPackagePayload {
            provider_configuration_projection: decode("provider_configuration_projection")
                .map_err(|e| vec![e])?,
            provider_execution_result_projection: decode("provider_execution_result_projection")
                .map_err(|e| vec![e])?,
            provider_output_validation_projection: decode("provider_output_validation_projection")
                .map_err(|e| vec![e])?,
            provider_output_review_projection: decode("provider_output_review_projection")
                .map_err(|e| vec![e])?,
            staged_candidate_conversion_proposal_projection: decode(
                "staged_candidate_conversion_proposal_projection",
            )
            .map_err(|e| vec![e])?,
            staged_candidate_conversion_validation_projection: decode(
                "staged_candidate_conversion_validation_projection",
            )
            .map_err(|e| vec![e])?,
            candidate_review_surface_projection: decode("candidate_review_surface_projection")
                .map_err(|e| vec![e])?,
            operator_candidate_decision_projection: decode(
                "operator_candidate_decision_projection",
            )
            .map_err(|e| vec![e])?,
            local_decision_ledger_projection: decode("local_decision_ledger_projection")
                .map_err(|e| vec![e])?,
            replay_status_projection: decode("replay_status_projection").map_err(|e| vec![e])?,
            local_session_evidence_export_projection: decode(
                "local_session_evidence_export_projection",
            )
            .map_err(|e| vec![e])?,
            phase_150_handoff_context_projection: decode("phase_150_handoff_context_projection")
                .map_err(|e| vec![e])?,
            no_release_marker: decode("no_release_marker").map_err(|e| vec![e])?,
            no_deployment_marker: decode("no_deployment_marker").map_err(|e| vec![e])?,
            no_readiness_marker: decode("no_readiness_marker").map_err(|e| vec![e])?,
        },
        absence_markers: local_session_package_absence_markers(),
    };
    validate_local_session_package(&package)?;
    Ok(package)
}

pub fn validate_local_session_package_read_back(
    package: &LocalSessionPackage,
) -> LocalSessionPackageProjection {
    project_local_session_package_status(
        Some(package),
        Some(LocalSessionPackageValidationStatus::Valid),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionHistoryStatus {
    NoSessionHistory,
    SessionHistoryProjected,
}

impl LocalSessionHistoryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoSessionHistory => "no_session_history",
            Self::SessionHistoryProjected => "session_history_projected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionHistoryEntry {
    pub package_id: String,
    pub package_version: String,
    pub package_classification: String,
    pub production_classification: String,
    pub package_status: LocalSessionPackageStatus,
    pub validation_status: LocalSessionPackageValidationStatus,
    pub read_back_validation_status: Option<LocalSessionPackageValidationStatus>,
    pub included_section_summary: Vec<String>,
    pub absence_marker_summary: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionHistoryProjection {
    pub status: LocalSessionHistoryStatus,
    pub entries: Vec<LocalSessionHistoryEntry>,
    pub selected_package_id: Option<String>,
    pub boundary_note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionRestoreStatus {
    RestoreNotRequested,
    PackageSelected,
    PackageReadBackValidated,
    RestorePreviewProjected,
    RestoreProjected,
    RestoreRejected,
    InvalidRestoreInput,
}

impl LocalSessionRestoreStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::RestoreNotRequested => "restore_not_requested",
            Self::PackageSelected => "package_selected",
            Self::PackageReadBackValidated => "package_read_back_validated",
            Self::RestorePreviewProjected => "restore_preview_projected",
            Self::RestoreProjected => "restore_projected",
            Self::RestoreRejected => "restore_rejected",
            Self::InvalidRestoreInput => "invalid_restore_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionRestoreValidationStatus {
    NotValidated,
    Valid,
    Invalid,
}

impl LocalSessionRestoreValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotValidated => "not_validated",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionRestoreReadBackStatus {
    NotRead,
    PackageReadBackValidated,
    ReadBackRejected,
}

impl LocalSessionRestoreReadBackStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotRead => "not_read",
            Self::PackageReadBackValidated => "package_read_back_validated",
            Self::ReadBackRejected => "read_back_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionRestoreBoundaryStatus {
    LocalRestoreProjectionOnly,
    NoRecoveryPromotion,
    NoReplayRepair,
    NoProductionPersistenceClaim,
    NoReadinessEffect,
    NoReleaseEffect,
    NoDeploymentEffect,
    NoPublicUseEffect,
}

impl LocalSessionRestoreBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalRestoreProjectionOnly => "local_restore_projection_only",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoProductionPersistenceClaim => "no_production_persistence_claim",
            Self::NoReadinessEffect => "no_readiness_effect",
            Self::NoReleaseEffect => "no_release_effect",
            Self::NoDeploymentEffect => "no_deployment_effect",
            Self::NoPublicUseEffect => "no_public_use_effect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalSessionRestoreError {
    NoPackageSelected,
    PackageReadFailed,
    PackageParseFailed,
    PackageValidationFailed,
    InvalidPackageClassification,
    InvalidProductionClassification,
    MissingRequiredPackageSection,
    MissingAbsenceMarker,
    ReadinessClaimDetected,
    ReleaseClaimDetected,
    DeploymentClaimDetected,
    PublicUseClaimDetected,
    ProviderTrustClaimDetected,
    CandidateApprovalClaimDetected,
    ActionExecutionClaimDetected,
    ReplayRepairClaimDetected,
    RecoveryPromotionClaimDetected,
    NondeterministicRestoreProjection,
}

impl LocalSessionRestoreError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoPackageSelected => "no_package_selected",
            Self::PackageReadFailed => "package_read_failed",
            Self::PackageParseFailed => "package_parse_failed",
            Self::PackageValidationFailed => "package_validation_failed",
            Self::InvalidPackageClassification => "invalid_package_classification",
            Self::InvalidProductionClassification => "invalid_production_classification",
            Self::MissingRequiredPackageSection => "missing_required_package_section",
            Self::MissingAbsenceMarker => "missing_absence_marker",
            Self::ReadinessClaimDetected => "readiness_claim_detected",
            Self::ReleaseClaimDetected => "release_claim_detected",
            Self::DeploymentClaimDetected => "deployment_claim_detected",
            Self::PublicUseClaimDetected => "public_use_claim_detected",
            Self::ProviderTrustClaimDetected => "provider_trust_claim_detected",
            Self::CandidateApprovalClaimDetected => "candidate_approval_claim_detected",
            Self::ActionExecutionClaimDetected => "action_execution_claim_detected",
            Self::ReplayRepairClaimDetected => "replay_repair_claim_detected",
            Self::RecoveryPromotionClaimDetected => "recovery_promotion_claim_detected",
            Self::NondeterministicRestoreProjection => "nondeterministic_restore_projection",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalSessionRestoreRequest {
    ExplicitPackagePayload(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionRestoreCandidate {
    pub package_id: Option<String>,
    pub package_version: Option<String>,
    pub package_classification: Option<String>,
    pub production_classification: Option<String>,
    pub read_back_status: LocalSessionRestoreReadBackStatus,
    pub validation_status: LocalSessionRestoreValidationStatus,
    pub errors: Vec<LocalSessionRestoreError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSessionRestoreProjection {
    pub status: LocalSessionRestoreStatus,
    pub package_id: Option<String>,
    pub package_version: Option<String>,
    pub package_classification: Option<String>,
    pub production_classification: Option<String>,
    pub validation_status: LocalSessionRestoreValidationStatus,
    pub read_back_status: LocalSessionRestoreReadBackStatus,
    pub errors: Vec<LocalSessionRestoreError>,
    pub included_section_summary: Vec<String>,
    pub absence_marker_summary: Vec<String>,
    pub boundary_status: Vec<LocalSessionRestoreBoundaryStatus>,
    pub local_only_note: String,
    pub read_back_note: String,
    pub preview_boundary_note: String,
    pub restored_projection_note: String,
    pub remote_background_note: String,
}

pub fn initial_local_session_history_projection() -> LocalSessionHistoryProjection {
    LocalSessionHistoryProjection {
        status: LocalSessionHistoryStatus::NoSessionHistory,
        entries: Vec::new(),
        selected_package_id: None,
        boundary_note: "Session history is derived only from explicit local package entries; No automatic filesystem scanning.".to_string(),
    }
}

pub fn initial_local_session_restore_projection() -> LocalSessionRestoreProjection {
    LocalSessionRestoreProjection {
        status: LocalSessionRestoreStatus::RestoreNotRequested,
        package_id: None,
        package_version: None,
        package_classification: None,
        production_classification: None,
        validation_status: LocalSessionRestoreValidationStatus::NotValidated,
        read_back_status: LocalSessionRestoreReadBackStatus::NotRead,
        errors: Vec::new(),
        included_section_summary: Vec::new(),
        absence_marker_summary: local_session_package_absence_markers().marker_summary,
        boundary_status: local_session_restore_boundary_statuses(),
        local_only_note: "Session restore is local-only and non-production.".to_string(),
        read_back_note: "Read-back validation checks package structure; it is not restore authority.".to_string(),
        preview_boundary_note: "Restore preview does not repair replay or promote recovery.".to_string(),
        restored_projection_note: "Restored session projection does not imply readiness, release, deployment, or public use.".to_string(),
        remote_background_note: "No remote sync or background restore is active.".to_string(),
    }
}

fn local_session_restore_boundary_statuses() -> Vec<LocalSessionRestoreBoundaryStatus> {
    vec![
        LocalSessionRestoreBoundaryStatus::LocalRestoreProjectionOnly,
        LocalSessionRestoreBoundaryStatus::NoRecoveryPromotion,
        LocalSessionRestoreBoundaryStatus::NoReplayRepair,
        LocalSessionRestoreBoundaryStatus::NoProductionPersistenceClaim,
        LocalSessionRestoreBoundaryStatus::NoReadinessEffect,
        LocalSessionRestoreBoundaryStatus::NoReleaseEffect,
        LocalSessionRestoreBoundaryStatus::NoDeploymentEffect,
        LocalSessionRestoreBoundaryStatus::NoPublicUseEffect,
    ]
}

pub fn project_local_session_history(
    packages: &[LocalSessionPackage],
) -> LocalSessionHistoryProjection {
    let entries = packages
        .iter()
        .map(|package| {
            let projection = project_local_session_package_status(
                Some(package),
                Some(LocalSessionPackageValidationStatus::Valid),
            );
            LocalSessionHistoryEntry {
                package_id: package.metadata.package_id.clone(),
                package_version: projection.package_version,
                package_classification: projection.package_classification,
                production_classification: projection.production_classification,
                package_status: projection.status,
                validation_status: projection.validation_status,
                read_back_validation_status: projection.read_back_validation_status,
                included_section_summary: projection.included_section_summary,
                absence_marker_summary: projection.absence_marker_summary,
            }
        })
        .collect::<Vec<_>>();
    LocalSessionHistoryProjection {
        status: if entries.is_empty() {
            LocalSessionHistoryStatus::NoSessionHistory
        } else {
            LocalSessionHistoryStatus::SessionHistoryProjected
        },
        selected_package_id: entries.first().map(|entry| entry.package_id.clone()),
        entries,
        boundary_note: "Session history is derived only from explicit local package entries; No automatic filesystem scanning.".to_string(),
    }
}

fn package_content_has_required_restore_sections(content: &str) -> bool {
    let required = [
        "ajentic_local_session_package",
        "package_id",
        "package_version",
        "package_classification",
        "production_classification",
        "package_status",
        "validation_status",
        "content_digest",
        "provider_configuration_projection",
        "provider_execution_result_projection",
        "provider_output_validation_projection",
        "provider_output_review_projection",
        "staged_candidate_conversion_proposal_projection",
        "staged_candidate_conversion_validation_projection",
        "candidate_review_surface_projection",
        "operator_candidate_decision_projection",
        "local_decision_ledger_projection",
        "replay_status_projection",
        "local_session_evidence_export_projection",
        "phase_150_handoff_context_projection",
        "no_release_marker",
        "no_deployment_marker",
        "no_readiness_marker",
        "absence_markers",
    ];
    let keys = content
        .lines()
        .filter_map(|line| line.split_once('=').map(|(key, _)| key))
        .collect::<std::collections::BTreeSet<_>>();
    required.iter().all(|key| keys.contains(key))
}

fn local_session_restore_claim_errors(text: &str) -> Vec<LocalSessionRestoreError> {
    let text = text.to_ascii_lowercase();
    let mut errors = Vec::new();
    if ["claim:readiness_approved", "claim:production_ready"]
        .iter()
        .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionRestoreError::ReadinessClaimDetected);
    }
    if [
        "claim:release_candidate_approved",
        "claim:production candidate status approved",
        "claim:github_release_created",
        "claim:release_tag_created",
        "claim:signing_enabled",
        "claim:publishing_enabled",
        "claim:installer_created",
        "claim:update_channel_enabled",
        "claim:public_download",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionRestoreError::ReleaseClaimDetected);
    }
    if ["claim:deployment_enabled"]
        .iter()
        .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionRestoreError::DeploymentClaimDetected);
    }
    if ["claim:public_use_approved"]
        .iter()
        .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionRestoreError::PublicUseClaimDetected);
    }
    if [
        "claim:provider_trusted",
        "claim:trusted_provider_output",
        "claim:provider trust granted",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionRestoreError::ProviderTrustClaimDetected);
    }
    if [
        "claim:candidate_approved",
        "claim:candidate approval granted",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionRestoreError::CandidateApprovalClaimDetected);
    }
    if [
        "claim:action_executed",
        "claim:execute action",
        "claim:action execution enabled",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionRestoreError::ActionExecutionClaimDetected);
    }
    if ["claim:replay_repaired", "claim:repair replay"]
        .iter()
        .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionRestoreError::ReplayRepairClaimDetected);
    }
    if ["claim:recovery_promoted", "claim:promote recovery"]
        .iter()
        .any(|needle| text.contains(needle))
    {
        errors.push(LocalSessionRestoreError::RecoveryPromotionClaimDetected);
    }
    errors
}

fn local_session_restore_errors(package: &LocalSessionPackage) -> Vec<LocalSessionRestoreError> {
    let mut errors = Vec::new();
    if package.metadata.package_id.is_empty() {
        errors.push(LocalSessionRestoreError::PackageValidationFailed);
    }
    if package.metadata.package_version.is_empty() {
        errors.push(LocalSessionRestoreError::PackageValidationFailed);
    }
    if package.metadata.package_classification
        != LocalSessionPackageClassification::LocalSessionPackageOnly.code()
    {
        errors.push(LocalSessionRestoreError::InvalidPackageClassification);
    }
    if package.metadata.production_classification
        != LocalSessionPackageProductionClassification::NonProduction.code()
    {
        errors.push(LocalSessionRestoreError::InvalidProductionClassification);
    }
    if validate_local_session_package(package).is_err() {
        errors.push(LocalSessionRestoreError::PackageValidationFailed);
    }
    if local_session_package_validation_errors(package)
        .contains(&LocalSessionPackageValidationError::MissingAbsenceMarker)
    {
        errors.push(LocalSessionRestoreError::MissingAbsenceMarker);
    }
    errors.extend(local_session_restore_claim_errors(&format!(
        "{:?}",
        package
    )));
    errors.sort_by_key(|error| error.code());
    errors.dedup();
    errors
}

pub fn reject_local_session_restore_request(
    error: LocalSessionRestoreError,
) -> LocalSessionRestoreProjection {
    let mut projection = initial_local_session_restore_projection();
    projection.status = LocalSessionRestoreStatus::RestoreRejected;
    projection.validation_status = LocalSessionRestoreValidationStatus::Invalid;
    projection.read_back_status = LocalSessionRestoreReadBackStatus::ReadBackRejected;
    projection.errors = vec![error];
    projection
}

pub fn create_local_session_restore_candidate(
    request: LocalSessionRestoreRequest,
) -> LocalSessionRestoreCandidate {
    let LocalSessionRestoreRequest::ExplicitPackagePayload(content) = request;
    if content.is_empty() {
        return LocalSessionRestoreCandidate {
            package_id: None,
            package_version: None,
            package_classification: None,
            production_classification: None,
            read_back_status: LocalSessionRestoreReadBackStatus::ReadBackRejected,
            validation_status: LocalSessionRestoreValidationStatus::Invalid,
            errors: vec![LocalSessionRestoreError::NoPackageSelected],
        };
    }
    if !package_content_has_required_restore_sections(&content) {
        return LocalSessionRestoreCandidate {
            package_id: None,
            package_version: None,
            package_classification: None,
            production_classification: None,
            read_back_status: LocalSessionRestoreReadBackStatus::ReadBackRejected,
            validation_status: LocalSessionRestoreValidationStatus::Invalid,
            errors: vec![LocalSessionRestoreError::MissingRequiredPackageSection],
        };
    }
    match parse_local_session_package(&content) {
        Ok(package) => {
            let errors = local_session_restore_errors(&package);
            LocalSessionRestoreCandidate {
                package_id: Some(package.metadata.package_id),
                package_version: Some(package.metadata.package_version),
                package_classification: Some(package.metadata.package_classification),
                production_classification: Some(package.metadata.production_classification),
                read_back_status: if errors.is_empty() {
                    LocalSessionRestoreReadBackStatus::PackageReadBackValidated
                } else {
                    LocalSessionRestoreReadBackStatus::ReadBackRejected
                },
                validation_status: if errors.is_empty() {
                    LocalSessionRestoreValidationStatus::Valid
                } else {
                    LocalSessionRestoreValidationStatus::Invalid
                },
                errors,
            }
        }
        Err(_) => LocalSessionRestoreCandidate {
            package_id: None,
            package_version: None,
            package_classification: None,
            production_classification: None,
            read_back_status: LocalSessionRestoreReadBackStatus::ReadBackRejected,
            validation_status: LocalSessionRestoreValidationStatus::Invalid,
            errors: vec![LocalSessionRestoreError::PackageParseFailed],
        },
    }
}

pub fn validate_local_session_restore_candidate(
    candidate: &LocalSessionRestoreCandidate,
) -> Result<(), Vec<LocalSessionRestoreError>> {
    if candidate.errors.is_empty()
        && candidate.validation_status == LocalSessionRestoreValidationStatus::Valid
    {
        Ok(())
    } else {
        Err(candidate.errors.clone())
    }
}

pub fn derive_local_session_restore_preview(
    package: &LocalSessionPackage,
) -> Result<LocalSessionRestoreProjection, Vec<LocalSessionRestoreError>> {
    let errors = local_session_restore_errors(package);
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(LocalSessionRestoreProjection {
        status: LocalSessionRestoreStatus::RestorePreviewProjected,
        package_id: Some(package.metadata.package_id.clone()),
        package_version: Some(package.metadata.package_version.clone()),
        package_classification: Some(package.metadata.package_classification.clone()),
        production_classification: Some(package.metadata.production_classification.clone()),
        validation_status: LocalSessionRestoreValidationStatus::Valid,
        read_back_status: LocalSessionRestoreReadBackStatus::PackageReadBackValidated,
        errors: Vec::new(),
        included_section_summary: local_session_package_included_sections(),
        absence_marker_summary: package.absence_markers.marker_summary.clone(),
        boundary_status: local_session_restore_boundary_statuses(),
        local_only_note: "Session restore is local-only and non-production.".to_string(),
        read_back_note: "Read-back validation checks package structure; it is not restore authority.".to_string(),
        preview_boundary_note: "Restore preview does not repair replay or promote recovery.".to_string(),
        restored_projection_note: "Restored session projection does not imply readiness, release, deployment, or public use.".to_string(),
        remote_background_note: "No remote sync or background restore is active.".to_string(),
    })
}

pub fn derive_local_session_restore_projection(
    package: &LocalSessionPackage,
) -> Result<LocalSessionRestoreProjection, Vec<LocalSessionRestoreError>> {
    let mut projection = derive_local_session_restore_preview(package)?;
    projection.status = LocalSessionRestoreStatus::RestoreProjected;
    Ok(projection)
}

pub fn project_local_session_restore_from_payload(content: &str) -> LocalSessionRestoreProjection {
    if !package_content_has_required_restore_sections(content) {
        let mut projection = reject_local_session_restore_request(
            LocalSessionRestoreError::MissingRequiredPackageSection,
        );
        projection.status = LocalSessionRestoreStatus::InvalidRestoreInput;
        return projection;
    }
    match parse_local_session_package(content) {
        Ok(package) => match derive_local_session_restore_preview(&package) {
            Ok(projection) => projection,
            Err(errors) => {
                let mut projection = initial_local_session_restore_projection();
                projection.status = LocalSessionRestoreStatus::RestoreRejected;
                projection.validation_status = LocalSessionRestoreValidationStatus::Invalid;
                projection.read_back_status = LocalSessionRestoreReadBackStatus::ReadBackRejected;
                projection.errors = errors;
                projection
            }
        },
        Err(_) => {
            reject_local_session_restore_request(LocalSessionRestoreError::PackageParseFailed)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialPackageStatus {
    NotPackaged,
    PackageProjected,
    PackageValidated,
    PackageWritten,
    PackageReadBackValidated,
    PackageRejected,
    InvalidPackageInput,
}

impl ControlledInternalTrialPackageStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotPackaged => "not_packaged",
            Self::PackageProjected => "package_projected",
            Self::PackageValidated => "package_validated",
            Self::PackageWritten => "package_written",
            Self::PackageReadBackValidated => "package_read_back_validated",
            Self::PackageRejected => "package_rejected",
            Self::InvalidPackageInput => "invalid_package_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialPackageValidationStatus {
    NotValidated,
    Valid,
    Invalid,
}

impl ControlledInternalTrialPackageValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotValidated => "not_validated",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialPackageClassification {
    ControlledInternalTrialPackageOnly,
}

impl ControlledInternalTrialPackageClassification {
    pub fn code(&self) -> &'static str {
        "controlled_internal_trial_package_only"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialPackageProductionClassification {
    NonProduction,
}

impl ControlledInternalTrialPackageProductionClassification {
    pub fn code(&self) -> &'static str {
        "non_production"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialPackageDistributionClassification {
    LocalOnlyNonPublic,
}

impl ControlledInternalTrialPackageDistributionClassification {
    pub fn code(&self) -> &'static str {
        "local_only_non_public"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialScope {
    pub scope_id: String,
    pub scope_summary: String,
    pub local_beta_workflow_scope: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialOperator {
    pub operator_id: String,
    pub display_label: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialParticipant {
    pub participant_id: String,
    pub display_label: String,
    pub role: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialStopCondition {
    StopOnProviderTrustClaim,
    StopOnReadinessClaim,
    StopOnReleaseOrDeploymentClaim,
    StopOnPublicUseClaim,
    StopOnActionAuthorizationClaim,
    StopOnReplayRepairOrRecoveryPromotionClaim,
    StopOnPackageValidationFailure,
    StopOnRestoreValidationFailure,
    StopOnWorkflowRejection,
    StopOnOperatorEscalation,
}

impl ControlledInternalTrialStopCondition {
    pub fn code(&self) -> &'static str {
        match self {
            Self::StopOnProviderTrustClaim => "stop_on_provider_trust_claim",
            Self::StopOnReadinessClaim => "stop_on_readiness_claim",
            Self::StopOnReleaseOrDeploymentClaim => "stop_on_release_or_deployment_claim",
            Self::StopOnPublicUseClaim => "stop_on_public_use_claim",
            Self::StopOnActionAuthorizationClaim => "stop_on_action_authorization_claim",
            Self::StopOnReplayRepairOrRecoveryPromotionClaim => {
                "stop_on_replay_repair_or_recovery_promotion_claim"
            }
            Self::StopOnPackageValidationFailure => "stop_on_package_validation_failure",
            Self::StopOnRestoreValidationFailure => "stop_on_restore_validation_failure",
            Self::StopOnWorkflowRejection => "stop_on_workflow_rejection",
            Self::StopOnOperatorEscalation => "stop_on_operator_escalation",
        }
    }
}

pub fn required_controlled_internal_trial_stop_conditions(
) -> Vec<ControlledInternalTrialStopCondition> {
    vec![
        ControlledInternalTrialStopCondition::StopOnProviderTrustClaim,
        ControlledInternalTrialStopCondition::StopOnReadinessClaim,
        ControlledInternalTrialStopCondition::StopOnReleaseOrDeploymentClaim,
        ControlledInternalTrialStopCondition::StopOnPublicUseClaim,
        ControlledInternalTrialStopCondition::StopOnActionAuthorizationClaim,
        ControlledInternalTrialStopCondition::StopOnReplayRepairOrRecoveryPromotionClaim,
        ControlledInternalTrialStopCondition::StopOnPackageValidationFailure,
        ControlledInternalTrialStopCondition::StopOnRestoreValidationFailure,
        ControlledInternalTrialStopCondition::StopOnWorkflowRejection,
        ControlledInternalTrialStopCondition::StopOnOperatorEscalation,
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialIncludedEvidence {
    pub local_beta_workflow_status: String,
    pub provider_output_pipeline_status: String,
    pub local_candidate_materialization_status: String,
    pub operator_decision_status: String,
    pub replay_status_summary: String,
    pub local_evidence_export_summary: String,
    pub local_session_package_summary: String,
    pub restore_history_summary: String,
    pub phase_160_gate_decision_context: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialPackageAbsenceMarkers {
    pub release_artifact_absent: bool,
    pub deployment_artifact_absent: bool,
    pub readiness_approval_absent: bool,
    pub public_use_approval_absent: bool,
    pub production_use_approval_absent: bool,
    pub provider_trust_absent: bool,
    pub action_authorization_absent: bool,
    pub replay_repair_absent: bool,
    pub recovery_promotion_absent: bool,
    pub default_persistence_absent: bool,
    pub marker_summary: Vec<String>,
}

pub fn controlled_internal_trial_package_absence_markers(
) -> ControlledInternalTrialPackageAbsenceMarkers {
    ControlledInternalTrialPackageAbsenceMarkers {
        release_artifact_absent: true,
        deployment_artifact_absent: true,
        readiness_approval_absent: true,
        public_use_approval_absent: true,
        production_use_approval_absent: true,
        provider_trust_absent: true,
        action_authorization_absent: true,
        replay_repair_absent: true,
        recovery_promotion_absent: true,
        default_persistence_absent: true,
        marker_summary: vec![
            "no_release_artifact".to_string(),
            "no_deployment_artifact".to_string(),
            "no_readiness_approval".to_string(),
            "no_public_use_approval".to_string(),
            "no_production_approval".to_string(),
            "no_provider_trust".to_string(),
            "no_action_authorization".to_string(),
            "no_replay_repair".to_string(),
            "no_recovery_promotion".to_string(),
            "no_default_persistence".to_string(),
        ],
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialPackageBoundaryStatus {
    LocalOnlyTrialPackage,
    NonPublicTrialPackage,
    NoReleaseArtifact,
    NoDeploymentArtifact,
    NoReadinessApproval,
    NoReleaseApproval,
    NoProductionApproval,
    NoPublicUseApproval,
    NoProviderTrust,
    NoActionAuthorization,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl ControlledInternalTrialPackageBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalOnlyTrialPackage => "local_only_trial_package",
            Self::NonPublicTrialPackage => "non_public_trial_package",
            Self::NoReleaseArtifact => "no_release_artifact",
            Self::NoDeploymentArtifact => "no_deployment_artifact",
            Self::NoReadinessApproval => "no_readiness_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoActionAuthorization => "no_action_authorization",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
        }
    }
}

pub fn controlled_internal_trial_package_boundary_statuses(
) -> Vec<ControlledInternalTrialPackageBoundaryStatus> {
    vec![
        ControlledInternalTrialPackageBoundaryStatus::LocalOnlyTrialPackage,
        ControlledInternalTrialPackageBoundaryStatus::NonPublicTrialPackage,
        ControlledInternalTrialPackageBoundaryStatus::NoReleaseArtifact,
        ControlledInternalTrialPackageBoundaryStatus::NoDeploymentArtifact,
        ControlledInternalTrialPackageBoundaryStatus::NoReadinessApproval,
        ControlledInternalTrialPackageBoundaryStatus::NoReleaseApproval,
        ControlledInternalTrialPackageBoundaryStatus::NoProductionApproval,
        ControlledInternalTrialPackageBoundaryStatus::NoPublicUseApproval,
        ControlledInternalTrialPackageBoundaryStatus::NoProviderTrust,
        ControlledInternalTrialPackageBoundaryStatus::NoActionAuthorization,
        ControlledInternalTrialPackageBoundaryStatus::NoReplayRepair,
        ControlledInternalTrialPackageBoundaryStatus::NoRecoveryPromotion,
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialPackageValidationError {
    MissingPackageId,
    MissingPackageVersion,
    InvalidPackageClassification,
    InvalidProductionClassification,
    InvalidDistributionClassification,
    MissingTrialScope,
    MissingNamedOperatorOrParticipant,
    MissingStopConditionMarkers,
    MissingAbsenceMarker,
    ForbiddenReleaseDeploymentReadinessPublicOrProductionUseClaim,
    ForbiddenProviderTrustActionReplayRepairOrRecoveryPromotionClaim,
    DeterministicContentMismatch,
    MalformedPackageInput,
}

impl ControlledInternalTrialPackageValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingPackageId => "missing_package_id",
            Self::MissingPackageVersion => "missing_package_version",
            Self::InvalidPackageClassification => "invalid_package_classification",
            Self::InvalidProductionClassification => "invalid_production_classification",
            Self::InvalidDistributionClassification => "invalid_distribution_classification",
            Self::MissingTrialScope => "missing_trial_scope",
            Self::MissingNamedOperatorOrParticipant => "missing_named_operator_or_participant",
            Self::MissingStopConditionMarkers => "missing_stop_condition_markers",
            Self::MissingAbsenceMarker => "missing_absence_marker",
            Self::ForbiddenReleaseDeploymentReadinessPublicOrProductionUseClaim => {
                "forbidden_release_deployment_readiness_public_or_production_use_claim"
            }
            Self::ForbiddenProviderTrustActionReplayRepairOrRecoveryPromotionClaim => {
                "forbidden_provider_trust_action_replay_repair_or_recovery_promotion_claim"
            }
            Self::DeterministicContentMismatch => "deterministic_content_mismatch",
            Self::MalformedPackageInput => "malformed_package_input",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialPackageMetadata {
    pub package_id: String,
    pub package_version: String,
    pub package_classification: String,
    pub production_classification: String,
    pub distribution_classification: String,
    pub package_status: ControlledInternalTrialPackageStatus,
    pub validation_status: ControlledInternalTrialPackageValidationStatus,
    pub content_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialPackagePayload {
    pub trial_scope: Option<ControlledInternalTrialScope>,
    pub named_internal_operators: Vec<ControlledInternalTrialOperator>,
    pub trial_participants: Vec<ControlledInternalTrialParticipant>,
    pub stop_conditions: Vec<ControlledInternalTrialStopCondition>,
    pub included_evidence: ControlledInternalTrialIncludedEvidence,
    pub boundary_status: Vec<ControlledInternalTrialPackageBoundaryStatus>,
    pub no_release_marker: String,
    pub no_deployment_marker: String,
    pub no_readiness_marker: String,
    pub no_public_use_marker: String,
    pub no_production_use_marker: String,
    pub no_provider_trust_marker: String,
    pub no_action_authorization_marker: String,
    pub no_replay_repair_marker: String,
    pub no_recovery_promotion_marker: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialPackage {
    pub metadata: ControlledInternalTrialPackageMetadata,
    pub payload: ControlledInternalTrialPackagePayload,
    pub absence_markers: ControlledInternalTrialPackageAbsenceMarkers,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialPackageProjection {
    pub status: ControlledInternalTrialPackageStatus,
    pub package_id: Option<String>,
    pub package_version: String,
    pub package_classification: String,
    pub production_classification: String,
    pub distribution_classification: String,
    pub trial_scope_summary: String,
    pub named_operator_participant_summary: Vec<String>,
    pub stop_condition_summary: Vec<String>,
    pub included_evidence_summary: Vec<String>,
    pub absence_marker_summary: Vec<String>,
    pub validation_status: ControlledInternalTrialPackageValidationStatus,
    pub validation_errors: Vec<String>,
    pub read_back_validation_status: Option<ControlledInternalTrialPackageValidationStatus>,
    pub boundary_status: Vec<ControlledInternalTrialPackageBoundaryStatus>,
    pub local_only_non_public_note: String,
    pub release_boundary_note: String,
    pub deployment_readiness_boundary_note: String,
    pub public_production_boundary_note: String,
    pub stop_condition_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialPackageWriteResult {
    pub status: ControlledInternalTrialPackageStatus,
    pub path: String,
    pub bytes_written: usize,
    pub projection: ControlledInternalTrialPackageProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialPackageReadResult {
    pub status: ControlledInternalTrialPackageStatus,
    pub path: String,
    pub package: Option<ControlledInternalTrialPackage>,
    pub projection: ControlledInternalTrialPackageProjection,
}

pub const CONTROLLED_INTERNAL_TRIAL_PACKAGE_VERSION: &str = "controlled-internal-trial-package-v1";

pub fn initial_controlled_internal_trial_package_projection(
) -> ControlledInternalTrialPackageProjection {
    ControlledInternalTrialPackageProjection {
        status: ControlledInternalTrialPackageStatus::NotPackaged,
        package_id: None,
        package_version: CONTROLLED_INTERNAL_TRIAL_PACKAGE_VERSION.to_string(),
        package_classification:
            ControlledInternalTrialPackageClassification::ControlledInternalTrialPackageOnly
                .code()
                .to_string(),
        production_classification:
            ControlledInternalTrialPackageProductionClassification::NonProduction
                .code()
                .to_string(),
        distribution_classification:
            ControlledInternalTrialPackageDistributionClassification::LocalOnlyNonPublic
                .code()
                .to_string(),
        trial_scope_summary: "trial scope not provided".to_string(),
        named_operator_participant_summary: Vec::new(),
        stop_condition_summary: required_controlled_internal_trial_stop_conditions()
            .into_iter()
            .map(|condition| condition.code().to_string())
            .collect(),
        included_evidence_summary: Vec::new(),
        absence_marker_summary: controlled_internal_trial_package_absence_markers().marker_summary,
        validation_status: ControlledInternalTrialPackageValidationStatus::NotValidated,
        validation_errors: Vec::new(),
        read_back_validation_status: None,
        boundary_status: controlled_internal_trial_package_boundary_statuses(),
        local_only_non_public_note:
            "Controlled internal trial package is local-only and non-public.".to_string(),
        release_boundary_note: "This package is not a release artifact.".to_string(),
        deployment_readiness_boundary_note: "This package is not deployment or readiness evidence."
            .to_string(),
        public_production_boundary_note:
            "This package does not approve public/general use or production use.".to_string(),
        stop_condition_note:
            "Stop conditions are trial metadata; they do not automate enforcement in Phase 161."
                .to_string(),
    }
}

fn controlled_internal_trial_included_evidence_from_state(
    state: &LocalOperatorShellState,
) -> ControlledInternalTrialIncludedEvidence {
    ControlledInternalTrialIncludedEvidence {
        local_beta_workflow_status: state
            .complete_local_operator_workflow
            .status
            .code()
            .to_string(),
        provider_output_pipeline_status: state
            .local_provider_output_pipeline
            .status
            .code()
            .to_string(),
        local_candidate_materialization_status: state
            .local_candidate_output
            .status
            .code()
            .to_string(),
        operator_decision_status: state.operator_candidate_decision.status.code().to_string(),
        replay_status_summary: state.run.decision_replay.replay_status.code().to_string(),
        local_evidence_export_summary: state
            .local_session_evidence_export
            .export_status
            .code()
            .to_string(),
        local_session_package_summary: state
            .local_session_package_projection
            .status
            .code()
            .to_string(),
        restore_history_summary: format!(
            "{} / {}",
            state.local_session_restore_projection.status.code(),
            state.local_session_history_projection.status.code()
        ),
        phase_160_gate_decision_context:
            "Phase 160 gate decision: proceed_with_caveats_to_controlled_internal_trial_packaging"
                .to_string(),
    }
}

fn controlled_internal_trial_package_content_basis(
    payload: &ControlledInternalTrialPackagePayload,
    markers: &ControlledInternalTrialPackageAbsenceMarkers,
) -> String {
    format!(
        "version={}|classification=controlled_internal_trial_package_only|production=non_production|distribution=local_only_non_public|payload={:?}|absence={:?}",
        CONTROLLED_INTERNAL_TRIAL_PACKAGE_VERSION, payload, markers
    )
}

fn stable_controlled_internal_trial_package_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub fn derive_controlled_internal_trial_package(
    state: &LocalOperatorShellState,
    trial_scope: ControlledInternalTrialScope,
    named_internal_operators: Vec<ControlledInternalTrialOperator>,
    trial_participants: Vec<ControlledInternalTrialParticipant>,
    stop_conditions: Vec<ControlledInternalTrialStopCondition>,
) -> ControlledInternalTrialPackage {
    let absence_markers = controlled_internal_trial_package_absence_markers();
    let payload = ControlledInternalTrialPackagePayload {
        trial_scope: Some(trial_scope),
        named_internal_operators,
        trial_participants,
        stop_conditions,
        included_evidence: controlled_internal_trial_included_evidence_from_state(state),
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
    };
    let content_digest = stable_controlled_internal_trial_package_digest(
        &controlled_internal_trial_package_content_basis(&payload, &absence_markers),
    );
    ControlledInternalTrialPackage {
        metadata: ControlledInternalTrialPackageMetadata {
            package_id: format!("controlled-internal-trial-package-{content_digest}"),
            package_version: CONTROLLED_INTERNAL_TRIAL_PACKAGE_VERSION.to_string(),
            package_classification:
                ControlledInternalTrialPackageClassification::ControlledInternalTrialPackageOnly
                    .code()
                    .to_string(),
            production_classification:
                ControlledInternalTrialPackageProductionClassification::NonProduction
                    .code()
                    .to_string(),
            distribution_classification:
                ControlledInternalTrialPackageDistributionClassification::LocalOnlyNonPublic
                    .code()
                    .to_string(),
            package_status: ControlledInternalTrialPackageStatus::PackageProjected,
            validation_status: ControlledInternalTrialPackageValidationStatus::Valid,
            content_digest,
        },
        payload,
        absence_markers,
    }
}

fn controlled_internal_trial_package_validation_errors(
    package: &ControlledInternalTrialPackage,
) -> Vec<ControlledInternalTrialPackageValidationError> {
    let mut errors = Vec::new();
    if package.metadata.package_id.is_empty() {
        errors.push(ControlledInternalTrialPackageValidationError::MissingPackageId);
    }
    if package.metadata.package_version.is_empty() {
        errors.push(ControlledInternalTrialPackageValidationError::MissingPackageVersion);
    }
    if package.metadata.package_classification
        != ControlledInternalTrialPackageClassification::ControlledInternalTrialPackageOnly.code()
    {
        errors.push(ControlledInternalTrialPackageValidationError::InvalidPackageClassification);
    }
    if package.metadata.production_classification
        != ControlledInternalTrialPackageProductionClassification::NonProduction.code()
    {
        errors.push(ControlledInternalTrialPackageValidationError::InvalidProductionClassification);
    }
    if package.metadata.distribution_classification
        != ControlledInternalTrialPackageDistributionClassification::LocalOnlyNonPublic.code()
    {
        errors
            .push(ControlledInternalTrialPackageValidationError::InvalidDistributionClassification);
    }
    match package.payload.trial_scope.as_ref() {
        Some(scope)
            if !scope.scope_id.is_empty()
                && !scope.scope_summary.is_empty()
                && !scope.local_beta_workflow_scope.is_empty() => {}
        _ => errors.push(ControlledInternalTrialPackageValidationError::MissingTrialScope),
    }
    if package.payload.named_internal_operators.is_empty()
        || package.payload.trial_participants.is_empty()
        || package
            .payload
            .named_internal_operators
            .iter()
            .any(|operator| {
                operator.operator_id.is_empty()
                    || operator.display_label.is_empty()
                    || operator.role.is_empty()
            })
        || package
            .payload
            .trial_participants
            .iter()
            .any(|participant| {
                participant.participant_id.is_empty()
                    || participant.display_label.is_empty()
                    || participant.role.is_empty()
            })
    {
        errors
            .push(ControlledInternalTrialPackageValidationError::MissingNamedOperatorOrParticipant);
    }
    let required = required_controlled_internal_trial_stop_conditions();
    if package.payload.stop_conditions.len() < required.len()
        || required
            .iter()
            .any(|condition| !package.payload.stop_conditions.contains(condition))
    {
        errors.push(ControlledInternalTrialPackageValidationError::MissingStopConditionMarkers);
    }
    let markers = &package.absence_markers;
    if !markers.release_artifact_absent
        || !markers.deployment_artifact_absent
        || !markers.readiness_approval_absent
        || !markers.public_use_approval_absent
        || !markers.production_use_approval_absent
        || !markers.provider_trust_absent
        || !markers.action_authorization_absent
        || !markers.replay_repair_absent
        || !markers.recovery_promotion_absent
        || !markers.default_persistence_absent
        || markers.marker_summary.is_empty()
    {
        errors.push(ControlledInternalTrialPackageValidationError::MissingAbsenceMarker);
    }
    let text = format!("{:?}", package).to_ascii_lowercase();
    if [
        "claim:readiness_approved",
        "claim:release_candidate_approved",
        "claim:deployment_enabled",
        "claim:public_use_approved",
        "claim:production_use_approved",
        "claim:production_persistence_enabled",
        "claim:release artifact",
        "claim:deployment artifact",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(ControlledInternalTrialPackageValidationError::ForbiddenReleaseDeploymentReadinessPublicOrProductionUseClaim);
    }
    if [
        "claim:provider_trusted",
        "claim:provider trust granted",
        "claim:action_authorized",
        "claim:action_executed",
        "claim:replay_repaired",
        "claim:recovery_promoted",
    ]
    .iter()
    .any(|needle| text.contains(needle))
    {
        errors.push(ControlledInternalTrialPackageValidationError::ForbiddenProviderTrustActionReplayRepairOrRecoveryPromotionClaim);
    }
    let expected_digest = stable_controlled_internal_trial_package_digest(
        &controlled_internal_trial_package_content_basis(
            &package.payload,
            &package.absence_markers,
        ),
    );
    let expected_id = format!("controlled-internal-trial-package-{expected_digest}");
    if package.metadata.content_digest != expected_digest
        || (!package.metadata.package_id.is_empty() && package.metadata.package_id != expected_id)
    {
        errors.push(ControlledInternalTrialPackageValidationError::DeterministicContentMismatch);
    }
    errors
}

pub fn validate_controlled_internal_trial_package(
    package: &ControlledInternalTrialPackage,
) -> Result<(), Vec<ControlledInternalTrialPackageValidationError>> {
    let errors = controlled_internal_trial_package_validation_errors(package);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn controlled_internal_trial_included_evidence_summary(
    evidence: &ControlledInternalTrialIncludedEvidence,
) -> Vec<String> {
    vec![
        format!(
            "local beta workflow status: {}",
            evidence.local_beta_workflow_status
        ),
        format!(
            "provider output pipeline status: {}",
            evidence.provider_output_pipeline_status
        ),
        format!(
            "local candidate materialization status: {}",
            evidence.local_candidate_materialization_status
        ),
        format!(
            "operator decision status: {}",
            evidence.operator_decision_status
        ),
        format!("replay/status summary: {}", evidence.replay_status_summary),
        format!(
            "local evidence export summary: {}",
            evidence.local_evidence_export_summary
        ),
        format!(
            "local session package summary: {}",
            evidence.local_session_package_summary
        ),
        format!(
            "restore/history summary: {}",
            evidence.restore_history_summary
        ),
        evidence.phase_160_gate_decision_context.clone(),
    ]
}

pub fn project_controlled_internal_trial_package_status(
    package: Option<&ControlledInternalTrialPackage>,
    read_back_status: Option<ControlledInternalTrialPackageValidationStatus>,
) -> ControlledInternalTrialPackageProjection {
    match package {
        None => initial_controlled_internal_trial_package_projection(),
        Some(package) => {
            let errors = controlled_internal_trial_package_validation_errors(package);
            let validation_status = if errors.is_empty() {
                ControlledInternalTrialPackageValidationStatus::Valid
            } else {
                ControlledInternalTrialPackageValidationStatus::Invalid
            };
            let scope_summary = package
                .payload
                .trial_scope
                .as_ref()
                .map(|scope| format!("{}: {}", scope.scope_id, scope.scope_summary))
                .unwrap_or_else(|| "trial scope not provided".to_string());
            let mut named = package
                .payload
                .named_internal_operators
                .iter()
                .map(|operator| format!("operator:{}:{}", operator.operator_id, operator.role))
                .collect::<Vec<_>>();
            named.extend(
                package
                    .payload
                    .trial_participants
                    .iter()
                    .map(|participant| {
                        format!(
                            "participant:{}:{}",
                            participant.participant_id, participant.role
                        )
                    }),
            );
            ControlledInternalTrialPackageProjection {
                status: if errors.is_empty() { package.metadata.package_status } else { ControlledInternalTrialPackageStatus::PackageRejected },
                package_id: Some(package.metadata.package_id.clone()),
                package_version: package.metadata.package_version.clone(),
                package_classification: package.metadata.package_classification.clone(),
                production_classification: package.metadata.production_classification.clone(),
                distribution_classification: package.metadata.distribution_classification.clone(),
                trial_scope_summary: scope_summary,
                named_operator_participant_summary: named,
                stop_condition_summary: package.payload.stop_conditions.iter().map(|condition| condition.code().to_string()).collect(),
                included_evidence_summary: controlled_internal_trial_included_evidence_summary(&package.payload.included_evidence),
                absence_marker_summary: package.absence_markers.marker_summary.clone(),
                validation_status,
                validation_errors: errors.into_iter().map(|error| error.code().to_string()).collect(),
                read_back_validation_status: read_back_status,
                boundary_status: package.payload.boundary_status.clone(),
                local_only_non_public_note: "Controlled internal trial package is local-only and non-public.".to_string(),
                release_boundary_note: "This package is not a release artifact.".to_string(),
                deployment_readiness_boundary_note: "This package is not deployment or readiness evidence.".to_string(),
                public_production_boundary_note: "This package does not approve public/general use or production use.".to_string(),
                stop_condition_note: "Stop conditions are trial metadata; they do not automate enforcement in Phase 161.".to_string(),
            }
        }
    }
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
    std::fs::write(caller_provided_path, content.as_bytes())
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
    let content = std::fs::read_to_string(caller_provided_path)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteLocalOperatorWorkflowStatus {
    NotStarted,
    InProgress,
    Blocked,
    Rejected,
    LocalCandidateMaterialized,
    CompleteLocalWorkflowProjected,
}

impl CompleteLocalOperatorWorkflowStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::InProgress => "in_progress",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
            Self::LocalCandidateMaterialized => "local_candidate_materialized",
            Self::CompleteLocalWorkflowProjected => "complete_local_workflow_projected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteLocalOperatorWorkflowStepStatus {
    NotStarted,
    Available,
    Completed,
    Blocked,
    Rejected,
    NotApplicable,
}

impl CompleteLocalOperatorWorkflowStepStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Available => "available",
            Self::Completed => "completed",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteLocalOperatorWorkflowStepKind {
    ProviderAdapterConfigured,
    AdapterDryRunAvailable,
    ConstrainedInvocationCompleted,
    ProviderOutputPipelineProjected,
    ProviderOutputValidated,
    ProviderOutputReviewed,
    StagedProposalCreated,
    StagedProposalValidated,
    CandidateReviewProjected,
    OperatorDecisionRecorded,
    LocalCandidateMaterialized,
    ReplayStatusProjected,
    LocalEvidenceExportProjected,
    SessionPackageProjected,
    RestoreStatusProjected,
}

impl CompleteLocalOperatorWorkflowStepKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ProviderAdapterConfigured => "provider_adapter_configured",
            Self::AdapterDryRunAvailable => "adapter_dry_run_available",
            Self::ConstrainedInvocationCompleted => "constrained_invocation_completed",
            Self::ProviderOutputPipelineProjected => "provider_output_pipeline_projected",
            Self::ProviderOutputValidated => "provider_output_validated",
            Self::ProviderOutputReviewed => "provider_output_reviewed",
            Self::StagedProposalCreated => "staged_proposal_created",
            Self::StagedProposalValidated => "staged_proposal_validated",
            Self::CandidateReviewProjected => "candidate_review_projected",
            Self::OperatorDecisionRecorded => "operator_decision_recorded",
            Self::LocalCandidateMaterialized => "local_candidate_materialized",
            Self::ReplayStatusProjected => "replay_status_projected",
            Self::LocalEvidenceExportProjected => "local_evidence_export_projected",
            Self::SessionPackageProjected => "session_package_projected",
            Self::RestoreStatusProjected => "restore_status_projected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteLocalOperatorWorkflowError {
    AdapterNotConfigured,
    InvocationMissing,
    InvocationRejected,
    ProviderPipelineBlocked,
    ProviderOutputValidationMissing,
    ProviderOutputValidationRejected,
    ProviderOutputReviewMissing,
    StagedProposalMissing,
    StagedProposalValidationMissing,
    StagedProposalValidationRejected,
    CandidateReviewMissing,
    OperatorDecisionMissing,
    OperatorDecisionRejected,
    LocalCandidateNotMaterialized,
    ReplayStatusMissing,
    EvidenceExportMissing,
    SessionPackageMissing,
    RestoreStatusMissing,
}

impl CompleteLocalOperatorWorkflowError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AdapterNotConfigured => "adapter_not_configured",
            Self::InvocationMissing => "invocation_missing",
            Self::InvocationRejected => "invocation_rejected",
            Self::ProviderPipelineBlocked => "provider_pipeline_blocked",
            Self::ProviderOutputValidationMissing => "provider_output_validation_missing",
            Self::ProviderOutputValidationRejected => "provider_output_validation_rejected",
            Self::ProviderOutputReviewMissing => "provider_output_review_missing",
            Self::StagedProposalMissing => "staged_proposal_missing",
            Self::StagedProposalValidationMissing => "staged_proposal_validation_missing",
            Self::StagedProposalValidationRejected => "staged_proposal_validation_rejected",
            Self::CandidateReviewMissing => "candidate_review_missing",
            Self::OperatorDecisionMissing => "operator_decision_missing",
            Self::OperatorDecisionRejected => "operator_decision_rejected",
            Self::LocalCandidateNotMaterialized => "local_candidate_not_materialized",
            Self::ReplayStatusMissing => "replay_status_missing",
            Self::EvidenceExportMissing => "evidence_export_missing",
            Self::SessionPackageMissing => "session_package_missing",
            Self::RestoreStatusMissing => "restore_status_missing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompleteLocalOperatorWorkflowBoundaryStatus {
    LocalBetaWorkflowOnly,
    NoProviderTrust,
    NoProductionApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoActionExecution,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl CompleteLocalOperatorWorkflowBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalBetaWorkflowOnly => "local_beta_workflow_only",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteLocalOperatorWorkflowStep {
    pub step: CompleteLocalOperatorWorkflowStepKind,
    pub status: CompleteLocalOperatorWorkflowStepStatus,
    pub error: Option<CompleteLocalOperatorWorkflowError>,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteLocalOperatorWorkflowEvidenceSummary {
    pub provider_output_pipeline_status: String,
    pub local_candidate_materialization_status: String,
    pub replay_status: String,
    pub local_evidence_export_status: String,
    pub session_package_status: String,
    pub session_history_status: String,
    pub restore_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteLocalOperatorWorkflowCapabilitySurface {
    pub local_only: bool,
    pub non_production: bool,
    pub provider_trust_granted: bool,
    pub action_execution_authorized: bool,
    pub readiness_approved: bool,
    pub release_approved: bool,
    pub deployment_approved: bool,
    pub public_use_approved: bool,
    pub replay_repair_performed: bool,
    pub recovery_promotion_performed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompleteLocalOperatorWorkflowProjection {
    pub status: CompleteLocalOperatorWorkflowStatus,
    pub classification: String,
    pub current_step: Option<CompleteLocalOperatorWorkflowStepKind>,
    pub next_required_step: Option<CompleteLocalOperatorWorkflowStepKind>,
    pub current_blocking_step: Option<CompleteLocalOperatorWorkflowStepKind>,
    pub current_error: Option<CompleteLocalOperatorWorkflowError>,
    pub steps: Vec<CompleteLocalOperatorWorkflowStep>,
    pub rejection_reasons: Vec<String>,
    pub evidence_summary: CompleteLocalOperatorWorkflowEvidenceSummary,
    pub boundary_statuses: Vec<CompleteLocalOperatorWorkflowBoundaryStatus>,
    pub capability_surface: CompleteLocalOperatorWorkflowCapabilitySurface,
    pub local_only_note: String,
    pub no_authority_note: String,
}

pub fn complete_local_operator_workflow_boundary_statuses(
) -> Vec<CompleteLocalOperatorWorkflowBoundaryStatus> {
    vec![
        CompleteLocalOperatorWorkflowBoundaryStatus::LocalBetaWorkflowOnly,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoProviderTrust,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoProductionApproval,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoReleaseApproval,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoDeploymentApproval,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoPublicUseApproval,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoActionExecution,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoReplayRepair,
        CompleteLocalOperatorWorkflowBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub fn complete_local_operator_workflow_step_order() -> Vec<CompleteLocalOperatorWorkflowStepKind> {
    vec![
        CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured,
        CompleteLocalOperatorWorkflowStepKind::AdapterDryRunAvailable,
        CompleteLocalOperatorWorkflowStepKind::ConstrainedInvocationCompleted,
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputPipelineProjected,
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputValidated,
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputReviewed,
        CompleteLocalOperatorWorkflowStepKind::StagedProposalCreated,
        CompleteLocalOperatorWorkflowStepKind::StagedProposalValidated,
        CompleteLocalOperatorWorkflowStepKind::CandidateReviewProjected,
        CompleteLocalOperatorWorkflowStepKind::OperatorDecisionRecorded,
        CompleteLocalOperatorWorkflowStepKind::LocalCandidateMaterialized,
        CompleteLocalOperatorWorkflowStepKind::ReplayStatusProjected,
        CompleteLocalOperatorWorkflowStepKind::LocalEvidenceExportProjected,
        CompleteLocalOperatorWorkflowStepKind::SessionPackageProjected,
        CompleteLocalOperatorWorkflowStepKind::RestoreStatusProjected,
    ]
}

pub fn initial_complete_local_operator_workflow_projection(
) -> CompleteLocalOperatorWorkflowProjection {
    let steps = complete_local_operator_workflow_step_order()
        .into_iter()
        .map(|step| {
            let (status, error, summary) = if step == CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured {
                (
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::AdapterNotConfigured),
                    "Provider adapter configuration is required before the local workflow can continue.".to_string(),
                )
            } else {
                (
                    CompleteLocalOperatorWorkflowStepStatus::NotStarted,
                    None,
                    "Waiting for earlier local workflow steps.".to_string(),
                )
            };
            CompleteLocalOperatorWorkflowStep { step, status, error, summary }
        })
        .collect();
    CompleteLocalOperatorWorkflowProjection {
        status: CompleteLocalOperatorWorkflowStatus::Blocked,
        classification: "local_beta_workflow_only".to_string(),
        current_step: Some(CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured),
        next_required_step: Some(CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured),
        current_blocking_step: Some(CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured),
        current_error: Some(CompleteLocalOperatorWorkflowError::AdapterNotConfigured),
        steps,
        rejection_reasons: Vec::new(),
        evidence_summary: CompleteLocalOperatorWorkflowEvidenceSummary {
            provider_output_pipeline_status: "not_started".to_string(),
            local_candidate_materialization_status: "not_materialized".to_string(),
            replay_status: "no_decision_recorded".to_string(),
            local_evidence_export_status: "no_completed_run_evidence".to_string(),
            session_package_status: "not_packaged".to_string(),
            session_history_status: "no_session_history".to_string(),
            restore_status: "restore_not_requested".to_string(),
        },
        boundary_statuses: complete_local_operator_workflow_boundary_statuses(),
        capability_surface: CompleteLocalOperatorWorkflowCapabilitySurface {
            local_only: true,
            non_production: true,
            provider_trust_granted: false,
            action_execution_authorized: false,
            readiness_approved: false,
            release_approved: false,
            deployment_approved: false,
            public_use_approved: false,
            replay_repair_performed: false,
            recovery_promotion_performed: false,
        },
        local_only_note: "Complete local workflow is local-only and non-production.".to_string(),
        no_authority_note: "Workflow completion does not approve readiness, release, deployment, public use, or production use. Provider output remains untrusted unless a later bounded phase explicitly changes that. Workflow status does not authorize actions. Replay is not repaired and recovery is not promoted.".to_string(),
    }
}

fn complete_local_operator_workflow_evidence_summary(
    state: &LocalOperatorShellState,
) -> CompleteLocalOperatorWorkflowEvidenceSummary {
    CompleteLocalOperatorWorkflowEvidenceSummary {
        provider_output_pipeline_status: state
            .local_provider_output_pipeline
            .status
            .code()
            .to_string(),
        local_candidate_materialization_status: state
            .local_candidate_output
            .status
            .code()
            .to_string(),
        replay_status: state.run.decision_replay.replay_status.code().to_string(),
        local_evidence_export_status: state
            .local_session_evidence_export
            .export_status
            .code()
            .to_string(),
        session_package_status: state
            .local_session_package_projection
            .status
            .code()
            .to_string(),
        session_history_status: state
            .local_session_history_projection
            .status
            .code()
            .to_string(),
        restore_status: state
            .local_session_restore_projection
            .status
            .code()
            .to_string(),
    }
}

fn workflow_step(
    step: CompleteLocalOperatorWorkflowStepKind,
    status: CompleteLocalOperatorWorkflowStepStatus,
    error: Option<CompleteLocalOperatorWorkflowError>,
    summary: impl Into<String>,
) -> CompleteLocalOperatorWorkflowStep {
    CompleteLocalOperatorWorkflowStep {
        step,
        status,
        error,
        summary: summary.into(),
    }
}

pub fn classify_complete_local_operator_workflow_step(
    state: &LocalOperatorShellState,
    step: CompleteLocalOperatorWorkflowStepKind,
) -> CompleteLocalOperatorWorkflowStep {
    match step {
        CompleteLocalOperatorWorkflowStepKind::ProviderAdapterConfigured => {
            match state.local_provider_adapter_registry.last_validation.status {
                LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Provider adapter declaration is accepted.",
                ),
                LocalProviderAdapterValidationStatus::AdapterRejected
                | LocalProviderAdapterValidationStatus::UnsupportedAdapter
                | LocalProviderAdapterValidationStatus::InvalidAdapterDeclaration => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::AdapterNotConfigured),
                    "Provider adapter declaration is rejected or invalid.",
                ),
                LocalProviderAdapterValidationStatus::RegistryProjected => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::AdapterNotConfigured),
                    "Provider adapter declaration is missing.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::AdapterDryRunAvailable => {
            match state.local_provider_adapter_dry_run.status {
                LocalProviderAdapterDryRunStatus::DryRunExecuted => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Controlled adapter dry run has executed.",
                ),
                LocalProviderAdapterDryRunStatus::DryRunRejected
                | LocalProviderAdapterDryRunStatus::UnsupportedAdapter
                | LocalProviderAdapterDryRunStatus::InvalidDryRunRequest => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::InvocationRejected),
                    format!(
                        "Controlled adapter dry run rejected: {}.",
                        state
                            .local_provider_adapter_dry_run
                            .error_codes
                            .iter()
                            .map(|error| error.code())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                ),
                _ if state.local_provider_adapter_registry.last_validation.status
                    == LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting =>
                {
                    workflow_step(
                        step,
                        CompleteLocalOperatorWorkflowStepStatus::Available,
                        None,
                        "Controlled adapter dry run is available.",
                    )
                }
                _ => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::NotStarted,
                    None,
                    "Controlled adapter dry run waits for adapter configuration.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::ConstrainedInvocationCompleted => match state
            .constrained_local_provider_invocation
            .status
        {
            ConstrainedLocalProviderInvocationStatus::InvocationExecuted => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Completed,
                None,
                "Constrained local provider invocation has executed.",
            ),
            ConstrainedLocalProviderInvocationStatus::InvocationRejected
            | ConstrainedLocalProviderInvocationStatus::UnsupportedProvider
            | ConstrainedLocalProviderInvocationStatus::InvalidInvocationRequest => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Rejected,
                Some(CompleteLocalOperatorWorkflowError::InvocationRejected),
                format!(
                    "Constrained local provider invocation rejected: {}.",
                    state
                        .constrained_local_provider_invocation
                        .error_codes
                        .iter()
                        .map(|error| error.code())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ),
            _ if state.local_provider_adapter_registry.last_validation.status
                == LocalProviderAdapterValidationStatus::AdapterDeclaredNonExecuting =>
            {
                workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::InvocationMissing),
                    "Constrained local provider invocation is missing.",
                )
            }
            _ => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::NotStarted,
                None,
                "Invocation waits for provider adapter configuration.",
            ),
        },
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputPipelineProjected => {
            match state.local_provider_output_pipeline.status {
                LocalProviderOutputPipelineValidationStatus::Valid => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Provider output pipeline is valid.",
                ),
                LocalProviderOutputPipelineValidationStatus::Rejected => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::ProviderPipelineBlocked),
                    format!(
                        "Provider output pipeline blocked: {}.",
                        state
                            .local_provider_output_pipeline
                            .errors
                            .iter()
                            .map(|error| error.code())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                ),
                _ => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::ProviderPipelineBlocked),
                    "Provider output pipeline projection is missing or incomplete.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputValidated => {
            match state.provider_output_validation.status {
                LocalProviderOutputValidationStatus::ReviewableUntrusted => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Provider output validation is reviewable and untrusted.",
                ),
                LocalProviderOutputValidationStatus::Rejected
                | LocalProviderOutputValidationStatus::InvalidValidationInput
                | LocalProviderOutputValidationStatus::ValidationNotApplicable => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::ProviderOutputValidationRejected),
                    format!(
                        "Provider output validation rejected: {}.",
                        state
                            .provider_output_validation
                            .reasons
                            .iter()
                            .map(|reason| reason.code())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                ),
                LocalProviderOutputValidationStatus::NotValidated => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::ProviderOutputValidationMissing),
                    "Provider output validation is missing.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::ProviderOutputReviewed => {
            match state.provider_output_validation.reviewability_status {
                LocalProviderOutputReviewabilityStatus::ReviewableUntrusted => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Provider output review surface is projected.",
                ),
                LocalProviderOutputReviewabilityStatus::RejectedBeforeReview => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::ProviderOutputValidationRejected),
                    "Provider output was rejected before review.",
                ),
                LocalProviderOutputReviewabilityStatus::NotReviewable => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::ProviderOutputReviewMissing),
                    "Provider output review is missing.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::StagedProposalCreated => {
            match state.staged_candidate_conversion_proposal.status {
                StagedCandidateConversionProposalStatus::StagedProposalCreated => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Staged candidate-conversion proposal exists.",
                ),
                StagedCandidateConversionProposalStatus::RejectedSourceNotEligible
                | StagedCandidateConversionProposalStatus::InvalidProposalRequest => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::StagedProposalMissing),
                    "Staged proposal creation was rejected.",
                ),
                _ => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::StagedProposalMissing),
                    "Staged candidate-conversion proposal is missing.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::StagedProposalValidated => match state
            .staged_candidate_conversion_validation
            .status
        {
            StagedCandidateConversionValidationStatus::StagedProposalShapeValid => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Completed,
                None,
                "Staged proposal shape and linkage are valid.",
            ),
            StagedCandidateConversionValidationStatus::RejectedStagedProposal
            | StagedCandidateConversionValidationStatus::InvalidValidationInput => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Rejected,
                Some(CompleteLocalOperatorWorkflowError::StagedProposalValidationRejected),
                format!(
                    "Staged proposal validation rejected: {}.",
                    state
                        .staged_candidate_conversion_validation
                        .reasons
                        .iter()
                        .map(|reason| reason.code())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ),
            StagedCandidateConversionValidationStatus::NotValidated => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Blocked,
                Some(CompleteLocalOperatorWorkflowError::StagedProposalValidationMissing),
                "Staged proposal validation is missing.",
            ),
        },
        CompleteLocalOperatorWorkflowStepKind::CandidateReviewProjected => {
            if state.local_provider_output_pipeline.candidate_review_status == "display_only" {
                workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Candidate review surface is projected as display-only.",
                )
            } else {
                workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::CandidateReviewMissing),
                    "Candidate review projection is missing.",
                )
            }
        }
        CompleteLocalOperatorWorkflowStepKind::OperatorDecisionRecorded => {
            match state.operator_candidate_decision.status {
                OperatorCandidateDecisionStatus::ApprovedValidatedStagedProposal => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Completed,
                    None,
                    "Operator decision on validated staged proposal is recorded.",
                ),
                OperatorCandidateDecisionStatus::RejectedValidatedStagedProposal
                | OperatorCandidateDecisionStatus::RejectedOperatorDecisionRequest
                | OperatorCandidateDecisionStatus::InvalidOperatorDecisionInput => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Rejected,
                    Some(CompleteLocalOperatorWorkflowError::OperatorDecisionRejected),
                    "Operator decision is rejected or invalid.",
                ),
                OperatorCandidateDecisionStatus::NoOperatorDecision => workflow_step(
                    step,
                    CompleteLocalOperatorWorkflowStepStatus::Blocked,
                    Some(CompleteLocalOperatorWorkflowError::OperatorDecisionMissing),
                    "Operator decision is missing.",
                ),
            }
        }
        CompleteLocalOperatorWorkflowStepKind::LocalCandidateMaterialized => match state
            .local_candidate_output
            .status
        {
            LocalCandidateMaterializationStatus::LocalCandidateMaterialized => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Completed,
                None,
                "Local candidate output is materialized.",
            ),
            LocalCandidateMaterializationStatus::MaterializationRejected
            | LocalCandidateMaterializationStatus::MaterializationPreconditionMissing
            | LocalCandidateMaterializationStatus::InvalidMaterializationRequest => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Rejected,
                Some(CompleteLocalOperatorWorkflowError::LocalCandidateNotMaterialized),
                format!(
                    "Local candidate materialization rejected: {}.",
                    state
                        .local_candidate_output
                        .error
                        .as_ref()
                        .map(|error| error.code())
                        .unwrap_or("unknown")
                ),
            ),
            LocalCandidateMaterializationStatus::NotMaterialized => workflow_step(
                step,
                CompleteLocalOperatorWorkflowStepStatus::Blocked,
                Some(CompleteLocalOperatorWorkflowError::LocalCandidateNotMaterialized),
                "Local candidate output is not materialized.",
            ),
        },
        CompleteLocalOperatorWorkflowStepKind::ReplayStatusProjected => workflow_step(
            step,
            CompleteLocalOperatorWorkflowStepStatus::Completed,
            None,
            format!(
                "Replay/status projection is {}.",
                state.run.decision_replay.replay_status.code()
            ),
        ),
        CompleteLocalOperatorWorkflowStepKind::LocalEvidenceExportProjected => workflow_step(
            step,
            CompleteLocalOperatorWorkflowStepStatus::Completed,
            None,
            format!(
                "Local evidence export is {}.",
                state.local_session_evidence_export.export_status.code()
            ),
        ),
        CompleteLocalOperatorWorkflowStepKind::SessionPackageProjected => workflow_step(
            step,
            CompleteLocalOperatorWorkflowStepStatus::Completed,
            None,
            format!(
                "Local session package status is {}.",
                state.local_session_package_projection.status.code()
            ),
        ),
        CompleteLocalOperatorWorkflowStepKind::RestoreStatusProjected => workflow_step(
            step,
            CompleteLocalOperatorWorkflowStepStatus::Completed,
            None,
            format!(
                "Restore/history status is {} / {}.",
                state.local_session_history_projection.status.code(),
                state.local_session_restore_projection.status.code()
            ),
        ),
    }
}

pub fn complete_local_operator_workflow_current_blocker(
    steps: &[CompleteLocalOperatorWorkflowStep],
) -> Option<CompleteLocalOperatorWorkflowStepKind> {
    steps
        .iter()
        .find(|step| {
            matches!(
                step.status,
                CompleteLocalOperatorWorkflowStepStatus::Blocked
                    | CompleteLocalOperatorWorkflowStepStatus::Rejected
            )
        })
        .map(|step| step.step)
}

pub fn derive_complete_local_operator_workflow_projection(
    state: &LocalOperatorShellState,
) -> CompleteLocalOperatorWorkflowProjection {
    let steps = complete_local_operator_workflow_step_order()
        .into_iter()
        .map(|step| classify_complete_local_operator_workflow_step(state, step))
        .collect::<Vec<_>>();
    let current_blocking_step = complete_local_operator_workflow_current_blocker(&steps);
    let current_error = current_blocking_step.and_then(|blocked| {
        steps
            .iter()
            .find(|step| step.step == blocked)
            .and_then(|step| step.error)
    });
    let rejection_reasons = steps
        .iter()
        .filter(|step| step.status == CompleteLocalOperatorWorkflowStepStatus::Rejected)
        .map(|step| {
            format!(
                "{}: {}",
                step.step.code(),
                step.error.map(|error| error.code()).unwrap_or("rejected")
            )
        })
        .collect::<Vec<_>>();
    let primary_status = if !rejection_reasons.is_empty() {
        CompleteLocalOperatorWorkflowStatus::Rejected
    } else if current_blocking_step.is_some() {
        CompleteLocalOperatorWorkflowStatus::Blocked
    } else if state.local_candidate_output.status
        == LocalCandidateMaterializationStatus::LocalCandidateMaterialized
    {
        CompleteLocalOperatorWorkflowStatus::CompleteLocalWorkflowProjected
    } else if steps
        .iter()
        .any(|step| step.status == CompleteLocalOperatorWorkflowStepStatus::Completed)
    {
        CompleteLocalOperatorWorkflowStatus::InProgress
    } else {
        CompleteLocalOperatorWorkflowStatus::NotStarted
    };
    let current_step = current_blocking_step.or_else(|| {
        steps
            .iter()
            .find(|step| step.status != CompleteLocalOperatorWorkflowStepStatus::Completed)
            .map(|step| step.step)
    });
    CompleteLocalOperatorWorkflowProjection {
        status: primary_status,
        classification: "local_beta_workflow_only".to_string(),
        current_step,
        next_required_step: current_step,
        current_blocking_step,
        current_error,
        steps,
        rejection_reasons,
        evidence_summary: complete_local_operator_workflow_evidence_summary(state),
        boundary_statuses: complete_local_operator_workflow_boundary_statuses(),
        capability_surface: CompleteLocalOperatorWorkflowCapabilitySurface {
            local_only: true,
            non_production: true,
            provider_trust_granted: false,
            action_execution_authorized: false,
            readiness_approved: false,
            release_approved: false,
            deployment_approved: false,
            public_use_approved: false,
            replay_repair_performed: false,
            recovery_promotion_performed: false,
        },
        local_only_note: "Complete local workflow is local-only and non-production.".to_string(),
        no_authority_note: "Workflow completion does not approve readiness, release, deployment, public use, or production use. Provider output remains untrusted unless a later bounded phase explicitly changes that. Workflow status does not authorize actions. Replay is not repaired and recovery is not promoted.".to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialOperatorRunbookStatus {
    NotAvailable,
    TrialPackageRequired,
    TrialOperatorRunbookProjected,
    Blocked,
    FailureDrillRequired,
    ReadyForManualTrialPreparation,
}

impl TrialOperatorRunbookStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotAvailable => "not_available",
            Self::TrialPackageRequired => "trial_package_required",
            Self::TrialOperatorRunbookProjected => "trial_operator_runbook_projected",
            Self::Blocked => "blocked",
            Self::FailureDrillRequired => "failure_drill_required",
            Self::ReadyForManualTrialPreparation => "ready_for_manual_trial_preparation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialOperatorRunbookStepStatus {
    NotStarted,
    Available,
    Completed,
    Blocked,
    Rejected,
    ManualActionRequired,
    NotApplicable,
}

impl TrialOperatorRunbookStepStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Available => "available",
            Self::Completed => "completed",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
            Self::ManualActionRequired => "manual_action_required",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialOperatorRunbookStepKind {
    ConfirmLocalBetaWorkflow,
    ConfirmControlledTrialPackage,
    ConfirmTrialScope,
    ConfirmNamedInternalOperator,
    ConfirmNamedTrialParticipant,
    ReviewStopConditions,
    ReviewCurrentBlocker,
    ReviewFailureDrill,
    ReviewRestoreStatus,
    ReviewReplayStatus,
    ReviewEvidenceExportStatus,
    PrepareManualTrialNotes,
    ConfirmNoPublicRelease,
    ConfirmNoProductionApproval,
}

impl TrialOperatorRunbookStepKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ConfirmLocalBetaWorkflow => "confirm_local_beta_workflow",
            Self::ConfirmControlledTrialPackage => "confirm_controlled_trial_package",
            Self::ConfirmTrialScope => "confirm_trial_scope",
            Self::ConfirmNamedInternalOperator => "confirm_named_internal_operator",
            Self::ConfirmNamedTrialParticipant => "confirm_named_trial_participant",
            Self::ReviewStopConditions => "review_stop_conditions",
            Self::ReviewCurrentBlocker => "review_current_blocker",
            Self::ReviewFailureDrill => "review_failure_drill",
            Self::ReviewRestoreStatus => "review_restore_status",
            Self::ReviewReplayStatus => "review_replay_status",
            Self::ReviewEvidenceExportStatus => "review_evidence_export_status",
            Self::PrepareManualTrialNotes => "prepare_manual_trial_notes",
            Self::ConfirmNoPublicRelease => "confirm_no_public_release",
            Self::ConfirmNoProductionApproval => "confirm_no_production_approval",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialOperatorRunbookStep {
    pub step: TrialOperatorRunbookStepKind,
    pub status: TrialOperatorRunbookStepStatus,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialOperatorRunbookCurrentBlocker {
    pub step: Option<TrialOperatorRunbookStepKind>,
    pub workflow_step: Option<CompleteLocalOperatorWorkflowStepKind>,
    pub workflow_error: Option<CompleteLocalOperatorWorkflowError>,
    pub guidance: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialRunbookBoundaryStatus {
    LocalTrialGuidanceOnly,
    NonPublicTrialGuidance,
    NoTrialExecution,
    NoStopConditionAutomation,
    NoAuthorityActivation,
    NoReadinessApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoProductionApproval,
    NoActionExecution,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl TrialRunbookBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalTrialGuidanceOnly => "local_trial_guidance_only",
            Self::NonPublicTrialGuidance => "non_public_trial_guidance",
            Self::NoTrialExecution => "no_trial_execution",
            Self::NoStopConditionAutomation => "no_stop_condition_automation",
            Self::NoAuthorityActivation => "no_authority_activation",
            Self::NoReadinessApproval => "no_readiness_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialRunbookCapabilitySurface {
    pub local_only: bool,
    pub non_public: bool,
    pub executes_trial: bool,
    pub automates_stop_conditions: bool,
    pub activates_authority: bool,
    pub approves_readiness: bool,
    pub approves_release: bool,
    pub approves_deployment: bool,
    pub approves_public_use: bool,
    pub approves_production: bool,
    pub executes_actions: bool,
    pub repairs_replay: bool,
    pub promotes_recovery: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrialFailureSeverity {
    Info,
    ManualAction,
    Blocked,
    Rejected,
}

impl TrialFailureSeverity {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::ManualAction => "manual_action",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialFailureDrillStatus {
    NoFailuresProjected,
    FailureDrillRequired,
    StopConditionDrillRequired,
}

impl TrialFailureDrillStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoFailuresProjected => "no_failures_projected",
            Self::FailureDrillRequired => "failure_drill_required",
            Self::StopConditionDrillRequired => "stop_condition_drill_required",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialFailureCategory {
    NoTrialPackage,
    TrialPackageValidationFailure,
    TrialPackageReadBackFailure,
    MissingTrialScope,
    MissingNamedOperator,
    MissingNamedParticipant,
    MissingStopConditions,
    WorkflowBlocked,
    WorkflowRejected,
    ProviderPipelineBlocked,
    ProviderOutputValidationRejected,
    StagedValidationRejected,
    OperatorDecisionRejected,
    CandidateMaterializationMissing,
    RestoreProjectionRejected,
    ReplayStatusIncomplete,
    EvidenceExportMissing,
    StopConditionPresent,
    SecurityEscalationRequired,
    ReleaseStewardReviewRequired,
    TrialCoordinatorReviewRequired,
}

impl TrialFailureCategory {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoTrialPackage => "no_trial_package",
            Self::TrialPackageValidationFailure => "trial_package_validation_failure",
            Self::TrialPackageReadBackFailure => "trial_package_read_back_failure",
            Self::MissingTrialScope => "missing_trial_scope",
            Self::MissingNamedOperator => "missing_named_operator",
            Self::MissingNamedParticipant => "missing_named_participant",
            Self::MissingStopConditions => "missing_stop_conditions",
            Self::WorkflowBlocked => "workflow_blocked",
            Self::WorkflowRejected => "workflow_rejected",
            Self::ProviderPipelineBlocked => "provider_pipeline_blocked",
            Self::ProviderOutputValidationRejected => "provider_output_validation_rejected",
            Self::StagedValidationRejected => "staged_validation_rejected",
            Self::OperatorDecisionRejected => "operator_decision_rejected",
            Self::CandidateMaterializationMissing => "candidate_materialization_missing",
            Self::RestoreProjectionRejected => "restore_projection_rejected",
            Self::ReplayStatusIncomplete => "replay_status_incomplete",
            Self::EvidenceExportMissing => "evidence_export_missing",
            Self::StopConditionPresent => "stop_condition_present",
            Self::SecurityEscalationRequired => "security_escalation_required",
            Self::ReleaseStewardReviewRequired => "release_steward_review_required",
            Self::TrialCoordinatorReviewRequired => "trial_coordinator_review_required",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialFailureCategoryProjection {
    pub category: TrialFailureCategory,
    pub severity: TrialFailureSeverity,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialStopConditionDrill {
    pub marker: String,
    pub status: TrialOperatorRunbookStepStatus,
    pub guidance: String,
    pub enforcement_automated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialEscalationRole {
    TrialCoordinator,
    SecurityReviewer,
    ReleaseSteward,
    OperatorManualReview,
}

impl TrialEscalationRole {
    pub fn code(&self) -> &'static str {
        match self {
            Self::TrialCoordinator => "trial_coordinator",
            Self::SecurityReviewer => "security_reviewer",
            Self::ReleaseSteward => "release_steward",
            Self::OperatorManualReview => "operator_manual_review",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialEscalationGuidance {
    pub role: TrialEscalationRole,
    pub categories: Vec<TrialFailureCategory>,
    pub guidance: String,
    pub descriptive_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialFailureDrillProjection {
    pub status: TrialFailureDrillStatus,
    pub highest_severity: TrialFailureSeverity,
    pub categories: Vec<TrialFailureCategoryProjection>,
    pub stop_condition_drills: Vec<TrialStopConditionDrill>,
    pub escalation_guidance: Vec<TrialEscalationGuidance>,
    pub manual_action_guidance: Vec<String>,
    pub rejection_summary: Vec<String>,
    pub boundary_statuses: Vec<TrialRunbookBoundaryStatus>,
    pub no_automation_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialOperatorRunbookProjection {
    pub status: TrialOperatorRunbookStatus,
    pub current_step: Option<TrialOperatorRunbookStepKind>,
    pub current_blocker: TrialOperatorRunbookCurrentBlocker,
    pub steps: Vec<TrialOperatorRunbookStep>,
    pub trial_package_status: ControlledInternalTrialPackageStatus,
    pub trial_package_id: Option<String>,
    pub trial_package_validation_status: ControlledInternalTrialPackageValidationStatus,
    pub trial_package_read_back_status: Option<ControlledInternalTrialPackageValidationStatus>,
    pub trial_scope_status: TrialOperatorRunbookStepStatus,
    pub named_operator_status: TrialOperatorRunbookStepStatus,
    pub named_participant_status: TrialOperatorRunbookStepStatus,
    pub stop_condition_summary: Vec<String>,
    pub local_workflow_status: CompleteLocalOperatorWorkflowStatus,
    pub local_candidate_materialization_status: String,
    pub provider_output_pipeline_status: String,
    pub replay_status_summary: String,
    pub local_evidence_export_summary: String,
    pub restore_history_status: String,
    pub failure_drill: TrialFailureDrillProjection,
    pub boundary_statuses: Vec<TrialRunbookBoundaryStatus>,
    pub capability_surface: TrialRunbookCapabilitySurface,
    pub local_only_non_public_note: String,
    pub no_trial_execution_note: String,
    pub no_authority_note: String,
}

pub fn trial_runbook_boundary_statuses() -> Vec<TrialRunbookBoundaryStatus> {
    vec![
        TrialRunbookBoundaryStatus::LocalTrialGuidanceOnly,
        TrialRunbookBoundaryStatus::NonPublicTrialGuidance,
        TrialRunbookBoundaryStatus::NoTrialExecution,
        TrialRunbookBoundaryStatus::NoStopConditionAutomation,
        TrialRunbookBoundaryStatus::NoAuthorityActivation,
        TrialRunbookBoundaryStatus::NoReadinessApproval,
        TrialRunbookBoundaryStatus::NoReleaseApproval,
        TrialRunbookBoundaryStatus::NoDeploymentApproval,
        TrialRunbookBoundaryStatus::NoPublicUseApproval,
        TrialRunbookBoundaryStatus::NoProductionApproval,
        TrialRunbookBoundaryStatus::NoActionExecution,
        TrialRunbookBoundaryStatus::NoReplayRepair,
        TrialRunbookBoundaryStatus::NoRecoveryPromotion,
    ]
}

fn trial_runbook_capability_surface() -> TrialRunbookCapabilitySurface {
    TrialRunbookCapabilitySurface {
        local_only: true,
        non_public: true,
        executes_trial: false,
        automates_stop_conditions: false,
        activates_authority: false,
        approves_readiness: false,
        approves_release: false,
        approves_deployment: false,
        approves_public_use: false,
        approves_production: false,
        executes_actions: false,
        repairs_replay: false,
        promotes_recovery: false,
    }
}

fn trial_package_is_invalid(package: &ControlledInternalTrialPackageProjection) -> bool {
    package.status == ControlledInternalTrialPackageStatus::PackageRejected
        || package.status == ControlledInternalTrialPackageStatus::InvalidPackageInput
        || package.validation_status == ControlledInternalTrialPackageValidationStatus::Invalid
        || package.read_back_validation_status
            == Some(ControlledInternalTrialPackageValidationStatus::Invalid)
}

fn trial_has_named_operator(package: &ControlledInternalTrialPackageProjection) -> bool {
    package
        .named_operator_participant_summary
        .iter()
        .any(|item| item.starts_with("operator:"))
}

fn trial_has_named_participant(package: &ControlledInternalTrialPackageProjection) -> bool {
    package
        .named_operator_participant_summary
        .iter()
        .any(|item| item.starts_with("participant:"))
}

fn trial_scope_step_status(
    package: &ControlledInternalTrialPackageProjection,
) -> TrialOperatorRunbookStepStatus {
    if package.trial_scope_summary == "trial scope not provided" {
        TrialOperatorRunbookStepStatus::Blocked
    } else {
        TrialOperatorRunbookStepStatus::Completed
    }
}

fn named_operator_step_status(
    package: &ControlledInternalTrialPackageProjection,
) -> TrialOperatorRunbookStepStatus {
    if trial_has_named_operator(package) {
        TrialOperatorRunbookStepStatus::Completed
    } else {
        TrialOperatorRunbookStepStatus::Blocked
    }
}

fn named_participant_step_status(
    package: &ControlledInternalTrialPackageProjection,
) -> TrialOperatorRunbookStepStatus {
    if trial_has_named_participant(package) {
        TrialOperatorRunbookStepStatus::Completed
    } else {
        TrialOperatorRunbookStepStatus::Blocked
    }
}

fn trial_runbook_step_order() -> Vec<TrialOperatorRunbookStepKind> {
    vec![
        TrialOperatorRunbookStepKind::ConfirmLocalBetaWorkflow,
        TrialOperatorRunbookStepKind::ConfirmControlledTrialPackage,
        TrialOperatorRunbookStepKind::ConfirmTrialScope,
        TrialOperatorRunbookStepKind::ConfirmNamedInternalOperator,
        TrialOperatorRunbookStepKind::ConfirmNamedTrialParticipant,
        TrialOperatorRunbookStepKind::ReviewStopConditions,
        TrialOperatorRunbookStepKind::ReviewCurrentBlocker,
        TrialOperatorRunbookStepKind::ReviewFailureDrill,
        TrialOperatorRunbookStepKind::ReviewRestoreStatus,
        TrialOperatorRunbookStepKind::ReviewReplayStatus,
        TrialOperatorRunbookStepKind::ReviewEvidenceExportStatus,
        TrialOperatorRunbookStepKind::PrepareManualTrialNotes,
        TrialOperatorRunbookStepKind::ConfirmNoPublicRelease,
        TrialOperatorRunbookStepKind::ConfirmNoProductionApproval,
    ]
}

pub fn classify_trial_runbook_step(
    state: &LocalOperatorShellState,
    step: TrialOperatorRunbookStepKind,
) -> TrialOperatorRunbookStep {
    let package = &state.controlled_internal_trial_package_projection;
    let failure_drill = derive_trial_failure_drill_projection(state);
    let (status, summary) = match step {
        TrialOperatorRunbookStepKind::ConfirmLocalBetaWorkflow => {
            let status = match state.complete_local_operator_workflow.status {
                CompleteLocalOperatorWorkflowStatus::Rejected => TrialOperatorRunbookStepStatus::Rejected,
                CompleteLocalOperatorWorkflowStatus::Blocked => TrialOperatorRunbookStepStatus::Blocked,
                CompleteLocalOperatorWorkflowStatus::NotStarted => TrialOperatorRunbookStepStatus::ManualActionRequired,
                _ => TrialOperatorRunbookStepStatus::Completed,
            };
            (status, format!("Complete local workflow status is {}.", state.complete_local_operator_workflow.status.code()))
        }
        TrialOperatorRunbookStepKind::ConfirmControlledTrialPackage => {
            if package.status == ControlledInternalTrialPackageStatus::NotPackaged {
                (TrialOperatorRunbookStepStatus::Blocked, "Controlled internal trial package is required before manual trial preparation.".to_string())
            } else if trial_package_is_invalid(package) {
                (TrialOperatorRunbookStepStatus::Rejected, format!("Controlled internal trial package validation is {}.", package.validation_status.code()))
            } else {
                (TrialOperatorRunbookStepStatus::Completed, format!("Controlled internal trial package status is {}.", package.status.code()))
            }
        }
        TrialOperatorRunbookStepKind::ConfirmTrialScope => (
            trial_scope_step_status(package),
            format!("Trial scope: {}.", package.trial_scope_summary),
        ),
        TrialOperatorRunbookStepKind::ConfirmNamedInternalOperator => (
            named_operator_step_status(package),
            format!("Named operator metadata: {}.", package.named_operator_participant_summary.join(", ")),
        ),
        TrialOperatorRunbookStepKind::ConfirmNamedTrialParticipant => (
            named_participant_step_status(package),
            format!("Named participant metadata: {}.", package.named_operator_participant_summary.join(", ")),
        ),
        TrialOperatorRunbookStepKind::ReviewStopConditions => {
            if package.stop_condition_summary.is_empty() {
                (TrialOperatorRunbookStepStatus::Blocked, "Stop-condition markers are missing from the package projection.".to_string())
            } else {
                (TrialOperatorRunbookStepStatus::ManualActionRequired, format!("Review {} stop-condition drill marker(s); guidance only.", package.stop_condition_summary.len()))
            }
        }
        TrialOperatorRunbookStepKind::ReviewCurrentBlocker => {
            if let Some(blocker) = state.complete_local_operator_workflow.current_blocking_step {
                (TrialOperatorRunbookStepStatus::ManualActionRequired, format!("Current workflow blocker is {}.", blocker.code()))
            } else {
                (TrialOperatorRunbookStepStatus::Completed, "No current complete-workflow blocker is projected.".to_string())
            }
        }
        TrialOperatorRunbookStepKind::ReviewFailureDrill => {
            if failure_drill.categories.is_empty() {
                (TrialOperatorRunbookStepStatus::Completed, "No failure categories are projected.".to_string())
            } else {
                (TrialOperatorRunbookStepStatus::ManualActionRequired, format!("Review {} failure drill categorization(s).", failure_drill.categories.len()))
            }
        }
        TrialOperatorRunbookStepKind::ReviewRestoreStatus => {
            let status = if state.local_session_restore_projection.validation_status == LocalSessionRestoreValidationStatus::Invalid {
                TrialOperatorRunbookStepStatus::Rejected
            } else {
                TrialOperatorRunbookStepStatus::Completed
            };
            (status, format!("Restore/history status is {} / {}.", state.local_session_restore_projection.status.code(), state.local_session_history_projection.status.code()))
        }
        TrialOperatorRunbookStepKind::ReviewReplayStatus => {
            let status = if state.run.decision_replay.integrity_status == LocalDecisionReplayIntegrityStatus::Inconsistent {
                TrialOperatorRunbookStepStatus::Rejected
            } else if state.run.decision_replay.replay_status == LocalDecisionReplayStatus::NoDecisionRecorded {
                TrialOperatorRunbookStepStatus::ManualActionRequired
            } else {
                TrialOperatorRunbookStepStatus::Completed
            };
            (status, format!("Replay/status projection is {} with {} integrity.", state.run.decision_replay.replay_status.code(), state.run.decision_replay.integrity_status.code()))
        }
        TrialOperatorRunbookStepKind::ReviewEvidenceExportStatus => {
            let status = if state.local_session_evidence_export.export_validation_status == LocalSessionEvidenceExportValidationStatus::Incomplete {
                TrialOperatorRunbookStepStatus::ManualActionRequired
            } else {
                TrialOperatorRunbookStepStatus::Completed
            };
            (status, format!("Local evidence export is {}.", state.local_session_evidence_export.export_status.code()))
        }
        TrialOperatorRunbookStepKind::PrepareManualTrialNotes => (
            TrialOperatorRunbookStepStatus::ManualActionRequired,
            "Prepare manual notes; this projection does not execute or authorize trial operation.".to_string(),
        ),
        TrialOperatorRunbookStepKind::ConfirmNoPublicRelease => (
            TrialOperatorRunbookStepStatus::Completed,
            "Boundary confirms no public release, publication, signing, or deployment behavior.".to_string(),
        ),
        TrialOperatorRunbookStepKind::ConfirmNoProductionApproval => (
            TrialOperatorRunbookStepStatus::Completed,
            "Boundary confirms no controlled human-use, public-use, production-use, release, deployment, or readiness approval.".to_string(),
        ),
    };
    TrialOperatorRunbookStep {
        step,
        status,
        summary,
    }
}

fn add_trial_failure_category(
    categories: &mut Vec<TrialFailureCategoryProjection>,
    category: TrialFailureCategory,
    severity: TrialFailureSeverity,
    summary: String,
) {
    if !categories.iter().any(|entry| entry.category == category) {
        categories.push(TrialFailureCategoryProjection {
            category,
            severity,
            summary,
        });
    }
}

pub fn derive_trial_stop_condition_drills(
    state: &LocalOperatorShellState,
) -> Vec<TrialStopConditionDrill> {
    state
        .controlled_internal_trial_package_projection
        .stop_condition_summary
        .iter()
        .map(|marker| TrialStopConditionDrill {
            marker: marker.clone(),
            status: TrialOperatorRunbookStepStatus::ManualActionRequired,
            guidance: format!(
                "If stop-condition marker {marker} is present during manual preparation, pause the local trial workflow, record notes, and involve the trial coordinator before continuing."
            ),
            enforcement_automated: false,
        })
        .collect()
}

pub fn derive_trial_escalation_guidance(
    categories: &[TrialFailureCategoryProjection],
) -> Vec<TrialEscalationGuidance> {
    let has = |category| categories.iter().any(|entry| entry.category == category);
    let mut guidance = vec![TrialEscalationGuidance {
        role: TrialEscalationRole::OperatorManualReview,
        categories: vec![
            TrialFailureCategory::CandidateMaterializationMissing,
            TrialFailureCategory::EvidenceExportMissing,
            TrialFailureCategory::ReplayStatusIncomplete,
        ],
        guidance: "Operator manual review checks local notes, missing materialization, replay/status, and evidence export gaps without executing remediation.".to_string(),
        descriptive_only: true,
    }];
    if has(TrialFailureCategory::ProviderPipelineBlocked)
        || has(TrialFailureCategory::ProviderOutputValidationRejected)
        || has(TrialFailureCategory::SecurityEscalationRequired)
    {
        guidance.push(TrialEscalationGuidance {
            role: TrialEscalationRole::SecurityReviewer,
            categories: vec![
                TrialFailureCategory::ProviderPipelineBlocked,
                TrialFailureCategory::ProviderOutputValidationRejected,
                TrialFailureCategory::SecurityEscalationRequired,
            ],
            guidance: "Security reviewer guidance applies to provider pipeline, provider output validation, and security-sensitive rejection categories; it does not trust provider output.".to_string(),
            descriptive_only: true,
        });
    }
    if has(TrialFailureCategory::ReleaseStewardReviewRequired)
        || categories
            .iter()
            .any(|entry| entry.summary.contains("release") || entry.summary.contains("deployment"))
    {
        guidance.push(TrialEscalationGuidance {
            role: TrialEscalationRole::ReleaseSteward,
            categories: vec![TrialFailureCategory::ReleaseStewardReviewRequired],
            guidance: "Release steward guidance applies only to reviewing release/deployment/readiness claims; it does not create release or deployment artifacts.".to_string(),
            descriptive_only: true,
        });
    }
    if has(TrialFailureCategory::WorkflowBlocked)
        || has(TrialFailureCategory::WorkflowRejected)
        || has(TrialFailureCategory::TrialCoordinatorReviewRequired)
        || has(TrialFailureCategory::StopConditionPresent)
        || has(TrialFailureCategory::NoTrialPackage)
    {
        guidance.push(TrialEscalationGuidance {
            role: TrialEscalationRole::TrialCoordinator,
            categories: vec![
                TrialFailureCategory::WorkflowBlocked,
                TrialFailureCategory::WorkflowRejected,
                TrialFailureCategory::TrialCoordinatorReviewRequired,
                TrialFailureCategory::StopConditionPresent,
                TrialFailureCategory::NoTrialPackage,
            ],
            guidance: "Trial coordinator guidance applies to workflow blockers, missing package preparation, and stop-condition drill review; it does not approve use authority.".to_string(),
            descriptive_only: true,
        });
    }
    guidance
}

pub fn derive_trial_failure_drill_projection(
    state: &LocalOperatorShellState,
) -> TrialFailureDrillProjection {
    let mut categories = Vec::new();
    let package = &state.controlled_internal_trial_package_projection;
    if package.status == ControlledInternalTrialPackageStatus::NotPackaged {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::NoTrialPackage,
            TrialFailureSeverity::Blocked,
            "Controlled internal trial package has not been projected.".to_string(),
        );
    }
    if package.validation_status == ControlledInternalTrialPackageValidationStatus::Invalid
        || package.status == ControlledInternalTrialPackageStatus::PackageRejected
        || package.status == ControlledInternalTrialPackageStatus::InvalidPackageInput
    {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::TrialPackageValidationFailure,
            TrialFailureSeverity::Rejected,
            format!(
                "Trial package validation errors: {}.",
                package.validation_errors.join(", ")
            ),
        );
    }
    if package.read_back_validation_status
        == Some(ControlledInternalTrialPackageValidationStatus::Invalid)
    {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::TrialPackageReadBackFailure,
            TrialFailureSeverity::Rejected,
            "Trial package read-back validation is invalid.".to_string(),
        );
    }
    if package.trial_scope_summary == "trial scope not provided" {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::MissingTrialScope,
            TrialFailureSeverity::Blocked,
            "Trial scope is missing.".to_string(),
        );
    }
    if !trial_has_named_operator(package) {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::MissingNamedOperator,
            TrialFailureSeverity::Blocked,
            "Named internal operator metadata is missing.".to_string(),
        );
    }
    if !trial_has_named_participant(package) {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::MissingNamedParticipant,
            TrialFailureSeverity::Blocked,
            "Named trial participant metadata is missing.".to_string(),
        );
    }
    if package.stop_condition_summary.is_empty() {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::MissingStopConditions,
            TrialFailureSeverity::Blocked,
            "Stop-condition markers are missing.".to_string(),
        );
    } else {
        add_trial_failure_category(&mut categories, TrialFailureCategory::StopConditionPresent, TrialFailureSeverity::ManualAction, "Stop-condition drill markers are present for operator review; enforcement is guidance only.".to_string());
    }
    match state.complete_local_operator_workflow.status {
        CompleteLocalOperatorWorkflowStatus::Blocked => add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::WorkflowBlocked,
            TrialFailureSeverity::Blocked,
            format!(
                "Workflow is blocked at {}.",
                state
                    .complete_local_operator_workflow
                    .current_blocking_step
                    .map(|step| step.code())
                    .unwrap_or("unknown")
            ),
        ),
        CompleteLocalOperatorWorkflowStatus::Rejected => add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::WorkflowRejected,
            TrialFailureSeverity::Rejected,
            format!(
                "Workflow rejection summary: {}.",
                state
                    .complete_local_operator_workflow
                    .rejection_reasons
                    .join(", ")
            ),
        ),
        _ => {}
    }
    match state.local_provider_output_pipeline.status {
        LocalProviderOutputPipelineValidationStatus::Blocked => add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::ProviderPipelineBlocked,
            TrialFailureSeverity::Blocked,
            "Provider output pipeline is blocked.".to_string(),
        ),
        LocalProviderOutputPipelineValidationStatus::Rejected => add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::ProviderPipelineBlocked,
            TrialFailureSeverity::Rejected,
            "Provider output pipeline is rejected.".to_string(),
        ),
        _ => {}
    }
    if state.provider_output_validation.status == LocalProviderOutputValidationStatus::Rejected {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::ProviderOutputValidationRejected,
            TrialFailureSeverity::Rejected,
            "Provider output validation is rejected.".to_string(),
        );
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::SecurityEscalationRequired,
            TrialFailureSeverity::ManualAction,
            "Provider output validation rejection requires security-review guidance.".to_string(),
        );
    }
    if state.staged_candidate_conversion_validation.status
        == StagedCandidateConversionValidationStatus::RejectedStagedProposal
    {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::StagedValidationRejected,
            TrialFailureSeverity::Rejected,
            "Staged proposal validation is rejected.".to_string(),
        );
    }
    if state.operator_candidate_decision.status
        == OperatorCandidateDecisionStatus::RejectedValidatedStagedProposal
    {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::OperatorDecisionRejected,
            TrialFailureSeverity::ManualAction,
            "Operator decision recorded a rejection of the validated staged proposal.".to_string(),
        );
    }
    if state.local_candidate_output.status == LocalCandidateMaterializationStatus::NotMaterialized {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::CandidateMaterializationMissing,
            TrialFailureSeverity::ManualAction,
            "Local candidate materialization is missing.".to_string(),
        );
    }
    if state.local_session_restore_projection.validation_status
        == LocalSessionRestoreValidationStatus::Invalid
        || state.local_session_restore_projection.status
            == LocalSessionRestoreStatus::RestoreRejected
    {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::RestoreProjectionRejected,
            TrialFailureSeverity::Rejected,
            "Restore/history projection is rejected or invalid.".to_string(),
        );
    }
    if state.run.decision_replay.integrity_status
        == LocalDecisionReplayIntegrityStatus::Inconsistent
        || state.run.decision_replay.replay_status == LocalDecisionReplayStatus::NoDecisionRecorded
    {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::ReplayStatusIncomplete,
            TrialFailureSeverity::ManualAction,
            format!(
                "Replay/status projection is {}.",
                state.run.decision_replay.replay_status.code()
            ),
        );
    }
    if state.local_session_evidence_export.export_validation_status
        == LocalSessionEvidenceExportValidationStatus::Incomplete
    {
        add_trial_failure_category(
            &mut categories,
            TrialFailureCategory::EvidenceExportMissing,
            TrialFailureSeverity::ManualAction,
            "Local evidence export is incomplete.".to_string(),
        );
    }
    if package.validation_errors.iter().any(|error| {
        error.contains("release")
            || error.contains("deployment")
            || error.contains("readiness")
            || error.contains("public")
            || error.contains("production")
    }) {
        add_trial_failure_category(&mut categories, TrialFailureCategory::ReleaseStewardReviewRequired, TrialFailureSeverity::ManualAction, "Package contains release/deployment/readiness/public/production-use claim rejection summary.".to_string());
    }
    if state
        .complete_local_operator_workflow
        .current_blocking_step
        .is_some()
    {
        add_trial_failure_category(&mut categories, TrialFailureCategory::TrialCoordinatorReviewRequired, TrialFailureSeverity::ManualAction, "Workflow blocker requires trial coordinator guidance before manual preparation continues.".to_string());
    }
    let highest_severity = categories
        .iter()
        .map(|entry| entry.severity)
        .max()
        .unwrap_or(TrialFailureSeverity::Info);
    let stop_condition_drills = derive_trial_stop_condition_drills(state);
    let status = if categories
        .iter()
        .any(|entry| entry.category == TrialFailureCategory::StopConditionPresent)
    {
        TrialFailureDrillStatus::StopConditionDrillRequired
    } else if categories.is_empty() {
        TrialFailureDrillStatus::NoFailuresProjected
    } else {
        TrialFailureDrillStatus::FailureDrillRequired
    };
    let manual_action_guidance = categories
        .iter()
        .filter(|entry| {
            entry.severity == TrialFailureSeverity::ManualAction
                || entry.severity == TrialFailureSeverity::Blocked
        })
        .map(|entry| {
            format!(
                "{}: review manually; no remediation is executed.",
                entry.category.code()
            )
        })
        .collect::<Vec<_>>();
    let rejection_summary = categories
        .iter()
        .filter(|entry| entry.severity == TrialFailureSeverity::Rejected)
        .map(|entry| format!("{}: {}", entry.category.code(), entry.summary))
        .collect::<Vec<_>>();
    let escalation_guidance = derive_trial_escalation_guidance(&categories);
    TrialFailureDrillProjection {
        status,
        highest_severity,
        categories,
        stop_condition_drills,
        escalation_guidance,
        manual_action_guidance,
        rejection_summary,
        boundary_statuses: trial_runbook_boundary_statuses(),
        no_automation_note:
            "Stop conditions are guidance only; enforcement is not automated in Phase 162."
                .to_string(),
    }
}

pub fn derive_trial_operator_runbook_projection(
    state: &LocalOperatorShellState,
) -> TrialOperatorRunbookProjection {
    let steps = trial_runbook_step_order()
        .into_iter()
        .map(|step| classify_trial_runbook_step(state, step))
        .collect::<Vec<_>>();
    let current_step = steps
        .iter()
        .find(|step| step.status != TrialOperatorRunbookStepStatus::Completed)
        .map(|step| step.step);
    let current_blocker_step = steps
        .iter()
        .find(|step| {
            matches!(
                step.status,
                TrialOperatorRunbookStepStatus::Blocked | TrialOperatorRunbookStepStatus::Rejected
            )
        })
        .map(|step| step.step);
    let current_blocker = TrialOperatorRunbookCurrentBlocker {
        step: current_blocker_step,
        workflow_step: state.complete_local_operator_workflow.current_blocking_step,
        workflow_error: state.complete_local_operator_workflow.current_error,
        guidance: match (current_blocker_step, state.complete_local_operator_workflow.current_blocking_step) {
            (Some(step), Some(workflow_step)) => format!("Current blocker guidance: review runbook step {} and workflow step {}; manual action only.", step.code(), workflow_step.code()),
            (Some(step), None) => format!("Current blocker guidance: review runbook step {}; manual action only.", step.code()),
            (None, Some(workflow_step)) => format!("Current blocker guidance: review workflow step {}; manual action only.", workflow_step.code()),
            (None, None) => "Current blocker guidance: no blocking step is projected; continue manual review without executing trial authority.".to_string(),
        },
    };
    let failure_drill = derive_trial_failure_drill_projection(state);
    let package = &state.controlled_internal_trial_package_projection;
    let status = if package.status == ControlledInternalTrialPackageStatus::NotPackaged {
        TrialOperatorRunbookStatus::TrialPackageRequired
    } else if trial_package_is_invalid(package) || current_blocker_step.is_some() {
        TrialOperatorRunbookStatus::Blocked
    } else if !failure_drill.categories.is_empty() {
        TrialOperatorRunbookStatus::FailureDrillRequired
    } else if steps
        .iter()
        .any(|step| step.status == TrialOperatorRunbookStepStatus::ManualActionRequired)
    {
        TrialOperatorRunbookStatus::TrialOperatorRunbookProjected
    } else {
        TrialOperatorRunbookStatus::ReadyForManualTrialPreparation
    };
    TrialOperatorRunbookProjection {
        status,
        current_step,
        current_blocker,
        steps,
        trial_package_status: package.status,
        trial_package_id: package.package_id.clone(),
        trial_package_validation_status: package.validation_status,
        trial_package_read_back_status: package.read_back_validation_status,
        trial_scope_status: trial_scope_step_status(package),
        named_operator_status: named_operator_step_status(package),
        named_participant_status: named_participant_step_status(package),
        stop_condition_summary: package.stop_condition_summary.clone(),
        local_workflow_status: state.complete_local_operator_workflow.status,
        local_candidate_materialization_status: state.local_candidate_output.status.code().to_string(),
        provider_output_pipeline_status: state.local_provider_output_pipeline.status.code().to_string(),
        replay_status_summary: state.run.decision_replay.summary.clone(),
        local_evidence_export_summary: state.local_session_evidence_export.summary.clone(),
        restore_history_status: format!("{} / {}", state.local_session_restore_projection.status.code(), state.local_session_history_projection.status.code()),
        failure_drill,
        boundary_statuses: trial_runbook_boundary_statuses(),
        capability_surface: trial_runbook_capability_surface(),
        local_only_non_public_note: "Trial operator runbook is local-only and non-public.".to_string(),
        no_trial_execution_note: "This runbook does not start a controlled trial.".to_string(),
        no_authority_note: "This runbook does not approve controlled human use, public use, production use, release, deployment, or readiness.".to_string(),
    }
}

pub fn initial_trial_failure_drill_projection() -> TrialFailureDrillProjection {
    TrialFailureDrillProjection {
        status: TrialFailureDrillStatus::NoFailuresProjected,
        highest_severity: TrialFailureSeverity::Info,
        categories: Vec::new(),
        stop_condition_drills: Vec::new(),
        escalation_guidance: Vec::new(),
        manual_action_guidance: Vec::new(),
        rejection_summary: Vec::new(),
        boundary_statuses: trial_runbook_boundary_statuses(),
        no_automation_note:
            "Stop conditions are guidance only; enforcement is not automated in Phase 162."
                .to_string(),
    }
}

pub fn initial_trial_operator_runbook_projection() -> TrialOperatorRunbookProjection {
    TrialOperatorRunbookProjection {
        status: TrialOperatorRunbookStatus::NotAvailable,
        current_step: None,
        current_blocker: TrialOperatorRunbookCurrentBlocker {
            step: None,
            workflow_step: None,
            workflow_error: None,
            guidance: "Current blocker guidance: projection not yet derived.".to_string(),
        },
        steps: Vec::new(),
        trial_package_status: ControlledInternalTrialPackageStatus::NotPackaged,
        trial_package_id: None,
        trial_package_validation_status: ControlledInternalTrialPackageValidationStatus::NotValidated,
        trial_package_read_back_status: None,
        trial_scope_status: TrialOperatorRunbookStepStatus::NotStarted,
        named_operator_status: TrialOperatorRunbookStepStatus::NotStarted,
        named_participant_status: TrialOperatorRunbookStepStatus::NotStarted,
        stop_condition_summary: Vec::new(),
        local_workflow_status: CompleteLocalOperatorWorkflowStatus::NotStarted,
        local_candidate_materialization_status: "not_materialized".to_string(),
        provider_output_pipeline_status: "not_projected".to_string(),
        replay_status_summary: "not projected".to_string(),
        local_evidence_export_summary: "not projected".to_string(),
        restore_history_status: "not projected".to_string(),
        failure_drill: initial_trial_failure_drill_projection(),
        boundary_statuses: trial_runbook_boundary_statuses(),
        capability_surface: trial_runbook_capability_surface(),
        local_only_non_public_note: "Trial operator runbook is local-only and non-public.".to_string(),
        no_trial_execution_note: "This runbook does not start a controlled trial.".to_string(),
        no_authority_note: "This runbook does not approve controlled human use, public use, production use, release, deployment, or readiness.".to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialSessionEvidenceStatus {
    NotCaptured,
    EvidenceProjected,
    EvidenceValidated,
    EvidenceWritten,
    EvidenceReadBackValidated,
    EvidenceRejected,
    InvalidEvidenceInput,
}

impl TrialSessionEvidenceStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotCaptured => "not_captured",
            Self::EvidenceProjected => "evidence_projected",
            Self::EvidenceValidated => "evidence_validated",
            Self::EvidenceWritten => "evidence_written",
            Self::EvidenceReadBackValidated => "evidence_read_back_validated",
            Self::EvidenceRejected => "evidence_rejected",
            Self::InvalidEvidenceInput => "invalid_evidence_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialSessionEvidenceValidationStatus {
    NotValidated,
    Valid,
    Invalid,
}

impl TrialSessionEvidenceValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotValidated => "not_validated",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialSessionEvidenceClassification {
    TrialSessionEvidenceOnly,
}

impl TrialSessionEvidenceClassification {
    pub fn code(&self) -> &'static str {
        "trial_session_evidence_only"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialSessionEvidenceProductionClassification {
    NonProduction,
}

impl TrialSessionEvidenceProductionClassification {
    pub fn code(&self) -> &'static str {
        "non_production"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialSessionEvidenceDistributionClassification {
    LocalOnlyNonPublic,
}

impl TrialSessionEvidenceDistributionClassification {
    pub fn code(&self) -> &'static str {
        "local_only_non_public"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialSessionEvidenceAuthorityClassification {
    NonAuthoritativeEvidence,
}

impl TrialSessionEvidenceAuthorityClassification {
    pub fn code(&self) -> &'static str {
        "non_authoritative_evidence"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialSessionEvidenceBoundaryStatus {
    LocalTrialEvidenceOnly,
    NonPublicEvidence,
    EvidenceNotAuthority,
    NoTrialExecution,
    NoControlledHumanUseApproval,
    NoReadinessApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoProductionApproval,
    NoProviderTrust,
    NoActionAuthorization,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl TrialSessionEvidenceBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalTrialEvidenceOnly => "local_trial_evidence_only",
            Self::NonPublicEvidence => "non_public_evidence",
            Self::EvidenceNotAuthority => "evidence_not_authority",
            Self::NoTrialExecution => "no_trial_execution",
            Self::NoControlledHumanUseApproval => "no_controlled_human_use_approval",
            Self::NoReadinessApproval => "no_readiness_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoActionAuthorization => "no_action_authorization",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
        }
    }
}

pub fn trial_session_evidence_boundary_statuses() -> Vec<TrialSessionEvidenceBoundaryStatus> {
    vec![
        TrialSessionEvidenceBoundaryStatus::LocalTrialEvidenceOnly,
        TrialSessionEvidenceBoundaryStatus::NonPublicEvidence,
        TrialSessionEvidenceBoundaryStatus::EvidenceNotAuthority,
        TrialSessionEvidenceBoundaryStatus::NoTrialExecution,
        TrialSessionEvidenceBoundaryStatus::NoControlledHumanUseApproval,
        TrialSessionEvidenceBoundaryStatus::NoReadinessApproval,
        TrialSessionEvidenceBoundaryStatus::NoReleaseApproval,
        TrialSessionEvidenceBoundaryStatus::NoDeploymentApproval,
        TrialSessionEvidenceBoundaryStatus::NoPublicUseApproval,
        TrialSessionEvidenceBoundaryStatus::NoProductionApproval,
        TrialSessionEvidenceBoundaryStatus::NoProviderTrust,
        TrialSessionEvidenceBoundaryStatus::NoActionAuthorization,
        TrialSessionEvidenceBoundaryStatus::NoReplayRepair,
        TrialSessionEvidenceBoundaryStatus::NoRecoveryPromotion,
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialSessionEvidenceValidationError {
    MissingEvidenceId,
    MissingEvidenceVersion,
    InvalidEvidenceClassification,
    InvalidProductionClassification,
    InvalidDistributionClassification,
    InvalidAuthorityClassification,
    MissingTrialPackageLinkage,
    MissingRunbookLinkage,
    MissingFailureDrillLinkage,
    MissingWorkflowSnapshot,
    MissingStopConditionSnapshot,
    MissingEscalationSnapshot,
    MissingAbsenceMarker,
    ReadinessClaimDetected,
    ReleaseClaimDetected,
    DeploymentClaimDetected,
    PublicUseClaimDetected,
    ProductionUseClaimDetected,
    ProviderTrustClaimDetected,
    ActionAuthorizationClaimDetected,
    ReplayRepairClaimDetected,
    RecoveryPromotionClaimDetected,
    DeterministicDigestMismatch,
    MalformedEvidenceInput,
}

impl TrialSessionEvidenceValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingEvidenceId => "missing_evidence_id",
            Self::MissingEvidenceVersion => "missing_evidence_version",
            Self::InvalidEvidenceClassification => "invalid_evidence_classification",
            Self::InvalidProductionClassification => "invalid_production_classification",
            Self::InvalidDistributionClassification => "invalid_distribution_classification",
            Self::InvalidAuthorityClassification => "invalid_authority_classification",
            Self::MissingTrialPackageLinkage => "missing_trial_package_linkage",
            Self::MissingRunbookLinkage => "missing_runbook_linkage",
            Self::MissingFailureDrillLinkage => "missing_failure_drill_linkage",
            Self::MissingWorkflowSnapshot => "missing_workflow_snapshot",
            Self::MissingStopConditionSnapshot => "missing_stop_condition_snapshot",
            Self::MissingEscalationSnapshot => "missing_escalation_snapshot",
            Self::MissingAbsenceMarker => "missing_absence_marker",
            Self::ReadinessClaimDetected => "readiness_claim_detected",
            Self::ReleaseClaimDetected => "release_claim_detected",
            Self::DeploymentClaimDetected => "deployment_claim_detected",
            Self::PublicUseClaimDetected => "public_use_claim_detected",
            Self::ProductionUseClaimDetected => "production_use_claim_detected",
            Self::ProviderTrustClaimDetected => "provider_trust_claim_detected",
            Self::ActionAuthorizationClaimDetected => "action_authorization_claim_detected",
            Self::ReplayRepairClaimDetected => "replay_repair_claim_detected",
            Self::RecoveryPromotionClaimDetected => "recovery_promotion_claim_detected",
            Self::DeterministicDigestMismatch => "deterministic_digest_mismatch",
            Self::MalformedEvidenceInput => "malformed_evidence_input",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialSessionEvidenceMetadata {
    pub evidence_id: String,
    pub evidence_version: String,
    pub evidence_classification: String,
    pub production_classification: String,
    pub distribution_classification: String,
    pub authority_classification: String,
    pub evidence_status: TrialSessionEvidenceStatus,
    pub validation_status: TrialSessionEvidenceValidationStatus,
    pub content_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialSessionEvidenceAbsenceMarkers {
    pub release_artifact_absent: bool,
    pub deployment_artifact_absent: bool,
    pub readiness_approval_absent: bool,
    pub public_use_approval_absent: bool,
    pub production_use_approval_absent: bool,
    pub provider_trust_absent: bool,
    pub action_authorization_absent: bool,
    pub replay_repair_absent: bool,
    pub recovery_promotion_absent: bool,
    pub default_persistence_absent: bool,
    pub automatic_save_absent: bool,
    pub background_persistence_absent: bool,
    pub network_sync_absent: bool,
    pub marker_summary: Vec<String>,
}

pub fn trial_session_evidence_absence_markers() -> TrialSessionEvidenceAbsenceMarkers {
    TrialSessionEvidenceAbsenceMarkers {
        release_artifact_absent: true,
        deployment_artifact_absent: true,
        readiness_approval_absent: true,
        public_use_approval_absent: true,
        production_use_approval_absent: true,
        provider_trust_absent: true,
        action_authorization_absent: true,
        replay_repair_absent: true,
        recovery_promotion_absent: true,
        default_persistence_absent: true,
        automatic_save_absent: true,
        background_persistence_absent: true,
        network_sync_absent: true,
        marker_summary: vec![
            "release artifact absent".to_string(),
            "deployment artifact absent".to_string(),
            "readiness approval absent".to_string(),
            "public use approval absent".to_string(),
            "production use approval absent".to_string(),
            "provider trust absent".to_string(),
            "action authorization absent".to_string(),
            "replay repair absent".to_string(),
            "recovery promotion absent".to_string(),
            "default persistence absent".to_string(),
            "automatic save absent".to_string(),
            "background persistence absent".to_string(),
            "network sync absent".to_string(),
        ],
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialSessionEvidencePayload {
    pub trial_package_id: String,
    pub trial_package_version: String,
    pub trial_package_status: String,
    pub trial_package_validation_status: String,
    pub trial_package_read_back_status: String,
    pub runbook_status: String,
    pub runbook_current_step: String,
    pub runbook_current_blocker: String,
    pub failure_drill_status: String,
    pub failure_drill_highest_severity: String,
    pub workflow_status_snapshot: String,
    pub local_candidate_materialization_snapshot: String,
    pub provider_output_pipeline_snapshot: String,
    pub operator_decision_snapshot: String,
    pub replay_status_snapshot: String,
    pub local_evidence_export_snapshot: String,
    pub local_session_package_snapshot: String,
    pub restore_history_snapshot: String,
    pub stop_condition_snapshot: Vec<String>,
    pub escalation_guidance_snapshot: Vec<String>,
    pub failure_category_snapshot: Vec<String>,
    pub current_blocker_snapshot: String,
    pub boundary_status: Vec<TrialSessionEvidenceBoundaryStatus>,
    pub no_approval_no_authority_markers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialSessionEvidenceRecord {
    pub metadata: TrialSessionEvidenceMetadata,
    pub payload: TrialSessionEvidencePayload,
    pub absence_markers: TrialSessionEvidenceAbsenceMarkers,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialSessionEvidenceProjection {
    pub status: TrialSessionEvidenceStatus,
    pub evidence_id: Option<String>,
    pub evidence_version: String,
    pub evidence_classification: String,
    pub production_classification: String,
    pub distribution_classification: String,
    pub authority_classification: String,
    pub trial_package_linkage: String,
    pub runbook_linkage: String,
    pub failure_drill_linkage: String,
    pub included_evidence_summary: Vec<String>,
    pub workflow_snapshot_summary: String,
    pub local_candidate_materialization_snapshot: String,
    pub replay_status_snapshot: String,
    pub evidence_export_session_package_restore_snapshot: String,
    pub stop_condition_snapshot: Vec<String>,
    pub escalation_snapshot: Vec<String>,
    pub failure_category_snapshot: Vec<String>,
    pub absence_marker_summary: Vec<String>,
    pub validation_status: TrialSessionEvidenceValidationStatus,
    pub validation_errors: Vec<String>,
    pub read_back_validation_status: Option<TrialSessionEvidenceValidationStatus>,
    pub boundary_status: Vec<TrialSessionEvidenceBoundaryStatus>,
    pub local_only_non_authoritative_note: String,
    pub local_preparation_only_note: String,
    pub no_trial_approval_note: String,
    pub no_release_deployment_readiness_note: String,
    pub read_back_validation_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialSessionEvidenceWriteResult {
    pub status: TrialSessionEvidenceStatus,
    pub path: String,
    pub bytes_written: usize,
    pub projection: TrialSessionEvidenceProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialSessionEvidenceReadResult {
    pub status: TrialSessionEvidenceStatus,
    pub path: String,
    pub record: Option<TrialSessionEvidenceRecord>,
    pub projection: TrialSessionEvidenceProjection,
}

pub const TRIAL_SESSION_EVIDENCE_VERSION: &str = "trial-session-evidence-v1";

pub fn initial_trial_session_evidence_projection() -> TrialSessionEvidenceProjection {
    TrialSessionEvidenceProjection {
        status: TrialSessionEvidenceStatus::NotCaptured,
        evidence_id: None,
        evidence_version: TRIAL_SESSION_EVIDENCE_VERSION.to_string(),
        evidence_classification: TrialSessionEvidenceClassification::TrialSessionEvidenceOnly.code().to_string(),
        production_classification: TrialSessionEvidenceProductionClassification::NonProduction.code().to_string(),
        distribution_classification: TrialSessionEvidenceDistributionClassification::LocalOnlyNonPublic.code().to_string(),
        authority_classification: TrialSessionEvidenceAuthorityClassification::NonAuthoritativeEvidence.code().to_string(),
        trial_package_linkage: "trial package linkage not captured".to_string(),
        runbook_linkage: "runbook linkage not captured".to_string(),
        failure_drill_linkage: "failure drill linkage not captured".to_string(),
        included_evidence_summary: Vec::new(),
        workflow_snapshot_summary: "workflow snapshot not captured".to_string(),
        local_candidate_materialization_snapshot: "local candidate materialization snapshot not captured".to_string(),
        replay_status_snapshot: "replay/status snapshot not captured".to_string(),
        evidence_export_session_package_restore_snapshot: "evidence export/session package/restore snapshot not captured".to_string(),
        stop_condition_snapshot: Vec::new(),
        escalation_snapshot: Vec::new(),
        failure_category_snapshot: Vec::new(),
        absence_marker_summary: trial_session_evidence_absence_markers().marker_summary,
        validation_status: TrialSessionEvidenceValidationStatus::NotValidated,
        validation_errors: Vec::new(),
        read_back_validation_status: None,
        boundary_status: trial_session_evidence_boundary_statuses(),
        local_only_non_authoritative_note: "Trial session evidence is local-only, non-public, and non-authoritative.".to_string(),
        local_preparation_only_note: "Evidence capture records local trial-preparation state only.".to_string(),
        no_trial_approval_note: "Evidence capture does not start or approve a controlled trial.".to_string(),
        no_release_deployment_readiness_note: "This evidence is not release, deployment, readiness, public-use, or production-use evidence.".to_string(),
        read_back_validation_note: "Read-back validation checks evidence structure only.".to_string(),
    }
}

fn stable_trial_session_evidence_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn trial_session_evidence_payload_basis(
    payload: &TrialSessionEvidencePayload,
    markers: &TrialSessionEvidenceAbsenceMarkers,
) -> String {
    format!(
        "version={}|classification=trial_session_evidence_only|production=non_production|distribution=local_only_non_public|authority=non_authoritative_evidence|payload={:?}|absence={:?}",
        TRIAL_SESSION_EVIDENCE_VERSION, payload, markers
    )
}

pub fn derive_trial_session_evidence_record(
    state: &LocalOperatorShellState,
) -> TrialSessionEvidenceRecord {
    let absence_markers = trial_session_evidence_absence_markers();
    let runbook = &state.trial_operator_runbook;
    let failure = &state.trial_failure_drill;
    let payload = TrialSessionEvidencePayload {
        trial_package_id: runbook.trial_package_id.clone().unwrap_or_default(),
        trial_package_version: state
            .controlled_internal_trial_package_projection
            .package_version
            .clone(),
        trial_package_status: runbook.trial_package_status.code().to_string(),
        trial_package_validation_status: runbook.trial_package_validation_status.code().to_string(),
        trial_package_read_back_status: runbook
            .trial_package_read_back_status
            .map(|status| status.code().to_string())
            .unwrap_or_else(|| "not_read".to_string()),
        runbook_status: runbook.status.code().to_string(),
        runbook_current_step: runbook
            .current_step
            .map(|step| step.code().to_string())
            .unwrap_or_else(|| "none".to_string()),
        runbook_current_blocker: runbook.current_blocker.guidance.clone(),
        failure_drill_status: failure.status.code().to_string(),
        failure_drill_highest_severity: failure.highest_severity.code().to_string(),
        workflow_status_snapshot: state
            .complete_local_operator_workflow
            .status
            .code()
            .to_string(),
        local_candidate_materialization_snapshot: state
            .local_candidate_output
            .status
            .code()
            .to_string(),
        provider_output_pipeline_snapshot: state
            .local_provider_output_pipeline
            .status
            .code()
            .to_string(),
        operator_decision_snapshot: state.operator_candidate_decision.status.code().to_string(),
        replay_status_snapshot: state.run.decision_replay.summary.clone(),
        local_evidence_export_snapshot: state.local_session_evidence_export.summary.clone(),
        local_session_package_snapshot: state
            .local_session_package_projection
            .status
            .code()
            .to_string(),
        restore_history_snapshot: format!(
            "{} / {}",
            state.local_session_restore_projection.status.code(),
            state.local_session_history_projection.status.code()
        ),
        stop_condition_snapshot: runbook.stop_condition_summary.clone(),
        escalation_guidance_snapshot: failure
            .escalation_guidance
            .iter()
            .map(|entry| format!("{}:{}", entry.role.code(), entry.guidance))
            .collect(),
        failure_category_snapshot: failure
            .categories
            .iter()
            .map(|entry| format!("{}={}", entry.category.code(), entry.severity.code()))
            .collect(),
        current_blocker_snapshot: format!(
            "step={}; workflow_step={}; workflow_error={}; guidance={}",
            runbook
                .current_blocker
                .step
                .map(|step| step.code())
                .unwrap_or("none"),
            runbook
                .current_blocker
                .workflow_step
                .map(|step| step.code())
                .unwrap_or("none"),
            runbook
                .current_blocker
                .workflow_error
                .map(|error| error.code())
                .unwrap_or("none"),
            runbook.current_blocker.guidance
        ),
        boundary_status: trial_session_evidence_boundary_statuses(),
        no_approval_no_authority_markers: vec![
            "no_controlled_human_use_approval".to_string(),
            "no_readiness_approval".to_string(),
            "no_release_approval".to_string(),
            "no_deployment_approval".to_string(),
            "no_public_use_approval".to_string(),
            "no_production_approval".to_string(),
            "no_provider_trust".to_string(),
            "no_action_authorization".to_string(),
            "no_replay_repair".to_string(),
            "no_recovery_promotion".to_string(),
        ],
    };
    let digest = stable_trial_session_evidence_digest(&trial_session_evidence_payload_basis(
        &payload,
        &absence_markers,
    ));
    TrialSessionEvidenceRecord {
        metadata: TrialSessionEvidenceMetadata {
            evidence_id: format!("trial-session-evidence-{digest}"),
            evidence_version: TRIAL_SESSION_EVIDENCE_VERSION.to_string(),
            evidence_classification: TrialSessionEvidenceClassification::TrialSessionEvidenceOnly
                .code()
                .to_string(),
            production_classification: TrialSessionEvidenceProductionClassification::NonProduction
                .code()
                .to_string(),
            distribution_classification:
                TrialSessionEvidenceDistributionClassification::LocalOnlyNonPublic
                    .code()
                    .to_string(),
            authority_classification:
                TrialSessionEvidenceAuthorityClassification::NonAuthoritativeEvidence
                    .code()
                    .to_string(),
            evidence_status: TrialSessionEvidenceStatus::EvidenceProjected,
            validation_status: TrialSessionEvidenceValidationStatus::Valid,
            content_digest: digest,
        },
        payload,
        absence_markers,
    }
}

fn trial_session_evidence_validation_errors(
    record: &TrialSessionEvidenceRecord,
) -> Vec<TrialSessionEvidenceValidationError> {
    let mut errors = Vec::new();
    if record.metadata.evidence_id.is_empty() {
        errors.push(TrialSessionEvidenceValidationError::MissingEvidenceId);
    }
    if record.metadata.evidence_version.is_empty() {
        errors.push(TrialSessionEvidenceValidationError::MissingEvidenceVersion);
    }
    if record.metadata.evidence_classification
        != TrialSessionEvidenceClassification::TrialSessionEvidenceOnly.code()
    {
        errors.push(TrialSessionEvidenceValidationError::InvalidEvidenceClassification);
    }
    if record.metadata.production_classification
        != TrialSessionEvidenceProductionClassification::NonProduction.code()
    {
        errors.push(TrialSessionEvidenceValidationError::InvalidProductionClassification);
    }
    if record.metadata.distribution_classification
        != TrialSessionEvidenceDistributionClassification::LocalOnlyNonPublic.code()
    {
        errors.push(TrialSessionEvidenceValidationError::InvalidDistributionClassification);
    }
    if record.metadata.authority_classification
        != TrialSessionEvidenceAuthorityClassification::NonAuthoritativeEvidence.code()
    {
        errors.push(TrialSessionEvidenceValidationError::InvalidAuthorityClassification);
    }
    if record.payload.trial_package_id.is_empty()
        || record.payload.trial_package_version.is_empty()
        || record.payload.trial_package_status
            == ControlledInternalTrialPackageStatus::NotPackaged.code()
        || record.payload.trial_package_validation_status
            != ControlledInternalTrialPackageValidationStatus::Valid.code()
    {
        errors.push(TrialSessionEvidenceValidationError::MissingTrialPackageLinkage);
    }
    if record.payload.runbook_status.is_empty()
        || record.payload.runbook_status == TrialOperatorRunbookStatus::NotAvailable.code()
        || record.payload.runbook_current_blocker.is_empty()
    {
        errors.push(TrialSessionEvidenceValidationError::MissingRunbookLinkage);
    }
    if record.payload.failure_drill_status.is_empty()
        || record.payload.failure_drill_highest_severity.is_empty()
    {
        errors.push(TrialSessionEvidenceValidationError::MissingFailureDrillLinkage);
    }
    if record.payload.workflow_status_snapshot.is_empty() {
        errors.push(TrialSessionEvidenceValidationError::MissingWorkflowSnapshot);
    }
    if record.payload.stop_condition_snapshot.is_empty() {
        errors.push(TrialSessionEvidenceValidationError::MissingStopConditionSnapshot);
    }
    if record.payload.escalation_guidance_snapshot.is_empty() {
        errors.push(TrialSessionEvidenceValidationError::MissingEscalationSnapshot);
    }
    let markers = &record.absence_markers;
    if !markers.release_artifact_absent
        || !markers.deployment_artifact_absent
        || !markers.readiness_approval_absent
        || !markers.public_use_approval_absent
        || !markers.production_use_approval_absent
        || !markers.provider_trust_absent
        || !markers.action_authorization_absent
        || !markers.replay_repair_absent
        || !markers.recovery_promotion_absent
        || !markers.default_persistence_absent
        || !markers.automatic_save_absent
        || !markers.background_persistence_absent
        || !markers.network_sync_absent
        || markers.marker_summary.is_empty()
    {
        errors.push(TrialSessionEvidenceValidationError::MissingAbsenceMarker);
    }
    let text = format!("{:?}", record).to_ascii_lowercase();
    let claim_checks = [
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
            "claim:production_persistence_enabled",
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
            "claim:action_executed",
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
    for (needle, error) in claim_checks {
        if text.contains(needle) && !errors.contains(&error) {
            errors.push(error);
        }
    }
    let expected_digest = stable_trial_session_evidence_digest(
        &trial_session_evidence_payload_basis(&record.payload, &record.absence_markers),
    );
    let expected_id = format!("trial-session-evidence-{expected_digest}");
    if record.metadata.content_digest != expected_digest
        || (!record.metadata.evidence_id.is_empty() && record.metadata.evidence_id != expected_id)
    {
        errors.push(TrialSessionEvidenceValidationError::DeterministicDigestMismatch);
    }
    errors
}

pub fn validate_trial_session_evidence_record(
    record: &TrialSessionEvidenceRecord,
) -> Result<(), Vec<TrialSessionEvidenceValidationError>> {
    let errors = trial_session_evidence_validation_errors(record);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn project_trial_session_evidence_status(
    record: Option<&TrialSessionEvidenceRecord>,
    read_back_status: Option<TrialSessionEvidenceValidationStatus>,
) -> TrialSessionEvidenceProjection {
    match record {
        None => initial_trial_session_evidence_projection(),
        Some(record) => {
            let errors = trial_session_evidence_validation_errors(record);
            let validation_status = if errors.is_empty() {
                TrialSessionEvidenceValidationStatus::Valid
            } else {
                TrialSessionEvidenceValidationStatus::Invalid
            };
            TrialSessionEvidenceProjection {
                status: if errors.is_empty() { record.metadata.evidence_status } else { TrialSessionEvidenceStatus::EvidenceRejected },
                evidence_id: Some(record.metadata.evidence_id.clone()),
                evidence_version: record.metadata.evidence_version.clone(),
                evidence_classification: record.metadata.evidence_classification.clone(),
                production_classification: record.metadata.production_classification.clone(),
                distribution_classification: record.metadata.distribution_classification.clone(),
                authority_classification: record.metadata.authority_classification.clone(),
                trial_package_linkage: format!("{} / {} / {}", record.payload.trial_package_id, record.payload.trial_package_status, record.payload.trial_package_validation_status),
                runbook_linkage: format!("{} / {}", record.payload.runbook_status, record.payload.runbook_current_step),
                failure_drill_linkage: format!("{} / {}", record.payload.failure_drill_status, record.payload.failure_drill_highest_severity),
                included_evidence_summary: vec![
                    format!("provider output pipeline snapshot: {}", record.payload.provider_output_pipeline_snapshot),
                    format!("operator decision snapshot: {}", record.payload.operator_decision_snapshot),
                    format!("local evidence export snapshot: {}", record.payload.local_evidence_export_snapshot),
                    format!("local session package snapshot: {}", record.payload.local_session_package_snapshot),
                    format!("restore/history snapshot: {}", record.payload.restore_history_snapshot),
                ],
                workflow_snapshot_summary: record.payload.workflow_status_snapshot.clone(),
                local_candidate_materialization_snapshot: record.payload.local_candidate_materialization_snapshot.clone(),
                replay_status_snapshot: record.payload.replay_status_snapshot.clone(),
                evidence_export_session_package_restore_snapshot: format!("{} / {} / {}", record.payload.local_evidence_export_snapshot, record.payload.local_session_package_snapshot, record.payload.restore_history_snapshot),
                stop_condition_snapshot: record.payload.stop_condition_snapshot.clone(),
                escalation_snapshot: record.payload.escalation_guidance_snapshot.clone(),
                failure_category_snapshot: record.payload.failure_category_snapshot.clone(),
                absence_marker_summary: record.absence_markers.marker_summary.clone(),
                validation_status,
                validation_errors: errors.into_iter().map(|error| error.code().to_string()).collect(),
                read_back_validation_status: read_back_status,
                boundary_status: record.payload.boundary_status.clone(),
                local_only_non_authoritative_note: "Trial session evidence is local-only, non-public, and non-authoritative.".to_string(),
                local_preparation_only_note: "Evidence capture records local trial-preparation state only.".to_string(),
                no_trial_approval_note: "Evidence capture does not start or approve a controlled trial.".to_string(),
                no_release_deployment_readiness_note: "This evidence is not release, deployment, readiness, public-use, or production-use evidence.".to_string(),
                read_back_validation_note: "Read-back validation checks evidence structure only.".to_string(),
            }
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialReplayRestoreVerificationStatus {
    NotVerified,
    VerificationProjected,
    VerificationPassed,
    VerificationRejected,
    VerificationInputMissing,
    InvalidVerificationInput,
}

impl TrialReplayRestoreVerificationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotVerified => "not_verified",
            Self::VerificationProjected => "trial_replay_restore_verification_projected",
            Self::VerificationPassed => "verification_passed",
            Self::VerificationRejected => "verification_rejected",
            Self::VerificationInputMissing => "verification_input_missing",
            Self::InvalidVerificationInput => "invalid_verification_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrialReplayRestoreVerificationMismatch {
    MissingTrialPackageReadBack,
    MissingTrialSessionEvidenceReadBack,
    TrialPackageReadBackInvalid,
    TrialSessionEvidenceReadBackInvalid,
    PackageIdMismatch,
    PackageVersionMismatch,
    PackageClassificationMismatch,
    ProductionClassificationMismatch,
    DistributionClassificationMismatch,
    AuthorityClassificationMismatch,
    WorkflowStatusMismatch,
    LocalCandidateMaterializationStatusMismatch,
    ProviderOutputPipelineStatusMismatch,
    OperatorDecisionStatusMismatch,
    ReplayStatusSnapshotMismatch,
    RestoreHistorySnapshotMismatch,
    StopConditionSnapshotMismatch,
    EscalationSnapshotMismatch,
    FailureCategorySnapshotMismatch,
    ReadinessClaimDetected,
    ReleaseClaimDetected,
    DeploymentClaimDetected,
    PublicUseClaimDetected,
    ProductionUseClaimDetected,
    ProviderTrustClaimDetected,
    ActionAuthorizationClaimDetected,
    ReplayRepairClaimDetected,
    RecoveryPromotionClaimDetected,
}

impl TrialReplayRestoreVerificationMismatch {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingTrialPackageReadBack => "missing_trial_package_read_back",
            Self::MissingTrialSessionEvidenceReadBack => "missing_trial_session_evidence_read_back",
            Self::TrialPackageReadBackInvalid => "trial_package_read_back_invalid",
            Self::TrialSessionEvidenceReadBackInvalid => "trial_session_evidence_read_back_invalid",
            Self::PackageIdMismatch => "package_id_mismatch",
            Self::PackageVersionMismatch => "package_version_mismatch",
            Self::PackageClassificationMismatch => "package_classification_mismatch",
            Self::ProductionClassificationMismatch => "production_classification_mismatch",
            Self::DistributionClassificationMismatch => "distribution_classification_mismatch",
            Self::AuthorityClassificationMismatch => "authority_classification_mismatch",
            Self::WorkflowStatusMismatch => "workflow_status_mismatch",
            Self::LocalCandidateMaterializationStatusMismatch => {
                "local_candidate_materialization_status_mismatch"
            }
            Self::ProviderOutputPipelineStatusMismatch => {
                "provider_output_pipeline_status_mismatch"
            }
            Self::OperatorDecisionStatusMismatch => "operator_decision_status_mismatch",
            Self::ReplayStatusSnapshotMismatch => "replay_status_snapshot_mismatch",
            Self::RestoreHistorySnapshotMismatch => "restore_history_snapshot_mismatch",
            Self::StopConditionSnapshotMismatch => "stop_condition_snapshot_mismatch",
            Self::EscalationSnapshotMismatch => "escalation_snapshot_mismatch",
            Self::FailureCategorySnapshotMismatch => "failure_category_snapshot_mismatch",
            Self::ReadinessClaimDetected => "readiness_claim_detected",
            Self::ReleaseClaimDetected => "release_claim_detected",
            Self::DeploymentClaimDetected => "deployment_claim_detected",
            Self::PublicUseClaimDetected => "public_use_claim_detected",
            Self::ProductionUseClaimDetected => "production_use_claim_detected",
            Self::ProviderTrustClaimDetected => "provider_trust_claim_detected",
            Self::ActionAuthorizationClaimDetected => "action_authorization_claim_detected",
            Self::ReplayRepairClaimDetected => "replay_repair_claim_detected",
            Self::RecoveryPromotionClaimDetected => "recovery_promotion_claim_detected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialReplayRestoreVerificationBoundaryStatus {
    LocalVerificationOnly,
    NonPublicVerification,
    VerificationNotAuthority,
    NoTrialExecution,
    NoControlledHumanUseApproval,
    NoReadinessApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoProductionApproval,
    NoProviderTrust,
    NoActionAuthorization,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl TrialReplayRestoreVerificationBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalVerificationOnly => "local_verification_only",
            Self::NonPublicVerification => "non_public_verification",
            Self::VerificationNotAuthority => "verification_not_authority",
            Self::NoTrialExecution => "no_trial_execution",
            Self::NoControlledHumanUseApproval => "no_controlled_human_use_approval",
            Self::NoReadinessApproval => "no_readiness_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoActionAuthorization => "no_action_authorization",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialReplayRestoreVerificationComparisonSummary {
    pub package_read_back_status: String,
    pub evidence_read_back_status: String,
    pub package_evidence_linkage_status: String,
    pub workflow_linkage_status: String,
    pub replay_status_comparison: String,
    pub restore_history_comparison: String,
    pub stop_condition_comparison: String,
    pub escalation_failure_comparison: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialReplayRestoreVerificationProjection {
    pub status: TrialReplayRestoreVerificationStatus,
    pub verification_id: Option<String>,
    pub comparison_summary: TrialReplayRestoreVerificationComparisonSummary,
    pub matched_fields: Vec<String>,
    pub mismatches: Vec<TrialReplayRestoreVerificationMismatch>,
    pub missing_inputs: Vec<String>,
    pub boundary_status: Vec<TrialReplayRestoreVerificationBoundaryStatus>,
    pub local_only_non_authority_note: String,
    pub comparison_scope_note: String,
    pub no_replay_repair_note: String,
    pub no_recovery_promotion_note: String,
    pub no_approval_note: String,
    pub no_action_execution_note: String,
}

pub fn trial_replay_restore_verification_boundary_statuses(
) -> Vec<TrialReplayRestoreVerificationBoundaryStatus> {
    vec![
        TrialReplayRestoreVerificationBoundaryStatus::LocalVerificationOnly,
        TrialReplayRestoreVerificationBoundaryStatus::NonPublicVerification,
        TrialReplayRestoreVerificationBoundaryStatus::VerificationNotAuthority,
        TrialReplayRestoreVerificationBoundaryStatus::NoTrialExecution,
        TrialReplayRestoreVerificationBoundaryStatus::NoControlledHumanUseApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoReadinessApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoReleaseApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoDeploymentApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoPublicUseApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoProductionApproval,
        TrialReplayRestoreVerificationBoundaryStatus::NoProviderTrust,
        TrialReplayRestoreVerificationBoundaryStatus::NoActionAuthorization,
        TrialReplayRestoreVerificationBoundaryStatus::NoReplayRepair,
        TrialReplayRestoreVerificationBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub fn initial_trial_replay_restore_verification_projection(
) -> TrialReplayRestoreVerificationProjection {
    TrialReplayRestoreVerificationProjection {
        status: TrialReplayRestoreVerificationStatus::NotVerified,
        verification_id: None,
        comparison_summary: TrialReplayRestoreVerificationComparisonSummary {
            package_read_back_status: "not_verified".to_string(),
            evidence_read_back_status: "not_verified".to_string(),
            package_evidence_linkage_status: "not_verified".to_string(),
            workflow_linkage_status: "not_verified".to_string(),
            replay_status_comparison: "not_verified".to_string(),
            restore_history_comparison: "not_verified".to_string(),
            stop_condition_comparison: "not_verified".to_string(),
            escalation_failure_comparison: "not_verified".to_string(),
        },
        matched_fields: Vec::new(),
        mismatches: Vec::new(),
        missing_inputs: Vec::new(),
        boundary_status: trial_replay_restore_verification_boundary_statuses(),
        local_only_non_authority_note: "Trial replay and restore verification is local-only, non-public, and non-authoritative.".to_string(),
        comparison_scope_note: "Verification compares trial artifacts and restore/replay projections.".to_string(),
        no_replay_repair_note: "Verification does not repair replay.".to_string(),
        no_recovery_promotion_note: "Verification does not promote recovery.".to_string(),
        no_approval_note: "Verification does not approve controlled human use, readiness, release, deployment, public use, or production use.".to_string(),
        no_action_execution_note: "Verification does not execute actions.".to_string(),
    }
}

fn stable_trial_replay_restore_verification_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("trial-replay-restore-verification-{hash:016x}")
}

fn add_verification_mismatch(
    mismatches: &mut Vec<TrialReplayRestoreVerificationMismatch>,
    mismatch: TrialReplayRestoreVerificationMismatch,
) {
    if !mismatches.contains(&mismatch) {
        mismatches.push(mismatch);
    }
}

fn compare_verification_field(
    matched_fields: &mut Vec<String>,
    mismatches: &mut Vec<TrialReplayRestoreVerificationMismatch>,
    field: &str,
    left: &str,
    right: &str,
    mismatch: TrialReplayRestoreVerificationMismatch,
) {
    if left == right {
        matched_fields.push(format!("{field}={left}"));
    } else {
        add_verification_mismatch(mismatches, mismatch);
    }
}

fn current_trial_verification_restore_history_snapshot(state: &LocalOperatorShellState) -> String {
    format!(
        "{} / {}",
        state.local_session_restore_projection.status.code(),
        state.local_session_history_projection.status.code()
    )
}

fn current_trial_verification_escalation_snapshot(state: &LocalOperatorShellState) -> Vec<String> {
    state
        .trial_failure_drill
        .escalation_guidance
        .iter()
        .map(|entry| format!("{}:{}", entry.role.code(), entry.guidance))
        .collect()
}

fn current_trial_verification_failure_snapshot(state: &LocalOperatorShellState) -> Vec<String> {
    state
        .trial_failure_drill
        .categories
        .iter()
        .map(|entry| format!("{}={}", entry.category.code(), entry.severity.code()))
        .collect()
}

fn collect_trial_verification_claim_mismatches(
    text: &str,
    mismatches: &mut Vec<TrialReplayRestoreVerificationMismatch>,
) {
    let checks = [
        (
            "claim:readiness_approved",
            TrialReplayRestoreVerificationMismatch::ReadinessClaimDetected,
        ),
        (
            "claim:release_candidate_approved",
            TrialReplayRestoreVerificationMismatch::ReleaseClaimDetected,
        ),
        (
            "claim:deployment_enabled",
            TrialReplayRestoreVerificationMismatch::DeploymentClaimDetected,
        ),
        (
            "claim:public_use_approved",
            TrialReplayRestoreVerificationMismatch::PublicUseClaimDetected,
        ),
        (
            "claim:production_use_approved",
            TrialReplayRestoreVerificationMismatch::ProductionUseClaimDetected,
        ),
        (
            "claim:production_persistence_enabled",
            TrialReplayRestoreVerificationMismatch::ProductionUseClaimDetected,
        ),
        (
            "claim:provider_trusted",
            TrialReplayRestoreVerificationMismatch::ProviderTrustClaimDetected,
        ),
        (
            "claim:action_authorized",
            TrialReplayRestoreVerificationMismatch::ActionAuthorizationClaimDetected,
        ),
        (
            "claim:action_executed",
            TrialReplayRestoreVerificationMismatch::ActionAuthorizationClaimDetected,
        ),
        (
            "claim:replay_repaired",
            TrialReplayRestoreVerificationMismatch::ReplayRepairClaimDetected,
        ),
        (
            "claim:recovery_promoted",
            TrialReplayRestoreVerificationMismatch::RecoveryPromotionClaimDetected,
        ),
    ];
    let lowered = text.to_ascii_lowercase();
    for (needle, mismatch) in checks {
        if lowered.contains(needle) {
            add_verification_mismatch(mismatches, mismatch);
        }
    }
}

pub fn derive_trial_replay_restore_verification_projection(
    state: &LocalOperatorShellState,
    package_read_back: Option<&ControlledInternalTrialPackage>,
    evidence_read_back: Option<&TrialSessionEvidenceRecord>,
) -> TrialReplayRestoreVerificationProjection {
    let mut projection = initial_trial_replay_restore_verification_projection();
    projection.status = TrialReplayRestoreVerificationStatus::VerificationProjected;
    let mut matched_fields = Vec::new();
    let mut mismatches = Vec::new();
    let mut missing_inputs = Vec::new();

    let Some(package) = package_read_back else {
        missing_inputs.push("trial_package_read_back".to_string());
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::MissingTrialPackageReadBack,
        );
        projection.status = TrialReplayRestoreVerificationStatus::VerificationInputMissing;
        projection.missing_inputs = missing_inputs;
        projection.mismatches = mismatches;
        projection.comparison_summary.package_read_back_status =
            "missing_trial_package_read_back".to_string();
        return projection;
    };
    let Some(evidence) = evidence_read_back else {
        missing_inputs.push("trial_session_evidence_read_back".to_string());
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::MissingTrialSessionEvidenceReadBack,
        );
        projection.status = TrialReplayRestoreVerificationStatus::VerificationInputMissing;
        projection.missing_inputs = missing_inputs;
        projection.mismatches = mismatches;
        projection.comparison_summary.package_read_back_status =
            "package_read_back_present".to_string();
        projection.comparison_summary.evidence_read_back_status =
            "missing_trial_session_evidence_read_back".to_string();
        return projection;
    };

    let package_valid = validate_controlled_internal_trial_package(package).is_ok()
        && package.metadata.package_status
            == ControlledInternalTrialPackageStatus::PackageReadBackValidated;
    if package_valid {
        matched_fields.push("trial_package_read_back=valid".to_string());
    } else {
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::TrialPackageReadBackInvalid,
        );
    }
    let evidence_valid = validate_trial_session_evidence_record(evidence).is_ok()
        && evidence.metadata.evidence_status
            == TrialSessionEvidenceStatus::EvidenceReadBackValidated;
    if evidence_valid {
        matched_fields.push("trial_session_evidence_read_back=valid".to_string());
    } else {
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::TrialSessionEvidenceReadBackInvalid,
        );
    }

    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "package_id",
        &package.metadata.package_id,
        &evidence.payload.trial_package_id,
        TrialReplayRestoreVerificationMismatch::PackageIdMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "package_version",
        &package.metadata.package_version,
        &evidence.payload.trial_package_version,
        TrialReplayRestoreVerificationMismatch::PackageVersionMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "package_classification",
        &package.metadata.package_classification,
        ControlledInternalTrialPackageClassification::ControlledInternalTrialPackageOnly.code(),
        TrialReplayRestoreVerificationMismatch::PackageClassificationMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "production_classification",
        &package.metadata.production_classification,
        TrialSessionEvidenceProductionClassification::NonProduction.code(),
        TrialReplayRestoreVerificationMismatch::ProductionClassificationMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "distribution_classification",
        &package.metadata.distribution_classification,
        TrialSessionEvidenceDistributionClassification::LocalOnlyNonPublic.code(),
        TrialReplayRestoreVerificationMismatch::DistributionClassificationMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "authority_classification",
        &evidence.metadata.authority_classification,
        TrialSessionEvidenceAuthorityClassification::NonAuthoritativeEvidence.code(),
        TrialReplayRestoreVerificationMismatch::AuthorityClassificationMismatch,
    );

    let included = &package.payload.included_evidence;
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "workflow_status",
        &included.local_beta_workflow_status,
        &evidence.payload.workflow_status_snapshot,
        TrialReplayRestoreVerificationMismatch::WorkflowStatusMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "local_candidate_materialization_status",
        &included.local_candidate_materialization_status,
        &evidence.payload.local_candidate_materialization_snapshot,
        TrialReplayRestoreVerificationMismatch::LocalCandidateMaterializationStatusMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "provider_output_pipeline_status",
        &included.provider_output_pipeline_status,
        &evidence.payload.provider_output_pipeline_snapshot,
        TrialReplayRestoreVerificationMismatch::ProviderOutputPipelineStatusMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "operator_decision_status",
        &included.operator_decision_status,
        &evidence.payload.operator_decision_snapshot,
        TrialReplayRestoreVerificationMismatch::OperatorDecisionStatusMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "replay_status_snapshot",
        &state.run.decision_replay.summary,
        &evidence.payload.replay_status_snapshot,
        TrialReplayRestoreVerificationMismatch::ReplayStatusSnapshotMismatch,
    );
    compare_verification_field(
        &mut matched_fields,
        &mut mismatches,
        "restore_history_snapshot",
        &current_trial_verification_restore_history_snapshot(state),
        &evidence.payload.restore_history_snapshot,
        TrialReplayRestoreVerificationMismatch::RestoreHistorySnapshotMismatch,
    );

    let package_stop_conditions = package
        .payload
        .stop_conditions
        .iter()
        .map(|condition| condition.code().to_string())
        .collect::<Vec<_>>();
    if package_stop_conditions == evidence.payload.stop_condition_snapshot {
        matched_fields.push("stop_condition_snapshot=matched".to_string());
    } else {
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::StopConditionSnapshotMismatch,
        );
    }
    if current_trial_verification_escalation_snapshot(state)
        == evidence.payload.escalation_guidance_snapshot
    {
        matched_fields.push("escalation_snapshot=matched".to_string());
    } else {
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::EscalationSnapshotMismatch,
        );
    }
    if current_trial_verification_failure_snapshot(state)
        == evidence.payload.failure_category_snapshot
    {
        matched_fields.push("failure_category_snapshot=matched".to_string());
    } else {
        add_verification_mismatch(
            &mut mismatches,
            TrialReplayRestoreVerificationMismatch::FailureCategorySnapshotMismatch,
        );
    }

    collect_trial_verification_claim_mismatches(
        &format!("{:?} {:?}", package, evidence),
        &mut mismatches,
    );
    mismatches.sort();

    let invalid_input = mismatches.iter().any(|mismatch| {
        matches!(
            mismatch,
            TrialReplayRestoreVerificationMismatch::TrialPackageReadBackInvalid
                | TrialReplayRestoreVerificationMismatch::TrialSessionEvidenceReadBackInvalid
        )
    });
    let passed = mismatches.is_empty();
    projection.status = if passed {
        TrialReplayRestoreVerificationStatus::VerificationPassed
    } else if invalid_input {
        TrialReplayRestoreVerificationStatus::InvalidVerificationInput
    } else {
        TrialReplayRestoreVerificationStatus::VerificationRejected
    };
    projection.verification_id = Some(stable_trial_replay_restore_verification_digest(&format!(
        "package={:?}|evidence={:?}|matched={:?}|mismatches={:?}",
        package.metadata, evidence.metadata, matched_fields, mismatches
    )));
    projection.comparison_summary = TrialReplayRestoreVerificationComparisonSummary {
        package_read_back_status: if package_valid { "package_read_back_valid" } else { "package_read_back_invalid" }.to_string(),
        evidence_read_back_status: if evidence_valid { "evidence_read_back_valid" } else { "evidence_read_back_invalid" }.to_string(),
        package_evidence_linkage_status: if passed { "package_evidence_linkage_matched" } else { "package_evidence_linkage_rejected" }.to_string(),
        workflow_linkage_status: if mismatches.iter().any(|m| matches!(m, TrialReplayRestoreVerificationMismatch::WorkflowStatusMismatch | TrialReplayRestoreVerificationMismatch::LocalCandidateMaterializationStatusMismatch | TrialReplayRestoreVerificationMismatch::ProviderOutputPipelineStatusMismatch | TrialReplayRestoreVerificationMismatch::OperatorDecisionStatusMismatch)) { "workflow_linkage_rejected" } else { "workflow_linkage_matched" }.to_string(),
        replay_status_comparison: if mismatches.contains(&TrialReplayRestoreVerificationMismatch::ReplayStatusSnapshotMismatch) { "replay/status comparison rejected" } else { "replay/status comparison matched" }.to_string(),
        restore_history_comparison: if mismatches.contains(&TrialReplayRestoreVerificationMismatch::RestoreHistorySnapshotMismatch) { "restore/history comparison rejected" } else { "restore/history comparison matched" }.to_string(),
        stop_condition_comparison: if mismatches.contains(&TrialReplayRestoreVerificationMismatch::StopConditionSnapshotMismatch) { "stop_condition_snapshot_rejected" } else { "stop_condition_snapshot_matched" }.to_string(),
        escalation_failure_comparison: if mismatches.iter().any(|m| matches!(m, TrialReplayRestoreVerificationMismatch::EscalationSnapshotMismatch | TrialReplayRestoreVerificationMismatch::FailureCategorySnapshotMismatch)) { "escalation_failure_snapshot_rejected" } else { "escalation_failure_snapshot_matched" }.to_string(),
    };
    projection.matched_fields = matched_fields;
    projection.mismatches = mismatches;
    projection.missing_inputs = missing_inputs;
    projection
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialRunStatus {
    NotStarted,
    PreconditionsMissing,
    ReadyForBoundedLocalTrialRun,
    TrialRunStarted,
    TrialRunInProgress,
    TrialRunBlocked,
    TrialRunCompletedLocally,
    TrialRunRejected,
    InvalidTrialRunRequest,
}

impl ControlledInternalTrialRunStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::PreconditionsMissing => "preconditions_missing",
            Self::ReadyForBoundedLocalTrialRun => "ready_for_bounded_local_trial_run",
            Self::TrialRunStarted => "trial_run_started",
            Self::TrialRunInProgress => "trial_run_in_progress",
            Self::TrialRunBlocked => "trial_run_blocked",
            Self::TrialRunCompletedLocally => "trial_run_completed_locally",
            Self::TrialRunRejected => "trial_run_rejected",
            Self::InvalidTrialRunRequest => "invalid_trial_run_request",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialRunStep {
    VerifyTrialPackage,
    VerifyRunbook,
    VerifyFailureDrill,
    VerifyTrialSessionEvidence,
    VerifyReplayRestore,
    VerifyCompleteLocalWorkflow,
    ObserveStopConditions,
    RecordManualOperatorStep,
    ProjectTrialRunSummary,
    ProjectTrialEvidenceLinkage,
    CloseLocalTrialRun,
}

impl ControlledInternalTrialRunStep {
    pub fn code(&self) -> &'static str {
        match self {
            Self::VerifyTrialPackage => "verify_trial_package",
            Self::VerifyRunbook => "verify_runbook",
            Self::VerifyFailureDrill => "verify_failure_drill",
            Self::VerifyTrialSessionEvidence => "verify_trial_session_evidence",
            Self::VerifyReplayRestore => "verify_replay_restore",
            Self::VerifyCompleteLocalWorkflow => "verify_complete_local_workflow",
            Self::ObserveStopConditions => "observe_stop_conditions",
            Self::RecordManualOperatorStep => "record_manual_operator_step",
            Self::ProjectTrialRunSummary => "project_trial_run_summary",
            Self::ProjectTrialEvidenceLinkage => "project_trial_evidence_linkage",
            Self::CloseLocalTrialRun => "close_local_trial_run",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialRunStepStatus {
    NotStarted,
    Available,
    Completed,
    Blocked,
    Rejected,
    ManualActionRequired,
    NotApplicable,
}

impl ControlledInternalTrialRunStepStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Available => "available",
            Self::Completed => "completed",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
            Self::ManualActionRequired => "manual_action_required",
            Self::NotApplicable => "not_applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialRunError {
    TrialPackageMissing,
    TrialPackageInvalid,
    RunbookMissing,
    RunbookBlocked,
    FailureDrillMissing,
    TrialSessionEvidenceMissing,
    TrialSessionEvidenceInvalid,
    ReplayRestoreVerificationMissing,
    ReplayRestoreVerificationRejected,
    ReplayRestoreVerificationNotPassed,
    CompleteLocalWorkflowMissing,
    CompleteLocalWorkflowBlocked,
    CompleteLocalWorkflowRejected,
    StopConditionObserved,
    ManualOperatorStepMissing,
    ReadinessClaimRejected,
    ReleaseClaimRejected,
    DeploymentClaimRejected,
    PublicUseClaimRejected,
    ProductionUseClaimRejected,
    ProviderTrustClaimRejected,
    ActionAuthorizationClaimRejected,
    ReplayRepairClaimRejected,
    RecoveryPromotionClaimRejected,
    SigningClaimRejected,
    PublishingClaimRejected,
    ReleaseArtifactClaimRejected,
}

impl ControlledInternalTrialRunError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::TrialPackageMissing => "trial_package_missing",
            Self::TrialPackageInvalid => "trial_package_invalid",
            Self::RunbookMissing => "runbook_missing",
            Self::RunbookBlocked => "runbook_blocked",
            Self::FailureDrillMissing => "failure_drill_missing",
            Self::TrialSessionEvidenceMissing => "trial_session_evidence_missing",
            Self::TrialSessionEvidenceInvalid => "trial_session_evidence_invalid",
            Self::ReplayRestoreVerificationMissing => "replay_restore_verification_missing",
            Self::ReplayRestoreVerificationRejected => "replay_restore_verification_rejected",
            Self::ReplayRestoreVerificationNotPassed => "replay_restore_verification_not_passed",
            Self::CompleteLocalWorkflowMissing => "complete_local_workflow_missing",
            Self::CompleteLocalWorkflowBlocked => "complete_local_workflow_blocked",
            Self::CompleteLocalWorkflowRejected => "complete_local_workflow_rejected",
            Self::StopConditionObserved => "stop_condition_observed",
            Self::ManualOperatorStepMissing => "manual_operator_step_missing",
            Self::ReadinessClaimRejected => "readiness_claim_rejected",
            Self::ReleaseClaimRejected => "release_claim_rejected",
            Self::DeploymentClaimRejected => "deployment_claim_rejected",
            Self::PublicUseClaimRejected => "public_use_claim_rejected",
            Self::ProductionUseClaimRejected => "production_use_claim_rejected",
            Self::ProviderTrustClaimRejected => "provider_trust_claim_rejected",
            Self::ActionAuthorizationClaimRejected => "action_authorization_claim_rejected",
            Self::ReplayRepairClaimRejected => "replay_repair_claim_rejected",
            Self::RecoveryPromotionClaimRejected => "recovery_promotion_claim_rejected",
            Self::SigningClaimRejected => "signing_claim_rejected",
            Self::PublishingClaimRejected => "publishing_claim_rejected",
            Self::ReleaseArtifactClaimRejected => "release_artifact_claim_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialManualStepStatus {
    NotStarted,
    ManualActionRequired,
    Recorded,
    Missing,
}

impl ControlledInternalTrialManualStepStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::ManualActionRequired => "manual_action_required",
            Self::Recorded => "recorded",
            Self::Missing => "manual_operator_step_missing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledInternalTrialExecutionBoundaryStatus {
    ControlledInternalTrialHarnessOnly,
    LocalTrialExecutionOnly,
    NonPublicTrialExecution,
    NoControlledHumanUseApproval,
    NoReadinessApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoProductionApproval,
    NoProviderTrust,
    NoActionAuthorization,
    NoReplayRepair,
    NoRecoveryPromotion,
    NoStopConditionAutomation,
    NoAutomatedEscalation,
}

impl ControlledInternalTrialExecutionBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ControlledInternalTrialHarnessOnly => "controlled_internal_trial_harness_only",
            Self::LocalTrialExecutionOnly => "local_trial_execution_only",
            Self::NonPublicTrialExecution => "non_public_trial_execution",
            Self::NoControlledHumanUseApproval => "no_controlled_human_use_approval",
            Self::NoReadinessApproval => "no_readiness_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoProductionApproval => "no_production_approval",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoActionAuthorization => "no_action_authorization",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
            Self::NoStopConditionAutomation => "no_stop_condition_automation",
            Self::NoAutomatedEscalation => "no_automated_escalation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialExecutionRequest {
    pub operator_step_recorded: bool,
    pub stop_condition_observed: bool,
    pub claims_readiness: bool,
    pub claims_release: bool,
    pub claims_deployment: bool,
    pub claims_public_use: bool,
    pub claims_production_use: bool,
    pub claims_provider_trust: bool,
    pub claims_action_authorization: bool,
    pub claims_replay_repair: bool,
    pub claims_recovery_promotion: bool,
    pub claims_signing: bool,
    pub claims_publishing: bool,
    pub claims_release_artifact: bool,
}

impl ControlledInternalTrialExecutionRequest {
    pub fn bounded_local_trial_run() -> Self {
        Self {
            operator_step_recorded: false,
            stop_condition_observed: false,
            claims_readiness: false,
            claims_release: false,
            claims_deployment: false,
            claims_public_use: false,
            claims_production_use: false,
            claims_provider_trust: false,
            claims_action_authorization: false,
            claims_replay_repair: false,
            claims_recovery_promotion: false,
            claims_signing: false,
            claims_publishing: false,
            claims_release_artifact: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialRunStepProjection {
    pub step: ControlledInternalTrialRunStep,
    pub status: ControlledInternalTrialRunStepStatus,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialStopObservation {
    pub status: String,
    pub observed: bool,
    pub markers: Vec<String>,
    pub enforcement_automated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialEvidenceLinkage {
    pub trial_package: String,
    pub runbook: String,
    pub failure_drill: String,
    pub trial_session_evidence: String,
    pub replay_restore_verification: String,
    pub local_workflow: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialExecutionCapabilitySurface {
    pub local_only: bool,
    pub non_public: bool,
    pub approves_controlled_human_use: bool,
    pub approves_readiness: bool,
    pub approves_release: bool,
    pub approves_deployment: bool,
    pub approves_public_use: bool,
    pub approves_production: bool,
    pub trusts_provider_output: bool,
    pub authorizes_actions: bool,
    pub repairs_replay: bool,
    pub promotes_recovery: bool,
    pub automates_stop_conditions: bool,
    pub automates_escalation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialRunProjection {
    pub run_id: String,
    pub status: ControlledInternalTrialRunStatus,
    pub current_step: Option<ControlledInternalTrialRunStep>,
    pub next_step: Option<ControlledInternalTrialRunStep>,
    pub steps: Vec<ControlledInternalTrialRunStepProjection>,
    pub current_blocker: Option<ControlledInternalTrialRunError>,
    pub rejection_reasons: Vec<ControlledInternalTrialRunError>,
    pub stop_condition_observation: ControlledInternalTrialStopObservation,
    pub manual_operator_step_status: ControlledInternalTrialManualStepStatus,
    pub evidence_linkage: ControlledInternalTrialEvidenceLinkage,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledInternalTrialExecutionProjection {
    pub status: ControlledInternalTrialRunStatus,
    pub active_run: Option<ControlledInternalTrialRunProjection>,
    pub last_rejected_run: Option<ControlledInternalTrialRunProjection>,
    pub precondition_status: Vec<String>,
    pub current_blocker: Option<ControlledInternalTrialRunError>,
    pub rejection_reasons: Vec<ControlledInternalTrialRunError>,
    pub evidence_linkage: ControlledInternalTrialEvidenceLinkage,
    pub boundary_statuses: Vec<ControlledInternalTrialExecutionBoundaryStatus>,
    pub capability_surface: ControlledInternalTrialExecutionCapabilitySurface,
    pub local_only_non_public_note: String,
    pub no_controlled_human_use_note: String,
    pub no_readiness_release_deployment_public_production_note: String,
    pub stop_condition_note: String,
    pub escalation_note: String,
    pub no_action_authorization_note: String,
}

pub fn controlled_internal_trial_execution_boundary_statuses(
) -> Vec<ControlledInternalTrialExecutionBoundaryStatus> {
    vec![
        ControlledInternalTrialExecutionBoundaryStatus::ControlledInternalTrialHarnessOnly,
        ControlledInternalTrialExecutionBoundaryStatus::LocalTrialExecutionOnly,
        ControlledInternalTrialExecutionBoundaryStatus::NonPublicTrialExecution,
        ControlledInternalTrialExecutionBoundaryStatus::NoControlledHumanUseApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoReadinessApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoReleaseApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoDeploymentApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoPublicUseApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoProductionApproval,
        ControlledInternalTrialExecutionBoundaryStatus::NoProviderTrust,
        ControlledInternalTrialExecutionBoundaryStatus::NoActionAuthorization,
        ControlledInternalTrialExecutionBoundaryStatus::NoReplayRepair,
        ControlledInternalTrialExecutionBoundaryStatus::NoRecoveryPromotion,
        ControlledInternalTrialExecutionBoundaryStatus::NoStopConditionAutomation,
        ControlledInternalTrialExecutionBoundaryStatus::NoAutomatedEscalation,
    ]
}

fn controlled_internal_trial_execution_capability_surface(
) -> ControlledInternalTrialExecutionCapabilitySurface {
    ControlledInternalTrialExecutionCapabilitySurface {
        local_only: true,
        non_public: true,
        approves_controlled_human_use: false,
        approves_readiness: false,
        approves_release: false,
        approves_deployment: false,
        approves_public_use: false,
        approves_production: false,
        trusts_provider_output: false,
        authorizes_actions: false,
        repairs_replay: false,
        promotes_recovery: false,
        automates_stop_conditions: false,
        automates_escalation: false,
    }
}

fn empty_controlled_internal_trial_execution_linkage() -> ControlledInternalTrialEvidenceLinkage {
    ControlledInternalTrialEvidenceLinkage {
        trial_package: "trial_package=missing".to_string(),
        runbook: "runbook=missing".to_string(),
        failure_drill: "failure_drill=missing".to_string(),
        trial_session_evidence: "trial_session_evidence=missing".to_string(),
        replay_restore_verification: "replay_restore_verification=missing".to_string(),
        local_workflow: "local_workflow=missing".to_string(),
    }
}

pub fn initial_controlled_internal_trial_execution_projection(
) -> ControlledInternalTrialExecutionProjection {
    ControlledInternalTrialExecutionProjection {
        status: ControlledInternalTrialRunStatus::NotStarted,
        active_run: None,
        last_rejected_run: None,
        precondition_status: Vec::new(),
        current_blocker: None,
        rejection_reasons: Vec::new(),
        evidence_linkage: empty_controlled_internal_trial_execution_linkage(),
        boundary_statuses: controlled_internal_trial_execution_boundary_statuses(),
        capability_surface: controlled_internal_trial_execution_capability_surface(),
        local_only_non_public_note: "Controlled internal trial execution harness is local-only and non-public.".to_string(),
        no_controlled_human_use_note: "The harness does not approve controlled human use.".to_string(),
        no_readiness_release_deployment_public_production_note: "The harness does not approve readiness, release, deployment, public use, or production use.".to_string(),
        stop_condition_note: "Stop conditions are observed only; enforcement is not automated in Phase 166.".to_string(),
        escalation_note: "Escalation is not automated.".to_string(),
        no_action_authorization_note: "No action authorization is granted.".to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialObservabilityStatus {
    NotObserved,
    ObservabilityProjected,
    TrialRunObserved,
    BlockedStateObserved,
    RejectedStateObserved,
    StopConditionObserved,
    VerificationMismatchObserved,
    ErrorReportProjected,
}

impl TrialObservabilityStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotObserved => "not_observed",
            Self::ObservabilityProjected => "observability_projected",
            Self::TrialRunObserved => "trial_run_observed",
            Self::BlockedStateObserved => "blocked_state_observed",
            Self::RejectedStateObserved => "rejected_state_observed",
            Self::StopConditionObserved => "stop_condition_observed",
            Self::VerificationMismatchObserved => "verification_mismatch_observed",
            Self::ErrorReportProjected => "error_report_projected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialErrorReportStatus {
    NoErrorsObserved,
    ErrorsObserved,
    BlockedErrorsObserved,
    RejectedErrorsObserved,
    InvalidObservabilityInput,
}

impl TrialErrorReportStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoErrorsObserved => "no_errors_observed",
            Self::ErrorsObserved => "errors_observed",
            Self::BlockedErrorsObserved => "blocked_errors_observed",
            Self::RejectedErrorsObserved => "rejected_errors_observed",
            Self::InvalidObservabilityInput => "invalid_observability_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialErrorCategory {
    NoTrialRun,
    TrialPreconditionMissing,
    TrialRunBlocked,
    TrialRunRejected,
    StopConditionObserved,
    ManualOperatorStepRequired,
    PackageMissing,
    PackageValidationFailed,
    PackageReadBackFailed,
    EvidenceMissing,
    EvidenceValidationFailed,
    EvidenceReadBackFailed,
    ReplayRestoreVerificationMissing,
    ReplayRestoreVerificationRejected,
    ReplayStatusMismatch,
    RestoreHistoryMismatch,
    ProviderPipelineBlocked,
    ProviderOutputValidationRejected,
    WorkflowBlocked,
    WorkflowRejected,
    MaterializationMissing,
    AuthorityClaimDetected,
    ReadinessClaimDetected,
    ReleaseOrDeploymentClaimDetected,
    PublicOrProductionUseClaimDetected,
    ActionAuthorizationClaimDetected,
    ReplayRepairOrRecoveryPromotionClaimDetected,
}

impl TrialErrorCategory {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoTrialRun => "no_trial_run",
            Self::TrialPreconditionMissing => "trial_precondition_missing",
            Self::TrialRunBlocked => "trial_run_blocked",
            Self::TrialRunRejected => "trial_run_rejected",
            Self::StopConditionObserved => "stop_condition_observed",
            Self::ManualOperatorStepRequired => "manual_operator_step_required",
            Self::PackageMissing => "package_missing",
            Self::PackageValidationFailed => "package_validation_failed",
            Self::PackageReadBackFailed => "package_read_back_failed",
            Self::EvidenceMissing => "evidence_missing",
            Self::EvidenceValidationFailed => "evidence_validation_failed",
            Self::EvidenceReadBackFailed => "evidence_read_back_failed",
            Self::ReplayRestoreVerificationMissing => "replay_restore_verification_missing",
            Self::ReplayRestoreVerificationRejected => "replay_restore_verification_rejected",
            Self::ReplayStatusMismatch => "replay_status_mismatch",
            Self::RestoreHistoryMismatch => "restore_history_mismatch",
            Self::ProviderPipelineBlocked => "provider_pipeline_blocked",
            Self::ProviderOutputValidationRejected => "provider_output_validation_rejected",
            Self::WorkflowBlocked => "workflow_blocked",
            Self::WorkflowRejected => "workflow_rejected",
            Self::MaterializationMissing => "materialization_missing",
            Self::AuthorityClaimDetected => "authority_claim_detected",
            Self::ReadinessClaimDetected => "readiness_claim_detected",
            Self::ReleaseOrDeploymentClaimDetected => "release_or_deployment_claim_detected",
            Self::PublicOrProductionUseClaimDetected => "public_or_production_use_claim_detected",
            Self::ActionAuthorizationClaimDetected => "action_authorization_claim_detected",
            Self::ReplayRepairOrRecoveryPromotionClaimDetected => {
                "replay_repair_or_recovery_promotion_claim_detected"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialErrorSeverity {
    Info,
    Warning,
    Blocking,
    StopCondition,
    InvalidInput,
}

impl TrialErrorSeverity {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Blocking => "blocking",
            Self::StopCondition => "stop_condition",
            Self::InvalidInput => "invalid_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialErrorSource {
    ControlledTrialExecutionHarness,
    ControlledInternalTrialPackage,
    TrialSessionEvidence,
    ReplayRestoreVerification,
    TrialRunbook,
    FailureDrill,
    CompleteLocalWorkflow,
    ProviderOutputPipeline,
    LocalCandidateMaterialization,
    SessionRestore,
}

impl TrialErrorSource {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ControlledTrialExecutionHarness => "controlled_trial_execution_harness",
            Self::ControlledInternalTrialPackage => "controlled_internal_trial_package",
            Self::TrialSessionEvidence => "trial_session_evidence",
            Self::ReplayRestoreVerification => "replay_restore_verification",
            Self::TrialRunbook => "trial_runbook",
            Self::FailureDrill => "failure_drill",
            Self::CompleteLocalWorkflow => "complete_local_workflow",
            Self::ProviderOutputPipeline => "provider_output_pipeline",
            Self::LocalCandidateMaterialization => "local_candidate_materialization",
            Self::SessionRestore => "session_restore",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialObservabilityBoundaryStatus {
    LocalObservabilityOnly,
    NonPublicObservability,
    NoProductionMonitoring,
    NoRemoteTelemetry,
    NoNetworkReporting,
    NoBackgroundService,
    NoAutomatedRemediation,
    NoAutomatedEscalation,
    NoStopConditionAutomation,
    NoActionExecution,
    NoReplayRepair,
    NoRecoveryPromotion,
    NoReadinessApproval,
    NoReleaseApproval,
    NoDeploymentApproval,
    NoPublicUseApproval,
    NoProductionApproval,
}

impl TrialObservabilityBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalObservabilityOnly => "local_observability_only",
            Self::NonPublicObservability => "non_public_observability",
            Self::NoProductionMonitoring => "no_production_monitoring",
            Self::NoRemoteTelemetry => "no_remote_telemetry",
            Self::NoNetworkReporting => "no_network_reporting",
            Self::NoBackgroundService => "no_background_service",
            Self::NoAutomatedRemediation => "no_automated_remediation",
            Self::NoAutomatedEscalation => "no_automated_escalation",
            Self::NoStopConditionAutomation => "no_stop_condition_automation",
            Self::NoActionExecution => "no_action_execution",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
            Self::NoReadinessApproval => "no_readiness_approval",
            Self::NoReleaseApproval => "no_release_approval",
            Self::NoDeploymentApproval => "no_deployment_approval",
            Self::NoPublicUseApproval => "no_public_use_approval",
            Self::NoProductionApproval => "no_production_approval",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialObservabilityCapabilitySurface {
    pub local_only: bool,
    pub non_public: bool,
    pub production_monitoring: bool,
    pub remote_telemetry: bool,
    pub network_reporting: bool,
    pub background_service: bool,
    pub remediates: bool,
    pub escalates: bool,
    pub automates_stop_conditions: bool,
    pub executes_actions: bool,
    pub repairs_replay: bool,
    pub promotes_recovery: bool,
    pub approves_readiness: bool,
    pub approves_release: bool,
    pub approves_deployment: bool,
    pub approves_public_use: bool,
    pub approves_production: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialBlockedStateSummary {
    pub status: String,
    pub current_blocker: String,
    pub rejection_reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialVerificationMismatchSummary {
    pub verification_status: String,
    pub mismatches: Vec<String>,
    pub replay_status_comparison: String,
    pub restore_history_comparison: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialStopConditionObservationSummary {
    pub status: String,
    pub observed: bool,
    pub markers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialErrorEvidenceLinkage {
    pub source: TrialErrorSource,
    pub linkage: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialErrorDetail {
    pub category: TrialErrorCategory,
    pub severity: TrialErrorSeverity,
    pub source: TrialErrorSource,
    pub summary: String,
    pub operator_guidance: String,
    pub evidence_linkage: TrialErrorEvidenceLinkage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialErrorReportProjection {
    pub status: TrialErrorReportStatus,
    pub error_count: usize,
    pub highest_severity: TrialErrorSeverity,
    pub details: Vec<TrialErrorDetail>,
    pub category_summary: Vec<String>,
    pub evidence_linkage_summary: Vec<String>,
    pub local_descriptive_only_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialObservabilityProjection {
    pub status: TrialObservabilityStatus,
    pub trial_run_status: String,
    pub current_trial_step: String,
    pub current_blocker: String,
    pub stop_condition_observation: TrialStopConditionObservationSummary,
    pub manual_operator_step_status: String,
    pub package_status: String,
    pub evidence_status: String,
    pub package_read_back_status: String,
    pub evidence_read_back_status: String,
    pub replay_restore_verification_status: String,
    pub mismatch_summary: TrialVerificationMismatchSummary,
    pub package_evidence_read_back_failure_summary: Vec<String>,
    pub replay_status_comparison_summary: String,
    pub restore_history_comparison_summary: String,
    pub runbook_failure_drill_category_summary: Vec<String>,
    pub evidence_linkage_summary: Vec<String>,
    pub blocked_state_summary: TrialBlockedStateSummary,
    pub boundary_statuses: Vec<TrialObservabilityBoundaryStatus>,
    pub capability_surface: TrialObservabilityCapabilitySurface,
    pub local_only_non_public_note: String,
    pub no_production_monitoring_note: String,
    pub no_remote_telemetry_note: String,
    pub no_background_service_note: String,
    pub no_remediation_escalation_stop_automation_note: String,
    pub no_approval_note: String,
}

pub fn trial_observability_boundary_statuses() -> Vec<TrialObservabilityBoundaryStatus> {
    vec![
        TrialObservabilityBoundaryStatus::LocalObservabilityOnly,
        TrialObservabilityBoundaryStatus::NonPublicObservability,
        TrialObservabilityBoundaryStatus::NoProductionMonitoring,
        TrialObservabilityBoundaryStatus::NoRemoteTelemetry,
        TrialObservabilityBoundaryStatus::NoNetworkReporting,
        TrialObservabilityBoundaryStatus::NoBackgroundService,
        TrialObservabilityBoundaryStatus::NoAutomatedRemediation,
        TrialObservabilityBoundaryStatus::NoAutomatedEscalation,
        TrialObservabilityBoundaryStatus::NoStopConditionAutomation,
        TrialObservabilityBoundaryStatus::NoActionExecution,
        TrialObservabilityBoundaryStatus::NoReplayRepair,
        TrialObservabilityBoundaryStatus::NoRecoveryPromotion,
        TrialObservabilityBoundaryStatus::NoReadinessApproval,
        TrialObservabilityBoundaryStatus::NoReleaseApproval,
        TrialObservabilityBoundaryStatus::NoDeploymentApproval,
        TrialObservabilityBoundaryStatus::NoPublicUseApproval,
        TrialObservabilityBoundaryStatus::NoProductionApproval,
    ]
}

pub fn trial_observability_capability_surface() -> TrialObservabilityCapabilitySurface {
    TrialObservabilityCapabilitySurface {
        local_only: true,
        non_public: true,
        production_monitoring: false,
        remote_telemetry: false,
        network_reporting: false,
        background_service: false,
        remediates: false,
        escalates: false,
        automates_stop_conditions: false,
        executes_actions: false,
        repairs_replay: false,
        promotes_recovery: false,
        approves_readiness: false,
        approves_release: false,
        approves_deployment: false,
        approves_public_use: false,
        approves_production: false,
    }
}

fn initial_trial_error_report_projection() -> TrialErrorReportProjection {
    TrialErrorReportProjection {
        status: TrialErrorReportStatus::ErrorsObserved,
        error_count: 1,
        highest_severity: TrialErrorSeverity::Warning,
        details: vec![TrialErrorDetail {
            category: TrialErrorCategory::NoTrialRun,
            severity: TrialErrorSeverity::Warning,
            source: TrialErrorSource::ControlledTrialExecutionHarness,
            summary: "No controlled internal trial run has been observed.".to_string(),
            operator_guidance:
                "Inspect local trial preconditions before starting a bounded local trial run."
                    .to_string(),
            evidence_linkage: TrialErrorEvidenceLinkage {
                source: TrialErrorSource::ControlledTrialExecutionHarness,
                linkage: "controlled_internal_trial_execution=not_started".to_string(),
            },
        }],
        category_summary: vec!["no_trial_run".to_string()],
        evidence_linkage_summary: vec![
            "controlled_internal_trial_execution=not_started".to_string()
        ],
        local_descriptive_only_note: "Error reporting is local and descriptive only.".to_string(),
    }
}

pub fn initial_trial_observability_projection() -> TrialObservabilityProjection {
    TrialObservabilityProjection {
        status: TrialObservabilityStatus::NotObserved,
        trial_run_status: "not_started".to_string(),
        current_trial_step: "none".to_string(),
        current_blocker: "none".to_string(),
        stop_condition_observation: TrialStopConditionObservationSummary {
            status: "not_started".to_string(),
            observed: false,
            markers: Vec::new(),
        },
        manual_operator_step_status: "not_started".to_string(),
        package_status: "not_packaged".to_string(),
        evidence_status: "not_captured".to_string(),
        package_read_back_status: "not_read".to_string(),
        evidence_read_back_status: "not_read".to_string(),
        replay_restore_verification_status: "not_verified".to_string(),
        mismatch_summary: TrialVerificationMismatchSummary {
            verification_status: "not_verified".to_string(),
            mismatches: Vec::new(),
            replay_status_comparison: "not_verified".to_string(),
            restore_history_comparison: "not_verified".to_string(),
        },
        package_evidence_read_back_failure_summary: Vec::new(),
        replay_status_comparison_summary: "not_verified".to_string(),
        restore_history_comparison_summary: "not_verified".to_string(),
        runbook_failure_drill_category_summary: Vec::new(),
        evidence_linkage_summary: Vec::new(),
        blocked_state_summary: TrialBlockedStateSummary {
            status: "not_observed".to_string(),
            current_blocker: "none".to_string(),
            rejection_reasons: Vec::new(),
        },
        boundary_statuses: trial_observability_boundary_statuses(),
        capability_surface: trial_observability_capability_surface(),
        local_only_non_public_note: "Trial observability is local-only and non-public.".to_string(),
        no_production_monitoring_note: "No production monitoring is active.".to_string(),
        no_remote_telemetry_note: "No remote telemetry is sent.".to_string(),
        no_background_service_note: "No background service is active.".to_string(),
        no_remediation_escalation_stop_automation_note: "No remediation, escalation, or stop-condition enforcement is automated.".to_string(),
        no_approval_note: "Observability does not approve controlled human use, readiness, release, deployment, public use, or production use.".to_string(),
    }
}

fn push_trial_error_detail(
    details: &mut Vec<TrialErrorDetail>,
    category: TrialErrorCategory,
    source: TrialErrorSource,
    summary: impl Into<String>,
    linkage: impl Into<String>,
) {
    let severity = classify_trial_error_severity(category);
    details.push(TrialErrorDetail {
        category,
        severity,
        source,
        summary: summary.into(),
        operator_guidance: trial_error_guidance(category).to_string(),
        evidence_linkage: TrialErrorEvidenceLinkage {
            source,
            linkage: linkage.into(),
        },
    });
}

fn classify_trial_error_severity(category: TrialErrorCategory) -> TrialErrorSeverity {
    match category {
        TrialErrorCategory::StopConditionObserved => TrialErrorSeverity::StopCondition,
        TrialErrorCategory::AuthorityClaimDetected
        | TrialErrorCategory::ReadinessClaimDetected
        | TrialErrorCategory::ReleaseOrDeploymentClaimDetected
        | TrialErrorCategory::PublicOrProductionUseClaimDetected
        | TrialErrorCategory::ActionAuthorizationClaimDetected
        | TrialErrorCategory::ReplayRepairOrRecoveryPromotionClaimDetected => {
            TrialErrorSeverity::InvalidInput
        }
        TrialErrorCategory::NoTrialRun
        | TrialErrorCategory::ManualOperatorStepRequired
        | TrialErrorCategory::TrialPreconditionMissing => TrialErrorSeverity::Warning,
        _ => TrialErrorSeverity::Blocking,
    }
}

fn trial_error_guidance(category: TrialErrorCategory) -> &'static str {
    match category {
        TrialErrorCategory::NoTrialRun => {
            "Inspect local trial preconditions before starting a bounded local trial run."
        }
        TrialErrorCategory::ManualOperatorStepRequired => {
            "Record the manual operator step locally before closing the trial run."
        }
        TrialErrorCategory::StopConditionObserved => {
            "Review the observed stop-condition marker locally; no enforcement is automated."
        }
        _ => "Review the linked local trial surface and preserve the diagnostic evidence.",
    }
}

pub fn derive_trial_error_report_projection(
    state: &LocalOperatorShellState,
) -> TrialErrorReportProjection {
    let mut details = Vec::new();
    let execution = &state.controlled_internal_trial_execution;
    let displayed_run = execution
        .active_run
        .as_ref()
        .or(execution.last_rejected_run.as_ref());

    if displayed_run.is_none() {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::NoTrialRun,
            TrialErrorSource::ControlledTrialExecutionHarness,
            "No controlled internal trial run has been observed.",
            "controlled_internal_trial_execution=not_started",
        );
    }
    if execution.status == ControlledInternalTrialRunStatus::PreconditionsMissing {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::TrialPreconditionMissing,
            TrialErrorSource::ControlledTrialExecutionHarness,
            "Trial preconditions are missing.",
            "controlled_internal_trial_execution=preconditions_missing",
        );
    }
    if execution.status == ControlledInternalTrialRunStatus::TrialRunBlocked {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::TrialRunBlocked,
            TrialErrorSource::ControlledTrialExecutionHarness,
            "Controlled internal trial run is blocked.",
            "controlled_internal_trial_execution=trial_run_blocked",
        );
    }
    if matches!(
        execution.status,
        ControlledInternalTrialRunStatus::TrialRunRejected
            | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
    ) {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::TrialRunRejected,
            TrialErrorSource::ControlledTrialExecutionHarness,
            "Controlled internal trial run is rejected.",
            "controlled_internal_trial_execution=trial_run_rejected",
        );
    }
    if let Some(run) = displayed_run {
        if run.stop_condition_observation.observed {
            push_trial_error_detail(
                &mut details,
                TrialErrorCategory::StopConditionObserved,
                TrialErrorSource::ControlledTrialExecutionHarness,
                "Stop condition was observed in the local trial run.",
                "stop_condition_observation=observed",
            );
        }
        if run.manual_operator_step_status
            == ControlledInternalTrialManualStepStatus::ManualActionRequired
            || run.manual_operator_step_status == ControlledInternalTrialManualStepStatus::Missing
        {
            push_trial_error_detail(
                &mut details,
                TrialErrorCategory::ManualOperatorStepRequired,
                TrialErrorSource::ControlledTrialExecutionHarness,
                "Manual operator step is required or missing.",
                "manual_operator_step_status=required",
            );
        }
    }

    if state.controlled_internal_trial_package_projection.status
        == ControlledInternalTrialPackageStatus::NotPackaged
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::PackageMissing,
            TrialErrorSource::ControlledInternalTrialPackage,
            "Controlled internal trial package is missing.",
            "trial_package=missing",
        );
    }
    if state
        .controlled_internal_trial_package_projection
        .validation_status
        != ControlledInternalTrialPackageValidationStatus::Valid
        && state.controlled_internal_trial_package_projection.status
            != ControlledInternalTrialPackageStatus::NotPackaged
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::PackageValidationFailed,
            TrialErrorSource::ControlledInternalTrialPackage,
            "Controlled internal trial package validation failed.",
            "trial_package_validation=failed",
        );
    }
    if matches!(state.controlled_internal_trial_package_projection.read_back_validation_status, Some(status) if status != ControlledInternalTrialPackageValidationStatus::Valid)
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::PackageReadBackFailed,
            TrialErrorSource::ControlledInternalTrialPackage,
            "Controlled internal trial package read-back failed.",
            "trial_package_read_back=failed",
        );
    }
    if state.trial_session_evidence_projection.status == TrialSessionEvidenceStatus::NotCaptured {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::EvidenceMissing,
            TrialErrorSource::TrialSessionEvidence,
            "Trial session evidence is missing.",
            "trial_session_evidence=missing",
        );
    }
    if state.trial_session_evidence_projection.validation_status
        != TrialSessionEvidenceValidationStatus::Valid
        && state.trial_session_evidence_projection.status != TrialSessionEvidenceStatus::NotCaptured
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::EvidenceValidationFailed,
            TrialErrorSource::TrialSessionEvidence,
            "Trial session evidence validation failed.",
            "trial_session_evidence_validation=failed",
        );
    }
    if matches!(state.trial_session_evidence_projection.read_back_validation_status, Some(status) if status != TrialSessionEvidenceValidationStatus::Valid)
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::EvidenceReadBackFailed,
            TrialErrorSource::TrialSessionEvidence,
            "Trial session evidence read-back failed.",
            "trial_session_evidence_read_back=failed",
        );
    }

    match state.trial_replay_restore_verification.status {
        TrialReplayRestoreVerificationStatus::NotVerified
        | TrialReplayRestoreVerificationStatus::VerificationInputMissing => {
            push_trial_error_detail(
                &mut details,
                TrialErrorCategory::ReplayRestoreVerificationMissing,
                TrialErrorSource::ReplayRestoreVerification,
                "Replay/restore verification is missing.",
                "replay_restore_verification=missing",
            )
        }
        TrialReplayRestoreVerificationStatus::VerificationRejected
        | TrialReplayRestoreVerificationStatus::InvalidVerificationInput => {
            push_trial_error_detail(
                &mut details,
                TrialErrorCategory::ReplayRestoreVerificationRejected,
                TrialErrorSource::ReplayRestoreVerification,
                "Replay/restore verification is rejected.",
                "replay_restore_verification=rejected",
            )
        }
        _ => {}
    }
    if state
        .trial_replay_restore_verification
        .mismatches
        .contains(&TrialReplayRestoreVerificationMismatch::ReplayStatusSnapshotMismatch)
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::ReplayStatusMismatch,
            TrialErrorSource::ReplayRestoreVerification,
            "Replay/status comparison mismatch observed.",
            "replay_status_comparison=mismatch",
        );
    }
    if state
        .trial_replay_restore_verification
        .mismatches
        .contains(&TrialReplayRestoreVerificationMismatch::RestoreHistorySnapshotMismatch)
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::RestoreHistoryMismatch,
            TrialErrorSource::ReplayRestoreVerification,
            "Restore/history comparison mismatch observed.",
            "restore_history_comparison=mismatch",
        );
    }
    if state.local_provider_output_pipeline.status
        == LocalProviderOutputPipelineValidationStatus::Rejected
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::ProviderPipelineBlocked,
            TrialErrorSource::ProviderOutputPipeline,
            "Provider output pipeline is blocked or rejected.",
            "provider_output_pipeline=rejected",
        );
    }
    if state.provider_output_validation.status == LocalProviderOutputValidationStatus::Rejected {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::ProviderOutputValidationRejected,
            TrialErrorSource::ProviderOutputPipeline,
            "Provider output validation is rejected.",
            "provider_output_validation=rejected",
        );
    }
    if state.complete_local_operator_workflow.status == CompleteLocalOperatorWorkflowStatus::Blocked
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::WorkflowBlocked,
            TrialErrorSource::CompleteLocalWorkflow,
            "Complete local workflow is blocked.",
            "complete_local_workflow=blocked",
        );
    }
    if state.complete_local_operator_workflow.status
        == CompleteLocalOperatorWorkflowStatus::Rejected
    {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::WorkflowRejected,
            TrialErrorSource::CompleteLocalWorkflow,
            "Complete local workflow is rejected.",
            "complete_local_workflow=rejected",
        );
    }
    if state.local_candidate_output.status == LocalCandidateMaterializationStatus::NotMaterialized {
        push_trial_error_detail(
            &mut details,
            TrialErrorCategory::MaterializationMissing,
            TrialErrorSource::LocalCandidateMaterialization,
            "Local candidate materialization is missing.",
            "local_candidate_materialization=missing",
        );
    }

    for reason in &execution.rejection_reasons {
        match reason {
            ControlledInternalTrialRunError::ReadinessClaimRejected => push_trial_error_detail(
                &mut details,
                TrialErrorCategory::ReadinessClaimDetected,
                TrialErrorSource::ControlledTrialExecutionHarness,
                "Readiness claim detected and rejected.",
                "claim=readiness",
            ),
            ControlledInternalTrialRunError::ReleaseClaimRejected
            | ControlledInternalTrialRunError::DeploymentClaimRejected
            | ControlledInternalTrialRunError::SigningClaimRejected
            | ControlledInternalTrialRunError::PublishingClaimRejected
            | ControlledInternalTrialRunError::ReleaseArtifactClaimRejected => {
                push_trial_error_detail(
                    &mut details,
                    TrialErrorCategory::ReleaseOrDeploymentClaimDetected,
                    TrialErrorSource::ControlledTrialExecutionHarness,
                    "Release or deployment claim detected and rejected.",
                    "claim=release_or_deployment",
                )
            }
            ControlledInternalTrialRunError::PublicUseClaimRejected
            | ControlledInternalTrialRunError::ProductionUseClaimRejected => {
                push_trial_error_detail(
                    &mut details,
                    TrialErrorCategory::PublicOrProductionUseClaimDetected,
                    TrialErrorSource::ControlledTrialExecutionHarness,
                    "Public or production use claim detected and rejected.",
                    "claim=public_or_production_use",
                )
            }
            ControlledInternalTrialRunError::ProviderTrustClaimRejected => push_trial_error_detail(
                &mut details,
                TrialErrorCategory::AuthorityClaimDetected,
                TrialErrorSource::ControlledTrialExecutionHarness,
                "Provider trust claim detected and rejected.",
                "claim=provider_trust",
            ),
            ControlledInternalTrialRunError::ActionAuthorizationClaimRejected => {
                push_trial_error_detail(
                    &mut details,
                    TrialErrorCategory::ActionAuthorizationClaimDetected,
                    TrialErrorSource::ControlledTrialExecutionHarness,
                    "Action authorization claim detected and rejected.",
                    "claim=action_authorization",
                )
            }
            ControlledInternalTrialRunError::ReplayRepairClaimRejected
            | ControlledInternalTrialRunError::RecoveryPromotionClaimRejected => {
                push_trial_error_detail(
                    &mut details,
                    TrialErrorCategory::ReplayRepairOrRecoveryPromotionClaimDetected,
                    TrialErrorSource::ControlledTrialExecutionHarness,
                    "Replay repair or recovery promotion claim detected and rejected.",
                    "claim=replay_repair_or_recovery_promotion",
                )
            }
            _ => {}
        }
    }

    details.sort_by_key(|detail| detail.category.code());
    let highest_severity = if details
        .iter()
        .any(|detail| detail.severity == TrialErrorSeverity::InvalidInput)
    {
        TrialErrorSeverity::InvalidInput
    } else if details
        .iter()
        .any(|detail| detail.severity == TrialErrorSeverity::StopCondition)
    {
        TrialErrorSeverity::StopCondition
    } else if details
        .iter()
        .any(|detail| detail.severity == TrialErrorSeverity::Blocking)
    {
        TrialErrorSeverity::Blocking
    } else if details
        .iter()
        .any(|detail| detail.severity == TrialErrorSeverity::Warning)
    {
        TrialErrorSeverity::Warning
    } else {
        TrialErrorSeverity::Info
    };
    let status = if details.is_empty() {
        TrialErrorReportStatus::NoErrorsObserved
    } else if execution.status == ControlledInternalTrialRunStatus::TrialRunBlocked {
        TrialErrorReportStatus::BlockedErrorsObserved
    } else if matches!(
        execution.status,
        ControlledInternalTrialRunStatus::TrialRunRejected
            | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
    ) {
        TrialErrorReportStatus::RejectedErrorsObserved
    } else if highest_severity == TrialErrorSeverity::InvalidInput {
        TrialErrorReportStatus::InvalidObservabilityInput
    } else {
        TrialErrorReportStatus::ErrorsObserved
    };
    TrialErrorReportProjection {
        status,
        error_count: details.len(),
        highest_severity,
        category_summary: details
            .iter()
            .map(|detail| detail.category.code().to_string())
            .collect(),
        evidence_linkage_summary: details
            .iter()
            .map(|detail| detail.evidence_linkage.linkage.clone())
            .collect(),
        details,
        local_descriptive_only_note: "Error reporting is local and descriptive only.".to_string(),
    }
}

pub fn derive_trial_observability_projection(
    state: &LocalOperatorShellState,
) -> TrialObservabilityProjection {
    let mut projection = initial_trial_observability_projection();
    let execution = &state.controlled_internal_trial_execution;
    let displayed_run = execution
        .active_run
        .as_ref()
        .or(execution.last_rejected_run.as_ref());
    projection.trial_run_status = displayed_run
        .map(|run| run.status.code())
        .unwrap_or_else(|| execution.status.code())
        .to_string();
    projection.current_trial_step = displayed_run
        .and_then(|run| run.current_step)
        .map(|step| step.code())
        .unwrap_or("none")
        .to_string();
    projection.current_blocker = displayed_run
        .and_then(|run| run.current_blocker)
        .or(execution.current_blocker)
        .map(|blocker| blocker.code())
        .unwrap_or("none")
        .to_string();
    projection.manual_operator_step_status = displayed_run
        .map(|run| run.manual_operator_step_status.code())
        .unwrap_or("not_started")
        .to_string();
    projection.package_status = state
        .controlled_internal_trial_package_projection
        .status
        .code()
        .to_string();
    projection.evidence_status = state
        .trial_session_evidence_projection
        .status
        .code()
        .to_string();
    projection.package_read_back_status = state
        .controlled_internal_trial_package_projection
        .read_back_validation_status
        .map(|status| status.code())
        .unwrap_or("not_read")
        .to_string();
    projection.evidence_read_back_status = state
        .trial_session_evidence_projection
        .read_back_validation_status
        .map(|status| status.code())
        .unwrap_or("not_read")
        .to_string();
    projection.replay_restore_verification_status = state
        .trial_replay_restore_verification
        .status
        .code()
        .to_string();
    if let Some(run) = displayed_run {
        projection.stop_condition_observation = TrialStopConditionObservationSummary {
            status: run.stop_condition_observation.status.clone(),
            observed: run.stop_condition_observation.observed,
            markers: run.stop_condition_observation.markers.clone(),
        };
        projection.evidence_linkage_summary = vec![
            run.evidence_linkage.trial_package.clone(),
            run.evidence_linkage.runbook.clone(),
            run.evidence_linkage.failure_drill.clone(),
            run.evidence_linkage.trial_session_evidence.clone(),
            run.evidence_linkage.replay_restore_verification.clone(),
            run.evidence_linkage.local_workflow.clone(),
        ];
    } else {
        projection.evidence_linkage_summary = vec![
            execution.evidence_linkage.trial_package.clone(),
            execution.evidence_linkage.runbook.clone(),
            execution.evidence_linkage.failure_drill.clone(),
            execution.evidence_linkage.trial_session_evidence.clone(),
            execution
                .evidence_linkage
                .replay_restore_verification
                .clone(),
            execution.evidence_linkage.local_workflow.clone(),
        ];
    }
    projection.mismatch_summary = TrialVerificationMismatchSummary {
        verification_status: state
            .trial_replay_restore_verification
            .status
            .code()
            .to_string(),
        mismatches: state
            .trial_replay_restore_verification
            .mismatches
            .iter()
            .map(|mismatch| mismatch.code().to_string())
            .collect(),
        replay_status_comparison: state
            .trial_replay_restore_verification
            .comparison_summary
            .replay_status_comparison
            .clone(),
        restore_history_comparison: state
            .trial_replay_restore_verification
            .comparison_summary
            .restore_history_comparison
            .clone(),
    };
    projection.package_evidence_read_back_failure_summary = state
        .trial_replay_restore_verification
        .mismatches
        .iter()
        .filter_map(|mismatch| match mismatch {
            TrialReplayRestoreVerificationMismatch::MissingTrialPackageReadBack
            | TrialReplayRestoreVerificationMismatch::TrialPackageReadBackInvalid
            | TrialReplayRestoreVerificationMismatch::MissingTrialSessionEvidenceReadBack
            | TrialReplayRestoreVerificationMismatch::TrialSessionEvidenceReadBackInvalid => {
                Some(mismatch.code().to_string())
            }
            _ => None,
        })
        .collect();
    projection.replay_status_comparison_summary = state
        .trial_replay_restore_verification
        .comparison_summary
        .replay_status_comparison
        .clone();
    projection.restore_history_comparison_summary = state
        .trial_replay_restore_verification
        .comparison_summary
        .restore_history_comparison
        .clone();
    projection.runbook_failure_drill_category_summary = state
        .trial_failure_drill
        .categories
        .iter()
        .map(|category| format!("{}={}", category.category.code(), category.severity.code()))
        .collect();
    projection.blocked_state_summary = TrialBlockedStateSummary {
        status: if matches!(
            execution.status,
            ControlledInternalTrialRunStatus::TrialRunBlocked
                | ControlledInternalTrialRunStatus::TrialRunRejected
                | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
        ) {
            "observed"
        } else {
            "not_observed"
        }
        .to_string(),
        current_blocker: projection.current_blocker.clone(),
        rejection_reasons: execution
            .rejection_reasons
            .iter()
            .map(|reason| reason.code().to_string())
            .collect(),
    };
    projection.status = if projection.stop_condition_observation.observed {
        TrialObservabilityStatus::StopConditionObserved
    } else if execution.status == ControlledInternalTrialRunStatus::TrialRunBlocked {
        TrialObservabilityStatus::BlockedStateObserved
    } else if matches!(
        execution.status,
        ControlledInternalTrialRunStatus::TrialRunRejected
            | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
    ) {
        TrialObservabilityStatus::RejectedStateObserved
    } else if !projection.mismatch_summary.mismatches.is_empty() {
        TrialObservabilityStatus::VerificationMismatchObserved
    } else if displayed_run.is_some() {
        TrialObservabilityStatus::TrialRunObserved
    } else {
        TrialObservabilityStatus::ObservabilityProjected
    };
    projection
}

pub fn refresh_trial_observability_state(
    mut state: LocalOperatorShellState,
) -> LocalOperatorShellState {
    state.trial_observability = derive_trial_observability_projection(&state);
    state.trial_error_report = derive_trial_error_report_projection(&state);
    state
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalOperatorShellState {
    pub harness_status: String,
    pub non_production: bool,
    pub run: LocalRunProjection,
    pub decision_ledger: LocalDecisionLedger,
    pub local_session_evidence_export: LocalSessionEvidenceExport,
    pub provider_configuration: LocalProviderConfiguration,
    pub local_provider_adapter_registry: LocalProviderAdapterRegistry,
    pub local_provider_adapter_dry_run: LocalProviderAdapterDryRunProjection,
    pub constrained_local_provider_invocation: ConstrainedLocalProviderInvocationProjection,
    pub local_provider_output_pipeline: LocalProviderOutputPipelineProjection,
    pub provider_execution: LocalProviderExecutionProjection,
    pub provider_output_validation: LocalProviderOutputValidationProjection,
    pub staged_candidate_conversion_proposal: StagedCandidateConversionProposalProjection,
    pub staged_candidate_conversion_validation: StagedCandidateConversionValidationProjection,
    pub operator_candidate_decision: OperatorCandidateDecisionProjection,
    pub local_candidate_output: LocalCandidateOutputProjection,
    pub phase_150_code_production_handoff: Phase150CodeProductionHandoff,
    pub local_session_package_projection: LocalSessionPackageProjection,
    pub local_session_history_projection: LocalSessionHistoryProjection,
    pub local_session_restore_projection: LocalSessionRestoreProjection,
    pub controlled_internal_trial_package_projection: ControlledInternalTrialPackageProjection,
    pub complete_local_operator_workflow: CompleteLocalOperatorWorkflowProjection,
    pub trial_operator_runbook: TrialOperatorRunbookProjection,
    pub trial_failure_drill: TrialFailureDrillProjection,
    pub trial_session_evidence_projection: TrialSessionEvidenceProjection,
    pub trial_replay_restore_verification: TrialReplayRestoreVerificationProjection,
    pub controlled_internal_trial_execution: ControlledInternalTrialExecutionProjection,
    pub trial_observability: TrialObservabilityProjection,
    pub trial_error_report: TrialErrorReportProjection,
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
    state.local_provider_output_pipeline = derive_local_provider_output_pipeline_projection(&state);
    state.local_session_evidence_export = project_local_session_evidence_export(&state);
    state.complete_local_operator_workflow =
        derive_complete_local_operator_workflow_projection(&state);
    state.trial_failure_drill = derive_trial_failure_drill_projection(&state);
    state.trial_operator_runbook = derive_trial_operator_runbook_projection(&state);
    state.controlled_internal_trial_execution =
        derive_controlled_internal_trial_execution_projection(&state);
    state
}

fn controlled_internal_trial_execution_evidence_linkage(
    state: &LocalOperatorShellState,
) -> ControlledInternalTrialEvidenceLinkage {
    ControlledInternalTrialEvidenceLinkage {
        trial_package: format!(
            "package={} status={} validation={}",
            state
                .controlled_internal_trial_package_projection
                .package_id
                .as_deref()
                .unwrap_or("none"),
            state
                .controlled_internal_trial_package_projection
                .status
                .code(),
            state
                .controlled_internal_trial_package_projection
                .validation_status
                .code()
        ),
        runbook: format!(
            "runbook_status={}",
            state.trial_operator_runbook.status.code()
        ),
        failure_drill: format!(
            "failure_drill_status={} highest_severity={}",
            state.trial_failure_drill.status.code(),
            state.trial_failure_drill.highest_severity.code()
        ),
        trial_session_evidence: format!(
            "evidence={} status={} validation={}",
            state
                .trial_session_evidence_projection
                .evidence_id
                .as_deref()
                .unwrap_or("none"),
            state.trial_session_evidence_projection.status.code(),
            state
                .trial_session_evidence_projection
                .validation_status
                .code()
        ),
        replay_restore_verification: format!(
            "verification={} status={}",
            state
                .trial_replay_restore_verification
                .verification_id
                .as_deref()
                .unwrap_or("none"),
            state.trial_replay_restore_verification.status.code()
        ),
        local_workflow: format!(
            "workflow_status={}",
            state.complete_local_operator_workflow.status.code()
        ),
    }
}

fn controlled_internal_trial_execution_claim_errors(
    request: &ControlledInternalTrialExecutionRequest,
) -> Vec<ControlledInternalTrialRunError> {
    let mut errors = Vec::new();
    if request.claims_readiness {
        errors.push(ControlledInternalTrialRunError::ReadinessClaimRejected);
    }
    if request.claims_release {
        errors.push(ControlledInternalTrialRunError::ReleaseClaimRejected);
    }
    if request.claims_deployment {
        errors.push(ControlledInternalTrialRunError::DeploymentClaimRejected);
    }
    if request.claims_public_use {
        errors.push(ControlledInternalTrialRunError::PublicUseClaimRejected);
    }
    if request.claims_production_use {
        errors.push(ControlledInternalTrialRunError::ProductionUseClaimRejected);
    }
    if request.claims_provider_trust {
        errors.push(ControlledInternalTrialRunError::ProviderTrustClaimRejected);
    }
    if request.claims_action_authorization {
        errors.push(ControlledInternalTrialRunError::ActionAuthorizationClaimRejected);
    }
    if request.claims_replay_repair {
        errors.push(ControlledInternalTrialRunError::ReplayRepairClaimRejected);
    }
    if request.claims_recovery_promotion {
        errors.push(ControlledInternalTrialRunError::RecoveryPromotionClaimRejected);
    }
    if request.claims_signing {
        errors.push(ControlledInternalTrialRunError::SigningClaimRejected);
    }
    if request.claims_publishing {
        errors.push(ControlledInternalTrialRunError::PublishingClaimRejected);
    }
    if request.claims_release_artifact {
        errors.push(ControlledInternalTrialRunError::ReleaseArtifactClaimRejected);
    }
    errors
}

pub fn validate_controlled_internal_trial_execution_preconditions(
    state: &LocalOperatorShellState,
) -> Vec<ControlledInternalTrialRunError> {
    let mut errors = Vec::new();
    let package = &state.controlled_internal_trial_package_projection;
    if package.status == ControlledInternalTrialPackageStatus::NotPackaged {
        errors.push(ControlledInternalTrialRunError::TrialPackageMissing);
    } else if trial_package_is_invalid(package)
        || package.validation_status != ControlledInternalTrialPackageValidationStatus::Valid
    {
        errors.push(ControlledInternalTrialRunError::TrialPackageInvalid);
    }

    match state.trial_operator_runbook.status {
        TrialOperatorRunbookStatus::NotAvailable => {
            errors.push(ControlledInternalTrialRunError::RunbookMissing)
        }
        TrialOperatorRunbookStatus::TrialPackageRequired | TrialOperatorRunbookStatus::Blocked => {
            errors.push(ControlledInternalTrialRunError::RunbookBlocked)
        }
        _ => {}
    }

    if state.trial_failure_drill.stop_condition_drills.is_empty()
        && state.trial_failure_drill.categories.is_empty()
    {
        errors.push(ControlledInternalTrialRunError::FailureDrillMissing);
    }

    match state.trial_session_evidence_projection.status {
        TrialSessionEvidenceStatus::NotCaptured => {
            errors.push(ControlledInternalTrialRunError::TrialSessionEvidenceMissing)
        }
        TrialSessionEvidenceStatus::EvidenceRejected
        | TrialSessionEvidenceStatus::InvalidEvidenceInput => {
            errors.push(ControlledInternalTrialRunError::TrialSessionEvidenceInvalid)
        }
        _ => {}
    }
    if state.trial_session_evidence_projection.validation_status
        != TrialSessionEvidenceValidationStatus::Valid
        && state
            .trial_session_evidence_projection
            .read_back_validation_status
            != Some(TrialSessionEvidenceValidationStatus::Valid)
        && !errors.contains(&ControlledInternalTrialRunError::TrialSessionEvidenceMissing)
    {
        errors.push(ControlledInternalTrialRunError::TrialSessionEvidenceInvalid);
    }

    match state.trial_replay_restore_verification.status {
        TrialReplayRestoreVerificationStatus::NotVerified => {
            errors.push(ControlledInternalTrialRunError::ReplayRestoreVerificationMissing)
        }
        TrialReplayRestoreVerificationStatus::VerificationRejected
        | TrialReplayRestoreVerificationStatus::InvalidVerificationInput => {
            errors.push(ControlledInternalTrialRunError::ReplayRestoreVerificationRejected)
        }
        TrialReplayRestoreVerificationStatus::VerificationInputMissing => {
            errors.push(ControlledInternalTrialRunError::ReplayRestoreVerificationMissing)
        }
        TrialReplayRestoreVerificationStatus::VerificationPassed => {}
        TrialReplayRestoreVerificationStatus::VerificationProjected => {
            errors.push(ControlledInternalTrialRunError::ReplayRestoreVerificationNotPassed)
        }
    }

    match state.complete_local_operator_workflow.status {
        CompleteLocalOperatorWorkflowStatus::NotStarted => {
            errors.push(ControlledInternalTrialRunError::CompleteLocalWorkflowMissing)
        }
        CompleteLocalOperatorWorkflowStatus::Blocked => {
            errors.push(ControlledInternalTrialRunError::CompleteLocalWorkflowBlocked)
        }
        CompleteLocalOperatorWorkflowStatus::Rejected => {
            errors.push(ControlledInternalTrialRunError::CompleteLocalWorkflowRejected)
        }
        CompleteLocalOperatorWorkflowStatus::InProgress => {
            errors.push(ControlledInternalTrialRunError::CompleteLocalWorkflowMissing)
        }
        CompleteLocalOperatorWorkflowStatus::CompleteLocalWorkflowProjected
        | CompleteLocalOperatorWorkflowStatus::LocalCandidateMaterialized => {}
    }
    errors
}

fn controlled_internal_trial_execution_precondition_status(
    state: &LocalOperatorShellState,
    errors: &[ControlledInternalTrialRunError],
) -> Vec<String> {
    vec![
        format!(
            "trial_package={}",
            state
                .controlled_internal_trial_package_projection
                .status
                .code()
        ),
        format!("runbook={}", state.trial_operator_runbook.status.code()),
        format!("failure_drill={}", state.trial_failure_drill.status.code()),
        format!(
            "trial_session_evidence={}",
            state.trial_session_evidence_projection.status.code()
        ),
        format!(
            "replay_restore_verification={}",
            state.trial_replay_restore_verification.status.code()
        ),
        format!(
            "local_workflow={}",
            state.complete_local_operator_workflow.status.code()
        ),
        format!(
            "precondition_errors={}",
            errors
                .iter()
                .map(|e| e.code())
                .collect::<Vec<_>>()
                .join(",")
        ),
    ]
}

fn controlled_internal_trial_step_order() -> Vec<ControlledInternalTrialRunStep> {
    vec![
        ControlledInternalTrialRunStep::VerifyTrialPackage,
        ControlledInternalTrialRunStep::VerifyRunbook,
        ControlledInternalTrialRunStep::VerifyFailureDrill,
        ControlledInternalTrialRunStep::VerifyTrialSessionEvidence,
        ControlledInternalTrialRunStep::VerifyReplayRestore,
        ControlledInternalTrialRunStep::VerifyCompleteLocalWorkflow,
        ControlledInternalTrialRunStep::ObserveStopConditions,
        ControlledInternalTrialRunStep::RecordManualOperatorStep,
        ControlledInternalTrialRunStep::ProjectTrialRunSummary,
        ControlledInternalTrialRunStep::ProjectTrialEvidenceLinkage,
        ControlledInternalTrialRunStep::CloseLocalTrialRun,
    ]
}

fn deterministic_controlled_internal_trial_run_id(
    state: &LocalOperatorShellState,
    request: &ControlledInternalTrialExecutionRequest,
) -> String {
    let basis = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        state
            .controlled_internal_trial_package_projection
            .package_id
            .as_deref()
            .unwrap_or("none"),
        state.trial_operator_runbook.status.code(),
        state
            .trial_session_evidence_projection
            .evidence_id
            .as_deref()
            .unwrap_or("none"),
        state
            .trial_replay_restore_verification
            .verification_id
            .as_deref()
            .unwrap_or("none"),
        state.complete_local_operator_workflow.status.code(),
        request.operator_step_recorded,
        request.stop_condition_observed,
        state.trial_failure_drill.highest_severity.code(),
    );
    format!(
        "controlled-internal-trial-run-{}",
        stable_trial_session_evidence_digest(&basis)
    )
}

pub fn derive_controlled_internal_trial_execution_projection(
    state: &LocalOperatorShellState,
) -> ControlledInternalTrialExecutionProjection {
    let mut projection = initial_controlled_internal_trial_execution_projection();
    let errors = validate_controlled_internal_trial_execution_preconditions(state);
    projection.status = if errors.is_empty() {
        ControlledInternalTrialRunStatus::ReadyForBoundedLocalTrialRun
    } else {
        ControlledInternalTrialRunStatus::PreconditionsMissing
    };
    projection.current_blocker = errors.first().copied();
    projection.rejection_reasons = errors.clone();
    projection.precondition_status =
        controlled_internal_trial_execution_precondition_status(state, &errors);
    projection.evidence_linkage = controlled_internal_trial_execution_evidence_linkage(state);
    projection
}

fn controlled_internal_trial_run_projection(
    state: &LocalOperatorShellState,
    request: &ControlledInternalTrialExecutionRequest,
    errors: Vec<ControlledInternalTrialRunError>,
) -> ControlledInternalTrialRunProjection {
    let blocked = request.stop_condition_observed || !errors.is_empty();
    let manual_status = if request.operator_step_recorded {
        ControlledInternalTrialManualStepStatus::Recorded
    } else if blocked {
        ControlledInternalTrialManualStepStatus::Missing
    } else {
        ControlledInternalTrialManualStepStatus::ManualActionRequired
    };
    let blocker = if request.stop_condition_observed {
        Some(ControlledInternalTrialRunError::StopConditionObserved)
    } else if !request.operator_step_recorded && errors.is_empty() {
        Some(ControlledInternalTrialRunError::ManualOperatorStepMissing)
    } else {
        errors.first().copied()
    };
    let status = if !errors.is_empty() {
        ControlledInternalTrialRunStatus::TrialRunRejected
    } else if request.stop_condition_observed {
        ControlledInternalTrialRunStatus::TrialRunBlocked
    } else if request.operator_step_recorded {
        ControlledInternalTrialRunStatus::TrialRunCompletedLocally
    } else {
        ControlledInternalTrialRunStatus::TrialRunInProgress
    };
    let steps = controlled_internal_trial_step_order()
        .into_iter()
        .map(|step| {
            let status = if !errors.is_empty() {
                ControlledInternalTrialRunStepStatus::Rejected
            } else if request.stop_condition_observed
                && matches!(
                    step,
                    ControlledInternalTrialRunStep::ObserveStopConditions
                        | ControlledInternalTrialRunStep::RecordManualOperatorStep
                        | ControlledInternalTrialRunStep::ProjectTrialRunSummary
                        | ControlledInternalTrialRunStep::ProjectTrialEvidenceLinkage
                        | ControlledInternalTrialRunStep::CloseLocalTrialRun
                )
            {
                ControlledInternalTrialRunStepStatus::Blocked
            } else if step == ControlledInternalTrialRunStep::RecordManualOperatorStep
                && !request.operator_step_recorded
            {
                ControlledInternalTrialRunStepStatus::ManualActionRequired
            } else if request.operator_step_recorded
                || step != ControlledInternalTrialRunStep::CloseLocalTrialRun
            {
                ControlledInternalTrialRunStepStatus::Completed
            } else {
                ControlledInternalTrialRunStepStatus::Available
            };
            ControlledInternalTrialRunStepProjection {
                step,
                status,
                summary: format!("{} => {}", step.code(), status.code()),
            }
        })
        .collect::<Vec<_>>();
    ControlledInternalTrialRunProjection {
        run_id: deterministic_controlled_internal_trial_run_id(state, request),
        status,
        current_step: steps
            .iter()
            .find(|step| step.status != ControlledInternalTrialRunStepStatus::Completed)
            .map(|step| step.step),
        next_step: steps
            .iter()
            .find(|step| {
                matches!(
                    step.status,
                    ControlledInternalTrialRunStepStatus::Available
                        | ControlledInternalTrialRunStepStatus::ManualActionRequired
                )
            })
            .map(|step| step.step),
        steps,
        current_blocker: blocker,
        rejection_reasons: errors,
        stop_condition_observation: ControlledInternalTrialStopObservation {
            status: if request.stop_condition_observed {
                "stop_condition_observed"
            } else {
                "no_stop_condition_observed"
            }
            .to_string(),
            observed: request.stop_condition_observed,
            markers: state
                .controlled_internal_trial_package_projection
                .stop_condition_summary
                .clone(),
            enforcement_automated: false,
        },
        manual_operator_step_status: manual_status,
        evidence_linkage: controlled_internal_trial_execution_evidence_linkage(state),
        summary: format!(
            "Bounded local controlled internal trial run status: {}.",
            status.code()
        ),
    }
}

pub fn start_controlled_internal_trial_execution(
    state: &LocalOperatorShellState,
    request: ControlledInternalTrialExecutionRequest,
) -> LocalOperatorShellState {
    let mut next = state.clone();
    let mut errors = validate_controlled_internal_trial_execution_preconditions(state);
    errors.extend(controlled_internal_trial_execution_claim_errors(&request));
    let run = controlled_internal_trial_run_projection(state, &request, errors.clone());
    let mut projection = derive_controlled_internal_trial_execution_projection(state);
    if errors.is_empty() && !request.stop_condition_observed {
        projection.status = run.status;
        projection.active_run = Some(run);
        projection.current_blocker = projection
            .active_run
            .as_ref()
            .and_then(|r| r.current_blocker);
    } else {
        projection.status = if controlled_internal_trial_execution_claim_errors(&request).is_empty()
        {
            run.status
        } else {
            ControlledInternalTrialRunStatus::InvalidTrialRunRequest
        };
        projection.active_run = state.controlled_internal_trial_execution.active_run.clone();
        projection.last_rejected_run = Some(run);
        projection.current_blocker = projection
            .last_rejected_run
            .as_ref()
            .and_then(|r| r.current_blocker);
        projection.rejection_reasons = errors;
    }
    next.controlled_internal_trial_execution = projection;
    refresh_trial_observability_state(next)
}

pub fn step_controlled_internal_trial_execution(
    state: &LocalOperatorShellState,
    mut request: ControlledInternalTrialExecutionRequest,
) -> LocalOperatorShellState {
    request.operator_step_recorded = true;
    start_controlled_internal_trial_execution(state, request)
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
    let mut state = LocalOperatorShellState {
        harness_status: "idle_local_harness".to_string(),
        non_production: true,
        run,
        decision_ledger: ledger,
        local_session_evidence_export: export,
        provider_configuration: initial_local_provider_configuration(),
        local_provider_adapter_registry: initial_local_provider_adapter_registry(),
        local_provider_adapter_dry_run: initial_local_provider_adapter_dry_run_projection(),
        constrained_local_provider_invocation:
            initial_constrained_local_provider_invocation_projection(),
        local_provider_output_pipeline: initial_local_provider_output_pipeline_projection(),
        provider_execution: initial_local_provider_execution_projection(),
        provider_output_validation: initial_local_provider_output_validation_projection(),
        staged_candidate_conversion_proposal:
            initial_staged_candidate_conversion_proposal_projection(),
        staged_candidate_conversion_validation:
            initial_staged_candidate_conversion_validation_projection(),
        operator_candidate_decision: initial_operator_candidate_decision_projection(),
        local_candidate_output: initial_local_candidate_output_projection(),
        phase_150_code_production_handoff: initial_phase_150_code_production_handoff(),
        local_session_package_projection: initial_local_session_package_projection(),
        local_session_history_projection: initial_local_session_history_projection(),
        local_session_restore_projection: initial_local_session_restore_projection(),
        controlled_internal_trial_package_projection:
            initial_controlled_internal_trial_package_projection(),
        complete_local_operator_workflow: initial_complete_local_operator_workflow_projection(),
        trial_operator_runbook: initial_trial_operator_runbook_projection(),
        trial_failure_drill: initial_trial_failure_drill_projection(),
        trial_session_evidence_projection: initial_trial_session_evidence_projection(),
        trial_replay_restore_verification: initial_trial_replay_restore_verification_projection(),
        controlled_internal_trial_execution: initial_controlled_internal_trial_execution_projection(
        ),
        trial_observability: initial_trial_observability_projection(),
        trial_error_report: initial_trial_error_report_projection(),
    };
    state.phase_150_code_production_handoff = derive_phase_150_code_production_handoff(&state);
    state.complete_local_operator_workflow =
        derive_complete_local_operator_workflow_projection(&state);
    state.trial_failure_drill = derive_trial_failure_drill_projection(&state);
    state.trial_operator_runbook = derive_trial_operator_runbook_projection(&state);
    refresh_trial_observability_state(state)
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
    SubmitProviderAdapterDeclaration(LocalProviderAdapterConfigurationCandidate),
    ExecuteProvider(LocalProviderExecutionRequest),
    RunProviderAdapterDryRun(LocalProviderAdapterDryRunRequest),
    InvokeConstrainedLocalProvider(ConstrainedLocalProviderInvocationRequest),
    CreateStagedCandidateConversionProposal(StagedCandidateConversionProposalRequest),
    ValidateStagedCandidateConversionProposal(StagedCandidateConversionValidationRequest),
    SubmitOperatorCandidateDecision(OperatorCandidateDecisionRequest),
    MaterializeLocalCandidateOutput(LocalCandidateMaterializationRequest),
    StartControlledInternalTrialExecution(ControlledInternalTrialExecutionRequest),
    StepControlledInternalTrialExecution(ControlledInternalTrialExecutionRequest),
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

pub fn submit_local_provider_adapter_declaration(
    transport: &mut LocalOperatorShellTransport,
    candidate: LocalProviderAdapterConfigurationCandidate,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::SubmitProviderAdapterDeclaration(
        candidate,
    ))
}

pub fn execute_local_provider(
    transport: &mut LocalOperatorShellTransport,
    request: LocalProviderExecutionRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::ExecuteProvider(request))
}

pub fn run_local_provider_adapter_dry_run(
    transport: &mut LocalOperatorShellTransport,
    request: LocalProviderAdapterDryRunRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::RunProviderAdapterDryRun(request))
}

pub fn invoke_constrained_local_provider(
    transport: &mut LocalOperatorShellTransport,
    request: ConstrainedLocalProviderInvocationRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::InvokeConstrainedLocalProvider(
        request,
    ))
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

pub fn submit_local_operator_candidate_decision(
    transport: &mut LocalOperatorShellTransport,
    request: OperatorCandidateDecisionRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::SubmitOperatorCandidateDecision(
        request,
    ))
}

pub fn request_local_candidate_materialization(
    transport: &mut LocalOperatorShellTransport,
    request: LocalCandidateMaterializationRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::MaterializeLocalCandidateOutput(
        request,
    ))
}

pub fn request_controlled_internal_trial_execution_start(
    transport: &mut LocalOperatorShellTransport,
    request: ControlledInternalTrialExecutionRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::StartControlledInternalTrialExecution(request))
}

pub fn request_controlled_internal_trial_execution_step(
    transport: &mut LocalOperatorShellTransport,
    request: ControlledInternalTrialExecutionRequest,
) -> LocalOperatorShellResponse {
    transport.step(LocalOperatorShellRequest::StepControlledInternalTrialExecution(request))
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
        LocalOperatorShellRequest::SubmitProviderAdapterDeclaration(candidate) => {
            match apply_local_provider_adapter_declaration(state, candidate) {
                Ok(next) => accepted("local_provider_adapter_declaration_accepted", next),
                Err(validation) => rejected(validation.reason, state.clone()),
            }
        }
        LocalOperatorShellRequest::ExecuteProvider(request) => {
            match apply_local_provider_execution(state, request) {
                Ok(next) => accepted("local_provider_execution_accepted", next),
                Err(validation) => rejected(validation.reason, state.clone()),
            }
        }
        LocalOperatorShellRequest::RunProviderAdapterDryRun(request) => {
            match apply_local_provider_adapter_dry_run(state, request) {
                Ok(next) => accepted("local_provider_adapter_dry_run_executed", next),
                Err(projection) => {
                    let response_state = if state.local_provider_adapter_dry_run.status
                        == LocalProviderAdapterDryRunStatus::DryRunExecuted
                    {
                        state.clone()
                    } else {
                        let mut rejected_state = state.clone();
                        rejected_state.local_provider_adapter_dry_run = *projection;
                        rejected_state
                    };
                    rejected("local_provider_adapter_dry_run_rejected", response_state)
                }
            }
        }
        LocalOperatorShellRequest::InvokeConstrainedLocalProvider(request) => {
            match execute_constrained_local_provider_invocation(state, request) {
                Ok(next) => accepted("constrained_local_provider_invocation_executed", next),
                Err(projection) => {
                    let response_state = if state.constrained_local_provider_invocation.status
                        == ConstrainedLocalProviderInvocationStatus::InvocationExecuted
                    {
                        state.clone()
                    } else {
                        let mut rejected_state = state.clone();
                        rejected_state.constrained_local_provider_invocation = *projection;
                        rejected_state.local_provider_output_pipeline =
                            derive_local_provider_output_pipeline_projection(&rejected_state);
                        rejected_state
                    };
                    rejected(
                        "constrained_local_provider_invocation_rejected",
                        response_state,
                    )
                }
            }
        }
        LocalOperatorShellRequest::CreateStagedCandidateConversionProposal(request) => {
            match create_staged_candidate_conversion_proposal(state, request) {
                Ok(next) => accepted("staged_candidate_conversion_proposal_created", next),
                Err(error) => rejected(error.code(), state.clone()),
            }
        }
        LocalOperatorShellRequest::ValidateStagedCandidateConversionProposal(request) => {
            let mut next =
                validate_staged_candidate_conversion_proposal_for_phase_147(state, request);
            next.phase_150_code_production_handoff =
                derive_phase_150_code_production_handoff(&next);
            if next.staged_candidate_conversion_validation.status
                == StagedCandidateConversionValidationStatus::StagedProposalShapeValid
            {
                accepted("staged_candidate_conversion_validation_completed", next)
            } else {
                rejected("staged_candidate_conversion_validation_rejected", next)
            }
        }
        LocalOperatorShellRequest::SubmitOperatorCandidateDecision(request) => {
            match submit_operator_candidate_decision(state, request) {
                Ok(next) => accepted("operator_candidate_decision_recorded", next),
                Err(error) => {
                    let mut response_state = state.clone();
                    response_state.operator_candidate_decision =
                        rejected_operator_candidate_decision_projection(error);
                    response_state.phase_150_code_production_handoff =
                        derive_phase_150_code_production_handoff(&response_state);
                    rejected(error.code(), response_state)
                }
            }
        }
        LocalOperatorShellRequest::MaterializeLocalCandidateOutput(request) => {
            match materialize_local_candidate_output(state, request) {
                Ok(next) => accepted("local_candidate_materialized", next),
                Err(error) => {
                    let mut response_state = state.clone();
                    response_state.local_candidate_output = reject_local_candidate_materialization(
                        &state.local_candidate_output,
                        error,
                    );
                    rejected(error.code(), response_state)
                }
            }
        }
        LocalOperatorShellRequest::StartControlledInternalTrialExecution(request) => {
            let next = start_controlled_internal_trial_execution(state, request);
            if matches!(
                next.controlled_internal_trial_execution.status,
                ControlledInternalTrialRunStatus::TrialRunRejected
                    | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
                    | ControlledInternalTrialRunStatus::TrialRunBlocked
            ) {
                rejected("controlled_internal_trial_execution_rejected", next)
            } else {
                accepted("controlled_internal_trial_execution_started", next)
            }
        }
        LocalOperatorShellRequest::StepControlledInternalTrialExecution(request) => {
            let next = step_controlled_internal_trial_execution(state, request);
            if matches!(
                next.controlled_internal_trial_execution.status,
                ControlledInternalTrialRunStatus::TrialRunRejected
                    | ControlledInternalTrialRunStatus::InvalidTrialRunRequest
                    | ControlledInternalTrialRunStatus::TrialRunBlocked
            ) {
                rejected("controlled_internal_trial_execution_step_rejected", next)
            } else {
                accepted("controlled_internal_trial_execution_stepped", next)
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
        state: attach_local_session_evidence_export(state),
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
        state: attach_local_session_evidence_export(state),
        capabilities: LocalOperatorShellCapabilities::local_stub_only(),
    }
}

#[cfg(test)]
mod tests {
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
        let package =
            derive_local_session_package(&phase_151_package_state_with_available_sections());
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
            (" claim:production_ready ", LocalSessionPackageValidationError::ForbiddenReleaseReadinessOrDeploymentClaim),
            (" claim:signing_enabled ", LocalSessionPackageValidationError::ForbiddenSigningPublishingInstallerOrUpdateClaim),
            (" claim:provider_trusted ", LocalSessionPackageValidationError::ForbiddenProviderTrustClaim),
            (" claim:candidate_approved ", LocalSessionPackageValidationError::ForbiddenCandidateApprovalClaim),
            (" claim:action_executed ", LocalSessionPackageValidationError::ForbiddenActionExecutionClaim),
            (" claim:durable persistence authority ", LocalSessionPackageValidationError::ForbiddenPersistenceAuthorityClaim),
        ] {
            let mut claimed = package.clone();
            claimed.payload.provider_configuration_projection.push_str(claim);
            assert!(validate_local_session_package(&claimed).unwrap_err().contains(&expected));
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
    fn phase_153_initial_adapter_registry_projection_is_deterministic() {
        let state = initial_local_operator_shell_state();
        let first = project_local_provider_adapter_registry(&state.local_provider_adapter_registry);
        let second =
            project_local_provider_adapter_registry(&state.local_provider_adapter_registry);

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
        rejected.constrained_local_provider_invocation =
            reject_constrained_local_provider_invocation(
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
        missing_decision.operator_candidate_decision =
            initial_operator_candidate_decision_projection();
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
                    provider_kind:
                        AllowlistedLocalProviderKind::AllowlistedLocalDeterministicProvider,
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
        rejected_decision.operator_candidate_decision =
            rejected_operator_candidate_decision_projection(
                OperatorCandidateDecisionError::PublicUseClaimRejected,
            );
        let decision_workflow =
            derive_complete_local_operator_workflow_projection(&rejected_decision);
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
            .any(|step| step.step
                == CompleteLocalOperatorWorkflowStepKind::ProviderOutputValidated
                && step.status == CompleteLocalOperatorWorkflowStepStatus::Completed));
    }

    fn phase_161_trial_scope() -> ControlledInternalTrialScope {
        ControlledInternalTrialScope {
            scope_id: "phase-161-internal-trial-scope".to_string(),
            scope_summary:
                "Controlled internal trial preparation from local beta workflow evidence"
                    .to_string(),
            local_beta_workflow_scope: "provider setup through restore/history projection"
                .to_string(),
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
            .contains(
                &ControlledInternalTrialPackageValidationError::InvalidPackageClassification
            ));

        let mut package = phase_161_trial_package();
        package.metadata.production_classification = "production".to_string();
        assert!(validate_controlled_internal_trial_package(&package)
            .unwrap_err()
            .contains(
                &ControlledInternalTrialPackageValidationError::InvalidProductionClassification
            ));

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
        state.trial_session_evidence_projection =
            validate_trial_session_evidence_read_back(&evidence);
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
        assert!(missing_evidence.mismatches.contains(
            &TrialReplayRestoreVerificationMismatch::MissingTrialSessionEvidenceReadBack
        ));

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
        assert!(rejected.mismatches.contains(
            &TrialReplayRestoreVerificationMismatch::TrialSessionEvidenceReadBackInvalid
        ));
    }

    #[test]
    fn phase_164_linkage_replay_restore_and_claim_mismatches_reject_in_order() {
        let (state, package, mut evidence) = phase_164_verification_inputs();
        evidence.payload.trial_package_id = "different-package".to_string();
        evidence.payload.trial_package_version = "different-version".to_string();
        evidence.payload.workflow_status_snapshot = "different-workflow".to_string();
        evidence.payload.provider_output_pipeline_snapshot =
            "different-provider-pipeline".to_string();
        evidence.payload.local_candidate_materialization_snapshot =
            "different-candidate".to_string();
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
        state.trial_session_evidence_projection =
            validate_trial_session_evidence_read_back(&evidence);
        state.trial_replay_restore_verification =
            derive_trial_replay_restore_verification_projection(
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

        let mut rejected_request =
            ControlledInternalTrialExecutionRequest::bounded_local_trial_run();
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
}
