import {
  allowlistedLocalProviderInvocationRequest,
  deterministicFakeAdapterDeclarationCandidate,
  deterministicFakeAdapterDryRunRequest,
  deterministicStubProviderConfigurationCandidate,
  deterministicStubProviderExecutionRequest,
  projectLocalProviderAdapterRegistry,
  projectLocalProviderConfiguration,
  projectLocalProviderExecution,
  type LocalOperatorIntentKind,
  type LocalOperatorShellState,
} from "./api/localOperatorShell.js";
import { renderProviderOutputReviewHtml } from "./api/providerOutputReview.js";
import {
  createLocalOperatorShellTransport,
  getInitialLocalOperatorShellState,
  requestDeterministicStubRun,
  executeLocalProvider,
  createLocalStagedCandidateConversionProposal,
  validateLocalStagedCandidateConversionProposal,
  submitLocalOperatorIntent,
  submitLocalProviderConfiguration,
  submitLocalProviderAdapterDeclaration,
  runLocalProviderAdapterDryRun,
  invokeConstrainedLocalProvider,
  type LocalOperatorShellResponse,
} from "./api/localOperatorShellTransport.js";

const transport = createLocalOperatorShellTransport();
let shellState = getInitialLocalOperatorShellState(transport).state;
let lastTransportMessage =
  "initial shell state loaded through local transport boundary";

function applyTransportResponse(response: LocalOperatorShellResponse): void {
  shellState = response.state;
  lastTransportMessage = `${response.status}: ${response.reason}`;
  render();
}

function startRun(): void {
  applyTransportResponse(requestDeterministicStubRun(transport));
}

function recordIntent(kind: LocalOperatorIntentKind): void {
  applyTransportResponse(
    submitLocalOperatorIntent(transport, {
      kind,
      operatorId: "local-operator",
      targetRunId: shellState.run.runId,
      targetCandidateId: shellState.run.candidate?.candidateId,
      reason: `${kind} selected in local non-production browser shell`,
    }),
  );
}

function submitAdapterDeclaration(): void {
  applyTransportResponse(
    submitLocalProviderAdapterDeclaration(
      transport,
      deterministicFakeAdapterDeclarationCandidate(),
    ),
  );
}

function submitUnsafeAdapterDeclaration(): void {
  applyTransportResponse(
    submitLocalProviderAdapterDeclaration(transport, {
      adapterKind: "unsupported_network_adapter",
      declarationId: "unsafe-network-adapter-declaration",
      fields: [],
    }),
  );
}

function runAdapterDryRun(): void {
  applyTransportResponse(
    runLocalProviderAdapterDryRun(
      transport,
      deterministicFakeAdapterDryRunRequest(),
    ),
  );
}

function runRejectedAdapterDryRun(): void {
  applyTransportResponse(
    runLocalProviderAdapterDryRun(transport, {
      inputSummary: "phase 154 browser dry run",
      fields: [{ key: "endpoint", value: "https://example.invalid" }],
    }),
  );
}

function invokeAllowlistedLocalProvider(): void {
  applyTransportResponse(
    invokeConstrainedLocalProvider(
      transport,
      allowlistedLocalProviderInvocationRequest(),
    ),
  );
}

function invokeRejectedLocalProvider(): void {
  applyTransportResponse(
    invokeConstrainedLocalProvider(transport, {
      providerKind: "unsupported_network_provider",
      inputSummary: "phase 156 browser rejected invocation",
      fields: [{ key: "endpoint", value: "https://example.invalid" }],
    }),
  );
}

function submitProviderConfiguration(): void {
  applyTransportResponse(
    submitLocalProviderConfiguration(
      transport,
      deterministicStubProviderConfigurationCandidate(),
    ),
  );
}

function submitUnsafeProviderConfiguration(): void {
  applyTransportResponse(
    submitLocalProviderConfiguration(transport, {
      providerKind: "deterministic_stub",
      fields: [{ key: "endpoint", value: "http://localhost:11434" }],
    }),
  );
}

