#[path = "local_operator_shell_provider_pipeline.rs"]
mod local_operator_shell_provider_pipeline;
pub use local_operator_shell_provider_pipeline::*;
#[path = "local_operator_shell_candidate.rs"]
mod local_operator_shell_candidate;
pub use local_operator_shell_candidate::*;
#[path = "local_operator_shell_trial_verification.rs"]
mod local_operator_shell_trial_verification;
pub use local_operator_shell_trial_verification::*;
#[path = "local_operator_shell_trial_observability.rs"]
mod local_operator_shell_trial_observability;
pub use local_operator_shell_trial_observability::*;
#[path = "local_operator_shell_trial_review.rs"]
mod local_operator_shell_trial_review;
pub use local_operator_shell_trial_review::*;
#[path = "local_operator_shell_codecs.rs"]
mod local_operator_shell_codecs;
use local_operator_shell_codecs::{
    controlled_internal_trial_package_content_basis,
    stable_controlled_internal_trial_package_digest, stable_trial_session_evidence_digest,
    trial_session_evidence_payload_basis,
};
pub use local_operator_shell_codecs::{
    parse_controlled_internal_trial_package, parse_trial_session_evidence_record,
    read_controlled_internal_trial_package, serialize_controlled_internal_trial_package,
    serialize_trial_session_evidence_record, validate_controlled_internal_trial_package_read_back,
    validate_trial_session_evidence_read_back, write_controlled_internal_trial_package,
};
#[path = "local_operator_shell_transport.rs"]
mod local_operator_shell_transport;
pub use local_operator_shell_transport::*;

use std as trial_standard;

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
    pub trial_evidence_review: TrialEvidenceReviewProjection,
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
    refresh_trial_evidence_review_state(refresh_trial_observability_state(next))
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
        trial_evidence_review: initial_trial_evidence_review_projection(),
    };
    state.phase_150_code_production_handoff = derive_phase_150_code_production_handoff(&state);
    state.complete_local_operator_workflow =
        derive_complete_local_operator_workflow_projection(&state);
    state.trial_failure_drill = derive_trial_failure_drill_projection(&state);
    state.trial_operator_runbook = derive_trial_operator_runbook_projection(&state);
    refresh_trial_evidence_review_state(refresh_trial_observability_state(state))
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

#[cfg(test)]
#[path = "local_operator_shell_tests.rs"]
mod local_operator_shell_tests;
