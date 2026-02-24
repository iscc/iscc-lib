# Next Work Package

## Step: Create Development documentation page

## Goal

Add a Development section to the docs site (`lib.iscc.codes`) containing contributor-facing content
currently only in the README: dev container setup, CID autonomous workflow, quality gates and
pre-commit hooks, project structure, and mise task runner. This unblocks the README rewrite (next
step) by providing a proper home for development workflow content that the target spec says must NOT
be in the README.

## Scope

- **Create**: `docs/development.md` — single consolidated Development page
- **Modify**: `zensical.toml` — add Development nav group after Benchmarks
- **Reference**: current `README.md` (source content to adapt),
    `.claude/context/specs/documentation.md` (spec requirements)

## Not In Scope

- Rewriting `README.md` — that is the next step; leave the current README as-is for now
- Creating multiple Development sub-pages (dev container, CID, quality gates as separate pages) —
    start with a single page; it can be split later if needed
- Adding abbreviations, CNAME, tutorials, or Rust how-to — those are separate work items
- Adding tabbed code examples to the Development page — this is contributor docs, not user-facing
    API examples

## Implementation Notes

The Development page content should be adapted from the current README sections:

1. **Dev Container Setup** — from README's "Prerequisites" and "Bootstrap" sections. Rewrite for the
    docs site audience (contributors, not first-time cloners). Include the tooling table.

2. **CID Autonomous Workflow** — from README's "Running the CID Loop" section. Include the agent
    table and all `mise run cid:*` commands.

3. **Quality Gates** — from README's "Quality Gates" section. Include pre-commit/pre-push hook
    descriptions, manual commands, and the prek tool reference.

4. **Project Structure** — from README's "Project Structure" section. Expand the tree to include
    `crates/` (iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi), `docs/`, `benchmarks/`, and
    `reference/` directories with brief descriptions.

5. **Mise Task Runner** — add a section listing all `mise run` tasks with descriptions (test, lint,
    format, check, and all cid:\* tasks).

Format as a well-structured single page with H2 sections for each topic. Use admonitions where
appropriate (e.g., `!!! tip` for the git identity note). The page should feel like polished
documentation, not a copy-paste of the README.

For `zensical.toml`, add the Development nav entry after Benchmarks:

```toml
{ "Development" = "development.md" },
```

## Verification

- `uv run zensical build` exits 0 (site builds successfully with new page)
- `docs/development.md` exists and contains sections for: dev container, CID workflow, quality
    gates, project structure, mise tasks
- `zensical.toml` nav includes a Development entry
- `grep -c "Development" zensical.toml` returns at least 1
- `grep -c "## " docs/development.md` returns at least 5 (one per major section)
- `cargo clippy --workspace --all-targets -- -D warnings` still clean (no Rust changes, but verify
    no regression)

## Done When

All verification criteria pass: the docs site builds with a Development page visible in navigation,
containing contributor-focused content covering dev container setup, CID workflow, quality gates,
project structure, and mise tasks.
