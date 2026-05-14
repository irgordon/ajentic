//! Session package and restore projections for the local operator shell.

use super::local_operator_shell_boundary_markers::local_session_restore_boundary_statuses;
use super::*;

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

pub(crate) fn hex_encode(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len() * 2);
    for byte in input.as_bytes() {
        encoded.push_str(&format!("{byte:02x}"));
    }
    encoded
}

pub(crate) fn hex_decode(input: &str) -> Result<String, LocalSessionPackageValidationError> {
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
