import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";

export function AuditScreen(): string {
  const { audit } = getUiReadModel();

  return SectionCard({
    title: "Audit",
    description: "Read-only audit projection summary surface.",
    children: [`Projection count: ${audit.projections}`, `Latest summary: ${audit.latestSummary}`, `Authority: ${audit.authority}`, `Summary: ${audit.summary}`]
  });
}
