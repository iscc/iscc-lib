# Project State

## Status: IN_PROGRESS

## Phase: All 5 crates complete (core + 4 bindings + C FFI) — docs, benchmark comparison, publishing remain

Rust core is fully implemented with all 9 `gen_*_v0` functions passing 143 conformance tests. All 4
binding crates (Python/PyO3, Node.js/napi-rs, WASM/wasm-bindgen, C FFI/cbindgen) are complete with
tests passing. CI has 4 jobs (Rust, Python, Node.js, WASM) all green. Remaining work is
documentation site, benchmark comparison publishing, and CI/CD publishing pipelines.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 conformance
    tests, clippy/fmt clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3 `_lowlevel`, type stubs,
    `py.typed`, module name `iscc_lib`
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), all passing
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 gen functions via napi-rs, `@iscc/lib` package
- **Node.js tests (`crates/iscc-napi/__tests__/`)**: 46 conformance tests, passing in CI
- **WASM bindings (`crates/iscc-wasm/`)**: all 9 gen functions via wasm-bindgen, conformance tests
    passing in CI
- **C FFI (`crates/iscc-ffi/`)**: all 9 gen functions as `extern "C"`, `iscc_free_string()`,
    `iscc_last_error()`, cbindgen.toml, 20 unit tests
- **Criterion benchmarks (`crates/iscc-lib/benches/`)**: all 9 gen functions
- **pytest-benchmark (`benchmarks/python/`)**: iscc-core baseline for all 9 gen functions
- **Conformance vectors**: `crates/iscc-lib/tests/data.json` vendored, shared by Rust/Python/JS/WASM
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 5 crate members
- **CI workflow**: `.github/workflows/ci.yml` with 4 jobs — all green
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **C test program**: target requires a C test program calling entrypoints — not yet created
- **pytest-benchmark for Rust bindings**: iscc_lib vs iscc-core comparison — not started
- **Documentation site**: no lib.iscc.codes content or build setup
- **CI/CD publishing**: no OIDC trusted publishing pipelines for crates.io, PyPI, or npm
- **Benchmark results in docs**: speedup factors not yet published

## CI

- **Status: PASSING** — all 4 jobs green (Rust, Python, Node.js, WASM)
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22290628493

## Verification

- `cargo test --workspace`: **163 passed** (143 core + 20 FFI), 0 failed
- `cargo clippy --workspace -- -D warnings`: clean (core and FFI)
- `cargo fmt --all --check`: clean
- `uv run pytest`: **49 passed**, 0 failed
- Node.js CI job: **passing** (46 conformance tests)
- WASM CI job: **passing** (conformance tests via wasm-pack)
- cbindgen generates valid C header from `iscc-ffi`
- No `unsafe` in core crate; FFI `unsafe` confined to boundary

## Next Milestone

**C test program**: Create a minimal C program that links `iscc-ffi` and calls the entrypoints to
verify correctness, completing the C FFI target criterion. Alternatively, the documentation site
(lib.iscc.codes) or benchmark comparison could be tackled next — all are independent workstreams.
