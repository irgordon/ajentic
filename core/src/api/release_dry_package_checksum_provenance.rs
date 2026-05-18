//! Checksum and provenance evidence for the local release artifact dry package.
//!
//! This evidence proves dry package content identity only. It does not sign,
//! publish, release, deploy, or approve the dry package.

use super::*;
use std::collections::BTreeMap;

pub const RELEASE_DRY_PACKAGE_CHECKSUM_PROVENANCE_VERSION: &str =
    "phase-173-checksum-provenance-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseDryPackageChecksumProvenanceStatus {
    NotGenerated,
    ChecksumProvenanceProjected,
    ChecksumProvenanceValidated,
    ChecksumProvenanceWritten,
    ChecksumProvenanceReadBackValidated,
    ChecksumProvenanceRejected,
    InvalidChecksumProvenanceInput,
}

impl ReleaseDryPackageChecksumProvenanceStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotGenerated => "not_generated",
            Self::ChecksumProvenanceProjected => "checksum_provenance_projected",
            Self::ChecksumProvenanceValidated => "checksum_provenance_validated",
            Self::ChecksumProvenanceWritten => "checksum_provenance_written",
            Self::ChecksumProvenanceReadBackValidated => "checksum_provenance_read_back_validated",
            Self::ChecksumProvenanceRejected => "checksum_provenance_rejected",
            Self::InvalidChecksumProvenanceInput => "invalid_checksum_provenance_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseDryPackageChecksumProvenanceClassification {
    ChecksumEvidenceOnly,
    ProvenanceEvidenceOnly,
}

