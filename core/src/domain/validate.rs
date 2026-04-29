//! Domain profile validation and compatibility checks.

use std::collections::HashSet;

use crate::domain::profile::DomainProfile;
use crate::errors::DomainValidationError;

pub fn validate_domain_profile(profile: &DomainProfile) -> Result<(), DomainValidationError> {
    if profile.id.is_empty() {
        return Err(DomainValidationError::MissingDomainId);
    }
    if profile.name.is_empty() {
        return Err(DomainValidationError::MissingDomainName);
    }
    if profile.objective_types.is_empty() {
        return Err(DomainValidationError::MissingObjectiveTypes);
    }
    if profile.constraint_types.is_empty() {
        return Err(DomainValidationError::MissingConstraintTypes);
    }
    if profile.required_evaluators.is_empty() {
        return Err(DomainValidationError::MissingRequiredEvaluators);
    }
    if profile.evidence_requirements.is_empty() {
        return Err(DomainValidationError::MissingEvidenceRequirements);
    }

    validate_unique_and_non_empty(
        &profile.objective_types,
        DomainValidationError::EmptyObjectiveType,
        |value| DomainValidationError::DuplicateObjectiveType {
            objective_type: value.to_string(),
        },
    )?;

    validate_unique_and_non_empty(
        &profile.constraint_types,
        DomainValidationError::EmptyConstraintType,
        |value| DomainValidationError::DuplicateConstraintType {
            constraint_type: value.to_string(),
        },
    )?;

    validate_unique_and_non_empty(
        &profile.required_evaluators,
        DomainValidationError::EmptyEvaluatorId,
        |value| DomainValidationError::DuplicateRequiredEvaluator {
            evaluator_id: value.to_string(),
        },
    )?;

    validate_unique_and_non_empty(
        &profile.optional_evaluators,
        DomainValidationError::EmptyEvaluatorId,
        |value| DomainValidationError::DuplicateOptionalEvaluator {
            evaluator_id: value.to_string(),
        },
    )?;

    validate_unique_and_non_empty(
        &profile.known_failure_modes,
        DomainValidationError::EmptyKnownFailureMode,
        |value| DomainValidationError::DuplicateKnownFailureMode {
            known_failure_mode: value.to_string(),
        },
    )?;

    validate_unique_and_non_empty(
        &profile.promotion_thresholds,
        DomainValidationError::EmptyPromotionThreshold,
        |value| DomainValidationError::DuplicatePromotionThreshold {
            promotion_threshold: value.to_string(),
        },
    )?;

    validate_unique_and_non_empty(
        &profile.evidence_requirements,
        DomainValidationError::EmptyEvidenceRequirement,
        |value| DomainValidationError::DuplicateEvidenceRequirement {
            evidence_requirement: value.to_string(),
        },
    )?;

    for evaluator_id in &profile.required_evaluators {
        if profile
            .optional_evaluators
            .iter()
            .any(|it| it == evaluator_id)
        {
            return Err(
                DomainValidationError::EvaluatorListedAsRequiredAndOptional {
                    evaluator_id: evaluator_id.clone(),
                },
            );
        }
    }

    Ok(())
}

pub fn objective_supported(
    profile: &DomainProfile,
    objective_type: &str,
) -> Result<(), DomainValidationError> {
    if objective_type.is_empty() {
        return Err(DomainValidationError::EmptyObjectiveType);
    }

    if !profile
        .objective_types
        .iter()
        .any(|it| it == objective_type)
    {
        return Err(DomainValidationError::UnsupportedObjectiveType {
            domain_id: profile.id.clone(),
            objective_type: objective_type.to_string(),
        });
    }

    Ok(())
}

pub fn required_evaluators_supported(
    profile: &DomainProfile,
    evaluator_ids: &[String],
) -> Result<(), DomainValidationError> {
    if evaluator_ids.is_empty() {
        return Err(DomainValidationError::MissingRequiredEvaluators);
    }

    for evaluator_id in evaluator_ids {
        if evaluator_id.is_empty() {
            return Err(DomainValidationError::EmptyEvaluatorId);
        }

        let supported = profile
            .required_evaluators
            .iter()
            .any(|it| it == evaluator_id)
            || profile
                .optional_evaluators
                .iter()
                .any(|it| it == evaluator_id);

        if !supported {
            return Err(DomainValidationError::UnsupportedEvaluator {
                domain_id: profile.id.clone(),
                evaluator_id: evaluator_id.clone(),
            });
        }
    }

    for required in &profile.required_evaluators {
        if !evaluator_ids.iter().any(|it| it == required) {
            return Err(DomainValidationError::MissingDomainRequiredEvaluator {
                domain_id: profile.id.clone(),
                evaluator_id: required.clone(),
            });
        }
    }

    Ok(())
}

