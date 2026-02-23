# Next Work Package

## Step: Set up zensical documentation scaffold

## Goal

Create the documentation build infrastructure using zensical (the tool specified in
notes/05-developer-tooling.md and used by iscc-usearch). This establishes the foundation for the
documentation site at lib.iscc.codes — the largest remaining gap before the project is complete.

## Scope

- **Create**: `zensical.toml`, `docs/index.md`
- **Modify**: `pyproject.toml` (add zensical + mkdocstrings dependencies to dev group)
- **Reference**: `notes/05-developer-tooling.md` (documentation section), `iscc/iscc-usearch`
    `zensical.toml` (via deepwiki MCP), `pyproject.toml` (current dev deps)

## Implementation Notes

### 1. Add zensical to pyproject.toml dev dependencies

Add these packages to `[dependency-groups] dev`:

- `zensical` — the static site generator (Material for MkDocs successor)
- `mkdocstrings[python]` — for auto-generating API reference from Python docstrings

### 2. Create `zensical.toml`

Adapt the iscc-usearch configuration for iscc-lib. Key differences from iscc-usearch:

- `site_name = "iscc-lib"`
- `site_description` = description about high-performance polyglot ISCC implementation
- `site_url = "https://lib.iscc.codes/"`
- `repo_url = "https://github.com/iscc/iscc-lib"`
- `repo_name = "iscc/iscc-lib"`
- `copyright = "Copyright &copy; 2025-2026 ISCC Foundation"`

**Keep it minimal for this first step** — do NOT reference `extra_css`, `custom_dir` (overrides),
`extra_javascript`, logo, or favicon yet. Those will be added in a follow-up step. The default
Material theme works without custom assets.

**Navigation** — start with a minimal nav. Only `index.md` for now:

```toml
nav = ["index.md"]
```

**Theme** — include basic features (code copy, navigation, search) but NO `custom_dir`, NO
`logo`/`favicon` (use defaults). Include both light/dark palette toggles.

**Plugins** — enable `mkdocstrings` with Python handler pointing to `crates/iscc-py/python/iscc_lib`
(where the Python stubs live).

**Markdown extensions** — copy the full set from iscc-usearch (admonition, pymdownx.\*, toc with
permalink, etc.). These are standard and don't require extra files.

**DO NOT** include the `pymdownx.snippets` `auto_append` for `abbreviations.md` since that file
doesn't exist yet.

**DO NOT** include the `pymdownx.emoji` extension with `zensical.extensions.emoji` references — only
include it if zensical supports it out of the box (it should, since it's a zensical built-in).

### 3. Create `docs/index.md`

Write a landing page for iscc-lib. Content should cover:

- Project title and one-line description
- What iscc-lib is (high-performance Rust implementation of ISO 24138:2024)
- Key features: 9 `gen_*_v0` functions, conformance with iscc-core, multi-language bindings
- Available bindings: Rust (crates.io), Python (PyPI), Node.js (npm), WASM (npm), C FFI
- Quick install examples for Rust (`cargo add iscc-lib`) and Python (`pip install iscc-lib`)
- Link to ISO 24138 and ISCC Foundation

Use Material for MkDocs admonitions, tabs, and code blocks to make it visually appealing.

### 4. Run `uv sync --group dev` after modifying pyproject.toml

This installs zensical. Then verify with `uv run zensical build`.

## Verification

- `uv sync --group dev` succeeds (zensical installs without errors)
- `uv run zensical build` succeeds and produces a `site/` directory
- `site/index.html` exists and contains the iscc-lib landing page content
- No pre-existing tests are broken (`cargo test -p iscc-lib` still passes)

## Done When

The advance agent is done when `uv run zensical build` produces a working documentation site in
`site/` with the landing page content, and all existing tests still pass.
