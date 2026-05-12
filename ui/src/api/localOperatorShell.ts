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


export type LocalProviderKind =
  | "deterministic_stub"
  | "local_model"
  | "cloud_model"
  | "external_http"
  | "shell_command"
  | "filesystem_provider"
  | "unknown";
export type LocalProviderConfigurationStatus = "not_configured" | "accepted" | "rejected" | "unsupported";
export type LocalProviderConfigurationError =
  | "missing_provider_kind"
  | "malformed_provider_kind"
  | "unsupported_provider_kind"
  | "forbidden_endpoint_field"
  | "forbidden_command_field"
  | "forbidden_path_field"
  | "forbidden_secret_field"
  | "provider_execution_rejected"
  | "trust_grant_rejected"
  | "readiness_claim_rejected"
  | "release_claim_rejected"
  | "deployment_claim_rejected"
  | "public_use_claim_rejected"
  | "signing_claim_rejected"
  | "publishing_claim_rejected"
  | "unknown_field_rejected";

export type LocalProviderConfigurationCapabilitySurface = Readonly<{
  configurationOnly: true;
  providerExecutionEnabled: false;
  cloudCallsEnabled: false;
  networkEnabled: false;
  shellCommandsEnabled: false;
  filesystemEnabled: false;
  secretsAllowed: false;
  trustGranted: false;
  readinessApproved: false;
  releaseApproved: false;
  deploymentEnabled: false;
  summary: string;
}>;

export type LocalProviderConfigurationValidation = Readonly<{
  status: LocalProviderConfigurationStatus;
  providerKind: LocalProviderKind | null;
  errorCodes: readonly LocalProviderConfigurationError[];
  reason: string;
}>;

export type LocalProviderConfigurationProjection = Readonly<{
  configuredProviderKind: string;
  status: LocalProviderConfigurationStatus;
  validationStatus: LocalProviderConfigurationStatus;
  validationReason: string;
  validationErrorCodes: readonly LocalProviderConfigurationError[];
  executionStatus: "disabled_not_executed";
  capabilitySurface: LocalProviderConfigurationCapabilitySurface;
  note: string;
}>;

export type LocalProviderConfiguration = Readonly<{
  configuredProviderKind: LocalProviderKind | null;
  status: LocalProviderConfigurationStatus;
  lastValidation: LocalProviderConfigurationValidation;
}>;

export type LocalProviderConfigurationCandidate = Readonly<{
  providerKind?: string;
  fields?: readonly Readonly<{ key: string; value: string }>[];
}>;

const supportedLocalProviderKinds: readonly LocalProviderKind[] = [
  "deterministic_stub",
  "local_model",
  "cloud_model",
  "external_http",
  "shell_command",
  "filesystem_provider",
  "unknown"
];

export function deterministicStubProviderConfigurationCandidate(): LocalProviderConfigurationCandidate {
  return { providerKind: "deterministic_stub", fields: [] };
}

export function localProviderConfigurationCapabilitySurface(): LocalProviderConfigurationCapabilitySurface {
  return {
    configurationOnly: true,
    providerExecutionEnabled: false,
    cloudCallsEnabled: false,
    networkEnabled: false,
    shellCommandsEnabled: false,
    filesystemEnabled: false,
    secretsAllowed: false,
    trustGranted: false,
    readinessApproved: false,
    releaseApproved: false,
    deploymentEnabled: false,
    summary: "deterministic_stub configuration-only surface; no execution, cloud, network, shell, filesystem, secrets, trust, readiness, release, or deployment capability"
  };
}

export function initialLocalProviderConfiguration(): LocalProviderConfiguration {
  return {
    configuredProviderKind: null,
    status: "not_configured",
    lastValidation: {
      status: "not_configured",
      providerKind: null,
      errorCodes: [],
      reason: "no executable provider configured; deterministic_stub is available for configuration-only validation"
    }
  };
}

export function projectLocalProviderConfiguration(configuration: LocalProviderConfiguration): LocalProviderConfigurationProjection {
  return {
    configuredProviderKind: configuration.configuredProviderKind ?? "none",
    status: configuration.status,
    validationStatus: configuration.lastValidation.status,
    validationReason: configuration.lastValidation.reason,
    validationErrorCodes: configuration.lastValidation.errorCodes,
    executionStatus: "disabled_not_executed",
    capabilitySurface: localProviderConfigurationCapabilitySurface(),
    note: "Phase 139 deterministic_stub is configuration-only; validation does not execute providers or approve trust/readiness/release/deployment."
  };
}

function forbiddenProviderConfigurationField(key: string, value: string): LocalProviderConfigurationError | null {
  const loweredKey = key.toLowerCase();
  const combined = `${loweredKey}=${value.toLowerCase()}`;
  if (loweredKey === "label" || loweredKey === "description") return null;
  if (["endpoint", "url", "host", "port", "http", "network"].some((needle) => combined.includes(needle))) return "forbidden_endpoint_field";
  if (["command", "args", "shell", "process"].some((needle) => combined.includes(needle))) return "forbidden_command_field";
  if (["path", "file", "directory"].some((needle) => combined.includes(needle))) return "forbidden_path_field";
  if (["secret", "token", "api_key", "apikey", "credential"].some((needle) => combined.includes(needle))) return "forbidden_secret_field";
  if (loweredKey === "provider_execution_enabled" && value === "true") return "provider_execution_rejected";
  if (loweredKey === "trust_granted" && value === "true") return "trust_grant_rejected";
  if (loweredKey === "readiness_approved" && value === "true") return "readiness_claim_rejected";
  if (loweredKey === "release_candidate_approved" && value === "true") return "release_claim_rejected";
  if (loweredKey === "deployment_enabled" && value === "true") return "deployment_claim_rejected";
  if (loweredKey === "public_use_approved" && value === "true") return "public_use_claim_rejected";
  if (loweredKey === "signing_enabled" && value === "true") return "signing_claim_rejected";
  if (loweredKey === "publishing_enabled" && value === "true") return "publishing_claim_rejected";
  return "unknown_field_rejected";
}

export function validateLocalProviderConfiguration(candidate: LocalProviderConfigurationCandidate): LocalProviderConfigurationValidation {
  const errors = new Set<LocalProviderConfigurationError>();
  let providerKind: LocalProviderKind | null = null;
  const rawKind = candidate.providerKind;
  if (rawKind === undefined || rawKind.trim().length === 0) {
    errors.add("missing_provider_kind");
  } else if (rawKind.trim() !== rawKind) {
    errors.add("malformed_provider_kind");
  } else if (supportedLocalProviderKinds.includes(rawKind as LocalProviderKind)) {
    providerKind = rawKind as LocalProviderKind;
    if (providerKind !== "deterministic_stub") errors.add("unsupported_provider_kind");
  } else {
    errors.add("unsupported_provider_kind");
  }

  for (const field of candidate.fields ?? []) {
    const error = forbiddenProviderConfigurationField(field.key, field.value);
    if (error) errors.add(error);
  }

  const errorCodes = [...errors].sort();
  if (errorCodes.length === 0 && providerKind === "deterministic_stub") {
    return {
      status: "accepted",
      providerKind,
      errorCodes: [],
      reason: "deterministic_stub configuration accepted as local-session configuration-only state; provider execution remains disabled"
    };
  }
  return {
    status: errors.has("unsupported_provider_kind") ? "unsupported" : "rejected",
    providerKind,
    errorCodes,
    reason: "provider configuration rejected fail-closed; prior configuration remains unchanged and no provider execution occurs"
  };
}

export function applyLocalProviderConfigurationCandidate(
  state: LocalOperatorShellState,
  candidate: LocalProviderConfigurationCandidate
): LocalOperatorIntentResult {
  const validation = validateLocalProviderConfiguration(candidate);
  if (validation.status !== "accepted") return { status: "rejected", reason: validation.reason, state };
  return {
    status: "accepted",
    reason: "local_provider_configuration_accepted",
    state: attachLocalSessionEvidenceExport({
      ...state,
      providerConfiguration: {
        configuredProviderKind: validation.providerKind,
        status: "accepted",
        lastValidation: validation
      }
    })
  };
}


export type LocalProviderExecutionStatus =
  | "not_executed"
  | "executed"
  | "rejected"
  | "unsupported_provider"
  | "configuration_required"
  | "invalid_request";
export type LocalProviderExecutionSandboxStatus =
  | "not_entered"
  | "sandboxed_deterministic_no_external_effects"
  | "rejected_before_sandbox";
export type LocalProviderExecutionError =
  | "missing_provider_configuration"
  | "rejected_provider_configuration"
  | "missing_provider_kind"
  | "malformed_provider_kind"
  | "unsupported_provider_kind"
  | "forbidden_endpoint_field"
  | "forbidden_command_field"
  | "forbidden_path_field"
  | "forbidden_secret_field"
  | "provider_execution_flag_rejected"
  | "trust_grant_rejected"
  | "readiness_claim_rejected"
  | "release_claim_rejected"
  | "deployment_claim_rejected"
  | "public_use_claim_rejected"
  | "signing_claim_rejected"
  | "publishing_claim_rejected"
  | "unknown_field_rejected";

