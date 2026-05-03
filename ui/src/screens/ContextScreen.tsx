import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";

export function ContextScreen(): string {
  const { context } = getUiReadModel();

  return SectionCard({
    title: "Context",
    description: "Read-only context projection surface.",
    children: [`Packet: ${context.packetId}`, `Slices: ${context.slices}`, `Sources: ${context.sources}`, `Budget: ${context.budgetUsed}/${context.budgetMax}`, `Authority: ${context.authority}`, `Summary: ${context.summary}`]
  });
}
