export type AuthoritySurface = "rust" | "ui" | "operator" | "provider" | "integration" | "unknown";

export type ProjectionStatus = "ready" | "blocked" | "rejected" | "unknown" | "not_available";

export type DiagnosticFamilyLabel =
  | "operator_intent"
  | "integration"
  | "runtime_config"
  | "read_projection"
  | "application_state"
  | "persistence_validation"
  | "persistence_execution"
  | "persistence_recovery"
  | "local_workflow"
  | "provider_boundary"
  | "provider_adapter"
  | "local_provider_config"
  | "controlled_run"
  | "promotion"
  | "promotion_replay"
  | "execution_decision"
  | "unknown";

export type ErrorDiagnosticProjection = Readonly<{
  family: DiagnosticFamilyLabel;
  code: string;
  summary: string;
  key: string;
}>;

export type PersistedRecordVerificationStatus =
  | "valid"
  | "missing_target"
  | "orphaned_temp"
  | "malformed_record"
  | "invalid_payload_hex"
  | "checksum_mismatch"
  | "payload_length_mismatch"
  | "stale_revision"
  | "unknown_payload_kind"
  | "read_failed";

export type PersistedRecordRecoveryAction = "none" | "manual_review_required";

export type PersistedRecordVerificationProjection = Readonly<{
  status: PersistedRecordVerificationStatus;
  recoveryAction: PersistedRecordRecoveryAction;
  recordId?: string;
  payloadKind?: string;
  revision?: number;
  checksum?: string;
  payloadLen?: number;
  summary: string;
  diagnostic?: ErrorDiagnosticProjection;
}>;

export type RuntimeSafetyUiProjection = Readonly<{ safetyLevel: string; requirePolicyPass: boolean; requireValidationPass: boolean; requireReplayVerification: boolean; allowProviderNetwork: boolean; allowFileIo: boolean; allowUiMutation: boolean; authority: AuthoritySurface; summary: string; }>;
export type LifecycleProjection = Readonly<{ lifecycle: string; revision: number; status: ProjectionStatus; authority: AuthoritySurface; summary: string; }>;
export type DecisionProjection = Readonly<{ label: string; decision: string; reason: string; status: ProjectionStatus; authority: AuthoritySurface; }>;
export type DecisionDetailProjection = Readonly<{ id: string; label: string; decision: string; reason: string; status: ProjectionStatus; authority: AuthoritySurface; summary: string; evidenceSummary: string; }>;
export type RunOverviewProjection = Readonly<{ runId: string; title: string; status: ProjectionStatus; currentLifecycle: string; executionDecision: string; promotionDecision: string; replayReadiness: string; cleanOutputAvailable: boolean; authority: AuthoritySurface; summary: string; }>;
export type ProviderTrustProjection = Readonly<{ providerKind: string; outputStatus: string; outputTrust: string; authoritative: boolean; authority: AuthoritySurface; summary: string; }>;
export type IntegrationTrustProjection = Readonly<{ sourceKind: string; outputStatus: string; outputTrust: string; authoritative: boolean; authority: AuthoritySurface; summary: string; }>;
export type ContextSliceProjection = Readonly<{ id: string; viewType: string; truthDimension: string; sourcePath: string; authority: AuthoritySurface; summary: string; provenance: string; }>;
export type MemoryEntryProjection = Readonly<{ id: string; memoryType: string; status: ProjectionStatus; authority: AuthoritySurface; summary: string; provenance: string; }>;
export type ContextProjection = Readonly<{ packetId: string; slices: number; sources: number; budgetUsed: number; budgetMax: number; authority: AuthoritySurface; summary: string; slicesPreview: readonly ContextSliceProjection[]; }>;
export type MemoryProjection = Readonly<{ snapshotId: string; activeEntries: number; disabledEntries: number; rejectedEntries: number; authority: AuthoritySurface; summary: string; entriesPreview: readonly MemoryEntryProjection[]; }>;
export type LedgerProjection = Readonly<{ events: number; lastRevision: number | null; status: ProjectionStatus; authority: AuthoritySurface; summary: string; }>;
export type LedgerTimelineEntryProjection = Readonly<{ id: string; revision: number; eventType: string; actor: string; evidenceRefs: number; status: ProjectionStatus; authority: AuthoritySurface; summary: string; }>;
export type ReplayProjection = Readonly<{ readiness: string; integrity: string; eventsReplayed: number; status: ProjectionStatus; authority: AuthoritySurface; summary: string; }>;
export type ReplayDetailProjection = Readonly<{ readiness: string; integrity: string; reconstructionStatus: string; finalLifecycle: string; finalRevision: number; eventsSeen: number; stateTransitionsApplied: number; authority: AuthoritySurface; summary: string; }>;
export type AuditProjectionSummary = Readonly<{ projections: number; latestSummary: string; authority: AuthoritySurface; summary: string; }>;
export type AuditDetailProjection = Readonly<{ id: string; projectionType: string; source: string; authority: AuthoritySurface; summary: string; details: readonly string[]; }>;
export type OutputProjection = Readonly<{ cleanOutputAvailable: boolean; rawOutputTrusted: boolean; authority: AuthoritySurface; summary: string; }>;
export type CleanOutputProjection = Readonly<{ id: string; cleanOutputAvailable: boolean; rawOutputTrusted: boolean; cleanOutputTrusted: boolean; authority: AuthoritySurface; summary: string; rawOutputSummary: string; cleanOutputSummary: string; }>;

