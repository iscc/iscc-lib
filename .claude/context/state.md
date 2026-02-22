# Project State

## Status: IN_PROGRESS

## Phase: Python bindings scaffolded — 1/9 functions exposed, type stub missing

The Rust core crate is complete with all 9 `gen_*_v0` functions passing 143 conformance tests. The
Python bindings crate (`iscc-py`) has been scaffolded with PyO3/maturin, exposing only
`gen_instance_code_v0` so far. The pre-push `ty` type checker fails because a `.pyi` stub is missing
for the native `_lowlevel` module.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: 8 modules, all 9 gen functions, 143 tests passing, clippy/fmt
    clean
- **Python bindings (`crates/iscc-py/`)**: scaffolded with PyO3 `_lowlevel` module pattern
    - Only `gen_instance_code_v0` exposed (1 of 9 functions)
    - `python/iscc/__init__.py` re-exports from `_lowlevel`
    - `py.typed` marker present, but no `.pyi` type stub (causes `ty` failure)
    - `cargo build -p iscc-py` succeeds, 3 Python smoke tests pass
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 2 workspace members
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks, devcontainer, CID agents
- **Architecture docs**: `notes/` (00-09)
- **50 commits** of iterative development

## What's Missing

- **Python bindings**: 8 remaining `gen_*_v0` functions not yet exposed through PyO3
- **`_lowlevel.pyi` type stub**: blocks pre-push hook (`ty` type checker fails)
- **Python conformance tests**: only 3 smoke tests exist, no full vector coverage from Python
- **`crates/iscc-node/`**: napi-rs Node.js bindings — not started
- **`crates/iscc-wasm/`**: wasm-bindgen WASM bindings — not started
- **`crates/iscc-cffi/`**: C FFI with cbindgen — not started
- **CI/CD workflows**: no `.github/workflows/`
- **Benchmarks**: no criterion or pytest-benchmark
- **Documentation site**: no lib.iscc.codes content

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed, 0 ignored
- `cargo clippy -p iscc-lib -- -D warnings`: **clean**
- `cargo clippy -p iscc-py -- -D warnings`: **clean**
- `cargo fmt -p iscc-lib --check`: **clean**
- `cargo build -p iscc-py`: **succeeds**
- `pytest tests/`: **3 passed** (smoke tests for `gen_instance_code_v0`)
- Pre-push `ty` hook: **FAILS** — `unresolved-import: iscc._lowlevel` (needs `.pyi` stub)

## Next Milestone

Add `_lowlevel.pyi` type stub to fix the `ty` pre-push hook failure, then expose the remaining 8
`gen_*_v0` functions in `iscc-py` with corresponding Python conformance tests. This completes the
Python bindings target.
