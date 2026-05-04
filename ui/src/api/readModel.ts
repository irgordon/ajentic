import { applicationProjectionFixture, uiReadModelFixture } from "./fixtures";
import type { ApplicationUiProjection, IntentSubmissionUiProjection, OperatorIntentPreviewProjection, UiReadModel } from "./projections";

export const UI_READ_MODEL_IS_SYNC = true as const;
export const UI_READ_MODEL_SOURCE = "fixture_or_supplied_projection" as const;
export const UI_READ_MODEL_MUTATION_CAPABLE = false as const;
export const UI_INTENT_SUBMISSION_ENABLED = false as const;
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
