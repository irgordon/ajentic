import { applicationProjectionFixture, uiOperatorIntentSubmissionContractFixtures, uiOperatorIntentSubmissionEnvelopeFixture, uiReadModelFixture, uiRustIntentPreviewRequestFixture, uiRustReadProjectionResponseFixture, uiRustTransportCapabilityFixture } from "./fixtures";
import type { ApplicationUiProjection, IntentSubmissionUiProjection, OperatorIntentPreviewProjection, UiOperatorIntentSubmissionContract, UiOperatorIntentSubmissionEnvelope, UiReadModel, UiRustIntentPreviewRequest, UiRustReadProjectionResponse } from "./projections";

export const UI_READ_MODEL_IS_SYNC = true as const;
export const UI_READ_MODEL_SOURCE = "fixture_or_supplied_projection" as const;
export const UI_READ_MODEL_MUTATION_CAPABLE = false as const;
export const UI_INTENT_SUBMISSION_ENABLED = true as const;
export const UI_INTENT_EXECUTION_ENABLED = false as const;
export const UI_INTENT_LEDGER_RECORDING_ENABLED = false as const;


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
