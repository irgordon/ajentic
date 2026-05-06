---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Operator Troubleshooting Boundary - Phase 98

## Scope
Phase 98 is documentation-only.

Phase 98 is operator-facing only.

Phase 98 introduces no new capability.

Phase 98 documents how an operator should run, interpret, and troubleshoot the existing local validation, dry-run, UI behavioral, golden invariant, adversarial corpus, build, and packaging-boundary commands.

Phase 98 does not add runtime behavior. Phase 98 does not add a new CLI surface. Phase 98 does not add packaging, installer, distribution, signing, or release engineering. Phase 98 does not add readiness claims, authority surfaces, transport behavior, provider behavior, persistence behavior, replay behavior, recovery behavior, export behavior, or action behavior.

Roadmap remains planned truth. `CHANGELOG.md` remains historical truth.

## Operator troubleshooting model
Operators should treat each command as a bounded local evidence probe. A failure means the specific checked local boundary did not pass for that checkout and environment. A pass means only that the checked local boundary passed at that moment.

Troubleshooting order:

1. Preserve the failing terminal output.
2. Identify which command failed and which boundary it checks.
3. Inspect the listed next surfaces for that command.
4. Remove generated artifact drift only when it is local/generated and not a source change.
5. Re-run the same command after documentation-only edits are complete.
6. If a gate is broken or masks failure while returning zero, record it as a validation-gate defect and defer repair to a later out-of-band maintenance phase.

Do not fix, rename, rewire, or add commands in Phase 98.

## Command interpretation rule
A passing command means only that the checked local boundary passed at that moment.

It does not approve production readiness, release-candidate readiness, public usability, packaging, distribution, installer use, provider execution, persistence authority, replay repair, recovery promotion, or action execution.

It also does not approve startup, package approval, distribution approval, installer approval, release-candidate approval, production-readiness approval, Production Candidate status, no authority surfaces, no transport, no provider, no persistence, no replay, no recovery, no export, or no action behavior beyond the exact boundary asserted by the command output and existing tests.

## Validation gate troubleshooting
Command:

```sh
./scripts/check.sh
```

### What it checks
- Repository policy, frontmatter, roadmap/changelog/checklist posture, schemas, Rust tests, Rust clippy, Rust boundary lint, UI dependency installation when needed, UI typecheck, UI lint, UI build, UI behavioral tests, UI AST boundary lint, and generated artifact hygiene as wired by the existing script.

### What a pass means
- The existing repository validation gate completed successfully for the local checkout and environment.
- The currently wired Rust, UI, documentation, schema, and lint checks passed at that moment.

### What a pass does not mean
- It does not approve production readiness, release-candidate readiness, public usability, packaging, distribution, installer use, provider execution, persistence authority, replay repair, recovery promotion, or action execution.
- It does not prove that every possible boundary is tested; it proves only that the wired gate passed.

### Common failure signals
- Frontmatter validation errors for missing or mismatched `truth_dimension`, `authority_level`, or `mutation_path`.
- Checklist validation errors for stale current-phase scope or unchecked closure items.
- Rust compiler, clippy, or test failures.
- Rust boundary lint failures.
- UI typecheck, lint, build, behavioral harness, or AST boundary lint failures.
- Generated artifact drift such as `core/target/.rustc_info.json`, UI build artifacts, node build artifacts, coverage output, export temp files, test temp files, or `scripts/__pycache__`.
- Non-failing npm warnings such as `http-proxy` dependency noise during install/update checks.

### What to inspect next
- `scripts/check.sh` for the currently wired validation sequence.
- The first failing command block in the terminal output.
- `checklists/current-phase.md` for phase scope and closure state.
- `CHANGELOG.md` for historical claims that must match the actual diff.
- Generated artifact paths reported by `git status --short`.

### What not to infer
- Do not infer that a pass is a release sign-off.
- Do not infer that a failure authorizes changing scripts, runtime behavior, package files, or lint semantics in Phase 98.
- Do not infer that `rg` scans are enforcement; they are discovery/evidence only.

## Dry-run troubleshooting
Command:

```sh
cargo run --manifest-path core/Cargo.toml -- dry-run
```

