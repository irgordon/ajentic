import { getUiReadModel } from "../api/readModel";
import { SectionCard } from "../components/SectionCard";

export function LedgerScreen(): string {
  const { ledger } = getUiReadModel();

  return SectionCard({
    title: "Ledger",
    description: "Read-only ledger projection surface.",
    children: [`Events: ${ledger.events}`, `Last revision: ${ledger.lastRevision ?? "none"}`, `Status: ${ledger.status}`, `Authority: ${ledger.authority}`, `Summary: ${ledger.summary}`]
  });
}
