use crate::domain::registry::{built_in_domains, code_patch_review_domain};
use crate::domain::validate::{find_domain, validate_domain_profile};
use crate::errors::DomainValidationError;

#[test]
fn duplicate_domain_registration_is_rejected() {
    let first = code_patch_review_domain();
    let mut second = code_patch_review_domain();
    second.name = "Code Patch Review Duplicate".to_string();

    let profiles = vec![first, second];

    assert_eq!(
        find_domain(&profiles, "code_patch_review"),
        Err(DomainValidationError::DuplicateDomainId {
            domain_id: "code_patch_review".to_string(),
        })
    );
}

#[test]
fn conflicting_domain_profile_is_rejected() {
    let mut profile = code_patch_review_domain();
    profile
        .required_evaluators
        .push("static_analysis".to_string());

    assert_eq!(
        validate_domain_profile(&profile),
        Err(DomainValidationError::DuplicateRequiredEvaluator {
            evaluator_id: "static_analysis".to_string(),
        })
    );
}

#[test]
fn registry_rejects_inconsistent_metadata() {
    let mut profile = code_patch_review_domain();
    profile
        .optional_evaluators
        .push("test_regression".to_string());

    assert_eq!(
        validate_domain_profile(&profile),
        Err(
            DomainValidationError::EvaluatorListedAsRequiredAndOptional {
                evaluator_id: "test_regression".to_string(),
            }
        )
    );
}

#[test]
fn registry_failure_is_deterministic() {
    let mut profile = code_patch_review_domain();
    profile.id.clear();

    let first = validate_domain_profile(&profile);
    let second = validate_domain_profile(&profile);

    assert_eq!(first, Err(DomainValidationError::MissingDomainId));
    assert_eq!(first, second);

    let mut duplicated = built_in_domains();
    duplicated.push(code_patch_review_domain());

    let first_duplicate_lookup = find_domain(&duplicated, "code_patch_review");
    let second_duplicate_lookup = find_domain(&duplicated, "code_patch_review");

    assert_eq!(
        first_duplicate_lookup,
        Err(DomainValidationError::DuplicateDomainId {
            domain_id: "code_patch_review".to_string(),
        })
    );
    assert_eq!(first_duplicate_lookup, second_duplicate_lookup);
}
