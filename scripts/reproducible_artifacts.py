#!/usr/bin/env python3
"""Build and compare internal candidate artifact manifests.

This script is validation evidence only. It does not publish, sign, tag,
upload, deploy, or create release artifacts.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path


EXCLUDED_DIRS = {
    ".git",
    ".mypy_cache",
    ".pytest_cache",
    "node_modules",
    "target",
    "dist",
    "__pycache__",
}

EXCLUDED_ROOT_DIRS = {
    ".codex",
    ".agents",
}


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo).resolve()

    if args.check:
        return check_reproducible_artifacts(repo_root)

    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate internal candidate artifact reproducibility."
    )
    parser.add_argument(
        "--repo",
        default=".",
        help="Repository root to copy and build. Defaults to current directory.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Run two clean builds and compare normalized manifests.",
    )
    return parser.parse_args()


def check_reproducible_artifacts(repo_root: Path) -> int:
    require_file(repo_root / "core" / "Cargo.toml")
    require_file(repo_root / "ui" / "package-lock.json")
    require_file(repo_root / "ui" / "package.json")

    source_date_epoch = resolve_source_date_epoch(repo_root)

    with tempfile.TemporaryDirectory(prefix="ajentic-phase-188-") as temp_dir:
        temp_root = Path(temp_dir)
        first_root = temp_root / "build-a"
        second_root = temp_root / "build-b"

        copy_source_tree(repo_root, first_root)
        copy_source_tree(repo_root, second_root)

        first_manifest = build_and_manifest(first_root, source_date_epoch)
        second_manifest = build_and_manifest(second_root, source_date_epoch)

        if first_manifest != second_manifest:
            report_manifest_mismatch(first_manifest, second_manifest)
            return 1

        print(f"Reproducible artifact manifests match ({len(first_manifest)} files).")
        return 0


def require_file(path: Path) -> None:
    if not path.is_file():
        raise SystemExit(f"required file missing: {path}")


def resolve_source_date_epoch(repo_root: Path) -> str:
    existing = os.environ.get("SOURCE_DATE_EPOCH")
    if existing:
        return existing

    result = subprocess.run(
        ["git", "log", "-1", "--format=%ct"],
        cwd=repo_root,
        check=True,
        text=True,
        capture_output=True,
    )
    return result.stdout.strip()


def copy_source_tree(source_root: Path, destination_root: Path) -> None:
    def ignore(directory: str, names: list[str]) -> set[str]:
        path = Path(directory)
        ignored = set()

        for name in names:
            child = path / name
            if name in EXCLUDED_DIRS:
                ignored.add(name)
            elif path == source_root and name in EXCLUDED_ROOT_DIRS:
                ignored.add(name)
            elif child.is_file() and name.endswith(".pyc"):
                ignored.add(name)

        return ignored

    shutil.copytree(source_root, destination_root, ignore=ignore)


def build_and_manifest(build_root: Path, source_date_epoch: str) -> list[dict[str, object]]:
    env = deterministic_environment(build_root, source_date_epoch)

    run(["npm", "ci"], cwd=build_root / "ui", env=env)
    run(["npm", "run", "build"], cwd=build_root / "ui", env=env)
    run(
        [
            "cargo",
            "build",
            "--manifest-path",
            "core/Cargo.toml",
            "--release",
            "--locked",
        ],
        cwd=build_root,
        env=env,
    )

    return normalized_manifest(build_root)


def deterministic_environment(build_root: Path, source_date_epoch: str) -> dict[str, str]:
    env = os.environ.copy()
    env.update(
        {
            "TZ": "UTC",
            "LC_ALL": "C",
            "CARGO_INCREMENTAL": "0",
            "SOURCE_DATE_EPOCH": source_date_epoch,
            "CARGO_TARGET_DIR": str(build_root / ".phase188-target"),
        }
    )
    return env


def run(command: list[str], cwd: Path, env: dict[str, str]) -> None:
    print(f"running: {' '.join(command)}")
    subprocess.run(command, cwd=cwd, env=env, check=True)


def normalized_manifest(build_root: Path) -> list[dict[str, object]]:
    entries = []
    entries.extend(rust_artifact_entries(build_root))
    entries.extend(ui_artifact_entries(build_root))

    if not entries:
        raise SystemExit("no internal candidate artifacts were produced")

    return sorted(entries, key=lambda entry: (str(entry["category"]), str(entry["path"])))


def rust_artifact_entries(build_root: Path) -> list[dict[str, object]]:
    release_dir = build_root / ".phase188-target" / "release"
    candidates = [
        path
        for path in release_dir.iterdir()
        if path.is_file() and is_rust_candidate(path)
    ]

    if not candidates:
        raise SystemExit(f"rust internal candidate artifact missing in {release_dir}")

    return [
        manifest_entry("rust", Path("rust") / path.name, path)
        for path in sorted(candidates)
    ]


def is_rust_candidate(path: Path) -> bool:
    if path.suffix in {".d", ".rlib", ".rmeta"}:
        return False
    if path.name.startswith("lib"):
        return False
    return os.access(path, os.X_OK) or path.suffix == ".exe"


def ui_artifact_entries(build_root: Path) -> list[dict[str, object]]:
    dist_dir = build_root / "ui" / "dist"
    if not dist_dir.is_dir():
        raise SystemExit(f"ui internal candidate artifact directory missing: {dist_dir}")

    entries = [
        manifest_entry("ui", Path("ui") / path.relative_to(dist_dir), path)
        for path in sorted(dist_dir.rglob("*"))
        if path.is_file()
    ]

    if not entries:
        raise SystemExit(f"ui internal candidate artifact directory is empty: {dist_dir}")

    return entries


def manifest_entry(category: str, relative_path: Path, file_path: Path) -> dict[str, object]:
    return {
        "category": category,
        "path": relative_path.as_posix(),
        "size": file_path.stat().st_size,
        "sha256": sha256_digest(file_path),
    }


def sha256_digest(file_path: Path) -> str:
    digest = hashlib.sha256()
    with file_path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def report_manifest_mismatch(
    first_manifest: list[dict[str, object]],
    second_manifest: list[dict[str, object]],
) -> None:
    first_by_path = manifest_by_path(first_manifest)
    second_by_path = manifest_by_path(second_manifest)
    all_paths = sorted(set(first_by_path) | set(second_by_path))

    print("::error::internal candidate artifact manifests differ")
    for path in all_paths:
        first_entry = first_by_path.get(path)
        second_entry = second_by_path.get(path)
        if first_entry != second_entry:
            print(f"mismatch: {path}")
            print(f"  build-a: {json.dumps(first_entry, sort_keys=True)}")
            print(f"  build-b: {json.dumps(second_entry, sort_keys=True)}")


def manifest_by_path(manifest: list[dict[str, object]]) -> dict[str, dict[str, object]]:
    return {f"{entry['category']}/{entry['path']}": entry for entry in manifest}


if __name__ == "__main__":
    sys.exit(main())
