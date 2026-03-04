## 2026-03-04 — Review of: Fix stale Go code examples in README and howto guide

**Verdict:** PASS

**Summary:** The advance agent fixed two Go documentation bugs exactly as specified. The README Go
quickstart was rewritten from the obsolete WASM-based runtime pattern to the current pure Go
package-level function pattern, and the `GenSumCodeV0` call in `docs/howto/go.md` was corrected to
include the missing `addUnits` 4th parameter. Both fixes match the actual Go function signatures.

**Verification:**

- [x] `grep -c 'NewRuntime\|rt\.Close\|rt\.Gen' README.md` returns 0 — confirmed, no stale runtime
    pattern
- [x] `grep 'GenMetaCodeV0' README.md` shows package-level call — confirmed, matches
    `func GenMetaCodeV0(name string, description, meta *string, bits uint32)` signature
- [x] `grep 'GenSumCodeV0' docs/howto/go.md` shows 4-argument call — confirmed, matches
    `func GenSumCodeV0(path string, bits uint32, wide bool, addUnits bool)` signature
- [x] `uv run zensical build` exits 0 — confirmed, site builds cleanly
- [x] `mise run check` passes — all 15 pre-commit hooks green

**Issues found:**

- (none)

**Codex review:** Codex confirmed the changes are correct — updated Go examples match the current Go
package function signatures. No actionable findings.

**Next:** All `critical` and `normal` priority work is complete. The CID loop is in maintenance/idle
state. All remaining issues in issues.md are `low` priority (C#, C++, Swift, Kotlin bindings, CDC
edge case, language logos). Next step should be an update-state to reflect the Go docs fixes, then
the loop can idle until new `normal`+ priority work is filed.

**Notes:** Minimal, surgical documentation fix. The diff touched only the two files specified in
next.md — perfect scope discipline. No code or test changes needed.
