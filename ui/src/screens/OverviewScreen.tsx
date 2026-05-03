import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";
import { StatusPill } from "../components/StatusPill";
import { IntentPreviewPanel } from "../components/IntentPreviewPanel";

export function OverviewScreen(): string {
  const { run, output, operatorIntentPreviews } = getUiReadModel();

  return SectionCard({
    title: "Overview",
    description: "Read-only run and state projection preview.",
    children: [
      `Run title: ${run.title}`,
      `Run ID: ${run.runId}`,
      StatusPill({ label: "Run status", status: run.status }),
      `Lifecycle: ${run.currentLifecycle}`,
      `Execution decision: ${run.executionDecision}`,
      `Promotion decision: ${run.promotionDecision}`,
      `Replay readiness: ${run.replayReadiness}`,
      `Clean output available: ${run.cleanOutputAvailable}`,
      `Authority: ${run.authority}`,
      `Run summary: ${run.summary}`,
      `Output trust: rawOutputTrusted=${output.rawOutputTrusted}`,
      "Operator intent previews: controls are previews only in this phase.",
      "No request is submitted.",
      "No state is mutated.",
      "Rust remains the authority.",
      IntentPreviewPanel({ intents: operatorIntentPreviews }),
      "This surface is read-only and fixture-backed for operator review only."
    ]
  });
}
