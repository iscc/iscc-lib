# Project State

## Status: IN_PROGRESS

## Phase: Documentation content in progress — Pages deploy, architecture page, and OIDC publishing remain

All 9 `gen_*_v0` functions are implemented in the Rust core with 143 conformance tests passing. All
4 binding crates (Python, Node.js, WASM, C FFI) are complete with tests. CI has 5 jobs all green.
Criterion and pytest benchmarks exist. Zensical docs now have a landing page and a Python API
reference page (`docs/api.md` with mkdocstrings). Still missing: architecture page, benchmark
results in docs, GitHub Pages deployment workflow, and OIDC publishing pipelines.

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
- **Documentation**: `zensical.toml`, `docs/index.md` (landing), `docs/api.md` (Python API ref via
    mkdocstrings)
- **Conformance vectors**: `data.json` vendored, shared by Rust/Python/JS/WASM
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 5 crate members
- **CI workflow**: 5 jobs (Rust, Python, Node.js, WASM, C FFI) — all green
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **Documentation content**: no architecture page, no benchmark results published in docs
- **GitHub Pages deployment**: no workflow to build/deploy docs to lib.iscc.codes
- **CI/CD publishing pipelines**: no OIDC trusted publishing for crates.io, PyPI, or npm

## CI

- **Status: PASSING** — all 5 jobs green (Rust, Python, Node.js, WASM, C FFI)
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22292101252

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo test -p iscc-ffi`: **20 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt -p iscc-lib --check`: clean
- Python tests: **49 passed** in CI (local requires `maturin develop` first)
- Node.js CI: **passing** (46 conformance tests)
- WASM CI: **passing** (conformance tests via wasm-pack)
- C FFI CI: **passing** (cbindgen + gcc compile + test execution)

## Next Milestone

**Add GitHub Pages deployment workflow**: The docs site has content (landing + API ref) and can be
built locally. The highest-value next step is adding a GitHub Actions workflow to deploy to
lib.iscc.codes via GitHub Pages, making the documentation publicly accessible. After that: add
architecture and benchmark pages, then OIDC publishing pipelines.
