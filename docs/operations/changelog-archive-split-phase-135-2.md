---
truth_dimension: orientation
authority_level: advisory
mutation_path: readme_update
---

# Phase 135.2 Changelog Archive Split Operations Report

## Scope

Phase 135.2 is out-of-band changelog maintenance only. The scope is limited to active changelog archive movement, archive index/pointer maintenance, procedural checklist update, and this operations report.

Decision status: `changelog_archive_complete_with_findings`.

## Evidence rule

Only committed repository files were counted as evidence. Prompt intent, chat summaries, expected ranges, and roadmap planned truth were not treated as changelog historical truth.

## Phase 135.2 maintenance boundary

- Changelog archiving is historical maintenance, not historical rewriting.
- Archive movement must preserve historical entry content.
- Archive ranges must remain contiguous and version-bounded.
- Active CHANGELOG.md remains the current historical surface.
- Archived changelog files preserve completed historical truth.
- Phase 135.2 does not change roadmap planned truth except narrow archive-reference clarification if required.
- Phase 135.2 does not approve readiness, Release Candidate status, Production Candidate status, or public/general use.

## Changelog archiving is historical maintenance, not historical rewriting

Historical entries moved from `CHANGELOG.md` were moved by version block. Entry headings, dates, status text, section labels, bullets, notes, and ordering inside each extracted archive were preserved without content normalization.

## Existing changelog archive inventory

Initial archive inventory from `ls docs/changelog`:

- `CHANGELOG-0001-0055.md`
- `CHANGELOG-0056-0104.md`

Post-split archive inventory:

- `CHANGELOG-0001-0055.md`
- `CHANGELOG-0056-0104.md`
- `CHANGELOG-0104-0115.md`
- `CHANGELOG-0116-0125.md`
- `CHANGELOG-0126-0134.md`

## Line-count inspection

Initial line counts from `wc -l CHANGELOG.md docs/changelog/*.md`:

```text
  1817 CHANGELOG.md
  1287 docs/changelog/CHANGELOG-0001-0055.md
  1443 docs/changelog/CHANGELOG-0056-0104.md
  4547 total
```

Post-split line counts:

```text
   176 CHANGELOG.md
  1287 docs/changelog/CHANGELOG-0001-0055.md
  1443 docs/changelog/CHANGELOG-0056-0104.md
   496 docs/changelog/CHANGELOG-0104-0115.md
   475 docs/changelog/CHANGELOG-0116-0125.md
   757 docs/changelog/CHANGELOG-0126-0134.md
  4634 total
```

Status: `archive_range_adjusted_for_size`.

## Version-range inspection

`rg -n "^## v|^## v0\.0\." CHANGELOG.md docs/changelog/*.md` showed the active changelog contained `v0.0.104.5` through `v0.0.135.1` before this split. Existing archives contained `v0.0.0` through `v0.0.104`, including already-recorded historical ordering notes in the earlier archive.

After the split:

- Active `CHANGELOG.md` contains `v0.0.135.2`, `v0.0.135.1`, and `v0.0.135`.
- `CHANGELOG-0126-0134.md` contains `v0.0.126` through `v0.0.134`, including patch-level entries in that range.
- `CHANGELOG-0116-0125.md` contains `v0.0.116` through `v0.0.125`.
- `CHANGELOG-0104-0115.md` contains `v0.0.104.5` through `v0.0.115`.
- Existing archive files were preserved.

## Archive range selection

Selected range status: `archive_range_selected`.

Selected archive movements:

- `CHANGELOG-0126-0134.md`: moved completed active entries `v0.0.126` through `v0.0.134`.
- `CHANGELOG-0116-0125.md`: moved completed active entries `v0.0.116` through `v0.0.125`.
- `CHANGELOG-0104-0115.md`: moved completed active entries `v0.0.104.5` through `v0.0.115`.

## Archive range rationale

The split preserves existing archive ranges and adds three bounded archives below the practical target size. The `0104-0115` archive is intentionally named with `0104` because `v0.0.104.5` is a completed patch-level historical entry after the existing `v0.0.104` boundary. This avoids leaving an old completed entry active and avoids rewriting or renaming the existing `CHANGELOG-0056-0104.md` archive.

Status: `archive_range_adjusted_for_size`.

## Active CHANGELOG.md posture

Active `CHANGELOG.md` remains the current historical surface. It now retains the Phase 135.2 entry and the current Phase 135 / Phase 135.1 entries only, plus archive pointers and archive guarantees.

## Archive index/pointer update

The active changelog archive pointers were updated to include the three new archive files and their version-bounded ranges.

Status: `archive_index_updated`.

## Historical entry preservation check

A block comparison against `git show HEAD:CHANGELOG.md` confirmed that 39 moved version entries are present in the new archive files with unchanged entry content when compared per archive file. No moved version entry is duplicated in active `CHANGELOG.md`.

## Validator compatibility assessment

`validate_structure.py` already allows `CHANGELOG-0000-0000.md` style archive names through `CHANGELOG_ARCHIVE_PATTERN = re.compile(r"^CHANGELOG-\d{4}-\d{4}\.md$")`. The new archive names match that pattern. `validate_docs.py` already recognizes `docs/changelog/` as historical authoritative changelog entries. No validation compatibility update was required.

Status: `not_applicable` for `archive_validation_updated`.

## Files changed

- `CHANGELOG.md`
- `checklists/current-phase.md`
- `docs/changelog/CHANGELOG-0104-0115.md`
- `docs/changelog/CHANGELOG-0116-0125.md`
- `docs/changelog/CHANGELOG-0126-0134.md`
- `docs/operations/changelog-archive-split-phase-135-2.md`

## Non-runtime statement

Phase 135.2 changed documentation and historical archive surfaces only. It did not change runtime behavior, source code, tests, schemas, release behavior, deployment behavior, monitoring behavior, installer/update-channel behavior, signing behavior, provider execution behavior, persistence authority behavior, replay repair behavior, recovery promotion behavior, or action execution behavior.

## Non-roadmap-authority statement

Phase 135.2 did not change roadmap planned truth. No roadmap file was modified.

## Non-readiness statement

Phase 135.2 does not approve readiness, Release Candidate status, Production Candidate status, or public/general use. It does not approve production human use.

## Validation log

Planned validation commands:

- `CARGO_TARGET_DIR=/tmp/ajentic-phase-135-2-target ./scripts/check.sh`
- `git diff --check`
- `git status --short`
- `wc -l CHANGELOG.md docs/changelog/*.md`
- `rg -n "^## v0\.0\." CHANGELOG.md docs/changelog/*.md`
- targeted Phase 135.2 enforcement-line scan
- duplicate-version scan
- guarded diff scan
- no-approval scan

## Deferred items

No archive validation update was required. No roadmap clarification was required.

## Confirmed vs suspected

Confirmed:

- Existing archive ranges were preserved.
- New archive names match validator expectations.
- Active `CHANGELOG.md` keeps a clear archive index/pointer.
- The `v0.0.135.2` entry remains active.
- Moved entries compare unchanged against the pre-split active changelog.
- New archive files are below the practical target size.
- Existing archive files remain above the target size but were preserved to avoid historical archive churn.

Suspected:

- `not_applicable`.
