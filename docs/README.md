# SigmaOS-gsettings-desktop-schemas — Documentation

## Purpose

SigmaOS gsettings-desktop-schemas component inspired by Linux Mint / Omarchy / Arch Linux.

## Architecture

- `src/lib.rs` — public API (ini archetype)
- `tests/` — integration tests
- Mechanism here; policy lives in configuration.

## Principles applied

Abstraction, loose coupling, high cohesion, KISS, DRY, secure defaults,
fail-fast, defensive programming, deterministic testing.
