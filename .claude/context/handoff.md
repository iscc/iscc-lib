## 2026-03-02 — Review of: Create C/C++ how-to guide (issue #22)

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent created `docs/howto/c-cpp.md` with all 12 required sections and added
the navigation entry in `zensical.toml`. The content is well-structured, follows the established
howto guide conventions, and the site builds successfully. Review found that all code examples used
un-prefixed type names (`FfiDataHasher`, `IsccSumCodeResult`) instead of the actual `iscc_`-prefixed
names from the generated header — these would not compile. Fixed directly by the review agent.

**Verification:**

- [x] `test -f docs/howto/c-cpp.md` — file exists
- [x] `grep -q 'c-cpp.md' zensical.toml` — nav entry present after Java
- [x] `grep -q 'iscc_gen_sum_code_v0' docs/howto/c-cpp.md` — ISCC-SUM quick start present
- [x] `grep -q 'iscc_data_hasher' docs/howto/c-cpp.md` — streaming section present
- [x] `grep -q 'iscc_last_error' docs/howto/c-cpp.md` — error handling present
- [x] `grep -q 'iscc_free_string' docs/howto/c-cpp.md` — memory management present
- [x] `grep -q 'RAII' docs/howto/c-cpp.md` — C++ RAII wrapper present
- [x] `grep -q 'cmake' docs/howto/c-cpp.md` — CMake integration present
- [x] `uv run zensical build` — site builds (0.57s)
- [x] `mise run check` — 14/14 hooks pass

**Issues found:**

- Type names in code examples used un-prefixed forms (`FfiDataHasher`, `IsccSumCodeResult`) instead
    of the `iscc_`-prefixed forms from `iscc.h` (`iscc_FfiDataHasher`, `iscc_IsccSumCodeResult`).
    cbindgen config has `[export] prefix = "iscc_"`. Fixed by review — all code blocks and ownership
    table now use correct type names.

**Issue resolved:** #22 (C/C++ how-to guide) — deleted from issues.md. Spec §1 checkboxes updated.

**Codex review:** One P1 finding — identical to the type name prefix issue already identified and
fixed by the review agent. No additional findings.

**Next:** Issue #25 (pre-built FFI release tarballs) is the remaining C FFI DX target item.
Alternatively, #21 (units support) or #16 (feature flags) are independent enhancements. The C FFI DX
spec §§1-3 are now fully met; only §4 (release tarballs) remains.

**Notes:** The `c-ffi-api.md` API reference page also uses un-prefixed type names in its type
definition blocks (e.g., `IsccByteBuffer` instead of `iscc_IsccByteBuffer`). This is an existing
documentation convention for exposition, but creates an inconsistency with the howto guide (which
now uses correct prefixed names). A future cleanup pass could align both pages. Not blocking.
