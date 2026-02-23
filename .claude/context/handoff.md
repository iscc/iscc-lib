## 2026-02-23 — Review of: Add structured result types for all 9 gen functions

**Verdict:** PASS

**Summary:** All 9 `gen_*_v0` functions now return dedicated `*CodeResult` structs with additional
fields (metahash, name, description, meta, characters, parts, datahash, filesize) matching
iscc-core's dict returns. The implementation is clean, correct, and well-tested — 163 Rust tests and
49 Python tests pass, all quality gates green.

**Issues found:**

- (none)

**Next:** Implement Python dict returns in the PyO3 binding layer. The `iscc-py` crate currently
extracts `.iscc` and returns plain strings. Convert it to return Python `dict` objects with all
fields from the `*CodeResult` structs, matching the iscc-core API. See
`notes/specs/python-bindings.md` for the complete spec and
`.claude/context/specs/python-bindings.md` for field listings. The `.pyi` type stubs and Python
conformance tests will need updating to expect dicts instead of strings.

**Notes:**

- The `build_meta_data_url` helper re-parses JSON from the original input string to check `@context`
    (the canonical bytes are already available from `parse_meta_json`). This is a minor inefficiency
    that could be optimized by threading the parsed `serde_json::Value` through, but it's negligible
    for typical metadata sizes.
- All binding crates (py, wasm, napi, ffi) were mechanically updated to add `.map(|r| r.iscc)`. The
    WASM, napi, and FFI crates can remain string-based for now — Python dict returns are the
    priority per the target state.
- Description normalization was correctly moved before the meta/text branch to match iscc-core
    behavior (Python normalizes description at line 57-59, before the `if meta:` check).
