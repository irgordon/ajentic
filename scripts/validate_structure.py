from pathlib import Path
import re
import sys

ROOT = Path(".")
failures: list[str] = []

REQUIRED_ROOT_FILES = {
    "AGENTS.md",
    "README.md",
    "CHANGELOG.md",
}

REQUIRED_DOC_ANCHORS = {
    "docs/governance/GOVERNANCE.md",
    "docs/architecture/ARCHITECTURE.md",
}

ALLOWED_TOP_LEVEL_FILES = REQUIRED_ROOT_FILES | {
    ".gitignore",
    "LICENSE",
    "Cargo.toml",
    "Cargo.lock",
    "package.json",
    "pnpm-lock.yaml",
    "package-lock.json",
    "yarn.lock",
}

ALLOWED_TOP_LEVEL_DIRS = {
    ".git",
    ".github",
    "core",
    "ui",
    "scripts",
    "tests",
    "memory",
    "checklists",
    "docs",
    "schemas",
}

ALLOWED_DOCS_DIRS = {
    "governance",
    "architecture",
    "changelog",
    "roadmap",
    "operations",
    "examples",
}

IGNORED_DIR_NAMES = {
    ".git",
    "target",
    "node_modules",
    "dist",
    "build",
    "coverage",
    "__pycache__",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
}

OPERATIONS_PROCEDURAL_NAME_MARKERS = (
    "template",
    "checklist",
)

CHANGELOG_ARCHIVE_PATTERN = re.compile(r"^CHANGELOG-\d{4}-\d{4}\.md$")


def fail(message: str) -> None:
    failures.append(message)


def should_ignore(path: Path) -> bool:
    return any(part in IGNORED_DIR_NAMES for part in path.parts)


def iter_files(root: Path, pattern: str = "*"):
    if not root.exists():
        return

    for path in root.rglob(pattern):
        if should_ignore(path):
            continue
        if path.is_file():
            yield path


def iter_markdown_files(root: Path):
    yield from iter_files(root, "*.md")


def read_text_utf8(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="strict")


def is_under(path: Path, root: Path) -> bool:
    path_posix = path.as_posix()
    root_posix = root.as_posix()
    return path_posix == root_posix or path_posix.startswith(root_posix.rstrip("/") + "/")


def is_github_instruction_path(path: Path) -> bool:
    return (
        len(path.parts) >= 3
        and path.parts[0] == ".github"
        and path.parts[1] == "instructions"
        and path.name.endswith(".instructions.md")
    )


def allowed_truth_dimensions_for_path(path: Path) -> set[str]:
    path_text = path.as_posix()
    name = path.name.lower()

    if path_text == "README.md":
        return {"orientation"}

    if path_text == "AGENTS.md":
        return {"navigation"}

    if path_text == "CHANGELOG.md":
        return {"historical"}

    if is_under(path, Path("docs/governance")):
        return {"normative"}

    if is_under(path, Path("docs/architecture")):
        return {"structural"}

    if is_under(path, Path("docs/changelog")):
        return {"historical"}

    if is_under(path, Path("docs/roadmap")):
        return {"planned"}

    if is_under(path, Path("docs/operations")):
        if any(marker in name for marker in OPERATIONS_PROCEDURAL_NAME_MARKERS):
            return {"procedural"}
        return {"orientation"}

    if is_under(path, Path("docs/examples")):
        return {"example"}

    if is_under(path, Path("checklists")):
        return {"procedural"}

    if is_under(path, Path("schemas")):
        return {"contract"}

    if is_under(path, Path("memory")):
        return {"data"}

    return set()


def allowed_for_truth(path: Path, truth_dimension: str) -> bool:
    allowed = allowed_truth_dimensions_for_path(path)
    if not allowed:
        fail(f"{path.as_posix()}: no truth dimensions are defined for this location")
        return False

    return truth_dimension in allowed


def parse_frontmatter(path: Path) -> dict[str, str]:
    text = read_text_utf8(path)
    lines = text.splitlines()

    if not lines or lines[0].strip() != "---":
        return {}

    data: dict[str, str] = {}

    for line in lines[1:]:
        if line.strip() == "---":
            return data

        if ":" not in line:
            continue

        key, value = line.split(":", 1)
        data[key.strip()] = value.strip().strip('"').strip("'")

    return {}


