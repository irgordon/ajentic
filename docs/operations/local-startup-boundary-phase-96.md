---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Local Startup Boundary - Phase 96

## Scope
Phase 96 is a local startup boundary candidate only.

Phase 96 defines the minimal local startup command/path an operator may use to verify that the repository can enter its bounded local non-authoritative workflow after the Phase 95 hardening evidence gates.

Phase 96 is a usability boundary only. It is not production readiness, packaging approval, public usability approval, release-candidate readiness, or Production Candidate approval.

## Startup boundary decision
**Decision: Option A - existing dry-run remains the local startup boundary.**

The supported local startup boundary is the existing dry-run command:

```sh
cargo run --manifest-path core/Cargo.toml -- dry-run
```

No new runtime behavior, CLI command, helper export, test command, script gate, UI startup path, package entry, or README surface was added for Phase 96.

This decision reuses the existing deterministic dry-run because it already reports that provider/model calls are not performed, persistence does not occur, provider output remains untrusted, and release/production readiness is not claimed.

## Supported local startup command
Run from the repository root:

```sh
cargo run --manifest-path core/Cargo.toml -- dry-run
```

The command is the only Phase 96-supported local startup command/path.

It is a bounded local report surface for operator testing. It is not a server start command, daemon command, UI launch command, package command, installer command, provider command, persistence command, replay repair command, export command, import command, or action-execution command.

## What this command does
The dry-run command:

- starts and exits as a foreground CLI process;
- constructs deterministic in-memory fixture request/output values;
- maps those fixture values through existing Rust-owned validation/mapping surfaces;
- reports that provider output remains untrusted;
- reports that no provider/model was called;
- reports that no persistence occurred;
- reports that no files were read or written;
- reports that release-candidate readiness is not claimed;
- reports that production readiness is not claimed.

The local startup report posture is:

| Field | Phase 96 posture |
| --- | --- |
| `startup_status` | `bounded_local_startup_reported` |
| `provider_call_performed` | `false` |
| `model_call_performed` | `false` |
| `live_transport_started` | `false` |
| `ui_bridge_started` | `false` |
| `server_started` | `false` |
| `background_process_started` | `false` |
| `persistence_written` | `false` |
| `ledger_appended` | `false` |
| `audit_appended` | `false` |
| `export_written` | `false` |
| `recovery_promoted` | `false` |
| `replay_repaired` | `false` |
| `action_executed` | `false` |
| `mutates_authority` | `false` |
| `production_candidate_approved` | `false` |
| `release_candidate_ready` | `false` |
| `public_usability_approved` | `false` |
| `startup_package_approved` | `false` |

## What happens when it fails
If the dry-run command fails, the operator should treat the local startup boundary as not satisfied for that checkout.

A failure means the local CLI report did not complete successfully. It does not imply provider failure, transport failure, persistence failure, server failure, package failure, public usability failure, or Production Candidate failure, because those surfaces are outside Phase 96 and are not started by the command.

The expected response is to inspect the terminal output, run the repository validation gates, and keep the phase in a non-ready state until the bounded local command and validation evidence pass.

## What this command does not do
The dry-run command does not:

- start a server;
- start a daemon;
- start a background process;
- open a socket;
- use network transport;
- start an async runtime;
- spawn threads or child processes;
- launch a browser;
- start a file watcher;
- start live UI/Rust transport;
- call a provider or model;
- expand provider execution;
- write persistence;
- append a durable ledger entry;
- append an audit record;
- write an export;
- import data;
- repair replay;
- promote recovery;
- replace global state;
- execute actions;
- add action authority;
- create package artifacts;
- create installer behavior;
- approve startup/package readiness;
- approve public usability;
- approve release-candidate readiness;
- approve production readiness;
- approve Production Candidate status.

## Operator expectations
Operators should use the dry-run command only as a local startup boundary smoke check.

A passing dry-run means the bounded local report path completed and emitted the expected non-authoritative posture. It does not mean the application is ready for public use, packaging, release engineering, production, or a Production Candidate decision.

Operators should run the full validation suite before relying on Phase 96 evidence.

## Non-authority guarantees
Phase 96 keeps local startup bounded, deterministic, non-authoritative, and non-executing.

