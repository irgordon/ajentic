import { projectLocalProviderConfiguration, projectLocalProviderExecution, projectLocalProviderOutputValidation, type LocalOperatorShellState } from "./localOperatorShell";
import { renderProviderOutputReviewText } from "./providerOutputReview";

export function renderLocalOperatorShellSnapshot(state: LocalOperatorShellState): string {
  const decisionHistory = state.run.decisionTimeline.records.length === 0
    ? state.run.decisionTimeline.emptyMessage
    : state.run.decisionTimeline.records
        .map((record) => `#${record.recordedSequence} ${record.intentKind} ${record.decisionStatus} run=${record.runId} candidate=${record.candidateId} operator=${record.operatorId}`)
        .join("\n");

  const replay = state.run.decisionReplay;
  const replayLines = [
    `Replay status: ${replay.replayStatus}`,
    `Decision count: ${replay.sourceDecisionCount}`,
    `Latest decision ID: ${replay.latestDecisionId ?? "none"}`,
    `Latest run ID: ${replay.latestRunId ?? "none"}`,
    `Latest candidate ID: ${replay.latestCandidateId ?? "none"}`,
    `Latest operator ID: ${replay.latestOperatorId ?? "none"}`,
    `Latest decision kind: ${replay.latestDecisionKind ?? "none"}`,
    `Integrity: ${replay.integrityStatus}`,
    `Summary: ${replay.summary}`
  ].join("\n");

  const exportPreview = state.localSessionEvidenceExport;
  const exportLines = [
    `Export ID: ${exportPreview.exportId}`,
    `Export classification: ${exportPreview.exportClassification}`,
    `Production classification: ${exportPreview.productionClassification}`,
    `Export status: ${exportPreview.exportStatus}`,
    `Run ID/status: ${exportPreview.runId} / ${exportPreview.runStatus}`,
    `Candidate ID: ${exportPreview.candidateId}`,
    `Validation/policy status: ${exportPreview.validationStatus} / ${exportPreview.policyStatus}`,
    `Decision count: ${exportPreview.decisionCount}`,
    `Replay status: ${exportPreview.replayStatus}`,
    `Replay integrity: ${exportPreview.replayIntegrityStatus}`,
    `Absence markers summary: ${exportPreview.absenceMarkers.markerSummary.join(", ")}`
  ].join("\n");

  const providerConfiguration = projectLocalProviderConfiguration(state.providerConfiguration);
  const providerExecution = projectLocalProviderExecution(state);
  const providerOutputValidation = projectLocalProviderOutputValidation(state);
  const providerConfigurationLines = [
    `Configured provider kind: ${providerConfiguration.configuredProviderKind}`,
    `Provider configuration status: ${providerConfiguration.status}`,
    `Provider validation status: ${providerConfiguration.validationStatus}`,
    `Provider validation reason: ${providerConfiguration.validationReason}`,
    `Provider validation error code: ${providerConfiguration.validationErrorCodes.join(", ") || "none"}`,
    `Execution status: ${providerConfiguration.executionStatus}`,
    `Capability surface: ${providerConfiguration.capabilitySurface.summary}`,
    providerConfiguration.note
  ].join("\n");

  const providerExecutionLines = [
    `Projection status: ${providerExecution.projectionStatus}`,
    `Configured provider kind: ${providerExecution.configuredProviderKind}`,
    `Execution status: ${providerExecution.status}`,
    `Sandbox status: ${providerExecution.sandboxStatus}`,
    `Execution result ID: ${providerExecution.result?.resultId ?? "none"}`,
    `Provider output summary: ${providerExecution.result?.outputSummary ?? "none"}`,
    `Output trust status: untrusted/descriptive (${providerExecution.outputTrustStatus})`,
    `Output materialization status: ${providerExecution.outputMaterializationStatus}`,
    `Output promotion status: ${providerExecution.outputPromotionStatus}`,
    `Promotion availability: ${providerExecution.promotionAvailabilityStatus}`,
    `Run/session linkage: ${providerExecution.linkage.runId} / ${providerExecution.linkage.shellStateLabel} / ${providerExecution.linkage.providerConfigurationKind} / ${providerExecution.linkage.providerConfigurationStatus}`,
    `Source boundary: ${providerExecution.linkage.sourceBoundary}`,
    `Projection validation: ${providerExecution.projectionValidation.status} (${providerExecution.projectionValidation.errorCodes.join(", ") || "none"})`,
    `Absence markers: ${providerExecution.absenceMarkers.markerSummary.join(", ")}`,
    `Explicit Phase 142 boundary: provider output is not candidate material and is not review/approval material`,
    `Validation/error reason: ${providerExecution.validationReason}`,
    `Validation/error code: ${providerExecution.validationErrorCodes.join(", ") || "none"}`,
    `Capability surface: ${providerExecution.capabilitySurface.summary}`,
    providerExecution.note
  ].join("\n");


  const providerOutputValidationLines = [
    `Validation status: ${providerOutputValidation.status}`,
    `Reviewability status: ${providerOutputValidation.reviewabilityStatus}`,
    `Candidate-boundary status: ${providerOutputValidation.candidateBoundaryStatus}`,
    `Candidate-boundary details: ${providerOutputValidation.candidateBoundaryStatuses.join(", ")}`,
    `Validation reasons: ${providerOutputValidation.reasons.join(", ") || "none"}`,
    `Provider execution result ID: ${providerOutputValidation.providerExecutionResultId ?? "none"}`,
    `Provider kind: ${providerOutputValidation.providerKind}`,
    `Output trust status: ${providerOutputValidation.outputTrustStatus}`,
    `Promotion status: ${providerOutputValidation.outputPromotionStatus}`,
    `No-effect summary: trust_effect=${providerOutputValidation.trustEffect}, candidate_effect=${providerOutputValidation.candidateEffect}, decision_ledger_effect=${providerOutputValidation.decisionLedgerEffect}, replay_effect=${providerOutputValidation.replayEffect}, export_effect=${providerOutputValidation.exportEffect}, action_effect=${providerOutputValidation.actionEffect}, readiness_effect=${providerOutputValidation.readinessEffect}, release_effect=${providerOutputValidation.releaseEffect}, deployment_effect=${providerOutputValidation.deploymentEffect}`,
    "Explicit Phase 144 note: reviewable_untrusted is not candidate material and cannot be approved in Phase 144; provider output is not promoted",
    providerOutputValidation.note
  ].join("\n");

  const stagedProjection = state.stagedCandidateConversionProposal;
  const stagedProposal = stagedProjection.proposal;
  const stagedProposalLines = [
    `Proposal status: ${stagedProjection.status}`,
    `Proposal ID: ${stagedProposal?.proposalId ?? "none"}`,
    `Source provider kind: ${stagedProposal?.sourceProviderKind ?? providerOutputValidation.providerKind}`,
    `Source execution result ID: ${stagedProposal?.sourceExecutionResultId ?? providerOutputValidation.providerExecutionResultId ?? "none"}`,
    `Source validation status: ${stagedProposal?.sourceValidationStatus ?? providerOutputValidation.status}`,
    `Source reviewability status: ${stagedProposal?.sourceReviewabilityStatus ?? providerOutputValidation.reviewabilityStatus}`,
    `Source candidate-boundary status: ${stagedProposal?.sourceCandidateBoundaryStatus ?? providerOutputValidation.candidateBoundaryStatus}`,
    `Staged-only boundary status: ${stagedProposal?.boundaryStatuses.join(", ") ?? "staging_only_not_candidate_material"}`,
    `Trust status: ${stagedProposal?.trustStatuses.join(", ") ?? "untrusted_source, not_trusted, not_approved"}`,
    `Approval status: not_approved`,
    `Candidate conversion status: candidate_conversion_not_performed`,
    `Future validation requirement: validation_required_in_future_phase`,
    `No-effect summary: ${stagedProposal?.effectStatuses.join(", ") ?? "no_decision_ledger_effect, no_replay_effect, no_export_effect, no_provider_configuration_effect, no_provider_execution_effect, no_action_effect, no_persistence_effect, no_readiness_effect, no_release_effect, no_deployment_effect, not_executable, not_persistent"}`,
    "This is a staged conversion proposal only. It is not candidate output.",
    "Provider output remains untrusted and not approved.",
    "Candidate conversion was not performed in Phase 146.",
    "Validation is required in a future phase before any candidate review.",
    "Approval is not available in Phase 146.",
    stagedProjection.note
  ].join("\n");

  const stagedValidation = state.stagedCandidateConversionValidation;
  const stagedValidationLines = [
    `Validation status: ${stagedValidation.status}`,
    `Validation reasons: ${stagedValidation.reasons.join(", ") || "none"}`,
    `Proposal ID: ${stagedValidation.proposalId ?? "none"}`,
    `Source provider kind: ${stagedValidation.sourceProviderKind}`,
    `Source execution result ID: ${stagedValidation.sourceExecutionResultId ?? "none"}`,
    `Source validation status: ${stagedValidation.sourceValidationStatus}`,
    `Source reviewability status: ${stagedValidation.sourceReviewabilityStatus}`,
    `Source candidate-boundary status: ${stagedValidation.sourceCandidateBoundaryStatus}`,
    `Deterministic linkage status: ${stagedValidation.deterministicLinkageStatus}`,
    `Materialization status: ${stagedValidation.materializationStatuses.join(", ")}`,
    `Future review boundary status: ${stagedValidation.futureReviewBoundaryStatus}`,
    `Operator decision availability: ${stagedValidation.operatorDecisionStatus}`,
    `Trust status: ${stagedValidation.trustStatuses.join(", ")}`,
    `Approval status: not_approved`,
    `No-effect summary: ${stagedValidation.noEffectSummary.join(", ")}`,
    "Validation checks staged proposal shape and source linkage only.",
    "Validated staged proposal is not candidate output.",
    "Candidate materialization was not performed in Phase 147.",
    "Future review boundary is required before any operator decision.",
    "Operator decision is not available in Phase 147.",
    "Provider output remains untrusted and not approved.",
    stagedValidation.note
  ].join("\n");

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
    "Local decision ledger",
    decisionHistory,
    "Local provider configuration",
    providerConfigurationLines,
    "Sandboxed provider execution",
    "Provider execution result projection",
    "Run deterministic provider",
    providerExecutionLines,
    "Provider output validation",
    "Provider output reviewability",
    providerOutputValidationLines,
    renderProviderOutputReviewText(state),
    "Candidate conversion staging",
    "Staged candidate-conversion proposal",
    "Create staged conversion proposal",
    stagedProposalLines,
    "Staged proposal validation",
    "Validate staged proposal shape/linkage",
    stagedValidationLines,
    "Bottom panel: Replay/status projection",
    replayLines,
    "Local session evidence export",
    exportLines
  ].join("\n");
}
