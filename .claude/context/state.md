# Project State

## Status: IN_PROGRESS

## Phase: Feature-complete — blocked on repo admin actions only

All 9 `gen_*_v0` functions implemented in Rust core with 143 conformance tests passing. All 4
binding crates (Python, Node.js, WASM, C FFI) complete with conformance tests. CI workflow (5 jobs)
green. Release workflow covers crates.io, PyPI, and npm (`@iscc/lib`, `@iscc/wasm`). Documentation
site has 5 pages. Only remaining blocker: GitHub Pages not enabled in repo settings.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 tests, clippy
    clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3, type stubs, `py.typed`
- **Python tests (`tests/`)**: 49 tests passing (46 conformance + 3 smoke)
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 functions via napi-rs, `@iscc/lib` package
- **Node.js tests**: 46 conformance tests in `__tests__/conformance.test.mjs`
- **WASM bindings (`crates/iscc-wasm/`)**: all 9 functions via wasm-bindgen, conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen.toml, 20 unit tests, C test
- **Criterion benchmarks**: all 9 gen functions benchmarked
- **Comparative pytest-benchmark**: iscc-core vs iscc_lib (1.3×–158× speedups measured)
- **Documentation**: landing, Python API, Rust API, architecture, benchmarks (5 pages)
- **CI workflow**: `.github/workflows/ci.yml` — 5 jobs (Rust, Python, Node.js, WASM, C FFI), green
- **Release workflow**: `.github/workflows/release.yml` — OIDC for crates.io + PyPI + npm
- **Docs workflow**: `.github/workflows/docs.yml` — exists but fails (Pages not enabled)
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 5 crate members
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer

## What's Missing

- **Docs deploy blocked**: GitHub Pages not enabled in repo settings (admin action required —
    Settings → Pages → Source: GitHub Actions). No code change can fix this.

## CI

- **CI workflow (ci.yml): PASSING** — all 5 jobs green
    (https://github.com/iscc/iscc-lib/actions/runs/22296972500)
- **Docs workflow (docs.yml): FAILING** — Pages not enabled in repo settings
    (https://github.com/iscc/iscc-lib/actions/runs/22296972511)

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- Python tests: **49 passed** (46 conformance + 3 smoke)
- Node.js: 46 conformance tests present
- WASM: conformance tests present
- C FFI: 20 unit tests + C test program present

## Next Milestone

Enable GitHub Pages in repo settings (Settings → Pages → Source: GitHub Actions). This is the sole
remaining action item and requires repository admin access. Once Pages is enabled, the docs workflow
will pass and all CI will be green. All code-level work for the target state is complete.
