import type { ProjectionStatus } from "../api/projections";

export type StatusPillProps = Readonly<{
  label: string;
  status: ProjectionStatus;
}>;

export function StatusPill({ label, status }: StatusPillProps): string {
  return `${label}: ${status}`;
}
