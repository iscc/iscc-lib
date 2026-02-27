# Next Work Package

## Step: Accept PIL pixel data in Python gen_image_code_v0

## Goal

Widen `gen_image_code_v0` in the Python wrapper to accept
`bytes | bytearray | memoryview | Sequence[int]` for the `pixels` parameter, enabling drop-in
replacement of `iscc-core` when used with PIL/Pillow's `Image.getdata()`. This closes GitHub issue
#4.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/python/iscc_lib/__init__.py` — widen `gen_image_code_v0` signature and add
        conversion logic
    - `tests/test_new_symbols.py` — add tests for `Sequence[int]`, `list[int]`, `bytearray`,
        `memoryview` inputs to `gen_image_code_v0`
- **Reference**:
    - `reference/iscc-core/iscc_core/code_content_image.py` — reference signature uses `Sequence[int]`
    - `.claude/context/specs/python-bindings.md` — verification criteria for PIL pixel data
    - `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — current type stub (stays `bytes` for Rust
        layer)

## Not In Scope

- Changing the Rust core `gen_image_code_v0` signature — it stays `&[u8]`
- Updating `_lowlevel.pyi` — the native Rust function still only accepts `bytes`
- Adding PIL/Pillow as a test dependency — use `list(range(256)) * 4` (1024 ints) as a synthetic
    `Sequence[int]` test input instead
- Implementing `MT`/`ST`/`VS` IntEnum classes (issue #6) — separate step
- Implementing `core_opts` SimpleNamespace (issue #8) — separate step

## Implementation Notes

The change is minimal — in `__init__.py`, modify the `gen_image_code_v0` wrapper:

```python
def gen_image_code_v0(
    pixels: bytes | bytearray | memoryview | Sequence[int], bits: int = 64
) -> ImageCodeResult:
    """Generate an ISCC Image-Code from pixel data."""
    if not isinstance(pixels, bytes):
        pixels = bytes(pixels)
    return ImageCodeResult(_gen_image_code_v0(pixels, bits))
```

This mirrors the pattern already used in `gen_data_code_v0` / `gen_instance_code_v0` for
`bytearray | memoryview` conversion. The `bytes()` constructor handles `bytearray`, `memoryview`,
and `Sequence[int]` (list of ints 0-255) uniformly, so the single `isinstance` check is sufficient.

The `Sequence` import is already present in `__init__.py` (line 6:
`from collections.abc import Sequence`).

For tests, create test inputs that exercise:

1. `list[int]` — `list(range(256)) * 4` (1024 values, each 0-255)
2. `bytearray` — `bytearray(b'\x80' * 1024)`
3. `memoryview` — `memoryview(b'\x80' * 1024)`
4. `bytes` (regression) — `b'\x80' * 1024`

All four should produce an `ImageCodeResult` with an `iscc` key. Additionally, verify that
`bytes(input)` and `list(input)` produce the same ISCC for identical pixel values — i.e., the
`list[int]` path and the `bytes` path produce identical results for equivalent data.

## Verification

- `uv run pytest tests/test_new_symbols.py -x` passes (all existing + new tests)
- `uv run pytest tests/ -x` passes (all 177+ tests)
- `uv run ruff check crates/iscc-py/python/` clean
- `uv run ruff format --check crates/iscc-py/python/` clean
- `python -c "from iscc_lib import gen_image_code_v0; r = gen_image_code_v0(list(range(256))*4); assert 'iscc' in r"`
    exits 0
- `python -c "from iscc_lib import gen_image_code_v0; r = gen_image_code_v0(bytearray(b'\\x80'*1024)); assert 'iscc' in r"`
    exits 0

## Done When

All verification criteria pass, confirming `gen_image_code_v0` accepts `bytes`, `bytearray`,
`memoryview`, and `Sequence[int]` inputs identically.
