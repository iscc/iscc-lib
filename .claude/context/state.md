# Project State

## Status: IN_PROGRESS

## Phase: CI green, docs deploy blocked on repo settings, release workflow added

All 9 `gen_*_v0` functions are implemented in the Rust core with 143 conformance tests passing. All
5 binding crates (Python, Node.js, WASM, C FFI, plus core) are complete with conformance tests. The
main CI workflow (5 jobs) is green. An OIDC release workflow for crates.io and PyPI was added. Docs
site has 3 pages but deployment fails because GitHub Pages is not enabled in repo settings.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 tests, clippy/fmt
    clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3, type stubs, `py.typed`
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), passing in CI
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 gen functions via napi-rs, `@iscc/lib` package
- **Node.js tests**: 46 conformance tests, passing in CI
- **WASM bindings (`crates/iscc-wasm/`)**: all 9 gen functions via wasm-bindgen, conformance tests
- **C FFI (`crates/iscc-ffi/`)**: 11 extern "C" symbols, cbindgen.toml, 20 unit tests, C test
- **Criterion benchmarks (`crates/iscc-lib/benches/`)**: all 9 gen functions
- **Comparative pytest-benchmark (`benchmarks/python/`)**: iscc-core vs iscc_lib bindings
- **Documentation**: `docs/index.md` (landing), `docs/api.md` (Python API ref),
    `docs/architecture.md` (crate model, modules, streaming, conformance)
- **Docs deployment**: `.github/workflows/docs.yml` — exists but fails (Pages not enabled)
- **Release workflow**: `.github/workflows/release.yml` — OIDC publishing for crates.io and PyPI
- **CI workflow**: `.github/workflows/ci.yml` — 5 jobs (Rust, Python, Node.js, WASM, C FFI) — green
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 5 crate members
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **Docs deploy blocked**: GitHub Pages not enabled in repo settings (admin action, not code fix)
- **Documentation content**: no Rust API page, no benchmark results page
- **npm publishing**: release workflow covers crates.io + PyPI but not npm (`@iscc/lib`,
    `@iscc/wasm`)

## CI

- **CI workflow: PASSING** — all 5 jobs green
- CI run: https://github.com/iscc/iscc-lib/actions/runs/22293007219
- **Docs workflow: FAILING** — `deploy-pages` 404 (GitHub Pages not enabled in repo settings)
- Docs run: https://github.com/iscc/iscc-lib/actions/runs/22293007226

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- Python tests: **49 passed** in CI
- Node.js CI: **passing** (46 conformance tests)
- WASM CI: **passing** (conformance tests via wasm-pack)
- C FFI CI: **passing** (cbindgen + gcc compile + test execution)

## Next Milestone

Add Rust API documentation page and benchmark results page to docs. Then add npm publishing jobs to
the release workflow for `@iscc/lib` and `@iscc/wasm`. The docs deploy failure requires Titusz to
enable GitHub Pages (Settings → Pages → Source: GitHub Actions) — flag it but don't block other work
on it.
