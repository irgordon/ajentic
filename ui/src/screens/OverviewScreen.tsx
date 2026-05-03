import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";
import { StatusPill } from "../components/StatusPill";

export function OverviewScreen(): string {
  const { run, output } = getUiReadModel();

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
      "This surface is read-only and fixture-backed for operator review only."
    ]
  });
}
