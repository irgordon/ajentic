import { SectionCard } from "../components/SectionCard";

export function PolicyScreen(): string {
  return SectionCard({
    title: "Policy",
    description: "Planned policy surface.",
    children: "This screen is a static, read-only placeholder in Phase 21. No authority actions, execution controls, or mutation paths are enabled."
  });
}
