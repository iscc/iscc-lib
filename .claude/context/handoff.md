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