export type IntentTypeProjection =
  | "approve"
  | "reject"
  | "retry"
  | "memory_write"
  | "memory_delete"
  | "run_start"
  | "run_stop"
  | "tool_request"
  | "policy_check"
  | "state_transition_request"
  | "context_rebuild_request"
  | "replay_request"
  | "memory_snapshot_request"
  | "memory_disable_request";

export type IntentTargetKindProjection = "run" | "candidate" | "replay" | "context" | "memory" | "output" | "unknown";

export type IntentSubmissionUiProjection = Readonly<{ submissionId: string; operatorId: string; intentType: IntentTypeProjection; targetKind: IntentTargetKindProjection; targetId: string; reason: string; requestPreviewEnabled: boolean; submissionEnabled: boolean; authority: AuthoritySurface; summary: string; }>;


export type UiSubmissionBoundaryStatus = "accepted_for_preview" | "rejected";

export type UiSubmissionBoundaryReasonCode =
  | "accepted_for_preview"
  | "empty_operator_id"
  | "empty_target_id"
  | "empty_intent_kind"
  | "unknown_intent_kind"
  | "authority_escalation_text_rejected"
  | "live_transport_flag_spoof_rejected"
  | "execution_flag_spoof_rejected"
  | "persistence_flag_spoof_rejected"
  | "ledger_recording_flag_spoof_rejected"
  | "audit_append_flag_spoof_rejected"
  | "provider_execution_flag_spoof_rejected"
  | "replay_repair_flag_spoof_rejected"
  | "authority_mutation_flag_spoof_rejected"
  | "malformed_submission_rejected";

export type UiSubmissionBoundaryInput = Readonly<{
  operatorId?: unknown;
  targetId?: unknown;
  intentKind?: unknown;
  intentType?: unknown;
  reason?: unknown;
  executionEnabled?: unknown;
  liveTransportEnabled?: unknown;
  persistenceEnabled?: unknown;
  ledgerRecordingEnabled?: unknown;
  auditAppendEnabled?: unknown;
  providerExecutionEnabled?: unknown;
  replayRepairEnabled?: unknown;
  mutatesAuthority?: unknown;
  mutatesApplicationState?: unknown;
}>;

export type UiSubmissionBoundaryResult = Readonly<{
  status: UiSubmissionBoundaryStatus;
  reasonCode: UiSubmissionBoundaryReasonCode;
  transportEligible: boolean;
  liveTransportCalled: boolean;
  liveTransportEnabled: boolean;
  executionEnabled: boolean;
  persistenceEnabled: boolean;
  ledgerRecordingEnabled: boolean;
  auditAppendEnabled: boolean;
  providerExecutionEnabled: boolean;
  replayRepairEnabled: boolean;
  mutatesAuthority: boolean;
  summary: string;
}>;

export type UiOperatorIntentSubmissionStatus =
  | "draft"
  | "submission_shaped"
  | "ingress_ready"
  | "authorization_ready"
  | "audit_eligible"
  | "rejected";

export type UiOperatorIntentSubmissionCapability = Readonly<{
  submissionShapeEnabled: boolean;
  liveTransportEnabled: false;
  executionEnabled: false;
  persistenceEnabled: false;
  ledgerRecordingEnabled: false;
  auditAppendEnabled: false;
  providerExecutionEnabled: false;
  replayRepairEnabled: false;
  mutatesApplicationState: false;
}>;

