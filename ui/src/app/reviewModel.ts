import type { LocalOperatorShellState } from "../api/localOperatorShell.js";

export type ReviewTab = "compare" | "result" | "activity" | "checks" | "diagnostics";
export type ReviewPreferences = {
  tab: ReviewTab;
  showDetails: boolean;
  expandedContext: boolean;
  showAllActivity: boolean;
  notes: string;
};
export type RequestFeedback = Readonly<{ status: string; reason: string }>;
export type ReviewView = Readonly<{
  state: LocalOperatorShellState;
  preferences: ReviewPreferences;
  feedback: RequestFeedback;
}>;

export const initialReviewPreferences = (): ReviewPreferences => ({
  tab: "compare", showDetails: false, expandedContext: false,
  showAllActivity: false, notes: "",
});

export const SAMPLE_FILE = {
  name: "sample-note.txt",
  before: ["# Project Notes", "", "This is a sample note for the demo.", "It explains how the draft looks", "This draft is ready to ship.", "", "Next steps:", "- Confirm with the team.", "", "Review checklist:", "- Compare with supplied source.", "- Read the recorded checks.", "- Keep unknowns visible.", "", "# End of sample"],
  after: ["# Project Notes", "", "This is a sample note for the demo.", "It explains how the draft looks", "This draft still needs review.", "", "Next steps:", "- Confirm with the team.", "", "Review checklist:", "- Compare with supplied source.", "- Read the recorded checks.", "- Keep unknowns visible.", "", "# End of sample"],
  changedLine: 5,
};

export function hasReviewDecision(state: LocalOperatorShellState): boolean {
  return state.decisionLedger.records.length > 0;
}

export function reviewDecisionLabel(state: LocalOperatorShellState): string {
  const record = state.decisionLedger.records[state.decisionLedger.records.length - 1];
  if (!record) return "Waiting for your review choice.";
  return record.intentKind === "approve" ? "Approval recorded for this local sample." : "Rejection recorded for this local sample.";
}

export function outputProblem(view: ReviewView): string | null {
  if (view.feedback.status === "rejected") return view.feedback.reason;
  return view.state.localCandidateOutput.error;
}
