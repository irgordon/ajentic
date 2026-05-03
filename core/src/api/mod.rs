#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiSurface {
    Cli,
    Http,
    Ipc,
    EventStream,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorIntentType {
    Approve,
    Reject,
    Retry,
    MemoryWrite,
    MemoryDelete,
    RunStart,
    RunStop,
    ToolRequest,
    PolicyCheck,
    StateTransitionRequest,
    ContextRebuildRequest,
    ReplayRequest,
    MemorySnapshotRequest,
    MemoryDisableRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorIntent {
    pub intent_type: OperatorIntentType,
    pub reason: String,
}

impl OperatorIntent {
    pub fn new(intent_type: OperatorIntentType, reason: impl Into<String>) -> Self {
        Self {
            intent_type,
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorRoute {
    Approval,
    Rejection,
    Retry,
    MemoryWrite,
    MemoryDelete,
    RunStart,
    RunStop,
    ToolRequest,
    PolicyCheck,
    StateTransitionRequest,
    ContextRebuildRequest,
    ReplayRequest,
    MemorySnapshotRequest,
    MemoryDisableRequest,
}

impl OperatorRoute {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Approval => "approval",
            Self::Rejection => "rejection",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorRouteResult {
    pub route: OperatorRoute,
    pub reason: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorRouteError {
    EmptyIntentReason,
}
impl OperatorRouteError {
    pub fn code(&self) -> &'static str {
        "empty_intent_reason"
    }
}

pub fn route_operator_intent(
    intent: &OperatorIntent,
) -> Result<OperatorRouteResult, OperatorRouteError> {
    if intent.reason.is_empty() {
        return Err(OperatorRouteError::EmptyIntentReason);
    }
    let route = match intent.intent_type {
        OperatorIntentType::Approve => OperatorRoute::Approval,
        OperatorIntentType::Reject => OperatorRoute::Rejection,
        OperatorIntentType::Retry => OperatorRoute::Retry,
        OperatorIntentType::MemoryWrite => OperatorRoute::MemoryWrite,
        OperatorIntentType::MemoryDelete => OperatorRoute::MemoryDelete,
        OperatorIntentType::RunStart => OperatorRoute::RunStart,
        OperatorIntentType::RunStop => OperatorRoute::RunStop,
        OperatorIntentType::ToolRequest => OperatorRoute::ToolRequest,
        OperatorIntentType::PolicyCheck => OperatorRoute::PolicyCheck,
        OperatorIntentType::StateTransitionRequest => OperatorRoute::StateTransitionRequest,
        OperatorIntentType::ContextRebuildRequest => OperatorRoute::ContextRebuildRequest,
        OperatorIntentType::ReplayRequest => OperatorRoute::ReplayRequest,
        OperatorIntentType::MemorySnapshotRequest => OperatorRoute::MemorySnapshotRequest,
        OperatorIntentType::MemoryDisableRequest => OperatorRoute::MemoryDisableRequest,
    };
    Ok(OperatorRouteResult {
        route,
        reason: "operator_intent_routed",
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationSourceKind {
    LocalLlm,
    Ide,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationOutputStatus {
    Received,
    Rejected,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationTrust {
    Untrusted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationRequest {
    pub id: String,
    pub source_kind: IntegrationSourceKind,
    pub prompt_summary: String,
    pub operator_context_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationOutput {
    pub id: String,
    pub request_id: String,
    pub source_kind: IntegrationSourceKind,
    pub content: String,
    pub status: IntegrationOutputStatus,
    pub trust: IntegrationTrust,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationBoundaryError {
    EmptyRequestId,
    EmptyPromptSummary,
    EmptyOperatorContextSummary,
    EmptyOutputId,
    EmptyOutputRequestId,
    EmptyOutputContent,
}
impl IntegrationBoundaryError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyRequestId => "empty_request_id",
            Self::EmptyPromptSummary => "empty_prompt_summary",
            Self::EmptyOperatorContextSummary => "empty_operator_context_summary",
            Self::EmptyOutputId => "empty_output_id",
            Self::EmptyOutputRequestId => "empty_output_request_id",
            Self::EmptyOutputContent => "empty_output_content",
        }
    }
}

impl IntegrationRequest {
    pub fn new(
        id: impl Into<String>,
        source_kind: IntegrationSourceKind,
        prompt_summary: impl Into<String>,
        operator_context_summary: impl Into<String>,
    ) -> Result<Self, IntegrationBoundaryError> {
        let id = id.into();
        if id.is_empty() {
            return Err(IntegrationBoundaryError::EmptyRequestId);
        }
        let prompt_summary = prompt_summary.into();
        if prompt_summary.is_empty() {
            return Err(IntegrationBoundaryError::EmptyPromptSummary);
        }
        let operator_context_summary = operator_context_summary.into();
        if operator_context_summary.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOperatorContextSummary);
        }
        Ok(Self {
            id,
            source_kind,
            prompt_summary,
            operator_context_summary,
        })
    }
}

impl IntegrationOutput {
    pub fn new_untrusted(
        id: impl Into<String>,
        request_id: impl Into<String>,
        source_kind: IntegrationSourceKind,
        content: impl Into<String>,
        status: IntegrationOutputStatus,
    ) -> Result<Self, IntegrationBoundaryError> {
        let id = id.into();
        if id.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOutputId);
        }
        let request_id = request_id.into();
        if request_id.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOutputRequestId);
        }
        let content = content.into();
        if content.is_empty() {
            return Err(IntegrationBoundaryError::EmptyOutputContent);
        }
        Ok(Self {
            id,
            request_id,
            source_kind,
            content,
            status,
            trust: IntegrationTrust::Untrusted,
        })
    }
}

pub fn integration_source_to_provider_kind(
    source_kind: IntegrationSourceKind,
) -> crate::execution::ProviderKind {
    match source_kind {
        IntegrationSourceKind::LocalLlm => crate::execution::ProviderKind::Local,
        IntegrationSourceKind::Ide => crate::execution::ProviderKind::Ide,
        IntegrationSourceKind::Unknown => crate::execution::ProviderKind::Unknown,
    }
}

pub fn integration_request_to_provider_request(
    request: &IntegrationRequest,
) -> Result<crate::execution::ProviderRequest, crate::execution::ProviderBoundaryError> {
    crate::execution::ProviderRequest::new(
        request.id.clone(),
        integration_source_to_provider_kind(request.source_kind),
        request.prompt_summary.clone(),
    )
}

pub fn integration_output_to_provider_output(
    output: &IntegrationOutput,
) -> Result<crate::execution::ProviderOutput, crate::execution::ProviderBoundaryError> {
    let status = match output.status {
        IntegrationOutputStatus::Received => crate::execution::ProviderOutputStatus::Received,
        IntegrationOutputStatus::Rejected => crate::execution::ProviderOutputStatus::Rejected,
        IntegrationOutputStatus::Unknown => crate::execution::ProviderOutputStatus::Unknown,
    };
    crate::execution::ProviderOutput::new_untrusted(
        output.id.clone(),
        output.request_id.clone(),
        integration_source_to_provider_kind(output.source_kind),
        output.content.clone(),
        status,
    )
}

pub fn integration_output_is_authoritative(_output: &IntegrationOutput) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_request_requires_id() {
        assert_eq!(
            IntegrationRequest::new("", IntegrationSourceKind::LocalLlm, "p", "c"),
            Err(IntegrationBoundaryError::EmptyRequestId)
        );
    }
    #[test]
    fn integration_request_requires_prompt_summary() {
        assert_eq!(
            IntegrationRequest::new("id", IntegrationSourceKind::LocalLlm, "", "c"),
            Err(IntegrationBoundaryError::EmptyPromptSummary)
        );
    }
    #[test]
    fn integration_request_requires_operator_context_summary() {
        assert_eq!(
            IntegrationRequest::new("id", IntegrationSourceKind::LocalLlm, "p", ""),
            Err(IntegrationBoundaryError::EmptyOperatorContextSummary)
        );
    }
    #[test]
    fn integration_output_requires_id() {
        assert_eq!(
            IntegrationOutput::new_untrusted(
                "",
                "req",
                IntegrationSourceKind::Ide,
                "c",
                IntegrationOutputStatus::Received
            ),
            Err(IntegrationBoundaryError::EmptyOutputId)
        );
    }
    #[test]
    fn integration_output_requires_request_id() {
        assert_eq!(
            IntegrationOutput::new_untrusted(
                "id",
                "",
                IntegrationSourceKind::Ide,
                "c",
                IntegrationOutputStatus::Received
            ),
            Err(IntegrationBoundaryError::EmptyOutputRequestId)
        );
    }
    #[test]
    fn integration_output_requires_content() {
        assert_eq!(
            IntegrationOutput::new_untrusted(
                "id",
                "req",
                IntegrationSourceKind::Ide,
                "",
                IntegrationOutputStatus::Received
            ),
            Err(IntegrationBoundaryError::EmptyOutputContent)
        );
    }
    #[test]
    fn integration_output_is_always_untrusted() {
        let o = IntegrationOutput::new_untrusted(
            "id",
            "req",
            IntegrationSourceKind::LocalLlm,
            "x",
            IntegrationOutputStatus::Received,
        )
        .unwrap();
        assert_eq!(o.trust, IntegrationTrust::Untrusted);
    }
    #[test]
    fn integration_output_is_not_authoritative() {
        for source in [
            IntegrationSourceKind::LocalLlm,
            IntegrationSourceKind::Ide,
            IntegrationSourceKind::Unknown,
        ] {
            let o = IntegrationOutput::new_untrusted(
                "id",
                "req",
                source,
                "x",
                IntegrationOutputStatus::Received,
            )
            .unwrap();
            assert!(!integration_output_is_authoritative(&o));
        }
    }
    #[test]
    fn integration_boundary_error_codes_are_stable() {
        assert_eq!(
            IntegrationBoundaryError::EmptyRequestId.code(),
            "empty_request_id"
        );
        assert_eq!(
            IntegrationBoundaryError::EmptyPromptSummary.code(),
            "empty_prompt_summary"
        );
        assert_eq!(
            IntegrationBoundaryError::EmptyOperatorContextSummary.code(),
            "empty_operator_context_summary"
        );
        assert_eq!(
            IntegrationBoundaryError::EmptyOutputId.code(),
            "empty_output_id"
        );
        assert_eq!(
            IntegrationBoundaryError::EmptyOutputRequestId.code(),
            "empty_output_request_id"
        );
        assert_eq!(
            IntegrationBoundaryError::EmptyOutputContent.code(),
            "empty_output_content"
        );
    }
    #[test]
    fn integration_source_maps_local_llm_to_local_provider() {
        assert_eq!(
            integration_source_to_provider_kind(IntegrationSourceKind::LocalLlm),
            crate::execution::ProviderKind::Local
        );
    }
    #[test]
    fn integration_source_maps_ide_to_ide_provider() {
        assert_eq!(
            integration_source_to_provider_kind(IntegrationSourceKind::Ide),
            crate::execution::ProviderKind::Ide
        );
    }
    #[test]
    fn integration_source_maps_unknown_to_unknown_provider() {
        assert_eq!(
            integration_source_to_provider_kind(IntegrationSourceKind::Unknown),
            crate::execution::ProviderKind::Unknown
        );
    }
    #[test]
    fn integration_request_maps_to_provider_request() {
        let r =
            IntegrationRequest::new("id", IntegrationSourceKind::Ide, "prompt", "context").unwrap();
        let p = integration_request_to_provider_request(&r).unwrap();
        assert_eq!(p.id, "id");
        assert_eq!(p.provider_kind, crate::execution::ProviderKind::Ide);
        assert_eq!(p.prompt_summary, "prompt");
        assert_eq!(r.operator_context_summary, "context");
    }
    #[test]
    fn integration_output_maps_to_untrusted_provider_output() {
        let o = IntegrationOutput::new_untrusted(
            "out",
            "req",
            IntegrationSourceKind::LocalLlm,
            "approved validated safe execute promote",
            IntegrationOutputStatus::Received,
        )
        .unwrap();
        let p = integration_output_to_provider_output(&o).unwrap();
        assert_eq!(p.trust, crate::execution::ProviderOutputTrust::Untrusted);
        assert_eq!(p.provider_kind, crate::execution::ProviderKind::Local);
    }
    #[test]
    fn integration_output_status_maps_to_provider_output_status() {
        for (i, e) in [
            (
                IntegrationOutputStatus::Received,
                crate::execution::ProviderOutputStatus::Received,
            ),
            (
                IntegrationOutputStatus::Rejected,
                crate::execution::ProviderOutputStatus::Rejected,
            ),
            (
                IntegrationOutputStatus::Unknown,
                crate::execution::ProviderOutputStatus::Unknown,
            ),
        ] {
            let o = IntegrationOutput::new_untrusted(
                "id",
                "req",
                IntegrationSourceKind::Unknown,
                "x",
                i,
            )
            .unwrap();
            assert_eq!(integration_output_to_provider_output(&o).unwrap().status, e);
        }
    }
    #[test]
    fn integration_output_content_does_not_infer_validation() {
        let o = IntegrationOutput::new_untrusted(
            "id",
            "req",
            IntegrationSourceKind::Ide,
            "validated approved",
            IntegrationOutputStatus::Unknown,
        )
        .unwrap();
        let p = integration_output_to_provider_output(&o).unwrap();
        assert_eq!(p.trust, crate::execution::ProviderOutputTrust::Untrusted);
        assert_eq!(p.status, crate::execution::ProviderOutputStatus::Unknown);
    }
    #[test]
    fn integration_output_content_does_not_infer_policy() {
        let o = IntegrationOutput::new_untrusted(
            "id",
            "req",
            IntegrationSourceKind::Ide,
            "safe policy allow",
            IntegrationOutputStatus::Rejected,
        )
        .unwrap();
        let p = integration_output_to_provider_output(&o).unwrap();
        assert_eq!(p.trust, crate::execution::ProviderOutputTrust::Untrusted);
        assert_eq!(p.status, crate::execution::ProviderOutputStatus::Rejected);
    }
    #[test]
    fn integration_output_content_does_not_infer_execution() {
        let o = IntegrationOutput::new_untrusted(
            "id",
            "req",
            IntegrationSourceKind::Ide,
            "execute now promote",
            IntegrationOutputStatus::Received,
        )
        .unwrap();
        let p = integration_output_to_provider_output(&o).unwrap();
        assert_eq!(p.trust, crate::execution::ProviderOutputTrust::Untrusted);
        assert_eq!(p.status, crate::execution::ProviderOutputStatus::Received);
    }
    #[test]
    fn integration_mapping_does_not_call_controlled_flow() {
        let r = IntegrationRequest::new("id", IntegrationSourceKind::LocalLlm, "p", "c").unwrap();
        let o = IntegrationOutput::new_untrusted(
            "out",
            "id",
            IntegrationSourceKind::LocalLlm,
            "content",
            IntegrationOutputStatus::Received,
        )
        .unwrap();
        assert!(integration_request_to_provider_request(&r).is_ok());
        assert!(integration_output_to_provider_output(&o).is_ok());
    }
}
