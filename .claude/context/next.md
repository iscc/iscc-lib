# Next Work Package

## Step: Add encode_base64, iscc_decompose, and sliding_window to Python bindings

## Goal

Expose 3 more Tier 1 symbols (`encode_base64`, `iscc_decompose`, `sliding_window`) in the Python
bindings, bringing the count from 14/23 to 17/23. All three are simple pass-through wrappers with no
complex types, matching the established text-utils pattern.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/src/lib.rs` — add 3 `#[pyfunction]` wrappers
    - `crates/iscc-py/python/iscc_lib/__init__.py` — re-export from `_lowlevel`, add to `__all__`
    - `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — add type stubs
- **Reference**:
    - `crates/iscc-lib/src/codec.rs` — `encode_base64` and `iscc_decompose` signatures
    - `crates/iscc-lib/src/simhash.rs` — `sliding_window` signature
    - `reference/iscc-core/iscc_core/codec.py` — Python reference for `encode_base64`,
        `iscc_decompose`
    - `reference/iscc-core/iscc_core/utils.py` — Python reference for `sliding_window`
    - `tests/test_text_utils.py` — established test pattern for simple binding tests

## Implementation Notes

All 3 functions follow the simple direct pass-through pattern (like `text_clean` — no `Python<'_>`
param, no `PyDict`):

1. **`encode_base64`**: `fn encode_base64(data: &[u8]) -> String` — direct passthrough. PyO3
    receives `&[u8]` directly from Python `bytes`.

2. **`iscc_decompose`**: `fn iscc_decompose(iscc_code: &str) -> IsccResult<Vec<String>>` — needs
    `map_err(PyValueError)` since it returns `Result`. Use `-> PyResult<Vec<String>>`. Returns
    `Vec<String>` which PyO3 auto-converts to Python `list[str]`.

3. **`sliding_window`**: `fn sliding_window(seq: &str, width: usize) -> Vec<String>` — direct
    passthrough. Add `#[pyo3(signature = (seq, width))]` for explicit parameter names. Returns
    `Vec<String>` → Python `list[str]`. Note: the Rust function uses `assert!` for width < 2, so it
    will panic. Wrap in a catch or convert to PyValueError if width < 2 before calling.

For each function:

- Add `#[pyfunction]` in `lib.rs` with docstring
- Register with `m.add_function(wrap_pyfunction!(...))` in `iscc_lowlevel`
- Re-export in `__init__.py` with `as X` for explicit re-export (not wrapped, since they return
    simple types)
- Add to `__all__`
- Add type stub in `_lowlevel.pyi` with docstring and type annotations

**Tests** — add to `tests/test_text_utils.py` (it already covers non-gen utility functions) or
create a new `tests/test_utils.py`. Test cases:

- `encode_base64(b"")` returns `""`
- `encode_base64(b"Hello")` returns the expected base64url-no-pad string (`"SGVsbG8"`)
- `encode_base64(b"\xff\xfe")` returns `"__4"` (base64url encoding)
- `iscc_decompose` on a known ISCC-CODE returns the expected list of unit strings — pick any
    `gen_iscc_code_v0` result from conformance vectors and verify it decomposes back
- `iscc_decompose` with invalid input raises `ValueError`
- `sliding_window("hello", 3)` returns `["hel", "ell", "llo"]`
- `sliding_window("ab", 3)` returns `[]` (input shorter than width)
- `sliding_window` with width < 2 raises an error

Build with: `VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop -m crates/iscc-py/Cargo.toml`

## Verification

- `maturin develop` builds successfully
- `pytest tests/` passes with new tests covering all 3 functions
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `cargo test -p iscc-lib` still passes (230 tests)
- `ty` check passes (stubs present in `_lowlevel.pyi`)
- Python `from iscc_lib import encode_base64, iscc_decompose, sliding_window` works
- `encode_base64(b"Hello")` returns `"SGVsbG8"`
- `iscc_decompose` roundtrips with gen functions
- `sliding_window("hello", 3)` returns `["hel", "ell", "llo"]`

## Done When

All verification criteria pass and the 3 new functions are callable from Python with correct
results.
