import {
  allowlistedLocalProviderInvocationRequest,
  deterministicFakeAdapterDeclarationCandidate,
  deterministicFakeAdapterDryRunRequest,
  deterministicStubProviderConfigurationCandidate,
  deterministicStubProviderExecutionRequest,
  localCandidateMaterializationRequestFromState,
  type LocalOperatorIntentKind,
  type LocalOperatorShellState,
} from "./api/localOperatorShell.js";
import { renderReviewWorkspace } from "./app/reviewWorkspace.js";
import { renderReviewDiagnostics } from "./app/reviewDiagnostics.js";
import { initialReviewPreferences, type ReviewTab, type RequestFeedback, type ReviewView } from "./app/reviewModel.js";
import { closeMenus, controlChecked, controlValue, eventAction, focusControl, isEscape, keyboardTab, listen, replaceHtml } from "./display/dom.js";
import {
  createLocalOperatorShellTransport,
  getInitialLocalOperatorShellState,
  requestDeterministicStubRun,
  executeLocalProvider,
  createLocalStagedCandidateConversionProposal,
  validateLocalStagedCandidateConversionProposal,
  submitLocalOperatorIntent,
  submitLocalProviderConfiguration,
  submitLocalProviderAdapterDeclaration,
  runLocalProviderAdapterDryRun,
  invokeConstrainedLocalProvider,
  requestLocalCandidateMaterialization,
  submitLocalOperatorCandidateDecision,
  type LocalOperatorShellResponse,
} from "./api/localOperatorShellTransport.js";

const transport = createLocalOperatorShellTransport();
let shellState = getInitialLocalOperatorShellState(transport).state;
let feedback: RequestFeedback = { status: "available", reason: "Local example initialized" };
const preferences = initialReviewPreferences();

function initialize(): void {
  connectWorkspaceEvents();
  render();
}

function connectWorkspaceEvents(): void {
  listen("click", handleWorkspaceClick);
  listen("input", handleNotesInput);
  listen("change", handleDetailsChange);
  listen("keydown", handleWorkspaceKey);
}

function handleWorkspaceClick(event: Event): void {
  const action = eventAction(event);
  if (action) applyWorkspaceAction(action);
  closeMenus(event);
}

function handleNotesInput(event: Event): void {
  const value = controlValue(event, "review-notes");
  if (value !== null) preferences.notes = value;
}

function handleDetailsChange(event: Event): void {
  const value = controlChecked(event, "show-details");
  if (value === null) return;
  preferences.showDetails = value;
  render();
  focusControl("show-details");
}

function handleWorkspaceKey(event: Event): void {
  const action = keyboardTab(event);
  if (action) applyWorkspaceAction(action);
  if (isEscape(event)) closeMenus();
}

function applyWorkspaceAction(action: string): void {
  if (action.startsWith("tab-")) return selectReviewTab(action);
  ACTIONS[action]?.();
  focusControl(action);
}

function selectReviewTab(action: string): void {
  const tab = action.slice(4) as ReviewTab;
  if (!REVIEW_TABS.includes(tab)) return;
  preferences.tab = tab;
  render();
  focusControl(tab === "checks" || tab === "diagnostics" ? "review-panel" : action);
}

function toggleContext(): void {
  preferences.expandedContext = !preferences.expandedContext;
  render();
}

function toggleActivity(): void {
  preferences.showAllActivity = !preferences.showAllActivity;
  render();
}

function openResult(): void {
  selectReviewTab("tab-result");
}

function applyTransportResponse(response: LocalOperatorShellResponse): void {
  shellState = response.state;
  feedback = { status: response.status, reason: response.reason };
  render();
}

function startRun(): void {
  applyTransportResponse(requestDeterministicStubRun(transport));
}

function recordIntent(kind: LocalOperatorIntentKind): void {
  applyTransportResponse(
    submitLocalOperatorIntent(transport, {
      kind,
      operatorId: "local-operator",
      targetRunId: shellState.run.runId,
      targetCandidateId: shellState.run.candidate?.candidateId,
      reason: `${kind} selected in local non-production browser shell`,
    }),
  );
}

function submitAdapterDeclaration(): void {
  applyTransportResponse(
    submitLocalProviderAdapterDeclaration(
      transport,
      deterministicFakeAdapterDeclarationCandidate(),
    ),
  );
}

function submitUnsafeAdapterDeclaration(): void {
  applyTransportResponse(
    submitLocalProviderAdapterDeclaration(transport, {
      adapterKind: "unsupported_network_adapter",
      declarationId: "unsafe-network-adapter-declaration",
      fields: [],
    }),
  );
}

