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
  context: {
    packetId: "ctx-fixture-0001",
    slices: 4,
    sources: 2,
    budgetUsed: 320,
    budgetMax: 1024,
    authority: "rust",
    summary: "Static context packet projection placeholder."
  },
  memory: {
    snapshotId: "mem-fixture-0001",
    activeEntries: 8,
    disabledEntries: 1,
    rejectedEntries: 2,
    authority: "rust",
    summary: "Static memory projection placeholder for shell display."
  },
  ledger: {
    events: 12,
    lastRevision: 12,
    status: "ready",
    authority: "rust",
    summary: "Fixture ledger projection for read-only timeline summary."
  },
  replay: {
    readiness: "replayable",
    integrity: "consistent",
    eventsReplayed: 12,
    status: "ready",
    authority: "rust",
    summary: "Static replay projection preview with no execution behavior."
  },
  audit: {
    projections: 5,
    latestSummary: "Fixture audit summary generated for display-only UI shell.",
    authority: "ui",
    summary: "Read-only audit projection summary placeholder."
  },
  output: {
    cleanOutputAvailable: true,
    rawOutputTrusted: false,
    authority: "rust",
    summary: "Clean output is previewable; raw model output remains untrusted."
  }
};
