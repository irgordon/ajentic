#!/usr/bin/env python3
"""Publish an AJENTIC release candidate through the GitHub REST API.

This script is intended for the guarded Phase 193 GitHub Actions workflow.
It uses GitHub's API instead of the gh CLI. It does not publish packages,
create installers, activate update channels, deploy, or create final v1.0.0
releases.
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


def main() -> int:
    args = parse_args()
    publisher = GitHubPublisher(args.repository, require_token(args.token_env))

    if args.preflight:
        publisher.preflight(args.rc_tag)
        print("GitHub API RC publication preflight passed.")
        return 0

    if args.publish:
        result = publisher.publish(
            rc_tag=args.rc_tag,
            expected_commit=args.expected_commit,
            release_notes=Path(args.release_notes),
            asset_dir=Path(args.asset_dir),
        )
        print(json.dumps(result, indent=2, sort_keys=True))
        return 0

    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Publish RC via GitHub REST API.")
    parser.add_argument("--repository", required=True, help="owner/repo")
    parser.add_argument("--rc-tag", required=True, help="RC tag")
    parser.add_argument("--expected-commit", help="Target commit SHA")
    parser.add_argument("--release-notes", help="Release notes markdown path")
    parser.add_argument("--asset-dir", help="Directory containing RC release assets")
    parser.add_argument(
        "--token-env",
        default="GITHUB_TOKEN",
        help="Environment variable containing the GitHub token.",
    )
    parser.add_argument(
        "--preflight",
        action="store_true",
        help="Verify tag and release absence.",
    )
    parser.add_argument(
        "--publish",
        action="store_true",
        help="Create annotated tag, prerelease, and assets.",
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


class GitHubPublisher:
    def __init__(self, repository: str, token: str):
        self.repository = repository
        self.token = token

    def preflight(self, rc_tag: str) -> None:
        self.expect_missing(f"/repos/{self.repository}/git/ref/tags/{rc_tag}")
        self.expect_missing(f"/repos/{self.repository}/releases/tags/{rc_tag}")

    def publish(
        self,
        rc_tag: str,
        expected_commit: str,
        release_notes: Path,
        asset_dir: Path,
    ) -> dict[str, object]:
        self.preflight(rc_tag)
        asset_paths = required_asset_paths(asset_dir)
        tag_object = self.create_annotated_tag(rc_tag, expected_commit)
        self.create_tag_ref(rc_tag, str(tag_object["sha"]))
        release = self.create_prerelease(rc_tag, expected_commit, release_notes)
        uploaded_assets = self.upload_assets(str(release["upload_url"]), asset_paths)
        return {
            "tag": rc_tag,
            "tag_object_sha": tag_object["sha"],
            "target_commit": expected_commit,
            "release_url": release["html_url"],
            "is_draft": release["draft"],
            "is_prerelease": release["prerelease"],
            "make_latest": release.get("make_latest"),
            "assets": uploaded_assets,
        }

    def create_annotated_tag(self, rc_tag: str, expected_commit: str) -> dict[str, object]:
        payload = {
            "tag": rc_tag,
            "message": "\n".join(
                [
                    f"AJENTIC {rc_tag} release candidate.",
                    "This is not final v1.0.0 release execution.",
                    "This is not production approval.",
                ]
            ),
            "object": expected_commit,
            "type": "commit",
        }
        return self.request("POST", f"/repos/{self.repository}/git/tags", payload)

    def create_tag_ref(self, rc_tag: str, tag_sha: str) -> dict[str, object]:
        payload = {
            "ref": f"refs/tags/{rc_tag}",
            "sha": tag_sha,
        }
        return self.request("POST", f"/repos/{self.repository}/git/refs", payload)

    def create_prerelease(
        self, rc_tag: str, expected_commit: str, release_notes: Path
    ) -> dict[str, object]:
        payload = {
            "tag_name": rc_tag,
            "target_commitish": expected_commit,
            "name": f"AJENTIC {rc_tag}",
            "body": release_notes.read_text(encoding="utf-8"),
            "draft": False,
            "prerelease": True,
            "make_latest": "false",
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
        self, method: str, path: str, payload: dict[str, object]
    ) -> dict[str, object]:
        status, body = self.raw_request(
            method,
            f"{API_ROOT}{path}",
            data=json.dumps(payload, sort_keys=True).encode("utf-8"),
            content_type="application/json",
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
            "User-Agent": "ajentic-phase-193-rc-publication",
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
        "rc-candidate-bundle.tar.gz",
        "rc-asset-manifest.json",
        "rc-checksums.json",
        "rc-sbom.json",
        "rc-provenance.json",
        "release-notes.md",
        "README-RC-",
    ]
    names = [path.name for path in paths]
    for fragment in required_fragments:
        if not any(fragment in name for name in names):
            raise SystemExit(f"required RC asset missing: {fragment}")

    return paths


if __name__ == "__main__":
    sys.exit(main())
