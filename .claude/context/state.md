# Project State

## Status: IN_PROGRESS

## Phase: All 5 crates + benchmarks complete — documentation site and OIDC publishing remain

All 9 `gen_*_v0` functions are implemented in the Rust core with 143 conformance tests passing. All
4 binding crates (Python, Node.js, WASM, C FFI) are complete with tests. CI has 5 jobs all green.
Criterion benchmarks and comparative pytest-benchmarks (iscc_lib vs iscc-core) are implemented.
Remaining work: documentation site and OIDC publishing pipelines.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 tests, clippy/fmt
    clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3, type stubs, `py.typed`,
    `iscc_lib` module
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), all passing
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 gen functions via napi-rs, `@iscc/lib` package
- **Node.js tests**: 46 conformance tests, passing in CI
- **WASM bindings (`crates/iscc-wasm/`)**: all 9 gen functions via wasm-bindgen, conformance tests
    passing
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen.toml, 20 unit tests, C test
    program
- **Criterion benchmarks (`crates/iscc-lib/benches/`)**: all 9 gen functions
- **Comparative pytest-benchmark (`benchmarks/python/`)**: both iscc-core baseline and iscc_lib
    bindings
- **Conformance vectors**: `data.json` vendored, shared by Rust/Python/JS/WASM
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 5 crate members
- **CI workflow**: 5 jobs (Rust, Python, Node.js, WASM, C FFI) — all green
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **Documentation site**: no lib.iscc.codes content, no build setup (target: GitHub Pages with Rust
    API, Python API, architecture)
- **Benchmark speedup factors in docs**: comparative benchmarks exist but results not published in
    documentation
- **CI/CD publishing pipelines**: no OIDC trusted publishing for crates.io, PyPI, or npm

## CI

- **Status: PASSING** — all 5 jobs green (Rust, Python, Node.js, WASM, C FFI)
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22291293408

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo test -p iscc-ffi`: **20 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `uv run pytest tests/`: **49 passed**, 0 failed
- Node.js CI: **passing** (46 conformance tests)
- WASM CI: **passing** (conformance tests via wasm-pack)
- C FFI CI: **passing** (cbindgen + gcc compile + test execution)

## Next Milestone

**Documentation site** (lib.iscc.codes): Set up a documentation build (mdBook or similar) with
GitHub Pages deployment covering Rust API, Python API, and architecture — styled like
`iscc/iscc-usearch` (zensical). This is the largest remaining gap before the project can be
considered complete. Alternatively, OIDC publishing pipelines are independent and could be tackled
in parallel.
