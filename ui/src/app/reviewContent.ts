import { escapeHtml, icon } from "../display/markup.js";
import { displayStatus, plainCategoryLabel } from "../display/statusCopy.js";
import { SAMPLE_FILE, hasReviewDecision, type ReviewView } from "./reviewModel.js";

export function renderComparison(view: ReviewView): string {
  return `<section class="comparison" aria-label="Sample Comparison">
    <div class="comparison-columns">${renderFileSide("Before", SAMPLE_FILE.before, "removed", view)}${renderFileSide("After", SAMPLE_FILE.after, "added", view)}</div>
    <button id="toggle-context" class="context-toggle" data-action="toggle-context" aria-expanded="${view.preferences.expandedContext}">
      ${icon(view.preferences.expandedContext ? "chevron-up" : "chevron-right")}<span>${view.preferences.expandedContext ? "Fold Unchanged Lines" : "Show All Unchanged Lines"}</span>
    </button>
    <p class="comparison-caption">Illustrative comparison only. This is not generated file output.</p>
  </section>`;
}

export function renderToolResult(view: ReviewView): string {
  return `<section class="tool-result" aria-label="Tool Result">
    <div class="tool-result-top"><div class="section-heading"><h2>${icon("list")}Tool Result</h2><span class="badge">Local Simulation</span></div>${renderRequestProblem(view)}</div>
    <div class="tool-result-meta">${renderResultSummary(view)}${renderEvidenceCounts(view)}</div>
  </section>`;
}

export function renderResult(view: ReviewView): string {
  const candidate = view.state.run.candidate;
  return `<section class="result-view" aria-label="Draft Output"><div class="section-heading"><h2>${icon("file-text")}Draft Output</h2><span class="badge">Untrusted Sample</span></div>
    <p class="muted">Proposed content from the local simulation, not a live AI response.</p>
    <div class="draft-document"><h3>${escapeHtml(candidate?.title ?? "Start With a Sample")}</h3>
      <p>${escapeHtml(candidate?.body ?? "Run the sample to inspect its draft, recorded display checks, and review history.")}</p></div>
    ${renderToolResult(view)}${renderRawChecks(view)}
  </section>`;
}

export function renderActivity(view: ReviewView): string {
  const events = activityEvents(view);
  return `<section class="activity" aria-label="Activity"><div class="section-heading"><h2>${icon("clock")}Activity</h2>
    <button id="toggle-activity" class="quiet-button" data-action="toggle-activity" aria-expanded="${view.preferences.showAllActivity}">${view.preferences.showAllActivity ? "Show Less" : "View All"}${icon("chevron-down")}</button></div>
    <ol class="activity-list">${events.map(renderActivityEvent).join("")}</ol>
    <details class="decision-history"><summary>Decision History</summary>${renderDecisionHistory(view)}</details>
  </section>`;
}

export function renderChecks(view: ReviewView): string {
  return `<section class="checks-view" aria-label="Checks"><h2>Checks</h2><p class="muted">Recorded local evidence, not release approval. Missing and blocked items remain visible.</p>
    ${renderRawChecks(view)}<h3>Release Checks</h3><ul class="check-list">${view.state.releaseCandidatePreparation.evidenceItems.map(renderCheckItem).join("")}</ul>
  </section>`;
}

function renderFileSide(title: string, lines: string[], kind: string, view: ReviewView): string {
  const visible = lines.map((text, index) => ({ text, line: index + 1 })).filter((row) => view.preferences.expandedContext || row.line <= 8);
  return `<section class="file-side" aria-label="${title}"><h2>${title}</h2><div class="code-lines" role="table" aria-label="${title} sample text" tabindex="0">
    ${visible.map((row) => renderCodeLine(row.text, row.line, kind)).join("")}</div></section>`;
}

function renderCodeLine(text: string, line: number, kind: string): string {
  const changed = line === SAMPLE_FILE.changedLine;
  const marker = changed ? (kind === "removed" ? "-" : "+") : "";
  return `<div class="code-line ${changed ? kind : ""}" role="row"><span class="line-number" role="cell">${line}</span><span class="change-marker" aria-hidden="true">${marker}</span><code role="cell">${escapeHtml(text)}</code></div>`;
}

function renderResultSummary(view: ReviewView): string {
  if (!view.state.run.candidate) return `<p class="result-summary muted">Run Sample to load a draft and its recorded checks. No tools have been executed.</p>`;
  return `<p class="result-summary">${icon("circle-check")}<strong>Sample Loaded</strong><span class="visually-hidden">Display checks recorded. No live tool was executed.</span></p>`;
}

