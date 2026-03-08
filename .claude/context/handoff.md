## 2026-03-08 — Review of: Fix "View as Markdown" 404 on docs site

**Verdict:** PASS

**Summary:** The advance agent correctly updated `scripts/gen_llms_full.py` to include all 20
documentation pages (up from 14) by adding the 6 missing entries to `ORDERED_PAGES` and implementing
auto-discovery via `rglob("*.md")`. The fix is well-scoped (single file), solves the root cause, and
prevents future drift. One minor cross-platform fix applied (reviewer step 9).

**Verification:**

- [x] `uv run zensical build && uv run python scripts/gen_llms_full.py` exits 0 — "20 pages, 236169
    bytes"
- [x] `test -f site/howto/ruby.md` exits 0
- [x] `test -f site/howto/dotnet.md` exits 0
- [x] `test -f site/howto/c-cpp.md` exits 0
- [x] `test -f site/c-ffi-api.md` exits 0
- [x] `test -f site/java-api.md` exits 0
- [x] `test -f site/ruby-api.md` exits 0
- [x] `wc -l site/llms-full.txt` shows 7833 lines (increased from prior ~5000)
- [x] `mise run check` — all 15 hooks passed

**Issues found:**

- Minor: `str(rel)` in `discover_pages()` produces backslash paths on Windows, causing set
    comparison with `ORDERED_PAGES` (forward slashes) to fail — all pages would appear twice in
    `all_pages`. Fixed in review step 9 by using `rel.as_posix()` instead. No behavior change on
    Linux/macOS.

**Codex review:** Codex identified the same Windows path separator issue (P3). Already fixed by the
reviewer's minor fix (`str(rel)` → `rel.as_posix()`). No other findings.

**Next:** One `normal` priority issue remains: Conan recipe fix (`conanfile.py` declares
`shared-library` but `package()` never copies the native `iscc_ffi` binary). After that, only `low`
priority issues remain and the CID loop should signal idle.

**Notes:**

- The "View as Markdown" issue has been deleted from issues.md (resolved by this iteration)
- The auto-discovery approach (`discover_pages()`) prints a message when extra pages are found on
    disk but not in `ORDERED_PAGES`, so developers know to add them for proper ordering
- The `ORDERED_PAGES` list now matches `zensical.toml` nav exactly (20 entries)
