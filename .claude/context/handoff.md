## 2026-02-23 — Python streaming support for gen_data_code_v0 and gen_instance_code_v0

**Done:** Added `BinaryIO` (file-like stream) support to `gen_data_code_v0` and
`gen_instance_code_v0` in the Python wrapper layer. Both functions now accept `bytes | BinaryIO`
using duck typing (`hasattr(data, "read")`), matching the iscc-core API. No Rust changes needed.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/__init__.py`: Added `BinaryIO` import from `typing`, updated
    `gen_data_code_v0` and `gen_instance_code_v0` signatures to accept `bytes | BinaryIO`, added
    `hasattr(data, "read")` duck-type check to read stream contents before passing to Rust
- `tests/test_smoke.py`: Added 6 streaming tests — BytesIO-vs-bytes equality for both functions,
    empty stream handling for both, and plain bytes regression tests for both

**Verification:** All quality gates pass. 143 Rust tests pass, 63 Python tests pass (46 conformance,
17 smoke). Pre-commit hooks clean. No regressions.

**Next:** The Python drop-in compatibility story is now complete (hybrid dict+attribute results,
BinaryIO streaming). Consult `target.md` for highest-impact next step. Candidates: structured
returns for Node.js/WASM/FFI bindings, remaining Tier 1 API symbols, or documentation improvements.

**Notes:** The implementation is minimal — 2 lines added per function (duck-type check + read). The
`from __future__ import annotations` already present enables the `bytes | BinaryIO` union syntax. No
surprises or technical debt introduced.
