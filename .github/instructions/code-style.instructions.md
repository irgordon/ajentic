---
applyTo: "core/**/*.rs,ui/**/*.ts,ui/**/*.tsx,scripts/**/*.sh"
---

# Code style instructions

This file provides coding style guidance.

It is not governance.

## General

Prefer simple code over clever code.

Prefer small functions.

Prefer explicit names.

Avoid broad abstractions before there are repeated concrete cases.

Avoid hidden behavior.

Avoid implicit authority.

Avoid long, redundant, or decorative comments.

## Rust

Rust is the authoritative runtime surface.

Rust code should make authority boundaries explicit.

Use typed errors for rejection paths.

Do not hide policy failures behind generic errors.

Do not implement lifecycle, validation, ledger, replay, or memory decisions outside Rust authority paths.

Run `cargo fmt --manifest-path core/Cargo.toml` before running `cargo fmt --manifest-path core/Cargo.toml -- --check`.

## TypeScript

TypeScript is a visibility and operator-intent surface.

UI code must not create runtime authority.

UI code must not directly mutate state, memory, ledger, replay, policy, or validation results.

Generated types must not be hand-edited.

Use generated runtime validators from JSON Schema where the project provides them.

## Bash

Bash is operator glue.

Scripts should call Rust CLI behavior.

Scripts must not implement policy.

Scripts must not directly edit authoritative runtime data.

Scripts must remain replaceable by direct Rust CLI invocation.

Local validation should use `./scripts/check.sh` when available.