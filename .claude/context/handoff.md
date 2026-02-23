## 2026-02-23 — Set up zensical documentation scaffold

**Done:** Created the documentation build infrastructure using zensical. Added `zensical` and
`mkdocstrings[python]` to pyproject.toml dev dependencies, created `zensical.toml` configuration
adapted from iscc-usearch (minimal: no custom assets, no overrides, no snippets auto_append), and
wrote `docs/index.md` landing page covering project description, features, code types table, quick
start examples (Rust + Python tabs), bindings table, and links.

**Files changed:**

- `pyproject.toml`: Added `zensical` and `mkdocstrings[python]` to dev dependency group
- `zensical.toml`: New file — documentation configuration with theme (light/dark palettes),
    mkdocstrings plugin pointing to `crates/iscc-py/python/iscc_lib`, and full markdown extension
    set
- `docs/index.md`: New file — landing page with project overview, feature list, code types table,
    tabbed quick start, bindings table, and external links

**Verification:**

- `uv sync --group dev` — succeeded, installed zensical 0.0.23 + 18 related packages
- `uv run zensical build` — succeeded in 0.31s, produced `site/` directory
- `site/index.html` — exists (696 lines), contains all landing page content (ISO 24138, gen
    functions, Quick Start, Available Bindings)
- `cargo test -p iscc-lib` — 143 passed, 0 failed (no regressions)
- `site/` directory already in `.gitignore`

**Next:** Follow-up documentation work: add custom ISCC branding (logo, favicon, extra.css), create
API reference page using mkdocstrings auto-generation, add GitHub Pages deployment workflow
(`.github/workflows/docs.yml`). Alternatively, tackle OIDC publishing pipelines as an independent
workstream.

**Notes:** The `pymdownx.smartsymbols` extension doesn't convert `---` to em dashes in this zensical
version — used actual Unicode em dash characters (`—`) instead. The `pymdownx.emoji` extension with
`zensical.extensions.emoji` references works correctly (zensical built-in). The `uv.lock` will need
to be committed as well since new dependencies were added.
