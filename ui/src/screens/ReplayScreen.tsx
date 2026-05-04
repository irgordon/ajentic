import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";

export function ReplayScreen(): string {
  const { application, replayDetail } = getUiReadModel();
  const { replay } = application;

  return SectionCard({
    title: "Replay",
    description: "Read-only replay projection surface.",
    children: [
      `Readiness: ${replay.readiness}`,
      `Integrity: ${replay.integrity}`,
      `Events replayed: ${replay.eventsReplayed}`,
      `Status: ${replay.status}`,
      `Authority: ${replay.authority}`,
      `Summary: ${replay.summary}`,
      "Replay detail",
      `Reconstruction status: ${replayDetail.reconstructionStatus}`,
      `Final lifecycle: ${replayDetail.finalLifecycle}`,
      `Final revision: ${replayDetail.finalRevision}`,
      `Events seen: ${replayDetail.eventsSeen}`,
      `State transitions applied: ${replayDetail.stateTransitionsApplied}`,
      `Detail readiness: ${replayDetail.readiness}`,
      `Detail integrity: ${replayDetail.integrity}`,
      `Detail authority: ${replayDetail.authority}`,
      `Detail summary: ${replayDetail.summary}`,
      "Read-only boundary: this UI does not repair, rerun, or reconstruct replay."
    ]
  });
}
