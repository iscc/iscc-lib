# Project State

## Status: IN_PROGRESS

## Phase: Rust core + Python bindings + Node.js bindings complete — WASM, C FFI, docs, publishing remain

Rust core is fully implemented with all 9 `gen_*_v0` functions passing 143 conformance tests. Python
bindings (PyO3) and Node.js bindings (napi-rs) are complete with conformance tests. Criterion and
pytest-benchmark baselines exist. All three CI jobs (Rust, Python, Node.js) are green. WASM
bindings, C FFI, documentation site, and publishing pipelines are not started.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 conformance
    tests, clippy/fmt clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3 `_lowlevel`, type stubs,
    `py.typed`, module name `iscc_lib`
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), all passing
- **Node.js bindings (`crates/iscc-napi/`)**: all 9 gen functions via napi-rs, `@iscc/lib` package
- **Node.js tests (`crates/iscc-napi/__tests__/`)**: 46 conformance tests, all passing in CI
- **Criterion benchmarks (`crates/iscc-lib/benches/`)**: all 9 gen functions
- **pytest-benchmark (`benchmarks/python/`)**: iscc-core baseline for all 9 gen functions
- **Conformance vectors**: `crates/iscc-lib/tests/data.json` vendored, shared by Rust/Python/JS
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 3 crate members
- **CI workflow**: `.github/workflows/ci.yml` with Rust, Python, Node.js jobs — all green
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents

## What's Missing

- **WASM bindings (`@iscc/wasm`)**: wasm-bindgen crate — not started
- **C FFI**: cbindgen headers and shared library — not started
- **pytest-benchmark for Rust bindings**: iscc_lib vs iscc-core comparison — not started
- **Documentation site**: no lib.iscc.codes content or build setup
- **CI/CD publishing**: no OIDC trusted publishing pipelines for crates.io, PyPI, or npm
- **Benchmark results in docs**: speedup factors not yet published

## CI

- **Status: PASSING** — all 3 jobs green (Rust, Python, Node.js)
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22289302383

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt --all --check`: clean
- `uv run pytest`: **49 passed**, 0 failed
- Node.js CI job: **passing** (46 conformance tests)
- No `unsafe` in core crate

## Next Milestone

**WASM bindings crate**: Create `crates/iscc-wasm/` with wasm-bindgen wrappers for all 9 gen
functions, conformance tests runnable in a WASM runtime, and a CI job. This follows the same
hub-and-spoke pattern as the napi crate.