impl ReleaseDryPackageChecksumProvenanceClassification {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ChecksumEvidenceOnly => "checksum_evidence_only",
            Self::ProvenanceEvidenceOnly => "provenance_evidence_only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseDryPackageChecksumProvenanceProductionClassification {
    NonProduction,
}
impl ReleaseDryPackageChecksumProvenanceProductionClassification {
    pub fn code(&self) -> &'static str {
        "non_production"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseDryPackageChecksumProvenanceDistributionClassification {
    LocalOnlyNonPublic,
}
impl ReleaseDryPackageChecksumProvenanceDistributionClassification {
    pub fn code(&self) -> &'static str {
        "local_only_non_public"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseDryPackageChecksumProvenanceAuthorityClassification {
    NonAuthoritativeEvidence,
}
impl ReleaseDryPackageChecksumProvenanceAuthorityClassification {
    pub fn code(&self) -> &'static str {
        "non_authoritative_evidence"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseDryPackageChecksumProvenanceReleaseClassification {
    ReleaseNotApproved,
}
impl ReleaseDryPackageChecksumProvenanceReleaseClassification {
    pub fn code(&self) -> &'static str {
        "release_not_approved"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseDryPackageChecksumProvenanceValidationStatus {
    NotValidated,
    Valid,
    Invalid,
}
impl ReleaseDryPackageChecksumProvenanceValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotValidated => "not_validated",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseDryPackageChecksumProvenanceValidationError {
    MissingDryPackage,
    DryPackageNotProjected,
    DryPackageRejected,
    DryPackageMalformed,
    DryPackageDigestMismatch,
    DryPackageIdMismatch,
    MissingPreparationLinkage,
    PreparationLinkageMismatch,
    MissingChecksumValue,
    MissingProvenanceId,
    MissingProvenanceVersion,
    InvalidChecksumClassification,
    InvalidProvenanceClassification,
    InvalidProductionClassification,
    InvalidDistributionClassification,
    InvalidAuthorityClassification,
    InvalidReleaseClassification,
    MalformedChecksumProvenanceInput,
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

impl ReleaseDryPackageChecksumProvenanceValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingDryPackage => "missing_dry_package",
            Self::DryPackageNotProjected => "dry_package_not_projected",
            Self::DryPackageRejected => "dry_package_rejected",
            Self::DryPackageMalformed => "dry_package_malformed",
            Self::DryPackageDigestMismatch => "dry_package_digest_mismatch",
            Self::DryPackageIdMismatch => "dry_package_id_mismatch",
            Self::MissingPreparationLinkage => "missing_preparation_linkage",
            Self::PreparationLinkageMismatch => "preparation_linkage_mismatch",
            Self::MissingChecksumValue => "missing_checksum_value",
            Self::MissingProvenanceId => "missing_provenance_id",
            Self::MissingProvenanceVersion => "missing_provenance_version",
            Self::InvalidChecksumClassification => "invalid_checksum_classification",
            Self::InvalidProvenanceClassification => "invalid_provenance_classification",
            Self::InvalidProductionClassification => "invalid_production_classification",
            Self::InvalidDistributionClassification => "invalid_distribution_classification",
            Self::InvalidAuthorityClassification => "invalid_authority_classification",
            Self::InvalidReleaseClassification => "invalid_release_classification",
            Self::MalformedChecksumProvenanceInput => "malformed_checksum_provenance_input",
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
pub enum ReleaseDryPackageChecksumProvenanceBoundaryStatus {
    ChecksumEvidenceOnly,
    ProvenanceEvidenceOnly,
    LocalChecksumProvenanceOnly,
    NonPublicChecksumProvenance,
    NonAuthoritativeEvidence,
    NoSignatureCreated,
    NoSigning,
    NoPublishing,
    ReleaseArtifactNotCreated,
    ReleaseReadinessNotApproved,
    ReleaseCandidateStatusNotApproved,
    ProductionStatusNotApproved,
    NoPublicDistribution,
    NoPublicDownload,
    NoGithubRelease,
    NoReleaseTag,
    NoInstallerActivation,
    NoUpdateChannelActivation,
    NoDeploymentArtifact,
    NoProviderTrust,
    NoActionAuthorization,
    NoReplayRepair,
    NoRecoveryPromotion,
}

impl ReleaseDryPackageChecksumProvenanceBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ChecksumEvidenceOnly => "checksum_evidence_only",
            Self::ProvenanceEvidenceOnly => "provenance_evidence_only",
            Self::LocalChecksumProvenanceOnly => "local_checksum_provenance_only",
            Self::NonPublicChecksumProvenance => "non_public_checksum_provenance",
            Self::NonAuthoritativeEvidence => "non_authoritative_evidence",
            Self::NoSignatureCreated => "no_signature_created",
            Self::NoSigning => "no_signing",
            Self::NoPublishing => "no_publishing",
            Self::ReleaseArtifactNotCreated => "release_artifact_not_created",
            Self::ReleaseReadinessNotApproved => "release_readiness_not_approved",
            Self::ReleaseCandidateStatusNotApproved => "release_candidate_status_not_approved",
            Self::ProductionStatusNotApproved => "production_status_not_approved",
            Self::NoPublicDistribution => "no_public_distribution",
            Self::NoPublicDownload => "no_public_download",
            Self::NoGithubRelease => "no_github_release",
            Self::NoReleaseTag => "no_release_tag",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseDryPackageChecksumAlgorithm {
    DeterministicFnv64DryPackagePayload,
}
impl ReleaseDryPackageChecksumAlgorithm {
    pub fn code(&self) -> &'static str {
        "deterministic_fnv64_dry_package_payload"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDryPackageChecksumEvidence {
    pub checksum_algorithm: ReleaseDryPackageChecksumAlgorithm,
    pub checksum_value: String,
    pub checksum_classification: ReleaseDryPackageChecksumProvenanceClassification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDryPackageProvenanceLinkage {
    pub phase_171_preparation_id: String,
    pub phase_171_preparation_status: ReleaseCandidatePreparationStatus,
    pub phase_172_dry_package_id: String,
    pub phase_172_dry_package_digest: String,
    pub phase_172_dry_package_status: ReleaseArtifactDryPackageStatus,
    pub included_evidence_count: usize,
    pub included_evidence_summary: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDryPackageProvenanceEvidence {
    pub provenance_id: String,
    pub provenance_classification: ReleaseDryPackageChecksumProvenanceClassification,
    pub linkage: ReleaseDryPackageProvenanceLinkage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDryPackageChecksumProvenanceMetadata {
    pub provenance_version: String,
    pub checksum_provenance_status: ReleaseDryPackageChecksumProvenanceStatus,
    pub validation_status: ReleaseDryPackageChecksumProvenanceValidationStatus,
    pub checksum_evidence: ReleaseDryPackageChecksumEvidence,
    pub provenance_evidence: ReleaseDryPackageProvenanceEvidence,
    pub production_classification: ReleaseDryPackageChecksumProvenanceProductionClassification,
    pub distribution_classification: ReleaseDryPackageChecksumProvenanceDistributionClassification,
    pub authority_classification: ReleaseDryPackageChecksumProvenanceAuthorityClassification,
    pub release_classification: ReleaseDryPackageChecksumProvenanceReleaseClassification,
    pub provenance_content_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDryPackageChecksumProvenancePayload {
    pub dry_package_serialized_payload: String,
    pub boundary_statuses: Vec<ReleaseDryPackageChecksumProvenanceBoundaryStatus>,
    pub checksum_proves_contents_note: String,
    pub no_sign_publish_release_deploy_approve_note: String,
    pub local_only_non_public_note: String,
    pub no_distribution_note: String,
    pub read_back_validation_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDryPackageChecksumProvenanceEvidence {
    pub metadata: ReleaseDryPackageChecksumProvenanceMetadata,
    pub payload: ReleaseDryPackageChecksumProvenancePayload,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDryPackageChecksumProvenanceProjection {
    pub status: ReleaseDryPackageChecksumProvenanceStatus,
    pub dry_package_id: Option<String>,
    pub checksum_value: Option<String>,
    pub checksum_algorithm: ReleaseDryPackageChecksumAlgorithm,
    pub provenance_id: Option<String>,
    pub checksum_classification: ReleaseDryPackageChecksumProvenanceClassification,
    pub provenance_classification: ReleaseDryPackageChecksumProvenanceClassification,
    pub production_classification: ReleaseDryPackageChecksumProvenanceProductionClassification,
    pub distribution_classification: ReleaseDryPackageChecksumProvenanceDistributionClassification,
    pub authority_classification: ReleaseDryPackageChecksumProvenanceAuthorityClassification,
    pub release_classification: ReleaseDryPackageChecksumProvenanceReleaseClassification,
    pub provenance_linkage_summary: String,
    pub validation_status: ReleaseDryPackageChecksumProvenanceValidationStatus,
    pub validation_errors: Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
    pub read_back_validation_status: ReleaseDryPackageChecksumProvenanceValidationStatus,
    pub boundary_statuses: Vec<ReleaseDryPackageChecksumProvenanceBoundaryStatus>,
    pub checksum_proves_contents_note: String,
    pub no_sign_publish_release_deploy_approve_note: String,
    pub local_only_non_public_note: String,
    pub no_distribution_note: String,
    pub read_back_validation_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDryPackageChecksumProvenanceReadBackProjection {
    pub status: ReleaseDryPackageChecksumProvenanceStatus,
    pub validation_status: ReleaseDryPackageChecksumProvenanceValidationStatus,
    pub validation_errors: Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
    pub dry_package_id: Option<String>,
    pub checksum_value: Option<String>,
    pub provenance_id: Option<String>,
    pub provenance_linkage_summary: String,
    pub read_back_validation_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDryPackageChecksumProvenanceWriteResult {
    pub path: String,
    pub projection: ReleaseDryPackageChecksumProvenanceProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseDryPackageChecksumProvenanceReadResult {
    pub path: String,
    pub evidence: ReleaseDryPackageChecksumProvenanceEvidence,
    pub read_back: ReleaseDryPackageChecksumProvenanceReadBackProjection,
}

pub fn release_dry_package_checksum_provenance_boundary_statuses(
) -> Vec<ReleaseDryPackageChecksumProvenanceBoundaryStatus> {
    vec![
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::ChecksumEvidenceOnly,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::ProvenanceEvidenceOnly,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::LocalChecksumProvenanceOnly,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NonPublicChecksumProvenance,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NonAuthoritativeEvidence,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoSignatureCreated,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoSigning,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoPublishing,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::ReleaseArtifactNotCreated,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::ReleaseReadinessNotApproved,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::ReleaseCandidateStatusNotApproved,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::ProductionStatusNotApproved,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoPublicDistribution,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoPublicDownload,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoGithubRelease,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoReleaseTag,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoInstallerActivation,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoUpdateChannelActivation,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoDeploymentArtifact,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoProviderTrust,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoActionAuthorization,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoReplayRepair,
        ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoRecoveryPromotion,
    ]
}

pub fn initial_release_dry_package_checksum_provenance_projection(
) -> ReleaseDryPackageChecksumProvenanceProjection {
    ReleaseDryPackageChecksumProvenanceProjection {
        status: ReleaseDryPackageChecksumProvenanceStatus::NotGenerated,
        dry_package_id: None,
        checksum_value: None,
        checksum_algorithm: ReleaseDryPackageChecksumAlgorithm::DeterministicFnv64DryPackagePayload,
        provenance_id: None,
        checksum_classification:
            ReleaseDryPackageChecksumProvenanceClassification::ChecksumEvidenceOnly,
        provenance_classification:
            ReleaseDryPackageChecksumProvenanceClassification::ProvenanceEvidenceOnly,
        production_classification:
            ReleaseDryPackageChecksumProvenanceProductionClassification::NonProduction,
        distribution_classification:
            ReleaseDryPackageChecksumProvenanceDistributionClassification::LocalOnlyNonPublic,
        authority_classification:
            ReleaseDryPackageChecksumProvenanceAuthorityClassification::NonAuthoritativeEvidence,
        release_classification:
            ReleaseDryPackageChecksumProvenanceReleaseClassification::ReleaseNotApproved,
        provenance_linkage_summary: "No dry package checksum/provenance generated.".to_string(),
        validation_status: ReleaseDryPackageChecksumProvenanceValidationStatus::NotValidated,
        validation_errors: Vec::new(),
        read_back_validation_status: ReleaseDryPackageChecksumProvenanceValidationStatus::NotValidated,
        boundary_statuses: release_dry_package_checksum_provenance_boundary_statuses(),
        checksum_proves_contents_note:
            "Checksum and provenance evidence proves what the dry package contains.".to_string(),
        no_sign_publish_release_deploy_approve_note:
            "This evidence does not sign, publish, release, deploy, or approve the dry package."
                .to_string(),
        local_only_non_public_note: "This evidence is local-only and non-public.".to_string(),
        no_distribution_note: "No signing, publishing, installer, update-channel, public download, GitHub release, release tag, deployment, or public distribution occurs.".to_string(),
        read_back_validation_note:
            "Read-back validation checks checksum/provenance structure only.".to_string(),
    }
}

pub fn derive_release_dry_package_checksum_provenance(
    dry_package: Option<&ReleaseArtifactDryPackage>,
) -> Result<
    ReleaseDryPackageChecksumProvenanceEvidence,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    let dry_package = dry_package.ok_or_else(|| {
        vec![ReleaseDryPackageChecksumProvenanceValidationError::MissingDryPackage]
    })?;
    validate_dry_package_input(dry_package)?;
    let dry_package_serialized_payload =
        serialize_release_artifact_dry_package(dry_package).map_err(map_dry_package_errors)?;
    let checksum_value = stable_release_dry_package_checksum(&dry_package_serialized_payload);
    let included_evidence_summary = dry_package
        .payload
        .included_evidence
        .iter()
        .map(|item| {
            format!(
                "{}:{}:{}:{}",
                item.category.code(),
                item.category_status.code(),
                item.source_surface,
                item.source_status
            )
        })
        .collect::<Vec<_>>();
    let linkage = ReleaseDryPackageProvenanceLinkage {
        phase_171_preparation_id: dry_package.payload.source_preparation_id.clone(),
        phase_171_preparation_status: dry_package.payload.source_preparation_status,
        phase_172_dry_package_id: dry_package.metadata.dry_package_id.clone(),
        phase_172_dry_package_digest: dry_package.metadata.content_digest.clone(),
        phase_172_dry_package_status: dry_package.metadata.dry_package_status,
        included_evidence_count: dry_package.payload.included_evidence_count,
        included_evidence_summary,
    };
    let payload = ReleaseDryPackageChecksumProvenancePayload {
        dry_package_serialized_payload,
        boundary_statuses: release_dry_package_checksum_provenance_boundary_statuses(),
        checksum_proves_contents_note:
            "Checksum and provenance evidence proves what the dry package contains.".to_string(),
        no_sign_publish_release_deploy_approve_note:
            "This evidence does not sign, publish, release, deploy, or approve the dry package."
                .to_string(),
        local_only_non_public_note: "This evidence is local-only and non-public.".to_string(),
        no_distribution_note: "No signing, publishing, installer, update-channel, public download, GitHub release, release tag, deployment, or public distribution occurs.".to_string(),
        read_back_validation_note:
            "Read-back validation checks checksum/provenance structure only.".to_string(),
    };
    let provenance_payload_basis = provenance_payload_basis(&checksum_value, &linkage, &payload);
    let provenance_content_digest = stable_release_dry_package_checksum(&provenance_payload_basis);
    let provenance_id = format!("release-dry-package-provenance-{provenance_content_digest}");
    let metadata = ReleaseDryPackageChecksumProvenanceMetadata {
        provenance_version: RELEASE_DRY_PACKAGE_CHECKSUM_PROVENANCE_VERSION.to_string(),
        checksum_provenance_status:
            ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceProjected,
        validation_status: ReleaseDryPackageChecksumProvenanceValidationStatus::NotValidated,
        checksum_evidence: ReleaseDryPackageChecksumEvidence {
            checksum_algorithm:
                ReleaseDryPackageChecksumAlgorithm::DeterministicFnv64DryPackagePayload,
            checksum_value,
            checksum_classification:
                ReleaseDryPackageChecksumProvenanceClassification::ChecksumEvidenceOnly,
        },
        provenance_evidence: ReleaseDryPackageProvenanceEvidence {
            provenance_id,
            provenance_classification:
                ReleaseDryPackageChecksumProvenanceClassification::ProvenanceEvidenceOnly,
            linkage,
        },
        production_classification:
            ReleaseDryPackageChecksumProvenanceProductionClassification::NonProduction,
        distribution_classification:
            ReleaseDryPackageChecksumProvenanceDistributionClassification::LocalOnlyNonPublic,
        authority_classification:
            ReleaseDryPackageChecksumProvenanceAuthorityClassification::NonAuthoritativeEvidence,
        release_classification:
            ReleaseDryPackageChecksumProvenanceReleaseClassification::ReleaseNotApproved,
        provenance_content_digest,
    };
    let mut evidence = ReleaseDryPackageChecksumProvenanceEvidence { metadata, payload };
    validate_release_dry_package_checksum_provenance(&evidence)?;
    evidence.metadata.checksum_provenance_status =
        ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceValidated;
    evidence.metadata.validation_status =
        ReleaseDryPackageChecksumProvenanceValidationStatus::Valid;
    Ok(evidence)
}

pub fn project_release_dry_package_checksum_provenance(
    dry_package: Option<&ReleaseArtifactDryPackage>,
) -> ReleaseDryPackageChecksumProvenanceProjection {
    match derive_release_dry_package_checksum_provenance(dry_package) {
        Ok(evidence) => checksum_provenance_projection_from_evidence(
            &evidence,
            ReleaseDryPackageChecksumProvenanceValidationStatus::NotValidated,
            Vec::new(),
        ),
        Err(errors) => {
            let mut projection = initial_release_dry_package_checksum_provenance_projection();
            projection.status =
                ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceRejected;
            projection.validation_status =
                ReleaseDryPackageChecksumProvenanceValidationStatus::Invalid;
            projection.validation_errors = errors;
            projection
        }
    }
}

pub fn validate_release_dry_package_checksum_provenance(
    evidence: &ReleaseDryPackageChecksumProvenanceEvidence,
) -> Result<(), Vec<ReleaseDryPackageChecksumProvenanceValidationError>> {
    let mut errors = Vec::new();
    if evidence.metadata.provenance_version != RELEASE_DRY_PACKAGE_CHECKSUM_PROVENANCE_VERSION {
        errors.push(ReleaseDryPackageChecksumProvenanceValidationError::MissingProvenanceVersion);
    }
    if evidence
        .metadata
        .checksum_evidence
        .checksum_value
        .is_empty()
    {
        errors.push(ReleaseDryPackageChecksumProvenanceValidationError::MissingChecksumValue);
    }
    if evidence
        .metadata
        .provenance_evidence
        .provenance_id
        .is_empty()
    {
        errors.push(ReleaseDryPackageChecksumProvenanceValidationError::MissingProvenanceId);
    }
    if evidence.metadata.checksum_evidence.checksum_classification
        != ReleaseDryPackageChecksumProvenanceClassification::ChecksumEvidenceOnly
    {
        errors.push(
            ReleaseDryPackageChecksumProvenanceValidationError::InvalidChecksumClassification,
        );
    }
    if evidence
        .metadata
        .provenance_evidence
        .provenance_classification
        != ReleaseDryPackageChecksumProvenanceClassification::ProvenanceEvidenceOnly
    {
        errors.push(
            ReleaseDryPackageChecksumProvenanceValidationError::InvalidProvenanceClassification,
        );
    }
    if evidence.metadata.production_classification
        != ReleaseDryPackageChecksumProvenanceProductionClassification::NonProduction
    {
        errors.push(
            ReleaseDryPackageChecksumProvenanceValidationError::InvalidProductionClassification,
        );
    }
    if evidence.metadata.distribution_classification
        != ReleaseDryPackageChecksumProvenanceDistributionClassification::LocalOnlyNonPublic
    {
        errors.push(
            ReleaseDryPackageChecksumProvenanceValidationError::InvalidDistributionClassification,
        );
    }
    if evidence.metadata.authority_classification
        != ReleaseDryPackageChecksumProvenanceAuthorityClassification::NonAuthoritativeEvidence
    {
        errors.push(
            ReleaseDryPackageChecksumProvenanceValidationError::InvalidAuthorityClassification,
        );
    }
    if evidence.metadata.release_classification
        != ReleaseDryPackageChecksumProvenanceReleaseClassification::ReleaseNotApproved
    {
        errors
            .push(ReleaseDryPackageChecksumProvenanceValidationError::InvalidReleaseClassification);
    }
    let dry_package = match parse_release_artifact_dry_package(
        &evidence.payload.dry_package_serialized_payload,
    ) {
        Ok(package) => Some(package),
        Err(dry_errors) => {
            errors.extend(map_dry_package_errors(dry_errors));
            None
        }
    };
    if let Some(dry_package) = dry_package.as_ref() {
        errors.extend(
            validate_dry_package_input(dry_package)
                .err()
                .unwrap_or_default(),
        );
        let expected_checksum =
            stable_release_dry_package_checksum(&evidence.payload.dry_package_serialized_payload);
        if evidence.metadata.checksum_evidence.checksum_value != expected_checksum {
            errors.push(
                ReleaseDryPackageChecksumProvenanceValidationError::DeterministicDigestMismatch,
            );
        }
        errors.extend(validate_provenance_linkage(evidence, dry_package));
    }
    let expected_digest = stable_release_dry_package_checksum(&provenance_payload_basis(
        &evidence.metadata.checksum_evidence.checksum_value,
        &evidence.metadata.provenance_evidence.linkage,
        &evidence.payload,
    ));
    if evidence.metadata.provenance_content_digest != expected_digest
        || evidence.metadata.provenance_evidence.provenance_id
            != format!("release-dry-package-provenance-{expected_digest}")
    {
        errors
            .push(ReleaseDryPackageChecksumProvenanceValidationError::DeterministicDigestMismatch);
    }
    errors.extend(detect_checksum_provenance_claims(evidence));
    errors.sort();
    errors.dedup();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn serialize_release_dry_package_checksum_provenance(
    evidence: &ReleaseDryPackageChecksumProvenanceEvidence,
) -> Result<String, Vec<ReleaseDryPackageChecksumProvenanceValidationError>> {
    validate_release_dry_package_checksum_provenance(evidence)?;
    let linkage = &evidence.metadata.provenance_evidence.linkage;
    let summary = linkage
        .included_evidence_summary
        .iter()
        .map(|item| hex_encode(item))
        .collect::<Vec<_>>()
        .join(";;");
    let boundaries = evidence
        .payload
        .boundary_statuses
        .iter()
        .map(|status| status.code())
        .collect::<Vec<_>>()
        .join(",");
    let lines = vec![
        "ajentic_release_dry_package_checksum_provenance=v1".to_string(),
        format!(
            "provenance_version={}",
            evidence.metadata.provenance_version
        ),
        format!(
            "checksum_provenance_status={}",
            evidence.metadata.checksum_provenance_status.code()
        ),
        format!(
            "validation_status={}",
            evidence.metadata.validation_status.code()
        ),
        format!(
            "checksum_algorithm={}",
            evidence
                .metadata
                .checksum_evidence
                .checksum_algorithm
                .code()
        ),
        format!(
            "checksum_value={}",
            evidence.metadata.checksum_evidence.checksum_value
        ),
        format!(
            "checksum_classification={}",
            evidence
                .metadata
                .checksum_evidence
                .checksum_classification
                .code()
        ),
        format!(
            "provenance_id={}",
            evidence.metadata.provenance_evidence.provenance_id
        ),
        format!(
            "provenance_classification={}",
            evidence
                .metadata
                .provenance_evidence
                .provenance_classification
                .code()
        ),
        format!(
            "production_classification={}",
            evidence.metadata.production_classification.code()
        ),
        format!(
            "distribution_classification={}",
            evidence.metadata.distribution_classification.code()
        ),
        format!(
            "authority_classification={}",
            evidence.metadata.authority_classification.code()
        ),
        format!(
            "release_classification={}",
            evidence.metadata.release_classification.code()
        ),
        format!(
            "provenance_content_digest={}",
            evidence.metadata.provenance_content_digest
        ),
        format!(
            "phase_171_preparation_id={}",
            linkage.phase_171_preparation_id
        ),
        format!(
            "phase_171_preparation_status={}",
            linkage.phase_171_preparation_status.code()
        ),
        format!(
            "phase_172_dry_package_id={}",
            linkage.phase_172_dry_package_id
        ),
        format!(
            "phase_172_dry_package_digest={}",
            linkage.phase_172_dry_package_digest
        ),
        format!(
            "phase_172_dry_package_status={}",
            linkage.phase_172_dry_package_status.code()
        ),
        format!(
            "included_evidence_count={}",
            linkage.included_evidence_count
        ),
        format!("included_evidence_summary={summary}"),
        format!(
            "dry_package_serialized_payload={}",
            hex_encode(&evidence.payload.dry_package_serialized_payload)
        ),
        format!("boundary_statuses={boundaries}"),
        format!(
            "checksum_proves_contents_note={}",
            hex_encode(&evidence.payload.checksum_proves_contents_note)
        ),
        format!(
            "no_sign_publish_release_deploy_approve_note={}",
            hex_encode(&evidence.payload.no_sign_publish_release_deploy_approve_note)
        ),
        format!(
            "local_only_non_public_note={}",
            hex_encode(&evidence.payload.local_only_non_public_note)
        ),
        format!(
            "no_distribution_note={}",
            hex_encode(&evidence.payload.no_distribution_note)
        ),
        format!(
            "read_back_validation_note={}",
            hex_encode(&evidence.payload.read_back_validation_note)
        ),
    ];
    Ok(format!("{}\n", lines.join("\n")))
}

pub fn parse_release_dry_package_checksum_provenance(
    input: &str,
) -> Result<
    ReleaseDryPackageChecksumProvenanceEvidence,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    let values = parse_key_values(input)?;
    if values
        .get("ajentic_release_dry_package_checksum_provenance")
        .map(String::as_str)
        != Some("v1")
    {
        return Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput,
        ]);
    }
    let linkage = ReleaseDryPackageProvenanceLinkage {
        phase_171_preparation_id: get_value(&values, "phase_171_preparation_id")?,
        phase_171_preparation_status: parse_preparation_status(&get_value(&values, "phase_171_preparation_status")?)?,
        phase_172_dry_package_id: get_value(&values, "phase_172_dry_package_id")?,
        phase_172_dry_package_digest: get_value(&values, "phase_172_dry_package_digest")?,
        phase_172_dry_package_status: parse_dry_package_status(&get_value(&values, "phase_172_dry_package_status")?)?,
        included_evidence_count: get_value(&values, "included_evidence_count")?.parse::<usize>().map_err(|_| vec![ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput])?,
        included_evidence_summary: parse_summary(&get_value(&values, "included_evidence_summary")?)?,
    };
    let evidence = ReleaseDryPackageChecksumProvenanceEvidence {
        metadata: ReleaseDryPackageChecksumProvenanceMetadata {
            provenance_version: get_value(&values, "provenance_version")?,
            checksum_provenance_status: parse_status(&get_value(
                &values,
                "checksum_provenance_status",
            )?)?,
            validation_status: parse_validation_status(&get_value(&values, "validation_status")?)?,
            checksum_evidence: ReleaseDryPackageChecksumEvidence {
                checksum_algorithm: parse_checksum_algorithm(&get_value(
                    &values,
                    "checksum_algorithm",
                )?)?,
                checksum_value: get_value(&values, "checksum_value")?,
                checksum_classification: parse_checksum_classification(&get_value(
                    &values,
                    "checksum_classification",
                )?)?,
            },
            provenance_evidence: ReleaseDryPackageProvenanceEvidence {
                provenance_id: get_value(&values, "provenance_id")?,
                provenance_classification: parse_provenance_classification(&get_value(
                    &values,
                    "provenance_classification",
                )?)?,
                linkage,
            },
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
            provenance_content_digest: get_value(&values, "provenance_content_digest")?,
        },
        payload: ReleaseDryPackageChecksumProvenancePayload {
            dry_package_serialized_payload: hex_decode(&get_value(
                &values,
                "dry_package_serialized_payload",
            )?)?,
            boundary_statuses: parse_boundary_statuses(&get_value(&values, "boundary_statuses")?)?,
            checksum_proves_contents_note: hex_decode(&get_value(
                &values,
                "checksum_proves_contents_note",
            )?)?,
            no_sign_publish_release_deploy_approve_note: hex_decode(&get_value(
                &values,
                "no_sign_publish_release_deploy_approve_note",
            )?)?,
            local_only_non_public_note: hex_decode(&get_value(
                &values,
                "local_only_non_public_note",
            )?)?,
            no_distribution_note: hex_decode(&get_value(&values, "no_distribution_note")?)?,
            read_back_validation_note: hex_decode(&get_value(
                &values,
                "read_back_validation_note",
            )?)?,
        },
    };
    validate_release_dry_package_checksum_provenance(&evidence)?;
    Ok(evidence)
}

pub fn validate_release_dry_package_checksum_provenance_read_back(
    input: &str,
) -> ReleaseDryPackageChecksumProvenanceReadBackProjection {
    match parse_release_dry_package_checksum_provenance(input) {
        Ok(evidence) => ReleaseDryPackageChecksumProvenanceReadBackProjection {
            status: ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceReadBackValidated,
            validation_status: ReleaseDryPackageChecksumProvenanceValidationStatus::Valid,
            validation_errors: Vec::new(),
            dry_package_id: Some(
                evidence
                    .metadata
                    .provenance_evidence
                    .linkage
                    .phase_172_dry_package_id
                    .clone(),
            ),
            checksum_value: Some(evidence.metadata.checksum_evidence.checksum_value.clone()),
            provenance_id: Some(evidence.metadata.provenance_evidence.provenance_id.clone()),
            provenance_linkage_summary: release_dry_package_provenance_linkage_summary(
                &evidence.metadata.provenance_evidence.linkage,
            ),
            read_back_validation_note: evidence.payload.read_back_validation_note,
        },
        Err(errors) => ReleaseDryPackageChecksumProvenanceReadBackProjection {
            status: ReleaseDryPackageChecksumProvenanceStatus::InvalidChecksumProvenanceInput,
            validation_status: ReleaseDryPackageChecksumProvenanceValidationStatus::Invalid,
            validation_errors: errors,
            dry_package_id: None,
            checksum_value: None,
            provenance_id: None,
            provenance_linkage_summary: "Checksum/provenance read-back rejected.".to_string(),
            read_back_validation_note:
                "Read-back validation checks checksum/provenance structure only.".to_string(),
        },
    }
}

pub fn release_dry_package_provenance_linkage_summary(
    linkage: &ReleaseDryPackageProvenanceLinkage,
) -> String {
    format!(
        "Phase 171 preparation {} ({}) -> Phase 172 dry package {} digest {} with {} included evidence items",
        linkage.phase_171_preparation_id,
        linkage.phase_171_preparation_status.code(),
        linkage.phase_172_dry_package_id,
        linkage.phase_172_dry_package_digest,
        linkage.included_evidence_count
    )
}

pub(crate) fn checksum_provenance_projection_from_evidence(
    evidence: &ReleaseDryPackageChecksumProvenanceEvidence,
    read_back_validation_status: ReleaseDryPackageChecksumProvenanceValidationStatus,
    validation_errors: Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
) -> ReleaseDryPackageChecksumProvenanceProjection {
    ReleaseDryPackageChecksumProvenanceProjection {
        status: evidence.metadata.checksum_provenance_status,
        dry_package_id: Some(
            evidence
                .metadata
                .provenance_evidence
                .linkage
                .phase_172_dry_package_id
                .clone(),
        ),
        checksum_value: Some(evidence.metadata.checksum_evidence.checksum_value.clone()),
        checksum_algorithm: evidence.metadata.checksum_evidence.checksum_algorithm,
        provenance_id: Some(evidence.metadata.provenance_evidence.provenance_id.clone()),
        checksum_classification: evidence.metadata.checksum_evidence.checksum_classification,
        provenance_classification: evidence
            .metadata
            .provenance_evidence
            .provenance_classification,
        production_classification: evidence.metadata.production_classification,
        distribution_classification: evidence.metadata.distribution_classification,
        authority_classification: evidence.metadata.authority_classification,
        release_classification: evidence.metadata.release_classification,
        provenance_linkage_summary: release_dry_package_provenance_linkage_summary(
            &evidence.metadata.provenance_evidence.linkage,
        ),
        validation_status: evidence.metadata.validation_status,
        validation_errors,
        read_back_validation_status,
        boundary_statuses: evidence.payload.boundary_statuses.clone(),
        checksum_proves_contents_note: evidence.payload.checksum_proves_contents_note.clone(),
        no_sign_publish_release_deploy_approve_note: evidence
            .payload
            .no_sign_publish_release_deploy_approve_note
            .clone(),
        local_only_non_public_note: evidence.payload.local_only_non_public_note.clone(),
        no_distribution_note: evidence.payload.no_distribution_note.clone(),
        read_back_validation_note: evidence.payload.read_back_validation_note.clone(),
    }
}

fn validate_dry_package_input(
    dry_package: &ReleaseArtifactDryPackage,
) -> Result<(), Vec<ReleaseDryPackageChecksumProvenanceValidationError>> {
    let mut errors = Vec::new();
    if dry_package.metadata.dry_package_id.is_empty() {
        errors.push(ReleaseDryPackageChecksumProvenanceValidationError::DryPackageIdMismatch);
    }
    if dry_package.metadata.dry_package_status == ReleaseArtifactDryPackageStatus::NotAssembled {
        errors.push(ReleaseDryPackageChecksumProvenanceValidationError::DryPackageNotProjected);
    }
    if dry_package.metadata.dry_package_status
        == ReleaseArtifactDryPackageStatus::DryPackageRejected
        || dry_package.metadata.validation_status
            == ReleaseArtifactDryPackageValidationStatus::Invalid
    {
        errors.push(ReleaseDryPackageChecksumProvenanceValidationError::DryPackageRejected);
    }
    if dry_package.payload.source_preparation_id.is_empty()
        || dry_package.payload.included_evidence.is_empty()
    {
        errors.push(ReleaseDryPackageChecksumProvenanceValidationError::MissingPreparationLinkage);
    }
    if let Err(dry_errors) = validate_release_artifact_dry_package(dry_package) {
        errors.extend(map_dry_package_errors(dry_errors));
    }
    errors.sort();
    errors.dedup();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_provenance_linkage(
    evidence: &ReleaseDryPackageChecksumProvenanceEvidence,
    dry_package: &ReleaseArtifactDryPackage,
) -> Vec<ReleaseDryPackageChecksumProvenanceValidationError> {
    let linkage = &evidence.metadata.provenance_evidence.linkage;
    let mut errors = Vec::new();
    if linkage.phase_171_preparation_id.is_empty() {
        errors.push(ReleaseDryPackageChecksumProvenanceValidationError::MissingPreparationLinkage);
    }
    if linkage.phase_171_preparation_id != dry_package.payload.source_preparation_id
        || linkage.phase_171_preparation_status != dry_package.payload.source_preparation_status
        || linkage.included_evidence_count != dry_package.payload.included_evidence_count
    {
        errors.push(ReleaseDryPackageChecksumProvenanceValidationError::PreparationLinkageMismatch);
    }
    if linkage.phase_172_dry_package_id != dry_package.metadata.dry_package_id {
        errors.push(ReleaseDryPackageChecksumProvenanceValidationError::DryPackageIdMismatch);
    }
    if linkage.phase_172_dry_package_digest != dry_package.metadata.content_digest {
        errors.push(ReleaseDryPackageChecksumProvenanceValidationError::DryPackageDigestMismatch);
    }
    errors
}

fn map_dry_package_errors(
    errors: Vec<ReleaseArtifactDryPackageValidationError>,
) -> Vec<ReleaseDryPackageChecksumProvenanceValidationError> {
    errors
        .into_iter()
        .map(|error| match error {
            ReleaseArtifactDryPackageValidationError::MissingPreparationContract
            | ReleaseArtifactDryPackageValidationError::PreparationNotProjected
            | ReleaseArtifactDryPackageValidationError::PreparationMissingRequiredEvidence => {
                ReleaseDryPackageChecksumProvenanceValidationError::MissingPreparationLinkage
            }
            ReleaseArtifactDryPackageValidationError::PreparationBlocked
            | ReleaseArtifactDryPackageValidationError::PreparationRejected => {
                ReleaseDryPackageChecksumProvenanceValidationError::DryPackageRejected
            }
            ReleaseArtifactDryPackageValidationError::MissingDryPackageId => {
                ReleaseDryPackageChecksumProvenanceValidationError::DryPackageIdMismatch
            }
            ReleaseArtifactDryPackageValidationError::DeterministicDigestMismatch => {
                ReleaseDryPackageChecksumProvenanceValidationError::DryPackageDigestMismatch
            }
            ReleaseArtifactDryPackageValidationError::ReadinessClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::ReadinessClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::ReleaseClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::ReleaseClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::DeploymentClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::DeploymentClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::PublicUseClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::PublicUseClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::ProductionUseClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::ProductionUseClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::SigningClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::SigningClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::PublishingClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::PublishingClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::InstallerClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::InstallerClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::UpdateChannelClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::UpdateChannelClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::PublicDownloadClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::PublicDownloadClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::GithubReleaseClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::GithubReleaseClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::ReleaseTagClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::ReleaseTagClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::ProviderTrustClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::ProviderTrustClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::ActionAuthorizationClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::ActionAuthorizationClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::ReplayRepairClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::ReplayRepairClaimDetected
            }
            ReleaseArtifactDryPackageValidationError::RecoveryPromotionClaimDetected => {
                ReleaseDryPackageChecksumProvenanceValidationError::RecoveryPromotionClaimDetected
            }
            _ => ReleaseDryPackageChecksumProvenanceValidationError::DryPackageMalformed,
        })
        .collect()
}

fn provenance_payload_basis(
    checksum_value: &str,
    linkage: &ReleaseDryPackageProvenanceLinkage,
    payload: &ReleaseDryPackageChecksumProvenancePayload,
) -> String {
    format!(
        "version={}|checksum={}|phase171={}|phase171_status={}|dry_package_id={}|dry_package_digest={}|dry_package_status={}|included_count={}|included={}|boundaries={}|dry_payload={}|notes={}|{}|{}|{}|{}",
        RELEASE_DRY_PACKAGE_CHECKSUM_PROVENANCE_VERSION,
        checksum_value,
        linkage.phase_171_preparation_id,
        linkage.phase_171_preparation_status.code(),
        linkage.phase_172_dry_package_id,
        linkage.phase_172_dry_package_digest,
        linkage.phase_172_dry_package_status.code(),
        linkage.included_evidence_count,
        linkage.included_evidence_summary.join(";;"),
        payload.boundary_statuses.iter().map(|status| status.code()).collect::<Vec<_>>().join(","),
        payload.dry_package_serialized_payload,
        payload.checksum_proves_contents_note,
        payload.no_sign_publish_release_deploy_approve_note,
        payload.local_only_non_public_note,
        payload.no_distribution_note,
        payload.read_back_validation_note,
    )
}

fn stable_release_dry_package_checksum(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn detect_checksum_provenance_claims(
    evidence: &ReleaseDryPackageChecksumProvenanceEvidence,
) -> Vec<ReleaseDryPackageChecksumProvenanceValidationError> {
    let text = format!(
        "{} {} {} {} {} {} {} {}",
        evidence.metadata.provenance_evidence.provenance_id,
        evidence
            .metadata
            .provenance_evidence
            .linkage
            .phase_171_preparation_id,
        evidence
            .metadata
            .provenance_evidence
            .linkage
            .phase_172_dry_package_id,
        evidence.payload.checksum_proves_contents_note,
        evidence.payload.no_sign_publish_release_deploy_approve_note,
        evidence.payload.local_only_non_public_note,
        evidence.payload.no_distribution_note,
        evidence.payload.read_back_validation_note,
    )
    .to_ascii_lowercase();
    let mut errors = Vec::new();
    push_claim_if(
        &mut errors,
        &text,
        &["readiness approved", "readiness approval granted"],
        ReleaseDryPackageChecksumProvenanceValidationError::ReadinessClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["release approved", "actual release", "release is approved"],
        ReleaseDryPackageChecksumProvenanceValidationError::ReleaseClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["deployment approved", "deployment occurs"],
        ReleaseDryPackageChecksumProvenanceValidationError::DeploymentClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["public use approved", "general use approved"],
        ReleaseDryPackageChecksumProvenanceValidationError::PublicUseClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["production use approved", "production approved"],
        ReleaseDryPackageChecksumProvenanceValidationError::ProductionUseClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["signing occurs", "artifact signed"],
        ReleaseDryPackageChecksumProvenanceValidationError::SigningClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["publishing occurs", "artifact published"],
        ReleaseDryPackageChecksumProvenanceValidationError::PublishingClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["installer activated", "installer generated"],
        ReleaseDryPackageChecksumProvenanceValidationError::InstallerClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["update channel activated", "update channel opened"],
        ReleaseDryPackageChecksumProvenanceValidationError::UpdateChannelClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["public download available", "public download occurs"],
        ReleaseDryPackageChecksumProvenanceValidationError::PublicDownloadClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["github release published", "github release occurs"],
        ReleaseDryPackageChecksumProvenanceValidationError::GithubReleaseClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["release tag created", "release tag occurs"],
        ReleaseDryPackageChecksumProvenanceValidationError::ReleaseTagClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["provider output trusted", "provider trust granted"],
        ReleaseDryPackageChecksumProvenanceValidationError::ProviderTrustClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["action authorization granted", "action authorized"],
        ReleaseDryPackageChecksumProvenanceValidationError::ActionAuthorizationClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["replay repair performed", "replay repaired"],
        ReleaseDryPackageChecksumProvenanceValidationError::ReplayRepairClaimDetected,
    );
    push_claim_if(
        &mut errors,
        &text,
        &["recovery promotion performed", "recovery promoted"],
        ReleaseDryPackageChecksumProvenanceValidationError::RecoveryPromotionClaimDetected,
    );
    errors.sort();
    errors.dedup();
    errors
}

fn push_claim_if(
    errors: &mut Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
    text: &str,
    needles: &[&str],
    error: ReleaseDryPackageChecksumProvenanceValidationError,
) {
    if needles.iter().any(|needle| text.contains(needle)) {
        errors.push(error);
    }
}

fn parse_key_values(
    input: &str,
) -> Result<BTreeMap<String, String>, Vec<ReleaseDryPackageChecksumProvenanceValidationError>> {
    let mut values = BTreeMap::new();
    for line in input.lines() {
        let Some((key, value)) = line.split_once('=') else {
            return Err(vec![ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput]);
        };
        if key.is_empty() || values.insert(key.to_string(), value.to_string()).is_some() {
            return Err(vec![ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput]);
        }
    }
    Ok(values)
}

fn get_value(
    values: &BTreeMap<String, String>,
    key: &str,
) -> Result<String, Vec<ReleaseDryPackageChecksumProvenanceValidationError>> {
    values.get(key).cloned().ok_or_else(|| {
        vec![ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput]
    })
}

fn parse_status(
    code: &str,
) -> Result<
    ReleaseDryPackageChecksumProvenanceStatus,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    match code {
        "not_generated" => Ok(ReleaseDryPackageChecksumProvenanceStatus::NotGenerated),
        "checksum_provenance_projected" => {
            Ok(ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceProjected)
        }
        "checksum_provenance_validated" => {
            Ok(ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceValidated)
        }
        "checksum_provenance_written" => {
            Ok(ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceWritten)
        }
        "checksum_provenance_read_back_validated" => {
            Ok(ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceReadBackValidated)
        }
        "checksum_provenance_rejected" => {
            Ok(ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceRejected)
        }
        "invalid_checksum_provenance_input" => {
            Ok(ReleaseDryPackageChecksumProvenanceStatus::InvalidChecksumProvenanceInput)
        }
        _ => Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput,
        ]),
    }
}

fn parse_validation_status(
    code: &str,
) -> Result<
    ReleaseDryPackageChecksumProvenanceValidationStatus,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    match code {
        "not_validated" => Ok(ReleaseDryPackageChecksumProvenanceValidationStatus::NotValidated),
        "valid" => Ok(ReleaseDryPackageChecksumProvenanceValidationStatus::Valid),
        "invalid" => Ok(ReleaseDryPackageChecksumProvenanceValidationStatus::Invalid),
        _ => Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput,
        ]),
    }
}

fn parse_checksum_algorithm(
    code: &str,
) -> Result<
    ReleaseDryPackageChecksumAlgorithm,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    match code {
        "deterministic_fnv64_dry_package_payload" => {
            Ok(ReleaseDryPackageChecksumAlgorithm::DeterministicFnv64DryPackagePayload)
        }
        _ => Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput,
        ]),
    }
}

fn parse_checksum_classification(
    code: &str,
) -> Result<
    ReleaseDryPackageChecksumProvenanceClassification,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    match code {
        "checksum_evidence_only" => {
            Ok(ReleaseDryPackageChecksumProvenanceClassification::ChecksumEvidenceOnly)
        }
        _ => Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::InvalidChecksumClassification,
        ]),
    }
}
fn parse_provenance_classification(
    code: &str,
) -> Result<
    ReleaseDryPackageChecksumProvenanceClassification,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    match code {
        "provenance_evidence_only" => {
            Ok(ReleaseDryPackageChecksumProvenanceClassification::ProvenanceEvidenceOnly)
        }
        _ => Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::InvalidProvenanceClassification,
        ]),
    }
}
fn parse_production_classification(
    code: &str,
) -> Result<
    ReleaseDryPackageChecksumProvenanceProductionClassification,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    match code {
        "non_production" => {
            Ok(ReleaseDryPackageChecksumProvenanceProductionClassification::NonProduction)
        }
        _ => Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::InvalidProductionClassification,
        ]),
    }
}
fn parse_distribution_classification(
    code: &str,
) -> Result<
    ReleaseDryPackageChecksumProvenanceDistributionClassification,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    match code {
        "local_only_non_public" => {
            Ok(ReleaseDryPackageChecksumProvenanceDistributionClassification::LocalOnlyNonPublic)
        }
        _ => Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::InvalidDistributionClassification,
        ]),
    }
}
fn parse_authority_classification(
    code: &str,
) -> Result<
    ReleaseDryPackageChecksumProvenanceAuthorityClassification,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    match code {
        "non_authoritative_evidence" => Ok(
            ReleaseDryPackageChecksumProvenanceAuthorityClassification::NonAuthoritativeEvidence,
        ),
        _ => Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::InvalidAuthorityClassification,
        ]),
    }
}
fn parse_release_classification(
    code: &str,
) -> Result<
    ReleaseDryPackageChecksumProvenanceReleaseClassification,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    match code {
        "release_not_approved" => {
            Ok(ReleaseDryPackageChecksumProvenanceReleaseClassification::ReleaseNotApproved)
        }
        _ => Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::InvalidReleaseClassification,
        ]),
    }
}
fn parse_preparation_status(
    code: &str,
) -> Result<
    ReleaseCandidatePreparationStatus,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
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
            ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput,
        ]),
    }
}
fn parse_dry_package_status(
    code: &str,
) -> Result<ReleaseArtifactDryPackageStatus, Vec<ReleaseDryPackageChecksumProvenanceValidationError>>
{
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
            ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput,
        ]),
    }
}
fn parse_boundary_statuses(
    input: &str,
) -> Result<
    Vec<ReleaseDryPackageChecksumProvenanceBoundaryStatus>,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    if input.is_empty() {
        return Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput,
        ]);
    }
    input.split(',').map(parse_boundary_status).collect()
}
fn parse_boundary_status(
    code: &str,
) -> Result<
    ReleaseDryPackageChecksumProvenanceBoundaryStatus,
    Vec<ReleaseDryPackageChecksumProvenanceValidationError>,
