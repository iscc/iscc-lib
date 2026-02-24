# Spec: Documentation Site

Detailed requirements for the documentation site at `lib.iscc.codes`. The reference is the
`iscc/iscc-usearch` documentation at `usearch.iscc.codes` — use deepwiki MCP to query
`iscc/iscc-usearch` for exact file contents.

## Branding

The site has the same ISCC branding as iscc-usearch:

- Custom CSS in `docs/stylesheets/extra.css` with ISCC brand colors (light: `#0054b2`, `#4596f5`,
    `#123663`; dark: `#4596f5`, `#0054b2`, `#123663`)
- Header, footer, links, and search button all use brand colors
- Logo and favicon from ISCC assets in `docs/assets/`
- Logo inverts correctly in dark mode
- `zensical.toml` references `extra_css`, logo, and favicon

## Social Sharing

HTML output includes Open Graph and Twitter Card meta tags for social sharing, implemented via a
custom template override in `overrides/main.html` (same pattern as iscc-usearch).

## Copy-Page and Agent-Friendly Docs

The site has the same copy-page feature as iscc-usearch:

- A split-button dropdown on each page with "Copy page" (copies markdown to clipboard), "View as
    Markdown" (opens raw `.md` file), and "Edit on GitHub" (opens GitHub editor)
- `docs/javascripts/copypage.js` implements the UI
- A build-time script `scripts/gen_llms_full.py` generates individual `.md` files in `site/` and a
    concatenated `site/llms-full.txt` for LLM consumption
- `docs/llms.txt` exists with site metadata and links to all documentation pages
- The docs workflow runs `gen_llms_full.py` after `zensical build`

## Abbreviations

`docs/includes/abbreviations.md` defines ISCC-specific abbreviations (ISCC, CDC, DCT, MinHash,
SimHash, WTA-Hash, BLAKE3, FFI, WASM, PyPI, etc.). The pymdownx.snippets extension auto-appends this
file to all pages. Abbreviations expand on hover throughout the site.

## Navigation and Structure

Content follows the Diátaxis framework with clear categories in navigation:

- **Landing page** (`index.md`): overview, quick start, supported code types
- **Tutorials**: getting started guide (installation, first ISCC code generation)
- **How-to guides**: per-language usage guides (Python, Rust, Node.js, WASM)
- **Explanation**: architecture (hub-and-spoke model, internal modules, streaming pattern)
- **Reference**: Python API (mkdocstrings autodoc), Rust API, C FFI reference
- **Benchmarks**: performance comparisons
- **Development**: contributor-facing content (dev container setup, CID autonomous workflow, quality
    gates and pre-commit hooks, project structure, mise task runner)

## Per-Language Code Examples

All code examples use tabbed format (`pymdownx.tabbed`) showing equivalent code in multiple
languages. Standard tab order: Python, Rust, Node.js, WASM. Only languages relevant to the example
are included. The landing page quick start and all how-to guides use this pattern.

## Custom Domain

`docs/CNAME` contains `lib.iscc.codes`.

## Verification Criteria

- [ ] `uv run zensical build` succeeds
- [ ] Built site has ISCC-branded header, footer, and links (matching iscc-usearch colors)
- [ ] Logo and favicon are visible
- [ ] Dark mode toggle works and brand colors adapt
- [ ] Copy-page split-button is visible on all pages
- [ ] `site/llms-full.txt` exists and contains content from all pages
- [ ] `docs/llms.txt` exists with site metadata
- [ ] Open Graph meta tags are present in built HTML
- [ ] All code examples use tabbed multi-language format
- [ ] Navigation has Diátaxis categories (tutorials, howto, explanation, reference)
- [ ] Abbreviations expand on hover
- [ ] `docs/CNAME` contains `lib.iscc.codes`
- [ ] Docs workflow generates `llms-full.txt` during build