export type UiOperatorIntentSubmissionContract = Readonly<{
  submissionId: string;
  operatorId: string;
  targetKind: IntentTargetKindProjection;
  targetId: string;
  intentType: IntentTypeProjection;
  reason: string;
  status: UiOperatorIntentSubmissionStatus;
  capability: UiOperatorIntentSubmissionCapability;
  diagnostic?: ErrorDiagnosticProjection;
  summary: string;
}>;

export type UiOperatorIntentSubmissionEnvelope = UiRustTransportEnvelope<UiOperatorIntentSubmissionContract>;

export type OperatorIntentPreviewProjection = Readonly<{ id: string; intentType: IntentTypeProjection; label: string; description: string; reasonPreview: string; routePreview: string; authority: AuthoritySurface; status: ProjectionStatus; disabled: boolean; submissionPreview: IntentSubmissionUiProjection; }>;

export type ApplicationUiProjection = Readonly<{ projectionId: string; runtimeConfigId: string; safety: RuntimeSafetyUiProjection; lifecycle: LifecycleProjection; run: RunOverviewProjection; provider: ProviderTrustProjection; integration: IntegrationTrustProjection; ledger: LedgerProjection; replay: ReplayProjection; audit: AuditProjectionSummary; context: ContextProjection; memory: MemoryProjection; output: OutputProjection; }>;

export type UiReadModel = Readonly<{ source: "fixture" | "supplied_projection"; application: ApplicationUiProjection; decisions: readonly DecisionProjection[]; policyDecisions: readonly DecisionDetailProjection[]; validationDecisions: readonly DecisionDetailProjection[]; executionDecisions: readonly DecisionDetailProjection[]; ledgerTimeline: readonly LedgerTimelineEntryProjection[]; replayDetail: ReplayDetailProjection; auditDetails: readonly AuditDetailProjection[]; cleanOutput: CleanOutputProjection; operatorIntentPreviews: readonly OperatorIntentPreviewProjection[]; persistedRecordVerification: PersistedRecordVerificationProjection; diagnostics: readonly ErrorDiagnosticProjection[]; localRuntimeReview: LocalRuntimeReviewSurface; }>;



export type LocalRuntimeCapabilityStatus = "enabled" | "disabled";

export type LocalRuntimeCapabilityIndicator = Readonly<{
  id: string;
  label: string;
  status: LocalRuntimeCapabilityStatus;
  summary: string;
}>;

export type LocalRuntimeReviewInteractionStatus = "previewed" | "rejected";

export type LocalRuntimeReviewInteraction = Readonly<{
  id: string;
  label: string;
  status: LocalRuntimeReviewInteractionStatus;
  result: UiSubmissionBoundaryResult;
  summary: string;
}>;

export type LocalRuntimeReviewState = Readonly<{
  workflowState: string;
  reviewState: string;
  escalationState: string;
  failureState: string;
  evidenceState: string;
  validationStatus: ProjectionStatus;
  dryRunSummary: string;
}>;

export type LocalRuntimeReviewSurface = Readonly<{
  surfaceId: string;
  title: string;
  launchInstruction: string;
  postureIndicators: readonly string[];
  disabledCapabilities: readonly LocalRuntimeCapabilityIndicator[];
  reviewState: LocalRuntimeReviewState;
  interactions: readonly LocalRuntimeReviewInteraction[];
  deterministicSummary: string;
  nonReadinessStatement: string;
}>;

export type UiRustTransportDirection = "ui_to_rust" | "rust_to_ui";

export type UiRustTransportStatus = "prepared" | "display_only" | "disabled";

export type UiRustTransportCapability = Readonly<{
  transportEnabled: false;
  mutationEnabled: false;
  submissionEnabled: boolean;
  executionEnabled: false;
  persistenceEnabled: false;
}>;

export type UiRustTransportEnvelope<TPayload> = Readonly<{
  envelopeId: string;
  direction: UiRustTransportDirection;
  status: UiRustTransportStatus;
  payload: TPayload;
  capability: UiRustTransportCapability;
  diagnostic?: ErrorDiagnosticProjection;
  summary: string;
}>;

export type UiRustReadProjectionResponse = UiRustTransportEnvelope<UiReadModel>;

export type UiRustIntentPreviewRequest = UiRustTransportEnvelope<IntentSubmissionUiProjection>;
