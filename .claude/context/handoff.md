## 2026-02-24 — Fix Python bytes-like input handling and chunked streaming

**Done:** Fixed 4 call sites in `__init__.py` to accept `bytearray`/`memoryview` inputs (converted
to `bytes` for Rust FFI) and use chunked 64 KiB reads for `BinaryIO` streams instead of unbounded
`.read()`. Added 10 new tests covering bytearray, memoryview, and chunked streaming for both
`gen_data_code_v0`/`gen_instance_code_v0` and `DataHasher`/`InstanceHasher`.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/__init__.py`: Widened type annotations to accept
    `bytes | bytearray | memoryview | BinaryIO`; stream detection uses
    `isinstance(data, (bytes, bytearray, memoryview))` tuple check (not `hasattr`); stream paths use
    chunked reads via `_CHUNK_SIZE = 65536`; bytearray/memoryview converted to `bytes` before Rust
    FFI calls
- `tests/test_streaming.py`: Added 10 new tests — 4 bytearray tests, 4 memoryview tests, 2 chunked
    streaming tests with large (215 KB) data

**Verification:**

- `pytest tests/test_streaming.py` — 41 passed (28 existing + 10 new + 3 conformance parametrized)
- `mise run check` — all 14 pre-commit hooks pass
- Pre-push hooks — all 15 hooks pass (clippy, cargo test, ty check, pytest, security, complexity)
- `grep -c 'isinstance(data, bytes)'` returns 4 (not 0) — the 4 matches are the new inner
    bytearray/memoryview→bytes conversion pattern
    (`if not isinstance(data, bytes): data =   bytes(data)`), NOT the old stream-detection pattern.
    The old pattern `if not isinstance(data, bytes): data = data.read()` is completely eliminated.
    The verification criterion as literally written in next.md conflicts with the implementation
    pattern also specified in next.md.

**Next:** The two Python binding issues (bytes-like misclassification + unbounded read) are resolved
and can be deleted from issues.md. Next candidates: JNI `jint` negative value validation, JNI local
reference overflow, napi version skew, napi npm packaging, napi unnecessary clone, wasm silent null
return, FFI video frame copy.

**Notes:** The `Union` import from `typing` was initially added per next.md's suggestion but removed
because `from __future__ import annotations` already enables `|` union syntax in annotations — the
`Union` import would be flagged as unused. The `_CHUNK_SIZE` constant was initially placed before
imports (causing ruff E402), moved after imports to fix.
