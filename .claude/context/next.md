# Next Work Package

## Step: Evaluate TypeScript port and add ecosystem docs page

## Goal

Research the community TypeScript implementation `branciard/iscc-core-ts`, assess its conformance
status against official `data.json` test vectors, and add an "Ecosystem" documentation page
mentioning community implementations. This closes the only remaining CID-actionable issue and
improves discoverability for developers exploring the ISCC ecosystem.

## Scope

- **Create**: `docs/ecosystem.md` — new documentation page covering community implementations
- **Modify**: `zensical.toml` — add navigation entry for the ecosystem page
- **Reference**: `branciard/iscc-core-ts` GitHub repository (use WebFetch or deepwiki MCP to
    examine), `docs/architecture.md` (for consistent formatting), `zensical.toml` (for nav pattern)

## Not In Scope

- Running the TypeScript test suite locally or in CI — we only assess by examining the repo
- Adding `iscc-core-ts` as a dependency or submodule
- Contributing to or forking the TypeScript repo
- Creating conformance test infrastructure for third-party implementations
- Modifying the root README to mention community implementations (future step if warranted)
- Adding links from other docs pages to the ecosystem page

## Implementation Notes

1. **Research phase**: Use WebFetch to examine the `branciard/iscc-core-ts` GitHub repo:

    - Read the README for feature coverage claims
    - Check if it has conformance tests against the official `data.json` vectors
    - Look at the test files to see which `gen_*_v0` functions are implemented
    - Check the package's npm status (published? version?)
    - Note the license

2. **Create `docs/ecosystem.md`**:

    - Add YAML front matter with `icon: lucide/globe` and a `description:` field
    - Start with a brief intro explaining that ISCC is an open standard (ISO 24138:2024) and
        community implementations are welcome
    - Add a "Community Implementations" section with a subsection for `iscc-core-ts`:
        - Repository link, author, license
        - Language/platform
        - Conformance status (based on research findings — be factual and neutral)
        - Brief description of what it covers
    - Add an "Official Implementations" section listing:
        - `iscc-core` (Python reference implementation)
        - `iscc-lib` (this project — Rust + polyglot bindings)
    - Add a note encouraging contributions and linking to the ISCC specification
    - Use admonition boxes for important notes (e.g., "Community implementations are independently
        maintained and may not track the latest specification changes")

3. **Update `zensical.toml`**: Add the ecosystem page to the nav. Place it after "Explanation" and
    before "Reference" as a top-level entry:

    ```
    { "Ecosystem" = "ecosystem.md" },
    ```

4. **Formatting**: Follow the established pattern from other docs pages — YAML front matter with
    `icon` and `description`, consistent heading levels, use of Material for MkDocs features
    (admonition boxes, external links).

## Verification

- `uv run zensical build` exits 0 (site builds successfully with the new page)
- `test -f docs/ecosystem.md` exits 0 (ecosystem page exists)
- `grep -q 'ecosystem.md' zensical.toml` exits 0 (navigation entry present)
- `grep -q 'iscc-core-ts' docs/ecosystem.md` exits 0 (TypeScript port mentioned)
- `grep -q 'branciard' docs/ecosystem.md` exits 0 (author/repo attributed)
- `grep -q 'iscc-core' docs/ecosystem.md` exits 0 (reference implementation mentioned)
- `grep -q 'icon:' docs/ecosystem.md` exits 0 (YAML front matter with icon present)
- `cargo clippy --workspace --all-targets -- -D warnings` remains clean (no Rust changes)

## Done When

All verification criteria pass — the documentation site builds with a new Ecosystem page that
accurately describes the `branciard/iscc-core-ts` community implementation alongside the official
ISCC implementations.
