#!/usr/bin/env python3
"""Publish AJENTIC v1.0.0 through the GitHub REST API.

This script is intended for the guarded Phase 194 GitHub Actions workflow.
It does not publish packages, create installers, activate update channels,
deploy, or create prereleases.
"""

from __future__ import annotations

import argparse
import json
import mimetypes
import os
import sys
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path


API_ROOT = "https://api.github.com"
UPLOAD_ROOT = "https://uploads.github.com"
FINAL_TAG = "v1.0.0"
SOURCE_RC_TAG = "v1.0.0-rc.1"


def main() -> int:
    args = parse_args()
    publisher = GitHubFinalPublisher(args.repository, require_token(args.token_env))

    if args.preflight:
        publisher.preflight(args.final_tag, args.source_rc_tag)
        print("GitHub API final release preflight passed.")
        return 0

    if args.publish:
        result = publisher.publish(
            final_tag=args.final_tag,
            expected_commit=args.expected_commit,
            source_rc_tag=args.source_rc_tag,
            release_notes=Path(args.release_notes),
            asset_dir=Path(args.asset_dir),
        )
        print(json.dumps(result, indent=2, sort_keys=True))
        return 0

    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Publish final v1 via GitHub REST API.")
    parser.add_argument("--repository", required=True, help="owner/repo")
    parser.add_argument("--final-tag", default=FINAL_TAG, help="Final tag")
    parser.add_argument("--source-rc-tag", default=SOURCE_RC_TAG, help="Source RC tag")
    parser.add_argument("--expected-commit", help="Target commit SHA")
    parser.add_argument("--release-notes", help="Release notes markdown path")
    parser.add_argument("--asset-dir", help="Directory containing final release assets")
    parser.add_argument(
        "--token-env",
        default="GITHUB_TOKEN",
        help="Environment variable containing the GitHub token.",
    )
    parser.add_argument(
        "--preflight",
        action="store_true",
        help="Verify source RC presence and final tag/release absence.",
    )
    parser.add_argument(
        "--publish",
        action="store_true",
        help="Create annotated final tag, final release, and assets.",
    )
    args = parser.parse_args()

    if args.preflight and args.publish:
        parser.error("--preflight and --publish are mutually exclusive")
    if args.publish:
        missing = [
            name
            for name, value in [
                ("--expected-commit", args.expected_commit),
                ("--release-notes", args.release_notes),
                ("--asset-dir", args.asset_dir),
            ]
            if not value
        ]
        if missing:
            parser.error(f"--publish requires {', '.join(missing)}")
    return args


def require_token(token_env: str) -> str:
    token = os.environ.get(token_env)
    if not token:
        raise SystemExit(f"{token_env} is required for GitHub API publication")
    return token


