#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextViewType {
    Code,
    Memory,
    Docs,
    Schema,
    Diff,
    Task,
    Example,
    System,
    ToolOutput,
    ModelOutput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruthDimension {
    Normative,
    Structural,
    Planned,
    Historical,
    Procedural,
    Executable,
    Contract,
    Data,
    Example,
    Orientation,
    Navigation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextBudgetUnit {
    Tokens,
    Bytes,
    Characters,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextError {
    EmptyPacketId,
    EmptyRunId,
    EmptyTaskId,
    EmptyTaskSummary,
    MissingSliceProvenance,
    EmptySliceId,
    EmptySliceContent,
    EmptySourcePath,
    MissingSources,
    BudgetExceeded,
    EmptyAssembledBy,
    EmptyAssemblyReason,
}

impl ContextError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyPacketId => "empty_packet_id",
            Self::EmptyRunId => "empty_run_id",
            Self::EmptyTaskId => "empty_task_id",
            Self::EmptyTaskSummary => "empty_task_summary",
            Self::MissingSliceProvenance => "missing_slice_provenance",
            Self::EmptySliceId => "empty_slice_id",
            Self::EmptySliceContent => "empty_slice_content",
            Self::EmptySourcePath => "empty_source_path",
            Self::MissingSources => "missing_sources",
            Self::BudgetExceeded => "budget_exceeded",
            Self::EmptyAssembledBy => "empty_assembled_by",
            Self::EmptyAssemblyReason => "empty_assembly_reason",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextProvenance {
    pub source: String,
    pub selected_by: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextSlice {
    pub id: String,
    pub view_type: ContextViewType,
    pub truth_dimension: TruthDimension,
    pub source_path: String,
    pub content: String,
    pub provenance: ContextProvenance,
}

impl ContextSlice {
    pub fn new(
        id: impl Into<String>,
        view_type: ContextViewType,
        truth_dimension: TruthDimension,
        source_path: impl Into<String>,
        content: impl Into<String>,
        provenance: ContextProvenance,
    ) -> Result<Self, ContextError> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(ContextError::EmptySliceId);
        }

        let source_path = source_path.into();
        if source_path.trim().is_empty() {
            return Err(ContextError::EmptySourcePath);
        }

        let content = content.into();
        if content.trim().is_empty() {
            return Err(ContextError::EmptySliceContent);
        }

        if provenance.source.trim().is_empty() || provenance.selected_by.trim().is_empty() {
            return Err(ContextError::MissingSliceProvenance);
        }

        Ok(Self {
            id,
            view_type,
            truth_dimension,
            source_path,
            content,
            provenance,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextSource {
    pub path: String,
    pub truth_dimension: TruthDimension,
}

impl ContextSource {
    pub fn new(
        path: impl Into<String>,
        truth_dimension: TruthDimension,
    ) -> Result<Self, ContextError> {
        let path = path.into();
        if path.trim().is_empty() {
            return Err(ContextError::EmptySourcePath);
        }

        Ok(Self {
            path,
            truth_dimension,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextBudget {
    pub max_units: u64,
    pub used_units: u64,
    pub unit: ContextBudgetUnit,
}

impl ContextBudget {
    pub fn new(
        max_units: u64,
        used_units: u64,
        unit: ContextBudgetUnit,
    ) -> Result<Self, ContextError> {
        if used_units > max_units {
            return Err(ContextError::BudgetExceeded);
        }

        Ok(Self {
            max_units,
            used_units,
            unit,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextTask {
    pub id: String,
    pub summary: String,
}

impl ContextTask {
    pub fn new(id: impl Into<String>, summary: impl Into<String>) -> Result<Self, ContextError> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(ContextError::EmptyTaskId);
        }

        let summary = summary.into();
        if summary.trim().is_empty() {
            return Err(ContextError::EmptyTaskSummary);
        }

        Ok(Self { id, summary })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextPacketProvenance {
    pub assembled_by: String,
    pub assembly_reason: String,
}

impl ContextPacketProvenance {
    pub fn new(
        assembled_by: impl Into<String>,
        assembly_reason: impl Into<String>,
    ) -> Result<Self, ContextError> {
        let assembled_by = assembled_by.into();
        if assembled_by.trim().is_empty() {
            return Err(ContextError::EmptyAssembledBy);
        }

        let assembly_reason = assembly_reason.into();
        if assembly_reason.trim().is_empty() {
            return Err(ContextError::EmptyAssemblyReason);
        }

        Ok(Self {
            assembled_by,
            assembly_reason,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextPacket {
    pub id: String,
    pub run_id: String,
    pub task: ContextTask,
    pub slices: Vec<ContextSlice>,
    pub budget: ContextBudget,
    pub sources: Vec<ContextSource>,
    pub provenance: ContextPacketProvenance,
}

impl ContextPacket {
    pub fn assemble(
        id: impl Into<String>,
        run_id: impl Into<String>,
        task: ContextTask,
        slices: Vec<ContextSlice>,
        budget: ContextBudget,
        sources: Vec<ContextSource>,
        provenance: ContextPacketProvenance,
    ) -> Result<Self, ContextError> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(ContextError::EmptyPacketId);
        }

        let run_id = run_id.into();
        if run_id.trim().is_empty() {
            return Err(ContextError::EmptyRunId);
        }

        if sources.is_empty() {
            return Err(ContextError::MissingSources);
        }

        Ok(Self {
            id,
            run_id,
            task,
            slices,
            budget,
            sources,
            provenance,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_provenance() -> ContextProvenance {
        ContextProvenance {
            source: "docs/architecture/context-pipeline.md".to_string(),
            selected_by: "operator".to_string(),
        }
    }

    fn valid_slice(id: &str, path: &str) -> ContextSlice {
        ContextSlice::new(
            id,
            ContextViewType::Docs,
            TruthDimension::Structural,
            path,
            "slice content",
            valid_provenance(),
        )
        .expect("valid slice")
    }

    fn valid_task() -> ContextTask {
        ContextTask::new("task-1", "assemble context").expect("valid task")
    }

    fn valid_budget() -> ContextBudget {
        ContextBudget::new(10, 5, ContextBudgetUnit::Tokens).expect("valid budget")
    }

    fn valid_packet_provenance() -> ContextPacketProvenance {
        ContextPacketProvenance::new("context_assembler", "phase_8_assembly")
            .expect("valid packet provenance")
    }

    fn valid_sources() -> Vec<ContextSource> {
        vec![
            ContextSource::new("docs/governance/GOVERNANCE.md", TruthDimension::Normative)
                .expect("valid source"),
        ]
    }

    #[test]
    fn context_budget_allows_used_equal_to_max() {
        let budget = ContextBudget::new(10, 10, ContextBudgetUnit::Tokens);
        assert!(budget.is_ok());
    }

    #[test]
    fn context_budget_rejects_used_greater_than_max() {
        let error = ContextBudget::new(10, 11, ContextBudgetUnit::Tokens).expect_err("must reject");
        assert_eq!(error, ContextError::BudgetExceeded);
    }

    #[test]
    fn context_slice_requires_id() {
        let error = ContextSlice::new(
            "",
            ContextViewType::Code,
            TruthDimension::Executable,
            "core/src/context/mod.rs",
            "content",
            valid_provenance(),
        )
        .expect_err("must reject");
        assert_eq!(error, ContextError::EmptySliceId);
    }

    #[test]
    fn context_slice_requires_source_path() {
        let error = ContextSlice::new(
            "slice-1",
            ContextViewType::Code,
            TruthDimension::Executable,
            "",
            "content",
            valid_provenance(),
        )
        .expect_err("must reject");
        assert_eq!(error, ContextError::EmptySourcePath);
    }

    #[test]
    fn context_slice_requires_content() {
        let error = ContextSlice::new(
            "slice-1",
            ContextViewType::Code,
            TruthDimension::Executable,
            "core/src/context/mod.rs",
            "",
            valid_provenance(),
        )
        .expect_err("must reject");
        assert_eq!(error, ContextError::EmptySliceContent);
    }

    #[test]
    fn context_slice_requires_provenance_source() {
        let error = ContextSlice::new(
            "slice-1",
            ContextViewType::Code,
            TruthDimension::Executable,
            "core/src/context/mod.rs",
            "content",
            ContextProvenance {
                source: "".to_string(),
                selected_by: "operator".to_string(),
            },
        )
        .expect_err("must reject");
        assert_eq!(error, ContextError::MissingSliceProvenance);
    }

    #[test]
    fn context_slice_requires_provenance_selected_by() {
        let error = ContextSlice::new(
            "slice-1",
            ContextViewType::Code,
            TruthDimension::Executable,
            "core/src/context/mod.rs",
            "content",
            ContextProvenance {
                source: "docs/governance/GOVERNANCE.md".to_string(),
                selected_by: "".to_string(),
            },
        )
        .expect_err("must reject");
        assert_eq!(error, ContextError::MissingSliceProvenance);
    }

    #[test]
    fn context_source_requires_path() {
        let error = ContextSource::new("", TruthDimension::Contract).expect_err("must reject");
        assert_eq!(error, ContextError::EmptySourcePath);
    }

    #[test]
    fn context_task_requires_id() {
        let error = ContextTask::new("", "summary").expect_err("must reject");
        assert_eq!(error, ContextError::EmptyTaskId);
    }

    #[test]
    fn context_task_requires_summary() {
        let error = ContextTask::new("task", "").expect_err("must reject");
        assert_eq!(error, ContextError::EmptyTaskSummary);
    }

    #[test]
    fn context_packet_provenance_requires_assembled_by() {
        let error = ContextPacketProvenance::new("", "reason").expect_err("must reject");
        assert_eq!(error, ContextError::EmptyAssembledBy);
    }

    #[test]
    fn context_packet_provenance_requires_assembly_reason() {
        let error = ContextPacketProvenance::new("assembler", "").expect_err("must reject");
        assert_eq!(error, ContextError::EmptyAssemblyReason);
    }

    #[test]
    fn context_packet_requires_id() {
        let error = ContextPacket::assemble(
            "",
            "run-1",
            valid_task(),
            vec![valid_slice("slice-1", "core/src/context/mod.rs")],
            valid_budget(),
            valid_sources(),
            valid_packet_provenance(),
        )
        .expect_err("must reject");
        assert_eq!(error, ContextError::EmptyPacketId);
    }

    #[test]
    fn context_packet_requires_run_id() {
        let error = ContextPacket::assemble(
            "packet-1",
            "",
            valid_task(),
            vec![valid_slice("slice-1", "core/src/context/mod.rs")],
            valid_budget(),
            valid_sources(),
            valid_packet_provenance(),
        )
        .expect_err("must reject");
        assert_eq!(error, ContextError::EmptyRunId);
    }

    #[test]
    fn context_packet_requires_sources() {
        let error = ContextPacket::assemble(
            "packet-1",
            "run-1",
            valid_task(),
            vec![valid_slice("slice-1", "core/src/context/mod.rs")],
            valid_budget(),
            vec![],
            valid_packet_provenance(),
        )
        .expect_err("must reject");
        assert_eq!(error, ContextError::MissingSources);
    }

    #[test]
    fn context_packet_assembles_fixed_inputs_deterministically() {
        let slices = vec![valid_slice("slice-1", "a"), valid_slice("slice-2", "b")];
        let sources = vec![
            ContextSource::new("a", TruthDimension::Procedural).expect("source a"),
            ContextSource::new("b", TruthDimension::Structural).expect("source b"),
        ];

        let first = ContextPacket::assemble(
            "packet-1",
            "run-1",
            valid_task(),
            slices.clone(),
            valid_budget(),
            sources.clone(),
            valid_packet_provenance(),
        )
        .expect("first packet");

        let second = ContextPacket::assemble(
            "packet-1",
            "run-1",
            valid_task(),
            slices,
            valid_budget(),
            sources,
            valid_packet_provenance(),
        )
        .expect("second packet");

        assert_eq!(first, second);
    }

    #[test]
    fn context_packet_preserves_slice_order() {
        let slices = vec![
            valid_slice("slice-a", "first"),
            valid_slice("slice-b", "second"),
        ];
        let packet = ContextPacket::assemble(
            "packet-1",
            "run-1",
            valid_task(),
            slices,
            valid_budget(),
            valid_sources(),
            valid_packet_provenance(),
        )
        .expect("packet");

        assert_eq!(packet.slices[0].id, "slice-a");
        assert_eq!(packet.slices[1].id, "slice-b");
    }

    #[test]
    fn context_packet_preserves_source_order() {
        let sources = vec![
            ContextSource::new("first", TruthDimension::Procedural).expect("first source"),
            ContextSource::new("second", TruthDimension::Structural).expect("second source"),
        ];
        let packet = ContextPacket::assemble(
            "packet-1",
            "run-1",
            valid_task(),
            vec![valid_slice("slice-1", "core/src/context/mod.rs")],
            valid_budget(),
            sources,
            valid_packet_provenance(),
        )
        .expect("packet");

        assert_eq!(packet.sources[0].path, "first");
        assert_eq!(packet.sources[1].path, "second");
    }

    #[test]
    fn context_error_codes_are_stable() {
        assert_eq!(ContextError::EmptyPacketId.code(), "empty_packet_id");
        assert_eq!(ContextError::EmptyRunId.code(), "empty_run_id");
        assert_eq!(ContextError::EmptyTaskId.code(), "empty_task_id");
        assert_eq!(ContextError::EmptyTaskSummary.code(), "empty_task_summary");
        assert_eq!(
            ContextError::MissingSliceProvenance.code(),
            "missing_slice_provenance"
        );
        assert_eq!(ContextError::EmptySliceId.code(), "empty_slice_id");
        assert_eq!(
            ContextError::EmptySliceContent.code(),
            "empty_slice_content"
        );
        assert_eq!(ContextError::EmptySourcePath.code(), "empty_source_path");
        assert_eq!(ContextError::MissingSources.code(), "missing_sources");
        assert_eq!(ContextError::BudgetExceeded.code(), "budget_exceeded");
        assert_eq!(ContextError::EmptyAssembledBy.code(), "empty_assembled_by");
        assert_eq!(
            ContextError::EmptyAssemblyReason.code(),
            "empty_assembly_reason"
        );
    }
}
