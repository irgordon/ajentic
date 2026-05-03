import { SectionCard } from "../components/SectionCard";

export function ValidationScreen(): string {
  return SectionCard({
    title: "Validation",
    description: "Planned validation surface.",
    children: "This screen is a static, read-only placeholder in Phase 21. No authority actions, execution controls, or mutation paths are enabled."
  });
}
