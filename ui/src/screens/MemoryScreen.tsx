import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";

export function MemoryScreen(): string {
  const { memory } = getUiReadModel();

  return SectionCard({
    title: "Memory",
    description: "Read-only memory projection surface.",
    children: [`Snapshot: ${memory.snapshotId}`, `Active: ${memory.activeEntries}`, `Disabled: ${memory.disabledEntries}`, `Rejected: ${memory.rejectedEntries}`, `Authority: ${memory.authority}`, `Summary: ${memory.summary}`]
  });
}
