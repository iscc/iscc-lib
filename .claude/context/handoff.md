## 2026-02-28 — Include all doc pages in llms-full.txt

**Done:** Updated `scripts/gen_llms_full.py` PAGES list from 5 to 14 entries (matching zensical.toml
nav order) and rewrote `docs/llms.txt` to reference all 14 documentation pages with accurate titles
and descriptions. Also updated the intro to mention Go and Java bindings.

**Files changed:**

- `scripts/gen_llms_full.py`: Expanded PAGES list to include all 14 doc pages (tutorials, howto
    guides, ecosystem, development) in zensical.toml navigation order
- `docs/llms.txt`: Added 9 missing page references (Getting Started, 6 howto guides, Ecosystem,
    Development), updated intro to mention Go/Java bindings and Go install command

**Verification:**

- `uv run python scripts/gen_llms_full.py` → "14 pages, 115291 bytes" ✓
- `grep 'howto/go.md' docs/llms.txt` → exits 0 ✓
- `grep 'tutorials/getting-started.md' docs/llms.txt` → exits 0 ✓
- `grep 'development.md' docs/llms.txt` → exits 0 ✓
- `mise run format` → no changes ✓
- `mise run check` → all 14 hooks pass ✓
- Note: `grep -c '---' site/llms-full.txt` returns 64 (not 13) because doc pages themselves contain
    `---` horizontal rules. The divider count verification in next.md was approximate — the script's
    "14 pages" output is the authoritative check.

**Next:** The develop branch now has complete documentation coverage (all 14 pages in LLM-friendly
output). Remaining target gaps are publishing infrastructure (OIDC, npm, Maven Central) and develop
→ main merge PR. The tabbed multi-language code examples in doc pages are still pending per the
documentation spec.

**Notes:** Straightforward docs-only change. No code logic changes, no new dependencies.
