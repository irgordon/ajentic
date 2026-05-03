import type { DecisionDetailProjection } from "../api/projections";
import { StatusPill } from "./StatusPill";

export type DecisionSummaryProps = Readonly<{
  decision: DecisionDetailProjection;
}>;

export function DecisionSummary({ decision }: DecisionSummaryProps): string {
  return [
    `${decision.label} (${decision.id})`,
    `Decision: ${decision.decision}`,
    `Reason: ${decision.reason}`,
    StatusPill({ label: "Status", status: decision.status }),
    `Authority: ${decision.authority}`,
    `Evidence: ${decision.evidenceSummary}`,
    `Summary: ${decision.summary}`
  ].join("\n");
}
