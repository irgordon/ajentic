use crate::domain::registry::built_in_domains;
use crate::governance::runtime::GovernanceStatus;
use crate::replay::status::ReplayFinalStatus;

fn canonical_ids() -> Vec<String> {
    built_in_domains()
        .into_iter()
        .map(|profile| profile.id)
        .collect()
}

#[test]
fn registry_cannot_change_after_initialization() {
    let registry_before = built_in_domains();

    let _ = built_in_domains();
    let _ = canonical_ids();

    let registry_after = built_in_domains();

    assert_eq!(registry_before, registry_after);
}

#[test]
fn registry_contents_are_stable_across_repeated_access() {
    let baseline = canonical_ids();

    for _ in 0..10 {
        assert_eq!(baseline, canonical_ids());
    }
}

#[test]
fn registry_identity_is_stable_across_repeated_construction() {
    let first = built_in_domains();
    let second = built_in_domains();
    let third = built_in_domains();

    assert_eq!(first, second);
    assert_eq!(second, third);
}

#[test]
fn registry_access_does_not_mutate_governance_or_replay_status() {
    let governance_status_before = GovernanceStatus::Unknown;
    let replay_status_before = ReplayFinalStatus::Unknown;

    let registry_before = built_in_domains();
    let _ = canonical_ids();
    let _ = built_in_domains();
    let registry_after = built_in_domains();

    let governance_status_after = GovernanceStatus::Unknown;
    let replay_status_after = ReplayFinalStatus::Unknown;

    assert_eq!(registry_before, registry_after);
    assert_eq!(governance_status_before, governance_status_after);
    assert_eq!(replay_status_before, replay_status_after);
}

#[test]
fn built_in_domain_extension_requires_intentional_inventory_update() {
    assert_eq!(
        canonical_ids(),
        vec![
            "code_patch_review".to_string(),
            "security_analysis".to_string(),
        ]
    );
}
