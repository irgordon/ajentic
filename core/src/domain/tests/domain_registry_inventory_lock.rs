use crate::domain::registry::built_in_domains;

fn registry_domain_ids() -> Vec<String> {
    built_in_domains()
        .into_iter()
        .map(|profile| profile.id)
        .collect()
}

#[test]
fn canonical_built_in_domain_inventory_is_locked() {
    let domain_ids = registry_domain_ids();

    assert_eq!(
        domain_ids,
        vec![
            "code_patch_review".to_string(),
            "security_analysis".to_string(),
        ]
    );
}

#[test]
fn canonical_inventory_order_is_deterministic() {
    let first = registry_domain_ids();
    let second = registry_domain_ids();

    assert_eq!(first, second);
    assert_eq!(
        first,
        vec![
            "code_patch_review".to_string(),
            "security_analysis".to_string(),
        ]
    );
}

#[test]
fn canonical_registry_identity_is_stable() {
    let first = built_in_domains();
    let second = built_in_domains();

    assert_eq!(first, second);
}
