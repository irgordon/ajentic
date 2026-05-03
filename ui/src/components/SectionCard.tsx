type Child = string | number | readonly Child[];

export type SectionCardProps = Readonly<{
  title: string;
  description: string;
  children?: Child;
}>;

const flattenChildren = (children: Child | undefined): string => {
  if (children === undefined) {
    return "";
  }
  if (typeof children === "string" || typeof children === "number") {
    return String(children);
  }
  return children.map((item) => flattenChildren(item)).filter((item) => item.length > 0).join("\n");
};

export function SectionCard(props: SectionCardProps): string {
  const details = flattenChildren(props.children);
  const content = details.length > 0 ? `\n${details}` : "";
  return `${props.title}\n${props.description}${content}`;
}
