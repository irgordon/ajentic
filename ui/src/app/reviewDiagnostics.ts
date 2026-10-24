import { localCandidateMaterializationRequestFromState, type LocalOperatorShellState } from "../api/localOperatorShell.js";
import { renderProviderOutputReviewHtml } from "../api/providerOutputReview.js";
import { escapeHtml } from "../display/markup.js";
import * as views from "./reviewDiagnosticViews.js";

type Diagnostic = readonly [string, string];

export function renderReviewDiagnostics(state: LocalOperatorShellState): string {
  return diagnosticSections(state).map(renderDiagnosticSection).join("");
}

function diagnosticSections(state: LocalOperatorShellState): Diagnostic[] {
  return DIAGNOSTICS.map(([title, renderer]) => [title, renderer(state)]);
}

function renderDiagnosticSection([title, body]: Diagnostic): string {
  return `<details class="diagnostic-section" data-open-key="${title}"><summary>${title}</summary><div class="diagnostic-content">${body}</div></details>`;
}

function renderWorkflow(state: LocalOperatorShellState): string {
  return views.renderCompleteLocalOperatorWorkflow(state.completeLocalOperatorWorkflow);
}

function renderStagedDecision(state: LocalOperatorShellState): string {
  const decision = state.operatorCandidateDecision;
  return `<p>Status: <code>${escapeHtml(decision.status)}</code></p><p>Decision: <code>${escapeHtml(decision.record?.decisionId ?? "none")}</code></p>
    <button id="approve-staged-proposal" ${state.stagedCandidateConversionValidation.status === "staged_proposal_shape_valid" ? "" : "disabled"}>Record Draft Decision</button>
    <p class="muted">Records local intent for the validated draft only. No tool execution or release approval.</p>`;
}

function renderOutputRequest(state: LocalOperatorShellState): string {
  return `<p>Status: <code>${escapeHtml(state.localCandidateOutput.status)}</code></p><p>Candidate: <code>${escapeHtml(state.localCandidateOutput.candidateId ?? "none")}</code></p>
    <p>Rejection Reason: <code>${escapeHtml(state.localCandidateOutput.error ?? "none")}</code></p>
    <button id="materialize-local-candidate" ${localCandidateMaterializationRequestFromState(state) ? "" : "disabled"}>Create Draft Output</button>
    <p class="muted">Local draft representation only. This does not write files or authorize actions.</p>`;
}

const DIAGNOSTICS: readonly [string, (state: LocalOperatorShellState) => string][] = [
  ["Release Checks", views.renderReleaseCandidatePreparation],
  ["Candidate Bundle", views.renderReleaseArtifactDryPackage],
  ["Distribution Details", views.renderInstallerDistributionContract],
  ["Attestation Details", views.renderSigningKeyCustodyDryRun],
  ["Evidence", views.renderReleaseCandidateEvidenceAssembly],
  ["Open Issues", views.renderReleaseCandidateGapReview],
  ["Workflow", renderWorkflow],
  ["Test Guide", views.renderTrialOperatorRunbook],
  ["Failure Tests", views.renderTrialFailureDrill],
  ["Stop Conditions", views.renderStopConditionDrill],
  ["Get Help", views.renderEscalationGuidance],
  ["Test Activity", views.renderTrialObservability],
  ["Test Errors", views.renderTrialErrorReporting],
  ["Blockers", views.renderTrialBlockedStateSummary],
  ["Sample Connector", views.renderAdapterRegistry],
  ["Connector Check", views.renderAdapterDryRun],
  ["Sample Request", views.renderConstrainedLocalProviderInvocation],
  ["Sample Settings", views.renderProviderConfiguration],
  ["Sample Tool", views.renderProviderExecution],
  ["Output Review", views.renderProviderOutputPipeline],
  ["Output Details", renderProviderOutputReviewHtml],
  ["Draft Checks", views.renderStagedProposalValidation],
  ["Draft Decision", renderStagedDecision],
  ["Output Request", renderOutputRequest],
  ["Session Summary", views.renderLocalSessionPackage],
  ["Session History", views.renderSessionHistory],
  ["Restore Details", views.renderLocalSessionRestore],
  ["Decision History", views.renderReplayProjection],
  ["Glossary", views.renderPlainEnglishGlossary],
];
