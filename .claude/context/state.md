# Project State

## Status: IN_PROGRESS

## Phase: Rust core + Python bindings complete, CI green — remaining bindings not started

Rust core is solid with all 9 `gen_*_v0` functions passing 143 conformance tests. Python bindings
are fully functional (49 tests pass) with the correct `iscc_lib` module name. CI is green. The
remaining target items (Node.js, WASM, C FFI, benchmarks, documentation) have not been started.

## What Exists

- **Rust core (`crates/iscc-lib/`)**: all 9 gen functions, 8 internal modules, 143 conformance tests
    passing, clippy/fmt clean, no `unsafe`
- **Python bindings (`crates/iscc-py/`)**: all 9 functions via PyO3 `_lowlevel` module, type stubs,
    `py.typed` marker, `__init__.py` re-exports, module name `iscc_lib`
- **Python tests (`tests/`)**: 46 conformance + 3 smoke tests (49 total), all passing
- **Conformance vectors**: `crates/iscc-lib/tests/data.json` vendored, shared by Rust and Python
- **Workspace**: root `Cargo.toml` with centralized deps, release profile, 2 crate members
- **CI workflow**: `.github/workflows/ci.yml` with Rust and Python jobs — all passing
- **Dev tooling**: mise.toml, pyproject.toml, pre-commit hooks (prek), devcontainer, CID agents
- **Architecture docs**: `notes/` (00-09)

## What's Missing

- **Node.js bindings (`@iscc/lib`)**: napi-rs crate — not started
- **WASM bindings (`@iscc/wasm`)**: wasm-bindgen crate — not started
- **C FFI**: cbindgen headers and shared library — not started
- **Benchmarks**: no criterion or pytest-benchmark — not started
- **Documentation site**: no lib.iscc.codes content or build setup
- **CI/CD publishing**: no OIDC trusted publishing pipelines for crates.io or PyPI

## CI

- **Status: PASSING** — all jobs green
- **Run**: https://github.com/iscc/iscc-lib/actions/runs/22287548017

## Verification

- `cargo test -p iscc-lib`: **143 passed**, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt --all --check`: clean
- `pytest`: **49 passed**, 0 failed
- No `unsafe` in core crate

## Next Milestone

**Node.js bindings** (`@iscc/lib` via napi-rs): Create a new `crates/iscc-napi/` crate that wraps
the Rust core and exposes all 9 `gen_*_v0` functions as a native Node.js addon with conformance
tests. Alternatively, **benchmarks** (criterion for Rust, pytest-benchmark for Python) could be
tackled first as a smaller deliverable.
