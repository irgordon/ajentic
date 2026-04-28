# Domain Authoring

This file reserves the location for the AJENTIC domain authoring guide.

## Purpose

A domain is a distinct problem class with its own objectives, constraints, evaluation criteria, and failure modes. Domain definitions enable cross-domain generalization without architectural redesign.

## Domain structure

See `schemas/domain.schema.json` for the canonical schema.
See `examples/minimal_run/domain.yaml` for a minimal example.

## Authoring guidelines

- Each domain must define its objective types, constraint types, evaluators, and known failure modes.
- Domain definitions are declarative YAML files.
- Domains must not contain governance logic.

## v0.0.0 status

This file reserves the location for domain authoring guidance. Implementation will be added in later phases.