export type LocalProviderExecutionResultProjectionStatus =
  | "not_executed"
  | "execution_projected"
  | "execution_rejected"
  | "unsupported_provider"
  | "invalid_execution_request";
export type LocalProviderOutputMaterializationStatus =
  | "not_materialized"
  | "projected_as_untrusted_output"
  | "not_candidate_material"
  | "candidate_material";
export type LocalProviderOutputTrustStatus = "untrusted_descriptive" | "trusted_output";
export type LocalProviderExecutionResultOutputTrustStatus = "untrusted/descriptive" | "trusted_output";
export type LocalProviderOutputPromotionStatus =
  | "not_promoted"
  | "promotion_not_available_in_phase_142"
  | "promoted";


export type LocalProviderOutputValidationStatus =
  | "not_validated"
  | "reviewable_untrusted"
  | "rejected"
  | "validation_not_applicable"
  | "invalid_validation_input";
export type LocalProviderOutputReviewabilityStatus =
  | "not_reviewable"
  | "reviewable_untrusted"
  | "rejected_before_review";
export type LocalProviderOutputCandidateBoundaryStatus =
  | "not_candidate_material"
  | "candidate_conversion_not_performed"
  | "candidate_conversion_requires_future_phase";
export type LocalProviderOutputValidationReason =
  | "no_provider_execution_result"
  | "provider_execution_not_projected"
  | "deterministic_stub_output_shape_valid"
  | "missing_execution_result"
  | "unsupported_provider_kind"
  | "empty_output"
  | "malformed_output"
  | "output_too_large"
  | "contains_forbidden_secret_marker"
  | "contains_execution_instruction"
  | "contains_network_instruction"
  | "contains_filesystem_instruction"
  | "contains_readiness_or_release_claim"
  | "contains_trust_or_approval_claim"
  | "contains_action_instruction"
  | "candidate_conversion_not_available_in_phase_143";
export type LocalProviderOutputValidationEffect = "none" | "effect_detected";
export type LocalProviderOutputValidationError =
  | "invalid_reviewable_trust_status"
  | "invalid_candidate_boundary_status"
  | "invalid_promotion_status"
  | "invalid_no_effect_boundary"
  | "missing_validation_reason";

export type LocalProviderOutputValidationProjection = Readonly<{
  status: LocalProviderOutputValidationStatus;
  reviewabilityStatus: LocalProviderOutputReviewabilityStatus;
  candidateBoundaryStatus: LocalProviderOutputCandidateBoundaryStatus;
  candidateBoundaryStatuses: readonly LocalProviderOutputCandidateBoundaryStatus[];
  reasons: readonly LocalProviderOutputValidationReason[];
  providerExecutionResultId: string | null;
  providerKind: string;
  outputTrustStatus: LocalProviderOutputTrustStatus;
  outputPromotionStatus: LocalProviderOutputPromotionStatus;
  trustEffect: LocalProviderOutputValidationEffect;
  candidateEffect: LocalProviderOutputValidationEffect;
  decisionLedgerEffect: LocalProviderOutputValidationEffect;
  replayEffect: LocalProviderOutputValidationEffect;
  exportEffect: LocalProviderOutputValidationEffect;
  actionEffect: LocalProviderOutputValidationEffect;
  readinessEffect: LocalProviderOutputValidationEffect;
  releaseEffect: LocalProviderOutputValidationEffect;
  deploymentEffect: LocalProviderOutputValidationEffect;
  note: string;
}>;

export type LocalProviderExecutionResultLinkage = Readonly<{
  shellStateLabel: string;
  runId: string;
  providerConfigurationKind: string;
  providerConfigurationStatus: string;
  executionResultId: string;
  candidateId: string;
  sourceBoundary: "sandboxed_deterministic_provider_execution";
}>;

export type LocalProviderExecutionResultAbsenceMarkers = Readonly<{
  noProcessSpawned: true;
  noNetworkSocketOpened: true;
  noFilesystemPersistence: true;
  noSecretsRead: true;
  noReleaseCreated: true;
  noDeploymentCreated: true;
  noSigningPerformed: true;
  noPublishingPerformed: true;
  noPublicUseApproved: true;
  noReadinessApproved: true;
  noReplayRepair: true;
  noRecoveryPromotion: true;
  noActionExecution: true;
  providerOutputNotCandidateMaterial: true;
  markerSummary: readonly string[];
}>;

export type LocalProviderExecutionResultProjectionValidation = Readonly<{
  status: "valid" | "invalid";
  errorCodes: readonly string[];
  reason: string;
}>;

export type LocalProviderExecutionRequest = Readonly<{
  providerKind?: string;
  inputSummary: string;
  fields?: readonly Readonly<{ key: string; value: string }>[];
}>;

export type LocalProviderExecutionCapabilitySurface = Readonly<{
  deterministicStubExecutionSupported: true;
  supportedProviderKind: "deterministic_stub";
  cloudCallsEnabled: false;
  networkEnabled: false;
  shellCommandsEnabled: false;
  filesystemEnabled: false;
  secretsAllowed: false;
  trustGranted: false;
  readinessApproved: false;
  releaseApproved: false;
  deploymentEnabled: false;
  signingEnabled: false;
  publishingEnabled: false;
  publicUseEnabled: false;
  summary: string;
}>;

export type LocalProviderExecutionResult = Readonly<{
  resultId: string;
  providerKind: "deterministic_stub";
  sandboxStatus: LocalProviderExecutionSandboxStatus;
  outputSummary: string;
  outputTrustStatus: LocalProviderExecutionResultOutputTrustStatus;
  outputMaterializationStatus: LocalProviderOutputMaterializationStatus;
  outputPromotionStatus: LocalProviderOutputPromotionStatus;
  promotionAvailabilityStatus: LocalProviderOutputPromotionStatus;
  descriptiveOnly: true;
  providerOutputTrusted: false;
  candidateOutputPromoted: false;
  decisionAppended: false;
  replayRepaired: false;
  releaseOrDeploymentEvidenceCreated: false;
}>;

export type LocalProviderExecutionValidation = Readonly<{
  status: LocalProviderExecutionStatus;
  providerKind: LocalProviderKind | null;
  errorCodes: readonly LocalProviderExecutionError[];
  reason: string;
}>;

export type LocalProviderExecutionProjection = Readonly<{
  status: LocalProviderExecutionStatus;
  projectionStatus: LocalProviderExecutionResultProjectionStatus;
  configuredProviderKind: string;
  sandboxStatus: LocalProviderExecutionSandboxStatus;
  result: LocalProviderExecutionResult | null;
  outputTrustStatus: LocalProviderOutputTrustStatus;
  outputMaterializationStatus: LocalProviderOutputMaterializationStatus;
  outputPromotionStatus: LocalProviderOutputPromotionStatus;
  promotionAvailabilityStatus: LocalProviderOutputPromotionStatus;
  linkage: LocalProviderExecutionResultLinkage;
  absenceMarkers: LocalProviderExecutionResultAbsenceMarkers;
  projectionValidation: LocalProviderExecutionResultProjectionValidation;
  validationStatus: LocalProviderExecutionStatus;
  validationErrorCodes: readonly LocalProviderExecutionError[];
  validationReason: string;
  capabilitySurface: LocalProviderExecutionCapabilitySurface;
  note: string;
}>;

export function deterministicStubProviderExecutionRequest(inputSummary = "local deterministic provider input"): LocalProviderExecutionRequest {
  return { providerKind: "deterministic_stub", inputSummary, fields: [] };
}

export function localProviderExecutionCapabilitySurface(): LocalProviderExecutionCapabilitySurface {
  return {
    deterministicStubExecutionSupported: true,
    supportedProviderKind: "deterministic_stub",
    cloudCallsEnabled: false,
    networkEnabled: false,
    shellCommandsEnabled: false,
    filesystemEnabled: false,
    secretsAllowed: false,
    trustGranted: false,
    readinessApproved: false,
    releaseApproved: false,
    deploymentEnabled: false,
    signingEnabled: false,
    publishingEnabled: false,
    publicUseEnabled: false,
    summary: "sandboxed deterministic provider execution supports deterministic_stub only; no cloud, network, shell, filesystem, secrets, trust, readiness, release, deployment, signing, publishing, or public-use capability"
  };
}


export function localProviderOutputValidationNoEffects(): LocalProviderOutputValidationEffect {
  return "none";
}

function localProviderOutputCandidateBoundaryStatuses(): readonly LocalProviderOutputCandidateBoundaryStatus[] {
  return ["not_candidate_material", "candidate_conversion_not_performed", "candidate_conversion_requires_future_phase"];
}

export function initialLocalProviderOutputValidationProjection(): LocalProviderOutputValidationProjection {
  return {
    status: "not_validated",
    reviewabilityStatus: "not_reviewable",
    candidateBoundaryStatus: "not_candidate_material",
    candidateBoundaryStatuses: localProviderOutputCandidateBoundaryStatuses(),
    reasons: ["no_provider_execution_result", "missing_execution_result", "candidate_conversion_not_available_in_phase_143"],
    providerExecutionResultId: null,
    providerKind: "none",
    outputTrustStatus: "untrusted_descriptive",
    outputPromotionStatus: "not_promoted",
    trustEffect: localProviderOutputValidationNoEffects(),
    candidateEffect: localProviderOutputValidationNoEffects(),
    decisionLedgerEffect: localProviderOutputValidationNoEffects(),
    replayEffect: localProviderOutputValidationNoEffects(),
    exportEffect: localProviderOutputValidationNoEffects(),
    actionEffect: localProviderOutputValidationNoEffects(),
    readinessEffect: localProviderOutputValidationNoEffects(),
    releaseEffect: localProviderOutputValidationNoEffects(),
    deploymentEffect: localProviderOutputValidationNoEffects(),
    note: "Provider output validation has not run; provider output is not candidate material and cannot be approved in Phase 143."
  };
}

