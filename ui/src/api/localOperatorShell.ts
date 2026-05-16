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

export function projectLocalDecisionTimeline(
  ledger: LocalDecisionLedger,
): LocalDecisionTimelineProjection {
  return {
    records: ledger.records,
    emptyMessage: "No recorded local operator decisions",
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
    summary:
      "No local operator decision has been recorded for replay projection.",
  };
}

function inconsistentLocalDecisionReplayProjection(
  ledger: LocalDecisionLedger,
  error: LocalDecisionReplayError,
): LocalDecisionReplayProjection {
  const latest =
    ledger.records.length === 0
      ? null
      : ledger.records[ledger.records.length - 1];
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
    summary: `Local decision ledger is inconsistent: ${error}.`,
  };
}

export function validateLocalDecisionReplayInput(
  run: LocalRunProjection,
  ledger: LocalDecisionLedger,
): LocalDecisionReplayError | null {
  for (const [index, record] of ledger.records.entries()) {
    const expectedSequence = index + 1;
    if (
      record.decisionId.length === 0 ||
      record.runId.length === 0 ||
      record.candidateId.length === 0 ||
      record.operatorId.length === 0
    ) {
      return "empty_record_field";
    }
    if (record.recordedSequence !== expectedSequence)
      return "sequence_mismatch";
    if (
      record.decisionId !==
      `local-decision-${String(expectedSequence).padStart(4, "0")}`
    )
      return "decision_id_mismatch";
    if (record.decisionStatus !== "recorded")
      return "unsupported_decision_status";
  }

  const latest =
    ledger.records.length === 0
      ? undefined
      : ledger.records[ledger.records.length - 1];
  if (latest) {
    if (latest.runId !== run.runId) return "run_mismatch";
    if (run.candidate && latest.candidateId !== run.candidate.candidateId)
      return "candidate_mismatch";
  }

  return null;
}

export function deriveLocalDecisionReplayProjection(
  run: LocalRunProjection,
  ledger: LocalDecisionLedger,
): LocalDecisionReplayProjection {
  if (ledger.records.length === 0)
    return initialLocalDecisionReplayProjection();

  const error = validateLocalDecisionReplayInput(run, ledger);
  if (error) return inconsistentLocalDecisionReplayProjection(ledger, error);

  const entries = ledger.records.map(
    (record): LocalDecisionReplayEntry => ({
      replaySequence: `local-replay-entry-${String(record.recordedSequence).padStart(4, "0")}`,
      decisionId: record.decisionId,
      runId: record.runId,
      candidateId: record.candidateId,
      operatorId: record.operatorId,
      decisionKind: record.intentKind,
      decisionStatus: record.decisionStatus,
    }),
  );
  const latest = ledger.records[ledger.records.length - 1];
  const replayStatus: LocalDecisionReplayStatus =
    latest.intentKind === "approve"
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
    summary: `Local decision replay projection derived ${ledger.records.length} recorded decision(s); latest decision ${latest.decisionId} is ${replayStatus}.`,
  };
}

export type LocalSessionEvidenceExportStatus =
  | "no_completed_run_evidence"
  | "run_evidence_projected"
  | "decision_evidence_projected";
export type LocalSessionEvidenceExportValidationStatus =
  | "complete"
  | "incomplete";

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
      "readiness absent",
    ],
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
export type LocalProviderConfigurationStatus =
  | "not_configured"
  | "accepted"
  | "rejected"
  | "unsupported";
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
  "unknown",
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
    summary:
      "deterministic_stub configuration-only surface; no execution, cloud, network, shell, filesystem, secrets, trust, readiness, release, or deployment capability",
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
      reason:
        "no executable provider configured; deterministic_stub is available for configuration-only validation",
    },
  };
}

export function projectLocalProviderConfiguration(
  configuration: LocalProviderConfiguration,
): LocalProviderConfigurationProjection {
  return {
    configuredProviderKind: configuration.configuredProviderKind ?? "none",
    status: configuration.status,
    validationStatus: configuration.lastValidation.status,
    validationReason: configuration.lastValidation.reason,
    validationErrorCodes: configuration.lastValidation.errorCodes,
    executionStatus: "disabled_not_executed",
    capabilitySurface: localProviderConfigurationCapabilitySurface(),
    note: "Phase 139 deterministic_stub is configuration-only; validation does not execute providers or approve trust/readiness/release/deployment.",
  };
}

