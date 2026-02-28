## 2026-02-28 — Review of: Fix stale Go example on landing page

**Verdict:** PASS

**Summary:** Clean, minimal docs-only change that replaces the stale WASM-bridge Go code snippet on
the landing page with the current pure Go API pattern. The diff touches exactly one content file
(`docs/index.md`, 4 lines removed, 2 added) and matches the howto guide pattern precisely. All
verification criteria pass, all 14 hooks pass, docs build cleanly.

**Verification:**

- [x] `grep -c "NewRuntime" docs/index.md` returns 0
- [x] `grep -c "context.Background" docs/index.md` returns 0
- [x] `grep "GenTextCodeV0" docs/index.md` shows pure Go API (no `ctx` parameter)
- [x] `grep "result.Iscc" docs/index.md` returns a match
- [x] `uv run zensical build` exits 0
- [x] `mise run format` produces no changes
- [x] `mise run check` — all 14 hooks pass

**Issues found:**

- (none)

**Codex review:** One P3 finding: the Go snippet discards the error (`_, _`) which could nil-deref
if someone modifies the arguments. Advisory only — all other language tabs on the landing page also
omit error handling for brevity. The howto guide (`docs/howto/go.md`) shows proper `if err != nil`
handling for production use.

**Next:** The remaining filed issue is tab order inconsistency (low priority, needs human review for
canonical order). Beyond doc polish, check target.md and state.md for next high-value work — likely
benchmark speedup documentation or publishing configuration. The "partially met" sections in
state.md are: Documentation (tab order), Benchmarks (speedup not published), CI/CD (Maven Central,
npm, crates.io publishing).

**Notes:** The "Landing page Go example uses stale WASM-bridge API" issue has been resolved and
deleted from issues.md. One issue remains: tab order inconsistency (low priority,
`HUMAN REVIEW REQUESTED` for canonical order decision). PR #10 (develop → main) is still open.
