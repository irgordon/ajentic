"""Phase 11 cloud OpenAI adapter.

Reads the existing AJENTIC key=value request from stdin and emits the same
key=value adapter response shape expected by the Rust trust boundary.
"""

from __future__ import annotations

import json
import os
import socket
import sys
import urllib.error
import urllib.request

from ajentic_adapter.protocol import REQUIRED_REQUEST_KEYS, line_value

ADAPTER_NAME = "openai_adapter"
ADAPTER_VERSION = "0.1.0"
DEFAULT_OPENAI_URL = "https://api.openai.com/v1/responses"
DEFAULT_TIMEOUT_SECONDS = 10


def parse_request(raw: str) -> dict[str, str]:
    values: dict[str, str] = {}

    for line in raw.splitlines():
        if not line:
            continue

        if "=" not in line:
            raise ValueError("malformed line")

        key, value = line.split("=", 1)
        values[key] = value

    for key in REQUIRED_REQUEST_KEYS:
        if key not in values:
            raise ValueError(f"missing required key: {key}")

    return values


def _build_response(
    request: dict[str, str],
    status: str,
    output_text: str,
    errors: list[str],
) -> str:
    lines = [
        f"protocol_version={request['protocol_version']}",
        f"adapter_name={ADAPTER_NAME}",
        f"adapter_version={ADAPTER_VERSION}",
        f"run_id={request['run_id']}",
        f"candidate_request_id={request['candidate_request_id']}",
        f"status={status}",
        f"raw_output_ref=openai://raw/{request['run_id']}",
        f"structured_output_ref=openai://structured/{request['run_id']}",
        f"output_text={line_value(output_text)}",
    ]

    for error in errors:
        lines.append(f"error={line_value(error)}")

    return "\n".join(lines)


def _provider_url() -> str:
    return os.environ.get("AJENTIC_OPENAI_URL", DEFAULT_OPENAI_URL)


def _provider_model(request: dict[str, str]) -> str:
    return os.environ.get("AJENTIC_OPENAI_MODEL", "").strip() or request["model"].strip()


def _api_key() -> str:
    return os.environ.get("AJENTIC_OPENAI_API_KEY", "").strip()


def _request_timeout_seconds(request: dict[str, str]) -> float:
    try:
        timeout_ms = int(request["timeout_ms"])
    except ValueError:
        return DEFAULT_TIMEOUT_SECONDS

    if timeout_ms <= 0:
        return DEFAULT_TIMEOUT_SECONDS

    return timeout_ms / 1000


def _request_max_output_bytes(request: dict[str, str]) -> int | None:
    try:
        max_output_bytes = int(request["max_output_bytes"])
    except ValueError:
        return None

    if max_output_bytes <= 0:
        return None

    return max_output_bytes


def _output_exceeds_limit(output_text: str, max_output_bytes: int | None) -> bool:
    return max_output_bytes is not None and len(output_text.encode("utf-8")) > max_output_bytes


def _extract_output_text(body: dict) -> str:
    text = body.get("output_text", "")
    if isinstance(text, str) and text.strip():
        return text

    output = body.get("output")
    if isinstance(output, list):
        for item in output:
            if not isinstance(item, dict):
                continue

            content = item.get("content")
            if not isinstance(content, list):
                continue

            for block in content:
                if isinstance(block, dict) and isinstance(block.get("text"), str):
                    if block["text"].strip():
                        return block["text"]

    return ""


def call_openai(
    request: dict[str, str],
    timeout_seconds: float | None = None,
) -> tuple[str, str, list[str]]:
    api_key = _api_key()
    if not api_key:
        return "FAILED", "", ["provider configuration error: missing AJENTIC_OPENAI_API_KEY"]

    model = _provider_model(request)
    if not model:
        return "FAILED", "", ["provider configuration error: missing openai model"]

    payload = {
        "model": model,
        "input": request["input_ref"],
    }

    data = json.dumps(payload).encode("utf-8")
    http_request = urllib.request.Request(
        _provider_url(),
        data=data,
        headers={
            "Content-Type": "application/json",
            "Authorization": f"Bearer {api_key}",
        },
        method="POST",
    )

    timeout = timeout_seconds if timeout_seconds is not None else _request_timeout_seconds(request)

    try:
        with urllib.request.urlopen(http_request, timeout=timeout) as response:
            raw_body = response.read().decode("utf-8")
    except TimeoutError:
        return "BLOCKED", "", ["provider timeout: openai request timed out"]
    except socket.timeout:
        return "BLOCKED", "", ["provider timeout: openai request timed out"]
    except urllib.error.HTTPError as error:
        if error.code == 429:
            return "BLOCKED", "", ["provider blocked: openai rate limited"]

        if error.code in (401, 403):
            return "FAILED", "", ["provider authorization error"]

        if 500 <= error.code <= 599:
            return "BLOCKED", "", [f"provider unavailable: openai http {error.code}"]

        return "FAILED", "", [f"provider error: openai http {error.code}"]
    except (urllib.error.URLError, OSError):
        return "FAILED", "", ["provider unavailable: openai endpoint not reachable"]

    try:
        body = json.loads(raw_body)
    except json.JSONDecodeError:
        return "FAILED", "", ["provider response malformed: invalid json"]

    if isinstance(body, dict) and body.get("error"):
        return "FAILED", "", ["provider error: openai returned error object"]

    if not isinstance(body, dict):
        return "FAILED", "", ["provider response malformed: expected object"]

    text = _extract_output_text(body)
    if not text:
        return "FAILED", "", ["provider response invalid: empty generated text"]

    if _output_exceeds_limit(text, _request_max_output_bytes(request)):
        return "FAILED", "", ["provider response invalid: output exceeds max_output_bytes"]

    return "COMPLETED", text, []


def run(raw_request: str) -> str:
    request = parse_request(raw_request)
    status, output_text, errors = call_openai(request)

    return _build_response(request, status, output_text, errors)


def main() -> int:
    try:
        response = run(sys.stdin.read())
    except ValueError as error:
        print(f"openai_adapter error: {error}", file=sys.stderr)
        return 2

    sys.stdout.write(response)
    sys.stdout.write("\n")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
