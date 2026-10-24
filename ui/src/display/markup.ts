const ENTITIES: Readonly<Record<string, string>> = {
  "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;",
};

export function escapeHtml(value: unknown): string {
  return String(value).replace(/[&<>"']/g, (character) => ENTITIES[character]);
}

export function icon(name: string): string {
  return `<img class="icon" src="/icons/${escapeHtml(name)}.svg" alt="" aria-hidden="true" width="18" height="18" />`;
}
