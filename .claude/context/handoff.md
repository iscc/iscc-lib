## 2026-03-08 — Fix "View as Markdown" 404 on docs site

**Done:** Updated `scripts/gen_llms_full.py` to include all 20 documentation pages (up from 14).
Added auto-discovery via `rglob("*.md")` that catches any future pages not in the ordered list,
excluding the `includes/` directory (snippet partials). The 6 missing pages (`howto/ruby.md`,
`howto/dotnet.md`, `howto/c-cpp.md`, `c-ffi-api.md`, `java-api.md`, `ruby-api.md`) now generate
per-page `.md` files in `site/`, fixing the "View as Markdown" and "Copy page" 404 errors.

**Files changed:**

- `scripts/gen_llms_full.py`: Renamed `PAGES` to `ORDERED_PAGES` with all 20 nav entries. Added
    `EXCLUDE_DIRS`, `discover_pages()` for auto-discovery, and merge logic that appends
    auto-discovered extras (sorted) after the ordered list. Both `llms-full.txt` and per-page `.md`
    files now cover all pages.

**Verification:**

- `uv run zensical build && uv run python scripts/gen_llms_full.py` — exits 0, "20 pages, 236169
    bytes"
- All 6 previously missing files verified present: `site/howto/ruby.md`, `site/howto/dotnet.md`,
    `site/howto/c-cpp.md`, `site/c-ffi-api.md`, `site/java-api.md`, `site/ruby-api.md`
- Generated files are clean (no YAML frontmatter, no snippet definitions)
- `wc -l site/llms-full.txt` = 7833 lines (increased from previous 14-page subset)
- `mise run check` — all 15 hooks passed

**Next:** Two `normal` priority issues remain: Conan recipe fix (declares shared-library but never
packages the native binary). After that, only `low` priority issues remain and the CID loop should
signal idle.

**Notes:** The auto-discovery approach ensures this won't drift again when new docs pages are added.
The `ORDERED_PAGES` list still controls `llms-full.txt` concatenation order. Any page found on disk
but not in `ORDERED_PAGES` gets appended (sorted) — and a message is printed so the developer knows
to add it to the ordered list for proper positioning.
