import { SectionCard } from "../components/SectionCard";

export function LedgerScreen(): string {
  return SectionCard({
    title: "Ledger",
    description: "Planned ledger surface.",
    children: "This screen is a static, read-only placeholder in Phase 21. No authority actions, execution controls, or mutation paths are enabled."
  });
}
