import { deterministicStubProviderConfigurationCandidate, deterministicStubProviderExecutionRequest, projectLocalProviderConfiguration, projectLocalProviderExecution, type LocalOperatorIntentKind, type LocalOperatorShellState } from "./api/localOperatorShell.js";
import {
  createLocalOperatorShellTransport,
  getInitialLocalOperatorShellState,
  requestDeterministicStubRun,
  executeLocalProvider,
  submitLocalOperatorIntent,
  submitLocalProviderConfiguration,
  type LocalOperatorShellResponse
} from "./api/localOperatorShellTransport.js";

const transport = createLocalOperatorShellTransport();
let shellState = getInitialLocalOperatorShellState(transport).state;
let lastTransportMessage = "initial shell state loaded through local transport boundary";

function applyTransportResponse(response: LocalOperatorShellResponse): void {
  shellState = response.state;
  lastTransportMessage = `${response.status}: ${response.reason}`;
  render();
}

function startRun(): void {
  applyTransportResponse(requestDeterministicStubRun(transport));
}

function recordIntent(kind: LocalOperatorIntentKind): void {
  applyTransportResponse(submitLocalOperatorIntent(transport, {
    kind,
    operatorId: "local-operator",
    targetRunId: shellState.run.runId,
    targetCandidateId: shellState.run.candidate?.candidateId,
    reason: `${kind} selected in local non-production browser shell`
  }));
}

function submitProviderConfiguration(): void {
  applyTransportResponse(submitLocalProviderConfiguration(transport, deterministicStubProviderConfigurationCandidate()));
}

function submitUnsafeProviderConfiguration(): void {
  applyTransportResponse(submitLocalProviderConfiguration(transport, {
    providerKind: "deterministic_stub",
    fields: [{ key: "endpoint", value: "http://localhost:11434" }]
  }));
}

function runDeterministicProvider(): void {
  applyTransportResponse(executeLocalProvider(transport, deterministicStubProviderExecutionRequest("local deterministic browser execution input")));
}

function runForbiddenProviderExecution(): void {
  applyTransportResponse(executeLocalProvider(transport, {
    providerKind: "deterministic_stub",
    inputSummary: "unsafe local browser execution input",
    fields: [{ key: "command", value: "run model" }]
  }));
}

function renderList(items: readonly string[], emptyText: string): string {
  if (items.length === 0) return `<p class="muted">${emptyText}</p>`;
  return `<ul>${items.map((item) => `<li>${item}</li>`).join("")}</ul>`;
}

function renderCandidate(state: LocalOperatorShellState): string {
  if (!state.run.candidate) return `<p class="muted">Start a deterministic stub run to display candidate output.</p>`;
  const candidate = state.run.candidate;
  return `
    <h3>${candidate.title}</h3>
    <p>${candidate.body}</p>
    <dl>
      <div><dt>Candidate</dt><dd>${candidate.candidateId}</dd></div>
      <div><dt>Provider</dt><dd>${candidate.providerKind}</dd></div>
      <div><dt>Provider output trusted</dt><dd>${candidate.providerOutputTrusted}</dd></div>
      <div><dt>Provider execution enabled</dt><dd>${candidate.providerExecutionEnabled}</dd></div>
    </dl>`;
}



function renderProviderConfiguration(state: LocalOperatorShellState): string {
  const providerConfiguration = projectLocalProviderConfiguration(state.providerConfiguration);
  return `
    <dl>
      <div><dt>Configured provider kind</dt><dd>${providerConfiguration.configuredProviderKind}</dd></div>
      <div><dt>Provider configuration status</dt><dd>${providerConfiguration.status}</dd></div>
      <div><dt>Validation status</dt><dd>${providerConfiguration.validationStatus}</dd></div>
      <div><dt>Validation reason</dt><dd>${providerConfiguration.validationReason}</dd></div>
      <div><dt>Validation reason/error code</dt><dd>${providerConfiguration.validationErrorCodes.join(", ") || "none"}</dd></div>
      <div><dt>Execution status</dt><dd>${providerConfiguration.executionStatus}</dd></div>
      <div><dt>Capability surface</dt><dd>${providerConfiguration.capabilitySurface.summary}</dd></div>
    </dl>
    <p class="muted">${providerConfiguration.note}</p>
    <div class="button-row">
      <button id="submit-provider-config" type="button">Submit deterministic_stub configuration</button>
      <button id="reject-provider-config" type="button">Submit forbidden endpoint candidate</button>
    </div>`;
}