export function localProviderOutputValidationReasons(execution: LocalProviderExecutionProjection): readonly LocalProviderOutputValidationReason[] {
  const reasons = new Set<LocalProviderOutputValidationReason>();
  if (execution.projectionStatus !== "execution_projected") reasons.add("provider_execution_not_projected");
  const result = execution.result;
  if (!result) {
    reasons.add("no_provider_execution_result");
    reasons.add("missing_execution_result");
    reasons.add("candidate_conversion_not_available_in_phase_143");
    return [...reasons].sort();
  }
  if (result.providerKind !== "deterministic_stub") reasons.add("unsupported_provider_kind");
  const output = result.outputSummary.trim();
  if (output.length === 0) reasons.add("empty_output");
  if (result.outputSummary.length > 1024) reasons.add("output_too_large");
  if (!result.outputSummary.startsWith("deterministic_stub descriptive output for input_bytes=") || !result.outputSummary.includes(" checksum=") || result.sandboxStatus !== "sandboxed_deterministic_no_external_effects" || result.outputTrustStatus !== "untrusted/descriptive" || result.outputMaterializationStatus !== "projected_as_untrusted_output" || result.outputPromotionStatus !== "not_promoted" || result.promotionAvailabilityStatus !== "promotion_not_available_in_phase_142" || !result.descriptiveOnly || result.providerOutputTrusted || result.candidateOutputPromoted || result.decisionAppended || result.replayRepaired || result.releaseOrDeploymentEvidenceCreated) reasons.add("malformed_output");
  const lower = result.outputSummary.toLowerCase();
  if (["secret", "token", "api_key", "apikey", "credential"].some((needle) => lower.includes(needle))) reasons.add("contains_forbidden_secret_marker");
  if (["command", "shell", "process", "execute ", "run "].some((needle) => lower.includes(needle))) reasons.add("contains_execution_instruction");
  if (["http://", "https://", "network", "socket", "fetch("].some((needle) => lower.includes(needle))) reasons.add("contains_network_instruction");
  if (["filesystem", "file ", "write ", "path", "directory", "fs::write"].some((needle) => lower.includes(needle))) reasons.add("contains_filesystem_instruction");
  if (["readiness", "release", "deployment", "public-use", "public use", "production ready"].some((needle) => lower.includes(needle))) reasons.add("contains_readiness_or_release_claim");
  if (["trusted_output", "trusted output", "approved_output", "approved output", "approval granted", "trust_granted"].some((needle) => lower.includes(needle))) reasons.add("contains_trust_or_approval_claim");
  if (["action_executed", "authorize action", "action authorization", "perform action"].some((needle) => lower.includes(needle))) reasons.add("contains_action_instruction");
  if (reasons.size === 0) reasons.add("deterministic_stub_output_shape_valid");
  reasons.add("candidate_conversion_not_available_in_phase_143");
  return [...reasons].sort();
}

export function validateLocalProviderOutput(execution: LocalProviderExecutionProjection): LocalProviderOutputValidationProjection {
  const reasons = localProviderOutputValidationReasons(execution);
  const result = execution.result;
  const validReasons = reasons.every((reason) => reason === "deterministic_stub_output_shape_valid" || reason === "candidate_conversion_not_available_in_phase_143");
  const status: LocalProviderOutputValidationStatus = !result
    ? "not_validated"
    : execution.projectionStatus !== "execution_projected"
      ? "validation_not_applicable"
      : validReasons
        ? "reviewable_untrusted"
        : "rejected";
  const reviewabilityStatus: LocalProviderOutputReviewabilityStatus = status === "reviewable_untrusted" ? "reviewable_untrusted" : status === "rejected" ? "rejected_before_review" : "not_reviewable";
  return {
    status,
    reviewabilityStatus,
    candidateBoundaryStatus: "not_candidate_material",
    candidateBoundaryStatuses: localProviderOutputCandidateBoundaryStatuses(),
    reasons,
    providerExecutionResultId: result?.resultId ?? null,
    providerKind: result?.providerKind ?? execution.configuredProviderKind,
    outputTrustStatus: "untrusted_descriptive",
    outputPromotionStatus: result?.outputPromotionStatus ?? "not_promoted",
    trustEffect: "none",
    candidateEffect: "none",
    decisionLedgerEffect: "none",
    replayEffect: "none",
    exportEffect: "none",
    actionEffect: "none",
    readinessEffect: "none",
    releaseEffect: "none",
    deploymentEffect: "none",
    note: "reviewable_untrusted is not candidate material and cannot be approved in Phase 143; provider output is not promoted."
  };
}

export function projectLocalProviderOutputValidation(state: LocalOperatorShellState): LocalProviderOutputValidationProjection {
  return validateLocalProviderOutput(projectLocalProviderExecution(state));
}

export function validateLocalProviderOutputValidationProjection(projection: LocalProviderOutputValidationProjection): readonly LocalProviderOutputValidationError[] {
  const errors: LocalProviderOutputValidationError[] = [];
  if (projection.reasons.length === 0) errors.push("missing_validation_reason");
  if (projection.status === "reviewable_untrusted" && projection.outputTrustStatus !== "untrusted_descriptive") errors.push("invalid_reviewable_trust_status");
  if (projection.candidateBoundaryStatus !== "not_candidate_material" || !projection.candidateBoundaryStatuses.includes("not_candidate_material") || !projection.candidateBoundaryStatuses.includes("candidate_conversion_not_performed") || !projection.candidateBoundaryStatuses.includes("candidate_conversion_requires_future_phase")) errors.push("invalid_candidate_boundary_status");
  if (projection.outputPromotionStatus !== "not_promoted") errors.push("invalid_promotion_status");
  const effects = [projection.trustEffect, projection.candidateEffect, projection.decisionLedgerEffect, projection.replayEffect, projection.exportEffect, projection.actionEffect, projection.readinessEffect, projection.releaseEffect, projection.deploymentEffect];
  if (effects.some((effect) => effect !== "none")) errors.push("invalid_no_effect_boundary");
  return errors;
}


export type StagedCandidateConversionProposalStatus =
  | "no_proposal"
  | "staged_proposal_created"
  | "source_not_reviewable_untrusted"
  | "rejected_source_not_eligible"
  | "invalid_proposal_request";
export type StagedCandidateConversionBoundaryStatus =
  | "staging_only_not_candidate_material"
  | "candidate_conversion_not_performed"
  | "validation_required_in_future_phase"
  | "approval_not_available_in_phase_146";
export type StagedCandidateConversionTrustStatus = "untrusted_source" | "not_trusted" | "not_approved";
export type StagedCandidateConversionEffectStatus =
  | "no_decision_ledger_effect"
  | "no_replay_effect"
  | "no_export_effect"
  | "no_provider_configuration_effect"
  | "no_provider_execution_effect"
  | "no_action_effect"
  | "no_persistence_effect"
  | "no_readiness_effect"
  | "no_release_effect"
  | "no_deployment_effect"
  | "not_executable"
  | "not_persistent";
export type StagedCandidateConversionSourceEligibilityStatus =
  | "eligible_reviewable_untrusted"
  | "missing_provider_execution_result"
  | "source_not_reviewable_untrusted"
  | "rejected_source_not_eligible"
  | "validation_not_applicable_source_not_eligible"
  | "invalid_validation_input_source_not_eligible"
  | "missing_or_inconsistent_validation_projection";
export type StagedCandidateConversionProposalError =
  | "missing_provider_execution_result"
  | "source_not_reviewable_untrusted"
  | "rejected_source_not_eligible"
  | "validation_not_applicable_source_not_eligible"
  | "invalid_validation_input_source_not_eligible"
  | "missing_or_inconsistent_validation_projection"
  | "invalid_proposal_request"
  | "invalid_proposal_boundary";

export type StagedCandidateConversionProposalRequest = Readonly<{
  operatorNote: string;
  claims?: readonly Readonly<{ key: string; value: string }>[];
}>;

export type StagedCandidateConversionProposal = Readonly<{
  proposalId: string;
  sourceProviderKind: string;
  sourceExecutionResultId: string;
  sourceValidationStatus: LocalProviderOutputValidationStatus;
  sourceReviewabilityStatus: LocalProviderOutputReviewabilityStatus;
  sourceCandidateBoundaryStatus: LocalProviderOutputCandidateBoundaryStatus;
  sourceBoundary: "provider_output_validation_phase_143";
  proposalBoundary: "staged_candidate_conversion_phase_146";
  boundaryStatuses: readonly StagedCandidateConversionBoundaryStatus[];
  trustStatuses: readonly StagedCandidateConversionTrustStatus[];
  effectStatuses: readonly StagedCandidateConversionEffectStatus[];
  sourceEligibilityStatus: StagedCandidateConversionSourceEligibilityStatus;
  note: string;
}>;

