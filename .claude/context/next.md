# Next Work Package

## Step: Include all doc pages in llms-full.txt

## Goal

Update the LLM-friendly docs generation to include all 14 documentation pages instead of only the
current 5. The spec requires `site/llms-full.txt` to contain "content from all pages" and
`docs/llms.txt` to list all documentation pages — both currently cover only 5 of 14 pages, omitting
all howto guides, tutorials, ecosystem, and development pages.

## Scope

- **Create**: (none)
- **Modify**: `scripts/gen_llms_full.py`, `docs/llms.txt`
- **Reference**: `docs/` directory listing (14 `.md` files),
    `.claude/context/specs/documentation.md` (verification criteria), `zensical.toml` (navigation
    order)

## Not In Scope

- Rewriting doc page content or adding tabbed multi-language code examples
- Changing the `gen_llms_full.py` script logic (stripping frontmatter, cleaning, concatenation)
- Modifying the docs workflow (`docs.yml`) — it already calls `gen_llms_full.py`
- Building the site locally or deploying — just fix the page lists

## Implementation Notes

**`scripts/gen_llms_full.py`** — Update the `PAGES` list to include all 14 doc pages. Use the same
order as `zensical.toml` navigation:

```python
PAGES = [
    "index.md",
    "tutorials/getting-started.md",
    "howto/rust.md",
    "howto/python.md",
    "howto/nodejs.md",
    "howto/wasm.md",
    "howto/go.md",
    "howto/java.md",
    "architecture.md",
    "ecosystem.md",
    "rust-api.md",
    "api.md",
    "benchmarks.md",
    "development.md",
]
```

**`docs/llms.txt`** — Add the missing 9 page references under `## Reference`. Keep the existing
format (`- [Title](URL): Description`). Match titles and descriptions to actual page content. Add
all howto guides, the tutorial, ecosystem, and development pages. Also add Java and Go to the
install line in the intro since they're now available.

## Verification

- `uv run python scripts/gen_llms_full.py` exits 0 and prints "14 pages"
- `grep -c '---' site/llms-full.txt` returns 13 (14 pages separated by 13 dividers)
- `grep 'howto/go.md' docs/llms.txt` exits 0 (Go howto is referenced)
- `grep 'tutorials/getting-started.md' docs/llms.txt` exits 0 (tutorial is referenced)
- `grep 'development.md' docs/llms.txt` exits 0 (development page is referenced)
- `mise run format` produces no formatting changes (pre-format before commit)

## Done When

All 14 documentation pages are included in both `scripts/gen_llms_full.py` PAGES list and
`docs/llms.txt` reference section, and all verification checks pass.
