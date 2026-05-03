---
truth_dimension: procedural
authority_level: authoritative
mutation_path: checklist_revision
---

# Current Phase: Phase 36 - Release Candidate Boundary

This is the active procedural execution surface for Phase 36.

## Phase name

Phase 36 - Release Candidate Boundary

## Phase goal

Define the minimum release-candidate capability boundary and required evidence without claiming release-candidate readiness or production readiness.

## Allowed surfaces

- `checklists/release.md`
- `checklists/current-phase.md`
- `CHANGELOG.md`
- `docs/roadmap/phase-map.md` (only if future planned sequencing drift is found)

## Boundary rules

- This phase is planning and evidence framing only.
- Do not claim release-candidate readiness is achieved.
- Do not claim production readiness is achieved.
- Preserve the distinction between release-candidate planning and production readiness evidence.
- `CHANGELOG.md` remains historical truth.
- `docs/roadmap/phase-map.md` remains planned truth.
- Before editing release-candidate boundary content, Phase 35 replay-verification idempotency findings must be carried forward and not re-investigated unless source files changed.
- Static scan debt is recorded as release-candidate evidence work only; no lint-tooling/workflow/dependency implementation in this phase.
- Do not modify runtime/UI behavior, schemas, scripts, workflows, governance docs, or architecture docs.

## Task checklist

- [x] Update `checklists/current-phase.md` to Phase 36 procedural scope.
- [x] Update `checklists/release.md` into a release-candidate evidence checklist.
- [x] Define release-candidate boundary and minimum required evidence without readiness claims.
- [x] Preserve release-candidate-planning vs production-readiness distinction.
- [x] Record static scan debt as evidence work, not tooling implementation.
- [x] Confirm `CHANGELOG.md` remains historical truth.
- [x] Confirm `docs/roadmap/phase-map.md` remains planned truth.
- [x] Confirm Phase 35 replay-verification idempotency findings are accounted for and not re-investigated absent source changes.
- [x] Add `CHANGELOG.md` entry `v0.0.36`.

## Validation checklist

- [x] `./scripts/check.sh`
- [x] `cd ui && npm run typecheck && npm run lint && npm run build`
- [x] `rg -n "production-ready|production ready|release candidate ready|release-candidate ready|RC ready|ready for production" docs checklists CHANGELOG.md README.md AGENTS.md`
- [x] `rg -n "fetch|localStorage|sessionStorage|WebSocket|EventSource|setInterval|setTimeout|onClick|onSubmit|submit|form|href=|method=|action=" ui/src`
- [x] `rg -n "reqwest|ureq|hyper|tokio|async|await|fetch|http|https|api key|apikey|token|Authorization|Bearer|TcpStream|UdpSocket|std::net" core scripts ui`
- [x] `rg -n "repair|auto-repair|reorder|append|remove|mutate|write|persist|std::fs|File::|read_to_string|serde|json" core/src/execution/mod.rs core/src/replay/mod.rs core/src/ledger/mod.rs`
- [x] `git status --short`
- [x] `git log --oneline -1`

## Release-candidate boundary checklist

- [x] Boundary statement is evidence-oriented and future-facing.
- [x] No release-candidate-ready claim is present.
- [x] No production-ready claim is present.
- [x] Provider output remains untrusted.
- [x] UI remains non-authoritative.

## Static scan debt checklist

- [x] Ripgrep-based scans documented as advisory transitional checks.
- [x] Known false-positive classes documented.
- [x] Future AST-aware enforcement options identified for RC evidence progression.
- [x] No lint tooling, workflow, or dependency implementation performed in this phase.

## Findings table

| Finding | Classification | Status | Notes |
| --- | --- | --- | --- |
| Roadmap/changelog truth split remains correct | Harmless | Closed | `CHANGELOG.md` remains historical truth and roadmap remains planned truth. |
| Phase 35 replay-verification idempotency findings carried forward | Harmless | Closed | No source changes to replay verification surfaces in this phase. |
| Required static scans return policy/boundary wording matches | Harmless | Closed | Matches are expected in governance/checklist/audit language; no readiness claims added. |

## Deferred items table

| Item | Reason deferred | Revisit phase |
| --- | --- | --- |
| AST-aware scan enforcement rollout (ESLint/Clippy guards) | Phase 36 scope is evidence framing only | Future release-candidate evidence hardening phase |

## Validation log table

| Command | Result | Notes |
| --- | --- | --- |
| `./scripts/check.sh` | Pass | Rust fmt, clippy, tests, and schema/docs structure checks passed. |
| `cd ui && npm run typecheck && npm run lint && npm run build` | Pass | UI static validation passed. |
| Readiness-phrase static scan | Pass | Matches reviewed; no current readiness claims introduced. |
| UI boundary static scan | Pass | Matches classified; no new prohibited behavior introduced in this phase. |
| Core/scripts/ui network/auth scan | Pass | Matches classified; no boundary-bypass implementation introduced in this phase. |
| Replay/ledger mutation-term scan | Pass | Matches classified as expected boundary/test language and existing APIs. |