function runDeterministicProvider(): void {
  applyTransportResponse(
    executeLocalProvider(
      transport,
      deterministicStubProviderExecutionRequest(
        "local deterministic browser execution input",
      ),
    ),
  );
}

function runForbiddenProviderExecution(): void {
  applyTransportResponse(
    executeLocalProvider(transport, {
      providerKind: "deterministic_stub",
      inputSummary: "unsafe local browser execution input",
      fields: [{ key: "command", value: "run model" }],
    }),
  );
}

function createStagedProposal(): void {
  applyTransportResponse(
    createLocalStagedCandidateConversionProposal(transport, {
      operatorNote: "local browser staged proposal",
    }),
  );
}

function validateStagedProposal(): void {
  applyTransportResponse(
    validateLocalStagedCandidateConversionProposal(transport),
  );
}

function renderList(items: readonly string[], emptyText: string): string {
  if (items.length === 0) return `<p class="muted">${emptyText}</p>`;
  return `<ul>${items.map((item) => `<li>${item}</li>`).join("")}</ul>`;
}

function renderCandidate(state: LocalOperatorShellState): string {
  if (!state.run.candidate)
    return `<p class="muted">Start a deterministic stub run to display candidate output.</p>`;
  const candidate = state.run.candidate;
  return `
    <h3>${candidate.title}</h3>
    <p>${candidate.body}</p>
    <dl>
      <div><dt>Candidate</dt><dd>${candidate.candidateId}</dd></div>
      <div><dt>Provider</dt><dd>${candidate.providerKind}</dd></div>
      <div><dt>Provider output trust flag</dt><dd>${candidate.providerOutputTrusted}</dd></div>
      <div><dt>Provider execution enabled</dt><dd>${candidate.providerExecutionEnabled}</dd></div>
    </dl>`;
}

function renderAdapterRegistry(state: LocalOperatorShellState): string {
  const registry = projectLocalProviderAdapterRegistry(
    state.localProviderAdapterRegistry,
  );
  const declarations =
    registry.declarations.length === 0
      ? `<p class="muted">No accepted adapter declarations yet.</p>`
      : `<ul>${registry.declarations.map((declaration) => `<li>${declaration.declarationId}: ${declaration.adapterKind} / ${declaration.status} / ${declaration.contract.executionStatus} / ${declaration.contract.trustStatus}</li>`).join("")}</ul>`;
  return `
    <p><strong>Adapter contract only; no model execution is available in Phase 153.</strong></p>
    <p>Accepted adapter declarations are non-executing.</p>
    <p>Adapter declaration does not grant provider trust.</p>
    <p>No network, shell, secret, or production persistence capability is enabled.</p>
    <dl>
      <div><dt>Registry status</dt><dd>${registry.registryStatus}</dd></div>
      <div><dt>Supported adapter declarations</dt><dd>${registry.supportedAdapterKinds.join(", ")}</dd></div>
      <div><dt>Rejected adapter declarations</dt><dd>${registry.rejectedAdapterKinds.join(", ")}</dd></div>
      <div><dt>Validation status</dt><dd>${registry.lastValidation.status}</dd></div>
      <div><dt>Validation reason</dt><dd>${registry.lastValidation.reason}</dd></div>
      <div><dt>Validation error/reason code</dt><dd>${registry.lastValidation.errorCodes.join(", ") || "none"}</dd></div>
      <div><dt>Capability surface</dt><dd>${registry.capabilitySurface.summary}</dd></div>
      <div><dt>Execution status</dt><dd>${registry.executionStatus}</dd></div>
      <div><dt>Trust status</dt><dd>${registry.trustStatus}</dd></div>
      <div><dt>Boundary status</dt><dd>${registry.boundaryStatuses.join(", ")}</dd></div>
    </dl>
    ${declarations}
    <p class="muted">${registry.note}</p>
    <div class="button-row">
      <button id="submit-adapter-declaration" type="button">Declare deterministic fake adapter contract</button>
      <button id="reject-adapter-declaration" type="button">Submit rejected network adapter declaration</button>
    </div>`;
}

