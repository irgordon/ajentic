pub mod api;
pub mod audit;
pub mod authority;
pub mod context;
pub mod errors;
pub mod execution;
pub mod integrity;
pub mod ledger;
pub mod memory;
pub mod metrics;
pub mod outcome;
pub mod policy;
pub mod replay;
pub mod state;
pub mod task;
pub mod validation;

#[cfg(test)]
mod tests {
    use crate::replay::{ReplayIntegrity, ReplayReadiness, ReplayReport, ReplayStatus};
    use crate::state::{HarnessState, LifecycleState};

    #[test]
    fn harness_state_genesis_is_created_at_revision_zero() {
        let state = HarnessState::genesis();
        assert_eq!(state.revision, 0);
        assert_eq!(state.lifecycle, LifecycleState::Created);
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
