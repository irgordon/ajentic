use crate::domain::registry::built_in_domains;
use crate::domain::validate::find_domain;
use crate::governance::runtime::GovernanceStatus;
use crate::replay::status::ReplayFinalStatus;

#[test]
fn registry_access_is_read_only() {
    let governance_before = GovernanceStatus::Unknown;
    let replay_before = ReplayFinalStatus::Unknown;

    let registry_before = built_in_domains();

    for _ in 0..5 {
        let registry = built_in_domains();

        for profile in &registry {
            let found = find_domain(&registry, &profile.id).unwrap();
            assert_eq!(found, profile);
        }
    }

    let governance_after = GovernanceStatus::Unknown;
    let replay_after = ReplayFinalStatus::Unknown;
    let registry_after = built_in_domains();

    assert_eq!(governance_before, governance_after);
    assert_eq!(replay_before, replay_after);
    assert_eq!(registry_before, registry_after);
}
