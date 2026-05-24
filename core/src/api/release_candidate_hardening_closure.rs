use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidateHardeningClosureStatus { NotStarted, Open, Blocked, ClosedWithCaveats, Invalid }
impl ReleaseCandidateHardeningClosureStatus { pub fn code(self)->&'static str { match self {Self::NotStarted=>"not_started",Self::Open=>"open",Self::Blocked=>"blocked",Self::ClosedWithCaveats=>"closed_with_caveats",Self::Invalid=>"invalid"}} }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateHardeningItemStatus { Open, Blocked, Closed, Caveated, Deferred, Informational }
impl ReleaseCandidateHardeningItemStatus { fn code(self)->&'static str { match self {Self::Open=>"open",Self::Blocked=>"blocked",Self::Closed=>"closed",Self::Caveated=>"caveated",Self::Deferred=>"deferred",Self::Informational=>"informational"}} }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateHardeningItemCategory { ManifestBlocker, ManifestCaveat, ReviewFinding, TargetedCleanup, ValidationGap, UpstreamEvidenceGap, BoundaryConfirmation }
impl ReleaseCandidateHardeningItemCategory { fn code(self)->&'static str { match self {Self::ManifestBlocker=>"manifest_blocker",Self::ManifestCaveat=>"manifest_caveat",Self::ReviewFinding=>"review_finding",Self::TargetedCleanup=>"targeted_cleanup",Self::ValidationGap=>"validation_gap",Self::UpstreamEvidenceGap=>"upstream_evidence_gap",Self::BoundaryConfirmation=>"boundary_confirmation"}} }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateHardeningItemSeverity { Info, Low, Medium, High, Blocking }
impl ReleaseCandidateHardeningItemSeverity { fn code(self)->&'static str { match self {Self::Info=>"info",Self::Low=>"low",Self::Medium=>"medium",Self::High=>"high",Self::Blocking=>"blocking"}} }

#[derive(Debug, Clone, PartialEq, Eq)] pub struct ReleaseCandidateHardeningLinkedEvidence { pub source: String, pub reference: String }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ReleaseCandidateHardeningLinkedReviewFinding { pub category: String, pub detail: String }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ReleaseCandidateHardeningItem { pub item_id:String,pub status:String,pub category:String,pub severity:String,pub detail:String,pub linked_evidence:ReleaseCandidateHardeningLinkedEvidence,pub linked_review_finding:Option<ReleaseCandidateHardeningLinkedReviewFinding> }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ReleaseCandidateHardeningCaveat { pub category:String,pub detail:String }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ReleaseCandidateHardeningBlocker { pub category:String,pub reason:String }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ReleaseCandidateHardeningValidationSummary { pub hardening_item_count:usize,pub blocking_item_count:usize,pub caveat_count:usize,pub blocker_count:usize }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ReleaseCandidateHardeningCapabilitySurface { pub approval_enabled:bool,pub signing_enabled:bool,pub publishing_enabled:bool,pub deployment_enabled:bool,pub public_distribution_enabled:bool }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ReleaseCandidateHardeningClosureProjection { pub status:String,pub closure_id:Option<String>,pub hardening_items:Vec<ReleaseCandidateHardeningItem>,pub caveats:Vec<ReleaseCandidateHardeningCaveat>,pub blockers:Vec<ReleaseCandidateHardeningBlocker>,pub validation_summary:ReleaseCandidateHardeningValidationSummary,pub boundary_statuses:Vec<String>,pub capability_surface:ReleaseCandidateHardeningCapabilitySurface }

pub fn initial_release_candidate_hardening_closure_projection() -> ReleaseCandidateHardeningClosureProjection { ReleaseCandidateHardeningClosureProjection{ status:ReleaseCandidateHardeningClosureStatus::NotStarted.code().into(), closure_id:None, hardening_items:vec![], caveats:vec![], blockers:vec![], validation_summary:ReleaseCandidateHardeningValidationSummary{hardening_item_count:0,blocking_item_count:0,caveat_count:0,blocker_count:0}, boundary_statuses: release_candidate_hardening_boundary_statuses(), capability_surface: ReleaseCandidateHardeningCapabilitySurface{approval_enabled:false,signing_enabled:false,publishing_enabled:false,deployment_enabled:false,public_distribution_enabled:false} } }

pub fn release_candidate_hardening_boundary_statuses()->Vec<String>{ vec!["hardening_closure_only","non_authoritative_closure","local_only_non_public","release_candidate_status_not_approved","release_readiness_not_approved","production_status_not_approved","public_use_not_approved","release_artifact_not_created","public_artifact_not_created","no_signing","no_publishing","no_deployment_artifact","no_public_distribution","no_public_download","no_github_release","no_release_tag","no_installer_activation","no_update_channel_activation","no_provider_trust","no_action_authorization","no_replay_repair","no_recovery_promotion"].into_iter().map(str::to_string).collect() }

pub fn derive_release_candidate_hardening_closure(manifest:&ReleaseCandidateEvidenceManifestProjection, review:&ReleaseCandidateReviewProjection)->ReleaseCandidateHardeningClosureProjection{
 let mut p=initial_release_candidate_hardening_closure_projection();
 if manifest.manifest_id.is_none() || review.review_id.is_none(){ p.status=ReleaseCandidateHardeningClosureStatus::Invalid.code().into(); return p; }
 p.caveats=manifest.caveats.iter().map(|c| ReleaseCandidateHardeningCaveat{category:c.category.clone(),detail:c.detail.clone()}).collect(); p.caveats.sort_by(|a,b|a.category.cmp(&b.category).then(a.detail.cmp(&b.detail)));
 p.blockers=manifest.blockers.iter().map(|b| ReleaseCandidateHardeningBlocker{category:b.category.clone(),reason:b.reason.clone()}).collect(); p.blockers.sort_by(|a,b|a.category.cmp(&b.category).then(a.reason.cmp(&b.reason)));
 let mut items=Vec::new();
 for b in &p.blockers { items.push(new_item(ReleaseCandidateHardeningItemStatus::Blocked,ReleaseCandidateHardeningItemCategory::ManifestBlocker,ReleaseCandidateHardeningItemSeverity::Blocking,b.reason.clone(),"release_candidate_evidence_manifest",b.category.clone(),None)); }
 for c in &p.caveats { items.push(new_item(ReleaseCandidateHardeningItemStatus::Caveated,ReleaseCandidateHardeningItemCategory::ManifestCaveat,ReleaseCandidateHardeningItemSeverity::Medium,c.detail.clone(),"release_candidate_evidence_manifest",c.category.clone(),None)); }
 for f in &review.review_findings { let sev=if f.severity=="blocking"{ReleaseCandidateHardeningItemSeverity::Blocking}else if f.severity=="high"{ReleaseCandidateHardeningItemSeverity::High}else if f.severity=="medium"{ReleaseCandidateHardeningItemSeverity::Medium}else if f.severity=="low"{ReleaseCandidateHardeningItemSeverity::Low}else{ReleaseCandidateHardeningItemSeverity::Info}; let blocked=sev==ReleaseCandidateHardeningItemSeverity::Blocking; items.push(new_item(if blocked{ReleaseCandidateHardeningItemStatus::Blocked}else{ReleaseCandidateHardeningItemStatus::Open}, if f.category=="targeted_cleanup"{ReleaseCandidateHardeningItemCategory::TargetedCleanup}else{ReleaseCandidateHardeningItemCategory::ReviewFinding}, sev, f.detail.clone(),"release_candidate_review",f.source.clone(), Some(ReleaseCandidateHardeningLinkedReviewFinding{category:f.category.clone(),detail:f.detail.clone()}))); }
 items.sort_by(|a,b|a.category.cmp(&b.category).then(a.severity.cmp(&b.severity)).then(a.detail.cmp(&b.detail)));
 for (idx,item) in items.iter_mut().enumerate(){ item.item_id=format!("hardening-item-{:03}",idx+1); }
 p.hardening_items=items;
 let blocking=p.hardening_items.iter().filter(|i|i.status=="blocked").count();
 p.validation_summary=ReleaseCandidateHardeningValidationSummary{hardening_item_count:p.hardening_items.len(),blocking_item_count:blocking,caveat_count:p.caveats.len(),blocker_count:p.blockers.len()};
 p.status=if blocking>0 {ReleaseCandidateHardeningClosureStatus::Blocked.code().into()} else if !p.hardening_items.is_empty(){ReleaseCandidateHardeningClosureStatus::ClosedWithCaveats.code().into()} else {ReleaseCandidateHardeningClosureStatus::Open.code().into()};
 p.closure_id=Some(format!("rc-hardening-{}-{}-{}",p.hardening_items.len(),p.caveats.len(),p.blockers.len()));
 p
}
fn new_item(status:ReleaseCandidateHardeningItemStatus,category:ReleaseCandidateHardeningItemCategory,severity:ReleaseCandidateHardeningItemSeverity,detail:String,source:&str,reference:String,linked_review_finding:Option<ReleaseCandidateHardeningLinkedReviewFinding>)->ReleaseCandidateHardeningItem{ ReleaseCandidateHardeningItem{item_id:String::new(),status:status.code().into(),category:category.code().into(),severity:severity.code().into(),detail,linked_evidence:ReleaseCandidateHardeningLinkedEvidence{source:source.into(),reference},linked_review_finding}}

#[cfg(test)]
mod tests { use super::*; #[test] fn invalid_when_missing_inputs(){ let m=initial_release_candidate_evidence_manifest_projection(); let r=initial_release_candidate_review_projection(); let p=derive_release_candidate_hardening_closure(&m,&r); assert_eq!(p.status,"invalid"); }
#[test] fn blocked_when_blockers_exist(){ let s=initial_local_operator_shell_state(); let p=derive_release_candidate_hardening_closure(&s.release_candidate_evidence_manifest,&s.release_candidate_review); assert_eq!(p.status,"blocked"); assert!(p.validation_summary.blocking_item_count>0); }
#[test] fn deterministic_order_and_id(){ let s=initial_local_operator_shell_state(); let a=derive_release_candidate_hardening_closure(&s.release_candidate_evidence_manifest,&s.release_candidate_review); let b=derive_release_candidate_hardening_closure(&s.release_candidate_evidence_manifest,&s.release_candidate_review); assert_eq!(a.closure_id,b.closure_id); assert_eq!(a.hardening_items,b.hardening_items); }
}
