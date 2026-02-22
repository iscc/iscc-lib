## 2026-02-22 — Scaffold iscc-py crate with PyO3/maturin and expose gen_instance_code_v0

**Done:** Created the `crates/iscc-py/` Python binding crate with PyO3/maturin, exposing
`gen_instance_code_v0` as the first function. The full build pipeline works end-to-end: Rust →
native module → Python import → correct conformance output. Also removed the unused
`IsccError::NotImplemented` variant from iscc-lib.

**Files changed:**

- `Cargo.toml`: Added `crates/iscc-py` to workspace members, added `pyo3` to workspace.dependencies
- `crates/iscc-lib/src/lib.rs`: Removed unused `IsccError::NotImplemented` variant
- `crates/iscc-py/Cargo.toml`: New crate — cdylib with iscc-lib + pyo3 dependencies
- `crates/iscc-py/src/lib.rs`: PyO3 module exposing `gen_instance_code_v0` with `bits` defaulting to
    64
- `crates/iscc-py/pyproject.toml`: Maturin build config with abi3-py310, `iscc._lowlevel` module
- `crates/iscc-py/python/iscc/__init__.py`: Pure Python wrapper re-exporting from `_lowlevel`
- `crates/iscc-py/python/iscc/py.typed`: PEP 561 type marker
- `tests/test_smoke.py`: Updated with 3 conformance tests for `gen_instance_code_v0`

**Verification:**

- `cargo build -p iscc-py` — compiles successfully
- `cargo test -p iscc-lib` — all 143 tests pass (NotImplemented removal is clean)
- `cargo clippy -p iscc-py -- -D warnings` — clean
- `cargo clippy -p iscc-lib -- -D warnings` — clean
- `cargo fmt --check` — clean
- `maturin develop --manifest-path crates/iscc-py/Cargo.toml` — builds and installs
- `python -c "from iscc import gen_instance_code_v0; print(gen_instance_code_v0(b'', 64))"` — prints
    `ISCC:IAA26E2JXH27TING`
- `uv run pytest tests/test_smoke.py -v` — 3/3 tests pass

**Next:** Expose the remaining 8 `gen_*_v0` functions through the Python bindings. The pattern is
established — each function needs a `#[pyfunction]` wrapper in `crates/iscc-py/src/lib.rs` that
converts `IsccError` to `PyValueError`, plus a re-export in `__init__.py`. Functions with complex
parameter types (e.g., `gen_video_code_v0` with `&[Vec<i32>]`, `gen_mixed_code_v0` with `&[&str]`)
will need PyO3 type conversion consideration.

**Notes:** The `pyproject.toml` needed `dynamic = ["version"]` added — the next.md template omitted
this and maturin refused to build without a version. The `VIRTUAL_ENV` mismatch between the CID
agent env and the project env (`/home/dev/.venvs/iscc-lib`) means `maturin develop` must target the
project venv explicitly (`VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop ...`) for
`uv run pytest` to find the module. The abi3-py310 build produces a single wheel usable across
Python 3.10+.
