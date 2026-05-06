---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Repository Audit - Phase 90

## Scope
Phase 90 is a roadmap and Production Candidate gap audit only.

Phase 90 reconciles committed evidence from Phases 85-89, audits remaining Production Candidate gaps, checks whether Phase 91-94 should remain pure hardening, and audits validation/lint fidelity against the current repository shape.

Phase 90 does not implement runtime behavior.

Phase 90 does not approve Production Candidate status unless committed evidence unexpectedly supports it.

Phase 90 does not repair validators or linters. Validation/lint defects, if found, require an out-of-band maintenance phase before further implementation.

## Evidence rule
Only committed artifacts count as Phase 90 evidence:

- committed code
- committed tests
- committed docs
- committed checklists
- committed validation logs
- passing validation gates

The audit does not count plans, intended hardening, architecture rationale alone, design capability statements, future roadmap items, or unmerged/non-history agent runs.

## Phase 85-89 boundary review
| Phase | Historical outcome from `CHANGELOG.md` and operations docs | Phase 90 reconciliation |
| --- | --- | --- |
| 85 | Roadmap/changelog alignment and Phase 85-100 expansion only. | Confirmed as alignment/planning only; no Production Candidate approval. |
| 86 | User-facing local workflow documentation only. | Confirmed as documentation only; no runtime capability, observability, export, startup, or packaging behavior. |
| 87 | Read-only supplied-evidence observability snapshots only. | Confirmed as snapshot/read-only/non-authoritative behavior; no export files, persistence write, replay repair, provider execution, action execution, or live transport. |
| 88 | Deterministic audit export encoding only. | Confirmed as canonical in-memory encoding only; no filesystem export, ledger/audit append, recovery input, replay repair, provider/action execution, or authority mutation. |
| 89 | Local export write boundary only. | Confirmed as verified local export write of already encoded bytes through the persistence boundary; not ledger append, recovery, promotion, replay repair, live telemetry, or authority. |

Phases 85-89 improved documentation, observability, encoding, and export write boundaries, but those outcomes do not close hardening, startup, packaging, troubleshooting, release dry-run, or final readiness-decision gaps.

## Roadmap/changelog alignment
The roadmap files preserve planned truth for Phases 85-100 and do not mark future phases as complete.

Roadmap remains planned truth.

`CHANGELOG.md` remains historical truth.

`CHANGELOG.md` records completed accepted work and does not encode future plans as completed work.

Phase 90 confirms the roadmap title and scope: `Roadmap and Production Candidate Gap Audit`; gap audit only, not approval.

## Production Candidate status
Production Candidate status: not approved.

Reason: Phases 85-89 improved documentation, observability, encoding, and export write boundaries, but the system still requires committed hardening evidence before startup/package work can be considered safe. Production Candidate status cannot be approved from plans or architectural intent.

Production Candidate status is not approved while hardening gaps remain.

## Production Candidate gap audit
Remaining gap categories from committed evidence:

- transport abuse and submission spoofing hardening
- authorization/audit/action mismatch hardening
- persistence corruption and append drift hardening
- provider output injection and replay abuse hardening
- validation/lint fidelity if any defect is confirmed
- local startup command boundary
- packaging artifact definition
- operator troubleshooting documentation
- release engineering dry run
- final Production Candidate readiness decision evidence

Committed Phases 85-89 evidence is useful, but it does not prove the negative-path hardening block, startup/package safety, release mechanics, or final readiness decision.

## Phase 91-94 hardening decision
The next block should remain hardening-only:

- Phase 91: transport abuse and submission spoofing hardening
- Phase 92: authorization/audit/action mismatch hardening
- Phase 93: persistence corruption and append drift hardening
- Phase 94: provider output injection and replay abuse hardening

No startup, packaging, release engineering, or readiness approval should begin before the hardening block completes and Phase 95 realigns the evidence.

Phase 91-94 remains hardening-only unless evidence later proves otherwise.

## AST/boundary lint fidelity audit
Phase 90 audited the Python validators, Rust boundary lint, UI AST lint, `scripts/check.sh`, and GitHub Actions gates against the current repository shape now that live Rust behavior exists.

`rg` scans were used as discovery/evidence only. Blocking enforcement comes from `scripts/check.sh`, the Rust boundary lint, the UI AST lint, compiler/type checks, and tests.

No validator or linter repair was made in Phase 90.

## Python validator fidelity
| Question | Finding |
| --- | --- |
| Do Python validators understand the current docs/checklists/operations/frontmatter layout? | Confirmed. `validate_structure.py` maps `docs/operations` to orientation truth, `checklists` to procedural truth, and requires frontmatter on docs/checklist Markdown. `validate_docs.py` enforces the same broad layout and root anchor expectations. |
| Do Python validators correctly allow `docs/operations/*` with orientation/readme_update? | Confirmed. Both validators allow `docs/operations/` orientation docs with `mutation_path: readme_update`; `validate_docs.py` does not require a fixed authority level for operations docs. |
| Do Python validators reject stale or misplaced authority surfaces? | Confirmed for known anchors and common drift. They reject root `GOVERNANCE.md`, root `ARCHITECTURE.md`, root `PHASE_MAP.md`, schemas outside `schemas/`, docs schemas, Markdown under `memory/`, roadmap completion wording, changelog future-planning wording, and authoritative README/AGENTS drift. |

No Python validator defect was confirmed in Phase 90.

