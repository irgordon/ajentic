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
