import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";

export function ReplayScreen(): string {
  const { replay } = getUiReadModel();

  return SectionCard({
    title: "Replay",
    description: "Read-only replay projection surface.",
    children: [`Readiness: ${replay.readiness}`, `Integrity: ${replay.integrity}`, `Events replayed: ${replay.eventsReplayed}`, `Status: ${replay.status}`, `Authority: ${replay.authority}`, `Summary: ${replay.summary}`]
  });
}
