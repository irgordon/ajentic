use crate::domain::registry::code_patch_review_domain;
use crate::domain::validate::{find_domain, validate_domain_profile};
use crate::errors::DomainValidationError;

#[test]
fn duplicate_domain_id_is_rejected() {
    let a = code_patch_review_domain();
    let mut b = code_patch_review_domain();
    b.name = "Duplicate".into();

    let domains = [a, b];
    let result = find_domain(&domains, "code_patch_review");
    assert_eq!(
        result,
        Err(DomainValidationError::DuplicateDomainId {
            domain_id: "code_patch_review".into(),
        })
    );
}

#[test]
fn malformed_domain_profile_is_rejected() {
    let mut profile = code_patch_review_domain();
    profile.required_evaluators.push(String::new());

    assert_eq!(
        validate_domain_profile(&profile),
        Err(DomainValidationError::EmptyEvaluatorId)
    );
}

#[test]
fn missing_required_domain_fields_fail_closed() {
    let mut profile = code_patch_review_domain();
    profile.required_evaluators.clear();

    assert_eq!(
        validate_domain_profile(&profile),
        Err(DomainValidationError::MissingRequiredEvaluators)
    );
}

#[test]
fn unknown_domain_reference_is_rejected() {
    let domains = [code_patch_review_domain()];
    let result = find_domain(&domains, "nonexistent-domain");

    assert_eq!(
        result,
        Err(DomainValidationError::DomainNotFound {
            domain_id: "nonexistent-domain".into(),
        })
    );
}
