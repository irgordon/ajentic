import { applicationProjectionFixture, uiOperatorIntentSubmissionContractFixtures, uiOperatorIntentSubmissionEnvelopeFixture, uiReadModelFixture, uiRustIntentPreviewRequestFixture, uiRustReadProjectionResponseFixture, uiRustTransportCapabilityFixture } from "./fixtures";
import type { ApplicationUiProjection, IntentSubmissionUiProjection, IntentTypeProjection, OperatorIntentPreviewProjection, UiOperatorIntentSubmissionContract, UiOperatorIntentSubmissionEnvelope, UiReadModel, UiRustIntentPreviewRequest, UiRustReadProjectionResponse, UiSubmissionBoundaryInput, UiSubmissionBoundaryReasonCode, UiSubmissionBoundaryResult } from "./projections";

export const UI_READ_MODEL_IS_SYNC = true as const;
export const UI_READ_MODEL_SOURCE = "fixture_or_supplied_projection" as const;
export const UI_READ_MODEL_MUTATION_CAPABLE = false as const;
export const UI_INTENT_SUBMISSION_ENABLED = true as const;
export const UI_INTENT_EXECUTION_ENABLED = false as const;
export const UI_INTENT_LEDGER_RECORDING_ENABLED = false as const;


const UI_ALLOWED_INTENT_KINDS: readonly IntentTypeProjection[] = [
  "approve",
  "reject",
  "retry",
  "memory_write",
  "memory_delete",
  "run_start",
  "run_stop",
  "tool_request",
  "policy_check",
  "state_transition_request",
  "context_rebuild_request",
  "replay_request",
  "memory_snapshot_request",
  "memory_disable_request"
];

const UI_RISKY_SUBMISSION_TEXT = [
  "admin override",
  "skip policy",
  "execute now",
  "write ledger",
  "append audit",
  "repair replay",
  "trust provider output",
  "promote recovered state"
] as const;

const UI_SPOOFED_FLAG_REASON_CODES: readonly [keyof UiSubmissionBoundaryInput, UiSubmissionBoundaryReasonCode][] = [
  ["liveTransportEnabled", "live_transport_flag_spoof_rejected"],
  ["executionEnabled", "execution_flag_spoof_rejected"],
  ["persistenceEnabled", "persistence_flag_spoof_rejected"],
  ["ledgerRecordingEnabled", "ledger_recording_flag_spoof_rejected"],
  ["auditAppendEnabled", "audit_append_flag_spoof_rejected"],
  ["providerExecutionEnabled", "provider_execution_flag_spoof_rejected"],
  ["replayRepairEnabled", "replay_repair_flag_spoof_rejected"],
  ["mutatesAuthority", "authority_mutation_flag_spoof_rejected"],
  ["mutatesApplicationState", "authority_mutation_flag_spoof_rejected"]
] as const;

function rejectedSubmissionBoundaryResult(reasonCode: UiSubmissionBoundaryReasonCode): UiSubmissionBoundaryResult {
  return {
    status: "rejected",
    reasonCode,
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
    summary: "Submission boundary rejected malformed or spoofed input before transport eligibility."
  };
}

function acceptedPreviewSubmissionBoundaryResult(): UiSubmissionBoundaryResult {
  return {
    status: "accepted_for_preview",
    reasonCode: "accepted_for_preview",
    transportEligible: false,
    liveTransportCalled: false,
    liveTransportEnabled: false,
    executionEnabled: UI_INTENT_EXECUTION_ENABLED,
    persistenceEnabled: false,
    ledgerRecordingEnabled: UI_INTENT_LEDGER_RECORDING_ENABLED,
    auditAppendEnabled: false,
    providerExecutionEnabled: false,
    replayRepairEnabled: false,
    mutatesAuthority: UI_READ_MODEL_MUTATION_CAPABLE,
    summary: "Submission is accepted for local preview only; it is not live, executable, persistent, or authority-mutating."
  };
}

