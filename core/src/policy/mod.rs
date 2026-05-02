#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allowed,
    Denied,
    Blocked,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyEvidence {
    pub has_required_context: bool,
    pub has_required_validation: bool,
    pub has_required_operator_intent: bool,
    pub model_output_claims_success: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyResult {
    pub decision: PolicyDecision,
    pub reason: &'static str,
}

impl PolicyResult {
    pub fn unknown() -> Self {
        Self {
            decision: PolicyDecision::Unknown,
            reason: "unknown_is_not_pass",
        }
    }

    pub fn allowed(reason: &'static str) -> Self {
        Self {
            decision: PolicyDecision::Allowed,
            reason,
        }
    }

    pub fn denied(reason: &'static str) -> Self {
        Self {
            decision: PolicyDecision::Denied,
            reason,
        }
    }

    pub fn blocked(reason: &'static str) -> Self {
        Self {
            decision: PolicyDecision::Blocked,
            reason,
        }
    }
}

pub fn evaluate_policy(evidence: &PolicyEvidence) -> PolicyResult {
    if !evidence.has_required_context {
        return PolicyResult::denied("missing_required_context");
    }

    if !evidence.has_required_validation {
        return PolicyResult::denied("missing_required_validation");
    }

    if !evidence.has_required_operator_intent {
        return PolicyResult::denied("missing_required_operator_intent");
    }

    PolicyResult::allowed("required_policy_evidence_present")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_unknown_is_not_pass() {
        let result = PolicyResult::unknown();

        assert_eq!(result.decision, PolicyDecision::Unknown);
        assert_eq!(result.reason, "unknown_is_not_pass");
    }

    #[test]
    fn policy_missing_context_denies() {
        let result = evaluate_policy(&PolicyEvidence {
            has_required_context: false,
            has_required_validation: true,
            has_required_operator_intent: true,
            model_output_claims_success: false,
        });

        assert_eq!(result.decision, PolicyDecision::Denied);
        assert_eq!(result.reason, "missing_required_context");
    }

    #[test]
    fn policy_missing_validation_denies() {
        let result = evaluate_policy(&PolicyEvidence {
            has_required_context: true,
            has_required_validation: false,
            has_required_operator_intent: true,
            model_output_claims_success: false,
        });

        assert_eq!(result.decision, PolicyDecision::Denied);
        assert_eq!(result.reason, "missing_required_validation");
    }

    #[test]
    fn policy_missing_operator_intent_denies() {
        let result = evaluate_policy(&PolicyEvidence {
            has_required_context: true,
            has_required_validation: true,
            has_required_operator_intent: false,
            model_output_claims_success: false,
        });

        assert_eq!(result.decision, PolicyDecision::Denied);
        assert_eq!(result.reason, "missing_required_operator_intent");
    }

    #[test]
    fn policy_required_evidence_allows() {
        let result = evaluate_policy(&PolicyEvidence {
            has_required_context: true,
            has_required_validation: true,
            has_required_operator_intent: true,
            model_output_claims_success: false,
        });

        assert_eq!(result.decision, PolicyDecision::Allowed);
        assert_eq!(result.reason, "required_policy_evidence_present");
    }

    #[test]
    fn policy_model_success_claim_alone_does_not_allow() {
        let result = evaluate_policy(&PolicyEvidence {
            has_required_context: false,
            has_required_validation: false,
            has_required_operator_intent: false,
            model_output_claims_success: true,
        });

        assert_eq!(result.decision, PolicyDecision::Denied);
        assert_ne!(result.decision, PolicyDecision::Allowed);
    }

    #[test]
    fn policy_model_success_claim_does_not_override_missing_validation() {
        let result = evaluate_policy(&PolicyEvidence {
            has_required_context: true,
            has_required_validation: false,
            has_required_operator_intent: true,
            model_output_claims_success: true,
        });

        assert_eq!(result.decision, PolicyDecision::Denied);
        assert_eq!(result.reason, "missing_required_validation");
    }

    #[test]
    fn policy_result_reasons_are_stable() {
        let all_missing = evaluate_policy(&PolicyEvidence {
            has_required_context: false,
            has_required_validation: false,
            has_required_operator_intent: false,
            model_output_claims_success: true,
        });
        assert_eq!(all_missing.reason, "missing_required_context");

        let missing_validation_and_intent = evaluate_policy(&PolicyEvidence {
            has_required_context: true,
            has_required_validation: false,
            has_required_operator_intent: false,
            model_output_claims_success: true,
        });
        assert_eq!(
            missing_validation_and_intent.reason,
            "missing_required_validation"
        );

        let passes = evaluate_policy(&PolicyEvidence {
            has_required_context: true,
            has_required_validation: true,
            has_required_operator_intent: true,
            model_output_claims_success: true,
        });
        assert_eq!(passes.reason, "required_policy_evidence_present");
    }
}