class GitHubFinalPublisher:
    def __init__(self, repository: str, token: str):
        self.repository = repository
        self.token = token

    def preflight(self, final_tag: str, source_rc_tag: str) -> None:
        self.ensure_final_inputs(final_tag, source_rc_tag)
        self.expect_missing(f"/repos/{self.repository}/git/ref/tags/{final_tag}")
        self.expect_missing(f"/repos/{self.repository}/releases/tags/{final_tag}")
        self.verify_source_rc(source_rc_tag)

    def publish(
        self,
        final_tag: str,
        expected_commit: str,
        source_rc_tag: str,
        release_notes: Path,
        asset_dir: Path,
    ) -> dict[str, object]:
        self.preflight(final_tag, source_rc_tag)
        asset_paths = required_asset_paths(asset_dir)
        tag_object = self.create_annotated_tag(final_tag, expected_commit)
        self.create_tag_ref(final_tag, str(tag_object["sha"]))
        release = self.create_final_release(final_tag, expected_commit, release_notes)
        uploaded_assets = self.upload_assets(str(release["upload_url"]), asset_paths)
        verified_release = self.request(
            "GET", f"/repos/{self.repository}/releases/tags/{final_tag}"
        )
        return {
            "tag": final_tag,
            "tag_object_sha": tag_object["sha"],
            "target_commit": expected_commit,
            "source_rc_tag": source_rc_tag,
            "release_url": verified_release["html_url"],
            "is_draft": verified_release["draft"],
            "is_prerelease": verified_release["prerelease"],
            "make_latest": verified_release.get("make_latest"),
            "assets": uploaded_assets,
        }

    def ensure_final_inputs(self, final_tag: str, source_rc_tag: str) -> None:
        if final_tag != FINAL_TAG:
            raise SystemExit("final_tag must be v1.0.0")
        if source_rc_tag != SOURCE_RC_TAG:
            raise SystemExit("source_rc_tag must be v1.0.0-rc.1")

    def verify_source_rc(self, source_rc_tag: str) -> None:
        release = self.request(
            "GET", f"/repos/{self.repository}/releases/tags/{source_rc_tag}"
        )
        if not release.get("prerelease"):
            raise SystemExit(f"source RC release is not marked prerelease: {source_rc_tag}")
        if release.get("draft"):
            raise SystemExit(f"source RC release is draft: {source_rc_tag}")
        self.request("GET", f"/repos/{self.repository}/git/ref/tags/{source_rc_tag}")

    def create_annotated_tag(self, final_tag: str, expected_commit: str) -> dict[str, object]:
        payload = {
            "tag": final_tag,
            "message": "\n".join(
                [
                    "AJENTIC v1.0.0 final release.",
                    "This release follows v1.0.0-rc.1 final acceptance.",
                    "Release assets and evidence do not become governance authority.",
                ]
            ),
            "object": expected_commit,
            "type": "commit",
        }
        return self.request("POST", f"/repos/{self.repository}/git/tags", payload)

    def create_tag_ref(self, final_tag: str, tag_sha: str) -> dict[str, object]:
        payload = {
            "ref": f"refs/tags/{final_tag}",
            "sha": tag_sha,
        }
        return self.request("POST", f"/repos/{self.repository}/git/refs", payload)

    def create_final_release(
        self, final_tag: str, expected_commit: str, release_notes: Path
    ) -> dict[str, object]:
        payload = {
            "tag_name": final_tag,
            "target_commitish": expected_commit,
            "name": "AJENTIC v1.0.0",
            "body": release_notes.read_text(encoding="utf-8"),
            "draft": False,
            "prerelease": False,
            "make_latest": "true",
        }
        return self.request("POST", f"/repos/{self.repository}/releases", payload)

    def upload_assets(
        self, upload_url_template: str, asset_paths: list[Path]
    ) -> list[dict[str, object]]:
        upload_url = upload_url_template.split("{", 1)[0]
        uploaded = []
        for path in asset_paths:
            query = urllib.parse.urlencode({"name": path.name})
            result = self.upload(
                f"{upload_url}?{query}",
                path.read_bytes(),
                content_type=mimetypes.guess_type(path.name)[0]
                or "application/octet-stream",
            )
            uploaded.append(
                {
                    "name": result["name"],
                    "size": result["size"],
                    "state": result["state"],
                    "browser_download_url": result["browser_download_url"],
                }
            )
        return uploaded

    def expect_missing(self, path: str) -> None:
        status, _ = self.raw_request("GET", f"{API_ROOT}{path}")
        if status == 404:
            return
        if status == 200:
            raise SystemExit(f"remote resource already exists: {path}")
        raise SystemExit(f"unexpected GitHub API status {status} for {path}")

    def request(
        self, method: str, path: str, payload: dict[str, object] | None = None
    ) -> dict[str, object]:
        data = None
        if payload is not None:
            data = json.dumps(payload, sort_keys=True).encode("utf-8")
        status, body = self.raw_request(
            method,
            f"{API_ROOT}{path}",
            data=data,
            content_type="application/json" if payload is not None else None,
        )
        if status < 200 or status >= 300:
            raise SystemExit(f"GitHub API {method} {path} failed: {status} {body}")
        return json.loads(body)

    def upload(self, url: str, data: bytes, content_type: str) -> dict[str, object]:
        status, body = self.raw_request(
            "POST",
            url,
            data=data,
            content_type=content_type,
            upload=True,
        )
        if status < 200 or status >= 300:
            raise SystemExit(f"GitHub asset upload failed: {status} {body}")
        return json.loads(body)

    def raw_request(
        self,
        method: str,
        url: str,
        data: bytes | None = None,
        content_type: str | None = None,
        upload: bool = False,
    ) -> tuple[int, str]:
        headers = {
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {self.token}",
            "User-Agent": "ajentic-phase-194-final-release",
            "X-GitHub-Api-Version": "2022-11-28",
        }
        if content_type:
            headers["Content-Type"] = content_type
        if upload and not url.startswith(UPLOAD_ROOT):
            raise SystemExit("refusing to upload assets outside GitHub uploads API")

        request = urllib.request.Request(url, data=data, headers=headers, method=method)
        try:
            with urllib.request.urlopen(request, timeout=60) as response:
                return response.status, response.read().decode("utf-8")
        except urllib.error.HTTPError as error:
            return error.code, error.read().decode("utf-8")


def required_asset_paths(asset_dir: Path) -> list[Path]:
    if not asset_dir.is_dir():
        raise SystemExit(f"asset directory missing: {asset_dir}")

    paths = sorted(path for path in asset_dir.iterdir() if path.is_file())
    required_fragments = [
        "final-candidate-bundle.tar.gz",
        "final-asset-manifest.json",
        "final-checksums.json",
        "final-sbom.json",
        "final-provenance.json",
        "release-notes.md",
        "README-FINAL-",
    ]
    names = [path.name for path in paths]
    for fragment in required_fragments:
        if not any(fragment in name for name in names):
            raise SystemExit(f"required final asset missing: {fragment}")
    for name in names:
        lowered = name.lower()
        for fragment in ["rc", "prerelease", "draft", "unstable", "installer", "update-channel", "deployment"]:
            if fragment in lowered:
                raise SystemExit(f"prohibited final asset name fragment {fragment}: {name}")

    return paths


if __name__ == "__main__":
    sys.exit(main())
