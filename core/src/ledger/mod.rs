#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerEventType {
    StateTransition,
    MemoryWrite,
    MemoryDelete,
    ExecutionStart,
    ExecutionEnd,
    ValidationPass,
    ValidationFail,
    PolicyPass,
    PolicyFail,
    ContextCreated,
    CandidateReceived,
    IntentAccepted,
    IntentRejected,
    ReplayRequested,
    AuditExported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEvent {
    pub revision: u64,
    pub event_type: LedgerEventType,
}
