# Next Work Package

## Step: Fix "View as Markdown" 404 on docs site

## Goal

Fix the broken "View as Markdown" and "Copy page" features on lib.iscc.codes by updating
`scripts/gen_llms_full.py` to generate per-page `.md` files for ALL documentation pages, not just
the original subset. This addresses the `normal` priority issue "Fix 'View as Markdown' (Copy Page)
on Zensical docs site".

## Scope

- **Create**: (none)
- **Modify**: `scripts/gen_llms_full.py` — update PAGES list to include all pages from zensical.toml
    nav, or better: auto-discover pages from `docs/` to prevent future drift
- **Reference**: `zensical.toml` (nav structure — lists all pages), `docs/javascripts/copypage.js`
    (URL construction logic), `.github/workflows/docs.yml` (build pipeline)

## Not In Scope

- Changing `copypage.js` — the URL construction logic is correct; only the missing `.md` files are
    the problem
- Changing the docs.yml workflow — `gen_llms_full.py` is already called there
- Fixing the Conan recipe issue — separate normal-priority issue for next iteration
- Adding new docs pages or changing docs content

## Implementation Notes

**Root cause**: `scripts/gen_llms_full.py` has a hardcoded `PAGES` list with only 14 entries. Six
pages added since the script was written are missing:

- `howto/ruby.md`
- `howto/dotnet.md`
- `howto/c-cpp.md`
- `c-ffi-api.md`
- `java-api.md`
- `ruby-api.md`

The script generates per-page `.md` files into `site/` during the docs build pipeline
(`.github/workflows/docs.yml`). The `copypage.js` JavaScript constructs URLs like `/howto/ruby.md` —
but since `gen_llms_full.py` never writes that file to `site/`, it 404s.

**Recommended approach**: Replace the hardcoded `PAGES` list with auto-discovery. Glob
`docs/**/*.md` (excluding `includes/` directory which contains partial snippets like
`abbreviations.md`) to find all markdown files. This ensures new pages are automatically included
without manual list maintenance.

**Alternative approach** (simpler but less robust): Just add the 6 missing pages to the PAGES list.
This fixes the immediate issue but will drift again when new pages are added.

Either approach is acceptable. Auto-discovery is preferred if it doesn't add complexity.

**Key constraint**: The `clean_content()` function strips YAML frontmatter and snippet definitions.
This must still work correctly for all discovered files. Test that the generated `.md` files are
clean (no frontmatter, no `*[...]: ...` lines).

**`llms-full.txt` ordering**: If using auto-discovery, maintain a sensible concatenation order for
`llms-full.txt`. Options: (1) parse zensical.toml nav for ordering, (2) use alphabetical order, (3)
keep a hardcoded order list for `llms-full.txt` but auto-discover for per-page `.md` files. Option
(3) is pragmatic — the per-page files just need to exist (order doesn't matter for "View as
Markdown"), while `llms-full.txt` benefits from logical ordering. Any approach that ensures all
per-page `.md` files are generated is acceptable.

## Verification

- `uv run zensical build && uv run python scripts/gen_llms_full.py` exits 0
- `test -f site/howto/ruby.md` exits 0 (the specific page from the issue report)
- `test -f site/howto/dotnet.md` exits 0
- `test -f site/howto/c-cpp.md` exits 0
- `test -f site/c-ffi-api.md` exits 0
- `test -f site/java-api.md` exits 0
- `test -f site/ruby-api.md` exits 0
- `wc -l site/llms-full.txt` shows increased line count (more pages included)
- `mise run check` passes (formatting, lint)

## Done When

All verification criteria pass — per-page `.md` files exist for every documentation page in the nav,
and `llms-full.txt` includes the full documentation corpus.
