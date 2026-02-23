# Next Work Package

## Step: Add architecture overview documentation page

## Goal

Create an architecture page (`docs/architecture.md`) that documents the hub-and-spoke crate model,
design decisions, and internal module structure — filling the key documentation gap before the docs
site goes live.

## Scope

- **Create**: `docs/architecture.md`
- **Modify**: `zensical.toml` (add `architecture.md` to the `nav` list)
- **Reference**: `notes/00-overview.md` (project structure, design decisions, tooling),
    `notes/01-workspace-structure.md` (workspace layout), `notes/03-async-and-streaming.md`
    (streaming API pattern), `notes/04-api-compatibility-safety.md` (API tiers),
    `crates/iscc-lib/src/lib.rs` (module structure), `docs/index.md` (existing style reference)

## Implementation Notes

This is a user-facing documentation page — write for developers who want to understand how iscc-lib
is structured, not for internal contributors. Focus on:

1. **Hub-and-spoke crate model**: One diagram (Mermaid) showing `iscc-lib` at center with `iscc-py`,
    `iscc-napi`, `iscc-wasm`, `iscc-ffi` as spokes. The config already has `pymdownx.superfences`
    with mermaid custom fences enabled.

2. **Workspace layout**: A tree showing the `crates/` directory structure with brief annotations for
    each crate's purpose. Keep it factual — list what each crate is, what it produces (wheel, npm
    package, `.so`/`.dll`, WASM), and what build tool it uses.

3. **Internal module structure**: List the modules inside `iscc-lib` core crate (api, codec,
    content_code, data_code, instance_code, meta_code, text_code, utils — check `lib.rs` for the
    actual list). Explain the tiered API: Tier 1 (`api` module, bound to all languages) vs Tier 2
    (`codec`, public Rust API but not FFI-bound) vs internal (`pub(crate)` modules).

4. **Streaming pattern**: Briefly describe the `new() → update(&[u8]) → finalize() → Result<T>`
    pattern used by Data-Code and Instance-Code, explaining why the core is synchronous while
    bindings may adapt to async runtimes.

5. **Conformance testing**: Explain that all bindings share the same `data.json` test vectors from
    iscc-core, ensuring cross-language consistency.

Style guidelines:

- Match the tone and structure of `docs/index.md` (concise, table-heavy, code examples where useful)
- Use Material for MkDocs features already configured: admonitions, tabs, tables, mermaid diagrams
- Keep the page focused — this is architecture overview, not a full developer guide
- Use the learnings about `mdformat-mkdocs` mangling `:::` directives — avoid inline mkdocstrings
    options (not needed on this page anyway)
- Use Unicode `—` for em dashes (not `---`, which smartsymbols doesn't convert)

## Verification

- `docs/architecture.md` exists with all 5 content sections (crate model, workspace layout, module
    structure, streaming pattern, conformance testing)
- `zensical.toml` `nav` includes `architecture.md` (positioned between `index.md` and `api.md`)
- Mermaid diagram renders valid syntax (has `graph` or `flowchart` directive, proper node/edge
    syntax)
- No broken markdown (headers, tables, code blocks all properly formed)
- Content is accurate — module names match actual `lib.rs`, crate names match actual workspace
    members
- All existing tests still pass (`cargo test -p iscc-lib`)

## Done When

The architecture page exists with accurate content covering all 5 sections, is wired into the nav,
and all existing tests still pass.
