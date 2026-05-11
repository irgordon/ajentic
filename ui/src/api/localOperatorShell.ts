export type LocalRunStatus = "idle" | "stub_completed" | "intent_recorded";
export type LocalOperatorIntentKind = "approve" | "reject";
export type LocalDecisionRecordStatus = "recorded";

export type LocalDecisionRecord = Readonly<{
  decisionId: string;
  runId: string;
  candidateId: string;
  operatorId: string;
  intentKind: LocalOperatorIntentKind;
  decisionStatus: LocalDecisionRecordStatus;
  validationStatus: string;
  recordedSequence: number;
  recordedAtLabel: string;
  reason: string;
}>;

export type LocalDecisionTimelineProjection = Readonly<{
  records: readonly LocalDecisionRecord[];
  emptyMessage: string;
}>;

export type LocalDecisionLedger = Readonly<{
  records: readonly LocalDecisionRecord[];
}>;

export function initialLocalDecisionLedger(): LocalDecisionLedger {
  return { records: [] };
}

export function projectLocalDecisionTimeline(ledger: LocalDecisionLedger): LocalDecisionTimelineProjection {
  return {
    records: ledger.records,
    emptyMessage: "No recorded local operator decisions"
  };
}

function nextLocalDecisionSequence(ledger: LocalDecisionLedger): number {
  return ledger.records.length + 1;
}

export type LocalCandidateOutput = Readonly<{
  candidateId: string;
  title: string;
  body: string;
  providerKind: "deterministic_stub";
  providerOutputTrusted: false;
  providerExecutionEnabled: false;
}>;

export type LocalValidationProjection = Readonly<{
  validationId: string;
  policyStatus: string;
  validationStatus: string;
  reason: string;
  authority: "rust";
}>;

export type LocalRunProjection = Readonly<{
  runId: string;
  status: LocalRunStatus;
  boundedContext: readonly string[];
  candidate: LocalCandidateOutput | null;
  validation: LocalValidationProjection | null;
  selectedIntent: LocalOperatorIntentKind | null;
  timeline: readonly string[];
  replayStatus: string;
  decisionTimeline: LocalDecisionTimelineProjection;
}>;

export type LocalOperatorShellState = Readonly<{
  harnessStatus: string;
  nonProduction: true;
  run: LocalRunProjection;
  decisionLedger: LocalDecisionLedger;
}>;

export type LocalOperatorIntent = Readonly<{
  kind: LocalOperatorIntentKind;
  operatorId: string;
  targetRunId: string;
  targetCandidateId?: string;
  reason: string;
  requestsAuthorityGrant?: boolean;
  requestsProviderExecution?: boolean;
  claimsReadiness?: boolean;
}>;

export type LocalOperatorIntentResult = Readonly<{
  status: "accepted" | "rejected";
  reason: string;
  state: LocalOperatorShellState;
}>;

export type LocalOperatorUiForbiddenAction =
  | "mark_production_readiness"
  | "approve_release_candidate_status"
  | "invoke_provider_execution";

export function initialLocalOperatorShellState(): LocalOperatorShellState {
  return {
    harnessStatus: "idle_local_harness",
    nonProduction: true,
    run: {
      runId: "local-stub-run-133",
      status: "idle",
      boundedContext: [],
      candidate: null,
      validation: null,
      selectedIntent: null,
      timeline: ["idle local harness initialized"],
      replayStatus: "placeholder: replay/status projection is typed but not executing",
      decisionTimeline: projectLocalDecisionTimeline(initialLocalDecisionLedger())
    },
    decisionLedger: initialLocalDecisionLedger()
  };
}

