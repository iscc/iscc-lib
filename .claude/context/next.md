# Next Work Package

## Step: Restructure documentation navigation into Diataxis categories

## Goal

Reorganize the flat site navigation into Diataxis framework sections (How-to Guides, Explanation,
Reference) and create per-language how-to guide pages for Python, Node.js, and WASM. This is the
largest remaining documentation gap per state.md and is an explicit verification criterion in the
documentation spec ("Navigation follows Diataxis framework").

## Scope

- **Create**:
    - `docs/howto/python.md` -- Python usage guide (installation, code generation, streaming,
        structured results)
    - `docs/howto/nodejs.md` -- Node.js usage guide (installation, code generation, streaming)
    - `docs/howto/wasm.md` -- WASM usage guide (installation, browser/Node.js setup, code generation)
- **Modify**:
    - `zensical.toml` -- restructure `nav` from flat list into Diataxis sections
- **Reference**:
    - `/workspace/iscc-lib/.claude/context/specs/documentation.md` (Diataxis nav requirements)
    - `/workspace/iscc-lib/docs/index.md` (existing landing page)
    - `/workspace/iscc-lib/docs/architecture.md` (existing explanation page)
    - `/workspace/iscc-lib/docs/rust-api.md` (existing Rust reference)
    - `/workspace/iscc-lib/docs/api.md` (existing Python API reference)
    - `/workspace/iscc-lib/docs/benchmarks.md` (existing benchmarks page)
    - `/workspace/iscc-lib/zensical.toml` (current config)
    - `/workspace/iscc-lib/crates/iscc-py/python/iscc_lib/__init__.py` (Python API surface)
    - `/workspace/iscc-lib/crates/iscc-napi/package.json` (npm package name)
    - `/workspace/iscc-lib/crates/iscc-wasm/src/lib.rs` (WASM API surface)

## Not In Scope

- Creating a "Tutorials / Getting Started" page -- the existing `index.md` already serves as a
    landing page with a quick start section; a dedicated tutorial can be added in a future step
- Moving existing markdown files into subdirectories (e.g., `docs/explanation/architecture.md`) --
    MkDocs Material supports section labels in nav without requiring files to match directory
    structure; keep existing files in place to avoid breaking links and llms.txt references
- Abbreviations file (`docs/includes/abbreviations.md`) -- separate step
- CNAME file (`docs/CNAME`) -- separate step
- OIDC publishing configuration -- separate step
- Changes to Rust source code or binding crates
- Updating `docs/llms.txt` or `scripts/gen_llms_full.py` for new pages -- the generation script
    auto-discovers all built pages, so new pages will be included automatically

## Implementation Notes

### Navigation structure in `zensical.toml`

Replace the flat `nav` list with Diataxis-organized sections. The zensical.toml `nav` uses TOML
inline table syntax (since zensical wraps MkDocs Material). The format is:

```toml
nav = [
  "index.md",
  { "How-to Guides" = [
    { "Python" = "howto/python.md" },
    { "Node.js" = "howto/nodejs.md" },
    { "WebAssembly" = "howto/wasm.md" },
  ] },
  { "Explanation" = [
    { "Architecture" = "architecture.md" },
  ] },
  { "Reference" = [
    { "Rust API" = "rust-api.md" },
    { "Python API" = "api.md" },
  ] },
  { "Benchmarks" = "benchmarks.md" },
]
```

Key decisions:

