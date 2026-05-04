import { getUiReadModel } from "../api/readModel";
import { ProjectionList } from "../components/ProjectionList";
import { SectionCard } from "../components/SectionCard";

export function ContextScreen(): string {
  const { application } = getUiReadModel();
  const { context } = application;

  const sliceRows = context.slicesPreview.map(
    (slice) =>
      `ID: ${slice.id} | View: ${slice.viewType} | Truth: ${slice.truthDimension} | Source: ${slice.sourcePath} | Provenance: ${slice.provenance} | Authority: ${slice.authority} | Summary: ${slice.summary}`
  );

  const sliceList = ProjectionList({
    title: "Context Slice Preview",
    rows: sliceRows
  });

  return SectionCard({
    title: "Context",
    description: "Read-only context projection inspection surface.",
    children: [
      "This surface is read-only and does not rebuild or mutate context.",
      `Packet: ${context.packetId}`,
      `Sources: ${context.sources}`,
      `Slices: ${context.slices}`,
      `Budget: ${context.budgetUsed}/${context.budgetMax}`,
      `Authority: ${context.authority}`,
      `Summary: ${context.summary}`,
      sliceList
    ]
  });
}
