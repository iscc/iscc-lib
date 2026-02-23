# Project State

## Status: IN_PROGRESS

## Phase: Rust core + Python + Node.js bindings complete, WASM scaffold exists — WASM tests, C FFI, docs, publishing remain

Rust core is fully implemented with all 9 `gen_*_v0` functions passing 143 conformance tests. Python
bindings (PyO3) and Node.js bindings (napi-rs) are complete with conformance tests and CI jobs. A
WASM bindings crate has been scaffolded with all 9 wrappers and compiles for
`wasm32-unknown-unknown`, but has no conformance tests and no CI job yet.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 conformance
    tests, clippy/fmt clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3 `_lowlevel`, type stubs,
    `py.typed`, module name `iscc_lib`
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), all passing
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 gen functions via napi-rs, `@iscc/lib` package
- **Node.js tests (`crates/iscc-napi/__tests__/`)**: 46 conformance tests, all passing in CI
- **WASM bindings (`crates/iscc-wasm/`)**: all 9 gen functions via wasm-bindgen, builds for
    wasm32-unknown-unknown — **no tests, no CI job**
- **Criterion benchmarks (`crates/iscc-lib/benches/`)**: all 9 gen functions
- **pytest-benchmark (`benchmarks/python/`)**: iscc-core baseline for all 9 gen functions
- **Conformance vectors**: `crates/iscc-lib/tests/data.json` vendored, shared by Rust/Python/JS
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 4 crate members
- **CI workflow**: `.github/workflows/ci.yml` with Rust, Python, Node.js jobs — all green
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **WASM conformance tests**: no tests for the WASM crate yet
- **WASM CI job**: no CI job for building/testing WASM
- **C FFI**: cbindgen headers and shared library — not started
- **pytest-benchmark for Rust bindings**: iscc_lib vs iscc-core comparison — not started
- **Documentation site**: no lib.iscc.codes content or build setup
- **CI/CD publishing**: no OIDC trusted publishing pipelines for crates.io, PyPI, or npm
- **Benchmark results in docs**: speedup factors not yet published

## CI

- **Status: PASSING** — all 3 jobs green (Rust, Python, Node.js)
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22289653347

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt --all --check`: clean
- `uv run pytest`: **49 passed**, 0 failed
- Node.js CI job: **passing** (46 conformance tests)
- WASM crate: **builds** for wasm32-unknown-unknown, no tests yet
- No `unsafe` in core crate

## Next Milestone

**WASM conformance tests + CI job**: Add conformance tests for the WASM crate (runnable in a WASM
runtime like `wasm-pack test --node`) and a CI job in the workflow. This completes the WASM bindings
target criterion.