- `index.md` stays at top level (landing page, not inside a section)
- `architecture.md` maps to Explanation (it explains *how* the system works)
- `rust-api.md` and `api.md` map to Reference (technical API docs)
- `benchmarks.md` stays at top level (doesn't fit neatly into any Diataxis quadrant)
- Existing files stay in their current locations -- no file moves required
- How-to guide files go in `docs/howto/` subdirectory (new pages)

### How-to guide content (`docs/howto/python.md`)

This is the most detailed guide since Python has the richest API surface (structured results via
`IsccResult` subclasses, `BinaryIO` support for streaming). Content should cover:

1. Installation (`pip install iscc-lib`)
2. Basic code generation (all 9 `gen_*_v0` functions with examples)
3. Structured results (accessing `.iscc`, `.characters`, `.datahash` etc. as attributes)
4. Streaming with `DataHasher` and `InstanceHasher` (file-like objects)
5. Text utilities (`text_clean`, `text_collapse`, etc.)

Use tabbed code blocks only where showing cross-language comparison adds value. In per-language
how-to guides, focus on the target language exclusively.

### How-to guide content (`docs/howto/nodejs.md`)

Cover:

1. Installation (`npm install @iscc/lib`)
2. Import pattern (`import { gen_text_code_v0 } from "@iscc/lib"`)
3. Basic code generation examples
4. Streaming with `DataHasher` and `InstanceHasher` (Buffer input)

### How-to guide content (`docs/howto/wasm.md`)

Cover:

1. Installation (`npm install @iscc/iscc-wasm`)
2. Browser setup (import from bundler vs CDN)
3. Node.js setup
4. Basic code generation examples
5. Note about `Uint8Array` for binary data (WASM does not have `Buffer`)

### General guidance

- Each how-to guide should be self-contained with working code examples
- Use level 1 heading (`#`) for the page title
- Include install instructions at the top of each guide
- Show realistic examples (not just "Hello World" -- include metadata, streaming, etc.)
- Reference the existing Quick Start in `index.md` for consistency in example patterns
- The Python quick start in `index.md` shows `json.loads()` usage but the Python bindings actually
    return dict-like `IsccResult` objects directly -- the how-to guide should show the idiomatic
    pattern with attribute access

## Verification

- `uv run zensical build` exits 0 (site builds with restructured nav)
- `test -f docs/howto/python.md` exits 0 (Python how-to guide exists)
- `test -f docs/howto/nodejs.md` exits 0 (Node.js how-to guide exists)
- `test -f docs/howto/wasm.md` exits 0 (WASM how-to guide exists)
- `grep -q 'How-to Guides' zensical.toml` exits 0 (Diataxis section in nav)
- `grep -q 'Explanation' zensical.toml` exits 0 (Diataxis section in nav)
- `grep -q 'Reference' zensical.toml` exits 0 (Diataxis section in nav)
- `grep -q 'howto/python' zensical.toml` exits 0 (Python guide in nav)
- `grep -q 'howto/nodejs' zensical.toml` exits 0 (Node.js guide in nav)
- `grep -q 'howto/wasm' zensical.toml` exits 0 (WASM guide in nav)
- `grep -c '^#' docs/howto/python.md` outputs at least 1 (has heading)
- `grep -c '^#' docs/howto/nodejs.md` outputs at least 1 (has heading)
- `grep -c '^#' docs/howto/wasm.md` outputs at least 1 (has heading)
- `grep -q 'pip install' docs/howto/python.md` exits 0 (has install instructions)
- `grep -q 'npm install' docs/howto/nodejs.md` exits 0 (has install instructions)
- `grep -q 'npm install' docs/howto/wasm.md` exits 0 (has install instructions)
- `grep -q 'gen_text_code_v0' docs/howto/python.md` exits 0 (has code generation example)
- `grep -q 'DataHasher' docs/howto/python.md` exits 0 (covers streaming)
- `uv run prek run --all-files` passes (pre-commit hooks clean)
- `cargo clippy --workspace --all-targets -- -D warnings` still clean (no Rust changes)

## Done When

All verification criteria pass: the documentation site builds with Diataxis-organized navigation
showing How-to Guides (Python, Node.js, WASM), Explanation (Architecture), and Reference (Rust API,
Python API) sections, with substantive per-language how-to guide pages containing installation
instructions, code examples, and streaming usage patterns.
