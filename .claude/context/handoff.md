## 2026-02-24 — Review of: Fix napi packaging, version skew, and unnecessary clone

**Verdict:** PASS

**Summary:** All three napi normal-priority issues resolved cleanly. Added `"files"` allowlist to
`package.json`, replaced `.iter().map(|c| Buffer::from(c.to_vec()))` with
`.into_iter().map(Buffer::from)` eliminating per-chunk allocation, and regenerated `index.js` with
correct `0.0.1` version. Diff is minimal, focused, and scope-correct.

**Verification:**

- [x] `cargo clippy -p iscc-napi -- -D warnings` clean
- [x] `npm test` passes in `crates/iscc-napi/` — all 103 tests (23 suites), 0 failures
- [x] `grep '"files"' crates/iscc-napi/package.json` returns a match
- [x] `grep 'c\.to_vec()' crates/iscc-napi/src/lib.rs` returns 0 matches
- [x] `grep -c 'expected 0.0.1' crates/iscc-napi/index.js` returns 26
- [x] `mise run check` passes all 14 pre-commit hooks
- [x] No quality gate circumvention patterns in diff

**Issues found:**

- (none)

**Next:** Three napi issues resolved and deleted from issues.md. Remaining normal-priority
candidates: WASM silent null return (`alg_cdc_chunks`), FFI video frame copy, codec header parsing
optimization, DataHasher allocation overhead. Low priority: alg_dct power-of-two, alg_wtahash bounds
check, JNI exception types, Python __version__, Python module docstring, WASM conformance_selftest
size, WASM stale CLAUDE.md, TypeScript port evaluation.

**Notes:** napi-rs `Buffer` implements `From<&[u8]>` directly in v3, so the fallback path with
`.to_vec()` was unnecessary. The test count is now 103 (up from 66 when the issues were originally
filed — streaming hasher tests were added in a prior iteration). The regenerated `index.js`/
`index.d.ts` remain gitignored per convention.
