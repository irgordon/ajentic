use crate::domain::registry::built_in_domains;
use crate::domain::validate::{find_domain, validate_domain_profile};

#[test]
fn domain_registry_identity_is_deterministic() {
    let first = built_in_domains();
    let second = built_in_domains();

    assert_eq!(first, second);
}

#[test]
fn domain_lookup_is_stable_across_runs() {
    let first = built_in_domains();
    let second = built_in_domains();

    let first_lookup = find_domain(&first, "code_patch_review").unwrap();
    let second_lookup = find_domain(&second, "code_patch_review").unwrap();

    assert_eq!(first_lookup, second_lookup);
}

#[test]
fn registry_ordering_is_deterministic() {
    let first = built_in_domains();
    let second = built_in_domains();

    let first_ids: Vec<&str> = first.iter().map(|profile| profile.id.as_str()).collect();
    let second_ids: Vec<&str> = second.iter().map(|profile| profile.id.as_str()).collect();

    assert_eq!(first_ids, second_ids);
}

#[test]
fn repeated_registry_initialization_is_idempotent() {
    let baseline = built_in_domains();

    for _ in 0..5 {
        let current = built_in_domains();

        assert_eq!(baseline, current);
        for profile in &current {
            assert_eq!(validate_domain_profile(profile), Ok(()));
        }
    }
}
