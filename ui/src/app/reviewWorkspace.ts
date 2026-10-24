import { escapeHtml, icon } from "../display/markup.js";
import { renderActivity, renderChecks, renderComparison, renderRequestProblem, renderResult, renderToolResult } from "./reviewContent.js";
import { hasReviewDecision, reviewDecisionLabel, SAMPLE_FILE, type ReviewTab, type ReviewView } from "./reviewModel.js";

export function renderReviewWorkspace(view: ReviewView, diagnostics: string): string {
  return `<main class="review-app">${renderHeader()}<div class="review-body">${renderSidebar(view)}
    <div class="review-main">${renderTabs(view)}<div class="review-scroll">${renderActiveView(view, diagnostics)}</div></div>
    </div>${renderReviewFooter(view)}</main>`;
}

function renderHeader(): string {
  return `<header class="review-header"><a class="brand" href="/" aria-label="AJENTIC Home">AJENTIC</a><span class="badge local-demo">Local Demo</span>
    <div class="header-actions"><button id="start-run" class="quiet-button" title="Load the built-in local sample">${icon("play")}Run Sample</button>
      <details class="view-menu"><summary>View ${icon("chevron-down")}</summary><div class="menu-content">
        <button data-action="tab-checks">${icon("circle-check")}Checks</button><button data-action="tab-diagnostics">${icon("settings-2")}Developer Details</button>
        <a href="/" title="Refresh to reset this in-memory example">${icon("rotate-ccw")}Reset Example</a></div></details>
      <a class="help-link" href="/help/index.html" target="_blank" rel="noreferrer">${icon("circle-question-mark")}Help</a>
    </div></header>`;
}

function renderSidebar(view: ReviewView): string {
  return `<aside class="file-sidebar" aria-label="File Changes"><h1>${icon("file-text")}File Changes</h1>
    <button class="file-item selected" data-action="tab-compare" aria-label="Compare sample-note.txt">${icon("file-text")}<span>${SAMPLE_FILE.name}</span><span class="change-count"><b>+1</b><b>-1</b></span></button>
    <details class="example-details" data-open-key="example-details" open><summary>Example Details ${icon("chevron-up")}</summary><dl>
      <div><dt>Path</dt><dd class="code-text">${SAMPLE_FILE.name}</dd></div><div><dt>Comparison</dt><dd>Illustrative Sample</dd></div>
      <div><dt>Lines Changed</dt><dd class="change-count"><b>+1</b><b>-1</b></dd></div><div><dt>Purpose</dt><dd>Practice reviewing a draft and its recorded checks. No files are written.</dd></div>
    </dl></details>${renderDetailsSwitch(view)}${renderSidebarMetadata(view)}
    <div class="sidebar-boundary">${icon("info")}<p>Local review only. No live tools or file writes.</p></div>
  </aside>`;
}

function renderDetailsSwitch(view: ReviewView): string {
  return `<label class="details-switch" for="show-details">Show Details<input id="show-details" type="checkbox" role="switch" ${view.preferences.showDetails ? "checked" : ""} /></label>`;
}

function renderSidebarMetadata(view: ReviewView): string {
  if (!view.preferences.showDetails) return "";
  return `<div class="sidebar-metadata"><dl><dt>Run Status</dt><dd><code>${escapeHtml(view.state.run.status)}</code></dd>
    <dt>Candidate</dt><dd><code>${escapeHtml(view.state.run.candidate?.candidateId ?? "not_started")}</code></dd>
    <dt>Source</dt><dd>Deterministic local fixture</dd><dt>Context</dt><dd><code>${escapeHtml(view.state.run.boundedContext.join("; ") || "No sample loaded")}</code></dd></dl></div>`;
}

function renderTabs(view: ReviewView): string {
  const tabs: readonly [ReviewTab, string, string][] = [["compare", "Compare", "file-text"], ["result", "Result", "list"], ["activity", "Activity", "clock"]];
  return `<div class="review-toolbar"><div class="review-tabs" role="tablist" aria-label="Review Views">${tabs.map((tab) => renderTab(tab, view.preferences.tab)).join("")}</div>
    <div class="file-meta"><span class="badge sample-badge">Sample Comparison</span><span class="filename">${icon("file-text")}<code>${SAMPLE_FILE.name}</code></span><span class="no-files">No Files Written</span></div></div>`;
}

function renderTab(tab: readonly [ReviewTab, string, string], active: ReviewTab): string {
  const [id, label, image] = tab;
  const focusable = id === active || (id === "compare" && (active === "checks" || active === "diagnostics"));
  return `<button id="tab-${id}" data-action="tab-${id}" role="tab" aria-selected="${id === active}" aria-controls="review-panel" tabindex="${focusable ? "0" : "-1"}">${icon(image)}${label}</button>`;
}

function renderActiveView(view: ReviewView, diagnostics: string): string {
  const content = activeViewContent(view, diagnostics);
  const label = VIEW_LABELS[view.preferences.tab];
  const notice = view.preferences.tab === "compare" || view.preferences.tab === "result" ? "" : renderRequestProblem(view, true);
  return `<div id="review-panel" role="tabpanel" aria-label="${escapeHtml(label)}" tabindex="0">${notice}${content}</div>`;
}

const VIEW_LABELS: Readonly<Record<ReviewTab, string>> = {
  compare: "Sample Comparison", result: "Draft Output", activity: "Activity",
  checks: "Checks", diagnostics: "Developer Details",
};

function activeViewContent(view: ReviewView, diagnostics: string): string {
  switch (view.preferences.tab) {
    case "compare": return renderComparison(view) + renderToolResult(view) + renderActivity(view);
    case "result": return renderResult(view);
    case "activity": return renderActivity(view);
    case "checks": return renderChecks(view);
    case "diagnostics": return `<section class="diagnostics"><h2>Developer Details</h2><p class="muted">Existing local test controls and raw evidence. These do not enable live tools, publishing, or release authority.</p>${diagnostics}</section>`;
  }
}

function renderReviewFooter(view: ReviewView): string {
  return `<footer class="review-footer"><section class="review-notes" aria-label="Review Notes"><h2>${icon("file-text")}Review Notes</h2>
    <label class="visually-hidden" for="review-notes">Notes About This Sample</label><textarea id="review-notes" maxlength="2000" rows="2" placeholder="Add a note about this draft (optional)...">${escapeHtml(view.preferences.notes)}</textarea>
    <div class="notes-meta"><button class="attachment-button" disabled title="Attachments are not connected in this local demo">${icon("paperclip")}Attachments Not Connected</button><span>Notes stay in this page only.</span></div>
    </section>${renderDecision(view)}</footer>`;
}

function renderDecision(view: ReviewView): string {
  const disabled = !view.state.run.candidate || hasReviewDecision(view.state);
  return `<section class="review-decision" aria-label="Review Decision"><h2>${icon("circle-check")}Review Decision</h2>
    <p class="muted">Sample draft review only. No tool execution or file writes.</p>
    <p class="decision-message" role="status">${escapeHtml(reviewDecisionLabel(view.state))}</p>
    <div class="decision-actions"><button id="approve-run" class="primary-button" ${disabled ? "disabled" : ""}>Record Approval</button><button id="reject-run" class="secondary-button" ${disabled ? "disabled" : ""}>Reject Draft</button></div></section>`;
}
