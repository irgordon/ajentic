#!/usr/bin/env python3
"""Fail the aggregate CI job unless every prerequisite explicitly succeeded."""

import json
import os
import sys

RESULTS_VARIABLE = "AJENTIC_CI_PREREQUISITES"
SUCCESS = "success"


def main():
    try:
        failures = failed_prerequisites(load_results())
    except (KeyError, ValueError) as error:
        print(f"::error::Invalid CI prerequisite evidence: {error}")
        return 1
    if failures:
        print(f"::error::CI prerequisites did not succeed: {', '.join(failures)}")
        return 1
    print("Every CI prerequisite explicitly succeeded.")
    return 0


def failed_prerequisites(results):
    if not isinstance(results, dict) or not results:
        raise ValueError("expected non-empty prerequisite results")
    return [name for name, result in results.items() if not successful_result(result)]


def successful_result(result):
    return isinstance(result, dict) and result.get("result") == SUCCESS


def load_results():
    return json.loads(os.environ[RESULTS_VARIABLE])


if __name__ == "__main__":
    sys.exit(main())
