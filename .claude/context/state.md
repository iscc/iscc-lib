# Project State

## Status: IN_PROGRESS

## Phase: Documentation content gaps and OIDC publishing remain

All 9 `gen_*_v0` functions are implemented in the Rust core with 143 conformance tests passing. All
5 binding crates (Python, Node.js, WASM, C FFI, plus core) are complete with conformance tests. The
CI workflow (5 jobs) is green. Criterion and pytest benchmarks exist. Docs site has landing page,
Python API reference, and architecture page. The docs deployment workflow exists but fails because
GitHub Pages is not enabled in the repo settings.

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
- **Comparative pytest-benchmark (`benchmarks/python/`)**: iscc-core vs iscc_lib bindings
- **Documentation**: `zensical.toml`, `docs/index.md` (landing), `docs/api.md` (Python API ref),
    `docs/architecture.md` (crate model, modules, streaming, conformance)
- **Docs deployment**: `.github/workflows/docs.yml` — builds with zensical, deploys to GitHub Pages
- **Conformance vectors**: `data.json` vendored, shared by Rust/Python/JS/WASM
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 5 crate members
- **CI workflow**: 5 jobs (Rust, Python, Node.js, WASM, C FFI) — all green
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **Docs deploy broken**: GitHub Pages not enabled in repo settings — `deploy-pages` returns 404
- **Documentation content**: no Rust API page, no benchmark results page in docs
- **CI/CD publishing pipelines**: no OIDC trusted publishing workflows for crates.io, PyPI, or npm

## CI

- **CI workflow: PASSING** — all 5 jobs green (Rust, Python, Node.js, WASM, C FFI)
- **CI run**: https://github.com/iscc/iscc-lib/actions/runs/22292682656
- **Docs workflow: FAILING** — `deploy-pages` 404 (GitHub Pages not enabled in repo settings)
- **Docs run**: https://github.com/iscc/iscc-lib/actions/runs/22292682655

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt -p iscc-lib --check`: clean (by CI)
- Python tests: **49 passed** in CI
- Node.js CI: **passing** (46 conformance tests)
- WASM CI: **passing** (conformance tests via wasm-pack)
- C FFI CI: **passing** (cbindgen + gcc compile + test execution)

## Next Milestone

**Fix docs deployment**: GitHub Pages must be enabled in the repo settings (Settings → Pages →
Source: GitHub Actions). This is a manual repo admin step, not a code fix. Once Pages is enabled,
the existing workflow will succeed. After that, add the remaining docs content (Rust API page,
benchmark results), then add OIDC trusted publishing workflows.
