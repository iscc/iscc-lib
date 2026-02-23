## 2026-02-23 — Add Open Graph and Twitter Card social meta tags

**Done:** Created `overrides/main.html` Jinja2 template override that injects Open Graph and Twitter
Card meta tags into every documentation page via the `extrahead` block. Added
`custom_dir = "overrides"` to `zensical.toml` to enable template overrides. Tags use page-specific
titles and canonical URLs with fallbacks to site-level defaults, and reference the ISCC logo as the
social preview image.

**Files changed:**

- `overrides/main.html`: Created Jinja2 template extending `base.html`, overriding `extrahead` block
    with 6 OG tags (`og:type`, `og:title`, `og:description`, `og:url`, `og:image`, `og:site_name`)
    and 4 Twitter Card tags (`twitter:card`, `twitter:title`, `twitter:description`,
    `twitter:image`)
- `zensical.toml`: Added `custom_dir = "overrides"` to `[project.theme]` section

**Verification:**

- [x] `uv run zensical build` exits 0 (site builds with the override)
- [x] `test -f overrides/main.html` exits 0 (override file exists)
- [x] `grep -q 'custom_dir' zensical.toml` exits 0 (custom_dir configured)
- [x] `grep -q 'overrides' zensical.toml` exits 0 (points to overrides directory)
- [x] `grep -q 'og:title' overrides/main.html` exits 0 (Open Graph title tag present)
- [x] `grep -q 'og:description' overrides/main.html` exits 0 (Open Graph description tag present)
- [x] `grep -q 'og:image' overrides/main.html` exits 0 (Open Graph image tag present)
- [x] `grep -q 'og:url' overrides/main.html` exits 0 (Open Graph URL tag present)
- [x] `grep -q 'twitter:card' overrides/main.html` exits 0 (Twitter Card tag present)
- [x] `grep -q 'twitter:title' overrides/main.html` exits 0 (Twitter Card title present)
- [x] `grep -q 'og:title' site/index.html` exits 0 (OG tags rendered in built HTML)
- [x] `grep -q 'og:title' site/architecture/index.html` exits 0 (OG tags on non-index page)
- [x] `grep -q 'twitter:card' site/index.html` exits 0 (Twitter Card rendered in built HTML)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean (no Rust changes)
- [x] `uv run prek run --all-files` — all 14 pre-commit hooks pass

**Next:** Continue documentation enhancements. Candidates: Diataxis navigation restructuring
(reorganize site nav into tutorials/howto/explanation/reference sections), abbreviations file
(`docs/includes/abbreviations.md`), per-language how-to guides, or shift to OIDC trusted publishing
pipeline configuration.

**Notes:** The next.md template used `{% extends "main.html" %}` which causes a template inheritance
cycle because the override file *is* `main.html` (zensical resolves `main.html` to the override
first). Fixed by extending `base.html` instead, which is the actual parent template (`main.html` in
zensical is just `{% extends "base.html" %}` with no additional content). Also, `page.meta.title`
caused an "undefined value" error because `page.meta` can be `None` on pages without YAML front
matter — switched from `or`-chain expressions to explicit `if/elif/else` blocks matching the pattern
used in zensical's own `base.html`. No Rust code was touched — this is a pure documentation change.