function renderProviderExecution(state: LocalOperatorShellState): string {
  const providerExecution = projectLocalProviderExecution(state);
  const result = providerExecution.result;
  return `
    <dl>
      <div><dt>Projection status</dt><dd>${providerExecution.projectionStatus}</dd></div>
      <div><dt>Configured provider kind</dt><dd>${providerExecution.configuredProviderKind}</dd></div>
      <div><dt>Execution status</dt><dd>${providerExecution.status}</dd></div>
      <div><dt>Sandbox status</dt><dd>${providerExecution.sandboxStatus}</dd></div>
      <div><dt>Execution result ID</dt><dd>${result?.resultId ?? "none"}</dd></div>
      <div><dt>Provider output summary</dt><dd>${result?.outputSummary ?? "none"}</dd></div>
      <div><dt>Output trust status</dt><dd>untrusted/descriptive (${providerExecution.outputTrustStatus})</dd></div>
      <div><dt>Output materialization status</dt><dd>${providerExecution.outputMaterializationStatus}</dd></div>
      <div><dt>Output promotion status</dt><dd>${providerExecution.outputPromotionStatus}</dd></div>
      <div><dt>Promotion availability</dt><dd>${providerExecution.promotionAvailabilityStatus}</dd></div>
      <div><dt>Run/session linkage</dt><dd>${providerExecution.linkage.runId} / ${providerExecution.linkage.shellStateLabel} / ${providerExecution.linkage.providerConfigurationKind} / ${providerExecution.linkage.providerConfigurationStatus}</dd></div>
      <div><dt>Source boundary</dt><dd>${providerExecution.linkage.sourceBoundary}</dd></div>
      <div><dt>Projection validation</dt><dd>${providerExecution.projectionValidation.status} (${providerExecution.projectionValidation.errorCodes.join(", ") || "none"})</dd></div>
      <div><dt>Absence markers</dt><dd>${providerExecution.absenceMarkers.markerSummary.join(", ")}</dd></div>
      <div><dt>Phase 142 boundary</dt><dd>provider output is not candidate material and is not review/approval material</dd></div>
      <div><dt>Validation/error reason</dt><dd>${providerExecution.validationReason}</dd></div>
      <div><dt>Validation/error code</dt><dd>${providerExecution.validationErrorCodes.join(", ") || "none"}</dd></div>
      <div><dt>Capability surface</dt><dd>${providerExecution.capabilitySurface.summary}</dd></div>
    </dl>
    <p class="muted">${providerExecution.note}</p>
    <div class="button-row">
      <button id="run-provider" type="button" ${state.providerConfiguration.status === "accepted" ? "" : "disabled"}>Run deterministic provider</button>
      <button id="reject-provider-execution" type="button">Submit forbidden command execution</button>
    </div>`;
}

function renderReplayProjection(state: LocalOperatorShellState): string {
  const replay = state.run.decisionReplay;
  return `
    <dl>
      <div><dt>Replay status</dt><dd>${replay.replayStatus}</dd></div>
      <div><dt>Decision count</dt><dd>${replay.sourceDecisionCount}</dd></div>
      <div><dt>Latest decision ID</dt><dd>${replay.latestDecisionId ?? "none"}</dd></div>
      <div><dt>Latest run ID</dt><dd>${replay.latestRunId ?? "none"}</dd></div>
      <div><dt>Latest candidate ID</dt><dd>${replay.latestCandidateId ?? "none"}</dd></div>
      <div><dt>Latest operator ID</dt><dd>${replay.latestOperatorId ?? "none"}</dd></div>
      <div><dt>Latest decision kind</dt><dd>${replay.latestDecisionKind ?? "none"}</dd></div>
      <div><dt>Integrity</dt><dd>${replay.integrityStatus}</dd></div>
      <div><dt>Summary</dt><dd>${replay.summary}</dd></div>
    </dl>`;
}

