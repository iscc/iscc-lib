# Next Work Package

## Step: Add conformance_selftest and text utilities to Python bindings

## Goal

Expose `conformance_selftest` and the 4 text utility functions (`text_clean`,
`text_remove_newlines`, `text_trim`, `text_collapse`) through the Python bindings. This begins the
process of completing the Python Tier 1 API surface beyond the 9 gen functions, starting with the
simplest symbols.

## Scope

- **Modify**: `crates/iscc-py/src/lib.rs` — add 5 `#[pyfunction]` wrappers and register them in the
    `#[pymodule]`
- **Modify**: `crates/iscc-py/python/iscc_lib/__init__.py` — add imports from `_lowlevel` and
    re-exports in `__all__`
- **Modify**: `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — add type stubs for the 5 functions
- **Reference**:
    - `crates/iscc-lib/src/utils.rs` — Rust signatures for `text_clean`, `text_remove_newlines`,
        `text_trim`, `text_collapse`
    - `crates/iscc-lib/src/conformance.rs` — Rust signature for `conformance_selftest`
    - `crates/iscc-lib/src/lib.rs` lines 18–26 — public re-exports confirming API surface

## Implementation Notes

**PyO3 wrappers in `lib.rs`:**

All 5 functions have trivial signatures that map directly to PyO3 types:

```rust
#[pyfunction]
fn conformance_selftest() -> bool {
    iscc_lib::conformance_selftest()
}

#[pyfunction]
fn text_clean(text: &str) -> String {
    iscc_lib::text_clean(text)
}

#[pyfunction]
fn text_remove_newlines(text: &str) -> String {
    iscc_lib::text_remove_newlines(text)
}

#[pyfunction]
#[pyo3(signature = (text, nbytes))]
fn text_trim(text: &str, nbytes: usize) -> String {
    iscc_lib::text_trim(text, nbytes)
}

#[pyfunction]
fn text_collapse(text: &str) -> String {
    iscc_lib::text_collapse(text)
}
```

Register all 5 in the `iscc_lowlevel` module function with `m.add_function(wrap_pyfunction!(...))`.

**Python `__init__.py`:**

Add imports from `_lowlevel` (these are pass-through, no wrapper class needed since they return
simple types):

```python
from iscc_lib._lowlevel import (
    conformance_selftest as conformance_selftest,
    text_clean as text_clean,
    text_collapse as text_collapse,
    text_remove_newlines as text_remove_newlines,
    text_trim as text_trim,
)
```

Add all 5 names to `__all__`.

**Type stubs in `_lowlevel.pyi`:**

Add stubs with docstrings matching the Rust source. `conformance_selftest() -> bool`,
`text_clean(text: str) -> str`, `text_remove_newlines(text: str) -> str`,
`text_trim(text: str, nbytes: int) -> str`, `text_collapse(text: str) -> str`.

**Tests:**

Add a test file `tests/test_text_utils.py` (or add to existing test file) that:

1. Calls `conformance_selftest()` and asserts it returns `True`
2. Tests `text_clean` with control chars, newlines, whitespace
3. Tests `text_remove_newlines` with multi-line input
4. Tests `text_trim` with truncation and Unicode boundary handling
5. Tests `text_collapse` with accents, punctuation, whitespace

Build with: `VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop -m crates/iscc-py/Cargo.toml`

## Verification

- `VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop -m crates/iscc-py/Cargo.toml` succeeds
- `pytest tests/` passes (existing 63 tests + new text utility tests)
- `from iscc_lib import conformance_selftest, text_clean, text_remove_newlines, text_trim, text_collapse`
    works
- `conformance_selftest()` returns `True`
- `text_clean("hello\tworld")` returns `"helloworld"`
- `text_collapse("Hello World")` returns `"helloworld"`
- `text_trim("hello world", 5)` returns `"hello"`
- `text_remove_newlines("hello\nworld")` returns `"hello world"`
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `uv run prek run --all-files` passes (or at minimum no regressions)

## Done When

All verification criteria pass — the 5 functions are importable from `iscc_lib`, return correct
results, all existing and new tests pass, and clippy is clean.
