import {
  applyLocalOperatorIntent,
  applyLocalProviderConfigurationCandidate,
  applyLocalProviderAdapterDeclaration,
  applyLocalProviderAdapterDryRun,
  applyConstrainedLocalProviderInvocation,
  applyLocalProviderExecution,
  createStagedCandidateConversionProposal,
  initialLocalOperatorShellState,
  validateStagedCandidateConversionProposalForPhase147,
  submitOperatorCandidateDecision,
  materializeLocalCandidateOutput,
  startDeterministicStubRun,
  startControlledInternalTrialExecution,
  stepControlledInternalTrialExecution,
  type ControlledInternalTrialExecutionRequest,
  type LocalOperatorIntent,
  type LocalProviderConfigurationCandidate,
  type LocalProviderAdapterConfigurationCandidate,
  type LocalProviderExecutionRequest,
  type LocalProviderAdapterDryRunRequest,
  type ConstrainedLocalProviderInvocationRequest,
  type StagedCandidateConversionProposalRequest,
  type StagedCandidateConversionValidationRequest,
  type OperatorCandidateDecisionRequest,
  type LocalCandidateMaterializationRequest,
  type LocalOperatorShellState,
  type LocalSessionEvidenceExport,
} from "./localOperatorShell";

export type LocalOperatorShellTransportStatus = "accepted" | "rejected";

export type LocalOperatorShellForbiddenRequest =
  | "authority_grant"
  | "provider_execution"
  | "readiness_claim"
  | "production_state_mutation"
  | "release_artifact_creation"
  | "deployment_enablement"
  | "signing_enablement"
  | "publishing_enablement";

export type LocalOperatorShellRequest =
  | Readonly<{ kind: "get_initial_state" }>
  | Readonly<{ kind: "get_current_state" }>
  | Readonly<{ kind: "start_deterministic_stub_run" }>
  | Readonly<{ kind: "start_controlled_internal_trial_execution"; request: ControlledInternalTrialExecutionRequest }>
  | Readonly<{ kind: "step_controlled_internal_trial_execution"; request: ControlledInternalTrialExecutionRequest }>
  | Readonly<{ kind: "submit_operator_intent"; intent: LocalOperatorIntent }>
  | Readonly<{
      kind: "submit_provider_configuration";
      candidate: LocalProviderConfigurationCandidate;
    }>
  | Readonly<{
      kind: "submit_provider_adapter_declaration";
      candidate: LocalProviderAdapterConfigurationCandidate;
    }>
  | Readonly<{
      kind: "execute_provider";
      request: LocalProviderExecutionRequest;
    }>
  | Readonly<{
      kind: "run_provider_adapter_dry_run";
      request: LocalProviderAdapterDryRunRequest;
    }>
  | Readonly<{
      kind: "invoke_constrained_local_provider";
      request: ConstrainedLocalProviderInvocationRequest;
    }>
  | Readonly<{
      kind: "create_staged_candidate_conversion_proposal";
      request: StagedCandidateConversionProposalRequest;
    }>
  | Readonly<{
      kind: "validate_staged_candidate_conversion_proposal";
      request: StagedCandidateConversionValidationRequest;
    }>
  | Readonly<{
      kind: "submit_operator_candidate_decision";
      request: OperatorCandidateDecisionRequest;
    }>
  | Readonly<{
      kind: "materialize_local_candidate_output";
      request: LocalCandidateMaterializationRequest;
    }>
  | Readonly<{
      kind: "forbidden";
      request: LocalOperatorShellForbiddenRequest;
    }>;

export type LocalOperatorShellCapabilities = Readonly<{
  localOnly: true;
  nonProduction: true;
  providerExecutionEnabled: false;
  cloudModelCallsEnabled: false;
  broadCommandExecutionEnabled: false;
  productionPersistenceEnabled: false;
  releaseArtifactCreationEnabled: false;
  deploymentEnabled: false;
  signingEnabled: false;
  publishingEnabled: false;
  readinessApprovalEnabled: false;
}>;

export type LocalOperatorShellResponse = Readonly<{
  status: LocalOperatorShellTransportStatus;
  reason: string;
  state: LocalOperatorShellState;
  localSessionEvidenceExport: LocalSessionEvidenceExport;
  capabilities: LocalOperatorShellCapabilities;
}>;

