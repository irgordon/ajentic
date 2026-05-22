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
pub enum ReleaseCandidateDryRunRehearsalUpstreamCategory { ReleaseCandidatePreparationContract, ReleaseArtifactDryPackage, DryPackageChecksumProvenance, InstallerDistributionContract, SigningKeyCustodyDryRun, ReleaseCandidateEvidenceAssembly, ReleaseCandidateGapReview }
impl ReleaseCandidateDryRunRehearsalUpstreamCategory { pub fn code(&self) -> &'static str { match self { Self::ReleaseCandidatePreparationContract => "release_candidate_preparation_contract", Self::ReleaseArtifactDryPackage => "release_artifact_dry_package", Self::DryPackageChecksumProvenance => "dry_package_checksum_provenance", Self::InstallerDistributionContract => "installer_distribution_contract", Self::SigningKeyCustodyDryRun => "signing_key_custody_dry_run", Self::ReleaseCandidateEvidenceAssembly => "release_candidate_evidence_assembly", Self::ReleaseCandidateGapReview => "release_candidate_gap_review" }}}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalUpstreamLinkage { pub category: String, pub source_id: String, pub source_status: String, pub source_summary: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalMissingEvidence { pub category: String, pub reason: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalBlocker { pub category: String, pub reason: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalGapReviewSummary { pub gap_review_id: String, pub status: String, pub blocking_gap_count: usize, pub gap_count: usize }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalArtifactSummary { pub artifact_id: String, pub artifact_kind: String, pub summary: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalValidationSummary { pub status: String, pub has_missing_evidence: bool, pub has_blockers: bool }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateDryRunRehearsalCapabilitySurface { pub approval_enabled: bool, pub signing_enabled: bool, pub publishing_enabled: bool, pub deployment_enabled: bool, pub public_distribution_enabled: bool }
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

pub fn initial_release_candidate_dry_run_rehearsal_projection() -> ReleaseCandidateDryRunRehearsalProjection { ReleaseCandidateDryRunRehearsalProjection { status: ReleaseCandidateDryRunRehearsalStatus::NotRehearsed.code().into(), rehearsal_id: None, upstream_linkage: vec![], missing_evidence: vec![], blockers: vec![], gap_review_summary: ReleaseCandidateDryRunRehearsalGapReviewSummary { gap_review_id: "not_rehearsed".into(), status: "not_reviewed".into(), blocking_gap_count: 0, gap_count: 0 }, rehearsal_artifact_summary: vec![], validation_summary: ReleaseCandidateDryRunRehearsalValidationSummary { status: "not_rehearsed".into(), has_missing_evidence: false, has_blockers: false }, boundary_statuses: vec!["dry_run_rehearsal_only".into(), "local_rehearsal_only".into(), "non_authoritative_rehearsal".into(), "local_only_non_public".into(), "release_candidate_status_not_approved".into(), "release_readiness_not_approved".into(), "production_status_not_approved".into(), "public_use_not_approved".into(), "release_artifact_not_created".into(), "public_artifact_not_created".into(), "deployment_artifact_not_created".into(), "no_real_signing_keys".into(), "no_signature_created".into(), "no_signing".into(), "no_publishing".into(), "no_installer_activation".into(), "no_update_channel_activation".into(), "no_public_distribution".into(), "no_public_download".into(), "no_github_release".into(), "no_release_tag".into(), "no_deployment_artifact".into(), "no_provider_trust".into(), "no_action_authorization".into(), "no_replay_repair".into(), "no_recovery_promotion".into()], capability_surface: ReleaseCandidateDryRunRehearsalCapabilitySurface { approval_enabled: false, signing_enabled: false, publishing_enabled: false, deployment_enabled: false, public_distribution_enabled: false } }}

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
        linkage(ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseCandidatePreparationContract, preparation.preparation_id.clone(), preparation.status.code().to_string()),
        linkage(ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseArtifactDryPackage, dry_package.dry_package_id.clone().unwrap_or_else(|| "missing_dry_package".into()), dry_package.status.code().to_string()),
        linkage(ReleaseCandidateDryRunRehearsalUpstreamCategory::DryPackageChecksumProvenance, checksum.provenance_id.clone().unwrap_or_else(|| "missing_checksum_provenance".into()), checksum.status.code().to_string()),
        linkage(ReleaseCandidateDryRunRehearsalUpstreamCategory::InstallerDistributionContract, installer.contract_id.clone().unwrap_or_else(|| "missing_installer_distribution_contract".into()), installer.status.code().to_string()),
        linkage(ReleaseCandidateDryRunRehearsalUpstreamCategory::SigningKeyCustodyDryRun, signing.evidence.as_ref().map(|e| e.evidence_id.clone()).unwrap_or_else(|| "missing_signing_key_custody_dry_run".into()), signing.status.code().to_string()),
        linkage(ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseCandidateEvidenceAssembly, assembly.assembly_id.clone().unwrap_or_else(|| "missing_release_candidate_evidence_assembly".into()), assembly.status.clone()),
        linkage(ReleaseCandidateDryRunRehearsalUpstreamCategory::ReleaseCandidateGapReview, gap.gap_review_id.clone().unwrap_or_else(|| "missing_release_candidate_gap_review".into()), gap.status.clone()),
    ];
    p.gap_review_summary = ReleaseCandidateDryRunRehearsalGapReviewSummary { gap_review_id: gap.gap_review_id.clone().unwrap_or_else(|| "missing_release_candidate_gap_review".into()), status: gap.status.clone(), blocking_gap_count: gap.blocking_gap_count, gap_count: gap.gap_count };

    for link in &p.upstream_linkage {
        if link.source_id.starts_with("missing_") || link.source_status.contains("not_") {
            p.missing_evidence.push(ReleaseCandidateDryRunRehearsalMissingEvidence { category: link.category.clone(), reason: format!("missing_{}", link.category) });
        }
        if link.source_status.contains("blocked") || link.source_status.contains("invalid") {
            p.blockers.push(ReleaseCandidateDryRunRehearsalBlocker { category: link.category.clone(), reason: format!("blocked_{}", link.category) });
        }
        if link.source_status.contains("rejected") {
            p.blockers.push(ReleaseCandidateDryRunRehearsalBlocker { category: link.category.clone(), reason: format!("rejected_{}", link.category) });
        }
    }
    if gap.blocking_gap_count > 0 { p.blockers.push(ReleaseCandidateDryRunRehearsalBlocker { category: "release_candidate_gap_review".into(), reason: "blocking_gap_review_findings".into() }); }
    p.missing_evidence.sort_by(|a,b| a.category.cmp(&b.category).then(a.reason.cmp(&b.reason)));
    p.blockers.sort_by(|a,b| a.category.cmp(&b.category).then(a.reason.cmp(&b.reason)));

    if p.missing_evidence.is_empty() && p.blockers.is_empty() {
        p.rehearsal_id = Some(format!("rc-dry-run-rehearsal-{}-{}", preparation.preparation_id, gap.gap_review_id.clone().unwrap_or_default()));
        p.status = ReleaseCandidateDryRunRehearsalStatus::RehearsalCompletedLocally.code().into();
        p.rehearsal_artifact_summary = vec![ReleaseCandidateDryRunRehearsalArtifactSummary { artifact_id: p.rehearsal_id.clone().unwrap_or_default(), artifact_kind: "local_rehearsal_evidence_artifact".into(), summary: "Local deterministic rehearsal evidence chain projection summary.".into() }];
    } else if p.blockers.iter().any(|b| b.reason.contains("rejected")) {
        p.status = ReleaseCandidateDryRunRehearsalStatus::RehearsalRejected.code().into();
    } else {
        p.status = ReleaseCandidateDryRunRehearsalStatus::RehearsalBlocked.code().into();
    }
    p.validation_summary = ReleaseCandidateDryRunRehearsalValidationSummary { status: if p.missing_evidence.is_empty() && p.blockers.is_empty() { "valid".into() } else { "invalid".into() }, has_missing_evidence: !p.missing_evidence.is_empty(), has_blockers: !p.blockers.is_empty() };
    p
}

