# Next Work Package

## Step: Rewrite README as public-facing polyglot introduction

## Goal

Replace the current development-workflow README with a public-facing polyglot developer README that
follows the structural pattern of the iscc-core README. The current dev workflow content already has
a home in `docs/development.md`, so the README can be fully replaced.

## Scope

- **Modify**: `README.md` — complete rewrite (replace all content)
- **Reference**: `reference/iscc-core/README.md` (structural model),
    `.claude/context/target.md#readme` (spec requirements), `docs/development.md` (confirms dev
    content lives there now)

## Not In Scope

- Modifying any Rust, Python, Node.js, WASM, or C code — this is a docs-only change
- Creating or updating any `docs/` pages — README is the only file touched
- Adding a CNAME, abbreviations file, or tutorials — those are separate work items
- Adding CI badge URLs for crates.io/PyPI/npm version badges — packages are not yet published; use
    the GitHub Actions CI badge only, plus placeholder comments for future package badges
- Generating or embedding actual architecture images — link to the iscc-core hosted image as the
    iscc-core README does

## Implementation Notes

The README must follow the target spec structure (modeled after iscc-core README). Sections in
order:

1. **Title + Badges**: `# iscc-lib` with CI status badge
    (`[![CI](https://github.com/iscc/iscc-lib/actions/workflows/ci.yml/badge.svg)](...)`)

2. **Tagline**: One-line bold text emphasizing polyglot, high-performance, ISO 24138

3. **Key Features**: Bullet list — similarity-preserving, multi-level identification,
    self-describing, ISO standardized, polyglot (Rust + Python + Node.js + WASM + C FFI),
    conformance-tested against iscc-core reference

4. **What is the ISCC**: Reuse text from iscc-core README (the two paragraphs under "What is the
    ISCC") — adapt minimally to avoid copy-paste issues but preserve the meaning

5. **What is iscc-lib**: Explain this is a high-performance polyglot Rust implementation, its
    relationship to `iscc-core` reference, and which ecosystems it serves. Mention the `iscc-core`
    tip about higher-level `iscc-sdk` — adapt it to note that `iscc-lib` is likewise a low-level
    codec/algorithm library

6. **ISCC Architecture**: Embed the same architecture diagram as iscc-core:
    `![ISCC Architecture](https://raw.githubusercontent.com/iscc/iscc-core/master/docs/images/iscc-codec-light.png)`

7. **ISCC MainTypes**: Copy the exact table from iscc-core README (6 rows: META through ISCC)

8. **Installation**: Per-language install instructions using H3 subsections:

    - Rust: `cargo add iscc-lib`
    - Python: `pip install iscc-lib`
    - Node.js: `npm install @iscc/lib`
    - WASM: `npm install @iscc/wasm` Note: packages are not yet published — add a note like "Coming
        soon" or phrase as future tense

9. **Quick Start**: Minimal `gen_meta_code_v0` example in each language using fenced code blocks
    with language tags. Keep examples short (3-5 lines each). Use the v0 function names (not
    convenience wrappers). For Rust show `iscc_lib::gen_meta_code_v0(...)`, for Python show
    `import iscc_lib as ic; ic.gen_meta_code_v0(...)`, for Node.js show
    `const ic = require('@iscc/lib'); ic.gen_meta_code_v0(...)`, for WASM show the import pattern

10. **Implementors Guide**: List the 9 `gen_*_v0` entry points (same list as iscc-core). Link to
    conformance test vectors. Link to docs site for detailed per-language guides

11. **Documentation**: Link to `https://lib.iscc.codes`

12. **Contributing**: Pull requests welcome, open issue first for significant changes, link to
    Telegram dev chat `https://t.me/iscc_dev`

13. **License**: Apache-2.0 (replace current "TBD")

14. **Maintainers**: `[@titusz](https://github.com/titusz)`

Important style notes:

- Do NOT include any dev container, CID loop, quality gates, pre-commit hooks, or project structure
    content — that lives in `docs/development.md` now
- Use standard markdown (no MkDocs-specific directives like `!!! tip` — this renders on GitHub)
- Keep the tone professional and concise, matching iscc-core's style

## Verification

- `test -f README.md && wc -l README.md | awk '{print ($1 >= 80)}'` returns 1 (file exists with
    substantive content)
- `grep -c "gen_meta_code_v0\|gen_text_code_v0\|gen_image_code_v0\|gen_audio_code_v0\|gen_video_code_v0\|gen_mixed_code_v0\|gen_data_code_v0\|gen_instance_code_v0\|gen_iscc_code_v0" README.md`
    returns at least 9 (all entry points listed)
- `grep -c "lib.iscc.codes" README.md` returns at least 1 (documentation link present)
- `grep -c "cargo add\|pip install\|npm install" README.md` returns at least 3 (multi-language
    install instructions)
- `grep -c "Apache-2.0" README.md` returns at least 1 (license declared)
- `grep -c "@titusz" README.md` returns at least 1 (maintainer credited)
- `grep -qv "Dev Container\|CID Loop\|pre-commit\|Quality Gates\|mise run" README.md` — README does
    NOT contain dev workflow content (verify manually: `grep -c` returns 0 for these terms)
- `grep -c "ISCC Architecture" README.md` returns at least 1 (architecture section present)
- `grep -c "MainType" README.md` returns at least 1 (MainTypes table present)
- `mise run check` passes (pre-commit hooks still green — mdformat may reformat the new content)

## Done When

All verification criteria pass: README.md is a public-facing polyglot developer introduction with
badges, key features, ISCC explanation, architecture diagram, MainTypes table, per-language
installation instructions, per-language quick start examples, implementors guide listing all 9
`gen_*_v0` entry points, documentation link, contributing section, Apache-2.0 license, and
maintainer credit — with zero development workflow content.