pub fn find_domain<'a>(
    profiles: &'a [DomainProfile],
    domain_id: &str,
) -> Result<&'a DomainProfile, DomainValidationError> {
    if domain_id.is_empty() {
        return Err(DomainValidationError::MissingDomainId);
    }

    let mut matches = profiles.iter().filter(|profile| profile.id == domain_id);

    let Some(first) = matches.next() else {
        return Err(DomainValidationError::DomainNotFound {
            domain_id: domain_id.to_string(),
        });
    };

    if matches.next().is_some() {
        return Err(DomainValidationError::DuplicateDomainId {
            domain_id: domain_id.to_string(),
        });
    }

    Ok(first)
}

fn validate_unique_and_non_empty<F>(
    values: &[String],
    empty_error: DomainValidationError,
    duplicate_error: F,
) -> Result<(), DomainValidationError>
where
    F: Fn(&str) -> DomainValidationError,
{
    let mut seen = HashSet::new();

    for value in values {
        if value.is_empty() {
            return Err(empty_error.clone());
        }

        if !seen.insert(value.as_str()) {
            return Err(duplicate_error(value));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::registry::{
        built_in_domains, code_patch_review_domain, security_analysis_domain,
    };

    #[test]
    fn valid_code_patch_review_domain_passes() {
        assert!(validate_domain_profile(&code_patch_review_domain()).is_ok());
    }

    #[test]
    fn valid_security_analysis_domain_passes() {
        assert!(validate_domain_profile(&security_analysis_domain()).is_ok());
    }

    #[test]
    fn built_in_domains_validate() {
        for domain in built_in_domains() {
            assert!(validate_domain_profile(&domain).is_ok());
        }
    }

    #[test]
    fn empty_domain_id_fails() {
        let mut profile = code_patch_review_domain();
        profile.id.clear();

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::MissingDomainId)
        );
    }

    #[test]
    fn empty_name_fails() {
        let mut profile = code_patch_review_domain();
        profile.name.clear();

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::MissingDomainName)
        );
    }

    #[test]
    fn empty_objective_types_fails() {
        let mut profile = code_patch_review_domain();
        profile.objective_types.clear();

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::MissingObjectiveTypes)
        );
    }

    #[test]
    fn empty_constraint_types_fails() {
        let mut profile = code_patch_review_domain();
        profile.constraint_types.clear();

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::MissingConstraintTypes)
        );
    }

    #[test]
    fn empty_required_evaluators_fails() {
        let mut profile = code_patch_review_domain();
        profile.required_evaluators.clear();

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::MissingRequiredEvaluators)
        );
    }

    #[test]
    fn empty_evidence_requirements_fails() {
        let mut profile = code_patch_review_domain();
        profile.evidence_requirements.clear();

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::MissingEvidenceRequirements)
        );
    }

    #[test]
    fn empty_objective_type_fails() {
        let mut profile = code_patch_review_domain();
        profile.objective_types.push(String::new());

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::EmptyObjectiveType)
        );
    }

    #[test]
    fn empty_required_evaluator_fails() {
        let mut profile = code_patch_review_domain();
        profile.required_evaluators.push(String::new());

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::EmptyEvaluatorId)
        );
    }

    #[test]
    fn empty_known_failure_mode_fails() {
        let mut profile = code_patch_review_domain();
        profile.known_failure_modes.push(String::new());

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::EmptyKnownFailureMode)
        );
    }

    #[test]
    fn empty_promotion_threshold_fails() {
        let mut profile = code_patch_review_domain();
        profile.promotion_thresholds.push(String::new());

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::EmptyPromotionThreshold)
        );
    }

    #[test]
    fn duplicate_objective_type_fails() {
        let mut profile = code_patch_review_domain();
        let value = profile.objective_types[0].clone();
        profile.objective_types.push(value.clone());

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::DuplicateObjectiveType {
                objective_type: value
            })
        );
    }

    #[test]
    fn duplicate_required_evaluator_fails() {
        let mut profile = code_patch_review_domain();
        let value = profile.required_evaluators[0].clone();
        profile.required_evaluators.push(value.clone());

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::DuplicateRequiredEvaluator {
                evaluator_id: value
            })
        );
    }

    #[test]
    fn duplicate_optional_evaluator_fails() {
        let mut profile = code_patch_review_domain();
        let value = profile.optional_evaluators[0].clone();
        profile.optional_evaluators.push(value.clone());

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::DuplicateOptionalEvaluator {
                evaluator_id: value
            })
        );
    }

    #[test]
    fn duplicate_known_failure_mode_fails() {
        let mut profile = code_patch_review_domain();
        let value = profile.known_failure_modes[0].clone();
        profile.known_failure_modes.push(value.clone());

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::DuplicateKnownFailureMode {
                known_failure_mode: value
            })
        );
    }

    #[test]
    fn duplicate_promotion_threshold_fails() {
        let mut profile = code_patch_review_domain();
        let value = profile.promotion_thresholds[0].clone();
        profile.promotion_thresholds.push(value.clone());

        assert_eq!(
            validate_domain_profile(&profile),
            Err(DomainValidationError::DuplicatePromotionThreshold {
                promotion_threshold: value
            })
        );
    }

    #[test]
    fn required_and_optional_overlap_fails() {
        let mut profile = code_patch_review_domain();
        let value = profile.required_evaluators[0].clone();
        profile.optional_evaluators.push(value.clone());

        assert_eq!(
            validate_domain_profile(&profile),
            Err(
                DomainValidationError::EvaluatorListedAsRequiredAndOptional {
                    evaluator_id: value,
                },
            )
        );
    }

    #[test]
    fn supported_objective_passes() {
        let profile = code_patch_review_domain();

        assert!(objective_supported(&profile, "patch_review").is_ok());
    }

    #[test]
    fn unsupported_objective_fails() {
        let profile = code_patch_review_domain();

        assert_eq!(
            objective_supported(&profile, "foo"),
            Err(DomainValidationError::UnsupportedObjectiveType {
                domain_id: profile.id,
                objective_type: "foo".into(),
            })
        );
    }

    #[test]
    fn empty_objective_fails() {
        let profile = code_patch_review_domain();

        assert_eq!(
            objective_supported(&profile, ""),
            Err(DomainValidationError::EmptyObjectiveType)
        );
    }

    #[test]
    fn complete_required_evaluators_pass() {
        let profile = code_patch_review_domain();

        assert!(required_evaluators_supported(&profile, &profile.required_evaluators).is_ok());
    }

    #[test]
    fn missing_required_evaluator_fails() {
        let profile = code_patch_review_domain();
        let ids = vec![profile.required_evaluators[0].clone()];

        assert_eq!(
            required_evaluators_supported(&profile, &ids),
            Err(DomainValidationError::MissingDomainRequiredEvaluator {
                domain_id: profile.id,
                evaluator_id: "test_regression".into(),
            })
        );
    }

    #[test]
    fn unsupported_evaluator_fails() {
        let profile = code_patch_review_domain();
        let ids = vec!["bad".into()];

        assert_eq!(
            required_evaluators_supported(&profile, &ids),
            Err(DomainValidationError::UnsupportedEvaluator {
                domain_id: profile.id,
                evaluator_id: "bad".into(),
            })
        );
    }

    #[test]
    fn empty_evaluator_id_fails() {
        let profile = code_patch_review_domain();
        let ids = vec![String::new()];

        assert_eq!(
            required_evaluators_supported(&profile, &ids),
            Err(DomainValidationError::EmptyEvaluatorId)
        );
    }

    #[test]
    fn optional_evaluator_can_be_present() {
        let profile = code_patch_review_domain();
        let mut ids = profile.required_evaluators.clone();
        ids.push(profile.optional_evaluators[0].clone());

        assert!(required_evaluators_supported(&profile, &ids).is_ok());
    }

    #[test]
    fn find_domain_missing_id_fails() {
        let profiles = built_in_domains();

        assert_eq!(
            find_domain(&profiles, "missing"),
            Err(DomainValidationError::DomainNotFound {
                domain_id: "missing".into(),
            })
        );
    }

    #[test]
    fn find_domain_duplicate_id_fails() {
        let mut profiles = built_in_domains();
        profiles.push(code_patch_review_domain());

        assert_eq!(
            find_domain(&profiles, "code_patch_review"),
            Err(DomainValidationError::DuplicateDomainId {
                domain_id: "code_patch_review".into(),
            })
        );
    }
}
