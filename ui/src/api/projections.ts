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

export type ContextProjection = Readonly<{
  packetId: string;
  slices: number;
  sources: number;
  budgetUsed: number;
  budgetMax: number;
  authority: AuthoritySurface;
  summary: string;
}>;

export type MemoryProjection = Readonly<{
  snapshotId: string;
  activeEntries: number;
  disabledEntries: number;
  rejectedEntries: number;
  authority: AuthoritySurface;
  summary: string;
}>;

export type LedgerProjection = Readonly<{
  events: number;
  lastRevision: number | null;
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

export type AuditProjectionSummary = Readonly<{
  projections: number;
  latestSummary: string;
  authority: AuthoritySurface;
  summary: string;
}>;

export type OutputProjection = Readonly<{
  cleanOutputAvailable: boolean;
  rawOutputTrusted: boolean;
  authority: AuthoritySurface;
  summary: string;
}>;

export type UiReadModel = Readonly<{
  lifecycle: LifecycleProjection;
  run: RunOverviewProjection;
  decisions: readonly DecisionProjection[];
  context: ContextProjection;
  memory: MemoryProjection;
  ledger: LedgerProjection;
  replay: ReplayProjection;
  audit: AuditProjectionSummary;
  output: OutputProjection;
}>;
