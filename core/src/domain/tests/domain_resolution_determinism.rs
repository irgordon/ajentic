use crate::domain::registry::built_in_domains;
use crate::domain::validate::find_domain;

#[test]
fn repeated_lookup_returns_identical_profile() {
    let registry = built_in_domains();

    let lookup_first = find_domain(&registry, "code_patch_review").unwrap();
    let lookup_second = find_domain(&registry, "code_patch_review").unwrap();

    assert_eq!(lookup_first, lookup_second);
}

#[test]
fn lookup_ordering_does_not_affect_result() {
    let registry = built_in_domains();

    let _ = find_domain(&registry, "security_analysis").unwrap();
    let lookup_after_alternate_order = find_domain(&registry, "code_patch_review").unwrap();

    let direct_lookup = find_domain(&registry, "code_patch_review").unwrap();

    assert_eq!(lookup_after_alternate_order, direct_lookup);
}

#[test]
fn registry_reconstruction_does_not_change_lookup_result() {
    let first_registry = built_in_domains();
    let second_registry = built_in_domains();

    let lookup_from_first_registry = find_domain(&first_registry, "security_analysis").unwrap();
    let lookup_from_second_registry = find_domain(&second_registry, "security_analysis").unwrap();

    assert_eq!(lookup_from_first_registry, lookup_from_second_registry);
}
