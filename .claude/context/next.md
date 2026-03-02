# Next Work Package

## Step: Expose `add_units` in Python binding (issue #21)

## Goal

Expose the `add_units: bool` parameter and `units` return field in the Python binding for
`gen_sum_code_v0`, so Python callers (especially `iscc-sdk`) can get Data-Code and Instance-Code
ISCC strings from a single optimized call. This is the first binding update for issue #21.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/src/lib.rs` — add `add_units` param, include `units` in returned dict
    - `crates/iscc-py/python/iscc_lib/__init__.py` — add `add_units` param to wrapper, add `units`
        type annotation to `SumCodeResult`
    - `tests/test_smoke.py` — add tests for `add_units=True` and `add_units=False`
- **Reference**:
    - `crates/iscc-lib/src/code_sum.rs` — Rust core `gen_sum_code_v0` signature and `SumCodeResult`
    - Existing Python test patterns in `tests/test_smoke.py`

## Not In Scope

- Updating Node.js, WASM, C FFI, JNI, or Go bindings (separate steps per binding)
- Updating `docs/rust-api.md` or `docs/architecture.md` (defer until all bindings are done)
- Changing the Rust core API (already complete)
- Adding conformance vectors for `gen_sum_code_v0` (none exist in `data.json`)

## Implementation Notes

**Rust side (`crates/iscc-py/src/lib.rs`):**

- Change the `#[pyo3(signature)]` from `(path, bits=64, wide=false)` to
    `(path, bits=64, wide=false, add_units=false)` — 4 params
- Pass `add_units` through to `iscc_lib::gen_sum_code_v0(Path::new(path), bits, wide, add_units)`
- When `r.units` is `Some(vec)`, set `dict.set_item("units", vec)?` — PyO3 auto-converts
    `Vec<String>` to a Python list of strings
- When `r.units` is `None`, do NOT set the "units" key (matching iscc-core pattern of omitting
    optional fields)

**Python side (`__init__.py`):**

- Update `SumCodeResult` class to add `units: list[str] | None` type annotation (after `filesize`)
- Update the `gen_sum_code_v0` wrapper signature to accept `add_units: bool = False`
- Pass `add_units` through: `_gen_sum_code_v0(os.fspath(path), bits, wide, add_units)`

**Tests (`tests/test_smoke.py`):**

- Add `test_gen_sum_code_v0_units_enabled`: create temp file, call with `add_units=True`, assert
    `"units"` key exists, assert it's a list of 2 strings, each starting with `"ISCC:"`
- Add `test_gen_sum_code_v0_units_disabled`: create temp file, call with `add_units=False`
    (default), assert `"units"` key is NOT present in the result dict
- Add `test_gen_sum_code_v0_units_attribute_access`: call with `add_units=True`, verify
    `result.units` attribute access works

## Verification

- `cargo build -p iscc-py` compiles without errors
- `mise run test` passes (all existing + 3 new Python tests)
- `cargo clippy -p iscc-py -- -D warnings` clean
- Calling `gen_sum_code_v0(path, add_units=True)` returns a dict with `"units"` key containing
    `[data_iscc, instance_iscc]`
- Calling `gen_sum_code_v0(path)` (default) returns a dict WITHOUT `"units"` key

## Done When

All verification criteria pass: Python binding exposes `add_units` parameter with correct dict
output for both `True` and `False` cases, and all tests (existing + 3 new) pass.
