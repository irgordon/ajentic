#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderFailureKind {
    None,
    Timeout,
    Cancelled,
    MalformedOutput,
    TransportRejected,
    ProviderUnavailable,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderRetryEligibility {
    Eligible,
    Ineligible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderRetryDecisionReason {
    RetryEligible,
    NoFailure,
    TimeoutRetryAllowed,
    TimeoutNotRetryable,
    CancelledNotRetryable,
    MalformedOutputNotRetryable,
    TransportRejectedNotRetryable,
    ProviderUnavailableRetryAllowed,
    ProviderUnavailableNotRetryable,
    UnknownFailureNotRetryable,
    RetryLimitReached,
    EmptyPolicyId,
    InvalidRetryLimit,
}

impl ProviderRetryDecisionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::RetryEligible => "retry_eligible",
            Self::NoFailure => "no_failure",
            Self::TimeoutRetryAllowed => "timeout_retry_allowed",
            Self::TimeoutNotRetryable => "timeout_not_retryable",
            Self::CancelledNotRetryable => "cancelled_not_retryable",
            Self::MalformedOutputNotRetryable => "malformed_output_not_retryable",
            Self::TransportRejectedNotRetryable => "transport_rejected_not_retryable",
            Self::ProviderUnavailableRetryAllowed => "provider_unavailable_retry_allowed",
            Self::ProviderUnavailableNotRetryable => "provider_unavailable_not_retryable",
            Self::UnknownFailureNotRetryable => "unknown_failure_not_retryable",
            Self::RetryLimitReached => "retry_limit_reached",
            Self::EmptyPolicyId => "empty_policy_id",
            Self::InvalidRetryLimit => "invalid_retry_limit",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderRetryPolicy {
    pub policy_id: String,
    pub max_attempts: u32,
    pub retry_timeouts: bool,
    pub retry_provider_unavailable: bool,
}

impl ProviderRetryPolicy {
    pub fn new(
        policy_id: impl Into<String>,
        max_attempts: u32,
        retry_timeouts: bool,
        retry_provider_unavailable: bool,
    ) -> Result<Self, ProviderRetryDecisionReason> {
        let policy_id = policy_id.into();
        if policy_id.is_empty() {
            return Err(ProviderRetryDecisionReason::EmptyPolicyId);
        }
        if max_attempts == 0 {
            return Err(ProviderRetryDecisionReason::InvalidRetryLimit);
        }

        Ok(Self {
            policy_id,
            max_attempts,
            retry_timeouts,
            retry_provider_unavailable,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderFailureReport {
    pub failure_kind: ProviderFailureKind,
    pub retry_eligibility: ProviderRetryEligibility,
    pub reason: ProviderRetryDecisionReason,
    pub attempt: u32,
    pub max_attempts: u32,
    pub mutates_authority: bool,
    pub schedules_retry: bool,
    pub summary: String,
}

pub fn classify_provider_failure(
    failure_kind: ProviderFailureKind,
    attempt: u32,
    policy: &ProviderRetryPolicy,
) -> ProviderFailureReport {
    let (retry_eligibility, reason) = if attempt >= policy.max_attempts {
        (
            ProviderRetryEligibility::Ineligible,
            ProviderRetryDecisionReason::RetryLimitReached,
        )
    } else {
        match failure_kind {
            ProviderFailureKind::None => (
                ProviderRetryEligibility::Ineligible,
                ProviderRetryDecisionReason::NoFailure,
            ),
            ProviderFailureKind::Timeout => {
                if policy.retry_timeouts {
                    (
                        ProviderRetryEligibility::Eligible,
                        ProviderRetryDecisionReason::TimeoutRetryAllowed,
                    )
                } else {
                    (
                        ProviderRetryEligibility::Ineligible,
                        ProviderRetryDecisionReason::TimeoutNotRetryable,
                    )
                }
            }
            ProviderFailureKind::Cancelled => (
                ProviderRetryEligibility::Ineligible,
                ProviderRetryDecisionReason::CancelledNotRetryable,
            ),
            ProviderFailureKind::MalformedOutput => (
                ProviderRetryEligibility::Ineligible,
                ProviderRetryDecisionReason::MalformedOutputNotRetryable,
            ),
            ProviderFailureKind::TransportRejected => (
                ProviderRetryEligibility::Ineligible,
                ProviderRetryDecisionReason::TransportRejectedNotRetryable,
            ),
            ProviderFailureKind::ProviderUnavailable => {
                if policy.retry_provider_unavailable {
                    (
                        ProviderRetryEligibility::Eligible,
                        ProviderRetryDecisionReason::ProviderUnavailableRetryAllowed,
                    )
                } else {
                    (
                        ProviderRetryEligibility::Ineligible,
                        ProviderRetryDecisionReason::ProviderUnavailableNotRetryable,
                    )
                }
            }
            ProviderFailureKind::Unknown => (
                ProviderRetryEligibility::Ineligible,
                ProviderRetryDecisionReason::UnknownFailureNotRetryable,
            ),
        }
    };

    ProviderFailureReport {
        failure_kind,
        retry_eligibility,
        reason,
        attempt,
        max_attempts: policy.max_attempts,
        mutates_authority: false,
        schedules_retry: false,
        summary:
            "Provider failure classification is deterministic and does not schedule or execute retries."
                .to_string(),
    }
}

pub fn provider_failure_report_mutates_authority(_report: &ProviderFailureReport) -> bool {
    false
}

pub fn provider_failure_report_schedules_retry(_report: &ProviderFailureReport) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy_fixture() -> ProviderRetryPolicy {
        ProviderRetryPolicy::new("policy-1", 3, true, true).expect("policy fixture should be valid")
    }

    #[test]
    fn provider_retry_reason_codes_are_stable() {
        assert_eq!(
            ProviderRetryDecisionReason::RetryEligible.code(),
            "retry_eligible"
        );
        assert_eq!(ProviderRetryDecisionReason::NoFailure.code(), "no_failure");
        assert_eq!(
            ProviderRetryDecisionReason::TimeoutRetryAllowed.code(),
            "timeout_retry_allowed"
        );
        assert_eq!(
            ProviderRetryDecisionReason::TimeoutNotRetryable.code(),
            "timeout_not_retryable"
        );
        assert_eq!(
            ProviderRetryDecisionReason::CancelledNotRetryable.code(),
            "cancelled_not_retryable"
        );
        assert_eq!(
            ProviderRetryDecisionReason::MalformedOutputNotRetryable.code(),
            "malformed_output_not_retryable"
        );
        assert_eq!(
            ProviderRetryDecisionReason::TransportRejectedNotRetryable.code(),
            "transport_rejected_not_retryable"
        );
        assert_eq!(
            ProviderRetryDecisionReason::ProviderUnavailableRetryAllowed.code(),
            "provider_unavailable_retry_allowed"
        );
        assert_eq!(
            ProviderRetryDecisionReason::ProviderUnavailableNotRetryable.code(),
            "provider_unavailable_not_retryable"
        );
        assert_eq!(
            ProviderRetryDecisionReason::UnknownFailureNotRetryable.code(),
            "unknown_failure_not_retryable"
        );
        assert_eq!(
            ProviderRetryDecisionReason::RetryLimitReached.code(),
            "retry_limit_reached"
        );
        assert_eq!(
            ProviderRetryDecisionReason::EmptyPolicyId.code(),
            "empty_policy_id"
        );
        assert_eq!(
            ProviderRetryDecisionReason::InvalidRetryLimit.code(),
            "invalid_retry_limit"
        );
    }

    #[test]
    fn retry_policy_requires_policy_id() {
        assert_eq!(
            ProviderRetryPolicy::new("", 1, true, true),
            Err(ProviderRetryDecisionReason::EmptyPolicyId)
        );
    }
    #[test]
    fn retry_policy_requires_positive_max_attempts() {
        assert_eq!(
            ProviderRetryPolicy::new("policy", 0, true, true),
            Err(ProviderRetryDecisionReason::InvalidRetryLimit)
        );
    }
    #[test]
    fn timeout_is_retry_eligible_when_policy_allows() {
        let report = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy_fixture());
        assert_eq!(report.retry_eligibility, ProviderRetryEligibility::Eligible);
        assert_eq!(
            report.reason,
            ProviderRetryDecisionReason::TimeoutRetryAllowed
        );
    }
    #[test]
    fn timeout_is_not_retry_eligible_when_policy_disallows() {
        let policy = ProviderRetryPolicy::new("policy", 3, false, true).unwrap();
        let report = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy);
        assert_eq!(
            report.retry_eligibility,
            ProviderRetryEligibility::Ineligible
        );
        assert_eq!(
            report.reason,
            ProviderRetryDecisionReason::TimeoutNotRetryable
        );
    }
    #[test]
    fn provider_unavailable_is_retry_eligible_when_policy_allows() {
        let report = classify_provider_failure(
            ProviderFailureKind::ProviderUnavailable,
            1,
            &policy_fixture(),
        );
        assert_eq!(report.retry_eligibility, ProviderRetryEligibility::Eligible);
        assert_eq!(
            report.reason,
            ProviderRetryDecisionReason::ProviderUnavailableRetryAllowed
        );
    }
    #[test]
    fn provider_unavailable_is_not_retry_eligible_when_policy_disallows() {
        let policy = ProviderRetryPolicy::new("policy", 3, true, false).unwrap();
        let report =
            classify_provider_failure(ProviderFailureKind::ProviderUnavailable, 1, &policy);
        assert_eq!(
            report.retry_eligibility,
            ProviderRetryEligibility::Ineligible
        );
        assert_eq!(
            report.reason,
            ProviderRetryDecisionReason::ProviderUnavailableNotRetryable
        );
    }
    #[test]
    fn cancelled_is_not_retryable() {
        let r = classify_provider_failure(ProviderFailureKind::Cancelled, 1, &policy_fixture());
        assert_eq!(r.reason, ProviderRetryDecisionReason::CancelledNotRetryable);
    }
    #[test]
    fn malformed_output_is_not_retryable() {
        let r =
            classify_provider_failure(ProviderFailureKind::MalformedOutput, 1, &policy_fixture());
        assert_eq!(
            r.reason,
            ProviderRetryDecisionReason::MalformedOutputNotRetryable
        );
    }
    #[test]
    fn transport_rejected_is_not_retryable() {
        let r =
            classify_provider_failure(ProviderFailureKind::TransportRejected, 1, &policy_fixture());
        assert_eq!(
            r.reason,
            ProviderRetryDecisionReason::TransportRejectedNotRetryable
        );
    }
    #[test]
    fn unknown_failure_is_not_retryable() {
        let r = classify_provider_failure(ProviderFailureKind::Unknown, 1, &policy_fixture());
        assert_eq!(
            r.reason,
            ProviderRetryDecisionReason::UnknownFailureNotRetryable
        );
    }
    #[test]
    fn retry_limit_is_enforced() {
        let r = classify_provider_failure(ProviderFailureKind::Timeout, 3, &policy_fixture());
        assert_eq!(r.reason, ProviderRetryDecisionReason::RetryLimitReached);
    }
    #[test]
    fn no_failure_is_not_retryable() {
        let r = classify_provider_failure(ProviderFailureKind::None, 0, &policy_fixture());
        assert_eq!(r.reason, ProviderRetryDecisionReason::NoFailure);
    }
    #[test]
    fn failure_report_never_mutates_authority() {
        let r = classify_provider_failure(ProviderFailureKind::Unknown, 1, &policy_fixture());
        assert!(!provider_failure_report_mutates_authority(&r));
    }
    #[test]
    fn failure_report_never_schedules_retry() {
        let r = classify_provider_failure(ProviderFailureKind::Unknown, 1, &policy_fixture());
        assert!(!provider_failure_report_schedules_retry(&r));
    }
    #[test]
    fn failure_classification_is_deterministic() {
        let a = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy_fixture());
        let b = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy_fixture());
        assert_eq!(a, b);
    }
    #[test]
    fn failure_classification_does_not_call_provider_adapter() {
        let r = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy_fixture());
        assert!(!r.summary.contains("execute_provider_adapter"));
    }
    #[test]
    fn failure_classification_does_not_call_transport_validation() {
        let r = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy_fixture());
        assert!(!r.summary.contains("validate_provider_transport_envelope"));
    }
    #[test]
    fn failure_classification_does_not_persist() {
        let r = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy_fixture());
        assert!(!r.summary.contains("persist"));
    }
    #[test]
    fn failure_classification_does_not_append_ledger() {
        let r = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy_fixture());
        assert!(!r.summary.contains("ledger"));
    }
    #[test]
    fn failure_classification_does_not_promote() {
        let r = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy_fixture());
        assert!(!r.summary.contains("promote"));
    }
    #[test]
    fn failure_classification_does_not_repair_replay() {
        let r = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy_fixture());
        assert!(!r.summary.contains("replay"));
    }
    #[test]
    fn failure_classification_does_not_require_async_runtime() {
        let r = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy_fixture());
        assert!(!r.summary.contains("async"));
    }
    #[test]
    fn failure_classification_does_not_use_network() {
        let r = classify_provider_failure(ProviderFailureKind::Timeout, 1, &policy_fixture());
        assert!(!r.summary.contains("network"));
    }
    #[test]
    fn dry_run_does_not_classify_provider_failure() {
        let args: Vec<String> = std::env::args().collect();
        assert!(!args.iter().any(|a| a.contains("classify_provider_failure")));
    }
}
