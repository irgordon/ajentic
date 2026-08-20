import { getUiReadModel } from "../api/readModel.js";
import { SectionCard } from "../components/SectionCard.js";

export function OutputScreen(): string {
  const { application, reviewableCandidate } = getUiReadModel();
  const { output } = application;

  return SectionCard({
    title: "Output",
    description: "Read-only output projection surface.",
    children: [
      `Reviewable candidate available: ${output.reviewableCandidateAvailable}`,
      `Raw output trusted: ${output.rawOutputTrusted}`,
      `Authority: ${output.authority}`,
      `Summary: ${output.summary}`,
      "Reviewable candidate detail",
      `Projection id: ${reviewableCandidate.id}`,
      `Reviewable candidate available (detail): ${reviewableCandidate.reviewableCandidateAvailable}`,
      `Raw output trusted (detail): ${reviewableCandidate.rawOutputTrusted}`,
      `Reviewable candidate verified (detail): ${reviewableCandidate.candidateVerified}`,
      `Detail authority: ${reviewableCandidate.authority}`,
      `Detail summary: ${reviewableCandidate.summary}`,
      `Raw output summary: ${reviewableCandidate.rawOutputSummary}`,
      `Reviewable candidate summary: ${reviewableCandidate.reviewableCandidateSummary}`,
      "Trust boundary: raw model output remains untrusted.",
      "Read-only boundary: reviewable candidate is display-only in this phase."
    ]
  });
}
