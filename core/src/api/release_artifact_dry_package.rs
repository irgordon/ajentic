//! Local release artifact dry package assembly.
//!
//! A dry package is rehearsal evidence, not a release artifact.

use super::*;
use std::collections::BTreeMap;
use std::path::Path;

pub const RELEASE_ARTIFACT_DRY_PACKAGE_VERSION: &str = "phase-172-dry-package-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseArtifactDryPackageStatus {
    NotAssembled,
    DryPackageProjected,
    DryPackageValidated,
    DryPackageWritten,
    DryPackageReadBackValidated,
    DryPackageRejected,
    InvalidDryPackageInput,
}

impl ReleaseArtifactDryPackageStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotAssembled => "not_assembled",
            Self::DryPackageProjected => "dry_package_projected",
            Self::DryPackageValidated => "dry_package_validated",
            Self::DryPackageWritten => "dry_package_written",
            Self::DryPackageReadBackValidated => "dry_package_read_back_validated",
            Self::DryPackageRejected => "dry_package_rejected",
            Self::InvalidDryPackageInput => "invalid_dry_package_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseArtifactDryPackageClassification {
    DryRunPackageOnly,
}

impl ReleaseArtifactDryPackageClassification {
    pub fn code(&self) -> &'static str {
        "dry_run_package_only"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseArtifactDryPackageProductionClassification {
    NonProduction,
}

impl ReleaseArtifactDryPackageProductionClassification {
    pub fn code(&self) -> &'static str {
        "non_production"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseArtifactDryPackageDistributionClassification {
    LocalOnlyNonPublic,
}

impl ReleaseArtifactDryPackageDistributionClassification {
    pub fn code(&self) -> &'static str {
        "local_only_non_public"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseArtifactDryPackageAuthorityClassification {
    NonAuthoritativeRehearsalEvidence,
}

impl ReleaseArtifactDryPackageAuthorityClassification {
    pub fn code(&self) -> &'static str {
        "non_authoritative_rehearsal_evidence"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseArtifactDryPackageReleaseClassification {
    ReleaseNotApproved,
}

impl ReleaseArtifactDryPackageReleaseClassification {
    pub fn code(&self) -> &'static str {
        "release_not_approved"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseArtifactDryPackageValidationStatus {
    NotValidated,
    Valid,
    Invalid,
}

impl ReleaseArtifactDryPackageValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotValidated => "not_validated",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseArtifactDryPackageValidationError {
    MissingPreparationContract,
    PreparationNotProjected,
    PreparationBlocked,
    PreparationRejected,
    PreparationMissingRequiredEvidence,
    MissingDryPackageId,
    MissingDryPackageVersion,
    InvalidDryPackageClassification,
    InvalidProductionClassification,
    InvalidDistributionClassification,
    InvalidAuthorityClassification,
    InvalidReleaseClassification,
    MissingIncludedEvidence,
    MalformedDryPackageInput,
    DeterministicDigestMismatch,
    ReadinessClaimDetected,
    ReleaseClaimDetected,
    DeploymentClaimDetected,
    PublicUseClaimDetected,
    ProductionUseClaimDetected,
    SigningClaimDetected,
    PublishingClaimDetected,
    InstallerClaimDetected,
    UpdateChannelClaimDetected,
    PublicDownloadClaimDetected,
    GithubReleaseClaimDetected,
    ReleaseTagClaimDetected,
    ProviderTrustClaimDetected,
    ActionAuthorizationClaimDetected,
    ReplayRepairClaimDetected,
    RecoveryPromotionClaimDetected,
}

impl ReleaseArtifactDryPackageValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingPreparationContract => "missing_preparation_contract",
            Self::PreparationNotProjected => "preparation_not_projected",
            Self::PreparationBlocked => "preparation_blocked",
            Self::PreparationRejected => "preparation_rejected",
            Self::PreparationMissingRequiredEvidence => "preparation_missing_required_evidence",
            Self::MissingDryPackageId => "missing_dry_package_id",
            Self::MissingDryPackageVersion => "missing_dry_package_version",
            Self::InvalidDryPackageClassification => "invalid_dry_package_classification",
            Self::InvalidProductionClassification => "invalid_production_classification",
            Self::InvalidDistributionClassification => "invalid_distribution_classification",
            Self::InvalidAuthorityClassification => "invalid_authority_classification",
            Self::InvalidReleaseClassification => "invalid_release_classification",
            Self::MissingIncludedEvidence => "missing_included_evidence",
            Self::MalformedDryPackageInput => "malformed_dry_package_input",
            Self::DeterministicDigestMismatch => "deterministic_digest_mismatch",
            Self::ReadinessClaimDetected => "readiness_claim_detected",
            Self::ReleaseClaimDetected => "release_claim_detected",
            Self::DeploymentClaimDetected => "deployment_claim_detected",
            Self::PublicUseClaimDetected => "public_use_claim_detected",
            Self::ProductionUseClaimDetected => "production_use_claim_detected",
            Self::SigningClaimDetected => "signing_claim_detected",
            Self::PublishingClaimDetected => "publishing_claim_detected",
            Self::InstallerClaimDetected => "installer_claim_detected",
            Self::UpdateChannelClaimDetected => "update_channel_claim_detected",
            Self::PublicDownloadClaimDetected => "public_download_claim_detected",
            Self::GithubReleaseClaimDetected => "github_release_claim_detected",
            Self::ReleaseTagClaimDetected => "release_tag_claim_detected",
            Self::ProviderTrustClaimDetected => "provider_trust_claim_detected",
            Self::ActionAuthorizationClaimDetected => "action_authorization_claim_detected",
            Self::ReplayRepairClaimDetected => "replay_repair_claim_detected",
            Self::RecoveryPromotionClaimDetected => "recovery_promotion_claim_detected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseArtifactDryPackageBoundaryStatus {
    DryRunPackageOnly,
    LocalDryPackageOnly,
    NonPublicDryPackage,
    NonAuthoritativeRehearsalEvidence,
    ReleaseArtifactNotCreated,
    ReleaseReadinessNotApproved,
    ReleaseCandidateStatusNotApproved,
    ProductionStatusNotApproved,
    NoPublicDistribution,
    NoPublicDownload,
    NoGithubRelease,
    NoReleaseTag,
    NoSigning,
    NoPublishing,
    NoInstallerActivation,
    NoUpdateChannelActivation,
    NoDeploymentArtifact,
    NoProviderTrust,
    NoActionAuthorization,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl ReleaseArtifactDryPackageBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DryRunPackageOnly => "dry_run_package_only",
            Self::LocalDryPackageOnly => "local_dry_package_only",
            Self::NonPublicDryPackage => "non_public_dry_package",
            Self::NonAuthoritativeRehearsalEvidence => "non_authoritative_rehearsal_evidence",
            Self::ReleaseArtifactNotCreated => "release_artifact_not_created",
            Self::ReleaseReadinessNotApproved => "release_readiness_not_approved",
            Self::ReleaseCandidateStatusNotApproved => "release_candidate_status_not_approved",
            Self::ProductionStatusNotApproved => "production_status_not_approved",
            Self::NoPublicDistribution => "no_public_distribution",
            Self::NoPublicDownload => "no_public_download",
            Self::NoGithubRelease => "no_github_release",
            Self::NoReleaseTag => "no_release_tag",
            Self::NoSigning => "no_signing",
            Self::NoPublishing => "no_publishing",
            Self::NoInstallerActivation => "no_installer_activation",
            Self::NoUpdateChannelActivation => "no_update_channel_activation",
            Self::NoDeploymentArtifact => "no_deployment_artifact",
            Self::NoProviderTrust => "no_provider_trust",
            Self::NoActionAuthorization => "no_action_authorization",
            Self::NoReplayRepair => "no_replay_repair",
            Self::NoRecoveryPromotion => "no_recovery_promotion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifactDryPackageIncludedEvidence {
    pub category: ReleaseCandidatePreparationEvidenceCategory,
    pub category_status: ReleaseCandidatePreparationEvidenceCategoryStatus,
    pub source_surface: String,
    pub source_status: String,
    pub source_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifactDryPackageMetadata {
    pub dry_package_id: String,
    pub dry_package_version: String,
    pub dry_package_classification: ReleaseArtifactDryPackageClassification,
    pub production_classification: ReleaseArtifactDryPackageProductionClassification,
    pub distribution_classification: ReleaseArtifactDryPackageDistributionClassification,
    pub authority_classification: ReleaseArtifactDryPackageAuthorityClassification,
    pub release_classification: ReleaseArtifactDryPackageReleaseClassification,
    pub dry_package_status: ReleaseArtifactDryPackageStatus,
    pub validation_status: ReleaseArtifactDryPackageValidationStatus,
    pub content_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifactDryPackagePayload {
    pub source_preparation_id: String,
    pub source_preparation_status: ReleaseCandidatePreparationStatus,
    pub source_preparation_validation_status: ReleaseCandidatePreparationValidationStatus,
    pub included_evidence: Vec<ReleaseArtifactDryPackageIncludedEvidence>,
    pub included_evidence_count: usize,
    pub boundary_statuses: Vec<ReleaseArtifactDryPackageBoundaryStatus>,
    pub dry_package_only_note: String,
    pub not_release_artifact_note: String,
    pub no_readiness_or_candidate_approval_note: String,
    pub local_only_non_public_note: String,
    pub no_distribution_note: String,
    pub read_back_validation_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifactDryPackage {
    pub metadata: ReleaseArtifactDryPackageMetadata,
    pub payload: ReleaseArtifactDryPackagePayload,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifactDryPackageProjection {
    pub status: ReleaseArtifactDryPackageStatus,
    pub dry_package_id: Option<String>,
    pub dry_package_classification: ReleaseArtifactDryPackageClassification,
    pub production_classification: ReleaseArtifactDryPackageProductionClassification,
    pub distribution_classification: ReleaseArtifactDryPackageDistributionClassification,
    pub authority_classification: ReleaseArtifactDryPackageAuthorityClassification,
    pub release_classification: ReleaseArtifactDryPackageReleaseClassification,
    pub validation_status: ReleaseArtifactDryPackageValidationStatus,
    pub validation_errors: Vec<ReleaseArtifactDryPackageValidationError>,
    pub read_back_validation_status: ReleaseArtifactDryPackageValidationStatus,
    pub included_evidence_count: usize,
    pub included_evidence: Vec<ReleaseArtifactDryPackageIncludedEvidence>,
    pub boundary_statuses: Vec<ReleaseArtifactDryPackageBoundaryStatus>,
    pub dry_package_only_note: String,
    pub not_release_artifact_note: String,
    pub no_readiness_or_candidate_approval_note: String,
    pub local_only_non_public_note: String,
    pub no_distribution_note: String,
    pub read_back_validation_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifactDryPackageReadBackProjection {
    pub status: ReleaseArtifactDryPackageStatus,
    pub validation_status: ReleaseArtifactDryPackageValidationStatus,
    pub validation_errors: Vec<ReleaseArtifactDryPackageValidationError>,
    pub dry_package_id: Option<String>,
    pub included_evidence_count: usize,
    pub read_back_validation_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifactDryPackageWriteResult {
    pub path: String,
    pub projection: ReleaseArtifactDryPackageProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifactDryPackageReadResult {
    pub path: String,
    pub package: ReleaseArtifactDryPackage,
    pub read_back: ReleaseArtifactDryPackageReadBackProjection,
}

pub fn release_artifact_dry_package_boundary_statuses(
) -> Vec<ReleaseArtifactDryPackageBoundaryStatus> {
    vec![
        ReleaseArtifactDryPackageBoundaryStatus::DryRunPackageOnly,
        ReleaseArtifactDryPackageBoundaryStatus::LocalDryPackageOnly,
        ReleaseArtifactDryPackageBoundaryStatus::NonPublicDryPackage,
        ReleaseArtifactDryPackageBoundaryStatus::NonAuthoritativeRehearsalEvidence,
        ReleaseArtifactDryPackageBoundaryStatus::ReleaseArtifactNotCreated,
        ReleaseArtifactDryPackageBoundaryStatus::ReleaseReadinessNotApproved,
        ReleaseArtifactDryPackageBoundaryStatus::ReleaseCandidateStatusNotApproved,
        ReleaseArtifactDryPackageBoundaryStatus::ProductionStatusNotApproved,
        ReleaseArtifactDryPackageBoundaryStatus::NoPublicDistribution,
        ReleaseArtifactDryPackageBoundaryStatus::NoPublicDownload,
        ReleaseArtifactDryPackageBoundaryStatus::NoGithubRelease,
        ReleaseArtifactDryPackageBoundaryStatus::NoReleaseTag,
        ReleaseArtifactDryPackageBoundaryStatus::NoSigning,
        ReleaseArtifactDryPackageBoundaryStatus::NoPublishing,
        ReleaseArtifactDryPackageBoundaryStatus::NoInstallerActivation,
        ReleaseArtifactDryPackageBoundaryStatus::NoUpdateChannelActivation,
        ReleaseArtifactDryPackageBoundaryStatus::NoDeploymentArtifact,
        ReleaseArtifactDryPackageBoundaryStatus::NoProviderTrust,
        ReleaseArtifactDryPackageBoundaryStatus::NoActionAuthorization,
        ReleaseArtifactDryPackageBoundaryStatus::NoReplayRepair,
        ReleaseArtifactDryPackageBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub fn release_artifact_dry_package_included_evidence_summary(
    preparation: &ReleaseCandidatePreparationProjection,
) -> Vec<ReleaseArtifactDryPackageIncludedEvidence> {
    let mut evidence = preparation
        .evidence_items
        .iter()
        .map(|item| ReleaseArtifactDryPackageIncludedEvidence {
            category: item.category,
            category_status: item.status,
            source_surface: item.source_linkage.source_surface.clone(),
            source_status: item.source_linkage.source_status.clone(),
            source_summary: item.source_linkage.source_summary.clone(),
        })
        .collect::<Vec<_>>();
    evidence.sort_by(|left, right| left.category.code().cmp(right.category.code()));
    evidence
}

pub fn initial_release_artifact_dry_package_projection() -> ReleaseArtifactDryPackageProjection {
    ReleaseArtifactDryPackageProjection {
        status: ReleaseArtifactDryPackageStatus::NotAssembled,
        dry_package_id: None,
        dry_package_classification: ReleaseArtifactDryPackageClassification::DryRunPackageOnly,
        production_classification: ReleaseArtifactDryPackageProductionClassification::NonProduction,
        distribution_classification:
            ReleaseArtifactDryPackageDistributionClassification::LocalOnlyNonPublic,
        authority_classification:
            ReleaseArtifactDryPackageAuthorityClassification::NonAuthoritativeRehearsalEvidence,
        release_classification: ReleaseArtifactDryPackageReleaseClassification::ReleaseNotApproved,
        validation_status: ReleaseArtifactDryPackageValidationStatus::NotValidated,
        validation_errors: Vec::new(),
        read_back_validation_status: ReleaseArtifactDryPackageValidationStatus::NotValidated,
        included_evidence_count: 0,
        included_evidence: Vec::new(),
        boundary_statuses: release_artifact_dry_package_boundary_statuses(),
        dry_package_only_note: "A dry package is rehearsal evidence, not a release artifact.".to_string(),
        not_release_artifact_note: "This local dry package is not a release artifact.".to_string(),
        no_readiness_or_candidate_approval_note:
            "This package does not approve release readiness or Release Candidate status."
                .to_string(),
        local_only_non_public_note: "This package is local-only and non-public.".to_string(),
        no_distribution_note: "No signing, publishing, installer, update-channel, public download, GitHub release, release tag, deployment, or public distribution occurs.".to_string(),
        read_back_validation_note: "Read-back validation checks dry package structure only.".to_string(),
    }
}

pub fn derive_release_artifact_dry_package(
    preparation: Option<&ReleaseCandidatePreparationProjection>,
) -> Result<ReleaseArtifactDryPackage, Vec<ReleaseArtifactDryPackageValidationError>> {
    let preparation = match preparation {
        Some(preparation) => preparation,
        None => {
            return Err(vec![
                ReleaseArtifactDryPackageValidationError::MissingPreparationContract,
            ])
        }
    };
    let preparation_errors = validate_preparation_for_dry_package(preparation);
    if !preparation_errors.is_empty() {
        return Err(preparation_errors);
    }
    let included_evidence = release_artifact_dry_package_included_evidence_summary(preparation);
    let payload = ReleaseArtifactDryPackagePayload {
        source_preparation_id: preparation.preparation_id.clone(),
        source_preparation_status: preparation.status,
        source_preparation_validation_status: preparation.validation_status,
        included_evidence_count: included_evidence.len(),
        included_evidence,
        boundary_statuses: release_artifact_dry_package_boundary_statuses(),
        dry_package_only_note: "A dry package is rehearsal evidence, not a release artifact.".to_string(),
        not_release_artifact_note: "This local dry package is not a release artifact.".to_string(),
        no_readiness_or_candidate_approval_note:
            "This package does not approve release readiness or Release Candidate status."
                .to_string(),
        local_only_non_public_note: "This package is local-only and non-public.".to_string(),
        no_distribution_note: "No signing, publishing, installer, update-channel, public download, GitHub release, release tag, deployment, or public distribution occurs.".to_string(),
        read_back_validation_note: "Read-back validation checks dry package structure only.".to_string(),
    };
    let content_digest =
        stable_release_artifact_dry_package_digest(&dry_package_payload_basis(&payload));
    let metadata = ReleaseArtifactDryPackageMetadata {
        dry_package_id: format!("release-artifact-dry-package-{content_digest}"),
        dry_package_version: RELEASE_ARTIFACT_DRY_PACKAGE_VERSION.to_string(),
        dry_package_classification: ReleaseArtifactDryPackageClassification::DryRunPackageOnly,
        production_classification: ReleaseArtifactDryPackageProductionClassification::NonProduction,
        distribution_classification:
            ReleaseArtifactDryPackageDistributionClassification::LocalOnlyNonPublic,
        authority_classification:
            ReleaseArtifactDryPackageAuthorityClassification::NonAuthoritativeRehearsalEvidence,
        release_classification: ReleaseArtifactDryPackageReleaseClassification::ReleaseNotApproved,
        dry_package_status: ReleaseArtifactDryPackageStatus::DryPackageProjected,
        validation_status: ReleaseArtifactDryPackageValidationStatus::NotValidated,
        content_digest,
    };
    let mut package = ReleaseArtifactDryPackage { metadata, payload };
    validate_release_artifact_dry_package(&package)?;
    package.metadata.dry_package_status = ReleaseArtifactDryPackageStatus::DryPackageValidated;
    package.metadata.validation_status = ReleaseArtifactDryPackageValidationStatus::Valid;
    Ok(package)
}

pub fn project_release_artifact_dry_package(
    preparation: Option<&ReleaseCandidatePreparationProjection>,
) -> ReleaseArtifactDryPackageProjection {
    match derive_release_artifact_dry_package(preparation) {
        Ok(package) => projection_from_package(
            &package,
            ReleaseArtifactDryPackageValidationStatus::NotValidated,
            Vec::new(),
        ),
        Err(errors) => {
            let mut projection = initial_release_artifact_dry_package_projection();
            projection.status = ReleaseArtifactDryPackageStatus::DryPackageRejected;
            projection.validation_status = ReleaseArtifactDryPackageValidationStatus::Invalid;
            projection.validation_errors = errors;
            projection
        }
    }
}

pub fn validate_release_artifact_dry_package(
    package: &ReleaseArtifactDryPackage,
) -> Result<(), Vec<ReleaseArtifactDryPackageValidationError>> {
    let mut errors = Vec::new();
    if package.metadata.dry_package_id.is_empty() {
        errors.push(ReleaseArtifactDryPackageValidationError::MissingDryPackageId);
    }
    if package.metadata.dry_package_version != RELEASE_ARTIFACT_DRY_PACKAGE_VERSION {
        errors.push(ReleaseArtifactDryPackageValidationError::MissingDryPackageVersion);
    }
    if package.metadata.dry_package_classification
        != ReleaseArtifactDryPackageClassification::DryRunPackageOnly
    {
        errors.push(ReleaseArtifactDryPackageValidationError::InvalidDryPackageClassification);
    }
    if package.metadata.production_classification
        != ReleaseArtifactDryPackageProductionClassification::NonProduction
    {
        errors.push(ReleaseArtifactDryPackageValidationError::InvalidProductionClassification);
    }
    if package.metadata.distribution_classification
        != ReleaseArtifactDryPackageDistributionClassification::LocalOnlyNonPublic
    {
        errors.push(ReleaseArtifactDryPackageValidationError::InvalidDistributionClassification);
    }
    if package.metadata.authority_classification
        != ReleaseArtifactDryPackageAuthorityClassification::NonAuthoritativeRehearsalEvidence
    {
        errors.push(ReleaseArtifactDryPackageValidationError::InvalidAuthorityClassification);
    }
    if package.metadata.release_classification
        != ReleaseArtifactDryPackageReleaseClassification::ReleaseNotApproved
    {
        errors.push(ReleaseArtifactDryPackageValidationError::InvalidReleaseClassification);
    }
    if package.payload.included_evidence.is_empty()
        || package.payload.included_evidence_count != package.payload.included_evidence.len()
    {
        errors.push(ReleaseArtifactDryPackageValidationError::MissingIncludedEvidence);
    }
    errors.extend(validate_payload_preparation_shape(&package.payload));
    errors.extend(detect_dry_package_claims(package));
    let expected_digest =
        stable_release_artifact_dry_package_digest(&dry_package_payload_basis(&package.payload));
    if package.metadata.content_digest != expected_digest
        || package.metadata.dry_package_id
            != format!("release-artifact-dry-package-{expected_digest}")
    {
        errors.push(ReleaseArtifactDryPackageValidationError::DeterministicDigestMismatch);
    }
    errors.sort();
    errors.dedup();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn serialize_release_artifact_dry_package(
    package: &ReleaseArtifactDryPackage,
) -> Result<String, Vec<ReleaseArtifactDryPackageValidationError>> {
    validate_release_artifact_dry_package(package)?;
    let evidence = package
        .payload
        .included_evidence
        .iter()
        .map(|item| {
            [
                item.category.code(),
                item.category_status.code(),
                &hex_encode(&item.source_surface),
                &hex_encode(&item.source_status),
                &hex_encode(&item.source_summary),
            ]
            .join("|")
        })
        .collect::<Vec<_>>()
        .join(";;");
    let boundaries = package
        .payload
        .boundary_statuses
        .iter()
        .map(|status| status.code())
        .collect::<Vec<_>>()
        .join(",");
    let lines = vec![
        "ajentic_release_artifact_dry_package=v1".to_string(),
        format!("dry_package_id={}", package.metadata.dry_package_id),
        format!(
            "dry_package_version={}",
            package.metadata.dry_package_version
        ),
        format!(
            "dry_package_classification={}",
            package.metadata.dry_package_classification.code()
        ),
        format!(
            "production_classification={}",
            package.metadata.production_classification.code()
        ),
        format!(
            "distribution_classification={}",
            package.metadata.distribution_classification.code()
        ),
        format!(
            "authority_classification={}",
            package.metadata.authority_classification.code()
        ),
        format!(
            "release_classification={}",
            package.metadata.release_classification.code()
        ),
        format!(
            "dry_package_status={}",
            package.metadata.dry_package_status.code()
        ),
        format!(
            "validation_status={}",
            package.metadata.validation_status.code()
        ),
        format!("content_digest={}", package.metadata.content_digest),
        format!(
            "source_preparation_id={}",
            package.payload.source_preparation_id
        ),
        format!(
            "source_preparation_status={}",
            package.payload.source_preparation_status.code()
        ),
        format!(
            "source_preparation_validation_status={}",
            package.payload.source_preparation_validation_status.code()
        ),
        format!(
            "included_evidence_count={}",
            package.payload.included_evidence_count
        ),
        format!("included_evidence={evidence}"),
        format!("boundary_statuses={boundaries}"),
        format!(
            "dry_package_only_note={}",
            hex_encode(&package.payload.dry_package_only_note)
        ),
        format!(
            "not_release_artifact_note={}",
            hex_encode(&package.payload.not_release_artifact_note)
        ),
        format!(
            "no_readiness_or_candidate_approval_note={}",
            hex_encode(&package.payload.no_readiness_or_candidate_approval_note)
        ),
        format!(
            "local_only_non_public_note={}",
            hex_encode(&package.payload.local_only_non_public_note)
        ),
        format!(
            "no_distribution_note={}",
            hex_encode(&package.payload.no_distribution_note)
        ),
        format!(
            "read_back_validation_note={}",
            hex_encode(&package.payload.read_back_validation_note)
        ),
    ];
    Ok(format!("{}\n", lines.join("\n")))
}

pub fn parse_release_artifact_dry_package(
    input: &str,
) -> Result<ReleaseArtifactDryPackage, Vec<ReleaseArtifactDryPackageValidationError>> {
    let values = parse_key_values(input)?;
    if values
        .get("ajentic_release_artifact_dry_package")
        .map(String::as_str)
        != Some("v1")
    {
        return Err(vec![
            ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput,
        ]);
    }
    let metadata = ReleaseArtifactDryPackageMetadata {
        dry_package_id: get_value(&values, "dry_package_id")?,
        dry_package_version: get_value(&values, "dry_package_version")?,
        dry_package_classification: parse_dry_package_classification(&get_value(
            &values,
            "dry_package_classification",
        )?)?,
        production_classification: parse_production_classification(&get_value(
            &values,
            "production_classification",
        )?)?,
        distribution_classification: parse_distribution_classification(&get_value(
            &values,
            "distribution_classification",
        )?)?,
        authority_classification: parse_authority_classification(&get_value(
            &values,
            "authority_classification",
        )?)?,
        release_classification: parse_release_classification(&get_value(
            &values,
            "release_classification",
        )?)?,
        dry_package_status: parse_dry_package_status(&get_value(&values, "dry_package_status")?)?,
        validation_status: parse_validation_status(&get_value(&values, "validation_status")?)?,
        content_digest: get_value(&values, "content_digest")?,
    };
    let included_evidence = parse_included_evidence(&get_value(&values, "included_evidence")?)?;
    let included_evidence_count = get_value(&values, "included_evidence_count")?
        .parse::<usize>()
        .map_err(|_| vec![ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput])?;
    let payload = ReleaseArtifactDryPackagePayload {
        source_preparation_id: get_value(&values, "source_preparation_id")?,
        source_preparation_status: parse_preparation_status(&get_value(
            &values,
            "source_preparation_status",
        )?)?,
        source_preparation_validation_status: parse_preparation_validation_status(&get_value(
            &values,
            "source_preparation_validation_status",
        )?)?,
        included_evidence,
        included_evidence_count,
        boundary_statuses: parse_boundary_statuses(&get_value(&values, "boundary_statuses")?)?,
        dry_package_only_note: hex_decode(&get_value(&values, "dry_package_only_note")?)?,
        not_release_artifact_note: hex_decode(&get_value(&values, "not_release_artifact_note")?)?,
        no_readiness_or_candidate_approval_note: hex_decode(&get_value(
            &values,
            "no_readiness_or_candidate_approval_note",
        )?)?,
        local_only_non_public_note: hex_decode(&get_value(&values, "local_only_non_public_note")?)?,
        no_distribution_note: hex_decode(&get_value(&values, "no_distribution_note")?)?,
        read_back_validation_note: hex_decode(&get_value(&values, "read_back_validation_note")?)?,
    };
    let package = ReleaseArtifactDryPackage { metadata, payload };
    validate_release_artifact_dry_package(&package)?;
    Ok(package)
}

pub fn write_release_artifact_dry_package(
    package: &ReleaseArtifactDryPackage,
    caller_provided_path: &Path,
) -> Result<ReleaseArtifactDryPackageWriteResult, Vec<ReleaseArtifactDryPackageValidationError>> {
    validate_release_artifact_dry_package(package)?;
    let serialized = serialize_release_artifact_dry_package(package)?;
    std::fs::write(caller_provided_path, serialized)
        .map_err(|_| vec![ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput])?;
    let mut written = package.clone();
    written.metadata.dry_package_status = ReleaseArtifactDryPackageStatus::DryPackageWritten;
    Ok(ReleaseArtifactDryPackageWriteResult {
        path: caller_provided_path.display().to_string(),
        projection: projection_from_package(
            &written,
            ReleaseArtifactDryPackageValidationStatus::NotValidated,
            Vec::new(),
        ),
    })
}

pub fn read_release_artifact_dry_package(
    caller_provided_path: &Path,
) -> Result<ReleaseArtifactDryPackageReadResult, Vec<ReleaseArtifactDryPackageValidationError>> {
    let contents = std::fs::read_to_string(caller_provided_path)
        .map_err(|_| vec![ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput])?;
    let package = parse_release_artifact_dry_package(&contents)?;
    let read_back = validate_release_artifact_dry_package_read_back(&contents);
    Ok(ReleaseArtifactDryPackageReadResult {
        path: caller_provided_path.display().to_string(),
        package,
        read_back,
    })
}

pub fn validate_release_artifact_dry_package_read_back(
    input: &str,
) -> ReleaseArtifactDryPackageReadBackProjection {
    match parse_release_artifact_dry_package(input) {
        Ok(package) => ReleaseArtifactDryPackageReadBackProjection {
            status: ReleaseArtifactDryPackageStatus::DryPackageReadBackValidated,
            validation_status: ReleaseArtifactDryPackageValidationStatus::Valid,
            validation_errors: Vec::new(),
            dry_package_id: Some(package.metadata.dry_package_id),
            included_evidence_count: package.payload.included_evidence_count,
            read_back_validation_note: package.payload.read_back_validation_note,
        },
        Err(errors) => ReleaseArtifactDryPackageReadBackProjection {
            status: ReleaseArtifactDryPackageStatus::InvalidDryPackageInput,
            validation_status: ReleaseArtifactDryPackageValidationStatus::Invalid,
            validation_errors: errors,
            dry_package_id: None,
            included_evidence_count: 0,
            read_back_validation_note: "Read-back validation checks dry package structure only."
                .to_string(),
        },
    }
}

fn projection_from_package(
    package: &ReleaseArtifactDryPackage,
    read_back_validation_status: ReleaseArtifactDryPackageValidationStatus,
    validation_errors: Vec<ReleaseArtifactDryPackageValidationError>,
) -> ReleaseArtifactDryPackageProjection {
    ReleaseArtifactDryPackageProjection {
        status: package.metadata.dry_package_status,
        dry_package_id: Some(package.metadata.dry_package_id.clone()),
        dry_package_classification: package.metadata.dry_package_classification,
        production_classification: package.metadata.production_classification,
        distribution_classification: package.metadata.distribution_classification,
        authority_classification: package.metadata.authority_classification,
        release_classification: package.metadata.release_classification,
        validation_status: package.metadata.validation_status,
        validation_errors,
        read_back_validation_status,
        included_evidence_count: package.payload.included_evidence_count,
        included_evidence: package.payload.included_evidence.clone(),
        boundary_statuses: package.payload.boundary_statuses.clone(),
        dry_package_only_note: package.payload.dry_package_only_note.clone(),
        not_release_artifact_note: package.payload.not_release_artifact_note.clone(),
        no_readiness_or_candidate_approval_note: package
            .payload
            .no_readiness_or_candidate_approval_note
            .clone(),
        local_only_non_public_note: package.payload.local_only_non_public_note.clone(),
        no_distribution_note: package.payload.no_distribution_note.clone(),
        read_back_validation_note: package.payload.read_back_validation_note.clone(),
    }
}

fn validate_preparation_for_dry_package(
    preparation: &ReleaseCandidatePreparationProjection,
) -> Vec<ReleaseArtifactDryPackageValidationError> {
    let mut errors = Vec::new();
    if preparation.preparation_id.is_empty() {
        errors.push(ReleaseArtifactDryPackageValidationError::MissingPreparationContract);
    }
    match preparation.status {
        ReleaseCandidatePreparationStatus::PreparationValidated => {}
        ReleaseCandidatePreparationStatus::PreparationBlocked => {
            errors.push(ReleaseArtifactDryPackageValidationError::PreparationBlocked)
        }
        ReleaseCandidatePreparationStatus::PreparationRejected => {
            errors.push(ReleaseArtifactDryPackageValidationError::PreparationRejected)
        }
        _ => errors.push(ReleaseArtifactDryPackageValidationError::PreparationNotProjected),
    }
    if preparation.validation_status != ReleaseCandidatePreparationValidationStatus::Valid {
        errors.push(ReleaseArtifactDryPackageValidationError::PreparationMissingRequiredEvidence);
    }
    if !preparation.missing_evidence.is_empty() || preparation.missing_evidence_count > 0 {
        errors.push(ReleaseArtifactDryPackageValidationError::PreparationMissingRequiredEvidence);
    }
    errors.extend(
        preparation
            .validation_errors
            .iter()
            .map(|error| match error {
                ReleaseCandidatePreparationValidationError::EvidenceCategoryBlocked => {
                    ReleaseArtifactDryPackageValidationError::PreparationBlocked
                }
                ReleaseCandidatePreparationValidationError::EvidenceCategoryRejected => {
                    ReleaseArtifactDryPackageValidationError::PreparationRejected
                }
                ReleaseCandidatePreparationValidationError::ReadinessClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::ReadinessClaimDetected
                }
                ReleaseCandidatePreparationValidationError::ReleaseClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::ReleaseClaimDetected
                }
                ReleaseCandidatePreparationValidationError::DeploymentClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::DeploymentClaimDetected
                }
                ReleaseCandidatePreparationValidationError::PublicUseClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::PublicUseClaimDetected
                }
                ReleaseCandidatePreparationValidationError::ProductionUseClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::ProductionUseClaimDetected
                }
                ReleaseCandidatePreparationValidationError::SigningClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::SigningClaimDetected
                }
                ReleaseCandidatePreparationValidationError::PublishingClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::PublishingClaimDetected
                }
                ReleaseCandidatePreparationValidationError::InstallerClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::InstallerClaimDetected
                }
                ReleaseCandidatePreparationValidationError::UpdateChannelClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::UpdateChannelClaimDetected
                }
                ReleaseCandidatePreparationValidationError::ProviderTrustClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::ProviderTrustClaimDetected
                }
                ReleaseCandidatePreparationValidationError::ActionAuthorizationClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::ActionAuthorizationClaimDetected
                }
                ReleaseCandidatePreparationValidationError::ReplayRepairClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::ReplayRepairClaimDetected
                }
                ReleaseCandidatePreparationValidationError::RecoveryPromotionClaimDetected => {
                    ReleaseArtifactDryPackageValidationError::RecoveryPromotionClaimDetected
                }
                _ => ReleaseArtifactDryPackageValidationError::PreparationMissingRequiredEvidence,
            }),
    );
    errors.sort();
    errors.dedup();
    errors
}

fn validate_payload_preparation_shape(
    payload: &ReleaseArtifactDryPackagePayload,
) -> Vec<ReleaseArtifactDryPackageValidationError> {
    let projection = ReleaseCandidatePreparationProjection {
        preparation_id: payload.source_preparation_id.clone(),
        status: payload.source_preparation_status,
        validation_status: payload.source_preparation_validation_status,
        classification: vec![],
        evidence_items: payload
            .included_evidence
            .iter()
            .map(|item| ReleaseCandidatePreparationEvidenceItem {
                category: item.category,
                status: item.category_status,
                reason: "included in dry package".to_string(),
                source_linkage: ReleaseCandidatePreparationSourceLinkage {
                    category: item.category,
                    source_surface: item.source_surface.clone(),
                    source_status: item.source_status.clone(),
                    source_summary: item.source_summary.clone(),
                },
            })
            .collect(),
        missing_evidence: Vec::new(),
        blockers: Vec::new(),
        validation_errors: Vec::new(),
        boundary_statuses: Vec::new(),
        capability_surface: release_candidate_preparation_capability_surface(),
        category_count: payload.included_evidence_count,
        present_evidence_count: payload.included_evidence_count,
        missing_evidence_count: 0,
        blocked_evidence_count: 0,
        rejected_evidence_count: 0,
        no_release_readiness_note: String::new(),
        no_release_artifact_note: String::new(),
        no_release_candidate_status_note: String::new(),
        no_production_deployment_public_signing_publishing_installer_update_note: String::new(),
        no_provider_trust_note: String::new(),
        no_action_authorization_note: String::new(),
    };
    validate_preparation_for_dry_package(&projection)
}

fn dry_package_payload_basis(payload: &ReleaseArtifactDryPackagePayload) -> String {
    let evidence = payload
        .included_evidence
        .iter()
        .map(|item| {
            format!(
                "{}:{}:{}:{}:{}",
                item.category.code(),
                item.category_status.code(),
                item.source_surface,
                item.source_status,
                item.source_summary
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    let boundaries = payload
        .boundary_statuses
        .iter()
        .map(|status| status.code())
        .collect::<Vec<_>>()
        .join("|");
    format!(
        "version={}|source_preparation_id={}|source_preparation_status={}|source_preparation_validation_status={}|included_evidence_count={}|included_evidence={}|boundary_statuses={}|dry_package_only_note={}|not_release_artifact_note={}|no_readiness_or_candidate_approval_note={}|local_only_non_public_note={}|no_distribution_note={}|read_back_validation_note={}",
        RELEASE_ARTIFACT_DRY_PACKAGE_VERSION,
        payload.source_preparation_id,
        payload.source_preparation_status.code(),
        payload.source_preparation_validation_status.code(),
        payload.included_evidence_count,
        evidence,
        boundaries,
        payload.dry_package_only_note,
        payload.not_release_artifact_note,
        payload.no_readiness_or_candidate_approval_note,
        payload.local_only_non_public_note,
        payload.no_distribution_note,
        payload.read_back_validation_note
    )
}

fn stable_release_artifact_dry_package_digest(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn detect_dry_package_claims(
    package: &ReleaseArtifactDryPackage,
) -> Vec<ReleaseArtifactDryPackageValidationError> {
    let text = format!(
        "{} {} {} {} {} {} {} {} {} {}",
        package.payload.source_preparation_id,
        package.payload.dry_package_only_note,
        package.payload.not_release_artifact_note,
        package.payload.no_readiness_or_candidate_approval_note,
        package.payload.local_only_non_public_note,
        package.payload.no_distribution_note,
        package.payload.read_back_validation_note,
        package
            .payload
            .included_evidence
            .iter()
            .map(|item| item.source_summary.as_str())
            .collect::<Vec<_>>()
            .join(" "),
        package
            .payload
            .included_evidence
            .iter()
            .map(|item| item.source_surface.as_str())
            .collect::<Vec<_>>()
            .join(" "),
        package
            .payload
            .included_evidence
            .iter()
            .map(|item| item.source_status.as_str())
            .collect::<Vec<_>>()
            .join(" "),
    )
    .to_ascii_lowercase();
    let mut errors = Vec::new();
    push_claim_if(
        &mut errors,
        &text,
        &["readiness approved", "readiness approval granted"],
        ReleaseArtifactDryPackageValidationError::ReadinessClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["release approved", "actual release", "release is approved"],
        ReleaseArtifactDryPackageValidationError::ReleaseClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["deployment approved", "deployment occurs"],
        ReleaseArtifactDryPackageValidationError::DeploymentClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["public use approved", "general use approved"],
        ReleaseArtifactDryPackageValidationError::PublicUseClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["production use approved", "production approved"],
        ReleaseArtifactDryPackageValidationError::ProductionUseClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["signing occurs", "artifact signed"],
        ReleaseArtifactDryPackageValidationError::SigningClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["publishing occurs", "artifact published"],
        ReleaseArtifactDryPackageValidationError::PublishingClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["installer activated", "installer generated"],
        ReleaseArtifactDryPackageValidationError::InstallerClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["update channel activated", "update channel opened"],
        ReleaseArtifactDryPackageValidationError::UpdateChannelClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["public download available", "public download occurs"],
        ReleaseArtifactDryPackageValidationError::PublicDownloadClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["github release published", "github release occurs"],
        ReleaseArtifactDryPackageValidationError::GithubReleaseClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["release tag created", "release tag occurs"],
        ReleaseArtifactDryPackageValidationError::ReleaseTagClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["provider output trusted", "provider trust granted"],
        ReleaseArtifactDryPackageValidationError::ProviderTrustClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["action authorization granted", "action authorized"],
        ReleaseArtifactDryPackageValidationError::ActionAuthorizationClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["replay repair performed", "replay repaired"],
        ReleaseArtifactDryPackageValidationError::ReplayRepairClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["recovery promotion performed", "recovery promoted"],
        ReleaseArtifactDryPackageValidationError::RecoveryPromotionClaimDetected,
    );
    errors.sort();
    errors.dedup();
    errors
}

fn push_claim_if(
    errors: &mut Vec<ReleaseArtifactDryPackageValidationError>,
    text: &str,
    needles: &[&str],
    error: ReleaseArtifactDryPackageValidationError,
) {
    if needles.iter().any(|needle| text.contains(needle)) {
        errors.push(error);
    }
}

fn parse_key_values(
    input: &str,
) -> Result<BTreeMap<String, String>, Vec<ReleaseArtifactDryPackageValidationError>> {
    let mut values = BTreeMap::new();
    for line in input.lines() {
        let Some((key, value)) = line.split_once('=') else {
            return Err(vec![
                ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput,
            ]);
        };
        if key.is_empty() || values.insert(key.to_string(), value.to_string()).is_some() {
            return Err(vec![
                ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput,
            ]);
        }
    }
    Ok(values)
}

fn get_value(
    values: &BTreeMap<String, String>,
    key: &str,
) -> Result<String, Vec<ReleaseArtifactDryPackageValidationError>> {
    values
        .get(key)
        .cloned()
        .ok_or_else(|| vec![ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput])
}

fn parse_dry_package_status(
    code: &str,
) -> Result<ReleaseArtifactDryPackageStatus, Vec<ReleaseArtifactDryPackageValidationError>> {
    match code {
        "not_assembled" => Ok(ReleaseArtifactDryPackageStatus::NotAssembled),
        "dry_package_projected" => Ok(ReleaseArtifactDryPackageStatus::DryPackageProjected),
        "dry_package_validated" => Ok(ReleaseArtifactDryPackageStatus::DryPackageValidated),
        "dry_package_written" => Ok(ReleaseArtifactDryPackageStatus::DryPackageWritten),
        "dry_package_read_back_validated" => {
            Ok(ReleaseArtifactDryPackageStatus::DryPackageReadBackValidated)
        }
        "dry_package_rejected" => Ok(ReleaseArtifactDryPackageStatus::DryPackageRejected),
        "invalid_dry_package_input" => Ok(ReleaseArtifactDryPackageStatus::InvalidDryPackageInput),
        _ => Err(vec![
            ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput,
        ]),
    }
}

fn parse_validation_status(
    code: &str,
) -> Result<ReleaseArtifactDryPackageValidationStatus, Vec<ReleaseArtifactDryPackageValidationError>>
{
    match code {
        "not_validated" => Ok(ReleaseArtifactDryPackageValidationStatus::NotValidated),
        "valid" => Ok(ReleaseArtifactDryPackageValidationStatus::Valid),
        "invalid" => Ok(ReleaseArtifactDryPackageValidationStatus::Invalid),
        _ => Err(vec![
            ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput,
        ]),
    }
}

fn parse_dry_package_classification(
    code: &str,
) -> Result<ReleaseArtifactDryPackageClassification, Vec<ReleaseArtifactDryPackageValidationError>>
{
    match code {
        "dry_run_package_only" => Ok(ReleaseArtifactDryPackageClassification::DryRunPackageOnly),
        _ => Err(vec![
            ReleaseArtifactDryPackageValidationError::InvalidDryPackageClassification,
        ]),
    }
}

fn parse_production_classification(
    code: &str,
) -> Result<
    ReleaseArtifactDryPackageProductionClassification,
    Vec<ReleaseArtifactDryPackageValidationError>,
> {
    match code {
        "non_production" => Ok(ReleaseArtifactDryPackageProductionClassification::NonProduction),
        _ => Err(vec![
            ReleaseArtifactDryPackageValidationError::InvalidProductionClassification,
        ]),
    }
}

fn parse_distribution_classification(
    code: &str,
) -> Result<
    ReleaseArtifactDryPackageDistributionClassification,
    Vec<ReleaseArtifactDryPackageValidationError>,
> {
    match code {
        "local_only_non_public" => {
            Ok(ReleaseArtifactDryPackageDistributionClassification::LocalOnlyNonPublic)
        }
        _ => Err(vec![
            ReleaseArtifactDryPackageValidationError::InvalidDistributionClassification,
        ]),
    }
}

fn parse_authority_classification(
    code: &str,
) -> Result<
    ReleaseArtifactDryPackageAuthorityClassification,
    Vec<ReleaseArtifactDryPackageValidationError>,
> {
    match code {
        "non_authoritative_rehearsal_evidence" => {
            Ok(ReleaseArtifactDryPackageAuthorityClassification::NonAuthoritativeRehearsalEvidence)
        }
        _ => Err(vec![
            ReleaseArtifactDryPackageValidationError::InvalidAuthorityClassification,
        ]),
    }
}

fn parse_release_classification(
    code: &str,
) -> Result<
    ReleaseArtifactDryPackageReleaseClassification,
    Vec<ReleaseArtifactDryPackageValidationError>,
> {
    match code {
        "release_not_approved" => {
            Ok(ReleaseArtifactDryPackageReleaseClassification::ReleaseNotApproved)
        }
        _ => Err(vec![
            ReleaseArtifactDryPackageValidationError::InvalidReleaseClassification,
        ]),
    }
}

fn parse_preparation_status(
    code: &str,
) -> Result<ReleaseCandidatePreparationStatus, Vec<ReleaseArtifactDryPackageValidationError>> {
    match code {
        "preparation_validated" => Ok(ReleaseCandidatePreparationStatus::PreparationValidated),
        "preparation_projected" => Ok(ReleaseCandidatePreparationStatus::PreparationProjected),
        "preparation_blocked" => Ok(ReleaseCandidatePreparationStatus::PreparationBlocked),
        "preparation_rejected" => Ok(ReleaseCandidatePreparationStatus::PreparationRejected),
        "invalid_preparation_input" => {
            Ok(ReleaseCandidatePreparationStatus::InvalidPreparationInput)
        }
        "not_prepared" => Ok(ReleaseCandidatePreparationStatus::NotPrepared),
        _ => Err(vec![
            ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput,
        ]),
    }
}

fn parse_preparation_validation_status(
    code: &str,
) -> Result<
    ReleaseCandidatePreparationValidationStatus,
    Vec<ReleaseArtifactDryPackageValidationError>,
> {
    match code {
        "not_validated" => Ok(ReleaseCandidatePreparationValidationStatus::NotValidated),
        "valid" => Ok(ReleaseCandidatePreparationValidationStatus::Valid),
        "invalid" => Ok(ReleaseCandidatePreparationValidationStatus::Invalid),
        _ => Err(vec![
            ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput,
        ]),
    }
}

fn parse_evidence_category(
    code: &str,
) -> Result<
    ReleaseCandidatePreparationEvidenceCategory,
    Vec<ReleaseArtifactDryPackageValidationError>,
> {
    release_candidate_preparation_evidence_categories()
        .into_iter()
        .find(|category| category.code() == code)
        .ok_or_else(|| vec![ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput])
}

fn parse_evidence_status(
    code: &str,
) -> Result<
    ReleaseCandidatePreparationEvidenceCategoryStatus,
    Vec<ReleaseArtifactDryPackageValidationError>,
> {
    match code {
        "present" => Ok(ReleaseCandidatePreparationEvidenceCategoryStatus::Present),
        "missing" => Ok(ReleaseCandidatePreparationEvidenceCategoryStatus::Missing),
        "blocked" => Ok(ReleaseCandidatePreparationEvidenceCategoryStatus::Blocked),
        "rejected" => Ok(ReleaseCandidatePreparationEvidenceCategoryStatus::Rejected),
        "deferred" => Ok(ReleaseCandidatePreparationEvidenceCategoryStatus::Deferred),
        "not_applicable" => Ok(ReleaseCandidatePreparationEvidenceCategoryStatus::NotApplicable),
        _ => Err(vec![
            ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput,
        ]),
    }
}

fn parse_boundary_status(
    code: &str,
) -> Result<ReleaseArtifactDryPackageBoundaryStatus, Vec<ReleaseArtifactDryPackageValidationError>>
{
    release_artifact_dry_package_boundary_statuses()
        .into_iter()
        .find(|status| status.code() == code)
        .ok_or_else(|| vec![ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput])
}

fn parse_boundary_statuses(
    input: &str,
) -> Result<
    Vec<ReleaseArtifactDryPackageBoundaryStatus>,
    Vec<ReleaseArtifactDryPackageValidationError>,
> {
    if input.is_empty() {
        return Err(vec![
            ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput,
        ]);
    }
    input.split(',').map(parse_boundary_status).collect()
}

fn parse_included_evidence(
    input: &str,
) -> Result<
    Vec<ReleaseArtifactDryPackageIncludedEvidence>,
    Vec<ReleaseArtifactDryPackageValidationError>,
> {
    if input.is_empty() {
        return Ok(Vec::new());
    }
    input
        .split(";;")
        .map(|record| {
            let parts = record.split('|').collect::<Vec<_>>();
            if parts.len() != 5 {
                return Err(vec![
                    ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput,
                ]);
            }
            Ok(ReleaseArtifactDryPackageIncludedEvidence {
                category: parse_evidence_category(parts[0])?,
                category_status: parse_evidence_status(parts[1])?,
                source_surface: hex_decode(parts[2])?,
                source_status: hex_decode(parts[3])?,
                source_summary: hex_decode(parts[4])?,
            })
        })
        .collect()
}

fn hex_encode(input: &str) -> String {
    input
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn hex_decode(input: &str) -> Result<String, Vec<ReleaseArtifactDryPackageValidationError>> {
    if !input.len().is_multiple_of(2) {
        return Err(vec![
            ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput,
        ]);
    }
    let mut bytes = Vec::new();
    let mut index = 0;
    while index < input.len() {
        let byte = u8::from_str_radix(&input[index..index + 2], 16).map_err(|_| {
            vec![ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput]
        })?;
        bytes.push(byte);
        index += 2;
    }
    String::from_utf8(bytes)
        .map_err(|_| vec![ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn complete_preparation() -> ReleaseCandidatePreparationProjection {
        derive_release_candidate_preparation_projection(
            &complete_release_candidate_preparation_input_snapshot(),
        )
    }

    fn dry_package() -> ReleaseArtifactDryPackage {
        derive_release_artifact_dry_package(Some(&complete_preparation())).unwrap()
    }

    #[test]
    fn deterministic_dry_package_assembly_id_and_serialization() {
        let preparation = complete_preparation();
        let first = derive_release_artifact_dry_package(Some(&preparation)).unwrap();
        let second = derive_release_artifact_dry_package(Some(&preparation)).unwrap();
        assert_eq!(first, second);
        assert_eq!(
            first.metadata.dry_package_id,
            second.metadata.dry_package_id
        );
        assert_eq!(
            first.metadata.content_digest,
            second.metadata.content_digest
        );
        assert_eq!(
            serialize_release_artifact_dry_package(&first).unwrap(),
            serialize_release_artifact_dry_package(&second).unwrap()
        );
        assert_eq!(
            first.metadata.dry_package_classification.code(),
            "dry_run_package_only"
        );
        assert_eq!(
            first.metadata.production_classification.code(),
            "non_production"
        );
        assert_eq!(
            first.metadata.distribution_classification.code(),
            "local_only_non_public"
        );
        assert_eq!(
            first.metadata.authority_classification.code(),
            "non_authoritative_rehearsal_evidence"
        );
        assert_eq!(
            first.metadata.release_classification.code(),
            "release_not_approved"
        );
        assert_eq!(
            first.payload.included_evidence_count,
            release_candidate_preparation_evidence_categories().len()
        );
    }

    #[test]
    fn missing_blocked_rejected_and_missing_evidence_preparation_reject() {
        assert_eq!(
            derive_release_artifact_dry_package(None).unwrap_err(),
            vec![ReleaseArtifactDryPackageValidationError::MissingPreparationContract]
        );

        let mut blocked = complete_release_candidate_preparation_input_snapshot();
        blocked.evidence_items[0].status =
            ReleaseCandidatePreparationEvidenceCategoryStatus::Blocked;
        let blocked_projection = derive_release_candidate_preparation_projection(&blocked);
        assert!(
            derive_release_artifact_dry_package(Some(&blocked_projection))
                .unwrap_err()
                .contains(&ReleaseArtifactDryPackageValidationError::PreparationBlocked)
        );

        let mut rejected = complete_release_candidate_preparation_input_snapshot();
        rejected.evidence_items[0].status =
            ReleaseCandidatePreparationEvidenceCategoryStatus::Rejected;
        let rejected_projection = derive_release_candidate_preparation_projection(&rejected);
        assert!(
            derive_release_artifact_dry_package(Some(&rejected_projection))
                .unwrap_err()
                .contains(&ReleaseArtifactDryPackageValidationError::PreparationRejected)
        );

        let missing_projection = initial_release_candidate_preparation_projection();
        assert!(
            derive_release_artifact_dry_package(Some(&missing_projection))
                .unwrap_err()
                .contains(
                    &ReleaseArtifactDryPackageValidationError::PreparationMissingRequiredEvidence
                )
        );
    }

    #[test]
    fn explicit_write_read_and_read_back_validation_use_caller_path() {
        let package = dry_package();
        let dir = phase_172_temp_dir("write-read");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("dry-package.txt");
        let write = write_release_artifact_dry_package(&package, &path).unwrap();
        assert_eq!(write.path, path.display().to_string());
        assert_eq!(
            write.projection.status,
            ReleaseArtifactDryPackageStatus::DryPackageWritten
        );
        let read = read_release_artifact_dry_package(&path).unwrap();
        assert_eq!(read.path, path.display().to_string());
        assert_eq!(
            read.package.metadata.dry_package_id,
            package.metadata.dry_package_id
        );
        assert_eq!(
            read.read_back.status,
            ReleaseArtifactDryPackageStatus::DryPackageReadBackValidated
        );
        assert_eq!(
            read.read_back.validation_status,
            ReleaseArtifactDryPackageValidationStatus::Valid
        );
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn malformed_and_digest_mismatch_read_back_reject() {
        let malformed = validate_release_artifact_dry_package_read_back("not a dry package");
        assert_eq!(
            malformed.validation_status,
            ReleaseArtifactDryPackageValidationStatus::Invalid
        );
        assert!(malformed
            .validation_errors
            .contains(&ReleaseArtifactDryPackageValidationError::MalformedDryPackageInput));

        let serialized = serialize_release_artifact_dry_package(&dry_package()).unwrap();
        let drifted = serialized.replacen("content_digest=", "content_digest=0000", 1);
        let read_back = validate_release_artifact_dry_package_read_back(&drifted);
        assert_eq!(
            read_back.validation_status,
            ReleaseArtifactDryPackageValidationStatus::Invalid
        );
        assert!(read_back
            .validation_errors
            .contains(&ReleaseArtifactDryPackageValidationError::DeterministicDigestMismatch));
    }

    #[test]
    fn no_authority_boundaries_and_claims_reject() {
        let package = dry_package();
        for marker in [
            ReleaseArtifactDryPackageBoundaryStatus::ReleaseArtifactNotCreated,
            ReleaseArtifactDryPackageBoundaryStatus::NoPublicDistribution,
            ReleaseArtifactDryPackageBoundaryStatus::NoSigning,
            ReleaseArtifactDryPackageBoundaryStatus::NoPublishing,
            ReleaseArtifactDryPackageBoundaryStatus::NoInstallerActivation,
            ReleaseArtifactDryPackageBoundaryStatus::NoUpdateChannelActivation,
            ReleaseArtifactDryPackageBoundaryStatus::NoDeploymentArtifact,
            ReleaseArtifactDryPackageBoundaryStatus::NoProviderTrust,
            ReleaseArtifactDryPackageBoundaryStatus::NoActionAuthorization,
            ReleaseArtifactDryPackageBoundaryStatus::NoReplayRepair,
            ReleaseArtifactDryPackageBoundaryStatus::NoRecoveryPromotion,
        ] {
            assert!(package.payload.boundary_statuses.contains(&marker));
        }

        let mut claimed = package.clone();
        claimed.payload.included_evidence[0].source_summary = "readiness approved; actual release; deployment occurs; public use approved; production use approved; signing occurs; publishing occurs; installer activated; update channel activated; public download available; github release published; release tag created; provider output trusted; action authorization granted; replay repair performed; recovery promotion performed".to_string();
        claimed.metadata.content_digest = stable_release_artifact_dry_package_digest(
            &dry_package_payload_basis(&claimed.payload),
        );
        claimed.metadata.dry_package_id = format!(
            "release-artifact-dry-package-{}",
            claimed.metadata.content_digest
        );
        let errors = validate_release_artifact_dry_package(&claimed).unwrap_err();
        for error in [
            ReleaseArtifactDryPackageValidationError::ReadinessClaimDetected,
            ReleaseArtifactDryPackageValidationError::ReleaseClaimDetected,
            ReleaseArtifactDryPackageValidationError::DeploymentClaimDetected,
            ReleaseArtifactDryPackageValidationError::PublicUseClaimDetected,
            ReleaseArtifactDryPackageValidationError::ProductionUseClaimDetected,
            ReleaseArtifactDryPackageValidationError::SigningClaimDetected,
            ReleaseArtifactDryPackageValidationError::PublishingClaimDetected,
            ReleaseArtifactDryPackageValidationError::InstallerClaimDetected,
            ReleaseArtifactDryPackageValidationError::UpdateChannelClaimDetected,
            ReleaseArtifactDryPackageValidationError::PublicDownloadClaimDetected,
            ReleaseArtifactDryPackageValidationError::GithubReleaseClaimDetected,
            ReleaseArtifactDryPackageValidationError::ReleaseTagClaimDetected,
            ReleaseArtifactDryPackageValidationError::ProviderTrustClaimDetected,
            ReleaseArtifactDryPackageValidationError::ActionAuthorizationClaimDetected,
            ReleaseArtifactDryPackageValidationError::ReplayRepairClaimDetected,
            ReleaseArtifactDryPackageValidationError::RecoveryPromotionClaimDetected,
        ] {
            assert!(errors.contains(&error), "missing {error:?}");
        }
    }

    fn phase_172_temp_dir(name: &str) -> PathBuf {
        PathBuf::from(format!("/tmp/ajentic-phase-172-{name}"))
    }
}
