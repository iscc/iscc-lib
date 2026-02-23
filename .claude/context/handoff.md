## 2026-02-23 — Add copy-page split-button and llms-full.txt generation

**Done:** Added the copy-page split-button dropdown (copies page as Markdown, view as Markdown, edit
on GitHub) and the `llms-full.txt` generation pipeline to the documentation site. Ported
`copypage.js` and copy-page CSS rules verbatim from iscc-usearch (with `REPO_URL` updated to
`iscc/iscc-lib`). Created a simplified `gen_llms_full.py` (no griffe dependency) that reads the 5
nav pages, cleans them, writes individual `.md` files to `site/`, and concatenates into
`site/llms-full.txt`. Added `docs/llms.txt` metadata file and wired the generation step into the
docs CI workflow.

**Files changed:**

- `docs/javascripts/copypage.js`: Created — split-button dropdown UI ported from iscc-usearch with
    REPO_URL updated
- `docs/stylesheets/extra.css`: Appended copy-page split-button CSS rules (heading wrapper, split
    button, dropdown menu, dark mode shadow, responsive hide)
- `scripts/gen_llms_full.py`: Created — generates per-page `.md` files and concatenated
    `llms-full.txt` in `site/`
- `docs/llms.txt`: Created — static metadata file with site description and links to all doc pages
- `zensical.toml`: Added `extra_javascript = ["javascripts/copypage.js"]`
- `.github/workflows/docs.yml`: Added "Generate LLM-friendly docs" step after zensical build

**Verification:** All 14 verification criteria pass:

- `uv run zensical build && uv run python scripts/gen_llms_full.py` exits 0 (5 pages, 29573 bytes)
- `test -f docs/javascripts/copypage.js` — PASS
- `test -f docs/llms.txt` — PASS
- `test -f scripts/gen_llms_full.py` — PASS
- `test -f site/llms-full.txt` — PASS
- `test -f site/index.md` — PASS
- `test -f site/architecture.md` — PASS
- `grep -q 'copy-page' docs/stylesheets/extra.css` — PASS
- `grep -q 'copy-page-heading' docs/stylesheets/extra.css` — PASS
- `grep -q 'copypage.js' zensical.toml` — PASS
- `grep -q 'iscc/iscc-lib' docs/javascripts/copypage.js` — PASS
- `grep -q 'gen_llms_full' .github/workflows/docs.yml` — PASS
- `grep -q 'llms-full.txt' docs/llms.txt` — PASS
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `uv run prek run --all-files` — all 14 hooks pass

**Next:** Continue with remaining documentation enhancements. Candidates: Diataxis navigation
restructuring (reorganize site nav into tutorials/howto/explanation/reference sections), social
sharing meta tags (`overrides/main.html`), or abbreviations file (`docs/includes/abbreviations.md`).
Alternatively, shift to OIDC publishing pipeline configuration since all binding targets are at full
Tier 1 parity and CI is green.

**Notes:** The `copypage.js` was ported verbatim from iscc-usearch with only the `REPO_URL` constant
changed. The comment "Item 2" (duplicated in the reference) was fixed to "Item 3" for the Edit on
GitHub entry. The `gen_llms_full.py` script is substantially simpler than the iscc-usearch version —
no `API_PAGE` dynamic generation, no griffe dependency, no `_API_CLASSES` list, no
`generate_api_reference()` function. All pages are static `.md` files read directly from `docs/`. No
Rust code was modified.