function renderAdapterDryRun(state: LocalOperatorShellState): string {
  const dryRun = state.localProviderAdapterDryRun;
  const activeDeclaration =
    state.localProviderAdapterRegistry.declarations[
      state.localProviderAdapterRegistry.declarations.length - 1
    ];
  const canRun =
    activeDeclaration?.adapterKind === "deterministic_fake_adapter" &&
    activeDeclaration.status === "adapter_declared_non_executing";
  return `
    <p><strong>Controlled adapter dry run only.</strong></p>
    <p>Only deterministic_fake_adapter can execute in Phase 154.</p>
    <p>No real model execution occurs in Phase 154.</p>
    <p>Dry-run output remains untrusted and descriptive.</p>
    <p>Dry run does not create candidate output or materialize candidates.</p>
    <p>Dry run does not approve readiness, release, deployment, or public use.</p>
    <dl>
      <div><dt>Dry-run status</dt><dd>${dryRun.status}</dd></div>
      <div><dt>Adapter kind</dt><dd>${activeDeclaration?.adapterKind ?? dryRun.adapterKind ?? "none"}</dd></div>
      <div><dt>Adapter declaration status</dt><dd>${activeDeclaration?.status ?? "none"}</dd></div>
      <div><dt>Result ID</dt><dd>${dryRun.result?.resultId ?? "none"}</dd></div>
      <div><dt>Output summary</dt><dd>${dryRun.result?.outputSummary ?? "none"}</dd></div>
      <div><dt>Output trust status</dt><dd>${dryRun.outputTrustStatus}</dd></div>
      <div><dt>Dry-run boundary status</dt><dd>${dryRun.boundaryStatuses.join(", ")}</dd></div>
      <div><dt>Capability surface</dt><dd>${dryRun.capabilitySurface.summary}</dd></div>
      <div><dt>No-real-model/no-process/no-network/no-shell/no-secret markers</dt><dd>no_real_model_execution, no_process_spawn, no_network, no_shell, no_secrets</dd></div>
      <div><dt>No-candidate/no-action/no-readiness/no-release/no-deployment/public-use markers</dt><dd>${dryRun.effectStatuses.join(", ")}</dd></div>
      <div><dt>Rejected reason</dt><dd>${dryRun.errorCodes.join(", ") || "none"}</dd></div>
    </dl>
    <div class="button-row">
      <button id="run-adapter-dry-run" type="button" ${canRun ? "" : "disabled"}>Run controlled adapter dry run</button>
      <button id="reject-adapter-dry-run" type="button">Submit rejected adapter dry-run request</button>
    </div>`;
}

function renderConstrainedLocalProviderInvocation(
  state: LocalOperatorShellState,
): string {
  const invocation = state.constrainedLocalProviderInvocation;
  const activeDeclaration =
    state.localProviderAdapterRegistry.declarations[
      state.localProviderAdapterRegistry.declarations.length - 1
    ];
  const canInvoke =
    activeDeclaration?.adapterKind === "deterministic_fake_adapter" &&
    activeDeclaration.status === "adapter_declared_non_executing";
  return `
    <p><strong>Constrained local provider invocation only.</strong></p>
    <p>Only one allowlisted local provider path is enabled in Phase 156.</p>
    <p>No arbitrary command execution is available.</p>
    <p>No shell, network, cloud, or secret capability is enabled.</p>
    <p>Provider output remains untrusted and descriptive.</p>
    <p>Invocation does not create candidate output or materialize candidates.</p>
    <p>Invocation does not approve readiness, release, deployment, or public use.</p>
    <dl>
      <div><dt>Invocation status</dt><dd>${invocation.status}</dd></div>
      <div><dt>Allowlisted provider kind</dt><dd>${invocation.capabilitySurface.allowlistedProviderKind}</dd></div>
      <div><dt>Adapter kind</dt><dd>${activeDeclaration?.adapterKind ?? invocation.adapterKind ?? "none"}</dd></div>
      <div><dt>Adapter declaration status</dt><dd>${activeDeclaration?.status ?? "none"}</dd></div>
      <div><dt>Result ID</dt><dd>${invocation.result?.resultId ?? "none"}</dd></div>
      <div><dt>Output summary</dt><dd>${invocation.result?.outputSummary ?? "none"}</dd></div>
      <div><dt>Output trust status</dt><dd>${invocation.outputTrustStatus}</dd></div>
      <div><dt>Capability surface</dt><dd>${invocation.capabilitySurface.summary}</dd></div>
      <div><dt>Boundary markers</dt><dd>${invocation.boundaryStatuses.join(", ")}</dd></div>
      <div><dt>No-arbitrary-command/no-shell/no-network/no-cloud/no-secret markers</dt><dd>no_arbitrary_command, no_shell, no_network, no_cloud, no_secrets</dd></div>
      <div><dt>No-candidate/no-action/no-readiness/no-release/no-deployment/public-use markers</dt><dd>${invocation.effectStatuses.join(", ")}</dd></div>
      <div><dt>Rejection reason</dt><dd>${invocation.errorCodes.join(", ") || "none"}</dd></div>
    </dl>
    <div class="button-row">
      <button id="invoke-constrained-local-provider" type="button" ${canInvoke ? "" : "disabled"}>Invoke allowlisted local provider</button>
      <button id="reject-constrained-local-provider" type="button">Submit rejected local invocation request</button>
    </div>`;
}


