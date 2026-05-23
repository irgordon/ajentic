use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidateLabelStatus { NotLabeled, SupportableWithCaveats, DeferredForHardening, BlockedByEvidenceGaps, BlockedByGuardrailDrift }
impl ReleaseCandidateLabelStatus { pub fn code(self)->&'static str { match self { Self::NotLabeled=>"not_labeled", Self::SupportableWithCaveats=>"supportable_with_caveats", Self::DeferredForHardening=>"deferred_for_hardening", Self::BlockedByEvidenceGaps=>"blocked_by_evidence_gaps", Self::BlockedByGuardrailDrift=>"blocked_by_guardrail_drift" }}}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidateEvidenceManifestStatus { NotGenerated, ManifestProjected, ManifestCompleteWithCaveats, ManifestIncomplete, ManifestBlocked, InvalidManifestInput }
impl ReleaseCandidateEvidenceManifestStatus { pub fn code(self)->&'static str { match self { Self::NotGenerated=>"not_generated", Self::ManifestProjected=>"manifest_projected", Self::ManifestCompleteWithCaveats=>"manifest_complete_with_caveats", Self::ManifestIncomplete=>"manifest_incomplete", Self::ManifestBlocked=>"manifest_blocked", Self::InvalidManifestInput=>"invalid_manifest_input" }}}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateEvidenceManifestCategory { ReleaseCandidatePreparationContract, ReleaseArtifactDryPackage, DryPackageChecksumProvenance, InstallerDistributionContract, SigningKeyCustodyDryRun, ReleaseCandidateEvidenceAssembly, ReleaseCandidateGapReview, ReleaseCandidateDryRunRehearsal }
impl ReleaseCandidateEvidenceManifestCategory { pub fn code(self)->&'static str { match self { Self::ReleaseCandidatePreparationContract=>"release_candidate_preparation_contract", Self::ReleaseArtifactDryPackage=>"release_artifact_dry_package", Self::DryPackageChecksumProvenance=>"dry_package_checksum_provenance", Self::InstallerDistributionContract=>"installer_distribution_contract", Self::SigningKeyCustodyDryRun=>"signing_key_custody_dry_run", Self::ReleaseCandidateEvidenceAssembly=>"release_candidate_evidence_assembly", Self::ReleaseCandidateGapReview=>"release_candidate_gap_review", Self::ReleaseCandidateDryRunRehearsal=>"release_candidate_dry_run_rehearsal" }}}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidateEvidenceManifestItemStatus { Present, Missing, Rejected, Blocked, Deferred, Informational }
impl ReleaseCandidateEvidenceManifestItemStatus { pub fn code(self)->&'static str { match self { Self::Present=>"present", Self::Missing=>"missing", Self::Rejected=>"rejected", Self::Blocked=>"blocked", Self::Deferred=>"deferred", Self::Informational=>"informational" }}}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceManifestSourceLinkage { pub source_surface:String, pub source_status:String, pub source_id:String, pub source_summary:String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceManifestItem { pub category:String, pub status:String, pub linkage:ReleaseCandidateEvidenceManifestSourceLinkage }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceManifestBlocker { pub category:String, pub reason:String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceManifestCaveat { pub category:String, pub detail:String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceManifestValidationSummary { pub item_count:usize, pub present_count:usize, pub missing_count:usize, pub blocked_count:usize, pub rejected_count:usize }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceManifestCapabilitySurface { pub approval_enabled:bool, pub signing_enabled:bool, pub publishing_enabled:bool, pub deployment_enabled:bool, pub public_distribution_enabled:bool }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateEvidenceManifestProjection { pub label_status:String, pub manifest_status:String, pub manifest_id:Option<String>, pub items:Vec<ReleaseCandidateEvidenceManifestItem>, pub blockers:Vec<ReleaseCandidateEvidenceManifestBlocker>, pub caveats:Vec<ReleaseCandidateEvidenceManifestCaveat>, pub validation_summary:ReleaseCandidateEvidenceManifestValidationSummary, pub boundary_statuses:Vec<String>, pub capability_surface:ReleaseCandidateEvidenceManifestCapabilitySurface }

fn class_preparation(s: ReleaseCandidatePreparationStatus)->ReleaseCandidateEvidenceManifestItemStatus{match s{ReleaseCandidatePreparationStatus::NotPrepared=>ReleaseCandidateEvidenceManifestItemStatus::Missing,ReleaseCandidatePreparationStatus::PreparationRejected=>ReleaseCandidateEvidenceManifestItemStatus::Rejected,ReleaseCandidatePreparationStatus::PreparationBlocked|ReleaseCandidatePreparationStatus::InvalidPreparationInput=>ReleaseCandidateEvidenceManifestItemStatus::Blocked,_=>ReleaseCandidateEvidenceManifestItemStatus::Present}}
fn class_dry(s: ReleaseArtifactDryPackageStatus)->ReleaseCandidateEvidenceManifestItemStatus{match s{ReleaseArtifactDryPackageStatus::NotAssembled=>ReleaseCandidateEvidenceManifestItemStatus::Missing,ReleaseArtifactDryPackageStatus::DryPackageRejected=>ReleaseCandidateEvidenceManifestItemStatus::Rejected,ReleaseArtifactDryPackageStatus::InvalidDryPackageInput=>ReleaseCandidateEvidenceManifestItemStatus::Blocked,_=>ReleaseCandidateEvidenceManifestItemStatus::Present}}
fn class_str(status:&str)->ReleaseCandidateEvidenceManifestItemStatus{match status{"not_generated"|"not_defined"|"not_rehearsed"|"not_reviewed"=>ReleaseCandidateEvidenceManifestItemStatus::Missing,"rejected"|"checksum_provenance_rejected"|"rehearsal_rejected"|"gap_review_blocked"=>ReleaseCandidateEvidenceManifestItemStatus::Rejected,"invalid_checksum_provenance_input"|"invalid_contract_input"|"invalid_signing_dry_run_input"|"invalid_evidence_assembly_input"|"invalid_gap_review_input"|"invalid_rehearsal_input"|"rehearsal_blocked"=>ReleaseCandidateEvidenceManifestItemStatus::Blocked,_=>ReleaseCandidateEvidenceManifestItemStatus::Present}}

pub fn initial_release_candidate_evidence_manifest_projection()->ReleaseCandidateEvidenceManifestProjection{ReleaseCandidateEvidenceManifestProjection{label_status:ReleaseCandidateLabelStatus::NotLabeled.code().into(),manifest_status:ReleaseCandidateEvidenceManifestStatus::NotGenerated.code().into(),manifest_id:None,items:vec![],blockers:vec![],caveats:release_candidate_evidence_manifest_caveats(),validation_summary:ReleaseCandidateEvidenceManifestValidationSummary{item_count:0,present_count:0,missing_count:0,blocked_count:0,rejected_count:0},boundary_statuses:release_candidate_evidence_manifest_boundary_statuses(),capability_surface:release_candidate_evidence_manifest_capability_surface()}}

pub fn derive_release_candidate_evidence_manifest(state:&LocalOperatorShellState)->ReleaseCandidateEvidenceManifestProjection{let mut items=vec![
item(ReleaseCandidateEvidenceManifestCategory::ReleaseCandidatePreparationContract,class_preparation(state.release_candidate_preparation.status),"release_candidate_preparation",state.release_candidate_preparation.status.code(),Some(state.release_candidate_preparation.preparation_id.as_str()),"phase 171 preparation contract"),
item(ReleaseCandidateEvidenceManifestCategory::ReleaseArtifactDryPackage,class_dry(state.release_artifact_dry_package.status),"release_artifact_dry_package",state.release_artifact_dry_package.status.code(),state.release_artifact_dry_package.dry_package_id.as_deref(),"phase 172 dry package"),
item(ReleaseCandidateEvidenceManifestCategory::DryPackageChecksumProvenance,class_str(&state.release_dry_package_checksum_provenance.status),"release_dry_package_checksum_provenance",&state.release_dry_package_checksum_provenance.status,state.release_dry_package_checksum_provenance.provenance_id.as_deref(),"phase 173 checksum/provenance"),
item(ReleaseCandidateEvidenceManifestCategory::InstallerDistributionContract,class_str(&state.installer_distribution_contract.status),"installer_distribution_contract",&state.installer_distribution_contract.status,state.installer_distribution_contract.contract_id.as_deref(),"phase 174 installer/distribution contract"),
item(ReleaseCandidateEvidenceManifestCategory::SigningKeyCustodyDryRun,class_str(&state.signing_key_custody_dry_run.status),"signing_key_custody_dry_run",&state.signing_key_custody_dry_run.status,state.signing_key_custody_dry_run.evidence_id.as_deref(),"phase 176 signing/key-custody dry run"),
item(ReleaseCandidateEvidenceManifestCategory::ReleaseCandidateEvidenceAssembly,class_str(&state.release_candidate_evidence_assembly.status),"release_candidate_evidence_assembly",&state.release_candidate_evidence_assembly.status,state.release_candidate_evidence_assembly.assembly_id.as_deref(),"phase 177 evidence assembly"),
item(ReleaseCandidateEvidenceManifestCategory::ReleaseCandidateGapReview,class_str(&state.release_candidate_gap_review.status),"release_candidate_gap_review",&state.release_candidate_gap_review.status,state.release_candidate_gap_review.gap_review_id.as_deref(),"phase 178 gap review"),
item(ReleaseCandidateEvidenceManifestCategory::ReleaseCandidateDryRunRehearsal,class_str(&state.release_candidate_dry_run_rehearsal.status),"release_candidate_dry_run_rehearsal",&state.release_candidate_dry_run_rehearsal.status,state.release_candidate_dry_run_rehearsal.rehearsal_id.as_deref(),"phase 179 dry-run rehearsal")];
items.sort_by(|a,b| a.category.cmp(&b.category));
let mut blockers:Vec<_>=items.iter().filter_map(|x|match x.status.as_str(){"missing"|"blocked"|"rejected"=>Some(ReleaseCandidateEvidenceManifestBlocker{category:x.category.clone(),reason:format!("{} evidence is {}",x.category,x.status)}),_=>None}).collect();
if state.release_candidate_dry_run_rehearsal.status=="rehearsal_blocked"{blockers.push(ReleaseCandidateEvidenceManifestBlocker{category:"release_candidate_dry_run_rehearsal".into(),reason:"rehearsal is blocked".into()});}
if state.release_candidate_gap_review.blocking_gap_count>0{blockers.push(ReleaseCandidateEvidenceManifestBlocker{category:"release_candidate_gap_review".into(),reason:"blocking gaps remain".into()});}
blockers.sort_by(|a,b| a.category.cmp(&b.category));
let caveats=release_candidate_evidence_manifest_caveats();
let summary=ReleaseCandidateEvidenceManifestValidationSummary{item_count:items.len(),present_count:items.iter().filter(|x|x.status=="present").count(),missing_count:items.iter().filter(|x|x.status=="missing").count(),blocked_count:items.iter().filter(|x|x.status=="blocked").count(),rejected_count:items.iter().filter(|x|x.status=="rejected").count()};
let label= if blockers.is_empty(){ReleaseCandidateLabelStatus::SupportableWithCaveats}else{ReleaseCandidateLabelStatus::BlockedByEvidenceGaps};
let manifest_status= if blockers.is_empty(){ReleaseCandidateEvidenceManifestStatus::ManifestCompleteWithCaveats}else{ReleaseCandidateEvidenceManifestStatus::ManifestBlocked};
let manifest_id=Some(format!("rc-evidence-manifest-{}-{}-{}",summary.item_count,summary.present_count,blockers.len()));
ReleaseCandidateEvidenceManifestProjection{label_status:label.code().into(),manifest_status:manifest_status.code().into(),manifest_id,items,blockers,caveats,validation_summary:summary,boundary_statuses:release_candidate_evidence_manifest_boundary_statuses(),capability_surface:release_candidate_evidence_manifest_capability_surface()}
}
fn item(c:ReleaseCandidateEvidenceManifestCategory,s:ReleaseCandidateEvidenceManifestItemStatus,surface:&str,status:&str,id:Option<&str>,summary:&str)->ReleaseCandidateEvidenceManifestItem{ReleaseCandidateEvidenceManifestItem{category:c.code().into(),status:s.code().into(),linkage:ReleaseCandidateEvidenceManifestSourceLinkage{source_surface:surface.into(),source_status:status.into(),source_id:id.unwrap_or("missing").into(),source_summary:summary.into()}}}
fn release_candidate_evidence_manifest_caveats()->Vec<ReleaseCandidateEvidenceManifestCaveat>{let mut v=vec![
ReleaseCandidateEvidenceManifestCaveat{category:"targeted_cleanup_required".into(),detail:"Targeted cleanup remains part of the Release Candidate stewardship path.".into()},
ReleaseCandidateEvidenceManifestCaveat{category:"oversized_module_risk".into(),detail:"Oversized module risk remains under stewardship review.".into()},
ReleaseCandidateEvidenceManifestCaveat{category:"string_heuristic_hardening_required".into(),detail:"String/status heuristic hardening remains active for targeted adapters.".into()},
ReleaseCandidateEvidenceManifestCaveat{category:"test_hardening_required".into(),detail:"Test hardening remains active for release-path stewardship.".into()},
ReleaseCandidateEvidenceManifestCaveat{category:"boundary_rule_duplication_risk".into(),detail:"Boundary-rule duplication risk remains a stewardship caveat.".into()}];v.sort_by(|a,b|a.category.cmp(&b.category));v}
fn release_candidate_evidence_manifest_boundary_statuses()->Vec<String>{vec!["evidence_manifest_only","release_candidate_label_only","supportability_with_caveats_only","non_authoritative_manifest","local_only_non_public","release_readiness_not_approved","production_status_not_approved","public_use_not_approved","release_artifact_not_created","public_artifact_not_created","no_signing","no_publishing","no_deployment_artifact","no_public_distribution"].into_iter().map(str::to_string).collect()}
fn release_candidate_evidence_manifest_capability_surface()->ReleaseCandidateEvidenceManifestCapabilitySurface{ReleaseCandidateEvidenceManifestCapabilitySurface{approval_enabled:false,signing_enabled:false,publishing_enabled:false,deployment_enabled:false,public_distribution_enabled:false}}