export function startDeterministicStubRun(state: LocalOperatorShellState): LocalOperatorShellState {
  return {
    ...state,
    harnessStatus: "deterministic_stub_completed",
    run: {
      ...state.run,
      status: "stub_completed",
      boundedContext: [
        "phase=133",
        "scope=local operator UI shell",
        "provider=deterministic stub only",
        "network=disabled"
      ],
      candidate: {
        candidateId: "candidate-local-stub-133",
        title: "Deterministic local stub candidate",
        body: "AJENTIC local shell rendered a deterministic candidate from a Rust-owned projection fixture.",
        providerKind: "deterministic_stub",
        providerOutputTrusted: false,
        providerExecutionEnabled: false
      },
      validation: {
        validationId: "validation-local-stub-133",
        policyStatus: "pass_for_local_stub_review",
        validationStatus: "pass_for_local_stub_review",
        reason: "deterministic fixture satisfies local shell display checks only",
        authority: "rust"
      },
      selectedIntent: null,
      timeline: [
        "idle local harness initialized",
        "deterministic stub run started",
        "candidate output projected",
        "validation and policy projection completed"
      ],
      decisionTimeline: projectLocalDecisionTimeline(state.decisionLedger)
    }
  };
}

export function applyLocalOperatorIntent(
  state: LocalOperatorShellState,
  intent: LocalOperatorIntent
): LocalOperatorIntentResult {
  const rejection = (reason: string): LocalOperatorIntentResult => ({ status: "rejected", reason, state });
  if (state.run.status === "idle") return rejection("run_not_started");
  if (intent.kind !== "approve" && intent.kind !== "reject") return rejection("unknown_intent_kind");
  if (intent.operatorId.length === 0) return rejection("empty_operator_id");
  if (intent.reason.length === 0) return rejection("empty_reason");
  if (intent.targetRunId !== state.run.runId) return rejection("target_mismatch");
  if (!state.run.candidate) return rejection("run_not_started");
  const candidate = state.run.candidate;
  if ((intent.targetCandidateId ?? "candidate-local-stub-133") !== candidate.candidateId) return rejection("candidate_target_mismatch");
  if (intent.requestsAuthorityGrant) return rejection("authority_grant_rejected");
  if (intent.requestsProviderExecution) return rejection("provider_execution_rejected");
  if (intent.claimsReadiness) return rejection("readiness_claim_rejected");

  if (state.decisionLedger.records.some((record) => record.runId === intent.targetRunId && record.candidateId === candidate.candidateId)) {
    return rejection("duplicate_decision_rejected");
  }

  const recordedSequence = nextLocalDecisionSequence(state.decisionLedger);
  const decisionRecord: LocalDecisionRecord = {
    decisionId: `local-decision-${String(recordedSequence).padStart(4, "0")}`,
    runId: intent.targetRunId,
    candidateId: candidate.candidateId,
    operatorId: intent.operatorId,
    intentKind: intent.kind,
    decisionStatus: "recorded",
    validationStatus: "accepted_by_rust_local_validation",
    recordedSequence,
    recordedAtLabel: `local-sequence-${String(recordedSequence).padStart(4, "0")}`,
    reason: intent.reason
  };
  const decisionLedger: LocalDecisionLedger = { records: [...state.decisionLedger.records, decisionRecord] };

  return {
    status: "accepted",
    reason: "local_operator_intent_recorded",
    state: {
      ...state,
      decisionLedger,
      run: {
        ...state.run,
        status: "intent_recorded",
        selectedIntent: intent.kind,
        decisionTimeline: projectLocalDecisionTimeline(decisionLedger),
        timeline: [...state.run.timeline, `operator intent recorded: ${intent.kind} by ${intent.operatorId} as decision ${decisionRecord.decisionId}`]
      }
    }
  };
}

export function applyForbiddenUiAction(
  state: LocalOperatorShellState,
  action: LocalOperatorUiForbiddenAction
): LocalOperatorIntentResult {
  const reasons: Record<LocalOperatorUiForbiddenAction, string> = {
    mark_production_readiness: "ui_cannot_mark_readiness",
    approve_release_candidate_status: "ui_cannot_approve_candidate_status",
    invoke_provider_execution: "ui_cannot_invoke_provider_execution"
  };
  return { status: "rejected", reason: reasons[action], state };
}
