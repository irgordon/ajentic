import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";
import { TimelineList } from "../components/TimelineList";

export function LedgerScreen(): string {
  const { ledger, ledgerTimeline } = getUiReadModel();

  const timeline = TimelineList({
    heading: "Ledger timeline entries",
    rows: ledgerTimeline.map((entry) => ({
      title: `${entry.id} (revision ${entry.revision})`,
      lines: [
        `Event type: ${entry.eventType}`,
        `Actor: ${entry.actor}`,
        `Evidence refs: ${entry.evidenceRefs}`,
        `Status: ${entry.status}`,
        `Authority: ${entry.authority}`,
        `Summary: ${entry.summary}`
      ]
    }))
  });

  return SectionCard({
    title: "Ledger",
    description: "Read-only ledger projection surface.",
    children: [
      `Event count: ${ledger.events}`,
      `Last revision: ${ledger.lastRevision ?? "none"}`,
      `Status: ${ledger.status}`,
      `Authority: ${ledger.authority}`,
      `Summary: ${ledger.summary}`,
      "Read-only boundary: this UI does not edit ledger events.",
      timeline
    ]
  });
}
