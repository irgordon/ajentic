export type AuthoritySurface = "rust" | "ui" | "operator" | "provider" | "integration" | "unknown";

export type ProjectionStatus = "ready" | "blocked" | "rejected" | "unknown" | "not_available";

export type RuntimeSafetyUiProjection = Readonly<{
  safetyLevel: string;
  requirePolicyPass: boolean;
  requireValidationPass: boolean;
  requireReplayVerification: boolean;
  allowProviderNetwork: boolean;
  allowFileIo: boolean;
  allowUiMutation: boolean;
  authority: AuthoritySurface;
  summary: string;
}>;

export type LifecycleProjection = Readonly<{
  lifecycle: string;
  revision: number;
  status: ProjectionStatus;
  authority: AuthoritySurface;
  summary: string;
}>;

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

export type IntentSubmissionUiProjection = Readonly<{ submissionId: string; operatorId: string; intentType: string; targetKind: string; targetId: string; reason: string; requestPreviewEnabled: boolean; submissionEnabled: false; authority: AuthoritySurface; summary: string; }>;

export type OperatorIntentPreviewProjection = Readonly<{ id: string; intentType: string; label: string; description: string; reasonPreview: string; routePreview: string; authority: AuthoritySurface; status: ProjectionStatus; disabled: boolean; submissionPreview: IntentSubmissionUiProjection; }>;

export type ApplicationUiProjection = Readonly<{
  projectionId: string;
  runtimeConfigId: string;
  safety: RuntimeSafetyUiProjection;
  lifecycle: LifecycleProjection;
  run: RunOverviewProjection;
  provider: ProviderTrustProjection;
  integration: IntegrationTrustProjection;
  ledger: LedgerProjection;
  replay: ReplayProjection;
  audit: AuditProjectionSummary;
  context: ContextProjection;
  memory: MemoryProjection;
  output: OutputProjection;
}>;

export type UiReadModel = Readonly<{
  source: "fixture" | "supplied_projection";
  application: ApplicationUiProjection;
  decisions: readonly DecisionProjection[];
  policyDecisions: readonly DecisionDetailProjection[];
  validationDecisions: readonly DecisionDetailProjection[];
  executionDecisions: readonly DecisionDetailProjection[];
  ledgerTimeline: readonly LedgerTimelineEntryProjection[];
  replayDetail: ReplayDetailProjection;
  auditDetails: readonly AuditDetailProjection[];
  cleanOutput: CleanOutputProjection;
  operatorIntentPreviews: readonly OperatorIntentPreviewProjection[];
}>;
