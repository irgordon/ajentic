"""Phase 10 local Ollama adapter.

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


ADAPTER_NAME = "ollama_adapter"
ADAPTER_VERSION = "0.1.0"
DEFAULT_OLLAMA_URL = "http://127.0.0.1:11434/api/generate"
DEFAULT_TIMEOUT_SECONDS = 10

REQUIRED_KEYS = (
    "protocol_version",
    "run_id",
    "candidate_request_id",
    "provider",
    "model",
    "objective_ref",
    "constraints_ref",
    "domain_ref",
    "input_ref",
    "timeout_ms",
    "max_output_bytes",
)


def parse_request(raw: str) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in raw.splitlines():
        if not line:
            continue
        if "=" not in line:
            raise ValueError("malformed line")
        key, value = line.split("=", 1)
        values[key] = value

    for key in REQUIRED_KEYS:
        if key not in values:
            raise ValueError(f"missing required key: {key}")

    return values


def _build_response(request: dict[str, str], status: str, output_text: str, errors: list[str]) -> str:
    lines = [
        f"protocol_version={request['protocol_version']}",
        f"adapter_name={ADAPTER_NAME}",
        f"adapter_version={ADAPTER_VERSION}",
        f"run_id={request['run_id']}",
        f"candidate_request_id={request['candidate_request_id']}",
        f"status={status}",
        f"raw_output_ref=ollama://raw/{request['run_id']}",
        f"structured_output_ref=ollama://structured/{request['run_id']}",
        f"output_text={output_text}",
    ]
    for error in errors:
        lines.append(f"error={error}")
    return "\n".join(lines)


def _provider_url() -> str:
    return os.environ.get("AJENTIC_OLLAMA_URL", DEFAULT_OLLAMA_URL)


def _provider_model(request: dict[str, str]) -> str:
    return os.environ.get("AJENTIC_OLLAMA_MODEL") or request["model"]


def call_ollama(request: dict[str, str], timeout_seconds: float | None = None) -> tuple[str, list[str], str]:
    model = _provider_model(request).strip()
    if not model:
        return "FAILED", "", ["provider configuration error: missing ollama model"]

    payload = {
        "model": model,
        "prompt": request["input_ref"],
        "stream": False,
    }

    data = json.dumps(payload).encode("utf-8")
    http_request = urllib.request.Request(
        _provider_url(),
        data=data,
        headers={"Content-Type": "application/json"},
        method="POST",
    )

    timeout = timeout_seconds if timeout_seconds is not None else DEFAULT_TIMEOUT_SECONDS

    try:
        with urllib.request.urlopen(http_request, timeout=timeout) as response:
            raw_body = response.read().decode("utf-8")
    except TimeoutError:
        return "BLOCKED", "", ["provider timeout: ollama request timed out"]
    except socket.timeout:
        return "BLOCKED", "", ["provider timeout: ollama request timed out"]
    except (urllib.error.URLError, OSError):
        return "FAILED", "", ["provider unavailable: ollama endpoint not reachable"]

    try:
        body = json.loads(raw_body)
    except json.JSONDecodeError:
        return "FAILED", "", ["provider response malformed: invalid json"]

    if isinstance(body, dict) and body.get("error"):
        return "FAILED", "", [f"provider error: {body['error']}"]

    text = ""
    if isinstance(body, dict):
        text = body.get("response", "")

    if not isinstance(text, str):
        return "FAILED", "", ["provider response malformed: response must be string"]

    if not text.strip():
        return "FAILED", "", ["provider response invalid: empty generated text"]

    return "COMPLETED", text, []


def run(raw_request: str) -> str:
    request = parse_request(raw_request)
    status, output_text, errors = call_ollama(request)
    return _build_response(request, status, output_text, errors)


def main() -> int:
    try:
        response = run(sys.stdin.read())
    except ValueError as error:
        print(f"ollama_adapter error: {error}", file=sys.stderr)
        return 2

    sys.stdout.write(response)
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
