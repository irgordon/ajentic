import {
  projectLocalProviderAdapterRegistry,
  projectLocalProviderConfiguration,
  projectLocalProviderExecution,
  projectLocalProviderOutputValidation,
  type LocalOperatorShellState,
} from "./localOperatorShell";
import { renderProviderOutputReviewText } from "./providerOutputReview";
import { renderCandidateReviewSurface } from "./candidateReviewSurface";

export function renderLocalOperatorShellSnapshot(
  state: LocalOperatorShellState,
): string {
  const decisionHistory =
    state.run.decisionTimeline.records.length === 0
      ? state.run.decisionTimeline.emptyMessage
      : state.run.decisionTimeline.records
          .map(
            (record) =>
              `#${record.recordedSequence} ${record.intentKind} ${record.decisionStatus} run=${record.runId} candidate=${record.candidateId} operator=${record.operatorId}`,
          )
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
    `Summary: ${replay.summary}`,
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
    `Absence markers summary: ${exportPreview.absenceMarkers.markerSummary.join(", ")}`,
  ].join("\n");


  const trialSessionEvidence = state.trialSessionEvidenceProjection;
  const trialSessionEvidenceLines = [
    `Evidence status: ${trialSessionEvidence.status}`,
    `Evidence ID: ${trialSessionEvidence.evidenceId ?? "none"}`,
    `Evidence version: ${trialSessionEvidence.evidenceVersion}`,
    `Evidence classification: ${trialSessionEvidence.evidenceClassification}`,
    `Production classification: ${trialSessionEvidence.productionClassification}`,
    `Distribution classification: ${trialSessionEvidence.distributionClassification}`,
    `Authority classification: ${trialSessionEvidence.authorityClassification}`,
    `Trial package linkage: ${trialSessionEvidence.trialPackageLinkage}`,
    `Runbook/failure drill linkage: ${trialSessionEvidence.runbookLinkage} / ${trialSessionEvidence.failureDrillLinkage}`,
    `Included evidence summary: ${trialSessionEvidence.includedEvidenceSummary.join(", ") || "not captured"}`,
    `Workflow snapshot summary: ${trialSessionEvidence.workflowSnapshotSummary}`,
    `Local candidate materialization snapshot: ${trialSessionEvidence.localCandidateMaterializationSnapshot}`,
    `Replay/status snapshot: ${trialSessionEvidence.replayStatusSnapshot}`,
    `Evidence export/session package/restore snapshot: ${trialSessionEvidence.evidenceExportSessionPackageRestoreSnapshot}`,
    `Stop-condition snapshot: ${trialSessionEvidence.stopConditionSnapshot.join(", ") || "not captured"}`,
    `Escalation snapshot: ${trialSessionEvidence.escalationSnapshot.join(" | ") || "not captured"}`,
    `Failure category snapshot: ${trialSessionEvidence.failureCategorySnapshot.join(", ") || "not captured"}`,
    `Absence marker summary: ${trialSessionEvidence.absenceMarkerSummary.join(", ")}`,
    `Validation status: ${trialSessionEvidence.validationStatus}`,
    `Validation errors: ${trialSessionEvidence.validationErrors.join(", ") || "none"}`,
    `Read-back validation status: ${trialSessionEvidence.readBackValidationStatus ?? "not_read"}`,
    `Boundary markers: ${trialSessionEvidence.boundaryStatus.join(", ")}`,
    trialSessionEvidence.localOnlyNonAuthoritativeNote,
    trialSessionEvidence.localPreparationOnlyNote,
    trialSessionEvidence.noTrialApprovalNote,
    trialSessionEvidence.noReleaseDeploymentReadinessNote,
    trialSessionEvidence.readBackValidationNote,
  ].join("\n");

  const trialPackage = state.controlledInternalTrialPackageProjection;
  const trialPackageLines = [
    `Package status: ${trialPackage.status}`,
    `Package ID: ${trialPackage.packageId ?? "none"}`,
    `Package version: ${trialPackage.packageVersion}`,
    `Package classification: ${trialPackage.packageClassification}`,
    `Production classification: ${trialPackage.productionClassification}`,
    `Distribution classification: ${trialPackage.distributionClassification}`,
    `Trial scope: ${trialPackage.trialScopeSummary}`,
    `Named operator/participant metadata: ${trialPackage.namedOperatorParticipantSummary.join(", ") || "not provided"}`,
    `Stop-condition summary: ${trialPackage.stopConditionSummary.join(", ")}`,
    `Included local beta evidence/artifact summary: ${trialPackage.includedEvidenceSummary.join(", ") || "not projected"}`,
    `Absence marker summary: ${trialPackage.absenceMarkerSummary.join(", ")}`,
    `Validation status: ${trialPackage.validationStatus}`,
    `Validation errors: ${trialPackage.validationErrors.join(", ") || "none"}`,
    `Read-back validation status: ${trialPackage.readBackValidationStatus ?? "not_read"}`,
    `Boundary status: ${trialPackage.boundaryStatus.join(", ")}`,
    trialPackage.localOnlyNonPublicNote,
    trialPackage.releaseBoundaryNote,
    trialPackage.deploymentReadinessBoundaryNote,
    trialPackage.publicProductionBoundaryNote,
    trialPackage.stopConditionNote,
    "This package does not approve controlled human use.",
  ].join("\n");



  const verification = state.trialReplayRestoreVerification;
  const verificationLines = [
    `Verification status: ${verification.status}`,
    `Verification ID: ${verification.verificationId ?? "none"}`,
    `Package read-back status: ${verification.comparisonSummary.packageReadBackStatus}`,
    `Evidence read-back status: ${verification.comparisonSummary.evidenceReadBackStatus}`,
    `Package/evidence linkage status: ${verification.comparisonSummary.packageEvidenceLinkageStatus}`,
    `Workflow linkage status: ${verification.comparisonSummary.workflowLinkageStatus}`,
    `Replay/status comparison: ${verification.comparisonSummary.replayStatusComparison}`,
    `Restore/history comparison: ${verification.comparisonSummary.restoreHistoryComparison}`,
    `Stop-condition comparison: ${verification.comparisonSummary.stopConditionComparison}`,
    `Escalation/failure comparison: ${verification.comparisonSummary.escalationFailureComparison}`,
    `Verification mismatch drilldown: ${verification.mismatches.join(", ") || "none"}`,
    `Missing verification inputs: ${verification.missingInputs.join(", ") || "none"}`,
    `Matched verification fields: ${verification.matchedFields.join(", ") || "none"}`,
    `Boundary markers: ${verification.boundaryStatus.join(", ")}`,
    verification.localOnlyNonAuthorityNote,
    verification.comparisonScopeNote,
    verification.noReplayRepairNote,
    verification.noRecoveryPromotionNote,
    verification.noApprovalNote,
    verification.noActionExecutionNote,
  ].join("\n");

  const trialObservability = state.trialObservability;
  const trialErrorReport = state.trialErrorReport;
  const observabilityLines = [
    `Observability status: ${trialObservability.status}`,
    `Trial run status: ${trialObservability.trialRunStatus}`,
    `Current trial step: ${trialObservability.currentTrialStep}`,
    `Current blocker: ${trialObservability.currentBlocker}`,
    `Stop-condition observation status: ${trialObservability.stopConditionObservation.status}`,
    `Manual operator step status: ${trialObservability.manualOperatorStepStatus}`,
    `Package status: ${trialObservability.packageStatus}`,
    `Evidence status: ${trialObservability.evidenceStatus}`,
    `Package read-back status: ${trialObservability.packageReadBackStatus}`,
    `Evidence read-back status: ${trialObservability.evidenceReadBackStatus}`,
    `Replay/restore verification status: ${trialObservability.replayRestoreVerificationStatus}`,
    `Mismatch summary: ${trialObservability.mismatchSummary.mismatches.join(", ") || "none"}`,
    `Replay/status comparison summary: ${trialObservability.replayStatusComparisonSummary}`,
    `Restore/history comparison summary: ${trialObservability.restoreHistoryComparisonSummary}`,
    `Package/evidence read-back failure summary: ${trialObservability.packageEvidenceReadBackFailureSummary.join(", ") || "none"}`,
    `Runbook/failure drill category summary: ${trialObservability.runbookFailureDrillCategorySummary.join(", ") || "none"}`,
    `Evidence linkage: ${trialObservability.evidenceLinkageSummary.join(" | ") || "none"}`,
    `Boundary markers: ${trialObservability.boundaryStatuses.join(", ")}`,
    trialObservability.localOnlyNonPublicNote,
    trialObservability.noProductionMonitoringNote,
    trialObservability.noRemoteTelemetryNote,
    trialObservability.noBackgroundServiceNote,
    trialObservability.noRemediationEscalationStopAutomationNote,
    trialObservability.noApprovalNote,
  ].join("\n");
  const errorReportLines = [
    `Error report status: ${trialErrorReport.status}`,
    `Failure category summary: ${trialErrorReport.categorySummary.join(", ") || "none"}`,
    `Highest error severity: ${trialErrorReport.highestSeverity}`,
    `Trial error drilldown: ${trialErrorReport.details.map((detail) => `${detail.category}/${detail.severity}/${detail.source}/${detail.evidenceLinkage.linkage}`).join(" | ") || "none"}`,
    `Evidence linkage: ${trialErrorReport.evidenceLinkageSummary.join(" | ") || "none"}`,
    trialErrorReport.localDescriptiveOnlyNote,
  ].join("\n");
  const blockedStateLines = [
    `Blocked-state summary: ${trialObservability.blockedStateSummary.status}`,
    `Blocked-state current blocker: ${trialObservability.blockedStateSummary.currentBlocker}`,
    `Blocked-state rejection reasons: ${trialObservability.blockedStateSummary.rejectionReasons.join(", ") || "none"}`,
  ].join("\n");

  const trialExecution = state.controlledInternalTrialExecution;
  const displayedTrialRun = trialExecution.activeRun ?? trialExecution.lastRejectedRun;
  const trialExecutionLines = [
    `Harness status: ${trialExecution.status}`,
    `Trial run status: ${displayedTrialRun?.status ?? trialExecution.status}`,
    `Trial run ID: ${displayedTrialRun?.runId ?? "none"}`,
    `Current trial step: ${displayedTrialRun?.currentStep ?? "none"}`,
    `Next trial step: ${displayedTrialRun?.nextStep ?? "none"}`,
    `Ordered trial run steps: ${displayedTrialRun?.steps.map((step) => `${step.step}=${step.status}`).join(", ") ?? "none"}`,
    `Current blocker: ${displayedTrialRun?.currentBlocker ?? trialExecution.currentBlocker ?? "none"}`,
    `Rejection reasons: ${(displayedTrialRun?.rejectionReasons ?? trialExecution.rejectionReasons).join(", ") || "none"}`,
    `Precondition status: ${trialExecution.preconditionStatus.join(", ") || "not evaluated"}`,
    `Start control: ${trialExecution.status === "ready_for_bounded_local_trial_run" ? "enabled" : "disabled"}`,
    `Step control: ${displayedTrialRun && displayedTrialRun.status === "trial_run_in_progress" && !displayedTrialRun.stopConditionObservation.observed ? "enabled" : "disabled"}`,
    `Trial stop-condition observation: ${displayedTrialRun?.stopConditionObservation.status ?? "not_started"}`,
    `Stop-condition observed: ${displayedTrialRun?.stopConditionObservation.observed ?? false}`,
    `Stop-condition markers: ${displayedTrialRun?.stopConditionObservation.markers.join(", ") || "none"}`,
    `Stop-condition enforcement automated: ${displayedTrialRun?.stopConditionObservation.enforcementAutomated ?? false}`,
    `Manual operator step status: ${displayedTrialRun?.manualOperatorStepStatus ?? "not_started"}`,
    `Trial evidence linkage: ${trialExecution.evidenceLinkage.trialPackage} | ${trialExecution.evidenceLinkage.runbook} | ${trialExecution.evidenceLinkage.failureDrill} | ${trialExecution.evidenceLinkage.trialSessionEvidence} | ${trialExecution.evidenceLinkage.replayRestoreVerification} | ${trialExecution.evidenceLinkage.localWorkflow}`,
    `Boundary markers: ${trialExecution.boundaryStatuses.join(", ")}`,
    trialExecution.localOnlyNonPublicNote,
    trialExecution.noControlledHumanUseNote,
    trialExecution.noReadinessReleaseDeploymentPublicProductionNote,
    trialExecution.stopConditionNote,
    trialExecution.escalationNote,
    trialExecution.noActionAuthorizationNote,
  ].join("\n");

  const runbook = state.trialOperatorRunbook;
  const runbookLines = [
    `Runbook status: ${runbook.status}`,
    `Current runbook step: ${runbook.currentStep ?? "none"}`,
    `Current blocker: ${runbook.currentBlocker.step ?? "none"}`,
    `Current blocker guidance: ${runbook.currentBlocker.guidance}`,
    `Trial package status: ${runbook.trialPackageStatus}`,
    `Trial package ID: ${runbook.trialPackageId ?? "none"}`,
    `Trial package validation/read-back: ${runbook.trialPackageValidationStatus} / ${runbook.trialPackageReadBackStatus ?? "not_read"}`,
    `Trial scope status: ${runbook.trialScopeStatus}`,
    `Named operator/participant status: ${runbook.namedOperatorStatus} / ${runbook.namedParticipantStatus}`,
    `Stop-condition summary: ${runbook.stopConditionSummary.join(", ") || "none"}`,
    `Local candidate materialization status: ${runbook.localCandidateMaterializationStatus}`,
    `Provider output pipeline status: ${runbook.providerOutputPipelineStatus}`,
    `Replay/status summary: ${runbook.replayStatusSummary}`,
    `Local evidence export summary: ${runbook.localEvidenceExportSummary}`,
    `Restore/history status: ${runbook.restoreHistoryStatus}`,
    `Ordered runbook steps: ${runbook.steps.map((step) => `${step.step}=${step.status}`).join(", ")}`,
    `Boundary markers: ${runbook.boundaryStatuses.join(", ")}`,
    runbook.localOnlyNonPublicNote,
    runbook.noTrialExecutionNote,
    runbook.noAuthorityNote,
  ].join("\n");

  const failureDrill = state.trialFailureDrill;
  const failureDrillLines = [
    `Failure drill status: ${failureDrill.status}`,
    `Failure severity: ${failureDrill.highestSeverity}`,
    `Failure category list: ${failureDrill.categories.map((entry) => `${entry.category}=${entry.severity}`).join(", ") || "none"}`,
    `Manual action guidance: ${failureDrill.manualActionGuidance.join(" | ") || "none"}`,
    `Rejection summary: ${failureDrill.rejectionSummary.join(" | ") || "none"}`,
    `Stop-condition drill: ${failureDrill.stopConditionDrills.map((entry) => `${entry.marker}=${entry.status}`).join(", ") || "none"}`,
    `Escalation guidance: ${failureDrill.escalationGuidance.map((entry) => `${entry.role}:${entry.categories.join("/")}`).join(", ") || "none"}`,
    failureDrill.noAutomationNote,
    "Escalation guidance is descriptive only and does not activate authority.",
  ].join("\n");

  const workflow = state.completeLocalOperatorWorkflow;
  const workflowLines = [
    `Workflow status: ${workflow.status}`,
    `Workflow classification: ${workflow.classification}`,
    `Current step: ${workflow.currentStep ?? "none"}`,
    `Next required step: ${workflow.nextRequiredStep ?? "none"}`,
    `Current blocking step: ${workflow.currentBlockingStep ?? "none"}`,
    `Current error: ${workflow.currentError ?? "none"}`,
    `Workflow steps: ${workflow.steps.map((step) => `${step.step}=${step.status}${step.error ? `/${step.error}` : ""}`).join(", ")}`,
    `Rejection reasons: ${workflow.rejectionReasons.join(", ") || "none"}`,
    `Provider output pipeline status: ${workflow.evidenceSummary.providerOutputPipelineStatus}`,
    `Local candidate materialization status: ${workflow.evidenceSummary.localCandidateMaterializationStatus}`,
    `Replay/status summary: ${workflow.evidenceSummary.replayStatus}`,
    `Local evidence export summary: ${workflow.evidenceSummary.localEvidenceExportStatus}`,
    `Session package status: ${workflow.evidenceSummary.sessionPackageStatus}`,
    `Restore/history status: ${workflow.evidenceSummary.restoreStatus} / ${workflow.evidenceSummary.sessionHistoryStatus}`,
    `Boundary markers: ${workflow.boundaryStatuses.join(", ")}`,
    workflow.localOnlyNote,
    "Workflow completion does not approve readiness, release, deployment, public use, or production use.",
    "Provider output remains untrusted unless a later bounded phase explicitly changes that.",
    "Workflow status does not authorize actions.",
    "Replay is not repaired and recovery is not promoted.",
  ].join("\n");

  const providerConfiguration = projectLocalProviderConfiguration(
    state.providerConfiguration,
  );
  const adapterRegistry = projectLocalProviderAdapterRegistry(
    state.localProviderAdapterRegistry,
  );
  const providerExecution = projectLocalProviderExecution(state);
  const providerOutputValidation = projectLocalProviderOutputValidation(state);
  const adapterDryRun = state.localProviderAdapterDryRun;
  const constrainedInvocation = state.constrainedLocalProviderInvocation;
  const constrainedInvocationLines = [
    `Invocation status: ${constrainedInvocation.status}`,
    `Allowlisted provider kind: ${constrainedInvocation.capabilitySurface.allowlistedProviderKind}`,
    `Adapter kind: ${constrainedInvocation.adapterKind ?? "none"}`,
    `Adapter declaration ID: ${constrainedInvocation.adapterDeclarationId ?? "none"}`,
    `Result ID: ${constrainedInvocation.result?.resultId ?? "none"}`,
    `Output summary: ${constrainedInvocation.result?.outputSummary ?? "none"}`,
    `Output trust status: ${constrainedInvocation.outputTrustStatus}`,
    `Boundary status: ${constrainedInvocation.boundaryStatuses.join(", ")}`,
    `Effect status: ${constrainedInvocation.effectStatuses.join(", ")}`,
    `Capability surface: ${constrainedInvocation.capabilitySurface.summary}`,
    `Rejected reason: ${constrainedInvocation.errorCodes.join(", ") || "none"}`,
    "Constrained local provider invocation only.",
    "Only one allowlisted local provider path is enabled in Phase 156.",
    "No arbitrary command execution is available.",
    "No shell, network, cloud, or secret capability is enabled.",
    "Provider output remains untrusted and descriptive.",
    "Invocation does not create candidate output or materialize candidates.",
    "Invocation does not approve readiness, release, deployment, or public use.",
  ].join("\n");
  const adapterDryRunLines = [
    `Dry-run status: ${adapterDryRun.status}`,
    `Adapter kind: ${adapterDryRun.adapterKind ?? "none"}`,
    `Adapter declaration ID: ${adapterDryRun.adapterDeclarationId ?? "none"}`,
    `Result ID: ${adapterDryRun.result?.resultId ?? "none"}`,
    `Output summary: ${adapterDryRun.result?.outputSummary ?? "none"}`,
    `Output trust status: ${adapterDryRun.outputTrustStatus}`,
    `Boundary status: ${adapterDryRun.boundaryStatuses.join(", ")}`,
    `Effect status: ${adapterDryRun.effectStatuses.join(", ")}`,
    `Capability surface: ${adapterDryRun.capabilitySurface.summary}`,
    `Rejected reason: ${adapterDryRun.errorCodes.join(", ") || "none"}`,
    "Controlled adapter dry run only.",
    "Only deterministic_fake_adapter can execute in Phase 154.",
    "No real model execution occurs in Phase 154.",
    "Dry-run output remains untrusted and descriptive.",
    "Dry run does not create candidate output or materialize candidates.",
    "Dry run does not approve readiness, release, deployment, or public use.",
  ].join("\n");
  const adapterDeclarations =
    adapterRegistry.declarations.length === 0
      ? "Adapter declarations: none"
      : adapterRegistry.declarations
          .map((declaration) =>
            [
              `Adapter declaration: ${declaration.declarationId}`,
              `Adapter kind: ${declaration.adapterKind}`,
              `Declaration/configuration status: ${declaration.status}`,
              `Execution status: ${declaration.contract.executionStatus}`,
              `Trust status: ${declaration.contract.trustStatus}`,
              `Boundary status: ${declaration.contract.boundaryStatuses.join(", ")}`,
            ].join("\n"),
          )
          .join("\n");
  const adapterRegistryLines = [
    `Registry status: ${adapterRegistry.registryStatus}`,
    `Supported adapter declarations: ${adapterRegistry.supportedAdapterKinds.join(", ")}`,
    `Rejected adapter declarations: ${adapterRegistry.rejectedAdapterKinds.join(", ")}`,
    `Validation status: ${adapterRegistry.lastValidation.status}`,
    `Validation reason: ${adapterRegistry.lastValidation.reason}`,
    `Validation error/reason code: ${adapterRegistry.lastValidation.errorCodes.join(", ") || "none"}`,
    `Capability surface: ${adapterRegistry.capabilitySurface.summary}`,
    `Execution status: ${adapterRegistry.executionStatus}`,
    `Trust status: ${adapterRegistry.trustStatus}`,
    `Boundary status: ${adapterRegistry.boundaryStatuses.join(", ")}`,
    adapterDeclarations,
    adapterRegistry.note,
  ].join("\n");

  const providerConfigurationLines = [
    `Configured provider kind: ${providerConfiguration.configuredProviderKind}`,
    `Provider configuration status: ${providerConfiguration.status}`,
    `Provider validation status: ${providerConfiguration.validationStatus}`,
    `Provider validation reason: ${providerConfiguration.validationReason}`,
    `Provider validation error code: ${providerConfiguration.validationErrorCodes.join(", ") || "none"}`,
    `Execution status: ${providerConfiguration.executionStatus}`,
    `Capability surface: ${providerConfiguration.capabilitySurface.summary}`,
    providerConfiguration.note,
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
    providerExecution.note,
  ].join("\n");

  const pipeline = state.localProviderOutputPipeline;
  const providerOutputPipelineLines = [
    `Pipeline source kind: ${pipeline.sourceKind ?? "none"}`,
    `Source invocation result ID: ${pipeline.sourceInvocationResultId ?? "none"}`,
    `Provider execution result ID: ${pipeline.providerExecutionResultId ?? "none"}`,
    `Pipeline status: ${pipeline.status}`,
    `Current stage: ${pipeline.currentStage ?? "none"}`,
    `Next required stage: ${pipeline.nextRequiredStage ?? "none"}`,
    `Stage list: ${pipeline.stages.map((stage) => `${stage.stage}:${stage.status}${stage.reason ? `:${stage.reason}` : ""}`).join(", ")}`,
    `Provider output validation status: ${pipeline.providerOutputValidationStatus}`,
    `Provider output review status: ${pipeline.providerOutputReviewStatus}`,
    `Staged proposal status: ${pipeline.stagedProposalStatus}`,
    `Staged proposal validation status: ${pipeline.stagedProposalValidationStatus}`,
    `Candidate review status: ${pipeline.candidateReviewStatus}`,
    `Operator decision status: ${pipeline.operatorDecisionStatus}`,
    `Blocked/rejected reasons: ${pipeline.errors.join(", ") || "none"}`,
    `Boundary markers: ${pipeline.boundaryStatuses.join(", ")}`,
    "Invocation output remains untrusted and descriptive.",
    "Pipeline integration does not create candidate output.",
    "Validation, review, staging, staged validation, candidate review, and operator decision boundaries cannot be skipped.",
    "Candidate materialization remains a later bounded step.",
    "Provider trust, readiness, release, deployment, and public-use approval are not granted.",
    pipeline.note,
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
    providerOutputValidation.note,
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
    stagedProjection.note,
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
    stagedValidation.note,
  ].join("\n");

  const candidate = state.run.candidate
    ? [
        `Candidate output: ${state.run.candidate.title}`,
        `Candidate body: ${state.run.candidate.body}`,
        `Provider execution enabled: ${state.run.candidate.providerExecutionEnabled}`,
      ].join("\n")
    : "Candidate output: waiting for deterministic stub run";

  const validation = state.run.validation
    ? `Validation/policy result: ${state.run.validation.policyStatus} / ${state.run.validation.validationStatus}`
    : "Validation/policy result: waiting for deterministic stub run";

  const operatorDecision = state.operatorCandidateDecision;
  const decisionRecord = operatorDecision.record;
  const decisionControls =
    stagedValidation.status === "staged_proposal_shape_valid"
      ? "Approve validated staged proposal | Reject validated staged proposal"
      : "Approve/reject controls hidden until staged proposal validation is staged_proposal_shape_valid";
  const operatorDecisionLines = [
    `Decision status: ${operatorDecision.status}`,
    `Decision kind: ${decisionRecord?.decisionKind ?? "none"}`,
    `Decision ID: ${decisionRecord?.decisionId ?? "none"}`,
    `Decision scope: ${decisionRecord?.decisionScope ?? "decision_scope_validated_staged_proposal_only"}`,
    `Staged proposal ID: ${decisionRecord?.stagedProposalId ?? stagedValidation.proposalId ?? "none"}`,
    `Provider execution result ID: ${decisionRecord?.providerExecutionResultId ?? stagedValidation.sourceExecutionResultId ?? "none"}`,
    `Staged proposal validation status: ${decisionRecord?.stagedProposalValidationStatus ?? stagedValidation.status}`,
    `Candidate materialization status: ${decisionRecord?.materializationStatus ?? "candidate_materialization_not_performed"}`,
    `Provider-output trust effect: ${decisionRecord?.trustStatus ?? "provider_output_remains_untrusted"}`,
    `Readiness effect: ${decisionRecord?.readinessStatus ?? "no_readiness_effect"}`,
    `Release effect: ${decisionRecord?.releaseStatus ?? "no_release_effect"}`,
    `Deployment effect: ${decisionRecord?.deploymentStatus ?? "no_deployment_effect"}`,
    `Public-use effect: ${decisionRecord?.publicUseStatus ?? "no_public_use_effect"}`,
    `Action effect: ${decisionRecord?.actionStatus ?? "no_action_effect"}`,
    `Persistence effect: ${decisionRecord?.persistenceStatus ?? "no_persistence_effect"}`,
    `Replay effect: ${decisionRecord?.replayRepairStatus ?? "no_replay_repair_effect"}`,
    `Recovery effect: ${decisionRecord?.recoveryPromotionStatus ?? "no_recovery_promotion_effect"}`,
    decisionControls,
    "This decision applies only to the validated staged proposal.",
    "No candidate output is created in Phase 149.",
    "Provider output remains untrusted and not approved.",
    "Candidate materialization remains a later bounded step.",
    "This decision does not approve readiness, release, deployment, or public use.",
    operatorDecision.note,
  ].join("\n");

  const localCandidate = state.localCandidateOutput;
  const localCandidateLinkage = localCandidate.sourceLinkage;
  const materializationControl =
    state.localProviderOutputPipeline.status === "valid" &&
    state.providerOutputValidation.status === "reviewable_untrusted" &&
    state.providerOutputValidation.reviewabilityStatus === "reviewable_untrusted" &&
    state.stagedCandidateConversionProposal.status === "staged_proposal_created" &&
    state.stagedCandidateConversionValidation.status === "staged_proposal_shape_valid" &&
    state.localProviderOutputPipeline.candidateReviewStatus === "display_only" &&
    state.operatorCandidateDecision.status === "approved_validated_staged_proposal"
      ? "Materialize local candidate output"
      : "Materialization control disabled until provider validation, review, staged proposal, staged validation, candidate review, and approved operator decision preconditions pass";
  const localCandidateLines = [
    `Materialization status: ${localCandidate.status}`,
    `Candidate ID: ${localCandidate.candidateId ?? "none"}`,
    `Candidate output summary: ${localCandidate.contentSummary ?? "none"}`,
    `Local candidate classification: ${localCandidate.outputClassification}`,
    `Production classification: ${localCandidate.productionClassification}`,
    `Source invocation result ID: ${localCandidateLinkage?.sourceInvocationResultId ?? pipeline.sourceInvocationResultId ?? "none"}`,
    `Source provider execution result ID: ${localCandidateLinkage?.providerExecutionResultId ?? pipeline.providerExecutionResultId ?? "none"}`,
    `Source staged proposal ID: ${localCandidateLinkage?.stagedProposalId ?? stagedValidation.proposalId ?? "none"}`,
    `Source operator decision ID: ${localCandidateLinkage?.operatorDecisionId ?? decisionRecord?.decisionId ?? "none"}`,
    `Provider output trust carry-forward: ${localCandidate.providerOutputTrustCarryForward}`,
    `Materialization boundary markers: ${localCandidate.boundaryStatuses.join(", ")}`,
    `No-effect markers: ${localCandidate.effectStatuses.join(", ")}`,
    `Rejection reason: ${localCandidate.error ?? "none"}`,
    materializationControl,
    "Local candidate output only.",
    "This candidate output is non-production.",
    "Provider output remains untrusted.",
    "Candidate materialization does not approve readiness, release, deployment, or public use.",
    "Candidate materialization does not authorize actions.",
    "Production approval remains unavailable.",
    localCandidate.note,
  ].join("\n");

  const handoff = state.phase150CodeProductionHandoff;
  const handoffLines = [
    `Handoff ID: ${handoff.handoffId}`,
    `Handoff status: ${handoff.status}`,
    "implemented capability evidence",
    handoff.implementedCapabilityEvidence.join("\n"),
    "remaining production-grade gaps",
    handoff.remainingProductionGradeGaps.join("\n"),
    handoff.remapRecommendations.join("\n"),
    "Phase 150 must remap toward larger product capability blocks using executable evidence from this handoff.",
    "Phase 149 does not edit roadmap files.",
  ].join("\n");

  return [
    "AJENTIC local operator shell - non-production",
    `Harness status: ${state.harnessStatus}`,
    `Run status: ${state.run.status}`,
    "Complete local operator workflow",
    workflowLines,
    "Trial operator runbook",
    runbookLines,
    "Trial failure drill",
    failureDrillLines,
    "Stop-condition drill",
    failureDrillLines,
    "Escalation guidance",
    failureDrillLines,
    "Trial observability",
    observabilityLines,
    "Trial error reporting",
    errorReportLines,
    "Trial error drilldown",
    errorReportLines,
    "Trial blocked-state summary",
    blockedStateLines,
    "Controlled internal trial execution harness",
    trialExecutionLines,
    "Trial run status",
    trialExecutionLines,
    "Trial stop-condition observation",
    trialExecutionLines,
    "Trial evidence linkage",
    trialExecutionLines,
    "Trial replay and restore verification",
    verificationLines,
    "Replay/restore verification",
    verificationLines,
    "Verification mismatch drilldown",
    verificationLines,
    "Trial session evidence",
    trialSessionEvidenceLines,
    "Trial evidence capture",
    trialSessionEvidenceLines,
    "Evidence read-back validation",
    trialSessionEvidenceLines,
    "Controlled internal trial package",
    trialPackageLines,
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
    "Local provider adapter contract",
    "Adapter registry",
    "Adapter configuration",
    adapterRegistryLines,
    "Controlled adapter dry run",
    adapterDryRunLines,
    "Constrained local provider invocation",
    constrainedInvocationLines,
    "Local provider invocation",
    "Local provider configuration",
    providerConfigurationLines,
    "Sandboxed provider execution",
    "Provider execution result projection",
    "Run deterministic provider",
    providerExecutionLines,
    "Provider output pipeline integration",
    "Provider output pipeline",
    providerOutputPipelineLines,
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
    renderCandidateReviewSurface(state),
    "Operator candidate decision",
    "Validated staged proposal decision",
    operatorDecisionLines,
    "Local candidate materialization",
    "Local candidate output",
    localCandidateLines,
    "Phase 150 code-production handoff",
    handoffLines,
    "Bottom panel: Replay/status projection",
    replayLines,
    "Local session evidence export",
    exportLines,
  ].join("\n");
}
