use super::{
    OperatorAuthorizationDecision, OperatorAuthorizationStatus, OperatorIntentAuditEligibility,
    OperatorIntentAuditRecord,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorActionKind {
    RecordExecutionDecision,
    PersistLedgerRecord,
    ExecuteProvider,
    RepairReplay,
    MutateApplicationState,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorActionExecutionStatus {
    Executed,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorActionExecutionReason {
    ExecutionDecisionRecorded,
    EmptyExecutionId,
    UnsupportedActionKind,
    AuthorizationRequired,
    AuditEligibilityRequired,
    AuthorizationAuditMismatch,
    ExecutionCapabilityDisabled,
    PersistenceNotAllowed,
    ProviderExecutionNotAllowed,
    ReplayRepairNotAllowed,
    ApplicationStateMutationNotAllowed,
    UnknownActionRejected,
}

impl OperatorActionExecutionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ExecutionDecisionRecorded => "execution_decision_recorded",
            Self::EmptyExecutionId => "empty_execution_id",
            Self::UnsupportedActionKind => "unsupported_action_kind",
            Self::AuthorizationRequired => "authorization_required",
            Self::AuditEligibilityRequired => "audit_eligibility_required",
            Self::AuthorizationAuditMismatch => "authorization_audit_mismatch",
            Self::ExecutionCapabilityDisabled => "execution_capability_disabled",
            Self::PersistenceNotAllowed => "persistence_not_allowed",
            Self::ProviderExecutionNotAllowed => "provider_execution_not_allowed",
            Self::ReplayRepairNotAllowed => "replay_repair_not_allowed",
            Self::ApplicationStateMutationNotAllowed => "application_state_mutation_not_allowed",
            Self::UnknownActionRejected => "unknown_action_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorActionExecutionRequest {
    pub execution_id: String,
    pub action_kind: OperatorActionKind,
    pub authorization_decision: OperatorAuthorizationDecision,
    pub audit_record: OperatorIntentAuditRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorActionExecutionReport {
    pub status: OperatorActionExecutionStatus,
    pub reason: OperatorActionExecutionReason,
    pub execution_id: String,
    pub action_kind: OperatorActionKind,
    pub submission_id: String,
    pub authorization_id: String,
    pub audit_record_id: String,
    pub operator_id: String,
    pub target_id: String,
    pub executed_real_world_effect: bool,
    pub persisted: bool,
    pub appended_ledger: bool,
    pub appended_audit: bool,
    pub called_provider: bool,
    pub repaired_replay: bool,
    pub mutated_application_state: bool,
    pub summary: String,
}

pub fn execute_operator_action_boundary(
    request: OperatorActionExecutionRequest,
) -> OperatorActionExecutionReport {
    if request.execution_id.is_empty() {
        return rejected(request, OperatorActionExecutionReason::EmptyExecutionId);
    }
    if request.action_kind == OperatorActionKind::Unknown {
        return rejected(
            request,
            OperatorActionExecutionReason::UnknownActionRejected,
        );
    }
    if request.action_kind == OperatorActionKind::PersistLedgerRecord {
        return rejected(
            request,
            OperatorActionExecutionReason::PersistenceNotAllowed,
        );
    }
    if request.action_kind == OperatorActionKind::ExecuteProvider {
        return rejected(
            request,
            OperatorActionExecutionReason::ProviderExecutionNotAllowed,
        );
    }
    if request.action_kind == OperatorActionKind::RepairReplay {
        return rejected(
            request,
            OperatorActionExecutionReason::ReplayRepairNotAllowed,
        );
    }
    if request.action_kind == OperatorActionKind::MutateApplicationState {
        return rejected(
            request,
            OperatorActionExecutionReason::ApplicationStateMutationNotAllowed,
        );
    }
    if request.action_kind != OperatorActionKind::RecordExecutionDecision {
        return rejected(
            request,
            OperatorActionExecutionReason::UnsupportedActionKind,
        );
    }
    if request.authorization_decision.status != OperatorAuthorizationStatus::Authorized {
        return rejected(
            request,
            OperatorActionExecutionReason::AuthorizationRequired,
        );
    }
    if request.audit_record.eligibility != OperatorIntentAuditEligibility::Eligible {
        return rejected(
            request,
            OperatorActionExecutionReason::AuditEligibilityRequired,
        );
    }
    if request.authorization_decision.submission_id != request.audit_record.submission_id
        || request.authorization_decision.operator_id != request.audit_record.operator_id
        || request.authorization_decision.target_kind != request.audit_record.target_kind
        || request.authorization_decision.target_id != request.audit_record.target_id
    {
        return rejected(
            request,
            OperatorActionExecutionReason::AuthorizationAuditMismatch,
        );
    }
    if request.authorization_decision.execution_enabled {
        return rejected(
            request,
            OperatorActionExecutionReason::ExecutionCapabilityDisabled,
        );
    }

    OperatorActionExecutionReport {
        status: OperatorActionExecutionStatus::Executed,
        reason: OperatorActionExecutionReason::ExecutionDecisionRecorded,
        execution_id: request.execution_id,
        action_kind: request.action_kind,
        submission_id: request.authorization_decision.submission_id,
        authorization_id: request.authorization_decision.authorization_id,
        audit_record_id: request.audit_record.audit_record_id,
        operator_id: request.authorization_decision.operator_id,
        target_id: request.authorization_decision.target_id,
        executed_real_world_effect: false,
        persisted: false,
        appended_ledger: false,
        appended_audit: false,
        called_provider: false,
        repaired_replay: false,
        mutated_application_state: false,
        summary: "Execution boundary accepted an authorized + audit-eligible proof set and recorded an in-memory execution decision report only; no real-world effect was executed.".to_string(),
    }
}

fn rejected(
    request: OperatorActionExecutionRequest,
    reason: OperatorActionExecutionReason,
) -> OperatorActionExecutionReport {
    OperatorActionExecutionReport {
        status: OperatorActionExecutionStatus::Rejected,
        reason,
        execution_id: request.execution_id,
        action_kind: request.action_kind,
        submission_id: request.authorization_decision.submission_id,
        authorization_id: request.authorization_decision.authorization_id,
        audit_record_id: request.audit_record.audit_record_id,
        operator_id: request.authorization_decision.operator_id,
        target_id: request.authorization_decision.target_id,
        executed_real_world_effect: false,
        persisted: false,
        appended_ledger: false,
        appended_audit: false,
        called_provider: false,
        repaired_replay: false,
        mutated_application_state: false,
        summary: "Operator action execution was rejected by the Rust-owned boundary; no real-world effect was executed.".to_string(),
    }
}

pub fn operator_action_report_has_real_world_effect(
    report: &OperatorActionExecutionReport,
) -> bool {
    report.executed_real_world_effect
}

pub fn operator_action_report_mutates_authority(report: &OperatorActionExecutionReport) -> bool {
    report.persisted
        || report.appended_ledger
        || report.appended_audit
        || report.called_provider
        || report.repaired_replay
        || report.mutated_application_state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        authorize_operator_intent, submit_operator_intent, OperatorAuthorizationReason,
        OperatorAuthorizationRequest, OperatorIdentity, OperatorIntent, OperatorIntentAuditReason,
        OperatorIntentAuditRecord, OperatorIntentTargetKind, OperatorIntentType,
        OperatorSafetyContext, OperatorTargetContext,
    };

    fn fixture_execution_request() -> OperatorActionExecutionRequest {
        let submission = crate::api::OperatorIntentSubmission::new(
            "sub-1",
            "op-1",
            OperatorIntent::new(OperatorIntentType::Approve, "ok"),
            OperatorIntentTargetKind::Run,
            "run-1",
        )
        .expect("submission");
        let ingress = submit_operator_intent(submission.clone());
        let auth = authorize_operator_intent(
            OperatorAuthorizationRequest::new(
                "auth-1",
                submission.clone(),
                ingress.clone(),
                OperatorIdentity::new("op-1", "s", "claim").expect("identity"),
                OperatorSafetyContext::new("cfg", "strict", false, false, false).expect("safety"),
                OperatorTargetContext::new(OperatorIntentTargetKind::Run, "run-1", "run-1")
                    .expect("target"),
            )
            .expect("request"),
        );
        let audit = OperatorIntentAuditRecord::new("audit-1", &submission, &ingress, &auth);

        OperatorActionExecutionRequest {
            execution_id: "exec-1".to_string(),
            action_kind: OperatorActionKind::RecordExecutionDecision,
            authorization_decision: auth,
            audit_record: audit,
        }
    }

    #[test]
    fn operator_action_reason_codes_are_stable() {
        assert_eq!(
            OperatorActionExecutionReason::ExecutionDecisionRecorded.code(),
            "execution_decision_recorded"
        );
    }
    #[test]
    fn action_execution_requires_execution_id() {
        let mut request = fixture_execution_request();
        request.execution_id.clear();
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::EmptyExecutionId
        );
    }
    #[test]
    fn action_execution_rejects_unknown_action() {
        let mut request = fixture_execution_request();
        request.action_kind = OperatorActionKind::Unknown;
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::UnknownActionRejected
        );
    }
    #[test]
    fn action_execution_rejects_persistence_action() {
        let mut request = fixture_execution_request();
        request.action_kind = OperatorActionKind::PersistLedgerRecord;
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::PersistenceNotAllowed
        );
    }
    #[test]
    fn action_execution_rejects_provider_action() {
        let mut request = fixture_execution_request();
        request.action_kind = OperatorActionKind::ExecuteProvider;
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::ProviderExecutionNotAllowed
        );
    }
    #[test]
    fn action_execution_rejects_replay_repair_action() {
        let mut request = fixture_execution_request();
        request.action_kind = OperatorActionKind::RepairReplay;
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::ReplayRepairNotAllowed
        );
    }
    #[test]
    fn action_execution_rejects_application_state_mutation_action() {
        let mut request = fixture_execution_request();
        request.action_kind = OperatorActionKind::MutateApplicationState;
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::ApplicationStateMutationNotAllowed
        );
    }
    #[test]
    fn action_execution_requires_authorized_decision() {
        let mut request = fixture_execution_request();
        request.authorization_decision.status = OperatorAuthorizationStatus::Denied;
        request.authorization_decision.reason = OperatorAuthorizationReason::IngressNotAccepted;
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::AuthorizationRequired
        );
    }
    #[test]
    fn action_execution_requires_audit_eligible_record() {
        let mut request = fixture_execution_request();
        request.audit_record.eligibility = OperatorIntentAuditEligibility::Ineligible;
        request.audit_record.reason = OperatorIntentAuditReason::AuthorizationNotSuccessful;
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::AuditEligibilityRequired
        );
    }
    #[test]
    fn action_execution_rejects_authorization_audit_submission_mismatch() {
        let mut request = fixture_execution_request();
        request.audit_record.submission_id = "sub-x".into();
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::AuthorizationAuditMismatch
        );
    }
    #[test]
    fn action_execution_rejects_authorization_audit_operator_mismatch() {
        let mut request = fixture_execution_request();
        request.audit_record.operator_id = "op-x".into();
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::AuthorizationAuditMismatch
        );
    }
    #[test]
    fn action_execution_rejects_authorization_audit_target_mismatch() {
        let mut request = fixture_execution_request();
        request.audit_record.target_id = "target-x".into();
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::AuthorizationAuditMismatch
        );
    }
    #[test]
    fn record_execution_decision_accepts_matching_authorized_audit_proof() {
        let report = execute_operator_action_boundary(fixture_execution_request());
        assert_eq!(report.status, OperatorActionExecutionStatus::Executed);
        assert_eq!(
            report.reason,
            OperatorActionExecutionReason::ExecutionDecisionRecorded
        );
    }
    #[test]
    fn record_execution_decision_has_no_real_world_effect() {
        let report = execute_operator_action_boundary(fixture_execution_request());
        assert!(!operator_action_report_has_real_world_effect(&report));
    }
    #[test]
    fn record_execution_decision_does_not_persist() {
        let report = execute_operator_action_boundary(fixture_execution_request());
        assert!(!report.persisted);
    }
    #[test]
    fn record_execution_decision_does_not_append_ledger() {
        let report = execute_operator_action_boundary(fixture_execution_request());
        assert!(!report.appended_ledger);
    }
    #[test]
    fn record_execution_decision_does_not_append_audit() {
        let report = execute_operator_action_boundary(fixture_execution_request());
        assert!(!report.appended_audit);
    }
    #[test]
    fn record_execution_decision_does_not_call_provider() {
        let report = execute_operator_action_boundary(fixture_execution_request());
        assert!(!report.called_provider);
    }
    #[test]
    fn record_execution_decision_does_not_repair_replay() {
        let report = execute_operator_action_boundary(fixture_execution_request());
        assert!(!report.repaired_replay);
    }
    #[test]
    fn record_execution_decision_does_not_mutate_application_state() {
        let report = execute_operator_action_boundary(fixture_execution_request());
        assert!(!report.mutated_application_state);
        assert!(!operator_action_report_mutates_authority(&report));
    }
    #[test]
    fn risky_reason_text_cannot_enable_unsupported_action() {
        let mut request = fixture_execution_request();
        request.authorization_decision.summary = "force persist or execute provider".to_string();
        request.action_kind = OperatorActionKind::ExecuteProvider;
        assert_eq!(
            execute_operator_action_boundary(request).reason,
            OperatorActionExecutionReason::ProviderExecutionNotAllowed
        );
    }
}
