use crate::domain::registry::built_in_domains;
use crate::domain::validate::find_domain;
use crate::errors::DomainValidationError;

#[test]
fn lookup_of_unknown_domain_id_fails() {
    let registry = built_in_domains();

    let result = find_domain(&registry, "unknown_domain_id");

    assert_eq!(
        result,
        Err(DomainValidationError::DomainNotFound {
            domain_id: "unknown_domain_id".to_string(),
        })
    );
}

#[test]
fn lookup_of_empty_domain_id_fails() {
    let registry = built_in_domains();

    let result = find_domain(&registry, "");

    assert_eq!(result, Err(DomainValidationError::MissingDomainId));
}

#[test]
fn lookup_of_malformed_domain_id_fails() {
    let registry = built_in_domains();

    let result = find_domain(&registry, "code patch review");

    assert_eq!(
        result,
        Err(DomainValidationError::DomainNotFound {
            domain_id: "code patch review".to_string(),
        })
    );
}

#[test]
fn repeated_lookup_failure_is_deterministic() {
    let registry = built_in_domains();

    let first_failure = find_domain(&registry, "invalid-domain-id");
    let second_failure = find_domain(&registry, "invalid-domain-id");

    assert_eq!(first_failure, second_failure);
}

#[test]
fn failure_does_not_mutate_registry_state() {
    let registry = built_in_domains();
    let registry_before = registry.clone();

    let _ = find_domain(&registry, "not_present");

    let registry_after = registry;
    assert_eq!(registry_before, registry_after);
}
