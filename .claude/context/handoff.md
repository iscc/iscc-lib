## 2026-02-28 — Review of: Add C FFI API reference page

**Verdict:** PASS

**Summary:** Clean, well-scoped documentation addition. The new `docs/c-ffi-api.md` documents all 44
exported `extern "C"` symbols from `crates/iscc-ffi/src/lib.rs` with accurate C type mappings,
struct layouts matching `#[repr(C)]` definitions, and complete memory management guidance. The doc
site builds cleanly, nav entry and llms.txt updated.

**Verification:**

- [x] `uv run zensical build` succeeds — builds in 0.89s with new page
- [x] `grep -q 'c-ffi-api.md' zensical.toml` exits 0 — nav entry present after Python API
- [x] `grep -q 'c-ffi' docs/llms.txt` exits 0 — reference line added
- [x] `grep -c 'iscc_gen_' docs/c-ffi-api.md` returns 21 (≥ 9) — all gen functions documented
- [x] `grep -c 'iscc_free_' docs/c-ffi-api.md` returns 29 (≥ 4) — memory management complete
- [x] `grep -q 'iscc_last_error' docs/c-ffi-api.md` exits 0 — error handling documented
- [x] `mise run check` — all 14 hooks pass

**Issues found:**

- (none)

**Codex review:** Codex identified a minor conservatism in the error handling docs: the page says
the `iscc_last_error()` pointer is "valid until the next `iscc_*` call" while the Rust source says
"next `iscc_gen_*` call". The doc is more conservative (safer for callers) — not actionable. Codex
verified `iscc_encode_component` parameter order and `iscc_alloc`/`iscc_dealloc` semantics are
accurately documented. No actionable findings.

**Next:** The project is in maintenance mode. All functional requirements are met. The documentation
spec also lists "Java API" reference under Reference, which is the only remaining doc page gap. All
other remaining work is human-dependent: merge PR #10 (develop → main), trigger releases, configure
Maven Central publishing, and decide canonical tab order.

**Notes:** The advance agent noted 44 exported symbols vs next.md's "43" — the advance correctly
counted and documented all 44. Internal helpers (`result_to_c_string`, `string_to_c`,
`vec_to_c_string_array`, etc.) are properly excluded from documentation.
