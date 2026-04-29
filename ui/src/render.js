function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

function badgeClass(value) {
  return `badge badge-${String(value).toLowerCase()}`;
}

function badge(value) {
  return `<span class="${badgeClass(value)}">${escapeHtml(value)}</span>`;
}

function listOrEmpty(items, emptyLabel) {
  if (!items || items.length === 0) {
    return `<li class="missing">${escapeHtml(emptyLabel)}</li>`;
  }

  return items.map((item) => `<li>${escapeHtml(item)}</li>`).join("");
}

function valueOrMissing(value) {
  if (!value) {
    return '<span class="missing">missing</span>';
  }

  return escapeHtml(value);
}

function renderSnapshot(snapshot) {
  const evaluationRows = snapshot.evaluations
    .map(
      (item) => `
      <tr>
        <td>${escapeHtml(item.evaluatorId)}</td>
        <td>${badge(item.status)}</td>
        <td>${valueOrMissing(item.evidenceRef)}</td>
        <td><ul>${listOrEmpty(item.failureReasons, "none")}</ul></td>
      </tr>`
    )
    .join("");

  const policyRows = snapshot.governance.policyChecks
    .map(
      (item) => `
      <tr>
        <td>${escapeHtml(item.id)}</td>
        <td>${badge(item.status)}</td>
        <td>${valueOrMissing(item.evidenceRef)}</td>
        <td><ul>${listOrEmpty(item.failureReasons, "none")}</ul></td>
      </tr>`
    )
    .join("");

  return `
    <section class="panel">
      <h2>Run overview</h2>
      <p><strong>Run ID:</strong> ${escapeHtml(snapshot.runId)}</p>
    </section>

    <section class="panel">
      <h2>Candidate record</h2>
      <p><strong>ID:</strong> ${escapeHtml(snapshot.candidate.id)}</p>
      <p><strong>Lifecycle:</strong> ${badge(snapshot.candidate.lifecycleState)}</p>
      <p><strong>Objective:</strong> ${escapeHtml(snapshot.candidate.objectiveId)}</p>
      <p><strong>Constraints:</strong> ${escapeHtml(snapshot.candidate.constraintsId)}</p>
      <p><strong>Domain:</strong> ${escapeHtml(snapshot.candidate.domainId)}</p>
      <p><strong>Adapter:</strong> ${escapeHtml(snapshot.candidate.adapterName)}</p>
      <p><strong>Output summary:</strong> ${escapeHtml(snapshot.candidate.outputSummary)}</p>
    </section>

    <section class="panel">
      <h2>Evaluation results</h2>
      <table>
        <thead>
          <tr>
            <th>Evaluator</th>
            <th>Status</th>
            <th>Evidence</th>
            <th>Failure reasons</th>
          </tr>
        </thead>
        <tbody>${evaluationRows}</tbody>
      </table>
    </section>

    <section class="panel">
      <h2>Governance result</h2>
      <p><strong>Status:</strong> ${badge(snapshot.governance.status)}</p>
      <p><strong>Required evaluators satisfied:</strong> ${escapeHtml(snapshot.governance.requiredEvaluatorsSatisfied)}</p>
      <h3>Policy checks</h3>
      <table>
        <thead>
          <tr>
            <th>Policy check</th>
            <th>Status</th>
            <th>Evidence</th>
            <th>Failure reasons</th>
          </tr>
        </thead>
        <tbody>${policyRows}</tbody>
      </table>
      <p><strong>Blocked reasons</strong></p>
      <ul>${listOrEmpty(snapshot.governance.blockedReasons, "none")}</ul>
      <p><strong>Failure reasons</strong></p>
      <ul>${listOrEmpty(snapshot.governance.failureReasons, "none")}</ul>
    </section>

    <section class="panel">
      <h2>Promotion decision</h2>
      <p><strong>Status:</strong> ${badge(snapshot.promotion.status)}</p>
      <p><strong>From state:</strong> ${escapeHtml(snapshot.promotion.fromState)}</p>
      <p><strong>To state:</strong> ${escapeHtml(snapshot.promotion.toState)}</p>
      <p><strong>Required checks passed:</strong> ${escapeHtml(snapshot.promotion.requiredChecksPassed)}</p>
      <p><strong>Evidence refs</strong></p>
      <ul>${listOrEmpty(snapshot.promotion.evidenceRefs, "missing")}</ul>
      <p><strong>Denial reasons</strong></p>
      <ul>${listOrEmpty(snapshot.promotion.denialReasons, "none")}</ul>
    </section>

    <section class="panel">
      <h2>Ledger / Audit / Replay</h2>
      <p>${escapeHtml(snapshot.ledgerSummary)}</p>
      <p>${escapeHtml(snapshot.auditSummary)}</p>
      <p>${escapeHtml(snapshot.replaySummary)}</p>
    </section>
  `;
}

export function renderApp(root, snapshots, lifecycleLegend, statusLegend) {
  const snapshot = snapshots[0];

  root.innerHTML = `
    <header>
      <h1>AJENTIC</h1>
      <p>Under Your Control</p>
      <p class="subtitle">Visibility surface only</p>
    </header>

    <section class="banner">
      <p><strong>Boundary:</strong> Rust is authority. UI does not decide.</p>
      <p>Evaluation result ingestion is not governance approval. Required evaluator satisfaction is not promotion eligibility.</p>
    </section>

    <section class="panel">
      <h2>Invariants</h2>
      <ul>
        <li>Generated output is untrusted by default.</li>
        <li>Only Rust may promote to Tier-1.</li>
        <li>UI is visibility only.</li>
        <li>UNKNOWN is not PASS.</li>
        <li>Tier-1 is reviewable, not production-approved.</li>
      </ul>
      <p><strong>Lifecycle states:</strong> ${escapeHtml(lifecycleLegend.join(", "))}</p>
      <p><strong>Review statuses:</strong> ${escapeHtml(statusLegend.join(", "))}</p>
    </section>

    ${renderSnapshot(snapshot)}
  `;
}
