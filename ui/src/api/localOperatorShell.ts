export type LocalRunStatus = "idle" | "stub_completed" | "intent_recorded";
export type LocalOperatorIntentKind = "approve" | "reject";
export type LocalDecisionRecordStatus = "recorded";
export type LocalDecisionReplayStatus =
  | "no_decision_recorded"
  | "approved_decision_replayed"
  | "rejected_decision_replayed"
  | "inconsistent_decision_ledger"
  | "replay_not_applicable";
export type LocalDecisionReplayIntegrityStatus = "consistent" | "inconsistent";
export type LocalDecisionReplayError =
  | "empty_record_field"
  | "sequence_mismatch"
  | "decision_id_mismatch"
  | "run_mismatch"
  | "candidate_mismatch"
  | "unsupported_decision_status";

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


export type LocalDecisionReplayEntry = Readonly<{
  replaySequence: string;
  decisionId: string;
  runId: string;
  candidateId: string;
  operatorId: string;
  decisionKind: LocalOperatorIntentKind;
  decisionStatus: LocalDecisionRecordStatus;
}>;

export type LocalDecisionReplayProjection = Readonly<{
  replayStatus: LocalDecisionReplayStatus;
  replaySequence: string;
  sourceDecisionCount: number;
  latestDecisionId: string | null;
  latestRunId: string | null;
  latestCandidateId: string | null;
  latestOperatorId: string | null;
  latestDecisionKind: LocalOperatorIntentKind | null;
  latestDecisionStatus: LocalDecisionRecordStatus | null;
  integrityStatus: LocalDecisionReplayIntegrityStatus;
  error: LocalDecisionReplayError | null;
  entries: readonly LocalDecisionReplayEntry[];
  summary: string;
}>;

export function initialLocalDecisionReplayProjection(): LocalDecisionReplayProjection {
  return {
    replayStatus: "no_decision_recorded",
    replaySequence: "local-replay-sequence-0000",
    sourceDecisionCount: 0,
    latestDecisionId: null,
    latestRunId: null,
    latestCandidateId: null,
    latestOperatorId: null,
    latestDecisionKind: null,
    latestDecisionStatus: null,
    integrityStatus: "consistent",
    error: null,
    entries: [],
    summary: "No local operator decision has been recorded for replay projection."
  };
}

function inconsistentLocalDecisionReplayProjection(
  ledger: LocalDecisionLedger,
  error: LocalDecisionReplayError
): LocalDecisionReplayProjection {
  const latest = ledger.records.length === 0 ? null : ledger.records[ledger.records.length - 1];
  return {
    replayStatus: "inconsistent_decision_ledger",
    replaySequence: `local-replay-sequence-${String(ledger.records.length).padStart(4, "0")}`,
    sourceDecisionCount: ledger.records.length,
    latestDecisionId: latest?.decisionId ?? null,
    latestRunId: latest?.runId ?? null,
    latestCandidateId: latest?.candidateId ?? null,
    latestOperatorId: latest?.operatorId ?? null,
    latestDecisionKind: latest?.intentKind ?? null,
    latestDecisionStatus: latest?.decisionStatus ?? null,
    integrityStatus: "inconsistent",
    error,
    entries: [],
    summary: `Local decision ledger is inconsistent: ${error}.`
  };
}

export function validateLocalDecisionReplayInput(
  run: LocalRunProjection,
  ledger: LocalDecisionLedger
): LocalDecisionReplayError | null {
  for (const [index, record] of ledger.records.entries()) {
    const expectedSequence = index + 1;
    if (record.decisionId.length === 0 || record.runId.length === 0 || record.candidateId.length === 0 || record.operatorId.length === 0) {
      return "empty_record_field";
    }
    if (record.recordedSequence !== expectedSequence) return "sequence_mismatch";
    if (record.decisionId !== `local-decision-${String(expectedSequence).padStart(4, "0")}`) return "decision_id_mismatch";
    if (record.decisionStatus !== "recorded") return "unsupported_decision_status";
  }

  const latest = ledger.records.length === 0 ? undefined : ledger.records[ledger.records.length - 1];
  if (latest) {
    if (latest.runId !== run.runId) return "run_mismatch";
    if (run.candidate && latest.candidateId !== run.candidate.candidateId) return "candidate_mismatch";
  }

  return null;
}

