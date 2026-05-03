import type { UiReadModel } from "./projections";

export const uiReadModelFixture: UiReadModel = {
  lifecycle: {
    lifecycle: "Evaluating",
    revision: 3,
    status: "ready",
    authority: "rust",
    summary: "Placeholder lifecycle projection for read-only UI preview."
  },
  run: {
    runId: "run-fixture-0023",
    title: "Phase 23 Read-Only Overview Fixture",
    status: "ready",
    currentLifecycle: "Evaluating",
    executionDecision: "allow",
    promotionDecision: "hold",
    replayReadiness: "replayable",
    cleanOutputAvailable: true,
    authority: "rust",
    summary: "Static run overview projection for display-only UI review."
  },
  decisions: [
    {
      label: "Policy gate",
      decision: "allow",
      reason: "fixture_only_preview",
      status: "ready",
      authority: "rust"
    },
    {
      label: "Validation gate",
      decision: "pending",
      reason: "no_live_runtime_data",
      status: "unknown",
      authority: "ui"
    }
  ],
  policyDecisions: [],
  validationDecisions: [],
  executionDecisions: [],
  context: {
    packetId: "ctx-fixture-0024",
    slices: 5,
    sources: 3,
    budgetUsed: 412,
    budgetMax: 1024,
    authority: "rust",
    summary: "Static context packet projection placeholder.",
    slicesPreview: []
  },
  memory: {
    snapshotId: "mem-fixture-0024",
    activeEntries: 8,
    disabledEntries: 1,
    rejectedEntries: 2,
    authority: "rust",
    summary: "Static memory projection placeholder for shell display.",
    entriesPreview: []
  },
  ledger: {
    events: 3,
    lastRevision: 3,
    status: "ready",
    authority: "rust",
    summary: "Ledger timeline projection is fixture-backed and read-only in this phase."
  },
  ledgerTimeline: [
    {
      id: "ledger-event-001",
      revision: 1,
      eventType: "run_opened",
      actor: "operator",
      evidenceRefs: 2,
      status: "ready",
      authority: "rust",
      summary: "Run opened and recorded by Rust-owned ledger projection."
    },
    {
      id: "ledger-event-002",
      revision: 2,
      eventType: "policy_checked",
      actor: "rust-core",
      evidenceRefs: 3,
      status: "ready",
      authority: "rust",
      summary: "Policy check result represented as display-only ledger timeline data."
    },
    {
      id: "ledger-event-003",
      revision: 3,
      eventType: "validation_recorded",
      actor: "rust-core",
      evidenceRefs: 4,
      status: "ready",
      authority: "rust",
      summary: "Validation event projection shown for review without edit capability."
    }
  ],
  replay: {
    readiness: "replayable",
    integrity: "consistent",
    eventsReplayed: 3,
    status: "ready",
    authority: "rust",
    summary: "Replay readiness projection is static and non-authoritative in the UI."
  },
  replayDetail: {
    readiness: "ready_for_replay",
    integrity: "integrity_verified",
    reconstructionStatus: "reconstruction_reported_by_rust_projection",
    finalLifecycle: "Validated",
    finalRevision: 3,
    eventsSeen: 3,
    stateTransitionsApplied: 3,
    authority: "rust",
    summary: "Replay detail is fixture display text; UI does not run replay reconstruction."
  },
  audit: {
    projections: 2,
    latestSummary: "Audit projection fixture summarizes replay and validation evidence.",
    authority: "rust",
    summary: "Audit summary is read-only projection data owned by Rust authority boundaries."
  },
  auditDetails: [
    {
      id: "audit-projection-001",
      projectionType: "ledger_alignment",
      source: "ledger timeline fixture",
      authority: "rust",
      summary: "Ledger revisions align with replay detail revision in fixture projections.",
      details: ["Sequential revisions: 1 to 3.", "No mutation controls are exposed in this UI phase."]
    },
    {
      id: "audit-projection-002",
      projectionType: "output_trust_boundary",
      source: "output projection fixture",
      authority: "rust",
      summary: "Raw model output remains untrusted while clean output is display-only.",
      details: ["rawOutputTrusted is false for raw model output.", "cleanOutputTrusted display value represents harness-cleaned fixture text only."]
    }
  ],
  output: {
    cleanOutputAvailable: true,
    rawOutputTrusted: false,
    authority: "rust",
    summary: "Clean output is previewable; raw model output remains untrusted."
  },
  cleanOutput: {
    id: "clean-output-001",
    cleanOutputAvailable: true,
    rawOutputTrusted: false,
    cleanOutputTrusted: true,
    authority: "rust",
    summary: "Clean output projection is read-only display data in this phase.",
    rawOutputSummary: "Raw model output is untrusted and not authoritative.",
    cleanOutputSummary: "Fixture indicates harness-cleaned output preview; UI does not infer or grant authority."
  },
  operatorIntentPreviews: [
    {
      id: "intent-preview-approve",
      intentType: "approve",
      label: "Approve candidate",
      description: "Non-authoritative operator request preview for approval review.",
      reasonPreview: "reason=operator reviewed evidence and requests acceptance",
      routePreview: "route=operator_intent_preview_only",
      authority: "operator",
      status: "unknown",
      disabled: true
    },
    {
      id: "intent-preview-reject",
      intentType: "reject",
      label: "Reject candidate",
      description: "Non-authoritative operator request preview for rejection review.",
      reasonPreview: "reason=operator requests rejection due to evidence mismatch",
      routePreview: "route=operator_intent_preview_only",
      authority: "operator",
      status: "unknown",
      disabled: false
    },
    {
      id: "intent-preview-retry",
      intentType: "retry",
      label: "Retry request",
      description: "Non-authoritative operator retry request preview in read-only UI.",
      reasonPreview: "reason=operator requests retry after reviewing blocked projection",
      routePreview: "route=operator_intent_preview_only",
      authority: "operator",
      status: "not_available",
      disabled: false
    },
    {
      id: "intent-preview-replay-request",
      intentType: "replay_request",
      label: "Replay request",
      description: "Non-authoritative replay request preview surface without execution behavior.",
      reasonPreview: "reason=operator requests replay review of ledger reconstruction",
      routePreview: "route=operator_intent_preview_only",
      authority: "ui",
      status: "not_available",
      disabled: false
    },
    {
      id: "intent-preview-context-rebuild",
      intentType: "context_rebuild_request",
      label: "Context rebuild request",
      description: "Non-authoritative context rebuild request preview for operator review only.",
      reasonPreview: "reason=operator requests context rebuild preview after stale slice inspection",
      routePreview: "route=operator_intent_preview_only",
      authority: "ui",
      status: "blocked",
      disabled: false
    },
    {
      id: "intent-preview-memory-snapshot",
      intentType: "memory_snapshot_request",
      label: "Memory snapshot request",
      description: "Non-authoritative memory snapshot request preview with no mutation behavior.",
      reasonPreview: "reason=operator requests memory snapshot preview for inspection",
      routePreview: "route=operator_intent_preview_only",
      authority: "operator",
      status: "not_available",
      disabled: false
    }
  ]
};
