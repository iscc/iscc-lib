# Project State

## Status: IN_PROGRESS

## Phase: Rust core + Python bindings complete — other bindings, CI, docs, benchmarks not started

The Rust core crate is complete with all 9 `gen_*_v0` functions passing 143 conformance tests.
Python bindings expose all 9 functions via PyO3 with 49 Python tests (46 conformance + 3 smoke)
passing. All quality gates (clippy, fmt, ruff, ty, pre-commit/pre-push hooks) are clean. No other
bindings, CI/CD, benchmarks, or documentation exist yet.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules (codec, simhash,
    minhash, cdc, dct, wtahash, utils, lib), 143 conformance tests passing
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3 `_lowlevel` module, type stubs,
    `py.typed` marker, `__init__.py` re-exports
- **Python tests**: 46 conformance vector tests + 3 smoke tests (49 total), all passing
- **Conformance vectors**: `crates/iscc-lib/tests/data.json` vendored, shared by Rust and Python
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 2 crate members
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents
- **Architecture docs**: `notes/` (00-09)
- **No `unsafe` code** in the core crate

## What's Missing

- **Node.js bindings (`@iscc/lib`)**: napi-rs crate — not started
- **WASM bindings (`@iscc/wasm`)**: wasm-bindgen crate — not started
- **C FFI**: cbindgen headers and shared library — not started
- **CI/CD workflows**: no `.github/workflows/` directory
- **Benchmarks**: no criterion benchmarks or pytest-benchmark comparisons
- **Documentation site**: no lib.iscc.codes content or build setup

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: **clean**
- `cargo fmt -p iscc-lib --check`: **clean**
- `pytest`: **49 passed** (46 conformance + 3 smoke)
- `ruff check` / `ruff format --check`: **clean**
- No `unsafe` in core crate

## Next Milestone

The Rust core and Python bindings are fully verified. The next logical step is Node.js bindings
(`crates/iscc-napi/`) via napi-rs, which would bring the second language binding online and validate
the hub-and-spoke crate model with a non-Python binding.
