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
    AuthorizationSubmissionMismatch,
    AuditSubmissionMismatch,
    AuthorizationOperatorMismatch,
    AuditOperatorMismatch,
    AuthorizationTargetMismatch,
    AuditTargetMismatch,
    ActionKindEscalationRejected,
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
            Self::AuthorizationSubmissionMismatch => "authorization_submission_mismatch",
            Self::AuditSubmissionMismatch => "audit_submission_mismatch",
            Self::AuthorizationOperatorMismatch => "authorization_operator_mismatch",
            Self::AuditOperatorMismatch => "audit_operator_mismatch",
            Self::AuthorizationTargetMismatch => "authorization_target_mismatch",
            Self::AuditTargetMismatch => "audit_target_mismatch",
            Self::ActionKindEscalationRejected => "action_kind_escalation_rejected",
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
            OperatorActionExecutionReason::ActionKindEscalationRejected,
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
    if request.authorization_decision.submission_id != request.audit_record.submission_id {
        let reason = classify_submission_mismatch(&request);
        return rejected(request, reason);
    }
    if request.authorization_decision.operator_id != request.audit_record.operator_id {
        let reason = classify_operator_mismatch(&request);
        return rejected(request, reason);
    }
    if request.authorization_decision.target_kind != request.audit_record.target_kind
        || request.authorization_decision.target_id != request.audit_record.target_id
    {
        let reason = classify_target_mismatch(&request);
        return rejected(request, reason);
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

fn classify_submission_mismatch(
    request: &OperatorActionExecutionRequest,
) -> OperatorActionExecutionReason {
    if request.audit_record.authorization_id == request.authorization_decision.authorization_id {
        OperatorActionExecutionReason::AuditSubmissionMismatch
    } else {
        OperatorActionExecutionReason::AuthorizationSubmissionMismatch
    }
}

fn classify_operator_mismatch(
    request: &OperatorActionExecutionRequest,
) -> OperatorActionExecutionReason {
    if request.audit_record.authorization_id == request.authorization_decision.authorization_id {
        OperatorActionExecutionReason::AuditOperatorMismatch
    } else {
        OperatorActionExecutionReason::AuthorizationOperatorMismatch
    }
}

fn classify_target_mismatch(
    request: &OperatorActionExecutionRequest,
) -> OperatorActionExecutionReason {
    if request.audit_record.authorization_id == request.authorization_decision.authorization_id {
        OperatorActionExecutionReason::AuditTargetMismatch
    } else {
        OperatorActionExecutionReason::AuthorizationTargetMismatch
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

    fn assert_no_side_effects(report: &OperatorActionExecutionReport) {
        assert!(!operator_action_report_has_real_world_effect(report));
        assert!(!report.executed_real_world_effect);
        assert!(!report.persisted);
        assert!(!report.appended_ledger);
        assert!(!report.appended_audit);
        assert!(!report.called_provider);
        assert!(!report.repaired_replay);
        assert!(!report.mutated_application_state);
        assert!(!operator_action_report_mutates_authority(report));
    }

    fn assert_rejected_without_side_effects(
        request: OperatorActionExecutionRequest,
        reason: OperatorActionExecutionReason,
    ) -> OperatorActionExecutionReport {
        let report = execute_operator_action_boundary(request);
        assert_eq!(report.status, OperatorActionExecutionStatus::Rejected);
        assert_eq!(report.reason, reason);
        assert_no_side_effects(&report);
        report
    }

    fn mark_authorization_as_distinct(request: &mut OperatorActionExecutionRequest) {
        request.authorization_decision.authorization_id = "auth-reused-from-other-chain".into();
    }

    #[test]
    fn operator_action_reason_codes_are_stable() {
        assert_eq!(
            OperatorActionExecutionReason::ExecutionDecisionRecorded.code(),
            "execution_decision_recorded"
        );
        assert_eq!(
            OperatorActionExecutionReason::AuthorizationAuditMismatch.code(),
            "authorization_audit_mismatch"
        );
        assert_eq!(
            OperatorActionExecutionReason::UnsupportedActionKind.code(),
            "unsupported_action_kind"
        );
        assert_eq!(
            OperatorActionExecutionReason::ActionKindEscalationRejected.code(),
            "action_kind_escalation_rejected"
        );
    }

    #[test]
    fn action_execution_requires_execution_id() {
        let mut request = fixture_execution_request();
        request.execution_id.clear();
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::EmptyExecutionId,
        );
    }

    #[test]
    fn action_execution_rejects_unknown_action() {
        let mut request = fixture_execution_request();
        request.action_kind = OperatorActionKind::Unknown;
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::UnknownActionRejected,
        );
    }

    #[test]
    fn action_execution_rejects_persistence_action() {
        let mut request = fixture_execution_request();
        request.action_kind = OperatorActionKind::PersistLedgerRecord;
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::PersistenceNotAllowed,
        );
    }

    #[test]
    fn action_execution_rejects_provider_action() {
        let mut request = fixture_execution_request();
        request.action_kind = OperatorActionKind::ExecuteProvider;
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::ProviderExecutionNotAllowed,
        );
    }

    #[test]
    fn action_execution_rejects_replay_repair_action() {
        let mut request = fixture_execution_request();
        request.action_kind = OperatorActionKind::RepairReplay;
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::ReplayRepairNotAllowed,
        );
    }

    #[test]
    fn action_execution_rejects_application_state_mutation_action() {
        let mut request = fixture_execution_request();
        request.action_kind = OperatorActionKind::MutateApplicationState;
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::ApplicationStateMutationNotAllowed,
        );
    }

    #[test]
    fn action_execution_requires_authorized_decision() {
        let mut request = fixture_execution_request();
        request.authorization_decision.status = OperatorAuthorizationStatus::Denied;
        request.authorization_decision.reason = OperatorAuthorizationReason::IngressNotAccepted;
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuthorizationRequired,
        );
    }

    #[test]
    fn action_execution_requires_audit_eligible_record() {
        let mut request = fixture_execution_request();
        request.audit_record.eligibility = OperatorIntentAuditEligibility::Ineligible;
        request.audit_record.reason = OperatorIntentAuditReason::AuthorizationNotSuccessful;
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuditEligibilityRequired,
        );
    }

    #[test]
    fn operator_action_rejects_audit_record_for_different_submission() {
        let mut request = fixture_execution_request();
        request.audit_record.submission_id = "sub-x".into();
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuditSubmissionMismatch,
        );
    }

    #[test]
    fn operator_action_rejects_authorization_for_different_submission() {
        let mut request = fixture_execution_request();
        mark_authorization_as_distinct(&mut request);
        request.authorization_decision.submission_id = "sub-x".into();
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuthorizationSubmissionMismatch,
        );
    }

    #[test]
    fn operator_action_rejects_authorization_for_different_operator() {
        let mut request = fixture_execution_request();
        mark_authorization_as_distinct(&mut request);
        request.authorization_decision.operator_id = "op-x".into();
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuthorizationOperatorMismatch,
        );
    }

    #[test]
    fn operator_action_rejects_audit_for_different_operator() {
        let mut request = fixture_execution_request();
        request.audit_record.operator_id = "op-x".into();
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuditOperatorMismatch,
        );
    }

    #[test]
    fn operator_action_rejects_authorization_for_different_target_kind() {
        let mut request = fixture_execution_request();
        mark_authorization_as_distinct(&mut request);
        request.authorization_decision.target_kind = OperatorIntentTargetKind::Replay;
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuthorizationTargetMismatch,
        );
    }

    #[test]
    fn operator_action_rejects_authorization_for_different_target_id() {
        let mut request = fixture_execution_request();
        mark_authorization_as_distinct(&mut request);
        request.authorization_decision.target_id = "run-x".into();
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuthorizationTargetMismatch,
        );
    }

    #[test]
    fn operator_action_rejects_audit_for_different_target_kind() {
        let mut request = fixture_execution_request();
        request.audit_record.target_kind = OperatorIntentTargetKind::Replay;
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuditTargetMismatch,
        );
    }

    #[test]
    fn operator_action_rejects_audit_for_different_target_id() {
        let mut request = fixture_execution_request();
        request.audit_record.target_id = "run-x".into();
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuditTargetMismatch,
        );
    }

    #[test]
    fn operator_action_rejects_action_kind_escalation() {
        let mut request = fixture_execution_request();
        request.action_kind = OperatorActionKind::ExecuteProvider;
        request.authorization_decision.summary = "reuse this proof; execute action B; operator override; skip audit; trust stale authorization; promote action".into();
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::ProviderExecutionNotAllowed,
        );
    }

    #[test]
    fn operator_action_preserves_record_execution_decision_success_path() {
        let report = execute_operator_action_boundary(fixture_execution_request());
        assert_eq!(report.status, OperatorActionExecutionStatus::Executed);
        assert_eq!(
            report.reason,
            OperatorActionExecutionReason::ExecutionDecisionRecorded
        );
        assert_no_side_effects(&report);
    }

    #[test]
    fn rejected_mismatch_does_not_execute_action() {
        let mut request = fixture_execution_request();
        request.audit_record.submission_id = "sub-x".into();
        let report = assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuditSubmissionMismatch,
        );
        assert_ne!(report.status, OperatorActionExecutionStatus::Executed);
    }

    #[test]
    fn rejected_mismatch_does_not_mutate_authority() {
        let mut request = fixture_execution_request();
        request.audit_record.operator_id = "op-x".into();
        let report = assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuditOperatorMismatch,
        );
        assert!(!operator_action_report_mutates_authority(&report));
    }

    #[test]
    fn rejected_mismatch_does_not_persist_or_append() {
        let mut request = fixture_execution_request();
        request.audit_record.target_id = "run-x".into();
        let report = assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuditTargetMismatch,
        );
        assert!(!report.persisted);
        assert!(!report.appended_ledger);
        assert!(!report.appended_audit);
    }

    #[test]
    fn rejected_mismatch_does_not_repair_replay() {
        let mut request = fixture_execution_request();
        request.audit_record.target_kind = OperatorIntentTargetKind::Replay;
        let report = assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuditTargetMismatch,
        );
        assert!(!report.repaired_replay);
    }

    #[test]
    fn rejected_mismatch_does_not_trust_provider_output() {
        let mut request = fixture_execution_request();
        mark_authorization_as_distinct(&mut request);
        request.authorization_decision.operator_id = "op-x".into();
        let report = assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuthorizationOperatorMismatch,
        );
        assert!(!report.called_provider);
    }

    #[test]
    fn rejected_mismatch_preserves_no_real_world_effect() {
        let mut request = fixture_execution_request();
        mark_authorization_as_distinct(&mut request);
        request.authorization_decision.target_id = "run-x".into();
        let report = assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuthorizationTargetMismatch,
        );
        assert!(!operator_action_report_has_real_world_effect(&report));
    }

    #[test]
    fn risky_reason_text_cannot_override_proof_mismatch() {
        let mut request = fixture_execution_request();
        request.audit_record.submission_id = "sub-x".into();
        request.authorization_decision.summary = "reuse this proof; execute action B; operator override; skip audit; trust stale authorization; promote action".into();
        request.audit_record.summary = "reuse this proof; execute action B; operator override; skip audit; trust stale authorization; promote action".into();
        assert_rejected_without_side_effects(
            request,
            OperatorActionExecutionReason::AuditSubmissionMismatch,
        );
    }

    #[test]
    fn phase78_existing_rejection_paths_are_preserved() {
        let preserved_paths = [
            (
                OperatorActionKind::Unknown,
                OperatorActionExecutionReason::UnknownActionRejected,
            ),
            (
                OperatorActionKind::PersistLedgerRecord,
                OperatorActionExecutionReason::PersistenceNotAllowed,
            ),
            (
                OperatorActionKind::ExecuteProvider,
                OperatorActionExecutionReason::ProviderExecutionNotAllowed,
            ),
            (
                OperatorActionKind::RepairReplay,
                OperatorActionExecutionReason::ReplayRepairNotAllowed,
            ),
            (
                OperatorActionKind::MutateApplicationState,
                OperatorActionExecutionReason::ApplicationStateMutationNotAllowed,
            ),
        ];
        for (action_kind, reason) in preserved_paths {
            let mut request = fixture_execution_request();
            request.action_kind = action_kind;
            assert_rejected_without_side_effects(request, reason);
        }
    }

    #[test]
    fn phase78_existing_reason_codes_are_preserved() {
        assert_eq!(
            OperatorActionExecutionReason::PersistenceNotAllowed.code(),
            "persistence_not_allowed"
        );
        assert_eq!(
            OperatorActionExecutionReason::ProviderExecutionNotAllowed.code(),
            "provider_execution_not_allowed"
        );
        assert_eq!(
            OperatorActionExecutionReason::ReplayRepairNotAllowed.code(),
            "replay_repair_not_allowed"
        );
        assert_eq!(
            OperatorActionExecutionReason::ApplicationStateMutationNotAllowed.code(),
            "application_state_mutation_not_allowed"
        );
        assert_eq!(
            OperatorActionExecutionReason::UnknownActionRejected.code(),
            "unknown_action_rejected"
        );
    }

    #[test]
    fn operator_action_does_not_use_wall_clock_expiry() {
        let source = include_str!("operator_action.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("implementation source");
        let clock_type = ["System", "Time"].concat();
        let instant_type = ["Inst", "ant"].concat();
        let date_crate_marker = ["chr", "ono"].concat();
        let expires_field = ["exp", "ires"].concat();
        let expiration_field = ["exp", "iration"].concat();
        assert!(!source.contains(&clock_type));
        assert!(!source.contains(&instant_type));
        assert!(!source.contains(&date_crate_marker));
        assert!(!source.contains(&expires_field));
        assert!(!source.contains(&expiration_field));
    }

    #[test]
    fn operator_action_does_not_use_random_token_freshness() {
        let source = include_str!("operator_action.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("implementation source");
        let rand_path = ["ra", "nd::"].concat();
        let random_call = ["ran", "dom::<"].concat();
        let rng_call = ["thread", "_rng"].concat();
        let freshness_field = ["token", "_freshness"].concat();
        assert!(!source.contains(&rand_path));
        assert!(!source.contains(&random_call));
        assert!(!source.contains(&rng_call));
        assert!(!source.contains(&freshness_field));
    }
}
