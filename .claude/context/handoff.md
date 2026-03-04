## 2026-03-04 — Fix stale Go code examples in README and howto guide

**Done:** Fixed two documentation bugs where Go code examples used outdated function signatures.
Rewrote the README Go quickstart section to use the pure Go package-level function pattern (removing
obsolete WASM-based `NewRuntime`/`rt.Close`/`rt.Gen` pattern). Added the missing `addUnits bool` 4th
parameter to `GenSumCodeV0` call in `docs/howto/go.md`.

**Files changed:**

- `README.md`: Rewrote Go quickstart (lines 160-188) — removed `context` import, `NewRuntime`,
    `defer rt.Close`, replaced `rt.GenMetaCodeV0(ctx, ...)` with package-level
    `iscc.GenMetaCodeV0(...)`, changed `code` to `result.Iscc`
- `docs/howto/go.md`: Fixed `GenSumCodeV0` call on line 206 — added missing 4th argument `false` for
    `addUnits` parameter

**Verification:**

- ✅ `grep -c 'NewRuntime\|rt\.Close\|rt\.Gen' README.md` returns 0 — no stale runtime pattern
- ✅ `grep 'GenMetaCodeV0' README.md` shows package-level call matching actual signature
- ✅ `grep 'GenSumCodeV0' docs/howto/go.md` shows 4-argument call matching actual signature
- ✅ `uv run zensical build` exits 0 — docs site builds cleanly
- ✅ `mise run check` passes — all 15 pre-commit hooks green

**Next:** All documentation bugs identified in this iteration are now fixed. The project may be
approaching maintenance mode — remaining issues in issues.md are all `low` priority. Next step could
be an update-state to reflect the docs fixes, or moving to any newly filed issues.

**Notes:** Straightforward documentation fix with no surprises. Both changes were purely cosmetic
(code examples in markdown) with zero impact on actual source code or tests.
