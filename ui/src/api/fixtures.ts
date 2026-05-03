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
  policyDecisions: [
    {
      id: "policy-decision-001",
      label: "Policy gate: execution path",
      decision: "allow",
      reason: "policy_projection_fixture_allow",
      status: "ready",
      authority: "rust",
      summary: "Execution path remains within fixture-defined policy boundaries.",
      evidenceSummary: "Evidence references approved lifecycle state and policy boundary checks in Rust-owned projections."
    },
    {
      id: "policy-decision-002",
      label: "Policy gate: authority boundary",
      decision: "hold",
      reason: "operator_intent_not_submitted",
      status: "blocked",
      authority: "rust",
      summary: "No operator intent submission is represented in this fixture projection.",
      evidenceSummary: "Evidence indicates display-only UI surface with no mutation or bypass controls enabled."
    }
  ],
  validationDecisions: [
    {
      id: "validation-decision-001",
      label: "Validation gate: candidate completeness",
      decision: "pass",
      reason: "fixture_projection_validated",
      status: "ready",
      authority: "rust",
      summary: "Candidate projection is marked as validated in fixture data.",
      evidenceSummary: "Evidence summary includes deterministic validation result from Rust-owned projection fields."
    },
    {
      id: "validation-decision-002",
      label: "Validation gate: evidence integrity",
      decision: "pending",
      reason: "no_live_runtime_inputs",
      status: "unknown",
      authority: "rust",
      summary: "Validation integrity is not re-evaluated in the UI projection layer.",
      evidenceSummary: "Evidence is fixture-backed and read-only; UI does not run validation logic."
    }
  ],
  executionDecisions: [
    {
      id: "execution-decision-001",
      label: "Execution decision projection",
      decision: "hold",
      reason: "display_only_surface",
      status: "blocked",
      authority: "rust",
      summary: "Execution decision is displayed for review and not triggered from the UI.",
      evidenceSummary: "Evidence shows execution status comes from fixture projection data with no execution controls."
    }
  ],
  context: {
    packetId: "ctx-fixture-0024",
    slices: 5,
    sources: 3,
    budgetUsed: 412,
    budgetMax: 1024,
    authority: "rust",
    summary: "Static context packet projection placeholder.",
    slicesPreview: [
      {
        id: "slice-docs-governance",
        viewType: "docs",
        truthDimension: "normative",
        sourcePath: "docs/governance/GOVERNANCE.md",
        authority: "rust",
        provenance: "fixture:phase24:governance-doc",
        summary: "Governance constraints included for authority boundary review."
      },
      {
        id: "slice-task-phase24",
        viewType: "task",
        truthDimension: "procedural",
        sourcePath: "checklists/current-phase.md",
        authority: "operator",
        provenance: "fixture:phase24:active-checklist",
        summary: "Active phase checklist scope and constraints for current UI work."
      },
      {
        id: "slice-schema-memory",
        viewType: "schema",
        truthDimension: "contract",
        sourcePath: "schemas/memory/",
        authority: "rust",
        provenance: "fixture:phase24:memory-schema-index",
        summary: "Memory contract references included as non-authoritative read context."
      }
    ]
  },
  memory: {
    snapshotId: "mem-fixture-0024",
    activeEntries: 8,
    disabledEntries: 1,
    rejectedEntries: 2,
    authority: "rust",
    summary: "Static memory projection placeholder for shell display.",
    entriesPreview: [
      {
        id: "mem-entry-001",
        memoryType: "policy-note",
        status: "ready",
        authority: "rust",
        provenance: "fixture:phase24:memory-ready",
        summary: "Accepted policy guidance retained for read-only operator inspection."
      },
      {
        id: "mem-entry-002",
        memoryType: "task-cache",
        status: "blocked",
        authority: "ui",
        provenance: "fixture:phase24:memory-blocked",
        summary: "Blocked task context due to missing runtime validation path."
      },
      {
        id: "mem-entry-003",
        memoryType: "candidate-output",
        status: "rejected",
        authority: "rust",
        provenance: "fixture:phase24:memory-rejected",
        summary: "Rejected candidate output remains non-authoritative and untrusted."
      }
    ]
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
