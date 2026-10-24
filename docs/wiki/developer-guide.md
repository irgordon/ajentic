---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Developer Guide

[Documentation home](README.md) | [Installation](installation.md) | [Maintainer guide](maintainer-guide.md)

This guide is for contributors changing code, contracts, tests, or documentation.
For a tour of the current product surface, complete the
[first sample review](getting-started.md#your-first-review) first.

## Set Up a Development Checkout

Follow [Installation](installation.md) for the supported Node versions, locked
npm installation, Rust/Python requirements, and local server. Then create a
branch from an up-to-date, clean `main`:

```sh
git fetch origin
git switch main
git pull --ff-only origin main
git switch -c mothra/describe-your-change
```

Do not discard unrelated local edits to make this sequence work. Finish or
preserve them deliberately before switching branches. The `mothra/` prefix is
the current agent-work branch convention; choose a descriptive suffix.

Run `npm run dev --prefix ui` to inspect the local UI. It compiles at startup,
not continuously: restart after edits. This command does not start a live Rust
API server. The Rust CLI's documented user path is the bounded `dry-run` command.

## Repository Map

| Path | Responsibility | Good Starting Point |
| --- | --- | --- |
| `core/src/api/` | Typed Rust request, authorization, and local workflow boundaries | [Local workflow](../../core/src/api/local_workflow.rs), [authorization](../../core/src/api/authorization.rs) |
| `core/src/authority/`, `verification/`, `validation/`, `policy/` | Bound evidence and verifier/decision contracts | [Authority](../../core/src/authority/mod.rs), [verification](../../core/src/verification/mod.rs) |
| `core/src/task/`, `outcome/` | Task criteria, observations, and derived result contracts | [Task](../../core/src/task/mod.rs), [outcome](../../core/src/outcome/mod.rs) |
| `core/src/execution/`, `state/`, `context/`, `memory/` | Controlled execution and runtime state/data boundaries | [Execution](../../core/src/execution/mod.rs) |
| `core/src/ledger/`, `replay/`, `audit/` | Event recording, replay verification, and audit projections | [Ledger](../../core/src/ledger/mod.rs) |
| `ui/src/main.ts` | Local request orchestration and delegated control wiring | [Entry point](../../ui/src/main.ts) |
| `ui/src/app/reviewWorkspace.ts`, `reviewContent.ts`, `reviewModel.ts` | File-first view composition, presentation preferences, and illustrative comparison data | [Workspace](../../ui/src/app/reviewWorkspace.ts), [content](../../ui/src/app/reviewContent.ts) |
| `ui/src/app/reviewDiagnostics.ts`, `reviewDiagnosticViews.ts` | Existing raw test/evidence panels behind View / Developer Details | [Diagnostics](../../ui/src/app/reviewDiagnostics.ts) |
| `ui/public/` | Locally bundled fonts, icons, license notices, and reviewed hashes | [Asset inventory](../../ui/public/third-party-assets.json) |
| `ui/src/api/` | Local browser transport, simulated data, and projections | [Transport](../../ui/src/api/localOperatorShellTransport.ts), [local shell](../../ui/src/api/localOperatorShell.ts) |
| `ui/src/display/`, `ui/help/` | Friendly status wording and bundled help | [Status copy](../../ui/src/display/statusCopy.ts) |
| `schemas/` | Shared JSON data contracts | [Schema coverage](../../tests/schema-coverage.json) |
| `tests/` | Root integration and adversarial regressions | [Outcome binding tests](../../tests/outcome_evidence_binding.rs) |
| `scripts/` | Local checks and artifact/evidence tooling | [Full check](../../scripts/check.sh) |
| `.github/workflows/` | Independent CI and separately gated publication paths | [CI](../../.github/workflows/ci.yml) |
| `docs/wiki/` | Audience-oriented usage and contributor guidance | This guide and [documentation home](README.md) |

The [architecture anchor](../architecture/ARCHITECTURE.md) describes the system
structure and targets. Check current source and tests before treating a target
as an integrated feature. Historical phase records explain decisions but are
not a replacement for current behavior.

## Preserve the Ownership Boundaries

Consult [governance](../governance/GOVERNANCE.md) and [AGENTS.md](../../AGENTS.md)
before making changes. The important division is:

- Rust owns authoritative runtime decisions, state, policy, validation, and
  execution boundaries.
- TypeScript displays projections and records typed local intent. Its current
  transport is in-memory simulation, not live Rust RPC.
- Python adapts/supports tooling; Bash wraps operator commands. Neither becomes
  a second runtime policy engine.
- Schemas describe shared shapes. Shape validation does not establish external
  factual truth or mint positive runtime authority.

The Single Level of Abstraction Principle (SLAP) is the repository's structural
discipline: orchestration describes steps, domain logic makes decisions, and
infrastructure wraps technical operations. Read the applicable instructions for
the exact layer and size constraints rather than copying unrelated patterns.

## Make a Focused Change

### Browser wording, layout, or local behavior

Find the control in `ui/src/main.ts`, follow its transport call, and inspect the
projection it renders. Update the friendly status mapping when adding a raw
status. Preserve visibility of unknown, failed, rejected, and missing states.
Test both a permitted interaction and a rejected or incomplete input.

View preferences and review notes are in-memory presentation state only. Notes
are not submitted as evidence or written to a file. The Before/After sample is
pre-recorded illustrative data, not a diff computed from provider output. Keep
that distinction visible. Existing test controls retain their action IDs and
transport calls behind Developer Details.

Do not add fetch calls, credentials, publishing controls, or computed release
authority merely to make a local demonstration look complete. A label such as
"Approve" must not silently become execution authorization.

### Rust result or authority logic

Start from the typed API and the module that owns the decision. Keep receipts
opaque and bound to the relevant task/run/candidate/evidence. Add focused tests
for substituted IDs, missing evidence, cross-run reuse, errors, uncertainty,
partial side effects, and rejected requests as relevant to the change.

Existing examples include [receipt binding](../../tests/receipt_binding.rs),
[verifier integrity](../../tests/verifier_receipt_integrity.rs), and
[outcome evidence binding](../../tests/outcome_evidence_binding.rs). These tests
are starting points, not a claim that all reviewed correctness gaps are closed.

### Shared contract changes

Inspect the schema, corresponding Rust types, and TypeScript projections together.
Update the contract through its declared mutation path; align the existing
operator-intent map when that surface changes. Add valid and invalid payload
cases, and update `tests/schema-coverage.json` for any new schema. A new schema
without a coverage entry fails the gate; compile-only coverage is explicit debt.

Do not hand-edit generated bindings or describe hand-written projections as
authoritative generated truth. The boundary lint and contract-map checks detect
some drift but do not prove full schema/Rust/TypeScript semantic equivalence.

### Documentation changes

Keep the README short. Put user instructions and contributor guides in this
wiki with advisory orientation frontmatter. Keep normative rules in governance,
structural decisions in architecture/ADRs, active tasks in the current checklist,
and accepted history in the changelog. The wiki explains those sources; it does
not redefine them. Use relative links and define technical terms for users.

## Run Focused Checks While Editing

From the repository root:

```sh
npm run typecheck --prefix ui
npm run lint --prefix ui
npm run build --prefix ui
npm run test:api --prefix ui
cargo fmt --manifest-path core/Cargo.toml -- --check
cargo test --manifest-path core/Cargo.toml --all-targets
cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings
```

UI lint is the boundary AST linter and its self-tests, not a general formatter.
Typechecking is separate. `npm run build` produces `ui/dist`; that is not a
publishing action. Rust integration targets are declared in `core/Cargo.toml`:

```sh
cargo test --manifest-path core/Cargo.toml --test outcome_evidence_binding
```

For docs, tooling, and contracts:

```sh
python3 scripts/validate_structure.py
python3 scripts/validate_docs.py
python3 scripts/check_help_pages.py
python3 -m unittest discover -s scripts -p 'test_validation_*.py'
node scripts/test_validate_schemas.mjs
node scripts/validate_schemas.mjs
node scripts/test_audit_dependency_licenses.mjs
node scripts/audit_dependency_licenses.mjs
git diff --check
```

When workflows change, run `actionlint` at the repository root. CI installs
actionlint 1.7.12 with a pinned archive checksum. Its condition regression tests
protect the publication workflow's whitespace-sensitive `if: >-` expressions.

## Run Full Validation From a Clean Commit

Review and commit your intended changes on the branch first. The wrapper checks
tracked working-tree/index cleanliness before and after validation; untracked
files still require your own review.

```sh
git status --short
CARGO_TARGET_DIR=/tmp/ajentic-check-target ./scripts/check.sh
```

It runs language checks, boundary/contract checks, artifact reproducibility,
integrity evidence, bundle/preflight checks, UI tests, and Rust checks. Expect
`All checks passed.` only at the end. A timeout or incomplete command is not a
pass. Artifact steps require registry access for uncached dependencies.

Individual internal evidence checks are available without dispatching publication:

```sh
python3 scripts/reproducible_artifacts.py --check
python3 scripts/reproducible_artifacts.py --check-evidence
```

The asset-copy helper copies only entries in `ui/public/third-party-assets.json`
and the inventory itself. The license audit verifies all bundled asset and notice
hashes as well as npm dependencies. Finder metadata and `.gitkeep` are not bundled.
Keep upstream font/icon sources and license notices when updating these assets.

## Toolchain and Validation Limits

CI builds with Node 22.23.4 and checks compatibility with 24.22.1. Explicit Python
jobs use 3.12; local tooling requires at least 3.11. Rust follows stable, so new
Clippy diagnostics may appear even when an older local compiler passed.
Third-party Action references still use major/stable tags rather than all being
SHA-pinned. Two-build reproducibility is scoped to its configured environment.

Artifact npm installation has a provisional 90-second deadline; build subprocesses
have 600 seconds. Fetches use 15 seconds with one retry. Record actual cold-runner
timings before treating those values as proven capacity limits.

Schema compilation covers all 22 schemas; payload cases cover 5, with 17 declared
compile-only gaps. The development-license inventory is not a vulnerability scan.
There is no dedicated dependency vulnerability-audit gate. Read
[ADR 001](../architecture/adr-001-independent-validation.md) for the rationale,
pins, shipped-license discrepancy, and remaining assurance gaps.

## Submit the Contribution

Open a PR to `main` with the problem, scoped change, test commands/results, and
known limitations. Do not claim that a projection, UI walkthrough, or successful
unit test proves independently verified task completion.

All nine required Actions checks must pass on an up-to-date branch. Optional
reviews are welcome, but the solo-maintainer policy requires zero approvals.
Administrators follow the same CI gate. Use the
[maintainer merge procedure](maintainer-guide.md#merge-a-pull-request) and do not
disable protection or use an admin bypass to land ordinary work.