### What it checks
- The existing local dry-run CLI report path.
- Deterministic in-memory fixture mapping through Rust-owned validation surfaces.
- The local startup boundary established by Phase 96.

### What a pass means
- The dry-run command started, emitted its bounded local report, and exited successfully.
- The command reported the existing non-authoritative posture for no provider/model call, no persistence, no file read/write, and no release/production readiness claim.

### What a pass does not mean
- It does not start a server, daemon, UI bridge, live transport, provider, model, persistence layer, replay repair, recovery promotion, export, import, package, installer, or action execution path.
- It does not approve startup, packaging, public usability, release-candidate readiness, production readiness, or Production Candidate status.

### Common failure signals
- Cargo compile errors.
- Panic output or failed assertions from Rust tests built as part of local compilation.
- Missing Rust toolchain or environment issues.
- Unexpected dry-run output that omits non-authority statements.

### What to inspect next
- `core/src/main.rs` dry-run summary output.
- `core/Cargo.toml` package and target configuration.
- Phase 96 local startup documentation.
- The full `./scripts/check.sh` gate if the dry-run failure appears tied to broader Rust validation.

### What not to infer
- Do not infer provider availability or provider failure.
- Do not infer persistence durability or persistence failure.
- Do not infer transport behavior.
- Do not infer package, installer, release, or production posture.

## Rust test troubleshooting
Command:

```sh
cargo test --manifest-path core/Cargo.toml --all-targets
```

### What it checks
- The existing Rust test suite across all targets in the core crate.
- Rust-owned validation, governance, boundary, replay, persistence, export, and hardening tests already present in the repository.

### What a pass means
- The existing Rust tests passed for the local checkout and environment.
- Rust compiler/test harness execution completed without failing tests.

### What a pass does not mean
- It does not approve runtime deployment, provider execution, public usability, release-candidate readiness, production readiness, or Production Candidate status.
- It does not prove UI behavior or documentation fidelity unless those are covered by other gates.

### Common failure signals
- `FAILED` test summaries.
- Panics, assertion failures, or mismatch output.
- Compiler errors or missing toolchain components.
- Cargo metadata drift such as `core/target/.rustc_info.json` appearing in `git status --short` after a run.

### What to inspect next
- The first failing Rust test name and assertion output.
- The owning Rust module named in the failure path.
- Existing phase operations docs for the boundary the failing test covers.
- `git status --short` for generated Cargo artifacts after the run.

### What not to infer
- Do not infer that Phase 98 may patch Rust source or tests.
- Do not infer that all UI/operator documentation boundaries are covered by Rust tests alone.

## Golden invariant troubleshooting
Command:

```sh
cargo test --manifest-path core/Cargo.toml golden --all-targets
```

### What it checks
- The existing Rust tests whose names or module paths match the `golden` filter.
- Cross-boundary golden invariant coverage established before Phase 98.

### What a pass means
- The filtered golden invariant tests passed at that moment.
- The checked invariants remained stable for the local checkout.

### What a pass does not mean
- It does not approve production readiness, packaging, release-candidate readiness, public usability, or Production Candidate status.
- It does not prove every invariant in the system; it proves only the filtered tests passed.

### Common failure signals
- A filtered test fails with assertion mismatch.
- The filter unexpectedly runs zero tests or a partial set compared with expected local evidence.
- Cargo build or test harness errors occur before test execution.

### What to inspect next
- The filtered test list and pass count in the terminal output.
- Phase 95.2 cross-boundary golden invariant documentation.
- Rust test modules containing `golden` in names or module paths.

### What not to infer
- Do not infer that passing golden tests replace full `cargo test --all-targets`.
- Do not infer that a reduced or unexpected test count is acceptable without recording it.

## Adversarial corpus troubleshooting
Command:

```sh
cargo test --manifest-path core/Cargo.toml adversarial --all-targets
```

### What it checks
- The existing Rust tests whose names or module paths match the `adversarial` filter.
- LLM-output adversarial corpus coverage established before Phase 98.

### What a pass means
- The filtered adversarial tests passed at that moment.
- The checked hostile or malformed-output examples remained rejected or bounded by existing Rust-owned paths.

