## 2026-02-27 — Add MT/ST/VS IntEnums and core_opts to Python

**Done:** Added three `IntEnum` classes (`MT`, `ST`, `VS`), a `core_opts` `SimpleNamespace`, and
wrapped `iscc_decode` to return IntEnum-typed tuple values in the Python `__init__.py`. All symbols
are exported in `__all__`. This closes the Python-side of Issues #6, #7, and #8.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/__init__.py`: Added `import enum` and
    `from types import   SimpleNamespace`; imported `iscc_decode` as `_iscc_decode` (renamed); added
    `MT`, `ST`, `VS` IntEnum classes; added `core_opts` SimpleNamespace; added `iscc_decode` wrapper
    returning IntEnum-typed tuples; added `MT`, `ST`, `VS`, `core_opts` to `__all__`
- `tests/test_new_symbols.py`: Added 14 new tests covering MT values, ST values with TEXT alias, VS
    value, core_opts attributes, iscc_decode IntEnum return types, and round-trip with enum values

**Verification:** All 39 tests in `test_new_symbols.py` pass (25 existing + 14 new). Full suite: 198
tests pass. All 14 pre-commit hooks pass. Ruff check and format clean. All three inline verification
commands from next.md exit 0.

**Next:** Issues #6, #7, #8 Python-side work is complete. Next step: propagate these extensions to
other bindings (Node.js TS const enums, WASM, C FFI, Java, Go) or tackle remaining items from
issues.md (e.g., Issue #5 dict meta propagation to other bindings).

**Notes:** The `GABTQLB6CQ6ILWLO` code from the next.md verification criteria was invalid (8-byte
body but header indicates 128-bit/16-byte digest). Used `GAA2XTPPAERUKZ4J` instead — a valid 64-bit
Data-Code generated via `encode_component`. The `ISCC:AAAZXZ6OU74YAZIM` code from verification
worked correctly for the `isinstance` check.
