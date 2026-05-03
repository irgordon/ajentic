import { SectionCard } from "../components/SectionCard";

export function OverviewScreen(): string {
  return SectionCard({
    title: "Overview",
    description: "Read-only browser UI shell baseline for AJENTIC.",
    children: "This phase provides layout scaffolding only. Runtime authority, mutation controls, and provider calls are intentionally not implemented."
  });
}
