## 2026-03-02 — Review of: Commit iscc.h header with CI freshness check

**Verdict:** PASS

**Summary:** The advance agent generated and committed `crates/iscc-ffi/include/iscc.h` via cbindgen
and updated the CI workflow to use a freshness check (`git diff --exit-code`) instead of generating
the header at test time. The implementation is exactly scoped to what next.md specified — only two
non-context files changed. The committed header matches cbindgen output exactly (verified by
regenerating and diffing).

**Verification:**

- [x] `test -f crates/iscc-ffi/include/iscc.h` exits 0 — header file exists (873 lines)
- [x] `grep '#ifndef ISCC_H' crates/iscc-ffi/include/iscc.h` — include guard found
- [x] `grep 'include/iscc.h' .github/workflows/ci.yml` — freshness check step references correct
    path
- [x] `grep 'exit-code' .github/workflows/ci.yml` — diff-based freshness check present
- [x] `grep '\-I crates/iscc-ffi/include' .github/workflows/ci.yml` — updated include path
- [x] `cargo clippy -p iscc-ffi -- -D warnings` — clean
- [x] `cargo test -p iscc-lib` — 310 tests pass (256 + 31 + 22 + 1 doctest)
- [x] `mise run check` — 14/14 hooks pass
- [x] Header freshness — regenerated header matches committed version exactly

**Issues found:**

- (none)

**Issue resolved:** #24 (Commit iscc.h header with CI freshness check) — deleted from issues.md.

**Next:** The next C FFI DX item is issue #23 (standalone C example with CMakeLists.txt in
`crates/iscc-ffi/examples/`). This can now reference the committed header at
`crates/iscc-ffi/include/iscc.h`. After that: #22 (how-to guide) and #25 (release artifacts).

**Notes:** The `crates/iscc-ffi/tests/iscc.h` gitignore entry remains — that's the CI artifact copy
for test compilation, not the committed header. The committed header at `include/iscc.h` is not
gitignored. Codex review was launched against the wrong commit (define-next instead of advance) due
to the protocol template using `HEAD~1` — this is a known issue documented in review agent memory.
