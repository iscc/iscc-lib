# Next Work Package

## Step: Fix Python bytes-like input handling and chunked streaming

## Goal

Fix two related Python binding issues: (1) `bytearray`/`memoryview` inputs are misclassified as
streams and crash with `AttributeError`, and (2) file-like stream inputs are read entirely into
memory via unbounded `.read()`, defeating the purpose of streaming. Both issues affect the same 4
call sites in `__init__.py` and should be fixed together.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-py/python/iscc_lib/__init__.py` (fix 4 stream-handling call sites),
    `tests/test_streaming.py` (add tests for bytearray, memoryview, and chunked streaming)
- **Reference**: `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (Rust-layer type signatures),
    `.claude/context/learnings.md` (ty type checker constraints on `hasattr`)

## Not In Scope

- Updating `_lowlevel.pyi` type stubs — the Rust layer correctly accepts `bytes`; the Python wrapper
    widens the accepted types
- Adding `__version__` attribute (separate low-priority issue)
- Fixing the module docstring in `lib.rs` (separate low-priority issue)
- Performance benchmarking of the chunked streaming — correctness is the goal here
- Changing the chunk size constant (64 KiB is a standard choice; tuning is premature)

## Implementation Notes

### Issue 1: Bytes-like input misclassification

The current code uses `if not isinstance(data, bytes)` to detect streams, but `bytearray` and
`memoryview` are not `bytes` instances and lack `.read()`.

**Fix pattern** — use tuple isinstance check to accept all bytes-like types:

```python
if not isinstance(data, (bytes, bytearray, memoryview)):
    # This is a stream (BinaryIO)
    ...
elif not isinstance(data, bytes):
    data = bytes(data)  # bytearray/memoryview → bytes for Rust FFI
```

**Important**: Do NOT use `hasattr(data, "read")` — the `ty` type checker does not support
`hasattr()`-based type narrowing (see learnings.md). The `isinstance` inversion pattern gives `ty`
proper narrowing: after `not isinstance(data, (bytes, bytearray, memoryview))`, the remaining type
is `BinaryIO`, which has `.read()`.

### Issue 2: Unbounded `.read()` defeats streaming

**For `gen_data_code_v0` and `gen_instance_code_v0`** — when given a stream, use the Rust streaming
hashers (`_DataHasher`/`_InstanceHasher`) with chunked reads instead of `.read()` + one-shot:

```python
_CHUNK_SIZE = 65536  # 64 KiB read chunks


def gen_data_code_v0(data, bits=64):
    if not isinstance(data, (bytes, bytearray, memoryview)):
        hasher = _DataHasher()
        while chunk := data.read(_CHUNK_SIZE):
            hasher.update(chunk)
        return DataCodeResult(hasher.finalize(bits))
    if not isinstance(data, bytes):
        data = bytes(data)
    return DataCodeResult(_gen_data_code_v0(data, bits))
```

Same pattern for `gen_instance_code_v0` using `_InstanceHasher`.

**For `DataHasher.update` and `InstanceHasher.update`** — when given a stream, read in chunks and
feed each chunk to the inner Rust hasher:

```python
def update(self, data):
    if not isinstance(data, (bytes, bytearray, memoryview)):
        while chunk := data.read(_CHUNK_SIZE):
            self._inner.update(chunk)
    else:
        if not isinstance(data, bytes):
            data = bytes(data)
        self._inner.update(data)
```

### Type annotations

Widen the type annotation for all 4 sites plus the constructor parameters:

```python
from typing import BinaryIO, Union

# Use bytes | bytearray | memoryview | BinaryIO for all data parameters
```

### Affected call sites (6 total, but constructors delegate to update)

1. `gen_data_code_v0` (line 149-150)
2. `gen_instance_code_v0` (line 156-157)
3. `DataHasher.update` (line 184-185)
4. `InstanceHasher.update` (line 208-209)
5. `DataHasher.__init__` — delegates to `self.update()`, no direct fix needed
6. `InstanceHasher.__init__` — delegates to `self.update()`, no direct fix needed

### New tests to add in `tests/test_streaming.py`

- `test_gen_data_code_v0_bytearray` — `bytearray` input matches `bytes` input
- `test_gen_data_code_v0_memoryview` — `memoryview` input matches `bytes` input
- `test_gen_instance_code_v0_bytearray` — same for instance code
- `test_gen_instance_code_v0_memoryview` — same for instance code
- `test_data_hasher_bytearray` — `DataHasher.update(bytearray(...))` works
- `test_data_hasher_memoryview` — `DataHasher.update(memoryview(...))` works
- `test_instance_hasher_bytearray` — `InstanceHasher.update(bytearray(...))` works
- `test_instance_hasher_memoryview` — `InstanceHasher.update(memoryview(...))` works
- `test_gen_data_code_v0_stream_chunked` — verify a large `BytesIO` stream produces the same result
    as one-shot bytes (proves chunked streaming works)
- `test_gen_instance_code_v0_stream_chunked` — same for instance code

## Verification

- `pytest tests/test_streaming.py` passes (existing 28 tests + ~10 new tests)
- `pytest` passes all 105+ tests
- `ruff check crates/iscc-py/python/ tests/` clean
- `ruff format --check crates/iscc-py/python/ tests/` clean
- `grep -c 'isinstance(data, bytes)' crates/iscc-py/python/iscc_lib/__init__.py` returns 0 (old
    pattern eliminated)

## Done When

All verification criteria pass: no `isinstance(data, bytes)` remains, bytearray/memoryview inputs
produce correct results, stream inputs use chunked reads, and all existing + new tests pass.
