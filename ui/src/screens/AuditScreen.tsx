import { SectionCard } from "../components/SectionCard";

export function AuditScreen(): string {
  return SectionCard({
    title: "Audit",
    description: "Planned audit surface.",
    children: "This screen is a static, read-only placeholder in Phase 21. No authority actions, execution controls, or mutation paths are enabled."
  });
}