function runAdapterDryRun(): void {
  applyTransportResponse(
    runLocalProviderAdapterDryRun(
      transport,
      deterministicFakeAdapterDryRunRequest(),
    ),
  );
}

function runRejectedAdapterDryRun(): void {
  applyTransportResponse(
    runLocalProviderAdapterDryRun(transport, {
      inputSummary: "phase 154 browser dry run",
      fields: [{ key: "endpoint", value: "https://example.invalid" }],
    }),
  );
}

function invokeAllowlistedLocalProvider(): void {
  applyTransportResponse(
    invokeConstrainedLocalProvider(
      transport,
      allowlistedLocalProviderInvocationRequest(),
    ),
  );
}

function invokeRejectedLocalProvider(): void {
  applyTransportResponse(
    invokeConstrainedLocalProvider(transport, {
      providerKind: "unsupported_network_provider",
      inputSummary: "phase 156 browser rejected invocation",
      fields: [{ key: "endpoint", value: "https://example.invalid" }],
    }),
  );
}

function submitProviderConfiguration(): void {
  applyTransportResponse(
    submitLocalProviderConfiguration(
      transport,
      deterministicStubProviderConfigurationCandidate(),
    ),
  );
}

function submitUnsafeProviderConfiguration(): void {
  applyTransportResponse(
    submitLocalProviderConfiguration(transport, {
      providerKind: "deterministic_stub",
      fields: [{ key: "endpoint", value: "http://localhost:11434" }],
    }),
  );
}

function runDeterministicProvider(): void {
  applyTransportResponse(
    executeLocalProvider(
      transport,
      deterministicStubProviderExecutionRequest(
        "local deterministic browser execution input",
      ),
    ),
  );
}

function runForbiddenProviderExecution(): void {
  applyTransportResponse(
    executeLocalProvider(transport, {
      providerKind: "deterministic_stub",
      inputSummary: "unsafe local browser execution input",
      fields: [{ key: "command", value: "run model" }],
    }),
  );
}

function createStagedProposal(): void {
  applyTransportResponse(
    createLocalStagedCandidateConversionProposal(transport, {
      operatorNote: "local browser staged proposal",
    }),
  );
}

function validateStagedProposal(): void {
  applyTransportResponse(
    validateLocalStagedCandidateConversionProposal(transport),
  );
}
function approveValidatedStagedProposal(): void {
  const proposal = shellState.stagedCandidateConversionProposal.proposal;
  if (!proposal) return;
  applyTransportResponse(
    submitLocalOperatorCandidateDecision(transport, {
      kind: "approve_validated_staged_proposal",
      stagedProposalId: proposal.proposalId,
      providerExecutionResultId: proposal.sourceExecutionResultId,
      stagedProposalValidationStatus:
        shellState.stagedCandidateConversionValidation.status,
    }),
  );
}

function materializeLocalCandidate(): void {
  const request = localCandidateMaterializationRequestFromState(shellState);
  if (!request) return;
  applyTransportResponse(
    requestLocalCandidateMaterialization(transport, request),
  );
}

function render(): void {
  const view: ReviewView = { state: shellState, preferences, feedback };
  const diagnostics = preferences.tab === "diagnostics" ? renderReviewDiagnostics(shellState) : "";
  replaceHtml("app", renderReviewWorkspace(view, diagnostics));
}

const REVIEW_TABS: readonly ReviewTab[] = ["compare", "result", "activity", "checks", "diagnostics"];
const ACTIONS: Readonly<Record<string, () => void>> = {
  "start-run": startRun,
  "approve-run": () => recordIntent("approve"),
  "reject-run": () => recordIntent("reject"),
  "toggle-context": toggleContext,
  "toggle-activity": toggleActivity,
  "open-result": openResult,
  "submit-adapter-declaration": submitAdapterDeclaration,
  "reject-adapter-declaration": submitUnsafeAdapterDeclaration,
  "run-adapter-dry-run": runAdapterDryRun,
  "reject-adapter-dry-run": runRejectedAdapterDryRun,
  "invoke-constrained-local-provider": invokeAllowlistedLocalProvider,
  "reject-constrained-local-provider": invokeRejectedLocalProvider,
  "submit-provider-config": submitProviderConfiguration,
  "reject-provider-config": submitUnsafeProviderConfiguration,
  "run-provider": runDeterministicProvider,
  "reject-provider-execution": runForbiddenProviderExecution,
  "create-staged-proposal": createStagedProposal,
  "validate-staged-proposal": validateStagedProposal,
  "approve-staged-proposal": approveValidatedStagedProposal,
  "materialize-local-candidate": materializeLocalCandidate,
};

initialize();
