# Project State

## Status: IN_PROGRESS

## Phase: Rust core + Python bindings + benchmarks complete, Node.js scaffold in place — JS tests and remaining bindings not started

Rust core is solid with all 9 `gen_*_v0` functions passing 143 conformance tests. Python bindings
are fully functional (49 tests pass). The napi-rs crate (`crates/iscc-napi/`) has been scaffolded
with all 9 gen function wrappers and compiles cleanly, but has no JavaScript conformance tests and
is not included in CI. WASM, C FFI, docs, and publishing remain untouched.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 conformance tests
    passing, clippy/fmt clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3 `_lowlevel` module, type stubs,
    `py.typed` marker, `__init__.py` re-exports, module name `iscc_lib`
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), all passing
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 gen function wrappers via napi-rs,
    `package.json` for `@iscc/lib`, compiles cleanly — **no JS tests yet, not in CI**
- **Criterion benchmarks (`crates/iscc-lib/benches/benchmarks.rs`)**: all 9 gen functions
- **pytest-benchmark (`benchmarks/python/bench_iscc_core.py`)**: iscc-core baseline for all 9 gen
    functions
- **Conformance vectors**: `crates/iscc-lib/tests/data.json` vendored, shared by Rust and Python
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 3 crate members
- **CI workflow**: `.github/workflows/ci.yml` with Rust and Python jobs — all passing
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **Node.js conformance tests**: napi crate exists but has zero JS/TS test files
- **Node.js CI job**: `ci.yml` does not build or test the napi crate
- **WASM bindings (`@iscc/wasm`)**: wasm-bindgen crate — not started
- **C FFI**: cbindgen headers and shared library — not started
- **pytest-benchmark for Rust bindings**: Phase 1 (iscc_lib vs iscc-core comparison) — not started
- **Documentation site**: no lib.iscc.codes content or build setup
- **CI/CD publishing**: no OIDC trusted publishing pipelines for crates.io or PyPI
- **Benchmark results in docs**: speedup factors not yet published

## CI

- **Status: PASSING** — all jobs green
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22288619366

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt --all --check`: clean
- `cargo check -p iscc-napi`: compiles, 0 tests (no JS test harness)
- `pytest`: **49 passed**, 0 failed
- No `unsafe` in core crate

## Next Milestone

**Node.js conformance tests**: Add JavaScript tests for the napi crate using the vendored
`data.json` vectors, build the native addon via `napi build`, and add a Node.js CI job to `ci.yml`.
The Rust wrappers exist — what's missing is the test harness to verify correctness.
