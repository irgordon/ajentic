#!/usr/bin/env python3
"""Prepare deterministic final GitHub Release assets.

This script stages files for the final GitHub Release only. It does not create
tags, push refs, create GitHub Releases, upload assets, publish packages,
deploy, or contact remote services.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import shutil
import sys
import tempfile
from pathlib import Path


ASSET_PREFIX = "ajentic"
FINAL_TAG = "v1.0.0"
SOURCE_RC_TAG = "v1.0.0-rc.1"
SCRIPT_NAME = "scripts/prepare_final_release_assets.py"
FORBIDDEN_NAME_FRAGMENTS = [
    "rc",
    "prerelease",
    "draft",
    "unstable",
    "installer",
    "update-channel",
    "deployment",
]


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo).resolve()

    if args.check:
        return check_assets(repo_root, args.final_tag, args.source_rc_tag)

    if args.build:
        output_dir = require_output_dir(args.output_dir)
        build_assets(repo_root, output_dir, args.final_tag, args.source_rc_tag)
        print(f"Final release assets written to {output_dir}")
        return 0

    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Prepare deterministic final GitHub Release assets."
    )
    parser.add_argument("--repo", default=".", help="Repository root.")
    parser.add_argument("--final-tag", default=FINAL_TAG, help="Final tag.")
    parser.add_argument("--source-rc-tag", default=SOURCE_RC_TAG, help="Source RC tag.")
    parser.add_argument(
        "--build",
        action="store_true",
        help="Build one final asset directory into --output-dir.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Build two final asset directories and compare them.",
    )
    parser.add_argument("--output-dir", help="Output directory for --build.")
    args = parser.parse_args()

    if args.build and args.check:
        parser.error("--build and --check are mutually exclusive")
    if args.build and not args.output_dir:
        parser.error("--build requires --output-dir")
    return args


def check_assets(repo_root: Path, final_tag: str, source_rc_tag: str) -> int:
    with tempfile.TemporaryDirectory(prefix="ajentic-phase-194-final-assets-") as temp_dir:
        temp_root = Path(temp_dir)
        first_output = temp_root / "build-a"
        second_output = temp_root / "build-b"

        build_assets(repo_root, first_output, final_tag, source_rc_tag)
        build_assets(repo_root, second_output, final_tag, source_rc_tag)

        first_files = file_digest_map(first_output)
        second_files = file_digest_map(second_output)
        if first_files != second_files:
            print("::error::Final release asset outputs differ")
            for path in sorted(set(first_files) | set(second_files)):
                if first_files.get(path) != second_files.get(path):
                    print(
                        f"mismatch: {path} build-a={first_files.get(path)} "
                        f"build-b={second_files.get(path)}"
                    )
            return 1

        print(f"Final release assets are deterministic ({len(first_files)} files).")
        return 0


def build_assets(
    repo_root: Path, output_dir: Path, final_tag: str, source_rc_tag: str
) -> None:
    ensure_final_tag(final_tag)
    ensure_release_notes(repo_root, final_tag)
    reset_directory(output_dir)

    internal_dir = output_dir / "_internal"
    release_bundle = load_release_bundle_module(repo_root)
    release_bundle.build_once(repo_root, internal_dir, final_tag)

    final_assets = final_asset_paths(output_dir, final_tag)
    copy_final_assets(repo_root, internal_dir, final_assets, final_tag)
    write_final_readme(final_assets["readme"], final_tag, source_rc_tag)
    write_asset_manifest(repo_root, output_dir, final_assets, final_tag, source_rc_tag)
    write_checksum_evidence(final_assets["checksums"], final_assets)
    check_asset_names(final_assets)
    shutil.rmtree(internal_dir)


def ensure_final_tag(final_tag: str) -> None:
    if final_tag != FINAL_TAG:
        raise SystemExit("final_tag must be v1.0.0")


def final_asset_paths(output_dir: Path, final_tag: str) -> dict[str, Path]:
    return {
        "bundle": output_dir / f"{ASSET_PREFIX}-{final_tag}-final-candidate-bundle.tar.gz",
        "manifest": output_dir / f"{ASSET_PREFIX}-{final_tag}-final-asset-manifest.json",
        "checksums": output_dir / f"{ASSET_PREFIX}-{final_tag}-final-checksums.json",
        "sbom": output_dir / f"{ASSET_PREFIX}-{final_tag}-final-sbom.json",
        "provenance": output_dir / f"{ASSET_PREFIX}-{final_tag}-final-provenance.json",
        "notes": output_dir / f"{ASSET_PREFIX}-{final_tag}-release-notes.md",
        "readme": output_dir / f"README-FINAL-{final_tag}.txt",
    }


def copy_final_assets(
    repo_root: Path, internal_dir: Path, final_assets: dict[str, Path], final_tag: str
) -> None:
    release_bundle = load_release_bundle_module(repo_root)
    shutil.copy2(internal_dir / release_bundle.ARCHIVE_NAME, final_assets["bundle"])
    shutil.copy2(internal_dir / "sbom-internal.json", final_assets["sbom"])
    shutil.copy2(internal_dir / "provenance-internal.json", final_assets["provenance"])
    shutil.copy2(release_notes_path(repo_root, final_tag), final_assets["notes"])


def write_asset_manifest(
    repo_root: Path,
    output_dir: Path,
    final_assets: dict[str, Path],
    final_tag: str,
    source_rc_tag: str,
) -> None:
    entries = [
        asset_entry(output_dir, path)
        for key, path in sorted(final_assets.items())
        if key not in {"manifest", "checksums"}
    ]
    write_json(
        final_assets["manifest"],
        {
            "schema_version": "phase-194.final-assets.v1",
            "project": "AJENTIC",
            "final_tag": final_tag,
            "source_rc_tag": source_rc_tag,
            "commit_sha": git_output(repo_root, ["rev-parse", "HEAD"]),
            "status": "final_github_release_assets",
            "github_release_only": True,
            "package_registry_publication": False,
            "installer": False,
            "update_channel": False,
            "deployment": False,
            "os_signing": False,
            "notarization": False,
            "generated_by": SCRIPT_NAME,
            "generated_at_normalized": "1970-01-01T00:00:00Z",
            "assets": entries,
        },
    )


def write_checksum_evidence(
    checksum_path: Path, final_assets: dict[str, Path]
) -> None:
    asset_paths = [
        path
        for key, path in sorted(final_assets.items())
        if key != "checksums" and path.is_file()
    ]
    write_json(
        checksum_path,
        {
            "schema_version": "phase-194.final-checksums.v1",
            "checksum_scope": "final GitHub Release asset checksums",
            "algorithm": "sha256",
            "entries": [
                {
                    "asset_name": path.name,
                    "size_bytes": path.stat().st_size,
                    "sha256": sha256_digest(path),
                }
                for path in asset_paths
            ],
        },
    )


def check_asset_names(final_assets: dict[str, Path]) -> None:
    for path in final_assets.values():
        lowered = path.name.lower()
        for fragment in FORBIDDEN_NAME_FRAGMENTS:
            if fragment in lowered:
                raise SystemExit(
                    f"final asset name contains prohibited fragment {fragment}: {path.name}"
                )


def asset_entry(root: Path, path: Path) -> dict[str, object]:
    return {
        "asset_name": path.name,
        "relative_path": path.relative_to(root).as_posix(),
        "size_bytes": path.stat().st_size,
        "sha256": sha256_digest(path),
    }


def write_final_readme(path: Path, final_tag: str, source_rc_tag: str) -> None:
    write_text(
        path,
        "\n".join(
            [
                f"AJENTIC {final_tag} final GitHub Release assets",
                "",
                f"Source release candidate: {source_rc_tag}",
                "These assets are GitHub Release assets only.",
                "This is not an installer.",
                "This does not activate an update channel.",
                "This does not publish npm or Cargo packages.",
                "This does not deploy anything.",
                "This does not include OS signing or notarization.",
                "Checksums, SBOM, provenance, and attestations are evidence only.",
                "Rust remains the authority boundary; TypeScript remains read-only display.",
                "",
            ]
        ),
    )


def ensure_release_notes(repo_root: Path, final_tag: str) -> None:
    path = release_notes_path(repo_root, final_tag)
    if not path.is_file():
        raise SystemExit(f"release notes missing: {path}")


def release_notes_path(repo_root: Path, final_tag: str) -> Path:
    return repo_root / "docs" / "releases" / f"{final_tag}.md"


def load_release_bundle_module(repo_root: Path):
    module_path = repo_root / "scripts" / "release_candidate_bundle.py"
    spec = importlib.util.spec_from_file_location("release_candidate_bundle", module_path)
    if spec is None or spec.loader is None:
        raise SystemExit(f"cannot load {module_path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


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
    return path.resolve()


def reset_directory(path: Path) -> None:
    if path.exists():
        shutil.rmtree(path)
    path.mkdir(parents=True)


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n",
        encoding="utf-8",
    )


def write_text(path: Path, value: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(value, encoding="utf-8")


def sha256_digest(file_path: Path) -> str:
    digest = hashlib.sha256()
    with file_path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def git_output(repo_root: Path, command: list[str]) -> str:
    import subprocess

    result = subprocess.run(
        ["git", *command],
        cwd=repo_root,
        check=True,
        text=True,
        capture_output=True,
    )
    return result.stdout.strip()


if __name__ == "__main__":
    sys.exit(main())
