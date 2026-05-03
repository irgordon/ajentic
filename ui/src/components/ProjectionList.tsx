export type ProjectionListProps = Readonly<{
  title: string;
  rows: readonly string[];
}>;

export function ProjectionList(props: ProjectionListProps): string {
  const listLines = props.rows.map((row, index) => `${index + 1}. ${row}`).join("\n");
  return `${props.title}\n${listLines}`;
}
