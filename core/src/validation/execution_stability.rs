//! Deterministic execution stability validation for governed workflow outcomes.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStabilityStatus {
    Stable,
    Unstable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStabilityError {
    NonDeterministicOutcomeDetected,
    ReplayMismatchDetected,
    LifecycleVarianceDetected,
    GovernanceVarianceDetected,
    PerformanceBoundExceeded,
    FailureClassificationVariance,
    AuthorityBoundaryChangeDetected,
    MeasurementInstabilityDetected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionStabilityEvidence {
    pub identical_inputs_produce_identical_outcomes: bool,
    pub replay_results_match_recorded_facts: bool,
    pub lifecycle_transitions_consistent: bool,
    pub governance_decisions_consistent: bool,
    pub execution_time_within_bounds: bool,
    pub failure_classification_consistent: bool,
    pub authority_boundaries_unchanged: bool,
    pub measurement_process_stable: bool,
}

pub fn validate_execution_stability(
    evidence: &ExecutionStabilityEvidence,
) -> Result<ExecutionStabilityStatus, ExecutionStabilityError> {
    if !evidence.identical_inputs_produce_identical_outcomes {
        return Err(ExecutionStabilityError::NonDeterministicOutcomeDetected);
    }

    if !evidence.replay_results_match_recorded_facts {
        return Err(ExecutionStabilityError::ReplayMismatchDetected);
    }

    if !evidence.lifecycle_transitions_consistent {
        return Err(ExecutionStabilityError::LifecycleVarianceDetected);
    }

    if !evidence.governance_decisions_consistent {
        return Err(ExecutionStabilityError::GovernanceVarianceDetected);
    }

    if !evidence.execution_time_within_bounds {
        return Err(ExecutionStabilityError::PerformanceBoundExceeded);
    }

    if !evidence.failure_classification_consistent {
        return Err(ExecutionStabilityError::FailureClassificationVariance);
    }

    if !evidence.authority_boundaries_unchanged {
        return Err(ExecutionStabilityError::AuthorityBoundaryChangeDetected);
    }

    if !evidence.measurement_process_stable {
        return Err(ExecutionStabilityError::MeasurementInstabilityDetected);
    }

    Ok(ExecutionStabilityStatus::Stable)
}

#[cfg(test)]
mod tests {
    use super::{
        validate_execution_stability, ExecutionStabilityError, ExecutionStabilityEvidence,
        ExecutionStabilityStatus,
    };

    #[test]
    fn stable_execution_is_accepted() {
        let evidence = ExecutionStabilityEvidence {
            identical_inputs_produce_identical_outcomes: true,
            replay_results_match_recorded_facts: true,
            lifecycle_transitions_consistent: true,
            governance_decisions_consistent: true,
            execution_time_within_bounds: true,
            failure_classification_consistent: true,
            authority_boundaries_unchanged: true,
            measurement_process_stable: true,
        };

        let result = validate_execution_stability(&evidence);

        assert_eq!(result, Ok(ExecutionStabilityStatus::Stable));
    }

    #[test]
    fn non_deterministic_outcome_is_rejected() {
        let evidence = ExecutionStabilityEvidence {
            identical_inputs_produce_identical_outcomes: false,
            replay_results_match_recorded_facts: true,
            lifecycle_transitions_consistent: true,
            governance_decisions_consistent: true,
            execution_time_within_bounds: true,
            failure_classification_consistent: true,
            authority_boundaries_unchanged: true,
            measurement_process_stable: true,
        };

        let result = validate_execution_stability(&evidence);

        assert_eq!(
            result,
            Err(ExecutionStabilityError::NonDeterministicOutcomeDetected)
        );
    }

    #[test]
    fn replay_mismatch_is_rejected() {
        let evidence = ExecutionStabilityEvidence {
            identical_inputs_produce_identical_outcomes: true,
            replay_results_match_recorded_facts: false,
            lifecycle_transitions_consistent: true,
            governance_decisions_consistent: true,
            execution_time_within_bounds: true,
            failure_classification_consistent: true,
            authority_boundaries_unchanged: true,
            measurement_process_stable: true,
        };

        let result = validate_execution_stability(&evidence);

        assert_eq!(result, Err(ExecutionStabilityError::ReplayMismatchDetected));
    }

    #[test]
    fn lifecycle_variance_is_rejected() {
        let evidence = ExecutionStabilityEvidence {
            identical_inputs_produce_identical_outcomes: true,
            replay_results_match_recorded_facts: true,
            lifecycle_transitions_consistent: false,
            governance_decisions_consistent: true,
            execution_time_within_bounds: true,
            failure_classification_consistent: true,
            authority_boundaries_unchanged: true,
            measurement_process_stable: true,
        };

        let result = validate_execution_stability(&evidence);

        assert_eq!(
            result,
            Err(ExecutionStabilityError::LifecycleVarianceDetected)
        );
    }

    #[test]
    fn governance_variance_is_rejected() {
        let evidence = ExecutionStabilityEvidence {
            identical_inputs_produce_identical_outcomes: true,
            replay_results_match_recorded_facts: true,
            lifecycle_transitions_consistent: true,
            governance_decisions_consistent: false,
            execution_time_within_bounds: true,
            failure_classification_consistent: true,
            authority_boundaries_unchanged: true,
            measurement_process_stable: true,
        };

        let result = validate_execution_stability(&evidence);

        assert_eq!(
            result,
            Err(ExecutionStabilityError::GovernanceVarianceDetected)
        );
    }

    #[test]
    fn performance_bound_exceeded_is_rejected() {
        let evidence = ExecutionStabilityEvidence {
            identical_inputs_produce_identical_outcomes: true,
            replay_results_match_recorded_facts: true,
            lifecycle_transitions_consistent: true,
            governance_decisions_consistent: true,
            execution_time_within_bounds: false,
            failure_classification_consistent: true,
            authority_boundaries_unchanged: true,
            measurement_process_stable: true,
        };

        let result = validate_execution_stability(&evidence);

        assert_eq!(
            result,
            Err(ExecutionStabilityError::PerformanceBoundExceeded)
        );
    }

    #[test]
    fn failure_classification_variance_is_rejected() {
        let evidence = ExecutionStabilityEvidence {
            identical_inputs_produce_identical_outcomes: true,
            replay_results_match_recorded_facts: true,
            lifecycle_transitions_consistent: true,
            governance_decisions_consistent: true,
            execution_time_within_bounds: true,
            failure_classification_consistent: false,
            authority_boundaries_unchanged: true,
            measurement_process_stable: true,
        };

        let result = validate_execution_stability(&evidence);

        assert_eq!(
            result,
            Err(ExecutionStabilityError::FailureClassificationVariance)
        );
    }

    #[test]
    fn authority_boundary_change_is_rejected() {
        let evidence = ExecutionStabilityEvidence {
            identical_inputs_produce_identical_outcomes: true,
            replay_results_match_recorded_facts: true,
            lifecycle_transitions_consistent: true,
            governance_decisions_consistent: true,
            execution_time_within_bounds: true,
            failure_classification_consistent: true,
            authority_boundaries_unchanged: false,
            measurement_process_stable: true,
        };

        let result = validate_execution_stability(&evidence);

        assert_eq!(
            result,
            Err(ExecutionStabilityError::AuthorityBoundaryChangeDetected)
        );
    }

    #[test]
    fn measurement_instability_is_rejected() {
        let evidence = ExecutionStabilityEvidence {
            identical_inputs_produce_identical_outcomes: true,
            replay_results_match_recorded_facts: true,
            lifecycle_transitions_consistent: true,
            governance_decisions_consistent: true,
            execution_time_within_bounds: true,
            failure_classification_consistent: true,
            authority_boundaries_unchanged: true,
            measurement_process_stable: false,
        };

        let result = validate_execution_stability(&evidence);

        assert_eq!(
            result,
            Err(ExecutionStabilityError::MeasurementInstabilityDetected)
        );
    }

    #[test]
    fn validation_is_deterministic() {
        let evidence = ExecutionStabilityEvidence {
            identical_inputs_produce_identical_outcomes: true,
            replay_results_match_recorded_facts: true,
            lifecycle_transitions_consistent: true,
            governance_decisions_consistent: true,
            execution_time_within_bounds: true,
            failure_classification_consistent: true,
            authority_boundaries_unchanged: true,
            measurement_process_stable: true,
        };

        let first = validate_execution_stability(&evidence);
        let second = validate_execution_stability(&evidence);

        assert_eq!(first, second);
        assert_eq!(first, Ok(ExecutionStabilityStatus::Stable));
    }
}
