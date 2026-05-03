import { SectionCard } from "../components/SectionCard";

export function StateScreen(): string {
  return SectionCard({
    title: "State",
    description: "Planned state surface.",
    children: "This screen is a static, read-only placeholder in Phase 21. No authority actions, execution controls, or mutation paths are enabled."
  });
}
