---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Maintainer Guide

[Documentation home](README.md) | [Developer checks](developer-guide.md) | [Capabilities and limits](capabilities-and-limits.md)

This guide is for the person maintaining `irgordon/ajentic`: reviewing changes,
keeping validation useful, updating dependencies deliberately, and preserving
release evidence. It explains existing policy; the normative rules remain in
[governance](../governance/GOVERNANCE.md).

## Authenticate as the Intended Account

The repository Owner is `irgordon`. Check the active GitHub CLI account without
printing a token:

```sh
gh auth status --hostname github.com
gh api user --jq .login
```

If that account is already stored, select it with
`gh auth switch --hostname github.com --user irgordon`. Otherwise use
`gh auth login --hostname github.com --git-protocol https --web` and complete
the browser sign-in yourself. An app connector, browser session, CLI, and Git
credential helper can use different accounts; verify the tool actually doing
the operation. Never paste tokens into chat, logs, issues, or repository files.

## Routine Maintenance

Review open PRs and Actions results, reproduce concrete failures, and keep the
documented user paths aligned with the actual interface. Check that new features
have a usable entry point, not merely a module or a status projection.

For changes to dependencies, schemas, workflows, or authority-bearing Rust,
review the negative cases as well as the happy path. Optional human review is
especially valuable for high-risk changes even though it is not a required gate.

Useful read-only commands:

```sh
gh pr list --repo irgordon/ajentic
gh run list --repo irgordon/ajentic --limit 10
git status --short --branch
```

## Merge a Pull Request

The standing solo-maintainer policy retains PRs and CI without requiring a
second person to approve the Owner's work. It is not an administrator bypass.

1. Confirm the PR targets `main` and contains only the intended scope.
2. Review the diff, its assumptions, failure cases, and validation evidence.
3. Verify checks belong to the current head SHA, not an older commit.
4. Confirm all nine required checks passed; do not merge pending or failed work.
5. Confirm the branch is up to date with `main` and conversations are resolved.
6. Use a normal squash or rebase merge, consistent with linear history.
7. Fetch and fast-forward local `main`; verify the merge result and working tree.

Required check names:

| Check | Principal Coverage |
| --- | --- |
| Repository validation | Node policy, placement/docs, contract map, Rust boundary lint |
| Rust | Formatting, build, tests, warnings-denied Clippy |
| TypeScript UI | Typecheck, boundary lint, build, API behavior tests |
| Node 24 compatibility | UI, contracts, and license inventory on the second supported Node line |
| Schemas | Ajv2020 compilation/fixtures and reviewed dependency licenses |
| Scripts | actionlint, infrastructure regressions, and shell parsing |
| Full deterministic check | Clean-tree aggregate plus artifact/evidence and repository checks |
| Internal candidate artifact reproducibility | Independent internal two-build artifact and evidence checks |
| Memory Placement Gate | Placement rules for repository and memory content |

The aggregate explicitly rejects failed, skipped, cancelled, missing, or unknown
prerequisite results. A skipped job is not adequate evidence of successful work.
Inspect each check result, not just a green-looking overall label.

For CLI-assisted review:

```sh
gh pr view NUMBER --repo irgordon/ajentic
gh pr diff NUMBER --repo irgordon/ajentic
gh pr checks NUMBER --repo irgordon/ajentic
```

Replace `NUMBER` with the actual PR number. Once all gates pass, one normal
linear-history-compatible choice is:

```sh
gh pr merge NUMBER --repo irgordon/ajentic --squash --match-head-commit FULL_HEAD_SHA
```

Replace `FULL_HEAD_SHA` with that PR's verified current head. Do not add `--admin`
to work around a failing check, and do not force-push a protected branch.

After a confirmed merge, with unrelated local edits preserved:

```sh
git fetch origin
git switch main
git pull --ff-only origin main
git log --oneline -5
git status --short --branch
```

## Verify Protection Without Changing It

```sh
gh api repos/irgordon/ajentic/branches/main/protection
gh api repos/irgordon/ajentic/rulesets
```

Check that strict/up-to-date required statuses are tied to GitHub Actions app
15368, administrator enforcement is enabled, and required approving reviews are
zero. Code-owner/latest-push approval requirements are off; conversations and
linear history are required; force-pushes and deletion of `main` are disabled.

