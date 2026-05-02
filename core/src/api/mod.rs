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
        match self {
            Self::EmptyIntentReason => "empty_intent_reason",
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_route_codes_are_stable() {
        assert_eq!(OperatorRoute::Approval.code(), "approval");
        assert_eq!(OperatorRoute::Rejection.code(), "rejection");
        assert_eq!(OperatorRoute::Retry.code(), "retry");
        assert_eq!(OperatorRoute::MemoryWrite.code(), "memory_write");
        assert_eq!(OperatorRoute::MemoryDelete.code(), "memory_delete");
        assert_eq!(OperatorRoute::RunStart.code(), "run_start");
        assert_eq!(OperatorRoute::RunStop.code(), "run_stop");
        assert_eq!(OperatorRoute::ToolRequest.code(), "tool_request");
        assert_eq!(OperatorRoute::PolicyCheck.code(), "policy_check");
        assert_eq!(
            OperatorRoute::StateTransitionRequest.code(),
            "state_transition_request"
        );
        assert_eq!(
            OperatorRoute::ContextRebuildRequest.code(),
            "context_rebuild_request"
        );
        assert_eq!(OperatorRoute::ReplayRequest.code(), "replay_request");
        assert_eq!(
            OperatorRoute::MemorySnapshotRequest.code(),
            "memory_snapshot_request"
        );
        assert_eq!(
            OperatorRoute::MemoryDisableRequest.code(),
            "memory_disable_request"
        );
    }

    #[test]
    fn operator_route_error_codes_are_stable() {
        assert_eq!(
            OperatorRouteError::EmptyIntentReason.code(),
            "empty_intent_reason"
        );
    }

    #[test]
    fn operator_intent_constructor_preserves_type_and_reason() {
        let intent = OperatorIntent::new(OperatorIntentType::PolicyCheck, "review_policy");

        assert_eq!(intent.intent_type, OperatorIntentType::PolicyCheck);
        assert_eq!(intent.reason, "review_policy");
    }

    #[test]
    fn route_operator_intent_rejects_empty_reason() {
        let intent = OperatorIntent::new(OperatorIntentType::Approve, "");

        assert_eq!(
            route_operator_intent(&intent),
            Err(OperatorRouteError::EmptyIntentReason)
        );
    }

    fn assert_route(intent_type: OperatorIntentType, expected_route: OperatorRoute) {
        let intent = OperatorIntent::new(intent_type, "any_non_empty_reason");

        let result = route_operator_intent(&intent).expect("route should succeed");

        assert_eq!(result.route, expected_route);
        assert_eq!(result.reason, "operator_intent_routed");
    }

    #[test]
    fn route_approve_intent() {
        assert_route(OperatorIntentType::Approve, OperatorRoute::Approval);
    }

    #[test]
    fn route_reject_intent() {
        assert_route(OperatorIntentType::Reject, OperatorRoute::Rejection);
    }

    #[test]
    fn route_retry_intent() {
        assert_route(OperatorIntentType::Retry, OperatorRoute::Retry);
    }

    #[test]
    fn route_memory_write_intent() {
        assert_route(OperatorIntentType::MemoryWrite, OperatorRoute::MemoryWrite);
    }

    #[test]
    fn route_memory_delete_intent() {
        assert_route(
            OperatorIntentType::MemoryDelete,
            OperatorRoute::MemoryDelete,
        );
    }

    #[test]
    fn route_run_start_intent() {
        assert_route(OperatorIntentType::RunStart, OperatorRoute::RunStart);
    }

    #[test]
    fn route_run_stop_intent() {
        assert_route(OperatorIntentType::RunStop, OperatorRoute::RunStop);
    }

    #[test]
    fn route_tool_request_intent() {
        assert_route(OperatorIntentType::ToolRequest, OperatorRoute::ToolRequest);
    }

    #[test]
    fn route_policy_check_intent() {
        assert_route(OperatorIntentType::PolicyCheck, OperatorRoute::PolicyCheck);
    }

    #[test]
    fn route_state_transition_request_intent() {
        assert_route(
            OperatorIntentType::StateTransitionRequest,
            OperatorRoute::StateTransitionRequest,
        );
    }

    #[test]
    fn route_context_rebuild_request_intent() {
        assert_route(
            OperatorIntentType::ContextRebuildRequest,
            OperatorRoute::ContextRebuildRequest,
        );
    }

    #[test]
    fn route_replay_request_intent() {
        assert_route(
            OperatorIntentType::ReplayRequest,
            OperatorRoute::ReplayRequest,
        );
    }

    #[test]
    fn route_memory_snapshot_request_intent() {
        assert_route(
            OperatorIntentType::MemorySnapshotRequest,
            OperatorRoute::MemorySnapshotRequest,
        );
    }

    #[test]
    fn route_memory_disable_request_intent() {
        assert_route(
            OperatorIntentType::MemoryDisableRequest,
            OperatorRoute::MemoryDisableRequest,
        );
    }

    #[test]
    fn route_does_not_depend_on_reason_content() {
        let intent_a = OperatorIntent::new(OperatorIntentType::ReplayRequest, "reason_a");
        let intent_b = OperatorIntent::new(OperatorIntentType::ReplayRequest, "reason_b");

        let result_a = route_operator_intent(&intent_a).expect("route should succeed");
        let result_b = route_operator_intent(&intent_b).expect("route should succeed");

        assert_eq!(result_a.route, OperatorRoute::ReplayRequest);
        assert_eq!(result_b.route, OperatorRoute::ReplayRequest);
        assert_eq!(result_a.reason, "operator_intent_routed");
        assert_eq!(result_b.reason, "operator_intent_routed");
    }
}
