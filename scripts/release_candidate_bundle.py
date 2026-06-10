#!/usr/bin/env python3
"""Build deterministic internal release-candidate bundles.

This script creates internal candidate evidence only. It does not publish,
sign, tag, upload, deploy, create installers, or mutate tracked repository
files.
"""

from __future__ import annotations

import argparse
import gzip
import importlib.util
import json
import os
import shutil
import subprocess
import sys
import tarfile
import tempfile
import tomllib
from pathlib import Path


BUNDLE_ROOT = "ajentic-internal-candidate"
ARCHIVE_NAME = "ajentic-internal-candidate.tar.gz"
MANIFEST_NAME = "release-candidate-manifest.json"
CHECKSUMS_NAME = "bundle-checksums.json"
README_NAME = "README-INTERNAL-CANDIDATE.txt"
NORMALIZED_TIME = 0
SOURCE_DATE_EPOCH = "0"
SCRIPT_NAME = "scripts/release_candidate_bundle.py"
EXCLUDED_PATHS = [
    ".git",
    ".agents",
    ".codex",
    "node_modules",
    "target",
    "ui/dist",
    "__pycache__",
    ".phase188-target",
    "incremental caches",
    "absolute temp paths",
    "usernames",
    "hostnames",
    "GitHub run IDs",
    "local timestamps",
]


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo).resolve()
    validate_repo_root(repo_root)
    ensure_versions_are_not_final(repo_root)

    if args.check:
        return check_bundle(repo_root, args.candidate_label)

    if args.build:
        output_dir = require_output_dir(args.output_dir)
        build_once(repo_root, output_dir, args.candidate_label)
        print(f"Internal candidate bundle written to {output_dir}")
        return 0

    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Build deterministic internal release-candidate bundles."
    )
    parser.add_argument(
        "--repo",
        default=".",
        help="Repository root to package. Defaults to current directory.",
    )
    parser.add_argument(
        "--candidate-label",
        default="phase-191-local-check",
        help="Internal candidate label to record in the manifest.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Build the internal candidate bundle twice and compare outputs.",
    )
    parser.add_argument(
        "--build",
        action="store_true",
        help="Build one internal candidate bundle into --output-dir.",
    )
    parser.add_argument(
        "--output-dir",
        help="Output directory for --build. Generated contents are replaced.",
    )
    args = parser.parse_args()

    if args.check and args.build:
        parser.error("--check and --build are mutually exclusive")
    if args.build and not args.output_dir:
        parser.error("--build requires --output-dir")
    return args


def validate_repo_root(repo_root: Path) -> None:
    required = [
        repo_root / "core" / "Cargo.toml",
        repo_root / "core" / "Cargo.lock",
        repo_root / "ui" / "package.json",
        repo_root / "ui" / "package-lock.json",
        repo_root / "LICENSE",
        repo_root / "scripts" / "reproducible_artifacts.py",
    ]
    missing = [path for path in required if not path.is_file()]
    if missing:
        for path in missing:
            print(f"::error::required file missing: {path}")
        raise SystemExit(1)


def ensure_versions_are_not_final(repo_root: Path) -> None:
    versions = package_versions(repo_root)
    final_versions = {
        name: value for name, value in versions.items() if str(value) == "1.0.0"
    }
    if final_versions:
        print("::error::package version unexpectedly set to 1.0.0")
        print(json.dumps(final_versions, indent=2, sort_keys=True))
        raise SystemExit(1)


def check_bundle(repo_root: Path, candidate_label: str) -> int:
    with tempfile.TemporaryDirectory(prefix="ajentic-phase-191-bundle-") as temp_dir:
        temp_root = Path(temp_dir)
        first_output = temp_root / "build-a"
        second_output = temp_root / "build-b"

        first_manifest = build_once(repo_root, first_output, candidate_label)
        second_manifest = build_once(repo_root, second_output, candidate_label)

        mismatches = compare_output_trees(first_output, second_output)
        if mismatches:
            print("::error::internal release-candidate bundle output differs")
            for mismatch in mismatches:
                print(mismatch)
            return 1

        print(
            "Internal release-candidate bundle is deterministic "
            f"({len(first_manifest['rust_artifacts'])} rust artifact(s), "
            f"{len(first_manifest['ui_artifacts'])} UI file(s))."
        )
        return 0


