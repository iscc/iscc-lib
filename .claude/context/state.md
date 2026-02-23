# Project State

## Status: IN_PROGRESS

## Phase: All 5 crates + C test program complete — docs, benchmark publishing, CI/CD publishing remain

All 9 `gen_*_v0` functions are implemented in the Rust core with 143 conformance tests passing. All
4 binding crates (Python, Node.js, WASM, C FFI) are complete with tests. A C test program verifies
FFI correctness. CI has 5 jobs all green. Remaining work: documentation site, benchmark comparison
publishing, and OIDC publishing pipelines.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 conformance
    tests, clippy/fmt clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3 `_lowlevel`, type stubs,
    `py.typed`, module name `iscc_lib`
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), all passing
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 gen functions via napi-rs, `@iscc/lib` package
- **Node.js tests**: 46 conformance tests, passing in CI
- **WASM bindings (`crates/iscc-wasm/`)**: all 9 gen functions via wasm-bindgen, conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen.toml, 20 unit tests
- **C test program (`crates/iscc-ffi/tests/test_iscc.c`)**: links and runs in CI
- **Criterion benchmarks (`crates/iscc-lib/benches/`)**: all 9 gen functions benchmarked
- **pytest-benchmark (`benchmarks/python/`)**: iscc-core baseline for all 9 gen functions
- **Conformance vectors**: `data.json` vendored, shared by Rust/Python/JS/WASM
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 5 crate members
- **CI workflow**: 5 jobs (Rust, Python, Node.js, WASM, C FFI) — all green
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **Documentation site**: no lib.iscc.codes content, no build setup (target: GitHub Pages)
- **Benchmark comparison publishing**: speedup factors not yet documented; comparative
    pytest-benchmark (iscc_lib vs iscc-core) not implemented
- **CI/CD publishing pipelines**: no OIDC trusted publishing for crates.io, PyPI, or npm

## CI

- **Status: PASSING** — all 5 jobs green (Rust, Python, Node.js, WASM, C FFI)
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22290956499

## Verification

- `cargo test --workspace`: **163 passed** (143 core + 20 FFI), 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `uv run pytest`: **49 passed**, 0 failed
- Node.js CI: **passing** (46 conformance tests)
- WASM CI: **passing** (conformance tests via wasm-pack)
- C FFI CI: **passing** (cbindgen + gcc compile + test execution)

## Next Milestone

**Documentation site** (lib.iscc.codes): Set up a documentation build (e.g., mdBook or zola) with
GitHub Pages deployment covering Rust API, Python API, and architecture. This is the largest
remaining gap. Alternatively, benchmark comparison or OIDC publishing pipelines could be tackled —
all are independent workstreams.
