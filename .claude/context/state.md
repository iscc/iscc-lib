# Project State

## Status: IN_PROGRESS

## Phase: All 4 binding crates complete with CI — C FFI, docs, benchmarks, publishing remain

Rust core is fully implemented with all 9 `gen_*_v0` functions passing 143 conformance tests. All 4
binding crates (Python/PyO3, Node.js/napi-rs, WASM/wasm-bindgen) have conformance tests passing and
CI jobs green. Remaining work is C FFI, documentation site, benchmark comparison, and publishing.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 conformance
    tests, clippy/fmt clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3 `_lowlevel`, type stubs,
    `py.typed`, module name `iscc_lib`
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), all passing
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 gen functions via napi-rs, `@iscc/lib` package
- **Node.js tests (`crates/iscc-napi/__tests__/`)**: 46 conformance tests, all passing in CI
- **WASM bindings (`crates/iscc-wasm/`)**: all 9 gen functions via wasm-bindgen, conformance tests
    passing in CI via `wasm-pack test --node`
- **Criterion benchmarks (`crates/iscc-lib/benches/`)**: all 9 gen functions
- **pytest-benchmark (`benchmarks/python/`)**: iscc-core baseline for all 9 gen functions
- **Conformance vectors**: `crates/iscc-lib/tests/data.json` vendored, shared by Rust/Python/JS/WASM
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 4 crate members
- **CI workflow**: `.github/workflows/ci.yml` with 4 jobs — all green
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **C FFI**: cbindgen headers and shared library — not started
- **pytest-benchmark for Rust bindings**: iscc_lib vs iscc-core comparison — not started
- **Documentation site**: no lib.iscc.codes content or build setup
- **CI/CD publishing**: no OIDC trusted publishing pipelines for crates.io, PyPI, or npm
- **Benchmark results in docs**: speedup factors not yet published

## CI

- **Status: PASSING** — all 4 jobs green (Rust, Python, Node.js, WASM)
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22290276896

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt --all --check`: clean
- `uv run pytest`: **49 passed**, 0 failed
- Node.js CI job: **passing** (46 conformance tests)
- WASM CI job: **passing** (conformance tests via wasm-pack)
- No `unsafe` in core crate

## Next Milestone

**C FFI crate**: Create `crates/iscc-ffi/` with cbindgen-generated C headers and a shared library
exposing all 9 `gen_*_v0` functions. This is the next unstarted target criterion. Includes a C test
program to verify correctness.
