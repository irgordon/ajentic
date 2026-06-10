import { getUiReadModel } from "../api/readModel.js";
import { SectionCard } from "../components/SectionCard.js";
import { StatusPill } from "../components/StatusPill.js";

export function StateScreen(): string {
  const { application } = getUiReadModel();
  const { lifecycle, run } = application;

  return SectionCard({
    title: "State",
    description: "Read-only lifecycle projection surface.",
    children: [
      `Lifecycle revision: ${lifecycle.revision}`,
      `Lifecycle value: ${lifecycle.lifecycle}`,
      StatusPill({ label: "Lifecycle status", status: lifecycle.status }),
      `Authority marker: ${lifecycle.authority}`,
      `Summary: ${lifecycle.summary}`,
      `Linked run: ${run.title} (${run.runId})`
    ]
  });
}
