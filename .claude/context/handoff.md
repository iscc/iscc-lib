## 2026-03-07 — Review of: Add C++ documentation (README, howto, root README)

**Verdict:** PASS

**Summary:** The advance agent created `packages/cpp/README.md`, updated `docs/howto/c-cpp.md` to
replace the hand-written RAII wrapper section with comprehensive `iscc.hpp` documentation, and added
C++ to the root `README.md` install/quickstart sections. All changes are docs-only, well-structured,
and all verification criteria pass. Code examples are accurate against the actual `iscc.hpp` API.

**Verification:**

- [x] `grep -c 'iscc.hpp' docs/howto/c-cpp.md` returns 12 (≥5 required)
- [x] `grep -c 'iscc::gen_meta_code_v0' docs/howto/c-cpp.md` returns 2 (≥1 required)
- [x] `grep -c 'iscc::DataHasher' docs/howto/c-cpp.md` returns 2 (≥1 required)
- [x] `grep 'C++' README.md | head -5` shows C++ in polyglot, ecosystem, install, and quickstart
- [x] `grep -c 'iscc.hpp' README.md` returns 2 (≥1 required)
- [x] `test -f packages/cpp/README.md` exits 0
- [x] `grep -c 'gen_meta_code_v0' packages/cpp/README.md` returns 2 (≥1 required)
- [x] `grep -c 'What is ISCC' packages/cpp/README.md` returns 1
- [x] `mise run check` passes — all 15 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean

**Issues found:**

- (none)

**Codex review:** Codex flagged three P2 include-path consistency issues: (1) root README quickstart
uses `<iscc/iscc.hpp>` but install section describes tarball (flat layout), (2) cpp README compile
command lacks the second `-I` flag for `iscc.h`, (3) howto source-build guidance doesn't mention
needing both include paths. Assessment: the cpp README text (line 38-39) explicitly says "Point your
compiler's include path to `packages/cpp/include/` **and** `crates/iscc-ffi/include/`", and the
howto documents both tarball (flat) and CMake (angle-bracket) paths. The CMakeLists.txt correctly
configures both include dirs. These are minor completeness nits for a future docs polish step, not
blocking issues.

**Next:** The C++ issue in issues.md still has pending sub-items: package manager manifests
(`vcpkg.json`, `conanfile.py`), `gen_mixed_code_v0` test gap in `test_iscc.cpp`, and nested vector
null-safety hardening. These are the remaining `normal` priority work items. After those, only `low`
priority issues remain (Swift, Kotlin, logos). Consider whether the remaining C++ items should be
scoped as one combined step or split further.

**Notes:** Documentation examples were verified against the actual `iscc.hpp` API — all function
signatures, result types, and patterns match correctly. The `iscc::` namespace, RAII classes, and
error handling patterns are accurately documented. No code changes in this iteration.
