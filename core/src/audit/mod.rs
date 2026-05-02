#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditProjectionType {
    Timeline,
    ValidationSummary,
    PolicySummary,
    MemorySummary,
    ContextSummary,
    ReplaySummary,
    OutputSummary,
    ExportSummary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditProjection {
    pub projection_type: AuditProjectionType,
    pub summary: String,
}
