# Project State

## Status: IN_PROGRESS

## Phase: Rust core complete, Python bindings functional but CI failing

Rust core is solid with all 9 `gen_*_v0` functions passing 143 conformance tests. Python bindings
work locally (49 tests pass) but CI is **failing** because `maturin` is not installed as a
dependency — `uv sync --group dev` doesn't provide it. The Python module name is still `iscc`
instead of the required `iscc_lib`.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 conformance tests
    passing, clippy/fmt clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3 `_lowlevel` module, type stubs,
    `py.typed` marker, `__init__.py` re-exports
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), all passing locally
- **Conformance vectors**: `crates/iscc-lib/tests/data.json` vendored, shared by Rust and Python
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 2 crate members
- **CI workflow**: `.github/workflows/ci.yml` with Rust and Python jobs
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents
- **Architecture docs**: `notes/` (00-09)

## What's Missing

- **CI fix**: Python job fails — `maturin` not in root `pyproject.toml` dev dependencies
- **Python module rename**: module is `iscc`, must be `iscc_lib` for PyPI package name
- **Node.js bindings (`@iscc/lib`)**: napi-rs crate — not started
- **WASM bindings (`@iscc/wasm`)**: wasm-bindgen crate — not started
- **C FFI**: cbindgen headers and shared library — not started
- **Benchmarks**: no criterion or pytest-benchmark — not started
- **Documentation site**: no lib.iscc.codes content or build setup

## CI

- **Status: FAILING** — Python job fails, Rust job passes
- **Failed job**: `Python (ruff, pytest)` — `maturin` binary not found during
    `uv run maturin develop`
- **Root cause**: `maturin` is listed in `crates/iscc-py/pyproject.toml` build-requires but not in
    root `pyproject.toml` dev dependencies; CI runs `uv sync --group dev` from root only
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22287386115

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean (verified by CI)
- `cargo fmt --all --check`: clean (verified by CI)
- Local `pytest`: 49 passed (not verifiable in CI due to maturin issue)
- No `unsafe` in core crate

## Next Milestone

**Fix CI** (top priority): Add `maturin` to root `pyproject.toml` dev dependencies so the Python CI
job can build bindings. Then fix the Python module name (`iscc` → `iscc_lib`).
