## 2026-02-27 — Accept PIL pixel data in Python gen_image_code_v0

**Done:** Widened `gen_image_code_v0` in the Python wrapper to accept
`bytes | bytearray | memoryview | Sequence[int]` for the `pixels` parameter, enabling drop-in
replacement of `iscc-core` when used with PIL/Pillow's `Image.getdata()`. This closes GitHub issue
#4. The change is a 3-line modification using the same `isinstance` + `bytes()` conversion pattern
already used by `gen_data_code_v0` and `gen_instance_code_v0`.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/__init__.py`: Widened `gen_image_code_v0` signature from `bytes`
    to `bytes | bytearray | memoryview | Sequence[int]`, added `isinstance` check with `bytes()`
    conversion for non-bytes inputs
- `tests/test_new_symbols.py`: Added 7 tests covering `list[int]`, `bytearray`, `memoryview`,
    `bytes` regression, and equivalence checks between all input types

**Verification:**

- [x] `uv run pytest tests/test_new_symbols.py -x` passes — 25 tests (18 existing + 7 new)
- [x] `uv run pytest tests/ -x` passes — 184 tests, 0 failures
- [x] `uv run ruff check crates/iscc-py/python/` clean
- [x] `uv run ruff format --check crates/iscc-py/python/` clean
- [x] `python -c "from iscc_lib import gen_image_code_v0; r = gen_image_code_v0(list(range(256))*4); assert 'iscc' in r"`
    exits 0
- [x] `python -c "from iscc_lib import gen_image_code_v0; r = gen_image_code_v0(bytearray(b'\\x80'*1024)); assert 'iscc' in r"`
    exits 0
- [x] All 14 pre-commit hooks pass (`mise run check`)

**Next:** Issue #4 Python layer is complete. Three remaining Python iscc-core drop-in extensions:
(1) Issue #6 — `MT`/`ST`/`VS` IntEnum classes for Python, (2) Issue #7 — `iscc_decode` returning
IntEnum-typed tuples, (3) Issue #8 — `core_opts` SimpleNamespace. All are independent Python-wrapper
changes. After these drop-in extensions are done, propagate the new Tier 1 symbols to the remaining
bindings (Node.js, WASM, C FFI, Java, Go).

**Notes:** The implementation is minimal and follows the exact pattern suggested in next.md. The
`Sequence` import was already present in `__init__.py` (line 6). The `bytes()` constructor handles
all three non-bytes types uniformly: `bytearray`, `memoryview`, and `Sequence[int]` (including PIL's
`ImagingCore`). No Rust changes needed — the conversion happens entirely in the Python wrapper.