export type LocalOperatorShellTransport = Readonly<{
  getInitialState: () => LocalOperatorShellResponse;
  getCurrentState: () => LocalOperatorShellResponse;
  startDeterministicStubRun: () => LocalOperatorShellResponse;
  startControlledInternalTrialExecution: (request: ControlledInternalTrialExecutionRequest) => LocalOperatorShellResponse;
  stepControlledInternalTrialExecution: (request: ControlledInternalTrialExecutionRequest) => LocalOperatorShellResponse;
  submitOperatorIntent: (
    intent: LocalOperatorIntent,
  ) => LocalOperatorShellResponse;
  submitProviderConfiguration: (
    candidate: LocalProviderConfigurationCandidate,
  ) => LocalOperatorShellResponse;
  submitProviderAdapterDeclaration: (
    candidate: LocalProviderAdapterConfigurationCandidate,
  ) => LocalOperatorShellResponse;
  executeProvider: (
    request: LocalProviderExecutionRequest,
  ) => LocalOperatorShellResponse;
  runProviderAdapterDryRun: (
    request: LocalProviderAdapterDryRunRequest,
  ) => LocalOperatorShellResponse;
  invokeConstrainedLocalProvider: (
    request: ConstrainedLocalProviderInvocationRequest,
  ) => LocalOperatorShellResponse;
  createStagedCandidateConversionProposal: (
    request: StagedCandidateConversionProposalRequest,
  ) => LocalOperatorShellResponse;
  validateStagedCandidateConversionProposal: (
    request?: StagedCandidateConversionValidationRequest,
  ) => LocalOperatorShellResponse;
  submitOperatorCandidateDecision: (
    request: OperatorCandidateDecisionRequest,
  ) => LocalOperatorShellResponse;
  materializeLocalCandidateOutput: (
    request: LocalCandidateMaterializationRequest,
  ) => LocalOperatorShellResponse;
  rejectForbiddenUiAction: (
    request: LocalOperatorShellForbiddenRequest,
  ) => LocalOperatorShellResponse;
}>;

const localStubOnlyCapabilities: LocalOperatorShellCapabilities = {
  localOnly: true,
  nonProduction: true,
  providerExecutionEnabled: false,
  cloudModelCallsEnabled: false,
  broadCommandExecutionEnabled: false,
  productionPersistenceEnabled: false,
  releaseArtifactCreationEnabled: false,
  deploymentEnabled: false,
  signingEnabled: false,
  publishingEnabled: false,
  readinessApprovalEnabled: false,
};

const forbiddenReasons: Record<LocalOperatorShellForbiddenRequest, string> = {
  authority_grant: "authority_grant_rejected",
  provider_execution: "provider_execution_rejected",
  readiness_claim: "readiness_claim_rejected",
  production_state_mutation: "production_state_mutation_rejected",
  release_artifact_creation: "release_artifact_creation_rejected",
  deployment_enablement: "deployment_enablement_rejected",
  signing_enablement: "signing_enablement_rejected",
  publishing_enablement: "publishing_enablement_rejected",
};

