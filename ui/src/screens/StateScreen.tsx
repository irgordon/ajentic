import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";

export function StateScreen(): string {
  const { lifecycle } = getUiReadModel();

  return SectionCard({
    title: "State",
    description: "Read-only lifecycle projection surface.",
    children: [`Lifecycle: ${lifecycle.lifecycle}`, `Revision: ${lifecycle.revision}`, `Status: ${lifecycle.status}`, `Authority: ${lifecycle.authority}`, `Summary: ${lifecycle.summary}`]
  });
}
