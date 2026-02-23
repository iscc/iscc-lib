# Next Work Package

## Step: Add Open Graph and Twitter Card social meta tags

## Goal

Add social sharing meta tags (Open Graph and Twitter Card) to the documentation site via a template
override, so that links to lib.iscc.codes render rich previews on social platforms and messaging
apps. This is an explicit verification criterion in the documentation spec ("Open Graph meta tags
are present in built HTML").

## Scope

- **Create**: `overrides/main.html` — Jinja2 template override extending zensical's `main.html`,
    injecting OG and Twitter Card meta tags into the `{% block extrahead %}` block
- **Modify**: `zensical.toml` — add `custom_dir = "overrides"` under `[project.theme]`
- **Reference**:
    `/home/dev/.venvs/iscc-lib/lib/python3.12/site-packages/zensical/templates/base.html` (lines
    8-38 for `site_meta` block, line 89 for `extrahead` block), `/workspace/iscc-lib/zensical.toml`
    (current config), `/workspace/iscc-lib/.claude/context/specs/documentation.md` (social sharing
    requirement)

## Not In Scope

- Diataxis navigation restructuring — separate step requiring new content pages and nav
    reorganization
- Per-language how-to guides (Node.js, WASM) — separate step
- Abbreviations file (`docs/includes/abbreviations.md`) — separate step
- OIDC publishing configuration — separate step
- Creating a social preview image (og:image) — the meta tags should reference the existing logo
    (`assets/logo_light.png`) but do NOT create a dedicated social card image or use the
    mkdocs-material social plugin
- Any changes to Rust source code or binding crates

## Implementation Notes

### `overrides/main.html`

Create a Jinja2 template that extends `main.html` and overrides the `{% block extrahead %}` block.
The pattern is standard MkDocs Material:

```jinja2
{% extends "main.html" %}

{% block extrahead %}
  {% set title = page.meta.title or page.title or config.site_name %}
  {% set description = page.meta.description or config.site_description %}
  {% set url = page.canonical_url or config.site_url %}
  {% set image = config.site_url ~ "assets/logo_light.png" %}

  <!-- Open Graph -->
  <meta property="og:type" content="website">
  <meta property="og:title" content="{{ title }}">
  <meta property="og:description" content="{{ description }}">
  <meta property="og:url" content="{{ url }}">
  <meta property="og:image" content="{{ image }}">
  <meta property="og:site_name" content="{{ config.site_name }}">

  <!-- Twitter Card -->
  <meta name="twitter:card" content="summary">
  <meta name="twitter:title" content="{{ title }}">
  <meta name="twitter:description" content="{{ description }}">
  <meta name="twitter:image" content="{{ image }}">
{% endblock %}
```

Key details:

1. Use `page.meta.title` (from YAML front matter) with fallback to `page.title` (auto-derived from
    H1), then `config.site_name` as final fallback
2. Use `page.meta.description` with fallback to `config.site_description` (defined in zensical.toml)
3. The `og:image` should use the absolute URL to `assets/logo_light.png` (already hosted)
4. Use `twitter:card` type `summary` (not `summary_large_image`) since we have a logo, not a
    dedicated social card image
5. Ensure no trailing slash issues in URLs by using Jinja2 `~` concatenation

### `zensical.toml`

Add `custom_dir = "overrides"` inside the existing `[project.theme]` section. This tells zensical
(MkDocs Material) to look in the `overrides/` directory for template overrides. Place it right after
the `favicon` line for readability.

### Build verification

After making changes, `uv run zensical build` must succeed, and the built HTML files in `site/` must
contain the Open Graph meta tags. Check by grepping any built HTML file (e.g., `site/index.html`)
for `og:title`.

## Verification

- `uv run zensical build` exits 0 (site builds with the override)
- `test -f overrides/main.html` exits 0 (override file exists)
- `grep -q 'custom_dir' zensical.toml` exits 0 (custom_dir configured)
- `grep -q 'overrides' zensical.toml` exits 0 (points to overrides directory)
- `grep -q 'og:title' overrides/main.html` exits 0 (Open Graph title tag present)
- `grep -q 'og:description' overrides/main.html` exits 0 (Open Graph description tag present)
- `grep -q 'og:image' overrides/main.html` exits 0 (Open Graph image tag present)
- `grep -q 'og:url' overrides/main.html` exits 0 (Open Graph URL tag present)
- `grep -q 'twitter:card' overrides/main.html` exits 0 (Twitter Card tag present)
- `grep -q 'twitter:title' overrides/main.html` exits 0 (Twitter Card title present)
- `grep -q 'og:title' site/index.html` exits 0 (OG tags rendered in built HTML)
- `grep -q 'og:title' site/architecture/index.html` exits 0 (OG tags rendered on non-index page)
- `grep -q 'twitter:card' site/index.html` exits 0 (Twitter Card rendered in built HTML)
- `cargo clippy --workspace --all-targets -- -D warnings` still clean (no Rust changes)
- `uv run prek run --all-files` passes (pre-commit hooks clean)

## Done When

All verification criteria pass: the documentation site builds with Open Graph and Twitter Card meta
tags present in every built HTML page, using page-specific titles/descriptions with proper
fallbacks, and referencing the ISCC logo as the social preview image.