export function createLocalOperatorShellTransport(): LocalOperatorShellTransport {
  let state = initialLocalOperatorShellState();

  function accepted(
    reason: string,
    next: LocalOperatorShellState,
  ): LocalOperatorShellResponse {
    state = next;
    return {
      status: "accepted",
      reason,
      state,
      localSessionEvidenceExport: state.localSessionEvidenceExport,
      capabilities: localStubOnlyCapabilities,
    };
  }

  function rejected(
    reason: string,
    responseState: LocalOperatorShellState = state,
  ): LocalOperatorShellResponse {
    return {
      status: "rejected",
      reason,
      state: responseState,
      localSessionEvidenceExport: responseState.localSessionEvidenceExport,
      capabilities: localStubOnlyCapabilities,
    };
  }

  function step(
    request: LocalOperatorShellRequest,
  ): LocalOperatorShellResponse {
    switch (request.kind) {
      case "get_initial_state":
        return accepted(
          "initial_shell_state_returned",
          initialLocalOperatorShellState(),
        );
      case "get_current_state":
        return accepted("current_shell_state_returned", state);
      case "start_deterministic_stub_run":
        return accepted(
          "deterministic_stub_run_completed",
          startDeterministicStubRun(state),
        );
      case "start_controlled_internal_trial_execution": {
        const next = startControlledInternalTrialExecution(state, request.request);
        if (["trial_run_rejected", "trial_run_blocked", "invalid_trial_run_request"].includes(next.controlledInternalTrialExecution.status)) return rejected("controlled_internal_trial_execution_rejected", next);
        return accepted("controlled_internal_trial_execution_started", next);
      }
      case "step_controlled_internal_trial_execution": {
        const next = stepControlledInternalTrialExecution(state, request.request);
        if (["trial_run_rejected", "trial_run_blocked", "invalid_trial_run_request"].includes(next.controlledInternalTrialExecution.status)) return rejected("controlled_internal_trial_execution_step_rejected", next);
        return accepted("controlled_internal_trial_execution_stepped", next);
      }
      case "submit_operator_intent": {
        const result = applyLocalOperatorIntent(state, request.intent);
        if (result.status === "accepted")
          return accepted(result.reason, result.state);
        return rejected(result.reason);
      }
      case "submit_provider_configuration": {
        const result = applyLocalProviderConfigurationCandidate(
          state,
          request.candidate,
        );
        if (result.status === "accepted")
          return accepted(result.reason, result.state);
        return rejected(result.reason);
      }
      case "submit_provider_adapter_declaration": {
        const result = applyLocalProviderAdapterDeclaration(
          state,
          request.candidate,
        );
        if (result.status === "accepted")
          return accepted(result.reason, result.state);
        return rejected(result.reason);
      }
      case "execute_provider": {
        const result = applyLocalProviderExecution(state, request.request);
        if (result.status === "accepted")
          return accepted(result.reason, result.state);
        return rejected(result.reason);
      }
      case "run_provider_adapter_dry_run": {
        const result = applyLocalProviderAdapterDryRun(state, request.request);
        if (result.status === "accepted")
          return accepted(result.reason, result.state);
        if (state.localProviderAdapterDryRun.status === "dry_run_executed")
          return rejected(result.reason, state);
        return rejected(result.reason, result.state);
      }
      case "invoke_constrained_local_provider": {
        const result = applyConstrainedLocalProviderInvocation(
          state,
          request.request,
        );
        if (result.status === "accepted")
          return accepted(result.reason, result.state);
        if (
          state.constrainedLocalProviderInvocation.status ===
          "invocation_executed"
        )
          return rejected(result.reason, state);
        return rejected(result.reason, result.state);
      }
      case "create_staged_candidate_conversion_proposal": {
        const result = createStagedCandidateConversionProposal(
          state,
          request.request,
        );
        if (result.status === "accepted")
          return accepted(result.reason, result.state);
        return rejected(result.reason);
      }
      case "validate_staged_candidate_conversion_proposal": {
        const result = validateStagedCandidateConversionProposalForPhase147(
          state,
          request.request,
        );
        if (result.status === "accepted")
          return accepted(result.reason, result.state);
        return rejected(result.reason, result.state);
      }
      case "submit_operator_candidate_decision": {
        const result = submitOperatorCandidateDecision(state, request.request);
        if (result.status === "accepted")
          return accepted(result.reason, result.state);
        return rejected(result.reason, result.state);
      }
      case "materialize_local_candidate_output": {
        const result = materializeLocalCandidateOutput(state, request.request);
        if (result.status === "accepted")
          return accepted(result.reason, result.state);
        return rejected(result.reason, result.state);
      }
      case "forbidden":
        return rejected(forbiddenReasons[request.request]);
    }
  }

  return {
    getInitialState: () => step({ kind: "get_initial_state" }),
    getCurrentState: () => step({ kind: "get_current_state" }),
    startDeterministicStubRun: () =>
      step({ kind: "start_deterministic_stub_run" }),
    startControlledInternalTrialExecution: (request) =>
      step({ kind: "start_controlled_internal_trial_execution", request }),
    stepControlledInternalTrialExecution: (request) =>
      step({ kind: "step_controlled_internal_trial_execution", request }),
    submitOperatorIntent: (intent) =>
      step({ kind: "submit_operator_intent", intent }),
    submitProviderConfiguration: (candidate) =>
      step({ kind: "submit_provider_configuration", candidate }),
    submitProviderAdapterDeclaration: (candidate) =>
      step({ kind: "submit_provider_adapter_declaration", candidate }),
    executeProvider: (request) => step({ kind: "execute_provider", request }),
    runProviderAdapterDryRun: (request) =>
      step({ kind: "run_provider_adapter_dry_run", request }),
    invokeConstrainedLocalProvider: (request) =>
      step({ kind: "invoke_constrained_local_provider", request }),
    createStagedCandidateConversionProposal: (request) =>
      step({ kind: "create_staged_candidate_conversion_proposal", request }),
    validateStagedCandidateConversionProposal: (request = {}) =>
      step({ kind: "validate_staged_candidate_conversion_proposal", request }),
    submitOperatorCandidateDecision: (request) =>
      step({ kind: "submit_operator_candidate_decision", request }),
    materializeLocalCandidateOutput: (request) =>
      step({ kind: "materialize_local_candidate_output", request }),
    rejectForbiddenUiAction: (request) => step({ kind: "forbidden", request }),
  };
}

