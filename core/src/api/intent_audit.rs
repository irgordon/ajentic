use super::{
    diagnostic_key, ErrorDiagnostic, OperatorAuthorizationDecision, OperatorAuthorizationStatus,
    OperatorIntentIngressReport, OperatorIntentSubmission, OperatorIntentTargetKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorIntentAuditEligibility {
    Eligible,
    Ineligible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorIntentAuditReason {
    EligibleForRecordConstruction,
    EmptyAuditRecordId,
    MissingSubmission,
    MissingIngressReport,
    MissingAuthorizationDecision,
    SubmissionIngressMismatch,
    SubmissionAuthorizationMismatch,
    OperatorMismatch,
    TargetMismatch,
    AuthorizationNotSuccessful,
    ExecutionEnabled,
    RouteMissing,
    UnsupportedAuditSubject,
}

impl OperatorIntentAuditReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EligibleForRecordConstruction => "eligible_for_record_construction",
            Self::EmptyAuditRecordId => "empty_audit_record_id",
            Self::MissingSubmission => "missing_submission",
            Self::MissingIngressReport => "missing_ingress_report",
            Self::MissingAuthorizationDecision => "missing_authorization_decision",
            Self::SubmissionIngressMismatch => "submission_ingress_mismatch",
            Self::SubmissionAuthorizationMismatch => "submission_authorization_mismatch",
            Self::OperatorMismatch => "operator_mismatch",
            Self::TargetMismatch => "target_mismatch",
            Self::AuthorizationNotSuccessful => "authorization_not_successful",
            Self::ExecutionEnabled => "execution_enabled",
            Self::RouteMissing => "route_missing",
            Self::UnsupportedAuditSubject => "unsupported_audit_subject",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorIntentAuditRecord {
    pub audit_record_id: String,
    pub eligibility: OperatorIntentAuditEligibility,
    pub reason: OperatorIntentAuditReason,
    pub submission_id: String,
    pub authorization_id: String,
    pub operator_id: String,
    pub target_kind: OperatorIntentTargetKind,
    pub target_id: String,
    pub intent_type_code: String,
    pub ingress_reason_code: String,
    pub authorization_reason_code: String,
    pub execution_enabled: bool,
    pub diagnostic_key: String,
    pub summary: String,
}

impl OperatorIntentAuditRecord {
    pub fn new(
        audit_record_id: impl Into<String>,
        submission: &OperatorIntentSubmission,
        ingress_report: &OperatorIntentIngressReport,
        authorization_decision: &OperatorAuthorizationDecision,
    ) -> Self {
        let audit_record_id = audit_record_id.into();

        let reason = if audit_record_id.is_empty() {
            OperatorIntentAuditReason::EmptyAuditRecordId
        } else if submission.submission_id != ingress_report.submission_id {
            OperatorIntentAuditReason::SubmissionIngressMismatch
        } else if submission.submission_id != authorization_decision.submission_id {
            OperatorIntentAuditReason::SubmissionAuthorizationMismatch
        } else if submission.operator_id != ingress_report.operator_id
            || submission.operator_id != authorization_decision.operator_id
        {
            OperatorIntentAuditReason::OperatorMismatch
        } else if submission.target_kind != ingress_report.target_kind
            || submission.target_kind != authorization_decision.target_kind
            || submission.target_id != ingress_report.target_id
            || submission.target_id != authorization_decision.target_id
        {
            OperatorIntentAuditReason::TargetMismatch
        } else if authorization_decision.status != OperatorAuthorizationStatus::Authorized {
            OperatorIntentAuditReason::AuthorizationNotSuccessful
        } else if authorization_decision.execution_enabled {
            OperatorIntentAuditReason::ExecutionEnabled
        } else if ingress_report.route.is_none() {
            OperatorIntentAuditReason::RouteMissing
        } else {
            OperatorIntentAuditReason::EligibleForRecordConstruction
        };

        let eligibility = if reason == OperatorIntentAuditReason::EligibleForRecordConstruction {
            OperatorIntentAuditEligibility::Eligible
        } else {
            OperatorIntentAuditEligibility::Ineligible
        };

        let diagnostic_key = diagnostic_key(&ErrorDiagnostic::new(
            super::DiagnosticFamily::OperatorIntentAudit,
            reason.code(),
            "Operator intent audit record eligibility.",
        ));

        Self {
            audit_record_id,
            eligibility,
            reason,
            submission_id: submission.submission_id.clone(),
            authorization_id: authorization_decision.authorization_id.clone(),
            operator_id: submission.operator_id.clone(),
            target_kind: submission.target_kind,
            target_id: submission.target_id.clone(),
            intent_type_code: submission.intent.intent_type.code().to_string(),
            ingress_reason_code: ingress_report.reason.code().to_string(),
            authorization_reason_code: authorization_decision.reason.code().to_string(),
            execution_enabled: authorization_decision.execution_enabled,
            diagnostic_key,
            summary: "Operator intent audit record construction is a proof object only; it does not execute actions or persist records.".to_string(),
        }
    }
}

pub fn operator_intent_audit_record_executes_actions(_record: &OperatorIntentAuditRecord) -> bool {
    false
}

pub fn operator_intent_audit_record_persists(_record: &OperatorIntentAuditRecord) -> bool {
    false
}

trait IntentTypeCode {
    fn code(&self) -> &'static str;
}

impl IntentTypeCode for super::OperatorIntentType {
    fn code(&self) -> &'static str {
        match self {
            Self::Approve => "approve",
            Self::Reject => "reject",
            Self::Retry => "retry",
            Self::MemoryWrite => "memory_write",
            Self::MemoryDelete => "memory_delete",
            Self::RunStart => "run_start",
            Self::RunStop => "run_stop",
            Self::ToolRequest => "tool_request",
            Self::PolicyCheck => "policy_check",
            Self::StateTransitionRequest => "state_transition_request",
            Self::ContextRebuildRequest => "context_rebuild_request",
            Self::ReplayRequest => "replay_request",
            Self::MemorySnapshotRequest => "memory_snapshot_request",
            Self::MemoryDisableRequest => "memory_disable_request",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        authorize_operator_intent, diagnostic_family_label,
        operator_authorization_executes_actions, operator_intent_ingress_executes_actions,
        submit_operator_intent, OperatorAuthorizationReason, OperatorIdentity,
        OperatorIntent, OperatorIntentIngressReason, OperatorIntentTargetKind, OperatorIntentType,
        OperatorSafetyContext, OperatorTargetContext,
    };

    fn fixture_submission() -> OperatorIntentSubmission {
        OperatorIntentSubmission::new(
            "sub-1",
            "op-1",
            OperatorIntent::new(OperatorIntentType::Approve, "ok"),
            OperatorIntentTargetKind::Run,
            "run-1",
        )
        .expect("valid")
    }

    fn fixture_decision(
        submission: &OperatorIntentSubmission,
        ingress_report: &OperatorIntentIngressReport,
    ) -> OperatorAuthorizationDecision {
        let request = crate::api::OperatorAuthorizationRequest::new(
            "auth-1",
            submission.clone(),
            ingress_report.clone(),
            OperatorIdentity::new("op-1", "s", "claim").expect("identity"),
            OperatorSafetyContext::new("cfg", "strict", false, false, false).expect("safety"),
            OperatorTargetContext::new(OperatorIntentTargetKind::Run, "run-1", "run-1")
                .expect("target"),
        )
        .expect("request");
        authorize_operator_intent(request)
    }

    #[test]
    fn operator_intent_audit_reason_codes_are_stable() {
        assert_eq!(
            OperatorIntentAuditReason::EligibleForRecordConstruction.code(),
            "eligible_for_record_construction"
        );
    }
    #[test]
    fn audit_record_requires_audit_record_id() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        assert_eq!(
            OperatorIntentAuditRecord::new("", &sub, &ingress, &auth).reason,
            OperatorIntentAuditReason::EmptyAuditRecordId
        );
    }
    #[test]
    fn audit_record_rejects_submission_ingress_mismatch() {
        let sub = fixture_submission();
        let mut ingress = submit_operator_intent(sub.clone());
        ingress.submission_id = "x".into();
        let auth = fixture_decision(&sub, &submit_operator_intent(sub.clone()));
        assert_eq!(
            OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth).reason,
            OperatorIntentAuditReason::SubmissionIngressMismatch
        );
    }
    #[test]
    fn audit_record_rejects_submission_authorization_mismatch() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let mut auth = fixture_decision(&sub, &ingress);
        auth.submission_id = "x".into();
        assert_eq!(
            OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth).reason,
            OperatorIntentAuditReason::SubmissionAuthorizationMismatch
        );
    }
    #[test]
    fn audit_record_rejects_operator_mismatch() {
        let sub = fixture_submission();
        let mut ingress = submit_operator_intent(sub.clone());
        ingress.operator_id = "x".into();
        let auth = fixture_decision(&sub, &submit_operator_intent(sub.clone()));
        assert_eq!(
            OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth).reason,
            OperatorIntentAuditReason::OperatorMismatch
        );
    }
    #[test]
    fn audit_record_rejects_target_kind_mismatch() {
        let sub = fixture_submission();
        let mut ingress = submit_operator_intent(sub.clone());
        ingress.target_kind = OperatorIntentTargetKind::Context;
        let auth = fixture_decision(&sub, &submit_operator_intent(sub.clone()));
        assert_eq!(
            OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth).reason,
            OperatorIntentAuditReason::TargetMismatch
        );
    }
    #[test]
    fn audit_record_rejects_target_id_mismatch() {
        let sub = fixture_submission();
        let mut ingress = submit_operator_intent(sub.clone());
        ingress.target_id = "zzz".into();
        let auth = fixture_decision(&sub, &submit_operator_intent(sub.clone()));
        assert_eq!(
            OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth).reason,
            OperatorIntentAuditReason::TargetMismatch
        );
    }
    #[test]
    fn audit_record_rejects_unsuccessful_authorization() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let mut auth = fixture_decision(&sub, &ingress);
        auth.status = OperatorAuthorizationStatus::Denied;
        auth.reason = OperatorAuthorizationReason::IngressNotAccepted;
        assert_eq!(
            OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth).reason,
            OperatorIntentAuditReason::AuthorizationNotSuccessful
        );
    }
    #[test]
    fn audit_record_rejects_execution_enabled_authorization() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let mut auth = fixture_decision(&sub, &ingress);
        auth.execution_enabled = true;
        assert_eq!(
            OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth).reason,
            OperatorIntentAuditReason::ExecutionEnabled
        );
    }
    #[test]
    fn audit_record_rejects_missing_route() {
        let sub = fixture_submission();
        let mut ingress = submit_operator_intent(sub.clone());
        ingress.route = None;
        let auth = fixture_decision(&sub, &submit_operator_intent(sub.clone()));
        assert_eq!(
            OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth).reason,
            OperatorIntentAuditReason::RouteMissing
        );
    }
    #[test]
    fn audit_record_constructs_eligible_record_for_consistent_authorized_metadata() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        let rec = OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth);
        assert_eq!(rec.eligibility, OperatorIntentAuditEligibility::Eligible);
        assert_eq!(
            rec.reason,
            OperatorIntentAuditReason::EligibleForRecordConstruction
        );
    }
    #[test]
    fn audit_record_preserves_submission_authorization_operator_and_target_ids() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        let rec = OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth);
        assert_eq!(rec.submission_id, "sub-1");
        assert_eq!(rec.authorization_id, "auth-1");
        assert_eq!(rec.operator_id, "op-1");
        assert_eq!(rec.target_id, "run-1");
    }
    #[test]
    fn audit_record_preserves_ingress_and_authorization_reason_codes() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        let rec = OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth);
        assert_eq!(
            rec.ingress_reason_code,
            OperatorIntentIngressReason::AcceptedForRouting.code()
        );
        assert_eq!(
            rec.authorization_reason_code,
            OperatorAuthorizationReason::AuthorizedForFutureExecution.code()
        );
    }
    #[test]
    fn audit_record_uses_stable_diagnostic_key() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        let rec = OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth);
        assert_eq!(
            rec.diagnostic_key,
            format!(
                "{}.{}",
                diagnostic_family_label(super::super::DiagnosticFamily::OperatorIntentAudit),
                OperatorIntentAuditReason::EligibleForRecordConstruction.code()
            )
        );
    }
    #[test]
    fn audit_record_never_executes_actions() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        let rec = OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth);
        assert!(!operator_intent_audit_record_executes_actions(&rec));
        assert!(!operator_intent_ingress_executes_actions(&ingress));
        assert!(!operator_authorization_executes_actions(&auth));
    }
    #[test]
    fn audit_record_never_persists() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        let rec = OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth);
        assert!(!operator_intent_audit_record_persists(&rec));
    }
    #[test]
    fn audit_record_does_not_append_ledger_or_audit_records() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        let rec = OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth);
        assert!(!rec.summary.to_lowercase().contains("append"));
    }
    #[test]
    fn audit_record_does_not_call_provider_or_model() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        let rec = OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth);
        assert!(!rec.summary.to_lowercase().contains("provider"));
        assert!(!rec.summary.to_lowercase().contains("model"));
    }
    #[test]
    fn audit_record_does_not_call_persistence_helpers() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        let rec = OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth);
        assert!(!rec
            .summary
            .to_lowercase()
            .contains("execute_local_persistence_plan"));
    }
    #[test]
    fn audit_record_does_not_touch_execution_module() {
        let sub = fixture_submission();
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        let rec = OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth);
        assert!(!rec.summary.to_lowercase().contains("execution::"));
    }
    #[test]
    fn risky_reason_text_does_not_make_audit_record_execute() {
        let sub = OperatorIntentSubmission::new(
            "sub",
            "op",
            OperatorIntent::new(OperatorIntentType::Approve, "force execute now"),
            OperatorIntentTargetKind::Run,
            "run-1",
        )
        .expect("valid");
        let ingress = submit_operator_intent(sub.clone());
        let auth = fixture_decision(&sub, &ingress);
        let rec = OperatorIntentAuditRecord::new("a", &sub, &ingress, &auth);
        assert!(!operator_intent_audit_record_executes_actions(&rec));
    }
}