function isRecord(value: unknown): value is UiSubmissionBoundaryInput {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function normalizedText(value: unknown): string | null {
  if (typeof value !== "string") return null;
  return value.trim();
}

function hasSpoofedCapabilityFlag(input: UiSubmissionBoundaryInput): UiSubmissionBoundaryReasonCode | null {
  for (const [flagName, reasonCode] of UI_SPOOFED_FLAG_REASON_CODES) {
    if (Object.prototype.hasOwnProperty.call(input, flagName)) return reasonCode;
  }
  return null;
}

function hasRiskySubmissionText(reason: string): boolean {
  const normalizedReason = reason.toLowerCase();
  return UI_RISKY_SUBMISSION_TEXT.some((riskyText) => normalizedReason.includes(riskyText));
}

export function buildUiSubmissionBoundaryResult(input: unknown): UiSubmissionBoundaryResult {
  if (!isRecord(input)) return rejectedSubmissionBoundaryResult("malformed_submission_rejected");

  const spoofedFlagReason = hasSpoofedCapabilityFlag(input);
  if (spoofedFlagReason) return rejectedSubmissionBoundaryResult(spoofedFlagReason);

  const operatorId = normalizedText(input.operatorId);
  if (!operatorId) return rejectedSubmissionBoundaryResult("empty_operator_id");

  const targetId = normalizedText(input.targetId);
  if (!targetId) return rejectedSubmissionBoundaryResult("empty_target_id");

  const intentKind = normalizedText(input.intentKind ?? input.intentType);
  if (!intentKind) return rejectedSubmissionBoundaryResult("empty_intent_kind");
  if (!UI_ALLOWED_INTENT_KINDS.includes(intentKind as IntentTypeProjection)) return rejectedSubmissionBoundaryResult("unknown_intent_kind");

  const reason = normalizedText(input.reason);
  if (reason && hasRiskySubmissionText(reason)) return rejectedSubmissionBoundaryResult("authority_escalation_text_rejected");

  return acceptedPreviewSubmissionBoundaryResult();
}

export function buildIntentSubmissionPreview(preview: OperatorIntentPreviewProjection): IntentSubmissionUiProjection {
  return {
    ...preview.submissionPreview,
    submissionEnabled: UI_INTENT_SUBMISSION_ENABLED,
    summary: "Submission-shaped request preview only; no submission or action execution occurs."
  };
}

export function buildUiReadModelFromApplicationProjection(projection: ApplicationUiProjection): UiReadModel {
  return {
    ...uiReadModelFixture,
    source: "supplied_projection",
    application: {
      ...projection,
      provider: { ...projection.provider, outputTrust: "untrusted", authoritative: false, summary: "Provider output remains untrusted and non-authoritative." },
      integration: { ...projection.integration, outputTrust: "untrusted", authoritative: false, summary: "Integration output remains untrusted and non-authoritative." },
      output: { ...projection.output, rawOutputTrusted: false }
    },
    cleanOutput: {
      ...uiReadModelFixture.cleanOutput,
      cleanOutputAvailable: projection.output.cleanOutputAvailable,
      rawOutputTrusted: false
    },
    operatorIntentPreviews: uiReadModelFixture.operatorIntentPreviews.map((preview) => ({
      ...preview,
      submissionPreview: buildIntentSubmissionPreview(preview)
    }))
  };
}

export function getUiReadModel(projection?: ApplicationUiProjection): UiReadModel {
  if (projection) {
    return buildUiReadModelFromApplicationProjection(projection);
  }
  return buildUiReadModelFromApplicationProjection(applicationProjectionFixture);
}


export function buildUiRustReadProjectionResponse(projection?: ApplicationUiProjection): UiRustReadProjectionResponse {
  const payload = getUiReadModel(projection);
  return {
    ...uiRustReadProjectionResponseFixture,
    payload,
    capability: uiRustTransportCapabilityFixture,
    summary: "Transport-shaped read projection envelope only; live transport remains disabled."
  };
}

export function buildUiRustIntentPreviewRequest(preview: OperatorIntentPreviewProjection): UiRustIntentPreviewRequest {
  return {
    ...uiRustIntentPreviewRequestFixture,
    payload: buildIntentSubmissionPreview(preview),
    capability: uiRustTransportCapabilityFixture,
    summary: "Transport-shaped intent preview request only; submission and execution remain disabled."
  };
}

export function getUiOperatorIntentSubmissionContracts(): readonly UiOperatorIntentSubmissionContract[] {
  return uiOperatorIntentSubmissionContractFixtures;
}

export function buildUiOperatorIntentSubmissionEnvelope(
  contract: UiOperatorIntentSubmissionContract
): UiOperatorIntentSubmissionEnvelope {
  return {
    ...uiOperatorIntentSubmissionEnvelopeFixture,
    payload: contract,
    summary: "Submission-shaped envelope is local-only contract construction; no live transport or action execution occurs."
  };
}
