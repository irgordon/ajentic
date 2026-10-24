import { localCandidateMaterializationRequestFromState, projectLocalProviderAdapterRegistry, projectLocalProviderConfiguration, projectLocalProviderExecution, type LocalOperatorShellState, type CompleteLocalOperatorWorkflowProjection } from "../api/localOperatorShell.js";
import { displayStatus, plainCategoryLabel } from "../display/statusCopy.js";

export function renderPlainEnglishGlossary(): string {
  return `
    <section class="panel glossary-panel" aria-label="Glossary">
      <h2>Glossary</h2>
      <details open>
        <summary>What does simulated mean?</summary>
        <p>The page uses predictable local test data instead of a live database or cloud service.</p>
      </details>
      <details>
        <summary>What does blocked mean?</summary>
        <p>Blocked means action is needed before local release-candidate work can continue.</p>
      </details>
      <details>
        <summary>What does evidence mean?</summary>
        <p>Evidence is local information used to inspect checks, artifacts, checksums, SBOM data, provenance, or review status.</p>
      </details>
      <details>
        <summary>What does local-only mean?</summary>
        <p>Local-only means this page does not contact cloud services, publish, sign, deploy, or create public releases.</p>
      </details>
      <details>
        <summary>What happens next?</summary>
        <p>Review the checks, fix blocked or missing items, and continue release-candidate work through the governed Rust-owned path.</p>
      </details>
    </section>`;
}

export function renderCompleteLocalOperatorWorkflow(
  workflow: CompleteLocalOperatorWorkflowProjection,
): string {
  const blocker = workflow.currentBlockingStep ?? "none";
  const current = workflow.currentStep ?? "none";
  const next = workflow.nextRequiredStep ?? "none";
  const rejectionReasons =
    workflow.rejectionReasons.length === 0
      ? "<li>none</li>"
      : workflow.rejectionReasons
          .map((reason) => `<li>${reason}</li>`)
          .join("");
  const stepItems = workflow.steps
    .map(
      (step) =>
        `<li><strong>${step.step}</strong>: ${step.status}${step.error ? ` (${step.error})` : ""}<br /><span class="muted">${step.summary}</span></li>`,
    )
    .join("");
  const evidence = workflow.evidenceSummary;
  return `
    <div class="workflow-summary">
      <p><strong>Workflow status:</strong> ${workflow.status}</p>
      <p><strong>Workflow classification:</strong> ${workflow.classification}</p>
      <p><strong>Current step:</strong> ${current}</p>
      <p><strong>Next required step:</strong> ${next}</p>
      <p><strong>Current blocking step:</strong> ${blocker}</p>
      <p><strong>Current error:</strong> ${workflow.currentError ?? "none"}</p>
      <h3>Workflow Steps</h3>
      <ol>${stepItems}</ol>
      <h3>Request Errors</h3>
      <ul>${rejectionReasons}</ul>
      <h3>Evidence Status</h3>
      <dl>
        <div><dt>Provider output pipeline</dt><dd>${evidence.providerOutputPipelineStatus}</dd></div>
        <div><dt>Local candidate materialization</dt><dd>${evidence.localCandidateMaterializationStatus}</dd></div>
        <div><dt>Replay/status projection</dt><dd>${evidence.replayStatus}</dd></div>
        <div><dt>Local evidence export</dt><dd>${evidence.localEvidenceExportStatus}</dd></div>
        <div><dt>Session package</dt><dd>${evidence.sessionPackageStatus}</dd></div>
        <div><dt>Session history</dt><dd>${evidence.sessionHistoryStatus}</dd></div>
        <div><dt>Restore status</dt><dd>${evidence.restoreStatus}</dd></div>
      </dl>
      <h3>Local Boundaries</h3>
      ${renderList(workflow.boundaryStatuses, "No boundary markers projected.")}
      <p>${workflow.localOnlyNote}</p>
      <p>Workflow completion does not approve readiness, release, deployment, public use, or production use.</p>
      <p>Provider output remains untrusted unless a later bounded phase explicitly changes that.</p>
      <p>Workflow status does not authorize actions.</p>
      <p>Replay is not repaired and recovery is not promoted.</p>
    </div>`;
}

