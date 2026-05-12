import {
  applyLocalOperatorIntent,
  applyLocalProviderConfigurationCandidate,
  applyLocalProviderExecution,
  createStagedCandidateConversionProposal,
  initialLocalOperatorShellState,
  startDeterministicStubRun,
  type LocalOperatorIntent,
  type LocalProviderConfigurationCandidate,
  type LocalProviderExecutionRequest,
  type StagedCandidateConversionProposalRequest,
  type LocalOperatorShellState,
  type LocalSessionEvidenceExport
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
  | Readonly<{ kind: "submit_operator_intent"; intent: LocalOperatorIntent }>
  | Readonly<{ kind: "submit_provider_configuration"; candidate: LocalProviderConfigurationCandidate }>
  | Readonly<{ kind: "execute_provider"; request: LocalProviderExecutionRequest }>
  | Readonly<{ kind: "create_staged_candidate_conversion_proposal"; request: StagedCandidateConversionProposalRequest }>
  | Readonly<{ kind: "forbidden"; request: LocalOperatorShellForbiddenRequest }>;

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
  submitOperatorIntent: (intent: LocalOperatorIntent) => LocalOperatorShellResponse;
  submitProviderConfiguration: (candidate: LocalProviderConfigurationCandidate) => LocalOperatorShellResponse;
  executeProvider: (request: LocalProviderExecutionRequest) => LocalOperatorShellResponse;
  createStagedCandidateConversionProposal: (request: StagedCandidateConversionProposalRequest) => LocalOperatorShellResponse;
  rejectForbiddenUiAction: (request: LocalOperatorShellForbiddenRequest) => LocalOperatorShellResponse;
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
  readinessApprovalEnabled: false
};

const forbiddenReasons: Record<LocalOperatorShellForbiddenRequest, string> = {
  authority_grant: "authority_grant_rejected",
  provider_execution: "provider_execution_rejected",
  readiness_claim: "readiness_claim_rejected",
  production_state_mutation: "production_state_mutation_rejected",
  release_artifact_creation: "release_artifact_creation_rejected",
  deployment_enablement: "deployment_enablement_rejected",
  signing_enablement: "signing_enablement_rejected",
  publishing_enablement: "publishing_enablement_rejected"
};

export function createLocalOperatorShellTransport(): LocalOperatorShellTransport {
  let state = initialLocalOperatorShellState();

  function accepted(reason: string, next: LocalOperatorShellState): LocalOperatorShellResponse {
    state = next;
    return { status: "accepted", reason, state, localSessionEvidenceExport: state.localSessionEvidenceExport, capabilities: localStubOnlyCapabilities };
  }

  function rejected(reason: string): LocalOperatorShellResponse {
    return { status: "rejected", reason, state, localSessionEvidenceExport: state.localSessionEvidenceExport, capabilities: localStubOnlyCapabilities };
  }

  function step(request: LocalOperatorShellRequest): LocalOperatorShellResponse {
    switch (request.kind) {
      case "get_initial_state":
        return accepted("initial_shell_state_returned", initialLocalOperatorShellState());
      case "get_current_state":
        return accepted("current_shell_state_returned", state);
      case "start_deterministic_stub_run":
        return accepted("deterministic_stub_run_completed", startDeterministicStubRun(state));
      case "submit_operator_intent": {
        const result = applyLocalOperatorIntent(state, request.intent);
        if (result.status === "accepted") return accepted(result.reason, result.state);
        return rejected(result.reason);
      }
      case "submit_provider_configuration": {
        const result = applyLocalProviderConfigurationCandidate(state, request.candidate);
        if (result.status === "accepted") return accepted(result.reason, result.state);
        return rejected(result.reason);
      }
      case "execute_provider": {
        const result = applyLocalProviderExecution(state, request.request);
        if (result.status === "accepted") return accepted(result.reason, result.state);
        return rejected(result.reason);
      }
      case "create_staged_candidate_conversion_proposal": {
        const result = createStagedCandidateConversionProposal(state, request.request);
        if (result.status === "accepted") return accepted(result.reason, result.state);
        return rejected(result.reason);
      }
      case "forbidden":
        return rejected(forbiddenReasons[request.request]);
    }
  }

  return {
    getInitialState: () => step({ kind: "get_initial_state" }),
    getCurrentState: () => step({ kind: "get_current_state" }),
    startDeterministicStubRun: () => step({ kind: "start_deterministic_stub_run" }),
    submitOperatorIntent: (intent) => step({ kind: "submit_operator_intent", intent }),
    submitProviderConfiguration: (candidate) => step({ kind: "submit_provider_configuration", candidate }),
    executeProvider: (request) => step({ kind: "execute_provider", request }),
    createStagedCandidateConversionProposal: (request) => step({ kind: "create_staged_candidate_conversion_proposal", request }),
    rejectForbiddenUiAction: (request) => step({ kind: "forbidden", request })
  };
}

export function getInitialLocalOperatorShellState(
  transport: LocalOperatorShellTransport = createLocalOperatorShellTransport()
): LocalOperatorShellResponse {
  return transport.getInitialState();
}

export function requestDeterministicStubRun(
  transport: LocalOperatorShellTransport
): LocalOperatorShellResponse {
  return transport.startDeterministicStubRun();
}

export function submitLocalOperatorIntent(
  transport: LocalOperatorShellTransport,
  intent: LocalOperatorIntent
): LocalOperatorShellResponse {
  return transport.submitOperatorIntent(intent);
}

export function rejectForbiddenUiAction(
  transport: LocalOperatorShellTransport,
  request: LocalOperatorShellForbiddenRequest
): LocalOperatorShellResponse {
  return transport.rejectForbiddenUiAction(request);
}

export function submitLocalProviderConfiguration(
  transport: LocalOperatorShellTransport,
  candidate: LocalProviderConfigurationCandidate
): LocalOperatorShellResponse {
  return transport.submitProviderConfiguration(candidate);
}

export function executeLocalProvider(
  transport: LocalOperatorShellTransport,
  request: LocalProviderExecutionRequest
): LocalOperatorShellResponse {
  return transport.executeProvider(request);
}

export function createLocalStagedCandidateConversionProposal(
  transport: LocalOperatorShellTransport,
  request: StagedCandidateConversionProposalRequest
): LocalOperatorShellResponse {
  return transport.createStagedCandidateConversionProposal(request);
}