### What a pass does not mean
- It does not prove all possible adversarial model output is handled.
- It does not approve provider execution, live transport, production readiness, public usability, release-candidate readiness, or Production Candidate status.

### Common failure signals
- Assertion failures on rejected/accepted corpus cases.
- Panic output.
- Unexpected zero-test or partial-test filter output.
- Cargo build failures before corpus execution.

### What to inspect next
- The failing adversarial test case name.
- Phase 95.3 LLM-output adversarial corpus documentation.
- Rust modules containing adversarial corpus fixtures and assertions.

### What not to infer
- Do not infer provider safety from passing local adversarial tests.
- Do not infer that new corpus cases should be added in Phase 98; record any gap as deferred maintenance.

## UI behavioral test troubleshooting
Command:

```sh
cd ui && npm run test:api
```

### What it checks
- The existing UI API behavioral harness.
- Temporary TypeScript compilation of the API projection/fixture/read-model/submission-boundary behavior tests.
- Behavioral assertions for the current non-authoritative UI submission boundary.

### What a pass means
- The UI API behavioral tests compiled and passed in the local environment.
- The harness reported all expected behavior tests as passing and exited zero.

### What a pass does not mean
- It does not start live UI/Rust transport.
- It does not approve UI authority, public usability, provider execution, persistence, action execution, release-candidate readiness, production readiness, or Production Candidate status.

### Common failure signals
- `not ok` output from the behavior harness.
- TypeScript compile failures inside the temporary behavior-test output directory.
- A final `UI API behavior tests failed` message.
- Partial pass counts or masked failure output; if the command returns zero while printing failure output, classify that as a validation-gate defect and defer repair.

### What to inspect next
- `ui/run-api-behavior-tests.mjs` for harness behavior.
- `ui/src/api/submissionBoundary.behavior.test.ts` for the failing assertion name.
- Phase 95.1 UI behavioral harness documentation.
- Temporary output cleanup if an interrupted run leaves local artifacts.

### What not to infer
- Do not infer browser runtime coverage beyond this harness.
- Do not infer authority transfer from UI to Rust.
- Do not infer that npm warnings alone are behavior failures when the command exits zero and test output is clean.

## UI typecheck/lint/build troubleshooting
Command:

```sh
cd ui && npm run typecheck && npm run lint && npm run build
```

### What it checks
- UI TypeScript typechecking.
- The existing UI lint script.
- The existing UI build script.

### What a pass means
- TypeScript typechecking completed successfully.
- The configured UI lint and build scripts exited zero for the local checkout.

### What a pass does not mean
- It does not prove UI behavioral correctness; use `npm run test:api` for the wired behavioral harness.
- It does not approve live transport, UI authority, packaging, distribution, public usability, release-candidate readiness, production readiness, or Production Candidate status.

### Common failure signals
- TypeScript `TS` errors.
- Nonzero lint output.
- Nonzero build output.
- Node/npm environment errors.
- Node build artifacts or coverage output left in the working tree.

### What to inspect next
- `ui/package.json` scripts for the current typecheck/lint/build wiring.
- TypeScript source paths named in the first error.
- `git status --short` for generated artifacts after the run.

### What not to infer
- Do not infer package readiness or distribution approval from a UI build pass.
- Do not infer that placeholder lint/build scripts are stronger than their actual configured command text.

## Local build artifact troubleshooting
Commands:

```sh
cargo build --manifest-path core/Cargo.toml
cd ui && npm run build
```

### What it checks
- The existing Rust local build path for the core crate.
- The existing UI build script.
- The Phase 97 packaging-boundary command surfaces.

### What a pass means
- The local build commands completed successfully for the current checkout and environment.
- Existing local build artifacts, if any, were produced only under existing build behavior.

### What a pass does not mean
- It does not create a release artifact, installer, signed artifact, update channel, public download, package registry publication, or distribution approval.
- It does not approve packaging, public usability, release-candidate readiness, production readiness, or Production Candidate status.

### Common failure signals
- Cargo compiler errors.
- UI build script nonzero exit.
- Generated `target` metadata drift or UI build artifact drift in tracked paths.
- Package/lockfile drift after npm commands.

