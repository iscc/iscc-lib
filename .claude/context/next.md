# Next Work Package

## Step: Add abbreviations file, CNAME, and snippets config

## Goal

Complete three small documentation infrastructure gaps — abbreviations with hover expansion, custom
domain CNAME, and the snippets extension config — bringing the docs site closer to full spec
compliance.

## Scope

- **Create**: `docs/CNAME`, `docs/includes/abbreviations.md`
- **Modify**: `zensical.toml`
- **Reference**: `.claude/context/specs/documentation.md`, iscc/iscc-usearch `zensical.toml` (via
    deepwiki MCP — already queried: config is
    `[project.markdown_extensions.pymdownx.snippets] auto_append = ["docs/includes/abbreviations.md"]`)

## Not In Scope

- Nav icons on top-level sections (separate step — requires modifying 9+ doc page front matter
    files)
- Tutorials section or Rust how-to guide (content creation steps)
- Modifying `scripts/gen_llms_full.py` — it already has `strip_snippets` to handle abbreviation
    lines
- Adding abbreviations for terms not yet used in the documentation

## Implementation Notes

1. **`docs/CNAME`** — single line: `lib.iscc.codes` (no trailing newline). This enables GitHub Pages
    custom domain serving.

2. **`docs/includes/abbreviations.md`** — use the `*[TERM]: Definition` syntax (one per line). The
    spec lists these terms: ISCC, CDC, DCT, MinHash, SimHash, WTA-Hash, BLAKE3, FFI, WASM, PyPI.
    Add additional relevant terms that appear in the docs: ISO, NAPI, PyO3, API, JCS, OIDC, CI/CD,
    ABI, CDN. Keep definitions concise (one short phrase each). Reference format from iscc-usearch:

    ```markdown
    *[ISCC]: International Standard Content Code
    *[CDC]: Content-Defined Chunking
    ```

3. **`zensical.toml`** — add the snippets extension under `[project.markdown_extensions.pymdownx]`.
    The exact config (matching iscc-usearch):

    ```toml
    [project.markdown_extensions.pymdownx.snippets]
    auto_append = ["docs/includes/abbreviations.md"]
    ```

    The `abbr` extension is already enabled (line 78 in current `zensical.toml`), so abbreviation
    rendering works once snippets auto-appends the definitions.

4. After changes, run `uv run zensical build` to verify the site builds correctly with the new
    snippets config. If the build fails due to the snippets path, check whether zensical expects a
    path relative to the docs directory or the project root — iscc-usearch uses
    `"docs/includes/abbreviations.md"` (project-root-relative).

## Verification

- `cat docs/CNAME` outputs exactly `lib.iscc.codes`
- `test -f docs/includes/abbreviations.md && echo ok` exits 0
- `grep -c '^\*\[' docs/includes/abbreviations.md` returns >= 10 (at least 10 abbreviation entries)
- `grep 'pymdownx.snippets' zensical.toml` finds the snippets config
- `grep 'auto_append' zensical.toml` finds the auto_append directive
- `uv run zensical build` succeeds (exit 0)
- `mise run check` passes (all pre-commit/pre-push hooks green)

## Done When

All verification commands pass — the abbreviations file exists with >= 10 terms, CNAME is in place,
snippets extension is configured, and the docs site builds successfully.
