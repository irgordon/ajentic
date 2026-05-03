import { SectionCard } from "../components/SectionCard";

export function ContextScreen(): string {
  return SectionCard({
    title: "Context",
    description: "Planned context surface.",
    children: "This screen is a static, read-only placeholder in Phase 21. No authority actions, execution controls, or mutation paths are enabled."
  });
}
