import type { OperatorIntentPreviewProjection } from "../api/projections";
import { ProjectionList } from "./ProjectionList";

export type IntentPreviewPanelProps = Readonly<{
  intents: readonly OperatorIntentPreviewProjection[];
}>;

export function IntentPreviewPanel(props: IntentPreviewPanelProps): string {
  const rows = props.intents.map((intent) =>
    [
      `ID: ${intent.id}`,
      `intentType: ${intent.intentType}`,
      `label: ${intent.label}`,
      `description: ${intent.description}`,
      `reasonPreview: ${intent.reasonPreview}`,
      `routePreview: ${intent.routePreview}`,
      `authority: ${intent.authority}`,
      `status: ${intent.status}`,
      `disabled: ${intent.disabled}`,
      "Intent is a request. Rust decides."
    ].join(" | ")
  );

  return ProjectionList({
    title: "Operator intent request previews",
    rows
  });
}
