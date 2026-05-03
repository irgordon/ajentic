export type NavigationItem = Readonly<{
  id: string;
  label: string;
  description: string;
}>;

export const navigationItems: readonly NavigationItem[] = [
  { id: "overview", label: "Overview", description: "Read-only summary of planned UI surfaces." },
  { id: "state", label: "State", description: "Lifecycle and state projection surface (planned)." },
  { id: "context", label: "Context", description: "Context packet inspection surface (planned)." },
  { id: "memory", label: "Memory", description: "Governed memory inspection surface (planned)." },
  { id: "policy", label: "Policy", description: "Policy decision review surface (planned)." },
  { id: "validation", label: "Validation", description: "Validation result review surface (planned)." },
  { id: "ledger", label: "Ledger", description: "Ledger event timeline surface (planned)." },
  { id: "replay", label: "Replay", description: "Deterministic replay status surface (planned)." },
  { id: "audit", label: "Audit", description: "Audit projection surface (planned)." },
  { id: "output", label: "Output", description: "Clean output projection surface (planned)." }
] as const;