export type StagedCandidateConversionProposalProjection = Readonly<{
  status: StagedCandidateConversionProposalStatus;
  proposal: StagedCandidateConversionProposal | null;
  sourceEligibilityStatus: StagedCandidateConversionSourceEligibilityStatus;
  error: StagedCandidateConversionProposalError | null;
  note: string;
}>;

export function stagedCandidateConversionNoEffects(): readonly StagedCandidateConversionEffectStatus[] {
  return [
    "no_decision_ledger_effect",
    "no_replay_effect",
    "no_export_effect",
    "no_provider_configuration_effect",
    "no_provider_execution_effect",
    "no_action_effect",
    "no_persistence_effect",
    "no_readiness_effect",
    "no_release_effect",
    "no_deployment_effect",
    "not_executable",
    "not_persistent"
  ];
}

function stagedCandidateConversionBoundaryStatuses(): readonly StagedCandidateConversionBoundaryStatus[] {
  return [
    "staging_only_not_candidate_material",
    "candidate_conversion_not_performed",
    "validation_required_in_future_phase",
    "approval_not_available_in_phase_146"
  ];
}

function stagedCandidateConversionTrustStatuses(): readonly StagedCandidateConversionTrustStatus[] {
  return ["untrusted_source", "not_trusted", "not_approved"];
}

export function initialStagedCandidateConversionProposalProjection(): StagedCandidateConversionProposalProjection {
  return {
    status: "no_proposal",
    proposal: null,
    sourceEligibilityStatus: "missing_provider_execution_result",
    error: null,
    note: "No staged candidate-conversion proposal exists; Phase 146 staging is proposal only and not candidate material."
  };
}

export type StagedCandidateConversionValidationStatus =
  | "not_validated"
  | "staged_proposal_shape_valid"
  | "rejected_staged_proposal"
  | "invalid_validation_input";
export type StagedCandidateConversionValidationReason =
  | "no_staged_proposal"
  | "source_linkage_validated"
  | "staged_proposal_shape_valid"
  | "source_not_reviewable_untrusted"
  | "provider_output_validation_missing"
  | "provider_output_validation_inconsistent"
  | "provider_execution_result_missing"
  | "provider_execution_result_malformed"
  | "deterministic_proposal_id_mismatch"
  | "execution_result_id_mismatch"
  | "source_validation_status_mismatch"
  | "source_reviewability_status_mismatch"
  | "source_candidate_boundary_status_mismatch"
  | "boundary_flag_missing"
  | "boundary_flag_drift"
  | "no_effect_field_missing"
  | "no_effect_field_drift"
  | "future_phase_marker_missing"
  | "contains_trust_claim"
  | "contains_approval_claim"
  | "contains_safety_claim"
  | "contains_readiness_claim"
  | "contains_release_claim"
  | "contains_deployment_claim"
  | "contains_public_use_claim"
  | "contains_action_claim"
  | "contains_persistence_claim"
  | "contains_execution_claim"
  | "contains_candidate_creation_claim"
  | "contains_candidate_materialization_claim"
  | "candidate_materialization_not_performed"
  | "future_review_boundary_required"
  | "operator_decision_not_available_in_phase_147";
export type StagedCandidateConversionMaterializationStatus =
  | "not_materialized"
  | "materialization_not_available_in_phase_147"
  | "materialization_requires_future_phase";
export type StagedCandidateConversionOperatorDecisionStatus = "not_available_in_phase_147";
export type StagedCandidateConversionValidationBoundaryStatus =
  | "validation_shape_only"
  | "candidate_materialization_not_performed"
  | "future_review_boundary_required"
  | "operator_decision_not_available_in_phase_147";
export type StagedCandidateConversionValidationRequest = Readonly<{
  proposalId?: string;
}>;
export type StagedCandidateConversionValidationProjection = Readonly<{
  status: StagedCandidateConversionValidationStatus;
  reasons: readonly StagedCandidateConversionValidationReason[];
  proposalId: string | null;
  sourceProviderKind: string;
  sourceExecutionResultId: string | null;
  sourceValidationStatus: string;
  sourceReviewabilityStatus: string;
  sourceCandidateBoundaryStatus: string;
  deterministicLinkageStatus: string;
  materializationStatuses: readonly StagedCandidateConversionMaterializationStatus[];
  futureReviewBoundaryStatus: StagedCandidateConversionValidationBoundaryStatus;
  operatorDecisionStatus: StagedCandidateConversionOperatorDecisionStatus;
  trustStatuses: readonly StagedCandidateConversionTrustStatus[];
  boundaryStatuses: readonly StagedCandidateConversionValidationBoundaryStatus[];
  noEffectSummary: readonly StagedCandidateConversionEffectStatus[];
  note: string;
}>;

function stagedCandidateConversionValidationBoundaryStatuses(): readonly StagedCandidateConversionValidationBoundaryStatus[] {
  return [
    "validation_shape_only",
    "candidate_materialization_not_performed",
    "future_review_boundary_required",
    "operator_decision_not_available_in_phase_147"
  ];
}

export function initialStagedCandidateConversionValidationProjection(): StagedCandidateConversionValidationProjection {
  return {
    status: "not_validated",
    reasons: [],
    proposalId: null,
    sourceProviderKind: "none",
    sourceExecutionResultId: null,
    sourceValidationStatus: "not_validated",
    sourceReviewabilityStatus: "not_reviewable",
    sourceCandidateBoundaryStatus: "not_candidate_material",
    deterministicLinkageStatus: "not_validated",
    materializationStatuses: ["not_materialized", "materialization_not_available_in_phase_147", "materialization_requires_future_phase"],
    futureReviewBoundaryStatus: "future_review_boundary_required",
    operatorDecisionStatus: "not_available_in_phase_147",
    trustStatuses: stagedCandidateConversionTrustStatuses(),
    boundaryStatuses: stagedCandidateConversionValidationBoundaryStatuses(),
    noEffectSummary: stagedCandidateConversionNoEffects(),
    note: "Validation checks staged proposal shape and source linkage only. Validated staged proposal is not candidate output. Candidate materialization was not performed in Phase 147. Future review boundary is required before any operator decision. Operator decision is not available in Phase 147. Provider output remains untrusted and not approved."
  };
}

function proposalNoteClaimReasons(proposal: StagedCandidateConversionProposal): StagedCandidateConversionValidationReason[] {
  const lower = proposal.note.toLowerCase();
  const reasons: StagedCandidateConversionValidationReason[] = [];
  if (["trust granted", "is trusted", "mark trusted"].some((needle) => lower.includes(needle))) reasons.push("contains_trust_claim");
  if (["approval granted", "is approved", "mark approved"].some((needle) => lower.includes(needle))) reasons.push("contains_approval_claim");
  if (["is safe", "safe output", "safe for"].some((needle) => lower.includes(needle))) reasons.push("contains_safety_claim");
  if (["readiness", "ready for"].some((needle) => lower.includes(needle))) reasons.push("contains_readiness_claim");
  if (["release claim", "release evidence"].some((needle) => lower.includes(needle))) reasons.push("contains_release_claim");
  if (["deployment claim", "deployment evidence"].some((needle) => lower.includes(needle))) reasons.push("contains_deployment_claim");
  if (["public use", "public-use"].some((needle) => lower.includes(needle))) reasons.push("contains_public_use_claim");
  if (["action claim", "action effect"].some((needle) => lower.includes(needle))) reasons.push("contains_action_claim");
  if (["persistence claim", "persisted"].some((needle) => lower.includes(needle))) reasons.push("contains_persistence_claim");
  if (["execution claim", "executed proposal"].some((needle) => lower.includes(needle))) reasons.push("contains_execution_claim");
  if (["candidate creation", "candidate output created"].some((needle) => lower.includes(needle))) reasons.push("contains_candidate_creation_claim");
  if (["candidate materialization", "materialized candidate"].some((needle) => lower.includes(needle))) reasons.push("contains_candidate_materialization_claim");
  return reasons;
}

function sameSet<T>(left: readonly T[], right: readonly T[]): boolean {
  return left.length === right.length && right.every((item) => left.includes(item));
}

