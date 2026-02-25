# Next Work Package

## Step: Add `__version__` attribute and fix iscc-py module docstring

## Goal

Add the standard `__version__` attribute to the iscc-py package and fix the incorrect module
docstring in the Rust source. These two small fixes improve Python packaging compliance and code
accuracy — `__version__` is expected by standard Python tooling for runtime version detection, and
the docstring currently references the wrong module name.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/python/iscc_lib/__init__.py` — add `__version__` via
        `importlib.metadata.version("iscc-lib")` and include it in `__all__`
    - `crates/iscc-py/src/lib.rs` — fix docstring from `iscc._lowlevel` to `iscc_lib._lowlevel` and
        update the reference to `python/iscc/__init__.py` → `python/iscc_lib/__init__.py`
    - `tests/test_smoke.py` — add test for `__version__`
- **Reference**:
    - `crates/iscc-py/pyproject.toml` — confirms package name is `iscc-lib`

## Not In Scope

- Adding `__version__` to other binding packages (Node.js, WASM, Java, Go) — separate steps
- Adding a `.pyi` stub entry for `__version__` — the attribute lives in pure Python `__init__.py`,
    not in the native extension, so `_lowlevel.pyi` doesn't need updating
- Changing maturin configuration or build system settings
- Any other `[low]` issues (exception mapping, conformance feature gate, stale CLAUDE.md, etc.)

## Implementation Notes

1. **`__version__` in `__init__.py`**: Add near the top, after the module docstring and
    `from __future__` import:

    ```python
    from importlib.metadata import version

    __version__ = version("iscc-lib")
    ```

    This is the standard approach recommended by Python packaging authorities. It reads the version
    from the installed package metadata, which maturin populates from `Cargo.toml` via the
    `dynamic = ["version"]` setting in `pyproject.toml`. Add `"__version__"` to the `__all__` list.

2. **Module docstring in `lib.rs`**: The top `//!` comments contain two incorrect references:

    - Line 1: `iscc._lowlevel` → `iscc_lib._lowlevel`
    - Line 4: `python/iscc/__init__.py` → `python/iscc_lib/__init__.py`

3. **Test**: Add a test function in `tests/test_smoke.py`:

    - `iscc_lib.__version__` exists and is a string
    - `iscc_lib.__version__` equals `"0.0.1"` (the current workspace version)
    - `"__version__"` is in `iscc_lib.__all__`

## Verification

- `pytest tests/test_smoke.py` passes (existing tests + new `__version__` test)
- `pytest tests/` passes (all 157 tests still green)
- `python -c "import iscc_lib; print(iscc_lib.__version__)"` prints `0.0.1`
- `grep -q 'iscc_lib._lowlevel' crates/iscc-py/src/lib.rs` exits 0 (docstring fixed)
- `grep 'iscc\._lowlevel' crates/iscc-py/src/lib.rs | grep -qv 'iscc_lib'` exits non-zero (old
    incorrect reference gone)
- `cargo clippy -p iscc-py -- -D warnings` clean

## Done When

All verification criteria pass: `__version__` is importable and correct, the module docstring
references the right module name, and all existing tests remain green.
