## 2026-02-24 — Add abbreviations file, CNAME, and snippets config

**Done:** Created `docs/CNAME` for GitHub Pages custom domain, created
`docs/includes/abbreviations.md` with 19 ISCC-specific term definitions, and added the
`pymdownx.snippets` extension config to `zensical.toml` with `auto_append` pointing to the
abbreviations file.

**Files changed:**

- `docs/CNAME`: new file containing `lib.iscc.codes` (no trailing newline)
- `docs/includes/abbreviations.md`: new file with 19 abbreviation entries covering all terms from
    the spec (ISCC, CDC, DCT, MinHash, SimHash, WTA-Hash, BLAKE3, FFI, WASM, PyPI) plus additional
    relevant terms (ISO, NAPI, PyO3, API, JCS, OIDC, CI/CD, ABI, CDN)
- `zensical.toml`: added `[project.markdown_extensions.pymdownx.snippets]` section with
    `auto_append = ["docs/includes/abbreviations.md"]`

**Verification:**

- `cat docs/CNAME` → `lib.iscc.codes` ✓
- `test -f docs/includes/abbreviations.md` → exits 0 ✓
- `grep -c '^\*\[' docs/includes/abbreviations.md` → 19 (>= 10) ✓
- `grep 'pymdownx.snippets' zensical.toml` → found ✓
- `grep 'auto_append' zensical.toml` → found ✓
- `uv run zensical build` → succeeded (exit 0) ✓
- `mise run check` → all 14 pre-commit hooks passed ✓

**Next:** The docs site build includes `abbreviations.md` as a standalone page at
`/includes/abbreviations/` (visible in build output). This is harmless — it's not in navigation —
but could be excluded with a zensical/mkdocs `not_in_nav` config or by moving the file outside
`docs/`. Consider this a low-priority cleanup. The remaining documentation spec gaps are nav icons
on top-level sections and the tutorials section. Alternatively, the tracked issues in issues.md
(`alg_simhash` panics, `sliding_window` panics) are normal-priority correctness items.

**Notes:** The `pymdownx.snippets` path `docs/includes/abbreviations.md` is project-root-relative,
matching the iscc-usearch reference pattern. The `abbr` extension was already enabled at line 78 of
`zensical.toml`, so abbreviation hover expansion works automatically once snippets auto-appends the
definitions to every page.