export function renderCandidate(state: LocalOperatorShellState): string {
  if (!state.run.candidate)
    return `<p class="muted">Run the local simulation to display candidate output.</p>`;
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

export function renderAdapterRegistry(state: LocalOperatorShellState): string {
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
      <button id="submit-adapter-declaration" type="button">Set Up Connector</button>
      <button id="reject-adapter-declaration" type="button">Test Blocked Connector</button>
    </div>`;
}

export function renderAdapterDryRun(state: LocalOperatorShellState): string {
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
      <button id="run-adapter-dry-run" type="button" ${canRun ? "" : "disabled"}>Check Connector</button>
      <button id="reject-adapter-dry-run" type="button">Test Blocked Check</button>
    </div>`;
}

export function renderConstrainedLocalProviderInvocation(
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
      <button id="invoke-constrained-local-provider" type="button" ${canInvoke ? "" : "disabled"}>Request Sample</button>
      <button id="reject-constrained-local-provider" type="button">Test Blocked Request</button>
    </div>`;
}

export function renderProviderOutputPipeline(state: LocalOperatorShellState): string {
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

export function renderProviderConfiguration(state: LocalOperatorShellState): string {
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
      <button id="submit-provider-config" type="button">Set Up Sample</button>
      <button id="reject-provider-config" type="button">Test Blocked Settings</button>
    </div>`;
}

export function renderProviderExecution(state: LocalOperatorShellState): string {
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
      <button id="run-provider" type="button" ${state.providerConfiguration.status === "accepted" ? "" : "disabled"}>Run Sample Tool</button>
      <button id="reject-provider-execution" type="button">Test Blocked Tool</button>
    </div>`;
}

export function renderStagedProposalValidation(
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
      <button id="create-staged-proposal" type="button">Prepare Draft</button>
      <button id="validate-staged-proposal" type="button">Check Draft</button>
    </div>`;
}

export function renderLocalSessionPackage(state: LocalOperatorShellState): string {
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

export function renderSessionHistory(state: LocalOperatorShellState): string {
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

export function renderLocalSessionRestore(state: LocalOperatorShellState): string {
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

export function renderTrialOperatorRunbook(state: LocalOperatorShellState): string {
  const runbook = state.trialOperatorRunbook;
  const steps = runbook.steps
    .map(
      (step) =>
        `<li><strong>${step.step}</strong>: ${step.status}<br /><span class="muted">${step.summary}</span></li>`,
    )
    .join("");
  return `
    <dl>
      <div><dt>Runbook status</dt><dd>${runbook.status}</dd></div>
      <div><dt>Current runbook step</dt><dd>${runbook.currentStep ?? "none"}</dd></div>
      <div><dt>Current blocker</dt><dd>${runbook.currentBlocker.step ?? "none"}</dd></div>
      <div><dt>Current blocker guidance</dt><dd>${runbook.currentBlocker.guidance}</dd></div>
      <div><dt>Trial package status</dt><dd>${runbook.trialPackageStatus}</dd></div>
      <div><dt>Trial package ID</dt><dd>${runbook.trialPackageId ?? "none"}</dd></div>
      <div><dt>Package validation/read-back</dt><dd>${runbook.trialPackageValidationStatus} / ${runbook.trialPackageReadBackStatus ?? "not_read"}</dd></div>
      <div><dt>Trial scope status</dt><dd>${runbook.trialScopeStatus}</dd></div>
      <div><dt>Named operator/participant status</dt><dd>${runbook.namedOperatorStatus} / ${runbook.namedParticipantStatus}</dd></div>
      <div><dt>Stop-condition summary</dt><dd>${runbook.stopConditionSummary.join(", ") || "none"}</dd></div>
      <div><dt>Workflow status</dt><dd>${runbook.localWorkflowStatus}</dd></div>
      <div><dt>Candidate materialization status</dt><dd>${runbook.localCandidateMaterializationStatus}</dd></div>
      <div><dt>Provider output pipeline status</dt><dd>${runbook.providerOutputPipelineStatus}</dd></div>
      <div><dt>Replay/status summary</dt><dd>${runbook.replayStatusSummary}</dd></div>
      <div><dt>Evidence export summary</dt><dd>${runbook.localEvidenceExportSummary}</dd></div>
      <div><dt>Restore/history status</dt><dd>${runbook.restoreHistoryStatus}</dd></div>
      <div><dt>Boundary markers</dt><dd>${runbook.boundaryStatuses.join(", ")}</dd></div>
    </dl>
    <h3>Test Steps</h3>
    <ol>${steps}</ol>
    <p class="muted">${runbook.localOnlyNonPublicNote}</p>
    <p class="muted">${runbook.noTrialExecutionNote}</p>
    <p class="muted">${runbook.noAuthorityNote}</p>`;
}

export function renderTrialFailureDrill(state: LocalOperatorShellState): string {
  const drill = state.trialFailureDrill;
  const categories =
    drill.categories.length === 0
      ? `<p class="muted">No failure categories projected.</p>`
      : `<ul>${drill.categories.map((entry) => `<li><strong>${entry.category}</strong>: ${entry.severity}<br /><span class="muted">${entry.summary}</span></li>`).join("")}</ul>`;
  return `
    <dl>
      <div><dt>Failure drill status</dt><dd>${drill.status}</dd></div>
      <div><dt>Failure severity</dt><dd>${drill.highestSeverity}</dd></div>
      <div><dt>Manual action guidance</dt><dd>${drill.manualActionGuidance.join(" | ") || "none"}</dd></div>
      <div><dt>Rejection summary</dt><dd>${drill.rejectionSummary.join(" | ") || "none"}</dd></div>
      <div><dt>Boundary markers</dt><dd>${drill.boundaryStatuses.join(", ")}</dd></div>
    </dl>
    <h3>Failure Categories</h3>
    ${categories}
    <p class="muted">${drill.noAutomationNote}</p>`;
}

export function renderStopConditionDrill(state: LocalOperatorShellState): string {
  const drills = state.trialFailureDrill.stopConditionDrills;
  const entries =
    drills.length === 0
      ? `<p class="muted">No stop-condition drill markers are projected.</p>`
      : `<ul>${drills.map((entry) => `<li><strong>${entry.marker}</strong>: ${entry.status}<br /><span class="muted">${entry.guidance}</span><br /><span class="muted">Automated enforcement: ${entry.enforcementAutomated}</span></li>`).join("")}</ul>`;
  return `
    ${entries}
    <p class="muted">Stop conditions are guidance only; enforcement is not automated in Phase 162.</p>`;
}

export function renderEscalationGuidance(state: LocalOperatorShellState): string {
  const guidance = state.trialFailureDrill.escalationGuidance;
  const entries =
    guidance.length === 0
      ? `<p class="muted">No escalation guidance projected.</p>`
      : `<ul>${guidance.map((entry) => `<li><strong>${entry.role}</strong>: ${entry.categories.join(", ")}<br /><span class="muted">${entry.guidance}</span><br /><span class="muted">Descriptive only: ${entry.descriptiveOnly}</span></li>`).join("")}</ul>`;
  return `
    ${entries}
    <p class="muted">Escalation guidance is descriptive only and does not activate authority.</p>`;
}

export function renderTrialObservability(state: LocalOperatorShellState): string {
  const observability = state.trialObservability;
  return `
    <dl>
      <div><dt>Observability status</dt><dd>${observability.status}</dd></div>
      <div><dt>Trial run status</dt><dd>${observability.trialRunStatus}</dd></div>
      <div><dt>Current trial step</dt><dd>${observability.currentTrialStep}</dd></div>
      <div><dt>Current blocker</dt><dd>${observability.currentBlocker}</dd></div>
      <div><dt>Stop-condition observation status</dt><dd>${observability.stopConditionObservation.status}</dd></div>
      <div><dt>Manual operator step status</dt><dd>${observability.manualOperatorStepStatus}</dd></div>
      <div><dt>Package status</dt><dd>${observability.packageStatus}</dd></div>
      <div><dt>Evidence status</dt><dd>${observability.evidenceStatus}</dd></div>
      <div><dt>Read-back status</dt><dd>package=${observability.packageReadBackStatus}; evidence=${observability.evidenceReadBackStatus}</dd></div>
      <div><dt>Replay/restore verification status</dt><dd>${observability.replayRestoreVerificationStatus}</dd></div>
      <div><dt>Mismatch summary</dt><dd>${observability.mismatchSummary.mismatches.join(", ") || "none"}</dd></div>
      <div><dt>Evidence linkage</dt><dd>${observability.evidenceLinkageSummary.join(" | ") || "none"}</dd></div>
      <div><dt>Boundary markers</dt><dd>${observability.boundaryStatuses.join(", ")}</dd></div>
    </dl>
    <p class="muted">${observability.localOnlyNonPublicNote}</p>
    <p class="muted">${observability.noProductionMonitoringNote}</p>
    <p class="muted">${observability.noRemoteTelemetryNote}</p>
    <p class="muted">${observability.noBackgroundServiceNote}</p>
    <p class="muted">${observability.noRemediationEscalationStopAutomationNote}</p>
    <p class="muted">${observability.noApprovalNote}</p>`;
}

export function renderTrialErrorReporting(state: LocalOperatorShellState): string {
  const report = state.trialErrorReport;
  const details =
    report.details.length === 0
      ? `<p class="muted">No local trial errors are projected.</p>`
      : `<ul>${report.details.map((detail) => `<li><strong>${detail.category}</strong> (${detail.severity})<br /><span class="muted">Source: ${detail.source}</span><br /><span class="muted">${detail.summary}</span><br /><span class="muted">Guidance: ${detail.operatorGuidance}</span><br /><span class="muted">Evidence: ${detail.evidenceLinkage.linkage}</span></li>`).join("")}</ul>`;
  return `
    <dl>
      <div><dt>Error report status</dt><dd>${report.status}</dd></div>
      <div><dt>Failure category summary</dt><dd>${report.categorySummary.join(", ") || "none"}</dd></div>
      <div><dt>Error severity</dt><dd>${report.highestSeverity}</dd></div>
      <div><dt>Evidence linkage</dt><dd>${report.evidenceLinkageSummary.join(" | ") || "none"}</dd></div>
    </dl>
    <h3>Test Errors</h3>
    ${details}
    <p class="muted">${report.localDescriptiveOnlyNote}</p>`;
}

export function renderTrialBlockedStateSummary(
  state: LocalOperatorShellState,
): string {
  const summary = state.trialObservability.blockedStateSummary;
  return `
    <dl>
      <div><dt>Blocked-state summary</dt><dd>${summary.status}</dd></div>
      <div><dt>Current blocker</dt><dd>${summary.currentBlocker}</dd></div>
      <div><dt>Rejection reasons</dt><dd>${summary.rejectionReasons.join(", ") || "none"}</dd></div>
    </dl>`;
}

export function renderReplayProjection(state: LocalOperatorShellState): string {
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

export function renderValidation(state: LocalOperatorShellState): string {
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

export function renderReleaseCandidatePreparation(
  state: LocalOperatorShellState,
): string {
  const projection = state.releaseCandidatePreparation;
  return `
    <div class="panel__header">
      <div>
        <p class="muted">Review local release-candidate checks before work continues.</p>
        ${renderStatusBadge(projection.status, "Raw preparation status")}
      </div>
      ${renderTechnicalDetails("Preparation details", [
        ["Preparation ID", projection.preparationId],
        ["Raw preparation status", projection.status],
        ["Boundary markers", projection.boundaryStatuses.join(", ")],
      ])}
    </div>
    ${renderEvidenceSummaryCards(projection)}
    <h3>Evidence Log</h3>
    ${renderEvidenceVerificationLog(projection)}
    <h3>Checks Needing Action</h3>
    ${renderEvidenceIssueList(
      "Blocked evidence",
      projection.blockers,
      "No blocked checks.",
    )}
    <h3>Checks Not Started</h3>
    ${renderEvidenceIssueList(
      "Missing Evidence",
      projection.missingEvidence,
      "No missing checks.",
    )}
    <div class="boundary-note">
      <p>${projection.noReleaseReadinessNote}</p>
      <p>${projection.noReleaseArtifactNote}</p>
      <p>${projection.noReleaseCandidateStatusNote}</p>
      <p>${projection.noProductionDeploymentPublicSigningPublishingInstallerUpdateNote}</p>
      <p>${projection.noProviderTrustNote}</p>
      <p>${projection.noActionAuthorizationNote}</p>
    </div>`;
}

export function renderReleaseArtifactDryPackage(
  state: LocalOperatorShellState,
): string {
  const projection = state.releaseArtifactDryPackage;
  return `
    <p><strong>Internal candidate bundle status:</strong> ${projection.status}</p>
    <p><strong>Dry package ID:</strong> ${projection.dryPackageId ?? "none"}</p>
    <p><strong>Dry package classification:</strong> ${projection.dryPackageClassification}</p>
    <p><strong>Production classification:</strong> ${projection.productionClassification}</p>
    <p><strong>Distribution classification:</strong> ${projection.distributionClassification}</p>
    <p><strong>Authority classification:</strong> ${projection.authorityClassification}</p>
    <p><strong>Release classification:</strong> ${projection.releaseClassification}</p>
    <p><strong>Included evidence count:</strong> ${projection.includedEvidenceCount}</p>
    <h3>Included Evidence</h3>
    ${renderList(
      projection.includedEvidence.map(
        (item) =>
          `${item.category}: ${item.categoryStatus} (${item.sourceSurface}:${item.sourceStatus}) ${item.sourceSummary}`,
      ),
      "No dry package evidence included.",
    )}
    <p><strong>Validation status:</strong> ${projection.validationStatus}</p>
    <p><strong>Read-back validation status:</strong> ${projection.readBackValidationStatus}</p>
    <p><strong>Rejection reason:</strong> ${projection.validationErrors.join(", ") || "none"}</p>
    <p><strong>Boundary markers:</strong> ${projection.boundaryStatuses.join(", ")}</p>
    <p class="muted">${projection.dryPackageOnlyNote}</p>
    <p class="muted">${projection.notReleaseArtifactNote}</p>
    <p class="muted">${projection.noReadinessOrCandidateApprovalNote}</p>
    <p class="muted">${projection.localOnlyNonPublicNote}</p>
    <p class="muted">${projection.noDistributionNote}</p>
    <p class="muted">${projection.readBackValidationNote}</p>`;
}

export function renderInstallerDistributionContract(state: LocalOperatorShellState): string {
  const projection = state.installerDistributionContract;
  return `
    <p><strong>Contract status:</strong> ${projection.status}</p>
    <p><strong>Contract ID:</strong> ${projection.contractId ?? "none"}</p>
    <p><strong>Contract classification:</strong> ${projection.classification}</p>
    <p><strong>Production classification:</strong> ${projection.productionClassification}</p>
    <p><strong>Distribution classification:</strong> ${projection.distributionClassification}</p>
    <p><strong>Authority classification:</strong> ${projection.authorityClassification}</p>
    <p><strong>Release classification:</strong> ${projection.releaseClassification}</p>
    <p><strong>Dry package linkage:</strong> ${projection.dryPackageLinkage}</p>
    <p><strong>Checksum/provenance linkage:</strong> ${projection.checksumProvenanceLinkage}</p>
    <p><strong>Validation status:</strong> ${projection.validationStatus}</p>
    <p class="muted">The installer/distribution contract describes constraints only.</p>
    <p class="muted">This contract does not create an installer.</p>
    <p class="muted">This contract does not activate an update channel.</p>`;
}

export function renderReleaseCandidateEvidenceAssembly(state: LocalOperatorShellState): string {
  const p = state.releaseCandidateEvidenceAssembly;
  return `
    <p><strong>Internal evidence status:</strong> ${p.status}</p>
    <p><strong>Assembly ID:</strong> ${p.assemblyId ?? "none"}</p>
    <p><strong>Evidence category count:</strong> ${p.categoryCount}</p>
    <p><strong>Present category count:</strong> ${p.presentCategoryCount}</p>
    <p><strong>Missing category count:</strong> ${p.missingCategoryCount}</p>
    <p><strong>Blocked category count:</strong> ${p.blockedCategoryCount}</p>
    <p><strong>Rejected category count:</strong> ${p.rejectedCategoryCount}</p>
    <h3>Source Connections</h3>
    ${renderList(p.evidenceItems.map((item) => `${item.category}: ${item.status} (${item.sourceLinkage.sourceSurface}:${item.sourceLinkage.sourceStatus})`), "No evidence items projected.")}
    <h3>Missing Evidence</h3>
    ${renderList(p.missingEvidence.map((item) => `${item.category}: ${item.reason}`), "No missing evidence.")}
    <h3>Blockers</h3>
    ${renderList(p.blockers.map((item) => `${item.category}: ${item.reason}`), "No blockers.")}
    <h3>Check Status</h3>
    ${renderList(p.validationSummaries.map((item) => `${item.sourceSurface}: ${item.validationStatus}`), "No validation summaries.")}
    <p class="muted">Evidence assembly organizes release-candidate evidence for review.</p>
    <p class="muted">Evidence assembly does not approve Release Candidate status.</p>
    <p class="muted">Evidence assembly does not approve release readiness.</p>
    <p class="muted">Evidence assembly does not sign, publish, deploy, release, or distribute anything.</p>
    <p class="muted">No public download, GitHub release, release tag, installer, or update channel is created.</p>`;
}

export function renderSigningKeyCustodyDryRun(state: LocalOperatorShellState): string {
  const projection = state.signingKeyCustodyDryRun;
  return `
    <p><strong>Internal attestation evidence status:</strong> ${projection.status}</p>
    <p><strong>Dry-run evidence ID:</strong> ${projection.evidenceId ?? "none"}</p>
    <p><strong>Dry-run classification:</strong> ${projection.classification}</p>
    <p><strong>Production classification:</strong> ${projection.productionClassification}</p>
    <p><strong>Distribution classification:</strong> ${projection.distributionClassification}</p>
    <p><strong>Authority classification:</strong> ${projection.authorityClassification}</p>
    <p><strong>Release classification:</strong> ${projection.releaseClassification}</p>
    <p><strong>Source Evidence:</strong> ${projection.upstreamEvidenceLinkage}</p>
    <p><strong>Placeholder key metadata:</strong> ${projection.placeholderKeyMetadataSummary}</p>
    <p><strong>Validation status:</strong> ${projection.validationStatus}</p>
    <p class="muted">Signing/key-custody dry run uses placeholder key metadata only.</p>
    <p class="muted">This dry run does not use real signing keys.</p>
    <p class="muted">This dry run does not create signatures.</p>
    <p class="muted">This dry run does not sign, publish, deploy, release, or distribute anything.</p>
    <p class="muted">This dry run does not approve release readiness or Release Candidate status.</p>`;
}

export function renderReleaseCandidateGapReview(state: LocalOperatorShellState): string {
  const g = state.releaseCandidateGapReview;
  const gaps = g.gaps.map((x) => `<li><strong>${x.severity}</strong> · ${x.affectedSurface} · ${x.category}<br/>${x.finding}</li>`).join("");
  const hardening = g.hardeningCandidates.map((x) => `<li>${x.summary} (${x.priority})</li>`).join("");
  const linkage = g.sourceLinkage.map((x) => `<li>${x.sourceSurface}: ${x.sourceStatus}</li>`).join("");
  const missing = g.missingEvidence.map((x) => `<li>${x.sourceSurface}: ${x.reason}</li>`).join("");
  const blockers = g.blockers.map((x) => `<li>${x.sourceSurface}: ${x.reason}</li>`).join("");
  return `<p class="muted">Gap review turns assembled evidence into hardening work.</p>
  <p>Status: <code>${g.status}</code></p>
  <p>Gap review ID: <code>${g.gapReviewId ?? "not_generated"}</code></p>
  <p>Gaps: ${g.gapCount}; Blocking: ${g.blockingGapCount}; Informational: ${g.informationalGapCount}; Suggested Fixes: ${g.hardeningCandidateCount}</p>
  <h3>Gaps</h3><ul>${gaps}</ul>
  <h3>Suggested Fixes</h3><ul>${hardening}</ul>
  <h3>Source Evidence</h3><ul>${linkage}</ul>
  <h3>Missing Evidence</h3><ul>${missing}</ul>
  <h3>Blockers</h3><ul>${blockers}</ul>
  <p class="muted">Gap review does not approve Release Candidate status.</p>
  <p class="muted">Gap review does not approve release readiness.</p>
  <p class="muted">Gap review does not sign, publish, deploy, release, or distribute anything.</p>
  <p class="muted">No public download, GitHub release, release tag, installer, or update channel is created.</p>`;
}

export function renderEvidenceSummaryCards(
  projection: LocalOperatorShellState["releaseCandidatePreparation"],
): string {
  const cards = [
    {
      label: "Total checks",
      value: projection.categoryCount,
      helper: "Local review checks tracked here.",
      tone: "neutral",
    },
    {
      label: "Available",
      value: projection.presentEvidenceCount,
      helper: "Evidence exists.",
      tone: "ready",
    },
    {
      label: "Needs attention",
      value: projection.blockedEvidenceCount,
      helper: "Action needed before continuing.",
      tone: "attention",
    },
    {
      label: "Not started",
      value: projection.missingEvidenceCount,
      helper: "Not supplied yet.",
      tone: "missing",
    },
    {
      label: "Rejected",
      value: projection.rejectedEvidenceCount,
      helper: "Explicitly rejected evidence.",
      tone: "rejected",
    },
  ];

  return `<div class="evidence-summary" aria-label="Evidence summary">${cards
    .map(
      (card) => `
        <article class="summary-card summary-card--${card.tone}" tabindex="0" title="${card.helper}">
          <span>${card.label}</span>
          <strong>${card.value}</strong>
          <p>${card.helper}</p>
        </article>`,
    )
    .join("")}</div>`;
}

export function renderEvidenceVerificationLog(
  projection: LocalOperatorShellState["releaseCandidatePreparation"],
): string {
  if (projection.evidenceItems.length === 0) {
    return `<p class="muted">No evidence checks are projected.</p>`;
  }

  return `<div class="evidence-log">${projection.evidenceItems
    .map((item) => {
      const status = displayStatus(item.status);
      return `
        <article class="evidence-row evidence-row--${status.tone}">
          <div class="evidence-row__main">
            <span class="status-dot" aria-hidden="true"></span>
            <div>
              <h4>${plainCategoryLabel(item.category)}</h4>
              <p>${item.sourceLinkage.sourceSummary}</p>
            </div>
          </div>
          ${renderStatusBadge(item.status, "Raw evidence status")}
          ${renderTechnicalDetails("Raw Details", [
            ["Raw category", item.category],
            ["Raw status", item.status],
            ["Source surface", item.sourceLinkage.sourceSurface],
            ["Source status", item.sourceLinkage.sourceStatus],
          ])}
        </article>`;
    })
    .join("")}</div>`;
}

export function renderEvidenceIssueList(
  title: string,
  items: LocalOperatorShellState["releaseCandidatePreparation"]["missingEvidence"],
  emptyText: string,
): string {
  if (items.length === 0) return `<p class="muted">${emptyText}</p>`;
  return `<ul class="issue-list" aria-label="${title}">${items
    .map(
      (item) =>
        `<li><strong>${plainCategoryLabel(item.category)}</strong><span>${item.reason}</span>${renderTechnicalDetails("Raw Details", [["Raw category", item.category], ["Source surface", item.sourceSurface]])}</li>`,
    )
    .join("")}</ul>`;
}

export function renderList(items: readonly string[], emptyText: string): string {
  if (items.length === 0) return `<p class="muted">${emptyText}</p>`;
  return `<ul>${items.map((item) => `<li>${item}</li>`).join("")}</ul>`;
}

export function renderStatusBadge(raw: string, rawLabel = "Raw status"): string {
  const status = displayStatus(raw);
  return `<span class="status-badge-group"><span class="status-badge status-badge--${status.tone}" tabindex="0" title="${status.description}" aria-label="${status.label}. ${status.description} ${rawLabel}: ${status.raw}."><span class="status-dot" aria-hidden="true"></span>${status.label}</span><span class="status-badge-help">${status.description}</span></span>`;
}

export function renderTechnicalDetails(
  summary: string,
  rows: readonly [string, string][],
): string {
  const terms = rows
    .map(
      ([term, value]) =>
        `<div><dt>${term}</dt><dd><code>${value}</code></dd></div>`,
    )
    .join("");
  return `<details class="technical-details"><summary>${summary}</summary><dl>${terms}</dl></details>`;
}
