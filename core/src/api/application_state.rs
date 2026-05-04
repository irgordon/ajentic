use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationStateError {
    EmptyStateId,
    EmptyProjectionId,
    EmptyRunId,
    EmptyContextPacketId,
    EmptyMemorySnapshotId,
    UnsafeRuntimeConfig,
    ProjectionFailed,
}

impl ApplicationStateError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyStateId => "empty_state_id",
            Self::EmptyProjectionId => "empty_projection_id",
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyContextPacketId => "empty_context_packet_id",
            Self::EmptyMemorySnapshotId => "empty_memory_snapshot_id",
            Self::UnsafeRuntimeConfig => "unsafe_runtime_config",
            Self::ProjectionFailed => "projection_failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationContextMetadata {
    pub packet_id: String,
    pub slice_count: usize,
    pub source_count: usize,
    pub budget_used: usize,
    pub budget_max: usize,
}

impl ApplicationContextMetadata {
    pub fn new(
        packet_id: impl Into<String>,
        slice_count: usize,
        source_count: usize,
        budget_used: usize,
        budget_max: usize,
    ) -> Result<Self, ApplicationStateError> {
        let packet_id = packet_id.into();
        if packet_id.is_empty() {
            return Err(ApplicationStateError::EmptyContextPacketId);
        }
        Ok(Self {
            packet_id,
            slice_count,
            source_count,
            budget_used,
            budget_max,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationMemoryMetadata {
    pub snapshot_id: String,
    pub active_entries: usize,
    pub disabled_entries: usize,
    pub rejected_entries: usize,
}

impl ApplicationMemoryMetadata {
    pub fn new(
        snapshot_id: impl Into<String>,
        active_entries: usize,
        disabled_entries: usize,
        rejected_entries: usize,
    ) -> Result<Self, ApplicationStateError> {
        let snapshot_id = snapshot_id.into();
        if snapshot_id.is_empty() {
            return Err(ApplicationStateError::EmptyMemorySnapshotId);
        }
        Ok(Self {
            snapshot_id,
            active_entries,
            disabled_entries,
            rejected_entries,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalApplicationState {
    pub state_id: String,
    pub projection_id: String,
    pub run_id: String,
    pub runtime_config: LocalRuntimeConfig,
    pub harness_state: crate::state::HarnessState,
    pub controlled_run: crate::execution::ControlledRunResult,
    pub provider_output: crate::execution::ProviderOutput,
    pub integration_output: IntegrationOutput,
    pub ledger: crate::ledger::Ledger,
    pub replay_report: crate::replay::ReplayReport,
    pub audit_projections: Vec<crate::audit::AuditProjection>,
    pub context: ApplicationContextMetadata,
    pub memory: ApplicationMemoryMetadata,
}

impl LocalApplicationState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        state_id: impl Into<String>,
        projection_id: impl Into<String>,
        run_id: impl Into<String>,
        runtime_config: LocalRuntimeConfig,
        harness_state: crate::state::HarnessState,
        controlled_run: crate::execution::ControlledRunResult,
        provider_output: crate::execution::ProviderOutput,
        integration_output: IntegrationOutput,
        ledger: crate::ledger::Ledger,
        replay_report: crate::replay::ReplayReport,
        audit_projections: Vec<crate::audit::AuditProjection>,
        context: ApplicationContextMetadata,
        memory: ApplicationMemoryMetadata,
    ) -> Result<Self, ApplicationStateError> {
        let state_id = state_id.into();
        if state_id.is_empty() {
            return Err(ApplicationStateError::EmptyStateId);
        }
        let projection_id = projection_id.into();
        if projection_id.is_empty() {
            return Err(ApplicationStateError::EmptyProjectionId);
        }
        let run_id = run_id.into();
        if run_id.is_empty() {
            return Err(ApplicationStateError::EmptyRunId);
        }
        if local_runtime_config_allows_authority_bypass(&runtime_config) {
            return Err(ApplicationStateError::UnsafeRuntimeConfig);
        }
        Ok(Self {
            state_id,
            projection_id,
            run_id,
            runtime_config,
            harness_state,
            controlled_run,
            provider_output,
            integration_output,
            ledger,
            replay_report,
            audit_projections,
            context,
            memory,
        })
    }

    pub fn state_id(&self) -> &str {
        &self.state_id
    }
    pub fn run_id(&self) -> &str {
        &self.run_id
    }
    pub fn ledger_event_count(&self) -> usize {
        self.ledger.events().len()
    }
    pub fn last_ledger_revision(&self) -> Option<u64> {
        self.ledger.last_revision()
    }

    pub fn derive_read_projection(
        &self,
    ) -> Result<ApplicationReadProjection, ApplicationStateError> {
        ApplicationReadProjection::new(
            self.projection_id.clone(),
            self.run_id.clone(),
            &self.runtime_config,
            &self.harness_state,
            &self.controlled_run,
            &self.provider_output,
            &self.integration_output,
            &self.ledger,
            &self.replay_report,
            &self.audit_projections,
            self.context.packet_id.clone(),
            self.context.slice_count,
            self.context.source_count,
            self.context.budget_used,
            self.context.budget_max,
            self.memory.snapshot_id.clone(),
            self.memory.active_entries,
            self.memory.disabled_entries,
            self.memory.rejected_entries,
        )
        .map_err(|error| match error {
            ReadProjectionError::UnsafeRuntimeConfig => ApplicationStateError::UnsafeRuntimeConfig,
            _ => ApplicationStateError::ProjectionFailed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsafe_runtime_config_cannot_initialize_state() {
        let mut defaults = RuntimeSafetyDefaults::strict();
        defaults.allow_ui_mutation = true;
        let cfg = LocalRuntimeConfig::new(
            "cfg",
            LocalRuntimeMode::DryRun,
            LocalProviderMode::Stub,
            RuntimeSafetyLevel::Strict,
            LocalWorkspaceMetadata::new("ws", "opaque://ws", "op").expect("valid"),
            defaults,
        );
        assert!(cfg.is_err());
    }
}

#[cfg(test)]
mod diagnostic_tests {
    use super::*;

    #[test]
    fn application_state_error_diagnostic_preserves_code() {
        let error = ApplicationStateError::EmptyStateId;
        let diagnostic = crate::api::application_state_error_diagnostic(error);
        assert_eq!(diagnostic.code, error.code());
    }
}
