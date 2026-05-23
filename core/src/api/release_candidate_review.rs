use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCandidateReviewStatus {
    NotReviewed,
    ReviewProjected,
    ReviewableWithCaveats,
    ReviewBlocked,
    ReviewIncomplete,
    InvalidReviewInput,
}

impl ReleaseCandidateReviewStatus {
    pub fn code(self) -> &'static str {
        match self {
            Self::NotReviewed => "not_reviewed",
            Self::ReviewProjected => "review_projected",
            Self::ReviewableWithCaveats => "reviewable_with_caveats",
            Self::ReviewBlocked => "review_blocked",
            Self::ReviewIncomplete => "review_incomplete",
            Self::InvalidReviewInput => "invalid_review_input",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateReviewSection {
    ManifestSummary,
    Caveats,
    Blockers,
    UpstreamLinkage,
    ValidationSummary,
    ReviewFindings,
    AuthorityBoundaries,
}

impl ReleaseCandidateReviewSection {
    pub fn code(self) -> &'static str {
        match self {
            Self::ManifestSummary => "manifest_summary",
            Self::Caveats => "caveats",
            Self::Blockers => "blockers",
            Self::UpstreamLinkage => "upstream_linkage",
            Self::ValidationSummary => "validation_summary",
            Self::ReviewFindings => "review_findings",
            Self::AuthorityBoundaries => "authority_boundaries",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateReviewFindingCategory {
    ManifestSupportability,
    ManifestBlocker,
    ManifestCaveat,
    UpstreamLinkage,
    ValidationSummary,
    TargetedCleanup,
    NoAuthorityBoundary,
}

impl ReleaseCandidateReviewFindingCategory {
    fn code(self) -> &'static str {
        match self {
            Self::ManifestSupportability => "manifest_supportability",
            Self::ManifestBlocker => "manifest_blocker",
            Self::ManifestCaveat => "manifest_caveat",
            Self::UpstreamLinkage => "upstream_linkage",
            Self::ValidationSummary => "validation_summary",
            Self::TargetedCleanup => "targeted_cleanup",
            Self::NoAuthorityBoundary => "no_authority_boundary",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseCandidateReviewFindingSeverity {
    Info,
    Low,
    Medium,
    High,
    Blocking,
}

impl ReleaseCandidateReviewFindingSeverity {
    fn code(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Blocking => "blocking",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateReviewFinding {
    pub category: String,
    pub severity: String,
    pub detail: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateReviewManifestSummary {
    pub label_status: String,
    pub manifest_status: String,
    pub manifest_id: Option<String>,
    pub item_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateReviewCaveatSummary {
    pub category: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateReviewBlockerSummary {
    pub category: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateReviewUpstreamLinkageSummary {
    pub category: String,
    pub source_surface: String,
    pub source_status: String,
    pub source_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateReviewValidationSummary {
    pub item_count: usize,
    pub present_count: usize,
    pub missing_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateReviewCapabilitySurface {
    pub approval_enabled: bool,
    pub signing_enabled: bool,
    pub publishing_enabled: bool,
    pub deployment_enabled: bool,
    pub public_distribution_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCandidateReviewProjection {
    pub status: String,
    pub review_id: Option<String>,
    pub sections: Vec<String>,
    pub manifest_summary: ReleaseCandidateReviewManifestSummary,
    pub caveats: Vec<ReleaseCandidateReviewCaveatSummary>,
    pub blockers: Vec<ReleaseCandidateReviewBlockerSummary>,
    pub upstream_linkage: Vec<ReleaseCandidateReviewUpstreamLinkageSummary>,
    pub validation_summary: ReleaseCandidateReviewValidationSummary,
    pub review_findings: Vec<ReleaseCandidateReviewFinding>,
    pub boundary_statuses: Vec<String>,
    pub capability_surface: ReleaseCandidateReviewCapabilitySurface,
}

pub fn initial_release_candidate_review_projection() -> ReleaseCandidateReviewProjection {
    ReleaseCandidateReviewProjection {
        status: ReleaseCandidateReviewStatus::NotReviewed.code().into(),
        review_id: None,
        sections: release_candidate_review_sections(),
        manifest_summary: ReleaseCandidateReviewManifestSummary {
            label_status: "not_labeled".into(),
            manifest_status: "not_generated".into(),
            manifest_id: None,
            item_count: 0,
        },
        caveats: vec![],
        blockers: vec![],
        upstream_linkage: vec![],
        validation_summary: ReleaseCandidateReviewValidationSummary {
            item_count: 0,
            present_count: 0,
            missing_count: 0,
            blocked_count: 0,
            rejected_count: 0,
        },
        review_findings: vec![],
        boundary_statuses: release_candidate_review_boundary_statuses(),
        capability_surface: release_candidate_review_capability_surface(),
    }
}

pub fn release_candidate_review_sections() -> Vec<String> {
    let mut sections = [
        ReleaseCandidateReviewSection::ManifestSummary,
        ReleaseCandidateReviewSection::Caveats,
        ReleaseCandidateReviewSection::Blockers,
        ReleaseCandidateReviewSection::UpstreamLinkage,
        ReleaseCandidateReviewSection::ValidationSummary,
        ReleaseCandidateReviewSection::ReviewFindings,
        ReleaseCandidateReviewSection::AuthorityBoundaries,
    ]
    .into_iter()
    .map(|section| section.code().to_string())
    .collect::<Vec<_>>();
    sections.sort();
    sections
}

pub fn release_candidate_review_boundary_statuses() -> Vec<String> {
    vec![
        "review_surface_only",
        "non_authoritative_review",
        "local_only_non_public",
        "release_candidate_status_not_approved",
        "release_readiness_not_approved",
        "production_status_not_approved",
        "public_use_not_approved",
        "release_artifact_not_created",
        "public_artifact_not_created",
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
    .map(str::to_string)
    .collect()
}

fn release_candidate_review_capability_surface() -> ReleaseCandidateReviewCapabilitySurface {
    ReleaseCandidateReviewCapabilitySurface {
        approval_enabled: false,
        signing_enabled: false,
        publishing_enabled: false,
        deployment_enabled: false,
        public_distribution_enabled: false,
    }
}

pub fn derive_release_candidate_review(
    manifest: &ReleaseCandidateEvidenceManifestProjection,
) -> ReleaseCandidateReviewProjection {
    let mut projection = initial_release_candidate_review_projection();

    projection.manifest_summary = ReleaseCandidateReviewManifestSummary {
        label_status: manifest.label_status.clone(),
        manifest_status: manifest.manifest_status.clone(),
        manifest_id: manifest.manifest_id.clone(),
        item_count: manifest.items.len(),
    };

    projection.caveats = manifest
        .caveats
        .iter()
        .map(|caveat| ReleaseCandidateReviewCaveatSummary {
            category: caveat.category.clone(),
            detail: caveat.detail.clone(),
        })
        .collect();
    projection
        .caveats
        .sort_by(|a, b| a.category.cmp(&b.category));

    projection.blockers = manifest
        .blockers
        .iter()
        .map(|blocker| ReleaseCandidateReviewBlockerSummary {
            category: blocker.category.clone(),
            reason: blocker.reason.clone(),
        })
        .collect();
    projection
        .blockers
        .sort_by(|a, b| a.category.cmp(&b.category));

    projection.upstream_linkage = manifest
        .items
        .iter()
        .map(|item| ReleaseCandidateReviewUpstreamLinkageSummary {
            category: item.category.clone(),
            source_surface: item.linkage.source_surface.clone(),
            source_status: item.linkage.source_status.clone(),
            source_id: item.linkage.source_id.clone(),
        })
        .collect();
    projection
        .upstream_linkage
        .sort_by(|a, b| a.category.cmp(&b.category));

    projection.validation_summary = ReleaseCandidateReviewValidationSummary {
        item_count: manifest.validation_summary.item_count,
        present_count: manifest.validation_summary.present_count,
        missing_count: manifest.validation_summary.missing_count,
        blocked_count: manifest.validation_summary.blocked_count,
        rejected_count: manifest.validation_summary.rejected_count,
    };

    projection
        .review_findings
        .push(ReleaseCandidateReviewFinding {
            category: ReleaseCandidateReviewFindingCategory::NoAuthorityBoundary
                .code()
                .into(),
            severity: ReleaseCandidateReviewFindingSeverity::Info.code().into(),
            detail: "Review surface only with no approval/readiness authority.".into(),
            source: "boundary_statuses".into(),
        });

    for blocker in &projection.blockers {
        projection
            .review_findings
            .push(ReleaseCandidateReviewFinding {
                category: ReleaseCandidateReviewFindingCategory::ManifestBlocker
                    .code()
                    .into(),
                severity: ReleaseCandidateReviewFindingSeverity::Blocking
                    .code()
                    .into(),
                detail: blocker.reason.clone(),
                source: blocker.category.clone(),
            });
    }

    for caveat in &projection.caveats {
        let category = if caveat.category == "targeted_cleanup_required" {
            ReleaseCandidateReviewFindingCategory::TargetedCleanup
        } else {
            ReleaseCandidateReviewFindingCategory::ManifestCaveat
        };
        projection
            .review_findings
            .push(ReleaseCandidateReviewFinding {
                category: category.code().into(),
                severity: ReleaseCandidateReviewFindingSeverity::Medium.code().into(),
                detail: caveat.detail.clone(),
                source: caveat.category.clone(),
            });
    }

    projection.review_findings.sort_by(|a, b| {
        (&a.category, &a.severity, &a.detail).cmp(&(&b.category, &b.severity, &b.detail))
    });

    projection.status = if manifest.manifest_status == "manifest_complete_with_caveats" {
        ReleaseCandidateReviewStatus::ReviewableWithCaveats
            .code()
            .into()
    } else if !projection.blockers.is_empty() {
        ReleaseCandidateReviewStatus::ReviewBlocked.code().into()
    } else if manifest.manifest_status == "not_generated" {
        ReleaseCandidateReviewStatus::ReviewIncomplete.code().into()
    } else {
        ReleaseCandidateReviewStatus::ReviewProjected.code().into()
    };

    let digest = format!(
        "{}|{}|{}|{}",
        projection.manifest_summary.manifest_status,
        projection.caveats.len(),
        projection.blockers.len(),
        projection.review_findings.len()
    );
    projection.review_id = Some(format!("rc-review-{:x}", simple_hash(&digest)));

    projection
}

fn simple_hash(input: &str) -> u64 {
    let mut hash = 1469598103934665603u64;
    for byte in input.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_manifest() -> ReleaseCandidateEvidenceManifestProjection {
        let mut manifest = initial_release_candidate_evidence_manifest_projection();
        manifest.label_status = "supportability_labeled".to_string();
        manifest.manifest_status = "manifest_complete_with_caveats".to_string();
        manifest.manifest_id = Some("manifest-phase-182-1".to_string());
        manifest.validation_summary.item_count = 3;
        manifest.validation_summary.present_count = 2;
        manifest.validation_summary.missing_count = 1;
        manifest.validation_summary.blocked_count = 0;
        manifest.validation_summary.rejected_count = 0;

        manifest.items = vec![
            ReleaseCandidateEvidenceManifestItem {
                category: "zeta_observability".to_string(),
                status: "present".to_string(),
                linkage: ReleaseCandidateEvidenceManifestSourceLinkage {
                    source_surface: "trial_observability".to_string(),
                    source_status: "observability_ready".to_string(),
                    source_id: "obs-1".to_string(),
                    source_summary: "observability evidence".to_string(),
                },
            },
            ReleaseCandidateEvidenceManifestItem {
                category: "alpha_trial_evidence".to_string(),
                status: "missing".to_string(),
                linkage: ReleaseCandidateEvidenceManifestSourceLinkage {
                    source_surface: "trial_evidence_review".to_string(),
                    source_status: "trial_evidence_missing".to_string(),
                    source_id: "evidence-1".to_string(),
                    source_summary: "trial evidence linkage".to_string(),
                },
            },
        ];

        manifest.caveats = vec![
            ReleaseCandidateEvidenceManifestCaveat {
                category: "targeted_cleanup_required".to_string(),
                detail: "Resolve legacy caveat backlog".to_string(),
            },
            ReleaseCandidateEvidenceManifestCaveat {
                category: "operator_follow_up".to_string(),
                detail: "Operator should inspect missing evidence".to_string(),
            },
        ];

        manifest
    }

    #[test]
    fn valid_manifest_produces_reviewable_state() {
        let review = derive_release_candidate_review(&make_manifest());
        assert_eq!(
            review.status,
            ReleaseCandidateReviewStatus::ReviewableWithCaveats.code()
        );
        assert_eq!(
            review.manifest_summary.manifest_status,
            "manifest_complete_with_caveats"
        );
    }

    #[test]
    fn blocked_manifest_produces_blockers() {
        let mut manifest = make_manifest();
        manifest.manifest_status = "manifest_incomplete".to_string();
        manifest.blockers = vec![ReleaseCandidateEvidenceManifestBlocker {
            category: "missing_evidence".to_string(),
            reason: "Evidence is missing".to_string(),
        }];

        let review = derive_release_candidate_review(&manifest);
        assert_eq!(
            review.status,
            ReleaseCandidateReviewStatus::ReviewBlocked.code()
        );
        assert_eq!(review.blockers.len(), 1);
    }

    #[test]
    fn caveat_and_blocker_summaries_and_upstream_are_derived() {
        let mut manifest = make_manifest();
        manifest.blockers = vec![ReleaseCandidateEvidenceManifestBlocker {
            category: "blocked_status".to_string(),
            reason: "Blocked upstream evidence".to_string(),
        }];
        let review = derive_release_candidate_review(&manifest);

        assert_eq!(review.caveats[0].category, "operator_follow_up");
        assert_eq!(review.blockers[0].category, "blocked_status");
        assert_eq!(review.upstream_linkage[0].category, "alpha_trial_evidence");
        assert_eq!(review.validation_summary.missing_count, 1);
    }

    #[test]
    fn targeted_cleanup_finding_is_derived() {
        let review = derive_release_candidate_review(&make_manifest());
        assert!(review
            .review_findings
            .iter()
            .any(|finding| finding.category == "targeted_cleanup"));
    }

    #[test]
    fn deterministic_review_id_and_ordering() {
        let review_a = derive_release_candidate_review(&make_manifest());
        let review_b = derive_release_candidate_review(&make_manifest());
        assert_eq!(review_a.review_id, review_b.review_id);
        assert_eq!(review_a.caveats, review_b.caveats);
        assert_eq!(review_a.blockers, review_b.blockers);
        assert_eq!(review_a.upstream_linkage, review_b.upstream_linkage);
        assert_eq!(review_a.review_findings, review_b.review_findings);
    }

    #[test]
    fn no_authority_boundaries_stay_disabled() {
        let review = derive_release_candidate_review(&make_manifest());
        assert!(review
            .boundary_statuses
            .contains(&"review_surface_only".to_string()));
        assert!(review
            .boundary_statuses
            .contains(&"release_readiness_not_approved".to_string()));
        assert!(!review.capability_surface.approval_enabled);
        assert!(!review.capability_surface.signing_enabled);
        assert!(!review.capability_surface.publishing_enabled);
        assert!(!review.capability_surface.deployment_enabled);
        assert!(!review.capability_surface.public_distribution_enabled);
    }
}
