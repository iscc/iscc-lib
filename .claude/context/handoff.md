## 2026-02-23 — Python dict returns for all 9 gen functions

**Done:** Converted all 9 `gen_*_v0` PyO3 bindings from returning plain `String` to returning
`PyDict` with all fields from the Rust `*CodeResult` structs. Updated `.pyi` type stubs to reflect
`dict[str, Any]` return types. Updated conformance and smoke tests to verify all dict fields (iscc,
name, metahash, description, meta, characters, parts, datahash, filesize).

**Files changed:**

- `crates/iscc-py/src/lib.rs`: Each function now takes `py: Python<'_>`, builds a `PyDict` from the
    result struct fields, and returns `PyResult<PyObject>`. Optional fields (`description`, `meta`)
    are only set when `Some`.
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: Return types changed from `str` to
    `dict[str, Any]`. Added `from typing import Any`. Docstring `:return:` lines describe dict keys.
- `tests/test_conformance.py`: All 9 test functions now verify `isinstance(result, dict)` and assert
    every output field from data.json (not just `iscc`). Meta tests check conditional
    presence/absence of `description` and `meta` keys.
- `tests/test_smoke.py`: Updated to use `result["iscc"]` access pattern. Added assertions for
    `datahash` and `filesize` fields.

**Verification:** All 49 Python tests pass (49 conformance + 3 smoke = 52 total, but the
parametrized conformance tests count as 49 including smoke). 163 Rust tests pass. Clippy clean. All
pre-commit hooks pass. `ruff check` and `ruff format` clean.

**Next:** The Python bindings now return dicts matching iscc-core. The remaining spec item from
`python-bindings.md` is accepting `BinaryIO` (file-like objects) for `gen_data_code_v0` and
`gen_instance_code_v0` streaming functions. This would require PyO3 to call Python's `.read()`
method and buffer the data before passing to Rust.

**Notes:** PyO3 0.23 `PyDict::new(py)` returns `Bound<'_, PyDict>` which converts to `PyObject` via
`.into()`. The `cargo fmt` hook reformatted a few function bodies (wrapping long `map_err` chains) —
the committed code includes the formatter's changes.
