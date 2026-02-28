## 2026-02-28 — Fix stale Go example on landing page

**Done:** Replaced the stale WASM-bridge Go code snippet on the documentation landing page
(`docs/index.md`) with the current pure Go API pattern. Removed `context.Background()`,
`NewRuntime`, `defer rt.Close`, and changed from method-on-runtime to package-level function
returning a struct with `.Iscc` field.

**Files changed:**

- `docs/index.md`: Updated Go tab code block (lines 114-122 → 114-118) to use pure Go API pattern
    matching the howto guide

**Verification:**

- `NewRuntime` — 0 occurrences in docs/index.md ✓
- `context.Background` — 0 occurrences in docs/index.md ✓
- `GenTextCodeV0` — shows pure Go API (no `ctx` parameter) ✓
- `result.Iscc` — match found ✓
- `uv run zensical build` — exits 0, builds in 0.73s ✓
- `mise run format` — no changes ✓
- `mise run check` — all 14 hooks pass ✓

**Next:** The remaining doc issues from the previous review are: (1) tab order inconsistency across
pages (landing page vs tutorial vs spec), and (2) WASM `init()` inconsistency in tutorial examples.
Both are low priority. The project could move on to other target items — check state.md and
target.md for the next high-value work.

**Notes:** Minimal change — only the Go tab code block was modified, exactly as specified. No
surprises. The import line and install command were already correct and left unchanged.