### What to inspect next
- Phase 97 packaging boundary documentation.
- `core/Cargo.toml` for Rust package build configuration.
- `ui/package.json` for UI build command text.
- `git status --short` for generated artifact drift.

### What not to infer
- Do not infer installer behavior.
- Do not infer distribution behavior.
- Do not infer signing or release engineering readiness.

## Generated artifact drift cleanup
Generated artifact drift is local tool output that should not be committed for Phase 98.

Examples include:

- `core/target/.rustc_info.json` or other Cargo-generated metadata drift.
- UI build artifacts.
- Node build artifacts.
- Coverage output.
- Test temp files.
- Export temp files.
- `scripts/__pycache__`.
- Incidental formatter, package, lockfile, or unrelated tool output.

Operator cleanup model:

1. Run `git status --short` before edits and after validation.
2. Classify every uncommitted path as intended documentation, generated artifact drift, or unrelated change.
3. Remove or revert generated artifact drift before staging.
4. Do not remove source files or user-authored changes without confirming they are generated and unrelated to Phase 98.
5. Re-run `git status --short` and confirm only allowed Phase 98 surfaces remain.

## Frontmatter and checklist validation failures
Frontmatter failures usually mean a markdown file is missing or mismatches the expected metadata block. For Phase 98, the operations document frontmatter is:

```yaml
---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---
```

Checklist failures usually mean `checklists/current-phase.md` does not match the active phase, includes stale unchecked boxes for completed work, or overclaims evidence not present in the final diff.

Inspect:

- The frontmatter block in the failing markdown file.
- `checklists/current-phase.md` phase name, goal, task checklist, validation checklist, findings, deferred items, and validation log.
- `CHANGELOG.md` for historical claims that must match the actual final diff.

Do not weaken validation scripts in Phase 98.

## Boundary lint failures
Boundary lint failures are blocking when they come from the existing Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, Rust tests, or `./scripts/check.sh`.

Explicit boundary lint commands:

```sh
node scripts/test_rust_boundary_lint.mjs
node scripts/rust_boundary_lint.mjs
node scripts/test_lint_ui_boundaries.mjs
node scripts/lint_ui_boundaries.mjs
```

Inspect:

- The exact path and pattern reported by the lint.
- Whether the finding is in an allowed Phase 98 documentation surface or an unmodified source surface.
- Whether the finding is a new source/tooling pattern that should be prohibited later.

If Phase 98 finds a new source/tooling pattern that should be prohibited later, do not patch it inside Phase 98. Document it as a deferred maintenance item. If lint behavior must change, add or update lint self-tests in a separate maintenance phase, not Phase 98.

## Non-failing npm warning posture
npm may print non-failing environment or dependency warnings while returning zero. Examples include `http-proxy` warning noise during dependency checks.

Operator posture:

- Record the warning if it appears in validation evidence.
- Treat it as non-blocking only when the command exits zero and no behavior failure, TypeScript error, lint error, assertion failure, panic, traceback, failed assertion, partial pass count, or masked failure appears.
- Do not infer that a warning-free npm run approves public usability, packaging, distribution, release-candidate readiness, production readiness, or Production Candidate status.
- Do not patch dependencies, package files, or lockfiles in Phase 98.

## What passing checks do not mean
Passing checks do not mean:

- Production readiness.
- Release-candidate readiness.
- Public usability.
- Startup approval.
- Package approval.
- Distribution approval.
- Installer approval.
- Production Candidate status.
- Provider execution approval.
- Persistence authority.
- Replay repair approval.
- Recovery promotion approval.
- Export behavior approval.
- Action execution approval.
- Transport behavior approval.
- Signing approval.
- Release engineering approval.

## What operators should not do
Operators should not:

- Add or rename commands for Phase 98.
- Change runtime behavior.
- Change Rust source, TypeScript source, tests, scripts, workflows, schemas, governance docs, architecture docs, README, AGENTS, package files, dependency files, lockfiles, UI config, or roadmap files.
- Add packaging, installer, distribution, signing, or release engineering.
- Add transport, provider, persistence, replay, recovery, export, or action behavior.
- Treat `rg` scans as enforcement.
- Treat passing commands as readiness claims.
- Patch broken gates inside Phase 98; document and defer the repair.

