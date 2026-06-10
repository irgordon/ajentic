import { getUiReadModel } from "../api/readModel.js";
import { DecisionSummary } from "../components/DecisionSummary.js";
import { ProjectionList } from "../components/ProjectionList.js";
import { SectionCard } from "../components/SectionCard.js";

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