for filename in sorted(REQUIRED_ROOT_FILES):
    if not Path(filename).is_file():
        fail(f"Missing required root file: {filename}")

for filename in sorted(REQUIRED_DOC_ANCHORS):
    if not Path(filename).is_file():
        fail(f"Missing required docs anchor: {filename}")

for filename in ["GOVERNANCE.md", "ARCHITECTURE.md", "PHASE_MAP.md"]:
    if Path(filename).exists():
        fail(f"{filename} must live under docs/, not repository root")

for child in ROOT.iterdir():
    name = child.name

    if child.is_dir():
        if name not in ALLOWED_TOP_LEVEL_DIRS:
            fail(f"Unexpected top-level directory: {name}")
        continue

    if child.is_file() and name not in ALLOWED_TOP_LEVEL_FILES:
        fail(f"Unexpected top-level file: {name}")

docs = Path("docs")
if docs.exists():
    for child in docs.iterdir():
        if child.is_dir() and child.name not in ALLOWED_DOCS_DIRS:
            fail(f"Unexpected docs/ subdirectory: {child.as_posix()}")

changelog_archive_dir = Path("docs/changelog")
if changelog_archive_dir.exists():
    for path in iter_files(changelog_archive_dir):
        if path.suffix != ".md":
            fail(f"{path.as_posix()}: changelog archive directory may contain Markdown files only")
            continue

        if not CHANGELOG_ARCHIVE_PATTERN.fullmatch(path.name):
            fail(
                f"{path.as_posix()}: changelog archive files must use "
                "CHANGELOG-0001-0055.md naming"
            )

anchor_locations = {
    "GOVERNANCE.md": Path("docs/governance/GOVERNANCE.md"),
    "ARCHITECTURE.md": Path("docs/architecture/ARCHITECTURE.md"),
}

for anchor, allowed_path in anchor_locations.items():
    matches = [
        path
        for path in iter_files(ROOT, anchor)
        if path != allowed_path
    ]

    for match in matches:
        fail(
            f"{anchor} must exist only at {allowed_path.as_posix()}; "
            f"found {match.as_posix()}"
        )

for path in iter_files(ROOT, "*.schema.json"):
    if not is_under(path, Path("schemas")):
        fail(f"{path.as_posix()}: schema files must live under schemas/")

memory = Path("memory")
if memory.exists():
    for path in iter_files(memory):
        if path.suffix == ".md":
            fail(f"{path.as_posix()}: memory/ must not contain Markdown documentation")

        if path.name.endswith(".schema.json"):
            fail(f"{path.as_posix()}: memory/ must not contain schemas")

ephemeral = Path("memory/ephemeral")
if ephemeral.exists():
    for path in iter_files(ephemeral):
        if path.name != ".gitignore":
            fail(f"{path.as_posix()}: ephemeral memory must not be committed")

instructions = Path(".github/instructions")
if instructions.exists():
    for path in instructions.glob("*.instructions.md"):
        fm = parse_frontmatter(path)
        if "applyTo" not in fm:
            fail(f"{path.as_posix()}: GitHub instruction files must declare applyTo frontmatter")

markdown_paths = [
    path
    for path in iter_markdown_files(ROOT)
    if not is_github_instruction_path(path)
]

for path in markdown_paths:
    path_text = path.as_posix()
    fm = parse_frontmatter(path)

    requires_frontmatter = (
        path.name in REQUIRED_ROOT_FILES
        or path_text in REQUIRED_DOC_ANCHORS
        or is_under(path, Path("docs"))
        or is_under(path, Path("checklists"))
    )

    if requires_frontmatter and not fm:
        fail(f"{path_text}: missing required frontmatter")
        continue

    if not fm:
        continue

    for key in ["truth_dimension", "authority_level", "mutation_path"]:
        if key not in fm:
            fail(f"{path_text}: missing frontmatter key '{key}'")

    truth_dimension = fm.get("truth_dimension")
    if truth_dimension and not allowed_for_truth(path, truth_dimension):
        fail(
            f"{path_text}: truth_dimension '{truth_dimension}' "
            "is not allowed in this location"
        )

for path in iter_files(Path("docs"), "*.schema.json"):
    fail(f"{path.as_posix()}: docs/ must not contain schemas")

if failures:
    for message in failures:
        print(f"::error::{message}")
    print(f"\nStructure lint failed with {len(failures)} violation(s).")
    sys.exit(1)

print("Repository structure is valid.")
