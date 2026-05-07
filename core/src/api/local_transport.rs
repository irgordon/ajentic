use std::collections::BTreeMap;

const MAX_LOCAL_TRANSPORT_PAYLOAD_BYTES: usize = 4096;
const LOCAL_BIND_HOST: &str = "127.0.0.1";
const LOCAL_TRANSPORT_VERSION: &str = "phase-104-local-transport-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalUiRustTransportStartupStatus {
    Started,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalUiRustTransportStartupReason {
    LocalOnlyLoopbackReady,
    RemoteOrPublicBindRejected,
    EmptyBindRejected,
}

impl LocalUiRustTransportStartupReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::LocalOnlyLoopbackReady => "local_only_loopback_ready",
            Self::RemoteOrPublicBindRejected => "remote_or_public_bind_rejected",
            Self::EmptyBindRejected => "empty_bind_rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalUiRustTransportStartupRequest {
    pub bind_host: String,
}

impl LocalUiRustTransportStartupRequest {
    pub fn loopback() -> Self {
        Self {
            bind_host: LOCAL_BIND_HOST.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalUiRustTransportStartupReport {
    pub status: LocalUiRustTransportStartupStatus,
    pub reason: LocalUiRustTransportStartupReason,
    pub bind_host: String,
    pub local_only: bool,
    pub public_network_exposed: bool,
    pub authenticated_remote_access: bool,
    pub background_execution_enabled: bool,
    pub provider_execution_enabled: bool,
    pub persistence_enabled: bool,
    pub replay_repair_enabled: bool,
    pub recovery_promotion_enabled: bool,
    pub action_execution_enabled: bool,
    pub summary: String,
}

pub fn start_bounded_local_ui_rust_transport(
    request: LocalUiRustTransportStartupRequest,
) -> LocalUiRustTransportStartupReport {
    if request.bind_host.trim().is_empty() {
        return local_transport_startup_report(
            LocalUiRustTransportStartupStatus::Rejected,
            LocalUiRustTransportStartupReason::EmptyBindRejected,
            request.bind_host,
            false,
        );
    }

    let is_loopback = matches!(
        request.bind_host.as_str(),
        "127.0.0.1" | "localhost" | "::1"
    );
    if !is_loopback {
        return local_transport_startup_report(
            LocalUiRustTransportStartupStatus::Rejected,
            LocalUiRustTransportStartupReason::RemoteOrPublicBindRejected,
            request.bind_host,
            false,
        );
    }

    local_transport_startup_report(
        LocalUiRustTransportStartupStatus::Started,
        LocalUiRustTransportStartupReason::LocalOnlyLoopbackReady,
        request.bind_host,
        true,
    )
}

fn local_transport_startup_report(
    status: LocalUiRustTransportStartupStatus,
    reason: LocalUiRustTransportStartupReason,
    bind_host: String,
    local_only: bool,
) -> LocalUiRustTransportStartupReport {
    LocalUiRustTransportStartupReport {
        status,
        reason,
        bind_host,
        local_only,
        public_network_exposed: false,
        authenticated_remote_access: false,
        background_execution_enabled: false,
        provider_execution_enabled: false,
        persistence_enabled: false,
        replay_repair_enabled: false,
        recovery_promotion_enabled: false,
        action_execution_enabled: false,
        summary: "bounded local-only UI-to-Rust transport prototype; deterministic review communication only; non-authoritative and side-effect free".to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalUiRustTransportStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalUiRustTransportReason {
    ReviewStateReturned,
    DryRunReturned,
    WorkflowReviewEscalationReturned,
    ReviewInteractionPreviewReturned,
    MalformedInputRejected,
    OversizedInputRejected,
    NonLocalRequestRejected,
    UnsupportedOperationRejected,
    AuthorityBearingRequestRejected,
    ProviderExecutionRejected,
    PersistenceRejected,
    DurableAppendRejected,
    ExportRejected,
    ReplayRepairRejected,
    RecoveryPromotionRejected,
    ActionExecutionRejected,
    InvalidWorkflowReviewEscalationRejected,
    ReplayShapedPayloadRejected,
    DuplicateRequestIdentifierRejected,
    MalformedStructuredPayloadRejected,
    InvalidTypedFieldRejected,
    InvalidEnumRejected,
}

impl LocalUiRustTransportReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ReviewStateReturned => "review_state_returned",
            Self::DryRunReturned => "dry_run_returned",
            Self::WorkflowReviewEscalationReturned => "workflow_review_escalation_returned",
            Self::ReviewInteractionPreviewReturned => "review_interaction_preview_returned",
            Self::MalformedInputRejected => "malformed_input_rejected",
            Self::OversizedInputRejected => "oversized_input_rejected",
            Self::NonLocalRequestRejected => "non_local_request_rejected",
            Self::UnsupportedOperationRejected => "unsupported_operation_rejected",
            Self::AuthorityBearingRequestRejected => "authority_bearing_request_rejected",
            Self::ProviderExecutionRejected => "provider_execution_rejected",
            Self::PersistenceRejected => "persistence_rejected",
            Self::DurableAppendRejected => "durable_append_rejected",
            Self::ExportRejected => "export_rejected",
            Self::ReplayRepairRejected => "replay_repair_rejected",
            Self::RecoveryPromotionRejected => "recovery_promotion_rejected",
            Self::ActionExecutionRejected => "action_execution_rejected",
            Self::InvalidWorkflowReviewEscalationRejected => {
                "invalid_workflow_review_escalation_rejected"
            }
            Self::ReplayShapedPayloadRejected => "replay_shaped_payload_rejected",
            Self::DuplicateRequestIdentifierRejected => "duplicate_request_identifier_rejected",
            Self::MalformedStructuredPayloadRejected => "malformed_structured_payload_rejected",
            Self::InvalidTypedFieldRejected => "invalid_typed_field_rejected",
            Self::InvalidEnumRejected => "invalid_enum_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalUiRustTransportOperation {
    ReviewState,
    DryRun,
    WorkflowReviewEscalationQuery,
    ReviewInteraction,
    Unsupported,
    AuthorityBearing,
    ProviderExecution,
    Persistence,
    DurableAppend,
    Export,
    ReplayRepair,
    RecoveryPromotion,
    ActionExecution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalUiRustTransportRequest {
    pub request_id: String,
    pub operation: LocalUiRustTransportOperation,
    pub local_only: bool,
    pub workflow_state: String,
    pub review_state: String,
    pub escalation_state: String,
    pub payload_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalUiRustTransportResponse {
    pub transport_version: String,
    pub request_id: String,
    pub status: LocalUiRustTransportStatus,
    pub reason: LocalUiRustTransportReason,
    pub local_only: bool,
    pub non_authoritative: bool,
    pub deterministic: bool,
    pub provider_execution_enabled: bool,
    pub persistence_enabled: bool,
    pub durable_append_enabled: bool,
    pub export_enabled: bool,
    pub replay_repair_enabled: bool,
    pub recovery_promotion_enabled: bool,
    pub action_execution_enabled: bool,
    pub workflow_state: String,
    pub review_state: String,
    pub escalation_state: String,
    pub dry_run_summary: String,
    pub summary: String,
}

pub fn handle_local_ui_rust_transport_payload(payload: &str) -> LocalUiRustTransportResponse {
    if payload.len() > MAX_LOCAL_TRANSPORT_PAYLOAD_BYTES {
        return local_transport_rejection(
            "unknown",
            LocalUiRustTransportReason::OversizedInputRejected,
        );
    }

    let request = match parse_local_transport_payload(payload) {
        Ok(request) => request,
        Err(reason) => return local_transport_rejection("unknown", reason),
    };

    handle_local_ui_rust_transport_request(request)
}

pub fn handle_local_ui_rust_transport_request(
    request: LocalUiRustTransportRequest,
) -> LocalUiRustTransportResponse {
    if !request.local_only {
        return local_transport_rejection(
            &request.request_id,
            LocalUiRustTransportReason::NonLocalRequestRejected,
        );
    }

    match request.operation {
        LocalUiRustTransportOperation::ReviewState => local_transport_acceptance(
            &request,
            LocalUiRustTransportReason::ReviewStateReturned,
            "review state returned through local non-authoritative transport prototype",
        ),
        LocalUiRustTransportOperation::DryRun => local_transport_acceptance(
            &request,
            LocalUiRustTransportReason::DryRunReturned,
            "dry-run read-model data returned without provider, persistence, recovery, replay repair, or action execution",
        ),
        LocalUiRustTransportOperation::WorkflowReviewEscalationQuery => {
            if !valid_workflow_review_escalation(&request) {
                return local_transport_rejection(
                    &request.request_id,
                    LocalUiRustTransportReason::InvalidWorkflowReviewEscalationRejected,
                );
            }
            local_transport_acceptance(
                &request,
                LocalUiRustTransportReason::WorkflowReviewEscalationReturned,
                "workflow, review, and escalation state returned deterministically for operator review",
            )
        }
        LocalUiRustTransportOperation::ReviewInteraction => local_transport_acceptance(
            &request,
            LocalUiRustTransportReason::ReviewInteractionPreviewReturned,
            "bounded review interaction preview returned; no automatic workflow approval occurs",
        ),
        LocalUiRustTransportOperation::Unsupported => local_transport_rejection(
            &request.request_id,
            LocalUiRustTransportReason::UnsupportedOperationRejected,
        ),
        LocalUiRustTransportOperation::AuthorityBearing => local_transport_rejection(
            &request.request_id,
            LocalUiRustTransportReason::AuthorityBearingRequestRejected,
        ),
        LocalUiRustTransportOperation::ProviderExecution => local_transport_rejection(
            &request.request_id,
            LocalUiRustTransportReason::ProviderExecutionRejected,
        ),
        LocalUiRustTransportOperation::Persistence => local_transport_rejection(
            &request.request_id,
            LocalUiRustTransportReason::PersistenceRejected,
        ),
        LocalUiRustTransportOperation::DurableAppend => local_transport_rejection(
            &request.request_id,
            LocalUiRustTransportReason::DurableAppendRejected,
        ),
        LocalUiRustTransportOperation::Export => local_transport_rejection(
            &request.request_id,
            LocalUiRustTransportReason::ExportRejected,
        ),
        LocalUiRustTransportOperation::ReplayRepair => local_transport_rejection(
            &request.request_id,
            LocalUiRustTransportReason::ReplayRepairRejected,
        ),
        LocalUiRustTransportOperation::RecoveryPromotion => local_transport_rejection(
            &request.request_id,
            LocalUiRustTransportReason::RecoveryPromotionRejected,
        ),
        LocalUiRustTransportOperation::ActionExecution => local_transport_rejection(
            &request.request_id,
            LocalUiRustTransportReason::ActionExecutionRejected,
        ),
    }
}

fn parse_local_transport_payload(
    payload: &str,
) -> Result<LocalUiRustTransportRequest, LocalUiRustTransportReason> {
    if payload.trim().is_empty() || payload.contains('\0') {
        return Err(LocalUiRustTransportReason::MalformedInputRejected);
    }

    let trimmed = payload.trim_start();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        return Err(LocalUiRustTransportReason::MalformedStructuredPayloadRejected);
    }

    let mut fields = BTreeMap::new();
    for line in payload.lines() {
        let Some((key, value)) = line.split_once('=') else {
            return Err(LocalUiRustTransportReason::MalformedInputRejected);
        };
        if key.trim().is_empty() || key != key.trim() || value != value.trim() {
            return Err(LocalUiRustTransportReason::MalformedInputRejected);
        }
        if fields.contains_key(key) {
            if key == "request_id" {
                return Err(LocalUiRustTransportReason::DuplicateRequestIdentifierRejected);
            }
            return Err(LocalUiRustTransportReason::MalformedInputRejected);
        }
        fields.insert(key.to_string(), value.to_string());
    }

    if contains_authority_bearing_field(&fields) {
        return Err(LocalUiRustTransportReason::AuthorityBearingRequestRejected);
    }
    if contains_replay_shaped_field(&fields) {
        return Err(LocalUiRustTransportReason::ReplayShapedPayloadRejected);
    }

    let request_id = required_field(&fields, "request_id")?;
    let operation_text = required_field(&fields, "operation")?;
    let operation = parse_operation(operation_text.as_str());
    if matches!(operation, LocalUiRustTransportOperation::Unsupported)
        && !matches!(operation_text.as_str(), "unsupported")
    {
        return Err(LocalUiRustTransportReason::InvalidEnumRejected);
    }
    let local_only = match required_field(&fields, "local_only")?.as_str() {
        "true" => true,
        "false" => false,
        _ => return Err(LocalUiRustTransportReason::InvalidTypedFieldRejected),
    };
    let workflow_state = required_field(&fields, "workflow_state")?;
    let review_state = required_field(&fields, "review_state")?;
    let escalation_state = required_field(&fields, "escalation_state")?;
    let payload_summary = fields
        .get("payload_summary")
        .cloned()
        .unwrap_or_else(|| "none".to_string());

    if request_id.is_empty()
        || workflow_state.is_empty()
        || review_state.is_empty()
        || escalation_state.is_empty()
        || payload_summary.is_empty()
    {
        return Err(LocalUiRustTransportReason::MalformedInputRejected);
    }

    if !is_safe_transport_token(&request_id)
        || !is_safe_transport_token(&workflow_state)
        || !is_safe_transport_token(&review_state)
        || !is_safe_transport_token(&escalation_state)
        || !is_safe_payload_summary(&payload_summary)
    {
        return Err(LocalUiRustTransportReason::InvalidTypedFieldRejected);
    }

    Ok(LocalUiRustTransportRequest {
        request_id,
        operation,
        local_only,
        workflow_state,
        review_state,
        escalation_state,
        payload_summary,
    })
}

fn contains_authority_bearing_field(fields: &BTreeMap<String, String>) -> bool {
    fields.iter().any(|(key, value)| {
        matches!(
            key.as_str(),
            "authority"
                | "authorization"
                | "admin"
                | "token"
                | "secret"
                | "credential"
                | "execute_provider_adapter"
                | "write_durable_append_transaction"
                | "write_local_export_bundle"
                | "execute_local_persistence_plan"
                | "accept_recovery_candidate_for_in_memory_use"
                | "action_executed"
        ) || matches!(
            value.as_str(),
            "authority"
                | "authority_escalation"
                | "auto_approve"
                | "admin"
                | "root"
                | "execute_provider_adapter"
                | "write_durable_append_transaction"
                | "write_local_export_bundle"
                | "execute_local_persistence_plan"
                | "accept_recovery_candidate_for_in_memory_use"
        )
    })
}

fn contains_replay_shaped_field(fields: &BTreeMap<String, String>) -> bool {
    fields.iter().any(|(key, value)| {
        matches!(
            key.as_str(),
            "replay_id"
                | "replay_nonce"
                | "previous_request_id"
                | "request_sequence"
                | "recorded_response"
                | "replay_checksum"
                | "replay_repaired"
        ) || matches!(
            value.as_str(),
            "replay" | "replay_repaired" | "recorded_response"
        )
    })
}

fn is_safe_transport_token(value: &str) -> bool {
    value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

fn is_safe_payload_summary(value: &str) -> bool {
    value
        .chars()
        .all(|character| character.is_ascii_graphic() || character == ' ')
}

fn required_field(
    fields: &BTreeMap<String, String>,
    key: &str,
) -> Result<String, LocalUiRustTransportReason> {
    fields
        .get(key)
        .cloned()
        .ok_or(LocalUiRustTransportReason::MalformedInputRejected)
}

fn parse_operation(operation: &str) -> LocalUiRustTransportOperation {
    match operation {
        "review_state" => LocalUiRustTransportOperation::ReviewState,
        "dry_run" => LocalUiRustTransportOperation::DryRun,
        "workflow_review_escalation_query" => {
            LocalUiRustTransportOperation::WorkflowReviewEscalationQuery
        }
        "review_interaction" => LocalUiRustTransportOperation::ReviewInteraction,
        "authority" | "authority_escalation" | "auto_approve" => {
            LocalUiRustTransportOperation::AuthorityBearing
        }
        "provider_execute" | "provider_execution" | "model_execute" => {
            LocalUiRustTransportOperation::ProviderExecution
        }
        "persist" | "persistence" | "persistence_write" => {
            LocalUiRustTransportOperation::Persistence
        }
        "durable_append" | "append_ledger" | "append_audit" => {
            LocalUiRustTransportOperation::DurableAppend
        }
        "export" | "export_write" => LocalUiRustTransportOperation::Export,
        "replay_repair" | "repair_replay" => LocalUiRustTransportOperation::ReplayRepair,
        "recovery_promote" | "promote_recovery" => LocalUiRustTransportOperation::RecoveryPromotion,
        "action_execute" | "execute_action" => LocalUiRustTransportOperation::ActionExecution,
        _ => LocalUiRustTransportOperation::Unsupported,
    }
}

fn valid_workflow_review_escalation(request: &LocalUiRustTransportRequest) -> bool {
    matches!(
        request.workflow_state.as_str(),
        "idle" | "draft" | "review" | "blocked" | "completed"
    ) && matches!(
        request.review_state.as_str(),
        "not_started" | "pending" | "in_review" | "accepted_for_review" | "rejected"
    ) && matches!(
        request.escalation_state.as_str(),
        "none" | "operator_required" | "blocked" | "manual_review_required"
    )
}

fn local_transport_acceptance(
    request: &LocalUiRustTransportRequest,
    reason: LocalUiRustTransportReason,
    summary: &str,
) -> LocalUiRustTransportResponse {
    local_transport_response(
        request.request_id.clone(),
        LocalUiRustTransportStatus::Accepted,
        reason,
        true,
        request.workflow_state.clone(),
        request.review_state.clone(),
        request.escalation_state.clone(),
        summary.to_string(),
    )
}

fn local_transport_rejection(
    request_id: &str,
    reason: LocalUiRustTransportReason,
) -> LocalUiRustTransportResponse {
    local_transport_response(
        request_id.to_string(),
        LocalUiRustTransportStatus::Rejected,
        reason,
        true,
        "blocked".to_string(),
        "rejected".to_string(),
        "manual_review_required".to_string(),
        format!("request rejected fail-closed: {}", reason.code()),
    )
}

#[allow(clippy::too_many_arguments)]
fn local_transport_response(
    request_id: String,
    status: LocalUiRustTransportStatus,
    reason: LocalUiRustTransportReason,
    local_only: bool,
    workflow_state: String,
    review_state: String,
    escalation_state: String,
    summary: String,
) -> LocalUiRustTransportResponse {
    LocalUiRustTransportResponse {
        transport_version: LOCAL_TRANSPORT_VERSION.to_string(),
        request_id,
        status,
        reason,
        local_only,
        non_authoritative: true,
        deterministic: true,
        provider_execution_enabled: false,
        persistence_enabled: false,
        durable_append_enabled: false,
        export_enabled: false,
        replay_repair_enabled: false,
        recovery_promotion_enabled: false,
        action_execution_enabled: false,
        workflow_state,
        review_state,
        escalation_state,
        dry_run_summary: "dry-run transport is deterministic and side-effect free".to_string(),
        summary,
    }
}
