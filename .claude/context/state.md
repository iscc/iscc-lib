# Project State

## Status: IN_PROGRESS

## Phase: Core complete, bindings complete, CI/docs/publishing polish remaining

All 9 `gen_*_v0` functions are implemented in the Rust core with 143 conformance tests passing. All
5 binding crates (Python, Node.js, WASM, C FFI, plus core) are complete with conformance tests. The
main CI workflow (5 jobs) is green. Docs have 4 content pages (landing, Python API, Rust API,
architecture). The docs workflow fails because GitHub Pages is not enabled in repo settings.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 tests, clippy/fmt
    clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3, type stubs, `py.typed`
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), passing
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 gen functions via napi-rs, `@iscc/lib` package
- **Node.js tests**: 46 conformance tests in `__tests__/conformance.test.mjs`
- **WASM bindings (`crates/iscc-wasm/`)**: all 9 gen functions via wasm-bindgen, conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen.toml, 20 unit tests, C test
- **Criterion benchmarks (`crates/iscc-lib/benches/`)**: all 9 gen functions
- **Comparative pytest-benchmark (`benchmarks/python/`)**: iscc-core vs iscc_lib bindings
- **Documentation**: `docs/index.md` (landing), `docs/api.md` (Python API ref), `docs/rust-api.md`
    (Rust API ref), `docs/architecture.md` (crate model, modules, conformance)
- **Docs deployment**: `.github/workflows/docs.yml` — exists but fails (Pages not enabled)
- **Release workflow**: `.github/workflows/release.yml` — OIDC publishing for crates.io and PyPI
- **CI workflow**: `.github/workflows/ci.yml` — 5 jobs (Rust, Python, Node.js, WASM, C FFI) — green
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 5 crate members
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **Docs deploy blocked**: GitHub Pages not enabled in repo settings (admin action, not code fix)
- **Documentation content**: no benchmark results page
- **npm publishing**: release workflow covers crates.io + PyPI but not npm (`@iscc/lib`,
    `@iscc/wasm`)
- **CI has "docs" job failing**: docs workflow triggers on push to main but Pages 404s

## CI

- **CI workflow (ci.yml): PASSING** — all 5 jobs green (Rust, Python, Node.js, WASM, C FFI)
- **Docs workflow (docs.yml): FAILING** — "docs" job fails (Pages not enabled in repo settings)
- Latest run: https://github.com/iscc/iscc-lib/actions/runs/22293382281

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- Python tests: **49 passed** (46 conformance + 3 smoke)
- Node.js: conformance tests present (`__tests__/conformance.test.mjs`)
- WASM: conformance tests present (`crates/iscc-wasm/tests/conformance.rs`)
- C FFI: unit tests + C test program present (`crates/iscc-ffi/tests/test_iscc.c`)

## Next Milestone

Add benchmark results page to documentation. Add npm publishing jobs to the release workflow for
`@iscc/lib` and `@iscc/wasm`. The docs deploy failure requires Titusz to enable GitHub Pages
(Settings → Pages → Source: GitHub Actions) — flag it but don't block other work on it.
