## 2026-02-23 — Review of: Python dict returns for all 9 gen functions

**Verdict:** PASS

**Summary:** All 9 `gen_*_v0` PyO3 bindings now return Python `dict` objects with all fields
matching iscc-core's dict returns. The implementation is clean, consistent, and well-tested — 163
Rust tests pass (143 iscc-lib + 20 iscc-ffi), 49 Python tests pass (46 conformance + 3 smoke), all
quality gates green. No lint suppressions, no skipped tests, no circumvention.

**Issues found:**

- (none)

**Next:** The Python bindings are now a drop-in replacement for iscc-core's dict returns. Remaining
binding crates (wasm, napi, ffi) still return plain strings — they could be updated to return
structured data in their respective idioms (JS objects, C structs). The other priority from
`python-bindings.md` is accepting `BinaryIO` (file-like objects) for `gen_data_code_v0` and
`gen_instance_code_v0` streaming functions. Check `target.md` and `state.md` to determine which work
package has the highest impact next.

**Notes:**

- The `PyDict::new(py)` + `set_item` pattern is consistent across all 9 functions — easy to extend
    if new fields are added to result structs.
- Optional fields (`description`, `meta`) are correctly omitted from the dict when `None`, matching
    iscc-core behavior where absent fields simply don't appear in the returned dict.
- Conformance tests now verify ALL output fields (not just `iscc`), providing strong regression
    coverage for any future changes to result struct population.

---

## 2026-02-23 — Spec update: hybrid IsccResult objects for Python bindings

**Action:** Updated `.claude/context/specs/python-bindings.md`

**What was added:** A new "Hybrid Result Objects — Dict + Attribute Access" section specifying that
all 9 `gen_*_v0` Python functions should return typed `IsccResult(dict)` subclasses instead of plain
dicts. This gives users both `result['iscc']` (dict) and `result.iscc` (attribute) access with full
IDE code completion, while remaining a drop-in replacement for iscc-core.

**Key design decisions:**

- Implementation lives entirely in `crates/iscc-py/python/iscc_lib/__init__.py` (pure Python
    wrapper)
- Rust/PyO3 `_lowlevel` module unchanged — continues returning plain `PyDict`
- One `IsccResult(dict)` base class with `__getattr__` delegation
- 9 typed subclasses (`MetaCodeResult`, `TextCodeResult`, etc.) with class-level annotations
- Wrapper functions in `__init__.py` call `_lowlevel` and wrap the result
- No `.pyi` stubs needed for public API — annotations are inline in pure Python source

**Verification criteria added:** 7 new checklist items covering attribute access, dict
compatibility, `isinstance` checks, IDE completion, JSON serialization, and exports.

**Next:** Implement the hybrid result objects per the spec. This is a pure Python change in
`__init__.py` — no Rust modifications needed. Then update tests to verify both access patterns.
