import { getUiReadModel } from "../api/readModel";
import { DecisionSummary } from "../components/DecisionSummary";
import { ProjectionList } from "../components/ProjectionList";
import { SectionCard } from "../components/SectionCard";

export function PolicyScreen(): string {
  const { policyDecisions } = getUiReadModel();
  const rows = policyDecisions.map((decision) => DecisionSummary({ decision }));

  return SectionCard({
    title: "Policy",
    description: "Read-only policy decision projections.",
    children: [
      "This review surface is read-only and does not evaluate policy, bypass policy, or apply authority actions.",
      ProjectionList({ title: "Policy decision details", rows })
    ]
  });
}
