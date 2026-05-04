use super::{
    ApplicationStateError, IntegrationBoundaryError, LocalHarnessWorkflowError,
    LocalHarnessWorkflowReason, LocalPersistenceError, LocalPersistenceValidationReason,
    LocalRuntimeConfigError, OperatorAuthorizationReason, OperatorIntentIngressReason,
    PersistedRecordEnvelopeError, ReadProjectionError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticFamily {
    OperatorIntent,
    OperatorAuthorization,
    Integration,
    RuntimeConfig,
    ReadProjection,
    ApplicationState,
    PersistenceValidation,
    PersistenceExecution,
    PersistenceRecovery,
    LocalWorkflow,
    ProviderBoundary,
    ProviderAdapter,
    LocalProviderConfig,
    ControlledRun,
    Promotion,
    PromotionReplay,
    ExecutionDecision,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorDiagnostic {
    pub family: DiagnosticFamily,
    pub code: &'static str,
    pub summary: &'static str,
}

impl ErrorDiagnostic {
    pub fn new(family: DiagnosticFamily, code: &'static str, summary: &'static str) -> Self {
        Self {
            family,
            code,
            summary,
        }
    }
}

pub fn diagnostic_family_label(family: DiagnosticFamily) -> &'static str {
    match family {
        DiagnosticFamily::OperatorIntent => "operator_intent",
        DiagnosticFamily::OperatorAuthorization => "operator_authorization",
        DiagnosticFamily::Integration => "integration",
        DiagnosticFamily::RuntimeConfig => "runtime_config",
        DiagnosticFamily::ReadProjection => "read_projection",
        DiagnosticFamily::ApplicationState => "application_state",
        DiagnosticFamily::PersistenceValidation => "persistence_validation",
        DiagnosticFamily::PersistenceExecution => "persistence_execution",
        DiagnosticFamily::PersistenceRecovery => "persistence_recovery",
        DiagnosticFamily::LocalWorkflow => "local_workflow",
        DiagnosticFamily::ProviderBoundary => "provider_boundary",
        DiagnosticFamily::ProviderAdapter => "provider_adapter",
        DiagnosticFamily::LocalProviderConfig => "local_provider_config",
        DiagnosticFamily::ControlledRun => "controlled_run",
        DiagnosticFamily::Promotion => "promotion",
        DiagnosticFamily::PromotionReplay => "promotion_replay",
        DiagnosticFamily::ExecutionDecision => "execution_decision",
        DiagnosticFamily::Unknown => "unknown",
    }
}

pub fn diagnostic_key(diagnostic: &ErrorDiagnostic) -> String {
    format!(
        "{}.{}",
        diagnostic_family_label(diagnostic.family),
        diagnostic.code
    )
}

pub fn operator_intent_ingress_reason_diagnostic(
    reason: OperatorIntentIngressReason,
) -> ErrorDiagnostic {
    ErrorDiagnostic::new(
        DiagnosticFamily::OperatorIntent,
        reason.code(),
        "Operator intent ingress was rejected.",
    )
}

pub fn operator_authorization_reason_diagnostic(
    reason: OperatorAuthorizationReason,
) -> ErrorDiagnostic {
    ErrorDiagnostic::new(
        DiagnosticFamily::OperatorAuthorization,
        reason.code(),
        "Operator intent authorization was denied.",
    )
}
pub fn integration_boundary_error_diagnostic(error: IntegrationBoundaryError) -> ErrorDiagnostic {
    ErrorDiagnostic::new(
        DiagnosticFamily::Integration,
        error.code(),
        "Integration boundary validation failed.",
    )
}

pub fn local_runtime_config_error_diagnostic(error: LocalRuntimeConfigError) -> ErrorDiagnostic {
    ErrorDiagnostic::new(
        DiagnosticFamily::RuntimeConfig,
        error.code(),
        "Runtime configuration is invalid.",
    )
}

pub fn read_projection_error_diagnostic(error: ReadProjectionError) -> ErrorDiagnostic {
    ErrorDiagnostic::new(
        DiagnosticFamily::ReadProjection,
        error.code(),
        "Read projection cannot be constructed.",
    )
}

pub fn application_state_error_diagnostic(error: ApplicationStateError) -> ErrorDiagnostic {
    ErrorDiagnostic::new(
        DiagnosticFamily::ApplicationState,
        error.code(),
        "Application state validation failed.",
    )
}

pub fn local_persistence_validation_reason_diagnostic(
    reason: LocalPersistenceValidationReason,
) -> ErrorDiagnostic {
    ErrorDiagnostic::new(
        DiagnosticFamily::PersistenceValidation,
        reason.code(),
        "Persistence validation rejected the request.",
    )
}

pub fn local_persistence_error_diagnostic(error: LocalPersistenceError) -> ErrorDiagnostic {
    ErrorDiagnostic::new(
        DiagnosticFamily::PersistenceExecution,
        error.code(),
        "Persistence operation failed.",
    )
}

pub fn persisted_record_envelope_error_diagnostic(
    error: PersistedRecordEnvelopeError,
) -> ErrorDiagnostic {
    ErrorDiagnostic::new(
        DiagnosticFamily::PersistenceRecovery,
        error.code(),
        "Persisted record envelope verification failed.",
    )
}

pub fn local_harness_workflow_error_diagnostic(
    error: LocalHarnessWorkflowError,
) -> ErrorDiagnostic {
    ErrorDiagnostic::new(
        DiagnosticFamily::LocalWorkflow,
        error.code(),
        "Local workflow request is invalid.",
    )
}

pub fn local_harness_workflow_reason_diagnostic(
    reason: LocalHarnessWorkflowReason,
) -> ErrorDiagnostic {
    ErrorDiagnostic::new(
        DiagnosticFamily::LocalWorkflow,
        reason.code(),
        "Local workflow did not complete successfully.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_family_labels_are_stable() {
        assert_eq!(
            diagnostic_family_label(DiagnosticFamily::OperatorIntent),
            "operator_intent"
        );
        assert_eq!(
            diagnostic_family_label(DiagnosticFamily::PersistenceExecution),
            "persistence_execution"
        );
        assert_eq!(
            diagnostic_family_label(DiagnosticFamily::Unknown),
            "unknown"
        );
    }

    #[test]
    fn diagnostic_key_combines_family_and_code_without_changing_code() {
        let d = ErrorDiagnostic::new(
            DiagnosticFamily::PersistenceExecution,
            "empty_payload",
            "Payload is empty.",
        );
        assert_eq!(d.code, "empty_payload");
        assert_eq!(diagnostic_key(&d), "persistence_execution.empty_payload");
    }

    #[test]
    fn duplicate_code_strings_are_distinguishable_by_family() {
        let integration =
            ErrorDiagnostic::new(DiagnosticFamily::Integration, "empty_request_id", "x");
        let state =
            ErrorDiagnostic::new(DiagnosticFamily::ApplicationState, "empty_request_id", "x");
        assert_ne!(diagnostic_key(&integration), diagnostic_key(&state));
    }

    #[test]
    fn diagnostics_do_not_require_global_code_uniqueness() {
        let first = ErrorDiagnostic::new(DiagnosticFamily::OperatorIntent, "route_rejected", "x");
        let second = ErrorDiagnostic::new(DiagnosticFamily::LocalWorkflow, "route_rejected", "x");
        assert_eq!(first.code, second.code);
        assert_ne!(diagnostic_key(&first), diagnostic_key(&second));
    }
}
