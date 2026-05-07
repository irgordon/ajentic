import type { ApplicationUiProjection, LocalRuntimeReviewSurface, UiOperatorIntentSubmissionContract, UiOperatorIntentSubmissionEnvelope, UiOperatorIntentSubmissionCapability, UiReadModel, UiRustIntentPreviewRequest, UiRustReadProjectionResponse, UiRustTransportCapability, UiSubmissionBoundaryInput } from "./projections";

export const applicationProjectionFixture: ApplicationUiProjection = {
  projectionId: "proj-fixture-0053",
  runtimeConfigId: "runtime-cfg-0053",
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
  lifecycle: { lifecycle: "Evaluating", revision: 53, status: "ready", authority: "rust", summary: "Lifecycle read projection from Rust-owned boundary." },
  run: { runId: "run-fixture-0053", title: "Phase 53 Typed Intent Submission Preview Fixture", status: "ready", currentLifecycle: "Evaluating", executionDecision: "allow", promotionDecision: "hold", replayReadiness: "replayable", cleanOutputAvailable: true, authority: "rust", summary: "Run projection is display-only and non-authoritative in UI." },
  provider: { providerKind: "deterministic_stub", outputStatus: "received", outputTrust: "untrusted", authoritative: false, authority: "provider", summary: "Provider output remains untrusted and non-authoritative." },
  integration: { sourceKind: "local_llm", outputStatus: "received", outputTrust: "untrusted", authoritative: false, authority: "integration", summary: "Integration output remains untrusted and non-authoritative." },
  ledger: { events: 3, lastRevision: 3, status: "ready", authority: "rust", summary: "Ledger summary is projection-only and does not append events from UI." },
  replay: { readiness: "replayable", integrity: "consistent", eventsReplayed: 3, status: "ready", authority: "rust", summary: "Replay summary is display-only and non-executing in UI." },
  audit: { projections: 2, latestSummary: "Audit projection summary from supplied read projection data.", authority: "rust", summary: "Audit projection is read-only and non-authoritative in UI." },
  context: { packetId: "ctx-fixture-0053", slices: 4, sources: 3, budgetUsed: 412, budgetMax: 1024, authority: "rust", summary: "Context summary from typed read projection metadata.", slicesPreview: [] },
  memory: { snapshotId: "mem-fixture-0053", activeEntries: 8, disabledEntries: 1, rejectedEntries: 2, authority: "rust", summary: "Memory summary from typed read projection metadata.", entriesPreview: [] },
  output: { cleanOutputAvailable: true, rawOutputTrusted: false, authority: "rust", summary: "Raw provider/model output remains untrusted and distinct from clean output." }
};

const previewBase = {
  authority: "operator" as const,
  status: "unknown" as const,
  disabled: true
};


export const localRuntimeReviewSurfaceFixture: LocalRuntimeReviewSurface = {
  surfaceId: "phase-103-local-runtime-review",
  title: "Phase 103 Local Runtime Review Surface",
  launchInstruction: "From the ui directory run npm run dev; the command prints a deterministic local-only review surface and does not start a server.",
  postureIndicators: [
    "local-only",
    "non-authoritative",
    "review-only",
    "not production-ready",
    "transport disabled",
    "provider execution disabled",
    "persistence authority disabled",
    "action execution disabled"
  ],
  disabledCapabilities: [
    { id: "transport", label: "Transport", status: "disabled", summary: "No live UI-to-Rust transport is started in Phase 103." },
    { id: "provider", label: "Provider execution", status: "disabled", summary: "Provider/model execution is not invoked by the UI runtime review surface." },
    { id: "persistence", label: "Persistence authority", status: "disabled", summary: "The UI does not write, append, export, or persist authority state." },
    { id: "recovery", label: "Recovery promotion", status: "disabled", summary: "Recovery promotion and replay repair are not available from the UI." },
    { id: "action", label: "Action execution", status: "disabled", summary: "Operator interactions remain local preview choices only." }
  ],
  reviewState: {
    workflowState: "workflow_visible_review_only",
    reviewState: "operator_review_pending_local_preview",
    escalationState: "manual_escalation_required_for_boundary_questions",
    failureState: "fail_closed_rejected_inputs_render_as_review_failures",
    evidenceState: "committed_fixture_and_behavior_test_evidence_only",
    validationStatus: "blocked",
    dryRunSummary: "Dry-run posture is rendered as local text; no provider, persistence, recovery, replay repair, or action behavior is started."
  },
  interactions: [
    {
      id: "review-valid-preview",
      label: "Preview policy-check intent",
      status: "previewed",
      result: {
        status: "accepted_for_preview",
        reasonCode: "accepted_for_preview",
        transportEligible: false,
        liveTransportCalled: false,
        liveTransportEnabled: false,
        executionEnabled: false,
        persistenceEnabled: false,
        ledgerRecordingEnabled: false,
        auditAppendEnabled: false,
        providerExecutionEnabled: false,
        replayRepairEnabled: false,
        mutatesAuthority: false,
        summary: "Accepted for local preview only."
      },
      summary: "A bounded operator review interaction can be previewed without transport or execution eligibility."
    },
    {
      id: "review-rejected-escalation",
      label: "Reject authority-escalation text",
      status: "rejected",
      result: {
        status: "rejected",
        reasonCode: "authority_escalation_text_rejected",
        transportEligible: false,
        liveTransportCalled: false,
        liveTransportEnabled: false,
        executionEnabled: false,
        persistenceEnabled: false,
        ledgerRecordingEnabled: false,
        auditAppendEnabled: false,
        providerExecutionEnabled: false,
        replayRepairEnabled: false,
        mutatesAuthority: false,
        summary: "Rejected before any live or authority path can be formed."
      },
      summary: "Failure-state rendering remains visible and fail-closed."
    }
  ],
  deterministicSummary: "Fixture-backed Phase 103 review data renders in a deterministic order for local operator testing.",
  nonReadinessStatement: "Phase 103 does not approve readiness, Production Candidate status, release-candidate readiness, production readiness, public usability, or production human use."
};

