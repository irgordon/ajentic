"""Deterministic Phase 4 mock adapter.

Reads a narrow key=value request from stdin and writes one deterministic
key=value response to stdout.
"""

from __future__ import annotations

import sys

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


def build_response(request: dict[str, str]) -> str:
    return "\n".join(
        [
            f"protocol_version={request['protocol_version']}",
            "adapter_name=mock_adapter",
            "adapter_version=0.1.0",
            f"run_id={request['run_id']}",
            f"candidate_request_id={request['candidate_request_id']}",
            "status=COMPLETED",
            f"raw_output_ref=mock://raw/{request['run_id']}",
            f"structured_output_ref=mock://structured/{request['run_id']}",
            "output_text=This is deterministic mock adapter output. It is untrusted and not evaluated.",
        ]
    )


def main() -> int:
    try:
        request = parse_request(sys.stdin.read())
    except ValueError as error:
        print(f"mock_adapter error: {error}", file=sys.stderr)
        return 2

    sys.stdout.write(build_response(request))
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
