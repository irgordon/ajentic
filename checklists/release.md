---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Release-candidate evidence checklist

This checklist defines procedural evidence requirements for a future release-candidate claim.

This document does not define governance rules or architecture authority.

This document does not record changelog history.

This document does not claim release-candidate readiness is achieved.

This document does not claim production readiness.

## Release-candidate boundary

A future release candidate requires evidence that AJENTIC is usable for controlled local model-driven work without losing Rust-owned authority.

## Required evidence

- [ ] Rust checks pass.
- [ ] UI typecheck/lint/build pass.
- [ ] Structure and documentation gates pass.
- [ ] Schema syntax validation passes.
- [ ] Script boundary checks pass.
- [ ] Provider and integration outputs remain untrusted.
- [ ] Controlled flow remains deterministic and in-memory.
- [ ] Replay verification remains read-only and idempotent.
- [ ] UI remains non-authoritative.
- [ ] Operator intent controls remain request-only unless a future phase explicitly implements Rust-owned intent submission.
- [ ] Raw provider/model output remains visibly distinct from clean output.
- [ ] No accepted output bypasses policy, validation, ledger, replay, or audit expectations where required.
- [ ] Static scan debt is documented and triaged before any production-readiness claim.
- [ ] No production-readiness claim is made without separate production evidence.

## Blocking conditions

- [ ] Provider output treated as trusted.
- [ ] UI authority over runtime decisions.
- [ ] Direct ledger/memory/replay mutation from UI.
- [ ] Replay repair without explicit command and audit trail.
- [ ] Failed validation gates.
- [ ] Undocumented architecture/governance drift.
- [ ] Changelog/roadmap truth-dimension drift.
- [ ] Raw JSON-only human review path for critical operator review.
- [ ] Untriaged static scan debt before production-readiness claim.
- [ ] Production capability claim without evidence.

## Validation commands

Required:

- `./scripts/check.sh`
- `cd ui && npm run typecheck && npm run lint && npm run build`

Optional/manual review:

- `rg -n "fetch|localStorage|sessionStorage|WebSocket|EventSource|setInterval|setTimeout|onClick|onSubmit|submit|form|href=|method=|action=" ui/src`
- `rg -n "reqwest|ureq|hyper|tokio|async|await|fetch|http|https|api key|apikey|token|Authorization|Bearer|TcpStream|UdpSocket|std::net" core scripts ui`
- `rg -n "repair|auto-repair|reorder|append|remove|mutate|write|persist|std::fs|File::|read_to_string|serde|json" core/src/execution/mod.rs core/src/replay/mod.rs core/src/ledger/mod.rs`

## Authority-boundary checks

- [ ] Rust remains the only authoritative runtime surface.
- [ ] No non-Rust surface bypasses policy, validation, ledger, replay, or audit boundaries.
- [ ] Provider/model output remains untrusted unless accepted through Rust-owned paths.

## UI evidence checks

- [ ] UI remains projection/review and request-only intent surface.
- [ ] UI does not directly mutate runtime authority surfaces.
- [ ] UI clearly distinguishes raw output from clean output.
- [ ] UI does not regress to raw JSON-only critical review paths.

## Provider/integration evidence checks

- [ ] Provider/local/IDE integration boundaries preserve untrusted output treatment.
- [ ] Provider integrations do not write authority state directly.
- [ ] Integration boundaries do not bypass Rust acceptance logic.

## Replay and audit evidence checks

- [ ] Replay verification remains deterministic for repeated runs on identical input.
- [ ] Replay verification remains read-only and non-repairing.
- [ ] Audit remains projection-only and non-authoritative.

## Documentation/truth-dimension checks

- [ ] `CHANGELOG.md` remains historical truth only.
- [ ] `docs/roadmap/phase-map.md` remains planned truth only.
- [ ] Procedural release evidence remains in checklist surfaces.

## Static scan debt

- [ ] Current ripgrep-based boundary scans are advisory transitional checks.
- [ ] Ripgrep scans are string-based and can produce false positives on comments, fixture text, labels, and documentation.
- [ ] Future release-candidate evidence must identify which checks may remain grep-based and which should be promoted to AST-aware linting before production readiness.
- [ ] Candidate AST-aware enforcement options include ESLint rules for TypeScript UI surfaces and Clippy/Rust test guards for Rust authority surfaces.
- [ ] Do not add ESLint rules, Clippy plugins, workflow changes, or new dependencies in Phase 36 unless validation is blocked and a minimal correction is required.

## Release decision record placeholder

| Evidence item | Status | Source | Reviewer note |
| --- | --- | --- | --- |
| Rust checks | pass | `./scripts/check.sh` (Phase 37 run) | Baseline evidence captured; no readiness claim. |
| UI typecheck/lint/build | pass | `cd ui && npm run typecheck && npm run lint && npm run build` (Phase 37 run) | Placeholder lint/build scripts passed in current repo state. |
| Structure and documentation gates | pass | `./scripts/check.sh` (structure/docs steps) | Procedural/documentation gates passed. |
| Schema syntax validation | pass | `./scripts/check.sh` (schema step) | JSON schema syntax gate passed. |
| Script boundary checks | pass | `./scripts/check.sh` (`bash -n scripts/*.sh`) | Script parse/boundary check passed. |
| Provider/integration untrusted boundary | pass | `core/src/api/mod.rs`, `core/src/execution/mod.rs`, plus trust keyword scan | Existing tests and scan evidence preserve untrusted boundary. |
| Controlled flow deterministic in-memory behavior | pass | `core/src/execution/mod.rs` tests via `./scripts/check.sh` | Deterministic in-memory controlled-flow tests passed. |
| Replay verification idempotency | pass | `docs/operations/repository-audit-phase-35.md`; `core/src/execution/mod.rs` tests in `./scripts/check.sh` | Phase 35 harmless finding carried forward; source unchanged in Phase 37. |
| UI non-authority and request-preview boundary | pass | `ui/src` boundary scan; `docs/operations/repository-audit-phase-30.md` | Phase 30 script/UI boundary audit plus current scan evidence; no UI authority change in Phase 37. |
| Raw provider/model output remains distinct from clean output | pass | `core/src/execution/mod.rs` tests and boundary wording scans | Existing controlled-flow tests keep raw output untrusted and separate from clean output summary. |
| Static scan debt triage | deferred | Phase 37 evidence report (`docs/operations/release-candidate-evidence-phase-37.md`) | Debt remains evidence debt pending future scoped AST-aware tooling phase. |
| Roadmap/changelog truth-dimension alignment | pass | `CHANGELOG.md`, `docs/roadmap/phase-map.md`, `docs/governance/truth-dimensions.md` | No drift correction needed in this phase; boundaries preserved. |
