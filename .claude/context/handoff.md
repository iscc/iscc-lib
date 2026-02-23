## 2026-02-23 — Target updated with detailed specs (Titusz + Claude)

**Done:**

- Enabled GitHub Pages with custom domain `lib.iscc.codes`
- Created `@iscc` npm org on npmjs.com
- Added `NPM_TOKEN` repo secret for first npm release
- Bumped Node.js from 20 to 22 in release workflow for trusted publishing
- Created detailed specs in `.claude/context/specs/`:
    - `python-bindings.md` — drop-in compatibility with iscc-core
    - `documentation.md` — ISCC branding and features matching iscc-usearch

**Key findings:**

- Python bindings return plain strings, but iscc-core returns dicts with additional fields
    (`metahash`, `name`, `characters`, `datahash`, `filesize`, `parts`). The Rust core also needs
    structured return types to carry these fields.
- Documentation is stock zensical theme — missing ISCC branding, copy-page feature, llms.txt,
    Diátaxis structure, per-language tabbed examples, abbreviations, Open Graph meta tags.

**Priority:** Python API compatibility first (Rust core structured types + Python binding changes),
then documentation refinement.

**Next:** Read `.claude/context/specs/python-bindings.md` and begin implementing structured return
types in the Rust core API.
