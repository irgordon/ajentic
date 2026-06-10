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
import tomllib
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

    if args.check_evidence:
        return check_integrity_evidence(repo_root)

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
    parser.add_argument(
        "--check-evidence",
        action="store_true",
        help="Run two clean builds and compare internal integrity evidence.",
    )
    return parser.parse_args()


def check_reproducible_artifacts(repo_root: Path) -> int:
    require_file(repo_root / "core" / "Cargo.toml")
    require_file(repo_root / "core" / "Cargo.lock")
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


def check_integrity_evidence(repo_root: Path) -> int:
    require_file(repo_root / "core" / "Cargo.toml")
    require_file(repo_root / "core" / "Cargo.lock")
    require_file(repo_root / "ui" / "package-lock.json")
    require_file(repo_root / "ui" / "package.json")

    build_context = build_evidence_context(repo_root)

    with tempfile.TemporaryDirectory(prefix="ajentic-phase-189-evidence-") as temp_dir:
        temp_root = Path(temp_dir)
        first_root = temp_root / "build-a"
        second_root = temp_root / "build-b"

        copy_source_tree(repo_root, first_root)
        copy_source_tree(repo_root, second_root)

        first_evidence = build_and_evidence(first_root, build_context)
        second_evidence = build_and_evidence(second_root, build_context)

        if canonical_json(first_evidence) != canonical_json(second_evidence):
            report_evidence_mismatch(first_evidence, second_evidence)
            return 1

        write_evidence(temp_root / "evidence", first_evidence)
        print("Internal checksum, SBOM, and provenance evidence match.")
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


def build_evidence_context(repo_root: Path) -> dict[str, object]:
    return {
        "repository": resolve_repository(repo_root),
        "commit_sha": git_output(repo_root, ["rev-parse", "HEAD"]),
        "branch": resolve_branch(repo_root),
        "source_date_epoch": resolve_source_date_epoch(repo_root),
        "workflow_name": os.environ.get("GITHUB_WORKFLOW") or None,
        "workflow_ref": os.environ.get("GITHUB_WORKFLOW_REF") or None,
        "runner_os": os.environ.get("RUNNER_OS") or None,
    }


def resolve_repository(repo_root: Path) -> str:
    remote_url = git_output(repo_root, ["config", "--get", "remote.origin.url"])
    if remote_url.endswith(".git"):
        remote_url = remote_url[:-4]
    if remote_url.startswith("https://github.com/"):
        return remote_url.removeprefix("https://github.com/")
    if remote_url.startswith("git@github.com:"):
        return remote_url.removeprefix("git@github.com:")
    return remote_url


def resolve_branch(repo_root: Path) -> str:
    branch = os.environ.get("GITHUB_REF_NAME")
    if branch:
        return branch
    return git_output(repo_root, ["rev-parse", "--abbrev-ref", "HEAD"])


