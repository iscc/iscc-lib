# Next Work Package

## Step: Add nav icons to all docs pages

## Goal

Add `icon` and `description` front matter to all documentation pages, matching the iscc-usearch
pattern at usearch.iscc.codes. This closes the last remaining documentation gap — after this step,
the Documentation section status moves from "partially met" to "met."

## Scope

- **Create**: (none)
- **Modify**: `docs/index.md`, `docs/architecture.md`, `docs/benchmarks.md`, `docs/development.md`,
    `docs/rust-api.md`, `docs/api.md`, `docs/howto/rust.md`, `docs/howto/python.md`,
    `docs/howto/nodejs.md`, `docs/howto/wasm.md` (10 files — each change is a 3–4 line YAML front
    matter addition, trivially small)
- **Reference**: `docs/tutorials/getting-started.md` (existing front matter example), iscc-usearch
    docs pages via deepwiki MCP (icon naming convention)

## Not In Scope

- Modifying page content, headings, or body text of any docs page
- Adding new docs pages or restructuring navigation
- Changing `zensical.toml` nav configuration (icons go in page front matter, not nav config)
- Updating `docs/rust-api.md` content to match current result struct types (separate issue noted in
    handoff — tracked for a future step)
- Fixing any issues from issues.md (correctness issues like `alg_simhash` panics)

## Implementation Notes

The iscc-usearch reference uses this front matter pattern in every docs page:

```yaml
---
icon: lucide/icon-name
description: One-line page description for SEO and social cards.
---
```

`docs/tutorials/getting-started.md` already has this pattern — use it as the local reference.

Recommended icon assignments (all from the Lucide icon set, matching iscc-usearch style):

| Page              | Icon                      | Rationale                                                   |
| ----------------- | ------------------------- | ----------------------------------------------------------- |
| `index.md`        | `lucide/house`            | Home/landing page (same as iscc-usearch)                    |
| `architecture.md` | `lucide/blocks`           | System architecture (same as iscc-usearch)                  |
| `benchmarks.md`   | `lucide/gauge`            | Performance metrics (same as iscc-usearch performance page) |
| `development.md`  | `lucide/git-pull-request` | Contributing/dev (same as iscc-usearch)                     |
| `rust-api.md`     | `lucide/book-open`        | API reference (same as iscc-usearch)                        |
| `api.md`          | `lucide/book-open`        | API reference (same as iscc-usearch)                        |
| `howto/rust.md`   | `lucide/cog`              | Rust tooling                                                |
| `howto/python.md` | `lucide/terminal`         | Python scripting                                            |
| `howto/nodejs.md` | `lucide/hexagon`          | Node.js (hexagonal logo association)                        |
| `howto/wasm.md`   | `lucide/globe`            | Web/browser (WASM runs in browsers)                         |

The advance agent may adjust specific icon choices if better options exist in the Lucide set, but
should keep the overall style consistent with iscc-usearch. Each description should be one sentence
summarizing the page's purpose.

**Important**: Insert front matter BEFORE the existing first line (the `#` heading). Do not remove
or alter any existing content.

## Verification

- `uv run zensical build` exits 0 (site builds successfully with all icons)
- `grep -c '^icon: lucide/' docs/index.md docs/architecture.md docs/benchmarks.md docs/development.md docs/rust-api.md docs/api.md docs/howto/rust.md docs/howto/python.md docs/howto/nodejs.md docs/howto/wasm.md docs/tutorials/getting-started.md`
    shows 11 files with matches (all 11 pages have icon front matter)
- `grep -c '^description:' docs/index.md docs/architecture.md docs/benchmarks.md docs/development.md docs/rust-api.md docs/api.md docs/howto/rust.md docs/howto/python.md docs/howto/nodejs.md docs/howto/wasm.md docs/tutorials/getting-started.md`
    shows 11 files with matches
- `cargo clippy --workspace --all-targets -- -D warnings` still clean (no Rust changes, but verify
    no regression)

## Done When

All 11 docs pages have `icon: lucide/...` and `description:` front matter, the site builds
successfully, and all verification commands pass.
