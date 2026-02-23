# Project State

## Status: DONE

## Phase: Feature-complete — all CI green, all criteria met

All 9 `gen_*_v0` functions implemented in Rust core with 143 conformance tests passing. All 4
binding crates (Python, Node.js, WASM, C FFI) complete with conformance tests. CI workflow (5 jobs)
and Docs workflow (build + deploy) all green. Release workflow covers crates.io, PyPI, and npm.
Documentation site deploys via GitHub Pages.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 tests, clippy
    clean, fmt clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3, type stubs, `py.typed`
- **Python tests (`tests/`)**: 49 tests passing (46 conformance + 3 smoke)
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 functions via napi-rs, `@iscc/lib` package
- **Node.js tests**: 46 conformance tests in `__tests__/conformance.test.mjs`
- **WASM bindings (`crates/iscc-wasm/`)**: all 9 functions via wasm-bindgen, conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen.toml, 20 unit tests, C test
- **Criterion benchmarks**: all 9 gen functions benchmarked
- **Comparative pytest-benchmark**: iscc-core vs iscc_lib (1.3×–158× speedups measured)
- **Documentation**: 5 pages (landing, Python API, Rust API, architecture, benchmarks)
- **CI workflow**: `.github/workflows/ci.yml` — 5 jobs (Rust, Python, Node.js, WASM, C FFI), green
- **Release workflow**: `.github/workflows/release.yml` — OIDC for crates.io + PyPI + npm
- **Docs workflow**: `.github/workflows/docs.yml` — build + deploy, both jobs green
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 5 crate members
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer

## What's Missing

Nothing. All target criteria are met at the code level. The project is ready for first release.

## CI

- **CI workflow (ci.yml): PASSING** — all 5 jobs green
    (https://github.com/iscc/iscc-lib/actions/runs/22297413350)
- **Docs workflow (docs.yml): PASSING** — build + deploy both green
    (https://github.com/iscc/iscc-lib/actions/runs/22297413346)

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt -p iscc-lib --check`: clean
- No `unsafe` in core crate
- Python tests: **49 passed** (46 conformance + 3 smoke)
- Node.js: 46 conformance tests in CI
- WASM: conformance tests in CI
- C FFI: 20 unit tests + C test program in CI

## Next Milestone

Project is complete. Ready for v0.1.0 release — tag and publish via the release workflow.
