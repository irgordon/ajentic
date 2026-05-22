use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidateGapReviewStatus { NotReviewed, GapReviewProjected, NoBlockingGaps, InformationalGapsOnly, HardeningRequired, GapReviewBlocked, InvalidGapReviewInput }
impl ReleaseCandidateGapReviewStatus { pub fn code(&self)->&'static str { match self { Self::NotReviewed=>"not_reviewed", Self::GapReviewProjected=>"gap_review_projected", Self::NoBlockingGaps=>"no_blocking_gaps", Self::InformationalGapsOnly=>"informational_gaps_only", Self::HardeningRequired=>"hardening_required", Self::GapReviewBlocked=>"gap_review_blocked", Self::InvalidGapReviewInput=>"invalid_gap_review_input" } } }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateGapCategory { MissingUpstreamEvidence, RejectedUpstreamEvidence, IncompleteUpstreamEvidence, ValidationCoverageGap, DocumentationHelpGap, DryPackageGap, ChecksumProvenanceGap, InstallerDistributionContractGap, SigningKeyCustodyDryRunGap, EvidenceAssemblyGap, NoAuthorityBoundaryConfirmation }
impl ReleaseCandidateGapCategory { pub fn code(&self)->&'static str { match self { Self::MissingUpstreamEvidence=>"missing_upstream_evidence", Self::RejectedUpstreamEvidence=>"rejected_upstream_evidence", Self::IncompleteUpstreamEvidence=>"incomplete_upstream_evidence", Self::ValidationCoverageGap=>"validation_coverage_gap", Self::DocumentationHelpGap=>"documentation_help_gap", Self::DryPackageGap=>"dry_package_gap", Self::ChecksumProvenanceGap=>"checksum_provenance_gap", Self::InstallerDistributionContractGap=>"installer_distribution_contract_gap", Self::SigningKeyCustodyDryRunGap=>"signing_key_custody_dry_run_gap", Self::EvidenceAssemblyGap=>"evidence_assembly_gap", Self::NoAuthorityBoundaryConfirmation=>"no_authority_boundary_confirmation" } } }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateGapSeverity { Info, Low, Medium, High, Blocking }
impl ReleaseCandidateGapSeverity { pub fn code(&self)->&'static str { match self { Self::Info=>"info", Self::Low=>"low", Self::Medium=>"medium", Self::High=>"high", Self::Blocking=>"blocking" } } }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateGapAffectedSurface { ReleaseCandidatePreparationContract, ReleaseArtifactDryPackage, DryPackageChecksumProvenance, InstallerDistributionContract, SigningKeyCustodyDryRun, ReleaseCandidateEvidenceAssembly, LocalBetaWorkflow, UserHelp, Validation, NoAuthorityBoundary }
impl ReleaseCandidateGapAffectedSurface { pub fn code(&self)->&'static str { match self { Self::ReleaseCandidatePreparationContract=>"release_candidate_preparation_contract", Self::ReleaseArtifactDryPackage=>"release_artifact_dry_package", Self::DryPackageChecksumProvenance=>"dry_package_checksum_provenance", Self::InstallerDistributionContract=>"installer_distribution_contract", Self::SigningKeyCustodyDryRun=>"signing_key_custody_dry_run", Self::ReleaseCandidateEvidenceAssembly=>"release_candidate_evidence_assembly", Self::LocalBetaWorkflow=>"local_beta_workflow", Self::UserHelp=>"user_help", Self::Validation=>"validation", Self::NoAuthorityBoundary=>"no_authority_boundary" } } }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateGapReviewSourceLinkage { pub source_surface: String, pub source_status: String, pub source_summary: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateGapFinding { pub category: String, pub severity: String, pub affected_surface: String, pub finding: String, pub source_linkage: ReleaseCandidateGapReviewSourceLinkage }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateHardeningCandidate { pub summary: String, pub priority: String, pub linked_gap_category: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateGapReviewMissingEvidenceSummary { pub source_surface: String, pub reason: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateGapReviewBlockerSummary { pub source_surface: String, pub reason: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateGapReviewValidationSummary { pub source_surface: String, pub validation_status: String }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidateGapReviewBoundaryStatus { GapReviewOnly, HardeningCandidatesOnly, NonAuthoritativeReview, LocalOnlyNonPublic, ReleaseReadinessNotApproved, ReleaseCandidateStatusNotApproved, ProductionStatusNotApproved, NoRealSigningKeys, NoSignatureCreated, NoSigning, NoPublishing, NoInstallerActivation, NoUpdateChannelActivation, NoPublicDistribution, NoPublicDownload, NoGithubRelease, NoReleaseTag, NoDeploymentArtifact, NoProviderTrust, NoActionAuthorization, NoReplayRepair, NoRecoveryPromotion }
impl ReleaseCandidateGapReviewBoundaryStatus { pub fn code(&self)->&'static str { match self { Self::GapReviewOnly=>"gap_review_only", Self::HardeningCandidatesOnly=>"hardening_candidates_only", Self::NonAuthoritativeReview=>"non_authoritative_review", Self::LocalOnlyNonPublic=>"local_only_non_public", Self::ReleaseReadinessNotApproved=>"release_readiness_not_approved", Self::ReleaseCandidateStatusNotApproved=>"release_candidate_status_not_approved", Self::ProductionStatusNotApproved=>"production_status_not_approved", Self::NoRealSigningKeys=>"no_real_signing_keys", Self::NoSignatureCreated=>"no_signature_created", Self::NoSigning=>"no_signing", Self::NoPublishing=>"no_publishing", Self::NoInstallerActivation=>"no_installer_activation", Self::NoUpdateChannelActivation=>"no_update_channel_activation", Self::NoPublicDistribution=>"no_public_distribution", Self::NoPublicDownload=>"no_public_download", Self::NoGithubRelease=>"no_github_release", Self::NoReleaseTag=>"no_release_tag", Self::NoDeploymentArtifact=>"no_deployment_artifact", Self::NoProviderTrust=>"no_provider_trust", Self::NoActionAuthorization=>"no_action_authorization", Self::NoReplayRepair=>"no_replay_repair", Self::NoRecoveryPromotion=>"no_recovery_promotion" } } }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateGapReviewCapabilitySurface { pub approval_enabled: bool, pub signing_enabled: bool, pub publishing_enabled: bool, pub deployment_enabled: bool, pub public_distribution_enabled: bool }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateGapReviewProjection { pub status: String, pub gap_review_id: Option<String>, pub gap_count: usize, pub blocking_gap_count: usize, pub informational_gap_count: usize, pub hardening_candidate_count: usize, pub source_linkage: Vec<ReleaseCandidateGapReviewSourceLinkage>, pub gaps: Vec<ReleaseCandidateGapFinding>, pub hardening_candidates: Vec<ReleaseCandidateHardeningCandidate>, pub missing_evidence: Vec<ReleaseCandidateGapReviewMissingEvidenceSummary>, pub blockers: Vec<ReleaseCandidateGapReviewBlockerSummary>, pub validation_summaries: Vec<ReleaseCandidateGapReviewValidationSummary>, pub boundary_statuses: Vec<String>, pub capability_surface: ReleaseCandidateGapReviewCapabilitySurface }

pub fn initial_release_candidate_gap_review_projection() -> ReleaseCandidateGapReviewProjection { ReleaseCandidateGapReviewProjection { status: ReleaseCandidateGapReviewStatus::NotReviewed.code().into(), gap_review_id: None, gap_count:0, blocking_gap_count:0, informational_gap_count:0, hardening_candidate_count:0, source_linkage: vec![], gaps: vec![], hardening_candidates: vec![], missing_evidence: vec![], blockers: vec![], validation_summaries: vec![], boundary_statuses: release_candidate_gap_review_boundary_statuses(), capability_surface: ReleaseCandidateGapReviewCapabilitySurface{approval_enabled:false,signing_enabled:false,publishing_enabled:false,deployment_enabled:false,public_distribution_enabled:false}} }
pub fn release_candidate_gap_review_boundary_statuses() -> Vec<String> { [ReleaseCandidateGapReviewBoundaryStatus::GapReviewOnly,ReleaseCandidateGapReviewBoundaryStatus::HardeningCandidatesOnly,ReleaseCandidateGapReviewBoundaryStatus::NonAuthoritativeReview,ReleaseCandidateGapReviewBoundaryStatus::LocalOnlyNonPublic,ReleaseCandidateGapReviewBoundaryStatus::ReleaseReadinessNotApproved,ReleaseCandidateGapReviewBoundaryStatus::ReleaseCandidateStatusNotApproved,ReleaseCandidateGapReviewBoundaryStatus::NoSigning,ReleaseCandidateGapReviewBoundaryStatus::NoPublishing,ReleaseCandidateGapReviewBoundaryStatus::NoDeploymentArtifact,ReleaseCandidateGapReviewBoundaryStatus::NoPublicDistribution,ReleaseCandidateGapReviewBoundaryStatus::NoPublicDownload,ReleaseCandidateGapReviewBoundaryStatus::NoGithubRelease,ReleaseCandidateGapReviewBoundaryStatus::NoReleaseTag,ReleaseCandidateGapReviewBoundaryStatus::NoInstallerActivation,ReleaseCandidateGapReviewBoundaryStatus::NoUpdateChannelActivation,ReleaseCandidateGapReviewBoundaryStatus::ProductionStatusNotApproved,ReleaseCandidateGapReviewBoundaryStatus::NoRealSigningKeys,ReleaseCandidateGapReviewBoundaryStatus::NoSignatureCreated,ReleaseCandidateGapReviewBoundaryStatus::NoProviderTrust,ReleaseCandidateGapReviewBoundaryStatus::NoActionAuthorization,ReleaseCandidateGapReviewBoundaryStatus::NoReplayRepair,ReleaseCandidateGapReviewBoundaryStatus::NoRecoveryPromotion].into_iter().map(|x|x.code().to_string()).collect() }

fn is_missing(s:&str)->bool{ s.contains("missing") || s=="not_prepared" || s=="not_packaged" || s=="not_reviewed" || s=="not_assembled" }
fn is_rejected(s:&str)->bool{ s.contains("reject") }

pub fn derive_release_candidate_gap_review(prep:&ReleaseCandidatePreparationProjection,dry:&ReleaseArtifactDryPackageProjection,cp:&ReleaseDryPackageChecksumProvenanceProjection,inst:&InstallerDistributionContractProjection,sign:&SigningKeyCustodyDryRunProjection,assembly:&ReleaseCandidateEvidenceAssemblyProjection)->ReleaseCandidateGapReviewProjection{
 let mut p=initial_release_candidate_gap_review_projection();
 let links=vec![("phase_171_preparation_contract",prep.status.code()),("phase_172_dry_package",dry.status.code()),("phase_173_checksum_provenance",cp.status.code()),("phase_174_installer_distribution",inst.status.code()),("phase_176_signing_key_custody_dry_run",sign.status.code()),("phase_177_evidence_assembly",assembly.status.as_str())];
 p.source_linkage=links.iter().map(|(a,b)|ReleaseCandidateGapReviewSourceLinkage{source_surface:(*a).into(),source_status:(*b).into(),source_summary:format!("{a} status is {b}")}).collect();
 let mut gaps=Vec::new();
 for (surface,status,affected) in [("phase_171_preparation_contract",prep.status.code(),ReleaseCandidateGapAffectedSurface::ReleaseCandidatePreparationContract),("phase_172_dry_package",dry.status.code(),ReleaseCandidateGapAffectedSurface::ReleaseArtifactDryPackage),("phase_173_checksum_provenance",cp.status.code(),ReleaseCandidateGapAffectedSurface::DryPackageChecksumProvenance),("phase_174_installer_distribution",inst.status.code(),ReleaseCandidateGapAffectedSurface::InstallerDistributionContract),("phase_176_signing_key_custody_dry_run",sign.status.code(),ReleaseCandidateGapAffectedSurface::SigningKeyCustodyDryRun),("phase_177_evidence_assembly",assembly.status.as_str(),ReleaseCandidateGapAffectedSurface::ReleaseCandidateEvidenceAssembly)] {
  if is_missing(status){ gaps.push((ReleaseCandidateGapCategory::MissingUpstreamEvidence,ReleaseCandidateGapSeverity::Blocking,affected,surface.to_string(),format!("{surface} is missing"))); }
  if is_rejected(status){ gaps.push((ReleaseCandidateGapCategory::RejectedUpstreamEvidence,ReleaseCandidateGapSeverity::High,affected,surface.to_string(),format!("{surface} is rejected"))); }
 }
 if gaps.is_empty(){ gaps.push((ReleaseCandidateGapCategory::NoAuthorityBoundaryConfirmation,ReleaseCandidateGapSeverity::Info,ReleaseCandidateGapAffectedSurface::NoAuthorityBoundary,"boundary".into(),"No-authority boundary confirmations remain enforced".into())); }
 gaps.sort_by_key(|g|(g.0,g.1,g.2,g.3.clone()));
 p.gaps=gaps.iter().map(|(c,s,a,source,f)|ReleaseCandidateGapFinding{category:c.code().into(),severity:s.code().into(),affected_surface:a.code().into(),finding:f.clone(),source_linkage:ReleaseCandidateGapReviewSourceLinkage{source_surface:source.clone(),source_status:"derived".into(),source_summary:f.clone()}}).collect();
 p.missing_evidence=p.gaps.iter().filter(|g|g.category=="missing_upstream_evidence").map(|g|ReleaseCandidateGapReviewMissingEvidenceSummary{source_surface:g.source_linkage.source_surface.clone(),reason:g.finding.clone()}).collect();
 p.blockers=p.gaps.iter().filter(|g|g.severity=="blocking").map(|g|ReleaseCandidateGapReviewBlockerSummary{source_surface:g.source_linkage.source_surface.clone(),reason:g.finding.clone()}).collect();
 p.validation_summaries=vec![ReleaseCandidateGapReviewValidationSummary{source_surface:"gap_review".into(),validation_status:"deterministic_local_projection".into()}];
 p.hardening_candidates=p.gaps.iter().filter(|g| g.severity=="blocking"||g.severity=="high").map(|g|ReleaseCandidateHardeningCandidate{summary:format!("Harden {} for {}",g.source_linkage.source_surface,g.category),priority:g.severity.clone(),linked_gap_category:g.category.clone()}).collect();
 p.hardening_candidates.sort_by(|a,b|a.summary.cmp(&b.summary));
 p.gap_count=p.gaps.len(); p.blocking_gap_count=p.gaps.iter().filter(|g|g.severity=="blocking").count(); p.informational_gap_count=p.gaps.iter().filter(|g|g.severity=="info").count(); p.hardening_candidate_count=p.hardening_candidates.len();
 let payload=format!("{:?}{:?}",p.gaps,p.source_linkage); let mut h:u64=0xcbf29ce484222325; for b in payload.as_bytes(){h^=*b as u64; h=h.wrapping_mul(0x100000001b3);} p.gap_review_id=Some(format!("release-candidate-gap-review-{h:016x}"));
 p.status= if p.blocking_gap_count>0 {"hardening_required".into()} else if p.gap_count==0 {"no_blocking_gaps".into()} else if p.hardening_candidate_count==0 {"informational_gaps_only".into()} else {"gap_review_projected".into()};
 p
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{initial_installer_distribution_contract_projection, initial_release_artifact_dry_package_projection, initial_release_candidate_evidence_assembly_projection, initial_release_candidate_preparation_projection, initial_release_dry_package_checksum_provenance_projection, initial_signing_key_custody_dry_run_projection};

    #[test]
    fn deterministic_gap_review_id() {
        let p = derive_release_candidate_gap_review(&initial_release_candidate_preparation_projection(), &initial_release_artifact_dry_package_projection(), &initial_release_dry_package_checksum_provenance_projection(), &initial_installer_distribution_contract_projection(), &initial_signing_key_custody_dry_run_projection(), &initial_release_candidate_evidence_assembly_projection());
        let q = derive_release_candidate_gap_review(&initial_release_candidate_preparation_projection(), &initial_release_artifact_dry_package_projection(), &initial_release_dry_package_checksum_provenance_projection(), &initial_installer_distribution_contract_projection(), &initial_signing_key_custody_dry_run_projection(), &initial_release_candidate_evidence_assembly_projection());
        assert_eq!(p.gap_review_id, q.gap_review_id);
    }
}