function forbiddenProviderConfigurationField(
  key: string,
  value: string,
): LocalProviderConfigurationError | null {
  const loweredKey = key.toLowerCase();
  const combined = `${loweredKey}=${value.toLowerCase()}`;
  if (loweredKey === "label" || loweredKey === "description") return null;
  if (
    ["endpoint", "url", "host", "port", "http", "network"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "forbidden_endpoint_field";
  if (
    ["command", "args", "shell", "process"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "forbidden_command_field";
  if (["path", "file", "directory"].some((needle) => combined.includes(needle)))
    return "forbidden_path_field";
  if (
    ["secret", "token", "api_key", "apikey", "credential"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "forbidden_secret_field";
  if (loweredKey === "provider_execution_enabled" && value === "true")
    return "provider_execution_rejected";
  if (loweredKey === "trust_granted" && value === "true")
    return "trust_grant_rejected";
  if (loweredKey === "readiness_approved" && value === "true")
    return "readiness_claim_rejected";
  if (loweredKey === "release_candidate_approved" && value === "true")
    return "release_claim_rejected";
  if (loweredKey === "deployment_enabled" && value === "true")
    return "deployment_claim_rejected";
  if (loweredKey === "public_use_approved" && value === "true")
    return "public_use_claim_rejected";
  if (loweredKey === "signing_enabled" && value === "true")
    return "signing_claim_rejected";
  if (loweredKey === "publishing_enabled" && value === "true")
    return "publishing_claim_rejected";
  return "unknown_field_rejected";
}

export function validateLocalProviderConfiguration(
  candidate: LocalProviderConfigurationCandidate,
): LocalProviderConfigurationValidation {
  const errors = new Set<LocalProviderConfigurationError>();
  let providerKind: LocalProviderKind | null = null;
  const rawKind = candidate.providerKind;
  if (rawKind === undefined || rawKind.trim().length === 0) {
    errors.add("missing_provider_kind");
  } else if (rawKind.trim() !== rawKind) {
    errors.add("malformed_provider_kind");
  } else if (
    supportedLocalProviderKinds.includes(rawKind as LocalProviderKind)
  ) {
    providerKind = rawKind as LocalProviderKind;
    if (providerKind !== "deterministic_stub")
      errors.add("unsupported_provider_kind");
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
      reason:
        "deterministic_stub configuration accepted as local-session configuration-only state; provider execution remains disabled",
    };
  }
  return {
    status: errors.has("unsupported_provider_kind")
      ? "unsupported"
      : "rejected",
    providerKind,
    errorCodes,
    reason:
      "provider configuration rejected fail-closed; prior configuration remains unchanged and no provider execution occurs",
  };
}

export function applyLocalProviderConfigurationCandidate(
  state: LocalOperatorShellState,
  candidate: LocalProviderConfigurationCandidate,
): LocalOperatorIntentResult {
  const validation = validateLocalProviderConfiguration(candidate);
  if (validation.status !== "accepted")
    return { status: "rejected", reason: validation.reason, state };
  return {
    status: "accepted",
    reason: "local_provider_configuration_accepted",
    state: attachLocalSessionEvidenceExport({
      ...state,
      providerConfiguration: {
        configuredProviderKind: validation.providerKind,
        status: "accepted",
        lastValidation: validation,
      },
    }),
  };
}

export type LocalProviderAdapterKind =
  | "deterministic_fake_adapter"
  | "local_model_adapter_contract"
  | "unsupported_local_model"
  | "unsupported_cloud_model"
  | "unsupported_network_adapter"
  | "unsupported_shell_adapter"
  | "unsupported_filesystem_adapter"
  | "unknown";
export type LocalProviderAdapterValidationStatus =
  | "registry_projected"
  | "adapter_declared_non_executing"
  | "adapter_rejected"
  | "unsupported_adapter"
  | "invalid_adapter_declaration";
export type LocalProviderAdapterValidationError =
  | "missing_adapter_kind"
  | "malformed_adapter_kind"
  | "unsupported_adapter"
  | "cloud_or_network_adapter_rejected"
  | "shell_adapter_rejected"
  | "filesystem_adapter_rejected"
  | "executable_path_rejected"
  | "endpoint_field_rejected"
  | "command_field_rejected"
  | "path_field_rejected"
  | "secret_field_rejected"
  | "execution_flag_rejected"
  | "provider_trust_flag_rejected"
  | "readiness_claim_rejected"
  | "release_claim_rejected"
  | "deployment_claim_rejected"
  | "public_use_claim_rejected"
  | "signing_claim_rejected"
  | "publishing_claim_rejected"
  | "unknown_field_rejected";
export type LocalProviderAdapterExecutionStatus =
  "execution_not_available_in_phase_153";
export type LocalProviderAdapterTrustStatus = "no_provider_trust";
export type LocalProviderAdapterBoundaryStatus =
  | "contract_only"
  | "no_execution"
  | "no_provider_trust"
  | "no_network"
  | "no_shell"
  | "no_secrets"
  | "no_production_persistence"
  | "no_readiness_effect"
  | "no_release_effect"
  | "no_deployment_effect"
  | "no_public_use_effect";

export type LocalProviderAdapterCapabilitySurface = Readonly<{
  contractOnly: true;
  noExecution: true;
  noProviderTrust: true;
  noNetwork: true;
  noShell: true;
  noSecrets: true;
  noProductionPersistence: true;
  noReadinessEffect: true;
  noReleaseEffect: true;
  noDeploymentEffect: true;
  noPublicUseEffect: true;
  summary: string;
}>;

export type LocalProviderAdapterContract = Readonly<{
  adapterKind: LocalProviderAdapterKind;
  capabilitySurface: LocalProviderAdapterCapabilitySurface;
  executionStatus: LocalProviderAdapterExecutionStatus;
  trustStatus: LocalProviderAdapterTrustStatus;
  boundaryStatuses: readonly LocalProviderAdapterBoundaryStatus[];
}>;

export type LocalProviderAdapterDeclaration = Readonly<{
  adapterKind: LocalProviderAdapterKind;
  declarationId: string;
  status: LocalProviderAdapterValidationStatus;
  contract: LocalProviderAdapterContract;
}>;

export type LocalProviderAdapterConfigurationCandidate = Readonly<{
  adapterKind?: string;
  declarationId?: string;
  fields?: readonly Readonly<{ key: string; value: string }>[];
}>;

export type LocalProviderAdapterValidation = Readonly<{
  status: LocalProviderAdapterValidationStatus;
  adapterKind: LocalProviderAdapterKind | null;
  declarationId: string | null;
  errorCodes: readonly LocalProviderAdapterValidationError[];
  reason: string;
}>;

export type LocalProviderAdapterRegistry = Readonly<{
  declarations: readonly LocalProviderAdapterDeclaration[];
  lastValidation: LocalProviderAdapterValidation;
}>;

export type LocalProviderAdapterRegistryProjection = Readonly<{
  registryStatus: LocalProviderAdapterValidationStatus;
  supportedAdapterKinds: readonly LocalProviderAdapterKind[];
  rejectedAdapterKinds: readonly LocalProviderAdapterKind[];
  declarations: readonly LocalProviderAdapterDeclaration[];
  lastValidation: LocalProviderAdapterValidation;
  capabilitySurface: LocalProviderAdapterCapabilitySurface;
  executionStatus: LocalProviderAdapterExecutionStatus;
  trustStatus: LocalProviderAdapterTrustStatus;
  boundaryStatuses: readonly LocalProviderAdapterBoundaryStatus[];
  note: string;
}>;

const supportedLocalProviderAdapterKinds: readonly LocalProviderAdapterKind[] =
  [
    "deterministic_fake_adapter",
    "local_model_adapter_contract",
    "unsupported_local_model",
    "unsupported_cloud_model",
    "unsupported_network_adapter",
    "unsupported_shell_adapter",
    "unsupported_filesystem_adapter",
    "unknown",
  ];

export function deterministicFakeAdapterDeclarationCandidate(): LocalProviderAdapterConfigurationCandidate {
  return {
    adapterKind: "deterministic_fake_adapter",
    declarationId: "local-adapter-declaration-deterministic-fake",
    fields: [],
  };
}

export function localModelAdapterContractDeclarationCandidate(): LocalProviderAdapterConfigurationCandidate {
  return {
    adapterKind: "local_model_adapter_contract",
    declarationId: "local-adapter-declaration-local-model-contract",
    fields: [],
  };
}

export function localProviderAdapterCapabilitySurface(): LocalProviderAdapterCapabilitySurface {
  return {
    contractOnly: true,
    noExecution: true,
    noProviderTrust: true,
    noNetwork: true,
    noShell: true,
    noSecrets: true,
    noProductionPersistence: true,
    noReadinessEffect: true,
    noReleaseEffect: true,
    noDeploymentEffect: true,
    noPublicUseEffect: true,
    summary:
      "Adapter contract only; no model execution is available in Phase 153. No network, shell, secret, or production persistence capability is enabled.",
  };
}

export function localProviderAdapterContract(
  adapterKind: LocalProviderAdapterKind,
): LocalProviderAdapterContract {
  return {
    adapterKind,
    capabilitySurface: localProviderAdapterCapabilitySurface(),
    executionStatus: "execution_not_available_in_phase_153",
    trustStatus: "no_provider_trust",
    boundaryStatuses: [
      "contract_only",
      "no_execution",
      "no_provider_trust",
      "no_network",
      "no_shell",
      "no_secrets",
      "no_production_persistence",
      "no_readiness_effect",
      "no_release_effect",
      "no_deployment_effect",
      "no_public_use_effect",
    ],
  };
}

export function initialLocalProviderAdapterRegistry(): LocalProviderAdapterRegistry {
  return {
    declarations: [],
    lastValidation: {
      status: "registry_projected",
      adapterKind: null,
      declarationId: null,
      errorCodes: [],
      reason:
        "initial local provider adapter registry projected; no adapter declarations execute in Phase 153",
    },
  };
}

export function projectLocalProviderAdapterRegistry(
  registry: LocalProviderAdapterRegistry,
): LocalProviderAdapterRegistryProjection {
  return {
    registryStatus: "registry_projected",
    supportedAdapterKinds: [
      "deterministic_fake_adapter",
      "local_model_adapter_contract",
    ],
    rejectedAdapterKinds: [
      "unsupported_local_model",
      "unsupported_cloud_model",
      "unsupported_network_adapter",
      "unsupported_shell_adapter",
      "unsupported_filesystem_adapter",
      "unknown",
    ],
    declarations: registry.declarations,
    lastValidation: registry.lastValidation,
    capabilitySurface: localProviderAdapterCapabilitySurface(),
    executionStatus: "execution_not_available_in_phase_153",
    trustStatus: "no_provider_trust",
    boundaryStatuses: localProviderAdapterContract("deterministic_fake_adapter")
      .boundaryStatuses,
    note: "Adapter contract only; no model execution is available in Phase 153. Accepted adapter declarations are non-executing. Adapter declaration does not grant provider trust. No network, shell, secret, or production persistence capability is enabled.",
  };
}

function forbiddenProviderAdapterDeclarationField(
  key: string,
  value: string,
): LocalProviderAdapterValidationError | null {
  const loweredKey = key.toLowerCase();
  const combined = `${loweredKey}=${value.toLowerCase()}`;
  if (loweredKey === "label" || loweredKey === "description") return null;
  if (
    ["endpoint", "url", "host", "port", "http", "network"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "endpoint_field_rejected";
  if (
    ["command", "args", "shell", "process"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "command_field_rejected";
  if (["executable", "binary"].some((needle) => combined.includes(needle)))
    return "executable_path_rejected";
  if (["path", "file", "directory"].some((needle) => combined.includes(needle)))
    return "path_field_rejected";
  if (
    ["secret", "token", "api_key", "apikey", "credential"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "secret_field_rejected";
  if (
    ["provider_execution", "execution_requested", "execution_flag"].some(
      (needle) => combined.includes(needle),
    )
  )
    return "execution_flag_rejected";
  if (
    ["trust_granted", "provider_trust", "trust_claimed"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "provider_trust_flag_rejected";
  if (["readiness", "ready"].some((needle) => combined.includes(needle)))
    return "readiness_claim_rejected";
  if (combined.includes("release")) return "release_claim_rejected";
  if (["deployment", "deploy"].some((needle) => combined.includes(needle)))
    return "deployment_claim_rejected";
  if (["public_use", "public-use"].some((needle) => combined.includes(needle)))
    return "public_use_claim_rejected";
  if (combined.includes("signing")) return "signing_claim_rejected";
  if (["publishing", "publish"].some((needle) => combined.includes(needle)))
    return "publishing_claim_rejected";
  return "unknown_field_rejected";
}

export function validateLocalProviderAdapterDeclaration(
  candidate: LocalProviderAdapterConfigurationCandidate,
): LocalProviderAdapterValidation {
  const errors = new Set<LocalProviderAdapterValidationError>();
  let adapterKind: LocalProviderAdapterKind | null = null;
  const rawKind = candidate.adapterKind;
  if (rawKind === undefined || rawKind.trim().length === 0) {
    errors.add("missing_adapter_kind");
  } else if (rawKind.trim() !== rawKind) {
    errors.add("malformed_adapter_kind");
  } else if (
    supportedLocalProviderAdapterKinds.includes(
      rawKind as LocalProviderAdapterKind,
    )
  ) {
    adapterKind = rawKind as LocalProviderAdapterKind;
    if (
      adapterKind === "unsupported_cloud_model" ||
      adapterKind === "unsupported_network_adapter"
    )
      errors.add("cloud_or_network_adapter_rejected");
    else if (adapterKind === "unsupported_shell_adapter")
      errors.add("shell_adapter_rejected");
    else if (adapterKind === "unsupported_filesystem_adapter")
      errors.add("filesystem_adapter_rejected");
    else if (
      adapterKind !== "deterministic_fake_adapter" &&
      adapterKind !== "local_model_adapter_contract"
    )
      errors.add("unsupported_adapter");
  } else {
    errors.add("unsupported_adapter");
  }

  for (const field of candidate.fields ?? []) {
    const error = forbiddenProviderAdapterDeclarationField(
      field.key,
      field.value,
    );
    if (error) errors.add(error);
  }

  const errorCodes = [...errors].sort();
  if (
    errorCodes.length === 0 &&
    (adapterKind === "deterministic_fake_adapter" ||
      adapterKind === "local_model_adapter_contract")
  ) {
    return {
      status: "adapter_declared_non_executing",
      adapterKind,
      declarationId: candidate.declarationId ?? null,
      errorCodes: [],
      reason:
        "adapter declaration accepted as contract-only local projection; no provider execution, trust, network, shell, secrets, or production persistence is enabled",
    };
  }
  return {
    status: errors.has("unsupported_adapter")
      ? "unsupported_adapter"
      : "invalid_adapter_declaration",
    adapterKind,
    declarationId: candidate.declarationId ?? null,
    errorCodes,
    reason:
      "adapter declaration rejected fail-closed; prior registry projection remains unchanged and no provider execution occurs",
  };
}

export function applyLocalProviderAdapterDeclaration(
  state: LocalOperatorShellState,
  candidate: LocalProviderAdapterConfigurationCandidate,
): LocalOperatorIntentResult {
  const validation = validateLocalProviderAdapterDeclaration(candidate);
  if (
    validation.status !== "adapter_declared_non_executing" ||
    !validation.adapterKind
  )
    return { status: "rejected", reason: validation.reason, state };
  const declaration: LocalProviderAdapterDeclaration = {
    adapterKind: validation.adapterKind,
    declarationId:
      validation.declarationId ??
      `local-adapter-declaration-${validation.adapterKind}`,
    status: validation.status,
    contract: localProviderAdapterContract(validation.adapterKind),
  };
  return {
    status: "accepted",
    reason: "local_provider_adapter_declaration_accepted",
    state: attachLocalSessionEvidenceExport({
      ...state,
      localProviderAdapterRegistry: {
        declarations: [declaration],
        lastValidation: validation,
      },
    }),
  };
}

export type LocalProviderAdapterDryRunStatus =
  | "not_run"
  | "dry_run_executed"
  | "dry_run_rejected"
  | "adapter_required"
  | "unsupported_adapter"
  | "invalid_dry_run_request";
export type LocalProviderAdapterDryRunError =
  | "no_adapter_declared"
  | "adapter_not_accepted"
  | "unsupported_adapter_kind"
  | "local_model_adapter_not_executable_in_phase_154"
  | "cloud_adapter_rejected"
  | "network_adapter_rejected"
  | "shell_adapter_rejected"
  | "filesystem_adapter_rejected"
  | "executable_path_rejected"
  | "endpoint_field_rejected"
  | "command_field_rejected"
  | "path_field_rejected"
  | "secret_field_rejected"
  | "execution_claim_rejected"
  | "trust_claim_rejected"
  | "readiness_claim_rejected"
  | "release_claim_rejected"
  | "deployment_claim_rejected"
  | "public_use_claim_rejected"
  | "signing_claim_rejected"
  | "publishing_claim_rejected"
  | "action_claim_rejected"
  | "persistence_claim_rejected";
export type LocalProviderAdapterDryRunBoundaryStatus =
  | "controlled_dry_run_only"
  | "deterministic_fake_adapter_only"
  | "no_real_model_execution"
  | "no_process_spawn"
  | "no_network"
  | "no_shell"
  | "no_secrets"
  | "no_provider_trust"
  | "no_candidate_materialization"
  | "no_action_execution"
  | "no_production_persistence"
  | "no_readiness_effect"
  | "no_release_effect"
  | "no_deployment_effect"
  | "no_public_use_effect";
export type LocalProviderAdapterDryRunTrustStatus = "untrusted_descriptive";
export type LocalProviderAdapterDryRunEffectStatus =
  | "no_provider_trust"
  | "no_candidate_materialization"
  | "no_action_execution"
  | "no_readiness_effect"
  | "no_release_effect"
  | "no_deployment_effect"
  | "no_public_use_effect";

export type LocalProviderAdapterDryRunCapabilitySurface = Readonly<{
  controlledDryRunOnly: true;
  deterministicFakeAdapterOnly: true;
  noRealModelExecution: true;
  noProcessSpawn: true;
  noNetwork: true;
  noShell: true;
  noSecrets: true;
  noProviderTrust: true;
  noCandidateMaterialization: true;
  noActionExecution: true;
  noProductionPersistence: true;
  noReadinessEffect: true;
  noReleaseEffect: true;
  noDeploymentEffect: true;
  noPublicUseEffect: true;
  summary: string;
}>;

export type LocalProviderAdapterDryRunRequest = Readonly<{
  inputSummary: string;
  fields?: readonly Readonly<{ key: string; value: string }>[];
}>;

export type LocalProviderAdapterDryRunResult = Readonly<{
  resultId: string;
  adapterKind: LocalProviderAdapterKind;
  adapterDeclarationId: string;
  outputSummary: string;
  outputTrustStatus: LocalProviderAdapterDryRunTrustStatus;
  boundaryStatuses: readonly LocalProviderAdapterDryRunBoundaryStatus[];
  effectStatuses: readonly LocalProviderAdapterDryRunEffectStatus[];
}>;

export type LocalProviderAdapterDryRunProjection = Readonly<{
  status: LocalProviderAdapterDryRunStatus;
  adapterKind: LocalProviderAdapterKind | null;
  adapterDeclarationId: string | null;
  result: LocalProviderAdapterDryRunResult | null;
  errorCodes: readonly LocalProviderAdapterDryRunError[];
  boundaryStatuses: readonly LocalProviderAdapterDryRunBoundaryStatus[];
  outputTrustStatus: LocalProviderAdapterDryRunTrustStatus;
  effectStatuses: readonly LocalProviderAdapterDryRunEffectStatus[];
  capabilitySurface: LocalProviderAdapterDryRunCapabilitySurface;
  registryDeclarationCount: number;
  reason: string;
}>;

export function deterministicFakeAdapterDryRunRequest(): LocalProviderAdapterDryRunRequest {
  return {
    inputSummary: "phase 154 deterministic fake adapter dry-run input",
    fields: [],
  };
}

export function localProviderAdapterDryRunBoundaryStatuses(): readonly LocalProviderAdapterDryRunBoundaryStatus[] {
  return [
    "controlled_dry_run_only",
    "deterministic_fake_adapter_only",
    "no_real_model_execution",
    "no_process_spawn",
    "no_network",
    "no_shell",
    "no_secrets",
    "no_provider_trust",
    "no_candidate_materialization",
    "no_action_execution",
    "no_production_persistence",
    "no_readiness_effect",
    "no_release_effect",
    "no_deployment_effect",
    "no_public_use_effect",
  ];
}

export function localProviderAdapterDryRunEffectStatuses(): readonly LocalProviderAdapterDryRunEffectStatus[] {
  return [
    "no_provider_trust",
    "no_candidate_materialization",
    "no_action_execution",
    "no_readiness_effect",
    "no_release_effect",
    "no_deployment_effect",
    "no_public_use_effect",
  ];
}

export function localProviderAdapterDryRunCapabilitySurface(): LocalProviderAdapterDryRunCapabilitySurface {
  return {
    controlledDryRunOnly: true,
    deterministicFakeAdapterOnly: true,
    noRealModelExecution: true,
    noProcessSpawn: true,
    noNetwork: true,
    noShell: true,
    noSecrets: true,
    noProviderTrust: true,
    noCandidateMaterialization: true,
    noActionExecution: true,
    noProductionPersistence: true,
    noReadinessEffect: true,
    noReleaseEffect: true,
    noDeploymentEffect: true,
    noPublicUseEffect: true,
    summary:
      "Controlled adapter dry run only; only deterministic_fake_adapter can execute in Phase 154. No real model execution, process, network, shell, secrets, provider trust, candidate materialization, action, readiness, release, deployment, public-use, or production persistence effect is enabled.",
  };
}

export function initialLocalProviderAdapterDryRunProjection(): LocalProviderAdapterDryRunProjection {
  return {
    status: "not_run",
    adapterKind: null,
    adapterDeclarationId: null,
    result: null,
    errorCodes: [],
    boundaryStatuses: localProviderAdapterDryRunBoundaryStatuses(),
    outputTrustStatus: "untrusted_descriptive",
    effectStatuses: localProviderAdapterDryRunEffectStatuses(),
    capabilitySurface: localProviderAdapterDryRunCapabilitySurface(),
    registryDeclarationCount: 0,
    reason:
      "adapter dry-run not_run; deterministic_fake_adapter declaration is required before Phase 154 dry run",
  };
}

function rejectLocalProviderAdapterDryRun(
  status: LocalProviderAdapterDryRunStatus,
  adapterKind: LocalProviderAdapterKind | null,
  adapterDeclarationId: string | null,
  registryDeclarationCount: number,
  errorCodes: readonly LocalProviderAdapterDryRunError[],
): LocalProviderAdapterDryRunProjection {
  return {
    status,
    adapterKind,
    adapterDeclarationId,
    result: null,
    errorCodes,
    boundaryStatuses: localProviderAdapterDryRunBoundaryStatuses(),
    outputTrustStatus: "untrusted_descriptive",
    effectStatuses: localProviderAdapterDryRunEffectStatuses(),
    capabilitySurface: localProviderAdapterDryRunCapabilitySurface(),
    registryDeclarationCount,
    reason:
      "adapter dry-run rejected fail-closed; prior accepted shell state is preserved and no provider trust, candidate, action, readiness, release, deployment, public-use, process, network, shell, secret, environment, or persistence effect occurs",
  };
}

function forbiddenProviderAdapterDryRunField(
  key: string,
  value: string,
): LocalProviderAdapterDryRunError | null {
  const loweredKey = key.toLowerCase();
  const combined = `${loweredKey}=${value.toLowerCase()}`;
  const safeInput =
    loweredKey === "inputsummary" ||
    loweredKey === "input_summary" ||
    loweredKey === "label" ||
    loweredKey === "description";
  const hasForbidden = [
    "endpoint",
    "url",
    "host",
    "port",
    "http",
    "network",
    "command",
    "args",
    "process",
    "shell",
    "executable",
    "binary",
    "path",
    "file",
    "directory",
    "secret",
    "token",
    "api_key",
    "apikey",
    "credential",
    "execution",
    "trust",
    "readiness",
    "ready",
    "release",
    "deployment",
    "deploy",
    "public_use",
    "public-use",
    "signing",
    "publishing",
    "publish",
    "action",
    "persistence",
  ].some((needle) => combined.includes(needle));
  if (safeInput && !hasForbidden) return null;
  if (
    ["endpoint", "url", "host", "port", "http", "network"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "endpoint_field_rejected";
  if (
    ["command", "args", "process", "shell"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "command_field_rejected";
  if (["executable", "binary"].some((needle) => combined.includes(needle)))
    return "executable_path_rejected";
  if (["path", "file", "directory"].some((needle) => combined.includes(needle)))
    return "path_field_rejected";
  if (
    ["secret", "token", "api_key", "apikey", "credential"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "secret_field_rejected";
  if (combined.includes("execution")) return "execution_claim_rejected";
  if (combined.includes("trust")) return "trust_claim_rejected";
  if (["readiness", "ready"].some((needle) => combined.includes(needle)))
    return "readiness_claim_rejected";
  if (combined.includes("release")) return "release_claim_rejected";
  if (["deployment", "deploy"].some((needle) => combined.includes(needle)))
    return "deployment_claim_rejected";
  if (["public_use", "public-use"].some((needle) => combined.includes(needle)))
    return "public_use_claim_rejected";
  if (combined.includes("signing")) return "signing_claim_rejected";
  if (["publishing", "publish"].some((needle) => combined.includes(needle)))
    return "publishing_claim_rejected";
  if (combined.includes("action")) return "action_claim_rejected";
  if (combined.includes("persistence")) return "persistence_claim_rejected";
  return null;
}

export function validateLocalProviderAdapterDryRunRequest(
  registry: LocalProviderAdapterRegistry,
  request: LocalProviderAdapterDryRunRequest,
): LocalProviderAdapterDeclaration | LocalProviderAdapterDryRunProjection {
  const declaration = registry.declarations[registry.declarations.length - 1];
  if (!declaration)
    return rejectLocalProviderAdapterDryRun(
      "adapter_required",
      null,
      null,
      registry.declarations.length,
      ["no_adapter_declared"],
    );
  const errors = new Set<LocalProviderAdapterDryRunError>();
  if (declaration.status !== "adapter_declared_non_executing")
    errors.add("adapter_not_accepted");
  if (declaration.adapterKind === "local_model_adapter_contract")
    errors.add("local_model_adapter_not_executable_in_phase_154");
  else if (declaration.adapterKind === "unsupported_cloud_model")
    errors.add("cloud_adapter_rejected");
  else if (declaration.adapterKind === "unsupported_network_adapter")
    errors.add("network_adapter_rejected");
  else if (declaration.adapterKind === "unsupported_shell_adapter")
    errors.add("shell_adapter_rejected");
  else if (declaration.adapterKind === "unsupported_filesystem_adapter")
    errors.add("filesystem_adapter_rejected");
  else if (declaration.adapterKind !== "deterministic_fake_adapter")
    errors.add("unsupported_adapter_kind");
  for (const field of request.fields ?? []) {
    const error = forbiddenProviderAdapterDryRunField(field.key, field.value);
    if (error) errors.add(error);
  }
  const inputError = forbiddenProviderAdapterDryRunField(
    "input_summary",
    request.inputSummary,
  );
  if (inputError) errors.add(inputError);
  const errorCodes = [...errors].sort();
  if (errorCodes.length === 0) return declaration;
  const unsupportedErrors: readonly LocalProviderAdapterDryRunError[] = [
    "local_model_adapter_not_executable_in_phase_154",
    "cloud_adapter_rejected",
    "network_adapter_rejected",
    "shell_adapter_rejected",
    "filesystem_adapter_rejected",
    "unsupported_adapter_kind",
  ];
  return rejectLocalProviderAdapterDryRun(
    errorCodes.some((error) => unsupportedErrors.includes(error))
      ? "unsupported_adapter"
      : "invalid_dry_run_request",
    declaration.adapterKind,
    declaration.declarationId,
    registry.declarations.length,
    errorCodes,
  );
}

function deterministicAdapterDryRunChecksum(input: string): bigint {
  let checksum = 154n;
  for (const char of input)
    checksum = BigInt.asUintN(64, checksum * 31n + BigInt(char.charCodeAt(0)));
  return checksum;
}

export function executeDeterministicFakeAdapterDryRun(
  declaration: LocalProviderAdapterDeclaration,
  request: LocalProviderAdapterDryRunRequest,
): LocalProviderAdapterDryRunResult {
  const checksum = deterministicAdapterDryRunChecksum(
    `${declaration.declarationId}|${declaration.adapterKind}|${request.inputSummary}`,
  );
  const checksumText = checksum.toString(16).padStart(16, "0");
  return {
    resultId: `local-provider-adapter-dry-run-${checksumText}`,
    adapterKind: declaration.adapterKind,
    adapterDeclarationId: declaration.declarationId,
    outputSummary: `deterministic_fake_adapter dry-run descriptive output for input_bytes=${new TextEncoder().encode(request.inputSummary).length} checksum=${checksumText}`,
    outputTrustStatus: "untrusted_descriptive",
    boundaryStatuses: localProviderAdapterDryRunBoundaryStatuses(),
    effectStatuses: localProviderAdapterDryRunEffectStatuses(),
  };
}

export function applyLocalProviderAdapterDryRun(
  state: LocalOperatorShellState,
  request: LocalProviderAdapterDryRunRequest,
): LocalOperatorIntentResult {
  const validation = validateLocalProviderAdapterDryRunRequest(
    state.localProviderAdapterRegistry,
    request,
  );
  if ("result" in validation)
    return {
      status: "rejected",
      reason: "local_provider_adapter_dry_run_rejected",
      state: { ...state, localProviderAdapterDryRun: validation },
    };
  const result = executeDeterministicFakeAdapterDryRun(validation, request);
  return {
    status: "accepted",
    reason: "local_provider_adapter_dry_run_executed",
    state: attachLocalSessionEvidenceExport({
      ...state,
      localProviderAdapterDryRun: {
        status: "dry_run_executed",
        adapterKind: result.adapterKind,
        adapterDeclarationId: result.adapterDeclarationId,
        result,
        errorCodes: [],
        boundaryStatuses: localProviderAdapterDryRunBoundaryStatuses(),
        outputTrustStatus: "untrusted_descriptive",
        effectStatuses: localProviderAdapterDryRunEffectStatuses(),
        capabilitySurface: localProviderAdapterDryRunCapabilitySurface(),
        registryDeclarationCount:
          state.localProviderAdapterRegistry.declarations.length,
        reason:
          "controlled adapter dry run executed through deterministic_fake_adapter only; output remains untrusted_descriptive and no provider trust, candidate, action, readiness, release, deployment, public-use, process, network, shell, secret, environment, or persistence effect occurs",
      },
    }),
  };
}

export type AllowlistedLocalProviderKind =
  | "allowlisted_local_deterministic_provider"
  | "unsupported_local_provider"
  | "unsupported_cloud_provider"
  | "unsupported_network_provider"
  | "unsupported_shell_provider";
export type ConstrainedLocalProviderInvocationStatus =
  | "not_invoked"
  | "invocation_executed"
  | "invocation_rejected"
  | "allowlisted_provider_required"
  | "unsupported_provider"
  | "invalid_invocation_request";
export type ConstrainedLocalProviderInvocationError =
  | "no_adapter_declared"
  | "adapter_not_accepted"
  | "provider_not_allowlisted"
  | "arbitrary_command_rejected"
  | "shell_field_rejected"
  | "process_field_rejected"
  | "args_field_rejected"
  | "endpoint_field_rejected"
  | "network_field_rejected"
  | "secret_field_rejected"
  | "path_field_rejected"
  | "trust_claim_rejected"
  | "provider_output_approval_claim_rejected"
  | "readiness_claim_rejected"
  | "release_claim_rejected"
  | "deployment_claim_rejected"
  | "public_use_claim_rejected"
  | "candidate_materialization_claim_rejected"
  | "action_claim_rejected"
  | "persistence_claim_rejected";
export type ConstrainedLocalProviderInvocationBoundaryStatus =
  | "constrained_local_invocation_only"
  | "allowlisted_provider_only"
  | "no_arbitrary_command"
  | "no_shell"
  | "no_network"
  | "no_cloud"
  | "no_secrets"
  | "no_provider_trust"
  | "no_candidate_materialization"
  | "no_action_execution"
  | "no_production_persistence"
  | "no_readiness_effect"
  | "no_release_effect"
  | "no_deployment_effect"
  | "no_public_use_effect";
export type ConstrainedLocalProviderInvocationTrustStatus =
  "untrusted_descriptive";
export type ConstrainedLocalProviderInvocationEffectStatus =
  | "no_provider_trust"
  | "no_candidate_materialization"
  | "no_action_execution"
  | "no_readiness_effect"
  | "no_release_effect"
  | "no_deployment_effect"
  | "no_public_use_effect";

export type ConstrainedLocalProviderInvocationCapabilitySurface = Readonly<{
  constrainedLocalInvocationOnly: true;
  allowlistedProviderOnly: true;
  allowlistedProviderKind: "allowlisted_local_deterministic_provider";
  noArbitraryCommand: true;
  noShell: true;
  noNetwork: true;
  noCloud: true;
  noSecrets: true;
  noProviderTrust: true;
  noCandidateMaterialization: true;
  noActionExecution: true;
  noProductionPersistence: true;
  noReadinessEffect: true;
  noReleaseEffect: true;
  noDeploymentEffect: true;
  noPublicUseEffect: true;
  summary: string;
}>;

export type ConstrainedLocalProviderInvocationRequest = Readonly<{
  providerKind: AllowlistedLocalProviderKind;
  inputSummary: string;
  fields?: readonly Readonly<{ key: string; value: string }>[];
}>;

export type ConstrainedLocalProviderInvocationResult = Readonly<{
  resultId: string;
  providerKind: AllowlistedLocalProviderKind;
  adapterKind: LocalProviderAdapterKind;
  adapterDeclarationId: string;
  outputSummary: string;
  outputTrustStatus: ConstrainedLocalProviderInvocationTrustStatus;
  boundaryStatuses: readonly ConstrainedLocalProviderInvocationBoundaryStatus[];
  effectStatuses: readonly ConstrainedLocalProviderInvocationEffectStatus[];
}>;

export type ConstrainedLocalProviderInvocationProjection = Readonly<{
  status: ConstrainedLocalProviderInvocationStatus;
  providerKind: AllowlistedLocalProviderKind | null;
  adapterKind: LocalProviderAdapterKind | null;
  adapterDeclarationId: string | null;
  result: ConstrainedLocalProviderInvocationResult | null;
  errorCodes: readonly ConstrainedLocalProviderInvocationError[];
  boundaryStatuses: readonly ConstrainedLocalProviderInvocationBoundaryStatus[];
  outputTrustStatus: ConstrainedLocalProviderInvocationTrustStatus;
  effectStatuses: readonly ConstrainedLocalProviderInvocationEffectStatus[];
  capabilitySurface: ConstrainedLocalProviderInvocationCapabilitySurface;
  registryDeclarationCount: number;
  reason: string;
}>;

export type LocalProviderOutputPipelineSourceKind =
  | "deterministic_stub_provider_execution"
  | "constrained_local_provider_invocation";
export type LocalProviderOutputPipelineStage =
  | "invocation_output_projected"
  | "provider_execution_result_projected"
  | "provider_output_validation_required"
  | "provider_output_validation_projected"
  | "provider_output_review_required"
  | "provider_output_review_projected"
  | "staged_conversion_required"
  | "staged_proposal_projected"
  | "staged_validation_required"
  | "staged_proposal_validation_projected"
  | "candidate_review_required"
  | "candidate_review_projected"
  | "operator_decision_required"
  | "operator_decision_projected";
export type LocalProviderOutputPipelineStageStatus =
  | "not_started"
  | "available"
  | "blocked"
  | "completed"
  | "rejected"
  | "not_applicable";
export type LocalProviderOutputPipelineValidationStatus =
  | "not_started"
  | "valid"
  | "blocked"
  | "rejected";
export type LocalProviderOutputPipelineError =
  | "no_invocation_output"
  | "invocation_output_rejected"
  | "invocation_output_not_untrusted_descriptive"
  | "invocation_result_id_mismatch"
  | "invocation_output_summary_mismatch"
  | "provider_output_validation_missing"
  | "provider_output_validation_rejected"
  | "provider_output_review_missing"
  | "staged_proposal_missing"
  | "staged_proposal_validation_missing"
  | "staged_proposal_validation_rejected"
  | "candidate_review_missing"
  | "operator_decision_missing"
  | "pipeline_skip_attempt_rejected"
  | "trust_claim_rejected"
  | "approval_claim_rejected"
  | "readiness_claim_rejected"
  | "release_claim_rejected"
  | "deployment_claim_rejected"
  | "public_use_claim_rejected"
  | "candidate_creation_claim_rejected"
  | "candidate_materialization_claim_rejected"
  | "action_claim_rejected"
  | "persistence_claim_rejected";
export type LocalProviderOutputPipelineBoundaryStatus =
  | "pipeline_integration_only"
  | "untrusted_descriptive_source"
  | "validation_required"
  | "review_required"
  | "staged_conversion_required"
  | "staged_validation_required"
  | "candidate_review_required"
  | "operator_decision_required"
  | "no_candidate_materialization"
  | "no_provider_trust"
  | "no_action_execution"
  | "no_readiness_effect"
  | "no_release_effect"
  | "no_deployment_effect"
  | "no_public_use_effect";
export type LocalProviderOutputPipelineEffectStatus =
  | "no_provider_execution"
  | "no_candidate_creation"
  | "no_candidate_materialization"
  | "no_decision_ledger_append"
  | "no_replay_mutation"
  | "no_export_promotion"
  | "no_provider_trust"
  | "no_file_write"
  | "no_network_socket"
  | "no_process_spawn"
  | "no_secret_read"
  | "no_action_execution";
export type LocalProviderOutputPipelineStageProjection = Readonly<{
  stage: LocalProviderOutputPipelineStage;
  status: LocalProviderOutputPipelineStageStatus;
  reason: LocalProviderOutputPipelineError | null;
}>;
export type InvocationProviderOutputBridgeProjection = Readonly<{
  sourceKind: LocalProviderOutputPipelineSourceKind;
  invocationResultId: string;
  providerExecutionResultId: string;
  outputSummary: string;
  outputTrustStatus: LocalProviderOutputTrustStatus;
  outputMaterializationStatus: LocalProviderOutputMaterializationStatus;
  outputPromotionStatus: LocalProviderOutputPromotionStatus;
  descriptiveOnly: boolean;
  notCandidateMaterial: boolean;
  notPromoted: boolean;
}>;
export type LocalProviderOutputPipelineProjection = Readonly<{
  status: LocalProviderOutputPipelineValidationStatus;
  sourceKind: LocalProviderOutputPipelineSourceKind | null;
  sourceInvocationResultId: string | null;
  providerExecutionResultId: string | null;
  currentStage: LocalProviderOutputPipelineStage | null;
  nextRequiredStage: LocalProviderOutputPipelineStage | null;
  stages: readonly LocalProviderOutputPipelineStageProjection[];
  providerOutputValidationStatus: LocalProviderOutputValidationStatus;
  providerOutputReviewStatus: LocalProviderOutputReviewabilityStatus;
  stagedProposalStatus: StagedCandidateConversionProposalStatus;
  stagedProposalValidationStatus: StagedCandidateConversionValidationStatus;
  candidateReviewStatus: string;
  operatorDecisionStatus: OperatorCandidateDecisionStatus;
  boundaryStatuses: readonly LocalProviderOutputPipelineBoundaryStatus[];
  effectStatuses: readonly LocalProviderOutputPipelineEffectStatus[];
  errors: readonly LocalProviderOutputPipelineError[];
  bridge: InvocationProviderOutputBridgeProjection | null;
  note: string;
}>;

export function localProviderOutputPipelineStageOrder(): readonly LocalProviderOutputPipelineStage[] {
  return [
    "invocation_output_projected",
    "provider_execution_result_projected",
    "provider_output_validation_required",
    "provider_output_validation_projected",
    "provider_output_review_required",
    "provider_output_review_projected",
    "staged_conversion_required",
    "staged_proposal_projected",
    "staged_validation_required",
    "staged_proposal_validation_projected",
    "candidate_review_required",
    "candidate_review_projected",
    "operator_decision_required",
    "operator_decision_projected",
  ];
}

export function localProviderOutputPipelineBoundaryStatuses(): readonly LocalProviderOutputPipelineBoundaryStatus[] {
  return [
    "pipeline_integration_only",
    "untrusted_descriptive_source",
    "validation_required",
    "review_required",
    "staged_conversion_required",
    "staged_validation_required",
    "candidate_review_required",
    "operator_decision_required",
    "no_candidate_materialization",
    "no_provider_trust",
    "no_action_execution",
    "no_readiness_effect",
    "no_release_effect",
    "no_deployment_effect",
    "no_public_use_effect",
  ];
}

export function localProviderOutputPipelineEffectStatuses(): readonly LocalProviderOutputPipelineEffectStatus[] {
  return [
    "no_provider_execution",
    "no_candidate_creation",
    "no_candidate_materialization",
    "no_decision_ledger_append",
    "no_replay_mutation",
    "no_export_promotion",
    "no_provider_trust",
    "no_file_write",
    "no_network_socket",
    "no_process_spawn",
    "no_secret_read",
    "no_action_execution",
  ];
}

function pipelineStage(
  stage: LocalProviderOutputPipelineStage,
  status: LocalProviderOutputPipelineStageStatus,
  reason: LocalProviderOutputPipelineError | null,
): LocalProviderOutputPipelineStageProjection {
  return { stage, status, reason };
}

export function initialLocalProviderOutputPipelineProjection(): LocalProviderOutputPipelineProjection {
  return {
    status: "not_started",
    sourceKind: null,
    sourceInvocationResultId: null,
    providerExecutionResultId: null,
    currentStage: null,
    nextRequiredStage: "invocation_output_projected",
    stages: localProviderOutputPipelineStageOrder().map((stage) =>
      pipelineStage(stage, "not_started", null),
    ),
    providerOutputValidationStatus: "not_validated",
    providerOutputReviewStatus: "not_reviewable",
    stagedProposalStatus: "no_proposal",
    stagedProposalValidationStatus: "not_validated",
    candidateReviewStatus: "not_available",
    operatorDecisionStatus: "no_operator_decision",
    boundaryStatuses: localProviderOutputPipelineBoundaryStatuses(),
    effectStatuses: localProviderOutputPipelineEffectStatuses(),
    errors: [],
    bridge: null,
    note: "Provider output pipeline integration has not started; invocation output remains untrusted and descriptive.",
  };
}

export function projectInvocationOutputIntoProviderPipeline(
  state: LocalOperatorShellState,
):
  | InvocationProviderOutputBridgeProjection
  | readonly LocalProviderOutputPipelineError[] {
  const invocation = state.constrainedLocalProviderInvocation;
  if (invocation.status === "not_invoked") return ["no_invocation_output"];
  if (invocation.status !== "invocation_executed")
    return ["invocation_output_rejected"];
  const result = invocation.result;
  if (!result) return ["no_invocation_output"];
  const errors = new Set<LocalProviderOutputPipelineError>();
  if (
    result.outputTrustStatus !== "untrusted_descriptive" ||
    invocation.outputTrustStatus !== "untrusted_descriptive"
  )
    errors.add("invocation_output_not_untrusted_descriptive");
  const prefix = "constrained-local-provider-invocation-";
  const checksum = result.resultId.startsWith(prefix)
    ? result.resultId.slice(prefix.length)
    : "";
  if (checksum.length === 0) errors.add("invocation_result_id_mismatch");
  if (
    !result.outputSummary.startsWith(
      "allowlisted_local_deterministic_provider descriptive output for input_bytes=",
    ) ||
    !result.outputSummary.includes(" checksum=") ||
    !result.outputSummary.endsWith(checksum)
  )
    errors.add("invocation_output_summary_mismatch");
  const lower = result.outputSummary.toLowerCase();
  if (
    [
      "trust",
      "trusted",
      "provider_output_approval",
      "approved output",
      "approval granted",
    ].some((needle) => lower.includes(needle))
  ) {
    errors.add("trust_claim_rejected");
    errors.add("approval_claim_rejected");
  }
  if (["readiness", "ready for"].some((needle) => lower.includes(needle)))
    errors.add("readiness_claim_rejected");
  if (lower.includes("release")) errors.add("release_claim_rejected");
  if (["deployment", "deploy"].some((needle) => lower.includes(needle)))
    errors.add("deployment_claim_rejected");
  if (
    ["public_use", "public-use", "public use"].some((needle) =>
      lower.includes(needle),
    )
  )
    errors.add("public_use_claim_rejected");
  if (
    ["candidate creation", "candidate_output"].some((needle) =>
      lower.includes(needle),
    )
  )
    errors.add("candidate_creation_claim_rejected");
  if (["candidate", "materialization"].some((needle) => lower.includes(needle)))
    errors.add("candidate_materialization_claim_rejected");
  if (lower.includes("action")) errors.add("action_claim_rejected");
  if (["persist", "persistence"].some((needle) => lower.includes(needle)))
    errors.add("persistence_claim_rejected");
  if (errors.size > 0) return [...errors].sort();
  return {
    sourceKind: "constrained_local_provider_invocation",
    invocationResultId: result.resultId,
    providerExecutionResultId: result.resultId,
    outputSummary: result.outputSummary,
    outputTrustStatus: "untrusted_descriptive",
    outputMaterializationStatus: "projected_as_untrusted_output",
    outputPromotionStatus: "not_promoted",
    descriptiveOnly: true,
    notCandidateMaterial: true,
    notPromoted: true,
  };
}

function isPipelineBridge(
  value:
    | InvocationProviderOutputBridgeProjection
    | readonly LocalProviderOutputPipelineError[],
): value is InvocationProviderOutputBridgeProjection {
  return !Array.isArray(value);
}

export function providerExecutionProjectionFromInvocationBridge(
  state: LocalOperatorShellState,
  bridge: InvocationProviderOutputBridgeProjection,
): LocalProviderExecutionProjection {
  return withProviderExecutionProjectionValidation({
    status: "executed",
    projectionStatus: "execution_projected",
    configuredProviderKind: "allowlisted_local_deterministic_provider",
    sandboxStatus: "sandboxed_deterministic_no_external_effects",
    result: {
      resultId: bridge.providerExecutionResultId,
      providerKind: "deterministic_stub",
      sandboxStatus: "sandboxed_deterministic_no_external_effects",
      outputSummary: bridge.outputSummary,
      outputTrustStatus: "untrusted/descriptive",
      outputMaterializationStatus: "projected_as_untrusted_output",
      outputPromotionStatus: "not_promoted",
      promotionAvailabilityStatus: "promotion_not_available_in_phase_142",
      descriptiveOnly: true,
      providerOutputTrusted: false,
      candidateOutputPromoted: false,
      decisionAppended: false,
      replayRepaired: false,
      releaseOrDeploymentEvidenceCreated: false,
    },
    outputTrustStatus: "untrusted_descriptive",
    outputMaterializationStatus: "not_candidate_material",
    outputPromotionStatus: "not_promoted",
    promotionAvailabilityStatus: "promotion_not_available_in_phase_142",
    linkage: {
      ...localProviderExecutionLinkage(state, bridge.providerExecutionResultId),
      executionResultId: bridge.providerExecutionResultId,
      sourceBoundary: "constrained_local_provider_invocation_pipeline_bridge",
    },
    absenceMarkers: localProviderExecutionResultAbsenceMarkers(),
    projectionValidation: {
      status: "invalid",
      errorCodes: [],
      reason: "projection validation pending",
    },
    validationStatus: "executed",
    validationErrorCodes: [],
    validationReason:
      "constrained invocation output projected into provider execution/result surface without changing trust boundary",
    capabilitySurface: localProviderExecutionCapabilitySurface(),
    note: "Constrained invocation output is projected as untrusted_descriptive provider output only; it is not candidate material, not promoted, and not approved.",
  });
}

export function deriveLocalProviderOutputPipelineProjection(
  state: LocalOperatorShellState,
): LocalProviderOutputPipelineProjection {
  if (state.constrainedLocalProviderInvocation.status === "not_invoked")
    return initialLocalProviderOutputPipelineProjection();
  const bridgeResult = projectInvocationOutputIntoProviderPipeline(state);
  if (!isPipelineBridge(bridgeResult)) {
    const reason = bridgeResult[0] ?? "no_invocation_output";
    return {
      ...initialLocalProviderOutputPipelineProjection(),
      status: "rejected",
      sourceKind: "constrained_local_provider_invocation",
      sourceInvocationResultId:
        state.constrainedLocalProviderInvocation.result?.resultId ?? null,
      currentStage: "invocation_output_projected",
      nextRequiredStage: "invocation_output_projected",
      stages: localProviderOutputPipelineStageOrder().map((stage) =>
        pipelineStage(
          stage,
          stage === "invocation_output_projected" ? "rejected" : "blocked",
          reason,
        ),
      ),
      errors: bridgeResult,
      note: "Provider output pipeline integration rejected fail-closed; no candidate output is created.",
    };
  }
  const stages: LocalProviderOutputPipelineStageProjection[] = [
    pipelineStage("invocation_output_projected", "completed", null),
    pipelineStage("provider_execution_result_projected", "completed", null),
    pipelineStage("provider_output_validation_required", "completed", null),
  ];
  const errors: LocalProviderOutputPipelineError[] = [];
  let nextRequiredStage: LocalProviderOutputPipelineStage | null = null;
  const blockRest = (
    startIndex: number,
    reason: LocalProviderOutputPipelineError,
  ) => {
    for (const stage of localProviderOutputPipelineStageOrder().slice(
      startIndex,
    ))
      stages.push(pipelineStage(stage, "blocked", reason));
  };
  const validationRejected = [
    "rejected",
    "validation_not_applicable",
    "invalid_validation_input",
  ].includes(state.providerOutputValidation.status);
  if (validationRejected) {
    errors.push("provider_output_validation_rejected");
    stages.push(
      pipelineStage(
        "provider_output_validation_projected",
        "rejected",
        "provider_output_validation_rejected",
      ),
    );
    blockRest(4, "provider_output_validation_rejected");
    nextRequiredStage = "provider_output_validation_projected";
  } else if (state.providerOutputValidation.status !== "reviewable_untrusted") {
    errors.push("provider_output_validation_missing");
    stages.push(
      pipelineStage(
        "provider_output_validation_projected",
        "available",
        "provider_output_validation_missing",
      ),
    );
    blockRest(4, "provider_output_validation_missing");
    nextRequiredStage = "provider_output_validation_projected";
  } else {
    stages.push(
      pipelineStage("provider_output_validation_projected", "completed", null),
    );
    stages.push(
      pipelineStage("provider_output_review_required", "completed", null),
    );
    stages.push(
      pipelineStage("provider_output_review_projected", "completed", null),
    );
    stages.push(pipelineStage("staged_conversion_required", "completed", null));
    if (
      state.stagedCandidateConversionProposal.status !==
      "staged_proposal_created"
    ) {
      errors.push("staged_proposal_missing");
      stages.push(
        pipelineStage(
          "staged_proposal_projected",
          "available",
          "staged_proposal_missing",
        ),
      );
      blockRest(8, "staged_proposal_missing");
      nextRequiredStage = "staged_proposal_projected";
    } else {
      stages.push(
        pipelineStage("staged_proposal_projected", "completed", null),
      );
      stages.push(
        pipelineStage("staged_validation_required", "completed", null),
      );
      if (
        state.stagedCandidateConversionValidation.status ===
        "staged_proposal_shape_valid"
      ) {
        stages.push(
          pipelineStage(
            "staged_proposal_validation_projected",
            "completed",
            null,
          ),
        );
        stages.push(
          pipelineStage("candidate_review_required", "completed", null),
        );
        stages.push(
          pipelineStage("candidate_review_projected", "completed", null),
        );
        stages.push(
          pipelineStage("operator_decision_required", "completed", null),
        );
        const decisionDone = [
          "approved_validated_staged_proposal",
          "rejected_validated_staged_proposal",
        ].includes(state.operatorCandidateDecision.status);
        if (decisionDone) {
          stages.push(
            pipelineStage("operator_decision_projected", "completed", null),
          );
        } else {
          errors.push("operator_decision_missing");
          stages.push(
            pipelineStage(
              "operator_decision_projected",
              "available",
              "operator_decision_missing",
            ),
          );
          nextRequiredStage = "operator_decision_projected";
        }
      } else {
        const reason =
          state.stagedCandidateConversionValidation.status ===
            "rejected_staged_proposal" ||
          state.stagedCandidateConversionValidation.status ===
            "invalid_validation_input"
            ? "staged_proposal_validation_rejected"
            : "staged_proposal_validation_missing";
        errors.push(reason);
        stages.push(
          pipelineStage(
            "staged_proposal_validation_projected",
            reason === "staged_proposal_validation_rejected"
              ? "rejected"
              : "available",
            reason,
          ),
        );
        blockRest(10, reason);
        nextRequiredStage = "staged_proposal_validation_projected";
      }
    }
  }
  const currentStage =
    [...stages].reverse().find((stage) => stage.status === "completed")
      ?.stage ?? null;
  return {
    status: errors.some((error) => error.endsWith("rejected"))
      ? "rejected"
      : errors.length > 0
        ? "blocked"
        : "valid",
    sourceKind: "constrained_local_provider_invocation",
    sourceInvocationResultId: bridgeResult.invocationResultId,
    providerExecutionResultId: bridgeResult.providerExecutionResultId,
    currentStage,
    nextRequiredStage,
    stages,
    providerOutputValidationStatus: state.providerOutputValidation.status,
    providerOutputReviewStatus:
      state.providerOutputValidation.reviewabilityStatus,
    stagedProposalStatus: state.stagedCandidateConversionProposal.status,
    stagedProposalValidationStatus:
      state.stagedCandidateConversionValidation.status,
    candidateReviewStatus:
      state.stagedCandidateConversionValidation.status ===
      "staged_proposal_shape_valid"
        ? "display_only"
        : "required",
    operatorDecisionStatus: state.operatorCandidateDecision.status,
    boundaryStatuses: localProviderOutputPipelineBoundaryStatuses(),
    effectStatuses: localProviderOutputPipelineEffectStatuses(),
    errors,
    bridge: bridgeResult,
    note: "Invocation output remains untrusted and descriptive. Pipeline integration does not create candidate output. Validation, review, staging, staged validation, candidate review, and operator decision boundaries cannot be skipped. Candidate materialization remains a later bounded step. Provider trust, readiness, release, deployment, and public-use approval are not granted.",
  };
}

export function validateProviderOutputPipelineStageOrder(
  projection: LocalProviderOutputPipelineProjection,
): LocalProviderOutputPipelineError | null {
  const actual = projection.stages.map((stage) => stage.stage);
  if (actual.join("|") !== localProviderOutputPipelineStageOrder().join("|"))
    return "pipeline_skip_attempt_rejected";
  let seenIncomplete = false;
  for (const stage of projection.stages) {
    if (seenIncomplete && stage.status === "completed")
      return "pipeline_skip_attempt_rejected";
    if (stage.status !== "completed") seenIncomplete = true;
  }
  return null;
}

export function allowlistedLocalProviderInvocationRequest(): ConstrainedLocalProviderInvocationRequest {
  return {
    providerKind: "allowlisted_local_deterministic_provider",
    inputSummary: "phase 156 constrained local provider invocation input",
    fields: [],
  };
}

export function constrainedLocalProviderInvocationBoundaryStatuses(): readonly ConstrainedLocalProviderInvocationBoundaryStatus[] {
  return [
    "constrained_local_invocation_only",
    "allowlisted_provider_only",
    "no_arbitrary_command",
    "no_shell",
    "no_network",
    "no_cloud",
    "no_secrets",
    "no_provider_trust",
    "no_candidate_materialization",
    "no_action_execution",
    "no_production_persistence",
    "no_readiness_effect",
    "no_release_effect",
    "no_deployment_effect",
    "no_public_use_effect",
  ];
}

export function constrainedLocalProviderInvocationEffectStatuses(): readonly ConstrainedLocalProviderInvocationEffectStatus[] {
  return [
    "no_provider_trust",
    "no_candidate_materialization",
    "no_action_execution",
    "no_readiness_effect",
    "no_release_effect",
    "no_deployment_effect",
    "no_public_use_effect",
  ];
}

export function constrainedLocalProviderInvocationCapabilitySurface(): ConstrainedLocalProviderInvocationCapabilitySurface {
  return {
    constrainedLocalInvocationOnly: true,
    allowlistedProviderOnly: true,
    allowlistedProviderKind: "allowlisted_local_deterministic_provider",
    noArbitraryCommand: true,
    noShell: true,
    noNetwork: true,
    noCloud: true,
    noSecrets: true,
    noProviderTrust: true,
    noCandidateMaterialization: true,
    noActionExecution: true,
    noProductionPersistence: true,
    noReadinessEffect: true,
    noReleaseEffect: true,
    noDeploymentEffect: true,
    noPublicUseEffect: true,
    summary:
      "Constrained local provider invocation only; only allowlisted_local_deterministic_provider is enabled in Phase 156. No arbitrary command, shell, network, cloud, secrets, provider trust, candidate materialization, action, readiness, release, deployment, public-use, or production persistence effect is enabled.",
  };
}

export function initialConstrainedLocalProviderInvocationProjection(): ConstrainedLocalProviderInvocationProjection {
  return {
    status: "not_invoked",
    providerKind: null,
    adapterKind: null,
    adapterDeclarationId: null,
    result: null,
    errorCodes: [],
    boundaryStatuses: constrainedLocalProviderInvocationBoundaryStatuses(),
    outputTrustStatus: "untrusted_descriptive",
    effectStatuses: constrainedLocalProviderInvocationEffectStatuses(),
    capabilitySurface: constrainedLocalProviderInvocationCapabilitySurface(),
    registryDeclarationCount: 0,
    reason:
      "constrained local provider invocation not_invoked; accepted deterministic_fake_adapter declaration and allowlisted_local_deterministic_provider are required for Phase 156 invocation",
  };
}

function rejectConstrainedLocalProviderInvocation(
  status: ConstrainedLocalProviderInvocationStatus,
  providerKind: AllowlistedLocalProviderKind | null,
  adapterKind: LocalProviderAdapterKind | null,
  adapterDeclarationId: string | null,
  registryDeclarationCount: number,
  errorCodes: readonly ConstrainedLocalProviderInvocationError[],
): ConstrainedLocalProviderInvocationProjection {
  return {
    status,
    providerKind,
    adapterKind,
    adapterDeclarationId,
    result: null,
    errorCodes,
    boundaryStatuses: constrainedLocalProviderInvocationBoundaryStatuses(),
    outputTrustStatus: "untrusted_descriptive",
    effectStatuses: constrainedLocalProviderInvocationEffectStatuses(),
    capabilitySurface: constrainedLocalProviderInvocationCapabilitySurface(),
    registryDeclarationCount,
    reason:
      "constrained local provider invocation rejected fail-closed; prior accepted invocation projection is preserved when present and no provider trust, candidate, action, readiness, release, deployment, public-use, command, shell, network, cloud, secret, environment, or persistence effect occurs",
  };
}

function forbiddenConstrainedLocalProviderInvocationField(
  key: string,
  value: string,
): ConstrainedLocalProviderInvocationError | null {
  const loweredKey = key.toLowerCase();
  const combined = `${loweredKey}=${value.toLowerCase()}`;
  const safeInput =
    loweredKey === "inputsummary" ||
    loweredKey === "input_summary" ||
    loweredKey === "label" ||
    loweredKey === "description";
  const forbidden = [
    "endpoint",
    "url",
    "host",
    "port",
    "http",
    "network",
    "cloud",
    "command",
    "shell",
    "process",
    "args",
    "path",
    "file",
    "directory",
    "secret",
    "token",
    "api_key",
    "apikey",
    "credential",
    "trust",
    "approved_output",
    "provider_output_approval",
    "readiness",
    "ready",
    "release",
    "deployment",
    "deploy",
    "public_use",
    "public-use",
    "candidate",
    "materialization",
    "action",
    "persistence",
  ].some((needle) => combined.includes(needle));
  if (safeInput && !forbidden) return null;
  if (combined.includes("command")) return "arbitrary_command_rejected";
  if (combined.includes("shell")) return "shell_field_rejected";
  if (combined.includes("process")) return "process_field_rejected";
  if (combined.includes("args")) return "args_field_rejected";
  if (
    ["endpoint", "url", "host", "port", "http"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "endpoint_field_rejected";
  if (["network", "cloud"].some((needle) => combined.includes(needle)))
    return "network_field_rejected";
  if (
    ["secret", "token", "api_key", "apikey", "credential"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "secret_field_rejected";
  if (["path", "file", "directory"].some((needle) => combined.includes(needle)))
    return "path_field_rejected";
  if (
    combined.includes("approved_output") ||
    combined.includes("provider_output_approval")
  )
    return "provider_output_approval_claim_rejected";
  if (combined.includes("trust")) return "trust_claim_rejected";
  if (["readiness", "ready"].some((needle) => combined.includes(needle)))
    return "readiness_claim_rejected";
  if (combined.includes("release")) return "release_claim_rejected";
  if (["deployment", "deploy"].some((needle) => combined.includes(needle)))
    return "deployment_claim_rejected";
  if (["public_use", "public-use"].some((needle) => combined.includes(needle)))
    return "public_use_claim_rejected";
  if (
    ["candidate", "materialization"].some((needle) => combined.includes(needle))
  )
    return "candidate_materialization_claim_rejected";
  if (combined.includes("action")) return "action_claim_rejected";
  if (combined.includes("persistence")) return "persistence_claim_rejected";
  return null;
}

export function validateConstrainedLocalProviderInvocationRequest(
  registry: LocalProviderAdapterRegistry,
  request: ConstrainedLocalProviderInvocationRequest,
):
  | LocalProviderAdapterDeclaration
  | ConstrainedLocalProviderInvocationProjection {
  const errors = new Set<ConstrainedLocalProviderInvocationError>();
  if (
    request.providerKind === "unsupported_cloud_provider" ||
    request.providerKind === "unsupported_network_provider"
  ) {
    errors.add("provider_not_allowlisted");
    errors.add("network_field_rejected");
  } else if (request.providerKind === "unsupported_shell_provider") {
    errors.add("provider_not_allowlisted");
    errors.add("shell_field_rejected");
  } else if (
    request.providerKind !== "allowlisted_local_deterministic_provider"
  ) {
    errors.add("provider_not_allowlisted");
  }
  const declaration = registry.declarations[registry.declarations.length - 1];
  if (!declaration) {
    errors.add("no_adapter_declared");
    return rejectConstrainedLocalProviderInvocation(
      "allowlisted_provider_required",
      request.providerKind,
      null,
      null,
      registry.declarations.length,
      [...errors].sort(),
    );
  }
  if (
    declaration.status !== "adapter_declared_non_executing" ||
    declaration.adapterKind !== "deterministic_fake_adapter"
  )
    errors.add("adapter_not_accepted");
  for (const field of request.fields ?? []) {
    const error = forbiddenConstrainedLocalProviderInvocationField(
      field.key,
      field.value,
    );
    if (error) errors.add(error);
  }
  const inputError = forbiddenConstrainedLocalProviderInvocationField(
    "input_summary",
    request.inputSummary,
  );
  if (inputError) errors.add(inputError);
  const errorCodes = [...errors].sort();
  if (errorCodes.length === 0) return declaration;
  const unsupported = errorCodes.some((error) =>
    [
      "provider_not_allowlisted",
      "network_field_rejected",
      "shell_field_rejected",
      "adapter_not_accepted",
    ].includes(error),
  );
  return rejectConstrainedLocalProviderInvocation(
    unsupported ? "unsupported_provider" : "invalid_invocation_request",
    request.providerKind,
    declaration.adapterKind,
    declaration.declarationId,
    registry.declarations.length,
    errorCodes,
  );
}

function deterministicConstrainedLocalProviderInvocationChecksum(
  input: string,
): bigint {
  let checksum = 156n;
  for (const char of input)
    checksum = BigInt.asUintN(64, checksum * 33n + BigInt(char.charCodeAt(0)));
  return checksum;
}

export function executeAllowlistedLocalDeterministicProviderInvocation(
  declaration: LocalProviderAdapterDeclaration,
  request: ConstrainedLocalProviderInvocationRequest,
): ConstrainedLocalProviderInvocationResult {
  const checksum = deterministicConstrainedLocalProviderInvocationChecksum(
    `${declaration.declarationId}|${declaration.adapterKind}|${request.providerKind}|${request.inputSummary}`,
  );
  const checksumText = checksum.toString(16).padStart(16, "0");
  return {
    resultId: `constrained-local-provider-invocation-${checksumText}`,
    providerKind: request.providerKind,
    adapterKind: declaration.adapterKind,
    adapterDeclarationId: declaration.declarationId,
    outputSummary: `allowlisted_local_deterministic_provider descriptive output for input_bytes=${new TextEncoder().encode(request.inputSummary).length} checksum=${checksumText}`,
    outputTrustStatus: "untrusted_descriptive",
    boundaryStatuses: constrainedLocalProviderInvocationBoundaryStatuses(),
    effectStatuses: constrainedLocalProviderInvocationEffectStatuses(),
  };
}

export function applyConstrainedLocalProviderInvocation(
  state: LocalOperatorShellState,
  request: ConstrainedLocalProviderInvocationRequest,
): LocalOperatorIntentResult {
  const validation = validateConstrainedLocalProviderInvocationRequest(
    state.localProviderAdapterRegistry,
    request,
  );
  if ("result" in validation) {
    const rejectedState = {
      ...state,
      constrainedLocalProviderInvocation: validation,
      localProviderOutputPipeline: deriveLocalProviderOutputPipelineProjection({
        ...state,
        constrainedLocalProviderInvocation: validation,
      }),
    };
    return {
      status: "rejected",
      reason: "constrained_local_provider_invocation_rejected",
      state: {
        ...rejectedState,
        completeLocalOperatorWorkflow:
          deriveCompleteLocalOperatorWorkflowProjection(rejectedState),
      },
    };
  }
  const result = executeAllowlistedLocalDeterministicProviderInvocation(
    validation,
    request,
  );
  const constrainedLocalProviderInvocation: ConstrainedLocalProviderInvocationProjection =
    {
      status: "invocation_executed",
      providerKind: result.providerKind,
      adapterKind: result.adapterKind,
      adapterDeclarationId: result.adapterDeclarationId,
      result,
      errorCodes: [],
      boundaryStatuses: constrainedLocalProviderInvocationBoundaryStatuses(),
      outputTrustStatus: "untrusted_descriptive",
      effectStatuses: constrainedLocalProviderInvocationEffectStatuses(),
      capabilitySurface: constrainedLocalProviderInvocationCapabilitySurface(),
      registryDeclarationCount:
        state.localProviderAdapterRegistry.declarations.length,
      reason:
        "constrained local provider invocation executed through exactly one allowlisted local provider path; output remains untrusted_descriptive and no provider trust, candidate, action, readiness, release, deployment, public-use, command, shell, network, cloud, secret, environment, or persistence effect occurs",
    };
  const bridgeState = { ...state, constrainedLocalProviderInvocation };
  const bridge = projectInvocationOutputIntoProviderPipeline(bridgeState);
  const providerExecution = isPipelineBridge(bridge)
    ? providerExecutionProjectionFromInvocationBridge(bridgeState, bridge)
    : state.providerExecution;
  const providerOutputValidation =
    validateLocalProviderOutput(providerExecution);
  const next = {
    ...state,
    constrainedLocalProviderInvocation,
    providerExecution,
    providerOutputValidation,
  };
  return {
    status: "accepted",
    reason: "constrained_local_provider_invocation_executed",
    state: attachLocalSessionEvidenceExport(next),
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
export type LocalProviderOutputTrustStatus =
  | "untrusted_descriptive"
  | "trusted_output";
export type LocalProviderExecutionResultOutputTrustStatus =
  | "untrusted/descriptive"
  | "trusted_output";
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
  sourceBoundary: string;
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

export function deterministicStubProviderExecutionRequest(
  inputSummary = "local deterministic provider input",
): LocalProviderExecutionRequest {
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
    summary:
      "sandboxed deterministic provider execution supports deterministic_stub only; no cloud, network, shell, filesystem, secrets, trust, readiness, release, deployment, signing, publishing, or public-use capability",
  };
}

export function localProviderOutputValidationNoEffects(): LocalProviderOutputValidationEffect {
  return "none";
}

function localProviderOutputCandidateBoundaryStatuses(): readonly LocalProviderOutputCandidateBoundaryStatus[] {
  return [
    "not_candidate_material",
    "candidate_conversion_not_performed",
    "candidate_conversion_requires_future_phase",
  ];
}

export function initialLocalProviderOutputValidationProjection(): LocalProviderOutputValidationProjection {
  return {
    status: "not_validated",
    reviewabilityStatus: "not_reviewable",
    candidateBoundaryStatus: "not_candidate_material",
    candidateBoundaryStatuses: localProviderOutputCandidateBoundaryStatuses(),
    reasons: [
      "no_provider_execution_result",
      "missing_execution_result",
      "candidate_conversion_not_available_in_phase_143",
    ],
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
    note: "Provider output validation has not run; provider output is not candidate material and cannot be approved in Phase 143.",
  };
}

export function localProviderOutputValidationReasons(
  execution: LocalProviderExecutionProjection,
): readonly LocalProviderOutputValidationReason[] {
  const reasons = new Set<LocalProviderOutputValidationReason>();
  if (execution.projectionStatus !== "execution_projected")
    reasons.add("provider_execution_not_projected");
  const result = execution.result;
  if (!result) {
    reasons.add("no_provider_execution_result");
    reasons.add("missing_execution_result");
    reasons.add("candidate_conversion_not_available_in_phase_143");
    return [...reasons].sort();
  }
  if (result.providerKind !== "deterministic_stub")
    reasons.add("unsupported_provider_kind");
  const output = result.outputSummary.trim();
  if (output.length === 0) reasons.add("empty_output");
  if (result.outputSummary.length > 1024) reasons.add("output_too_large");
  if (
    !(
      result.outputSummary.startsWith(
        "deterministic_stub descriptive output for input_bytes=",
      ) ||
      result.outputSummary.startsWith(
        "allowlisted_local_deterministic_provider descriptive output for input_bytes=",
      )
    ) ||
    !result.outputSummary.includes(" checksum=") ||
    result.sandboxStatus !== "sandboxed_deterministic_no_external_effects" ||
    result.outputTrustStatus !== "untrusted/descriptive" ||
    result.outputMaterializationStatus !== "projected_as_untrusted_output" ||
    result.outputPromotionStatus !== "not_promoted" ||
    result.promotionAvailabilityStatus !==
      "promotion_not_available_in_phase_142" ||
    !result.descriptiveOnly ||
    result.providerOutputTrusted ||
    result.candidateOutputPromoted ||
    result.decisionAppended ||
    result.replayRepaired ||
    result.releaseOrDeploymentEvidenceCreated
  )
    reasons.add("malformed_output");
  const lower = result.outputSummary.toLowerCase();
  if (
    ["secret", "token", "api_key", "apikey", "credential"].some((needle) =>
      lower.includes(needle),
    )
  )
    reasons.add("contains_forbidden_secret_marker");
  if (
    ["command", "shell", "process", "execute ", "run "].some((needle) =>
      lower.includes(needle),
    )
  )
    reasons.add("contains_execution_instruction");
  if (
    ["http://", "https://", "network", "socket", "fetch("].some((needle) =>
      lower.includes(needle),
    )
  )
    reasons.add("contains_network_instruction");
  if (
    ["filesystem", "file ", "write ", "path", "directory", "fs::write"].some(
      (needle) => lower.includes(needle),
    )
  )
    reasons.add("contains_filesystem_instruction");
  if (
    [
      "readiness",
      "release",
      "deployment",
      "public-use",
      "public use",
      "production ready",
    ].some((needle) => lower.includes(needle))
  )
    reasons.add("contains_readiness_or_release_claim");
  if (
    [
      "trusted_output",
      "trusted output",
      "approved_output",
      "approved output",
      "approval granted",
      "trust_granted",
    ].some((needle) => lower.includes(needle))
  )
    reasons.add("contains_trust_or_approval_claim");
  if (
    [
      "action_executed",
      "authorize action",
      "action authorization",
      "perform action",
    ].some((needle) => lower.includes(needle))
  )
    reasons.add("contains_action_instruction");
  if (reasons.size === 0) reasons.add("deterministic_stub_output_shape_valid");
  reasons.add("candidate_conversion_not_available_in_phase_143");
  return [...reasons].sort();
}

export function validateLocalProviderOutput(
  execution: LocalProviderExecutionProjection,
): LocalProviderOutputValidationProjection {
  const reasons = localProviderOutputValidationReasons(execution);
  const result = execution.result;
  const validReasons = reasons.every(
    (reason) =>
      reason === "deterministic_stub_output_shape_valid" ||
      reason === "candidate_conversion_not_available_in_phase_143",
  );
  const status: LocalProviderOutputValidationStatus = !result
    ? "not_validated"
    : execution.projectionStatus !== "execution_projected"
      ? "validation_not_applicable"
      : validReasons
        ? "reviewable_untrusted"
        : "rejected";
  const reviewabilityStatus: LocalProviderOutputReviewabilityStatus =
    status === "reviewable_untrusted"
      ? "reviewable_untrusted"
      : status === "rejected"
        ? "rejected_before_review"
        : "not_reviewable";
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
    note: "reviewable_untrusted is not candidate material and cannot be approved in Phase 143; provider output is not promoted.",
  };
}

export function projectLocalProviderOutputValidation(
  state: LocalOperatorShellState,
): LocalProviderOutputValidationProjection {
  return validateLocalProviderOutput(projectLocalProviderExecution(state));
}

export function validateLocalProviderOutputValidationProjection(
  projection: LocalProviderOutputValidationProjection,
): readonly LocalProviderOutputValidationError[] {
  const errors: LocalProviderOutputValidationError[] = [];
  if (projection.reasons.length === 0) errors.push("missing_validation_reason");
  if (
    projection.status === "reviewable_untrusted" &&
    projection.outputTrustStatus !== "untrusted_descriptive"
  )
    errors.push("invalid_reviewable_trust_status");
  if (
    projection.candidateBoundaryStatus !== "not_candidate_material" ||
    !projection.candidateBoundaryStatuses.includes("not_candidate_material") ||
    !projection.candidateBoundaryStatuses.includes(
      "candidate_conversion_not_performed",
    ) ||
    !projection.candidateBoundaryStatuses.includes(
      "candidate_conversion_requires_future_phase",
    )
  )
    errors.push("invalid_candidate_boundary_status");
  if (projection.outputPromotionStatus !== "not_promoted")
    errors.push("invalid_promotion_status");
  const effects = [
    projection.trustEffect,
    projection.candidateEffect,
    projection.decisionLedgerEffect,
    projection.replayEffect,
    projection.exportEffect,
    projection.actionEffect,
    projection.readinessEffect,
    projection.releaseEffect,
    projection.deploymentEffect,
  ];
  if (effects.some((effect) => effect !== "none"))
    errors.push("invalid_no_effect_boundary");
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
export type StagedCandidateConversionTrustStatus =
  | "untrusted_source"
  | "not_trusted"
  | "not_approved";
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
    "not_persistent",
  ];
}

function stagedCandidateConversionBoundaryStatuses(): readonly StagedCandidateConversionBoundaryStatus[] {
  return [
    "staging_only_not_candidate_material",
    "candidate_conversion_not_performed",
    "validation_required_in_future_phase",
    "approval_not_available_in_phase_146",
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
    note: "No staged candidate-conversion proposal exists; Phase 146 staging is proposal only and not candidate material.",
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
export type StagedCandidateConversionOperatorDecisionStatus =
  "not_available_in_phase_147";
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
    "operator_decision_not_available_in_phase_147",
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
    materializationStatuses: [
      "not_materialized",
      "materialization_not_available_in_phase_147",
      "materialization_requires_future_phase",
    ],
    futureReviewBoundaryStatus: "future_review_boundary_required",
    operatorDecisionStatus: "not_available_in_phase_147",
    trustStatuses: stagedCandidateConversionTrustStatuses(),
    boundaryStatuses: stagedCandidateConversionValidationBoundaryStatuses(),
    noEffectSummary: stagedCandidateConversionNoEffects(),
    note: "Validation checks staged proposal shape and source linkage only. Validated staged proposal is not candidate output. Candidate materialization was not performed in Phase 147. Future review boundary is required before any operator decision. Operator decision is not available in Phase 147. Provider output remains untrusted and not approved.",
  };
}

function proposalNoteClaimReasons(
  proposal: StagedCandidateConversionProposal,
): StagedCandidateConversionValidationReason[] {
  const lower = proposal.note.toLowerCase();
  const reasons: StagedCandidateConversionValidationReason[] = [];
  if (
    ["trust granted", "is trusted", "mark trusted"].some((needle) =>
      lower.includes(needle),
    )
  )
    reasons.push("contains_trust_claim");
  if (
    ["approval granted", "is approved", "mark approved"].some((needle) =>
      lower.includes(needle),
    )
  )
    reasons.push("contains_approval_claim");
  if (
    ["is safe", "safe output", "safe for"].some((needle) =>
      lower.includes(needle),
    )
  )
    reasons.push("contains_safety_claim");
  if (["readiness", "ready for"].some((needle) => lower.includes(needle)))
    reasons.push("contains_readiness_claim");
  if (
    ["release claim", "release evidence"].some((needle) =>
      lower.includes(needle),
    )
  )
    reasons.push("contains_release_claim");
  if (
    ["deployment claim", "deployment evidence"].some((needle) =>
      lower.includes(needle),
    )
  )
    reasons.push("contains_deployment_claim");
  if (["public use", "public-use"].some((needle) => lower.includes(needle)))
    reasons.push("contains_public_use_claim");
  if (
    ["action claim", "action effect"].some((needle) => lower.includes(needle))
  )
    reasons.push("contains_action_claim");
  if (
    ["persistence claim", "persisted"].some((needle) => lower.includes(needle))
  )
    reasons.push("contains_persistence_claim");
  if (
    ["execution claim", "executed proposal"].some((needle) =>
      lower.includes(needle),
    )
  )
    reasons.push("contains_execution_claim");
  if (
    ["candidate creation", "candidate output created"].some((needle) =>
      lower.includes(needle),
    )
  )
    reasons.push("contains_candidate_creation_claim");
  if (
    ["candidate materialization", "materialized candidate"].some((needle) =>
      lower.includes(needle),
    )
  )
    reasons.push("contains_candidate_materialization_claim");
  return reasons;
}

function sameSet<T>(left: readonly T[], right: readonly T[]): boolean {
  return (
    left.length === right.length && right.every((item) => left.includes(item))
  );
}

export function stagedCandidateConversionValidationReasons(
  state: LocalOperatorShellState,
  request: StagedCandidateConversionValidationRequest = {},
): readonly StagedCandidateConversionValidationReason[] {
  const reasons = new Set<StagedCandidateConversionValidationReason>();
  const proposal = state.stagedCandidateConversionProposal.proposal;
  if (!proposal) return ["no_staged_proposal"];
  if (request.proposalId && request.proposalId !== proposal.proposalId)
    reasons.add("deterministic_proposal_id_mismatch");
  const result = state.providerExecution.result;
  if (!result) return ["provider_execution_result_missing"];
  if (
    state.providerExecution.projectionValidation.status !== "valid" ||
    state.providerExecution.projectionStatus !== "execution_projected"
  )
    reasons.add("provider_execution_result_malformed");
  if (
    state.providerOutputValidation.reasons.length === 0 ||
    !state.providerOutputValidation.providerExecutionResultId
  ) {
    reasons.add("provider_output_validation_missing");
  } else if (
    validateLocalProviderOutputValidationProjection(
      state.providerOutputValidation,
    ).length > 0 ||
    JSON.stringify(projectLocalProviderOutputValidation(state)) !==
      JSON.stringify(state.providerOutputValidation)
  ) {
    reasons.add("provider_output_validation_inconsistent");
  }
  if (state.providerOutputValidation.status !== "reviewable_untrusted")
    reasons.add("source_not_reviewable_untrusted");
  const validationExecutionId =
    state.providerOutputValidation.providerExecutionResultId;
  if (!validationExecutionId)
    return [...reasons, "provider_output_validation_missing"];
  if (
    result.resultId !== validationExecutionId ||
    proposal.sourceExecutionResultId !== validationExecutionId
  )
    reasons.add("execution_result_id_mismatch");
  if (
    proposal.proposalId !==
    deterministicStagedCandidateConversionProposalId(
      validationExecutionId,
      state.providerOutputValidation,
    )
  )
    reasons.add("deterministic_proposal_id_mismatch");
  if (
    proposal.sourceValidationStatus !== state.providerOutputValidation.status ||
    proposal.sourceValidationStatus !== "reviewable_untrusted"
  )
    reasons.add("source_validation_status_mismatch");
  if (
    proposal.sourceReviewabilityStatus !==
      state.providerOutputValidation.reviewabilityStatus ||
    proposal.sourceReviewabilityStatus !== "reviewable_untrusted"
  )
    reasons.add("source_reviewability_status_mismatch");
  if (
    proposal.sourceCandidateBoundaryStatus !==
      state.providerOutputValidation.candidateBoundaryStatus ||
    proposal.sourceCandidateBoundaryStatus !== "not_candidate_material"
  )
    reasons.add("source_candidate_boundary_status_mismatch");
  if (
    !stagedCandidateConversionBoundaryStatuses().every((status) =>
      proposal.boundaryStatuses.includes(status),
    )
  )
    reasons.add("boundary_flag_missing");
  if (
    !sameSet(
      proposal.boundaryStatuses,
      stagedCandidateConversionBoundaryStatuses(),
    )
  )
    reasons.add("boundary_flag_drift");
  if (
    !proposal.boundaryStatuses.includes(
      "validation_required_in_future_phase",
    ) ||
    !proposal.boundaryStatuses.includes("approval_not_available_in_phase_146")
  )
    reasons.add("future_phase_marker_missing");
  if (
    !stagedCandidateConversionTrustStatuses().every((status) =>
      proposal.trustStatuses.includes(status),
    )
  )
    reasons.add("boundary_flag_missing");
  if (
    !sameSet(proposal.trustStatuses, stagedCandidateConversionTrustStatuses())
  )
    reasons.add("boundary_flag_drift");
  if (
    !stagedCandidateConversionNoEffects().every((status) =>
      proposal.effectStatuses.includes(status),
    )
  )
    reasons.add("no_effect_field_missing");
  if (!sameSet(proposal.effectStatuses, stagedCandidateConversionNoEffects()))
    reasons.add("no_effect_field_drift");
  if (
    proposal.sourceBoundary !== "provider_output_validation_phase_143" ||
    proposal.proposalBoundary !== "staged_candidate_conversion_phase_146" ||
    proposal.sourceEligibilityStatus !== "eligible_reviewable_untrusted"
  )
    reasons.add("boundary_flag_drift");
  for (const reason of proposalNoteClaimReasons(proposal)) reasons.add(reason);
  if (reasons.size === 0)
    return [
      "candidate_materialization_not_performed",
      "future_review_boundary_required",
      "operator_decision_not_available_in_phase_147",
      "source_linkage_validated",
      "staged_proposal_shape_valid",
    ].sort() as StagedCandidateConversionValidationReason[];
  return [...reasons].sort();
}

export function projectStagedCandidateConversionValidation(
  state: LocalOperatorShellState,
  request: StagedCandidateConversionValidationRequest = {},
): StagedCandidateConversionValidationProjection {
  const reasons = stagedCandidateConversionValidationReasons(state, request);
  const proposal = state.stagedCandidateConversionProposal.proposal;
  const validReasons: readonly StagedCandidateConversionValidationReason[] = [
    "candidate_materialization_not_performed",
    "future_review_boundary_required",
    "operator_decision_not_available_in_phase_147",
    "source_linkage_validated",
    "staged_proposal_shape_valid",
  ];
  const status: StagedCandidateConversionValidationStatus = reasons.includes(
    "no_staged_proposal",
  )
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
    sourceValidationStatus:
      proposal?.sourceValidationStatus ?? state.providerOutputValidation.status,
    sourceReviewabilityStatus:
      proposal?.sourceReviewabilityStatus ??
      state.providerOutputValidation.reviewabilityStatus,
    sourceCandidateBoundaryStatus:
      proposal?.sourceCandidateBoundaryStatus ??
      state.providerOutputValidation.candidateBoundaryStatus,
    deterministicLinkageStatus:
      status === "staged_proposal_shape_valid"
        ? "source_linkage_validated"
        : "not_validated",
    materializationStatuses: [
      "not_materialized",
      "materialization_not_available_in_phase_147",
      "materialization_requires_future_phase",
    ],
    futureReviewBoundaryStatus: "future_review_boundary_required",
    operatorDecisionStatus: "not_available_in_phase_147",
    trustStatuses: stagedCandidateConversionTrustStatuses(),
    boundaryStatuses: stagedCandidateConversionValidationBoundaryStatuses(),
    noEffectSummary: stagedCandidateConversionNoEffects(),
    note: "Validation checks staged proposal shape and source linkage only. Validated staged proposal is not candidate output. Candidate materialization was not performed in Phase 147. Future review boundary is required before any operator decision. Operator decision is not available in Phase 147. Provider output remains untrusted and not approved.",
  };
}

export function validateStagedCandidateConversionProposalForPhase147(
  state: LocalOperatorShellState,
  request: StagedCandidateConversionValidationRequest = {},
): LocalOperatorIntentResult {
  const next = {
    ...state,
    stagedCandidateConversionValidation:
      projectStagedCandidateConversionValidation(state, request),
  };
  return {
    status:
      next.stagedCandidateConversionValidation.status ===
      "staged_proposal_shape_valid"
        ? "accepted"
        : "rejected",
    reason:
      next.stagedCandidateConversionValidation.status ===
      "staged_proposal_shape_valid"
        ? "staged_candidate_conversion_validation_completed"
        : "staged_candidate_conversion_validation_rejected",
    state: attachLocalSessionEvidenceExport(next),
  };
}

export type OperatorCandidateDecisionKind =
  | "approve_validated_staged_proposal"
  | "reject_validated_staged_proposal";
export type OperatorCandidateDecisionStatus =
  | "no_operator_decision"
  | "approved_validated_staged_proposal"
  | "rejected_validated_staged_proposal"
  | "rejected_operator_decision_request"
  | "invalid_operator_decision_input";
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

export type LocalCandidateMaterializationStatus =
  | "not_materialized"
  | "local_candidate_materialized"
  | "materialization_rejected"
  | "materialization_precondition_missing"
  | "invalid_materialization_request";

export type LocalCandidateMaterializationError =
  | "provider_output_validation_missing"
  | "provider_output_validation_not_reviewable_untrusted"
  | "provider_output_review_missing"
  | "staged_proposal_missing"
  | "staged_proposal_validation_missing"
  | "staged_proposal_validation_not_valid"
  | "candidate_review_missing"
  | "operator_decision_missing"
  | "operator_decision_rejected"
  | "operator_decision_not_approved"
  | "provider_pipeline_incomplete"
  | "provider_pipeline_rejected"
  | "source_linkage_mismatch"
  | "invocation_result_id_mismatch"
  | "provider_execution_result_id_mismatch"
  | "staged_proposal_id_mismatch"
  | "operator_decision_id_mismatch"
  | "trust_claim_rejected"
  | "safety_claim_rejected"
  | "readiness_claim_rejected"
  | "release_claim_rejected"
  | "deployment_claim_rejected"
  | "public_use_claim_rejected"
  | "provider_output_approval_claim_rejected"
  | "action_claim_rejected"
  | "persistence_claim_rejected"
  | "execution_claim_rejected"
  | "signing_claim_rejected"
  | "publishing_claim_rejected";

export type LocalCandidateMaterializationBoundaryStatus =
  | "local_candidate_output_only"
  | "non_production_candidate"
  | "provider_output_remains_untrusted"
  | "no_provider_trust"
  | "no_production_approval"
  | "no_release_approval"
  | "no_deployment_approval"
  | "no_public_use_approval"
  | "no_action_execution"
  | "no_replay_repair"
  | "no_recovery_promotion";

export type LocalCandidateMaterializationEffectStatus =
  | LocalCandidateMaterializationBoundaryStatus
  | "no_file_write"
  | "no_network_socket"
  | "no_process_spawn"
  | "no_secret_read"
  | "no_provider_execution"
  | "no_provider_configuration_mutation"
  | "no_provider_execution_mutation"
  | "no_provider_output_validation_mutation"
  | "no_staged_proposal_mutation"
  | "no_operator_decision_mutation"
  | "no_export_promotion";

export type LocalCandidateMaterializationRequest = Readonly<{
  stagedProposalId: string;
  operatorDecisionId: string;
  providerExecutionResultId: string;
  sourceInvocationResultId: string;
  claimsTrust?: boolean;
  claimsSafety?: boolean;
  claimsReadiness?: boolean;
  claimsRelease?: boolean;
  claimsDeployment?: boolean;
  claimsPublicUse?: boolean;
  claimsProviderOutputApproval?: boolean;
  claimsAction?: boolean;
  claimsPersistence?: boolean;
  claimsExecution?: boolean;
  claimsSigning?: boolean;
  claimsPublishing?: boolean;
}>;

export type LocalCandidateOutputSourceLinkage = Readonly<{
  sourceInvocationResultId: string;
  providerExecutionResultId: string;
  providerOutputValidationStatus: LocalProviderOutputValidationStatus;
  providerOutputReviewStatus: LocalProviderOutputReviewabilityStatus;
  stagedProposalId: string;
  stagedProposalValidationStatus: StagedCandidateConversionValidationStatus;
  operatorDecisionId: string;
  operatorDecisionStatus: OperatorCandidateDecisionStatus;
}>;

export type LocalCandidateMaterializationCapabilitySurface = Readonly<{
  localOnly: true;
  nonProduction: true;
  materializesCandidateOutput: true;
  providerTrustEnabled: false;
  actionExecutionEnabled: false;
  productionApprovalEnabled: false;
  releaseApprovalEnabled: false;
  deploymentApprovalEnabled: false;
  publicUseApprovalEnabled: false;
  summary: string;
}>;

export type LocalCandidateOutputProjection = Readonly<{
  status: LocalCandidateMaterializationStatus;
  candidateId: string | null;
  contentSummary: string | null;
  outputClassification: "local_candidate_output_only";
  productionClassification: "non_production_candidate";
  sourceLinkage: LocalCandidateOutputSourceLinkage | null;
  providerOutputTrustCarryForward: "provider_output_remains_untrusted";
  boundaryStatuses: readonly LocalCandidateMaterializationBoundaryStatus[];
  effectStatuses: readonly LocalCandidateMaterializationEffectStatus[];
  error: LocalCandidateMaterializationError | null;
  capabilitySurface: LocalCandidateMaterializationCapabilitySurface;
  note: string;
}>;

export function localCandidateMaterializationBoundaryStatuses(): readonly LocalCandidateMaterializationBoundaryStatus[] {
  return [
    "local_candidate_output_only",
    "non_production_candidate",
    "provider_output_remains_untrusted",
    "no_provider_trust",
    "no_production_approval",
    "no_release_approval",
    "no_deployment_approval",
    "no_public_use_approval",
    "no_action_execution",
    "no_replay_repair",
    "no_recovery_promotion",
  ];
}

export function localCandidateMaterializationEffectStatuses(): readonly LocalCandidateMaterializationEffectStatus[] {
  return [
    ...localCandidateMaterializationBoundaryStatuses(),
    "no_file_write",
    "no_network_socket",
    "no_process_spawn",
    "no_secret_read",
    "no_provider_execution",
    "no_provider_configuration_mutation",
    "no_provider_execution_mutation",
    "no_provider_output_validation_mutation",
    "no_staged_proposal_mutation",
    "no_operator_decision_mutation",
    "no_export_promotion",
  ];
}

export function localCandidateMaterializationCapabilitySurface(): LocalCandidateMaterializationCapabilitySurface {
  return {
    localOnly: true,
    nonProduction: true,
    materializesCandidateOutput: true,
    providerTrustEnabled: false,
    actionExecutionEnabled: false,
    productionApprovalEnabled: false,
    releaseApprovalEnabled: false,
    deploymentApprovalEnabled: false,
    publicUseApprovalEnabled: false,
    summary:
      "Local candidate output materialization only; non-production; provider output remains untrusted; no provider trust, action, production, readiness, release, deployment, or public-use approval.",
  };
}

export function initialLocalCandidateOutputProjection(): LocalCandidateOutputProjection {
  return {
    status: "not_materialized",
    candidateId: null,
    contentSummary: null,
    outputClassification: "local_candidate_output_only",
    productionClassification: "non_production_candidate",
    sourceLinkage: null,
    providerOutputTrustCarryForward: "provider_output_remains_untrusted",
    boundaryStatuses: localCandidateMaterializationBoundaryStatuses(),
    effectStatuses: localCandidateMaterializationEffectStatuses(),
    error: null,
    capabilitySurface: localCandidateMaterializationCapabilitySurface(),
    note: "Local candidate output only. This candidate output is non-production. Provider output remains untrusted. Candidate materialization does not approve readiness, release, deployment, or public use. Candidate materialization does not authorize actions. Production approval remains unavailable.",
  };
}

export function localCandidateMaterializationRequestFromState(
  state: LocalOperatorShellState,
): LocalCandidateMaterializationRequest | null {
  const proposal = state.stagedCandidateConversionProposal.proposal;
  const decision = state.operatorCandidateDecision.record;
  if (!proposal || !decision) return null;
  return {
    stagedProposalId: proposal.proposalId,
    operatorDecisionId: decision.decisionId,
    providerExecutionResultId: proposal.sourceExecutionResultId,
    sourceInvocationResultId:
      state.localProviderOutputPipeline.sourceInvocationResultId ?? "",
  };
}

function stableLocalCandidateDigest(input: string): string {
  let hash = 0xcbf29ce484222325n;
  for (let index = 0; index < input.length; index += 1) {
    hash ^= BigInt(input.charCodeAt(index));
    hash = BigInt.asUintN(64, hash * 0x100000001b3n);
  }
  return hash.toString(16).padStart(16, "0");
}

export function deriveLocalCandidateOutputContent(
  state: LocalOperatorShellState,
  linkage: LocalCandidateOutputSourceLinkage,
): string {
  const proposal = state.stagedCandidateConversionProposal.proposal;
  const outputSummary = state.providerExecution.result?.outputSummary ?? "none";
  return `Local candidate output only. staged_proposal=${linkage.stagedProposalId} operator_decision=${linkage.operatorDecisionId} provider_execution=${linkage.providerExecutionResultId} invocation=${linkage.sourceInvocationResultId} validation=${linkage.providerOutputValidationStatus} review=${linkage.providerOutputReviewStatus} source_summary=${outputSummary} proposal_boundary=${proposal?.proposalBoundary ?? "none"}`;
}

export function validateLocalCandidateMaterializationRequest(
  state: LocalOperatorShellState,
  request: LocalCandidateMaterializationRequest,
): LocalCandidateOutputSourceLinkage | LocalCandidateMaterializationError {
  if (request.claimsTrust) return "trust_claim_rejected";
  if (request.claimsSafety) return "safety_claim_rejected";
  if (request.claimsReadiness) return "readiness_claim_rejected";
  if (request.claimsRelease) return "release_claim_rejected";
  if (request.claimsDeployment) return "deployment_claim_rejected";
  if (request.claimsPublicUse) return "public_use_claim_rejected";
  if (request.claimsProviderOutputApproval)
    return "provider_output_approval_claim_rejected";
  if (request.claimsAction) return "action_claim_rejected";
  if (request.claimsPersistence) return "persistence_claim_rejected";
  if (request.claimsExecution) return "execution_claim_rejected";
  if (request.claimsSigning) return "signing_claim_rejected";
  if (request.claimsPublishing) return "publishing_claim_rejected";
  if (state.localProviderOutputPipeline.status === "rejected")
    return "provider_pipeline_rejected";
  if (state.localProviderOutputPipeline.status !== "valid")
    return "provider_pipeline_incomplete";
  if (
    validateProviderOutputPipelineStageOrder(state.localProviderOutputPipeline)
  )
    return "provider_pipeline_incomplete";
  if (state.providerOutputValidation.status === "not_validated")
    return "provider_output_validation_missing";
  if (state.providerOutputValidation.status !== "reviewable_untrusted")
    return "provider_output_validation_not_reviewable_untrusted";
  if (
    state.providerOutputValidation.reviewabilityStatus !==
    "reviewable_untrusted"
  )
    return "provider_output_review_missing";
  const proposal = state.stagedCandidateConversionProposal.proposal;
  if (
    !proposal ||
    state.stagedCandidateConversionProposal.status !== "staged_proposal_created"
  )
    return "staged_proposal_missing";
  if (state.stagedCandidateConversionValidation.status === "not_validated")
    return "staged_proposal_validation_missing";
  if (
    state.stagedCandidateConversionValidation.status !==
    "staged_proposal_shape_valid"
  )
    return "staged_proposal_validation_not_valid";
  if (
    state.localProviderOutputPipeline.candidateReviewStatus !== "display_only"
  )
    return "candidate_review_missing";
  if (state.operatorCandidateDecision.status === "no_operator_decision")
    return "operator_decision_missing";
  if (
    state.operatorCandidateDecision.status ===
    "rejected_validated_staged_proposal"
  )
    return "operator_decision_rejected";
  if (
    state.operatorCandidateDecision.status !==
    "approved_validated_staged_proposal"
  )
    return "operator_decision_not_approved";
  const decision = state.operatorCandidateDecision.record;
  if (!decision) return "operator_decision_missing";
  if (
    request.stagedProposalId !== proposal.proposalId ||
    request.stagedProposalId !== decision.stagedProposalId ||
    request.stagedProposalId !==
      state.stagedCandidateConversionValidation.proposalId
  )
    return "staged_proposal_id_mismatch";
  if (request.operatorDecisionId !== decision.decisionId)
    return "operator_decision_id_mismatch";
  const providerResultId = state.providerExecution.result?.resultId;
  if (
    !providerResultId ||
    request.providerExecutionResultId !== providerResultId ||
    request.providerExecutionResultId !== proposal.sourceExecutionResultId ||
    request.providerExecutionResultId !== decision.providerExecutionResultId ||
    request.providerExecutionResultId !==
      state.providerOutputValidation.providerExecutionResultId ||
    request.providerExecutionResultId !==
      state.stagedCandidateConversionValidation.sourceExecutionResultId ||
    request.providerExecutionResultId !==
      state.localProviderOutputPipeline.providerExecutionResultId
  )
    return "provider_execution_result_id_mismatch";
  const invocationResultId =
    state.constrainedLocalProviderInvocation.result?.resultId;
  if (
    !invocationResultId ||
    request.sourceInvocationResultId !== invocationResultId ||
    request.sourceInvocationResultId !==
      state.localProviderOutputPipeline.sourceInvocationResultId
  )
    return "invocation_result_id_mismatch";
  const reprojected = projectStagedCandidateConversionValidation(state, {
    proposalId: proposal.proposalId,
  });
  if (
    JSON.stringify(reprojected) !==
    JSON.stringify(state.stagedCandidateConversionValidation)
  )
    return "source_linkage_mismatch";
  return {
    sourceInvocationResultId: request.sourceInvocationResultId,
    providerExecutionResultId: request.providerExecutionResultId,
    providerOutputValidationStatus: state.providerOutputValidation.status,
    providerOutputReviewStatus:
      state.providerOutputValidation.reviewabilityStatus,
    stagedProposalId: request.stagedProposalId,
    stagedProposalValidationStatus:
      state.stagedCandidateConversionValidation.status,
    operatorDecisionId: request.operatorDecisionId,
    operatorDecisionStatus: state.operatorCandidateDecision.status,
  };
}

export function rejectLocalCandidateMaterialization(
  previous: LocalCandidateOutputProjection,
  error: LocalCandidateMaterializationError,
): LocalCandidateOutputProjection {
  const invalidErrors: readonly LocalCandidateMaterializationError[] = [
    "trust_claim_rejected",
    "safety_claim_rejected",
    "readiness_claim_rejected",
    "release_claim_rejected",
    "deployment_claim_rejected",
    "public_use_claim_rejected",
    "provider_output_approval_claim_rejected",
    "action_claim_rejected",
    "persistence_claim_rejected",
    "execution_claim_rejected",
    "signing_claim_rejected",
    "publishing_claim_rejected",
  ];
  const missingErrors: readonly LocalCandidateMaterializationError[] = [
    "operator_decision_missing",
    "provider_output_validation_missing",
    "provider_output_review_missing",
    "staged_proposal_missing",
    "staged_proposal_validation_missing",
    "candidate_review_missing",
    "provider_pipeline_incomplete",
  ];
  return {
    ...previous,
    status: invalidErrors.includes(error)
      ? "invalid_materialization_request"
      : missingErrors.includes(error)
        ? "materialization_precondition_missing"
        : "materialization_rejected",
    error,
    note: `Local candidate materialization rejected: ${error}. Local candidate output only; provider output remains untrusted; no production, release, deployment, public-use, or action authorization effect occurred.`,
  };
}

export function projectLocalCandidateOutput(
  state: LocalOperatorShellState,
  request: LocalCandidateMaterializationRequest,
): LocalCandidateOutputProjection | LocalCandidateMaterializationError {
  const linkage = validateLocalCandidateMaterializationRequest(state, request);
  if (typeof linkage === "string") return linkage;
  const contentSummary = deriveLocalCandidateOutputContent(state, linkage);
  const digest = stableLocalCandidateDigest(
    `phase_158|${linkage.sourceInvocationResultId}|${linkage.providerExecutionResultId}|${linkage.stagedProposalId}|${linkage.operatorDecisionId}|${contentSummary}`,
  );
  return {
    status: "local_candidate_materialized",
    candidateId: `local-candidate-output-${digest}`,
    contentSummary,
    outputClassification: "local_candidate_output_only",
    productionClassification: "non_production_candidate",
    sourceLinkage: linkage,
    providerOutputTrustCarryForward: "provider_output_remains_untrusted",
    boundaryStatuses: localCandidateMaterializationBoundaryStatuses(),
    effectStatuses: localCandidateMaterializationEffectStatuses(),
    error: null,
    capabilitySurface: localCandidateMaterializationCapabilitySurface(),
    note: "Local candidate output only. This candidate output is non-production. Provider output remains untrusted. Candidate materialization does not approve readiness, release, deployment, or public use. Candidate materialization does not authorize actions. Production approval remains unavailable.",
  };
}

export function materializeLocalCandidateOutput(
  state: LocalOperatorShellState,
  request: LocalCandidateMaterializationRequest,
): LocalOperatorIntentResult {
  const projection = projectLocalCandidateOutput(state, request);
  if (typeof projection === "string") {
    return {
      status: "rejected",
      reason: projection,
      state: attachLocalSessionEvidenceExport({
        ...state,
        localCandidateOutput: rejectLocalCandidateMaterialization(
          state.localCandidateOutput,
          projection,
        ),
      }),
    };
  }
  return {
    status: "accepted",
    reason: "local_candidate_materialized",
    state: attachLocalSessionEvidenceExport({
      ...state,
      localCandidateOutput: projection,
    }),
  };
}

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
    note: "No operator candidate decision has been recorded. Decision applies only to validated staged proposal when present; no candidate output is created in Phase 149.",
  };
}

function deterministicOperatorCandidateDecisionId(
  request: OperatorCandidateDecisionRequest,
): string {
  const input = `${request.kind}|${request.stagedProposalId}|${request.providerExecutionResultId}|${request.stagedProposalValidationStatus}|phase_149`;
  let accumulator = 2166136261;
  for (let index = 0; index < input.length; index += 1) {
    accumulator ^= input.charCodeAt(index);
    accumulator = Math.imul(accumulator, 16777619) >>> 0;
  }
  return `operator-candidate-decision-${accumulator.toString(16).padStart(8, "0")}`;
}

export function validateOperatorCandidateDecisionRequest(
  state: LocalOperatorShellState,
  request: OperatorCandidateDecisionRequest,
): OperatorCandidateDecisionError | null {
  if (request.claimsTrust) return "trust_claim_rejected";
  if (request.claimsProviderOutputApproval)
    return "provider_output_approval_claim_rejected";
  if (request.claimsReadiness) return "readiness_claim_rejected";
  if (request.claimsRelease) return "release_claim_rejected";
  if (request.claimsDeployment) return "deployment_claim_rejected";
  if (request.claimsPublicUse) return "public_use_claim_rejected";
  if (request.claimsAction) return "action_claim_rejected";
  if (request.claimsExecution) return "execution_claim_rejected";
  if (request.claimsPersistence) return "persistence_claim_rejected";
  if (request.claimsCandidateCreation)
    return "candidate_creation_claim_rejected";
  if (request.claimsCandidateMaterialization)
    return "candidate_materialization_claim_rejected";
  const proposal = state.stagedCandidateConversionProposal.proposal;
  if (!proposal) return "no_staged_proposal";
  if (state.stagedCandidateConversionValidation.status === "not_validated")
    return "staged_proposal_not_validated";
  if (
    state.stagedCandidateConversionValidation.status ===
    "rejected_staged_proposal"
  )
    return "staged_proposal_validation_rejected";
  if (
    state.stagedCandidateConversionValidation.status ===
    "invalid_validation_input"
  )
    return "invalid_validation_input";
  if (request.stagedProposalValidationStatus !== "staged_proposal_shape_valid")
    return "source_linkage_inconsistent";
  if (
    request.stagedProposalId !== proposal.proposalId ||
    request.providerExecutionResultId !== proposal.sourceExecutionResultId
  )
    return "source_linkage_inconsistent";
  if (
    state.stagedCandidateConversionValidation.proposalId !==
      request.stagedProposalId ||
    state.stagedCandidateConversionValidation.sourceExecutionResultId !==
      request.providerExecutionResultId
  )
    return "source_linkage_inconsistent";
  if (
    state.stagedCandidateConversionValidation.deterministicLinkageStatus !==
    "source_linkage_validated"
  )
    return "source_linkage_inconsistent";
  const reprojected = projectStagedCandidateConversionValidation(state, {
    proposalId: proposal.proposalId,
  });
  if (
    JSON.stringify(reprojected) !==
    JSON.stringify(state.stagedCandidateConversionValidation)
  )
    return "source_linkage_inconsistent";
  return null;
}

export function projectOperatorCandidateDecision(
  request: OperatorCandidateDecisionRequest,
): OperatorCandidateDecisionProjection {
  const status: OperatorCandidateDecisionStatus =
    request.kind === "approve_validated_staged_proposal"
      ? "approved_validated_staged_proposal"
      : "rejected_validated_staged_proposal";
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
      recoveryPromotionStatus: "no_recovery_promotion_effect",
    },
    note: "This decision applies only to the validated staged proposal. No candidate output is created in Phase 149. Provider output remains untrusted and not approved. This decision does not approve readiness, release, deployment, or public use.",
  };
}

export function rejectedOperatorCandidateDecisionProjection(
  error: OperatorCandidateDecisionError,
): OperatorCandidateDecisionProjection {
  return {
    status: "rejected_operator_decision_request",
    record: null,
    error,
    note: "Operator candidate decision request rejected; authoritative shell state is preserved and no candidate materialization is performed.",
  };
}

export function submitOperatorCandidateDecision(
  state: LocalOperatorShellState,
  request: OperatorCandidateDecisionRequest,
): LocalOperatorIntentResult {
  const error = validateOperatorCandidateDecisionRequest(state, request);
  if (error)
    return {
      status: "rejected",
      reason: error,
      state: {
        ...state,
        operatorCandidateDecision:
          rejectedOperatorCandidateDecisionProjection(error),
        phase150CodeProductionHandoff: derivePhase150CodeProductionHandoff({
          ...state,
          operatorCandidateDecision:
            rejectedOperatorCandidateDecisionProjection(error),
        }),
      },
    };
  const next = {
    ...state,
    operatorCandidateDecision: projectOperatorCandidateDecision(request),
  };
  return {
    status: "accepted",
    reason: "operator_candidate_decision_recorded",
    state: attachLocalSessionEvidenceExport({
      ...next,
      phase150CodeProductionHandoff: derivePhase150CodeProductionHandoff(next),
    }),
  };
}

export function phase150RemainingProductionGaps(): readonly string[] {
  return [
    "local session persistence",
    "session restore",
    "real adapter contract",
    "real provider invocation",
    "candidate materialization",
    "complete local operator workflow",
    "run history",
    "export package",
    "controlled trial readiness",
    "deployment/package path",
  ];
}

export function derivePhase150CodeProductionHandoff(
  state: LocalOperatorShellState,
): Phase150CodeProductionHandoff {
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
      `operator decision boundary: ${state.operatorCandidateDecision.status}`,
    ],
    remainingProductionGradeGaps: phase150RemainingProductionGaps(),
    remapRecommendations: [
      "Phase 150 should perform an aggressive production-path remap.",
      "Phase 150 should group larger product capability phases.",
      "Safety checks remain embedded in implementation phases.",
      "Phase 150 is the roadmap/changelog alignment phase.",
      "Phase 149 does not edit roadmap files.",
    ],
    phase149RoadmapEditStatus: "phase_149_does_not_edit_roadmap_files",
  };
}

function deterministicStagedCandidateConversionProposalId(
  executionResultId: string,
  validation: LocalProviderOutputValidationProjection,
): string {
  const input = `${validation.providerKind}|${executionResultId}|${validation.status}|${validation.reviewabilityStatus}|${validation.candidateBoundaryStatus}|phase_146`;
  let accumulator = 2166136261;
  for (let index = 0; index < input.length; index += 1) {
    accumulator ^= input.charCodeAt(index);
    accumulator = Math.imul(accumulator, 16777619) >>> 0;
  }
  return `staged-candidate-conversion-proposal-${accumulator.toString(16).padStart(8, "0")}`;
}

function proposalRequestContainsForbiddenClaim(
  request: StagedCandidateConversionProposalRequest,
): boolean {
  const claims = request.claims ?? [];
  return claims.some(({ key, value }) => {
    const text = `${key} ${value}`.toLowerCase();
    return [
      "trust",
      "approval",
      "approved",
      "safe",
      "readiness",
      "ready",
      "release",
      "deployment",
      "public-use",
      "public_use",
      "execute",
      "execution",
      "persistence",
      "persistent",
      "action",
      "candidate_creation",
      "candidate_output",
      "candidate_material",
      "conversion_performed",
    ].some((needle) => text.includes(needle));
  });
}

export function validateStagedCandidateConversionSource(
  state: LocalOperatorShellState,
): StagedCandidateConversionSourceEligibilityStatus {
  if (!state.providerExecution.result)
    return "missing_provider_execution_result";
  if (
    validateLocalProviderOutputValidationProjection(
      state.providerOutputValidation,
    ).length > 0
  )
    return "missing_or_inconsistent_validation_projection";
  const projected = projectLocalProviderOutputValidation(state);
  if (
    JSON.stringify(projected) !== JSON.stringify(state.providerOutputValidation)
  )
    return "missing_or_inconsistent_validation_projection";
  if (state.providerOutputValidation.status === "rejected")
    return "rejected_source_not_eligible";
  if (state.providerOutputValidation.status === "validation_not_applicable")
    return "validation_not_applicable_source_not_eligible";
  if (state.providerOutputValidation.status === "invalid_validation_input")
    return "invalid_validation_input_source_not_eligible";
  if (state.providerOutputValidation.status !== "reviewable_untrusted")
    return "source_not_reviewable_untrusted";
  if (
    state.providerOutputValidation.reviewabilityStatus !==
      "reviewable_untrusted" ||
    state.providerOutputValidation.candidateBoundaryStatus !==
      "not_candidate_material" ||
    !state.providerOutputValidation.candidateBoundaryStatuses.includes(
      "candidate_conversion_not_performed",
    ) ||
    !state.providerOutputValidation.candidateBoundaryStatuses.includes(
      "candidate_conversion_requires_future_phase",
    )
  )
    return "source_not_reviewable_untrusted";
  return "eligible_reviewable_untrusted";
}

function sourceEligibilityError(
  status: StagedCandidateConversionSourceEligibilityStatus,
): StagedCandidateConversionProposalError | null {
  if (status === "eligible_reviewable_untrusted") return null;
  return status === "source_not_reviewable_untrusted"
    ? "source_not_reviewable_untrusted"
    : status;
}

export function validateStagedCandidateConversionProposal(
  projection: StagedCandidateConversionProposalProjection,
): StagedCandidateConversionProposalError | null {
  if (projection.status === "no_proposal") return null;
  const proposal = projection.proposal;
  if (!proposal) return "invalid_proposal_boundary";
  if (
    !stagedCandidateConversionBoundaryStatuses().every((status) =>
      proposal.boundaryStatuses.includes(status),
    )
  )
    return "invalid_proposal_boundary";
  if (
    !stagedCandidateConversionTrustStatuses().every((status) =>
      proposal.trustStatuses.includes(status),
    )
  )
    return "invalid_proposal_boundary";
  if (
    !stagedCandidateConversionNoEffects().every((status) =>
      proposal.effectStatuses.includes(status),
    )
  )
    return "invalid_proposal_boundary";
  if (
    proposal.sourceValidationStatus !== "reviewable_untrusted" ||
    proposal.sourceReviewabilityStatus !== "reviewable_untrusted" ||
    proposal.sourceCandidateBoundaryStatus !== "not_candidate_material" ||
    proposal.sourceBoundary !== "provider_output_validation_phase_143" ||
    proposal.proposalBoundary !== "staged_candidate_conversion_phase_146" ||
    proposal.sourceEligibilityStatus !== "eligible_reviewable_untrusted"
  )
    return "invalid_proposal_boundary";
  return null;
}

export function createStagedCandidateConversionProposal(
  state: LocalOperatorShellState,
  request: StagedCandidateConversionProposalRequest,
): LocalOperatorIntentResult {
  if (proposalRequestContainsForbiddenClaim(request))
    return { status: "rejected", reason: "invalid_proposal_request", state };
  const eligibility = validateStagedCandidateConversionSource(state);
  const eligibilityError = sourceEligibilityError(eligibility);
  if (eligibilityError)
    return { status: "rejected", reason: eligibilityError, state };
  const executionResultId =
    state.providerOutputValidation.providerExecutionResultId;
  if (!executionResultId)
    return {
      status: "rejected",
      reason: "missing_provider_execution_result",
      state,
    };
  const proposal: StagedCandidateConversionProposal = {
    proposalId: deterministicStagedCandidateConversionProposalId(
      executionResultId,
      state.providerOutputValidation,
    ),
    sourceProviderKind: state.providerOutputValidation.providerKind,
    sourceExecutionResultId: executionResultId,
    sourceValidationStatus: state.providerOutputValidation.status,
    sourceReviewabilityStatus:
      state.providerOutputValidation.reviewabilityStatus,
    sourceCandidateBoundaryStatus:
      state.providerOutputValidation.candidateBoundaryStatus,
    sourceBoundary: "provider_output_validation_phase_143",
    proposalBoundary: "staged_candidate_conversion_phase_146",
    boundaryStatuses: stagedCandidateConversionBoundaryStatuses(),
    trustStatuses: stagedCandidateConversionTrustStatuses(),
    effectStatuses: stagedCandidateConversionNoEffects(),
    sourceEligibilityStatus: "eligible_reviewable_untrusted",
    note: `${request.operatorNote} This proposal is not persistent, not executable, not approved, and not candidate material.`,
  };
  const projection: StagedCandidateConversionProposalProjection = {
    status: "staged_proposal_created",
    proposal,
    sourceEligibilityStatus: "eligible_reviewable_untrusted",
    error: null,
    note: "This is a staged conversion proposal only. It is not candidate output.",
  };
  const error = validateStagedCandidateConversionProposal(projection);
  if (error) return { status: "rejected", reason: error, state };
  return {
    status: "accepted",
    reason: "staged_candidate_conversion_proposal_created",
    state: attachLocalSessionEvidenceExport({
      ...state,
      stagedCandidateConversionProposal: projection,
    }),
  };
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
      "provider output is not candidate material",
    ],
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
    sourceBoundary: "sandboxed_deterministic_provider_execution",
  };
}

function localProviderExecutionLinkage(
  state: LocalOperatorShellState,
  executionResultId: string,
): LocalProviderExecutionResultLinkage {
  return {
    shellStateLabel: state.harnessStatus,
    runId: state.run.runId,
    providerConfigurationKind:
      state.providerConfiguration.configuredProviderKind ?? "none",
    providerConfigurationStatus: state.providerConfiguration.status,
    executionResultId,
    candidateId: state.run.candidate?.candidateId ?? "not_candidate_material",
    sourceBoundary: "sandboxed_deterministic_provider_execution",
  };
}

export function validateLocalProviderExecutionResultProjection(
  projection: LocalProviderExecutionProjection,
): LocalProviderExecutionResultProjectionValidation {
  const errors: string[] = [];
  if (projection.outputTrustStatus !== "untrusted_descriptive")
    errors.push("invalid_trust_status");
  if (projection.outputMaterializationStatus === "candidate_material")
    errors.push("invalid_materialization_status");
  if (
    projection.outputPromotionStatus !== "not_promoted" ||
    projection.promotionAvailabilityStatus !==
      "promotion_not_available_in_phase_142"
  )
    errors.push("invalid_promotion_status");
  const markers = projection.absenceMarkers;
  if (
    !markers.noProcessSpawned ||
    !markers.noNetworkSocketOpened ||
    !markers.noFilesystemPersistence ||
    !markers.noSecretsRead ||
    !markers.noReleaseCreated ||
    !markers.noDeploymentCreated ||
    !markers.noSigningPerformed ||
    !markers.noPublishingPerformed ||
    !markers.noPublicUseApproved ||
    !markers.noReadinessApproved ||
    !markers.noReplayRepair ||
    !markers.noRecoveryPromotion ||
    !markers.noActionExecution ||
    !markers.providerOutputNotCandidateMaterial ||
    markers.markerSummary.length === 0
  )
    errors.push("missing_absence_marker");
  if (
    projection.linkage.runId.length === 0 ||
    projection.linkage.providerConfigurationKind.length === 0 ||
    projection.linkage.providerConfigurationStatus.length === 0 ||
    projection.linkage.executionResultId.length === 0 ||
    ![
      "sandboxed_deterministic_provider_execution",
      "constrained_local_provider_invocation_pipeline_bridge",
    ].includes(projection.linkage.sourceBoundary)
  )
    errors.push("missing_linkage");
  return errors.length === 0
    ? {
        status: "valid",
        errorCodes: [],
        reason:
          "provider execution result projection is valid; output remains untrusted_descriptive, not_candidate_material, and not_promoted",
      }
    : {
        status: "invalid",
        errorCodes: errors,
        reason: "provider execution result projection rejected fail-closed",
      };
}

function withProviderExecutionProjectionValidation(
  projection: LocalProviderExecutionProjection,
): LocalProviderExecutionProjection {
  return {
    ...projection,
    projectionValidation:
      validateLocalProviderExecutionResultProjection(projection),
  };
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
    projectionValidation: {
      status: "invalid",
      errorCodes: [],
      reason: "projection validation pending",
    },
    validationStatus: "not_executed",
    validationErrorCodes: [],
    validationReason: "deterministic_stub execution has not been requested",
    capabilitySurface: localProviderExecutionCapabilitySurface(),
    note: "Provider execution result projection is projection_only evidence; provider output is untrusted_descriptive, not_candidate_material, not_promoted, and not eligible for approve/reject in Phase 142.",
  });
}

export function projectLocalProviderExecution(
  state: LocalOperatorShellState,
): LocalProviderExecutionProjection {
  return withProviderExecutionProjectionValidation({
    ...state.providerExecution,
    configuredProviderKind:
      state.providerConfiguration.configuredProviderKind ?? "none",
    linkage: localProviderExecutionLinkage(
      state,
      state.providerExecution.result?.resultId ?? "none",
    ),
  });
}

function forbiddenProviderExecutionField(
  key: string,
  value: string,
): LocalProviderExecutionError {
  const loweredKey = key.toLowerCase();
  const combined = `${loweredKey}=${value.toLowerCase()}`;
  if (
    ["endpoint", "url", "host", "port", "http", "network"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "forbidden_endpoint_field";
  if (
    ["command", "args", "shell", "process"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "forbidden_command_field";
  if (["path", "file", "directory"].some((needle) => combined.includes(needle)))
    return "forbidden_path_field";
  if (
    ["secret", "token", "api_key", "apikey", "credential"].some((needle) =>
      combined.includes(needle),
    )
  )
    return "forbidden_secret_field";
  if (loweredKey === "provider_execution_enabled")
    return "provider_execution_flag_rejected";
  if (loweredKey === "trust_granted") return "trust_grant_rejected";
  if (loweredKey === "readiness_approved") return "readiness_claim_rejected";
  if (loweredKey === "release_candidate_approved")
    return "release_claim_rejected";
  if (loweredKey === "deployment_enabled") return "deployment_claim_rejected";
  if (loweredKey === "public_use_approved") return "public_use_claim_rejected";
  if (loweredKey === "signing_enabled") return "signing_claim_rejected";
  if (loweredKey === "publishing_enabled") return "publishing_claim_rejected";
  return "unknown_field_rejected";
}

export function validateLocalProviderExecutionRequest(
  configuration: LocalProviderConfiguration,
  request: LocalProviderExecutionRequest,
): LocalProviderExecutionValidation {
  const errors = new Set<LocalProviderExecutionError>();
  if (
    configuration.status !== "accepted" ||
    configuration.configuredProviderKind !== "deterministic_stub"
  ) {
    errors.add(
      configuration.status === "not_configured"
        ? "missing_provider_configuration"
        : "rejected_provider_configuration",
    );
  }

  let providerKind: LocalProviderKind | null = null;
  const rawKind = request.providerKind;
  if (rawKind === undefined || rawKind.trim().length === 0) {
    errors.add("missing_provider_kind");
  } else if (rawKind.trim() !== rawKind) {
    errors.add("malformed_provider_kind");
  } else if (
    supportedLocalProviderKinds.includes(rawKind as LocalProviderKind)
  ) {
    providerKind = rawKind as LocalProviderKind;
    if (providerKind !== "deterministic_stub")
      errors.add("unsupported_provider_kind");
  } else {
    errors.add("unsupported_provider_kind");
  }

  if (
    request.inputSummary.trim().length === 0 ||
    request.inputSummary.length > 4096
  )
    errors.add("malformed_provider_kind");
  for (const field of request.fields ?? [])
    errors.add(forbiddenProviderExecutionField(field.key, field.value));

  const errorCodes = [...errors].sort();
  if (errorCodes.length === 0 && providerKind === "deterministic_stub") {
    return {
      status: "executed",
      providerKind,
      errorCodes: [],
      reason:
        "deterministic_stub execution accepted inside Rust-owned sandboxed deterministic boundary",
    };
  }
  const status: LocalProviderExecutionStatus = errors.has(
    "unsupported_provider_kind",
  )
    ? "unsupported_provider"
    : errors.has("missing_provider_configuration") ||
        errors.has("rejected_provider_configuration")
      ? "configuration_required"
      : "invalid_request";
  return {
    status,
    providerKind,
    errorCodes,
    reason:
      "provider execution rejected fail-closed; previous shell state is preserved",
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

export function executeSandboxedDeterministicProvider(
  request: LocalProviderExecutionRequest,
): LocalProviderExecutionResult {
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
    releaseOrDeploymentEvidenceCreated: false,
  };
}

export function applyLocalProviderExecution(
  state: LocalOperatorShellState,
  request: LocalProviderExecutionRequest,
): LocalOperatorIntentResult {
  const validation = validateLocalProviderExecutionRequest(
    state.providerConfiguration,
    request,
  );
  if (validation.status !== "executed")
    return { status: "rejected", reason: validation.reason, state };
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
    projectionValidation: {
      status: "invalid",
      errorCodes: [],
      reason: "projection validation pending",
    },
    validationStatus: validation.status,
    validationErrorCodes: [],
    validationReason: validation.reason,
    capabilitySurface: localProviderExecutionCapabilitySurface(),
    note: "Provider execution result projection is projection_only evidence; provider output is untrusted_descriptive, not_candidate_material, not_promoted, promotion_not_available_in_phase_142, and not eligible for approve/reject in Phase 142.",
  });
  return {
    status: "accepted",
    reason: "local_provider_execution_accepted",
    state: {
      ...state,
      providerExecution,
      providerOutputValidation: validateLocalProviderOutput(providerExecution),
    },
  };
}

export type LocalSessionPackageStatus =
  | "not_packaged"
  | "package_projected"
  | "package_written"
  | "package_read_back_validated"
  | "package_rejected"
  | "invalid_package_input";

export type LocalSessionPackageValidationStatus =
  | "not_validated"
  | "valid"
  | "invalid";
export type LocalSessionPackageRestoreStatus =
  | "not_restored"
  | "read_back_validated_structure_only"
  | "restore_projection_rejected";

export type LocalSessionPackageProjection = Readonly<{
  status: LocalSessionPackageStatus;
  packageId: string | null;
  packageVersion: string;
  packageClassification: "local_session_package_only";
  productionClassification: "non_production";
  validationStatus: LocalSessionPackageValidationStatus;
  validationErrors: readonly string[];
  readBackValidationStatus: LocalSessionPackageValidationStatus | null;
  restoreStatus: LocalSessionPackageRestoreStatus;
  includedSectionSummary: readonly string[];
  absenceMarkerSummary: readonly string[];
  localOnlyNote: "Local session package is local-only and non-production.";
  releaseBoundaryNote: "This package is not a release artifact.";
  deploymentReadinessBoundaryNote: "This package is not deployment or readiness evidence.";
  restoreBoundaryNote: "Package restore/read-back validates structure only and does not promote recovery.";
}>;

export function initialLocalSessionPackageProjection(): LocalSessionPackageProjection {
  return {
    status: "not_packaged",
    packageId: null,
    packageVersion: "local-session-package-v1",
    packageClassification: "local_session_package_only",
    productionClassification: "non_production",
    validationStatus: "not_validated",
    validationErrors: [],
    readBackValidationStatus: null,
    restoreStatus: "not_restored",
    includedSectionSummary: [],
    absenceMarkerSummary: [
      "release artifact absent",
      "deployment artifact absent",
      "readiness evidence absent",
      "production persistence absent",
      "installer absent",
      "update-channel absent",
      "signing absent",
      "publishing absent",
      "public-use absent",
      "provider trust absent",
      "candidate approval absent",
      "action execution absent",
      "automatic persistence absent",
      "background service absent",
      "remote sync absent",
    ],
    localOnlyNote: "Local session package is local-only and non-production.",
    releaseBoundaryNote: "This package is not a release artifact.",
    deploymentReadinessBoundaryNote:
      "This package is not deployment or readiness evidence.",
    restoreBoundaryNote:
      "Package restore/read-back validates structure only and does not promote recovery.",
  };
}

export type ControlledInternalTrialPackageStatus =
  | "not_packaged"
  | "package_projected"
  | "package_validated"
  | "package_written"
  | "package_read_back_validated"
  | "package_rejected"
  | "invalid_package_input";

export type ControlledInternalTrialPackageValidationStatus =
  | "not_validated"
  | "valid"
  | "invalid";

export type ControlledInternalTrialPackageBoundaryStatus =
  | "local_only_trial_package"
  | "non_public_trial_package"
  | "no_release_artifact"
  | "no_deployment_artifact"
  | "no_readiness_approval"
  | "no_release_approval"
  | "no_production_approval"
  | "no_public_use_approval"
  | "no_provider_trust"
  | "no_action_authorization"
  | "no_replay_repair"
  | "no_recovery_promotion";

export type ControlledInternalTrialPackageProjection = Readonly<{
  status: ControlledInternalTrialPackageStatus;
  packageId: string | null;
  packageVersion: "controlled-internal-trial-package-v1";
  packageClassification: "controlled_internal_trial_package_only";
  productionClassification: "non_production";
  distributionClassification: "local_only_non_public";
  trialScopeSummary: string;
  namedOperatorParticipantSummary: readonly string[];
  stopConditionSummary: readonly string[];
  includedEvidenceSummary: readonly string[];
  absenceMarkerSummary: readonly string[];
  validationStatus: ControlledInternalTrialPackageValidationStatus;
  validationErrors: readonly string[];
  readBackValidationStatus: ControlledInternalTrialPackageValidationStatus | null;
  boundaryStatus: readonly ControlledInternalTrialPackageBoundaryStatus[];
  localOnlyNonPublicNote: "Controlled internal trial package is local-only and non-public.";
  releaseBoundaryNote: "This package is not a release artifact.";
  deploymentReadinessBoundaryNote: "This package is not deployment or readiness evidence.";
  publicProductionBoundaryNote: "This package does not approve public/general use or production use.";
  stopConditionNote: "Stop conditions are trial metadata; they do not automate enforcement in Phase 161.";
}>;

export function initialControlledInternalTrialPackageProjection(): ControlledInternalTrialPackageProjection {
  return {
    status: "not_packaged",
    packageId: null,
    packageVersion: "controlled-internal-trial-package-v1",
    packageClassification: "controlled_internal_trial_package_only",
    productionClassification: "non_production",
    distributionClassification: "local_only_non_public",
    trialScopeSummary: "trial scope not provided",
    namedOperatorParticipantSummary: [],
    stopConditionSummary: [
      "stop_on_provider_trust_claim",
      "stop_on_readiness_claim",
      "stop_on_release_or_deployment_claim",
      "stop_on_public_use_claim",
      "stop_on_action_authorization_claim",
      "stop_on_replay_repair_or_recovery_promotion_claim",
      "stop_on_package_validation_failure",
      "stop_on_restore_validation_failure",
      "stop_on_workflow_rejection",
      "stop_on_operator_escalation",
    ],
    includedEvidenceSummary: [],
    absenceMarkerSummary: [
      "no_release_artifact",
      "no_deployment_artifact",
      "no_readiness_approval",
      "no_public_use_approval",
      "no_production_approval",
      "no_provider_trust",
      "no_action_authorization",
      "no_replay_repair",
      "no_recovery_promotion",
      "no_default_persistence",
    ],
    validationStatus: "not_validated",
    validationErrors: [],
    readBackValidationStatus: null,
    boundaryStatus: [
      "local_only_trial_package",
      "non_public_trial_package",
      "no_release_artifact",
      "no_deployment_artifact",
      "no_readiness_approval",
      "no_release_approval",
      "no_production_approval",
      "no_public_use_approval",
      "no_provider_trust",
      "no_action_authorization",
      "no_replay_repair",
      "no_recovery_promotion",
    ],
    localOnlyNonPublicNote:
      "Controlled internal trial package is local-only and non-public.",
    releaseBoundaryNote: "This package is not a release artifact.",
    deploymentReadinessBoundaryNote:
      "This package is not deployment or readiness evidence.",
    publicProductionBoundaryNote:
      "This package does not approve public/general use or production use.",
    stopConditionNote:
      "Stop conditions are trial metadata; they do not automate enforcement in Phase 161.",
  };
}

export type LocalSessionHistoryStatus =
  | "no_session_history"
  | "session_history_projected";

export type LocalSessionHistoryEntry = Readonly<{
  packageId: string;
  packageVersion: string;
  packageClassification: "local_session_package_only";
  productionClassification: "non_production";
  packageStatus: LocalSessionPackageStatus;
  validationStatus: LocalSessionPackageValidationStatus;
  readBackValidationStatus: LocalSessionPackageValidationStatus | null;
  includedSectionSummary: readonly string[];
  absenceMarkerSummary: readonly string[];
}>;

export type LocalSessionHistoryProjection = Readonly<{
  status: LocalSessionHistoryStatus;
  entries: readonly LocalSessionHistoryEntry[];
  selectedPackageId: string | null;
  boundaryNote: "Session history is derived only from explicit local package entries; No automatic filesystem scanning.";
}>;

export type LocalSessionRestoreStatus =
  | "restore_not_requested"
  | "package_selected"
  | "package_read_back_validated"
  | "restore_preview_projected"
  | "restore_projected"
  | "restore_rejected"
  | "invalid_restore_input";

export type LocalSessionRestoreValidationStatus =
  | "not_validated"
  | "valid"
  | "invalid";
export type LocalSessionRestoreReadBackStatus =
  | "not_read"
  | "package_read_back_validated"
  | "read_back_rejected";
export type LocalSessionRestoreBoundaryStatus =
  | "local_restore_projection_only"
  | "no_recovery_promotion"
  | "no_replay_repair"
  | "no_production_persistence_claim"
  | "no_readiness_effect"
  | "no_release_effect"
  | "no_deployment_effect"
  | "no_public_use_effect";

export type LocalSessionRestoreError =
  | "no_package_selected"
  | "package_read_failed"
  | "package_parse_failed"
  | "package_validation_failed"
  | "invalid_package_classification"
  | "invalid_production_classification"
  | "missing_required_package_section"
  | "missing_absence_marker"
  | "readiness_claim_detected"
  | "release_claim_detected"
  | "deployment_claim_detected"
  | "public_use_claim_detected"
  | "provider_trust_claim_detected"
  | "candidate_approval_claim_detected"
  | "action_execution_claim_detected"
  | "replay_repair_claim_detected"
  | "recovery_promotion_claim_detected"
  | "nondeterministic_restore_projection";

export type LocalSessionRestoreProjection = Readonly<{
  status: LocalSessionRestoreStatus;
  packageId: string | null;
  packageVersion: string | null;
  packageClassification: "local_session_package_only" | null;
  productionClassification: "non_production" | null;
  validationStatus: LocalSessionRestoreValidationStatus;
  readBackStatus: LocalSessionRestoreReadBackStatus;
  errors: readonly LocalSessionRestoreError[];
  includedSectionSummary: readonly string[];
  absenceMarkerSummary: readonly string[];
  boundaryStatus: readonly LocalSessionRestoreBoundaryStatus[];
  localOnlyNote: "Session restore is local-only and non-production.";
  readBackNote: "Read-back validation checks package structure; it is not restore authority.";
  previewBoundaryNote: "Restore preview does not repair replay or promote recovery.";
  restoredProjectionNote: "Restored session projection does not imply readiness, release, deployment, or public use.";
  remoteBackgroundNote: "No remote sync or background restore is active.";
}>;

export function initialLocalSessionHistoryProjection(): LocalSessionHistoryProjection {
  return {
    status: "no_session_history",
    entries: [],
    selectedPackageId: null,
    boundaryNote:
      "Session history is derived only from explicit local package entries; No automatic filesystem scanning.",
  };
}

export function initialLocalSessionRestoreProjection(): LocalSessionRestoreProjection {
  return {
    status: "restore_not_requested",
    packageId: null,
    packageVersion: null,
    packageClassification: null,
    productionClassification: null,
    validationStatus: "not_validated",
    readBackStatus: "not_read",
    errors: [],
    includedSectionSummary: [],
    absenceMarkerSummary:
      initialLocalSessionPackageProjection().absenceMarkerSummary,
    boundaryStatus: [
      "local_restore_projection_only",
      "no_recovery_promotion",
      "no_replay_repair",
      "no_production_persistence_claim",
      "no_readiness_effect",
      "no_release_effect",
      "no_deployment_effect",
      "no_public_use_effect",
    ],
    localOnlyNote: "Session restore is local-only and non-production.",
    readBackNote:
      "Read-back validation checks package structure; it is not restore authority.",
    previewBoundaryNote:
      "Restore preview does not repair replay or promote recovery.",
    restoredProjectionNote:
      "Restored session projection does not imply readiness, release, deployment, or public use.",
    remoteBackgroundNote: "No remote sync or background restore is active.",
  };
}

export function projectLocalSessionHistoryFromPackages(
  packages: readonly LocalSessionPackageProjection[],
): LocalSessionHistoryProjection {
  const entries = packages
    .filter(
      (
        projection,
      ): projection is LocalSessionPackageProjection & { packageId: string } =>
        projection.packageId !== null,
    )
    .map((projection) => ({
      packageId: projection.packageId,
      packageVersion: projection.packageVersion,
      packageClassification: projection.packageClassification,
      productionClassification: projection.productionClassification,
      packageStatus: projection.status,
      validationStatus: projection.validationStatus,
      readBackValidationStatus: projection.readBackValidationStatus,
      includedSectionSummary: projection.includedSectionSummary,
      absenceMarkerSummary: projection.absenceMarkerSummary,
    }));
  return {
    status:
      entries.length === 0 ? "no_session_history" : "session_history_projected",
    entries,
    selectedPackageId: entries[0]?.packageId ?? null,
    boundaryNote:
      "Session history is derived only from explicit local package entries; No automatic filesystem scanning.",
  };
}

export function projectLocalSessionRestoreFromPackageProjection(
  projection: LocalSessionPackageProjection,
): LocalSessionRestoreProjection {
  if (projection.packageId === null) {
    return {
      ...initialLocalSessionRestoreProjection(),
      status: "restore_rejected",
      validationStatus: "invalid",
      readBackStatus: "read_back_rejected",
      errors: ["no_package_selected"],
    };
  }
  if (projection.validationStatus !== "valid") {
    return {
      ...initialLocalSessionRestoreProjection(),
      status: "restore_rejected",
      packageId: projection.packageId,
      packageVersion: projection.packageVersion,
      packageClassification: projection.packageClassification,
      productionClassification: projection.productionClassification,
      validationStatus: "invalid",
      readBackStatus: "read_back_rejected",
      errors: projection.validationErrors.includes(
        "invalid_package_classification",
      )
        ? ["invalid_package_classification"]
        : ["package_validation_failed"],
    };
  }
  return {
    status: "restore_preview_projected",
    packageId: projection.packageId,
    packageVersion: projection.packageVersion,
    packageClassification: projection.packageClassification,
    productionClassification: projection.productionClassification,
    validationStatus: "valid",
    readBackStatus: "package_read_back_validated",
    errors: [],
    includedSectionSummary: projection.includedSectionSummary,
    absenceMarkerSummary: projection.absenceMarkerSummary,
    boundaryStatus: initialLocalSessionRestoreProjection().boundaryStatus,
    localOnlyNote: "Session restore is local-only and non-production.",
    readBackNote:
      "Read-back validation checks package structure; it is not restore authority.",
    previewBoundaryNote:
      "Restore preview does not repair replay or promote recovery.",
    restoredProjectionNote:
      "Restored session projection does not imply readiness, release, deployment, or public use.",
    remoteBackgroundNote: "No remote sync or background restore is active.",
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

export type CompleteLocalOperatorWorkflowStatus =
  | "not_started"
  | "in_progress"
  | "blocked"
  | "rejected"
  | "local_candidate_materialized"
  | "complete_local_workflow_projected";

export type CompleteLocalOperatorWorkflowStepStatus =
  | "not_started"
  | "available"
  | "completed"
  | "blocked"
  | "rejected"
  | "not_applicable";

export type CompleteLocalOperatorWorkflowStepKind =
  | "provider_adapter_configured"
  | "adapter_dry_run_available"
  | "constrained_invocation_completed"
  | "provider_output_pipeline_projected"
  | "provider_output_validated"
  | "provider_output_reviewed"
  | "staged_proposal_created"
  | "staged_proposal_validated"
  | "candidate_review_projected"
  | "operator_decision_recorded"
  | "local_candidate_materialized"
  | "replay_status_projected"
  | "local_evidence_export_projected"
  | "session_package_projected"
  | "restore_status_projected";

export type CompleteLocalOperatorWorkflowError =
  | "adapter_not_configured"
  | "invocation_missing"
  | "invocation_rejected"
  | "provider_pipeline_blocked"
  | "provider_output_validation_missing"
  | "provider_output_validation_rejected"
  | "provider_output_review_missing"
  | "staged_proposal_missing"
  | "staged_proposal_validation_missing"
  | "staged_proposal_validation_rejected"
  | "candidate_review_missing"
  | "operator_decision_missing"
  | "operator_decision_rejected"
  | "local_candidate_not_materialized"
  | "replay_status_missing"
  | "evidence_export_missing"
  | "session_package_missing"
  | "restore_status_missing";

export type CompleteLocalOperatorWorkflowBoundaryStatus =
  | "local_beta_workflow_only"
  | "no_provider_trust"
  | "no_production_approval"
  | "no_release_approval"
  | "no_deployment_approval"
  | "no_public_use_approval"
  | "no_action_execution"
  | "no_replay_repair"
  | "no_recovery_promotion";

export type CompleteLocalOperatorWorkflowStep = Readonly<{
  step: CompleteLocalOperatorWorkflowStepKind;
  status: CompleteLocalOperatorWorkflowStepStatus;
  error: CompleteLocalOperatorWorkflowError | null;
  summary: string;
}>;

export type CompleteLocalOperatorWorkflowEvidenceSummary = Readonly<{
  providerOutputPipelineStatus: string;
  localCandidateMaterializationStatus: string;
  replayStatus: string;
  localEvidenceExportStatus: string;
  sessionPackageStatus: string;
  sessionHistoryStatus: string;
  restoreStatus: string;
}>;

export type CompleteLocalOperatorWorkflowCapabilitySurface = Readonly<{
  localOnly: true;
  nonProduction: true;
  providerTrustGranted: false;
  actionExecutionAuthorized: false;
  readinessApproved: false;
  releaseApproved: false;
  deploymentApproved: false;
  publicUseApproved: false;
  replayRepairPerformed: false;
  recoveryPromotionPerformed: false;
}>;

export type CompleteLocalOperatorWorkflowProjection = Readonly<{
  status: CompleteLocalOperatorWorkflowStatus;
  classification: "local_beta_workflow_only";
  currentStep: CompleteLocalOperatorWorkflowStepKind | null;
  nextRequiredStep: CompleteLocalOperatorWorkflowStepKind | null;
  currentBlockingStep: CompleteLocalOperatorWorkflowStepKind | null;
  currentError: CompleteLocalOperatorWorkflowError | null;
  steps: readonly CompleteLocalOperatorWorkflowStep[];
  rejectionReasons: readonly string[];
  evidenceSummary: CompleteLocalOperatorWorkflowEvidenceSummary;
  boundaryStatuses: readonly CompleteLocalOperatorWorkflowBoundaryStatus[];
  capabilitySurface: CompleteLocalOperatorWorkflowCapabilitySurface;
  localOnlyNote: string;
  noAuthorityNote: string;
}>;

export type TrialOperatorRunbookStatus =
  | "not_available"
  | "trial_package_required"
  | "trial_operator_runbook_projected"
  | "blocked"
  | "failure_drill_required"
  | "ready_for_manual_trial_preparation";

export type TrialOperatorRunbookStepStatus =
  | "not_started"
  | "available"
  | "completed"
  | "blocked"
  | "rejected"
  | "manual_action_required"
  | "not_applicable";

export type TrialOperatorRunbookStepKind =
  | "confirm_local_beta_workflow"
  | "confirm_controlled_trial_package"
  | "confirm_trial_scope"
  | "confirm_named_internal_operator"
  | "confirm_named_trial_participant"
  | "review_stop_conditions"
  | "review_current_blocker"
  | "review_failure_drill"
  | "review_restore_status"
  | "review_replay_status"
  | "review_evidence_export_status"
  | "prepare_manual_trial_notes"
  | "confirm_no_public_release"
  | "confirm_no_production_approval";

export type TrialOperatorRunbookStep = Readonly<{
  step: TrialOperatorRunbookStepKind;
  status: TrialOperatorRunbookStepStatus;
  summary: string;
}>;

export type TrialOperatorRunbookCurrentBlocker = Readonly<{
  step: TrialOperatorRunbookStepKind | null;
  workflowStep: CompleteLocalOperatorWorkflowStepKind | null;
  workflowError: CompleteLocalOperatorWorkflowError | null;
  guidance: string;
}>;

export type TrialRunbookBoundaryStatus =
  | "local_trial_guidance_only"
  | "non_public_trial_guidance"
  | "no_trial_execution"
  | "no_stop_condition_automation"
  | "no_authority_activation"
  | "no_readiness_approval"
  | "no_release_approval"
  | "no_deployment_approval"
  | "no_public_use_approval"
  | "no_production_approval"
  | "no_action_execution"
  | "no_replay_repair"
  | "no_recovery_promotion";

export type TrialRunbookCapabilitySurface = Readonly<{
  localOnly: true;
  nonPublic: true;
  executesTrial: false;
  automatesStopConditions: false;
  activatesAuthority: false;
  approvesReadiness: false;
  approvesRelease: false;
  approvesDeployment: false;
  approvesPublicUse: false;
  approvesProduction: false;
  executesActions: false;
  repairsReplay: false;
  promotesRecovery: false;
}>;

export type TrialFailureSeverity =
  | "info"
  | "manual_action"
  | "blocked"
  | "rejected";
export type TrialFailureDrillStatus =
  | "no_failures_projected"
  | "failure_drill_required"
  | "stop_condition_drill_required";
export type TrialFailureCategory =
  | "no_trial_package"
  | "trial_package_validation_failure"
  | "trial_package_read_back_failure"
  | "missing_trial_scope"
  | "missing_named_operator"
  | "missing_named_participant"
  | "missing_stop_conditions"
  | "workflow_blocked"
  | "workflow_rejected"
  | "provider_pipeline_blocked"
  | "provider_output_validation_rejected"
  | "staged_validation_rejected"
  | "operator_decision_rejected"
  | "candidate_materialization_missing"
  | "restore_projection_rejected"
  | "replay_status_incomplete"
  | "evidence_export_missing"
  | "stop_condition_present"
  | "security_escalation_required"
  | "release_steward_review_required"
  | "trial_coordinator_review_required";

export type TrialFailureCategoryProjection = Readonly<{
  category: TrialFailureCategory;
  severity: TrialFailureSeverity;
  summary: string;
}>;

export type TrialStopConditionDrill = Readonly<{
  marker: string;
  status: TrialOperatorRunbookStepStatus;
  guidance: string;
  enforcementAutomated: false;
}>;

export type TrialEscalationRole =
  | "trial_coordinator"
  | "security_reviewer"
  | "release_steward"
  | "operator_manual_review";

export type TrialEscalationGuidance = Readonly<{
  role: TrialEscalationRole;
  categories: readonly TrialFailureCategory[];
  guidance: string;
  descriptiveOnly: true;
}>;

export type TrialFailureDrillProjection = Readonly<{
  status: TrialFailureDrillStatus;
  highestSeverity: TrialFailureSeverity;
  categories: readonly TrialFailureCategoryProjection[];
  stopConditionDrills: readonly TrialStopConditionDrill[];
  escalationGuidance: readonly TrialEscalationGuidance[];
  manualActionGuidance: readonly string[];
  rejectionSummary: readonly string[];
  boundaryStatuses: readonly TrialRunbookBoundaryStatus[];
  noAutomationNote: string;
}>;

export type TrialOperatorRunbookProjection = Readonly<{
  status: TrialOperatorRunbookStatus;
  currentStep: TrialOperatorRunbookStepKind | null;
  currentBlocker: TrialOperatorRunbookCurrentBlocker;
  steps: readonly TrialOperatorRunbookStep[];
  trialPackageStatus: ControlledInternalTrialPackageStatus;
  trialPackageId: string | null;
  trialPackageValidationStatus: ControlledInternalTrialPackageValidationStatus;
  trialPackageReadBackStatus: ControlledInternalTrialPackageValidationStatus | null;
  trialScopeStatus: TrialOperatorRunbookStepStatus;
  namedOperatorStatus: TrialOperatorRunbookStepStatus;
  namedParticipantStatus: TrialOperatorRunbookStepStatus;
  stopConditionSummary: readonly string[];
  localWorkflowStatus: CompleteLocalOperatorWorkflowStatus;
  localCandidateMaterializationStatus: string;
  providerOutputPipelineStatus: string;
  replayStatusSummary: string;
  localEvidenceExportSummary: string;
  restoreHistoryStatus: string;
  failureDrill: TrialFailureDrillProjection;
  boundaryStatuses: readonly TrialRunbookBoundaryStatus[];
  capabilitySurface: TrialRunbookCapabilitySurface;
  localOnlyNonPublicNote: string;
  noTrialExecutionNote: string;
  noAuthorityNote: string;
}>;

export type TrialSessionEvidenceStatus =
  | "not_captured"
  | "evidence_projected"
  | "evidence_validated"
  | "evidence_written"
  | "evidence_read_back_validated"
  | "evidence_rejected"
  | "invalid_evidence_input";

export type TrialSessionEvidenceValidationStatus =
  | "not_validated"
  | "valid"
  | "invalid";

export type TrialSessionEvidenceBoundaryStatus =
  | "local_trial_evidence_only"
  | "non_public_evidence"
  | "evidence_not_authority"
  | "no_trial_execution"
  | "no_controlled_human_use_approval"
  | "no_readiness_approval"
  | "no_release_approval"
  | "no_deployment_approval"
  | "no_public_use_approval"
  | "no_production_approval"
  | "no_provider_trust"
  | "no_action_authorization"
  | "no_replay_repair"
  | "no_recovery_promotion";

export type TrialSessionEvidenceProjection = Readonly<{
  status: TrialSessionEvidenceStatus;
  evidenceId: string | null;
  evidenceVersion: "trial-session-evidence-v1";
  evidenceClassification: "trial_session_evidence_only";
  productionClassification: "non_production";
  distributionClassification: "local_only_non_public";
  authorityClassification: "non_authoritative_evidence";
  trialPackageLinkage: string;
  runbookLinkage: string;
  failureDrillLinkage: string;
  includedEvidenceSummary: readonly string[];
  workflowSnapshotSummary: string;
  localCandidateMaterializationSnapshot: string;
  replayStatusSnapshot: string;
  evidenceExportSessionPackageRestoreSnapshot: string;
  stopConditionSnapshot: readonly string[];
  escalationSnapshot: readonly string[];
  failureCategorySnapshot: readonly string[];
  absenceMarkerSummary: readonly string[];
  validationStatus: TrialSessionEvidenceValidationStatus;
  validationErrors: readonly string[];
  readBackValidationStatus: TrialSessionEvidenceValidationStatus | null;
  boundaryStatus: readonly TrialSessionEvidenceBoundaryStatus[];
  localOnlyNonAuthoritativeNote: "Trial session evidence is local-only, non-public, and non-authoritative.";
  localPreparationOnlyNote: "Evidence capture records local trial-preparation state only.";
  noTrialApprovalNote: "Evidence capture does not start or approve a controlled trial.";
  noReleaseDeploymentReadinessNote: "This evidence is not release, deployment, readiness, public-use, or production-use evidence.";
  readBackValidationNote: "Read-back validation checks evidence structure only.";
}>;

export function trialSessionEvidenceBoundaryStatuses(): readonly TrialSessionEvidenceBoundaryStatus[] {
  return [
    "local_trial_evidence_only",
    "non_public_evidence",
    "evidence_not_authority",
    "no_trial_execution",
    "no_controlled_human_use_approval",
    "no_readiness_approval",
    "no_release_approval",
    "no_deployment_approval",
    "no_public_use_approval",
    "no_production_approval",
    "no_provider_trust",
    "no_action_authorization",
    "no_replay_repair",
    "no_recovery_promotion",
  ];
}

export function trialSessionEvidenceAbsenceMarkerSummary(): readonly string[] {
  return [
    "release artifact absent",
    "deployment artifact absent",
    "readiness approval absent",
    "public use approval absent",
    "production use approval absent",
    "provider trust absent",
    "action authorization absent",
    "replay repair absent",
    "recovery promotion absent",
    "default persistence absent",
    "automatic save absent",
    "background persistence absent",
    "network sync absent",
  ];
}

export function initialTrialSessionEvidenceProjection(): TrialSessionEvidenceProjection {
  return {
    status: "not_captured",
    evidenceId: null,
    evidenceVersion: "trial-session-evidence-v1",
    evidenceClassification: "trial_session_evidence_only",
    productionClassification: "non_production",
    distributionClassification: "local_only_non_public",
    authorityClassification: "non_authoritative_evidence",
    trialPackageLinkage: "trial package linkage not captured",
    runbookLinkage: "runbook linkage not captured",
    failureDrillLinkage: "failure drill linkage not captured",
    includedEvidenceSummary: [],
    workflowSnapshotSummary: "workflow snapshot not captured",
    localCandidateMaterializationSnapshot:
      "local candidate materialization snapshot not captured",
    replayStatusSnapshot: "replay/status snapshot not captured",
    evidenceExportSessionPackageRestoreSnapshot:
      "evidence export/session package/restore snapshot not captured",
    stopConditionSnapshot: [],
    escalationSnapshot: [],
    failureCategorySnapshot: [],
    absenceMarkerSummary: trialSessionEvidenceAbsenceMarkerSummary(),
    validationStatus: "not_validated",
    validationErrors: [],
    readBackValidationStatus: null,
    boundaryStatus: trialSessionEvidenceBoundaryStatuses(),
    localOnlyNonAuthoritativeNote:
      "Trial session evidence is local-only, non-public, and non-authoritative.",
    localPreparationOnlyNote:
      "Evidence capture records local trial-preparation state only.",
    noTrialApprovalNote:
      "Evidence capture does not start or approve a controlled trial.",
    noReleaseDeploymentReadinessNote:
      "This evidence is not release, deployment, readiness, public-use, or production-use evidence.",
    readBackValidationNote:
      "Read-back validation checks evidence structure only.",
  };
}

export type TrialReplayRestoreVerificationStatus =
  | "not_verified"
  | "trial_replay_restore_verification_projected"
  | "verification_passed"
  | "verification_rejected"
  | "verification_input_missing"
  | "invalid_verification_input";

export type TrialReplayRestoreVerificationMismatch =
  | "missing_trial_package_read_back"
  | "missing_trial_session_evidence_read_back"
  | "trial_package_read_back_invalid"
  | "trial_session_evidence_read_back_invalid"
  | "package_id_mismatch"
  | "package_version_mismatch"
  | "package_classification_mismatch"
  | "production_classification_mismatch"
  | "distribution_classification_mismatch"
  | "authority_classification_mismatch"
  | "workflow_status_mismatch"
  | "local_candidate_materialization_status_mismatch"
  | "provider_output_pipeline_status_mismatch"
  | "operator_decision_status_mismatch"
  | "replay_status_snapshot_mismatch"
  | "restore_history_snapshot_mismatch"
  | "stop_condition_snapshot_mismatch"
  | "escalation_snapshot_mismatch"
  | "failure_category_snapshot_mismatch"
  | "readiness_claim_detected"
  | "release_claim_detected"
  | "deployment_claim_detected"
  | "public_use_claim_detected"
  | "production_use_claim_detected"
  | "provider_trust_claim_detected"
  | "action_authorization_claim_detected"
  | "replay_repair_claim_detected"
  | "recovery_promotion_claim_detected";

export type TrialReplayRestoreVerificationBoundaryStatus =
  | "local_verification_only"
  | "non_public_verification"
  | "verification_not_authority"
  | "no_trial_execution"
  | "no_controlled_human_use_approval"
  | "no_readiness_approval"
  | "no_release_approval"
  | "no_deployment_approval"
  | "no_public_use_approval"
  | "no_production_approval"
  | "no_provider_trust"
  | "no_action_authorization"
  | "no_replay_repair"
  | "no_recovery_promotion";

export type TrialReplayRestoreVerificationComparisonSummary = Readonly<{
  packageReadBackStatus: string;
  evidenceReadBackStatus: string;
  packageEvidenceLinkageStatus: string;
  workflowLinkageStatus: string;
  replayStatusComparison: string;
  restoreHistoryComparison: string;
  stopConditionComparison: string;
  escalationFailureComparison: string;
}>;

export type TrialReplayRestoreVerificationProjection = Readonly<{
  status: TrialReplayRestoreVerificationStatus;
  verificationId: string | null;
  comparisonSummary: TrialReplayRestoreVerificationComparisonSummary;
  matchedFields: readonly string[];
  mismatches: readonly TrialReplayRestoreVerificationMismatch[];
  missingInputs: readonly string[];
  boundaryStatus: readonly TrialReplayRestoreVerificationBoundaryStatus[];
  localOnlyNonAuthorityNote: string;
  comparisonScopeNote: "Verification compares trial artifacts and restore/replay projections.";
  noReplayRepairNote: "Verification does not repair replay.";
  noRecoveryPromotionNote: "Verification does not promote recovery.";
  noApprovalNote: "Verification does not approve controlled human use, readiness, release, deployment, public use, or production use.";
  noActionExecutionNote: "Verification does not execute actions.";
}>;

export function initialTrialReplayRestoreVerificationProjection(): TrialReplayRestoreVerificationProjection {
  return {
    status: "not_verified",
    verificationId: null,
    comparisonSummary: {
      packageReadBackStatus: "not_verified",
      evidenceReadBackStatus: "not_verified",
      packageEvidenceLinkageStatus: "not_verified",
      workflowLinkageStatus: "not_verified",
      replayStatusComparison: "not_verified",
      restoreHistoryComparison: "not_verified",
      stopConditionComparison: "not_verified",
      escalationFailureComparison: "not_verified",
    },
    matchedFields: [],
    mismatches: [],
    missingInputs: [],
    boundaryStatus: [
      "local_verification_only",
      "non_public_verification",
      "verification_not_authority",
      "no_trial_execution",
      "no_controlled_human_use_approval",
      "no_readiness_approval",
      "no_release_approval",
      "no_deployment_approval",
      "no_public_use_approval",
      "no_production_approval",
      "no_provider_trust",
      "no_action_authorization",
      "no_replay_repair",
      "no_recovery_promotion",
    ],
    localOnlyNonAuthorityNote:
      "Trial replay and restore verification is local-only, non-public, and non-authoritative.",
    comparisonScopeNote:
      "Verification compares trial artifacts and restore/replay projections.",
    noReplayRepairNote: "Verification does not repair replay.",
    noRecoveryPromotionNote: "Verification does not promote recovery.",
    noApprovalNote:
      "Verification does not approve controlled human use, readiness, release, deployment, public use, or production use.",
    noActionExecutionNote: "Verification does not execute actions.",
  };
}

export type ControlledInternalTrialRunStatus =
  | "not_started"
  | "preconditions_missing"
  | "ready_for_bounded_local_trial_run"
  | "trial_run_started"
  | "trial_run_in_progress"
  | "trial_run_blocked"
  | "trial_run_completed_locally"
  | "trial_run_rejected"
  | "invalid_trial_run_request";

export type ControlledInternalTrialRunStep =
  | "verify_trial_package"
  | "verify_runbook"
  | "verify_failure_drill"
  | "verify_trial_session_evidence"
  | "verify_replay_restore"
  | "verify_complete_local_workflow"
  | "observe_stop_conditions"
  | "record_manual_operator_step"
  | "project_trial_run_summary"
  | "project_trial_evidence_linkage"
  | "close_local_trial_run";

export type ControlledInternalTrialRunStepStatus =
  | "not_started"
  | "available"
  | "completed"
  | "blocked"
  | "rejected"
  | "manual_action_required"
  | "not_applicable";

export type ControlledInternalTrialRunError =
  | "trial_package_missing"
  | "trial_package_invalid"
  | "runbook_missing"
  | "runbook_blocked"
  | "failure_drill_missing"
  | "trial_session_evidence_missing"
  | "trial_session_evidence_invalid"
  | "replay_restore_verification_missing"
  | "replay_restore_verification_rejected"
  | "replay_restore_verification_not_passed"
  | "complete_local_workflow_missing"
  | "complete_local_workflow_blocked"
  | "complete_local_workflow_rejected"
  | "stop_condition_observed"
  | "manual_operator_step_missing"
  | "readiness_claim_rejected"
  | "release_claim_rejected"
  | "deployment_claim_rejected"
  | "public_use_claim_rejected"
  | "production_use_claim_rejected"
  | "provider_trust_claim_rejected"
  | "action_authorization_claim_rejected"
  | "replay_repair_claim_rejected"
  | "recovery_promotion_claim_rejected"
  | "signing_claim_rejected"
  | "publishing_claim_rejected"
  | "release_artifact_claim_rejected";

export type ControlledInternalTrialManualStepStatus =
  | "not_started"
  | "manual_action_required"
  | "recorded"
  | "manual_operator_step_missing";

export type ControlledInternalTrialExecutionBoundaryStatus =
  | "controlled_internal_trial_harness_only"
  | "local_trial_execution_only"
  | "non_public_trial_execution"
  | "no_controlled_human_use_approval"
  | "no_readiness_approval"
  | "no_release_approval"
  | "no_deployment_approval"
  | "no_public_use_approval"
  | "no_production_approval"
  | "no_provider_trust"
  | "no_action_authorization"
  | "no_replay_repair"
  | "no_recovery_promotion"
  | "no_stop_condition_automation"
  | "no_automated_escalation";

export type ControlledInternalTrialExecutionRequest = Readonly<{
  operatorStepRecorded?: boolean;
  stopConditionObserved?: boolean;
  claimsReadiness?: boolean;
  claimsRelease?: boolean;
  claimsDeployment?: boolean;
  claimsPublicUse?: boolean;
  claimsProductionUse?: boolean;
  claimsProviderTrust?: boolean;
  claimsActionAuthorization?: boolean;
  claimsReplayRepair?: boolean;
  claimsRecoveryPromotion?: boolean;
  claimsSigning?: boolean;
  claimsPublishing?: boolean;
  claimsReleaseArtifact?: boolean;
}>;

export type ControlledInternalTrialRunStepProjection = Readonly<{
  step: ControlledInternalTrialRunStep;
  status: ControlledInternalTrialRunStepStatus;
  summary: string;
}>;

export type ControlledInternalTrialStopObservation = Readonly<{
  status: string;
  observed: boolean;
  markers: readonly string[];
  enforcementAutomated: false;
}>;

export type ControlledInternalTrialEvidenceLinkage = Readonly<{
  trialPackage: string;
  runbook: string;
  failureDrill: string;
  trialSessionEvidence: string;
  replayRestoreVerification: string;
  localWorkflow: string;
}>;

export type ControlledInternalTrialExecutionCapabilitySurface = Readonly<{
  localOnly: true;
  nonPublic: true;
  approvesControlledHumanUse: false;
  approvesReadiness: false;
  approvesRelease: false;
  approvesDeployment: false;
  approvesPublicUse: false;
  approvesProduction: false;
  trustsProviderOutput: false;
  authorizesActions: false;
  repairsReplay: false;
  promotesRecovery: false;
  automatesStopConditions: false;
  automatesEscalation: false;
}>;

export type ControlledInternalTrialRunProjection = Readonly<{
  runId: string;
  status: ControlledInternalTrialRunStatus;
  currentStep: ControlledInternalTrialRunStep | null;
  nextStep: ControlledInternalTrialRunStep | null;
  steps: readonly ControlledInternalTrialRunStepProjection[];
  currentBlocker: ControlledInternalTrialRunError | null;
  rejectionReasons: readonly ControlledInternalTrialRunError[];
  stopConditionObservation: ControlledInternalTrialStopObservation;
  manualOperatorStepStatus: ControlledInternalTrialManualStepStatus;
  evidenceLinkage: ControlledInternalTrialEvidenceLinkage;
  summary: string;
}>;

export type ControlledInternalTrialExecutionProjection = Readonly<{
  status: ControlledInternalTrialRunStatus;
  activeRun: ControlledInternalTrialRunProjection | null;
  lastRejectedRun: ControlledInternalTrialRunProjection | null;
  preconditionStatus: readonly string[];
  currentBlocker: ControlledInternalTrialRunError | null;
  rejectionReasons: readonly ControlledInternalTrialRunError[];
  evidenceLinkage: ControlledInternalTrialEvidenceLinkage;
  boundaryStatuses: readonly ControlledInternalTrialExecutionBoundaryStatus[];
  capabilitySurface: ControlledInternalTrialExecutionCapabilitySurface;
  localOnlyNonPublicNote: "Controlled internal trial execution harness is local-only and non-public.";
  noControlledHumanUseNote: "The harness does not approve controlled human use.";
  noReadinessReleaseDeploymentPublicProductionNote: "The harness does not approve readiness, release, deployment, public use, or production use.";
  stopConditionNote: "Stop conditions are observed only; enforcement is not automated in Phase 166.";
  escalationNote: "Escalation is not automated.";
  noActionAuthorizationNote: "No action authorization is granted.";
}>;

function emptyControlledInternalTrialEvidenceLinkage(): ControlledInternalTrialEvidenceLinkage {
  return {
    trialPackage: "trial_package=missing",
    runbook: "runbook=missing",
    failureDrill: "failure_drill=missing",
    trialSessionEvidence: "trial_session_evidence=missing",
    replayRestoreVerification: "replay_restore_verification=missing",
    localWorkflow: "local_workflow=missing",
  };
}

export function initialControlledInternalTrialExecutionProjection(): ControlledInternalTrialExecutionProjection {
  return {
    status: "not_started",
    activeRun: null,
    lastRejectedRun: null,
    preconditionStatus: [],
    currentBlocker: null,
    rejectionReasons: [],
    evidenceLinkage: emptyControlledInternalTrialEvidenceLinkage(),
    boundaryStatuses: [
      "controlled_internal_trial_harness_only",
      "local_trial_execution_only",
      "non_public_trial_execution",
      "no_controlled_human_use_approval",
      "no_readiness_approval",
      "no_release_approval",
      "no_deployment_approval",
      "no_public_use_approval",
      "no_production_approval",
      "no_provider_trust",
      "no_action_authorization",
      "no_replay_repair",
      "no_recovery_promotion",
      "no_stop_condition_automation",
      "no_automated_escalation",
    ],
    capabilitySurface: {
      localOnly: true,
      nonPublic: true,
      approvesControlledHumanUse: false,
      approvesReadiness: false,
      approvesRelease: false,
      approvesDeployment: false,
      approvesPublicUse: false,
      approvesProduction: false,
      trustsProviderOutput: false,
      authorizesActions: false,
      repairsReplay: false,
      promotesRecovery: false,
      automatesStopConditions: false,
      automatesEscalation: false,
    },
    localOnlyNonPublicNote:
      "Controlled internal trial execution harness is local-only and non-public.",
    noControlledHumanUseNote:
      "The harness does not approve controlled human use.",
    noReadinessReleaseDeploymentPublicProductionNote:
      "The harness does not approve readiness, release, deployment, public use, or production use.",
    stopConditionNote:
      "Stop conditions are observed only; enforcement is not automated in Phase 166.",
    escalationNote: "Escalation is not automated.",
    noActionAuthorizationNote: "No action authorization is granted.",
  };
}

export function deriveControlledInternalTrialExecutionProjection(
  state: LocalOperatorShellState,
): ControlledInternalTrialExecutionProjection {
  const errors: ControlledInternalTrialRunError[] = [];
  if (state.controlledInternalTrialPackageProjection.status === "not_packaged")
    errors.push("trial_package_missing");
  if (state.trialOperatorRunbook.status === "not_available")
    errors.push("runbook_missing");
  if (
    ["trial_package_required", "blocked"].includes(
      state.trialOperatorRunbook.status,
    )
  )
    errors.push("runbook_blocked");
  if (
    state.trialFailureDrill.stopConditionDrills.length === 0 &&
    state.trialFailureDrill.categories.length === 0
  )
    errors.push("failure_drill_missing");
  if (state.trialSessionEvidenceProjection.status === "not_captured")
    errors.push("trial_session_evidence_missing");
  if (
    state.trialSessionEvidenceProjection.validationStatus !== "valid" &&
    state.trialSessionEvidenceProjection.readBackValidationStatus !== "valid" &&
    !errors.includes("trial_session_evidence_missing")
  )
    errors.push("trial_session_evidence_invalid");
  if (state.trialReplayRestoreVerification.status === "not_verified")
    errors.push("replay_restore_verification_missing");
  if (
    state.trialReplayRestoreVerification.status ===
    "trial_replay_restore_verification_projected"
  )
    errors.push("replay_restore_verification_not_passed");
  if (
    ["verification_rejected", "invalid_verification_input"].includes(
      state.trialReplayRestoreVerification.status,
    )
  )
    errors.push("replay_restore_verification_rejected");
  if (
    state.completeLocalOperatorWorkflow.status === "not_started" ||
    state.completeLocalOperatorWorkflow.status === "in_progress"
  )
    errors.push("complete_local_workflow_missing");
  if (state.completeLocalOperatorWorkflow.status === "blocked")
    errors.push("complete_local_workflow_blocked");
  if (state.completeLocalOperatorWorkflow.status === "rejected")
    errors.push("complete_local_workflow_rejected");
  return {
    ...initialControlledInternalTrialExecutionProjection(),
    status:
      errors.length === 0
        ? "ready_for_bounded_local_trial_run"
        : "preconditions_missing",
    preconditionStatus: [
      `trial_package=${state.controlledInternalTrialPackageProjection.status}`,
      `runbook=${state.trialOperatorRunbook.status}`,
      `failure_drill=${state.trialFailureDrill.status}`,
      `trial_session_evidence=${state.trialSessionEvidenceProjection.status}`,
      `replay_restore_verification=${state.trialReplayRestoreVerification.status}`,
      `local_workflow=${state.completeLocalOperatorWorkflow.status}`,
    ],
    currentBlocker: errors[0] ?? null,
    rejectionReasons: errors,
    evidenceLinkage: {
      trialPackage: `package=${state.controlledInternalTrialPackageProjection.packageId ?? "none"} status=${state.controlledInternalTrialPackageProjection.status}`,
      runbook: `runbook_status=${state.trialOperatorRunbook.status}`,
      failureDrill: `failure_drill_status=${state.trialFailureDrill.status}`,
      trialSessionEvidence: `evidence=${state.trialSessionEvidenceProjection.evidenceId ?? "none"} status=${state.trialSessionEvidenceProjection.status}`,
      replayRestoreVerification: `verification=${state.trialReplayRestoreVerification.verificationId ?? "none"} status=${state.trialReplayRestoreVerification.status}`,
      localWorkflow: `workflow_status=${state.completeLocalOperatorWorkflow.status}`,
    },
  };
}

export type TrialObservabilityStatus =
  | "not_observed"
  | "observability_projected"
  | "trial_run_observed"
  | "blocked_state_observed"
  | "rejected_state_observed"
  | "stop_condition_observed"
  | "verification_mismatch_observed"
  | "error_report_projected";

export type TrialErrorReportStatus =
  | "no_errors_observed"
  | "errors_observed"
  | "blocked_errors_observed"
  | "rejected_errors_observed"
  | "invalid_observability_input";

export type TrialErrorCategory =
  | "no_trial_run"
  | "trial_precondition_missing"
  | "trial_run_blocked"
  | "trial_run_rejected"
  | "stop_condition_observed"
  | "manual_operator_step_required"
  | "package_missing"
  | "package_validation_failed"
  | "package_read_back_failed"
  | "evidence_missing"
  | "evidence_validation_failed"
  | "evidence_read_back_failed"
  | "replay_restore_verification_missing"
  | "replay_restore_verification_rejected"
  | "replay_status_mismatch"
  | "restore_history_mismatch"
  | "provider_pipeline_blocked"
  | "provider_output_validation_rejected"
  | "workflow_blocked"
  | "workflow_rejected"
  | "materialization_missing"
  | "authority_claim_detected"
  | "readiness_claim_detected"
  | "release_or_deployment_claim_detected"
  | "public_or_production_use_claim_detected"
  | "action_authorization_claim_detected"
  | "replay_repair_or_recovery_promotion_claim_detected";

export type TrialErrorSeverity =
  | "info"
  | "warning"
  | "blocking"
  | "stop_condition"
  | "invalid_input";

export type TrialErrorSource =
  | "controlled_trial_execution_harness"
  | "controlled_internal_trial_package"
  | "trial_session_evidence"
  | "replay_restore_verification"
  | "trial_runbook"
  | "failure_drill"
  | "complete_local_workflow"
  | "provider_output_pipeline"
  | "local_candidate_materialization"
  | "session_restore";

export type TrialObservabilityBoundaryStatus =
  | "local_observability_only"
  | "non_public_observability"
  | "no_production_monitoring"
  | "no_remote_telemetry"
  | "no_network_reporting"
  | "no_background_service"
  | "no_automated_remediation"
  | "no_automated_escalation"
  | "no_stop_condition_automation"
  | "no_action_execution"
  | "no_replay_repair"
  | "no_recovery_promotion"
  | "no_readiness_approval"
  | "no_release_approval"
  | "no_deployment_approval"
  | "no_public_use_approval"
  | "no_production_approval";

export type TrialObservabilityCapabilitySurface = Readonly<{
  localOnly: true;
  nonPublic: true;
  productionMonitoring: false;
  remoteTelemetry: false;
  networkReporting: false;
  backgroundService: false;
  remediates: false;
  escalates: false;
  automatesStopConditions: false;
  executesActions: false;
  repairsReplay: false;
  promotesRecovery: false;
  approvesReadiness: false;
  approvesRelease: false;
  approvesDeployment: false;
  approvesPublicUse: false;
  approvesProduction: false;
}>;

export type TrialBlockedStateSummary = Readonly<{
  status: string;
  currentBlocker: string;
  rejectionReasons: readonly string[];
}>;

export type TrialVerificationMismatchSummary = Readonly<{
  verificationStatus: string;
  mismatches: readonly string[];
  replayStatusComparison: string;
  restoreHistoryComparison: string;
}>;

export type TrialStopConditionObservationSummary = Readonly<{
  status: string;
  observed: boolean;
  markers: readonly string[];
}>;

export type TrialErrorEvidenceLinkage = Readonly<{
  source: TrialErrorSource;
  linkage: string;
}>;

export type TrialErrorDetail = Readonly<{
  category: TrialErrorCategory;
  severity: TrialErrorSeverity;
  source: TrialErrorSource;
  summary: string;
  operatorGuidance: string;
  evidenceLinkage: TrialErrorEvidenceLinkage;
}>;

export type TrialErrorReportProjection = Readonly<{
  status: TrialErrorReportStatus;
  errorCount: number;
  highestSeverity: TrialErrorSeverity;
  details: readonly TrialErrorDetail[];
  categorySummary: readonly TrialErrorCategory[];
  evidenceLinkageSummary: readonly string[];
  localDescriptiveOnlyNote: "Error reporting is local and descriptive only.";
}>;

export type TrialObservabilityProjection = Readonly<{
  status: TrialObservabilityStatus;
  trialRunStatus: string;
  currentTrialStep: string;
  currentBlocker: string;
  stopConditionObservation: TrialStopConditionObservationSummary;
  manualOperatorStepStatus: string;
  packageStatus: string;
  evidenceStatus: string;
  packageReadBackStatus: string;
  evidenceReadBackStatus: string;
  replayRestoreVerificationStatus: string;
  mismatchSummary: TrialVerificationMismatchSummary;
  packageEvidenceReadBackFailureSummary: readonly string[];
  replayStatusComparisonSummary: string;
  restoreHistoryComparisonSummary: string;
  runbookFailureDrillCategorySummary: readonly string[];
  evidenceLinkageSummary: readonly string[];
  blockedStateSummary: TrialBlockedStateSummary;
  boundaryStatuses: readonly TrialObservabilityBoundaryStatus[];
  capabilitySurface: TrialObservabilityCapabilitySurface;
  localOnlyNonPublicNote: "Trial observability is local-only and non-public.";
  noProductionMonitoringNote: "No production monitoring is active.";
  noRemoteTelemetryNote: "No remote telemetry is sent.";
  noBackgroundServiceNote: "No background service is active.";
  noRemediationEscalationStopAutomationNote: "No remediation, escalation, or stop-condition enforcement is automated.";
  noApprovalNote: "Observability does not approve controlled human use, readiness, release, deployment, public use, or production use.";
}>;

export function trialObservabilityBoundaryStatuses(): readonly TrialObservabilityBoundaryStatus[] {
  return [
    "local_observability_only",
    "non_public_observability",
    "no_production_monitoring",
    "no_remote_telemetry",
    "no_network_reporting",
    "no_background_service",
    "no_automated_remediation",
    "no_automated_escalation",
    "no_stop_condition_automation",
    "no_action_execution",
    "no_replay_repair",
    "no_recovery_promotion",
    "no_readiness_approval",
    "no_release_approval",
    "no_deployment_approval",
    "no_public_use_approval",
    "no_production_approval",
  ];
}

function trialObservabilityCapabilitySurface(): TrialObservabilityCapabilitySurface {
  return {
    localOnly: true,
    nonPublic: true,
    productionMonitoring: false,
    remoteTelemetry: false,
    networkReporting: false,
    backgroundService: false,
    remediates: false,
    escalates: false,
    automatesStopConditions: false,
    executesActions: false,
    repairsReplay: false,
    promotesRecovery: false,
    approvesReadiness: false,
    approvesRelease: false,
    approvesDeployment: false,
    approvesPublicUse: false,
    approvesProduction: false,
  };
}

export function initialTrialObservabilityProjection(): TrialObservabilityProjection {
  return {
    status: "not_observed",
    trialRunStatus: "not_started",
    currentTrialStep: "none",
    currentBlocker: "none",
    stopConditionObservation: {
      status: "not_started",
      observed: false,
      markers: [],
    },
    manualOperatorStepStatus: "not_started",
    packageStatus: "not_packaged",
    evidenceStatus: "not_captured",
    packageReadBackStatus: "not_read",
    evidenceReadBackStatus: "not_read",
    replayRestoreVerificationStatus: "not_verified",
    mismatchSummary: {
      verificationStatus: "not_verified",
      mismatches: [],
      replayStatusComparison: "not_verified",
      restoreHistoryComparison: "not_verified",
    },
    packageEvidenceReadBackFailureSummary: [],
    replayStatusComparisonSummary: "not_verified",
    restoreHistoryComparisonSummary: "not_verified",
    runbookFailureDrillCategorySummary: [],
    evidenceLinkageSummary: [],
    blockedStateSummary: {
      status: "not_observed",
      currentBlocker: "none",
      rejectionReasons: [],
    },
    boundaryStatuses: trialObservabilityBoundaryStatuses(),
    capabilitySurface: trialObservabilityCapabilitySurface(),
    localOnlyNonPublicNote: "Trial observability is local-only and non-public.",
    noProductionMonitoringNote: "No production monitoring is active.",
    noRemoteTelemetryNote: "No remote telemetry is sent.",
    noBackgroundServiceNote: "No background service is active.",
    noRemediationEscalationStopAutomationNote:
      "No remediation, escalation, or stop-condition enforcement is automated.",
    noApprovalNote:
      "Observability does not approve controlled human use, readiness, release, deployment, public use, or production use.",
  };
}

function trialErrorSeverity(category: TrialErrorCategory): TrialErrorSeverity {
  if (category === "stop_condition_observed") return "stop_condition";
  if (
    [
      "authority_claim_detected",
      "readiness_claim_detected",
      "release_or_deployment_claim_detected",
      "public_or_production_use_claim_detected",
      "action_authorization_claim_detected",
      "replay_repair_or_recovery_promotion_claim_detected",
    ].includes(category)
  )
    return "invalid_input";
  if (
    [
      "no_trial_run",
      "manual_operator_step_required",
      "trial_precondition_missing",
    ].includes(category)
  )
    return "warning";
  return "blocking";
}

function trialErrorGuidance(category: TrialErrorCategory): string {
  if (category === "no_trial_run")
    return "Inspect local trial preconditions before starting a bounded local trial run.";
  if (category === "manual_operator_step_required")
    return "Record the manual operator step locally before closing the trial run.";
  if (category === "stop_condition_observed")
    return "Review the observed stop-condition marker locally; no enforcement is automated.";
  return "Review the linked local trial surface and preserve the diagnostic evidence.";
}

function trialErrorDetail(
  category: TrialErrorCategory,
  source: TrialErrorSource,
  summary: string,
  linkage: string,
): TrialErrorDetail {
  return {
    category,
    severity: trialErrorSeverity(category),
    source,
    summary,
    operatorGuidance: trialErrorGuidance(category),
    evidenceLinkage: { source, linkage },
  };
}

export function initialTrialErrorReportProjection(): TrialErrorReportProjection {
  const detail = trialErrorDetail(
    "no_trial_run",
    "controlled_trial_execution_harness",
    "No controlled internal trial run has been observed.",
    "controlled_internal_trial_execution=not_started",
  );
  return {
    status: "errors_observed",
    errorCount: 1,
    highestSeverity: "warning",
    details: [detail],
    categorySummary: [detail.category],
    evidenceLinkageSummary: [detail.evidenceLinkage.linkage],
    localDescriptiveOnlyNote: "Error reporting is local and descriptive only.",
  };
}

export function deriveTrialErrorReportProjection(
  state: LocalOperatorShellState,
): TrialErrorReportProjection {
  const details: TrialErrorDetail[] = [];
  const execution = state.controlledInternalTrialExecution;
  const displayedRun = execution.activeRun ?? execution.lastRejectedRun;
  if (!displayedRun)
    details.push(
      trialErrorDetail(
        "no_trial_run",
        "controlled_trial_execution_harness",
        "No controlled internal trial run has been observed.",
        "controlled_internal_trial_execution=not_started",
      ),
    );
  if (execution.status === "preconditions_missing")
    details.push(
      trialErrorDetail(
        "trial_precondition_missing",
        "controlled_trial_execution_harness",
        "Trial preconditions are missing.",
        "controlled_internal_trial_execution=preconditions_missing",
      ),
    );
  if (execution.status === "trial_run_blocked")
    details.push(
      trialErrorDetail(
        "trial_run_blocked",
        "controlled_trial_execution_harness",
        "Controlled internal trial run is blocked.",
        "controlled_internal_trial_execution=trial_run_blocked",
      ),
    );
  if (
    ["trial_run_rejected", "invalid_trial_run_request"].includes(
      execution.status,
    )
  )
    details.push(
      trialErrorDetail(
        "trial_run_rejected",
        "controlled_trial_execution_harness",
        "Controlled internal trial run is rejected.",
        "controlled_internal_trial_execution=trial_run_rejected",
      ),
    );
  if (displayedRun?.stopConditionObservation.observed)
    details.push(
      trialErrorDetail(
        "stop_condition_observed",
        "controlled_trial_execution_harness",
        "Stop condition was observed in the local trial run.",
        "stop_condition_observation=observed",
      ),
    );
  if (
    ["manual_action_required", "manual_operator_step_missing"].includes(
      displayedRun?.manualOperatorStepStatus ?? "",
    )
  )
    details.push(
      trialErrorDetail(
        "manual_operator_step_required",
        "controlled_trial_execution_harness",
        "Manual operator step is required or missing.",
        "manual_operator_step_status=required",
      ),
    );
  if (state.controlledInternalTrialPackageProjection.status === "not_packaged")
    details.push(
      trialErrorDetail(
        "package_missing",
        "controlled_internal_trial_package",
        "Controlled internal trial package is missing.",
        "trial_package=missing",
      ),
    );
  if (
    state.controlledInternalTrialPackageProjection.status !== "not_packaged" &&
    state.controlledInternalTrialPackageProjection.validationStatus !== "valid"
  )
    details.push(
      trialErrorDetail(
        "package_validation_failed",
        "controlled_internal_trial_package",
        "Controlled internal trial package validation failed.",
        "trial_package_validation=failed",
      ),
    );
  if (
    state.controlledInternalTrialPackageProjection.readBackValidationStatus !==
      null &&
    state.controlledInternalTrialPackageProjection.readBackValidationStatus !==
      "valid"
  )
    details.push(
      trialErrorDetail(
        "package_read_back_failed",
        "controlled_internal_trial_package",
        "Controlled internal trial package read-back failed.",
        "trial_package_read_back=failed",
      ),
    );
  if (state.trialSessionEvidenceProjection.status === "not_captured")
    details.push(
      trialErrorDetail(
        "evidence_missing",
        "trial_session_evidence",
        "Trial session evidence is missing.",
        "trial_session_evidence=missing",
      ),
    );
  if (
    state.trialSessionEvidenceProjection.status !== "not_captured" &&
    state.trialSessionEvidenceProjection.validationStatus !== "valid"
  )
    details.push(
      trialErrorDetail(
        "evidence_validation_failed",
        "trial_session_evidence",
        "Trial session evidence validation failed.",
        "trial_session_evidence_validation=failed",
      ),
    );
  if (
    state.trialSessionEvidenceProjection.readBackValidationStatus !== null &&
    state.trialSessionEvidenceProjection.readBackValidationStatus !== "valid"
  )
    details.push(
      trialErrorDetail(
        "evidence_read_back_failed",
        "trial_session_evidence",
        "Trial session evidence read-back failed.",
        "trial_session_evidence_read_back=failed",
      ),
    );
  if (
    ["not_verified", "verification_input_missing"].includes(
      state.trialReplayRestoreVerification.status,
    )
  )
    details.push(
      trialErrorDetail(
        "replay_restore_verification_missing",
        "replay_restore_verification",
        "Replay/restore verification is missing.",
        "replay_restore_verification=missing",
      ),
    );
  if (
    ["verification_rejected", "invalid_verification_input"].includes(
      state.trialReplayRestoreVerification.status,
    )
  )
    details.push(
      trialErrorDetail(
        "replay_restore_verification_rejected",
        "replay_restore_verification",
        "Replay/restore verification is rejected.",
        "replay_restore_verification=rejected",
      ),
    );
  if (
    state.trialReplayRestoreVerification.mismatches.includes(
      "replay_status_snapshot_mismatch",
    )
  )
    details.push(
      trialErrorDetail(
        "replay_status_mismatch",
        "replay_restore_verification",
        "Replay/status comparison mismatch observed.",
        "replay_status_comparison=mismatch",
      ),
    );
  if (
    state.trialReplayRestoreVerification.mismatches.includes(
      "restore_history_snapshot_mismatch",
    )
  )
    details.push(
      trialErrorDetail(
        "restore_history_mismatch",
        "replay_restore_verification",
        "Restore/history comparison mismatch observed.",
        "restore_history_comparison=mismatch",
      ),
    );
  if (state.localProviderOutputPipeline.status === "rejected")
    details.push(
      trialErrorDetail(
        "provider_pipeline_blocked",
        "provider_output_pipeline",
        "Provider output pipeline is blocked or rejected.",
        "provider_output_pipeline=rejected",
      ),
    );
  if (state.providerOutputValidation.status === "rejected")
    details.push(
      trialErrorDetail(
        "provider_output_validation_rejected",
        "provider_output_pipeline",
        "Provider output validation is rejected.",
        "provider_output_validation=rejected",
      ),
    );
  if (state.completeLocalOperatorWorkflow.status === "blocked")
    details.push(
      trialErrorDetail(
        "workflow_blocked",
        "complete_local_workflow",
        "Complete local workflow is blocked.",
        "complete_local_workflow=blocked",
      ),
    );
  if (state.completeLocalOperatorWorkflow.status === "rejected")
    details.push(
      trialErrorDetail(
        "workflow_rejected",
        "complete_local_workflow",
        "Complete local workflow is rejected.",
        "complete_local_workflow=rejected",
      ),
    );
  if (state.localCandidateOutput.status === "not_materialized")
    details.push(
      trialErrorDetail(
        "materialization_missing",
        "local_candidate_materialization",
        "Local candidate materialization is missing.",
        "local_candidate_materialization=missing",
      ),
    );
  for (const reason of execution.rejectionReasons) {
    if (reason === "readiness_claim_rejected")
      details.push(
        trialErrorDetail(
          "readiness_claim_detected",
          "controlled_trial_execution_harness",
          "Readiness claim detected and rejected.",
          "claim=readiness",
        ),
      );
    if (
      [
        "release_claim_rejected",
        "deployment_claim_rejected",
        "signing_claim_rejected",
        "publishing_claim_rejected",
        "release_artifact_claim_rejected",
      ].includes(reason)
    )
      details.push(
        trialErrorDetail(
          "release_or_deployment_claim_detected",
          "controlled_trial_execution_harness",
          "Release or deployment claim detected and rejected.",
          "claim=release_or_deployment",
        ),
      );
    if (
      ["public_use_claim_rejected", "production_use_claim_rejected"].includes(
        reason,
      )
    )
      details.push(
        trialErrorDetail(
          "public_or_production_use_claim_detected",
          "controlled_trial_execution_harness",
          "Public or production use claim detected and rejected.",
          "claim=public_or_production_use",
        ),
      );
    if (reason === "provider_trust_claim_rejected")
      details.push(
        trialErrorDetail(
          "authority_claim_detected",
          "controlled_trial_execution_harness",
          "Provider trust claim detected and rejected.",
          "claim=provider_trust",
        ),
      );
    if (reason === "action_authorization_claim_rejected")
      details.push(
        trialErrorDetail(
          "action_authorization_claim_detected",
          "controlled_trial_execution_harness",
          "Action authorization claim detected and rejected.",
          "claim=action_authorization",
        ),
      );
    if (
      [
        "replay_repair_claim_rejected",
        "recovery_promotion_claim_rejected",
      ].includes(reason)
    )
      details.push(
        trialErrorDetail(
          "replay_repair_or_recovery_promotion_claim_detected",
          "controlled_trial_execution_harness",
          "Replay repair or recovery promotion claim detected and rejected.",
          "claim=replay_repair_or_recovery_promotion",
        ),
      );
  }
  details.sort((left, right) => left.category.localeCompare(right.category));
  const highestSeverity: TrialErrorSeverity = details.some(
    (detail) => detail.severity === "invalid_input",
  )
    ? "invalid_input"
    : details.some((detail) => detail.severity === "stop_condition")
      ? "stop_condition"
      : details.some((detail) => detail.severity === "blocking")
        ? "blocking"
        : details.some((detail) => detail.severity === "warning")
          ? "warning"
          : "info";
  const status: TrialErrorReportStatus =
    details.length === 0
      ? "no_errors_observed"
      : execution.status === "trial_run_blocked"
        ? "blocked_errors_observed"
        : ["trial_run_rejected", "invalid_trial_run_request"].includes(
              execution.status,
            )
          ? "rejected_errors_observed"
          : highestSeverity === "invalid_input"
            ? "invalid_observability_input"
            : "errors_observed";
  return {
    status,
    errorCount: details.length,
    highestSeverity,
    details,
    categorySummary: details.map((detail) => detail.category),
    evidenceLinkageSummary: details.map(
      (detail) => detail.evidenceLinkage.linkage,
    ),
    localDescriptiveOnlyNote: "Error reporting is local and descriptive only.",
  };
}

export function deriveTrialObservabilityProjection(
  state: LocalOperatorShellState,
): TrialObservabilityProjection {
  const base = initialTrialObservabilityProjection();
  const execution = state.controlledInternalTrialExecution;
  const displayedRun = execution.activeRun ?? execution.lastRejectedRun;
  const mismatches = state.trialReplayRestoreVerification.mismatches;
  const stopObserved = displayedRun?.stopConditionObservation.observed === true;
  const status: TrialObservabilityStatus = stopObserved
    ? "stop_condition_observed"
    : execution.status === "trial_run_blocked"
      ? "blocked_state_observed"
      : ["trial_run_rejected", "invalid_trial_run_request"].includes(
            execution.status,
          )
        ? "rejected_state_observed"
        : mismatches.length > 0
          ? "verification_mismatch_observed"
          : displayedRun
            ? "trial_run_observed"
            : "observability_projected";
  const linkage = displayedRun?.evidenceLinkage ?? execution.evidenceLinkage;
  return {
    ...base,
    status,
    trialRunStatus: displayedRun?.status ?? execution.status,
    currentTrialStep: displayedRun?.currentStep ?? "none",
    currentBlocker:
      displayedRun?.currentBlocker ?? execution.currentBlocker ?? "none",
    stopConditionObservation: {
      status: displayedRun?.stopConditionObservation.status ?? "not_started",
      observed: displayedRun?.stopConditionObservation.observed ?? false,
      markers: displayedRun?.stopConditionObservation.markers ?? [],
    },
    manualOperatorStepStatus:
      displayedRun?.manualOperatorStepStatus ?? "not_started",
    packageStatus: state.controlledInternalTrialPackageProjection.status,
    evidenceStatus: state.trialSessionEvidenceProjection.status,
    packageReadBackStatus:
      state.controlledInternalTrialPackageProjection.readBackValidationStatus ??
      "not_read",
    evidenceReadBackStatus:
      state.trialSessionEvidenceProjection.readBackValidationStatus ??
      "not_read",
    replayRestoreVerificationStatus:
      state.trialReplayRestoreVerification.status,
    mismatchSummary: {
      verificationStatus: state.trialReplayRestoreVerification.status,
      mismatches,
      replayStatusComparison:
        state.trialReplayRestoreVerification.comparisonSummary
          .replayStatusComparison,
      restoreHistoryComparison:
        state.trialReplayRestoreVerification.comparisonSummary
          .restoreHistoryComparison,
    },
    packageEvidenceReadBackFailureSummary: mismatches.filter((mismatch) =>
      [
        "missing_trial_package_read_back",
        "missing_trial_session_evidence_read_back",
        "trial_package_read_back_invalid",
        "trial_session_evidence_read_back_invalid",
      ].includes(mismatch),
    ),
    replayStatusComparisonSummary:
      state.trialReplayRestoreVerification.comparisonSummary
        .replayStatusComparison,
    restoreHistoryComparisonSummary:
      state.trialReplayRestoreVerification.comparisonSummary
        .restoreHistoryComparison,
    runbookFailureDrillCategorySummary: state.trialFailureDrill.categories.map(
      (entry) => `${entry.category}=${entry.severity}`,
    ),
    evidenceLinkageSummary: [
      linkage.trialPackage,
      linkage.runbook,
      linkage.failureDrill,
      linkage.trialSessionEvidence,
      linkage.replayRestoreVerification,
      linkage.localWorkflow,
    ],
    blockedStateSummary: {
      status: [
        "trial_run_blocked",
        "trial_run_rejected",
        "invalid_trial_run_request",
      ].includes(execution.status)
        ? "observed"
        : "not_observed",
      currentBlocker:
        displayedRun?.currentBlocker ?? execution.currentBlocker ?? "none",
      rejectionReasons: execution.rejectionReasons,
    },
  };
}

export type TrialEvidenceReviewStatus =
  | "not_reviewed"
  | "review_projected"
  | "review_with_findings"
  | "review_blocked"
  | "review_rejected"
  | "hardening_candidates_projected"
  | "invalid_review_input";

export type TrialEvidenceReviewFindingCategory =
  | "trial_package"
  | "trial_execution"
  | "trial_session_evidence"
  | "replay_restore_verification"
  | "trial_observability"
  | "error_reporting"
  | "stop_condition"
  | "escalation_guidance"
  | "workflow_blocker"
  | "provider_pipeline"
  | "candidate_materialization"
  | "restore_history"
  | "replay_status"
  | "local_session_package"
  | "operator_usability"
  | "documentation_needed"
  | "user_help_needed"
  | "local_beta_hardening";

export type TrialEvidenceReviewFindingSeverity =
  | "info"
  | "warning"
  | "blocking"
  | "stop_condition"
  | "hardening_required"
  | "invalid_input";
export type TrialEvidenceReviewFindingDisposition =
  | "unresolved"
  | "requires_phase_169_hardening"
  | "requires_operator_review"
  | "requires_security_reviewer_review"
  | "requires_release_steward_review"
  | "requires_trial_coordinator_review"
  | "deferred"
  | "not_applicable";
export type TrialEvidenceReviewSource =
  | "controlled_internal_trial_package"
  | "trial_operator_runbook"
  | "trial_failure_drill"
  | "trial_session_evidence"
  | "replay_restore_verification"
  | "controlled_trial_execution_harness"
  | "trial_observability"
  | "trial_error_report"
  | "complete_local_workflow"
  | "local_candidate_materialization"
  | "session_package"
  | "restore_history";
export type TrialEvidenceReviewBoundaryStatus =
  | "local_evidence_review_only"
  | "non_public_review"
  | "review_not_authority"
  | "no_controlled_human_use_approval"
  | "no_readiness_approval"
  | "no_release_approval"
  | "no_deployment_approval"
  | "no_public_use_approval"
  | "no_production_approval"
  | "no_provider_trust"
  | "no_action_execution"
  | "no_automated_remediation"
  | "no_automated_escalation"
  | "no_stop_condition_automation"
  | "no_replay_repair"
  | "no_recovery_promotion";

export type TrialEvidenceReviewEvidenceLinkage = Readonly<{
  source: TrialEvidenceReviewSource;
  surface: string;
  summary: string;
}>;
export type TrialEvidenceReviewFinding = Readonly<{
  findingId: string;
  category: TrialEvidenceReviewFindingCategory;
  severity: TrialEvidenceReviewFindingSeverity;
  disposition: TrialEvidenceReviewFindingDisposition;
  source: TrialEvidenceReviewSource;
  summary: string;
  evidenceLinkage: TrialEvidenceReviewEvidenceLinkage;
}>;
export type TrialEvidenceReviewHardeningCandidate = Readonly<{
  candidateId: string;
  sourceFindingId: string;
  targetSurface: string;
  localBetaScope: "local_beta_only_phase_169_input";
  summary: string;
}>;
export type TrialEvidenceReviewCapabilitySurface = Readonly<{
  localOnly: true;
  nonPublic: true;
  authoritative: false;
  approvesControlledHumanUse: false;
  approvesReadiness: false;
  approvesRelease: false;
  approvesDeployment: false;
  approvesPublicUse: false;
  approvesProduction: false;
  trustsProviderOutput: false;
  executesActions: false;
  automatesRemediation: false;
  automatesEscalation: false;
  automatesStopConditions: false;
  repairsReplay: false;
  promotesRecovery: false;
}>;
export type TrialEvidenceReviewBlockerSummary = Readonly<{
  currentBlocker: string;
  unresolvedBlockerCount: number;
  blockers: readonly string[];
}>;
export type TrialEvidenceReviewMismatchSummary = Readonly<{
  mismatchCount: number;
  errorCategoryCount: number;
  replayStatusComparison: string;
  restoreHistoryComparison: string;
  packageEvidenceReadBackFailures: readonly string[];
}>;
export type TrialEvidenceReviewProjection = Readonly<{
  status: TrialEvidenceReviewStatus;
  reviewId: string;
  controlledTrialPackageStatus: string;
  trialExecutionStatus: string;
  trialSessionEvidenceStatus: string;
  replayRestoreVerificationStatus: string;
  observabilityStatus: string;
  errorReportStatus: string;
  stopConditionObservation: string;
  escalationGuidanceSummary: readonly string[];
  evidenceLinkage: readonly TrialEvidenceReviewEvidenceLinkage[];
  findings: readonly TrialEvidenceReviewFinding[];
  unresolvedBlockers: TrialEvidenceReviewBlockerSummary;
  mismatchErrorSummary: TrialEvidenceReviewMismatchSummary;
  unresolvedFindingCount: number;
  hardeningCandidates: readonly TrialEvidenceReviewHardeningCandidate[];
  boundaryStatuses: readonly TrialEvidenceReviewBoundaryStatus[];
  capabilitySurface: TrialEvidenceReviewCapabilitySurface;
  localOnlyNonPublicNote: string;
  noAuthorityNote: string;
  noAutomationNote: string;
  noReplayRepairNote: string;
  hardeningCandidateNote: string;
}>;

export function trialEvidenceReviewBoundaryStatuses(): readonly TrialEvidenceReviewBoundaryStatus[] {
  return [
    "local_evidence_review_only",
    "non_public_review",
    "review_not_authority",
    "no_controlled_human_use_approval",
    "no_readiness_approval",
    "no_release_approval",
    "no_deployment_approval",
    "no_public_use_approval",
    "no_production_approval",
    "no_provider_trust",
    "no_action_execution",
    "no_automated_remediation",
    "no_automated_escalation",
    "no_stop_condition_automation",
    "no_replay_repair",
    "no_recovery_promotion",
  ];
}

function trialEvidenceReviewCapabilitySurface(): TrialEvidenceReviewCapabilitySurface {
  return {
    localOnly: true,
    nonPublic: true,
    authoritative: false,
    approvesControlledHumanUse: false,
    approvesReadiness: false,
    approvesRelease: false,
    approvesDeployment: false,
    approvesPublicUse: false,
    approvesProduction: false,
    trustsProviderOutput: false,
    executesActions: false,
    automatesRemediation: false,
    automatesEscalation: false,
    automatesStopConditions: false,
    repairsReplay: false,
    promotesRecovery: false,
  };
}

export function initialTrialEvidenceReviewProjection(): TrialEvidenceReviewProjection {
  return {
    status: "not_reviewed",
    reviewId: "trial-evidence-review-initial",
    controlledTrialPackageStatus: "not_packaged",
    trialExecutionStatus: "not_started",
    trialSessionEvidenceStatus: "not_captured",
    replayRestoreVerificationStatus: "not_verified",
    observabilityStatus: "not_observed",
    errorReportStatus: "no_errors_observed",
    stopConditionObservation: "not_observed",
    escalationGuidanceSummary: [],
    evidenceLinkage: [],
    findings: [],
    unresolvedBlockers: {
      currentBlocker: "not_reviewed",
      unresolvedBlockerCount: 0,
      blockers: [],
    },
    mismatchErrorSummary: {
      mismatchCount: 0,
      errorCategoryCount: 0,
      replayStatusComparison: "not_reviewed",
      restoreHistoryComparison: "not_reviewed",
      packageEvidenceReadBackFailures: [],
    },
    unresolvedFindingCount: 0,
    hardeningCandidates: [],
    boundaryStatuses: trialEvidenceReviewBoundaryStatuses(),
    capabilitySurface: trialEvidenceReviewCapabilitySurface(),
    localOnlyNonPublicNote:
      "Trial evidence review is local-only and non-public.",
    noAuthorityNote:
      "Review findings are evidence, not approval. Review does not approve controlled human use, readiness, release, deployment, public use, or production use.",
    noAutomationNote:
      "Review does not automate remediation, escalation, or stop-condition enforcement.",
    noReplayRepairNote: "Review does not repair replay or promote recovery.",
    hardeningCandidateNote:
      "Hardening candidates are inputs for Phase 169 code work, not approvals.",
  };
}

function reviewFinding(
  category: TrialEvidenceReviewFindingCategory,
  severity: TrialEvidenceReviewFindingSeverity,
  disposition: TrialEvidenceReviewFindingDisposition,
  source: TrialEvidenceReviewSource,
  summary: string,
  surface: string,
  linkage: string,
): TrialEvidenceReviewFinding {
  return {
    findingId: "",
    category,
    severity,
    disposition,
    source,
    summary,
    evidenceLinkage: { source, surface, summary: linkage },
  };
}

function orderedReviewFindings(
  findings: TrialEvidenceReviewFinding[],
): TrialEvidenceReviewFinding[] {
  return [...findings]
    .sort((a, b) =>
      `${a.severity}|${a.category}|${a.source}|${a.summary}`.localeCompare(
        `${b.severity}|${b.category}|${b.source}|${b.summary}`,
      ),
    )
    .map((finding, index) => ({
      ...finding,
      findingId: `trial-evidence-review-finding-${String(index + 1).padStart(4, "0")}`,
    }));
}

export function deriveTrialEvidenceReviewFindings(
  state: LocalOperatorShellState,
): readonly TrialEvidenceReviewFinding[] {
  const findings: TrialEvidenceReviewFinding[] = [];
  const pkg = state.controlledInternalTrialPackageProjection;
  if (pkg.status === "not_packaged")
    findings.push(
      reviewFinding(
        "trial_package",
        "blocking",
        "requires_trial_coordinator_review",
        "controlled_internal_trial_package",
        "Controlled internal trial package is missing.",
        "controlled_internal_trial_package",
        `package_status=${pkg.status}`,
      ),
    );
  if (
    pkg.validationStatus === "invalid" ||
    (pkg.readBackValidationStatus !== null &&
      pkg.readBackValidationStatus !== "valid")
  )
    findings.push(
      reviewFinding(
        "trial_package",
        "blocking",
        "requires_operator_review",
        "controlled_internal_trial_package",
        "Controlled internal trial package validation or read-back failed.",
        "controlled_internal_trial_package",
        `validation=${pkg.validationStatus} read_back=${pkg.readBackValidationStatus ?? "not_read"}`,
      ),
    );
  const execution = state.controlledInternalTrialExecution;
  if (execution.status === "not_started")
    findings.push(
      reviewFinding(
        "trial_execution",
        "warning",
        "requires_operator_review",
        "controlled_trial_execution_harness",
        "Controlled internal trial execution harness has no trial run.",
        "controlled_trial_execution_harness",
        `trial_execution_status=${execution.status}`,
      ),
    );
  if (
    ["trial_run_rejected", "invalid_trial_run_request"].includes(
      execution.status,
    )
  )
    findings.push(
      reviewFinding(
        "trial_execution",
        "blocking",
        "requires_trial_coordinator_review",
        "controlled_trial_execution_harness",
        `Controlled trial run rejected: ${execution.rejectionReasons.join(", ") || "none"}.`,
        "controlled_trial_execution_harness",
        `trial_execution_status=${execution.status}`,
      ),
    );
  if (execution.status === "trial_run_blocked")
    findings.push(
      reviewFinding(
        "workflow_blocker",
        "blocking",
        "requires_operator_review",
        "controlled_trial_execution_harness",
        `Controlled trial run blocked at ${execution.currentBlocker ?? "unknown"}.`,
        "controlled_trial_execution_harness",
        `current_blocker=${execution.currentBlocker ?? "none"}`,
      ),
    );
  const evidence = state.trialSessionEvidenceProjection;
  if (evidence.status === "not_captured")
    findings.push(
      reviewFinding(
        "trial_session_evidence",
        "blocking",
        "requires_operator_review",
        "trial_session_evidence",
        "Trial session evidence is missing.",
        "trial_session_evidence",
        `evidence_status=${evidence.status}`,
      ),
    );
  if (
    evidence.validationStatus === "invalid" ||
    (evidence.readBackValidationStatus !== null &&
      evidence.readBackValidationStatus !== "valid")
  )
    findings.push(
      reviewFinding(
        "trial_session_evidence",
        "blocking",
        "requires_operator_review",
        "trial_session_evidence",
        "Trial session evidence validation or read-back failed.",
        "trial_session_evidence",
        `validation=${evidence.validationStatus} read_back=${evidence.readBackValidationStatus ?? "not_read"}`,
      ),
    );
  const verification = state.trialReplayRestoreVerification;
  if (verification.status === "not_verified")
    findings.push(
      reviewFinding(
        "replay_restore_verification",
        "blocking",
        "requires_operator_review",
        "replay_restore_verification",
        "Replay/restore verification is missing.",
        "replay_restore_verification",
        `verification_status=${verification.status}`,
      ),
    );
  if (
    ["verification_rejected", "invalid_verification_input"].includes(
      verification.status,
    )
  )
    findings.push(
      reviewFinding(
        "replay_restore_verification",
        "blocking",
        "requires_operator_review",
        "replay_restore_verification",
        "Replay/restore verification did not pass.",
        "replay_restore_verification",
        `verification_status=${verification.status}`,
      ),
    );
  for (const mismatch of verification.mismatches) {
    const category: TrialEvidenceReviewFindingCategory =
      mismatch === "replay_status_snapshot_mismatch"
        ? "replay_status"
        : mismatch === "restore_history_snapshot_mismatch"
          ? "restore_history"
          : mismatch.includes("evidence")
            ? "trial_session_evidence"
            : mismatch.includes("package")
              ? "trial_package"
              : "replay_restore_verification";
    findings.push(
      reviewFinding(
        category,
        "blocking",
        "requires_operator_review",
        "replay_restore_verification",
        `Replay/restore verification mismatch observed: ${mismatch}.`,
        "replay_restore_verification",
        `mismatch=${mismatch}`,
      ),
    );
  }
  if (state.trialObservability.stopConditionObservation.observed)
    findings.push(
      reviewFinding(
        "stop_condition",
        "stop_condition",
        "requires_security_reviewer_review",
        "trial_observability",
        "Stop-condition observation is present for operator review.",
        "trial_observability",
        state.trialObservability.stopConditionObservation.status,
      ),
    );
  for (const detail of state.trialErrorReport.details) {
    const category: TrialEvidenceReviewFindingCategory =
      detail.category === "stop_condition_observed"
        ? "stop_condition"
        : detail.category === "replay_status_mismatch"
          ? "replay_status"
          : detail.category === "restore_history_mismatch"
            ? "restore_history"
            : detail.category === "materialization_missing"
              ? "candidate_materialization"
              : detail.category.includes("workflow")
                ? "workflow_blocker"
                : detail.category.includes("package")
                  ? "trial_package"
                  : detail.category.includes("evidence")
                    ? "trial_session_evidence"
                    : "error_reporting";
    findings.push(
      reviewFinding(
        category,
        detail.severity === "invalid_input" ? "invalid_input" : detail.severity,
        "requires_operator_review",
        "trial_error_report",
        detail.summary,
        "trial_error_report",
        detail.evidenceLinkage.linkage,
      ),
    );
  }
  if (state.completeLocalOperatorWorkflow.currentBlockingStep)
    findings.push(
      reviewFinding(
        "workflow_blocker",
        "blocking",
        "requires_operator_review",
        "complete_local_workflow",
        `Complete local workflow blocker: ${state.completeLocalOperatorWorkflow.currentBlockingStep}.`,
        "complete_local_workflow",
        `workflow_status=${state.completeLocalOperatorWorkflow.status}`,
      ),
    );
  if (state.localCandidateOutput.status === "not_materialized")
    findings.push(
      reviewFinding(
        "candidate_materialization",
        "warning",
        "requires_phase_169_hardening",
        "local_candidate_materialization",
        "Local candidate materialization is missing from the trial evidence chain.",
        "local_candidate_materialization",
        `materialization_status=${state.localCandidateOutput.status}`,
      ),
    );
  findings.push(
    reviewFinding(
      "user_help_needed",
      "hardening_required",
      "requires_phase_169_hardening",
      "trial_operator_runbook",
      "User-facing local help documentation is not yet represented in the trial review surface.",
      "trial_operator_runbook",
      "Phase 169 owns user-facing local HTML help if still missing.",
    ),
  );
  for (const guidance of state.trialFailureDrill.escalationGuidance)
    findings.push(
      reviewFinding(
        "escalation_guidance",
        "info",
        "requires_trial_coordinator_review",
        "trial_failure_drill",
        `Escalation guidance remains descriptive for role ${guidance.role}.`,
        "trial_failure_drill",
        guidance.categories.join(","),
      ),
    );
  return orderedReviewFindings(findings);
}

export function deriveTrialHardeningCandidates(
  findings: readonly TrialEvidenceReviewFinding[],
): readonly TrialEvidenceReviewHardeningCandidate[] {
  return findings
    .filter(
      (finding) =>
        finding.disposition === "requires_phase_169_hardening" ||
        ["blocking", "hardening_required"].includes(finding.severity),
    )
    .map(
      (finding): TrialEvidenceReviewHardeningCandidate => ({
        candidateId: "",
        sourceFindingId: finding.findingId,
        targetSurface: finding.evidenceLinkage.surface,
        localBetaScope: "local_beta_only_phase_169_input",
        summary: `Phase 169 local-beta hardening candidate from ${finding.category}: ${finding.summary}`,
      }),
    )
    .sort((a, b) =>
      `${a.targetSurface}|${a.sourceFindingId}|${a.summary}`.localeCompare(
        `${b.targetSurface}|${b.sourceFindingId}|${b.summary}`,
      ),
    )
    .map((candidate, index) => ({
      ...candidate,
      candidateId: `trial-hardening-candidate-${String(index + 1).padStart(4, "0")}`,
    }));
}

export function deriveTrialEvidenceReviewProjection(
  state: LocalOperatorShellState,
): TrialEvidenceReviewProjection {
  const findings = deriveTrialEvidenceReviewFindings(state);
  const hardeningCandidates = deriveTrialHardeningCandidates(findings);
  const blockerFindings = findings.filter((finding) =>
    ["blocking", "stop_condition", "invalid_input"].includes(finding.severity),
  );
  const status: TrialEvidenceReviewStatus = findings.some(
    (finding) => finding.severity === "invalid_input",
  )
    ? "invalid_review_input"
    : findings.some((finding) => finding.severity === "stop_condition")
      ? "review_blocked"
      : ["trial_run_rejected", "invalid_trial_run_request"].includes(
            state.controlledInternalTrialExecution.status,
          )
        ? "review_rejected"
        : hardeningCandidates.length > 0
          ? "hardening_candidates_projected"
          : findings.length > 0
            ? "review_with_findings"
            : "review_projected";
  return {
    status,
    reviewId: `trial-evidence-review-findings-${String(findings.length).padStart(4, "0")}-hardening-${String(hardeningCandidates.length).padStart(4, "0")}`,
    controlledTrialPackageStatus:
      state.controlledInternalTrialPackageProjection.status,
    trialExecutionStatus: state.controlledInternalTrialExecution.status,
    trialSessionEvidenceStatus: state.trialSessionEvidenceProjection.status,
    replayRestoreVerificationStatus:
      state.trialReplayRestoreVerification.status,
    observabilityStatus: state.trialObservability.status,
    errorReportStatus: state.trialErrorReport.status,
    stopConditionObservation:
      state.trialObservability.stopConditionObservation.status,
    escalationGuidanceSummary: state.trialFailureDrill.escalationGuidance.map(
      (guidance) => `${guidance.role}:${guidance.categories.join("/")}`,
    ),
    evidenceLinkage: [
      {
        source: "controlled_internal_trial_package",
        surface: "controlled_internal_trial_package",
        summary: `package_status=${state.controlledInternalTrialPackageProjection.status} package_id=${state.controlledInternalTrialPackageProjection.packageId ?? "none"}`,
      },
      {
        source: "controlled_trial_execution_harness",
        surface: "controlled_trial_execution_harness",
        summary: `trial_execution_status=${state.controlledInternalTrialExecution.status}`,
      },
      {
        source: "trial_session_evidence",
        surface: "trial_session_evidence",
        summary: `evidence_status=${state.trialSessionEvidenceProjection.status} evidence_id=${state.trialSessionEvidenceProjection.evidenceId ?? "none"}`,
      },
      {
        source: "replay_restore_verification",
        surface: "replay_restore_verification",
        summary: `verification_status=${state.trialReplayRestoreVerification.status}`,
      },
      {
        source: "trial_observability",
        surface: "trial_observability",
        summary: `observability_status=${state.trialObservability.status}`,
      },
      {
        source: "trial_error_report",
        surface: "trial_error_report",
        summary: `error_report_status=${state.trialErrorReport.status} categories=${state.trialErrorReport.categorySummary.length}`,
      },
    ],
    findings,
    unresolvedBlockers: {
      currentBlocker: state.trialObservability.currentBlocker,
      unresolvedBlockerCount: blockerFindings.length,
      blockers: blockerFindings.map(
        (finding) => `${finding.findingId}:${finding.summary}`,
      ),
    },
    mismatchErrorSummary: {
      mismatchCount: state.trialObservability.mismatchSummary.mismatches.length,
      errorCategoryCount: state.trialErrorReport.categorySummary.length,
      replayStatusComparison:
        state.trialObservability.replayStatusComparisonSummary,
      restoreHistoryComparison:
        state.trialObservability.restoreHistoryComparisonSummary,
      packageEvidenceReadBackFailures:
        state.trialObservability.packageEvidenceReadBackFailureSummary,
    },
    unresolvedFindingCount: findings.filter(
      (finding) => finding.disposition !== "not_applicable",
    ).length,
    hardeningCandidates,
    boundaryStatuses: trialEvidenceReviewBoundaryStatuses(),
    capabilitySurface: trialEvidenceReviewCapabilitySurface(),
    localOnlyNonPublicNote:
      "Trial evidence review is local-only and non-public.",
    noAuthorityNote:
      "Review findings are evidence, not approval. Review does not approve controlled human use, readiness, release, deployment, public use, or production use.",
    noAutomationNote:
      "Review does not automate remediation, escalation, or stop-condition enforcement.",
    noReplayRepairNote: "Review does not repair replay or promote recovery.",
    hardeningCandidateNote:
      "Hardening candidates are inputs for Phase 169 code work, not approvals.",
  };
}

function refreshTrialObservabilityState(
  state: LocalOperatorShellState,
): LocalOperatorShellState {
  const withoutReports = {
    ...state,
    trialObservability: initialTrialObservabilityProjection(),
    trialErrorReport: initialTrialErrorReportProjection(),
  };
  const refreshed = {
    ...withoutReports,
    trialObservability: deriveTrialObservabilityProjection(withoutReports),
    trialErrorReport: deriveTrialErrorReportProjection(withoutReports),
  };
  return {
    ...refreshed,
    trialEvidenceReview: deriveTrialEvidenceReviewProjection(refreshed),
  };
}

export type ReleaseCandidatePreparationStatus =
  | "not_prepared"
  | "preparation_projected"
  | "preparation_validated"
  | "preparation_blocked"
  | "preparation_rejected"
  | "invalid_preparation_input";
export type ReleaseCandidatePreparationEvidenceCategory =
  | "local_beta_workflow"
  | "controlled_internal_trial_execution"
  | "trial_observability"
  | "trial_error_reporting"
  | "trial_evidence_review"
  | "user_facing_help"
  | "local_html_help_pages"
  | "visible_help_entry_point"
  | "provider_configuration"
  | "provider_adapter_contract"
  | "provider_adapter_dry_run"
  | "constrained_provider_invocation"
  | "provider_output_pipeline"
  | "provider_output_validation"
  | "provider_output_review"
  | "staged_proposal"
  | "staged_proposal_validation"
  | "candidate_review"
  | "operator_decision"
  | "local_candidate_materialization"
  | "replay_status"
  | "evidence_export"
  | "local_session_package"
  | "session_restore_history"
  | "controlled_internal_trial_package"
  | "trial_operator_runbook"
  | "trial_failure_drill"
  | "trial_session_evidence"
  | "replay_restore_verification"
  | "deterministic_validation";
export type ReleaseCandidatePreparationEvidenceCategoryStatus =
  | "present"
  | "missing"
  | "blocked"
  | "rejected"
  | "deferred"
  | "not_applicable";
export type ReleaseCandidatePreparationValidationStatus =
  | "not_validated"
  | "valid"
  | "invalid";
export type ReleaseCandidatePreparationValidationError =
  | "missing_local_beta_workflow"
  | "missing_trial_execution"
  | "missing_trial_observability"
  | "missing_trial_evidence_review"
  | "missing_user_help"
  | "missing_help_index"
  | "missing_visible_help_entry_point"
  | "missing_candidate_materialization"
  | "missing_replay_status"
  | "missing_session_package"
  | "missing_restore_history"
  | "missing_trial_package"
  | "missing_trial_session_evidence"
  | "missing_replay_restore_verification"
  | "evidence_category_blocked"
  | "evidence_category_rejected"
  | "readiness_claim_detected"
  | "release_claim_detected"
  | "deployment_claim_detected"
  | "public_use_claim_detected"
  | "production_use_claim_detected"
  | "signing_claim_detected"
  | "publishing_claim_detected"
  | "installer_claim_detected"
  | "update_channel_claim_detected"
  | "provider_trust_claim_detected"
  | "action_authorization_claim_detected"
  | "replay_repair_claim_detected"
  | "recovery_promotion_claim_detected";
export type ReleaseCandidatePreparationBoundaryStatus =
  | "release_candidate_preparation_only"
  | "release_readiness_not_approved"
  | "release_candidate_status_not_approved"
  | "production_status_not_approved"
  | "no_release_artifact"
  | "no_deployment_artifact"
  | "no_public_distribution"
  | "no_signing"
  | "no_publishing"
  | "no_installer_activation"
  | "no_update_channel_activation"
  | "no_provider_trust"
  | "no_action_authorization"
  | "no_replay_repair"
  | "no_recovery_promotion";
export type ReleaseCandidatePreparationClassification =
  | "release_candidate_preparation_only"
  | "non_production"
  | "release_not_approved"
  | "no_public_distribution";
export type ReleaseCandidatePreparationSourceLinkage = Readonly<{
  category: ReleaseCandidatePreparationEvidenceCategory;
  sourceSurface: string;
  sourceStatus: string;
  sourceSummary: string;
}>;
export type ReleaseCandidatePreparationEvidenceItem = Readonly<{
  category: ReleaseCandidatePreparationEvidenceCategory;
  status: ReleaseCandidatePreparationEvidenceCategoryStatus;
  reason: string;
  sourceLinkage: ReleaseCandidatePreparationSourceLinkage;
}>;
export type ReleaseCandidatePreparationMissingEvidence = Readonly<{
  category: ReleaseCandidatePreparationEvidenceCategory;
  reason: string;
  sourceSurface: string;
}>;
export type ReleaseCandidatePreparationBlocker = Readonly<{
  category: ReleaseCandidatePreparationEvidenceCategory;
  reason: string;
  sourceSurface: string;
}>;
export type ReleaseCandidatePreparationCapabilitySurface = Readonly<{
  createsReleaseArtifacts: false;
  createsDeploymentArtifacts: false;
  createsPublicDownloads: false;
  createsReleaseTags: false;
  createsGithubReleases: false;
  signsArtifacts: false;
  publishesArtifacts: false;
  createsInstallers: false;
  activatesUpdateChannels: false;
  approvesReleaseCandidateStatus: false;
  approvesReleaseReadiness: false;
  approvesProductionStatus: false;
  approvesProductionReadiness: false;
  approvesDeployment: false;
  approvesPublicUse: false;
  approvesProductionUse: false;
  trustsProviderOutput: false;
  authorizesActions: false;
  repairsReplay: false;
  promotesRecovery: false;
}>;
export type ReleaseCandidatePreparationProjection = Readonly<{
  preparationId: string;
  status: ReleaseCandidatePreparationStatus;
  validationStatus: ReleaseCandidatePreparationValidationStatus;
  classification: readonly ReleaseCandidatePreparationClassification[];
  evidenceItems: readonly ReleaseCandidatePreparationEvidenceItem[];
  missingEvidence: readonly ReleaseCandidatePreparationMissingEvidence[];
  blockers: readonly ReleaseCandidatePreparationBlocker[];
  validationErrors: readonly ReleaseCandidatePreparationValidationError[];
  boundaryStatuses: readonly ReleaseCandidatePreparationBoundaryStatus[];
  capabilitySurface: ReleaseCandidatePreparationCapabilitySurface;
  categoryCount: number;
  presentEvidenceCount: number;
  missingEvidenceCount: number;
  blockedEvidenceCount: number;
  rejectedEvidenceCount: number;
  noReleaseReadinessNote: string;
  noReleaseArtifactNote: string;
  noReleaseCandidateStatusNote: string;
  noProductionDeploymentPublicSigningPublishingInstallerUpdateNote: string;
  noProviderTrustNote: string;
  noActionAuthorizationNote: string;
}>;

export function releaseCandidatePreparationEvidenceCategories(): readonly ReleaseCandidatePreparationEvidenceCategory[] {
  return [
    "local_beta_workflow",
    "controlled_internal_trial_execution",
    "trial_observability",
    "trial_error_reporting",
    "trial_evidence_review",
    "user_facing_help",
    "local_html_help_pages",
    "visible_help_entry_point",
    "provider_configuration",
    "provider_adapter_contract",
    "provider_adapter_dry_run",
    "constrained_provider_invocation",
    "provider_output_pipeline",
    "provider_output_validation",
    "provider_output_review",
    "staged_proposal",
    "staged_proposal_validation",
    "candidate_review",
    "operator_decision",
    "local_candidate_materialization",
    "replay_status",
    "evidence_export",
    "local_session_package",
    "session_restore_history",
    "controlled_internal_trial_package",
    "trial_operator_runbook",
    "trial_failure_drill",
    "trial_session_evidence",
    "replay_restore_verification",
    "deterministic_validation",
  ];
}
function releaseCandidatePreparationBoundaryStatuses(): readonly ReleaseCandidatePreparationBoundaryStatus[] {
  return [
    "release_candidate_preparation_only",
    "release_readiness_not_approved",
    "release_candidate_status_not_approved",
    "production_status_not_approved",
    "no_release_artifact",
    "no_deployment_artifact",
    "no_public_distribution",
    "no_signing",
    "no_publishing",
    "no_installer_activation",
    "no_update_channel_activation",
    "no_provider_trust",
    "no_action_authorization",
    "no_replay_repair",
    "no_recovery_promotion",
  ];
}
function releaseCandidatePreparationCapabilitySurface(): ReleaseCandidatePreparationCapabilitySurface {
  return {
    createsReleaseArtifacts: false,
    createsDeploymentArtifacts: false,
    createsPublicDownloads: false,
    createsReleaseTags: false,
    createsGithubReleases: false,
    signsArtifacts: false,
    publishesArtifacts: false,
    createsInstallers: false,
    activatesUpdateChannels: false,
    approvesReleaseCandidateStatus: false,
    approvesReleaseReadiness: false,
    approvesProductionStatus: false,
    approvesProductionReadiness: false,
    approvesDeployment: false,
    approvesPublicUse: false,
    approvesProductionUse: false,
    trustsProviderOutput: false,
    authorizesActions: false,
    repairsReplay: false,
    promotesRecovery: false,
  };
}
function releaseCandidatePreparationItem(
  category: ReleaseCandidatePreparationEvidenceCategory,
  status: ReleaseCandidatePreparationEvidenceCategoryStatus,
  sourceSurface: string,
  sourceStatus: string,
  reason = status === "present"
    ? "committed local shell evidence is present"
    : "required committed evidence is missing",
): ReleaseCandidatePreparationEvidenceItem {
  return {
    category,
    status,
    reason,
    sourceLinkage: {
      category,
      sourceSurface,
      sourceStatus,
      sourceSummary: `${sourceSurface}:${sourceStatus}`,
    },
  };
}
function releaseCandidatePreparationId(
  items: readonly ReleaseCandidatePreparationEvidenceItem[],
): string {
  let hash = 0x811c9dc5;
  for (const item of items)
    for (const char of `${item.category}:${item.status}:${item.sourceLinkage.sourceStatus};`)
      hash = Math.imul((hash ^ char.charCodeAt(0)) >>> 0, 0x01000193) >>> 0;
  return `release-candidate-preparation-${hash.toString(16).padStart(8, "0")}`;
}
export function deriveReleaseCandidatePreparationProjection(
  state: LocalOperatorShellState,
): ReleaseCandidatePreparationProjection {
  const statusFor = (
    condition: boolean,
    sourceStatus: string,
  ): ReleaseCandidatePreparationEvidenceCategoryStatus =>
    condition
      ? "present"
      : sourceStatus.includes("rejected") || sourceStatus.includes("invalid")
        ? "rejected"
        : sourceStatus.includes("blocked") ||
            sourceStatus.includes("required") ||
            sourceStatus.includes("missing")
          ? "blocked"
          : "missing";
  const items: ReleaseCandidatePreparationEvidenceItem[] = [
    releaseCandidatePreparationItem(
      "local_beta_workflow",
      statusFor(
        state.completeLocalOperatorWorkflow.status ===
          "complete_local_workflow_projected" ||
          state.completeLocalOperatorWorkflow.status ===
            "local_candidate_materialized",
        state.completeLocalOperatorWorkflow.status,
      ),
      "completeLocalOperatorWorkflow",
      state.completeLocalOperatorWorkflow.status,
    ),
    releaseCandidatePreparationItem(
      "controlled_internal_trial_execution",
      statusFor(
        state.controlledInternalTrialExecution.status ===
          "trial_run_completed_locally",
        state.controlledInternalTrialExecution.status,
      ),
      "controlledInternalTrialExecution",
      state.controlledInternalTrialExecution.status,
    ),
    releaseCandidatePreparationItem(
      "trial_observability",
      statusFor(
        state.trialObservability.status !== "not_observed" &&
          state.trialObservability.status !== "observability_projected",
        state.trialObservability.status,
      ),
      "trialObservability",
      state.trialObservability.status,
    ),
    releaseCandidatePreparationItem(
      "trial_error_reporting",
      state.trialErrorReport.status === "invalid_observability_input"
        ? "rejected"
        : "present",
      "trialErrorReport",
      state.trialErrorReport.status,
    ),
    releaseCandidatePreparationItem(
      "trial_evidence_review",
      statusFor(
        ![
          "not_reviewed",
          "review_blocked",
          "review_rejected",
          "invalid_review_input",
        ].includes(state.trialEvidenceReview.status),
        state.trialEvidenceReview.status,
      ),
      "trialEvidenceReview",
      state.trialEvidenceReview.status,
    ),
    releaseCandidatePreparationItem(
      "user_facing_help",
      "present",
      "uiHelpSurface",
      "present",
    ),
    releaseCandidatePreparationItem(
      "local_html_help_pages",
      "present",
      "uiHelpSurface",
      "present",
    ),
    releaseCandidatePreparationItem(
      "visible_help_entry_point",
      "present",
      "uiHelpEntryPoint",
      "present",
    ),
    releaseCandidatePreparationItem(
      "provider_configuration",
      statusFor(
        state.providerConfiguration.status === "accepted",
        state.providerConfiguration.status,
      ),
      "providerConfiguration",
      state.providerConfiguration.status,
    ),
    releaseCandidatePreparationItem(
      "provider_adapter_contract",
      statusFor(
        state.localProviderAdapterRegistry.declarations.some(
          (declaration) =>
            declaration.status === "adapter_declared_non_executing",
        ),
        `declarations=${state.localProviderAdapterRegistry.declarations.length}`,
      ),
      "localProviderAdapterRegistry",
      `declarations=${state.localProviderAdapterRegistry.declarations.length}`,
    ),
    releaseCandidatePreparationItem(
      "provider_adapter_dry_run",
      statusFor(
        state.localProviderAdapterDryRun.status === "dry_run_executed",
        state.localProviderAdapterDryRun.status,
      ),
      "localProviderAdapterDryRun",
      state.localProviderAdapterDryRun.status,
    ),
    releaseCandidatePreparationItem(
      "constrained_provider_invocation",
      statusFor(
        state.constrainedLocalProviderInvocation.status ===
          "invocation_executed",
        state.constrainedLocalProviderInvocation.status,
      ),
      "constrainedLocalProviderInvocation",
      state.constrainedLocalProviderInvocation.status,
    ),
    releaseCandidatePreparationItem(
      "provider_output_pipeline",
      statusFor(
        state.localProviderOutputPipeline.status === "valid",
        state.localProviderOutputPipeline.status,
      ),
      "localProviderOutputPipeline",
      state.localProviderOutputPipeline.status,
    ),
    releaseCandidatePreparationItem(
      "provider_output_validation",
      statusFor(
        state.providerOutputValidation.status === "reviewable_untrusted",
        state.providerOutputValidation.status,
      ),
      "providerOutputValidation",
      state.providerOutputValidation.status,
    ),
    releaseCandidatePreparationItem(
      "provider_output_review",
      statusFor(
        state.providerOutputValidation.reviewabilityStatus ===
          "reviewable_untrusted",
        state.providerOutputValidation.reviewabilityStatus,
      ),
      "providerOutputValidation.reviewability",
      state.providerOutputValidation.reviewabilityStatus,
    ),
    releaseCandidatePreparationItem(
      "staged_proposal",
      statusFor(
        state.stagedCandidateConversionProposal.status ===
          "staged_proposal_created",
        state.stagedCandidateConversionProposal.status,
      ),
      "stagedCandidateConversionProposal",
      state.stagedCandidateConversionProposal.status,
    ),
    releaseCandidatePreparationItem(
      "staged_proposal_validation",
      statusFor(
        state.stagedCandidateConversionValidation.status ===
          "staged_proposal_shape_valid",
        state.stagedCandidateConversionValidation.status,
      ),
      "stagedCandidateConversionValidation",
      state.stagedCandidateConversionValidation.status,
    ),
    releaseCandidatePreparationItem(
      "candidate_review",
      statusFor(
        state.localProviderOutputPipeline.candidateReviewStatus ===
          "display_only",
        state.localProviderOutputPipeline.candidateReviewStatus,
      ),
      "localProviderOutputPipeline.candidateReviewStatus",
      state.localProviderOutputPipeline.candidateReviewStatus,
    ),
    releaseCandidatePreparationItem(
      "operator_decision",
      statusFor(
        state.operatorCandidateDecision.status ===
          "approved_validated_staged_proposal",
        state.operatorCandidateDecision.status,
      ),
      "operatorCandidateDecision",
      state.operatorCandidateDecision.status,
    ),
    releaseCandidatePreparationItem(
      "local_candidate_materialization",
      statusFor(
        state.localCandidateOutput.status === "local_candidate_materialized",
        state.localCandidateOutput.status,
      ),
      "localCandidateOutput",
      state.localCandidateOutput.status,
    ),
    releaseCandidatePreparationItem(
      "replay_status",
      state.run.replayStatus.length > 0 ? "present" : "missing",
      "run.replayStatus",
      state.run.replayStatus,
    ),
    releaseCandidatePreparationItem(
      "evidence_export",
      state.localSessionEvidenceExport.exportId.length > 0
        ? "present"
        : "missing",
      "localSessionEvidenceExport",
      state.localSessionEvidenceExport.exportStatus,
    ),
    releaseCandidatePreparationItem(
      "local_session_package",
      statusFor(
        state.localSessionPackageProjection.status !== "not_packaged",
        state.localSessionPackageProjection.status,
      ),
      "localSessionPackageProjection",
      state.localSessionPackageProjection.status,
    ),
    releaseCandidatePreparationItem(
      "session_restore_history",
      statusFor(
        state.localSessionHistoryProjection.status ===
          "session_history_projected" &&
          !state.localSessionRestoreProjection.status.includes("rejected"),
        state.localSessionRestoreProjection.status,
      ),
      "localSessionRestoreHistory",
      state.localSessionRestoreProjection.status,
    ),
    releaseCandidatePreparationItem(
      "controlled_internal_trial_package",
      statusFor(
        state.controlledInternalTrialPackageProjection.status !==
          "not_packaged",
        state.controlledInternalTrialPackageProjection.status,
      ),
      "controlledInternalTrialPackageProjection",
      state.controlledInternalTrialPackageProjection.status,
    ),
    releaseCandidatePreparationItem(
      "trial_operator_runbook",
      statusFor(
        state.trialOperatorRunbook.status ===
          "ready_for_manual_trial_preparation" ||
          state.trialOperatorRunbook.status ===
            "trial_operator_runbook_projected",
        state.trialOperatorRunbook.status,
      ),
      "trialOperatorRunbook",
      state.trialOperatorRunbook.status,
    ),
    releaseCandidatePreparationItem(
      "trial_failure_drill",
      "present",
      "trialFailureDrill",
      state.trialFailureDrill.status,
    ),
    releaseCandidatePreparationItem(
      "trial_session_evidence",
      statusFor(
        state.trialSessionEvidenceProjection.status !== "not_captured",
        state.trialSessionEvidenceProjection.status,
      ),
      "trialSessionEvidenceProjection",
      state.trialSessionEvidenceProjection.status,
    ),
    releaseCandidatePreparationItem(
      "replay_restore_verification",
      statusFor(
        state.trialReplayRestoreVerification.status === "verification_passed" ||
          state.trialReplayRestoreVerification.status ===
            "trial_replay_restore_verification_projected",
        state.trialReplayRestoreVerification.status,
      ),
      "trialReplayRestoreVerification",
      state.trialReplayRestoreVerification.status,
    ),
    releaseCandidatePreparationItem(
      "deterministic_validation",
      state.run.validation ? "present" : "missing",
      "run.validation",
      state.run.validation?.validationStatus ?? "missing",
    ),
  ];
  const missingEvidence = items
    .filter((item) => item.status === "missing" || item.status === "deferred")
    .map((item) => ({
      category: item.category,
      reason: item.reason,
      sourceSurface: item.sourceLinkage.sourceSurface,
    }));
  const blockers = items
    .filter((item) => item.status === "blocked" || item.status === "rejected")
    .map((item) => ({
      category: item.category,
      reason: item.reason,
      sourceSurface: item.sourceLinkage.sourceSurface,
    }));
  const validationErrors: ReleaseCandidatePreparationValidationError[] = [];
  if (missingEvidence.some((item) => item.category === "local_beta_workflow"))
    validationErrors.push("missing_local_beta_workflow");
  if (
    missingEvidence.some(
      (item) => item.category === "controlled_internal_trial_execution",
    )
  )
    validationErrors.push("missing_trial_execution");
  if (missingEvidence.some((item) => item.category === "trial_observability"))
    validationErrors.push("missing_trial_observability");
  if (missingEvidence.some((item) => item.category === "trial_evidence_review"))
    validationErrors.push("missing_trial_evidence_review");
  if (
    missingEvidence.some((item) => item.category === "visible_help_entry_point")
  )
    validationErrors.push("missing_visible_help_entry_point");
  if (items.some((item) => item.status === "blocked"))
    validationErrors.push("evidence_category_blocked");
  if (items.some((item) => item.status === "rejected"))
    validationErrors.push("evidence_category_rejected");
  const status: ReleaseCandidatePreparationStatus = validationErrors.includes(
    "evidence_category_rejected",
  )
    ? "preparation_rejected"
    : validationErrors.includes("evidence_category_blocked")
      ? "preparation_blocked"
      : validationErrors.length === 0
        ? "preparation_validated"
        : "invalid_preparation_input";
  return {
    preparationId: releaseCandidatePreparationId(items),
    status,
    validationStatus: validationErrors.length === 0 ? "valid" : "invalid",
    classification: [
      "release_candidate_preparation_only",
      "non_production",
      "release_not_approved",
      "no_public_distribution",
    ],
    evidenceItems: items,
    missingEvidence,
    blockers,
    validationErrors,
    boundaryStatuses: releaseCandidatePreparationBoundaryStatuses(),
    capabilitySurface: releaseCandidatePreparationCapabilitySurface(),
    categoryCount: items.length,
    presentEvidenceCount: items.filter((item) => item.status === "present")
      .length,
    missingEvidenceCount: missingEvidence.length,
    blockedEvidenceCount: items.filter((item) => item.status === "blocked")
      .length,
    rejectedEvidenceCount: items.filter((item) => item.status === "rejected")
      .length,
    noReleaseReadinessNote:
      "Release-candidate preparation is not release readiness.",
    noReleaseArtifactNote: "This contract does not create release artifacts.",
    noReleaseCandidateStatusNote:
      "This contract does not approve Release Candidate status.",
    noProductionDeploymentPublicSigningPublishingInstallerUpdateNote:
      "This contract does not approve production, deployment, public use, signing, publishing, installer, or update-channel behavior.",
    noProviderTrustNote: "Provider output remains untrusted.",
    noActionAuthorizationNote: "No action authorization is granted.",
  };
}

export type LocalOperatorShellState = Readonly<{
  harnessStatus: string;
  nonProduction: true;
  run: LocalRunProjection;
  decisionLedger: LocalDecisionLedger;
  localSessionEvidenceExport: LocalSessionEvidenceExport;
  providerConfiguration: LocalProviderConfiguration;
  localProviderAdapterRegistry: LocalProviderAdapterRegistry;
  localProviderAdapterDryRun: LocalProviderAdapterDryRunProjection;
  constrainedLocalProviderInvocation: ConstrainedLocalProviderInvocationProjection;
  localProviderOutputPipeline: LocalProviderOutputPipelineProjection;
  providerExecution: LocalProviderExecutionProjection;
  providerOutputValidation: LocalProviderOutputValidationProjection;
  stagedCandidateConversionProposal: StagedCandidateConversionProposalProjection;
  stagedCandidateConversionValidation: StagedCandidateConversionValidationProjection;
  operatorCandidateDecision: OperatorCandidateDecisionProjection;
  localCandidateOutput: LocalCandidateOutputProjection;
  phase150CodeProductionHandoff: Phase150CodeProductionHandoff;
  localSessionPackageProjection: LocalSessionPackageProjection;
  localSessionHistoryProjection: LocalSessionHistoryProjection;
  localSessionRestoreProjection: LocalSessionRestoreProjection;
  controlledInternalTrialPackageProjection: ControlledInternalTrialPackageProjection;
  completeLocalOperatorWorkflow: CompleteLocalOperatorWorkflowProjection;
  trialOperatorRunbook: TrialOperatorRunbookProjection;
  trialFailureDrill: TrialFailureDrillProjection;
  trialSessionEvidenceProjection: TrialSessionEvidenceProjection;
  trialReplayRestoreVerification: TrialReplayRestoreVerificationProjection;
  controlledInternalTrialExecution: ControlledInternalTrialExecutionProjection;
  trialObservability: TrialObservabilityProjection;
  trialErrorReport: TrialErrorReportProjection;
  trialEvidenceReview: TrialEvidenceReviewProjection;
  releaseCandidatePreparation: ReleaseCandidatePreparationProjection;
}>;

export function completeLocalOperatorWorkflowBoundaryStatuses(): readonly CompleteLocalOperatorWorkflowBoundaryStatus[] {
  return [
    "local_beta_workflow_only",
    "no_provider_trust",
    "no_production_approval",
    "no_release_approval",
    "no_deployment_approval",
    "no_public_use_approval",
    "no_action_execution",
    "no_replay_repair",
    "no_recovery_promotion",
  ];
}

export function completeLocalOperatorWorkflowStepOrder(): readonly CompleteLocalOperatorWorkflowStepKind[] {
  return [
    "provider_adapter_configured",
    "adapter_dry_run_available",
    "constrained_invocation_completed",
    "provider_output_pipeline_projected",
    "provider_output_validated",
    "provider_output_reviewed",
    "staged_proposal_created",
    "staged_proposal_validated",
    "candidate_review_projected",
    "operator_decision_recorded",
    "local_candidate_materialized",
    "replay_status_projected",
    "local_evidence_export_projected",
    "session_package_projected",
    "restore_status_projected",
  ];
}

function completeLocalOperatorWorkflowCapabilitySurface(): CompleteLocalOperatorWorkflowCapabilitySurface {
  return {
    localOnly: true,
    nonProduction: true,
    providerTrustGranted: false,
    actionExecutionAuthorized: false,
    readinessApproved: false,
    releaseApproved: false,
    deploymentApproved: false,
    publicUseApproved: false,
    replayRepairPerformed: false,
    recoveryPromotionPerformed: false,
  };
}

function workflowStep(
  step: CompleteLocalOperatorWorkflowStepKind,
  status: CompleteLocalOperatorWorkflowStepStatus,
  error: CompleteLocalOperatorWorkflowError | null,
  summary: string,
): CompleteLocalOperatorWorkflowStep {
  return { step, status, error, summary };
}

export function initialCompleteLocalOperatorWorkflowProjection(): CompleteLocalOperatorWorkflowProjection {
  return {
    status: "blocked",
    classification: "local_beta_workflow_only",
    currentStep: "provider_adapter_configured",
    nextRequiredStep: "provider_adapter_configured",
    currentBlockingStep: "provider_adapter_configured",
    currentError: "adapter_not_configured",
    steps: completeLocalOperatorWorkflowStepOrder().map((step) =>
      step === "provider_adapter_configured"
        ? workflowStep(
            step,
            "blocked",
            "adapter_not_configured",
            "Provider adapter declaration is missing.",
          )
        : workflowStep(
            step,
            "not_started",
            null,
            "Waiting for earlier local workflow steps.",
          ),
    ),
    rejectionReasons: [],
    evidenceSummary: {
      providerOutputPipelineStatus: "not_started",
      localCandidateMaterializationStatus: "not_materialized",
      replayStatus: "no_decision_recorded",
      localEvidenceExportStatus: "no_completed_run_evidence",
      sessionPackageStatus: "not_packaged",
      sessionHistoryStatus: "no_session_history",
      restoreStatus: "restore_not_requested",
    },
    boundaryStatuses: completeLocalOperatorWorkflowBoundaryStatuses(),
    capabilitySurface: completeLocalOperatorWorkflowCapabilitySurface(),
    localOnlyNote: "Complete local workflow is local-only and non-production.",
    noAuthorityNote:
      "Workflow completion does not approve readiness, release, deployment, public use, or production use. Provider output remains untrusted unless a later bounded phase explicitly changes that. Workflow status does not authorize actions. Replay is not repaired and recovery is not promoted.",
  };
}

export function classifyCompleteLocalOperatorWorkflowStep(
  state: LocalOperatorShellState,
  step: CompleteLocalOperatorWorkflowStepKind,
): CompleteLocalOperatorWorkflowStep {
  switch (step) {
    case "provider_adapter_configured":
      if (
        state.localProviderAdapterRegistry.lastValidation.status ===
        "adapter_declared_non_executing"
      )
        return workflowStep(
          step,
          "completed",
          null,
          "Provider adapter declaration is accepted.",
        );
      if (
        state.localProviderAdapterRegistry.lastValidation.status ===
        "registry_projected"
      )
        return workflowStep(
          step,
          "blocked",
          "adapter_not_configured",
          "Provider adapter declaration is missing.",
        );
      return workflowStep(
        step,
        "rejected",
        "adapter_not_configured",
        "Provider adapter declaration is rejected or invalid.",
      );
    case "adapter_dry_run_available":
      if (state.localProviderAdapterDryRun.status === "dry_run_executed")
        return workflowStep(
          step,
          "completed",
          null,
          "Controlled adapter dry run has executed.",
        );
      if (
        [
          "dry_run_rejected",
          "unsupported_adapter",
          "invalid_dry_run_request",
        ].includes(state.localProviderAdapterDryRun.status)
      )
        return workflowStep(
          step,
          "rejected",
          "invocation_rejected",
          `Controlled adapter dry run rejected: ${state.localProviderAdapterDryRun.errorCodes.join(", ")}.`,
        );
      if (
        state.localProviderAdapterRegistry.lastValidation.status ===
        "adapter_declared_non_executing"
      )
        return workflowStep(
          step,
          "available",
          null,
          "Controlled adapter dry run is available.",
        );
      return workflowStep(
        step,
        "not_started",
        null,
        "Controlled adapter dry run waits for adapter configuration.",
      );
    case "constrained_invocation_completed":
      if (
        state.constrainedLocalProviderInvocation.status ===
        "invocation_executed"
      )
        return workflowStep(
          step,
          "completed",
          null,
          "Constrained local provider invocation has executed.",
        );
      if (
        [
          "invocation_rejected",
          "unsupported_provider",
          "invalid_invocation_request",
        ].includes(state.constrainedLocalProviderInvocation.status)
      )
        return workflowStep(
          step,
          "rejected",
          "invocation_rejected",
          `Constrained local provider invocation rejected: ${state.constrainedLocalProviderInvocation.errorCodes.join(", ")}.`,
        );
      if (
        state.localProviderAdapterRegistry.lastValidation.status ===
        "adapter_declared_non_executing"
      )
        return workflowStep(
          step,
          "blocked",
          "invocation_missing",
          "Constrained local provider invocation is missing.",
        );
      return workflowStep(
        step,
        "not_started",
        null,
        "Invocation waits for provider adapter configuration.",
      );
    case "provider_output_pipeline_projected":
      if (state.localProviderOutputPipeline.status === "valid")
        return workflowStep(
          step,
          "completed",
          null,
          "Provider output pipeline is valid.",
        );
      if (state.localProviderOutputPipeline.status === "rejected")
        return workflowStep(
          step,
          "rejected",
          "provider_pipeline_blocked",
          `Provider output pipeline blocked: ${state.localProviderOutputPipeline.errors.join(", ")}.`,
        );
      return workflowStep(
        step,
        "blocked",
        "provider_pipeline_blocked",
        "Provider output pipeline projection is missing or incomplete.",
      );
    case "provider_output_validated":
      if (state.providerOutputValidation.status === "reviewable_untrusted")
        return workflowStep(
          step,
          "completed",
          null,
          "Provider output validation is reviewable and untrusted.",
        );
      if (state.providerOutputValidation.status === "not_validated")
        return workflowStep(
          step,
          "blocked",
          "provider_output_validation_missing",
          "Provider output validation is missing.",
        );
      return workflowStep(
        step,
        "rejected",
        "provider_output_validation_rejected",
        `Provider output validation rejected: ${state.providerOutputValidation.reasons.join(", ")}.`,
      );
    case "provider_output_reviewed":
      if (
        state.providerOutputValidation.reviewabilityStatus ===
        "reviewable_untrusted"
      )
        return workflowStep(
          step,
          "completed",
          null,
          "Provider output review surface is projected.",
        );
      if (
        state.providerOutputValidation.reviewabilityStatus ===
        "rejected_before_review"
      )
        return workflowStep(
          step,
          "rejected",
          "provider_output_validation_rejected",
          "Provider output was rejected before review.",
        );
      return workflowStep(
        step,
        "blocked",
        "provider_output_review_missing",
        "Provider output review is missing.",
      );
    case "staged_proposal_created":
      if (
        state.stagedCandidateConversionProposal.status ===
        "staged_proposal_created"
      )
        return workflowStep(
          step,
          "completed",
          null,
          "Staged candidate-conversion proposal exists.",
        );
      if (
        ["rejected_source_not_eligible", "invalid_proposal_request"].includes(
          state.stagedCandidateConversionProposal.status,
        )
      )
        return workflowStep(
          step,
          "rejected",
          "staged_proposal_missing",
          "Staged proposal creation was rejected.",
        );
      return workflowStep(
        step,
        "blocked",
        "staged_proposal_missing",
        "Staged candidate-conversion proposal is missing.",
      );
    case "staged_proposal_validated":
      if (
        state.stagedCandidateConversionValidation.status ===
        "staged_proposal_shape_valid"
      )
        return workflowStep(
          step,
          "completed",
          null,
          "Staged proposal shape and linkage are valid.",
        );
      if (state.stagedCandidateConversionValidation.status === "not_validated")
        return workflowStep(
          step,
          "blocked",
          "staged_proposal_validation_missing",
          "Staged proposal validation is missing.",
        );
      return workflowStep(
        step,
        "rejected",
        "staged_proposal_validation_rejected",
        `Staged proposal validation rejected: ${state.stagedCandidateConversionValidation.reasons.join(", ")}.`,
      );
    case "candidate_review_projected":
      return state.localProviderOutputPipeline.candidateReviewStatus ===
        "display_only"
        ? workflowStep(
            step,
            "completed",
            null,
            "Candidate review surface is projected as display-only.",
          )
        : workflowStep(
            step,
            "blocked",
            "candidate_review_missing",
            "Candidate review projection is missing.",
          );
    case "operator_decision_recorded":
      if (
        state.operatorCandidateDecision.status ===
        "approved_validated_staged_proposal"
      )
        return workflowStep(
          step,
          "completed",
          null,
          "Operator decision on validated staged proposal is recorded.",
        );
      if (state.operatorCandidateDecision.status === "no_operator_decision")
        return workflowStep(
          step,
          "blocked",
          "operator_decision_missing",
          "Operator decision is missing.",
        );
      return workflowStep(
        step,
        "rejected",
        "operator_decision_rejected",
        "Operator decision is rejected or invalid.",
      );
    case "local_candidate_materialized":
      if (state.localCandidateOutput.status === "local_candidate_materialized")
        return workflowStep(
          step,
          "completed",
          null,
          "Local candidate output is materialized.",
        );
      if (state.localCandidateOutput.status === "not_materialized")
        return workflowStep(
          step,
          "blocked",
          "local_candidate_not_materialized",
          "Local candidate output is not materialized.",
        );
      return workflowStep(
        step,
        "rejected",
        "local_candidate_not_materialized",
        `Local candidate materialization rejected: ${state.localCandidateOutput.error ?? "unknown"}.`,
      );
    case "replay_status_projected":
      return workflowStep(
        step,
        "completed",
        null,
        `Replay/status projection is ${state.run.decisionReplay.replayStatus}.`,
      );
    case "local_evidence_export_projected":
      return workflowStep(
        step,
        "completed",
        null,
        `Local evidence export is ${state.localSessionEvidenceExport.exportStatus}.`,
      );
    case "session_package_projected":
      return workflowStep(
        step,
        "completed",
        null,
        `Local session package status is ${state.localSessionPackageProjection.status}.`,
      );
    case "restore_status_projected":
      return workflowStep(
        step,
        "completed",
        null,
        `Restore/history status is ${state.localSessionHistoryProjection.status} / ${state.localSessionRestoreProjection.status}.`,
      );
  }
}

export function deriveCompleteLocalOperatorWorkflowProjection(
  state: LocalOperatorShellState,
): CompleteLocalOperatorWorkflowProjection {
  const steps = completeLocalOperatorWorkflowStepOrder().map((step) =>
    classifyCompleteLocalOperatorWorkflowStep(state, step),
  );
  const blocker =
    steps.find(
      (step) => step.status === "blocked" || step.status === "rejected",
    ) ?? null;
  const rejectionReasons = steps
    .filter((step) => step.status === "rejected")
    .map((step) => `${step.step}: ${step.error ?? "rejected"}`);
  const status: CompleteLocalOperatorWorkflowStatus =
    rejectionReasons.length > 0
      ? "rejected"
      : blocker
        ? "blocked"
        : state.localCandidateOutput.status === "local_candidate_materialized"
          ? "complete_local_workflow_projected"
          : steps.some((step) => step.status === "completed")
            ? "in_progress"
            : "not_started";
  const currentStep =
    blocker?.step ??
    steps.find((step) => step.status !== "completed")?.step ??
    null;
  return {
    status,
    classification: "local_beta_workflow_only",
    currentStep,
    nextRequiredStep: currentStep,
    currentBlockingStep: blocker?.step ?? null,
    currentError: blocker?.error ?? null,
    steps,
    rejectionReasons,
    evidenceSummary: {
      providerOutputPipelineStatus: state.localProviderOutputPipeline.status,
      localCandidateMaterializationStatus: state.localCandidateOutput.status,
      replayStatus: state.run.decisionReplay.replayStatus,
      localEvidenceExportStatus: state.localSessionEvidenceExport.exportStatus,
      sessionPackageStatus: state.localSessionPackageProjection.status,
      sessionHistoryStatus: state.localSessionHistoryProjection.status,
      restoreStatus: state.localSessionRestoreProjection.status,
    },
    boundaryStatuses: completeLocalOperatorWorkflowBoundaryStatuses(),
    capabilitySurface: completeLocalOperatorWorkflowCapabilitySurface(),
    localOnlyNote: "Complete local workflow is local-only and non-production.",
    noAuthorityNote:
      "Workflow completion does not approve readiness, release, deployment, public use, or production use. Provider output remains untrusted unless a later bounded phase explicitly changes that. Workflow status does not authorize actions. Replay is not repaired and recovery is not promoted.",
  };
}

export function deriveLocalSessionEvidenceExport(
  harnessStatus: string,
  nonProduction: boolean,
  run: LocalRunProjection,
  ledger: LocalDecisionLedger,
): LocalSessionEvidenceExport {
  const replay = deriveLocalDecisionReplayProjection(run, ledger);
  const exportStatus: LocalSessionEvidenceExportStatus =
    run.status === "idle"
      ? "no_completed_run_evidence"
      : ledger.records.length === 0
        ? "run_evidence_projected"
        : "decision_evidence_projected";
  const candidateId =
    run.candidate?.candidateId ?? "not_applicable_until_stub_run";
  const candidateOutputSummary = run.candidate
    ? `${run.candidate.title}: ${run.candidate.body}`
    : "no completed deterministic stub run candidate evidence";
  const validationStatus =
    run.validation?.validationStatus ?? "not_applicable_until_stub_run";
  const policyStatus =
    run.validation?.policyStatus ?? "not_applicable_until_stub_run";
  const exportPayload: Omit<
    LocalSessionEvidenceExport,
    "exportValidationStatus"
  > = {
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
    summary: `Local session evidence export preview for run ${run.runId} is ${exportStatus}; local only, non-production, and non-mutating.`,
  };
  return {
    ...exportPayload,
    exportValidationStatus: validateLocalSessionEvidenceExport(
      exportPayload,
      nonProduction,
    )
      ? "complete"
      : "incomplete",
  };
}

export function validateLocalSessionEvidenceExport(
  exportPayload:
    | Omit<LocalSessionEvidenceExport, "exportValidationStatus">
    | LocalSessionEvidenceExport,
  nonProduction: boolean,
): boolean {
  if (
    exportPayload.exportId.length === 0 ||
    exportPayload.shellStatus.length === 0 ||
    exportPayload.runId.length === 0
  )
    return false;
  if (exportPayload.exportClassification !== "local_session_evidence_only")
    return false;
  if (
    !nonProduction ||
    exportPayload.productionClassification !== "non-production"
  )
    return false;
  const markers = exportPayload.absenceMarkers;
  if (
    !markers.providerExecutionAbsent ||
    !markers.persistenceAbsent ||
    !markers.releaseAbsent ||
    !markers.deploymentAbsent ||
    !markers.signingAbsent ||
    !markers.publishingAbsent ||
    !markers.installerAbsent ||
    !markers.updateChannelAbsent ||
    !markers.publicUseAbsent ||
    !markers.readinessApprovalAbsent
  )
    return false;
  if (
    exportPayload.exportStatus !== "no_completed_run_evidence" &&
    (exportPayload.boundedContextSummary.length === 0 ||
      exportPayload.candidateId === "not_applicable_until_stub_run" ||
      exportPayload.validationStatus === "not_applicable_until_stub_run" ||
      exportPayload.policyStatus === "not_applicable_until_stub_run")
  )
    return false;
  if (
    exportPayload.exportStatus === "decision_evidence_projected" &&
    (exportPayload.decisionCount === 0 ||
      exportPayload.decisionRecords.length === 0 ||
      exportPayload.replayStatus === "no_decision_recorded")
  )
    return false;
  return true;
}

export function trialRunbookBoundaryStatuses(): readonly TrialRunbookBoundaryStatus[] {
  return [
    "local_trial_guidance_only",
    "non_public_trial_guidance",
    "no_trial_execution",
    "no_stop_condition_automation",
    "no_authority_activation",
    "no_readiness_approval",
    "no_release_approval",
    "no_deployment_approval",
    "no_public_use_approval",
    "no_production_approval",
    "no_action_execution",
    "no_replay_repair",
    "no_recovery_promotion",
  ];
}

function trialRunbookCapabilitySurface(): TrialRunbookCapabilitySurface {
  return {
    localOnly: true,
    nonPublic: true,
    executesTrial: false,
    automatesStopConditions: false,
    activatesAuthority: false,
    approvesReadiness: false,
    approvesRelease: false,
    approvesDeployment: false,
    approvesPublicUse: false,
    approvesProduction: false,
    executesActions: false,
    repairsReplay: false,
    promotesRecovery: false,
  };
}

function hasNamedOperator(
  packageProjection: ControlledInternalTrialPackageProjection,
): boolean {
  return packageProjection.namedOperatorParticipantSummary.some((item) =>
    item.startsWith("operator:"),
  );
}

function hasNamedParticipant(
  packageProjection: ControlledInternalTrialPackageProjection,
): boolean {
  return packageProjection.namedOperatorParticipantSummary.some((item) =>
    item.startsWith("participant:"),
  );
}

function packageIsInvalid(
  packageProjection: ControlledInternalTrialPackageProjection,
): boolean {
  return (
    packageProjection.status === "package_rejected" ||
    packageProjection.status === "invalid_package_input" ||
    packageProjection.validationStatus === "invalid" ||
    packageProjection.readBackValidationStatus === "invalid"
  );
}

function trialScopeStatus(
  packageProjection: ControlledInternalTrialPackageProjection,
): TrialOperatorRunbookStepStatus {
  return packageProjection.trialScopeSummary === "trial scope not provided"
    ? "blocked"
    : "completed";
}

function namedOperatorStatus(
  packageProjection: ControlledInternalTrialPackageProjection,
): TrialOperatorRunbookStepStatus {
  return hasNamedOperator(packageProjection) ? "completed" : "blocked";
}

function namedParticipantStatus(
  packageProjection: ControlledInternalTrialPackageProjection,
): TrialOperatorRunbookStepStatus {
  return hasNamedParticipant(packageProjection) ? "completed" : "blocked";
}

function trialRunbookStepOrder(): readonly TrialOperatorRunbookStepKind[] {
  return [
    "confirm_local_beta_workflow",
    "confirm_controlled_trial_package",
    "confirm_trial_scope",
    "confirm_named_internal_operator",
    "confirm_named_trial_participant",
    "review_stop_conditions",
    "review_current_blocker",
    "review_failure_drill",
    "review_restore_status",
    "review_replay_status",
    "review_evidence_export_status",
    "prepare_manual_trial_notes",
    "confirm_no_public_release",
    "confirm_no_production_approval",
  ];
}

function addTrialFailureCategory(
  categories: TrialFailureCategoryProjection[],
  category: TrialFailureCategory,
  severity: TrialFailureSeverity,
  summary: string,
): void {
  if (!categories.some((entry) => entry.category === category))
    categories.push({ category, severity, summary });
}

export function deriveTrialStopConditionDrills(
  state: LocalOperatorShellState,
): readonly TrialStopConditionDrill[] {
  return state.controlledInternalTrialPackageProjection.stopConditionSummary.map(
    (marker) => ({
      marker,
      status: "manual_action_required",
      guidance: `If stop-condition marker ${marker} is present during manual preparation, pause the local trial workflow, record notes, and involve the trial coordinator before continuing.`,
      enforcementAutomated: false,
    }),
  );
}

export function deriveTrialEscalationGuidance(
  categories: readonly TrialFailureCategoryProjection[],
): readonly TrialEscalationGuidance[] {
  const has = (category: TrialFailureCategory) =>
    categories.some((entry) => entry.category === category);
  const guidance: TrialEscalationGuidance[] = [
    {
      role: "operator_manual_review",
      categories: [
        "candidate_materialization_missing",
        "evidence_export_missing",
        "replay_status_incomplete",
      ],
      guidance:
        "Operator manual review checks local notes, missing materialization, replay/status, and evidence export gaps without executing remediation.",
      descriptiveOnly: true,
    },
  ];
  if (
    has("provider_pipeline_blocked") ||
    has("provider_output_validation_rejected") ||
    has("security_escalation_required")
  ) {
    guidance.push({
      role: "security_reviewer",
      categories: [
        "provider_pipeline_blocked",
        "provider_output_validation_rejected",
        "security_escalation_required",
      ],
      guidance:
        "Security reviewer guidance applies to provider pipeline, provider output validation, and security-sensitive rejection categories; it does not trust provider output.",
      descriptiveOnly: true,
    });
  }
  if (
    has("release_steward_review_required") ||
    categories.some(
      (entry) =>
        entry.summary.includes("release") ||
        entry.summary.includes("deployment"),
    )
  ) {
    guidance.push({
      role: "release_steward",
      categories: ["release_steward_review_required"],
      guidance:
        "Release steward guidance applies only to reviewing release/deployment/readiness claims; it does not create release or deployment artifacts.",
      descriptiveOnly: true,
    });
  }
  if (
    has("workflow_blocked") ||
    has("workflow_rejected") ||
    has("trial_coordinator_review_required") ||
    has("stop_condition_present") ||
    has("no_trial_package")
  ) {
    guidance.push({
      role: "trial_coordinator",
      categories: [
        "workflow_blocked",
        "workflow_rejected",
        "trial_coordinator_review_required",
        "stop_condition_present",
        "no_trial_package",
      ],
      guidance:
        "Trial coordinator guidance applies to workflow blockers, missing package preparation, and stop-condition drill review; it does not approve use authority.",
      descriptiveOnly: true,
    });
  }
  return guidance;
}

export function deriveTrialFailureDrillProjection(
  state: LocalOperatorShellState,
): TrialFailureDrillProjection {
  const categories: TrialFailureCategoryProjection[] = [];
  const packageProjection = state.controlledInternalTrialPackageProjection;
  if (packageProjection.status === "not_packaged")
    addTrialFailureCategory(
      categories,
      "no_trial_package",
      "blocked",
      "Controlled internal trial package has not been projected.",
    );
  if (packageIsInvalid(packageProjection))
    addTrialFailureCategory(
      categories,
      "trial_package_validation_failure",
      "rejected",
      `Trial package validation errors: ${packageProjection.validationErrors.join(", ")}.`,
    );
  if (packageProjection.readBackValidationStatus === "invalid")
    addTrialFailureCategory(
      categories,
      "trial_package_read_back_failure",
      "rejected",
      "Trial package read-back validation is invalid.",
    );
  if (packageProjection.trialScopeSummary === "trial scope not provided")
    addTrialFailureCategory(
      categories,
      "missing_trial_scope",
      "blocked",
      "Trial scope is missing.",
    );
  if (!hasNamedOperator(packageProjection))
    addTrialFailureCategory(
      categories,
      "missing_named_operator",
      "blocked",
      "Named internal operator metadata is missing.",
    );
  if (!hasNamedParticipant(packageProjection))
    addTrialFailureCategory(
      categories,
      "missing_named_participant",
      "blocked",
      "Named trial participant metadata is missing.",
    );
  if (packageProjection.stopConditionSummary.length === 0)
    addTrialFailureCategory(
      categories,
      "missing_stop_conditions",
      "blocked",
      "Stop-condition markers are missing.",
    );
  else
    addTrialFailureCategory(
      categories,
      "stop_condition_present",
      "manual_action",
      "Stop-condition drill markers are present for operator review; enforcement is guidance only.",
    );
  if (state.completeLocalOperatorWorkflow.status === "blocked")
    addTrialFailureCategory(
      categories,
      "workflow_blocked",
      "blocked",
      `Workflow is blocked at ${state.completeLocalOperatorWorkflow.currentBlockingStep ?? "unknown"}.`,
    );
  if (state.completeLocalOperatorWorkflow.status === "rejected")
    addTrialFailureCategory(
      categories,
      "workflow_rejected",
      "rejected",
      `Workflow rejection summary: ${state.completeLocalOperatorWorkflow.rejectionReasons.join(", ")}.`,
    );
  if (
    state.localProviderOutputPipeline.status === "blocked" ||
    state.localProviderOutputPipeline.status === "rejected"
  )
    addTrialFailureCategory(
      categories,
      "provider_pipeline_blocked",
      state.localProviderOutputPipeline.status === "rejected"
        ? "rejected"
        : "blocked",
      `Provider output pipeline is ${state.localProviderOutputPipeline.status}.`,
    );
  if (state.providerOutputValidation.status === "rejected") {
    addTrialFailureCategory(
      categories,
      "provider_output_validation_rejected",
      "rejected",
      "Provider output validation is rejected.",
    );
    addTrialFailureCategory(
      categories,
      "security_escalation_required",
      "manual_action",
      "Provider output validation rejection requires security-review guidance.",
    );
  }
  if (
    state.stagedCandidateConversionValidation.status ===
    "rejected_staged_proposal"
  )
    addTrialFailureCategory(
      categories,
      "staged_validation_rejected",
      "rejected",
      "Staged proposal validation is rejected.",
    );
  if (
    state.operatorCandidateDecision.status ===
    "rejected_validated_staged_proposal"
  )
    addTrialFailureCategory(
      categories,
      "operator_decision_rejected",
      "manual_action",
      "Operator decision recorded a rejection of the validated staged proposal.",
    );
  if (state.localCandidateOutput.status === "not_materialized")
    addTrialFailureCategory(
      categories,
      "candidate_materialization_missing",
      "manual_action",
      "Local candidate materialization is missing.",
    );
  if (
    state.localSessionRestoreProjection.validationStatus === "invalid" ||
    state.localSessionRestoreProjection.status === "restore_rejected"
  )
    addTrialFailureCategory(
      categories,
      "restore_projection_rejected",
      "rejected",
      "Restore/history projection is rejected or invalid.",
    );
  if (
    state.run.decisionReplay.integrityStatus === "inconsistent" ||
    state.run.decisionReplay.replayStatus === "no_decision_recorded"
  )
    addTrialFailureCategory(
      categories,
      "replay_status_incomplete",
      "manual_action",
      `Replay/status projection is ${state.run.decisionReplay.replayStatus}.`,
    );
  if (state.localSessionEvidenceExport.exportValidationStatus === "incomplete")
    addTrialFailureCategory(
      categories,
      "evidence_export_missing",
      "manual_action",
      "Local evidence export is incomplete.",
    );
  if (
    packageProjection.validationErrors.some((error) =>
      /release|deployment|readiness|public|production/.test(error),
    )
  )
    addTrialFailureCategory(
      categories,
      "release_steward_review_required",
      "manual_action",
      "Package contains release/deployment/readiness/public/production-use claim rejection summary.",
    );
  if (state.completeLocalOperatorWorkflow.currentBlockingStep)
    addTrialFailureCategory(
      categories,
      "trial_coordinator_review_required",
      "manual_action",
      "Workflow blocker requires trial coordinator guidance before manual preparation continues.",
    );
  const severityOrder: Record<TrialFailureSeverity, number> = {
    info: 0,
    manual_action: 1,
    blocked: 2,
    rejected: 3,
  };
  const highestSeverity = categories.reduce<TrialFailureSeverity>(
    (highest, entry) =>
      severityOrder[entry.severity] > severityOrder[highest]
        ? entry.severity
        : highest,
    "info",
  );
  const status: TrialFailureDrillStatus = categories.some(
    (entry) => entry.category === "stop_condition_present",
  )
    ? "stop_condition_drill_required"
    : categories.length === 0
      ? "no_failures_projected"
      : "failure_drill_required";
  return {
    status,
    highestSeverity,
    categories,
    stopConditionDrills: deriveTrialStopConditionDrills(state),
    escalationGuidance: deriveTrialEscalationGuidance(categories),
    manualActionGuidance: categories
      .filter(
        (entry) =>
          entry.severity === "manual_action" || entry.severity === "blocked",
      )
      .map(
        (entry) =>
          `${entry.category}: review manually; no remediation is executed.`,
      ),
    rejectionSummary: categories
      .filter((entry) => entry.severity === "rejected")
      .map((entry) => `${entry.category}: ${entry.summary}`),
    boundaryStatuses: trialRunbookBoundaryStatuses(),
    noAutomationNote:
      "Stop conditions are guidance only; enforcement is not automated in Phase 162.",
  };
}

export function classifyTrialRunbookStep(
  state: LocalOperatorShellState,
  step: TrialOperatorRunbookStepKind,
): TrialOperatorRunbookStep {
  const packageProjection = state.controlledInternalTrialPackageProjection;
  const failureDrill = deriveTrialFailureDrillProjection(state);
  if (step === "confirm_local_beta_workflow") {
    const status =
      state.completeLocalOperatorWorkflow.status === "rejected"
        ? "rejected"
        : state.completeLocalOperatorWorkflow.status === "blocked"
          ? "blocked"
          : state.completeLocalOperatorWorkflow.status === "not_started"
            ? "manual_action_required"
            : "completed";
    return {
      step,
      status,
      summary: `Complete local workflow status is ${state.completeLocalOperatorWorkflow.status}.`,
    };
  }
  if (step === "confirm_controlled_trial_package") {
    if (packageProjection.status === "not_packaged")
      return {
        step,
        status: "blocked",
        summary:
          "Controlled internal trial package is required before manual trial preparation.",
      };
    if (packageIsInvalid(packageProjection))
      return {
        step,
        status: "rejected",
        summary: `Controlled internal trial package validation is ${packageProjection.validationStatus}.`,
      };
    return {
      step,
      status: "completed",
      summary: `Controlled internal trial package status is ${packageProjection.status}.`,
    };
  }
  if (step === "confirm_trial_scope")
    return {
      step,
      status: trialScopeStatus(packageProjection),
      summary: `Trial scope: ${packageProjection.trialScopeSummary}.`,
    };
  if (step === "confirm_named_internal_operator")
    return {
      step,
      status: namedOperatorStatus(packageProjection),
      summary: `Named operator metadata: ${packageProjection.namedOperatorParticipantSummary.join(", ")}.`,
    };
  if (step === "confirm_named_trial_participant")
    return {
      step,
      status: namedParticipantStatus(packageProjection),
      summary: `Named participant metadata: ${packageProjection.namedOperatorParticipantSummary.join(", ")}.`,
    };
  if (step === "review_stop_conditions")
    return packageProjection.stopConditionSummary.length === 0
      ? {
          step,
          status: "blocked",
          summary:
            "Stop-condition markers are missing from the package projection.",
        }
      : {
          step,
          status: "manual_action_required",
          summary: `Review ${packageProjection.stopConditionSummary.length} stop-condition drill marker(s); guidance only.`,
        };
  if (step === "review_current_blocker")
    return state.completeLocalOperatorWorkflow.currentBlockingStep
      ? {
          step,
          status: "manual_action_required",
          summary: `Current workflow blocker is ${state.completeLocalOperatorWorkflow.currentBlockingStep}.`,
        }
      : {
          step,
          status: "completed",
          summary: "No current complete-workflow blocker is projected.",
        };
  if (step === "review_failure_drill")
    return failureDrill.categories.length === 0
      ? {
          step,
          status: "completed",
          summary: "No failure categories are projected.",
        }
      : {
          step,
          status: "manual_action_required",
          summary: `Review ${failureDrill.categories.length} failure drill categorization(s).`,
        };
  if (step === "review_restore_status")
    return {
      step,
      status:
        state.localSessionRestoreProjection.validationStatus === "invalid"
          ? "rejected"
          : "completed",
      summary: `Restore/history status is ${state.localSessionRestoreProjection.status} / ${state.localSessionHistoryProjection.status}.`,
    };
  if (step === "review_replay_status")
    return {
      step,
      status:
        state.run.decisionReplay.integrityStatus === "inconsistent"
          ? "rejected"
          : state.run.decisionReplay.replayStatus === "no_decision_recorded"
            ? "manual_action_required"
            : "completed",
      summary: `Replay/status projection is ${state.run.decisionReplay.replayStatus} with ${state.run.decisionReplay.integrityStatus} integrity.`,
    };
  if (step === "review_evidence_export_status")
    return {
      step,
      status:
        state.localSessionEvidenceExport.exportValidationStatus === "incomplete"
          ? "manual_action_required"
          : "completed",
      summary: `Local evidence export is ${state.localSessionEvidenceExport.exportStatus}.`,
    };
  if (step === "prepare_manual_trial_notes")
    return {
      step,
      status: "manual_action_required",
      summary:
        "Prepare manual notes; this projection does not execute or authorize trial operation.",
    };
  if (step === "confirm_no_public_release")
    return {
      step,
      status: "completed",
      summary:
        "Boundary confirms no public release, publication, signing, or deployment behavior.",
    };
  return {
    step,
    status: "completed",
    summary:
      "Boundary confirms no controlled human-use, public-use, production-use, release, deployment, or readiness approval.",
  };
}

export function initialTrialFailureDrillProjection(): TrialFailureDrillProjection {
  return {
    status: "no_failures_projected",
    highestSeverity: "info",
    categories: [],
    stopConditionDrills: [],
    escalationGuidance: [],
    manualActionGuidance: [],
    rejectionSummary: [],
    boundaryStatuses: trialRunbookBoundaryStatuses(),
    noAutomationNote:
      "Stop conditions are guidance only; enforcement is not automated in Phase 162.",
  };
}

export function initialTrialOperatorRunbookProjection(): TrialOperatorRunbookProjection {
  return {
    status: "not_available",
    currentStep: null,
    currentBlocker: {
      step: null,
      workflowStep: null,
      workflowError: null,
      guidance: "Current blocker guidance: projection not yet derived.",
    },
    steps: [],
    trialPackageStatus: "not_packaged",
    trialPackageId: null,
    trialPackageValidationStatus: "not_validated",
    trialPackageReadBackStatus: null,
    trialScopeStatus: "not_started",
    namedOperatorStatus: "not_started",
    namedParticipantStatus: "not_started",
    stopConditionSummary: [],
    localWorkflowStatus: "not_started",
    localCandidateMaterializationStatus: "not_materialized",
    providerOutputPipelineStatus: "not_projected",
    replayStatusSummary: "not projected",
    localEvidenceExportSummary: "not projected",
    restoreHistoryStatus: "not projected",
    failureDrill: initialTrialFailureDrillProjection(),
    boundaryStatuses: trialRunbookBoundaryStatuses(),
    capabilitySurface: trialRunbookCapabilitySurface(),
    localOnlyNonPublicNote:
      "Trial operator runbook is local-only and non-public.",
    noTrialExecutionNote: "This runbook does not start a controlled trial.",
    noAuthorityNote:
      "This runbook does not approve controlled human use, public use, production use, release, deployment, or readiness.",
  };
}

export function deriveTrialOperatorRunbookProjection(
  state: LocalOperatorShellState,
): TrialOperatorRunbookProjection {
  const steps = trialRunbookStepOrder().map((step) =>
    classifyTrialRunbookStep(state, step),
  );
  const currentStep =
    steps.find((step) => step.status !== "completed")?.step ?? null;
  const currentBlockerStep =
    steps.find(
      (step) => step.status === "blocked" || step.status === "rejected",
    )?.step ?? null;
  const failureDrill = deriveTrialFailureDrillProjection(state);
  const packageProjection = state.controlledInternalTrialPackageProjection;
  const status: TrialOperatorRunbookStatus =
    packageProjection.status === "not_packaged"
      ? "trial_package_required"
      : packageIsInvalid(packageProjection) || currentBlockerStep
        ? "blocked"
        : failureDrill.categories.length > 0
          ? "failure_drill_required"
          : steps.some((step) => step.status === "manual_action_required")
            ? "trial_operator_runbook_projected"
            : "ready_for_manual_trial_preparation";
  const workflowStep = state.completeLocalOperatorWorkflow.currentBlockingStep;
  const guidance =
    currentBlockerStep && workflowStep
      ? `Current blocker guidance: review runbook step ${currentBlockerStep} and workflow step ${workflowStep}; manual action only.`
      : currentBlockerStep
        ? `Current blocker guidance: review runbook step ${currentBlockerStep}; manual action only.`
        : workflowStep
          ? `Current blocker guidance: review workflow step ${workflowStep}; manual action only.`
          : "Current blocker guidance: no blocking step is projected; continue manual review without executing trial authority.";
  return {
    status,
    currentStep,
    currentBlocker: {
      step: currentBlockerStep,
      workflowStep,
      workflowError: state.completeLocalOperatorWorkflow.currentError,
      guidance,
    },
    steps,
    trialPackageStatus: packageProjection.status,
    trialPackageId: packageProjection.packageId,
    trialPackageValidationStatus: packageProjection.validationStatus,
    trialPackageReadBackStatus: packageProjection.readBackValidationStatus,
    trialScopeStatus: trialScopeStatus(packageProjection),
    namedOperatorStatus: namedOperatorStatus(packageProjection),
    namedParticipantStatus: namedParticipantStatus(packageProjection),
    stopConditionSummary: packageProjection.stopConditionSummary,
    localWorkflowStatus: state.completeLocalOperatorWorkflow.status,
    localCandidateMaterializationStatus: state.localCandidateOutput.status,
    providerOutputPipelineStatus: state.localProviderOutputPipeline.status,
    replayStatusSummary: state.run.decisionReplay.summary,
    localEvidenceExportSummary: state.localSessionEvidenceExport.summary,
    restoreHistoryStatus: `${state.localSessionRestoreProjection.status} / ${state.localSessionHistoryProjection.status}`,
    failureDrill,
    boundaryStatuses: trialRunbookBoundaryStatuses(),
    capabilitySurface: trialRunbookCapabilitySurface(),
    localOnlyNonPublicNote:
      "Trial operator runbook is local-only and non-public.",
    noTrialExecutionNote: "This runbook does not start a controlled trial.",
    noAuthorityNote:
      "This runbook does not approve controlled human use, public use, production use, release, deployment, or readiness.",
  };
}

function attachLocalSessionEvidenceExport(
  state: Omit<
    LocalOperatorShellState,
    | "localSessionEvidenceExport"
    | "completeLocalOperatorWorkflow"
    | "trialOperatorRunbook"
    | "trialFailureDrill"
    | "trialSessionEvidenceProjection"
    | "trialReplayRestoreVerification"
    | "controlledInternalTrialExecution"
    | "trialObservability"
    | "trialErrorReport"
    | "trialEvidenceReview"
    | "releaseCandidatePreparation"
  >,
): LocalOperatorShellState {
  const next = {
    ...state,
    localProviderOutputPipeline: deriveLocalProviderOutputPipelineProjection(
      state as LocalOperatorShellState,
    ),
  };
  const withExport: LocalOperatorShellState = {
    ...next,
    localSessionEvidenceExport: deriveLocalSessionEvidenceExport(
      next.harnessStatus,
      next.nonProduction,
      next.run,
      next.decisionLedger,
    ),
    completeLocalOperatorWorkflow:
      initialCompleteLocalOperatorWorkflowProjection(),
    trialOperatorRunbook: initialTrialOperatorRunbookProjection(),
    trialFailureDrill: initialTrialFailureDrillProjection(),
    trialSessionEvidenceProjection: initialTrialSessionEvidenceProjection(),
    trialReplayRestoreVerification:
      initialTrialReplayRestoreVerificationProjection(),
    controlledInternalTrialExecution:
      initialControlledInternalTrialExecutionProjection(),
    trialObservability: initialTrialObservabilityProjection(),
    trialErrorReport: initialTrialErrorReportProjection(),
    trialEvidenceReview: initialTrialEvidenceReviewProjection(),
    releaseCandidatePreparation: deriveReleaseCandidatePreparationProjection({
      ...(state as LocalOperatorShellState),
      localSessionEvidenceExport: deriveLocalSessionEvidenceExport(
        next.harnessStatus,
        next.nonProduction,
        next.run,
        next.decisionLedger,
      ),
      completeLocalOperatorWorkflow:
        initialCompleteLocalOperatorWorkflowProjection(),
      trialOperatorRunbook: initialTrialOperatorRunbookProjection(),
      trialFailureDrill: initialTrialFailureDrillProjection(),
      trialSessionEvidenceProjection: initialTrialSessionEvidenceProjection(),
      trialReplayRestoreVerification:
        initialTrialReplayRestoreVerificationProjection(),
      controlledInternalTrialExecution:
        initialControlledInternalTrialExecutionProjection(),
      trialObservability: initialTrialObservabilityProjection(),
      trialErrorReport: initialTrialErrorReportProjection(),
      trialEvidenceReview: initialTrialEvidenceReviewProjection(),
    }),
  };
  const withWorkflow: LocalOperatorShellState = {
    ...withExport,
    completeLocalOperatorWorkflow:
      deriveCompleteLocalOperatorWorkflowProjection(withExport),
  };
  const refreshed = refreshTrialObservabilityState({
    ...withWorkflow,
    trialFailureDrill: deriveTrialFailureDrillProjection(withWorkflow),
    trialOperatorRunbook: deriveTrialOperatorRunbookProjection(withWorkflow),
    trialSessionEvidenceProjection:
      (state as Partial<LocalOperatorShellState>)
        .trialSessionEvidenceProjection ??
      initialTrialSessionEvidenceProjection(),
    trialReplayRestoreVerification:
      (state as Partial<LocalOperatorShellState>)
        .trialReplayRestoreVerification ??
      initialTrialReplayRestoreVerificationProjection(),
    controlledInternalTrialExecution:
      (state as Partial<LocalOperatorShellState>)
        .controlledInternalTrialExecution ??
      initialControlledInternalTrialExecutionProjection(),
    trialObservability:
      (state as Partial<LocalOperatorShellState>).trialObservability ??
      initialTrialObservabilityProjection(),
    trialErrorReport:
      (state as Partial<LocalOperatorShellState>).trialErrorReport ??
      initialTrialErrorReportProjection(),
    trialEvidenceReview:
      (state as Partial<LocalOperatorShellState>).trialEvidenceReview ??
      initialTrialEvidenceReviewProjection(),
    releaseCandidatePreparation:
      (state as Partial<LocalOperatorShellState>).releaseCandidatePreparation ??
      deriveReleaseCandidatePreparationProjection(withWorkflow),
  });
  return {
    ...refreshed,
    releaseCandidatePreparation:
      deriveReleaseCandidatePreparationProjection(refreshed),
  };
}

function controlledInternalTrialClaimErrors(
  request: ControlledInternalTrialExecutionRequest,
): ControlledInternalTrialRunError[] {
  const errors: ControlledInternalTrialRunError[] = [];
  if (request.claimsReadiness) errors.push("readiness_claim_rejected");
  if (request.claimsRelease) errors.push("release_claim_rejected");
  if (request.claimsDeployment) errors.push("deployment_claim_rejected");
  if (request.claimsPublicUse) errors.push("public_use_claim_rejected");
  if (request.claimsProductionUse) errors.push("production_use_claim_rejected");
  if (request.claimsProviderTrust) errors.push("provider_trust_claim_rejected");
  if (request.claimsActionAuthorization)
    errors.push("action_authorization_claim_rejected");
  if (request.claimsReplayRepair) errors.push("replay_repair_claim_rejected");
  if (request.claimsRecoveryPromotion)
    errors.push("recovery_promotion_claim_rejected");
  if (request.claimsSigning) errors.push("signing_claim_rejected");
  if (request.claimsPublishing) errors.push("publishing_claim_rejected");
  if (request.claimsReleaseArtifact)
    errors.push("release_artifact_claim_rejected");
  return errors;
}

function controlledInternalTrialRunStepOrder(): readonly ControlledInternalTrialRunStep[] {
  return [
    "verify_trial_package",
    "verify_runbook",
    "verify_failure_drill",
    "verify_trial_session_evidence",
    "verify_replay_restore",
    "verify_complete_local_workflow",
    "observe_stop_conditions",
    "record_manual_operator_step",
    "project_trial_run_summary",
    "project_trial_evidence_linkage",
    "close_local_trial_run",
  ];
}

function deterministicTrialExecutionId(
  state: LocalOperatorShellState,
  request: ControlledInternalTrialExecutionRequest,
): string {
  const basis = `${state.controlledInternalTrialPackageProjection.packageId ?? "none"}|${state.trialOperatorRunbook.status}|${state.trialSessionEvidenceProjection.evidenceId ?? "none"}|${state.trialReplayRestoreVerification.verificationId ?? "none"}|${state.completeLocalOperatorWorkflow.status}|${request.operatorStepRecorded === true}|${request.stopConditionObserved === true}`;
  let hash = 0x811c9dc5;
  for (const char of basis)
    hash = Math.imul(hash ^ char.charCodeAt(0), 0x01000193) >>> 0;
  return `controlled-internal-trial-run-${hash.toString(16).padStart(8, "0")}`;
}

export function startControlledInternalTrialExecution(
  state: LocalOperatorShellState,
  request: ControlledInternalTrialExecutionRequest = {},
): LocalOperatorShellState {
  const base = deriveControlledInternalTrialExecutionProjection(state);
  const errors = [
    ...base.rejectionReasons,
    ...controlledInternalTrialClaimErrors(request),
  ];
  const stopped = request.stopConditionObserved === true;
  const status: ControlledInternalTrialRunStatus =
    errors.length > 0
      ? "trial_run_rejected"
      : stopped
        ? "trial_run_blocked"
        : request.operatorStepRecorded === true
          ? "trial_run_completed_locally"
          : "trial_run_in_progress";
  const run: ControlledInternalTrialRunProjection = {
    runId: deterministicTrialExecutionId(state, request),
    status,
    currentStep: stopped
      ? "observe_stop_conditions"
      : request.operatorStepRecorded
        ? null
        : "record_manual_operator_step",
    nextStep:
      stopped || request.operatorStepRecorded
        ? null
        : "record_manual_operator_step",
    steps: controlledInternalTrialRunStepOrder().map((step) => ({
      step,
      status:
        errors.length > 0
          ? "rejected"
          : stopped &&
              [
                "observe_stop_conditions",
                "record_manual_operator_step",
                "project_trial_run_summary",
                "project_trial_evidence_linkage",
                "close_local_trial_run",
              ].includes(step)
            ? "blocked"
            : step === "record_manual_operator_step" &&
                !request.operatorStepRecorded
              ? "manual_action_required"
              : "completed",
      summary: step,
    })),
    currentBlocker: stopped
      ? "stop_condition_observed"
      : (errors[0] ??
        (request.operatorStepRecorded ? null : "manual_operator_step_missing")),
    rejectionReasons: errors,
    stopConditionObservation: {
      status: stopped
        ? "stop_condition_observed"
        : "no_stop_condition_observed",
      observed: stopped,
      markers:
        state.controlledInternalTrialPackageProjection.stopConditionSummary,
      enforcementAutomated: false,
    },
    manualOperatorStepStatus: request.operatorStepRecorded
      ? "recorded"
      : stopped || errors.length > 0
        ? "manual_operator_step_missing"
        : "manual_action_required",
    evidenceLinkage: base.evidenceLinkage,
    summary: `Bounded local controlled internal trial run status: ${status}.`,
  };
  return refreshTrialObservabilityState({
    ...state,
    controlledInternalTrialExecution: {
      ...base,
      status:
        controlledInternalTrialClaimErrors(request).length > 0
          ? "invalid_trial_run_request"
          : status,
      activeRun:
        errors.length === 0 && !stopped
          ? run
          : state.controlledInternalTrialExecution.activeRun,
      lastRejectedRun: errors.length > 0 || stopped ? run : null,
      currentBlocker: run.currentBlocker,
      rejectionReasons: errors,
    },
  });
}

export function stepControlledInternalTrialExecution(
  state: LocalOperatorShellState,
  request: ControlledInternalTrialExecutionRequest = {},
): LocalOperatorShellState {
  return startControlledInternalTrialExecution(state, {
    ...request,
    operatorStepRecorded: true,
  });
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
      decisionReplay,
    },
    decisionLedger,
    providerConfiguration: initialLocalProviderConfiguration(),
    localProviderAdapterRegistry: initialLocalProviderAdapterRegistry(),
    localProviderAdapterDryRun: initialLocalProviderAdapterDryRunProjection(),
    constrainedLocalProviderInvocation:
      initialConstrainedLocalProviderInvocationProjection(),
    localProviderOutputPipeline: initialLocalProviderOutputPipelineProjection(),
    providerExecution: initialLocalProviderExecutionProjection(),
    providerOutputValidation: initialLocalProviderOutputValidationProjection(),
    stagedCandidateConversionProposal:
      initialStagedCandidateConversionProposalProjection(),
    stagedCandidateConversionValidation:
      initialStagedCandidateConversionValidationProjection(),
    operatorCandidateDecision: initialOperatorCandidateDecisionProjection(),
    localCandidateOutput: initialLocalCandidateOutputProjection(),
    phase150CodeProductionHandoff: {
      handoffId:
        "phase-150-code-production-handoff-not_configured-not_validated-no_operator_decision",
      status: "phase_150_code_production_handoff",
      implementedCapabilityEvidence: [],
      remainingProductionGradeGaps: phase150RemainingProductionGaps(),
      remapRecommendations: [],
      phase149RoadmapEditStatus: "phase_149_does_not_edit_roadmap_files",
    },
    localSessionPackageProjection: initialLocalSessionPackageProjection(),
    localSessionHistoryProjection: initialLocalSessionHistoryProjection(),
    localSessionRestoreProjection: initialLocalSessionRestoreProjection(),
    controlledInternalTrialPackageProjection:
      initialControlledInternalTrialPackageProjection(),
  });
  return {
    ...state,
    phase150CodeProductionHandoff: derivePhase150CodeProductionHandoff(state),
  };
}

export function startDeterministicStubRun(
  state: LocalOperatorShellState,
): LocalOperatorShellState {
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
        "network=disabled",
      ],
      candidate: {
        candidateId: "candidate-local-stub-133",
        title: "Deterministic local stub candidate",
        body: "AJENTIC local shell rendered a deterministic candidate from a Rust-owned projection fixture.",
        providerKind: "deterministic_stub",
        providerOutputTrusted: false,
        providerExecutionEnabled: false,
      },
      validation: {
        validationId: "validation-local-stub-133",
        policyStatus: "pass_for_local_stub_review",
        validationStatus: "pass_for_local_stub_review",
        reason:
          "deterministic fixture satisfies local shell display checks only",
        authority: "rust",
      },
      selectedIntent: null,
      timeline: [
        "idle local harness initialized",
        "deterministic stub run started",
        "candidate output projected",
        "validation and policy projection completed",
      ],
      decisionTimeline: projectLocalDecisionTimeline(state.decisionLedger),
      decisionReplay: deriveLocalDecisionReplayProjection(
        state.run,
        state.decisionLedger,
      ),
      replayStatus: deriveLocalDecisionReplayProjection(
        state.run,
        state.decisionLedger,
      ).replayStatus,
    },
  });
}

export function applyLocalOperatorIntent(
  state: LocalOperatorShellState,
  intent: LocalOperatorIntent,
): LocalOperatorIntentResult {
  const rejection = (reason: string): LocalOperatorIntentResult => ({
    status: "rejected",
    reason,
    state,
  });
  if (state.run.status === "idle") return rejection("run_not_started");
  if (intent.kind !== "approve" && intent.kind !== "reject")
    return rejection("unknown_intent_kind");
  if (intent.operatorId.length === 0) return rejection("empty_operator_id");
  if (intent.reason.length === 0) return rejection("empty_reason");
  if (intent.targetRunId !== state.run.runId)
    return rejection("target_mismatch");
  if (!state.run.candidate) return rejection("run_not_started");
  const candidate = state.run.candidate;
  if (
    (intent.targetCandidateId ?? "candidate-local-stub-133") !==
    candidate.candidateId
  )
    return rejection("candidate_target_mismatch");
  if (intent.requestsAuthorityGrant)
    return rejection("authority_grant_rejected");
  if (intent.requestsProviderExecution)
    return rejection("provider_execution_rejected");
  if (intent.claimsReadiness) return rejection("readiness_claim_rejected");

  if (
    state.decisionLedger.records.some(
      (record) =>
        record.runId === intent.targetRunId &&
        record.candidateId === candidate.candidateId,
    )
  ) {
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
    reason: intent.reason,
  };
  const decisionLedger: LocalDecisionLedger = {
    records: [...state.decisionLedger.records, decisionRecord],
  };

  const nextRun: LocalRunProjection = {
    ...state.run,
    status: "intent_recorded",
    selectedIntent: intent.kind,
    decisionTimeline: projectLocalDecisionTimeline(decisionLedger),
    timeline: [
      ...state.run.timeline,
      `operator intent recorded: ${intent.kind} by ${intent.operatorId} as decision ${decisionRecord.decisionId}`,
    ],
  };
  const decisionReplay = deriveLocalDecisionReplayProjection(
    nextRun,
    decisionLedger,
  );

  return {
    status: "accepted",
    reason: "local_operator_intent_recorded",
    state: attachLocalSessionEvidenceExport({
      ...state,
      decisionLedger,
      run: {
        ...nextRun,
        decisionReplay,
        replayStatus: decisionReplay.replayStatus,
      },
    }),
  };
}

export function applyForbiddenUiAction(
  state: LocalOperatorShellState,
  action: LocalOperatorUiForbiddenAction,
): LocalOperatorIntentResult {
  const reasons: Record<LocalOperatorUiForbiddenAction, string> = {
    mark_production_readiness: "ui_cannot_mark_readiness",
    approve_release_candidate_status: "ui_cannot_approve_candidate_status",
    invoke_provider_execution: "ui_cannot_invoke_provider_execution",
  };
  return { status: "rejected", reason: reasons[action], state };
}
