import { SectionCard } from "../components/SectionCard";

export function OutputScreen(): string {
  return SectionCard({
    title: "Output",
    description: "Planned output surface.",
    children: "This screen is a static, read-only placeholder in Phase 21. No authority actions, execution controls, or mutation paths are enabled."
  });
}