export function deriveLocalDecisionReplayProjection(
  run: LocalRunProjection,
  ledger: LocalDecisionLedger
): LocalDecisionReplayProjection {
  if (ledger.records.length === 0) return initialLocalDecisionReplayProjection();

  const error = validateLocalDecisionReplayInput(run, ledger);
  if (error) return inconsistentLocalDecisionReplayProjection(ledger, error);

  const entries = ledger.records.map((record): LocalDecisionReplayEntry => ({
    replaySequence: `local-replay-entry-${String(record.recordedSequence).padStart(4, "0")}`,
    decisionId: record.decisionId,
    runId: record.runId,
    candidateId: record.candidateId,
    operatorId: record.operatorId,
    decisionKind: record.intentKind,
    decisionStatus: record.decisionStatus
  }));
  const latest = ledger.records[ledger.records.length - 1];
  const replayStatus: LocalDecisionReplayStatus = latest.intentKind === "approve"
    ? "approved_decision_replayed"
    : "rejected_decision_replayed";

  return {
    replayStatus,
    replaySequence: `local-replay-sequence-${String(ledger.records.length).padStart(4, "0")}`,
    sourceDecisionCount: ledger.records.length,
    latestDecisionId: latest.decisionId,
    latestRunId: latest.runId,
    latestCandidateId: latest.candidateId,
    latestOperatorId: latest.operatorId,
    latestDecisionKind: latest.intentKind,
    latestDecisionStatus: latest.decisionStatus,
    integrityStatus: "consistent",
    error: null,
    entries,
    summary: `Local decision replay projection derived ${ledger.records.length} recorded decision(s); latest decision ${latest.decisionId} is ${replayStatus}.`
  };
}


export type LocalSessionEvidenceExportStatus =
  | "no_completed_run_evidence"
  | "run_evidence_projected"
  | "decision_evidence_projected";
export type LocalSessionEvidenceExportValidationStatus = "complete" | "incomplete";

export type LocalSessionEvidenceExportAbsenceMarkers = Readonly<{
  providerExecutionAbsent: true;
  persistenceAbsent: true;
  releaseAbsent: true;
  deploymentAbsent: true;
  signingAbsent: true;
  publishingAbsent: true;
  installerAbsent: true;
  updateChannelAbsent: true;
  publicUseAbsent: true;
  readinessApprovalAbsent: true;
  markerSummary: readonly string[];
}>;

export type LocalSessionEvidenceExport = Readonly<{
  exportId: string;
  exportStatus: LocalSessionEvidenceExportStatus;
  exportClassification: "local_session_evidence_only";
  productionClassification: "non-production";
  shellStatus: string;
  runId: string;
  runStatus: LocalRunStatus;
  boundedContextSummary: readonly string[];
  candidateId: string;
  candidateOutputSummary: string;
  validationStatus: string;
  policyStatus: string;
  decisionCount: number;
  decisionRecords: readonly LocalDecisionRecord[];
  replayStatus: LocalDecisionReplayStatus;
  replayIntegrityStatus: LocalDecisionReplayIntegrityStatus;
  absenceMarkers: LocalSessionEvidenceExportAbsenceMarkers;
  exportValidationStatus: LocalSessionEvidenceExportValidationStatus;
  summary: string;
}>;

export function localSessionEvidenceExportAbsenceMarkers(): LocalSessionEvidenceExportAbsenceMarkers {
  return {
    providerExecutionAbsent: true,
    persistenceAbsent: true,
    releaseAbsent: true,
    deploymentAbsent: true,
    signingAbsent: true,
    publishingAbsent: true,
    installerAbsent: true,
    updateChannelAbsent: true,
    publicUseAbsent: true,
    readinessApprovalAbsent: true,
    markerSummary: [
      "provider execution absent",
      "persistence absent",
      "release absent",
      "deployment absent",
      "signing absent",
      "publishing absent",
      "installer absent",
      "update-channel absent",
      "public-use absent",
      "readiness absent"
    ]
  };
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
  replayStatus: LocalDecisionReplayStatus;
  decisionTimeline: LocalDecisionTimelineProjection;
  decisionReplay: LocalDecisionReplayProjection;
}>;

export type LocalOperatorShellState = Readonly<{
  harnessStatus: string;
  nonProduction: true;
  run: LocalRunProjection;
  decisionLedger: LocalDecisionLedger;
  localSessionEvidenceExport: LocalSessionEvidenceExport;
}>;