function renderProviderOutputPipeline(state: LocalOperatorShellState): string {
  const pipeline = state.localProviderOutputPipeline;
  const stages = pipeline.stages
    .map(
      (stage) =>
        `<li><code>${stage.stage}</code>: <strong>${stage.status}</strong>${
          stage.reason ? ` — ${stage.reason}` : ""
        }</li>`,
    )
    .join("");
  return `
    <p><strong>Provider output pipeline integration</strong></p>
    <p>Invocation output remains untrusted and descriptive.</p>
    <p>Pipeline integration does not create candidate output.</p>
    <p>Validation, review, staging, staged validation, candidate review, and operator decision boundaries cannot be skipped.</p>
    <p>Candidate materialization remains a later bounded step.</p>
    <p>Provider trust, readiness, release, deployment, and public-use approval are not granted.</p>
    <dl>
      <div><dt>Pipeline source kind</dt><dd>${pipeline.sourceKind ?? "none"}</dd></div>
      <div><dt>Source invocation result ID</dt><dd>${pipeline.sourceInvocationResultId ?? "none"}</dd></div>
      <div><dt>Provider execution result ID</dt><dd>${pipeline.providerExecutionResultId ?? "none"}</dd></div>
      <div><dt>Pipeline status</dt><dd>${pipeline.status}</dd></div>
      <div><dt>Current stage</dt><dd>${pipeline.currentStage ?? "none"}</dd></div>
      <div><dt>Next required stage</dt><dd>${pipeline.nextRequiredStage ?? "none"}</dd></div>
      <div><dt>Provider output validation status</dt><dd>${pipeline.providerOutputValidationStatus}</dd></div>
      <div><dt>Provider output review status</dt><dd>${pipeline.providerOutputReviewStatus}</dd></div>
      <div><dt>Staged proposal status</dt><dd>${pipeline.stagedProposalStatus}</dd></div>
      <div><dt>Staged proposal validation status</dt><dd>${pipeline.stagedProposalValidationStatus}</dd></div>
      <div><dt>Candidate review status</dt><dd>${pipeline.candidateReviewStatus}</dd></div>
      <div><dt>Operator decision status</dt><dd>${pipeline.operatorDecisionStatus}</dd></div>
      <div><dt>Blocked/rejected reasons</dt><dd>${pipeline.errors.join(", ") || "none"}</dd></div>
      <div><dt>Boundary markers</dt><dd>${pipeline.boundaryStatuses.join(", ")}</dd></div>
      <div><dt>No-effect markers</dt><dd>${pipeline.effectStatuses.join(", ")}</dd></div>
    </dl>
    <ol>${stages}</ol>
    <p class="muted">${pipeline.note}</p>`;
}

