export type TimelineListRow = Readonly<{
  title: string;
  lines: readonly string[];
}>;

export type TimelineListProps = Readonly<{
  heading: string;
  rows: readonly TimelineListRow[];
}>;

export function TimelineList(props: TimelineListProps): string {
  const content = props.rows
    .map((row, index) => {
      const rowLines = row.lines.map((line) => `   - ${line}`).join("\n");
      return `${index + 1}. ${row.title}\n${rowLines}`;
    })
    .join("\n");

  return `${props.heading}\n${content}`;
}