> {
    release_dry_package_checksum_provenance_boundary_statuses()
        .into_iter()
        .find(|status| status.code() == code)
        .ok_or_else(|| vec![ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput])
}
fn parse_summary(
    input: &str,
) -> Result<Vec<String>, Vec<ReleaseDryPackageChecksumProvenanceValidationError>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }
    input.split(";;").map(hex_decode).collect()
}

fn hex_encode(input: &str) -> String {
    input
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
fn hex_decode(
    input: &str,
) -> Result<String, Vec<ReleaseDryPackageChecksumProvenanceValidationError>> {
    if !input.len().is_multiple_of(2) {
        return Err(vec![
            ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput,
        ]);
    }
    let mut bytes = Vec::new();
    let mut index = 0;
    while index < input.len() {
        let byte = u8::from_str_radix(&input[index..index + 2], 16).map_err(|_| vec![ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput])?;
        bytes.push(byte);
        index += 2;
    }
    String::from_utf8(bytes).map_err(|_| {
        vec![ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput]
    })
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

    fn evidence() -> ReleaseDryPackageChecksumProvenanceEvidence {
        derive_release_dry_package_checksum_provenance(Some(&dry_package())).unwrap()
    }

    #[test]
    fn deterministic_checksum_provenance_and_serialization() {
        let package = dry_package();
        let first = derive_release_dry_package_checksum_provenance(Some(&package)).unwrap();
        let second = derive_release_dry_package_checksum_provenance(Some(&package)).unwrap();
        assert_eq!(first, second);
        assert_eq!(
            first.metadata.checksum_evidence.checksum_value,
            second.metadata.checksum_evidence.checksum_value
        );
        assert_eq!(
            first.metadata.provenance_evidence.provenance_id,
            second.metadata.provenance_evidence.provenance_id
        );
        assert_eq!(
            serialize_release_dry_package_checksum_provenance(&first).unwrap(),
            serialize_release_dry_package_checksum_provenance(&second).unwrap()
        );
        assert_eq!(
            first
                .metadata
                .checksum_evidence
                .checksum_classification
                .code(),
            "checksum_evidence_only"
        );
        assert_eq!(
            first
                .metadata
                .provenance_evidence
                .provenance_classification
                .code(),
            "provenance_evidence_only"
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
            "non_authoritative_evidence"
        );
        assert_eq!(
            first.metadata.release_classification.code(),
            "release_not_approved"
        );
    }

    #[test]
    fn changed_dry_package_changes_checksum() {
        let package = dry_package();
        let first = derive_release_dry_package_checksum_provenance(Some(&package)).unwrap();
        let mut changed_snapshot = complete_release_candidate_preparation_input_snapshot();
        changed_snapshot.evidence_items[0]
            .source_linkage
            .source_summary
            .push_str(" with changed local dry package basis");
        let changed_preparation =
            derive_release_candidate_preparation_projection(&changed_snapshot);
        let changed = derive_release_artifact_dry_package(Some(&changed_preparation)).unwrap();
        let second = derive_release_dry_package_checksum_provenance(Some(&changed)).unwrap();
        assert_ne!(
            first.metadata.checksum_evidence.checksum_value,
            second.metadata.checksum_evidence.checksum_value
        );
    }

    #[test]
    fn missing_invalid_malformed_and_linkage_rejections() {
        assert_eq!(
            derive_release_dry_package_checksum_provenance(None).unwrap_err(),
            vec![ReleaseDryPackageChecksumProvenanceValidationError::MissingDryPackage]
        );
        let mut rejected = dry_package();
        rejected.metadata.dry_package_status = ReleaseArtifactDryPackageStatus::DryPackageRejected;
        assert!(
            derive_release_dry_package_checksum_provenance(Some(&rejected))
                .unwrap_err()
                .contains(&ReleaseDryPackageChecksumProvenanceValidationError::DryPackageRejected)
        );
        let mut malformed = dry_package();
        malformed.payload.included_evidence.clear();
        malformed.payload.included_evidence_count = 0;
        assert!(
            derive_release_dry_package_checksum_provenance(Some(&malformed))
                .unwrap_err()
                .contains(
                    &ReleaseDryPackageChecksumProvenanceValidationError::MissingPreparationLinkage
                )
        );
        let mut mismatch = dry_package();
        mismatch.metadata.content_digest = "bad".to_string();
        assert!(
            derive_release_dry_package_checksum_provenance(Some(&mismatch))
                .unwrap_err()
                .contains(
                    &ReleaseDryPackageChecksumProvenanceValidationError::DryPackageDigestMismatch
                )
        );
        let mut missing_linkage = dry_package();
        missing_linkage.payload.source_preparation_id.clear();
        assert!(
            derive_release_dry_package_checksum_provenance(Some(&missing_linkage))
                .unwrap_err()
                .contains(
                    &ReleaseDryPackageChecksumProvenanceValidationError::MissingPreparationLinkage
                )
        );
    }

    #[test]
    fn provenance_links_phase_171_and_phase_172_evidence() {
        let package = dry_package();
        let evidence = derive_release_dry_package_checksum_provenance(Some(&package)).unwrap();
        let linkage = &evidence.metadata.provenance_evidence.linkage;
        assert_eq!(
            linkage.phase_171_preparation_id,
            package.payload.source_preparation_id
        );
        assert_eq!(
            linkage.phase_172_dry_package_id,
            package.metadata.dry_package_id
        );
        assert_eq!(
            linkage.phase_172_dry_package_digest,
            package.metadata.content_digest
        );
        assert_eq!(
            linkage.included_evidence_count,
            package.payload.included_evidence_count
        );
        assert!(release_dry_package_provenance_linkage_summary(linkage)
            .contains("Phase 171 preparation"));
    }

    #[test]
    fn explicit_write_read_and_read_back_validation_use_caller_path() {
        let evidence = evidence();
        let path = phase_173_temp_path("write-read");
        let write = write_release_dry_package_checksum_provenance(&evidence, &path).unwrap();
        assert_eq!(write.path, path.display().to_string());
        assert_eq!(
            write.projection.status,
            ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceWritten
        );
        let read = read_release_dry_package_checksum_provenance(&path).unwrap();
        assert_eq!(read.path, path.display().to_string());
        assert_eq!(
            read.evidence.metadata.checksum_evidence.checksum_value,
            evidence.metadata.checksum_evidence.checksum_value
        );
        assert_eq!(
            read.read_back.status,
            ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceReadBackValidated
        );
        assert!(read.read_back.checksum_value.is_some());
    }

    #[test]
    fn malformed_and_digest_mismatch_read_back_reject() {
        let malformed =
            validate_release_dry_package_checksum_provenance_read_back("not checksum provenance");
        assert_eq!(
            malformed.validation_status,
            ReleaseDryPackageChecksumProvenanceValidationStatus::Invalid
        );
        assert!(malformed.validation_errors.contains(
            &ReleaseDryPackageChecksumProvenanceValidationError::MalformedChecksumProvenanceInput
        ));
        let serialized = serialize_release_dry_package_checksum_provenance(&evidence()).unwrap();
        let drifted = serialized.replacen(
            "provenance_content_digest=",
            "provenance_content_digest=0000",
            1,
        );
        let read_back = validate_release_dry_package_checksum_provenance_read_back(&drifted);
        assert_eq!(
            read_back.validation_status,
            ReleaseDryPackageChecksumProvenanceValidationStatus::Invalid
        );
        assert!(read_back.validation_errors.contains(
            &ReleaseDryPackageChecksumProvenanceValidationError::DeterministicDigestMismatch
        ));
    }

    #[test]
    fn no_authority_boundaries_and_claims_reject() {
        let mut evidence = evidence();
        for marker in [
            ReleaseDryPackageChecksumProvenanceBoundaryStatus::ChecksumEvidenceOnly,
            ReleaseDryPackageChecksumProvenanceBoundaryStatus::ProvenanceEvidenceOnly,
            ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoSigning,
            ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoPublishing,
            ReleaseDryPackageChecksumProvenanceBoundaryStatus::ReleaseArtifactNotCreated,
            ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoPublicDistribution,
            ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoDeploymentArtifact,
            ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoProviderTrust,
            ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoActionAuthorization,
            ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoReplayRepair,
            ReleaseDryPackageChecksumProvenanceBoundaryStatus::NoRecoveryPromotion,
        ] {
            assert!(evidence.payload.boundary_statuses.contains(&marker));
        }
        evidence.payload.no_distribution_note = "readiness approved; actual release; deployment occurs; public use approved; production use approved; signing occurs; publishing occurs; installer activated; update channel activated; public download available; github release published; release tag created; provider output trusted; action authorization granted; replay repair performed; recovery promotion performed".to_string();
        let digest = stable_release_dry_package_checksum(&provenance_payload_basis(
            &evidence.metadata.checksum_evidence.checksum_value,
            &evidence.metadata.provenance_evidence.linkage,
            &evidence.payload,
        ));
        evidence.metadata.provenance_content_digest = digest.clone();
        evidence.metadata.provenance_evidence.provenance_id =
            format!("release-dry-package-provenance-{digest}");
        let errors = validate_release_dry_package_checksum_provenance(&evidence).unwrap_err();
        for error in [
            ReleaseDryPackageChecksumProvenanceValidationError::ReadinessClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::ReleaseClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::DeploymentClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::PublicUseClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::ProductionUseClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::SigningClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::PublishingClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::InstallerClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::UpdateChannelClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::PublicDownloadClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::GithubReleaseClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::ReleaseTagClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::ProviderTrustClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::ActionAuthorizationClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::ReplayRepairClaimDetected,
            ReleaseDryPackageChecksumProvenanceValidationError::RecoveryPromotionClaimDetected,
        ] {
            assert!(errors.contains(&error), "missing {error:?}");
        }
    }

    fn phase_173_temp_path(name: &str) -> PathBuf {
        PathBuf::from(format!("/tmp/ajentic-phase-173-{name}.txt"))
    }
}
