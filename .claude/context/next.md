# Next Work Package

## Step: Add gen_sum_code_v0 to Python bindings

## Goal

Expose `gen_sum_code_v0` and `SumCodeResult` in the Python package (`iscc_lib`), completing 32/32
Tier 1 symbols in the primary consumer binding. This is the first binding propagation step for issue
#15.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/src/lib.rs` — add PyO3 `gen_sum_code_v0` wrapper + register in pymodule
    - `crates/iscc-py/python/iscc_lib/__init__.py` — add `SumCodeResult` class, import + wrapper
        function, update `__all__`
    - `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — add type stub for `gen_sum_code_v0`
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` line 967 — Rust `gen_sum_code_v0(path: &Path, bits, wide)`
    - `crates/iscc-lib/src/types.rs` lines 95-105 — `SumCodeResult { iscc, datahash, filesize }`
    - `crates/iscc-py/src/lib.rs` lines 300-313 — existing `gen_instance_code_v0` PyO3 wrapper
        (returns dict with same 3 keys: `iscc`, `datahash`, `filesize`)
    - `crates/iscc-py/python/iscc_lib/__init__.py` lines 169-174 — `InstanceCodeResult` class (same
        fields as `SumCodeResult`)
    - `crates/iscc-py/python/iscc_lib/__init__.py` lines 245-256 — `gen_instance_code_v0` public
        wrapper pattern

## Not In Scope

- Propagating to Node.js, WASM, C FFI, Java, or Go bindings (separate future steps)
- Adding `units: Vec<String>` field to `SumCodeResult` (deferred per prior scope decision)
- Updating README, docs site, or howto guides (defer until all bindings propagated)
- Adding Criterion benchmarks for `gen_sum_code_v0` (separate step)
- Modifying the Rust core crate (`iscc-lib`) in any way
- Accepting `BinaryIO` / file-like objects — `gen_sum_code_v0` does its own file I/O from a path

## Implementation Notes

**PyO3 wrapper (`crates/iscc-py/src/lib.rs`):**

- `gen_sum_code_v0` takes `&Path` in Rust. The PyO3 wrapper accepts `path: &str`, converts to
    `std::path::Path`, and calls `iscc_lib::gen_sum_code_v0`.
- Signature:
    `fn gen_sum_code_v0(py: Python<'_>, path: &str, bits: u32, wide: bool) -> PyResult<PyObject>`
- Use `#[pyo3(signature = (path, bits=64, wide=false))]` for Python-side defaults.
- Return `PyDict` with keys `iscc`, `datahash`, `filesize` — same pattern as `gen_instance_code_v0`.
- Error mapping: `.map_err(|e| PyValueError::new_err(e.to_string()))`.
- Register: `m.add_function(wrap_pyfunction!(gen_sum_code_v0, m)?)?;` after `gen_iscc_code_v0`.

**Python public API (`__init__.py`):**

- Import: `gen_sum_code_v0 as _gen_sum_code_v0` from `_lowlevel` (add to import block).
- Add `import os` for `os.PathLike` type and `os.fspath()` conversion.
- Add `SumCodeResult(IsccResult)` class with annotations: `iscc: str`, `datahash: str`,
    `filesize: int`. Place after `IsccCodeResult` (line ~181).
- Public wrapper:

```python
def gen_sum_code_v0(
    path: str | os.PathLike, bits: int = 64, wide: bool = False
) -> SumCodeResult:
    """Generate Data-Code + Instance-Code + ISCC-CODE from a file path in a single pass."""
    return SumCodeResult(_gen_sum_code_v0(os.fspath(path), bits, wide))
```

- Add `"SumCodeResult"` and `"gen_sum_code_v0"` to `__all__` (alphabetical order).

**Type stub (`_lowlevel.pyi`):**

- Add between `gen_iscc_code_v0` and `conformance_selftest`:

```python
def gen_sum_code_v0(path: str, bits: int = 64, wide: bool = False) -> dict[str, Any]:
    """Generate Data-Code + Instance-Code + ISCC-CODE from a file path in a single pass.

    :param path: File system path to the file.
    :param bits: Bit length for the Data-Code body (default 64).
    :param wide: Whether to produce a wide (256-bit) ISCC-CODE (default False).
    :return: Dict with ``iscc``, ``datahash``, and ``filesize`` keys.
    """
    ...
```

**Tests (add to `tests/test_smoke.py`):**

1. Equivalence: temp file → `gen_sum_code_v0(path)` → compare `datahash`/`filesize` with
    `gen_instance_code_v0(data)`.
2. PathLike: pass `pathlib.Path` object — verify works via `os.fspath()` conversion.
3. Error: nonexistent path → verify `ValueError` raised.
4. Result type: verify `isinstance(result, SumCodeResult)` and `isinstance(result, dict)`.
5. Attribute access: `result.iscc`, `result.datahash`, `result.filesize` all work.
6. Wide mode: `wide=True` produces different ISCC string than `wide=False`.

## Verification

- `cargo test -p iscc-py` passes (existing Rust-side tests still green)
- `cargo clippy -p iscc-py -- -D warnings` clean
- `uv run pytest tests/ -x` passes (existing 198+ tests + new gen_sum_code_v0 tests)
- `uv run ruff check` clean
- `uv run python -c "from iscc_lib import gen_sum_code_v0, SumCodeResult; print('OK')"` exits 0
- `uv run python -c "import iscc_lib; assert 'gen_sum_code_v0' in iscc_lib.__all__; assert 'SumCodeResult' in iscc_lib.__all__"`
    exits 0

## Done When

The advance agent is done when all verification criteria pass: `gen_sum_code_v0` and `SumCodeResult`
are importable from `iscc_lib`, produce correct results for file paths (str and PathLike), support
dict+attribute access, and all existing tests remain green.
