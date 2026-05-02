#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionPlan {
    pub id: String,
    pub status: ExecutionStatus,
}

impl ExecutionPlan {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: ExecutionStatus::NotStarted,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionDecision {
    Allowed,
    Blocked,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionDecisionReason {
    ReadyForExecution,
    LifecycleNotPassed,
    PolicyNotAllowed,
    ValidationNotPassed,
    ReplayNotReady,
}

impl ExecutionDecisionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ReadyForExecution => "ready_for_execution",
            Self::LifecycleNotPassed => "lifecycle_not_passed",
            Self::PolicyNotAllowed => "policy_not_allowed",
            Self::ValidationNotPassed => "validation_not_passed",
            Self::ReplayNotReady => "replay_not_ready",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionDecisionReport {
    pub decision: ExecutionDecision,
    pub reason: ExecutionDecisionReason,
}

impl ExecutionDecisionReport {
    pub fn allowed() -> Self {
        Self {
            decision: ExecutionDecision::Allowed,
            reason: ExecutionDecisionReason::ReadyForExecution,
        }
    }

    pub fn blocked(reason: ExecutionDecisionReason) -> Self {
        Self {
            decision: ExecutionDecision::Blocked,
            reason,
        }
    }

    pub fn rejected(reason: ExecutionDecisionReason) -> Self {
        Self {
            decision: ExecutionDecision::Rejected,
            reason,
        }
    }
}

pub fn decide_execution(
    lifecycle: crate::state::LifecycleState,
    policy: &crate::policy::PolicyResult,
    validation: &crate::validation::ValidationResult,
    replay: &crate::replay::ReplayReport,
) -> ExecutionDecisionReport {
    if lifecycle != crate::state::LifecycleState::Passed {
        return ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed);
    }

    if policy.decision != crate::policy::PolicyDecision::Allowed {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed);
    }

    if validation.status != crate::validation::ValidationStatus::Pass {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed);
    }

    if replay.readiness != crate::replay::ReplayReadiness::Ready {
        return ExecutionDecisionReport::blocked(ExecutionDecisionReason::ReplayNotReady);
    }

    ExecutionDecisionReport::allowed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{PolicyDecision, PolicyResult};
    use crate::replay::{ReplayIntegrity, ReplayReadiness, ReplayReport, ReplayStatus};
    use crate::state::LifecycleState;
    use crate::validation::{ValidationResult, ValidationStatus};

    fn ready_policy() -> PolicyResult {
        PolicyResult::allowed("required_policy_evidence_present")
    }

    fn ready_validation() -> ValidationResult {
        ValidationResult::pass("validation_evidence_passed")
    }

    fn ready_replay() -> ReplayReport {
        ReplayReport::replayable(2)
    }

    #[test]
    fn execution_decision_reason_codes_are_stable() {
        assert_eq!(
            ExecutionDecisionReason::ReadyForExecution.code(),
            "ready_for_execution"
        );
        assert_eq!(
            ExecutionDecisionReason::LifecycleNotPassed.code(),
            "lifecycle_not_passed"
        );
        assert_eq!(
            ExecutionDecisionReason::PolicyNotAllowed.code(),
            "policy_not_allowed"
        );
        assert_eq!(
            ExecutionDecisionReason::ValidationNotPassed.code(),
            "validation_not_passed"
        );
        assert_eq!(
            ExecutionDecisionReason::ReplayNotReady.code(),
            "replay_not_ready"
        );
    }

    #[test]
    fn execution_allows_when_all_inputs_are_ready() {
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(report, ExecutionDecisionReport::allowed());
    }

    #[test]
    fn execution_rejects_when_lifecycle_not_passed() {
        let report = decide_execution(
            LifecycleState::Created,
            &ready_policy(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn execution_blocks_when_policy_not_allowed() {
        let report = decide_execution(
            LifecycleState::Passed,
            &PolicyResult::unknown(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed)
        );
    }

    #[test]
    fn execution_blocks_when_validation_not_passed() {
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ValidationResult::unknown(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed)
        );
    }

    #[test]
    fn execution_blocks_when_replay_not_ready() {
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ready_validation(),
            &ReplayReport::unknown(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ReplayNotReady)
        );
    }

    #[test]
    fn execution_priority_lifecycle_before_policy() {
        let report = decide_execution(
            LifecycleState::Created,
            &PolicyResult::unknown(),
            &ready_validation(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::rejected(ExecutionDecisionReason::LifecycleNotPassed)
        );
    }

    #[test]
    fn execution_priority_policy_before_validation() {
        let report = decide_execution(
            LifecycleState::Passed,
            &PolicyResult::unknown(),
            &ValidationResult::unknown(),
            &ready_replay(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::PolicyNotAllowed)
        );
    }

    #[test]
    fn execution_priority_validation_before_replay() {
        let report = decide_execution(
            LifecycleState::Passed,
            &ready_policy(),
            &ValidationResult::unknown(),
            &ReplayReport::unknown(),
        );

        assert_eq!(
            report,
            ExecutionDecisionReport::blocked(ExecutionDecisionReason::ValidationNotPassed)
        );
    }

    #[test]
    fn execution_does_not_require_reason_strings_for_decision() {
        let policy_a = PolicyResult {
            decision: PolicyDecision::Allowed,
            reason: "policy_reason_a",
        };
        let policy_b = PolicyResult {
            decision: PolicyDecision::Allowed,
            reason: "policy_reason_b",
        };

        let validation_a = ValidationResult {
            status: ValidationStatus::Pass,
            message: "validation_message_a",
        };
        let validation_b = ValidationResult {
            status: ValidationStatus::Pass,
            message: "validation_message_b",
        };

        let replay_a = ReplayReport {
            status: ReplayStatus::Replayable,
            integrity: ReplayIntegrity::Valid,
            readiness: ReplayReadiness::Ready,
            events_replayed: 1,
            reason: "replay_reason_a",
        };
        let replay_b = ReplayReport {
            status: ReplayStatus::Replayable,
            integrity: ReplayIntegrity::Valid,
            readiness: ReplayReadiness::Ready,
            events_replayed: 99,
            reason: "replay_reason_b",
        };

        let report_a =
            decide_execution(LifecycleState::Passed, &policy_a, &validation_a, &replay_a);
        let report_b =
            decide_execution(LifecycleState::Passed, &policy_b, &validation_b, &replay_b);

        assert_eq!(report_a, ExecutionDecisionReport::allowed());
        assert_eq!(report_b, ExecutionDecisionReport::allowed());
        assert_eq!(report_a, report_b);
    }
}
