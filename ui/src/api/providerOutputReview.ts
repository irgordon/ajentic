import {
  projectLocalProviderExecution,
  projectLocalProviderOutputValidation,
  type LocalOperatorShellState,
  type LocalProviderExecutionProjection,
  type LocalProviderOutputValidationProjection
} from "./localOperatorShell";

export type ProviderOutputReviewProjection = Readonly<{
  execution: LocalProviderExecutionProjection;
  validation: LocalProviderOutputValidationProjection;
}>;

export function projectProviderOutputReview(state: LocalOperatorShellState): ProviderOutputReviewProjection {
  return {
    execution: projectLocalProviderExecution(state),
    validation: projectLocalProviderOutputValidation(state)
  };
}

function joined(values: readonly string[]): string {
  return values.length === 0 ? "none" : values.join(", ");
}

function validationStateMessage(validation: LocalProviderOutputValidationProjection): string {
  if (validation.status === "not_validated") return "No provider output has been validated.";
  if (validation.status === "validation_not_applicable") return `Validation is not applicable: ${joined(validation.reasons)}.`;
  if (validation.status === "invalid_validation_input") return `Validation input is invalid: ${joined(validation.reasons)}.`;
  if (validation.status === "rejected") return `Rejected before review with closed reason(s): ${joined(validation.reasons)}.`;
  return "Visible for human inspection only.";
}

function reviewVisualState(validation: LocalProviderOutputValidationProjection): string {
  if (validation.status === "reviewable_untrusted") return "inspection-only reviewable_untrusted";
  if (validation.status === "rejected") return "rejected not reviewable";
  if (validation.status === "validation_not_applicable") return "validation not applicable";
  if (validation.status === "invalid_validation_input") return "invalid validation input";
  return "not validated";
}

function noEffectSummary(validation: LocalProviderOutputValidationProjection): string {
  return [
    `trust effect=${validation.trustEffect}`,
    `candidate effect=${validation.candidateEffect}`,
    `decision ledger effect=${validation.decisionLedgerEffect}`,
    `replay effect=${validation.replayEffect}`,
    `export effect=${validation.exportEffect}`,
    `action effect=${validation.actionEffect}`,
    `readiness effect=${validation.readinessEffect}`,
    `release effect=${validation.releaseEffect}`,
    `deployment effect=${validation.deploymentEffect}`
  ].join(", ");
}

