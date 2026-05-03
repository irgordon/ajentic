import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";
import { TimelineList } from "../components/TimelineList";

export function AuditScreen(): string {
  const { audit, auditDetails } = getUiReadModel();

  const details = TimelineList({
    heading: "Audit detail projections",
    rows: auditDetails.map((detail) => ({
      title: `${detail.id} (${detail.projectionType})`,
      lines: [
        `Source: ${detail.source}`,
        `Authority: ${detail.authority}`,
        `Summary: ${detail.summary}`,
        `Details: ${detail.details.join(" | ")}`
      ]
    }))
  });

  return SectionCard({
    title: "Audit",
    description: "Read-only audit projection summary surface.",
    children: [
      `Projection count: ${audit.projections}`,
      `Latest summary: ${audit.latestSummary}`,
      `Authority: ${audit.authority}`,
      `Summary: ${audit.summary}`,
      "Read-only boundary: this UI does not export, mutate, or generate audit records.",
      details
    ]
  });
}
