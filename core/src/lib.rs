pub mod api;
pub mod audit;
pub mod context;
pub mod errors;
pub mod execution;
pub mod ledger;
pub mod memory;
pub mod policy;
pub mod replay;
pub mod state;
pub mod validation;

#[cfg(test)]
mod tests {
    use crate::policy::{PolicyDecision, PolicyResult};
    use crate::replay::{ReplayIntegrity, ReplayReadiness, ReplayReport, ReplayStatus};
    use crate::state::{HarnessState, LifecycleState};
    use crate::validation::{ValidationResult, ValidationStatus};

    #[test]
    fn harness_state_genesis_is_created_at_revision_zero() {
        let state = HarnessState::genesis();
        assert_eq!(state.revision, 0);
        assert_eq!(state.lifecycle, LifecycleState::Created);
    }

    #[test]
    fn policy_result_unknown_is_not_pass() {
        let result = PolicyResult::unknown();
        assert_eq!(result.decision, PolicyDecision::Unknown);
        assert_eq!(result.reason, "unknown_is_not_pass");
    }

    #[test]
    fn validation_result_unknown_is_not_pass() {
        let result = ValidationResult::unknown();
        assert_eq!(result.status, ValidationStatus::Unknown);
        assert_eq!(result.message, "unknown_is_not_pass");
    }

    #[test]
    fn replay_report_unknown_has_zero_events() {
        let report = ReplayReport::unknown();
        assert_eq!(report.status, ReplayStatus::Unknown);
        assert_eq!(report.integrity, ReplayIntegrity::Unknown);
        assert_eq!(report.readiness, ReplayReadiness::NotReady);
        assert_eq!(report.events_replayed, 0);
        assert_eq!(report.reason, "unknown_is_not_pass");
    }
}
