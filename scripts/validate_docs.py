from pathlib import Path
import re
import sys

failures: list[str] = []

REQUIRED_FRONTMATTER_KEYS = {
    "truth_dimension",
    "authority_level",
    "mutation_path",
}

EXPECTED_DOCS_ANCHORS = {
    "docs/governance/GOVERNANCE.md": {
        "truth_dimension": "normative",
        "authority_level": "authoritative",
        "mutation_path": "governance_pr",
    },
    "docs/architecture/ARCHITECTURE.md": {
        "truth_dimension": "structural",
        "authority_level": "authoritative",
        "mutation_path": "architecture_pr",
    },
}

ROOT_EXPECTED = {
    "README.md": {
        "truth_dimension": "orientation",
        "authority_level": "non_authoritative",
        "mutation_path": "readme_update",
    },
    "AGENTS.md": {
        "truth_dimension": "navigation",
        "authority_level": "non_authoritative",
        "mutation_path": "agents_update",
    },
    "CHANGELOG.md": {
        "truth_dimension": "historical",
        "authority_level": "authoritative",
        "mutation_path": "changelog_entry",
    },
}


PATH_EXPECTED = {
    "docs/operations/early-human-use-evidence-capture-template-phase-124.md": {
        "truth_dimension": "procedural",
        "authority_level": "advisory",
        "mutation_path": "checklist_revision",
    },
}

LOCATION_EXPECTED = [
    ("docs/governance/", "normative", "authoritative", "governance_pr"),
    ("docs/architecture/", "structural", "authoritative", "architecture_pr"),
    ("docs/changelog/", "historical", "authoritative", "changelog_entry"),
    ("docs/roadmap/", "planned", None, "roadmap_update"),
    ("docs/operations/", "orientation", None, "readme_update"),
    ("docs/examples/", "example", "non_authoritative", "example_update"),
    ("checklists/", "procedural", "authoritative", "checklist_revision"),
]

CHANGELOG_FUTURE_PATTERNS = [
    r"\bplanned next\b",
    r"\bfuture plan\b",
    r"\bcoming soon\b",
    r"\bto be implemented\b",
    r"\bnext phase will\b",
]

ROADMAP_COMPLETION_PATTERNS = [
    r"\bstatus:\s*complete\b",
    r"\bcompleted on\b",
    r"\bwas completed\b",
    r"\bhas been completed\b",
    r"\bshipped\b",
    r"\breleased in v",
]

GOVERNANCE_STATUS_PATTERNS = [
    r"\bstatus:\s*(complete|in progress|pending|done)\b",
    r"\bcurrently implemented\b",
    r"\bphase\s+\d+\s+is\s+complete\b",
]

ARCHITECTURE_HISTORY_OR_PLAN_PATTERNS = [
    r"\bstatus:\s*(complete|in progress|pending|done)\b",
    r"\bcompleted on\b",
    r"\bwas completed\b",
    r"\bhas been completed\b",
    r"\bnext phase will\b",
    r"\bplanned phase\b",
]

README_AUTHORITY_PATTERNS = [
    r"\brequired mutation path\b",
    r"\bgovernance_pr\b",
    r"\barchitecture_pr\b",
    r"\bschema_change\b",
    r"\bchecklist_revision\b",
]

AGENTS_MONOLITH_PATTERNS = [
    r"\bfull governance\b",
    r"\bphase-by-phase implementation\b",
    r"\bprovider-specific instructions\b",
    r"\blong prompt library\b",
]


def fail(message: str) -> None:
    failures.append(message)


def is_git_path(path: Path) -> bool:
    return ".git" in path.parts


def is_github_instruction_path(path: Path) -> bool:
    return (
        len(path.parts) >= 3
        and path.parts[0] == ".github"
        and path.parts[1] == "instructions"
        and path.name.endswith(".instructions.md")
    )


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="strict")


def parse_frontmatter(path: Path) -> dict[str, str]:
    lines = read_text(path).splitlines()
    if not lines or lines[0].strip() != "---":
        return {}

    data: dict[str, str] = {}
    for line in lines[1:]:
        if line.strip() == "---":
            return data
        if ":" in line:
            key, value = line.split(":", 1)
            data[key.strip()] = value.strip().strip('"').strip("'")
    return {}


def body_text(path: Path) -> str:
    lines = read_text(path).splitlines()
    if lines and lines[0].strip() == "---":
        for index, line in enumerate(lines[1:], start=1):
            if line.strip() == "---":
                return "\n".join(lines[index + 1 :])
    return "\n".join(lines)


