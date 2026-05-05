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

pub const MAX_PROJECTION_SLICE_LIMIT: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionSliceSurface {
    Ledger,
    Replay,
    Audit,
    Memory,
    Context,
    Output,
    Intent,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionSliceMode {
    Summary,
    Detail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionSliceStatus {
    Ready,
    Empty,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionSliceRejectionReason {
    Accepted,
    EmptyRequestId,
    UnknownSurface,
    ZeroLimit,
    LimitTooLarge,
    OffsetOutOfRange,
    UnsupportedMode,
}

impl ProjectionSliceRejectionReason {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::EmptyRequestId => "empty_request_id",
            Self::UnknownSurface => "unknown_surface",
            Self::ZeroLimit => "zero_limit",
            Self::LimitTooLarge => "limit_too_large",
            Self::OffsetOutOfRange => "offset_out_of_range",
            Self::UnsupportedMode => "unsupported_mode",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionSliceRequest {
    pub request_id: String,
    pub surface: ProjectionSliceSurface,
    pub mode: ProjectionSliceMode,
    pub offset: usize,
    pub limit: usize,
}

impl ProjectionSliceRequest {
    pub fn new(
        request_id: impl Into<String>,
        surface: ProjectionSliceSurface,
        mode: ProjectionSliceMode,
        offset: usize,
        limit: usize,
    ) -> Result<Self, ProjectionSliceRejectionReason> {
        let request_id = request_id.into();
        if request_id.trim().is_empty() {
            return Err(ProjectionSliceRejectionReason::EmptyRequestId);
        }
        if surface == ProjectionSliceSurface::Unknown {
            return Err(ProjectionSliceRejectionReason::UnknownSurface);
        }
        if limit == 0 {
            return Err(ProjectionSliceRejectionReason::ZeroLimit);
        }
        if limit > MAX_PROJECTION_SLICE_LIMIT {
            return Err(ProjectionSliceRejectionReason::LimitTooLarge);
        }

        Ok(Self {
            request_id,
            surface,
            mode,
            offset,
            limit,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionSliceMetadata {
    pub request_id: String,
    pub surface: ProjectionSliceSurface,
    pub mode: ProjectionSliceMode,
    pub offset: usize,
    pub limit: usize,
    pub total_items: usize,
    pub returned_items: usize,
    pub has_more: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionSlice {
    pub status: ProjectionSliceStatus,
    pub reason: ProjectionSliceRejectionReason,
    pub metadata: Option<ProjectionSliceMetadata>,
    pub items: Vec<String>,
    pub summary: String,
}

pub fn build_projection_slice(
    request: ProjectionSliceRequest,
    items: &[String],
) -> ProjectionSlice {
    if request.offset > items.len() {
        return ProjectionSlice {
            status: ProjectionSliceStatus::Rejected,
            reason: ProjectionSliceRejectionReason::OffsetOutOfRange,
            metadata: None,
            items: vec![],
            summary: "Read-only bounded projection slice rejected: offset is out of range."
                .to_string(),
        };
    }

    let total_items = items.len();
    let bounded_items: Vec<String> = items
        .iter()
        .skip(request.offset)
        .take(request.limit)
        .cloned()
        .collect();
    let returned_items = bounded_items.len();
    let metadata = ProjectionSliceMetadata {
        request_id: request.request_id.clone(),
        surface: request.surface,
        mode: request.mode,
        offset: request.offset,
        limit: request.limit,
        total_items,
        returned_items,
        has_more: request.offset + returned_items < total_items,
    };

    let status = if total_items == 0 && request.offset == 0 {
        ProjectionSliceStatus::Empty
    } else {
        ProjectionSliceStatus::Ready
    };

    ProjectionSlice {
        status,
        reason: ProjectionSliceRejectionReason::Accepted,
        metadata: Some(metadata),
        items: bounded_items,
        summary: "Read-only bounded projection slice derived from caller-supplied in-memory items; no state mutation or authority inference is performed.".to_string(),
    }
}

pub fn projection_slice_executes_actions(_slice: &ProjectionSlice) -> bool {
    false
}

pub fn projection_slice_reads_persistence(_slice: &ProjectionSlice) -> bool {
    false
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

    fn request(
        surface: ProjectionSliceSurface,
        offset: usize,
        limit: usize,
    ) -> ProjectionSliceRequest {
        ProjectionSliceRequest::new(
            "req-1",
            surface,
            ProjectionSliceMode::Summary,
            offset,
            limit,
        )
        .expect("request should be valid")
    }

    fn sample_items() -> Vec<String> {
        vec!["a".into(), "b".into(), "c".into(), "d".into()]
    }

    #[test]
    fn projection_slice_rejection_reason_codes_are_stable() {
        assert_eq!(ProjectionSliceRejectionReason::Accepted.code(), "accepted");
        assert_eq!(
            ProjectionSliceRejectionReason::EmptyRequestId.code(),
            "empty_request_id"
        );
        assert_eq!(
            ProjectionSliceRejectionReason::UnknownSurface.code(),
            "unknown_surface"
        );
        assert_eq!(
            ProjectionSliceRejectionReason::ZeroLimit.code(),
            "zero_limit"
        );
        assert_eq!(
            ProjectionSliceRejectionReason::LimitTooLarge.code(),
            "limit_too_large"
        );
        assert_eq!(
            ProjectionSliceRejectionReason::OffsetOutOfRange.code(),
            "offset_out_of_range"
        );
        assert_eq!(
            ProjectionSliceRejectionReason::UnsupportedMode.code(),
            "unsupported_mode"
        );
    }
    #[test]
    fn projection_slice_request_requires_request_id() {
        assert!(matches!(
            ProjectionSliceRequest::new(
                "",
                ProjectionSliceSurface::Ledger,
                ProjectionSliceMode::Summary,
                0,
                1
            ),
            Err(ProjectionSliceRejectionReason::EmptyRequestId)
        ));
    }
    #[test]
    fn projection_slice_request_rejects_unknown_surface() {
        assert!(matches!(
            ProjectionSliceRequest::new(
                "r",
                ProjectionSliceSurface::Unknown,
                ProjectionSliceMode::Summary,
                0,
                1
            ),
            Err(ProjectionSliceRejectionReason::UnknownSurface)
        ));
    }
    #[test]
    fn projection_slice_request_rejects_zero_limit() {
        assert!(matches!(
            ProjectionSliceRequest::new(
                "r",
                ProjectionSliceSurface::Ledger,
                ProjectionSliceMode::Summary,
                0,
                0
            ),
            Err(ProjectionSliceRejectionReason::ZeroLimit)
        ));
    }
    #[test]
    fn projection_slice_request_rejects_limit_over_max() {
        assert!(matches!(
            ProjectionSliceRequest::new(
                "r",
                ProjectionSliceSurface::Ledger,
                ProjectionSliceMode::Summary,
                0,
                MAX_PROJECTION_SLICE_LIMIT + 1
            ),
            Err(ProjectionSliceRejectionReason::LimitTooLarge)
        ));
    }
    #[test]
    fn projection_slice_allows_limit_at_max() {
        assert!(ProjectionSliceRequest::new(
            "r",
            ProjectionSliceSurface::Ledger,
            ProjectionSliceMode::Summary,
            0,
            MAX_PROJECTION_SLICE_LIMIT
        )
        .is_ok());
    }
    #[test]
    fn projection_slice_rejects_offset_out_of_range() {
        let slice = build_projection_slice(
            request(ProjectionSliceSurface::Ledger, 5, 1),
            &sample_items(),
        );
        assert_eq!(slice.status, ProjectionSliceStatus::Rejected);
        assert_eq!(
            slice.reason,
            ProjectionSliceRejectionReason::OffsetOutOfRange
        );
    }
    #[test]
    fn projection_slice_returns_empty_for_empty_items() {
        let empty: Vec<String> = vec![];
        let slice = build_projection_slice(request(ProjectionSliceSurface::Ledger, 0, 1), &empty);
        assert_eq!(slice.status, ProjectionSliceStatus::Empty);
        assert_eq!(slice.reason, ProjectionSliceRejectionReason::Accepted);
        assert_eq!(slice.metadata.expect("metadata").returned_items, 0);
    }
    #[test]
    fn projection_slice_returns_bounded_items() {
        let slice = build_projection_slice(
            request(ProjectionSliceSurface::Ledger, 1, 2),
            &sample_items(),
        );
        assert_eq!(slice.items, vec!["b".to_string(), "c".to_string()]);
    }
    #[test]
    fn projection_slice_sets_has_more_when_more_items_exist() {
        let slice = build_projection_slice(
            request(ProjectionSliceSurface::Ledger, 0, 2),
            &sample_items(),
        );
        assert!(slice.metadata.expect("metadata").has_more);
    }
    #[test]
    fn projection_slice_sets_has_more_false_at_end() {
        let slice = build_projection_slice(
            request(ProjectionSliceSurface::Ledger, 3, 2),
            &sample_items(),
        );
        assert!(!slice.metadata.expect("metadata").has_more);
    }
    #[test]
    fn projection_slice_is_deterministic_for_same_input() {
        let req = request(ProjectionSliceSurface::Ledger, 1, 2);
        let items = sample_items();
        assert_eq!(
            build_projection_slice(req.clone(), &items),
            build_projection_slice(req, &items)
        );
    }
    #[test]
    fn projection_slice_does_not_mutate_source_items() {
        let items = sample_items();
        let clone = items.clone();
        let _ = build_projection_slice(request(ProjectionSliceSurface::Ledger, 1, 2), &items);
        assert_eq!(items, clone);
    }
    #[test]
    fn projection_slice_does_not_infer_authority_from_item_text() {
        let items = vec![
            "approved".into(),
            "trusted".into(),
            "execute".into(),
            "persist".into(),
            "admin override".into(),
        ];
        let slice = build_projection_slice(request(ProjectionSliceSurface::Audit, 0, 3), &items);
        assert_eq!(slice.status, ProjectionSliceStatus::Ready);
        assert_eq!(slice.reason, ProjectionSliceRejectionReason::Accepted);
        assert_eq!(
            slice.metadata.expect("metadata").surface,
            ProjectionSliceSurface::Audit
        );
    }
    #[test]
    fn projection_slice_supports_ledger_surface() {
        assert!(ProjectionSliceRequest::new(
            "r",
            ProjectionSliceSurface::Ledger,
            ProjectionSliceMode::Summary,
            0,
            1
        )
        .is_ok());
    }
    #[test]
    fn projection_slice_supports_replay_surface() {
        assert!(ProjectionSliceRequest::new(
            "r",
            ProjectionSliceSurface::Replay,
            ProjectionSliceMode::Summary,
            0,
            1
        )
        .is_ok());
    }
    #[test]
    fn projection_slice_supports_audit_surface() {
        assert!(ProjectionSliceRequest::new(
            "r",
            ProjectionSliceSurface::Audit,
            ProjectionSliceMode::Summary,
            0,
            1
        )
        .is_ok());
    }
    #[test]
    fn projection_slice_supports_memory_surface() {
        assert!(ProjectionSliceRequest::new(
            "r",
            ProjectionSliceSurface::Memory,
            ProjectionSliceMode::Summary,
            0,
            1
        )
        .is_ok());
    }
    #[test]
    fn projection_slice_supports_context_surface() {
        assert!(ProjectionSliceRequest::new(
            "r",
            ProjectionSliceSurface::Context,
            ProjectionSliceMode::Summary,
            0,
            1
        )
        .is_ok());
    }
    #[test]
    fn projection_slice_supports_output_surface() {
        assert!(ProjectionSliceRequest::new(
            "r",
            ProjectionSliceSurface::Output,
            ProjectionSliceMode::Summary,
            0,
            1
        )
        .is_ok());
    }
    #[test]
    fn projection_slice_supports_intent_surface() {
        assert!(ProjectionSliceRequest::new(
            "r",
            ProjectionSliceSurface::Intent,
            ProjectionSliceMode::Summary,
            0,
            1
        )
        .is_ok());
    }
    #[test]
    fn projection_slice_never_executes_actions() {
        let slice = build_projection_slice(
            request(ProjectionSliceSurface::Ledger, 0, 1),
            &sample_items(),
        );
        assert!(!projection_slice_executes_actions(&slice));
    }
    #[test]
    fn projection_slice_never_reads_persistence() {
        let slice = build_projection_slice(
            request(ProjectionSliceSurface::Ledger, 0, 1),
            &sample_items(),
        );
        assert!(!projection_slice_reads_persistence(&slice));
    }
    #[test]
    fn projection_slice_does_not_call_provider_or_model() {
        let slice = build_projection_slice(
            request(ProjectionSliceSurface::Output, 0, 1),
            &sample_items(),
        );
        assert_eq!(slice.reason, ProjectionSliceRejectionReason::Accepted);
    }
    #[test]
    fn projection_slice_does_not_repair_replay() {
        let slice = build_projection_slice(
            request(ProjectionSliceSurface::Replay, 0, 1),
            &sample_items(),
        );
        assert_eq!(slice.status, ProjectionSliceStatus::Ready);
    }
    #[test]
    fn projection_slice_does_not_append_ledger() {
        let slice = build_projection_slice(
            request(ProjectionSliceSurface::Ledger, 0, 1),
            &sample_items(),
        );
        assert_eq!(slice.items.len(), 1);
    }
    #[test]
    fn projection_slice_does_not_require_ui_transport() {
        let slice = build_projection_slice(
            request(ProjectionSliceSurface::Context, 0, 1),
            &sample_items(),
        );
        assert!(slice.summary.contains("Read-only bounded projection slice"));
    }
}
