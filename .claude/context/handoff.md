## 2026-02-24 — Review of: Fix Python bytes-like input handling and chunked streaming

**Verdict:** PASS

**Summary:** All 4 call sites in `__init__.py` correctly updated to accept `bytearray`/`memoryview`
inputs (with conversion to `bytes` for Rust FFI) and use chunked 64 KiB reads for `BinaryIO` streams
instead of unbounded `.read()`. 10 new tests cover all input types and chunked streaming.
Implementation matches next.md exactly, with clean scope discipline.

**Verification:**

- [x] `pytest tests/test_streaming.py` passes — 41 passed (28 existing + 10 new + 3 conformance)
- [x] `pytest` passes all tests — 157 passed
- [x] `ruff check crates/iscc-py/python/ tests/` clean
- [x] `ruff format --check crates/iscc-py/python/ tests/` clean
- [x] Old pattern eliminated — `grep 'data = data\.read()' __init__.py` returns 0 matches; the 4
    `isinstance(data, bytes)` hits are the new inner bytearray/memoryview→bytes conversion, not the
    old stream-detection pattern
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] No quality gate circumvention — no suppressions, no test skipping, no threshold changes

**Issues found:**

- (none) — clean implementation, exact scope match

**Next:** Two resolved Python binding issues deleted from issues.md. Remaining normal-priority
candidates: JNI `jint` negative value validation, JNI local reference overflow, napi version skew,
napi npm packaging, napi unnecessary clone, wasm silent null return, FFI video frame copy. Low
priority: `alg_dct` power-of-two, `alg_wtahash` bounds check, JNI exception types, Python
`__version__`, Python module docstring, WASM conformance_selftest size, WASM stale CLAUDE.md,
TypeScript port evaluation.

**Notes:** The verification criterion `grep -c 'isinstance(data, bytes)' returns 0` in next.md was
contradictory with the implementation pattern it also specified (inner `isinstance(data, bytes)` for
bytearray→bytes conversion). Future next.md criteria should use more specific patterns — e.g.,
`grep 'data = data\.read()'` to test for the actual problematic code.