export const uiReadModelFixture: UiReadModel = {
  source: "fixture",
  application: applicationProjectionFixture,
  decisions: [
    { label: "Policy gate", decision: "allow", reason: "projection_display_only", status: "ready", authority: "rust" },
    { label: "Validation gate", decision: "pending", reason: "no_runtime_transport_in_phase_53", status: "unknown", authority: "ui" }
  ],
  policyDecisions: [],
  validationDecisions: [],
  executionDecisions: [],
  ledgerTimeline: [],
  replayDetail: { readiness: "ready_for_replay", integrity: "integrity_verified", reconstructionStatus: "reported_by_projection", finalLifecycle: "Validated", finalRevision: 3, eventsSeen: 3, stateTransitionsApplied: 3, authority: "rust", summary: "Replay detail remains non-executing UI text." },
  auditDetails: [],
  cleanOutput: { id: "clean-output-0053", cleanOutputAvailable: true, rawOutputTrusted: false, cleanOutputTrusted: true, authority: "rust", summary: "Clean output projection is read-only text.", rawOutputSummary: "Raw provider/model output remains untrusted and non-authoritative.", cleanOutputSummary: "Clean output preview is distinct display data and does not grant authority." },
  persistedRecordVerification: { status: "checksum_mismatch", recoveryAction: "manual_review_required", recordId: "record-0053", payloadKind: "run_record", revision: 3, checksum: "deadbeefdeadbeef", payloadLen: 144, summary: "Persisted record verification reports a checksum mismatch and remains read-only.", diagnostic: { family: "persistence_recovery", code: "checksum_mismatch", summary: "Persisted record envelope verification failed.", key: "persistence_recovery.checksum_mismatch" } },
  diagnostics: [
    { family: "operator_intent", code: "accepted_for_routing", summary: "Operator intent ingress accepted for routing preview.", key: "operator_intent.accepted_for_routing" },
    { family: "persistence_recovery", code: "checksum_mismatch", summary: "Persisted record envelope verification failed.", key: "persistence_recovery.checksum_mismatch" }
  ],
  localRuntimeReview: localRuntimeReviewSurfaceFixture,
  operatorIntentPreviews: [
    { id: "intent-preview-approve", intentType: "approve", label: "Approve candidate", description: "Submission-shaped request preview only.", reasonPreview: "operator requests approval review", routePreview: "operator_intent_preview_only", ...previewBase, submissionPreview: { submissionId: "sub-approve-0001", operatorId: "operator-phase53", intentType: "approve", targetKind: "run", targetId: "run-fixture-0053", reason: "Approve candidate for downstream policy review.", requestPreviewEnabled: true, submissionEnabled: false, authority: "operator", summary: "Submission-shaped preview only; no action executes." } },
    { id: "intent-preview-reject", intentType: "reject", label: "Reject candidate", description: "Submission-shaped request preview only.", reasonPreview: "operator requests rejection review", routePreview: "operator_intent_preview_only", ...previewBase, submissionPreview: { submissionId: "sub-reject-0001", operatorId: "operator-phase53", intentType: "reject", targetKind: "run", targetId: "run-fixture-0053", reason: "Reject candidate pending additional evidence.", requestPreviewEnabled: true, submissionEnabled: false, authority: "operator", summary: "Submission-shaped preview only; no action executes." } },
    { id: "intent-preview-retry", intentType: "retry", label: "Retry evaluation", description: "Submission-shaped request preview only.", reasonPreview: "operator requests retry review", routePreview: "operator_intent_preview_only", ...previewBase, submissionPreview: { submissionId: "sub-retry-0001", operatorId: "operator-phase53", intentType: "retry", targetKind: "run", targetId: "run-fixture-0053", reason: "Retry evaluation after non-authoritative review.", requestPreviewEnabled: true, submissionEnabled: false, authority: "operator", summary: "Submission-shaped preview only; no action executes." } },
    { id: "intent-preview-replay", intentType: "replay_request", label: "Replay request", description: "Submission-shaped request preview only.", reasonPreview: "operator requests replay review", routePreview: "operator_intent_preview_only", ...previewBase, submissionPreview: { submissionId: "sub-replay-0001", operatorId: "operator-phase53", intentType: "replay_request", targetKind: "replay", targetId: "replay-fixture-0053", reason: "Request replay review with no execution in UI.", requestPreviewEnabled: true, submissionEnabled: false, authority: "operator", summary: "Submission-shaped preview only; no action executes." } },
    { id: "intent-preview-context", intentType: "context_rebuild_request", label: "Context rebuild request", description: "Submission-shaped request preview only.", reasonPreview: "operator requests context rebuild review", routePreview: "operator_intent_preview_only", ...previewBase, submissionPreview: { submissionId: "sub-context-0001", operatorId: "operator-phase53", intentType: "context_rebuild_request", targetKind: "context", targetId: "ctx-fixture-0053", reason: "Request context rebuild review without UI execution.", requestPreviewEnabled: true, submissionEnabled: false, authority: "operator", summary: "Submission-shaped preview only; no action executes." } },
    { id: "intent-preview-memory", intentType: "memory_snapshot_request", label: "Memory snapshot request", description: "Submission-shaped request preview only.", reasonPreview: "operator requests memory snapshot review", routePreview: "operator_intent_preview_only", ...previewBase, submissionPreview: { submissionId: "sub-memory-0001", operatorId: "operator-phase53", intentType: "memory_snapshot_request", targetKind: "memory", targetId: "mem-fixture-0053", reason: "Request memory snapshot review without UI execution.", requestPreviewEnabled: false, submissionEnabled: false, authority: "operator", summary: "Submission-shaped preview only; no action executes." } }
  ]
};


