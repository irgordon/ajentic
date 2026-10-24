export function replaceHtml(id: string, html: string): void {
  const element = document.getElementById(id);
  if (!element) return;
  const details = captureDetails(element);
  element.innerHTML = html;
  restoreDetails(element, details);
}

function captureDetails(element: HTMLElement): Map<string, boolean> {
  return new Map(Array.from(element.querySelectorAll<HTMLDetailsElement>("details[data-open-key]")).map((detail) => [detail.dataset.openKey!, detail.open]));
}

function restoreDetails(element: HTMLElement, states: Map<string, boolean>): void {
  for (const detail of element.querySelectorAll<HTMLDetailsElement>("details[data-open-key]")) {
    if (states.has(detail.dataset.openKey!)) detail.open = states.get(detail.dataset.openKey!)!;
  }
}

export function listen(name: string, handler: EventListener): void {
  document.addEventListener(name, handler);
}

export function eventAction(event: Event): string | null {
  if (!(event.target instanceof Element)) return null;
  const button = event.target.closest<HTMLElement>("[data-action], button[id]");
  return button?.dataset.action ?? button?.id ?? null;
}

export function controlValue(event: Event, id: string): string | null {
  if (!(event.target instanceof HTMLTextAreaElement) || event.target.id !== id) return null;
  return event.target.value;
}

export function controlChecked(event: Event, id: string): boolean | null {
  if (!(event.target instanceof HTMLInputElement) || event.target.id !== id) return null;
  return event.target.checked;
}

export function focusControl(id: string): void {
  document.getElementById(id)?.focus();
}

export function closeMenus(event?: Event): void {
  if (event?.target instanceof Element && event.target.closest(".view-menu")) return;
  document.querySelectorAll<HTMLDetailsElement>(".view-menu[open]").forEach((menu) => {
    menu.open = false;
    if (!event) menu.querySelector("summary")?.focus();
  });
}

export function keyboardTab(event: Event): string | null {
  if (!(event instanceof KeyboardEvent) || !(event.target instanceof HTMLElement)) return null;
  if (event.target.getAttribute("role") !== "tab") return null;
  const tabs = Array.from(document.querySelectorAll<HTMLElement>('[role="tab"]'));
  const index = tabs.indexOf(event.target);
  const next = tabIndexForKey(event.key, index, tabs.length);
  if (next === null) return null;
  event.preventDefault();
  return tabs[next].id;
}

export function isEscape(event: Event): boolean {
  return event instanceof KeyboardEvent && event.key === "Escape";
}

function tabIndexForKey(key: string, index: number, count: number): number | null {
  if (key === "ArrowRight") return (index + 1) % count;
  if (key === "ArrowLeft") return (index + count - 1) % count;
  if (key === "Home") return 0;
  if (key === "End") return count - 1;
  return null;
}