export function stagedCandidateConversionValidationReasons(
  state: LocalOperatorShellState,
  request: StagedCandidateConversionValidationRequest = {}
): readonly StagedCandidateConversionValidationReason[] {
  const reasons = new Set<StagedCandidateConversionValidationReason>();
  const proposal = state.stagedCandidateConversionProposal.proposal;
  if (!proposal) return ["no_staged_proposal"];
  if (request.proposalId && request.proposalId !== proposal.proposalId) reasons.add("deterministic_proposal_id_mismatch");
  const result = state.providerExecution.result;
  if (!result) return ["provider_execution_result_missing"];
  if (state.providerExecution.projectionValidation.status !== "valid" || state.providerExecution.projectionStatus !== "execution_projected") reasons.add("provider_execution_result_malformed");
  if (state.providerOutputValidation.reasons.length === 0 || !state.providerOutputValidation.providerExecutionResultId) {
    reasons.add("provider_output_validation_missing");
  } else if (validateLocalProviderOutputValidationProjection(state.providerOutputValidation).length > 0 || JSON.stringify(projectLocalProviderOutputValidation(state)) !== JSON.stringify(state.providerOutputValidation)) {
    reasons.add("provider_output_validation_inconsistent");
  }
  if (state.providerOutputValidation.status !== "reviewable_untrusted") reasons.add("source_not_reviewable_untrusted");
  const validationExecutionId = state.providerOutputValidation.providerExecutionResultId;
  if (!validationExecutionId) return [...reasons, "provider_output_validation_missing"];
  if (result.resultId !== validationExecutionId || proposal.sourceExecutionResultId !== validationExecutionId) reasons.add("execution_result_id_mismatch");
  if (proposal.proposalId !== deterministicStagedCandidateConversionProposalId(validationExecutionId, state.providerOutputValidation)) reasons.add("deterministic_proposal_id_mismatch");
  if (proposal.sourceValidationStatus !== state.providerOutputValidation.status || proposal.sourceValidationStatus !== "reviewable_untrusted") reasons.add("source_validation_status_mismatch");
  if (proposal.sourceReviewabilityStatus !== state.providerOutputValidation.reviewabilityStatus || proposal.sourceReviewabilityStatus !== "reviewable_untrusted") reasons.add("source_reviewability_status_mismatch");
  if (proposal.sourceCandidateBoundaryStatus !== state.providerOutputValidation.candidateBoundaryStatus || proposal.sourceCandidateBoundaryStatus !== "not_candidate_material") reasons.add("source_candidate_boundary_status_mismatch");
  if (!stagedCandidateConversionBoundaryStatuses().every((status) => proposal.boundaryStatuses.includes(status))) reasons.add("boundary_flag_missing");
  if (!sameSet(proposal.boundaryStatuses, stagedCandidateConversionBoundaryStatuses())) reasons.add("boundary_flag_drift");
  if (!proposal.boundaryStatuses.includes("validation_required_in_future_phase") || !proposal.boundaryStatuses.includes("approval_not_available_in_phase_146")) reasons.add("future_phase_marker_missing");
  if (!stagedCandidateConversionTrustStatuses().every((status) => proposal.trustStatuses.includes(status))) reasons.add("boundary_flag_missing");
  if (!sameSet(proposal.trustStatuses, stagedCandidateConversionTrustStatuses())) reasons.add("boundary_flag_drift");
  if (!stagedCandidateConversionNoEffects().every((status) => proposal.effectStatuses.includes(status))) reasons.add("no_effect_field_missing");
  if (!sameSet(proposal.effectStatuses, stagedCandidateConversionNoEffects())) reasons.add("no_effect_field_drift");
  if (proposal.sourceBoundary !== "provider_output_validation_phase_143" || proposal.proposalBoundary !== "staged_candidate_conversion_phase_146" || proposal.sourceEligibilityStatus !== "eligible_reviewable_untrusted") reasons.add("boundary_flag_drift");
  for (const reason of proposalNoteClaimReasons(proposal)) reasons.add(reason);
  if (reasons.size === 0) return ["candidate_materialization_not_performed", "future_review_boundary_required", "operator_decision_not_available_in_phase_147", "source_linkage_validated", "staged_proposal_shape_valid"].sort() as StagedCandidateConversionValidationReason[];
  return [...reasons].sort();
}

export function projectStagedCandidateConversionValidation(
  state: LocalOperatorShellState,
  request: StagedCandidateConversionValidationRequest = {}
): StagedCandidateConversionValidationProjection {
  const reasons = stagedCandidateConversionValidationReasons(state, request);
  const proposal = state.stagedCandidateConversionProposal.proposal;
  const validReasons: readonly StagedCandidateConversionValidationReason[] = ["candidate_materialization_not_performed", "future_review_boundary_required", "operator_decision_not_available_in_phase_147", "source_linkage_validated", "staged_proposal_shape_valid"];
  const status: StagedCandidateConversionValidationStatus = reasons.includes("no_staged_proposal")
    ? "invalid_validation_input"
    : reasons.every((reason) => validReasons.includes(reason))
      ? "staged_proposal_shape_valid"
      : "rejected_staged_proposal";
  return {
    status,
    reasons,
    proposalId: proposal?.proposalId ?? null,
    sourceProviderKind: proposal?.sourceProviderKind ?? "none",
    sourceExecutionResultId: proposal?.sourceExecutionResultId ?? null,
    sourceValidationStatus: proposal?.sourceValidationStatus ?? state.providerOutputValidation.status,
    sourceReviewabilityStatus: proposal?.sourceReviewabilityStatus ?? state.providerOutputValidation.reviewabilityStatus,
    sourceCandidateBoundaryStatus: proposal?.sourceCandidateBoundaryStatus ?? state.providerOutputValidation.candidateBoundaryStatus,
    deterministicLinkageStatus: status === "staged_proposal_shape_valid" ? "source_linkage_validated" : "not_validated",
    materializationStatuses: ["not_materialized", "materialization_not_available_in_phase_147", "materialization_requires_future_phase"],
    futureReviewBoundaryStatus: "future_review_boundary_required",
    operatorDecisionStatus: "not_available_in_phase_147",
    trustStatuses: stagedCandidateConversionTrustStatuses(),
    boundaryStatuses: stagedCandidateConversionValidationBoundaryStatuses(),
    noEffectSummary: stagedCandidateConversionNoEffects(),
    note: "Validation checks staged proposal shape and source linkage only. Validated staged proposal is not candidate output. Candidate materialization was not performed in Phase 147. Future review boundary is required before any operator decision. Operator decision is not available in Phase 147. Provider output remains untrusted and not approved."
  };
}

export function validateStagedCandidateConversionProposalForPhase147(
  state: LocalOperatorShellState,
  request: StagedCandidateConversionValidationRequest = {}
): LocalOperatorIntentResult {
  const next = { ...state, stagedCandidateConversionValidation: projectStagedCandidateConversionValidation(state, request) };
  return {
    status: next.stagedCandidateConversionValidation.status === "staged_proposal_shape_valid" ? "accepted" : "rejected",
    reason: next.stagedCandidateConversionValidation.status === "staged_proposal_shape_valid" ? "staged_candidate_conversion_validation_completed" : "staged_candidate_conversion_validation_rejected",
    state: next
  };
}

export type OperatorCandidateDecisionKind = "approve_validated_staged_proposal" | "reject_validated_staged_proposal";
export type OperatorCandidateDecisionStatus = "no_operator_decision" | "approved_validated_staged_proposal" | "rejected_validated_staged_proposal" | "rejected_operator_decision_request" | "invalid_operator_decision_input";
export type OperatorCandidateDecisionError =
  | "no_staged_proposal"
  | "staged_proposal_not_validated"
  | "staged_proposal_validation_rejected"
  | "invalid_validation_input"
  | "source_linkage_inconsistent"
  | "trust_claim_rejected"
  | "provider_output_approval_claim_rejected"
  | "readiness_claim_rejected"
  | "release_claim_rejected"
  | "deployment_claim_rejected"
  | "public_use_claim_rejected"
  | "action_claim_rejected"
  | "execution_claim_rejected"
  | "persistence_claim_rejected"
  | "candidate_creation_claim_rejected"
  | "candidate_materialization_claim_rejected";

export type OperatorCandidateDecisionRequest = Readonly<{
  kind: OperatorCandidateDecisionKind;
  stagedProposalId: string;
  providerExecutionResultId: string;
  stagedProposalValidationStatus: StagedCandidateConversionValidationStatus;
  claimsTrust?: boolean;
  claimsProviderOutputApproval?: boolean;
  claimsReadiness?: boolean;
  claimsRelease?: boolean;
  claimsDeployment?: boolean;
  claimsPublicUse?: boolean;
  claimsAction?: boolean;
  claimsExecution?: boolean;
  claimsPersistence?: boolean;
  claimsCandidateCreation?: boolean;
  claimsCandidateMaterialization?: boolean;
}>;

export type OperatorCandidateDecisionRecord = Readonly<{
  decisionId: string;
  decisionKind: OperatorCandidateDecisionKind;
  stagedProposalId: string;
  providerExecutionResultId: string;
  stagedProposalValidationStatus: StagedCandidateConversionValidationStatus;
  decisionScope: "decision_scope_validated_staged_proposal_only";
  materializationStatus: "candidate_materialization_not_performed";
  trustStatus: "provider_output_remains_untrusted";
  readinessStatus: "no_readiness_effect";
  releaseStatus: "no_release_effect";
  deploymentStatus: "no_deployment_effect";
  publicUseStatus: "no_public_use_effect";
  actionStatus: "no_action_effect";
  persistenceStatus: "no_persistence_effect";
  replayRepairStatus: "no_replay_repair_effect";
  recoveryPromotionStatus: "no_recovery_promotion_effect";
}>;

export type OperatorCandidateDecisionProjection = Readonly<{
  status: OperatorCandidateDecisionStatus;
  record: OperatorCandidateDecisionRecord | null;
  error: OperatorCandidateDecisionError | null;
  note: string;
}>;

