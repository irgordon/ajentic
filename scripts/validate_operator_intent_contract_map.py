#!/usr/bin/env python3
"""Validate the non-authoritative operator-intent contract drift map."""

from __future__ import annotations

import json
import sys
from pathlib import Path

SCHEMA_PATH = Path("schemas/intents/operator-intent.schema.json")
MAP_PATH = Path("docs/contracts/operator-intent-contract-map.json")
RUST_PATH = "core/src/api/operator_intent.rs"
TYPESCRIPT_PATH = "ui/src/api/projections.ts"
ALLOWED_STATUSES = {
    "lossless",
    "lossy_projection",
    "unsupported",
    "intentionally_unknown_projection",
}
NON_LOSSLESS_STATUSES = ALLOWED_STATUSES - {"lossless"}
SUCCESS_WORDS = {"success", "successful", "passed", "approved", "ready"}


def fail(message: str) -> None:
    print(f"operator intent contract map validation failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def read_json(path: Path) -> object:
    try:
        with path.open("r", encoding="utf-8") as handle:
            return json.load(handle)
    except FileNotFoundError:
        fail(f"missing file: {path.as_posix()}")
    except json.JSONDecodeError as error:
        fail(f"malformed JSON in {path.as_posix()}: {error}")


def require_dict(value: object, label: str) -> dict[str, object]:
    if not isinstance(value, dict):
        fail(f"{label} must be an object")
    return value


def require_list(value: object, label: str) -> list[object]:
    if not isinstance(value, list):
        fail(f"{label} must be a list")
    return value


def schema_enum(schema: dict[str, object], path: tuple[str, ...]) -> list[str]:
    current: object = schema
    for key in path:
        current = require_dict(current, ".".join(path)).get(key)
        if current is None:
            fail(f"schema missing {'.'.join(path)}")
    values = require_list(require_dict(current, ".".join(path)).get("enum"), ".".join(path) + ".enum")
    result: list[str] = []
    for value in values:
        if not isinstance(value, str):
            fail(f"schema enum {'.'.join(path)} contains non-string value")
        result.append(value)
    return result


def mapping_by_schema_value(map_data: dict[str, object], key: str) -> dict[str, dict[str, object]]:
    entries = require_list(map_data.get(key), key)
    result: dict[str, dict[str, object]] = {}
    for index, raw_entry in enumerate(entries):
        entry = require_dict(raw_entry, f"{key}[{index}]")
        schema_value = entry.get("schema_value")
        if not isinstance(schema_value, str) or not schema_value:
            fail(f"{key}[{index}] missing non-empty schema_value")
        if schema_value in result:
            fail(f"{key} contains duplicate schema_value {schema_value}")
        result[schema_value] = entry
    return result


def validate_entry(entry: dict[str, object], label: str) -> None:
    status = entry.get("status")
    if status not in ALLOWED_STATUSES:
        fail(f"{label} has unsupported status {status!r}")
    if status in NON_LOSSLESS_STATUSES:
        explanation = entry.get("explanation")
        if not isinstance(explanation, str) or not explanation.strip():
            fail(f"{label} requires explanation for status {status}")
    combined_text = " ".join(str(value).lower() for value in entry.values() if value is not None)
    if status != "lossless" and any(word in combined_text.split() for word in SUCCESS_WORDS):
        fail(f"{label} uses success language while status is {status}")


def validate_required_values(required_values: list[str], mappings: dict[str, dict[str, object]], label: str) -> None:
    missing = sorted(set(required_values) - set(mappings))
    extra = sorted(set(mappings) - set(required_values))
    if missing:
        fail(f"{label} missing schema values: {', '.join(missing)}")
    if extra:
        fail(f"{label} contains values absent from schema: {', '.join(extra)}")
    for value in required_values:
        validate_entry(mappings[value], f"{label}.{value}")


def validate_projection_values(map_data: dict[str, object]) -> None:
    for index, raw_entry in enumerate(require_list(map_data.get("projection_only_values"), "projection_only_values")):
        entry = require_dict(raw_entry, f"projection_only_values[{index}]")
        validate_entry(entry, f"projection_only_values[{index}]")
        if entry.get("status") != "intentionally_unknown_projection":
            fail(f"projection_only_values[{index}] must use intentionally_unknown_projection")


def main() -> None:
    schema = require_dict(read_json(SCHEMA_PATH), SCHEMA_PATH.as_posix())
    map_data = require_dict(read_json(MAP_PATH), MAP_PATH.as_posix())

    if map_data.get("schema_file") != SCHEMA_PATH.as_posix():
        fail("map references the wrong schema file")
    if map_data.get("rust_file") != RUST_PATH:
        fail("map must reference core/src/api/operator_intent.rs")
    if map_data.get("typescript_file") != TYPESCRIPT_PATH:
        fail("map must reference ui/src/api/projections.ts")

    rules = require_dict(map_data.get("rules"), "rules")
    if rules.get("not_runtime_authority") is not True:
        fail("rules.not_runtime_authority must be true")
    if rules.get("does_not_replace_rust_validation") is not True:
        fail("rules.does_not_replace_rust_validation must be true")
    if rules.get("unknown_unmapped_values_must_not_render_as_success") is not True:
        fail("unknown/unmapped values must be explicitly blocked from success rendering")
    if rules.get("fully_aligned_claim") is not False:
        fail("map must not claim full alignment")

    allowed_statuses = set(require_list(map_data.get("allowed_statuses"), "allowed_statuses"))
    if allowed_statuses != ALLOWED_STATUSES:
        fail("allowed_statuses must match validator status set")

    intent_values = schema_enum(schema, ("properties", "intent_type"))
    target_values = schema_enum(schema, ("properties", "target", "properties", "type"))
    validate_required_values(intent_values, mapping_by_schema_value(map_data, "intent_type_mappings"), "intent_type_mappings")
    validate_required_values(target_values, mapping_by_schema_value(map_data, "target_type_mappings"), "target_type_mappings")
    validate_projection_values(map_data)

    print("Operator intent contract map validation passed.")


if __name__ == "__main__":
    main()