export function deriveLocalSessionEvidenceExport(
  harnessStatus: string,
  nonProduction: boolean,
  run: LocalRunProjection,
  ledger: LocalDecisionLedger
): LocalSessionEvidenceExport {
  const replay = deriveLocalDecisionReplayProjection(run, ledger);
  const exportStatus: LocalSessionEvidenceExportStatus = run.status === "idle"
    ? "no_completed_run_evidence"
    : ledger.records.length === 0
      ? "run_evidence_projected"
      : "decision_evidence_projected";
  const candidateId = run.candidate?.candidateId ?? "not_applicable_until_stub_run";
  const candidateOutputSummary = run.candidate
    ? `${run.candidate.title}: ${run.candidate.body}`
    : "no completed deterministic stub run candidate evidence";
  const validationStatus = run.validation?.validationStatus ?? "not_applicable_until_stub_run";
  const policyStatus = run.validation?.policyStatus ?? "not_applicable_until_stub_run";
  const exportPayload: Omit<LocalSessionEvidenceExport, "exportValidationStatus"> = {
    exportId: `local-session-evidence-export-${run.runId}-decisions-${String(ledger.records.length).padStart(4, "0")}`,
    exportStatus,
    exportClassification: "local_session_evidence_only",
    productionClassification: "non-production",
    shellStatus: harnessStatus,
    runId: run.runId,
    runStatus: run.status,
    boundedContextSummary: run.boundedContext,
    candidateId,
    candidateOutputSummary,
    validationStatus,
    policyStatus,
    decisionCount: ledger.records.length,
    decisionRecords: ledger.records,
    replayStatus: replay.replayStatus,
    replayIntegrityStatus: replay.integrityStatus,
    absenceMarkers: localSessionEvidenceExportAbsenceMarkers(),
    summary: `Local session evidence export preview for run ${run.runId} is ${exportStatus}; local only, non-production, and non-mutating.`
  };
  return {
    ...exportPayload,
    exportValidationStatus: validateLocalSessionEvidenceExport(exportPayload, nonProduction) ? "complete" : "incomplete"
  };
}

export function validateLocalSessionEvidenceExport(
  exportPayload: Omit<LocalSessionEvidenceExport, "exportValidationStatus"> | LocalSessionEvidenceExport,
  nonProduction: boolean
): boolean {
  if (exportPayload.exportId.length === 0 || exportPayload.shellStatus.length === 0 || exportPayload.runId.length === 0) return false;
  if (exportPayload.exportClassification !== "local_session_evidence_only") return false;
  if (!nonProduction || exportPayload.productionClassification !== "non-production") return false;
  const markers = exportPayload.absenceMarkers;
  if (!markers.providerExecutionAbsent || !markers.persistenceAbsent || !markers.releaseAbsent || !markers.deploymentAbsent || !markers.signingAbsent || !markers.publishingAbsent || !markers.installerAbsent || !markers.updateChannelAbsent || !markers.publicUseAbsent || !markers.readinessApprovalAbsent) return false;
  if (exportPayload.exportStatus !== "no_completed_run_evidence" && (exportPayload.boundedContextSummary.length === 0 || exportPayload.candidateId === "not_applicable_until_stub_run" || exportPayload.validationStatus === "not_applicable_until_stub_run" || exportPayload.policyStatus === "not_applicable_until_stub_run")) return false;
  if (exportPayload.exportStatus === "decision_evidence_projected" && (exportPayload.decisionCount === 0 || exportPayload.decisionRecords.length === 0 || exportPayload.replayStatus === "no_decision_recorded")) return false;
  return true;
}

function attachLocalSessionEvidenceExport(state: Omit<LocalOperatorShellState, "localSessionEvidenceExport">): LocalOperatorShellState {
  return {
    ...state,
    localSessionEvidenceExport: deriveLocalSessionEvidenceExport(
      state.harnessStatus,
      state.nonProduction,
      state.run,
      state.decisionLedger
    )
  };
}

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
  const decisionLedger = initialLocalDecisionLedger();
  const decisionReplay = initialLocalDecisionReplayProjection();
  return attachLocalSessionEvidenceExport({
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
      replayStatus: decisionReplay.replayStatus,
      decisionTimeline: projectLocalDecisionTimeline(decisionLedger),
      decisionReplay
    },
    decisionLedger
  });
}

export function startDeterministicStubRun(state: LocalOperatorShellState): LocalOperatorShellState {
  return attachLocalSessionEvidenceExport({
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
      decisionTimeline: projectLocalDecisionTimeline(state.decisionLedger),
      decisionReplay: deriveLocalDecisionReplayProjection(state.run, state.decisionLedger),
      replayStatus: deriveLocalDecisionReplayProjection(state.run, state.decisionLedger).replayStatus
    }
  });
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

  const nextRun: LocalRunProjection = {
    ...state.run,
    status: "intent_recorded",
    selectedIntent: intent.kind,
    decisionTimeline: projectLocalDecisionTimeline(decisionLedger),
    timeline: [...state.run.timeline, `operator intent recorded: ${intent.kind} by ${intent.operatorId} as decision ${decisionRecord.decisionId}`]
  };
  const decisionReplay = deriveLocalDecisionReplayProjection(nextRun, decisionLedger);

  return {
    status: "accepted",
    reason: "local_operator_intent_recorded",
    state: attachLocalSessionEvidenceExport({
      ...state,
      decisionLedger,
      run: {
        ...nextRun,
        decisionReplay,
        replayStatus: decisionReplay.replayStatus
      }
    })
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
