import {
  applyLocalOperatorIntent,
  initialLocalOperatorShellState,
  startDeterministicStubRun,
  type LocalOperatorIntentKind,
  type LocalOperatorShellState
} from "./api/localOperatorShell.js";

let shellState = initialLocalOperatorShellState();

function setState(next: LocalOperatorShellState): void {
  shellState = next;
  render();
}

function startRun(): void {
  setState(startDeterministicStubRun(shellState));
}

function recordIntent(kind: LocalOperatorIntentKind): void {
  const result = applyLocalOperatorIntent(shellState, {
    kind,
    operatorId: "local-operator",
    targetRunId: shellState.run.runId,
    reason: `${kind} selected in local non-production browser shell`
  });
  setState(result.state);
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

      <section class="panel replay-panel">
        <h2>Replay / status projection placeholder</h2>
        <p>${shellState.run.replayStatus}</p>
        <p class="muted">Typed local state backs this placeholder; replay repair and status promotion are not available in the UI.</p>
      </section>
    </main>`;

  document.querySelector<HTMLButtonElement>("#start-run")?.addEventListener("click", startRun);
  document.querySelector<HTMLButtonElement>("#approve-run")?.addEventListener("click", () => recordIntent("approve"));
  document.querySelector<HTMLButtonElement>("#reject-run")?.addEventListener("click", () => recordIntent("reject"));
}

render();
