import { createLocalOperatorShellTransport, requestDeterministicStubRun, submitLocalOperatorIntent } from "../api/localOperatorShellTransport.js";
import { initialReviewPreferences, type ReviewView } from "./reviewModel.js";
import { renderReviewWorkspace } from "./reviewWorkspace.js";
import { renderReviewDiagnostics } from "./reviewDiagnostics.js";

export const reviewWorkspaceTests = [
  { name: "review_workspace_starts_with_an_explicit_sample_not_real_file_output", run: () => {
    const html = renderReviewWorkspace(fixture(), "");
    contains(html, "Sample Comparison"); contains(html, "No Files Written");
    contains(html, "Illustrative comparison only"); contains(html, "Attachments Not Connected");
    contains(html, 'id="approve-run" class="primary-button" disabled');
  } },
  { name: "review_workspace_escapes_notes_and_candidate_content", run: () => {
    const view = fixture(); view.preferences.notes = '</textarea><img src=x onerror=alert(1)>';
    const html = renderReviewWorkspace(view, "");
    contains(html, "&lt;/textarea&gt;&lt;img");
    excludes(html, '</textarea><img src=x');
    const run = requestDeterministicStubRun(createLocalOperatorShellTransport());
    const state = { ...run.state, run: { ...run.state.run, candidate: { ...run.state.run.candidate!, body: "<script>alert('candidate')</script>" } } };
    view.preferences.tab = "result";
    const candidateHtml = renderReviewWorkspace({ ...view, state }, "");
    contains(candidateHtml, "&lt;script&gt;"); excludes(candidateHtml, "<script>");
  } },
  { name: "review_workspace_context_toggle_reveals_additional_recorded_sample_lines", run: () => {
    const view = fixture(); excludes(renderReviewWorkspace(view, ""), "# End of sample");
    view.preferences.expandedContext = true;
    contains(renderReviewWorkspace(view, ""), "# End of sample");
  } },
  { name: "review_workspace_records_review_without_enabling_authority", run: () => {
    const transport = createLocalOperatorShellTransport(); const run = requestDeterministicStubRun(transport);
    const response = submitLocalOperatorIntent(transport, { kind: "approve", operatorId: "reviewer", targetRunId: run.state.run.runId, reason: "sample review" });
    const html = renderReviewWorkspace({ ...fixture(), state: response.state }, "");
    contains(html, "Approval recorded for this local sample."); contains(html, 'id="approve-run" class="primary-button" disabled');
    contains(html, "No tool execution or file writes.");
  } },
  { name: "review_workspace_keeps_rejected_requests_visible_in_every_view", run: () => {
    for (const tab of ["compare", "result", "activity", "checks", "diagnostics"] as const) {
      const view = fixture(); view.preferences.tab = tab;
      const html = renderReviewWorkspace({ ...view, feedback: { status: "rejected", reason: "provider_execution_result_id_mismatch" } }, "");
      contains(html, "Request Rejected"); contains(html, "provider_execution_result_id_mismatch");
      contains(html, "No draft output was created.");
    }
  } },
  { name: "review_workspace_preserves_diagnostic_controls_with_short_labels", run: () => {
    const html = renderReviewDiagnostics(fixture().state);
    contains(html, "Sample Connector"); contains(html, 'id="submit-adapter-declaration"');
    contains(html, "Record Draft Decision"); contains(html, 'id="materialize-local-candidate"');
    excludes(html, "Declare local test adapter contract");
  } },
  { name: "review_workspace_exposes_missing_and_blocked_evidence_counts", run: () => {
    const html = renderReviewWorkspace(fixture(), "");
    contains(html, "Need Attention"); contains(html, "Not Started"); contains(html, "Rejected");
    contains(html, 'data-action="tab-checks"');
  } },
  { name: "review_workspace_keeps_view_preferences_out_of_runtime_state", run: () => {
    const view = fixture(); const original = JSON.stringify(view.state);
    view.preferences.showDetails = true; view.preferences.notes = "private local note";
    renderReviewWorkspace(view, "");
    if (JSON.stringify(view.state) !== original) throw new Error("Presentation mutated runtime projection");
  } },
];

function fixture(): ReviewView {
  return { state: createLocalOperatorShellTransport().getCurrentState().state, preferences: initialReviewPreferences(), feedback: { status: "available", reason: "local example" } };
}

function contains(html: string, expected: string): void {
  if (!html.includes(expected)) throw new Error(`Expected rendered content: ${expected}`);
}

function excludes(html: string, unexpected: string): void {
  if (html.includes(unexpected)) throw new Error(`Unexpected rendered content: ${unexpected}`);
}