def build_once(
    repo_root: Path, output_dir: Path, candidate_label: str
) -> dict[str, object]:
    reproducible = load_reproducible_module(repo_root)
    output_dir = output_dir.resolve()
    reset_directory(output_dir)

    with tempfile.TemporaryDirectory(prefix="ajentic-phase-191-source-") as temp_dir:
        build_root = Path(temp_dir) / "source"
        reproducible.copy_source_tree(repo_root, build_root)

        build_context = build_context_for(repo_root, reproducible)
        artifact_manifest = reproducible.build_and_manifest(
            build_root, build_context["source_date_epoch"]
        )
        evidence = reproducible.integrity_evidence(
            build_root, build_context, artifact_manifest
        )
        stage_root = output_dir / BUNDLE_ROOT
        create_staging_tree(
            build_root,
            stage_root,
            build_context,
            artifact_manifest,
            evidence,
            candidate_label,
        )
        archive_path = output_dir / ARCHIVE_NAME
        write_deterministic_archive(stage_root, archive_path)
        write_external_checksum_evidence(output_dir, archive_path)
        return read_json(output_dir / MANIFEST_NAME)


def load_reproducible_module(repo_root: Path):
    module_path = repo_root / "scripts" / "reproducible_artifacts.py"
    spec = importlib.util.spec_from_file_location(
        "ajentic_reproducible_artifacts", module_path
    )
    if spec is None or spec.loader is None:
        raise SystemExit(f"cannot load {module_path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def build_context_for(repo_root: Path, reproducible) -> dict[str, str]:
    return {
        "repository": reproducible.resolve_repository(repo_root),
        "commit_sha": reproducible.git_output(repo_root, ["rev-parse", "HEAD"]),
        "branch": reproducible.resolve_branch(repo_root),
        "source_date_epoch": SOURCE_DATE_EPOCH,
        "workflow_name": os.environ.get("GITHUB_WORKFLOW") or None,
        "workflow_ref": os.environ.get("GITHUB_WORKFLOW_REF") or None,
        "runner_os": os.environ.get("RUNNER_OS") or None,
    }


def create_staging_tree(
    build_root: Path,
    stage_root: Path,
    build_context: dict[str, object],
    artifact_manifest: list[dict[str, object]],
    evidence: dict[str, object],
    candidate_label: str,
) -> None:
    metadata_dir = stage_root / "metadata"
    evidence_dir = stage_root / "evidence"
    rust_dir = stage_root / "rust"
    ui_dist_dir = stage_root / "ui" / "dist"
    metadata_dir.mkdir(parents=True)
    evidence_dir.mkdir(parents=True)
    rust_dir.mkdir(parents=True)
    ui_dist_dir.mkdir(parents=True)

    copy_rust_artifacts(build_root, rust_dir)
    shutil.copytree(build_root / "ui" / "dist", ui_dist_dir, dirs_exist_ok=True)

    checksums = evidence["checksums"]
    sbom = evidence["sbom"]
    provenance = internal_attestation_ready_provenance(evidence["provenance"])

    write_json(evidence_dir / "checksums-internal.json", checksums)
    write_json(evidence_dir / "sbom-internal.json", sbom)
    write_json(evidence_dir / "provenance-internal.json", provenance)
    write_json(metadata_dir / "sbom-internal.json", sbom)
    write_json(metadata_dir / "build-provenance-internal.json", provenance)

    manifest = release_candidate_manifest(
        build_root,
        build_context,
        artifact_manifest,
        candidate_label,
    )
    write_json(metadata_dir / MANIFEST_NAME, manifest)
    write_text(stage_root / README_NAME, internal_candidate_readme())
    internal_checksums = internal_checksum_evidence(stage_root)
    write_json(metadata_dir / CHECKSUMS_NAME, internal_checksums)

    copy_bundle_metadata_outputs(stage_root.parent, metadata_dir, evidence_dir)


def copy_rust_artifacts(build_root: Path, rust_dir: Path) -> None:
    release_dir = build_root / ".phase188-target" / "release"
    candidates = [
        path
        for path in release_dir.iterdir()
        if path.is_file()
        and path.suffix not in {".d", ".rlib", ".rmeta"}
        and not path.name.startswith("lib")
        and (os.access(path, os.X_OK) or path.suffix == ".exe")
    ]
    if not candidates:
        raise SystemExit(f"rust internal candidate artifact missing in {release_dir}")
    for path in sorted(candidates):
        shutil.copy2(path, rust_dir / path.name)


def release_candidate_manifest(
    build_root: Path,
    build_context: dict[str, object],
    artifact_manifest: list[dict[str, object]],
    candidate_label: str,
) -> dict[str, object]:
    return {
        "release_candidate_schema_version": "phase-191.1",
        "project_name": "AJENTIC",
        "candidate_label": candidate_label,
        "commit_sha": build_context["commit_sha"],
        "branch": build_context["branch"],
        "package_versions": package_versions(build_root),
        "license": "MIT",
        "artifact_scope": "internal_candidate",
        "release_status": "not_published",
        "signing_status": "github_artifact_attestation_optional",
        "publication_status": "not_published",
        "generated_by": SCRIPT_NAME,
        "generated_at_normalized": "1970-01-01T00:00:00Z",
        "source_date_epoch": SOURCE_DATE_EPOCH,
        "rust_artifacts": artifacts_for_category(artifact_manifest, "rust"),
        "ui_artifacts": artifacts_for_category(artifact_manifest, "ui"),
        "evidence_files": [
            "evidence/checksums-internal.json",
            "evidence/sbom-internal.json",
            "evidence/provenance-internal.json",
            "metadata/bundle-checksums.json",
        ],
        "excluded_paths": EXCLUDED_PATHS,
        "validation_commands": [
            "python3 scripts/release_candidate_bundle.py --check",
            "python3 scripts/reproducible_artifacts.py --check",
            "python3 scripts/reproducible_artifacts.py --check-evidence",
            "CARGO_TARGET_DIR=/tmp/ajentic-phase-191-target ./scripts/check.sh",
        ],
    }


def artifacts_for_category(
    artifact_manifest: list[dict[str, object]], category: str
) -> list[dict[str, object]]:
    return [
        {
            "relative_path": entry["path"],
            "size_bytes": entry["size"],
            "sha256": entry["sha256"],
        }
        for entry in artifact_manifest
        if entry["category"] == category
    ]


def package_versions(repo_root: Path) -> dict[str, str]:
    ui = read_json(repo_root / "ui" / "package.json")
    core = read_toml(repo_root / "core" / "Cargo.toml")["package"]
    return {
        "core": str(core["version"]),
        "ui": str(ui["version"]),
    }


def internal_attestation_ready_provenance(
    provenance: dict[str, object]
) -> dict[str, object]:
    updated = dict(provenance)
    updated["provenance_format_status"] = "internal_unsigned_or_attested"
    updated["attestation_status"] = "optional_internal_github_artifact_attestation"
    updated["release_status"] = "not_public_release_artifact"
    return updated


def internal_checksum_evidence(stage_root: Path) -> dict[str, object]:
    entries = []
    for path in sorted(stage_root.rglob("*")):
        if path.is_file() and path.name != CHECKSUMS_NAME:
            entries.append(checksum_entry(stage_root, path))
    return {
        "evidence_version": "phase-191.1",
        "checksum_scope": "internal bundle checksum evidence",
        "release_status": "not_public_release_artifact",
        "self_reference_note": (
            "The archive-level checksum is written outside the archive after "
            "archive finalization to avoid self-referential checksum drift."
        ),
        "entries": entries,
    }


def write_external_checksum_evidence(output_dir: Path, archive_path: Path) -> None:
    stage_root = output_dir / BUNDLE_ROOT
    entries = [checksum_entry(output_dir, archive_path)]
    entries.extend(
        checksum_entry(output_dir, path)
        for path in sorted(output_dir.glob("*.json"))
        if path.name != CHECKSUMS_NAME
    )
    entries.append(
        checksum_entry(
            output_dir,
            stage_root / "metadata" / CHECKSUMS_NAME,
            label=f"{BUNDLE_ROOT}/metadata/{CHECKSUMS_NAME}",
        )
    )
    write_json(
        output_dir / CHECKSUMS_NAME,
        {
            "evidence_version": "phase-191.1",
            "checksum_scope": "internal bundle checksum evidence",
            "release_status": "not_public_release_artifact",
            "entries": entries,
        },
    )


def checksum_entry(root: Path, path: Path, label: str | None = None) -> dict[str, object]:
    return {
        "relative_path": label or path.relative_to(root).as_posix(),
        "size_bytes": path.stat().st_size,
        "sha256": sha256_digest(path),
    }


def internal_candidate_readme() -> str:
    return "\n".join(
        [
            "AJENTIC internal candidate bundle",
            "",
            "This is an internal candidate bundle.",
            "This is not a public release artifact.",
            "This is not v1.0 release execution.",
            "This is not production approval.",
            "This is not an installer.",
            "This is not an update-channel artifact.",
            "GitHub Releases and tags remain future-gated.",
            "RC publication remains Phase 193.",
            "Final release execution remains Phase 194.",
            "",
        ]
    )


def copy_bundle_metadata_outputs(
    output_dir: Path, metadata_dir: Path, evidence_dir: Path
) -> None:
    output_dir.mkdir(parents=True, exist_ok=True)
    shutil.copy2(metadata_dir / MANIFEST_NAME, output_dir / MANIFEST_NAME)
    shutil.copy2(metadata_dir / "sbom-internal.json", output_dir / "sbom-internal.json")
    shutil.copy2(
        metadata_dir / "build-provenance-internal.json",
        output_dir / "provenance-internal.json",
    )
    shutil.copy2(
        evidence_dir / "checksums-internal.json", output_dir / "checksums-internal.json"
    )


def write_deterministic_archive(stage_root: Path, archive_path: Path) -> None:
    with archive_path.open("wb") as raw:
        with gzip.GzipFile(filename="", mode="wb", fileobj=raw, mtime=NORMALIZED_TIME) as gz:
            with tarfile.open(fileobj=gz, mode="w") as tar:
                for path in sorted(stage_root.rglob("*")):
                    arcname = path.relative_to(stage_root.parent).as_posix()
                    info = tar.gettarinfo(str(path), arcname=arcname)
                    info.uid = 0
                    info.gid = 0
                    info.uname = ""
                    info.gname = ""
                    info.mtime = NORMALIZED_TIME
                    if path.is_dir():
                        info.mode = 0o755
                        tar.addfile(info)
                    elif path.is_file():
                        info.mode = 0o755 if os.access(path, os.X_OK) else 0o644
                        with path.open("rb") as handle:
                            tar.addfile(info, handle)


def compare_output_trees(first_output: Path, second_output: Path) -> list[str]:
    first_files = file_digest_map(first_output)
    second_files = file_digest_map(second_output)
    mismatches = []
    for path in sorted(set(first_files) | set(second_files)):
        if first_files.get(path) != second_files.get(path):
            mismatches.append(
                f"mismatch: {path} build-a={first_files.get(path)} "
                f"build-b={second_files.get(path)}"
            )
    return mismatches


def file_digest_map(root: Path) -> dict[str, str]:
    return {
        path.relative_to(root).as_posix(): sha256_digest(path)
        for path in sorted(root.rglob("*"))
        if path.is_file()
    }


def require_output_dir(output_dir: str | None) -> Path:
    if not output_dir:
        raise SystemExit("--output-dir is required for --build")
    path = Path(output_dir)
    if path.resolve() == Path.cwd().resolve():
        raise SystemExit("--output-dir must not be the repository root")
    return path


def reset_directory(path: Path) -> None:
    if path.exists():
        shutil.rmtree(path)
    path.mkdir(parents=True)


def read_json(path: Path) -> dict[str, object]:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def read_toml(path: Path) -> dict[str, object]:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(canonical_json(value).decode("utf-8") + "\n", encoding="utf-8")


def write_text(path: Path, value: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(value, encoding="utf-8")


def canonical_json(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")


def sha256_digest(file_path: Path) -> str:
    digest = __import__("hashlib").sha256()
    with file_path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


if __name__ == "__main__":
    sys.exit(main())
