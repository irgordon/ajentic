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
pub enum OperatorIntentIngressStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorIntentIngressReason {
    AcceptedForRouting,
    EmptySubmissionId,
    EmptyOperatorId,
    EmptyReason,
    EmptyTargetId,
    UnsupportedIntentType,
    RouteRejected,
}

impl OperatorIntentIngressReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AcceptedForRouting => "accepted_for_routing",
            Self::EmptySubmissionId => "empty_submission_id",
            Self::EmptyOperatorId => "empty_operator_id",
            Self::EmptyReason => "empty_reason",
            Self::EmptyTargetId => "empty_target_id",
            Self::UnsupportedIntentType => "unsupported_intent_type",
            Self::RouteRejected => "route_rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorIntentTargetKind {
    Run,
    Candidate,
    Replay,
    Context,
    Memory,
    Output,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorIntentSubmission {
    pub submission_id: String,
    pub operator_id: String,
    pub intent: OperatorIntent,
    pub target_kind: OperatorIntentTargetKind,
    pub target_id: String,
}

impl OperatorIntentSubmission {
    pub fn new(
        submission_id: impl Into<String>,
        operator_id: impl Into<String>,
        intent: OperatorIntent,
        target_kind: OperatorIntentTargetKind,
        target_id: impl Into<String>,
    ) -> Result<Self, OperatorIntentIngressReason> {
        let submission_id = submission_id.into();
        if submission_id.is_empty() {
            return Err(OperatorIntentIngressReason::EmptySubmissionId);
        }
        let operator_id = operator_id.into();
        if operator_id.is_empty() {
            return Err(OperatorIntentIngressReason::EmptyOperatorId);
        }
        if intent.reason.is_empty() {
            return Err(OperatorIntentIngressReason::EmptyReason);
        }
        let target_id = target_id.into();
        if target_id.is_empty() {
            return Err(OperatorIntentIngressReason::EmptyTargetId);
        }
        if target_kind == OperatorIntentTargetKind::Unknown {
            return Err(OperatorIntentIngressReason::UnsupportedIntentType);
        }
        Ok(Self {
            submission_id,
            operator_id,
            intent,
            target_kind,
            target_id,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorIntentIngressReport {
    pub status: OperatorIntentIngressStatus,
    pub reason: OperatorIntentIngressReason,
    pub route: Option<OperatorRoute>,
    pub submission_id: String,
    pub operator_id: String,
    pub target_kind: OperatorIntentTargetKind,
    pub target_id: String,
}

pub fn submit_operator_intent(submission: OperatorIntentSubmission) -> OperatorIntentIngressReport {
    match route_operator_intent(&submission.intent) {
        Ok(result) => OperatorIntentIngressReport {
            status: OperatorIntentIngressStatus::Accepted,
            reason: OperatorIntentIngressReason::AcceptedForRouting,
            route: Some(result.route),
            submission_id: submission.submission_id,
            operator_id: submission.operator_id,
            target_kind: submission.target_kind,
            target_id: submission.target_id,
        },
        Err(_) => OperatorIntentIngressReport {
            status: OperatorIntentIngressStatus::Rejected,
            reason: OperatorIntentIngressReason::RouteRejected,
            route: None,
            submission_id: submission.submission_id,
            operator_id: submission.operator_id,
            target_kind: submission.target_kind,
            target_id: submission.target_id,
        },
    }
}

pub fn operator_intent_ingress_executes_actions(_report: &OperatorIntentIngressReport) -> bool {
    false
}