## Relationship to Phase 96 local startup boundary
Phase 96 selected the existing dry-run command as the local startup boundary:

```sh
cargo run --manifest-path core/Cargo.toml -- dry-run
```

Phase 98 documents how to interpret and troubleshoot that command. It does not approve startup, public usability, packaging, release-candidate readiness, production readiness, or Production Candidate status.

## Relationship to Phase 97 packaging boundary
Phase 97 documented existing local build artifact boundaries using existing commands:

```sh
cargo build --manifest-path core/Cargo.toml
cd ui && npm run build
```

Phase 98 documents how to interpret and troubleshoot those commands. It does not create package artifacts, installer artifacts, signed artifacts, release artifacts, update channels, registry publishing, public downloads, or distribution approval.

## Relationship to Phase 99 release engineering dry run
Phase 99 remains responsible for release engineering dry-run boundary work if Phase 98 evidence permits it.

Phase 98 does not start Phase 99. Phase 98 does not add release engineering, signing, publishing, installer, distribution, release-candidate readiness, production readiness, or Production Candidate approval.

## AST/boundary lint parity
Do not rely on `rg` scans as enforcement. `rg` scans are discovery/evidence only.

Blocking enforcement must come from `./scripts/check.sh`, Rust boundary lint, UI AST lint, compiler/type checks, clippy, UI behavioral tests, and Rust tests.

If Phase 98 finds a new source/tooling pattern that should be prohibited later, do not patch it inside Phase 98. Document it as a deferred maintenance item. If lint behavior must change, add or update lint self-tests in a separate maintenance phase, not Phase 98.

## Test fidelity
Phase 98 is documentation-only, so no new Rust or TypeScript tests are expected.

Full existing test/lint/check suite must pass after final documentation edits. If tests are skipped after final edits, Phase 98 is not complete.

If any validation command prints failure output while still returning zero, classify that as a validation-gate defect and document the required follow-up. Do not repair it in Phase 98.

## Validation evidence
Phase 98 validation evidence is expected from the existing commands only:

```sh
./scripts/check.sh
cargo test --manifest-path core/Cargo.toml --all-targets
cargo test --manifest-path core/Cargo.toml golden --all-targets
cargo test --manifest-path core/Cargo.toml adversarial --all-targets
cd ui && npm run test:api
node scripts/test_rust_boundary_lint.mjs
node scripts/rust_boundary_lint.mjs
node scripts/test_lint_ui_boundaries.mjs
node scripts/lint_ui_boundaries.mjs
cd ui && npm run typecheck && npm run lint && npm run build
cargo run --manifest-path core/Cargo.toml -- dry-run
cargo build --manifest-path core/Cargo.toml
cd ui && npm run build
```

## Confirmed vs suspected
Confirmed findings are supported by command output, file content, or the final diff.

Suspected findings are not closure evidence. If suspected behavior needs repair, additional lint coverage, new tests, or command wiring, defer it to a later maintenance phase.

## Deferred items
Deferred items for Phase 98 are documentation-only observations that require later work outside this phase. Phase 98 does not implement deferred repairs.

Potential deferred categories:

| Category | Phase 98 action |
| --- | --- |
| Broken validation gate | Document the signal and defer repair. |
| Masked failure returning zero | Classify as validation-gate defect and defer repair. |
| Missing lint pattern | Document as maintenance follow-up; do not patch lint. |
| New test needed | Document as follow-up; do not add tests in Phase 98. |
| Dependency/package warning | Record if observed; do not change package files in Phase 98. |

## Non-readiness statement
Phase 98 is documentation-only and operator-facing only. Phase 98 introduces no new capability, no runtime behavior, no new CLI surface, no packaging, no installer, no distribution, no signing, no release engineering, no readiness claims, no authority surfaces, no transport, no provider, no persistence, no replay, no recovery, no export, and no action behavior.

Phase 98 does not approve startup, packaging, public usability, release-candidate readiness, production readiness, Production Candidate status, provider execution, persistence authority, replay repair, recovery promotion, export behavior, action execution, installer use, distribution, signing, or release engineering.
