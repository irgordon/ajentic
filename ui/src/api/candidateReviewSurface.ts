import type { LocalOperatorShellState, StagedCandidateConversionValidationProjection } from "./localOperatorShell";

type CandidateReviewSurfaceStatus =
  | "no_validated_staged_proposal"
  | "validated_staged_proposal_review"
  | "validated_staged_proposal_review_unavailable";

function candidateReviewSurfaceStatus(validation: StagedCandidateConversionValidationProjection): CandidateReviewSurfaceStatus {
  if (validation.status === "staged_proposal_shape_valid") return "validated_staged_proposal_review";
  if (validation.status === "not_validated") return "no_validated_staged_proposal";
  return "validated_staged_proposal_review_unavailable";
}

function candidateReviewSurfaceMessage(status: CandidateReviewSurfaceStatus, validation: StagedCandidateConversionValidationProjection): string {
  if (status === "validated_staged_proposal_review") {
    return "Validated staged proposal is inspectable by a human operator; review remains display-only.";
  }

  if (validation.status === "rejected_staged_proposal") {
    return "Candidate review surface unavailable because staged proposal validation was rejected; closed rejection reasons are shown below.";
  }

  if (validation.status === "invalid_validation_input") {
    return "Candidate review surface unavailable because staged proposal validation input was invalid or missing.";
  }

  return "No validated staged proposal exists; candidate review surface is visible but unavailable until staged proposal validation succeeds.";
}

export function renderCandidateReviewSurface(state: LocalOperatorShellState): string {
  const validation = state.stagedCandidateConversionValidation;
  const status = candidateReviewSurfaceStatus(validation);
  const proposalExists = state.stagedCandidateConversionProposal.proposal !== null;
  const reasons = validation.reasons.length === 0 ? "none" : validation.reasons.join(", ");

  return [
    "Candidate review surface",
    "Candidate review surface is display-only in Phase 148.",
    `Review surface status: ${status}`,
    `Review state: ${candidateReviewSurfaceMessage(status, validation)}`,
    "Review status",
    `Staged proposal validation status: ${validation.status}`,
    `Validation reasons: ${reasons}`,
    `Proposal present: ${proposalExists ? "yes" : "no"}`,
    "Source linkage",
    `Proposal ID: ${validation.proposalId ?? "none"}`,
    `Source provider kind: ${validation.sourceProviderKind}`,
    `Source execution result ID: ${validation.sourceExecutionResultId ?? "none"}`,
    `Source validation status: ${validation.sourceValidationStatus}`,
    `Source reviewability status: ${validation.sourceReviewabilityStatus}`,
    `Source candidate-boundary status: ${validation.sourceCandidateBoundaryStatus}`,
    `Deterministic linkage status: ${validation.deterministicLinkageStatus}`,
    "Validation summary",
    "Validation checks staged proposal shape and source linkage only.",
    "Materialization boundary",
    "Validated staged proposal is not candidate output.",
    "Candidate materialization was not performed in Phase 148.",
    "Materialization status: candidate_materialization_not_performed, materialization_not_available_in_phase_148",
    "Operator-decision boundary",
    "Operator decision is not available in Phase 148.",
    "Future operator-decision boundary: future_operator_decision_required",
    "Approval controls are reserved for a later bounded phase.",
    "Trust and approval boundary",
    "Provider output remains untrusted and not approved.",
    `Trust status: ${validation.trustStatuses.join(", ")}`,
    "Approval status: not_approved",
    "No-effect summary",
    `No-effect summary: ${validation.noEffectSummary.join(", ")}`,
    "Display-only review does not mutate decision ledger, replay state, export state, provider configuration, provider execution state, staged proposal validation, or candidate output.",
    "Phase 148 boundary",
    "Review surface interactions are display-only and do not mutate authoritative state.",
    "This surface does not expose approve/reject controls for staged proposals.",
    "This surface does not expose candidate creation controls.",
    "This surface does not expose candidate materialization controls."
  ].join("\n");
}
