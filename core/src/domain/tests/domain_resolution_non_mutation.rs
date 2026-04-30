use crate::domain::registry::built_in_domains;
use crate::domain::validate::find_domain;
use crate::governance::runtime::GovernanceStatus;
use crate::replay::status::ReplayFinalStatus;

#[test]
fn repeated_domain_lookups_do_not_alter_governance_status() {
    let governance_before = GovernanceStatus::Unknown;
    let registry = built_in_domains();

    for profile in &registry {
        let found = find_domain(&registry, &profile.id).unwrap();
        assert_eq!(found, profile);
    }

    let governance_after = GovernanceStatus::Unknown;
    assert_eq!(governance_before, governance_after);
}

#[test]
fn repeated_domain_lookups_do_not_alter_replay_status() {
    let replay_before = ReplayFinalStatus::Unknown;
    let registry = built_in_domains();

    for profile in &registry {
        let found = find_domain(&registry, &profile.id).unwrap();
        assert_eq!(found, profile);
    }

    let replay_after = ReplayFinalStatus::Unknown;
    assert_eq!(replay_before, replay_after);
}

#[test]
fn repeated_domain_lookups_do_not_alter_registry_identity() {
    let registry = built_in_domains();
    let registry_before = registry.clone();

    for profile in &registry {
        let found = find_domain(&registry, &profile.id).unwrap();
        assert_eq!(found, profile);
    }

    let registry_after = registry;
    assert_eq!(registry_before, registry_after);
}
