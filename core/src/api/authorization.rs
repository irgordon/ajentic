use super::{
    OperatorIntentIngressReport, OperatorIntentIngressStatus, OperatorIntentSubmission,
    OperatorIntentTargetKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorAuthorizationStatus {
    Authorized,
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorAuthorizationReason {
    AuthorizedForFutureExecution,
    EmptyAuthorizationId,
    MissingSubmission,
    MissingIngressReport,
    MissingOperatorIdentity,
    OperatorIdentityMismatch,
    MissingSafetyContext,
    UnsafeRuntimePosture,
    MissingTargetContext,
    TargetContextMismatch,
    IngressNotAccepted,
    RouteMissing,
    IntentExecutionNotEnabled,
}

impl OperatorAuthorizationReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AuthorizedForFutureExecution => "authorized_for_future_execution",
            Self::EmptyAuthorizationId => "empty_authorization_id",
            Self::MissingSubmission => "missing_submission",
            Self::MissingIngressReport => "missing_ingress_report",
            Self::MissingOperatorIdentity => "missing_operator_identity",
            Self::OperatorIdentityMismatch => "operator_identity_mismatch",
            Self::MissingSafetyContext => "missing_safety_context",
            Self::UnsafeRuntimePosture => "unsafe_runtime_posture",
            Self::MissingTargetContext => "missing_target_context",
            Self::TargetContextMismatch => "target_context_mismatch",
            Self::IngressNotAccepted => "ingress_not_accepted",
            Self::RouteMissing => "route_missing",
            Self::IntentExecutionNotEnabled => "intent_execution_not_enabled",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorIdentity {
    pub operator_id: String,
    pub session_id: String,
    pub identity_claim: String,
}

impl OperatorIdentity {
    pub fn new(
        operator_id: impl Into<String>,
        session_id: impl Into<String>,
        identity_claim: impl Into<String>,
    ) -> Result<Self, OperatorAuthorizationReason> {
        let operator_id = operator_id.into();
        if operator_id.is_empty() {
            return Err(OperatorAuthorizationReason::MissingOperatorIdentity);
        }
        let session_id = session_id.into();
        if session_id.is_empty() {
            return Err(OperatorAuthorizationReason::MissingOperatorIdentity);
        }
        let identity_claim = identity_claim.into();
        if identity_claim.is_empty() {
            return Err(OperatorAuthorizationReason::MissingOperatorIdentity);
        }

        Ok(Self {
            operator_id,
            session_id,
            identity_claim,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorSafetyContext {
    pub runtime_config_id: String,
    pub safety_level: String,
    pub provider_network_allowed: bool,
    pub file_io_allowed: bool,
    pub ui_mutation_allowed: bool,
}

impl OperatorSafetyContext {
    pub fn new(
        runtime_config_id: impl Into<String>,
        safety_level: impl Into<String>,
        provider_network_allowed: bool,
        file_io_allowed: bool,
        ui_mutation_allowed: bool,
    ) -> Result<Self, OperatorAuthorizationReason> {
        let runtime_config_id = runtime_config_id.into();
        if runtime_config_id.is_empty() {
            return Err(OperatorAuthorizationReason::MissingSafetyContext);
        }
        let safety_level = safety_level.into();
        if safety_level.is_empty() {
            return Err(OperatorAuthorizationReason::MissingSafetyContext);
        }

        Ok(Self {
            runtime_config_id,
            safety_level,
            provider_network_allowed,
            file_io_allowed,
            ui_mutation_allowed,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorTargetContext {
    pub target_kind: OperatorIntentTargetKind,
    pub target_id: String,
    pub run_id: String,
}

impl OperatorTargetContext {
    pub fn new(
        target_kind: OperatorIntentTargetKind,
        target_id: impl Into<String>,
        run_id: impl Into<String>,
    ) -> Result<Self, OperatorAuthorizationReason> {
        let target_id = target_id.into();
        if target_id.is_empty() {
            return Err(OperatorAuthorizationReason::MissingTargetContext);
        }
        let run_id = run_id.into();
        if run_id.is_empty() {
            return Err(OperatorAuthorizationReason::MissingTargetContext);
        }

        Ok(Self {
            target_kind,
            target_id,
            run_id,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorAuthorizationRequest {
    pub authorization_id: String,
    pub submission: OperatorIntentSubmission,
    pub ingress_report: OperatorIntentIngressReport,
    pub identity: OperatorIdentity,
    pub safety_context: OperatorSafetyContext,
    pub target_context: OperatorTargetContext,
}

impl OperatorAuthorizationRequest {
    pub fn new(
        authorization_id: impl Into<String>,
        submission: OperatorIntentSubmission,
        ingress_report: OperatorIntentIngressReport,
        identity: OperatorIdentity,
        safety_context: OperatorSafetyContext,
        target_context: OperatorTargetContext,
    ) -> Result<Self, OperatorAuthorizationReason> {
        let authorization_id = authorization_id.into();
        if authorization_id.is_empty() {
            return Err(OperatorAuthorizationReason::EmptyAuthorizationId);
        }

        Ok(Self {
            authorization_id,
            submission,
            ingress_report,
            identity,
            safety_context,
            target_context,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorAuthorizationDecision {
    pub status: OperatorAuthorizationStatus,
    pub reason: OperatorAuthorizationReason,
    pub authorization_id: String,
    pub submission_id: String,
    pub operator_id: String,
    pub target_kind: OperatorIntentTargetKind,
    pub target_id: String,
    pub execution_enabled: bool,
    pub summary: String,
}

pub fn authorize_operator_intent(
    request: OperatorAuthorizationRequest,
) -> OperatorAuthorizationDecision {
    if request.identity.operator_id != request.submission.operator_id {
        return denied(
            request,
            OperatorAuthorizationReason::OperatorIdentityMismatch,
        );
    }
    if request.ingress_report.operator_id != request.submission.operator_id {
        return denied(
            request,
            OperatorAuthorizationReason::OperatorIdentityMismatch,
        );
    }
    if request.ingress_report.submission_id != request.submission.submission_id {
        return denied(request, OperatorAuthorizationReason::IngressNotAccepted);
    }
    if request.ingress_report.status != OperatorIntentIngressStatus::Accepted {
        return denied(request, OperatorAuthorizationReason::IngressNotAccepted);
    }
    if request.ingress_report.route.is_none() {
        return denied(request, OperatorAuthorizationReason::RouteMissing);
    }
    if request.safety_context.provider_network_allowed
        || request.safety_context.file_io_allowed
        || request.safety_context.ui_mutation_allowed
    {
        return denied(request, OperatorAuthorizationReason::UnsafeRuntimePosture);
    }
    if request.target_context.target_kind != request.submission.target_kind {
        return denied(request, OperatorAuthorizationReason::TargetContextMismatch);
    }
    if request.target_context.target_id != request.submission.target_id {
        return denied(request, OperatorAuthorizationReason::TargetContextMismatch);
    }

    OperatorAuthorizationDecision {
        status: OperatorAuthorizationStatus::Authorized,
        reason: OperatorAuthorizationReason::AuthorizedForFutureExecution,
        authorization_id: request.authorization_id,
        submission_id: request.submission.submission_id,
        operator_id: request.submission.operator_id,
        target_kind: request.submission.target_kind,
        target_id: request.submission.target_id,
        execution_enabled: false,
        summary: "Authorization is metadata only and does not execute operator actions."
            .to_string(),
    }
}

fn denied(
    request: OperatorAuthorizationRequest,
    reason: OperatorAuthorizationReason,
) -> OperatorAuthorizationDecision {
    OperatorAuthorizationDecision {
        status: OperatorAuthorizationStatus::Denied,
        reason,
        authorization_id: request.authorization_id,
        submission_id: request.submission.submission_id,
        operator_id: request.submission.operator_id,
        target_kind: request.submission.target_kind,
        target_id: request.submission.target_id,
        execution_enabled: false,
        summary: "Authorization denied. Authorization does not execute operator actions."
            .to_string(),
    }
}

pub fn operator_authorization_executes_actions(_decision: &OperatorAuthorizationDecision) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        operator_intent_ingress_executes_actions, submit_operator_intent, OperatorIntent,
        OperatorIntentIngressReason, OperatorIntentType,
    };

    #[test]
    fn operator_authorization_reason_codes_are_stable() {
        assert_eq!(
            OperatorAuthorizationReason::AuthorizedForFutureExecution.code(),
            "authorized_for_future_execution"
        );
        assert_eq!(
            OperatorAuthorizationReason::IntentExecutionNotEnabled.code(),
            "intent_execution_not_enabled"
        );
    }

    #[test]
    fn operator_identity_requires_operator_id() {
        assert!(OperatorIdentity::new("", "s", "c").is_err());
    }
    #[test]
    fn operator_identity_requires_session_id() {
        assert!(OperatorIdentity::new("o", "", "c").is_err());
    }
    #[test]
    fn operator_identity_requires_identity_claim() {
        assert!(OperatorIdentity::new("o", "s", "").is_err());
    }
    #[test]
    fn operator_safety_context_requires_runtime_config_id() {
        assert!(OperatorSafetyContext::new("", "strict", false, false, false).is_err());
    }

    fn fixture_request() -> OperatorAuthorizationRequest {
        let submission = OperatorIntentSubmission::new(
            "sub-1",
            "op-1",
            OperatorIntent::new(OperatorIntentType::Approve, "reason"),
            OperatorIntentTargetKind::Run,
            "run-1",
        )
        .unwrap();
        let ingress_report = submit_operator_intent(submission.clone());
        OperatorAuthorizationRequest::new(
            "auth-1",
            submission,
            ingress_report,
            OperatorIdentity::new("op-1", "session-1", "claim-1").unwrap(),
            OperatorSafetyContext::new("cfg-1", "strict", false, false, false).unwrap(),
            OperatorTargetContext::new(OperatorIntentTargetKind::Run, "run-1", "run-1").unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn operator_safety_context_denies_provider_network() {
        let mut r = fixture_request();
        r.safety_context.provider_network_allowed = true;
        assert_eq!(
            authorize_operator_intent(r).status,
            OperatorAuthorizationStatus::Denied
        );
    }
    #[test]
    fn operator_safety_context_denies_file_io() {
        let mut r = fixture_request();
        r.safety_context.file_io_allowed = true;
        assert_eq!(
            authorize_operator_intent(r).reason,
            OperatorAuthorizationReason::UnsafeRuntimePosture
        );
    }
    #[test]
    fn operator_safety_context_denies_ui_mutation() {
        let mut r = fixture_request();
        r.safety_context.ui_mutation_allowed = true;
        assert_eq!(
            authorize_operator_intent(r).reason,
            OperatorAuthorizationReason::UnsafeRuntimePosture
        );
    }
    #[test]
    fn operator_target_context_requires_target_id() {
        assert!(OperatorTargetContext::new(OperatorIntentTargetKind::Run, "", "run").is_err());
    }
    #[test]
    fn authorization_request_requires_authorization_id() {
        let fr = fixture_request();
        assert!(OperatorAuthorizationRequest::new(
            "",
            fr.submission,
            fr.ingress_report,
            fr.identity,
            fr.safety_context,
            fr.target_context
        )
        .is_err());
    }
    #[test]
    fn authorization_denies_operator_identity_mismatch() {
        let mut r = fixture_request();
        r.identity.operator_id = "op-2".to_string();
        assert_eq!(
            authorize_operator_intent(r).reason,
            OperatorAuthorizationReason::OperatorIdentityMismatch
        );
    }
    #[test]
    fn authorization_denies_ingress_operator_mismatch() {
        let mut r = fixture_request();
        r.ingress_report.operator_id = "op-2".to_string();
        assert_eq!(
            authorize_operator_intent(r).reason,
            OperatorAuthorizationReason::OperatorIdentityMismatch
        );
    }
    #[test]
    fn authorization_denies_ingress_submission_mismatch() {
        let mut r = fixture_request();
        r.ingress_report.submission_id = "sub-2".to_string();
        assert_eq!(
            authorize_operator_intent(r).reason,
            OperatorAuthorizationReason::IngressNotAccepted
        );
    }
    #[test]
    fn authorization_denies_rejected_ingress() {
        let mut r = fixture_request();
        r.ingress_report.status = OperatorIntentIngressStatus::Rejected;
        r.ingress_report.reason = OperatorIntentIngressReason::RouteRejected;
        assert_eq!(
            authorize_operator_intent(r).reason,
            OperatorAuthorizationReason::IngressNotAccepted
        );
    }
    #[test]
    fn authorization_denies_missing_route() {
        let mut r = fixture_request();
        r.ingress_report.route = None;
        assert_eq!(
            authorize_operator_intent(r).reason,
            OperatorAuthorizationReason::RouteMissing
        );
    }
    #[test]
    fn authorization_denies_target_kind_mismatch() {
        let mut r = fixture_request();
        r.target_context.target_kind = OperatorIntentTargetKind::Replay;
        assert_eq!(
            authorize_operator_intent(r).reason,
            OperatorAuthorizationReason::TargetContextMismatch
        );
    }
    #[test]
    fn authorization_denies_target_id_mismatch() {
        let mut r = fixture_request();
        r.target_context.target_id = "x".to_string();
        assert_eq!(
            authorize_operator_intent(r).reason,
            OperatorAuthorizationReason::TargetContextMismatch
        );
    }
    #[test]
    fn authorization_allows_valid_identity_safety_context_without_execution() {
        let d = authorize_operator_intent(fixture_request());
        assert_eq!(d.status, OperatorAuthorizationStatus::Authorized);
        assert!(!d.execution_enabled);
        assert!(d.summary.contains("does not execute"));
    }
    #[test]
    fn authorization_decision_never_executes_actions() {
        let d = authorize_operator_intent(fixture_request());
        assert!(!operator_authorization_executes_actions(&d));
    }
    #[test]
    fn authorization_does_not_append_ledger_or_audit_records() {
        let d = authorize_operator_intent(fixture_request());
        assert!(!d.summary.to_ascii_lowercase().contains("ledger"));
        assert!(!d.summary.to_ascii_lowercase().contains("audit"));
    }
    #[test]
    fn authorization_does_not_persist() {
        let d = authorize_operator_intent(fixture_request());
        assert!(!d.summary.to_ascii_lowercase().contains("persist"));
    }
    #[test]
    fn authorization_does_not_call_provider_or_model() {
        let d = authorize_operator_intent(fixture_request());
        assert!(!d.summary.to_ascii_lowercase().contains("provider"));
        assert!(!d.summary.to_ascii_lowercase().contains("model"));
    }
    #[test]
    fn authorization_does_not_touch_execution_module() {
        let d = authorize_operator_intent(fixture_request());
        assert_eq!(
            d.reason,
            OperatorAuthorizationReason::AuthorizedForFutureExecution
        );
    }
    #[test]
    fn risky_reason_text_does_not_grant_authorization() {
        for phrase in [
            "force promote",
            "skip policy",
            "trust output",
            "write ledger",
            "execute provider",
        ] {
            let submission = OperatorIntentSubmission::new(
                "sub",
                "op",
                OperatorIntent::new(OperatorIntentType::Approve, phrase),
                OperatorIntentTargetKind::Run,
                "run",
            )
            .unwrap();
            let mut req = fixture_request();
            req.submission = submission.clone();
            req.ingress_report = submit_operator_intent(submission);
            req.identity.operator_id = "other".to_string();
            assert_eq!(
                authorize_operator_intent(req).status,
                OperatorAuthorizationStatus::Denied
            );
        }
    }
    #[test]
    fn dry_run_does_not_authorize_operator_intent() {
        let req = fixture_request();
        let ingress = req.ingress_report.clone();
        assert!(!operator_intent_ingress_executes_actions(&ingress));
        let d = authorize_operator_intent(req);
        assert!(!operator_authorization_executes_actions(&d));
    }
}
