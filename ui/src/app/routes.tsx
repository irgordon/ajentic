export type RouteMetadata = Readonly<{
  id: string;
  label: string;
  description: string;
}>;

export const routeMetadata: readonly RouteMetadata[] = [
  { id: "overview", label: "Overview", description: "Entry screen for non-authoritative browser UI shell." },
  { id: "state", label: "State", description: "Read-only lifecycle and run-state projection area." },
  { id: "context", label: "Context", description: "Read-only context packet inspection area." },
  { id: "memory", label: "Memory", description: "Read-only memory and provenance inspection area." },
  { id: "policy", label: "Policy", description: "Read-only policy outcome inspection area." },
  { id: "validation", label: "Validation", description: "Read-only validation evidence inspection area." },
  { id: "ledger", label: "Ledger", description: "Read-only accepted event history area." },
  { id: "replay", label: "Replay", description: "Read-only replay result inspection area." },
  { id: "audit", label: "Audit", description: "Read-only audit projection and timeline area." },
  { id: "output", label: "Output", description: "Read-only clean output review area." }
] as const;
