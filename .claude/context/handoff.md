## 2026-02-23 — Add ISCC brand colors, logo, and favicon to documentation site

**Done:** Added ISCC Foundation visual branding to the documentation site. Created
`docs/stylesheets/extra.css` with brand color overrides for light/dark mode (ISCC Blue header, Deep
Navy footer, link colors, logo inversion, mermaid edge softening). Downloaded `logo_light.png` and
`favicon.png` from the iscc-usearch repository. Updated `zensical.toml` with `extra_css`, `logo`,
and `favicon` configuration.

**Files changed:**

- `docs/stylesheets/extra.css`: ISCC brand color CSS ported from iscc-usearch, excluding copy-page
    split-button rules
- `docs/assets/logo_light.png`: ISCC Foundation logo (5000x1906 gray+alpha PNG, downloaded from
    iscc-usearch)
- `docs/assets/favicon.png`: ISCC Foundation favicon (300x300 RGBA PNG, downloaded from
    iscc-usearch)
- `zensical.toml`: Added `extra_css = ["stylesheets/extra.css"]` to `[project]` section, added
    `logo` and `favicon` to `[project.theme]` section

**Verification:** All 11 verification criteria from next.md pass:

- `uv run zensical build` exits 0 (site builds successfully)
- `test -f docs/stylesheets/extra.css` exits 0
- `test -f docs/assets/logo_light.png` exits 0
- `test -f docs/assets/favicon.png` exits 0
- `grep -q 'extra_css' zensical.toml` exits 0
- `grep -q 'logo.*=.*logo_light.png' zensical.toml` exits 0
- `grep -q 'favicon.*=.*favicon.png' zensical.toml` exits 0
- `grep -q '#0054b2' docs/stylesheets/extra.css` exits 0
- `grep -q 'filter: invert(1)' docs/stylesheets/extra.css` exits 0
- `grep -q 'copy-page' docs/stylesheets/extra.css; test $? -ne 0` exits 0 (copy-page CSS absent)
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `mise run check` all 14 hooks pass

**Next:** Consider adding the copy-page split-button dropdown (`copypage.js` + corresponding CSS
rules) as the next documentation enhancement step. Alternatively, add `llms-full.txt` generation,
social sharing meta tags, or the Diataxis navigation restructuring.

**Notes:** The CSS was ported verbatim from iscc-usearch's `docs/stylesheets/extra.css`, with the
copy-page split-button section removed as specified. No `primary = "indigo"` / `accent = "indigo"`
palette keys were added to `zensical.toml` since the CSS selectors already match the default. No
`custom_dir` was added since that is only needed when `overrides/main.html` is introduced later. No
Rust code changes were made in this step.
