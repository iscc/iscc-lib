## 2026-02-23 — Implement hybrid IsccResult(dict) objects in Python

**Done:** Replaced direct `_lowlevel` re-exports in `__init__.py` with an `IsccResult(dict)` base
class, 9 typed subclasses (`MetaCodeResult`, `TextCodeResult`, etc.), and 9 wrapper functions. All
`gen_*_v0` Python functions now return typed result objects supporting both `result['iscc']` dict
access and `result.iscc` attribute access with IDE code completion.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/__init__.py`: Replaced direct re-exports with `IsccResult` base
    class, 9 typed subclasses with class-level annotations, 9 wrapper functions calling `_lowlevel`
    prefixed imports, and expanded `__all__` to 19 symbols
- `tests/test_smoke.py`: Added 8 new tests for attribute access, `isinstance` checks, `json.dumps`
    serialization, `AttributeError` on missing keys, and dict/attribute equality

**Verification:** All quality gates pass clean. 163 Rust tests pass, 57 Python tests pass (46
conformance + 11 smoke). `ruff check` and `ruff format` clean. Conformance tests unchanged and still
pass — dict access backward compatible. Specific verifications: `gen_meta_code_v0("Test").iscc`
returns valid ISCC string, `isinstance` checks pass for `dict`, `IsccResult`, and specific
subclasses, `json.dumps` works, `AttributeError` raised for missing attributes.

**Next:** The Python bindings are now feature-complete with hybrid dict+attribute access. Next
priorities from `target.md` could include: accepting `BinaryIO` (file-like objects) for streaming
functions (`gen_data_code_v0`, `gen_instance_code_v0`), updating napi/wasm/ffi bindings to return
structured data, or documentation improvements.

**Notes:** No Rust changes required — this was a pure Python wrapper implementation. The
`__getattr__` → `__getitem__` delegation pattern is minimal and robust. The `from None` in the
`AttributeError` raise correctly suppresses the `KeyError` chain for clean tracebacks. The
`__annotations__` on subclasses are purely for IDE/type-checker consumption — `dict.__init__`
handles all data storage.
