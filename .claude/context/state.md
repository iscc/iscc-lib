# Project State

## Status: IN_PROGRESS

## Phase: Documentation scaffold created — content pages, GitHub Pages deploy, and OIDC publishing remain

All 9 `gen_*_v0` functions are implemented in the Rust core with 143 conformance tests passing. All
4 binding crates (Python, Node.js, WASM, C FFI) are complete with tests. CI has 5 jobs all green.
Benchmarks exist. Zensical documentation scaffold (`zensical.toml`, `docs/index.md`) was added but
only has a landing page — no API reference, architecture, or GitHub Pages deployment yet.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 tests, clippy/fmt
    clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3, type stubs, `py.typed`,
    `iscc_lib` module
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), passing in CI
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 gen functions via napi-rs, `@iscc/lib` package
- **Node.js tests**: 46 conformance tests, passing in CI
- **WASM bindings (`crates/iscc-wasm/`)**: all 9 gen functions via wasm-bindgen, conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen.toml, 20 unit tests, C test
    program
- **Criterion benchmarks (`crates/iscc-lib/benches/`)**: all 9 gen functions
- **Comparative pytest-benchmark (`benchmarks/python/`)**: iscc-core baseline and iscc_lib bindings
- **Documentation scaffold**: `zensical.toml` config + `docs/index.md` landing page (builds locally)
- **Conformance vectors**: `data.json` vendored, shared by Rust/Python/JS/WASM
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 5 crate members
- **CI workflow**: 5 jobs (Rust, Python, Node.js, WASM, C FFI) — all green
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **Documentation content**: only landing page exists — no API reference pages (Rust, Python), no
    architecture page, no benchmark results published
- **GitHub Pages deployment**: no workflow to build/deploy docs to lib.iscc.codes
- **CI/CD publishing pipelines**: no OIDC trusted publishing for crates.io, PyPI, or npm

## CI

- **Status: PASSING** — all 5 jobs green (Rust, Python, Node.js, WASM, C FFI)
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22291677768

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo test -p iscc-ffi`: **20 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- Python tests: **49 passed** in CI (local requires `maturin develop` first)
- Node.js CI: **passing** (46 conformance tests)
- WASM CI: **passing** (conformance tests via wasm-pack)
- C FFI CI: **passing** (cbindgen + gcc compile + test execution)

## Next Milestone

**Expand documentation**: Add API reference pages (Python via mkdocstrings, Rust via rustdoc link),
architecture overview, and benchmark results to the zensical site. Then add a GitHub Pages
deployment workflow. OIDC publishing pipelines are an independent workstream that could proceed in
parallel.