function renderValidation(state: LocalOperatorShellState): string {
  if (!state.run.validation) return `<p class="muted">Validation and policy projection appears after the stub run.</p>`;
  const validation = state.run.validation;
  return `
    <dl>
      <div><dt>Policy</dt><dd>${validation.policyStatus}</dd></div>
      <div><dt>Validation</dt><dd>${validation.validationStatus}</dd></div>
      <div><dt>Authority</dt><dd>${validation.authority}</dd></div>
      <div><dt>Reason</dt><dd>${validation.reason}</dd></div>
    </dl>`;
}

function render(): void {
  const app = document.querySelector<HTMLDivElement>("#app");
  if (!app) return;

  app.innerHTML = `
    <main class="local-shell">
      <header class="local-shell__banner">
        <div>
          <p class="eyebrow">AJENTIC local operator shell - non-production</p>
          <h1>Usable Local Operator UI Shell</h1>
          <p>Rust-owned typed projection fixture with deterministic local stub behavior. No provider, cloud, release, signing, or deployment behavior is enabled.</p>
        </div>
        <div class="status-box">
          <span>Harness status</span>
          <strong>${shellState.harnessStatus}</strong>
          <span>Run status</span>
          <strong>${shellState.run.status}</strong>
        </div>
      </header>

      <section class="panel" aria-label="Local transport boundary status">
        <h2>Local transport boundary</h2>
        <p>${lastTransportMessage}</p>
        <p class="muted">The browser shell submits typed local requests only; Rust remains the authoritative state-transition owner.</p>
      </section>

      <section class="local-shell__grid" aria-label="AJENTIC local operator workflow">
        <aside class="panel timeline-panel">
          <h2>Run history / timeline</h2>
          ${renderList(shellState.run.timeline, "No run events yet.")}
        </aside>

        <section class="panel center-panel">
          <div class="panel__header">
            <h2>Bounded context</h2>
            <button id="start-run" type="button">Start deterministic stub run</button>
          </div>
          ${renderList(shellState.run.boundedContext, "Idle local harness state. Start the stub run to load bounded context.")}
          <hr />
          <h2>Candidate output</h2>
          ${renderCandidate(shellState)}
        </section>

        <aside class="panel controls-panel">
          <h2>Validation / policy result</h2>
          ${renderValidation(shellState)}
          <hr />
          <h2>Operator controls</h2>
          <p class="muted">Controls submit local operator intent only through the typed non-authoritative boundary.</p>
          <div class="button-row">
            <button id="approve-run" type="button" ${shellState.run.candidate ? "" : "disabled"}>Approve</button>
            <button id="reject-run" type="button" ${shellState.run.candidate ? "" : "disabled"}>Reject</button>
          </div>
          <p><strong>Selected operator intent:</strong> ${shellState.run.selectedIntent ?? "none"}</p>
        </aside>
      </section>

      <section class="panel" aria-label="Local provider configuration">
        <h2>Local provider configuration</h2>
        ${renderProviderConfiguration(shellState)}
      </section>

      <section class="panel" aria-label="Sandboxed provider execution">
        <h2>Sandboxed provider execution</h2>
        ${renderProviderExecution(shellState)}
      </section>

      <section class="panel replay-panel">
        <h2>Replay / status projection</h2>
        ${renderReplayProjection(shellState)}
        <p class="muted">Rust-derived local replay projection only; replay repair, recovery promotion, readiness approval, provider execution, and action execution are not available in the UI.</p>
      </section>
    </main>`;

  document.querySelector<HTMLButtonElement>("#start-run")?.addEventListener("click", startRun);
  document.querySelector<HTMLButtonElement>("#approve-run")?.addEventListener("click", () => recordIntent("approve"));
  document.querySelector<HTMLButtonElement>("#reject-run")?.addEventListener("click", () => recordIntent("reject"));
  document.querySelector<HTMLButtonElement>("#submit-provider-config")?.addEventListener("click", submitProviderConfiguration);
  document.querySelector<HTMLButtonElement>("#reject-provider-config")?.addEventListener("click", submitUnsafeProviderConfiguration);
  document.querySelector<HTMLButtonElement>("#run-provider")?.addEventListener("click", runDeterministicProvider);
  document.querySelector<HTMLButtonElement>("#reject-provider-execution")?.addEventListener("click", runForbiddenProviderExecution);
}

render();
