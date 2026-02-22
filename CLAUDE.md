# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Summary

**iscc-lib** is a high-performance polyglot (Rust + language bindings) implementation of ISO 24138:2024
International Standard Content Code (ISCC). It must be compatible with the official `iscc/iscc-core`
Python reference implementation.

The project goal and Milestone 1 targets are defined in `.claude/context/ideal-state.md`.

## Current State

This repository is in the **planning/design phase**. The `notes/` directory contains 10 architectural
documents (00 through 09) covering every aspect of the target implementation. There is no source code
yet.

## Architecture

Detailed architecture is documented in `notes/00-overview.md` and its sub-documents.

### Core Principles

- **Sync core, streaming interface**: The Rust core is synchronous (CPU-bound, no I/O). Each binding
  adapts async idiomatically. Pattern: `new() → update(&[u8]) → finalize() → Result<T>`
- **Tiered API surface**: Only `iscc::api` (Tier 1) is bound to foreign languages. Internal modules
  use `pub(crate)` and are never exposed across FFI
- **Hub-and-spoke crate model**: All binding crates depend on the pure-Rust `iscc` core crate

### Target Directory Layout

```
iscc-lib/
├── Cargo.toml              # Virtual workspace root
├── mise.toml               # Tool versions + task runner
├── pyproject.toml           # Python project (uv + maturin)
├── crates/
│   ├── iscc/               # Pure Rust core (Phase 0-1)
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── api.rs      # Tier 1: stable entrypoints bound everywhere
│   │       ├── meta.rs, content.rs, data.rs, instance.rs, codec.rs
│   └── iscc-py/            # Python bindings via PyO3 + maturin (Phase 1)
│       ├── src/lib.rs
│       └── python/iscc_lib/
```

Phase 2 adds `iscc-node/` (napi-rs) and `iscc-wasm/` (wasm-bindgen).
Phase 3 adds `iscc-ffi/` (C FFI via cbindgen) and `iscc-cli/`.

### Core Entrypoints (Milestone 1)

All `*_v0` functions must match the `iscc-core` reference implementation:
`gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`,
`gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`, `gen_instance_code_v0`,
`gen_iscc_code_v0`

### Conformance Testing

Official test vectors from `iscc-core/data.json` are the correctness baseline. The Rust test harness
must pass all vectors before any bindings are added. Target: 100% test coverage.

## Build & Development Commands (Target)

Tool versions and tasks are managed by **mise** (`mise.toml`). Python environment uses **uv**.

```bash
mise install              # Install all tool versions
mise run setup            # Install dev dependencies
mise run build:rust       # Build Rust core
mise run build:python     # Build Python bindings (maturin)
mise run test:rust        # Run Rust tests (cargo test)
mise run test:python      # Run Python tests (pytest)
mise run lint             # Format checks + clippy + ruff + ty
mise run format           # Apply formatting
mise run docs:serve       # Serve documentation locally (zensical)
```

Individual Rust test: `cargo test -p iscc test_name`
Individual Python test: `pytest tests/test_file.py::test_name`

## Tooling Stack

| Concern | Tool |
|---------|------|
| Tool versions + tasks | mise |
| Rust release | cargo-dist |
| Python build | maturin + PyO3 |
| Python env | uv |
| Python lint/format | ruff |
| Python types | ty (astral-sh, replaces mypy) |
| Pre-commit | prek |
| Documentation | zensical (hosted at lib.iscc.codes) |
| Supply chain | cargo deny |
| Rust benchmarks | criterion |
| Python benchmarks | pytest-benchmark |
| Version sync | scripts/sync_versions.py |

## Key Architectural References

| Document | When to consult |
|----------|----------------|
| `notes/00-overview.md` | Starting point, phases, key decisions |
| `notes/01-workspace-structure.md` | Creating crates, workspace config |
| `notes/02-language-bindings.md` | Adding or modifying binding crates |
| `notes/03-async-and-streaming.md` | Core API design, streaming processors |
| `notes/04-api-compatibility-safety.md` | API tiers, error handling, conformance |
| `notes/05-developer-tooling.md` | Dev environment, CI quality gates |
| `notes/06-build-cicd-publishing.md` | CI config, release workflows |
| `notes/07-security-versioning.md` | Security setup, release flow |
| `notes/09-performance-benchmarks.md` | Benchmark strategy, performance tracking |

## Prior Art

This project restructures and extends `bio-codes/iscc-sum`, carrying forward: CDC algorithm, BLAKE3
hashing, streaming `update()/result()` pattern, release profile settings, and Python tooling. The
`iscc-sum` crate achieved 50-130x speedup over the Python reference with sync streaming.

## Design Constraints

- `workspace.dependencies` in root `Cargo.toml` for centralized version management
- Python bindings use `abi3-py310` (one wheel per platform, not per Python version)
- Release profile: `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"`
- No network access in tests (conformance vectors vendored as files)
- OIDC trusted publishing for crates.io and PyPI (no long-lived API keys)
