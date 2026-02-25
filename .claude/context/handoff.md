## 2026-02-25 — Review of: Create Java how-to guide and add to navigation

**Verdict:** PASS

**Summary:** Created `docs/howto/java.md` (319 lines) — the last remaining how-to guide — covering
all 10 required sections. Added Java navigation entry to `zensical.toml` after Go. All API
signatures verified against `IsccLib.java`. Clean implementation, no issues found.

**Verification:**

- [x] `test -f docs/howto/java.md` exits 0 — file exists
- [x] `grep 'io.iscc' docs/howto/java.md` exits 0 — Maven coordinates present (groupId + import)
- [x] `grep 'System.loadLibrary' docs/howto/java.md` exits 0 — native library setup documented
- [x] `grep 'genMetaCodeV0' docs/howto/java.md` exits 0 — code generation documented (3 matches)
- [x] `grep 'dataHasherNew' docs/howto/java.md` exits 0 — streaming documented (2 matches)
- [x] `grep 'textClean' docs/howto/java.md` exits 0 — text utilities documented
- [x] `grep 'conformanceSelftest' docs/howto/java.md` exits 0 — conformance testing documented
- [x] `grep '"Java"' zensical.toml` exits 0 — nav entry present
- [x] `grep 'howto/java.md' zensical.toml` exits 0 — nav path correct
- [x] `uv run zensical build` exits 0 — site builds with `/howto/java/` in output
- [x] All 14 pre-commit hooks pass
- [x] No quality gate circumvention in diff

**Issues found:**

- (none)

**Next:** All 6 how-to guides are complete (Rust, Python, Node.js, WASM, Go, Java). The
documentation site is fully functional. Consider addressing tracked issues from issues.md: codec
header parsing optimization (`[normal]`), DataHasher buffer allocation (`[normal]`), iscc-ffi video
frame allocation (`[normal]`), or lower-priority items like `__version__` attribute, module
docstring fix, WASM conformance feature gate, or the TypeScript port evaluation.

**Notes:** The Java guide is 319 lines, within the target range of 300-400. All 9 `gen*V0` method
signatures match `IsccLib.java` exactly. Streaming section correctly emphasizes opaque `long`
handles with try-finally memory management. The guide correctly documents current behavior
(`IllegalArgumentException` for all errors) — the `IllegalStateException` improvement is tracked
separately in issues.md.
