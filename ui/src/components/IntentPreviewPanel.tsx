import type { OperatorIntentPreviewProjection } from "../api/projections";
import { ProjectionList } from "./ProjectionList";

export type IntentPreviewPanelProps = Readonly<{
  intents: readonly OperatorIntentPreviewProjection[];
}>;

export function IntentPreviewPanel(props: IntentPreviewPanelProps): string {
  const rows = props.intents.map((intent) => {
    const submission = intent.submissionPreview;

    return [
      `Intent ${intent.id}`,
      `Type: ${intent.intentType}`,
      `Label: ${intent.label}`,
      `Preview summary: ${intent.description}`,
      `Submission ID: ${submission.submissionId}`,
      `Operator ID: ${submission.operatorId}`,
      `Target kind: ${submission.targetKind}`,
      `Target ID: ${submission.targetId}`,
      `Reason: ${submission.reason}`,
      `Request preview enabled: ${submission.requestPreviewEnabled}`,
      `Submission enabled: ${submission.submissionEnabled}`,
      `Authority text: ${submission.authority}`,
      `Status text: ${intent.status}`,
      "No submission occurs in this phase.",
      "Rust ingress is not called by the UI in this phase.",
      "No action executes from this preview."
    ].join(" | ");
  });

  return ProjectionList({
    title: "Operator intent request previews",
    rows
  });
}
