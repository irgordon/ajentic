use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidateEvidenceAssemblyStatus {
    NotAssembled,
    EvidenceAssembled,
    EvidenceIncomplete,
    EvidenceBlocked,
    EvidenceRejected,
    InvalidEvidenceInput,
}
impl ReleaseCandidateEvidenceAssemblyStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotAssembled => "not_assembled",
            Self::EvidenceAssembled => "evidence_assembled",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::EvidenceBlocked => "evidence_blocked",
            Self::EvidenceRejected => "evidence_rejected",
            Self::InvalidEvidenceInput => "invalid_evidence_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateEvidenceAssemblyCategory {
    ReleaseCandidatePreparationContract,
    ReleaseArtifactDryPackage,
    DryPackageChecksumProvenance,
    InstallerDistributionContract,
    SigningKeyCustodyDryRun,
    UserFacingHelp,
    LocalHtmlHelpPages,
    LocalBetaWorkflow,
    ValidationResults,
    NoReleaseReadinessApproval,
    NoReleaseCandidateApproval,
    NoSigningAuthority,
    NoPublishingAuthority,
    NoDeploymentAuthority,
    NoPublicDistributionAuthority,
}
impl ReleaseCandidateEvidenceAssemblyCategory {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ReleaseCandidatePreparationContract => "release_candidate_preparation_contract",
            Self::ReleaseArtifactDryPackage => "release_artifact_dry_package",
            Self::DryPackageChecksumProvenance => "dry_package_checksum_provenance",
            Self::InstallerDistributionContract => "installer_distribution_contract",
            Self::SigningKeyCustodyDryRun => "signing_key_custody_dry_run",
            Self::UserFacingHelp => "user_facing_help",
            Self::LocalHtmlHelpPages => "local_html_help_pages",
            Self::LocalBetaWorkflow => "local_beta_workflow",
            Self::ValidationResults => "validation_results",
            Self::NoReleaseReadinessApproval => "no_release_readiness_approval",
            Self::NoReleaseCandidateApproval => "no_release_candidate_approval",
            Self::NoSigningAuthority => "no_signing_authority",
            Self::NoPublishingAuthority => "no_publishing_authority",
            Self::NoDeploymentAuthority => "no_deployment_authority",
            Self::NoPublicDistributionAuthority => "no_public_distribution_authority",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidateEvidenceAssemblyCategoryStatus {
    Present,
    Missing,
    Blocked,
    Rejected,
    Deferred,
    NotApplicable,
}
impl ReleaseCandidateEvidenceAssemblyCategoryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Missing => "missing",
            Self::Blocked => "blocked",
            Self::Rejected => "rejected",
            Self::Deferred => "deferred",
            Self::NotApplicable => "not_applicable",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidateEvidenceAssemblyBoundaryStatus {
    EvidenceAssemblyOnly,
    ReviewSurfaceOnly,
    NonAuthoritativeReview,
    LocalOnlyNonPublic,
    ReleaseReadinessNotApproved,
    ReleaseCandidateStatusNotApproved,
    NoSigning,
    NoPublishing,
    NoDeploymentArtifact,
    NoPublicDistribution,
    NoPublicDownload,
    NoGithubRelease,
    NoReleaseTag,
    NoInstallerActivation,
    NoUpdateChannelActivation,
}
impl ReleaseCandidateEvidenceAssemblyBoundaryStatus {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EvidenceAssemblyOnly => "evidence_assembly_only",
            Self::ReviewSurfaceOnly => "review_surface_only",
            Self::NonAuthoritativeReview => "non_authoritative_review",
            Self::LocalOnlyNonPublic => "local_only_non_public",
            Self::ReleaseReadinessNotApproved => "release_readiness_not_approved",
            Self::ReleaseCandidateStatusNotApproved => "release_candidate_status_not_approved",
            Self::NoSigning => "no_signing",
            Self::NoPublishing => "no_publishing",
            Self::NoDeploymentArtifact => "no_deployment_artifact",
            Self::NoPublicDistribution => "no_public_distribution",
            Self::NoPublicDownload => "no_public_download",
            Self::NoGithubRelease => "no_github_release",
            Self::NoReleaseTag => "no_release_tag",
            Self::NoInstallerActivation => "no_installer_activation",
            Self::NoUpdateChannelActivation => "no_update_channel_activation",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceAssemblySourceLinkage {
    pub source_surface: String,
    pub source_status: String,
    pub source_summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceAssemblyItem {
    pub category: String,
    pub status: String,
    pub source_linkage: ReleaseCandidateEvidenceAssemblySourceLinkage,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceAssemblyMissingEvidence {
    pub category: String,
    pub reason: String,
    pub source_surface: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceAssemblyBlocker {
    pub category: String,
    pub reason: String,
    pub source_surface: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceAssemblyValidationSummary {
    pub source_surface: String,
    pub validation_status: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceAssemblyCapabilitySurface {
    pub approval_enabled: bool,
    pub signing_enabled: bool,
    pub publishing_enabled: bool,
    pub deployment_enabled: bool,
    pub public_distribution_enabled: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceAssemblyProjection {
    pub status: String,
    pub assembly_id: Option<String>,
    pub category_count: usize,
    pub present_category_count: usize,
    pub missing_category_count: usize,
    pub blocked_category_count: usize,
    pub rejected_category_count: usize,
    pub evidence_items: Vec<ReleaseCandidateEvidenceAssemblyItem>,
    pub missing_evidence: Vec<ReleaseCandidateEvidenceAssemblyMissingEvidence>,
    pub blockers: Vec<ReleaseCandidateEvidenceAssemblyBlocker>,
    pub validation_summaries: Vec<ReleaseCandidateEvidenceAssemblyValidationSummary>,
    pub boundary_statuses: Vec<String>,
    pub capability_surface: ReleaseCandidateEvidenceAssemblyCapabilitySurface,
}

fn stable_digest(input: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}
pub fn initial_release_candidate_evidence_assembly_projection(
) -> ReleaseCandidateEvidenceAssemblyProjection {
    ReleaseCandidateEvidenceAssemblyProjection {
        status: ReleaseCandidateEvidenceAssemblyStatus::NotAssembled
            .code()
            .into(),
        assembly_id: None,
        category_count: 0,
        present_category_count: 0,
        missing_category_count: 0,
        blocked_category_count: 0,
        rejected_category_count: 0,
        evidence_items: vec![],
        missing_evidence: vec![],
        blockers: vec![],
        validation_summaries: vec![],
        boundary_statuses: release_candidate_evidence_assembly_boundary_statuses(),
        capability_surface: ReleaseCandidateEvidenceAssemblyCapabilitySurface {
            approval_enabled: false,
            signing_enabled: false,
            publishing_enabled: false,
            deployment_enabled: false,
            public_distribution_enabled: false,
        },
    }
}
pub fn release_candidate_evidence_assembly_boundary_statuses() -> Vec<String> {
    vec![
        "evidence_assembly_only",
        "review_surface_only",
        "non_authoritative_review",
        "local_only_non_public",
        "release_readiness_not_approved",
        "release_candidate_status_not_approved",
        "no_signing",
        "no_publishing",
        "no_deployment_artifact",
        "no_public_distribution",
        "no_public_download",
        "no_github_release",
        "no_release_tag",
        "no_installer_activation",
        "no_update_channel_activation",
    ]
    .into_iter()
    .map(ToString::to_string)
    .collect()
}

pub fn derive_release_candidate_evidence_assembly(
    prep: &ReleaseCandidatePreparationProjection,
    dry: &ReleaseArtifactDryPackageProjection,
    cp: &ReleaseDryPackageChecksumProvenanceProjection,
    inst: &InstallerDistributionContractProjection,
    sign: &SigningKeyCustodyDryRunProjection,
) -> ReleaseCandidateEvidenceAssemblyProjection {
    let mut p = initial_release_candidate_evidence_assembly_projection();
    let mut items = vec![
        (
            ReleaseCandidateEvidenceAssemblyCategory::ReleaseCandidatePreparationContract,
            prep.status.code().to_string(),
            "phase_171_preparation_contract".to_string(),
        ),
        (
            ReleaseCandidateEvidenceAssemblyCategory::ReleaseArtifactDryPackage,
            dry.status.code().to_string(),
            "phase_172_dry_package".to_string(),
        ),
        (
            ReleaseCandidateEvidenceAssemblyCategory::DryPackageChecksumProvenance,
            cp.status.code().to_string(),
            "phase_173_checksum_provenance".to_string(),
        ),
        (
            ReleaseCandidateEvidenceAssemblyCategory::InstallerDistributionContract,
            inst.status.code().to_string(),
            "phase_174_installer_distribution_contract".to_string(),
        ),
        (
            ReleaseCandidateEvidenceAssemblyCategory::SigningKeyCustodyDryRun,
            sign.status.code().to_string(),
            "phase_176_signing_key_custody_dry_run".to_string(),
        ),
    ];
    items.sort_by_key(|x| x.0);
    for (c, s, src) in items {
        let st = if s.contains("rejected") {
            "rejected"
        } else if s.contains("blocked") {
            "blocked"
        } else if s.contains("not") || s.contains("invalid") {
            "missing"
        } else {
            "present"
        };
        p.evidence_items.push(ReleaseCandidateEvidenceAssemblyItem {
            category: c.code().into(),
            status: st.into(),
            source_linkage: ReleaseCandidateEvidenceAssemblySourceLinkage {
                source_surface: src,
                source_status: s,
                source_summary: "upstream evidence linkage".into(),
            },
        });
    }
    for item in &p.evidence_items {
        if item.status == "missing" {
            p.missing_evidence
                .push(ReleaseCandidateEvidenceAssemblyMissingEvidence {
                    category: item.category.clone(),
                    reason: "required upstream evidence missing".into(),
                    source_surface: item.source_linkage.source_surface.clone(),
                });
        }
        if item.status == "blocked" || item.status == "rejected" {
            p.blockers.push(ReleaseCandidateEvidenceAssemblyBlocker {
                category: item.category.clone(),
                reason: "upstream evidence blocked or rejected".into(),
                source_surface: item.source_linkage.source_surface.clone(),
            });
        }
        p.validation_summaries
            .push(ReleaseCandidateEvidenceAssemblyValidationSummary {
                source_surface: item.source_linkage.source_surface.clone(),
                validation_status: item.source_linkage.source_status.clone(),
            });
    }
    p.missing_evidence
        .sort_by(|a, b| a.category.cmp(&b.category));
    p.blockers.sort_by(|a, b| a.category.cmp(&b.category));
    p.category_count = p.evidence_items.len();
    p.present_category_count = p
        .evidence_items
        .iter()
        .filter(|x| x.status == "present")
        .count();
    p.missing_category_count = p.missing_evidence.len();
    p.blocked_category_count = p
        .evidence_items
        .iter()
        .filter(|x| x.status == "blocked")
        .count();
    p.rejected_category_count = p
        .evidence_items
        .iter()
        .filter(|x| x.status == "rejected")
        .count();
    let payload = format!(
        "{:?}{:?}{:?}",
        p.evidence_items, p.missing_evidence, p.blockers
    );
    p.assembly_id = Some(format!("rc-evidence-assembly-{}", stable_digest(&payload)));
    p.status = if p.rejected_category_count > 0 {
        "evidence_rejected".into()
    } else if p.blocked_category_count > 0 {
        "evidence_blocked".into()
    } else if p.missing_category_count > 0 {
        "evidence_incomplete".into()
    } else {
        "evidence_assembled".into()
    };
    p
}
