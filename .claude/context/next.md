# Next Work Package

## Step: Add tutorials getting-started guide

## Goal

Create a getting-started tutorial that walks users through installing iscc-lib and generating their
first ISCC code. This closes the "Tutorials" Diátaxis category gap — currently the only missing
top-level nav category — and satisfies the documentation spec verification criterion "Navigation has
Diátaxis categories (tutorials, howto, explanation, reference)."

## Scope

- **Create**: `docs/tutorials/getting-started.md`
- **Modify**: `zensical.toml` (add Tutorials nav section)
- **Reference**: `docs/howto/python.md`, `docs/howto/nodejs.md`, `docs/howto/wasm.md` (existing
    style/patterns), `docs/index.md` (existing quick start to differentiate from),
    `.claude/context/specs/documentation.md` (requirements)

## Not In Scope

- Adding nav icons to pages (separate step — requires modifying all doc pages)
- Creating the Rust how-to guide (`docs/howto/rust.md`) — separate step
- Adding a second tutorial (e.g., "streaming large files") — one tutorial suffices for the spec
- Modifying existing how-to guides or other documentation pages
- Adding new pymdownx extensions or site configuration beyond the nav entry

## Implementation Notes

**Structure** — Follow the iscc-usearch getting-started pattern: YAML front matter with
`icon: lucide/rocket` and a `description:` field, then progressive sections that build on each
other. The tutorial is a hands-on walkthrough, not a reference guide.

**Content sections** (in order):

1. **Getting started** — brief intro: what you'll learn (install, generate your first ISCC code,
    understand the result)
2. **Prerequisites** — Python 3.10+, pip or uv. Mention Rust/Node.js/WASM alternatives exist (link
    to how-to guides) but the tutorial uses Python for simplicity
3. **Install** — `pip install iscc-lib` (or `uv add iscc-lib`). Include a "Verify installation"
    snippet: `python -c "from iscc_lib import conformance_selftest; print(conformance_selftest())"`
4. **Generate your first ISCC code** — use `gen_meta_code_v0` with a simple name/description.
    Explain the result fields (`.iscc`, `.name`, `.metahash`). This matches the quick start in
    README but goes deeper
5. **Understand the ISCC structure** — use `iscc_decompose` on the generated code to show the
    component breakdown. Explain what MainType, SubType, Version, and length mean
6. **Try other code types** — brief examples of `gen_text_code_v0` (from text) and
    `gen_instance_code_v0` (from bytes, using `DataHasher` for streaming). Show that different
    inputs produce different ISCC types
7. **Next steps** — link to how-to guides (Python, Node.js, WASM), architecture explanation, and API
    reference

**Style** — use Python code blocks (not tabbed multi-language format; this is a tutorial for
beginners, multi-language tabs belong in how-to guides). Keep code snippets short and runnable.
Include expected output as comments in the code.

**Nav entry in zensical.toml** — add a Tutorials section BEFORE "How-to Guides" in the nav array
(matching Diátaxis ordering: tutorials → how-to → explanation → reference):

```toml
{ "Tutorials" = [
    { "Getting Started" = "tutorials/getting-started.md" },
] },
```

## Verification

- `test -f docs/tutorials/getting-started.md` exits 0
- `head -5 docs/tutorials/getting-started.md` shows YAML front matter with `icon: lucide/rocket`
- `grep -c 'gen_meta_code_v0' docs/tutorials/getting-started.md` returns >= 1
- `grep -c 'iscc_decompose' docs/tutorials/getting-started.md` returns >= 1
- `grep 'Tutorials' zensical.toml` finds the nav entry
- `grep 'getting-started' zensical.toml` finds the page reference
- `uv run zensical build` exits 0 (site builds successfully)
- `mise run check` exits 0 (all pre-commit/pre-push hooks pass)

## Done When

All 8 verification criteria pass: the tutorials directory and getting-started page exist with
correct front matter and content, the nav is updated, the site builds, and all quality gates are
clean.
