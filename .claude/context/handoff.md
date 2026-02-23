## 2026-02-23 — Add encode_base64, iscc_decompose, and sliding_window to Python bindings

**Done:** Added 3 new Tier 1 functions (`encode_base64`, `iscc_decompose`, `sliding_window`) to the
Python bindings, bringing the count from 14/23 to 17/23. All three follow the simple direct
pass-through pattern (no `Python<'_>` param, no `PyDict`). The `sliding_window` binding catches the
Rust `assert!` panic by validating `width < 2` before calling the Rust function, raising
`ValueError` instead.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: Added 3 `#[pyfunction]` wrappers and registered them in the module
- `crates/iscc-py/python/iscc_lib/__init__.py`: Re-exported 3 functions from `_lowlevel`, added to
    `__all__`
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: Added type stubs with docstrings for all 3
    functions
- `tests/test_text_utils.py`: Added 13 new tests covering all 3 functions (4 encode_base64, 4
    iscc_decompose, 5 sliding_window)

**Verification:** All quality gates pass:

- 98 Python tests pass (63 existing + 35 in test_text_utils)
- 230 Rust tests pass (180 unit + 28 algo integration + 22 text utils integration)
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- All pre-commit hooks pass (`mise run check`)
- Python imports work: `from iscc_lib import encode_base64, iscc_decompose, sliding_window`
- `encode_base64(b"Hello")` returns `"SGVsbG8"` ✓
- `iscc_decompose("AAAYPXW445FTYNJ3")` returns `["AAAYPXW445FTYNJ3"]` ✓
- `sliding_window("hello", 3)` returns `["hel", "ell", "llo"]` ✓

**Next:** Continue expanding binding API surface. Remaining 6 functions to add to Python bindings:
`alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`, `DataHasher`, `InstanceHasher`. The streaming
types (`DataHasher`, `InstanceHasher`) require PyO3 class wrappers (not simple pass-throughs) and
are the most complex remaining items. Alternatively, continue horizontal expansion by adding the
same utility functions to Node.js, WASM, and C FFI bindings.

**Notes:** The `sliding_window` Rust function uses `assert!` for width < 2 (which would cause a
panic across FFI). The PyO3 wrapper validates width before calling Rust, converting to `ValueError`.
This is a better pattern than letting the panic propagate. The `iscc_decompose` function naturally
returns `Result` and uses the standard `map_err(PyValueError)` pattern.
