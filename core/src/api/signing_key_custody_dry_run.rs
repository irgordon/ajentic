//! Signing/key-custody dry-run evidence projection for phase 176.
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigningKeyCustodyDryRunStatus {
    NotGenerated,
    DryRunProjected,
    DryRunValidated,
    DryRunBlocked,
    DryRunRejected,
    InvalidDryRunInput,
}
impl SigningKeyCustodyDryRunStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotGenerated => "not_generated",
            Self::DryRunProjected => "dry_run_projected",
            Self::DryRunValidated => "dry_run_validated",
            Self::DryRunBlocked => "dry_run_blocked",
            Self::DryRunRejected => "dry_run_rejected",
            Self::InvalidDryRunInput => "invalid_dry_run_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigningKeyCustodyDryRunValidationStatus {
    NotValidated,
    Valid,
    Invalid,
}
impl SigningKeyCustodyDryRunValidationStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotValidated => "not_validated",
            Self::Valid => "valid",
            Self::Invalid => "invalid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigningKeyCustodyDryRunValidationError {
    MissingPreparationContract,
    PreparationRejected,
    MissingDryPackage,
    DryPackageRejected,
    MissingChecksumProvenance,
    ChecksumProvenanceRejected,
    MissingInstallerDistributionContract,
    InstallerDistributionContractRejected,
    UpstreamLinkageMismatch,
    RealSigningKeyClaimDetected,
    PrivateKeyClaimDetected,
    CertificateClaimDetected,
    KmsClaimDetected,
    SecretClaimDetected,
    EnvironmentVariableClaimDetected,
    SignatureCreationClaimDetected,
    PublicSigningClaimDetected,
    SigningClaimDetected,
    PublishingClaimDetected,
    InstallerClaimDetected,
    UpdateChannelClaimDetected,
    PublicDownloadClaimDetected,
    GithubReleaseClaimDetected,
    ReleaseTagClaimDetected,
    ReleaseClaimDetected,
    DeploymentClaimDetected,
    ReadinessClaimDetected,
    PublicUseClaimDetected,
    ProductionUseClaimDetected,
    ProviderTrustClaimDetected,
    ActionAuthorizationClaimDetected,
    ReplayRepairClaimDetected,
    RecoveryPromotionClaimDetected,
}
impl SigningKeyCustodyDryRunValidationError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingPreparationContract => "missing_preparation_contract",
            Self::PreparationRejected => "preparation_rejected",
            Self::MissingDryPackage => "missing_dry_package",
            Self::DryPackageRejected => "dry_package_rejected",
            Self::MissingChecksumProvenance => "missing_checksum_provenance",
            Self::ChecksumProvenanceRejected => "checksum_provenance_rejected",
            Self::MissingInstallerDistributionContract => "missing_installer_distribution_contract",
            Self::InstallerDistributionContractRejected => {
                "installer_distribution_contract_rejected"
            }
            Self::UpstreamLinkageMismatch => "upstream_linkage_mismatch",
            Self::RealSigningKeyClaimDetected => "real_signing_key_claim_detected",
            Self::PrivateKeyClaimDetected => "private_key_claim_detected",
            Self::CertificateClaimDetected => "certificate_claim_detected",
            Self::KmsClaimDetected => "kms_claim_detected",
            Self::SecretClaimDetected => "secret_claim_detected",
            Self::EnvironmentVariableClaimDetected => "environment_variable_claim_detected",
            Self::SignatureCreationClaimDetected => "signature_creation_claim_detected",
            Self::PublicSigningClaimDetected => "public_signing_claim_detected",
            Self::SigningClaimDetected => "signing_claim_detected",
            Self::PublishingClaimDetected => "publishing_claim_detected",
            Self::InstallerClaimDetected => "installer_claim_detected",
            Self::UpdateChannelClaimDetected => "update_channel_claim_detected",
            Self::PublicDownloadClaimDetected => "public_download_claim_detected",
            Self::GithubReleaseClaimDetected => "github_release_claim_detected",
            Self::ReleaseTagClaimDetected => "release_tag_claim_detected",
            Self::ReleaseClaimDetected => "release_claim_detected",
            Self::DeploymentClaimDetected => "deployment_claim_detected",
            Self::ReadinessClaimDetected => "readiness_claim_detected",
            Self::PublicUseClaimDetected => "public_use_claim_detected",
            Self::ProductionUseClaimDetected => "production_use_claim_detected",
            Self::ProviderTrustClaimDetected => "provider_trust_claim_detected",
            Self::ActionAuthorizationClaimDetected => "action_authorization_claim_detected",
            Self::ReplayRepairClaimDetected => "replay_repair_claim_detected",
            Self::RecoveryPromotionClaimDetected => "recovery_promotion_claim_detected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigningKeyCustodyDryRunUpstreamLinkage {
    pub preparation_contract_id: String,
    pub dry_package_id: String,
    pub checksum_provenance_id: String,
    pub installer_distribution_contract_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigningKeyCustodyPlaceholderKeyMetadata {
    pub key_profile: String,
    pub key_id: String,
    pub custody_model: String,
    pub key_material_present: bool,
    pub real_key_reference_present: bool,
    pub secret_material_present: bool,
    pub certificate_material_present: bool,
    pub kms_binding_present: bool,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigningKeyCustodyDryRunCapabilitySurface {
    pub sign_enabled: bool,
    pub publish_enabled: bool,
    pub deploy_enabled: bool,
    pub release_enabled: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigningKeyCustodyDryRunEvidence {
    pub evidence_id: String,
    pub linkage: SigningKeyCustodyDryRunUpstreamLinkage,
    pub placeholder_key_metadata: SigningKeyCustodyPlaceholderKeyMetadata,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigningKeyCustodyDryRunProjection {
    pub status: SigningKeyCustodyDryRunStatus,
    pub evidence: Option<SigningKeyCustodyDryRunEvidence>,
    pub classification: String,
    pub production_classification: String,
    pub distribution_classification: String,
    pub authority_classification: String,
    pub release_classification: String,
    pub missing_evidence: Vec<String>,
    pub blockers: Vec<String>,
    pub validation_status: SigningKeyCustodyDryRunValidationStatus,
    pub validation_errors: Vec<SigningKeyCustodyDryRunValidationError>,
    pub boundary_statuses: Vec<String>,
    pub capability_surface: SigningKeyCustodyDryRunCapabilitySurface,
}

pub fn signing_key_custody_placeholder_key_metadata() -> SigningKeyCustodyPlaceholderKeyMetadata {
    SigningKeyCustodyPlaceholderKeyMetadata { key_profile:"placeholder_key_profile".into(), key_id:"placeholder-key-id".into(), custody_model:"dry_run_placeholder_custody".into(), key_material_present:false, real_key_reference_present:false, secret_material_present:false, certificate_material_present:false, kms_binding_present:false, summary:"placeholder key metadata only; no real signing key, private key, certificate, KMS binding, or secret material is present".into() }
}
pub fn signing_key_custody_dry_run_capability_surface() -> SigningKeyCustodyDryRunCapabilitySurface
{
    SigningKeyCustodyDryRunCapabilitySurface {
        sign_enabled: false,
        publish_enabled: false,
        deploy_enabled: false,
        release_enabled: false,
    }
}
pub fn signing_key_custody_dry_run_boundary_statuses() -> Vec<String> {
    vec![
        "dry_run_evidence_only",
        "local_signing_dry_run_only",
        "placeholder_key_metadata_only",
        "no_real_signing_keys",
        "no_private_keys",
        "no_certificate_material",
        "no_kms_binding",
        "no_secret_material",
        "no_signature_created",
        "no_public_signing",
        "no_signing",
        "no_publishing",
        "release_artifact_not_created",
        "release_readiness_not_approved",
        "release_candidate_status_not_approved",
        "production_status_not_approved",
        "no_public_distribution",
        "no_public_download",
        "no_github_release",
        "no_release_tag",
        "no_installer_activation",
        "no_update_channel_activation",
        "no_deployment_artifact",
        "no_provider_trust",
        "no_action_authorization",
        "no_replay_repair",
        "no_recovery_promotion",
    ]
    .into_iter()
    .map(ToString::to_string)
    .collect()
}

pub fn initial_signing_key_custody_dry_run_projection() -> SigningKeyCustodyDryRunProjection {
    SigningKeyCustodyDryRunProjection {
        status: SigningKeyCustodyDryRunStatus::NotGenerated,
        evidence: None,
        classification: "dry_run_evidence_only".into(),
        production_classification: "non_production".into(),
        distribution_classification: "local_only_non_public".into(),
        authority_classification: "non_authoritative_dry_run_evidence".into(),
        release_classification: "release_not_approved".into(),
        missing_evidence: vec![],
        blockers: vec![],
        validation_status: SigningKeyCustodyDryRunValidationStatus::NotValidated,
        validation_errors: vec![],
        boundary_statuses: signing_key_custody_dry_run_boundary_statuses(),
        capability_surface: signing_key_custody_dry_run_capability_surface(),
    }
}

fn stable_digest(input: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}

pub fn derive_signing_key_custody_dry_run(
    prep: Option<&ReleaseCandidatePreparationProjection>,
    dry: Option<&ReleaseArtifactDryPackageProjection>,
    cp: Option<&ReleaseDryPackageChecksumProvenanceProjection>,
    inst: Option<&InstallerDistributionContractProjection>,
) -> SigningKeyCustodyDryRunProjection {
    let mut p = initial_signing_key_custody_dry_run_projection();
    let mut errs = Vec::new();
    if prep.is_none() {
        errs.push(SigningKeyCustodyDryRunValidationError::MissingPreparationContract);
    }
    if dry.is_none() {
        errs.push(SigningKeyCustodyDryRunValidationError::MissingDryPackage);
    }
    if cp.is_none() {
        errs.push(SigningKeyCustodyDryRunValidationError::MissingChecksumProvenance);
    }
    if inst.is_none() {
        errs.push(SigningKeyCustodyDryRunValidationError::MissingInstallerDistributionContract);
    }
    if let Some(x) = prep {
        if x.status != ReleaseCandidatePreparationStatus::PreparationValidated {
            errs.push(SigningKeyCustodyDryRunValidationError::PreparationRejected);
        }
    }
    if let Some(x) = dry {
        if x.status != ReleaseArtifactDryPackageStatus::DryPackageProjected {
            errs.push(SigningKeyCustodyDryRunValidationError::DryPackageRejected);
        }
    }
    if let Some(x) = cp {
        if x.status != ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceValidated {
            errs.push(SigningKeyCustodyDryRunValidationError::ChecksumProvenanceRejected);
        }
    }
    if let Some(x) = inst {
        if x.status != InstallerDistributionContractStatus::ContractValidated {
            errs.push(
                SigningKeyCustodyDryRunValidationError::InstallerDistributionContractRejected,
            );
        }
    }
    if let (Some(pr), Some(dr), Some(cr), Some(ir)) = (prep, dry, cp, inst) {
        let pid = pr.preparation_id.clone();
        let did = dr.dry_package_id.clone().unwrap_or_default();
        let cid = cr.dry_package_id.clone().unwrap_or_default();
        if did.is_empty()
            || pid.is_empty()
            || cid != did
            || ir
                .dry_package_linkage
                .as_ref()
                .map(|x| x.dry_package_id.as_str())
                .unwrap_or_default()
                != did
        {
            errs.push(SigningKeyCustodyDryRunValidationError::UpstreamLinkageMismatch);
        }
        let linkage = SigningKeyCustodyDryRunUpstreamLinkage {
            preparation_contract_id: pid,
            dry_package_id: did,
            checksum_provenance_id: cr.provenance_id.clone().unwrap_or_default(),
            installer_distribution_contract_id: ir.contract_id.clone().unwrap_or_default(),
        };
        let placeholder = signing_key_custody_placeholder_key_metadata();
        let digest = stable_digest(&format!(
            "{}|{}|{}",
            linkage.preparation_contract_id,
            linkage.dry_package_id,
            p.boundary_statuses.join("|")
        ));
        p.evidence = Some(SigningKeyCustodyDryRunEvidence {
            evidence_id: format!("signing-key-custody-dry-run-{digest}"),
            linkage,
            placeholder_key_metadata: placeholder,
        });
    }
    if !errs.is_empty() {
        errs.sort_by_key(|e| e.code());
        errs.dedup();
        let codes: Vec<String> = errs.iter().map(|e| e.code().to_string()).collect();
        p.missing_evidence = codes
            .iter()
            .filter(|x| x.starts_with("missing_"))
            .cloned()
            .collect();
        p.blockers = codes
            .iter()
            .filter(|x| !x.starts_with("missing_"))
            .cloned()
            .collect();
        p.status = SigningKeyCustodyDryRunStatus::DryRunRejected;
        p.validation_status = SigningKeyCustodyDryRunValidationStatus::Invalid;
        p.validation_errors = errs;
        return p;
    }
    p.status = SigningKeyCustodyDryRunStatus::DryRunValidated;
    p.validation_status = SigningKeyCustodyDryRunValidationStatus::Valid;
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        initial_installer_distribution_contract_projection,
        initial_release_artifact_dry_package_projection,
        initial_release_candidate_preparation_projection,
        initial_release_dry_package_checksum_provenance_projection,
    };
    fn valid_inputs() -> (
        ReleaseCandidatePreparationProjection,
        ReleaseArtifactDryPackageProjection,
        ReleaseDryPackageChecksumProvenanceProjection,
        InstallerDistributionContractProjection,
    ) {
        let mut p = initial_release_candidate_preparation_projection();
        p.status = ReleaseCandidatePreparationStatus::PreparationValidated;
        p.preparation_id = "prep-1".into();
        let mut d = initial_release_artifact_dry_package_projection();
        d.status = ReleaseArtifactDryPackageStatus::DryPackageProjected;
        d.dry_package_id = Some("dry-1".into());
        let mut c = initial_release_dry_package_checksum_provenance_projection();
        c.status = ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceValidated;
        c.dry_package_id = Some("dry-1".into());
        c.provenance_id = Some("prov-1".into());
        let mut i = initial_installer_distribution_contract_projection();
        i.status = InstallerDistributionContractStatus::ContractValidated;
        i.contract_id = Some("inst-1".into());
        i.dry_package_linkage = Some(InstallerDistributionDryPackageLinkage {
            dry_package_id: "dry-1".into(),
            dry_package_status: "dry_package_projected".into(),
        });
        (p, d, c, i)
    }
    #[test]
    fn validates_mapping_and_deterministic_id() {
        let (p, d, c, i) = valid_inputs();
        let a = derive_signing_key_custody_dry_run(Some(&p), Some(&d), Some(&c), Some(&i));
        let b = derive_signing_key_custody_dry_run(Some(&p), Some(&d), Some(&c), Some(&i));
        assert_eq!(a.status, SigningKeyCustodyDryRunStatus::DryRunValidated);
        assert_eq!(
            a.evidence.as_ref().unwrap().evidence_id,
            b.evidence.as_ref().unwrap().evidence_id
        );
    }
    #[test]
    fn rejects_missing_upstream_inputs() {
        let p = derive_signing_key_custody_dry_run(None, None, None, None);
        assert_eq!(p.status, SigningKeyCustodyDryRunStatus::DryRunRejected);
        assert_eq!(
            p.missing_evidence,
            vec![
                "missing_checksum_provenance",
                "missing_dry_package",
                "missing_installer_distribution_contract",
                "missing_preparation_contract"
            ]
        );
    }
}
