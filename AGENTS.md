# AGENTS.md — LLM Coding Agent Guide

## Project

AJENTIC

## Current phase

Bootstrap only (v0.0.0)

## Goal

Create a clean, understandable project skeleton. Do not implement runtime behavior.

## Language ownership

| Language   | Role        |
|------------|-------------|
| Rust       | authority   |
| Python     | adaptation  |
| TypeScript | visibility  |
| Bash       | glue        |
| Go         | optional later, not part of bootstrap |

## Agents may

- Create placeholder files
- Create documentation
- Create scaffolding
- Create schemas
- Create examples
- Create and update check scripts

## Agents must not add

- Runtime governance
- Provider integrations
- Model calls
- Evaluators
- Ledger persistence
- Replay logic
- Policy engines
- Dashboards
- APIs
- Database setup
- Docker setup
- Third-party dependencies

## Final report requirements

Every agent session must produce a final report with:

1. Files created or changed.
2. Commands run.
3. Whether checks passed.
4. Any deviations from the plan.
