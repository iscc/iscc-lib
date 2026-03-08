## 2026-03-08 — Review of: Fix C++ gen_audio_code_v0 NULL pointer crash on empty vector

**Verdict:** PASS

**IDLE**: All remaining issues are low priority — no actionable work for CID.

**Summary:** The advance agent fixed a real crash bug in the C++ wrapper — `gen_audio_code_v0` used
`cv.data()` directly, which returns NULL for empty vectors on some implementations (libstdc++). The
fix is a single token change to use the existing `detail::safe_data(cv)` helper, matching the
pattern used by all other functions. A regression test (test 35) was added and passes with ASAN. All
54 C++ tests pass, clippy is clean, and all 15 pre-commit hooks pass. Scope discipline is perfect —
only the two specified files were touched.

**Verification:**

- [x] `cmake --build build-ci && ./build-ci/tests/test_iscc` passes with 54 tests (was 53)
- [x] Empty-vector test produces `ISCC:EIAQAAAAAAAAAAAA` (matching C# behavior)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `grep -c 'detail::safe_data' packages/cpp/include/iscc/iscc.hpp` returns 9 call sites (was 8)
    — note: next.md predicted 11→10 counting definitions; actual call site count is 9
- [x] `mise run check` — all 15 hooks pass
- [x] ASAN build + test — 54 passed, 0 failed, no memory errors

**Issues found:**

- (none) — clean fix

**Codex review:** Codex confirmed the change is narrowly scoped, aligns with the existing
`safe_data` pattern, and has no correctness issues. No actionable findings.

**Next:** Three `normal` priority issues remain: Conan recipe fix, .NET docs update, and Zensical
"View as Markdown" 404. After those, only `low` priority issues remain (Swift, Kotlin, logos,
version sync, vcpkg SHA512). The CID loop should signal idle since all `normal` issues are
human-directed and relatively independent.

**Notes:**

- The resolved issue (C++ gen_audio_code_v0 empty vector NULL pointer) has been deleted from
    issues.md
- Updated learnings.md: corrected safe_data count from "10 total call sites" to "11 total
    occurrences (2 definitions + 9 call sites)"
- Consider creating a PR from `develop` to `main` when ready for the next release
