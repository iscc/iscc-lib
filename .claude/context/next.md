# Next Work Package

## Step: Add ISCC brand colors, logo, and favicon to documentation site

## Goal

Add ISCC Foundation visual branding to the documentation site at lib.iscc.codes, matching the
branding used by iscc-usearch at usearch.iscc.codes. This is the foundational branding step that
establishes the visual identity before adding interactive features (copy-page, llms-full.txt) in
subsequent steps.

## Scope

- **Create**: `docs/stylesheets/extra.css` — ISCC brand color overrides for light/dark mode
- **Create**: `docs/assets/logo_light.png` — ISCC logo (download from iscc-usearch repo)
- **Create**: `docs/assets/favicon.png` — ISCC favicon (download from iscc-usearch repo)
- **Modify**: `zensical.toml` — add `extra_css`, `logo`, and `favicon` configuration
- **Reference**: iscc-usearch `docs/stylesheets/extra.css` (fetch via
    `gh api   repos/iscc/iscc-usearch/contents/docs/stylesheets/extra.css`), iscc-usearch
    `zensical.toml` (fetch via `gh api repos/iscc/iscc-usearch/contents/zensical.toml`)

## Not In Scope

- Copy-page split-button dropdown (`copypage.js`) — separate step after branding is in place
- `llms-full.txt` generation script (`gen_llms_full.py`) — separate step
- Social sharing meta tags (`overrides/main.html`) — separate step
- Abbreviations file (`docs/includes/abbreviations.md`) and `pymdownx.snippets` config — separate
    step
- `docs/llms.txt` metadata file — separate step
- Diataxis navigation restructuring — separate step (requires creating new page files)
- Any changes to documentation content or page structure
- `docs/stylesheets/copilot.css` or `docs/javascripts/copilot.js` — the ISCC AI copilot widget is
    project-specific and should not be added without explicit instruction

## Implementation Notes

### CSS: `docs/stylesheets/extra.css`

Port the ISCC brand color CSS from iscc-usearch verbatim. The file contains:

- Light mode primary/accent color overrides (`#0054b2`, `#4596f5`, `#123663`)
- Dark mode primary/accent color overrides (inverted lighter/darker values)
- Link color customization for both schemes
- Light mode header styling (ISCC Blue `#0054b2` background, white text)
- Dark mode header styling (Deep Navy `#123663` background)
- Footer styling (Deep Navy `#123663` background for light mode)
- Logo inversion via CSS `filter: invert(1)` for dark mode nav and light mode header
- Mermaid edge color softening for dark mode

**Important**: Do NOT include the copy-page split-button CSS rules (`.copy-page-heading`,
`.copy-page`, `.copy-page__split`, etc.) — those belong in a future step when `copypage.js` is
added. Only include the brand color and layout rules.

The CSS uses `[data-md-color-primary="indigo"]` and `[data-md-color-accent="indigo"]` attribute
selectors. The current `zensical.toml` palette sections do not specify `primary` or `accent` keys,
so Zensical uses its default (which is `indigo`). Do NOT add explicit `primary = "indigo"` /
`accent = "indigo"` to the palette config — the CSS selectors already match the default.

### Logo and Favicon Assets

Download the ISCC assets from the iscc-usearch repository. These are shared ISCC Foundation branding
assets:

```bash
mkdir -p docs/assets
curl -sL https://raw.githubusercontent.com/iscc/iscc-usearch/main/docs/assets/logo_light.png -o docs/assets/logo_light.png
curl -sL https://raw.githubusercontent.com/iscc/iscc-usearch/main/docs/assets/favicon.png -o docs/assets/favicon.png
```

### `zensical.toml` Changes

Add three lines to the `[project]` section:

```toml
extra_css = ["stylesheets/extra.css"]
```

Add two lines to the `[project.theme]` section:

```toml
logo = "assets/logo_light.png"
favicon = "assets/favicon.png"
```

Do NOT add `custom_dir` yet — that is needed only when the `overrides/main.html` template is added
in a future step.

Use the **`zensical-customizer`** skill for guidance on custom CSS and theme configuration if
needed.

## Verification

- `uv run zensical build` exits 0 (site builds successfully with new CSS/assets)
- `test -f docs/stylesheets/extra.css` exits 0 (CSS file exists)
- `test -f docs/assets/logo_light.png` exits 0 (logo file exists)
- `test -f docs/assets/favicon.png` exits 0 (favicon file exists)
- `grep -q 'extra_css' zensical.toml` exits 0 (extra_css configured)
- `grep -q 'logo.*=.*logo_light.png' zensical.toml` exits 0 (logo configured)
- `grep -q 'favicon.*=.*favicon.png' zensical.toml` exits 0 (favicon configured)
- `grep -q '#0054b2' docs/stylesheets/extra.css` exits 0 (ISCC Blue primary color present)
- `grep -q 'filter: invert(1)' docs/stylesheets/extra.css` exits 0 (dark mode logo inversion
    present)
- `grep -q 'copy-page' docs/stylesheets/extra.css; test $? -ne 0` exits 0 (copy-page CSS NOT
    present)
- `cargo clippy --workspace --all-targets -- -D warnings` still clean (no Rust changes, sanity
    check)

## Done When

All verification criteria pass: the documentation site builds with ISCC brand colors (blue header,
navy footer, correct link colors in both light and dark modes), the ISCC logo appears in the header
with proper dark mode inversion, and the favicon is configured.
