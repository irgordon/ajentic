import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";

export function OverviewScreen(): string {
  const readModel = getUiReadModel();

  return SectionCard({
    title: "Overview",
    description: "Read-only browser UI projection preview.",
    children: [
      `Lifecycle: ${readModel.lifecycle.summary}`,
      `Decisions: ${readModel.decisions.length}`,
      `Ledger: ${readModel.ledger.summary}`,
      `Replay: ${readModel.replay.summary}`,
      `Output: ${readModel.output.summary}`
    ]
  });
}