Rulesets are a separate enforcement surface: do not assume a disabled or empty
ruleset means classic protection is absent, or that classic settings describe
every active rule. If a second-person gate reappears, inspect the actual rules
and obtain explicit Owner instruction before changing policy. Do not solve it
by deleting protection. [ADR 002](../architecture/adr-002-solo-maintainer-ci-policy.md)
records the rationale and enforcement limits.

## Handle a Failed Check

Open the failing job, retain its run URL and source SHA, and identify the first
concrete failing step. Reproduce that command locally where possible. Fix the
defect on the PR branch and rerun CI; do not silently weaken a lint or schema
rule to make the status green.

Use `gh run view RUN_ID --repo irgordon/ajentic --log-failed` for a failed run's
logs. A timeout is incomplete validation, not evidence that the product is
incorrect or correct. Distinguish registry outages, runner/toolchain changes,
environmental metadata, and genuine repository defects in the report.

Do not dispatch `rc-publication` or `final-release` to investigate an ordinary
CI failure. Their publication permissions and effects are different.

## Review Dependencies and Toolchain Changes

Check manifest and lockfile together, retained third-party notices, transitive
changes, supported runtimes, and actual clean-runner behavior. Keep `private:
true` on the UI package; an MIT license does not authorize registry publication.

The license gate reviews seven locked development packages against shipped
license files. `fast-uri@3.0.1` declares MIT in metadata but ships a BSD-3-Clause
notice; TypeScript uses Apache-2.0. Preserve the notices and update the reviewed
inventory deliberately when the locked selection changes. Do not regenerate
license hashes merely to silence an unexplained failure.

It also verifies the locally bundled Inter/Courier Prime fonts, Lucide icons,
and retained notices against `ui/public/third-party-assets.json`. These assets
have their own OFL/ISC/MIT terms; they do not become MIT-only because the project
is MIT-licensed. The package-oriented SBOM is not a substitute for that asset
inventory. Builds copy reviewed assets without remote font loading.

There is no dedicated vulnerability-audit gate. Install-time `--no-audit` is a
network/performance choice, not a security review. Assess current advisories
separately and record findings and compatibility tradeoffs rather than implying
the license check performed that work.

Rust stable, runner images, and third-party Action tags can change independently
of the repository. Review new diagnostics, record timing evidence for provisional
timeouts, and test both supported Node lines when changing their floors. Do not
assume a checksum-pinned actionlint binary means every Action is SHA-pinned.

## Preserve Release Evidence

The existing final v1.0.0 release is already published. Its tag targets
`6734a5b3cc223c41288b14575ad40f6fcf23fb6f`; `v1.0.0-rc.1` remains its source RC.
Use read-only evidence checks, not a redispatch:

```sh
git ls-remote --tags origin
gh api repos/irgordon/ajentic/releases/tags/v1.0.0
gh api repos/irgordon/ajentic/actions/runs/27307966684
```

Preserve the [final workflow](https://github.com/irgordon/ajentic/actions/runs/27307966684),
[release](https://github.com/irgordon/ajentic/releases/tag/v1.0.0),
[attestation](https://github.com/irgordon/ajentic/attestations/30684159), and
[closure record](../operations/post-v1-release-closure-phase-195.md).
For annotated tags, distinguish a tag object from its peeled commit target.

The existing final-release workflow is bounded to v1.0.0 and the original RC;
it is not a generic v1.0.1 publisher. The RC workflow is likewise bounded. Do not
reuse their confirmation values as permission for a new release, manually move
tags, or recreate an existing release. A future release needs an explicitly
authorized scope, reviewed version strategy, correctly bounded workflow, and
separately verified evidence. This handbook grants none of that authorization.

The release tag and `core`/`ui` package versions are separate identity surfaces;
the package manifests currently retain 0.1.0. Do not silently normalize them
to match a tag. Record any authorized version strategy before making a change.

## Keep the Documentation Honest

Maintain one audience-oriented entry point in this wiki and keep the root README
compact. Recheck exact labels and expected outcomes when the interface changes.
Explain technical terms for users; keep implementation detail in developer and
maintainer references. Record actual commands, skipped/unrun checks, and limits.

The basic simulation is supported as a local example. The combined provider
exercise currently rejects its materialization step with
`provider_execution_result_id_mismatch`; keep this visible until a scoped fix
is implemented and verified. Other reviewed outcome/claim authority gaps are
not closed merely because CI passes. See [capabilities and limits](capabilities-and-limits.md).

Follow the active checklist and changelog boundaries. Do not make historical
phase documents into a parallel task tracker or invent new governance-only work
in place of fixing functional problems.
