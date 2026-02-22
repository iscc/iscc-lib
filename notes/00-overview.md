# iscc-lib: Polyglot Rust Architecture Overview

High-performance polyglot (Rust + bindings) implementation of ISO 24138:2024 International Standard
Content Code (ISCC), compatible with the official `iscc/iscc-core` reference implementation.

Based on research from `mitsuhiko/minijinja` and `kreuzberg-dev/kreuzberg` — two production polyglot
Rust projects — combined with lessons from `bio-codes/iscc-sum`, an existing partial Rust ISCC
implementation with Python bindings.

> Note (2026-02-20): Versions, MSRVs, and workflow snippets in the sub-documents are illustrative
> snapshots. Prefer linked upstream docs for exact, current details.

## Sub-Documents

| #   | File                                                             | Topic                                                                     | When to load                                   |
| --- | ---------------------------------------------------------------- | ------------------------------------------------------------------------- | ---------------------------------------------- |
| 1   | [01-workspace-structure.md](01-workspace-structure.md)           | Repository layout, Cargo workspace design, crate architecture             | Setting up the repo, creating crates           |
| 2   | [02-language-bindings.md](02-language-bindings.md)               | Python/PyO3, Node.js/napi-rs, WASM, C FFI, UniFFI/Diplomat                | Adding or modifying any binding crate          |
| 3   | [03-async-and-streaming.md](03-async-and-streaming.md)           | Sync core principle, streaming API shape, per-binding async               | Designing the core API or streaming processors |
| 4   | [04-api-compatibility-safety.md](04-api-compatibility-safety.md) | API tiers, FFI stability, feature flags, safety, error model, conformance | API design, error handling, testing            |
| 5   | [05-developer-tooling.md](05-developer-tooling.md)               | mise, task runners, linting, quality gates, docs, pre-commit              | Setting up dev environment, configuring CI     |
| 6   | [06-build-cicd-publishing.md](06-build-cicd-publishing.md)       | Binary size, release profiles, CI matrices, publishing workflows          | Configuring CI, release workflows              |
| 7   | [07-security-versioning.md](07-security-versioning.md)           | Supply chain, cargo-deny, dependabot, version sync                        | Security setup, release flow                   |
| 8   | [08-references.md](08-references.md)                             | All tool and project links organized by category                          | Quick lookup of upstream docs                  |
| 9   | [09-performance-benchmarks.md](09-performance-benchmarks.md)     | Benchmark matrix, report format, CI integration, tooling                  | Benchmarking, performance tracking             |

## Project Comparison

| Aspect                 | minijinja                                 | kreuzberg                                                     |
| ---------------------- | ----------------------------------------- | ------------------------------------------------------------- |
| **Author**             | Armin Ronacher (creator of Flask, Jinja2) | Na'aman Hirschfeld                                            |
| **Purpose**            | Template engine (Jinja2 for Rust)         | Document intelligence (OCR, extraction)                       |
| **Rust edition**       | 2021                                      | 2024                                                          |
| **MSRV**               | 1.70                                      | 1.91                                                          |
| **Bindings**           | Python, JS/WASM, C                        | Python, Node.js, WASM, C FFI, Ruby, Go, Java, C#, PHP, Elixir |
| **Task runner**        | Make                                      | go-task (Taskfile.yml)                                        |
| **Python build**       | maturin + PyO3                            | maturin + PyO3                                                |
| **Node.js build**      | wasm-pack (WASM only)                     | napi-rs (native) + wasm-pack (browser)                        |
| **C bindings**         | cbindgen (minijinja-cabi)                 | Custom FFI crate (kreuzberg-ffi)                              |
| **Release automation** | cargo-dist + maturin-action               | Custom publish.yaml workflow                                  |
| **Workspace deps**     | Per-crate versions                        | workspace.dependencies (centralized)                          |
| **Complexity**         | Moderate (7 crates)                       | High (10+ crates, 8 language packages)                        |

**Key takeaway**: minijinja's focused approach is the better starting model for `iscc-lib`, adopting
kreuzberg's patterns selectively as bindings are added.

## Development Phases

### Phase 0: Conformance Baseline

Establish correctness before adding bindings:

1. Import official conformance vectors (`iscc_core/data.json`) as a pinned snapshot
2. Wire up a Rust test harness that runs them in CI
3. Keep Rust APIs aligned with reference entrypoints (`*_v0` functions)
4. Only after Rust core passes vectors, add bindings running the same suite

### Phase 1: Core + Python

Minimum viable polyglot setup — pure Rust core (`crates/iscc/`) + Python bindings
(`crates/iscc-py/`) via PyO3 + maturin. See [01-workspace-structure.md](01-workspace-structure.md)
for the full directory layout.

### Phase 2: Node.js + WASM

Add `crates/iscc-node/` (napi-rs native addon) and `crates/iscc-wasm/` (browser WASM via
wasm-bindgen). See [02-language-bindings.md](02-language-bindings.md).

### Phase 3: C FFI + CLI

Add `crates/iscc-ffi/` (C FFI via cbindgen, enables Go/Java/C#) and `crates/iscc-cli/` (command-line
tool). See [02-language-bindings.md](02-language-bindings.md).

### Migration from iscc-sum

**Carry forward**: CDC algorithm, BLAKE3 hashing, streaming `update()/result()` pattern, release
profile settings, CI matrix, Python tooling (uv, ruff, bandit, pytest) — replace mypy with ty.

**Restructure**:

