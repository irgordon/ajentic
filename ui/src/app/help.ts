export const localHelpIndexHref = "help/index.html";

export function renderLocalHelpEntryText(): string {
  return [
    "Local help",
    `Open help index: ${localHelpIndexHref}`,
    "Operator help pages are explanatory only.",
    "Local help is local-only and non-production.",
    "The help entry point does not change runtime state, run providers, write packages, use network, publish, deploy, sign, release, or approve actions.",
  ].join("\n");
}

export function renderLocalHelpEntryHtml(): string {
  return `
    <section class="panel" aria-label="Local help">
      <h2>Local help</h2>
      <p>Operator help pages are explanatory only.</p>
      <p>Local help is local-only and non-production.</p>
      <p><a href="${localHelpIndexHref}" target="_blank" rel="noreferrer">Open help index</a></p>
      <p class="muted">The help entry point does not change runtime state, run providers, write packages, use network, publish, deploy, sign, release, or approve actions.</p>
    </section>`;
}
