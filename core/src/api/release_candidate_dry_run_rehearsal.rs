use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidateDryRunRehearsalStatus {
    NotRehearsed,
    RehearsalProjected,
    RehearsalCompletedLocally,
    RehearsalBlocked,
    RehearsalRejected,
    InvalidRehearsalInput,
}
impl ReleaseCandidateDryRunRehearsalStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotRehearsed => "not_rehearsed",
            Self::RehearsalProjected => "rehearsal_projected",
            Self::RehearsalCompletedLocally => "rehearsal_completed_locally",
            Self::RehearsalBlocked => "rehearsal_blocked",
            Self::RehearsalRejected => "rehearsal_rejected",
            Self::InvalidRehearsalInput => "invalid_rehearsal_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateDryRunRehearsalUpstreamCategory {
    ReleaseCandidatePreparationContract,
    ReleaseArtifactDryPackage,
    DryPackageChecksumProvenance,
    InstallerDistributionContract,
    SigningKeyCustodyDryRun,
    ReleaseCandidateEvidenceAssembly,
    ReleaseCandidateGapReview,
}
impl ReleaseCandidateDryRunRehearsalUpstreamCategory {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ReleaseCandidatePreparationContract => "release_candidate_preparation_contract",
            Self::ReleaseArtifactDryPackage => "release_artifact_dry_package",
            Self::DryPackageChecksumProvenance => "dry_package_checksum_provenance",
            Self::InstallerDistributionContract => "installer_distribution_contract",
            Self::SigningKeyCustodyDryRun => "signing_key_custody_dry_run",
            Self::ReleaseCandidateEvidenceAssembly => "release_candidate_evidence_assembly",
            Self::ReleaseCandidateGapReview => "release_candidate_gap_review",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UpstreamEvidenceClassification {
    Present,
    Missing,
    Rejected,
    Blocked,
    Invalid,
    InformationalOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalUpstreamLinkage {
    pub category: String,
    pub source_id: String,
    pub source_status: String,
    pub source_summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalMissingEvidence {
    pub category: String,
    pub reason: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalBlocker {
    pub category: String,
    pub reason: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalGapReviewSummary {
    pub gap_review_id: String,
    pub status: String,
    pub blocking_gap_count: usize,
    pub gap_count: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalArtifactSummary {
    pub artifact_id: String,
    pub artifact_kind: String,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalValidationSummary {
    pub status: String,
    pub has_missing_evidence: bool,
    pub has_blockers: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalCapabilitySurface {
    pub approval_enabled: bool,
    pub signing_enabled: bool,
    pub publishing_enabled: bool,
    pub deployment_enabled: bool,
    pub public_distribution_enabled: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalProjection {
    pub status: String,
    pub rehearsal_id: Option<String>,
    pub upstream_linkage: Vec<ReleaseCandidateDryRunRehearsalUpstreamLinkage>,
    pub missing_evidence: Vec<ReleaseCandidateDryRunRehearsalMissingEvidence>,
    pub blockers: Vec<ReleaseCandidateDryRunRehearsalBlocker>,
    pub gap_review_summary: ReleaseCandidateDryRunRehearsalGapReviewSummary,
    pub rehearsal_artifact_summary: Vec<ReleaseCandidateDryRunRehearsalArtifactSummary>,
    pub validation_summary: ReleaseCandidateDryRunRehearsalValidationSummary,
    pub boundary_statuses: Vec<String>,
    pub capability_surface: ReleaseCandidateDryRunRehearsalCapabilitySurface,
}

pub fn initial_release_candidate_dry_run_rehearsal_projection(
) -> ReleaseCandidateDryRunRehearsalProjection {
    ReleaseCandidateDryRunRehearsalProjection {
        status: ReleaseCandidateDryRunRehearsalStatus::NotRehearsed
            .code()
            .into(),
        rehearsal_id: None,
        upstream_linkage: vec![],
        missing_evidence: vec![],
        blockers: vec![],
        gap_review_summary: ReleaseCandidateDryRunRehearsalGapReviewSummary {
            gap_review_id: "not_rehearsed".into(),
            status: "not_reviewed".into(),
            blocking_gap_count: 0,
            gap_count: 0,
        },
        rehearsal_artifact_summary: vec![],
        validation_summary: ReleaseCandidateDryRunRehearsalValidationSummary {
            status: "not_rehearsed".into(),
            has_missing_evidence: false,
            has_blockers: false,
        },
        boundary_statuses: vec![
            "dry_run_rehearsal_only".into(),
            "local_rehearsal_only".into(),
            "non_authoritative_rehearsal".into(),
            "local_only_non_public".into(),
            "release_candidate_status_not_approved".into(),
            "release_readiness_not_approved".into(),
            "production_status_not_approved".into(),
            "public_use_not_approved".into(),
            "release_artifact_not_created".into(),
            "public_artifact_not_created".into(),
            "deployment_artifact_not_created".into(),
            "no_real_signing_keys".into(),
            "no_signature_created".into(),
            "no_signing".into(),
            "no_publishing".into(),
            "no_installer_activation".into(),
            "no_update_channel_activation".into(),
            "no_public_distribution".into(),
            "no_public_download".into(),
            "no_github_release".into(),
            "no_release_tag".into(),
            "no_deployment_artifact".into(),
            "no_provider_trust".into(),
            "no_action_authorization".into(),
            "no_replay_repair".into(),
            "no_recovery_promotion".into(),
        ],
        capability_surface: ReleaseCandidateDryRunRehearsalCapabilitySurface {
            approval_enabled: false,
            signing_enabled: false,
            publishing_enabled: false,
            deployment_enabled: false,
            public_distribution_enabled: false,
        },
    }
}

fn classify_preparation_rehearsal_status(
    status: ReleaseCandidatePreparationStatus,
) -> UpstreamEvidenceClassification {
    match status {
        ReleaseCandidatePreparationStatus::NotPrepared => UpstreamEvidenceClassification::Missing,
        ReleaseCandidatePreparationStatus::PreparationRejected => {
            UpstreamEvidenceClassification::Rejected
        }
        ReleaseCandidatePreparationStatus::PreparationBlocked => {
            UpstreamEvidenceClassification::Blocked
        }
        ReleaseCandidatePreparationStatus::InvalidPreparationInput => {
            UpstreamEvidenceClassification::Invalid
        }
        _ => UpstreamEvidenceClassification::Present,
    }
}
fn classify_dry_package_rehearsal_status(
    status: ReleaseArtifactDryPackageStatus,
) -> UpstreamEvidenceClassification {
    match status {
        ReleaseArtifactDryPackageStatus::NotAssembled => UpstreamEvidenceClassification::Missing,
        ReleaseArtifactDryPackageStatus::DryPackageRejected => {
            UpstreamEvidenceClassification::Rejected
        }
        ReleaseArtifactDryPackageStatus::InvalidDryPackageInput => {
            UpstreamEvidenceClassification::Invalid
        }
        _ => UpstreamEvidenceClassification::Present,
    }
}
fn classify_checksum_provenance_rehearsal_status(
    status: ReleaseDryPackageChecksumProvenanceStatus,
) -> UpstreamEvidenceClassification {
    match status {
        ReleaseDryPackageChecksumProvenanceStatus::NotGenerated => {
            UpstreamEvidenceClassification::Missing
        }
        ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceRejected => {
            UpstreamEvidenceClassification::Rejected
        }
        ReleaseDryPackageChecksumProvenanceStatus::InvalidChecksumProvenanceInput => {
            UpstreamEvidenceClassification::Invalid
        }
        _ => UpstreamEvidenceClassification::Present,
    }
}
fn classify_installer_distribution_rehearsal_status(
    status: InstallerDistributionContractStatus,
) -> UpstreamEvidenceClassification {
    match status {
        InstallerDistributionContractStatus::NotDefined => UpstreamEvidenceClassification::Missing,
        InstallerDistributionContractStatus::ContractRejected => {
            UpstreamEvidenceClassification::Rejected
        }
        InstallerDistributionContractStatus::InvalidContractInput => {
            UpstreamEvidenceClassification::Invalid
        }
        _ => UpstreamEvidenceClassification::Present,
    }
}
fn classify_signing_key_custody_rehearsal_status(
    status: SigningKeyCustodyDryRunStatus,
) -> UpstreamEvidenceClassification {
    match status {
        SigningKeyCustodyDryRunStatus::NotGenerated => UpstreamEvidenceClassification::Missing,
        SigningKeyCustodyDryRunStatus::DryRunRejected => UpstreamEvidenceClassification::Rejected,
        SigningKeyCustodyDryRunStatus::DryRunBlocked => UpstreamEvidenceClassification::Blocked,
        SigningKeyCustodyDryRunStatus::InvalidDryRunInput => {
            UpstreamEvidenceClassification::Invalid
        }
        _ => UpstreamEvidenceClassification::Present,
    }
}
fn classify_evidence_assembly_rehearsal_status(status: &str) -> UpstreamEvidenceClassification {
    match status {
        "not_assembled" => UpstreamEvidenceClassification::Missing,
        "evidence_rejected" => UpstreamEvidenceClassification::Rejected,
        "evidence_blocked" => UpstreamEvidenceClassification::Blocked,
        "invalid_evidence_input" => UpstreamEvidenceClassification::Invalid,
        _ => UpstreamEvidenceClassification::Present,
    }
}
fn classify_gap_review_rehearsal_status(
    status: &str,
    blocking_gap_count: usize,
) -> UpstreamEvidenceClassification {
    match status {
        "not_reviewed" => UpstreamEvidenceClassification::Missing,
        "gap_review_blocked" | "invalid_gap_review_input" => {
            UpstreamEvidenceClassification::Blocked
        }
        "informational_gaps_only" if blocking_gap_count == 0 => {
            UpstreamEvidenceClassification::InformationalOnly
        }
        _ if blocking_gap_count > 0 => UpstreamEvidenceClassification::Blocked,
        _ => UpstreamEvidenceClassification::Present,
    }
}

pub fn derive_release_candidate_dry_run_rehearsal(
    preparation: &ReleaseCandidatePreparationProjection,
    dry_package: &ReleaseArtifactDryPackageProjection,
    checksum: &ReleaseDryPackageChecksumProvenanceProjection,
    installer: &InstallerDistributionContractProjection,
    signing: &SigningKeyCustodyDryRunProjection,
    assembly: &ReleaseCandidateEvidenceAssemblyProjection,
    gap: &ReleaseCandidateGapReviewProjection,
) -> ReleaseCandidateDryRunRehearsalProjection {
    let mut p = initial_release_candidate_dry_run_rehearsal_projection();
    p.upstream_linkage = vec![
        linkage(
            ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseCandidatePreparationContract,
            preparation.preparation_id.clone(),
            preparation.status.code().to_string(),
        ),
        linkage(
            ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseArtifactDryPackage,
            dry_package
                .dry_package_id
                .clone()
                .unwrap_or_else(|| "missing_dry_package".into()),
            dry_package.status.code().to_string(),
        ),
        linkage(
            ReleaseCandidateDryRunRehearsalUpstreamCategory::DryPackageChecksumProvenance,
            checksum
                .provenance_id
                .clone()
                .unwrap_or_else(|| "missing_checksum_provenance".into()),
            checksum.status.code().to_string(),
        ),
        linkage(
            ReleaseCandidateDryRunRehearsalUpstreamCategory::InstallerDistributionContract,
            installer
                .contract_id
                .clone()
                .unwrap_or_else(|| "missing_installer_distribution_contract".into()),
            installer.status.code().to_string(),
        ),
        linkage(
            ReleaseCandidateDryRunRehearsalUpstreamCategory::SigningKeyCustodyDryRun,
            signing
                .evidence
                .as_ref()
                .map(|e| e.evidence_id.clone())
                .unwrap_or_else(|| "missing_signing_key_custody_dry_run".into()),
            signing.status.code().to_string(),
        ),
        linkage(
            ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseCandidateEvidenceAssembly,
            assembly
                .assembly_id
                .clone()
                .unwrap_or_else(|| "missing_release_candidate_evidence_assembly".into()),
            assembly.status.clone(),
        ),
        linkage(
            ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseCandidateGapReview,
            gap.gap_review_id
                .clone()
                .unwrap_or_else(|| "missing_release_candidate_gap_review".into()),
            gap.status.clone(),
        ),
    ];

    let classifications = [
        (
            ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseCandidatePreparationContract
                .code(),
            classify_preparation_rehearsal_status(preparation.status),
        ),
        (
            ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseArtifactDryPackage.code(),
            classify_dry_package_rehearsal_status(dry_package.status),
        ),
        (
            ReleaseCandidateDryRunRehearsalUpstreamCategory::DryPackageChecksumProvenance.code(),
            classify_checksum_provenance_rehearsal_status(checksum.status),
        ),
        (
            ReleaseCandidateDryRunRehearsalUpstreamCategory::InstallerDistributionContract.code(),
            classify_installer_distribution_rehearsal_status(installer.status),
        ),
        (
            ReleaseCandidateDryRunRehearsalUpstreamCategory::SigningKeyCustodyDryRun.code(),
            classify_signing_key_custody_rehearsal_status(signing.status),
        ),
        (
            ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseCandidateEvidenceAssembly
                .code(),
            classify_evidence_assembly_rehearsal_status(&assembly.status),
        ),
        (
            ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseCandidateGapReview.code(),
            classify_gap_review_rehearsal_status(&gap.status, gap.blocking_gap_count),
        ),
    ];

    for (category, c) in classifications {
        match c {
            UpstreamEvidenceClassification::Missing => {
                p.missing_evidence
                    .push(ReleaseCandidateDryRunRehearsalMissingEvidence {
                        category: category.into(),
                        reason: format!("missing_{category}"),
                    })
            }
            UpstreamEvidenceClassification::Rejected => {
                p.blockers.push(ReleaseCandidateDryRunRehearsalBlocker {
                    category: category.into(),
                    reason: format!("rejected_{category}"),
                })
            }
            UpstreamEvidenceClassification::Blocked | UpstreamEvidenceClassification::Invalid => {
                p.blockers.push(ReleaseCandidateDryRunRehearsalBlocker {
                    category: category.into(),
                    reason: format!("blocked_{category}"),
                })
            }
            _ => {}
        }
    }

    p.gap_review_summary = ReleaseCandidateDryRunRehearsalGapReviewSummary {
        gap_review_id: gap
            .gap_review_id
            .clone()
            .unwrap_or_else(|| "missing_release_candidate_gap_review".into()),
        status: gap.status.clone(),
        blocking_gap_count: gap.blocking_gap_count,
        gap_count: gap.gap_count,
    };
    p.missing_evidence
        .sort_by(|a, b| a.category.cmp(&b.category).then(a.reason.cmp(&b.reason)));
    p.blockers
        .sort_by(|a, b| a.category.cmp(&b.category).then(a.reason.cmp(&b.reason)));

    if p.missing_evidence.is_empty() && p.blockers.is_empty() {
        p.rehearsal_id = Some(format!(
            "rc-dry-run-rehearsal-{}-{}",
            preparation.preparation_id,
            gap.gap_review_id.clone().unwrap_or_default()
        ));
        p.status = ReleaseCandidateDryRunRehearsalStatus::RehearsalCompletedLocally
            .code()
            .into();
        p.rehearsal_artifact_summary = vec![ReleaseCandidateDryRunRehearsalArtifactSummary {
            artifact_id: p.rehearsal_id.clone().unwrap_or_default(),
            artifact_kind: "local_rehearsal_evidence_artifact".into(),
            summary: "Local deterministic rehearsal evidence chain projection summary.".into(),
        }];
    } else if p.blockers.iter().any(|b| b.reason.starts_with("rejected_")) {
        p.status = ReleaseCandidateDryRunRehearsalStatus::RehearsalRejected
            .code()
            .into();
    } else {
        p.status = ReleaseCandidateDryRunRehearsalStatus::RehearsalBlocked
            .code()
            .into();
    }
    p.validation_summary = ReleaseCandidateDryRunRehearsalValidationSummary {
        status: if p.missing_evidence.is_empty() && p.blockers.is_empty() {
            "valid".into()
        } else {
            "invalid".into()
        },
        has_missing_evidence: !p.missing_evidence.is_empty(),
        has_blockers: !p.blockers.is_empty(),
    };
    p
}

fn linkage(
    category: ReleaseCandidateDryRunRehearsalUpstreamCategory,
    source_id: String,
    source_status: String,
) -> ReleaseCandidateDryRunRehearsalUpstreamLinkage {
    ReleaseCandidateDryRunRehearsalUpstreamLinkage {
        category: category.code().into(),
        source_summary: format!("{}:{}", source_id, source_status),
        source_id,
        source_status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::initial_local_operator_shell_state;
    fn base() -> LocalOperatorShellState {
        let mut s = initial_local_operator_shell_state();
        s.release_candidate_preparation.status =
            ReleaseCandidatePreparationStatus::PreparationValidated;
        s.release_artifact_dry_package.status =
            ReleaseArtifactDryPackageStatus::DryPackageValidated;
        s.release_dry_package_checksum_provenance.status =
            ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceValidated;
        s.installer_distribution_contract.status =
            InstallerDistributionContractStatus::ContractValidated;
        s.signing_key_custody_dry_run.status = SigningKeyCustodyDryRunStatus::DryRunValidated;
        s.release_candidate_evidence_assembly.status = "evidence_assembled".into();
        s.release_candidate_gap_review.status = "no_blocking_gaps".into();
        s.release_candidate_gap_review.blocking_gap_count = 0;
        s
    }
    fn derive(s: &LocalOperatorShellState) -> ReleaseCandidateDryRunRehearsalProjection {
        derive_release_candidate_dry_run_rehearsal(
            &s.release_candidate_preparation,
            &s.release_artifact_dry_package,
            &s.release_dry_package_checksum_provenance,
            &s.installer_distribution_contract,
            &s.signing_key_custody_dry_run,
            &s.release_candidate_evidence_assembly,
            &s.release_candidate_gap_review,
        )
    }
    #[test]
    fn initial_not_rehearsed_projection() {
        assert_eq!(
            initial_release_candidate_dry_run_rehearsal_projection().status,
            "not_rehearsed"
        );
    }
    #[test]
    fn valid_upstream_evidence_produces_deterministic_rehearsal_projection() {
        let s = base();
        let p1 = derive(&s);
        let p2 = derive(&s);
        assert_eq!(p1, p2);
        assert_eq!(p1.status, "rehearsal_completed_locally");
    }
    #[test]
    fn missing_preparation_blocks_rehearsal() {
        let mut s = base();
        s.release_candidate_preparation.status = ReleaseCandidatePreparationStatus::NotPrepared;
        assert_eq!(derive(&s).status, "rehearsal_blocked");
    }
    #[test]
    fn missing_dry_package_blocks_rehearsal() {
        let mut s = base();
        s.release_artifact_dry_package.status = ReleaseArtifactDryPackageStatus::NotAssembled;
        assert_eq!(derive(&s).status, "rehearsal_blocked");
    }
    #[test]
    fn missing_checksum_provenance_blocks_rehearsal() {
        let mut s = base();
        s.release_dry_package_checksum_provenance.status =
            ReleaseDryPackageChecksumProvenanceStatus::NotGenerated;
        assert_eq!(derive(&s).status, "rehearsal_blocked");
    }
    #[test]
    fn missing_installer_distribution_contract_blocks_rehearsal() {
        let mut s = base();
        s.installer_distribution_contract.status = InstallerDistributionContractStatus::NotDefined;
        assert_eq!(derive(&s).status, "rehearsal_blocked");
    }
    #[test]
    fn missing_signing_key_custody_dry_run_blocks_rehearsal() {
        let mut s = base();
        s.signing_key_custody_dry_run.status = SigningKeyCustodyDryRunStatus::NotGenerated;
        assert_eq!(derive(&s).status, "rehearsal_blocked");
    }
    #[test]
    fn missing_evidence_assembly_blocks_rehearsal() {
        let mut s = base();
        s.release_candidate_evidence_assembly.status = "not_assembled".into();
        assert_eq!(derive(&s).status, "rehearsal_blocked");
    }
    #[test]
    fn missing_gap_review_blocks_rehearsal() {
        let mut s = base();
        s.release_candidate_gap_review.status = "not_reviewed".into();
        assert_eq!(derive(&s).status, "rehearsal_blocked");
    }
    #[test]
    fn rejected_preparation_blocks_rehearsal() {
        let mut s = base();
        s.release_candidate_preparation.status =
            ReleaseCandidatePreparationStatus::PreparationRejected;
        assert_eq!(derive(&s).status, "rehearsal_rejected");
    }
    #[test]
    fn rejected_dry_package_blocks_rehearsal() {
        let mut s = base();
        s.release_artifact_dry_package.status = ReleaseArtifactDryPackageStatus::DryPackageRejected;
        assert_eq!(derive(&s).status, "rehearsal_rejected");
    }
    #[test]
    fn rejected_checksum_provenance_blocks_rehearsal() {
        let mut s = base();
        s.release_dry_package_checksum_provenance.status =
            ReleaseDryPackageChecksumProvenanceStatus::ChecksumProvenanceRejected;
        assert_eq!(derive(&s).status, "rehearsal_rejected");
    }
    #[test]
    fn rejected_installer_distribution_contract_blocks_rehearsal() {
        let mut s = base();
        s.installer_distribution_contract.status =
            InstallerDistributionContractStatus::ContractRejected;
        assert_eq!(derive(&s).status, "rehearsal_rejected");
    }
    #[test]
    fn rejected_signing_key_custody_dry_run_blocks_rehearsal() {
        let mut s = base();
        s.signing_key_custody_dry_run.status = SigningKeyCustodyDryRunStatus::DryRunRejected;
        assert_eq!(derive(&s).status, "rehearsal_rejected");
    }
    #[test]
    fn rejected_evidence_assembly_blocks_rehearsal() {
        let mut s = base();
        s.release_candidate_evidence_assembly.status = "evidence_rejected".into();
        assert_eq!(derive(&s).status, "rehearsal_rejected");
    }
    #[test]
    fn blocking_gap_review_blocks_rehearsal() {
        let mut s = base();
        s.release_candidate_gap_review.blocking_gap_count = 1;
        assert_eq!(derive(&s).status, "rehearsal_blocked");
    }
    #[test]
    fn informational_only_gap_review_does_not_block_rehearsal() {
        let mut s = base();
        s.release_candidate_gap_review.status = "informational_gaps_only".into();
        s.release_candidate_gap_review.blocking_gap_count = 0;
        assert_eq!(derive(&s).status, "rehearsal_completed_locally");
    }
    #[test]
    fn deterministic_rehearsal_id_for_identical_inputs() {
        let s = base();
        assert_eq!(derive(&s).rehearsal_id, derive(&s).rehearsal_id);
    }
    #[test]
    fn deterministic_upstream_linkage_ordering() {
        let s = base();
        let p = derive(&s);
        let cats: Vec<_> = p.upstream_linkage.into_iter().map(|x| x.category).collect();
        assert_eq!(
            cats,
            vec![
                "release_candidate_preparation_contract",
                "release_artifact_dry_package",
                "dry_package_checksum_provenance",
                "installer_distribution_contract",
                "signing_key_custody_dry_run",
                "release_candidate_evidence_assembly",
                "release_candidate_gap_review"
            ]
        );
    }
    #[test]
    fn deterministic_missing_evidence_ordering() {
        let mut s = base();
        s.release_artifact_dry_package.status = ReleaseArtifactDryPackageStatus::NotAssembled;
        s.release_candidate_preparation.status = ReleaseCandidatePreparationStatus::NotPrepared;
        let p = derive(&s);
        assert!(p
            .missing_evidence
            .windows(2)
            .all(|w| w[0].category <= w[1].category));
    }
    #[test]
    fn deterministic_blocker_ordering() {
        let mut s = base();
        s.signing_key_custody_dry_run.status = SigningKeyCustodyDryRunStatus::DryRunRejected;
        s.installer_distribution_contract.status =
            InstallerDistributionContractStatus::ContractRejected;
        let p = derive(&s);
        assert!(p
            .blockers
            .windows(2)
            .all(|w| w[0].category <= w[1].category));
    }
    #[test]
    fn no_authority_boundary_markers() {
        let p = initial_release_candidate_dry_run_rehearsal_projection();
        assert!(p
            .boundary_statuses
            .contains(&"dry_run_rehearsal_only".into()));
        assert!(
            !p.capability_surface.approval_enabled
                && !p.capability_surface.signing_enabled
                && !p.capability_surface.publishing_enabled
                && !p.capability_surface.deployment_enabled
                && !p.capability_surface.public_distribution_enabled
        );
    }
}
