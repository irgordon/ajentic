export type DisplayTone =
  | "ready"
  | "attention"
  | "missing"
  | "rejected"
  | "neutral";

export type DisplayStatus = Readonly<{
  label: string;
  description: string;
  tone: DisplayTone;
  raw: string;
}>;

const statusCopy: Record<string, Omit<DisplayStatus, "raw">> = {
  available: {
    label: "Available",
    description: "Evidence exists for this local check.",
    tone: "ready",
  },
  blocked: {
    label: "Needs attention",
    description: "Action is needed before this local review can continue.",
    tone: "attention",
  },
  deterministic_stub_completed: {
    label: "Completed locally",
    description: "The predictable local test run completed.",
    tone: "ready",
  },
  missing: {
    label: "Not started",
    description: "This check has not been supplied or started yet.",
    tone: "missing",
  },
  not_assembled: {
    label: "Not started",
    description: "Internal evidence has not been assembled yet.",
    tone: "missing",
  },
  present: {
    label: "Available",
    description: "Evidence exists for this local check.",
    tone: "ready",
  },
  preparation_blocked: {
    label: "Preparation blocked",
    description: "Action required. Review the blocked checks below to continue.",
    tone: "attention",
  },
  preparation_rejected: {
    label: "Rejected",
    description: "Evidence was explicitly rejected.",
    tone: "rejected",
  },
  preparation_validated: {
    label: "Completed locally",
    description: "Local preparation checks have completed.",
    tone: "ready",
  },
  rejected: {
    label: "Rejected",
    description: "Evidence was explicitly rejected.",
    tone: "rejected",
  },
  simulation_ready: {
    label: "Ready (simulated)",
    description:
      "The system is running locally using predictable pre-made test data instead of a live database.",
    tone: "ready",
  },
  stub_completed: {
    label: "Completed locally",
    description: "The predictable local test run completed.",
    tone: "ready",
  },
};

export function displayStatus(raw: string): DisplayStatus {
  const copy = statusCopy[raw];
  if (!copy) {
    return {
      label: "Unknown status",
      description: "This raw status has no display mapping yet.",
      tone: "neutral",
      raw,
    };
  }

  return {
    ...copy,
    raw,
  };
}

export function plainCategoryLabel(raw: string): string {
  const words = raw.replace(/_/g, " ").split(" ");
  return words.map(capitalizeWord).join(" ");
}

function capitalizeWord(word: string): string {
  if (word.length === 0) return word;
  return `${word[0].toUpperCase()}${word.slice(1)}`;
}
