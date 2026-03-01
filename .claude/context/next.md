# Next Work Package

## Step: Add gen_sum_code_v0 to Python bindings

## Goal

Propagate `gen_sum_code_v0` and `SumCodeResult` to the Python bindings (`crates/iscc-py/`), making
Python the first binding to expose the 32nd Tier 1 symbol. This is the primary consumer language and
the handoff-recommended starting point for binding propagation (issue #15).

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/src/lib.rs` — add PyO3 `gen_sum_code_v0` wrapper + register in module
    - `crates/iscc-py/python/iscc_lib/__init__.py` — add `SumCodeResult` class, public
        `gen_sum_code_v0(path, bits, wide)` wrapper, update imports and `__all__`
    - `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — add type stub for low-level `gen_sum_code_v0`
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines 967-1010 — Rust `gen_sum_code_v0` signature and
        implementation
    - `crates/iscc-lib/src/types.rs` lines 96-105 — `SumCodeResult` struct definition
    - `crates/iscc-py/src/lib.rs` lines 300-327 — existing `gen_instance_code_v0` PyO3 wrapper (dict
        with `iscc`, `datahash`, `filesize` — same fields as `SumCodeResult`)
    - `crates/iscc-py/python/iscc_lib/__init__.py` lines 111-181 — existing result class pattern
        (`IsccResult(dict)` base, typed subclasses)
    - `crates/iscc-py/python/iscc_lib/__init__.py` lines 247-256 — existing `gen_instance_code_v0`
        public wrapper pattern

## Not In Scope

- Propagating to Node.js, WASM, C FFI, Java, or Go bindings (separate future steps)
- Adding `units: Vec<String>` field to `SumCodeResult` (deferred per prior scope decision)
- Updating README or documentation pages for gen_sum_code_v0 (do after all bindings complete)
- Adding Criterion benchmarks for gen_sum_code_v0 (separate step)
- Modifying the Rust core crate in any way

## Implementation Notes

**PyO3 wrapper (`crates/iscc-py/src/lib.rs`):**

- Unlike existing gen functions that accept `&[u8]`, `gen_sum_code_v0` takes `&Path`. The PyO3
    wrapper should accept `path: &str` (not bytes), convert to `std::path::Path`, and call
    `iscc_lib::gen_sum_code_v0`.
- Signature:
    `fn gen_sum_code_v0(py: Python<'_>, path: &str, bits: u32, wide: bool) -> PyResult<PyObject>`
- Use `#[pyo3(signature = (path, bits=64, wide=false))]` for defaults matching iscc-core convention.
- Return a `PyDict` with keys `iscc` (String), `datahash` (String), `filesize` (u64) — same pattern
    as `gen_instance_code_v0`.
- Error mapping: `.map_err(|e| PyValueError::new_err(e.to_string()))` as with all other wrappers.
- Register via `m.add_function(wrap_pyfunction!(gen_sum_code_v0, m)?)` in the module init.

**Python public API (`__init__.py`):**

- Add `SumCodeResult(IsccResult)` class with annotations: `iscc: str`, `datahash: str`,
    `filesize: int`. Same shape as `InstanceCodeResult`.
- Add public
    `gen_sum_code_v0(path: str | os.PathLike, bits: int = 64, wide: bool = False) -> SumCodeResult`.
- The public wrapper converts `os.PathLike` to string via `os.fspath(path)` before passing to the
    low-level Rust function. Use `str(os.fspath(path))` to handle both `str` and `pathlib.Path`.
- Import low-level `gen_sum_code_v0 as _gen_sum_code_v0` from `_lowlevel`.
- Add `"SumCodeResult"` and `"gen_sum_code_v0"` to `__all__` (keep alphabetically sorted).

**Type stub (`_lowlevel.pyi`):**

- Add `def gen_sum_code_v0(path: str, bits: int = 64, wide: bool = False) -> dict[str, Any]: ...`
- Docstring mentioning it generates both Data-Code and Instance-Code from a file path.

**Tests (in `tests/test_smoke.py` or similar):**

- Test with `str` path: write temp file, call `gen_sum_code_v0(str(path))`, verify result has
    `iscc`, `datahash`, `filesize` keys.
- Test with `pathlib.Path`: call `gen_sum_code_v0(pathlib.Path(path))`, verify same result.
- Test equivalence: verify `gen_sum_code_v0(path).datahash == gen_instance_code_v0(data).datahash`
    and `.filesize` match.
- Test file not found: verify `ValueError` raised with descriptive message.
- Test `isinstance(result, SumCodeResult)` and `isinstance(result, dict)`.
- Test attribute access: `result.iscc`, `result.datahash`, `result.filesize`.

## Verification

- `cargo test -p iscc-py` passes (existing 78+ Rust unit tests still green)
- `cargo clippy -p iscc-py -- -D warnings` clean
- `uv run pytest` passes (existing 198+ tests + new gen_sum_code_v0 tests)
- `uv run ruff check` clean
- `python -c "from iscc_lib import gen_sum_code_v0, SumCodeResult; print('OK')"` exits 0
- `python -c "import iscc_lib; assert 'gen_sum_code_v0' in iscc_lib.__all__; assert 'SumCodeResult' in iscc_lib.__all__"`
    exits 0

## Done When

The advance agent is done when all verification criteria pass, confirming that `gen_sum_code_v0` and
`SumCodeResult` are fully functional in the Python bindings with path-based input, dict+attribute
result access, and equivalence to the two-pass Rust approach.