## Rust boundary lint fidelity
| Question | Finding |
| --- | --- |
| Do Rust boundary lint rules still match the current Rust file ownership model? | Confirmed with caveat. The lint keeps `core/src/api/mod.rs` module/re-export only, restricts persistence-only filesystem tokens to `core/src/api/persistence.rs`, and scans all Rust files outside `.git`, `target`, and `node_modules`. |
| Do Rust lint rules allow filesystem behavior only in persistence-owned surfaces? | Confirmed for configured filesystem tokens and helper names. Filesystem write/read helper behavior is allowed in `core/src/api/persistence.rs` and rejected elsewhere by self-tests and production lint. |
| Do Rust lint rules distinguish forbidden live network/process/async behavior from test/doc strings? | Confirmed for strings and comments through stripping before scanning, and confirmed for forbidden live network/process/async/thread tokens through global regexes. Process tokens remain globally forbidden; the script contains a special-case for `std::process` in persistence but not for `Command::`, and no committed persistence process use exists. |

A Rust boundary lint self-test reporting defect was confirmed in Phase 90: `node scripts/test_rust_boundary_lint.mjs` exits 0 but prints `Rust boundary lint self-tests passed (12/13).` This does not show an assertion failure, panic, traceback, or failed assertion output, but it is a validation-gate fidelity defect because the self-test count reports incomplete coverage while the command succeeds.

## UI AST lint fidelity
| Question | Finding |
| --- | --- |
| Do UI AST lint rules still match the current UI source shape? | Confirmed. The lint targets `ui/src` by default, parses `.ts` and `.tsx`, and checks UI source with TypeScript AST rather than text-only scans. |
| Do UI AST lint rules distinguish forbidden behavior from static strings? | Confirmed by self-tests. Static text containing forbidden words is allowed, while AST nodes for calls, constructors, storage identifiers, imports, JSX handlers, forms, buttons, anchor hrefs, and submit inputs are rejected. |
| Do lint self-tests prove both positive and negative cases? | Confirmed. Rust and UI lint self-tests include passing fixtures and failing fixtures, including CLI failure diagnostics. |

No UI AST lint defect was confirmed in Phase 90.

## check.sh and CI gate fidelity
| Question | Finding |
| --- | --- |
| Does `scripts/check.sh` fail if any lint self-test fails? | Confirmed by script wiring and `set -euo pipefail`; the Rust boundary lint self-tests, Rust boundary lint, UI AST lint self-tests, and UI AST lint are direct commands in the local gate. |
| Do GitHub Actions invoke equivalent gates, or is local validation stronger than CI? | Local validation is stronger than CI as a single aggregate gate. CI invokes structure validation, documentation validation, Rust formatting/check/test/clippy, UI typecheck/lint/build, UI AST lint self-tests, UI AST lint, schema syntax, shell parse, and memory placement across separate workflows, while `scripts/check.sh` additionally runs bootstrap idempotence, Python compile checks, and Rust boundary lint/self-tests locally. |

A local gate fidelity defect was confirmed: `scripts/check.sh` invokes the Rust boundary lint self-tests and exits 0, but it propagates the `Rust boundary lint self-tests passed (12/13).` output without failing. CI does not invoke the Rust boundary lint or Rust boundary lint self-tests, so local validation remains stronger than CI but the local Rust lint self-test reporting defect requires out-of-band validation-tool repair.

## Validation anomaly review
No validation command printed an assertion failure, panic, traceback, or failed assertion output during Phase 90 validation.

A validation-gate fidelity defect was confirmed because the Rust boundary lint self-test command exits 0 while printing `Rust boundary lint self-tests passed (12/13).` Phase 90 therefore does not call the validation posture clean and does not proceed to Phase 91.

Recommended out-of-band repair phase: `Phase 90.1 - Out-of-Band Validation Gate Repair`.

Goal: repair stale Python validator, Rust boundary lint, UI AST lint, or `check.sh` enforcement behavior discovered during Phase 90.

Boundary: validation tooling only; no runtime behavior.

## Required follow-ups
- Open `Phase 90.1 - Out-of-Band Validation Gate Repair` before Phase 91 to repair the Rust boundary lint self-test reporting defect.
- Keep Phase 91-94 as a hardening-only block after validation gate fidelity is repaired.
- Reconcile the hardening evidence in Phase 95 before startup/package work.
- Preserve roadmap planned truth and `CHANGELOG.md` historical truth.
- Preserve non-readiness posture until a future evidence-complete decision gate approves a Production Candidate.

## Deferred items
- Phase 91 transport abuse and submission spoofing hardening.
- Phase 92 authorization/audit/action mismatch hardening.
- Phase 93 persistence corruption and append drift hardening.
- Phase 94 provider output injection and replay abuse hardening.
- Phase 95 evidence realignment.
- Phase 96 startup command boundary.
- Phase 97 packaging artifact definition.
- Phase 98 operator troubleshooting documentation.
- Phase 99 release engineering dry run.
- Phase 100 Production Candidate readiness decision evidence.

## Confirmed vs suspected
### Confirmed
- Phase 90 is a gap audit only.
- Phase 90 does not implement runtime behavior.
- Phase 90 does not begin Phase 91 hardening.
- Phase 90 does not approve Production Candidate status.
- Roadmap remains planned truth.
- `CHANGELOG.md` remains historical truth.
- Phases 85-89 match their historical boundaries.
- Phase 91-94 should remain hardening-only.
- A Rust boundary lint self-test reporting defect was confirmed: `node scripts/test_rust_boundary_lint.mjs` prints `Rust boundary lint self-tests passed (12/13).` while exiting 0.

### Suspected
- Additional hardening may still be required after Phase 94 depending on the committed negative-path evidence produced in Phases 91-94 and reconciled in Phase 95.

## Non-readiness statement
Production Candidate status: not approved.

Phase 90 does not implement runtime behavior.

Phase 90 does not approve Production Candidate status.

Phase 90 does not begin Phase 91 hardening.

Production Candidate status is not approved because hardening gaps remain and committed evidence does not yet support startup/package/release/readiness approval.
