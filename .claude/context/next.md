# Next Work Package

## Step: Convert getting-started tutorial to tabbed multi-language examples

## Goal

The getting-started tutorial (`docs/tutorials/getting-started.md`) is currently Python-only. Convert
its code examples to use tabbed multi-language format (matching the landing page pattern) so
developers using any supported language can follow along. This addresses the documentation spec
requirement that "all code examples use tabbed multi-language format."

## Scope

- **Create**: (none)
- **Modify**: `docs/tutorials/getting-started.md`
- **Reference**:
    - `docs/index.md` — existing tabbed pattern to follow (tab syntax, ordering)
    - `docs/howto/python.md` — Python code snippets
    - `docs/howto/rust.md` — Rust code snippets
    - `docs/howto/nodejs.md` — Node.js code snippets
    - `docs/howto/java.md` — Java code snippets
    - `docs/howto/go.md` — Go code snippets
    - `docs/howto/wasm.md` — WASM code snippets
    - `.claude/context/specs/documentation.md` — doc spec (tab order, requirements)

## Not In Scope

- Converting per-language howto guides to use tabs — they are language-specific by design and each
    covers one language in depth
- Adding new concepts or tutorial sections beyond what already exists
- Changing the tutorial's narrative structure or learning progression
- Updating `scripts/gen_llms_full.py` or `docs/llms.txt` (regenerated at build time)
- Modifying any other documentation pages

## Implementation Notes

**Tab order**: Follow the landing page pattern: Python, Rust, Node.js, Java, Go, WASM (6 tabs).
Python stays first since it is the most accessible language for newcomers.

**Sections to convert to tabbed format** (at least 5 tabbed groups):

1. **Prerequisites + Install** — Replace the Python-only prerequisites with a general statement.
    Convert the install section to tabs showing each language's package manager command. Drop the
    `uv add` alternative (keep it simple — one install command per language).

2. **Conformance selftest** — Show `conformance_selftest()` in all 6 languages.

3. **gen_meta_code_v0 example** — Show the Meta-Code generation example in all 6 languages. Keep the
    JSON serialization example Python-only (in an admonition or after the tabs) since JSON output
    semantics vary by language.

4. **iscc_decompose example** — Show decomposition in all 6 languages.

5. **gen_text_code_v0 + gen_instance_code_v0** — Convert the "Try other code types" section. Each
    code type sub-section gets its own tabbed block.

**Streaming hasher**: Keep the InstanceHasher streaming example in a separate tabbed block. Each
language has its own idiomatic streaming pattern — pull snippets from the howto guides. For WASM,
streaming uses `Uint8Array` chunks.

**Code snippet sources**: Pull accurate code from the per-language howto guides. Each guide has
sections for code generation, streaming, and codec operations with working examples. Adapt the
snippets to match the tutorial's variable names and descriptions (e.g., use "The Neverending Story"
as the metadata title consistently).

**Pattern reference**: The landing page `docs/index.md` lines 53+ show the exact `=== "Language"`
pymdownx.tabbed syntax. Each tab is indented 4 spaces, with code blocks inside.

**Keep it concise**: Each tab should show 3-6 lines of code, not a full program. The howto guides
cover comprehensive usage — the tutorial should be a taste, not a feast.

## Verification

- `grep -c '=== "Python"' docs/tutorials/getting-started.md` returns ≥ 5 (at least 5 tabbed
    sections)
- `grep -c '=== "Rust"' docs/tutorials/getting-started.md` returns ≥ 5
- `grep -c '=== "Go"' docs/tutorials/getting-started.md` returns ≥ 5
- `uv run zensical build` exits 0 (site builds cleanly)
- `mise run format` produces no changes (pre-formatted)

## Done When

The advance agent is done when `docs/tutorials/getting-started.md` uses tabbed multi-language code
examples for all major code blocks (install, selftest, meta-code, decompose, text-code,
instance-code, streaming) and all 5 verification checks pass.
