import { getUiReadModel } from "./readModel";
import { deterministicStubProviderConfigurationCandidate, deterministicStubProviderExecutionRequest } from "./localOperatorShell";
import { renderLocalOperatorShellSnapshot } from "./localOperatorShellView";
import { createLocalOperatorShellTransport, createLocalStagedCandidateConversionProposal, executeLocalProvider, submitLocalProviderConfiguration } from "./localOperatorShellTransport";

export function renderLocalRuntimeReviewSurface(): string {
  const runtimeReview = getUiReadModel().localRuntimeReview;

  const disabledCapabilities = runtimeReview.disabledCapabilities
    .map((capability) => `- ${capability.label}: ${capability.status} — ${capability.summary}`)
    .join("\n");

  const localInteractions = runtimeReview.interactions
    .map((interaction) => [
      `- ${interaction.label} (${interaction.status})`,
      `  Result: ${interaction.result.status} / ${interaction.result.reasonCode}`,
      `  Transport eligible: ${interaction.result.transportEligible}`,
      `  Execution enabled: ${interaction.result.executionEnabled}`,
      `  Persistence enabled: ${interaction.result.persistenceEnabled}`,
      `  Provider execution enabled: ${interaction.result.providerExecutionEnabled}`,
      `  Replay repair enabled: ${interaction.result.replayRepairEnabled}`,
      `  Mutates authority: ${interaction.result.mutatesAuthority}`,
      `  Summary: ${interaction.summary}`
    ].join("\n"))
    .join("\n");

  const transport = createLocalOperatorShellTransport();
  submitLocalProviderConfiguration(transport, deterministicStubProviderConfigurationCandidate());
  executeLocalProvider(transport, deterministicStubProviderExecutionRequest("phase 146 local UI staged proposal"));
  createLocalStagedCandidateConversionProposal(transport, { operatorNote: "local UI display request" });
  const stagedProposalSnapshot = renderLocalOperatorShellSnapshot(transport.getCurrentState().state);

  return [
    runtimeReview.title,
    `Launch: ${runtimeReview.launchInstruction}`,
    `Posture: ${runtimeReview.postureIndicators.join(", ")}`,
    `Workflow state: ${runtimeReview.reviewState.workflowState}`,
    `Review state: ${runtimeReview.reviewState.reviewState}`,
    `Escalation state: ${runtimeReview.reviewState.escalationState}`,
    `Failure state: ${runtimeReview.reviewState.failureState}`,
    `Evidence state: ${runtimeReview.reviewState.evidenceState}`,
    `Validation status: ${runtimeReview.reviewState.validationStatus}`,
    `Dry-run result: ${runtimeReview.reviewState.dryRunSummary}`,
    `Deterministic rendering: ${runtimeReview.deterministicSummary}`,
    `Non-readiness: ${runtimeReview.nonReadinessStatement}`,
    "Disabled capabilities",
    disabledCapabilities,
    "Bounded local operator review interactions",
    localInteractions,
    "Local staged conversion proposal shell",
    stagedProposalSnapshot
  ].join("\n");
}