export function renderRequestProblem(view: ReviewView, errorsOnly = false): string {
  const reason = view.feedback.status === "rejected" ? view.feedback.reason : view.state.localCandidateOutput.error;
  if (!reason) return errorsOnly ? "" : `<p class="output-note">${icon("info")}No file output has been created. File Changes is an illustrative comparison.</p>`;
  return `<div class="request-problem" role="status">${icon("triangle-alert")}<div><strong>Request Rejected</strong><p>${escapeHtml(friendlyProblem(reason))}</p></div>
    <details><summary>Details</summary><code>${escapeHtml(reason)}</code></details></div>`;
}

function renderEvidenceCounts(view: ReviewView): string {
  const checks = view.state.releaseCandidatePreparation;
  return `<button class="evidence-counts" data-action="tab-checks" title="Inspect every blocked, missing, or rejected local release check">${checks.blockedEvidenceCount} Need Attention <span aria-hidden="true">/</span> ${checks.missingEvidenceCount} Not Started <span aria-hidden="true">/</span> ${checks.rejectedEvidenceCount} Rejected</button>`;
}

function friendlyProblem(reason: string): string {
  if (reason === "provider_execution_result_id_mismatch") return "Result IDs do not match. No draft output was created.";
  if (reason === "duplicate_decision_rejected") return "A decision is already recorded for this sample.";
  return "This request was not accepted. Review the technical reason in Details.";
}

function renderRawChecks(view: ReviewView): string {
  const validation = view.state.run.validation;
  return `<details class="raw-checks"><summary>Check Details</summary><dl class="detail-list">
    ${detailRow("Run Status", view.state.run.status)}${detailRow("Policy", validation?.policyStatus ?? "not_started")}
    ${detailRow("Validation", validation?.validationStatus ?? "not_started")}${detailRow("Check Scope", validation?.reason ?? "No sample checks recorded.")}
    ${detailRow("Output Trust", String(view.state.run.candidate?.providerOutputTrusted ?? "not_started"))}${detailRow("Last Request", `${view.feedback.status}: ${view.feedback.reason}`)}
  </dl></details>`;
}

function detailRow(label: string, value: string): string {
  return `<div><dt>${label}</dt><dd><code>${escapeHtml(value)}</code></dd></div>`;
}

type ActivityEvent = Readonly<{ title: string; summary: string; tone: string; status: string }>;

function activityEvents(view: ReviewView): ActivityEvent[] {
  const events: ActivityEvent[] = [{ title: "Local Example Ready", summary: "In-memory simulation. No cloud connection.", tone: "complete", status: "Available" }];
  if (view.state.run.candidate) events.push({ title: "Sample Draft Loaded", summary: view.state.run.candidate.title, tone: "complete", status: "Completed Locally" });
  if (view.state.run.validation) events.push({ title: "Display Checks Recorded", summary: "The checks cover the local fixture, not factual truth.", tone: "complete", status: "Recorded" });
  if (view.preferences.showAllActivity) appendRawEvents(events, view);
  events.push({ title: "Review Decision", summary: hasReviewDecision(view.state) ? `Recorded: ${view.state.decisionLedger.records[view.state.decisionLedger.records.length - 1]?.intentKind}` : "Waiting for your review choice.", tone: hasReviewDecision(view.state) ? "complete" : "pending", status: hasReviewDecision(view.state) ? "Recorded" : "Pending" });
  return events;
}

function appendRawEvents(events: ActivityEvent[], view: ReviewView): void {
  for (const summary of view.state.run.timeline) events.push({ title: "Recorded Event", summary, tone: "neutral", status: "Local" });
}

function renderActivityEvent(event: ActivityEvent): string {
  return `<li class="activity-event ${event.tone}"><span class="event-icon">${icon(event.tone === "complete" ? "circle-check" : "clock")}</span>
    <div><strong>${event.title}</strong><p>${escapeHtml(event.summary)}</p></div><span class="event-status ${event.tone}">${event.status}</span></li>`;
}

function renderDecisionHistory(view: ReviewView): string {
  const records = view.state.decisionLedger.records;
  if (records.length === 0) return `<p class="muted">Decision Count: 0. No decision recorded yet.</p>`;
  return `<p>Decision Count: ${records.length}</p><ol>${records.map((record) => `<li>${escapeHtml(record.intentKind)} <code>${escapeHtml(record.decisionId)}</code></li>`).join("")}</ol>
    <p class="muted">${escapeHtml(view.state.run.decisionReplay.summary)}</p><code>${escapeHtml(view.state.run.decisionReplay.replayStatus)}</code>`;
}

function renderCheckItem(item: ReviewView["state"]["releaseCandidatePreparation"]["evidenceItems"][number]): string {
  const status = displayStatus(item.status);
  return `<li><div><strong>${plainCategoryLabel(item.category)}</strong><p>${escapeHtml(item.sourceLinkage.sourceSummary)}</p></div>
    <span class="badge badge--${status.tone}">${status.label}</span><details><summary>Details</summary><code>${escapeHtml(item.status)}</code></details></li>`;
}
