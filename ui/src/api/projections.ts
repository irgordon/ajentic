export type AuthoritySurface = "rust" | "ui" | "operator" | "provider" | "unknown";

export type ProjectionStatus = "ready" | "blocked" | "rejected" | "unknown" | "not_available";

export type LifecycleProjection = Readonly<{
  lifecycle: string;
  revision: number;
  status: ProjectionStatus;
  authority: AuthoritySurface;
  summary: string;
}>;

export type DecisionProjection = Readonly<{
  label: string;
  decision: string;
  reason: string;
  status: ProjectionStatus;
  authority: AuthoritySurface;
}>;

export type DecisionDetailProjection = Readonly<{
  id: string;
  label: string;
  decision: string;
  reason: string;
  status: ProjectionStatus;
  authority: AuthoritySurface;
  summary: string;
  evidenceSummary: string;
}>;

export type RunOverviewProjection = Readonly<{
  runId: string;
  title: string;
  status: ProjectionStatus;
  currentLifecycle: string;
  executionDecision: string;
  promotionDecision: string;
  replayReadiness: string;
  cleanOutputAvailable: boolean;
  authority: AuthoritySurface;
  summary: string;
}>;

export type ContextSliceProjection = Readonly<{
  id: string;
  viewType: string;
  truthDimension: string;
  sourcePath: string;
  authority: AuthoritySurface;
  summary: string;
  provenance: string;
}>;

export type MemoryEntryProjection = Readonly<{
  id: string;
  memoryType: string;
  status: ProjectionStatus;
  authority: AuthoritySurface;
  summary: string;
  provenance: string;
}>;

export type ContextProjection = Readonly<{
  packetId: string;
  slices: number;
  sources: number;
  budgetUsed: number;
  budgetMax: number;
  authority: AuthoritySurface;
  summary: string;
  slicesPreview: readonly ContextSliceProjection[];
}>;

export type MemoryProjection = Readonly<{
  snapshotId: string;
  activeEntries: number;
  disabledEntries: number;
  rejectedEntries: number;
  authority: AuthoritySurface;
  summary: string;
  entriesPreview: readonly MemoryEntryProjection[];
}>;

export type LedgerProjection = Readonly<{
  events: number;
  lastRevision: number | null;
  status: ProjectionStatus;
  authority: AuthoritySurface;
  summary: string;
}>;

export type LedgerTimelineEntryProjection = Readonly<{
  id: string;
  revision: number;
  eventType: string;
  actor: string;
  evidenceRefs: number;
  status: ProjectionStatus;
  authority: AuthoritySurface;
  summary: string;
}>;

export type ReplayProjection = Readonly<{
  readiness: string;
  integrity: string;
  eventsReplayed: number;
  status: ProjectionStatus;
  authority: AuthoritySurface;
  summary: string;
}>;

export type ReplayDetailProjection = Readonly<{
  readiness: string;
  integrity: string;
  reconstructionStatus: string;
  finalLifecycle: string;
  finalRevision: number;
  eventsSeen: number;
  stateTransitionsApplied: number;
  authority: AuthoritySurface;
  summary: string;
}>;

export type AuditProjectionSummary = Readonly<{
  projections: number;
  latestSummary: string;
  authority: AuthoritySurface;
  summary: string;
}>;

export type AuditDetailProjection = Readonly<{
  id: string;
  projectionType: string;
  source: string;
  authority: AuthoritySurface;
  summary: string;
  details: readonly string[];
}>;

export type OutputProjection = Readonly<{
  cleanOutputAvailable: boolean;
  rawOutputTrusted: boolean;
  authority: AuthoritySurface;
  summary: string;
}>;

export type CleanOutputProjection = Readonly<{
  id: string;
  cleanOutputAvailable: boolean;
  rawOutputTrusted: boolean;
  cleanOutputTrusted: boolean;
  authority: AuthoritySurface;
  summary: string;
  rawOutputSummary: string;
  cleanOutputSummary: string;
}>;


export type OperatorIntentPreviewProjection = Readonly<{
  id: string;
  intentType: string;
  label: string;
  description: string;
  reasonPreview: string;
  routePreview: string;
  authority: AuthoritySurface;
  status: ProjectionStatus;
  disabled: boolean;
}>;
export type UiReadModel = Readonly<{
  lifecycle: LifecycleProjection;
  run: RunOverviewProjection;
  decisions: readonly DecisionProjection[];
  policyDecisions: readonly DecisionDetailProjection[];
  validationDecisions: readonly DecisionDetailProjection[];
  executionDecisions: readonly DecisionDetailProjection[];
  context: ContextProjection;
  memory: MemoryProjection;
  ledger: LedgerProjection;
  ledgerTimeline: readonly LedgerTimelineEntryProjection[];
  replay: ReplayProjection;
  replayDetail: ReplayDetailProjection;
  audit: AuditProjectionSummary;
  auditDetails: readonly AuditDetailProjection[];
  output: OutputProjection;
  cleanOutput: CleanOutputProjection;
  operatorIntentPreviews: readonly OperatorIntentPreviewProjection[];
}>;
