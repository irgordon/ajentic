---
truth_dimension: structural
authority_level: authoritative
mutation_path: architecture_pr
---

# ADR 004: File-First Local Review Workspace

Date: 2026-10-23

Status: Accepted by Owner selection of the refined file-first design

## Context

The local browser entry point mixed the primary sample review with a long list
of implementation-oriented release, provider, and trial panels. Long headings,
oversized introductory copy, and repeated warnings made the first review hard
to find. The Owner selected a file-first visual direction with shorter Title
Case labels, a compact sans-serif UI, and Courier-style code text.

## Decision and Why

Use a file sidebar, Compare/Result/Activity views, a compact Tool Result summary,
and persistent desktop Review Notes/Review Decision areas. Keep the familiar
sample, checks, and decision together without turning every section into a card.
On narrow screens, stack the comparison and decision controls without horizontal
page overflow; code can scroll within its own container.

Use Inter at 14px for UI text, compact 16px headings, and Courier Prime at 13px
for code and diffs. Bundle the fonts locally rather than loading a remote font
service. Use unmodified Lucide icons, with familiar symbols and accessible names,
instead of hand-drawn SVGs. Retain the font and icon notices under `ui/public/`.

Keep existing advanced test/evidence controls under View / Developer Details.
Preserve their action IDs and transport calls. Extract the existing diagnostic
renderers mechanically rather than rewriting their local behavior. Expose actual
rejections and blocked/missing/rejected evidence counts outside the collapsed
details so progressive disclosure does not conceal material problems.

The comparison is explicitly pre-recorded illustrative data, not a new diff
engine or provider-generated file output. Review Notes is presentation-only
memory, not evidence submitted to the transport. The attachment control remains
disabled. Record Approval and Reject Draft still record one local intent; they
do not authorize tool execution or file writes. The result-ID mismatch in the
combined advanced sample remains a visible rejection, not an invented success.

## Asset Integrity

`ui/public/third-party-assets.json` records upstream sources, versions/commits,
licenses, and SHA-256 hashes for fonts, icons, and notices. The existing license
audit also checks this inventory. The build helper copies only reviewed entries
and the inventory, not arbitrary public-directory content or OS metadata.

Inter and Courier Prime retain OFL-1.1 terms. Lucide retains its ISC and derived
Feather MIT notices. These third-party assets do not become project-MIT-only.
The existing package-oriented SBOM is not a substitute for the asset inventory.

## Alternatives

| Alternative | Why Not Selected |
| --- | --- |
| Rename headings but keep the long page | Does not improve the primary task's placement or reduce scanning effort. |
| Replace the app with a React Native stack | The references inform interaction design, not a framework migration. |
| Delete the diagnostic panels | Removes useful rejected-state and evidence inspection paths. |
| Show a successful write/diff from the provider | Claims functionality the local shell does not supply. |
| Add live uploads or execution controls | Expands capability/authority beyond this UI scope. |
| Load fonts or icon scripts from a CDN | Adds runtime network dependencies to the local review surface. |

## Consequences and Verification

Keep rendering, presentation preferences, and generic DOM operations separate
from the unchanged local transport. Unit tests cover sample labelling, escaped
notes, context expansion, decision recording, rejected-state visibility, and
diagnostic controls. Browser checks cover keyboard tabs, menus, focus, decision
recording, notes, diagnostics, fonts, assets, and desktop/mobile overflow.

The [design QA report](../operations/file-review-workspace-design-qa.md) records
the approved visual, same-viewport comparisons, spacing correction, intentional
truth-preserving deviations, and final acceptance evidence. The documentation
must follow the actual labels and retain the current capability limits.

No Rust behavior, package version, branch-protection policy, release workflow
permission, publishing, signing, installer, update-channel, or deployment path
is changed by this decision.
