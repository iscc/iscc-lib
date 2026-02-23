# Project State

## Status: IN_PROGRESS

## Phase: Documentation content and OIDC publishing remain

All 9 `gen_*_v0` functions are implemented in the Rust core with 143 conformance tests passing. All
5 binding crates (Python, Node.js, WASM, C FFI, plus core) are complete with tests. CI has 5 jobs
all green. Criterion and pytest benchmarks exist. Docs site has a landing page, Python API
reference, and a GitHub Pages deployment workflow. Still missing: Rust API docs page, architecture
page, benchmark results in docs, and OIDC publishing pipelines.

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
- **Documentation**: `zensical.toml`, `docs/index.md` (landing), `docs/api.md` (Python API ref)
- **Docs deployment**: `.github/workflows/docs.yml` — builds with zensical, deploys to GitHub Pages
- **Conformance vectors**: `data.json` vendored, shared by Rust/Python/JS/WASM
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 5 crate members
- **CI workflow**: 5 jobs (Rust, Python, Node.js, WASM, C FFI) — all green
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **Documentation content**: no Rust API page, no architecture page, no benchmark results in docs
- **CI/CD publishing pipelines**: no OIDC trusted publishing workflows for crates.io, PyPI, or npm

## CI

- **Status: PASSING** — all 5 jobs green (Rust, Python, Node.js, WASM, C FFI)
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22292377589

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt -p iscc-lib --check`: clean
- Python tests: **49 passed** in CI
- Node.js CI: **passing** (46 conformance tests)
- WASM CI: **passing** (conformance tests via wasm-pack)
- C FFI CI: **passing** (cbindgen + gcc compile + test execution)

## Next Milestone

**Expand documentation content**: Add a Rust API documentation page (using rustdoc-style content or
inline docs), an architecture overview page, and published benchmark results. After documentation is
complete, add OIDC trusted publishing workflows for crates.io, PyPI, and npm.
