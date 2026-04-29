//! Built-in deterministic domain profiles.
//! These profiles define compatibility expectations only.
//! They do not register providers, execute evaluators, or define promotion authority.

use crate::domain::profile::DomainProfile;

pub fn code_patch_review_domain() -> DomainProfile {
    DomainProfile {
        id: "code_patch_review".to_string(),
        name: "Code Patch Review".to_string(),
        objective_types: vec!["patch_review".to_string(), "refactor_review".to_string()],
        constraint_types: vec!["style".to_string(), "correctness".to_string()],
        required_evaluators: vec!["static_analysis".to_string(), "test_regression".to_string()],
        optional_evaluators: vec!["readability_review".to_string()],
        known_failure_modes: vec!["logic_regression".to_string(), "missing_tests".to_string()],
        promotion_thresholds: vec!["all_required_pass".to_string()],
        evidence_requirements: vec!["diff".to_string(), "test_report".to_string()],
    }
}

pub fn security_analysis_domain() -> DomainProfile {
    DomainProfile {
        id: "security_analysis".to_string(),
        name: "Security Analysis".to_string(),
        objective_types: vec![
            "threat_review".to_string(),
            "vulnerability_triage".to_string(),
        ],
        constraint_types: vec!["security".to_string(), "compliance".to_string()],
        required_evaluators: vec!["threat_model_check".to_string(), "cwe_mapping".to_string()],
        optional_evaluators: vec!["exploitability_review".to_string()],
        known_failure_modes: vec![
            "false_negative".to_string(),
            "severity_mislabel".to_string(),
        ],
        promotion_thresholds: vec!["all_required_pass".to_string()],
        evidence_requirements: vec!["threat_report".to_string(), "cwe_evidence".to_string()],
    }
}

pub fn built_in_domains() -> Vec<DomainProfile> {
    vec![code_patch_review_domain(), security_analysis_domain()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::validate::validate_domain_profile;

    #[test]
    fn built_in_domains_have_distinct_ids() {
        let domains = built_in_domains();

        assert_ne!(domains[0].id, domains[1].id);
    }

    #[test]
    fn built_in_domains_have_distinct_required_evaluators() {
        let code = code_patch_review_domain();
        let security = security_analysis_domain();

        assert_ne!(code.required_evaluators, security.required_evaluators);
    }

    #[test]
    fn built_in_domains_validate() {
        for domain in built_in_domains() {
            validate_domain_profile(&domain).unwrap();
        }
    }
}
