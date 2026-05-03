import { SectionCard } from "../components/SectionCard";

export function ReplayScreen(): string {
  return SectionCard({
    title: "Replay",
    description: "Planned replay surface.",
    children: "This screen is a static, read-only placeholder in Phase 21. No authority actions, execution controls, or mutation paths are enabled."
  });
}
