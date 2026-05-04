import type { ApplicationUiProjection, UiReadModel } from "./projections";

export const applicationProjectionFixture: ApplicationUiProjection = {
  projectionId: "proj-fixture-0052",
  runtimeConfigId: "runtime-cfg-0052",
  safety: {
    safetyLevel: "strict",
    requirePolicyPass: true,
    requireValidationPass: true,
    requireReplayVerification: true,
    allowProviderNetwork: false,
    allowFileIo: false,
    allowUiMutation: false,
    authority: "rust",
    summary: "Runtime safety defaults are closed; provider network, file IO, and UI mutation are disallowed."
  },
  lifecycle: { lifecycle: "Evaluating", revision: 52, status: "ready", authority: "rust", summary: "Lifecycle read projection from Rust-owned boundary." },
  run: { runId: "run-fixture-0052", title: "Phase 52 Typed Read Projection Fixture", status: "ready", currentLifecycle: "Evaluating", executionDecision: "allow", promotionDecision: "hold", replayReadiness: "replayable", cleanOutputAvailable: true, authority: "rust", summary: "Run projection is display-only and non-authoritative in UI." },
  provider: { providerKind: "deterministic_stub", outputStatus: "received", outputTrust: "untrusted", authoritative: false, authority: "provider", summary: "Provider output remains untrusted and non-authoritative." },
  integration: { sourceKind: "local_llm", outputStatus: "received", outputTrust: "untrusted", authoritative: false, authority: "integration", summary: "Integration output remains untrusted and non-authoritative." },
  ledger: { events: 3, lastRevision: 3, status: "ready", authority: "rust", summary: "Ledger summary is projection-only and does not append events from UI." },
  replay: { readiness: "replayable", integrity: "consistent", eventsReplayed: 3, status: "ready", authority: "rust", summary: "Replay summary is display-only and non-executing in UI." },
  audit: { projections: 2, latestSummary: "Audit projection summary from supplied read projection data.", authority: "rust", summary: "Audit projection is read-only and non-authoritative in UI." },
  context: { packetId: "ctx-fixture-0052", slices: 4, sources: 3, budgetUsed: 412, budgetMax: 1024, authority: "rust", summary: "Context summary from typed read projection metadata.", slicesPreview: [] },
  memory: { snapshotId: "mem-fixture-0052", activeEntries: 8, disabledEntries: 1, rejectedEntries: 2, authority: "rust", summary: "Memory summary from typed read projection metadata.", entriesPreview: [] },
  output: { cleanOutputAvailable: true, rawOutputTrusted: false, authority: "rust", summary: "Raw provider/model output remains untrusted and distinct from clean output." }
};

export const uiReadModelFixture: UiReadModel = {
  source: "fixture",
  application: applicationProjectionFixture,
  decisions: [
    { label: "Policy gate", decision: "allow", reason: "projection_display_only", status: "ready", authority: "rust" },
    { label: "Validation gate", decision: "pending", reason: "no_runtime_transport_in_phase_52", status: "unknown", authority: "ui" }
  ],
  policyDecisions: [],
  validationDecisions: [],
  executionDecisions: [],
  ledgerTimeline: [],
  replayDetail: { readiness: "ready_for_replay", integrity: "integrity_verified", reconstructionStatus: "reported_by_projection", finalLifecycle: "Validated", finalRevision: 3, eventsSeen: 3, stateTransitionsApplied: 3, authority: "rust", summary: "Replay detail remains non-executing UI text." },
  auditDetails: [],
  cleanOutput: { id: "clean-output-0052", cleanOutputAvailable: true, rawOutputTrusted: false, cleanOutputTrusted: true, authority: "rust", summary: "Clean output projection is read-only text.", rawOutputSummary: "Raw provider/model output remains untrusted and non-authoritative.", cleanOutputSummary: "Clean output preview is distinct display data and does not grant authority." },
  operatorIntentPreviews: [
    { id: "intent-preview-approve", intentType: "approve", label: "Approve candidate", description: "Request preview only.", reasonPreview: "operator requests approval review", routePreview: "operator_intent_preview_only", authority: "operator", status: "unknown", disabled: true }
  ]
};