export const uiRustTransportCapabilityFixture: UiRustTransportCapability = {
  transportEnabled: false,
  mutationEnabled: false,
  submissionEnabled: false,
  executionEnabled: false,
  persistenceEnabled: false
};

export const uiRustReadProjectionResponseFixture: UiRustReadProjectionResponse = {
  envelopeId: "transport-read-0053",
  direction: "rust_to_ui",
  status: "display_only",
  payload: uiReadModelFixture,
  capability: uiRustTransportCapabilityFixture,
  diagnostic: uiReadModelFixture.diagnostics[1],
  summary: "Transport-shaped read projection envelope only; live transport remains disabled."
};

export const uiRustIntentPreviewRequestFixture: UiRustIntentPreviewRequest = {
  envelopeId: "transport-intent-preview-0053",
  direction: "ui_to_rust",
  status: "display_only",
  payload: uiReadModelFixture.operatorIntentPreviews[0].submissionPreview,
  capability: uiRustTransportCapabilityFixture,
  diagnostic: uiReadModelFixture.diagnostics[0],
  summary: "Transport-shaped intent preview request only; submission and execution remain disabled."
};

export const uiOperatorIntentSubmissionCapabilityFixture: UiOperatorIntentSubmissionCapability = {
  submissionShapeEnabled: true,
  liveTransportEnabled: false,
  executionEnabled: false,
  persistenceEnabled: false,
  ledgerRecordingEnabled: false,
  auditAppendEnabled: false,
  providerExecutionEnabled: false,
  replayRepairEnabled: false,
  mutatesApplicationState: false
};