fn linkage(category: ReleaseCandidateDryRunRehearsalUpstreamCategory, source_id: String, source_status: String) -> ReleaseCandidateDryRunRehearsalUpstreamLinkage { ReleaseCandidateDryRunRehearsalUpstreamLinkage { category: category.code().into(), source_summary: format!("{}:{}", source_id, source_status), source_id, source_status }}

#[cfg(test)]
mod tests { use super::*; use crate::api::initial_local_operator_shell_state;
    #[test] fn rehearsal_deterministic_and_boundaries() { let s = initial_local_operator_shell_state(); let p1 = derive_release_candidate_dry_run_rehearsal(&s.release_candidate_preparation,&s.release_artifact_dry_package,&s.release_dry_package_checksum_provenance,&s.installer_distribution_contract,&s.signing_key_custody_dry_run,&s.release_candidate_evidence_assembly,&s.release_candidate_gap_review); let p2 = derive_release_candidate_dry_run_rehearsal(&s.release_candidate_preparation,&s.release_artifact_dry_package,&s.release_dry_package_checksum_provenance,&s.installer_distribution_contract,&s.signing_key_custody_dry_run,&s.release_candidate_evidence_assembly,&s.release_candidate_gap_review); assert_eq!(p1,p2); assert!(p1.boundary_statuses.contains(&"dry_run_rehearsal_only".into())); }
}
