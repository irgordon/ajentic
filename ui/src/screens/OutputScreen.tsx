import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";

export function OutputScreen(): string {
  const { application, cleanOutput } = getUiReadModel();
  const { output } = application;

  return SectionCard({
    title: "Output",
    description: "Read-only output projection surface.",
    children: [
      `Clean output available: ${output.cleanOutputAvailable}`,
      `Raw output trusted: ${output.rawOutputTrusted}`,
      `Authority: ${output.authority}`,
      `Summary: ${output.summary}`,
      "Clean output detail",
      `Projection id: ${cleanOutput.id}`,
      `Clean output available (detail): ${cleanOutput.cleanOutputAvailable}`,
      `Raw output trusted (detail): ${cleanOutput.rawOutputTrusted}`,
      `Clean output trusted (detail): ${cleanOutput.cleanOutputTrusted}`,
      `Detail authority: ${cleanOutput.authority}`,
      `Detail summary: ${cleanOutput.summary}`,
      `Raw output summary: ${cleanOutput.rawOutputSummary}`,
      `Clean output summary: ${cleanOutput.cleanOutputSummary}`,
      "Trust boundary: raw model output remains untrusted.",
      "Read-only boundary: clean output is display-only in this phase."
    ]
  });
}
