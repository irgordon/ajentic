import { getUiReadModel } from "../api/readModel.js";
import { DecisionSummary } from "../components/DecisionSummary.js";
import { ProjectionList } from "../components/ProjectionList.js";
import { SectionCard } from "../components/SectionCard.js";

export function ValidationScreen(): string {
  const { validationDecisions, executionDecisions } = getUiReadModel();

  return SectionCard({
    title: "Validation",
    description: "Read-only validation and execution decision projections.",
    children: [
      "This review surface is read-only and does not evaluate validation, override validation, or trigger execution.",
      ProjectionList({
        title: "Validation decision details",
        rows: validationDecisions.map((decision) => DecisionSummary({ decision }))
      }),
      ProjectionList({
        title: "Execution decision details (display-only)",
        rows: executionDecisions.map((decision) => DecisionSummary({ decision }))
      })
    ]
  });
}
