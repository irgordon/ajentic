use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceEvidenceValidationStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GovernanceEvidenceReason {
    AttributionOnlyAccepted,
    MissingGovernanceEvidenceIdentifier,
    MissingGovernanceSourceReference,
    MissingGovernanceVersionFingerprint,
    MissingPolicyVersionLabel,
    MissingPolicySourceReference,
    MissingChangelogReference,
    MissingRoadmapReference,
    MissingValidationRunReference,
    DuplicateGovernanceEvidenceIdentifier,
    ContradictoryGovernanceVersionLabel,
    ContradictoryPolicyVersionLabel,
    UnsupportedTruthDimensionClaim,
    GovernanceAuthorityRewriteRejected,
    PolicyAuthorityGrantRejected,
    ReadinessApprovalRejected,
    DeploymentApprovalRejected,
    ReleaseCandidateApprovalRejected,
    ProductionCandidateApprovalRejected,
    PublicUseApprovalRejected,
    ProductionHumanUseApprovalRejected,
    DeploymentAutomationRejected,
    ReleaseArtifactCreationRejected,
    ProviderTrustRejected,
    ProviderOutputPromotionRejected,
    PersistenceAuthorityExpansionRejected,
    ReplayRepairRejected,
    RecoveryPromotionRejected,
    ActionExecutionRejected,
}