function renderProviderConfiguration(state: LocalOperatorShellState): string {
  const providerConfiguration = projectLocalProviderConfiguration(
    state.providerConfiguration,
  );
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

function renderStagedProposalValidation(
  state: LocalOperatorShellState,
): string {
  const proposalProjection = state.stagedCandidateConversionProposal;
  const proposal = proposalProjection.proposal;
  const validation = state.stagedCandidateConversionValidation;
  return `
    <p>Validation checks staged proposal shape and source linkage only.</p>
    <p>Validated staged proposal is not candidate output.</p>
    <p>Candidate materialization was not performed in Phase 147.</p>
    <p>Future review boundary is required before any operator decision.</p>
    <p>Operator decision is not available in Phase 147.</p>
    <p>Provider output remains untrusted and not approved.</p>
    <dl>
      <div><dt>Proposal status</dt><dd>${proposalProjection.status}</dd></div>
      <div><dt>Validation status</dt><dd>${validation.status}</dd></div>
      <div><dt>Validation reasons</dt><dd>${validation.reasons.join(", ") || "none"}</dd></div>
      <div><dt>Proposal ID</dt><dd>${proposal?.proposalId ?? validation.proposalId ?? "none"}</dd></div>
      <div><dt>Source provider kind</dt><dd>${proposal?.sourceProviderKind ?? validation.sourceProviderKind}</dd></div>
      <div><dt>Source execution result ID</dt><dd>${proposal?.sourceExecutionResultId ?? validation.sourceExecutionResultId ?? "none"}</dd></div>
      <div><dt>Source validation status</dt><dd>${proposal?.sourceValidationStatus ?? validation.sourceValidationStatus}</dd></div>
      <div><dt>Source reviewability status</dt><dd>${proposal?.sourceReviewabilityStatus ?? validation.sourceReviewabilityStatus}</dd></div>
      <div><dt>Source candidate-boundary status</dt><dd>${proposal?.sourceCandidateBoundaryStatus ?? validation.sourceCandidateBoundaryStatus}</dd></div>
      <div><dt>Deterministic linkage status</dt><dd>${validation.deterministicLinkageStatus}</dd></div>
      <div><dt>Materialization status</dt><dd>${validation.materializationStatuses.join(", ")}</dd></div>
      <div><dt>Future review boundary status</dt><dd>${validation.futureReviewBoundaryStatus}</dd></div>
      <div><dt>Operator decision availability</dt><dd>${validation.operatorDecisionStatus}</dd></div>
      <div><dt>Trust status</dt><dd>${validation.trustStatuses.join(", ")}</dd></div>
      <div><dt>Approval status</dt><dd>not_approved</dd></div>
      <div><dt>No-effect summary</dt><dd>${validation.noEffectSummary.join(", ")}</dd></div>
    </dl>
    <div class="button-row">
      <button id="create-staged-proposal" type="button">Create staged conversion proposal</button>
      <button id="validate-staged-proposal" type="button">Validate staged proposal shape/linkage</button>
    </div>`;
}

function renderLocalSessionPackage(state: LocalOperatorShellState): string {
  const projection = state.localSessionPackageProjection;
  return `
    <dl>
      <div><dt>Package status</dt><dd>${projection.status}</dd></div>
      <div><dt>Package ID</dt><dd>${projection.packageId ?? "none"}</dd></div>
      <div><dt>Package version</dt><dd>${projection.packageVersion}</dd></div>
      <div><dt>Package classification</dt><dd>${projection.packageClassification}</dd></div>
      <div><dt>Production classification</dt><dd>${projection.productionClassification}</dd></div>
      <div><dt>Validation status</dt><dd>${projection.validationStatus}</dd></div>
      <div><dt>Read-back validation status</dt><dd>${projection.readBackValidationStatus ?? "none"}</dd></div>
      <div><dt>Restore status</dt><dd>${projection.restoreStatus}</dd></div>
      <div><dt>Included sections</dt><dd>${projection.includedSectionSummary.join(", ") || "none until package is derived"}</dd></div>
      <div><dt>Absence markers</dt><dd>${projection.absenceMarkerSummary.join(", ")}</dd></div>
      <div><dt>Validation errors</dt><dd>${projection.validationErrors.join(", ") || "none"}</dd></div>
    </dl>
    <p class="muted">${projection.localOnlyNote}</p>
    <p class="muted">${projection.releaseBoundaryNote}</p>
    <p class="muted">${projection.deploymentReadinessBoundaryNote}</p>
    <p class="muted">${projection.restoreBoundaryNote}</p>`;
}

function renderSessionHistory(state: LocalOperatorShellState): string {
  const history = state.localSessionHistoryProjection;
  const entries =
    history.entries.length === 0
      ? `<p class="muted">No explicit local session package entries are available.</p>`
      : `<ul>${history.entries
          .map(
            (entry) => `
        <li>
          <strong>${entry.packageId}</strong>
          <dl>
            <div><dt>Package version</dt><dd>${entry.packageVersion}</dd></div>
            <div><dt>Package classification</dt><dd>${entry.packageClassification}</dd></div>
            <div><dt>Production classification</dt><dd>${entry.productionClassification}</dd></div>
            <div><dt>Read-back validation status</dt><dd>${entry.readBackValidationStatus ?? "none"}</dd></div>
          </dl>
        </li>`,
          )
          .join("")}</ul>`;
  return `
    <dl>
      <div><dt>History status</dt><dd>${history.status}</dd></div>
      <div><dt>Selected package status</dt><dd>${history.selectedPackageId ? "package_selected" : "no_package_selected"}</dd></div>
      <div><dt>Selected package ID</dt><dd>${history.selectedPackageId ?? "none"}</dd></div>
    </dl>
    ${entries}
    <p class="muted">${history.boundaryNote}</p>`;
}

function renderLocalSessionRestore(state: LocalOperatorShellState): string {
  const restore = state.localSessionRestoreProjection;
  return `
    <dl>
      <div><dt>Selected package status</dt><dd>${restore.packageId ? "package_selected" : "no_package_selected"}</dd></div>
      <div><dt>Package ID</dt><dd>${restore.packageId ?? "none"}</dd></div>
      <div><dt>Package version</dt><dd>${restore.packageVersion ?? "none"}</dd></div>
      <div><dt>Package classification</dt><dd>${restore.packageClassification ?? "none"}</dd></div>
      <div><dt>Production classification</dt><dd>${restore.productionClassification ?? "none"}</dd></div>
      <div><dt>Read-back validation status</dt><dd>${restore.readBackStatus}</dd></div>
      <div><dt>Restore status</dt><dd>${restore.status}</dd></div>
      <div><dt>Restore rejection reason</dt><dd>${restore.errors.join(", ") || "none"}</dd></div>
      <div><dt>Included sections</dt><dd>${restore.includedSectionSummary.join(", ") || "none until package is selected"}</dd></div>
      <div><dt>Absence markers</dt><dd>${restore.absenceMarkerSummary.join(", ")}</dd></div>
      <div><dt>Boundary status</dt><dd>${restore.boundaryStatus.join(", ")}</dd></div>
    </dl>
    <p class="muted">${restore.localOnlyNote}</p>
    <p class="muted">${restore.readBackNote}</p>
    <p class="muted">${restore.previewBoundaryNote}</p>
    <p class="muted">${restore.restoredProjectionNote}</p>
    <p class="muted">${restore.remoteBackgroundNote}</p>`;
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
  if (!state.run.validation)
    return `<p class="muted">Validation and policy projection appears after the stub run.</p>`;
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

      <section class="panel" aria-label="Local provider adapter contract">
        <h2>Local provider adapter contract</h2>
        <h3>Adapter registry</h3>
        <h3>Adapter configuration</h3>
        ${renderAdapterRegistry(shellState)}
      </section>

      <section class="panel" aria-label="Controlled adapter dry run">
        <h2>Controlled adapter dry run</h2>
        ${renderAdapterDryRun(shellState)}
      </section>

      <section class="panel" aria-label="Constrained local provider invocation">
        <h2>Constrained local provider invocation</h2>
        ${renderConstrainedLocalProviderInvocation(shellState)}
      </section>

      <section class="panel" aria-label="Local provider configuration">
        <h2>Local provider configuration</h2>
        ${renderProviderConfiguration(shellState)}
      </section>

      <section class="panel" aria-label="Sandboxed provider execution">
        <h2>Sandboxed provider execution</h2>
        ${renderProviderExecution(shellState)}
      </section>

      <section class="panel" aria-label="Provider output pipeline integration">
        <h2>Provider output pipeline</h2>
        ${renderProviderOutputPipeline(shellState)}
      </section>

      <section class="panel" aria-label="Provider output review">
        ${renderProviderOutputReviewHtml(shellState)}
      </section>

      <section class="panel" aria-label="Staged proposal validation">
        <h2>Staged proposal validation</h2>
        ${renderStagedProposalValidation(shellState)}
      </section>


      <section class="panel" aria-label="Local session package">
        <h2>Local session package</h2>
        ${renderLocalSessionPackage(shellState)}
      </section>

      <section class="panel" aria-label="Session history">
        <h2>Session history</h2>
        ${renderSessionHistory(shellState)}
      </section>

      <section class="panel" aria-label="Local session restore">
        <h2>Local session restore</h2>
        ${renderLocalSessionRestore(shellState)}
      </section>

      <section class="panel" aria-label="Restore preview">
        <h2>Restore preview</h2>
        ${renderLocalSessionRestore(shellState)}
      </section>

      <section class="panel replay-panel">
        <h2>Replay / status projection</h2>
        ${renderReplayProjection(shellState)}
        <p class="muted">Rust-derived local replay projection only; replay repair, recovery promotion, readiness approval, provider execution, and action execution are not available in the UI.</p>
      </section>
    </main>`;

  document
    .querySelector<HTMLButtonElement>("#start-run")
    ?.addEventListener("click", startRun);
  document
    .querySelector<HTMLButtonElement>("#approve-run")
    ?.addEventListener("click", () => recordIntent("approve"));
  document
    .querySelector<HTMLButtonElement>("#reject-run")
    ?.addEventListener("click", () => recordIntent("reject"));
  document
    .querySelector<HTMLButtonElement>("#submit-adapter-declaration")
    ?.addEventListener("click", submitAdapterDeclaration);
  document
    .querySelector<HTMLButtonElement>("#reject-adapter-declaration")
    ?.addEventListener("click", submitUnsafeAdapterDeclaration);
  document
    .querySelector<HTMLButtonElement>("#run-adapter-dry-run")
    ?.addEventListener("click", runAdapterDryRun);
  document
    .querySelector<HTMLButtonElement>("#reject-adapter-dry-run")
    ?.addEventListener("click", runRejectedAdapterDryRun);
  document
    .querySelector<HTMLButtonElement>("#invoke-constrained-local-provider")
    ?.addEventListener("click", invokeAllowlistedLocalProvider);
  document
    .querySelector<HTMLButtonElement>("#reject-constrained-local-provider")
    ?.addEventListener("click", invokeRejectedLocalProvider);
  document
    .querySelector<HTMLButtonElement>("#submit-provider-config")
    ?.addEventListener("click", submitProviderConfiguration);
  document
    .querySelector<HTMLButtonElement>("#reject-provider-config")
    ?.addEventListener("click", submitUnsafeProviderConfiguration);
  document
    .querySelector<HTMLButtonElement>("#run-provider")
    ?.addEventListener("click", runDeterministicProvider);
  document
    .querySelector<HTMLButtonElement>("#reject-provider-execution")
    ?.addEventListener("click", runForbiddenProviderExecution);
  document
    .querySelector<HTMLButtonElement>("#create-staged-proposal")
    ?.addEventListener("click", createStagedProposal);
  document
    .querySelector<HTMLButtonElement>("#validate-staged-proposal")
    ?.addEventListener("click", validateStagedProposal);
}

render();