export function getInitialLocalOperatorShellState(
  transport: LocalOperatorShellTransport = createLocalOperatorShellTransport(),
): LocalOperatorShellResponse {
  return transport.getInitialState();
}

export function requestDeterministicStubRun(
  transport: LocalOperatorShellTransport,
): LocalOperatorShellResponse {
  return transport.startDeterministicStubRun();
}

export function submitLocalOperatorIntent(
  transport: LocalOperatorShellTransport,
  intent: LocalOperatorIntent,
): LocalOperatorShellResponse {
  return transport.submitOperatorIntent(intent);
}

export function rejectForbiddenUiAction(
  transport: LocalOperatorShellTransport,
  request: LocalOperatorShellForbiddenRequest,
): LocalOperatorShellResponse {
  return transport.rejectForbiddenUiAction(request);
}

export function submitLocalProviderConfiguration(
  transport: LocalOperatorShellTransport,
  candidate: LocalProviderConfigurationCandidate,
): LocalOperatorShellResponse {
  return transport.submitProviderConfiguration(candidate);
}

export function submitLocalProviderAdapterDeclaration(
  transport: LocalOperatorShellTransport,
  candidate: LocalProviderAdapterConfigurationCandidate,
): LocalOperatorShellResponse {
  return transport.submitProviderAdapterDeclaration(candidate);
}

export function executeLocalProvider(
  transport: LocalOperatorShellTransport,
  request: LocalProviderExecutionRequest,
): LocalOperatorShellResponse {
  return transport.executeProvider(request);
}

export function invokeConstrainedLocalProvider(
  transport: LocalOperatorShellTransport,
  request: ConstrainedLocalProviderInvocationRequest,
): LocalOperatorShellResponse {
  return transport.invokeConstrainedLocalProvider(request);
}

export function createLocalStagedCandidateConversionProposal(
  transport: LocalOperatorShellTransport,
  request: StagedCandidateConversionProposalRequest,
): LocalOperatorShellResponse {
  return transport.createStagedCandidateConversionProposal(request);
}

export function validateLocalStagedCandidateConversionProposal(
  transport: LocalOperatorShellTransport,
  request: StagedCandidateConversionValidationRequest = {},
): LocalOperatorShellResponse {
  return transport.validateStagedCandidateConversionProposal(request);
}

export function submitLocalOperatorCandidateDecision(
  transport: LocalOperatorShellTransport,
  request: OperatorCandidateDecisionRequest,
): LocalOperatorShellResponse {
  return transport.submitOperatorCandidateDecision(request);
}

export function runLocalProviderAdapterDryRun(
  transport: LocalOperatorShellTransport,
  request: LocalProviderAdapterDryRunRequest,
): LocalOperatorShellResponse {
  return transport.runProviderAdapterDryRun(request);
}

export function requestLocalCandidateMaterialization(
  transport: LocalOperatorShellTransport,
  request: LocalCandidateMaterializationRequest,
): LocalOperatorShellResponse {
  return transport.materializeLocalCandidateOutput(request);
}

export function requestControlledInternalTrialExecutionStart(transport: LocalOperatorShellTransport, request: ControlledInternalTrialExecutionRequest): LocalOperatorShellResponse {
  return transport.startControlledInternalTrialExecution(request);
}

export function requestControlledInternalTrialExecutionStep(transport: LocalOperatorShellTransport, request: ControlledInternalTrialExecutionRequest): LocalOperatorShellResponse {
  return transport.stepControlledInternalTrialExecution(request);
}
