use std::path::{Component, Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactManifestValidationStatus {
    AcceptedLocalNonPublic,
    Rejected,
    Deferred,
}

impl ArtifactManifestValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AcceptedLocalNonPublic => "accepted_local_non_public",
            Self::Rejected => "rejected",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalArtifactManifestValidationReason {
    MissingRequiredField,
    InvalidArtifactId,
    InvalidArtifactKind,
    UnsafeOutputPath,
    OutputPathNotLocalArtifacts,
    NonPublicNotTrue,
    ReleaseArtifactClaimPresent,
    ReadinessClaimPresent,
    PublishingClaimPresent,
    DeploymentClaimPresent,
    SigningClaimPresent,
    ChecksumClaimNotDeferred,
    ProvenanceClaimNotDeferred,
    InstallerOrUpdateChannelClaimPresent,
    InvalidCreatedByPhase,
    InvalidDeferredToPhase,
    MalformedManifest,
}

impl LocalArtifactManifestValidationReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingRequiredField => "missing_required_field",
            Self::InvalidArtifactId => "invalid_artifact_id",
            Self::InvalidArtifactKind => "invalid_artifact_kind",
            Self::UnsafeOutputPath => "unsafe_output_path",
            Self::OutputPathNotLocalArtifacts => "output_path_not_local_artifacts",
            Self::NonPublicNotTrue => "non_public_not_true",
            Self::ReleaseArtifactClaimPresent => "release_artifact_claim_present",
            Self::ReadinessClaimPresent => "readiness_claim_present",
            Self::PublishingClaimPresent => "publishing_claim_present",
            Self::DeploymentClaimPresent => "deployment_claim_present",
            Self::SigningClaimPresent => "signing_claim_present",
            Self::ChecksumClaimNotDeferred => "checksum_claim_not_deferred",
            Self::ProvenanceClaimNotDeferred => "provenance_claim_not_deferred",
            Self::InstallerOrUpdateChannelClaimPresent => {
                "installer_or_update_channel_claim_present"
            }
            Self::InvalidCreatedByPhase => "invalid_created_by_phase",
            Self::InvalidDeferredToPhase => "invalid_deferred_to_phase",
            Self::MalformedManifest => "malformed_manifest",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalArtifactKind {
    LocalRuntimeBuild,
    LocalUiBuild,
    LocalDocumentationBundle,
}

impl LocalArtifactKind {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalRuntimeBuild => "local_runtime_build",
            Self::LocalUiBuild => "local_ui_build",
            Self::LocalDocumentationBundle => "local_documentation_bundle",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalArtifactKindField {
    Known(LocalArtifactKind),
    Unsupported(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalArtifactEvidenceStatus {
    Absent,
    Deferred,
    ClaimedComplete,
}

impl LocalArtifactEvidenceStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Deferred => "deferred",
            Self::ClaimedComplete => "claimed_complete",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalArtifactManifestProducerStatus {
    Produced,
    ProducedAndValidated,
    Rejected,
    Deferred,
}

impl LocalArtifactManifestProducerStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Produced => "produced",
            Self::ProducedAndValidated => "produced_and_validated",
            Self::Rejected => "rejected",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocalArtifactManifestProducerReason {
    MissingArtifactId,
    MissingArtifactName,
    MissingArtifactKind,
    MissingSourceRevision,
    MissingBuildCommand,
    InvalidOutputPath,
    ValidationRejectedManifest,
    FilesystemWriteNotRequested,
    FilesystemWriteRejected,
    ManifestWriteDeferred,
}

impl LocalArtifactManifestProducerReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingArtifactId => "missing_artifact_id",
            Self::MissingArtifactName => "missing_artifact_name",
            Self::MissingArtifactKind => "missing_artifact_kind",
            Self::MissingSourceRevision => "missing_source_revision",
            Self::MissingBuildCommand => "missing_build_command",
            Self::InvalidOutputPath => "invalid_output_path",
            Self::ValidationRejectedManifest => "validation_rejected_manifest",
            Self::FilesystemWriteNotRequested => "filesystem_write_not_requested",
            Self::FilesystemWriteRejected => "filesystem_write_rejected",
            Self::ManifestWriteDeferred => "manifest_write_deferred",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalArtifactManifestProducerInput {
    pub artifact_id: Option<String>,
    pub artifact_name: Option<String>,
    pub artifact_kind: Option<LocalArtifactKind>,
    pub source_revision: Option<String>,
    pub build_command: Option<String>,
    pub output_path: Option<String>,
    pub deferred_to_phase: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalArtifactManifestProducerReport {
    pub status: LocalArtifactManifestProducerStatus,
    pub reasons: Vec<LocalArtifactManifestProducerReason>,
    pub manifest: Option<LocalArtifactManifest>,
    pub validation_report: Option<LocalArtifactManifestValidationReport>,
    pub output_path: Option<String>,
    pub mutates_filesystem: bool,
    pub creates_artifact: bool,
    pub creates_checksum: bool,
    pub creates_provenance: bool,
    pub signs_artifact: bool,
    pub publishes_artifact: bool,
    pub deploys_artifact: bool,
    pub readiness_granted: bool,
    pub summary: String,
}

pub fn produce_local_artifact_manifest_candidate(
    input: &LocalArtifactManifestProducerInput,
) -> LocalArtifactManifestProducerReport {
    let mut reasons = Vec::new();

    require_producer_input(input, &mut reasons);

    let output_path = input.output_path.clone().or_else(|| {
        input
            .artifact_id
            .as_ref()
            .map(|artifact_id| format!("artifacts/local/phase-132-3/{artifact_id}-manifest.json"))
    });

    if output_path
        .as_deref()
        .is_some_and(producer_output_path_is_invalid)
    {
        reasons.push(LocalArtifactManifestProducerReason::InvalidOutputPath);
    }

    if !reasons.is_empty() {
        reasons.sort();
        reasons.dedup();
        return rejected_producer_report(reasons, output_path);
    }

    let manifest = LocalArtifactManifest {
        artifact_id: input.artifact_id.clone(),
        artifact_name: input.artifact_name.clone(),
        artifact_kind: input.artifact_kind.map(LocalArtifactKindField::Known),
        source_revision: input.source_revision.clone(),
        build_command: input.build_command.clone(),
        output_path: output_path.clone(),
        created_by_phase: Some("phase_132_3".to_string()),
        non_public: Some(true),
        release_artifact_claim: Some(false),
        checksum_status: Some(LocalArtifactEvidenceStatus::Deferred),
        provenance_status: Some(LocalArtifactEvidenceStatus::Deferred),
        signing_status: Some(LocalArtifactEvidenceStatus::Deferred),
        publishing_status: Some(LocalArtifactEvidenceStatus::Absent),
        deployment_status: Some(LocalArtifactEvidenceStatus::Absent),
        readiness_claim: Some(false),
        deferred_to_phase: input
            .deferred_to_phase
            .clone()
            .or_else(|| Some("phase_133".to_string())),
        extra_fields: Vec::new(),
    };

    let validation_report = validate_local_artifact_manifest(&manifest);
    if validation_report.status != ArtifactManifestValidationStatus::AcceptedLocalNonPublic {
        return LocalArtifactManifestProducerReport {
            status: LocalArtifactManifestProducerStatus::Rejected,
            reasons: vec![LocalArtifactManifestProducerReason::ValidationRejectedManifest],
            manifest: Some(manifest),
            output_path,
            validation_report: Some(validation_report),
            mutates_filesystem: false,
            creates_artifact: false,
            creates_checksum: false,
            creates_provenance: false,
            signs_artifact: false,
            publishes_artifact: false,
            deploys_artifact: false,
            readiness_granted: false,
            summary: "local artifact manifest candidate rejected by validator".to_string(),
        };
    }

    LocalArtifactManifestProducerReport {
        status: LocalArtifactManifestProducerStatus::ProducedAndValidated,
        reasons: vec![LocalArtifactManifestProducerReason::FilesystemWriteNotRequested],
        manifest: Some(manifest),
        validation_report: Some(validation_report),
        output_path,
        mutates_filesystem: false,
        creates_artifact: false,
        creates_checksum: false,
        creates_provenance: false,
        signs_artifact: false,
        publishes_artifact: false,
        deploys_artifact: false,
        readiness_granted: false,
        summary: "local non-public artifact manifest candidate produced and validated without filesystem writes".to_string(),
    }
}

fn require_producer_input(
    input: &LocalArtifactManifestProducerInput,
    reasons: &mut Vec<LocalArtifactManifestProducerReason>,
) {
    if input.artifact_id.as_deref().is_none_or(str::is_empty) {
        reasons.push(LocalArtifactManifestProducerReason::MissingArtifactId);
    }
    if input.artifact_name.as_deref().is_none_or(str::is_empty) {
        reasons.push(LocalArtifactManifestProducerReason::MissingArtifactName);
    }
    if input.artifact_kind.is_none() {
        reasons.push(LocalArtifactManifestProducerReason::MissingArtifactKind);
    }
    if input.source_revision.as_deref().is_none_or(str::is_empty) {
        reasons.push(LocalArtifactManifestProducerReason::MissingSourceRevision);
    }
    if input.build_command.as_deref().is_none_or(str::is_empty) {
        reasons.push(LocalArtifactManifestProducerReason::MissingBuildCommand);
    }
}

fn producer_output_path_is_invalid(output_path: &str) -> bool {
    output_path.is_empty()
        || output_path.starts_with('~')
        || output_path.contains('\\')
        || Path::new(output_path).is_absolute()
        || has_traversal(output_path)
        || contains_forbidden_path_segment(output_path)
        || !output_path.starts_with("artifacts/local/")
        || output_path == "artifacts/local/"
}

fn rejected_producer_report(
    reasons: Vec<LocalArtifactManifestProducerReason>,
    output_path: Option<String>,
) -> LocalArtifactManifestProducerReport {
    LocalArtifactManifestProducerReport {
        status: LocalArtifactManifestProducerStatus::Rejected,
        reasons,
        manifest: None,
        validation_report: None,
        output_path,
        mutates_filesystem: false,
        creates_artifact: false,
        creates_checksum: false,
        creates_provenance: false,
        signs_artifact: false,
        publishes_artifact: false,
        deploys_artifact: false,
        readiness_granted: false,
        summary: "local artifact manifest candidate production rejected".to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalArtifactManifest {
    pub artifact_id: Option<String>,
    pub artifact_name: Option<String>,
    pub artifact_kind: Option<LocalArtifactKindField>,
    pub source_revision: Option<String>,
    pub build_command: Option<String>,
    pub output_path: Option<String>,
    pub created_by_phase: Option<String>,
    pub non_public: Option<bool>,
    pub release_artifact_claim: Option<bool>,
    pub checksum_status: Option<LocalArtifactEvidenceStatus>,
    pub provenance_status: Option<LocalArtifactEvidenceStatus>,
    pub signing_status: Option<LocalArtifactEvidenceStatus>,
    pub publishing_status: Option<LocalArtifactEvidenceStatus>,
    pub deployment_status: Option<LocalArtifactEvidenceStatus>,
    pub readiness_claim: Option<bool>,
    pub deferred_to_phase: Option<String>,
    pub extra_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalArtifactManifestValidationReport {
    pub status: ArtifactManifestValidationStatus,
    pub reasons: Vec<LocalArtifactManifestValidationReason>,
    pub artifact_id: Option<String>,
    pub output_path: Option<String>,
    pub non_public: bool,
    pub mutates_filesystem: bool,
    pub creates_artifact: bool,
    pub creates_checksum: bool,
    pub creates_provenance: bool,
    pub signs_artifact: bool,
    pub publishes_artifact: bool,
    pub deploys_artifact: bool,
    pub readiness_granted: bool,
    pub summary: String,
}

pub fn validate_local_artifact_manifest(
    manifest: &LocalArtifactManifest,
) -> LocalArtifactManifestValidationReport {
    let mut reasons = Vec::new();

    require_fields(manifest, &mut reasons);
    validate_artifact_id(manifest.artifact_id.as_deref(), &mut reasons);
    validate_artifact_kind(manifest.artifact_kind.as_ref(), &mut reasons);
    validate_output_path(manifest.output_path.as_deref(), &mut reasons);
    validate_phase_fields(manifest, &mut reasons);
    validate_non_public_boundary(manifest, &mut reasons);
    validate_claim_boundaries(manifest, &mut reasons);
    validate_extra_fields(&manifest.extra_fields, &mut reasons);

    reasons.sort();
    reasons.dedup();

    let status = if reasons.is_empty() {
        ArtifactManifestValidationStatus::AcceptedLocalNonPublic
    } else {
        ArtifactManifestValidationStatus::Rejected
    };

    LocalArtifactManifestValidationReport {
        status,
        reasons,
        artifact_id: manifest.artifact_id.clone(),
        output_path: manifest.output_path.clone(),
        non_public: manifest.non_public == Some(true),
        mutates_filesystem: false,
        creates_artifact: false,
        creates_checksum: false,
        creates_provenance: false,
        signs_artifact: false,
        publishes_artifact: false,
        deploys_artifact: false,
        readiness_granted: false,
        summary: match status {
            ArtifactManifestValidationStatus::AcceptedLocalNonPublic => {
                "local non-public artifact manifest evidence accepted".to_string()
            }
            ArtifactManifestValidationStatus::Rejected => {
                "local artifact manifest evidence rejected".to_string()
            }
            ArtifactManifestValidationStatus::Deferred => {
                "local artifact manifest evidence deferred".to_string()
            }
        },
    }
}

fn require_fields(
    manifest: &LocalArtifactManifest,
    reasons: &mut Vec<LocalArtifactManifestValidationReason>,
) {
    if manifest.artifact_id.is_none()
        || manifest.artifact_name.is_none()
        || manifest.artifact_kind.is_none()
        || manifest.source_revision.is_none()
        || manifest.build_command.is_none()
        || manifest.output_path.is_none()
        || manifest.created_by_phase.is_none()
        || manifest.non_public.is_none()
        || manifest.release_artifact_claim.is_none()
        || manifest.checksum_status.is_none()
        || manifest.provenance_status.is_none()
        || manifest.signing_status.is_none()
        || manifest.publishing_status.is_none()
        || manifest.deployment_status.is_none()
        || manifest.readiness_claim.is_none()
        || manifest.deferred_to_phase.is_none()
    {
        reasons.push(LocalArtifactManifestValidationReason::MissingRequiredField);
    }
}

fn validate_artifact_id(
    artifact_id: Option<&str>,
    reasons: &mut Vec<LocalArtifactManifestValidationReason>,
) {
    let Some(artifact_id) = artifact_id else {
        return;
    };

    if artifact_id.len() < 3
        || artifact_id.len() > 128
        || !artifact_id.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'-' | b'_' | b'.')
        })
        || contains_forbidden_public_term(artifact_id)
    {
        reasons.push(LocalArtifactManifestValidationReason::InvalidArtifactId);
    }
}

fn validate_artifact_kind(
    artifact_kind: Option<&LocalArtifactKindField>,
    reasons: &mut Vec<LocalArtifactManifestValidationReason>,
) {
    if matches!(artifact_kind, Some(LocalArtifactKindField::Unsupported(_))) {
        reasons.push(LocalArtifactManifestValidationReason::InvalidArtifactKind);
    }
}

fn validate_output_path(
    output_path: Option<&str>,
    reasons: &mut Vec<LocalArtifactManifestValidationReason>,
) {
    let Some(output_path) = output_path else {
        return;
    };

    if output_path.is_empty()
        || output_path.starts_with('~')
        || output_path.contains('\\')
        || Path::new(output_path).is_absolute()
        || has_traversal(output_path)
        || contains_forbidden_path_segment(output_path)
    {
        reasons.push(LocalArtifactManifestValidationReason::UnsafeOutputPath);
    }

    if !output_path.starts_with("artifacts/local/") || output_path == "artifacts/local/" {
        reasons.push(LocalArtifactManifestValidationReason::OutputPathNotLocalArtifacts);
    }
}

fn validate_phase_fields(
    manifest: &LocalArtifactManifest,
    reasons: &mut Vec<LocalArtifactManifestValidationReason>,
) {
    if let Some(created_by_phase) = manifest.created_by_phase.as_deref() {
        if !matches!(
            created_by_phase,
            "phase_132_3" | "132.3" | "phase_136_2" | "136.2"
        ) {
            reasons.push(LocalArtifactManifestValidationReason::InvalidCreatedByPhase);
        }
    }

    if let Some(deferred_to_phase) = manifest.deferred_to_phase.as_deref() {
        if !matches!(deferred_to_phase, "phase_133" | "phase_139" | "133" | "139") {
            reasons.push(LocalArtifactManifestValidationReason::InvalidDeferredToPhase);
        }
    }
}

fn validate_non_public_boundary(
    manifest: &LocalArtifactManifest,
    reasons: &mut Vec<LocalArtifactManifestValidationReason>,
) {
    if manifest.non_public != Some(true) {
        reasons.push(LocalArtifactManifestValidationReason::NonPublicNotTrue);
    }
}

fn validate_claim_boundaries(
    manifest: &LocalArtifactManifest,
    reasons: &mut Vec<LocalArtifactManifestValidationReason>,
) {
    if manifest.release_artifact_claim == Some(true) {
        reasons.push(LocalArtifactManifestValidationReason::ReleaseArtifactClaimPresent);
    }
    if manifest.readiness_claim == Some(true) {
        reasons.push(LocalArtifactManifestValidationReason::ReadinessClaimPresent);
    }
    if manifest.publishing_status == Some(LocalArtifactEvidenceStatus::ClaimedComplete) {
        reasons.push(LocalArtifactManifestValidationReason::PublishingClaimPresent);
    }
    if manifest.deployment_status == Some(LocalArtifactEvidenceStatus::ClaimedComplete) {
        reasons.push(LocalArtifactManifestValidationReason::DeploymentClaimPresent);
    }
    if manifest.signing_status == Some(LocalArtifactEvidenceStatus::ClaimedComplete) {
        reasons.push(LocalArtifactManifestValidationReason::SigningClaimPresent);
    }
    if manifest.checksum_status == Some(LocalArtifactEvidenceStatus::ClaimedComplete) {
        reasons.push(LocalArtifactManifestValidationReason::ChecksumClaimNotDeferred);
    }
    if manifest.provenance_status == Some(LocalArtifactEvidenceStatus::ClaimedComplete) {
        reasons.push(LocalArtifactManifestValidationReason::ProvenanceClaimNotDeferred);
    }
}

fn validate_extra_fields(
    extra_fields: &[String],
    reasons: &mut Vec<LocalArtifactManifestValidationReason>,
) {
    if extra_fields.iter().any(|field| {
        let normalized = field.to_ascii_lowercase();
        normalized.contains("installer")
            || normalized.contains("update_channel")
            || normalized.contains("appcast")
            || normalized.contains("download")
            || normalized.contains("public")
            || normalized.contains("release")
    }) {
        reasons.push(LocalArtifactManifestValidationReason::InstallerOrUpdateChannelClaimPresent);
    }
}

fn has_traversal(output_path: &str) -> bool {
    Path::new(output_path)
        .components()
        .any(|component| matches!(component, Component::ParentDir))
}

fn contains_forbidden_path_segment(output_path: &str) -> bool {
    Path::new(output_path).components().any(|component| {
        let Component::Normal(segment) = component else {
            return false;
        };
        let segment = segment.to_string_lossy().to_ascii_lowercase();
        matches!(
            segment.as_str(),
            ".git"
                | "target"
                | "node_modules"
                | "release"
                | "releases"
                | "download"
                | "downloads"
                | "dist"
                | "public"
                | "tmp"
                | "temp"
                | "etc"
                | "var"
                | "usr"
                | "bin"
                | "opt"
        )
    })
}

fn contains_forbidden_public_term(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    ["release", "public", "download", "deploy", "prod"]
        .iter()
        .any(|term| value.contains(term))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn producer_input() -> LocalArtifactManifestProducerInput {
        LocalArtifactManifestProducerInput {
            artifact_id: Some("phase-132-3-local-runtime".to_string()),
            artifact_name: Some("Phase 132.3 local runtime manifest candidate".to_string()),
            artifact_kind: Some(LocalArtifactKind::LocalRuntimeBuild),
            source_revision: Some("abcdef1234567890".to_string()),
            build_command: Some("cargo build --manifest-path core/Cargo.toml".to_string()),
            output_path: None,
            deferred_to_phase: None,
        }
    }

    fn valid_manifest() -> LocalArtifactManifest {
        LocalArtifactManifest {
            artifact_id: Some("phase-136-2-local-runtime".to_string()),
            artifact_name: Some("Phase 136.2 local runtime evidence".to_string()),
            artifact_kind: Some(LocalArtifactKindField::Known(
                LocalArtifactKind::LocalRuntimeBuild,
            )),
            source_revision: Some("abcdef1234567890".to_string()),
            build_command: Some("cargo build --manifest-path core/Cargo.toml".to_string()),
            output_path: Some("artifacts/local/phase-136-2/runtime-manifest.json".to_string()),
            created_by_phase: Some("phase_136_2".to_string()),
            non_public: Some(true),
            release_artifact_claim: Some(false),
            checksum_status: Some(LocalArtifactEvidenceStatus::Deferred),
            provenance_status: Some(LocalArtifactEvidenceStatus::Deferred),
            signing_status: Some(LocalArtifactEvidenceStatus::Absent),
            publishing_status: Some(LocalArtifactEvidenceStatus::Absent),
            deployment_status: Some(LocalArtifactEvidenceStatus::Absent),
            readiness_claim: Some(false),
            deferred_to_phase: Some("phase_139".to_string()),
            extra_fields: Vec::new(),
        }
    }

    fn assert_rejected_for(
        manifest: LocalArtifactManifest,
        reason: LocalArtifactManifestValidationReason,
    ) {
        let report = validate_local_artifact_manifest(&manifest);
        assert_eq!(report.status, ArtifactManifestValidationStatus::Rejected);
        assert!(report.reasons.contains(&reason), "{:?}", report.reasons);
    }

    #[test]
    fn producer_creates_manifest_candidate_accepted_by_validator() {
        let report = produce_local_artifact_manifest_candidate(&producer_input());

        assert_eq!(
            report.status,
            LocalArtifactManifestProducerStatus::ProducedAndValidated
        );
        assert!(report
            .reasons
            .contains(&LocalArtifactManifestProducerReason::FilesystemWriteNotRequested));
        let manifest = report.manifest.as_ref().expect("manifest candidate");
        let validation_report = validate_local_artifact_manifest(manifest);

        assert_eq!(
            validation_report.status,
            ArtifactManifestValidationStatus::AcceptedLocalNonPublic
        );
        assert_eq!(report.validation_report, Some(validation_report));
    }

    #[test]
    fn producer_defaults_output_path_under_local_artifacts() {
        let report = produce_local_artifact_manifest_candidate(&producer_input());

        assert_eq!(
            report.output_path.as_deref(),
            Some("artifacts/local/phase-132-3/phase-132-3-local-runtime-manifest.json")
        );
    }

    #[test]
    fn producer_sets_non_public_and_rejects_release_or_readiness_claims() {
        let report = produce_local_artifact_manifest_candidate(&producer_input());
        let manifest = report.manifest.expect("manifest candidate");

        assert_eq!(manifest.non_public, Some(true));
        assert_eq!(manifest.release_artifact_claim, Some(false));
        assert_eq!(manifest.readiness_claim, Some(false));
        assert!(!report.creates_artifact);
        assert!(!report.publishes_artifact);
        assert!(!report.deploys_artifact);
        assert!(!report.readiness_granted);
    }

    #[test]
    fn producer_keeps_downstream_statuses_deferred_or_absent() {
        let report = produce_local_artifact_manifest_candidate(&producer_input());
        let manifest = report.manifest.expect("manifest candidate");

        assert_eq!(
            manifest.checksum_status,
            Some(LocalArtifactEvidenceStatus::Deferred)
        );
        assert_eq!(
            manifest.provenance_status,
            Some(LocalArtifactEvidenceStatus::Deferred)
        );
        assert_eq!(
            manifest.signing_status,
            Some(LocalArtifactEvidenceStatus::Deferred)
        );
        assert_eq!(
            manifest.publishing_status,
            Some(LocalArtifactEvidenceStatus::Absent)
        );
        assert_eq!(
            manifest.deployment_status,
            Some(LocalArtifactEvidenceStatus::Absent)
        );
        assert_eq!(manifest.deferred_to_phase.as_deref(), Some("phase_133"));
        assert!(!report.creates_checksum);
        assert!(!report.creates_provenance);
        assert!(!report.signs_artifact);
    }

    #[test]
    fn producer_is_deterministic_for_identical_input() {
        let input = producer_input();

        let first = produce_local_artifact_manifest_candidate(&input);
        let second = produce_local_artifact_manifest_candidate(&input);

        assert_eq!(first, second);
    }

    #[test]
    fn producer_rejects_unsafe_requested_output_path() {
        let mut input = producer_input();
        input.output_path = Some("artifacts/local/phase-132-3/../manifest.json".to_string());

        let report = produce_local_artifact_manifest_candidate(&input);

        assert_eq!(report.status, LocalArtifactManifestProducerStatus::Rejected);
        assert!(report
            .reasons
            .contains(&LocalArtifactManifestProducerReason::InvalidOutputPath));
        assert!(report.manifest.is_none());
    }

    #[test]
    fn producer_rejects_public_release_download_and_dist_output_paths() {
        for path in [
            "artifacts/local/phase-132-3/public/manifest.json",
            "artifacts/local/phase-132-3/release/manifest.json",
            "artifacts/local/phase-132-3/download/manifest.json",
            "artifacts/local/phase-132-3/dist/manifest.json",
        ] {
            let mut input = producer_input();
            input.output_path = Some(path.to_string());

            let report = produce_local_artifact_manifest_candidate(&input);

            assert_eq!(report.status, LocalArtifactManifestProducerStatus::Rejected);
            assert!(report
                .reasons
                .contains(&LocalArtifactManifestProducerReason::InvalidOutputPath));
            assert!(report.manifest.is_none());
        }
    }

    #[test]
    fn producer_does_not_create_files_by_default() {
        let input = producer_input();
        let report = produce_local_artifact_manifest_candidate(&input);
        let output_path = report.output_path.clone().expect("output path");

        assert!(!Path::new(&output_path).exists());
        assert!(!report.mutates_filesystem);
        assert!(report
            .reasons
            .contains(&LocalArtifactManifestProducerReason::FilesystemWriteNotRequested));
    }

    #[test]
    fn producer_missing_required_inputs_rejects_without_manifest() {
        let input = LocalArtifactManifestProducerInput {
            artifact_id: None,
            artifact_name: None,
            artifact_kind: None,
            source_revision: None,
            build_command: None,
            output_path: None,
            deferred_to_phase: None,
        };

        let report = produce_local_artifact_manifest_candidate(&input);

        assert_eq!(report.status, LocalArtifactManifestProducerStatus::Rejected);
        assert!(report.manifest.is_none());
        assert!(report
            .reasons
            .contains(&LocalArtifactManifestProducerReason::MissingArtifactId));
        assert!(report
            .reasons
            .contains(&LocalArtifactManifestProducerReason::MissingArtifactName));
        assert!(report
            .reasons
            .contains(&LocalArtifactManifestProducerReason::MissingArtifactKind));
        assert!(report
            .reasons
            .contains(&LocalArtifactManifestProducerReason::MissingSourceRevision));
        assert!(report
            .reasons
            .contains(&LocalArtifactManifestProducerReason::MissingBuildCommand));
    }

    #[test]
    fn valid_local_non_public_manifest_is_accepted() {
        let report = validate_local_artifact_manifest(&valid_manifest());

        assert_eq!(
            report.status,
            ArtifactManifestValidationStatus::AcceptedLocalNonPublic
        );
        assert!(report.reasons.is_empty());
        assert!(report.non_public);
        assert!(!report.mutates_filesystem);
        assert!(!report.creates_artifact);
        assert!(!report.creates_checksum);
        assert!(!report.creates_provenance);
        assert!(!report.signs_artifact);
        assert!(!report.publishes_artifact);
        assert!(!report.deploys_artifact);
        assert!(!report.readiness_granted);
    }

    #[test]
    fn missing_fields_reject() {
        let mut manifest = valid_manifest();
        manifest.artifact_id = None;

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::MissingRequiredField,
        );
    }

    #[test]
    fn invalid_artifact_kind_rejects() {
        let mut manifest = valid_manifest();
        manifest.artifact_kind = Some(LocalArtifactKindField::Unsupported("release".to_string()));

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::InvalidArtifactKind,
        );
    }

    #[test]
    fn unsafe_output_path_rejects() {
        let mut manifest = valid_manifest();
        manifest.output_path = Some("artifacts/local/phase-136-2/.git/config".to_string());

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::UnsafeOutputPath,
        );
    }

    #[test]
    fn absolute_output_path_rejects() {
        let mut manifest = valid_manifest();
        manifest.output_path = Some("/tmp/ajentic/artifact".to_string());

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::UnsafeOutputPath,
        );
    }

    #[test]
    fn path_traversal_output_path_rejects() {
        let mut manifest = valid_manifest();
        manifest.output_path = Some("artifacts/local/phase-136-2/../../release".to_string());

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::UnsafeOutputPath,
        );
    }

    #[test]
    fn public_release_claim_rejects() {
        let mut manifest = valid_manifest();
        manifest.release_artifact_claim = Some(true);

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::ReleaseArtifactClaimPresent,
        );
    }

    #[test]
    fn readiness_claim_rejects() {
        let mut manifest = valid_manifest();
        manifest.readiness_claim = Some(true);

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::ReadinessClaimPresent,
        );
    }

    #[test]
    fn publishing_claim_rejects() {
        let mut manifest = valid_manifest();
        manifest.publishing_status = Some(LocalArtifactEvidenceStatus::ClaimedComplete);

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::PublishingClaimPresent,
        );
    }

    #[test]
    fn deployment_claim_rejects() {
        let mut manifest = valid_manifest();
        manifest.deployment_status = Some(LocalArtifactEvidenceStatus::ClaimedComplete);

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::DeploymentClaimPresent,
        );
    }

    #[test]
    fn signing_claim_rejects() {
        let mut manifest = valid_manifest();
        manifest.signing_status = Some(LocalArtifactEvidenceStatus::ClaimedComplete);

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::SigningClaimPresent,
        );
    }

    #[test]
    fn checksum_complete_claim_rejects() {
        let mut manifest = valid_manifest();
        manifest.checksum_status = Some(LocalArtifactEvidenceStatus::ClaimedComplete);

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::ChecksumClaimNotDeferred,
        );
    }

    #[test]
    fn provenance_complete_claim_rejects() {
        let mut manifest = valid_manifest();
        manifest.provenance_status = Some(LocalArtifactEvidenceStatus::ClaimedComplete);

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::ProvenanceClaimNotDeferred,
        );
    }

    #[test]
    fn installer_or_update_channel_claim_rejects() {
        let mut manifest = valid_manifest();
        manifest.extra_fields = vec!["installer_appcast_url".to_string()];

        assert_rejected_for(
            manifest,
            LocalArtifactManifestValidationReason::InstallerOrUpdateChannelClaimPresent,
        );
    }

    #[test]
    fn validation_is_deterministic_across_repeated_calls() {
        let manifest = valid_manifest();

        let first = validate_local_artifact_manifest(&manifest);
        let second = validate_local_artifact_manifest(&manifest);

        assert_eq!(first, second);
    }

    #[test]
    fn validation_does_not_create_files_or_mutate_input() {
        let manifest = valid_manifest();
        let before_manifest = manifest.clone();
        let output_path = manifest.output_path.clone().expect("output path");
        let report = validate_local_artifact_manifest(&manifest);

        assert_eq!(manifest, before_manifest);
        assert!(!Path::new(&output_path).exists());
        assert!(!report.mutates_filesystem);
        assert!(!report.creates_artifact);
    }
}
