---
truth_dimension: structural
authority_level: authoritative
mutation_path: architecture_pr
---

# ADR 002: Solo-Maintainer CI-Gated Merges

Date: 2026-10-23

Status: Accepted by Owner instruction

## Context

PR #300 passed CI but required another approving reviewer. The sole maintainer
could not approve their own PR. The Owner temporarily removed protection and
squash-merged it as `fa1581f`, then selected a standing policy: retain required
CI without requiring another reviewer's approval.

## Decision

Use classic branch protection for `main`, with PRs, strict up-to-date checks,
administrator enforcement, resolved conversations, and linear history. Require
all nine current validation checks from the GitHub Actions app (ID 15368),
including Node 24 compatibility. The normative check list lives in
`docs/governance/GOVERNANCE.md`.

Set required approving reviews to zero; disable code-owner and latest-push
approval requirements. Keep force-pushes and deletion of `main` disabled.
Do not use an administrator bypass for the ordinary solo-maintainer merge path.

The authenticated protection API confirmed these settings on 2026-10-23.
Repository administrators can still change protection settings; applying checks
to administrators is not a claim that the Owner cannot reconfigure the repository.

## Why

CI can independently validate the committed source without pretending that a
second eligible human reviewer exists. PRs retain a reviewable diff, check
evidence, conversations, and a merge record. Applying protection to administrators
prevents ordinary owner merges from silently skipping those checks.

Keeping protection disabled would remove the validation gate. Requiring one
approval would recreate the self-approval deadlock. Routine admin bypass would
make the stated CI policy optional. None is the selected standard.

## Limits

Zero required reviews does not provide independent human review. Optional review
remains useful for high-risk changes. This policy does not alter Rust runtime
authority, authorize release execution, or resolve outstanding correctness
findings. Only explicit Owner instruction changes the standing merge policy.

## Verification

Inspect the authenticated API with:

```sh
gh api repos/irgordon/ajentic/branches/main/protection
```

Confirm nine required Actions checks, `strict: true`, administrator enforcement,
zero approving reviews, no code-owner/latest-push approval requirement, resolved
conversations, linear history, and disabled force-pushes/deletion. Verify the
actual PR checks and merge state before merging; documentation is not enforcement.

[GitHub branch-protection API](https://docs.github.com/en/rest/branches/branch-protection#update-branch-protection)
documents zero as the supported review count when reviewers are not required.