function escapeHtml(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

function detailRows(rows: readonly [string, string][]): string {
  return rows
    .map(([label, value]) => `<div><dt>${escapeHtml(label)}</dt><dd>${escapeHtml(value)}</dd></div>`)
    .join("");
}

export function renderProviderOutputReviewText(state: LocalOperatorShellState): string {
  return renderProviderOutputReviewProjectionText(projectProviderOutputReview(state));
}

export function renderProviderOutputReviewProjectionText(review: ProviderOutputReviewProjection): string {
  const { execution, validation } = review;
  return [
    "Provider output review",
    "Execution result",
    `Provider kind: ${validation.providerKind}`,
    `Execution result ID: ${validation.providerExecutionResultId ?? "none"}`,
    `Execution status: ${execution.status}`,
    `Output summary: ${execution.result?.outputSummary ?? "none"}`,
    "Validation result",
    `Validation status: ${validation.status}`,
    `Validation state detail: ${validationStateMessage(validation)}`,
    "Reviewability",
    `Reviewability status: ${validation.reviewabilityStatus}`,
    `Review visual state: ${reviewVisualState(validation)}`,
    "Candidate boundary",
    `Candidate-boundary status: ${validation.candidateBoundaryStatus}`,
    `Candidate-boundary details: ${joined(validation.candidateBoundaryStatuses)}`,
    "Rejection reasons",
    `Validation reasons: ${joined(validation.reasons)}`,
    "Trust and promotion",
    `Output trust status: ${validation.outputTrustStatus}`,
    `Promotion status: ${validation.outputPromotionStatus}`,
    "No-effect summary",
    noEffectSummary(validation),
    "Absence markers",
    `Absence marker summary: ${joined(execution.absenceMarkers.markerSummary)}`,
    "Absence markers show prohibited or inactive effects. They do not mean the output is safe or ready.",
    "absence markers are not safety; absence markers are not readiness; rendered only as negative capability/effect markers.",
    "Phase 144 boundary",
    "Reviewable untrusted output is visible for inspection only. It is not candidate material and cannot be approved in this phase.",
    "reviewable_untrusted inspection-only: not candidate material; cannot be approved in Phase 144; candidate conversion not performed; future conversion boundary required.",
    "No approve/reject controls for provider output are present; review UI is display-only and does not mutate decision ledger, replay state, export state, provider configuration, provider execution result, or provider output validation."
  ].join("\n");
}

export function renderProviderOutputReviewHtml(state: LocalOperatorShellState): string {
  const review = projectProviderOutputReview(state);
  const { execution, validation } = review;
  const statusClass = validation.status === "reviewable_untrusted"
    ? "provider-output-review__status--reviewable"
    : validation.status === "rejected"
      ? "provider-output-review__status--rejected"
      : "provider-output-review__status--neutral";

  return `
    <section class="provider-output-review" aria-label="Provider output review">
      <div class="panel__header">
        <h2>Provider output review</h2>
        <span class="provider-output-review__status ${statusClass}">${escapeHtml(reviewVisualState(validation))}</span>
      </div>
      <p class="boundary-note">Reviewable untrusted output is visible for inspection only. It is not candidate material and cannot be approved in this phase.</p>
      <p class="boundary-note">Absence markers show prohibited or inactive effects. They do not mean the output is safe or ready.</p>
      <div class="provider-output-review__grid">
        <section>
          <h3>Execution result</h3>
          <dl>${detailRows([
            ["Provider kind", validation.providerKind],
            ["Execution result ID", validation.providerExecutionResultId ?? "none"],
            ["Execution status", execution.status],
            ["Output summary", execution.result?.outputSummary ?? "none"],
            ["Output trust status", validation.outputTrustStatus],
            ["Promotion status", validation.outputPromotionStatus]
          ])}</dl>
        </section>
        <section>
          <h3>Validation result</h3>
          <dl>${detailRows([
            ["Validation status", validation.status],
            ["Reviewability status", validation.reviewabilityStatus],
            ["Candidate-boundary status", validation.candidateBoundaryStatus],
            ["Candidate-boundary details", joined(validation.candidateBoundaryStatuses)],
            ["Validation state detail", validationStateMessage(validation)],
            ["Validation reasons", joined(validation.reasons)]
          ])}</dl>
        </section>
      </div>
      <section>
        <h3>Rejection reasons</h3>
        <p class="${validation.status === "rejected" ? "provider-output-review__rejection" : "muted"}">${escapeHtml(validation.status === "rejected" ? joined(validation.reasons) : validationStateMessage(validation))}</p>
      </section>
      <section>
        <h3>Absence markers</h3>
        <p>${escapeHtml(joined(execution.absenceMarkers.markerSummary))}</p>
        <p class="muted">absence markers are not safety; absence markers are not readiness; rendered only as negative capability/effect markers.</p>
      </section>
      <section>
        <h3>No-effect summary</h3>
        <p>${escapeHtml(noEffectSummary(validation))}</p>
      </section>
      <section>
        <h3>Phase 144 boundary</h3>
        <p>reviewable_untrusted inspection-only: not candidate material; cannot be approved in Phase 144; candidate conversion not performed; future conversion boundary required.</p>
        <p class="muted">No approve/reject controls for provider output are present; review UI is display-only and does not mutate decision ledger, replay state, export state, provider configuration, provider execution result, or provider output validation.</p>
      </section>
    </section>`;
}