export const uiOperatorIntentSubmissionContractFixtures: readonly UiOperatorIntentSubmissionContract[] = [
  {
    submissionId: "sub-phase77-ingress-ready",
    operatorId: "operator-phase77",
    targetKind: "run",
    targetId: "run-fixture-0053",
    intentType: "approve",
    reason: "Route for ingress eligibility preview only.",
    status: "ingress_ready",
    capability: uiOperatorIntentSubmissionCapabilityFixture,
    summary: "Accepted-for-routing style local submission contract preview only; no action execution occurs."
  },
  {
    submissionId: "sub-phase77-auth-ready",
    operatorId: "operator-phase77",
    targetKind: "run",
    targetId: "run-fixture-0053",
    intentType: "policy_check",
    reason: "Prepare authorization-shape preview only.",
    status: "authorization_ready",
    capability: uiOperatorIntentSubmissionCapabilityFixture,
    summary: "Authorization-ready local submission contract preview only; no action execution occurs."
  },
  {
    submissionId: "sub-phase77-audit-eligible",
    operatorId: "operator-phase77",
    targetKind: "replay",
    targetId: "replay-fixture-0053",
    intentType: "replay_request",
    reason: "Prepare audit-eligibility preview only.",
    status: "audit_eligible",
    capability: uiOperatorIntentSubmissionCapabilityFixture,
    summary: "Audit-eligible local submission contract preview only; no action execution occurs."
  },
  {
    submissionId: "sub-phase77-rejected",
    operatorId: "operator-phase77",
    targetKind: "unknown",
    targetId: "",
    intentType: "retry",
    reason: "Deliberate ineligible preview fixture.",
    status: "rejected",
    capability: uiOperatorIntentSubmissionCapabilityFixture,
    diagnostic: { family: "operator_intent", code: "route_missing", summary: "Submission preview is ineligible because routing context is unavailable.", key: "operator_intent.route_missing" },
    summary: "Rejected local submission contract preview only; no action execution occurs."
  }
];

export const uiOperatorIntentSubmissionEnvelopeFixture: UiOperatorIntentSubmissionEnvelope = {
  envelopeId: "transport-intent-submission-0077",
  direction: "ui_to_rust",
  status: "display_only",
  payload: uiOperatorIntentSubmissionContractFixtures[0],
  capability: uiRustTransportCapabilityFixture,
  summary: "Submission-shaped envelope is local display data only; no live transport or action execution occurs."
};


export const uiSubmissionBoundaryAcceptedPreviewFixture: UiSubmissionBoundaryInput = {
  operatorId: "operator-phase91",
  targetId: "run-fixture-0053",
  intentKind: "policy_check",
  reason: "Prepare local preview without execution."
};

export const uiSubmissionBoundaryMalformedFixtures: readonly UiSubmissionBoundaryInput[] = [
  { operatorId: "", targetId: "run-fixture-0053", intentKind: "policy_check", reason: "empty operator" },
  { operatorId: "operator-phase91", targetId: "", intentKind: "policy_check", reason: "empty target" },
  { operatorId: "operator-phase91", targetId: "run-fixture-0053", intentKind: "", reason: "empty intent" },
  { operatorId: "operator-phase91", targetId: "run-fixture-0053", intentKind: "root_override", reason: "unknown intent" }
];

export const uiSubmissionBoundaryRiskyTextFixtures: readonly UiSubmissionBoundaryInput[] = [
  { operatorId: "operator-phase91", targetId: "run-fixture-0053", intentKind: "policy_check", reason: "admin override" },
  { operatorId: "operator-phase91", targetId: "run-fixture-0053", intentKind: "policy_check", reason: "skip policy" },
  { operatorId: "operator-phase91", targetId: "run-fixture-0053", intentKind: "policy_check", reason: "execute now" },
  { operatorId: "operator-phase91", targetId: "run-fixture-0053", intentKind: "policy_check", reason: "write ledger" },
  { operatorId: "operator-phase91", targetId: "run-fixture-0053", intentKind: "policy_check", reason: "append audit" },
  { operatorId: "operator-phase91", targetId: "run-fixture-0053", intentKind: "policy_check", reason: "repair replay" },
  { operatorId: "operator-phase91", targetId: "run-fixture-0053", intentKind: "policy_check", reason: "trust provider output" },
  { operatorId: "operator-phase91", targetId: "run-fixture-0053", intentKind: "policy_check", reason: "promote recovered state" }
];

export const uiSubmissionBoundarySpoofedCapabilityFixture: UiSubmissionBoundaryInput = {
  operatorId: "operator-phase91",
  targetId: "run-fixture-0053",
  intentKind: "policy_check",
  reason: "Attempt spoofed capability flags.",
  executionEnabled: true,
  liveTransportEnabled: true,
  persistenceEnabled: true,
  ledgerRecordingEnabled: true,
  auditAppendEnabled: true,
  providerExecutionEnabled: true,
  replayRepairEnabled: true,
  mutatesAuthority: true
};
