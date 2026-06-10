---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 192 - Read-Only Browser UI/UX Clarity and Visual Polish

## Decision

Phase 192 makes the browser shell clearer for local release-candidate review.

The UI remains a read-only TypeScript review surface. Rust remains the owner of runtime authority, validation, state transitions, evidence derivation, and release authority.

## UI Scope

Phase 192 updates:

- Primary header copy.
- Harness and preparation status labels.
- Evidence summary cards.
- Evidence verification rows.
- Help and reference copy.
- Plain-English glossary entries.
- Semantic visual tokens.
- Focus, details, and responsive styling.

The UI uses plain labels first and keeps raw technical values in details or raw snapshot text.

## Required Plain Labels

- `Ready (simulated)` means the page is using predictable local test data instead of a live database.
- `Preparation blocked` means action is required before local release-candidate work can continue.
- `Available` means evidence exists.
- `Needs attention` means blocked/action needed.
- `Not started` means missing or not supplied yet.
- `Rejected` means explicitly rejected.
- `Unknown status` is neutral and never treated as success.

## Local-Only Boundary

The browser shell communicates that it is local, safe for offline testing, simulated with predictable data, and has no cloud contact.

It does not publish, sign, deploy, create public artifacts, change credentials, or approve releases.

Ready or completed local checks do not mean production readiness, public-release readiness, or final v1 approval.

## Visual Tokens

The UI typography uses local system fonts only:

- UI font: `system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif`
- Code font: `"SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace`

No external fonts, font CDNs, Tailwind, or new heavy UI dependencies are added.

Semantic colors distinguish local ready/available, blocked/action-needed, missing/not-started, rejected/error, and neutral/slate states. Statuses include words and icons; color is not the only signal.

## Accessibility

Details and status explanations are keyboard/click/tap accessible and are not hover-only.

Blocked, missing, rejected, unknown, malformed, and failed states must remain visible in the primary UI.

The shell keeps visible focus styling and responsive layouts for desktop, tablet, and narrow browser widths.

## Non-Release Statement

Phase 192 does not create Git tags.

Phase 192 does not create GitHub Releases.

Phase 192 does not publish packages.

Phase 192 does not publish public artifacts.

Phase 192 does not create installers.

Phase 192 does not create update channels.

Phase 192 does not deploy.

Phase 192 does not activate signing.

Phase 192 does not change package versions.

Phase 192 does not change backend authority or release workflow mechanics.

## Workflow Boundary

Existing workflows remain validation/internal-candidate workflows.

The Phase 191 attestation job remains the only scoped use of `id-token: write` and `attestations: write`.

Phase 192 does not add `contents: write`, `packages: write`, `deployments: write`, `artifact-metadata: write`, release publication, package publication, deployment, installer, or update-channel behavior.
