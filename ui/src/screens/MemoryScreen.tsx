import { getUiReadModel } from "../api/readModel.js";
import { ProjectionList } from "../components/ProjectionList.js";
import { SectionCard } from "../components/SectionCard.js";

export function MemoryScreen(): string {
  const { application } = getUiReadModel();
  const { memory } = application;

  const entryRows = memory.entriesPreview.map(
    (entry) =>
      `ID: ${entry.id} | Type: ${entry.memoryType} | Status: ${entry.status} | Provenance: ${entry.provenance} | Authority: ${entry.authority} | Summary: ${entry.summary}`
  );

  const entryList = ProjectionList({
    title: "Memory Entry Preview",
    rows: entryRows
  });

  return SectionCard({
    title: "Memory",
    description: "Read-only memory projection inspection surface.",
    children: [
      "This surface is read-only and does not edit, disable, delete, or mutate memory.",
      `Snapshot: ${memory.snapshotId}`,
      `Active: ${memory.activeEntries}`,
      `Disabled: ${memory.disabledEntries}`,
      `Rejected: ${memory.rejectedEntries}`,
      `Authority: ${memory.authority}`,
      `Summary: ${memory.summary}`,
      entryList
    ]
  });
}