export type Phase150CodeProductionHandoff = Readonly<{
  handoffId: string;
  status: "phase_150_code_production_handoff";
  implementedCapabilityEvidence: readonly string[];
  remainingProductionGradeGaps: readonly string[];
  remapRecommendations: readonly string[];
  phase149RoadmapEditStatus: "phase_149_does_not_edit_roadmap_files";
}>;

export function initialOperatorCandidateDecisionProjection(): OperatorCandidateDecisionProjection {
  return {
    status: "no_operator_decision",
    record: null,
    error: null,
    note: "No operator candidate decision has been recorded. Decision applies only to validated staged proposal when present; no candidate output is created in Phase 149."
  };
}

function deterministicOperatorCandidateDecisionId(request: OperatorCandidateDecisionRequest): string {
  const input = `${request.kind}|${request.stagedProposalId}|${request.providerExecutionResultId}|${request.stagedProposalValidationStatus}|phase_149`;
  let accumulator = 2166136261;
  for (let index = 0; index < input.length; index += 1) {
    accumulator ^= input.charCodeAt(index);
    accumulator = Math.imul(accumulator, 16777619) >>> 0;
  }
  return `operator-candidate-decision-${accumulator.toString(16).padStart(8, "0")}`;
}

export function validateOperatorCandidateDecisionRequest(state: LocalOperatorShellState, request: OperatorCandidateDecisionRequest): OperatorCandidateDecisionError | null {
  if (request.claimsTrust) return "trust_claim_rejected";
  if (request.claimsProviderOutputApproval) return "provider_output_approval_claim_rejected";
  if (request.claimsReadiness) return "readiness_claim_rejected";
  if (request.claimsRelease) return "release_claim_rejected";
  if (request.claimsDeployment) return "deployment_claim_rejected";
  if (request.claimsPublicUse) return "public_use_claim_rejected";
  if (request.claimsAction) return "action_claim_rejected";
  if (request.claimsExecution) return "execution_claim_rejected";
  if (request.claimsPersistence) return "persistence_claim_rejected";
  if (request.claimsCandidateCreation) return "candidate_creation_claim_rejected";
  if (request.claimsCandidateMaterialization) return "candidate_materialization_claim_rejected";
  const proposal = state.stagedCandidateConversionProposal.proposal;
  if (!proposal) return "no_staged_proposal";
  if (state.stagedCandidateConversionValidation.status === "not_validated") return "staged_proposal_not_validated";
  if (state.stagedCandidateConversionValidation.status === "rejected_staged_proposal") return "staged_proposal_validation_rejected";
  if (state.stagedCandidateConversionValidation.status === "invalid_validation_input") return "invalid_validation_input";
  if (request.stagedProposalValidationStatus !== "staged_proposal_shape_valid") return "source_linkage_inconsistent";
  if (request.stagedProposalId !== proposal.proposalId || request.providerExecutionResultId !== proposal.sourceExecutionResultId) return "source_linkage_inconsistent";
  if (state.stagedCandidateConversionValidation.proposalId !== request.stagedProposalId || state.stagedCandidateConversionValidation.sourceExecutionResultId !== request.providerExecutionResultId) return "source_linkage_inconsistent";
  if (state.stagedCandidateConversionValidation.deterministicLinkageStatus !== "source_linkage_validated") return "source_linkage_inconsistent";
  const reprojected = projectStagedCandidateConversionValidation(state, { proposalId: proposal.proposalId });
  if (JSON.stringify(reprojected) !== JSON.stringify(state.stagedCandidateConversionValidation)) return "source_linkage_inconsistent";
  return null;
}

export function projectOperatorCandidateDecision(request: OperatorCandidateDecisionRequest): OperatorCandidateDecisionProjection {
  const status: OperatorCandidateDecisionStatus = request.kind === "approve_validated_staged_proposal" ? "approved_validated_staged_proposal" : "rejected_validated_staged_proposal";
  return {
    status,
    error: null,
    record: {
      decisionId: deterministicOperatorCandidateDecisionId(request),
      decisionKind: request.kind,
      stagedProposalId: request.stagedProposalId,
      providerExecutionResultId: request.providerExecutionResultId,
      stagedProposalValidationStatus: request.stagedProposalValidationStatus,
      decisionScope: "decision_scope_validated_staged_proposal_only",
      materializationStatus: "candidate_materialization_not_performed",
      trustStatus: "provider_output_remains_untrusted",
      readinessStatus: "no_readiness_effect",
      releaseStatus: "no_release_effect",
      deploymentStatus: "no_deployment_effect",
      publicUseStatus: "no_public_use_effect",
      actionStatus: "no_action_effect",
      persistenceStatus: "no_persistence_effect",
      replayRepairStatus: "no_replay_repair_effect",
      recoveryPromotionStatus: "no_recovery_promotion_effect"
    },
    note: "This decision applies only to the validated staged proposal. No candidate output is created in Phase 149. Provider output remains untrusted and not approved. This decision does not approve readiness, release, deployment, or public use."
  };
}

export function rejectedOperatorCandidateDecisionProjection(error: OperatorCandidateDecisionError): OperatorCandidateDecisionProjection {
  return {
    status: "rejected_operator_decision_request",
    record: null,
    error,
    note: "Operator candidate decision request rejected; authoritative shell state is preserved and no candidate materialization is performed."
  };
}

export function submitOperatorCandidateDecision(state: LocalOperatorShellState, request: OperatorCandidateDecisionRequest): LocalOperatorIntentResult {
  const error = validateOperatorCandidateDecisionRequest(state, request);
  if (error) return { status: "rejected", reason: error, state: { ...state, operatorCandidateDecision: rejectedOperatorCandidateDecisionProjection(error), phase150CodeProductionHandoff: derivePhase150CodeProductionHandoff({ ...state, operatorCandidateDecision: rejectedOperatorCandidateDecisionProjection(error) }) } };
  const next = { ...state, operatorCandidateDecision: projectOperatorCandidateDecision(request) };
  return { status: "accepted", reason: "operator_candidate_decision_recorded", state: { ...next, phase150CodeProductionHandoff: derivePhase150CodeProductionHandoff(next) } };
}

export function phase150RemainingProductionGaps(): readonly string[] {
  return ["local session persistence", "session restore", "real adapter contract", "real provider invocation", "candidate materialization", "complete local operator workflow", "run history", "export package", "controlled trial readiness", "deployment/package path"];
}

export function derivePhase150CodeProductionHandoff(state: LocalOperatorShellState): Phase150CodeProductionHandoff {
  return {
    handoffId: `phase-150-code-production-handoff-${state.providerConfiguration.status}-${state.stagedCandidateConversionValidation.status}-${state.operatorCandidateDecision.status}`,
    status: "phase_150_code_production_handoff",
    implementedCapabilityEvidence: [
      `provider configuration: ${state.providerConfiguration.status}`,
      `deterministic provider execution: ${state.providerExecution.projectionStatus}`,
      `provider execution result projection: ${state.providerExecution.result?.resultId ?? "none"}`,
      `provider output validation: ${state.providerOutputValidation.status}`,
      `provider output review: ${state.providerOutputValidation.reviewabilityStatus}`,
      `staged candidate-conversion proposal: ${state.stagedCandidateConversionProposal.status}`,
      `staged proposal validation: ${state.stagedCandidateConversionValidation.status}`,
      `candidate review surface: ${state.stagedCandidateConversionValidation.status === "staged_proposal_shape_valid" ? "validated_staged_proposal_review" : "not_available"}`,
      `operator decision boundary: ${state.operatorCandidateDecision.status}`
    ],
    remainingProductionGradeGaps: phase150RemainingProductionGaps(),
    remapRecommendations: [
      "Phase 150 should perform an aggressive production-path remap.",
      "Phase 150 should group larger product capability phases.",
      "Safety checks remain embedded in implementation phases.",
      "Phase 150 is the roadmap/changelog alignment phase.",
      "Phase 149 does not edit roadmap files."
    ],
    phase149RoadmapEditStatus: "phase_149_does_not_edit_roadmap_files"
  };
}

function deterministicStagedCandidateConversionProposalId(executionResultId: string, validation: LocalProviderOutputValidationProjection): string {
  const input = `${validation.providerKind}|${executionResultId}|${validation.status}|${validation.reviewabilityStatus}|${validation.candidateBoundaryStatus}|phase_146`;
  let accumulator = 2166136261;
  for (let index = 0; index < input.length; index += 1) {
    accumulator ^= input.charCodeAt(index);
    accumulator = Math.imul(accumulator, 16777619) >>> 0;
  }
  return `staged-candidate-conversion-proposal-${accumulator.toString(16).padStart(8, "0")}`;
}

function proposalRequestContainsForbiddenClaim(request: StagedCandidateConversionProposalRequest): boolean {
  const claims = request.claims ?? [];
  return claims.some(({ key, value }) => {
    const text = `${key} ${value}`.toLowerCase();
    return ["trust", "approval", "approved", "safe", "readiness", "ready", "release", "deployment", "public-use", "public_use", "execute", "execution", "persistence", "persistent", "action", "candidate_creation", "candidate_output", "candidate_material", "conversion_performed"].some((needle) => text.includes(needle));
  });
}

