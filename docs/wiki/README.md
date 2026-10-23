---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# AJENTIC Wiki

Set up AJENTIC from source, explore the local browser interface, and run the
project checks. Start with the browser if you want to look around; the full
validation suite is for contributors checking repository changes.

## Choose a Path

- [Install and open the browser interface](#install-and-open-the-browser-interface)
- [Run the Rust CLI dry run](#run-the-rust-cli-dry-run)
- [Validate a contribution](#validate-a-contribution)
- [Troubleshoot setup](#troubleshooting)
- [Understand the release and its limits](#release-and-runtime-boundaries)
- [Find architecture, workflow, and license details](#further-reading)

## Prerequisites

Run the commands below in a terminal. Installation downloads locked development
dependencies, so an initial setup needs network access.

| Tool | Required for | Supported setup |
| --- | --- | --- |
| Git | Cloning the source | A working `git` command |
| Node.js and npm | Browser interface and schema checks | Node 22.23.4 or newer within 22.x, or 24.22.1 or newer within 24.x |
| Rust and Cargo | CLI and full validation | A toolchain with `rustfmt` and `clippy` |
| Python | Full validation | Python 3.11 or newer, available as `python3` |
| Bash | Full validation | Available to run `scripts/check.sh` |
| actionlint | Workflow changes | Version 1.7.12; Actions installs a checksum-verified binary |

Install a supported LTS line from the [official Node.js downloads](https://nodejs.org/en/download)
for your operating system, including npm. For the Rust path, follow the
[official rustup setup](https://rust-lang.org/tools/install/). For full validation,
install Python from [Python.org](https://www.python.org/downloads/) if `python3`
is not already available. Follow each installer's platform instructions and
reopen your terminal afterward so the installed commands are on `PATH`.

The Node preflight rejects older patches, prereleases, and other major lines.
For example, Node 26 is not a supported project runtime even though its version
number is higher. Select a supported Node version in your shell before running npm.

Check your browser prerequisites:

```sh
git --version
node --version
npm --version
```

For Rust and full validation, also check:

```sh
cargo --version
rustc --version
python3 --version
bash --version
```

If you manage Rust with rustup, add the formatting and linting components:

```sh
rustup component add rustfmt clippy
```

GitHub Actions pins its build runtime to Node 22.23.4 and separately tests Node
24.22.1. Explicit Python jobs use 3.12. The Rust CI toolchain follows stable;
it is not an exact compiler-version pin.

## Install and Open the Browser Interface

### 1. Clone the repository

```sh
git clone https://github.com/irgordon/ajentic.git
cd ajentic
```

These commands use current `main`, which includes post-v1 maintenance changes.
For the published v1.0.0 snapshot, check out the existing `v1.0.0` tag in a clean
clone and use the instructions included in that snapshot. Checking out an
existing tag does not create a new release.

### 2. Install locked dependencies

From the repository root:

```sh
npm ci --prefix ui --no-audit --no-fund --fetch-retries=1 --fetch-timeout=15000
```

`npm ci` uses `ui/package-lock.json` and installs into `ui/node_modules`.
Use it instead of changing the dependency versions just to get started.
The `--no-audit` flag skips npm's install-time audit request; it does not claim
the dependencies are free of vulnerabilities.

### 3. Start the local interface

```sh
npm run dev --prefix ui
```

Wait for compilation to finish, then open
[http://127.0.0.1:5173](http://127.0.0.1:5173) in your browser.
The development server binds to the loopback address on port 5173; it is not a
deployment service. Leave the terminal running while using the interface.
Press `Ctrl+C` in that terminal to stop it.

The page is titled **AJENTIC Local Operator Shell**. Explore its local review
surfaces and help to inspect status, evidence, and recorded errors. Local shell
interactions are not live Rust backend execution or release approval. You do
not need provider credentials merely to open this interface.

The server compiles into `/tmp/ajentic-ui-dev` at startup. After editing source
files, stop and restart it to recompile; it does not provide a hot-reload workflow.

## Run the Rust CLI Dry Run

From the repository root, with Rust installed:

```sh
CARGO_TARGET_DIR=/tmp/ajentic-check-target cargo run --manifest-path core/Cargo.toml -- dry-run
```

The dry run prints a non-executing CLI summary. It does not call a model provider,
execute actions, write persistent runtime state, or start a live UI/Rust connection.
See the [local workflow guide](../operations/local-workflow-guide.md) for the
supported command boundaries.

## Validate a Contribution

Install dependencies first. The full check requires a clean tracked working tree
and index, so commit your changes on a branch before running it.

```sh
git status --short
CARGO_TARGET_DIR=/tmp/ajentic-check-target ./scripts/check.sh
```

Expect explicit progress and a final `All checks passed.` on success. A nonzero
exit status means validation did not complete successfully; inspect the failing
step instead of treating earlier passing steps as a full pass. Artifact checks
perform repeated builds and may download dependencies.

### Run focused checks

All commands below run from the repository root:

```sh
npm run typecheck --prefix ui
npm run lint --prefix ui
npm run build --prefix ui
npm run test:api --prefix ui
cargo fmt --manifest-path core/Cargo.toml -- --check
cargo test --manifest-path core/Cargo.toml --all-targets
cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings
python3 scripts/validate_structure.py
python3 scripts/validate_docs.py
python3 scripts/check_help_pages.py
git diff --check
```

`npm run lint` runs the browser boundary AST linter and its self-tests. It checks
boundary rules, not formatting or general style; typechecking is a separate step.
`npm run build` produces local files in `ui/dist`; it does not publish them.

Schema and development-license checks can also run independently:

```sh
node scripts/test_validate_schemas.mjs
node scripts/validate_schemas.mjs
node scripts/audit_dependency_licenses.mjs
```

The schema validator uses Ajv2020 for Draft 2020-12. It compiles every schema and
tests valid/invalid payloads where fixtures exist. The coverage manifest at
[`tests/schema-coverage.json`](../../tests/schema-coverage.json) distinguishes
5 payload-tested schemas from 17 compile-only schemas. Contract validation does
not independently establish the factual truth of observations.

For internal artifact and integrity-evidence checks:

```sh
python3 scripts/reproducible_artifacts.py --check
python3 scripts/reproducible_artifacts.py --check-evidence
```

Artifact dependency installation is bounded to 90 seconds and individual build
commands to 10 minutes. Dependency fetches use a 15-second timeout with one retry.
These limits are provisional, not guarantees that every cold runner will finish
in time. A timeout fails validation with a diagnostic.

When changing workflows, run `actionlint` from the repository root. Actions runs
actionlint 1.7.12 against workflows and embedded shell scripts and checks its
download against a pinned SHA-256. Publication workflows are not dispatched by
ordinary validation.

### Open a pull request

Use a branch and a PR to `main`. All nine required Actions checks must pass on an
up-to-date branch, including for administrators. The solo-maintainer policy
requires zero approving reviews; optional reviews are welcome. Resolve review
conversations and use squash or rebase merging. Do not disable checks or use an
admin bypass to complete an ordinary merge.

The standing policy is in [governance](../governance/GOVERNANCE.md#solo-maintainer-merge-policy).

## Troubleshooting

| Symptom | What to check |
| --- | --- |
| Node preflight fails | Run `node --version` in the same terminal. Select a supported 22.x or 24.x patch; newer major lines are not automatically supported. |
| `npm ci` cannot retrieve dependencies | Check registry access and the reported timeout. Retry after connectivity is restored; do not delete or regenerate the lockfile as a workaround. |
| TypeScript or Ajv cannot be found | Run the locked dependency installation from the repository root before running builds or schema checks. |
| Port 5173 is already in use | Stop the existing process using that port before restarting this development server. Its current script uses a fixed port. |
| The page does not open | Wait for compilation, keep the server terminal open, and use `http://127.0.0.1:5173`, not a directly opened HTML source file. |
| Source edits do not appear | Restart `npm run dev --prefix ui` to compile the updated source, then refresh the browser. |
| Full validation rejects the working tree | Inspect `git status --short` and commit intended changes on your branch. Do not discard unrelated work merely to make the check pass. |
| Rust formatting or Clippy is missing | Install `rustfmt` and `clippy` for the active toolchain. |
| Clippy differs between local and CI | Compare compiler versions: CI follows stable and can introduce new diagnostics. Fix the concrete diagnostic rather than suppressing the gate. |

## Release and Runtime Boundaries

[v1.0.0 is published on GitHub Releases](https://github.com/irgordon/ajentic/releases/tag/v1.0.0).
Its review assets include the final candidate bundle, asset manifest, checksums,
SBOM evidence, provenance evidence, release notes, and a final README asset.
The source release candidate `v1.0.0-rc.1` remains intact.

This is a source/local workflow, not an npm or Cargo package installation or a
packaged app installer. There is no package-registry publication, automatic
update channel, deployment service, OS signing, or notarization path. A GitHub
Release is not a production-readiness or automatic production-approval claim.

AI output is untrusted input. A reviewable draft is not proof of completed work
or independently verified facts. Errors, partial side effects, missing evidence,
and uncertainty still matter. Rust owns runtime authority; browser projections,
scripts, release assets, workflows, and attestations are visibility or evidence
surfaces, not decision authority.

## Development-Tool Licenses

AJENTIC is MIT-licensed. Development tools have their own notices: Ajv 8.20.0,
ajv-formats 3.0.1, and actionlint are MIT-licensed; TypeScript uses Apache-2.0.
The shipped `fast-uri@3.0.1` LICENSE uses BSD-3-Clause terms despite MIT package
metadata. Preserve that notice rather than treating metadata as the complete
license review.

`node scripts/audit_dependency_licenses.mjs` verifies the reviewed inventory and
license-file hashes. It is separate from vulnerability scanning; no dedicated
dependency vulnerability-audit gate is configured. See
[ADR 001](../architecture/adr-001-independent-validation.md) for tool pins,
coverage limits, timeouts, and the reasons behind the validation choices.

## Further Reading

- [Local workflow guide](../operations/local-workflow-guide.md)
- [Browser interface architecture](../architecture/browser-ui.md)
- [System architecture](../architecture/ARCHITECTURE.md)
- [Post-v1 release closure evidence](../operations/post-v1-release-closure-phase-195.md)
- [Solo-maintainer CI policy rationale](../architecture/adr-002-solo-maintainer-ci-policy.md)
- [Project license](../../LICENSE)
- [Project overview](../../README.md)

This wiki is setup and orientation guidance, not a source of runtime authority.