impl GovernanceEvidenceReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AttributionOnlyAccepted => "attribution_only_accepted",
            Self::MissingGovernanceEvidenceIdentifier => "missing_governance_evidence_identifier",
            Self::MissingGovernanceSourceReference => "missing_governance_source_reference",
            Self::MissingGovernanceVersionFingerprint => "missing_governance_version_fingerprint",
            Self::MissingPolicyVersionLabel => "missing_policy_version_label",
            Self::MissingPolicySourceReference => "missing_policy_source_reference",
            Self::MissingChangelogReference => "missing_changelog_reference",
            Self::MissingRoadmapReference => "missing_roadmap_reference",
            Self::MissingValidationRunReference => "missing_validation_run_reference",
            Self::DuplicateGovernanceEvidenceIdentifier => {
                "duplicate_governance_evidence_identifier"
            }
            Self::ContradictoryGovernanceVersionLabel => "contradictory_governance_version_label",
            Self::ContradictoryPolicyVersionLabel => "contradictory_policy_version_label",
            Self::UnsupportedTruthDimensionClaim => "unsupported_truth_dimension_claim",
            Self::GovernanceAuthorityRewriteRejected => "governance_authority_rewrite_rejected",
            Self::PolicyAuthorityGrantRejected => "policy_authority_grant_rejected",
            Self::ReadinessApprovalRejected => "readiness_approval_rejected",
            Self::DeploymentApprovalRejected => "deployment_approval_rejected",
            Self::ReleaseCandidateApprovalRejected => "release_candidate_approval_rejected",
            Self::ProductionCandidateApprovalRejected => "production_candidate_approval_rejected",
            Self::PublicUseApprovalRejected => "public_use_approval_rejected",
            Self::ProductionHumanUseApprovalRejected => "production_human_use_approval_rejected",
            Self::DeploymentAutomationRejected => "deployment_automation_rejected",
            Self::ReleaseArtifactCreationRejected => "release_artifact_creation_rejected",
            Self::ProviderTrustRejected => "provider_trust_rejected",
            Self::ProviderOutputPromotionRejected => "provider_output_promotion_rejected",
            Self::PersistenceAuthorityExpansionRejected => {
                "persistence_authority_expansion_rejected"
            }
            Self::ReplayRepairRejected => "replay_repair_rejected",
            Self::RecoveryPromotionRejected => "recovery_promotion_rejected",
            Self::ActionExecutionRejected => "action_execution_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GovernanceEvidenceTruthDimension {
    Normative,
    Planned,
    Historical,
    Procedural,
    Orientation,
    Executable,
    Contract,
    Navigation,
    Unsupported(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GovernanceEvidenceSourceReference {
    pub path: String,
    pub truth_dimension: GovernanceEvidenceTruthDimension,
    pub version_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GovernanceEvidenceVersion {
    pub evidence_id: String,
    pub version_label: String,
    pub deterministic_snapshot_label: String,
    pub source_commit: String,
    pub governance_sources: Vec<GovernanceEvidenceSourceReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolicyVersionEvidence {
    pub evidence_id: String,
    pub policy_version_label: String,
    pub policy_sources: Vec<GovernanceEvidenceSourceReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GovernanceEvidenceChangelogReference {
    pub path: String,
    pub version_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GovernanceEvidenceRoadmapReference {
    pub path: String,
    pub phase_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GovernanceEvidenceValidationRunReference {
    pub command: String,
    pub deterministic_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceEvidenceAuthorityDenialSnapshot {
    pub governance_authority_rewritten: bool,
    pub policy_authority_granted: bool,
    pub deployment_approved: bool,
    pub release_candidate_approved: bool,
    pub production_candidate_approved: bool,
    pub public_use_approved: bool,
    pub production_human_use_approved: bool,
    pub readiness_approved: bool,
    pub deployment_automation_enabled: bool,
    pub release_artifact_created: bool,
    pub provider_trust_granted: bool,
    pub provider_output_promoted: bool,
    pub persistence_authority_expanded: bool,
    pub replay_repaired: bool,
    pub recovery_promoted: bool,
    pub action_executed: bool,
}

impl GovernanceEvidenceAuthorityDenialSnapshot {
    pub fn all_denied() -> Self {
        Self {
            governance_authority_rewritten: false,
            policy_authority_granted: false,
            deployment_approved: false,
            release_candidate_approved: false,
            production_candidate_approved: false,
            public_use_approved: false,
            production_human_use_approved: false,
            readiness_approved: false,
            deployment_automation_enabled: false,
            release_artifact_created: false,
            provider_trust_granted: false,
            provider_output_promoted: false,
            persistence_authority_expanded: false,
            replay_repaired: false,
            recovery_promoted: false,
            action_executed: false,
        }
    }

    fn claimed_authority_reasons(&self) -> Vec<GovernanceEvidenceReason> {
        let mut reasons = Vec::new();
        if self.governance_authority_rewritten {
            reasons.push(GovernanceEvidenceReason::GovernanceAuthorityRewriteRejected);
        }
        if self.policy_authority_granted {
            reasons.push(GovernanceEvidenceReason::PolicyAuthorityGrantRejected);
        }
        if self.readiness_approved {
            reasons.push(GovernanceEvidenceReason::ReadinessApprovalRejected);
        }
        if self.deployment_approved {
            reasons.push(GovernanceEvidenceReason::DeploymentApprovalRejected);
        }
        if self.release_candidate_approved {
            reasons.push(GovernanceEvidenceReason::ReleaseCandidateApprovalRejected);
        }
        if self.production_candidate_approved {
            reasons.push(GovernanceEvidenceReason::ProductionCandidateApprovalRejected);
        }
        if self.public_use_approved {
            reasons.push(GovernanceEvidenceReason::PublicUseApprovalRejected);
        }
        if self.production_human_use_approved {
            reasons.push(GovernanceEvidenceReason::ProductionHumanUseApprovalRejected);
        }
        if self.deployment_automation_enabled {
            reasons.push(GovernanceEvidenceReason::DeploymentAutomationRejected);
        }
        if self.release_artifact_created {
            reasons.push(GovernanceEvidenceReason::ReleaseArtifactCreationRejected);
        }
        if self.provider_trust_granted {
            reasons.push(GovernanceEvidenceReason::ProviderTrustRejected);
        }
        if self.provider_output_promoted {
            reasons.push(GovernanceEvidenceReason::ProviderOutputPromotionRejected);
        }
        if self.persistence_authority_expanded {
            reasons.push(GovernanceEvidenceReason::PersistenceAuthorityExpansionRejected);
        }
        if self.replay_repaired {
            reasons.push(GovernanceEvidenceReason::ReplayRepairRejected);
        }
        if self.recovery_promoted {
            reasons.push(GovernanceEvidenceReason::RecoveryPromotionRejected);
        }
        if self.action_executed {
            reasons.push(GovernanceEvidenceReason::ActionExecutionRejected);
        }
        reasons
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceEvidenceAttributionInput {
    pub governance_versions: Vec<GovernanceEvidenceVersion>,
    pub policy_versions: Vec<PolicyVersionEvidence>,
    pub changelog_references: Vec<GovernanceEvidenceChangelogReference>,
    pub roadmap_references: Vec<GovernanceEvidenceRoadmapReference>,
    pub operations_report_references: Vec<GovernanceEvidenceSourceReference>,
    pub checklist_references: Vec<GovernanceEvidenceSourceReference>,
    pub validation_run_references: Vec<GovernanceEvidenceValidationRunReference>,
    pub reason_codes: Vec<GovernanceEvidenceReason>,
    pub authority_denial_snapshot: GovernanceEvidenceAuthorityDenialSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceEvidenceAttributionReport {
    pub status: GovernanceEvidenceValidationStatus,
    pub reasons: Vec<GovernanceEvidenceReason>,
    pub reason_codes: Vec<&'static str>,
    pub governance_evidence_ids: Vec<String>,
    pub policy_evidence_ids: Vec<String>,
    pub governance_version_labels: Vec<String>,
    pub policy_version_labels: Vec<String>,
    pub authority_denial_snapshot: GovernanceEvidenceAuthorityDenialSnapshot,
    pub attribution_only: bool,
    pub non_authoritative: bool,
}

pub fn validate_governance_evidence_attribution(
    input: &GovernanceEvidenceAttributionInput,
) -> GovernanceEvidenceAttributionReport {
    let mut reasons = BTreeSet::new();
    collect_required_field_rejections(input, &mut reasons);
    collect_duplicate_identifier_rejections(input, &mut reasons);
    collect_contradictory_version_rejections(input, &mut reasons);
    collect_truth_dimension_rejections(input, &mut reasons);
    for reason in input.authority_denial_snapshot.claimed_authority_reasons() {
        reasons.insert(reason);
    }

    if reasons.is_empty() {
        reasons.insert(GovernanceEvidenceReason::AttributionOnlyAccepted);
    }

    let reasons: Vec<_> = reasons.into_iter().collect();
    let status = if reasons == [GovernanceEvidenceReason::AttributionOnlyAccepted] {
        GovernanceEvidenceValidationStatus::Accepted
    } else {
        GovernanceEvidenceValidationStatus::Rejected
    };

    GovernanceEvidenceAttributionReport {
        status,
        reason_codes: reasons.iter().map(GovernanceEvidenceReason::code).collect(),
        reasons,
        governance_evidence_ids: sorted_unique(
            input
                .governance_versions
                .iter()
                .map(|version| version.evidence_id.clone()),
        ),
        policy_evidence_ids: sorted_unique(
            input
                .policy_versions
                .iter()
                .map(|version| version.evidence_id.clone()),
        ),
        governance_version_labels: sorted_unique(
            input
                .governance_versions
                .iter()
                .map(|version| version.version_label.clone()),
        ),
        policy_version_labels: sorted_unique(
            input
                .policy_versions
                .iter()
                .map(|version| version.policy_version_label.clone()),
        ),
        authority_denial_snapshot: GovernanceEvidenceAuthorityDenialSnapshot::all_denied(),
        attribution_only: status == GovernanceEvidenceValidationStatus::Accepted,
        non_authoritative: true,
    }
}

pub fn governance_evidence_grants_authority(report: &GovernanceEvidenceAttributionReport) -> bool {
    report
        .authority_denial_snapshot
        .governance_authority_rewritten
        || report.authority_denial_snapshot.policy_authority_granted
        || report.authority_denial_snapshot.deployment_approved
        || report.authority_denial_snapshot.release_candidate_approved
        || report
            .authority_denial_snapshot
            .production_candidate_approved
        || report.authority_denial_snapshot.public_use_approved
        || report
            .authority_denial_snapshot
            .production_human_use_approved
        || report.authority_denial_snapshot.readiness_approved
        || report
            .authority_denial_snapshot
            .deployment_automation_enabled
        || report.authority_denial_snapshot.release_artifact_created
        || report.authority_denial_snapshot.provider_trust_granted
        || report.authority_denial_snapshot.provider_output_promoted
        || report
            .authority_denial_snapshot
            .persistence_authority_expanded
        || report.authority_denial_snapshot.replay_repaired
        || report.authority_denial_snapshot.recovery_promoted
        || report.authority_denial_snapshot.action_executed
}

fn collect_required_field_rejections(
    input: &GovernanceEvidenceAttributionInput,
    reasons: &mut BTreeSet<GovernanceEvidenceReason>,
) {
    if input.governance_versions.is_empty()
        || input
            .governance_versions
            .iter()
            .any(|version| version.evidence_id.trim().is_empty())
    {
        reasons.insert(GovernanceEvidenceReason::MissingGovernanceEvidenceIdentifier);
    }

    if input.governance_versions.iter().any(|version| {
        version.governance_sources.is_empty()
            || version
                .governance_sources
                .iter()
                .any(|source| source.path.trim().is_empty())
    }) {
        reasons.insert(GovernanceEvidenceReason::MissingGovernanceSourceReference);
    }

    if input.governance_versions.iter().any(|version| {
        version.version_label.trim().is_empty()
            || version.deterministic_snapshot_label.trim().is_empty()
            || version.source_commit.trim().is_empty()
            || version
                .governance_sources
                .iter()
                .any(|source| source.version_fingerprint.trim().is_empty())
    }) {
        reasons.insert(GovernanceEvidenceReason::MissingGovernanceVersionFingerprint);
    }

    if input.policy_versions.is_empty()
        || input
            .policy_versions
            .iter()
            .any(|version| version.policy_version_label.trim().is_empty())
    {
        reasons.insert(GovernanceEvidenceReason::MissingPolicyVersionLabel);
    }

    if input.policy_versions.iter().any(|version| {
        version.evidence_id.trim().is_empty()
            || version.policy_sources.is_empty()
            || version.policy_sources.iter().any(|source| {
                source.path.trim().is_empty() || source.version_fingerprint.trim().is_empty()
            })
    }) {
        reasons.insert(GovernanceEvidenceReason::MissingPolicySourceReference);
    }

    if input.changelog_references.is_empty()
        || input.changelog_references.iter().any(|reference| {
            reference.path.trim().is_empty() || reference.version_label.trim().is_empty()
        })
    {
        reasons.insert(GovernanceEvidenceReason::MissingChangelogReference);
    }

    if input.roadmap_references.is_empty()
        || input.roadmap_references.iter().any(|reference| {
            reference.path.trim().is_empty() || reference.phase_label.trim().is_empty()
        })
    {
        reasons.insert(GovernanceEvidenceReason::MissingRoadmapReference);
    }

    if input.validation_run_references.is_empty()
        || input.validation_run_references.iter().any(|reference| {
            reference.command.trim().is_empty() || reference.deterministic_label.trim().is_empty()
        })
    {
        reasons.insert(GovernanceEvidenceReason::MissingValidationRunReference);
    }
}

fn collect_duplicate_identifier_rejections(
    input: &GovernanceEvidenceAttributionInput,
    reasons: &mut BTreeSet<GovernanceEvidenceReason>,
) {
    let mut counts = BTreeMap::new();
    for evidence_id in input
        .governance_versions
        .iter()
        .map(|version| version.evidence_id.trim())
        .filter(|evidence_id| !evidence_id.is_empty())
    {
        *counts.entry(evidence_id).or_insert(0usize) += 1;
    }
    if counts.values().any(|count| *count > 1) {
        reasons.insert(GovernanceEvidenceReason::DuplicateGovernanceEvidenceIdentifier);
    }
}

fn collect_contradictory_version_rejections(
    input: &GovernanceEvidenceAttributionInput,
    reasons: &mut BTreeSet<GovernanceEvidenceReason>,
) {
    let governance_labels: BTreeSet<_> = input
        .governance_versions
        .iter()
        .map(|version| version.version_label.trim())
        .filter(|label| !label.is_empty())
        .collect();
    if governance_labels.len() > 1 {
        reasons.insert(GovernanceEvidenceReason::ContradictoryGovernanceVersionLabel);
    }

    let policy_labels: BTreeSet<_> = input
        .policy_versions
        .iter()
        .map(|version| version.policy_version_label.trim())
        .filter(|label| !label.is_empty())
        .collect();
    if policy_labels.len() > 1 {
        reasons.insert(GovernanceEvidenceReason::ContradictoryPolicyVersionLabel);
    }
}

fn collect_truth_dimension_rejections(
    input: &GovernanceEvidenceAttributionInput,
    reasons: &mut BTreeSet<GovernanceEvidenceReason>,
) {
    let unsupported = input
        .governance_versions
        .iter()
        .flat_map(|version| version.governance_sources.iter())
        .chain(
            input
                .policy_versions
                .iter()
                .flat_map(|version| version.policy_sources.iter()),
        )
        .chain(input.operations_report_references.iter())
        .chain(input.checklist_references.iter())
        .any(|source| {
            matches!(
                source.truth_dimension,
                GovernanceEvidenceTruthDimension::Unsupported(_)
            )
        });

    if unsupported {
        reasons.insert(GovernanceEvidenceReason::UnsupportedTruthDimensionClaim);
    }
}

fn sorted_unique(values: impl Iterator<Item = String>) -> Vec<String> {
    values
        .filter(|value| !value.trim().is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(
        path: &str,
        truth_dimension: GovernanceEvidenceTruthDimension,
    ) -> GovernanceEvidenceSourceReference {
        GovernanceEvidenceSourceReference {
            path: path.to_string(),
            truth_dimension,
            version_fingerprint: format!("fingerprint:{path}"),
        }
    }

    fn valid_input() -> GovernanceEvidenceAttributionInput {
        GovernanceEvidenceAttributionInput {
            governance_versions: vec![GovernanceEvidenceVersion {
                evidence_id: "governance-evidence-phase-114".to_string(),
                version_label: "governance-phase-114".to_string(),
                deterministic_snapshot_label: "phase-114-snapshot".to_string(),
                source_commit: "committed-source-reference".to_string(),
                governance_sources: vec![source(
                    "docs/governance/GOVERNANCE.md",
                    GovernanceEvidenceTruthDimension::Normative,
                )],
            }],
            policy_versions: vec![PolicyVersionEvidence {
                evidence_id: "policy-evidence-phase-114".to_string(),
                policy_version_label: "policy-phase-114".to_string(),
                policy_sources: vec![source(
                    "docs/governance/phase-execution-contract.md",
                    GovernanceEvidenceTruthDimension::Normative,
                )],
            }],
            changelog_references: vec![GovernanceEvidenceChangelogReference {
                path: "CHANGELOG.md".to_string(),
                version_label: "v0.0.114".to_string(),
            }],
            roadmap_references: vec![GovernanceEvidenceRoadmapReference {
                path: "docs/roadmap/phase-map.md".to_string(),
                phase_label: "Phase 114".to_string(),
            }],
            operations_report_references: vec![source(
                "docs/operations/policy-governance-versioning-phase-114.md",
                GovernanceEvidenceTruthDimension::Orientation,
            )],
            checklist_references: vec![source(
                "checklists/current-phase.md",
                GovernanceEvidenceTruthDimension::Procedural,
            )],
            validation_run_references: vec![GovernanceEvidenceValidationRunReference {
                command: "cargo test --manifest-path core/Cargo.toml phase_114 --all-targets"
                    .to_string(),
                deterministic_label: "phase-114-validation".to_string(),
            }],
            reason_codes: vec![GovernanceEvidenceReason::AttributionOnlyAccepted],
            authority_denial_snapshot: GovernanceEvidenceAuthorityDenialSnapshot::all_denied(),
        }
    }

    #[test]
    fn phase_114_valid_governance_policy_evidence_validates_as_attribution_only() {
        let report = validate_governance_evidence_attribution(&valid_input());
        assert_eq!(report.status, GovernanceEvidenceValidationStatus::Accepted);
        assert!(report.attribution_only);
        assert!(report.non_authoritative);
        assert!(!governance_evidence_grants_authority(&report));
        assert_eq!(report.reason_codes, vec!["attribution_only_accepted"]);
    }

    #[test]
    fn phase_114_missing_governance_source_reference_rejects() {
        let mut input = valid_input();
        input.governance_versions[0].governance_sources.clear();
        assert!(validate_governance_evidence_attribution(&input)
            .reasons
            .contains(&GovernanceEvidenceReason::MissingGovernanceSourceReference));
    }

    #[test]
    fn phase_114_missing_policy_version_label_rejects() {
        let mut input = valid_input();
        input.policy_versions[0].policy_version_label.clear();
        assert!(validate_governance_evidence_attribution(&input)
            .reasons
            .contains(&GovernanceEvidenceReason::MissingPolicyVersionLabel));
    }

    #[test]
    fn phase_114_missing_changelog_reference_rejects() {
        let mut input = valid_input();
        input.changelog_references.clear();
        assert!(validate_governance_evidence_attribution(&input)
            .reasons
            .contains(&GovernanceEvidenceReason::MissingChangelogReference));
    }

    #[test]
    fn phase_114_missing_roadmap_reference_rejects() {
        let mut input = valid_input();
        input.roadmap_references.clear();
        assert!(validate_governance_evidence_attribution(&input)
            .reasons
            .contains(&GovernanceEvidenceReason::MissingRoadmapReference));
    }

    #[test]
    fn phase_114_missing_validation_run_reference_rejects() {
        let mut input = valid_input();
        input.validation_run_references.clear();
        assert!(validate_governance_evidence_attribution(&input)
            .reasons
            .contains(&GovernanceEvidenceReason::MissingValidationRunReference));
    }

    #[test]
    fn phase_114_duplicate_evidence_identifiers_reject() {
        let mut input = valid_input();
        input
            .governance_versions
            .push(input.governance_versions[0].clone());
        assert!(validate_governance_evidence_attribution(&input)
            .reasons
            .contains(&GovernanceEvidenceReason::DuplicateGovernanceEvidenceIdentifier));
    }

    #[test]
    fn phase_114_contradictory_governance_labels_reject() {
        let mut input = valid_input();
        let mut second = input.governance_versions[0].clone();
        second.evidence_id = "governance-evidence-phase-114-second".to_string();
        second.version_label = "contradictory-governance-label".to_string();
        input.governance_versions.push(second);
        assert!(validate_governance_evidence_attribution(&input)
            .reasons
            .contains(&GovernanceEvidenceReason::ContradictoryGovernanceVersionLabel));
    }

    #[test]
    fn phase_114_unsupported_truth_dimension_rejects() {
        let mut input = valid_input();
        input.governance_versions[0].governance_sources[0].truth_dimension =
            GovernanceEvidenceTruthDimension::Unsupported("authority".to_string());
        assert!(validate_governance_evidence_attribution(&input)
            .reasons
            .contains(&GovernanceEvidenceReason::UnsupportedTruthDimensionClaim));
    }

    #[test]
    fn phase_114_authority_and_approval_claims_reject() {
        let claims = [
            (
                "governance_authority_rewritten",
                GovernanceEvidenceReason::GovernanceAuthorityRewriteRejected,
            ),
            (
                "policy_authority_granted",
                GovernanceEvidenceReason::PolicyAuthorityGrantRejected,
            ),
            (
                "readiness_approved",
                GovernanceEvidenceReason::ReadinessApprovalRejected,
            ),
            (
                "deployment_approved",
                GovernanceEvidenceReason::DeploymentApprovalRejected,
            ),
            (
                "release_candidate_approved",
                GovernanceEvidenceReason::ReleaseCandidateApprovalRejected,
            ),
            (
                "production_candidate_approved",
                GovernanceEvidenceReason::ProductionCandidateApprovalRejected,
            ),
            (
                "public_use_approved",
                GovernanceEvidenceReason::PublicUseApprovalRejected,
            ),
            (
                "production_human_use_approved",
                GovernanceEvidenceReason::ProductionHumanUseApprovalRejected,
            ),
            (
                "provider_trust_granted",
                GovernanceEvidenceReason::ProviderTrustRejected,
            ),
            (
                "provider_output_promoted",
                GovernanceEvidenceReason::ProviderOutputPromotionRejected,
            ),
            (
                "persistence_authority_expanded",
                GovernanceEvidenceReason::PersistenceAuthorityExpansionRejected,
            ),
            (
                "replay_repaired",
                GovernanceEvidenceReason::ReplayRepairRejected,
            ),
            (
                "recovery_promoted",
                GovernanceEvidenceReason::RecoveryPromotionRejected,
            ),
            (
                "action_executed",
                GovernanceEvidenceReason::ActionExecutionRejected,
            ),
        ];

        for (field, reason) in claims {
            let mut input = valid_input();
            match field {
                "governance_authority_rewritten" => {
                    input
                        .authority_denial_snapshot
                        .governance_authority_rewritten = true
                }
                "policy_authority_granted" => {
                    input.authority_denial_snapshot.policy_authority_granted = true
                }
                "readiness_approved" => input.authority_denial_snapshot.readiness_approved = true,
                "deployment_approved" => input.authority_denial_snapshot.deployment_approved = true,
                "release_candidate_approved" => {
                    input.authority_denial_snapshot.release_candidate_approved = true
                }
                "production_candidate_approved" => {
                    input
                        .authority_denial_snapshot
                        .production_candidate_approved = true
                }
                "public_use_approved" => input.authority_denial_snapshot.public_use_approved = true,
                "production_human_use_approved" => {
                    input
                        .authority_denial_snapshot
                        .production_human_use_approved = true
                }
                "provider_trust_granted" => {
                    input.authority_denial_snapshot.provider_trust_granted = true
                }
                "provider_output_promoted" => {
                    input.authority_denial_snapshot.provider_output_promoted = true
                }
                "persistence_authority_expanded" => {
                    input
                        .authority_denial_snapshot
                        .persistence_authority_expanded = true
                }
                "replay_repaired" => input.authority_denial_snapshot.replay_repaired = true,
                "recovery_promoted" => input.authority_denial_snapshot.recovery_promoted = true,
                "action_executed" => input.authority_denial_snapshot.action_executed = true,
                _ => unreachable!(),
            }
            let report = validate_governance_evidence_attribution(&input);
            assert_eq!(report.status, GovernanceEvidenceValidationStatus::Rejected);
            assert!(report.reasons.contains(&reason), "{field}");
            assert!(!governance_evidence_grants_authority(&report));
        }
    }

    #[test]
    fn phase_114_deterministic_equivalent_evidence_produces_deterministic_report() {
        let first = validate_governance_evidence_attribution(&valid_input());
        let second = validate_governance_evidence_attribution(&valid_input());
        assert_eq!(first, second);
    }
}