export function validateStagedCandidateConversionSource(state: LocalOperatorShellState): StagedCandidateConversionSourceEligibilityStatus {
  if (!state.providerExecution.result) return "missing_provider_execution_result";
  if (validateLocalProviderOutputValidationProjection(state.providerOutputValidation).length > 0) return "missing_or_inconsistent_validation_projection";
  const projected = projectLocalProviderOutputValidation(state);
  if (JSON.stringify(projected) !== JSON.stringify(state.providerOutputValidation)) return "missing_or_inconsistent_validation_projection";
  if (state.providerOutputValidation.status === "rejected") return "rejected_source_not_eligible";
  if (state.providerOutputValidation.status === "validation_not_applicable") return "validation_not_applicable_source_not_eligible";
  if (state.providerOutputValidation.status === "invalid_validation_input") return "invalid_validation_input_source_not_eligible";
  if (state.providerOutputValidation.status !== "reviewable_untrusted") return "source_not_reviewable_untrusted";
  if (state.providerOutputValidation.reviewabilityStatus !== "reviewable_untrusted" || state.providerOutputValidation.candidateBoundaryStatus !== "not_candidate_material" || !state.providerOutputValidation.candidateBoundaryStatuses.includes("candidate_conversion_not_performed") || !state.providerOutputValidation.candidateBoundaryStatuses.includes("candidate_conversion_requires_future_phase")) return "source_not_reviewable_untrusted";
  return "eligible_reviewable_untrusted";
}

function sourceEligibilityError(status: StagedCandidateConversionSourceEligibilityStatus): StagedCandidateConversionProposalError | null {
  if (status === "eligible_reviewable_untrusted") return null;
  return status === "source_not_reviewable_untrusted" ? "source_not_reviewable_untrusted" : status;
}

export function validateStagedCandidateConversionProposal(projection: StagedCandidateConversionProposalProjection): StagedCandidateConversionProposalError | null {
  if (projection.status === "no_proposal") return null;
  const proposal = projection.proposal;
  if (!proposal) return "invalid_proposal_boundary";
  if (!stagedCandidateConversionBoundaryStatuses().every((status) => proposal.boundaryStatuses.includes(status))) return "invalid_proposal_boundary";
  if (!stagedCandidateConversionTrustStatuses().every((status) => proposal.trustStatuses.includes(status))) return "invalid_proposal_boundary";
  if (!stagedCandidateConversionNoEffects().every((status) => proposal.effectStatuses.includes(status))) return "invalid_proposal_boundary";
  if (proposal.sourceValidationStatus !== "reviewable_untrusted" || proposal.sourceReviewabilityStatus !== "reviewable_untrusted" || proposal.sourceCandidateBoundaryStatus !== "not_candidate_material" || proposal.sourceBoundary !== "provider_output_validation_phase_143" || proposal.proposalBoundary !== "staged_candidate_conversion_phase_146" || proposal.sourceEligibilityStatus !== "eligible_reviewable_untrusted") return "invalid_proposal_boundary";
  return null;
}

export function createStagedCandidateConversionProposal(state: LocalOperatorShellState, request: StagedCandidateConversionProposalRequest): LocalOperatorIntentResult {
  if (proposalRequestContainsForbiddenClaim(request)) return { status: "rejected", reason: "invalid_proposal_request", state };
  const eligibility = validateStagedCandidateConversionSource(state);
  const eligibilityError = sourceEligibilityError(eligibility);
  if (eligibilityError) return { status: "rejected", reason: eligibilityError, state };
  const executionResultId = state.providerOutputValidation.providerExecutionResultId;
  if (!executionResultId) return { status: "rejected", reason: "missing_provider_execution_result", state };
  const proposal: StagedCandidateConversionProposal = {
    proposalId: deterministicStagedCandidateConversionProposalId(executionResultId, state.providerOutputValidation),
    sourceProviderKind: state.providerOutputValidation.providerKind,
    sourceExecutionResultId: executionResultId,
    sourceValidationStatus: state.providerOutputValidation.status,
    sourceReviewabilityStatus: state.providerOutputValidation.reviewabilityStatus,
    sourceCandidateBoundaryStatus: state.providerOutputValidation.candidateBoundaryStatus,
    sourceBoundary: "provider_output_validation_phase_143",
    proposalBoundary: "staged_candidate_conversion_phase_146",
    boundaryStatuses: stagedCandidateConversionBoundaryStatuses(),
    trustStatuses: stagedCandidateConversionTrustStatuses(),
    effectStatuses: stagedCandidateConversionNoEffects(),
    sourceEligibilityStatus: "eligible_reviewable_untrusted",
    note: `${request.operatorNote} This proposal is not persistent, not executable, not approved, and not candidate material.`
  };
  const projection: StagedCandidateConversionProposalProjection = {
    status: "staged_proposal_created",
    proposal,
    sourceEligibilityStatus: "eligible_reviewable_untrusted",
    error: null,
    note: "This is a staged conversion proposal only. It is not candidate output."
  };
  const error = validateStagedCandidateConversionProposal(projection);
  if (error) return { status: "rejected", reason: error, state };
  return { status: "accepted", reason: "staged_candidate_conversion_proposal_created", state: { ...state, stagedCandidateConversionProposal: projection } };
}

export function localProviderExecutionResultAbsenceMarkers(): LocalProviderExecutionResultAbsenceMarkers {
  return {
    noProcessSpawned: true,
    noNetworkSocketOpened: true,
    noFilesystemPersistence: true,
    noSecretsRead: true,
    noReleaseCreated: true,
    noDeploymentCreated: true,
    noSigningPerformed: true,
    noPublishingPerformed: true,
    noPublicUseApproved: true,
    noReadinessApproved: true,
    noReplayRepair: true,
    noRecoveryPromotion: true,
    noActionExecution: true,
    providerOutputNotCandidateMaterial: true,
    markerSummary: [
      "no process",
      "no network",
      "no filesystem persistence",
      "no secrets",
      "no release/deployment/signing/publishing/public-use/readiness",
      "no replay repair/recovery promotion/action execution",
      "provider output is not candidate material"
    ]
  };
}

function initialLocalProviderExecutionLinkage(): LocalProviderExecutionResultLinkage {
  return {
    shellStateLabel: "idle_local_harness",
    runId: "local-stub-run-133",
    providerConfigurationKind: "none",
    providerConfigurationStatus: "not_configured",
    executionResultId: "none",
    candidateId: "not_candidate_material",
    sourceBoundary: "sandboxed_deterministic_provider_execution"
  };
}

function localProviderExecutionLinkage(state: LocalOperatorShellState, executionResultId: string): LocalProviderExecutionResultLinkage {
  return {
    shellStateLabel: state.harnessStatus,
    runId: state.run.runId,
    providerConfigurationKind: state.providerConfiguration.configuredProviderKind ?? "none",
    providerConfigurationStatus: state.providerConfiguration.status,
    executionResultId,
    candidateId: state.run.candidate?.candidateId ?? "not_candidate_material",
    sourceBoundary: "sandboxed_deterministic_provider_execution"
  };
}

export function validateLocalProviderExecutionResultProjection(projection: LocalProviderExecutionProjection): LocalProviderExecutionResultProjectionValidation {
  const errors: string[] = [];
  if (projection.outputTrustStatus !== "untrusted_descriptive") errors.push("invalid_trust_status");
  if (projection.outputMaterializationStatus === "candidate_material") errors.push("invalid_materialization_status");
  if (projection.outputPromotionStatus !== "not_promoted" || projection.promotionAvailabilityStatus !== "promotion_not_available_in_phase_142") errors.push("invalid_promotion_status");
  const markers = projection.absenceMarkers;
  if (!markers.noProcessSpawned || !markers.noNetworkSocketOpened || !markers.noFilesystemPersistence || !markers.noSecretsRead || !markers.noReleaseCreated || !markers.noDeploymentCreated || !markers.noSigningPerformed || !markers.noPublishingPerformed || !markers.noPublicUseApproved || !markers.noReadinessApproved || !markers.noReplayRepair || !markers.noRecoveryPromotion || !markers.noActionExecution || !markers.providerOutputNotCandidateMaterial || markers.markerSummary.length === 0) errors.push("missing_absence_marker");
  if (projection.linkage.runId.length === 0 || projection.linkage.providerConfigurationKind.length === 0 || projection.linkage.providerConfigurationStatus.length === 0 || projection.linkage.executionResultId.length === 0 || projection.linkage.sourceBoundary !== "sandboxed_deterministic_provider_execution") errors.push("missing_linkage");
  return errors.length === 0
    ? { status: "valid", errorCodes: [], reason: "provider execution result projection is valid; output remains untrusted_descriptive, not_candidate_material, and not_promoted" }
    : { status: "invalid", errorCodes: errors, reason: "provider execution result projection rejected fail-closed" };
}

function withProviderExecutionProjectionValidation(projection: LocalProviderExecutionProjection): LocalProviderExecutionProjection {
  return { ...projection, projectionValidation: validateLocalProviderExecutionResultProjection(projection) };
}

