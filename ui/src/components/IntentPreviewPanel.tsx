import type { OperatorIntentPreviewProjection } from "../api/projections";
import { ProjectionList } from "./ProjectionList";

export type IntentPreviewPanelProps = Readonly<{
  intents: readonly OperatorIntentPreviewProjection[];
}>;

export function IntentPreviewPanel(props: IntentPreviewPanelProps): string {
  const rows = props.intents.map((intent) =>
    [
      `Intent ${intent.id}`,
      `Type: ${intent.intentType}`,
      `Label: ${intent.label}`,
      `Preview summary: ${intent.description}`,
      `Reason preview: ${intent.reasonPreview}`,
      `Route preview: ${intent.routePreview}`,
      `Authority text: ${intent.authority}`,
      `Status text: ${intent.status}`,
      `Request-preview state: ${intent.disabled ? "disabled/request-preview-only" : "preview-only"}`,
      "Intent is a request preview only. Rust decides whether any typed request is accepted."
    ].join(" | ")
  );

  return ProjectionList({
    title: "Operator intent request previews",
    rows
  });
}