def check_expected_frontmatter(path: Path, expected: dict[str, str]) -> None:
    fm = parse_frontmatter(path)
    if not fm:
        fail(f"{path}: missing required frontmatter")
        return

    missing = REQUIRED_FRONTMATTER_KEYS - set(fm)
    for key in sorted(missing):
        fail(f"{path}: missing frontmatter key '{key}'")

    for key, value in expected.items():
        if fm.get(key) != value:
            fail(f"{path}: expected {key}: {value}, found {fm.get(key)!r}")


def check_patterns(path: Path, patterns: list[str], message: str) -> None:
    text = body_text(path).lower()
    for pattern in patterns:
        if re.search(pattern, text):
            fail(f"{path}: {message}; matched pattern {pattern!r}")


markdown_files = [
    path for path in Path(".").rglob("*.md") if not is_git_path(path) and not is_github_instruction_path(path)
]

if Path("GOVERNANCE.md").exists():
    fail("GOVERNANCE.md must live at docs/governance/GOVERNANCE.md, not repository root")
if Path("ARCHITECTURE.md").exists():
    fail("ARCHITECTURE.md must live at docs/architecture/ARCHITECTURE.md, not repository root")
if Path("PHASE_MAP.md").exists():
    fail("PHASE_MAP.md must live under docs/roadmap/, not repository root")

for anchor, expected in EXPECTED_DOCS_ANCHORS.items():
    path = Path(anchor)
    if not path.is_file():
        fail(f"Missing docs anchor: {anchor}")
    else:
        check_expected_frontmatter(path, expected)

for root_file, expected in ROOT_EXPECTED.items():
    path = Path(root_file)
    if path.is_file():
        check_expected_frontmatter(path, expected)

for path in markdown_files:
    path_text = path.as_posix()
    fm = parse_frontmatter(path)

    requires_frontmatter = (
        path_text in EXPECTED_DOCS_ANCHORS
        or path_text in ROOT_EXPECTED
        or path_text.startswith("docs/")
        or path_text.startswith("checklists/")
    )

    if requires_frontmatter and not fm:
        fail(f"{path}: missing required frontmatter")
        continue

    if path_text.startswith("memory/"):
        fail(f"{path}: Markdown documentation must not live in memory/")

    path_expected = PATH_EXPECTED.get(path_text)
    if path_expected:
        for key, expected_value in path_expected.items():
            if fm.get(key) != expected_value:
                fail(f"{path}: expected {key}: {expected_value}, found {fm.get(key)!r}")
    else:
        for prefix, truth_dimension, authority_level, mutation_path in LOCATION_EXPECTED:
            if path_text.startswith(prefix):
                if fm.get("truth_dimension") != truth_dimension:
                    fail(f"{path}: expected truth_dimension: {truth_dimension}, found {fm.get('truth_dimension')!r}")
                if authority_level and fm.get("authority_level") != authority_level:
                    fail(f"{path}: expected authority_level: {authority_level}, found {fm.get('authority_level')!r}")
                if fm.get("mutation_path") != mutation_path:
                    fail(f"{path}: expected mutation_path: {mutation_path}, found {fm.get('mutation_path')!r}")

    if path_text == "CHANGELOG.md":
        check_patterns(path, CHANGELOG_FUTURE_PATTERNS, "changelog must not contain future planning language")

    if path_text.startswith("docs/roadmap/"):
        check_patterns(path, ROADMAP_COMPLETION_PATTERNS, "roadmap must not record completed work")

    if path_text.startswith("docs/governance/"):
        check_patterns(path, GOVERNANCE_STATUS_PATTERNS, "governance must not contain implementation status")

    if path_text.startswith("docs/architecture/"):
        check_patterns(
            path,
            ARCHITECTURE_HISTORY_OR_PLAN_PATTERNS,
            "architecture must not contain implementation status or roadmap planning",
        )

    if path_text == "README.md":
        if fm.get("authority_level") == "authoritative":
            fail(f"{path}: README must not be authoritative")
        check_patterns(path, README_AUTHORITY_PATTERNS, "README must remain orientation and not define governance")

    if path_text == "AGENTS.md":
        if fm.get("authority_level") == "authoritative":
            fail(f"{path}: AGENTS.md must not be authoritative")
        check_patterns(path, AGENTS_MONOLITH_PATTERNS, "AGENTS.md must remain a navigation contract")

    if path_text.startswith("docs/examples/") and fm.get("authority_level") == "authoritative":
        fail(f"{path}: examples must not be authoritative")

if failures:
    for message in failures:
        print(f"::error::{message}")
    print(f"\nDocumentation boundary checks failed with {len(failures)} violation(s).")
    sys.exit(1)

print("Documentation boundary checks passed.")
