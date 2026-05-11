import type { LocalOperatorShellState } from "./localOperatorShell";

export function renderLocalOperatorShellSnapshot(state: LocalOperatorShellState): string {
  const candidate = state.run.candidate
    ? [
        `Candidate output: ${state.run.candidate.title}`,
        `Candidate body: ${state.run.candidate.body}`,
        `Provider execution enabled: ${state.run.candidate.providerExecutionEnabled}`
      ].join("\n")
    : "Candidate output: waiting for deterministic stub run";

  const validation = state.run.validation
    ? `Validation/policy result: ${state.run.validation.policyStatus} / ${state.run.validation.validationStatus}`
    : "Validation/policy result: waiting for deterministic stub run";

  return [
    "AJENTIC local operator shell - non-production",
    `Harness status: ${state.harnessStatus}`,
    `Run status: ${state.run.status}`,
    "Left panel: Run history / timeline",
    state.run.timeline.join(" | "),
    "Center panel: Bounded context and candidate output",
    state.run.boundedContext.join(" | ") || "Idle local harness state",
    candidate,
    "Right panel: Validation/policy results and operator controls",
    validation,
    "Approve",
    "Reject",
    `Selected operator intent: ${state.run.selectedIntent ?? "none"}`,
    "Bottom panel: Replay/status projection placeholder",
    state.run.replayStatus
  ].join("\n");
}
