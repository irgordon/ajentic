"""AJENTIC Python adapter protocol constants and helpers.

Python adapters are non-authoritative and emit adapter output for Rust-owned
validation and downstream governance workflows.
"""

PROTOCOL_VERSION = "0.1.0"
ADAPTER_ROLE = "non-authoritative"

REQUIRED_REQUEST_KEYS = (
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


def line_value(value: str) -> str:
    """Escape a line-protocol value for key=value output."""
    return value.replace("\\", "\\\\").replace("\n", "\\n").replace("\r", "\\r")
