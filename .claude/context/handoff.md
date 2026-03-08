## 2026-03-08 — Review of: Add SHA512 checksums to vcpkg portfile

**Verdict:** PASS

**Summary:** The advance agent replaced `SKIP_SHA512` with per-platform SHA512 checksums in
`packages/cpp/portfile.cmake`. All 5 checksums were independently verified by downloading the v0.2.0
release tarballs and computing SHA512 sums — every hash matches. The change is minimal, correct, and
within scope.

**Verification:**

- [x] `SKIP_SHA512` removed from portfile — grep confirms absence
- [x] `ISCC_SHA512` appears exactly 6 times (5 `set()` + 1 `SHA512` usage)
- [x] `SHA512 "${ISCC_SHA512}"` pattern present in download call
- [x] Each SHA512 hash is 128 lowercase hex characters — `grep -cP` returns 5
- [x] CMake syntax valid — errors on vcpkg functions (expected), not on syntax
- [x] All 15 pre-commit hooks pass (`mise run check`)
- [x] All 5 checksums independently verified against actual downloaded tarballs
- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean

**Issues found:**

- (none) — clean implementation

**Codex review:** No actionable findings. Codex confirmed the change is limited to replacing
`SKIP_SHA512` with per-platform SHA512 pins and introduces no correctness or compatibility
regressions.

**Next:** The only remaining `normal`-priority issue is "Add programming language logos to README
and docs." After that, only `low`-priority items remain (Swift/Kotlin bindings) and the CID loop
should signal idle.

**Notes:** The resolved vcpkg SHA512 issue has been deleted from issues.md. The advance handoff
correctly noted that future releases will need SHA512 checksums updated — this is not automated yet
(explicitly out of scope per next.md). The Conan recipe (`conanfile.py`) also lacks SHA512
verification but no issue is filed for it. State.md line 135-136 still references the open issue —
update-state will pick this up next cycle.
