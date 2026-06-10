export const localHelpIndexHref = "help/index.html";

export function renderLocalHelpEntryText(): string {
  return [
    "Help and reference",
    "Learn what each check means and how to read this local review page.",
    `Open help: ${localHelpIndexHref}`,
    "This page is local and read-only. It does not publish releases, change credentials, or contact external services.",
    "The help entry point does not change runtime state, run providers, write packages, use network, publish, deploy, sign, release, or approve actions.",
  ].join("\n");
}

export function renderLocalHelpEntryHtml(): string {
  return `
    <section class="panel help-panel" aria-label="Help and reference">
      <p class="eyebrow">Help and reference</p>
      <h2>Help and reference</h2>
      <p>Learn what each check means and how to read this local review page.</p>
      <p class="secure-notice">This page is local and read-only. It does not publish releases, change credentials, or contact external services.</p>
      <p><a class="button-link" href="${localHelpIndexHref}" target="_blank" rel="noreferrer">Open help</a></p>
      <p class="muted">The help entry point does not change runtime state, run providers, write packages, use network, publish, deploy, sign, release, or approve actions.</p>
    </section>`;
}