The command path does not mutate authority, call providers/models, write persistence, append ledger/audit records, export files, repair replay, promote recovery, replace state, execute actions, start live transport, start a server, create package artifacts, or launch a public app surface.

Model output remains untrusted until validated through Rust-owned paths.

## Relationship to Phase 95.4 evidence gate
Phase 95.4 aligned committed post-hardening evidence and found that Phase 96 may begin as the next bounded planned non-readiness phase.

Phase 96 relies on that gate only to define a local startup boundary candidate. It does not reopen Phase 95.1, Phase 95.2, Phase 95.3, or Phase 95.4 hardening evidence, and it does not convert those gates into readiness approval.

## Relationship to Phase 97 packaging boundary
Phase 97 remains responsible for packaging boundary work if Phase 96 evidence permits it.

Phase 96 does not define package artifacts, package naming, installer behavior, distribution behavior, release mechanics, or package approval.

## Validation evidence
Phase 96 validation requires the full repository gate, explicit Rust tests, explicit golden/adversarial filters, UI behavioral tests, boundary lint self-tests and lints, UI validation, CLI dry-run, startup boundary scans, no-live-startup scans, no-authority scans, source guards, Rust source guards, readiness scans, roadmap/changelog scans, lint wiring scans, clean working-tree review, staged-file review, artifact cleanup, and final commit review.

Because Phase 96 is documentation-only and reuses the existing dry-run, no new runtime tests were required.

## AST/boundary lint parity
`rg` scans are discovery/evidence only and are not enforcement.

Blocking enforcement remains in `scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests.

Phase 96 did not identify a new source pattern requiring lint maintenance. No lint behavior changed, so no lint self-tests were added or updated.

## Test fidelity
No runtime behavior changed in Phase 96, so no new tests were required.

The existing full validation suite still had to pass after final edits. No phase is complete if validation is skipped after final edits.

Cross-boundary behavior was not expanded. Root integration coverage was not changed because the public CLI behavior already existed and remained unchanged.

## Confirmed vs suspected
| Item | Status | Finding |
| --- | --- | --- |
| Phase 96 title/scope | Confirmed | Roadmap planned truth identifies Phase 96 as Local Startup Command Boundary with a minimal local startup command surface for operator testing. |
| Startup command decision | Confirmed | Existing `cargo run --manifest-path core/Cargo.toml -- dry-run` remains sufficient for Phase 96. |
| Runtime behavior | Confirmed | No new runtime behavior was added. |
| Provider/model calls | Confirmed | The supported dry-run reports no provider/model call and validation scans found no new provider/model startup behavior. |
| Persistence/export/action authority | Confirmed | Phase 96 adds no persistence write, export write, ledger/audit append, recovery promotion, replay repair, or action execution. |
| Readiness approval | Confirmed | Phase 96 makes no startup/package, public usability, release-candidate, production-readiness, or Production Candidate approval claim. |
| New lint need | Confirmed | No new concrete source pattern requiring lint maintenance was found. |
| Future packaging work | Confirmed | Phase 97 remains responsible for packaging boundary work if Phase 96 evidence permits it. |

## Deferred items
| Item | Status | Reason |
| --- | --- | --- |
| New local-startup CLI alias/report | Deferred | Existing dry-run is sufficient; adding a second command would add unnecessary surface. |
| README startup section | Deferred | Operations documentation is sufficient and avoids elevating the command into a public-facing readiness claim. |
| Script gate changes | Deferred | Existing validation already runs the relevant checks; no new gate was needed for documentation-only Phase 96. |
| UI startup path | Deferred | Phase 96 does not start UI, browser, bridge, server, or live transport. |
| Packaging artifacts/installers | Deferred | Phase 97 owns packaging boundary work. |
| Production Candidate approval | Deferred | Phase 100 is the planned decision gate; no approval exists in Phase 96. |

## Non-readiness statement
Phase 96 is a local startup boundary candidate only.

Phase 96 does not approve startup/package readiness.

Phase 96 does not approve public usability.

Phase 96 does not approve release-candidate readiness.

Phase 96 does not approve production readiness.

Phase 96 does not approve Production Candidate status.

Roadmap remains planned truth.

`CHANGELOG.md` remains historical truth.