def git_output(repo_root: Path, command: list[str]) -> str:
    result = subprocess.run(
        ["git", *command],
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


def build_and_evidence(
    build_root: Path, build_context: dict[str, object]
) -> dict[str, object]:
    artifact_manifest = build_and_manifest(
        build_root, str(build_context["source_date_epoch"])
    )
    return integrity_evidence(build_root, build_context, artifact_manifest)


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


def integrity_evidence(
    build_root: Path,
    build_context: dict[str, object],
    artifact_manifest: list[dict[str, object]],
) -> dict[str, object]:
    return {
        "checksums": checksum_evidence(build_context, artifact_manifest),
        "sbom": sbom_evidence(build_root, build_context),
        "provenance": provenance_evidence(build_root, build_context),
    }


def checksum_evidence(
    build_context: dict[str, object],
    artifact_manifest: list[dict[str, object]],
) -> dict[str, object]:
    return {
        "evidence_version": "phase-189.1",
        "repository": build_context["repository"],
        "commit_sha": build_context["commit_sha"],
        "source_date_epoch": build_context["source_date_epoch"],
        "generated_by": "scripts/reproducible_artifacts.py --check-evidence",
        "artifact_scope": "internal_candidate",
        "release_status": "not_release_artifact",
        "entries": [
            {
                "category": entry["category"],
                "relative_path": entry["path"],
                "size_bytes": entry["size"],
                "sha256": entry["sha256"],
            }
            for entry in artifact_manifest
        ],
    }


def sbom_evidence(build_root: Path, build_context: dict[str, object]) -> dict[str, object]:
    ui_package = read_json(build_root / "ui" / "package.json")
    core_package = read_toml(build_root / "core" / "Cargo.toml")["package"]

    return {
        "evidence_version": "phase-189.1",
        "sbom_format": "ajentic-internal-sbom-json",
        "sbom_format_status": "internal_not_standards_complete",
        "repository": build_context["repository"],
        "commit_sha": build_context["commit_sha"],
        "package_name": "ajentic",
        "package_version": None,
        "license": "MIT",
        "release_status": "not_release_artifact",
        "components": sorted(
            [
                *cargo_components(build_root, core_package),
                *npm_components(build_root, ui_package),
            ],
            key=lambda component: (
                str(component["ecosystem"]),
                str(component["name"]),
                str(component["version"]),
                str(component["source_manifest_path"]),
            ),
        ),
    }


def cargo_components(build_root: Path, core_package: dict[str, object]) -> list[dict[str, object]]:
    lock = read_toml(build_root / "core" / "Cargo.lock")
    packages = lock.get("package", [])
    components = [
        component_entry(
            name=str(core_package["name"]),
            version=str(core_package["version"]),
            ecosystem="cargo",
            source_manifest_path="core/Cargo.toml",
            license_value=str(core_package.get("license") or "UNKNOWN"),
            integrity_digest=sha256_digest(build_root / "core" / "Cargo.toml"),
        )
    ]

    for package in packages:
        components.append(
            component_entry(
                name=str(package.get("name") or "UNKNOWN"),
                version=str(package.get("version") or "UNKNOWN"),
                ecosystem="cargo",
                source_manifest_path="core/Cargo.lock",
                license_value="UNKNOWN",
                integrity_digest=package.get("checksum"),
            )
        )

    return components


def npm_components(build_root: Path, ui_package: dict[str, object]) -> list[dict[str, object]]:
    lock = read_json(build_root / "ui" / "package-lock.json")
    packages = lock.get("packages", {})
    components = [
        component_entry(
            name=str(ui_package["name"]),
            version=str(ui_package["version"]),
            ecosystem="npm",
            source_manifest_path="ui/package.json",
            license_value=str(ui_package.get("license") or "UNKNOWN"),
            integrity_digest=sha256_digest(build_root / "ui" / "package.json"),
        )
    ]

    for package_path, package in packages.items():
        if package_path == "":
            continue
        components.append(
            component_entry(
                name=package_path.removeprefix("node_modules/") or "UNKNOWN",
                version=str(package.get("version") or "UNKNOWN"),
                ecosystem="npm",
                source_manifest_path="ui/package-lock.json",
                license_value=package.get("license") or "UNKNOWN",
                integrity_digest=package.get("integrity"),
            )
        )

    return components


def component_entry(
    name: str,
    version: str,
    ecosystem: str,
    source_manifest_path: str,
    license_value: object,
    integrity_digest: object,
) -> dict[str, object]:
    return {
        "name": name,
        "version": version,
        "ecosystem": ecosystem,
        "source_manifest_path": source_manifest_path,
        "license": license_value,
        "integrity_digest": integrity_digest,
    }


def provenance_evidence(
    build_root: Path, build_context: dict[str, object]
) -> dict[str, object]:
    return {
        "evidence_version": "phase-189.1",
        "provenance_format": "ajentic-internal-provenance",
        "provenance_format_status": "internal_unsigned",
        "repository": build_context["repository"],
        "commit_sha": build_context["commit_sha"],
        "branch": build_context["branch"],
        "workflow_name": build_context["workflow_name"],
        "workflow_ref": build_context["workflow_ref"],
        "runner_os": build_context["runner_os"],
        "build_command_summary": [
            "npm ci",
            "npm run build",
            "cargo build --manifest-path core/Cargo.toml --release --locked",
        ],
        "artifact_surfaces": [
            "rust release executable internal candidates",
            "ui dist internal candidates",
        ],
        "source_manifests": source_manifest_entries(build_root),
        "source_date_epoch": build_context["source_date_epoch"],
        "release_status": "not_release_artifact",
        "signing_status": "unsigned",
        "attestation_status": "not_attested",
        "publication_status": "not_published",
    }


def source_manifest_entries(build_root: Path) -> list[dict[str, object]]:
    manifest_paths = [
        Path("core/Cargo.toml"),
        Path("core/Cargo.lock"),
        Path("ui/package.json"),
        Path("ui/package-lock.json"),
    ]
    return [
        {
            "path": path.as_posix(),
            "sha256": sha256_digest(build_root / path),
        }
        for path in manifest_paths
    ]


def read_json(path: Path) -> dict[str, object]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def read_toml(path: Path) -> dict[str, object]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


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


def canonical_json(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")


def write_evidence(evidence_dir: Path, evidence: dict[str, object]) -> None:
    evidence_dir.mkdir(parents=True, exist_ok=True)
    for name, content in evidence.items():
        path = evidence_dir / f"{name}.json"
        path.write_bytes(canonical_json(content) + b"\n")


def report_evidence_mismatch(
    first_evidence: dict[str, object],
    second_evidence: dict[str, object],
) -> None:
    print("::error::internal integrity evidence differs")
    for name in sorted(set(first_evidence) | set(second_evidence)):
        first_content = first_evidence.get(name)
        second_content = second_evidence.get(name)
        if canonical_json(first_content) != canonical_json(second_content):
            print(f"mismatch: {name}")
            print(f"  build-a: {json.dumps(first_content, sort_keys=True)}")
            print(f"  build-b: {json.dumps(second_content, sort_keys=True)}")


if __name__ == "__main__":
    sys.exit(main())
