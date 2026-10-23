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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionRiskClass {
    ReadOnly,
    ReversibleMutation,
    IrreversibleMutation,
    ExternalCommunication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionReversibility {
    Reversible,
    Compensatable,
    Irreversible,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactActionApprovalRequest {
    pub binding: crate::authority::AuthorityBinding,
    pub tool: String,
    pub argument_digest: crate::integrity::Digest,
    pub target: String,
    pub recipient: Option<String>,
    pub disclosed_data_digest: crate::integrity::Digest,
    pub risk_class: ActionRiskClass,
    pub reversibility: ActionReversibility,
    pub expected_cost_microunits: u64,
    pub operator_id: String,
    pub expires_after_revision: u64,
    pub nonce: String,
    pub previous_approval_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactActionApprovalReceipt {
    binding: crate::authority::AuthorityBinding,
    approval_id: String,
    tool: String,
    argument_digest: crate::integrity::Digest,
    target: String,
    recipient: Option<String>,
    disclosed_data_digest: crate::integrity::Digest,
    risk_class: ActionRiskClass,
    reversibility: ActionReversibility,
    expected_cost_microunits: u64,
    operator_id: String,
    expires_after_revision: u64,
    nonce: String,
    previous_approval_id: Option<String>,
    receipt_digest: crate::integrity::Digest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactActionApprovalError {
    AuthorizationDenied,
    ExecutionFlagInvalid,
    OperatorMismatch,
    EmptyTool,
    EmptyTarget,
    EmptyNonce,
    InvalidExpiry,
    BindingMismatch,
    ToolMismatch,
    ArgumentMismatch,
    TargetMismatch,
    RecipientMismatch,
    DisclosureMismatch,
    ApprovalExpired,
    NonceAlreadyConsumed,
}

impl ExactActionApprovalError {
    pub fn code(self) -> &'static str {
        match self {
            Self::AuthorizationDenied => "authorization_denied",
            Self::ExecutionFlagInvalid => "execution_flag_invalid",
            Self::OperatorMismatch => "operator_mismatch",
            Self::EmptyTool => "empty_tool",
            Self::EmptyTarget => "empty_target",
            Self::EmptyNonce => "empty_nonce",
            Self::InvalidExpiry => "invalid_expiry",
            Self::BindingMismatch => "binding_mismatch",
            Self::ToolMismatch => "tool_mismatch",
            Self::ArgumentMismatch => "argument_mismatch",
            Self::TargetMismatch => "target_mismatch",
            Self::RecipientMismatch => "recipient_mismatch",
            Self::DisclosureMismatch => "disclosure_mismatch",
            Self::ApprovalExpired => "approval_expired",
            Self::NonceAlreadyConsumed => "nonce_already_consumed",
        }
    }
}

impl ExactActionApprovalReceipt {
    pub fn binding(&self) -> &crate::authority::AuthorityBinding {
        &self.binding
    }

    pub fn approval_id(&self) -> &str {
        &self.approval_id
    }

    pub fn nonce(&self) -> &str {
        &self.nonce
    }

    pub fn digest(&self) -> &crate::integrity::Digest {
        &self.receipt_digest
    }
}

pub fn authorize_exact_action(
    decision: &OperatorAuthorizationDecision,
    request: ExactActionApprovalRequest,
    current_revision: u64,
) -> Result<ExactActionApprovalReceipt, ExactActionApprovalError> {
    validate_approval_request(decision, &request, current_revision)?;
    Ok(issue_exact_action_receipt(decision, request))
}

pub fn verify_exact_action_approval(
    receipt: &ExactActionApprovalReceipt,
    proposed: &ExactActionApprovalRequest,
    current_revision: u64,
    consumed_nonces: &std::collections::HashSet<String>,
) -> Result<(), ExactActionApprovalError> {
    verify_action_binding(receipt, proposed)?;
    verify_action_shape(receipt, proposed)?;
    verify_action_liveness(receipt, current_revision, consumed_nonces)
}

fn validate_approval_request(
    decision: &OperatorAuthorizationDecision,
    request: &ExactActionApprovalRequest,
    current_revision: u64,
) -> Result<(), ExactActionApprovalError> {
    validate_authorization_decision(decision, request)?;
    validate_exact_action_fields(request)?;
    validate_approval_expiry(request, current_revision)
}

fn validate_authorization_decision(
    decision: &OperatorAuthorizationDecision,
    request: &ExactActionApprovalRequest,
) -> Result<(), ExactActionApprovalError> {
    if decision.status != OperatorAuthorizationStatus::Authorized {
        return Err(ExactActionApprovalError::AuthorizationDenied);
    }
    if decision.execution_enabled {
        return Err(ExactActionApprovalError::ExecutionFlagInvalid);
    }
    if decision.target_kind != OperatorIntentTargetKind::Run
        || decision.target_id != request.binding.run_id()
    {
        return Err(ExactActionApprovalError::BindingMismatch);
    }
    if decision.operator_id == request.operator_id {
        return Ok(());
    }
    Err(ExactActionApprovalError::OperatorMismatch)
}

fn validate_exact_action_fields(
    request: &ExactActionApprovalRequest,
) -> Result<(), ExactActionApprovalError> {
    validate_action_text(&request.tool, ExactActionApprovalError::EmptyTool)?;
    validate_action_text(&request.target, ExactActionApprovalError::EmptyTarget)?;
    validate_action_text(&request.nonce, ExactActionApprovalError::EmptyNonce)
}

fn validate_approval_expiry(
    request: &ExactActionApprovalRequest,
    current_revision: u64,
) -> Result<(), ExactActionApprovalError> {
    if request.expires_after_revision >= current_revision
        && request.binding.valid_through_revision() >= current_revision
    {
        return Ok(());
    }
    Err(ExactActionApprovalError::InvalidExpiry)
}

fn issue_exact_action_receipt(
    decision: &OperatorAuthorizationDecision,
    request: ExactActionApprovalRequest,
) -> ExactActionApprovalReceipt {
    let approval_id = format!(
        "exact-action:{}:{}",
        decision.authorization_id, request.nonce
    );
    let receipt_digest = exact_action_receipt_digest(&approval_id, &request);
    ExactActionApprovalReceipt {
        binding: request.binding,
        approval_id,
        tool: request.tool,
        argument_digest: request.argument_digest,
        target: request.target,
        recipient: request.recipient,
        disclosed_data_digest: request.disclosed_data_digest,
        risk_class: request.risk_class,
        reversibility: request.reversibility,
        expected_cost_microunits: request.expected_cost_microunits,
        operator_id: request.operator_id,
        expires_after_revision: request.expires_after_revision,
        nonce: request.nonce,
        previous_approval_id: request.previous_approval_id,
        receipt_digest,
    }
}

fn exact_action_receipt_digest(
    approval_id: &str,
    request: &ExactActionApprovalRequest,
) -> crate::integrity::Digest {
    crate::integrity::Digest::of_text(&format!(
        "approval|{}|{}|{}|{}|{:?}|{}|{}|{:?}|{:?}|{}|{}|{}|{}|{:?}",
        approval_id,
        request.binding.run_id(),
        request.binding.task_digest().as_str(),
        request.tool,
        request.argument_digest,
        request.target,
        request.recipient.as_deref().unwrap_or(""),
        request.risk_class,
        request.reversibility,
        request.disclosed_data_digest.as_str(),
        request.expected_cost_microunits,
        request.operator_id,
        request.expires_after_revision,
        request.previous_approval_id
    ))
}

fn verify_action_binding(
    receipt: &ExactActionApprovalReceipt,
    proposed: &ExactActionApprovalRequest,
) -> Result<(), ExactActionApprovalError> {
    if receipt.binding == proposed.binding && receipt.operator_id == proposed.operator_id {
        return Ok(());
    }
    Err(ExactActionApprovalError::BindingMismatch)
}

fn verify_action_shape(
    receipt: &ExactActionApprovalReceipt,
    proposed: &ExactActionApprovalRequest,
) -> Result<(), ExactActionApprovalError> {
    verify_tool(receipt, proposed)?;
    verify_arguments(receipt, proposed)?;
    verify_target(receipt, proposed)?;
    verify_recipient(receipt, proposed)?;
    verify_disclosure(receipt, proposed)
}

fn verify_tool(
    receipt: &ExactActionApprovalReceipt,
    proposed: &ExactActionApprovalRequest,
) -> Result<(), ExactActionApprovalError> {
    if receipt.tool == proposed.tool {
        return Ok(());
    }
    Err(ExactActionApprovalError::ToolMismatch)
}

fn verify_arguments(
    receipt: &ExactActionApprovalReceipt,
    proposed: &ExactActionApprovalRequest,
) -> Result<(), ExactActionApprovalError> {
    if receipt.argument_digest == proposed.argument_digest {
        return Ok(());
    }
    Err(ExactActionApprovalError::ArgumentMismatch)
}

fn verify_target(
    receipt: &ExactActionApprovalReceipt,
    proposed: &ExactActionApprovalRequest,
) -> Result<(), ExactActionApprovalError> {
    if receipt.target == proposed.target {
        return Ok(());
    }
    Err(ExactActionApprovalError::TargetMismatch)
}

fn verify_recipient(
    receipt: &ExactActionApprovalReceipt,
    proposed: &ExactActionApprovalRequest,
) -> Result<(), ExactActionApprovalError> {
    if receipt.recipient == proposed.recipient {
        return Ok(());
    }
    Err(ExactActionApprovalError::RecipientMismatch)
}

fn verify_disclosure(
    receipt: &ExactActionApprovalReceipt,
    proposed: &ExactActionApprovalRequest,
) -> Result<(), ExactActionApprovalError> {
    if receipt.disclosed_data_digest == proposed.disclosed_data_digest
        && receipt.risk_class == proposed.risk_class
        && receipt.reversibility == proposed.reversibility
        && receipt.expected_cost_microunits == proposed.expected_cost_microunits
        && receipt.previous_approval_id == proposed.previous_approval_id
    {
        return Ok(());
    }
    Err(ExactActionApprovalError::DisclosureMismatch)
}

fn verify_action_liveness(
    receipt: &ExactActionApprovalReceipt,
    current_revision: u64,
    consumed_nonces: &std::collections::HashSet<String>,
) -> Result<(), ExactActionApprovalError> {
    if current_revision > receipt.expires_after_revision {
        return Err(ExactActionApprovalError::ApprovalExpired);
    }
    if consumed_nonces.contains(receipt.nonce()) {
        return Err(ExactActionApprovalError::NonceAlreadyConsumed);
    }
    Ok(())
}

fn validate_action_text(
    value: &str,
    error: ExactActionApprovalError,
) -> Result<(), ExactActionApprovalError> {
    if !value.trim().is_empty() {
        return Ok(());
    }
    Err(error)
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

    fn exact_action_request() -> ExactActionApprovalRequest {
        ExactActionApprovalRequest {
            binding: crate::authority::AuthorityBinding::new(
                crate::authority::AuthorityBindingInput {
                    run_id: "run-1".into(),
                    task_digest: crate::integrity::Digest::of_text("task"),
                    operator_intent_digest: crate::integrity::Digest::of_text("intent"),
                    context_packet_digest: crate::integrity::Digest::of_text("context"),
                    candidate_digest: crate::integrity::Digest::of_text("candidate"),
                    policy_bundle_digest: crate::integrity::Digest::of_text("policy"),
                    evidence_manifest_digest: crate::integrity::Digest::of_text("evidence"),
                    verifier_id: "approval-verifier".into(),
                    verifier_version: "1.0.0".into(),
                    valid_through_revision: 3,
                },
            )
            .unwrap(),
            tool: "send_email".into(),
            argument_digest: crate::integrity::Digest::of_text("to=A;body=X"),
            target: "mailbox".into(),
            recipient: Some("A".into()),
            disclosed_data_digest: crate::integrity::Digest::of_text("body=X"),
            risk_class: ActionRiskClass::ExternalCommunication,
            reversibility: ActionReversibility::Irreversible,
            expected_cost_microunits: 1,
            operator_id: "op-1".into(),
            expires_after_revision: 3,
            nonce: "nonce-1".into(),
            previous_approval_id: None,
        }
    }

    #[test]
    fn exact_action_approval_matches_unchanged_action() {
        let decision = authorize_operator_intent(fixture_request());
        let request = exact_action_request();
        let receipt = authorize_exact_action(&decision, request.clone(), 2).unwrap();
        assert_eq!(
            verify_exact_action_approval(&receipt, &request, 2, &std::collections::HashSet::new(),),
            Ok(())
        );
    }

    #[test]
    fn edited_arguments_invalidate_exact_action_approval() {
        let decision = authorize_operator_intent(fixture_request());
        let request = exact_action_request();
        let receipt = authorize_exact_action(&decision, request.clone(), 2).unwrap();
        let mut edited = request;
        edited.argument_digest = crate::integrity::Digest::of_text("to=B;body=X");
        assert_eq!(
            verify_exact_action_approval(&receipt, &edited, 2, &std::collections::HashSet::new(),),
            Err(ExactActionApprovalError::ArgumentMismatch)
        );
    }
}
