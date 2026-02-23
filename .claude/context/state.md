# Project State

## Status: IN_PROGRESS

## Phase: Rust core + Python bindings + Node.js bindings complete — Node.js CI job, WASM, C FFI, docs, publishing not started

Rust core is solid with all 9 `gen_*_v0` functions passing 143 conformance tests. Python bindings
are fully functional (49 tests pass). Node.js bindings via napi-rs are scaffolded with all 9 gen
function wrappers and 46 JavaScript conformance tests written — but there is no Node.js CI job in
the workflow yet. WASM, C FFI, docs, benchmarks in docs, and publishing remain untouched.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 conformance tests
    passing, clippy/fmt clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3 `_lowlevel` module, type stubs,
    `py.typed` marker, `__init__.py` re-exports, module name `iscc_lib`
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), all passing
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 gen function wrappers via napi-rs,
    `package.json` for `@iscc/lib`, compiles cleanly
- **Node.js tests (`crates/iscc-napi/__tests__/conformance.test.mjs`)**: 46 conformance tests for
    all 9 gen functions — **not yet integrated into CI**
- **Criterion benchmarks (`crates/iscc-lib/benches/benchmarks.rs`)**: all 9 gen functions
- **pytest-benchmark (`benchmarks/python/bench_iscc_core.py`)**: iscc-core baseline for all 9 gen
    functions
- **Conformance vectors**: `crates/iscc-lib/tests/data.json` vendored, shared by Rust, Python, JS
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 3 crate members
- **CI workflow**: `.github/workflows/ci.yml` with Rust and Python jobs — both passing
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **Node.js CI job**: `ci.yml` only has Rust and Python jobs — no build/test step for napi crate
- **WASM bindings (`@iscc/wasm`)**: wasm-bindgen crate — not started
- **C FFI**: cbindgen headers and shared library — not started
- **pytest-benchmark for Rust bindings**: iscc_lib vs iscc-core comparison — not started
- **Documentation site**: no lib.iscc.codes content or build setup
- **CI/CD publishing**: no OIDC trusted publishing pipelines for crates.io or PyPI
- **Benchmark results in docs**: speedup factors not yet published

## CI

- **Status: PASSING** — all jobs green (Rust, Python)
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22289005466

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt --all --check`: clean
- `uv run pytest`: **49 passed**, 0 failed
- Node.js tests: exist but not run in CI (no Node.js job configured)
- No `unsafe` in core crate

## Next Milestone

**Add Node.js CI job**: The napi crate has wrappers and conformance tests already written. The
immediate next step is adding a Node.js job to `ci.yml` that builds the native addon and runs the 46
JavaScript conformance tests, bringing Node.js to the same verification level as Rust and Python.
