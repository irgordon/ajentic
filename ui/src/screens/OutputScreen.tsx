import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";

export function OutputScreen(): string {
  const { output } = getUiReadModel();

  return SectionCard({
    title: "Output",
    description: "Read-only output projection surface.",
    children: [`Clean output available: ${output.cleanOutputAvailable}`, `Raw output trusted: ${output.rawOutputTrusted}`, `Authority: ${output.authority}`, `Summary: ${output.summary}`]
  });
}