| iscc-sum (current)                           | iscc-lib (target)                              | Why                                                  |
| -------------------------------------------- | ---------------------------------------------- | ---------------------------------------------------- |
| Single crate with `[lib]` + `[[bin]]` + PyO3 | Workspace with separate crates                 | Core must be usable without PyO3 dependency          |
| `Cargo.toml` at root                         | `crates/iscc/Cargo.toml`                       | Workspace pattern for multiple binding crates        |
| PyO3 in same crate as core                   | `crates/iscc-py/` separate from `crates/iscc/` | Core crate stays pure Rust, publishable to crates.io |
| `poethepoet` for tasks                       | `mise tasks`                                   | Unifies tool versions + tasks in one tool            |
| `src/main.rs` CLI in same crate              | `crates/iscc-cli/`                             | CLI is a consumer of the core API, not part of it    |
| `clap`, `walkdir`, `globset` in core deps    | Move to CLI crate only                         | Core crate should not depend on CLI libraries        |

**Extraction steps**:

1. Create workspace root `Cargo.toml` with `workspace.dependencies`
2. Move core algorithms (CDC, MinHash, BLAKE3 hashing, codec) to `crates/iscc/`
3. Remove PyO3 dependency from core — it becomes a pure Rust library
4. Create `crates/iscc-py/` that depends on `crates/iscc/` and adds PyO3 bindings
5. Move CLI to `crates/iscc-cli/` with `clap`, `walkdir`, `globset` as its own dependencies
6. Expand core to implement remaining ISCC units (Content-Code, Meta-Code, ISCC-CODE)

## Tooling Stack

| Concern           | Tool                        | Why                                                                |
| ----------------- | --------------------------- | ------------------------------------------------------------------ |
| Tool versions     | **mise**                    | Unified polyglot manager, replaces sdkman/nvm/pyenv                |
| Task runner       | **mise tasks**              | Integrated with tool manager, TOML config, cross-platform          |
| Rust release      | **cargo-dist**              | Auto-generates CI, handles crates.io + GitHub Releases             |
| Python build      | **maturin** + PyO3          | Industry standard for Rust→Python                                  |
| Python env        | **uv**                      | Fast lockfiles, venv management                                    |
| Python lint       | **ruff**                    | Replaces black + flake8 + isort                                    |
| Python types      | **ty** (astral-sh)          | Fast Rust-based type checker, 10-100x faster than mypy             |
| Python security   | **bandit**                  | Static security scanner (already used in iscc-sum)                 |
| Node.js build     | **napi-rs**                 | Native performance, prebuilt binaries                              |
| Node.js pkg       | **pnpm**                    | Efficient, workspace-aware                                         |
| JS lint           | **biome**                   | Replaces eslint + prettier                                         |
| WASM build        | **wasm-pack**               | Standard for Rust→WASM                                             |
| C headers         | **cbindgen**                | Auto-generates C headers from Rust                                 |
| Pre-commit        | **prek**                    | Rust-based pre-commit framework, drop-in replacement, 10x faster   |
| Markdown fmt      | **mdformat-mkdocs**         | Markdown formatting in pre-commit hooks                            |
| Documentation     | **zensical**                | Rust-powered static site generator (Material for MkDocs successor) |
| API compat        | **cargo-semver-checks**     | Catches breaking changes in core crate before they reach bindings  |
| Supply chain      | **cargo deny**              | License, advisory, and duplicate dependency checks                 |
| Version sync      | Custom script               | Python script to sync versions across manifests                    |
| Rust benchmarks   | **criterion**               | Statistical microbenchmarks with JSON output                       |
| Python benchmarks | **pytest-benchmark**        | Side-by-side iscc-core vs iscc-lib benchmarks                      |
| Perf tracking     | **CodSpeed** or **Bencher** | Historical regression tracking in CI                               |

## Key Design Decisions

1. **Use kreuzberg's `crates/` directory pattern** — cleaner than minijinja's flat layout once you
    have 4+ binding crates. → [01-workspace-structure.md](01-workspace-structure.md)

2. **Use workspace.dependencies** — centralize dependency versions to avoid drift across crates. →
    [01-workspace-structure.md](01-workspace-structure.md)

3. **Use mise over sdkman/manual tooling** — single config file for all tool versions,
    cross-platform, includes task running. → [05-developer-tooling.md](05-developer-tooling.md)

4. **Separate CI workflows per concern** — don't try to do everything in one workflow. Use separate
    workflows for testing, Rust release, and Python wheels (minijinja pattern). Migrate to
    kreuzberg's unified pattern only when you have 4+ registries. →
    [06-build-cicd-publishing.md](06-build-cicd-publishing.md)

5. **Use OIDC trusted publishing** — for crates.io and PyPI, avoid long-lived API keys entirely. →
    [07-security-versioning.md](07-security-versioning.md)

6. **Start with Python bindings** — the existing iscc-core ecosystem is Python, so this provides
    the highest adoption impact and lets you validate API design before adding more bindings. →
    [02-language-bindings.md](02-language-bindings.md)

7. **Sync core, streaming interface** — the Rust core is synchronous with `update()/finalize()`
    streaming. Each binding adapts to its runtime's async model. Never expose Rust async across
    FFI. → [03-async-and-streaming.md](03-async-and-streaming.md)

8. **Tiered API surface** — only `iscc::api` is bound. Internal modules are not exposed. This
    prevents accidental ABI/API drift across languages. →
    [04-api-compatibility-safety.md](04-api-compatibility-safety.md)

9. **Supply-chain hygiene from day one** — `cargo deny`, `bandit`, dependabot, signed tags, OIDC
    publishing. → [07-security-versioning.md](07-security-versioning.md)

10. **Continuous performance benchmarks** — every ISCC unit and key internal operation is
    benchmarked across all language targets against the Python reference. Results are published to
    the project docs with speedup factors. CI tracks regressions. →
    [09-performance-benchmarks.md](09-performance-benchmarks.md)
