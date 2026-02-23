# Next Work Package

## Step: Python streaming support for gen_data_code_v0 and gen_instance_code_v0

## Goal

Make `gen_data_code_v0` and `gen_instance_code_v0` accept both `bytes` and file-like objects
(`BinaryIO`) in Python, matching the iscc-core API signature. This completes the Python drop-in
compatibility story — iscc-core users can switch to iscc-lib without changing code that passes file
streams.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/python/iscc_lib/__init__.py` — add `BinaryIO` union type + read logic
    - `tests/test_smoke.py` — add streaming tests
- **Reference**:
    - `reference/iscc-core/iscc_core/code_data.py` — shows `stream.read()` pattern
    - `reference/iscc-core/iscc_core/code_instance.py` — shows `stream.read()` pattern

## Implementation Notes

**Approach**: Handle streaming entirely in the Python wrapper layer. No Rust changes needed. The
Rust `_lowlevel` functions continue to accept `bytes`. The Python wrappers detect file-like objects
and read all bytes before calling Rust.

**Pattern for both `gen_data_code_v0` and `gen_instance_code_v0`**:

```python
def gen_data_code_v0(data: bytes | BinaryIO, bits: int = 64) -> DataCodeResult:
    if hasattr(data, "read"):
        data = data.read()
    return DataCodeResult(_gen_data_code_v0(data, bits))
```

Use duck typing (`hasattr(data, "read")`) rather than `isinstance(BinaryIO)` — this matches how
iscc-core interacts with streams (calling `.read()`) and works with any object that has a `.read()`
method (file handles, `io.BytesIO`, custom streams).

**Type annotations**: Use `typing.BinaryIO` in the type hint for documentation/IDE support:
`data: bytes | BinaryIO`. Import `BinaryIO` from `typing`. The `from __future__ import annotations`
is already present, so the union syntax works.

**Important**: iscc-core calls `stream.read(io_read_size)` in a loop, but since we're reading all
bytes to pass to Rust anyway, a single `data.read()` (no size argument = read all) is correct and
simpler.

**Tests**: Add tests to `test_smoke.py` that verify:

1. `gen_data_code_v0(io.BytesIO(b"..."))` produces the same result as `gen_data_code_v0(b"...")`
2. `gen_instance_code_v0(io.BytesIO(b"..."))` produces the same result as
    `gen_instance_code_v0(b"...")`
3. Both functions still work with plain `bytes` (existing tests cover this)
4. Empty `BytesIO()` works correctly

## Verification

- `pytest tests/` passes with all existing + new streaming tests
- `gen_data_code_v0(io.BytesIO(b"Hello"))` returns same ISCC as `gen_data_code_v0(b"Hello")`
- `gen_instance_code_v0(io.BytesIO(b"Hello"))` returns same result (iscc, datahash, filesize) as
    `gen_instance_code_v0(b"Hello")`
- `gen_data_code_v0(io.BytesIO(b""))` works (empty stream)
- Plain `bytes` input still works unchanged for both functions
- `cargo test -p iscc-lib` still passes (no Rust changes)
- `ruff check` and `ruff format --check` clean

## Done When

All verification criteria pass — both `gen_data_code_v0` and `gen_instance_code_v0` accept
`bytes | BinaryIO` in Python with identical output for both input types.
