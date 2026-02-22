# Project State

## Status: IN_PROGRESS

## Phase: Python bindings complete — conformance tests needed, other bindings not started

The Rust core crate is complete with all 9 `gen_*_v0` functions passing 143 conformance tests. All 9
functions are exposed through PyO3 bindings with type stubs, and all pre-commit/pre-push hooks pass
cleanly. Python tests only cover `gen_instance_code_v0` with 3 smoke tests — full conformance vector
coverage from Python is missing.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 143 conformance tests passing
- **Python bindings (`crates/iscc-py/`)**: all 9 functions exposed via PyO3 `_lowlevel` module
    - `__init__.py` re-exports all 9 functions
    - `_lowlevel.pyi` type stub present — `ty` type checker passes
    - `py.typed` marker present
    - `maturin develop` builds and installs successfully
- **Quality gates**: all pre-commit and pre-push hooks pass (fmt, clippy, ty, ruff, tests)
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 2 crate members
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks, devcontainer, CID agents
- **Architecture docs**: `notes/` (00-09)

## What's Missing

- **Python conformance tests**: only 3 smoke tests for `gen_instance_code_v0` — need full
    conformance vector coverage for all 9 functions from Python
- **`crates/iscc-node/`**: napi-rs Node.js bindings — not started
- **`crates/iscc-wasm/`**: wasm-bindgen WASM bindings — not started
- **`crates/iscc-cffi/`**: C FFI with cbindgen — not started
- **CI/CD workflows**: no `.github/workflows/`
- **Benchmarks**: no criterion or pytest-benchmark
- **Documentation site**: no lib.iscc.codes content

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: **clean**
- `cargo clippy -p iscc-py -- -D warnings`: **clean**
- `cargo fmt --all --check`: **clean**
- `pytest tests/`: **3 passed** (smoke tests only)
- Pre-commit hooks: **all passed**
- Pre-push hooks: **all passed** (including ty, clippy, cargo test, pytest)

## Next Milestone

Add comprehensive Python conformance tests that exercise all 9 `gen_*_v0` functions against the
vendored `data.json` test vectors. This validates the PyO3 bindings produce correct results
end-to-end and completes the Python bindings target criteria.
