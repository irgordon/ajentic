use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadProjectionStatus {
    Ready,
    Blocked,
    Rejected,
    Unknown,
    NotAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadProjectionAuthority {
    Rust,
    Ui,
    Operator,
    Provider,
    Integration,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSafetyProjection {
    pub safety_level: RuntimeSafetyLevel,
    pub require_policy_pass: bool,
    pub require_validation_pass: bool,
    pub require_replay_verification: bool,
    pub allow_provider_network: bool,
    pub allow_file_io: bool,
    pub allow_ui_mutation: bool,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleReadProjection {
    pub lifecycle: crate::state::LifecycleState,
    pub revision: u64,
    pub status: ReadProjectionStatus,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunReadProjection {
    pub run_id: String,
    pub status: crate::execution::ControlledRunStatus,
    pub reason: crate::execution::ControlledRunReason,
    pub clean_output_available: bool,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderReadProjection {
    pub provider_kind: crate::execution::ProviderKind,
    pub output_status: crate::execution::ProviderOutputStatus,
    pub output_trust: crate::execution::ProviderOutputTrust,
    pub authoritative: bool,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationReadProjection {
    pub source_kind: IntegrationSourceKind,
    pub output_status: IntegrationOutputStatus,
    pub output_trust: IntegrationTrust,
    pub authoritative: bool,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerReadProjection {
    pub event_count: usize,
    pub last_revision: Option<u64>,
    pub status: ReadProjectionStatus,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReadProjection {
    pub readiness: crate::replay::ReplayReadiness,
    pub integrity: crate::replay::ReplayIntegrity,
    pub events_replayed: usize,
    pub status: ReadProjectionStatus,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditReadProjection {
    pub projection_count: usize,
    pub latest_summary: String,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextReadProjection {
    pub packet_id: String,
    pub slice_count: usize,
    pub source_count: usize,
    pub budget_used: usize,
    pub budget_max: usize,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryReadProjection {
    pub snapshot_id: String,
    pub active_entries: usize,
    pub disabled_entries: usize,
    pub rejected_entries: usize,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputReadProjection {
    pub clean_output_available: bool,
    pub raw_output_trusted: bool,
    pub clean_output_summary: Option<String>,
    pub authority: ReadProjectionAuthority,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationReadProjection {
    pub projection_id: String,
    pub runtime_config_id: String,
    pub safety: RuntimeSafetyProjection,
    pub lifecycle: LifecycleReadProjection,
    pub run: RunReadProjection,
    pub provider: ProviderReadProjection,
    pub integration: IntegrationReadProjection,
    pub ledger: LedgerReadProjection,
    pub replay: ReplayReadProjection,
    pub audit: AuditReadProjection,
    pub context: ContextReadProjection,
    pub memory: MemoryReadProjection,
    pub output: OutputReadProjection,
}

#[cfg(test)]
mod diagnostic_tests {
    use super::*;

    #[test]
    fn read_projection_error_diagnostic_preserves_code() {
        let error = ReadProjectionError::EmptyProjectionId;
        let diagnostic = crate::api::read_projection_error_diagnostic(error);
        assert_eq!(diagnostic.code, error.code());
    }
}