export function initialLocalProviderExecutionProjection(): LocalProviderExecutionProjection {
  return withProviderExecutionProjectionValidation({
    status: "not_executed",
    projectionStatus: "not_executed",
    configuredProviderKind: "none",
    sandboxStatus: "not_entered",
    result: null,
    outputTrustStatus: "untrusted_descriptive",
    outputMaterializationStatus: "not_materialized",
    outputPromotionStatus: "not_promoted",
    promotionAvailabilityStatus: "promotion_not_available_in_phase_142",
    linkage: initialLocalProviderExecutionLinkage(),
    absenceMarkers: localProviderExecutionResultAbsenceMarkers(),
    projectionValidation: { status: "invalid", errorCodes: [], reason: "projection validation pending" },
    validationStatus: "not_executed",
    validationErrorCodes: [],
    validationReason: "deterministic_stub execution has not been requested",
    capabilitySurface: localProviderExecutionCapabilitySurface(),
    note: "Provider execution result projection is projection_only evidence; provider output is untrusted_descriptive, not_candidate_material, not_promoted, and not eligible for approve/reject in Phase 142."
  });
}

export function projectLocalProviderExecution(state: LocalOperatorShellState): LocalProviderExecutionProjection {
  return withProviderExecutionProjectionValidation({
    ...state.providerExecution,
    configuredProviderKind: state.providerConfiguration.configuredProviderKind ?? "none",
    linkage: localProviderExecutionLinkage(state, state.providerExecution.result?.resultId ?? "none")
  });
}

function forbiddenProviderExecutionField(key: string, value: string): LocalProviderExecutionError {
  const loweredKey = key.toLowerCase();
  const combined = `${loweredKey}=${value.toLowerCase()}`;
  if (["endpoint", "url", "host", "port", "http", "network"].some((needle) => combined.includes(needle))) return "forbidden_endpoint_field";
  if (["command", "args", "shell", "process"].some((needle) => combined.includes(needle))) return "forbidden_command_field";
  if (["path", "file", "directory"].some((needle) => combined.includes(needle))) return "forbidden_path_field";
  if (["secret", "token", "api_key", "apikey", "credential"].some((needle) => combined.includes(needle))) return "forbidden_secret_field";
  if (loweredKey === "provider_execution_enabled") return "provider_execution_flag_rejected";
  if (loweredKey === "trust_granted") return "trust_grant_rejected";
  if (loweredKey === "readiness_approved") return "readiness_claim_rejected";
  if (loweredKey === "release_candidate_approved") return "release_claim_rejected";
  if (loweredKey === "deployment_enabled") return "deployment_claim_rejected";
  if (loweredKey === "public_use_approved") return "public_use_claim_rejected";
  if (loweredKey === "signing_enabled") return "signing_claim_rejected";
  if (loweredKey === "publishing_enabled") return "publishing_claim_rejected";
  return "unknown_field_rejected";
}

export function validateLocalProviderExecutionRequest(
  configuration: LocalProviderConfiguration,
  request: LocalProviderExecutionRequest
): LocalProviderExecutionValidation {
  const errors = new Set<LocalProviderExecutionError>();
  if (configuration.status !== "accepted" || configuration.configuredProviderKind !== "deterministic_stub") {
    errors.add(configuration.status === "not_configured" ? "missing_provider_configuration" : "rejected_provider_configuration");
  }

  let providerKind: LocalProviderKind | null = null;
  const rawKind = request.providerKind;
  if (rawKind === undefined || rawKind.trim().length === 0) {
    errors.add("missing_provider_kind");
  } else if (rawKind.trim() !== rawKind) {
    errors.add("malformed_provider_kind");
  } else if (supportedLocalProviderKinds.includes(rawKind as LocalProviderKind)) {
    providerKind = rawKind as LocalProviderKind;
    if (providerKind !== "deterministic_stub") errors.add("unsupported_provider_kind");
  } else {
    errors.add("unsupported_provider_kind");
  }

  if (request.inputSummary.trim().length === 0 || request.inputSummary.length > 4096) errors.add("malformed_provider_kind");
  for (const field of request.fields ?? []) errors.add(forbiddenProviderExecutionField(field.key, field.value));

  const errorCodes = [...errors].sort();
  if (errorCodes.length === 0 && providerKind === "deterministic_stub") {
    return {
      status: "executed",
      providerKind,
      errorCodes: [],
      reason: "deterministic_stub execution accepted inside Rust-owned sandboxed deterministic boundary"
    };
  }
  const status: LocalProviderExecutionStatus = errors.has("unsupported_provider_kind")
    ? "unsupported_provider"
    : errors.has("missing_provider_configuration") || errors.has("rejected_provider_configuration")
      ? "configuration_required"
      : "invalid_request";
  return {
    status,
    providerKind,
    errorCodes,
    reason: "provider execution rejected fail-closed; previous shell state is preserved"
  };
}

function deterministicProviderInputChecksum(input: string): number {
  let accumulator = 0x141;
  for (let index = 0; index < input.length; index += 1) {
    accumulator = Math.imul(accumulator, 33) + input.charCodeAt(index);
    accumulator >>>= 0;
  }
  return accumulator;
}

export function executeSandboxedDeterministicProvider(request: LocalProviderExecutionRequest): LocalProviderExecutionResult {
  const checksum = deterministicProviderInputChecksum(request.inputSummary);
  const hex = checksum.toString(16).padStart(8, "0");
  return {
    resultId: `local-provider-execution-deterministic_stub-${hex}`,
    providerKind: "deterministic_stub",
    sandboxStatus: "sandboxed_deterministic_no_external_effects",
    outputSummary: `deterministic_stub descriptive output for input_bytes=${request.inputSummary.length} checksum=${hex}`,
    outputTrustStatus: "untrusted/descriptive",
    outputMaterializationStatus: "projected_as_untrusted_output",
    outputPromotionStatus: "not_promoted",
    promotionAvailabilityStatus: "promotion_not_available_in_phase_142",
    descriptiveOnly: true,
    providerOutputTrusted: false,
    candidateOutputPromoted: false,
    decisionAppended: false,
    replayRepaired: false,
    releaseOrDeploymentEvidenceCreated: false
  };
}

export function applyLocalProviderExecution(
  state: LocalOperatorShellState,
  request: LocalProviderExecutionRequest
): LocalOperatorIntentResult {
  const validation = validateLocalProviderExecutionRequest(state.providerConfiguration, request);
  if (validation.status !== "executed") return { status: "rejected", reason: validation.reason, state };
  const result = executeSandboxedDeterministicProvider(request);
  const providerExecution = withProviderExecutionProjectionValidation({
    status: "executed",
        projectionStatus: "execution_projected",
        configuredProviderKind: "deterministic_stub",
        sandboxStatus: "sandboxed_deterministic_no_external_effects",
        result,
        outputTrustStatus: "untrusted_descriptive",
        outputMaterializationStatus: "not_candidate_material",
        outputPromotionStatus: result.outputPromotionStatus,
        promotionAvailabilityStatus: result.promotionAvailabilityStatus,
        linkage: localProviderExecutionLinkage(state, result.resultId),
        absenceMarkers: localProviderExecutionResultAbsenceMarkers(),
        projectionValidation: { status: "invalid", errorCodes: [], reason: "projection validation pending" },
        validationStatus: validation.status,
        validationErrorCodes: [],
        validationReason: validation.reason,
        capabilitySurface: localProviderExecutionCapabilitySurface(),
    note: "Provider execution result projection is projection_only evidence; provider output is untrusted_descriptive, not_candidate_material, not_promoted, promotion_not_available_in_phase_142, and not eligible for approve/reject in Phase 142."
  });
  return {
    status: "accepted",
    reason: "local_provider_execution_accepted",
    state: {
      ...state,
      providerExecution,
      providerOutputValidation: validateLocalProviderOutput(providerExecution)
    }
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
  providerConfiguration: LocalProviderConfiguration;
  providerExecution: LocalProviderExecutionProjection;
  providerOutputValidation: LocalProviderOutputValidationProjection;
  stagedCandidateConversionProposal: StagedCandidateConversionProposalProjection;
  stagedCandidateConversionValidation: StagedCandidateConversionValidationProjection;
  operatorCandidateDecision: OperatorCandidateDecisionProjection;
  phase150CodeProductionHandoff: Phase150CodeProductionHandoff;
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
  const state = attachLocalSessionEvidenceExport({
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
    decisionLedger,
    providerConfiguration: initialLocalProviderConfiguration(),
    providerExecution: initialLocalProviderExecutionProjection(),
    providerOutputValidation: initialLocalProviderOutputValidationProjection(),
    stagedCandidateConversionProposal: initialStagedCandidateConversionProposalProjection(),
    stagedCandidateConversionValidation: initialStagedCandidateConversionValidationProjection(),
    operatorCandidateDecision: initialOperatorCandidateDecisionProjection(),
    phase150CodeProductionHandoff: {
      handoffId: "phase-150-code-production-handoff-not_configured-not_validated-no_operator_decision",
      status: "phase_150_code_production_handoff",
      implementedCapabilityEvidence: [],
      remainingProductionGradeGaps: phase150RemainingProductionGaps(),
      remapRecommendations: [],
      phase149RoadmapEditStatus: "phase_149_does_not_edit_roadmap_files"
    }
  });
  return { ...state, phase150CodeProductionHandoff: derivePhase150CodeProductionHandoff(state) };
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
