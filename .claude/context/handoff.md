## 2026-11-20 — Binding specs stop naming repository paths that do not exist

**Done:** Removed every backticked repo path in the wasm, go and nodejs binding specs that no file
backs. These were the five prescribed sites plus one sibling (the napi `npm/*/package.json` row).
The wasm Version Sync table became prose that says where npm metadata really comes from.

**Files changed:**

- `.claude/context/specs/wasm-bindings.md`: dropped the top-level `├── package.json` tree line (the
    `pkg/` child stays). Replaced the Version Sync table with prose: wasm-pack generates
    `pkg/package.json` from `crates/iscc-wasm/Cargo.toml` (version, `license`, `description`), and
    release.yml's "Set npm package name and version" step patches `name`, `repository` and
    `version`. The prose points to ci-cd.md's "Automatic propagation" section by name. Checked
    against `release.yml` lines 428-439 and `crates/iscc-wasm/Cargo.toml`, which has `description`
    and `license.workspace`
- `.claude/context/specs/go-bindings.md`: dropped the `packages/go/version.go` table and the
    "embedded constant is synced" sentence. Tag form corrected to `packages/go/vX.Y.Z`. Dropped the
    `docs/go-api.md` line
- `.claude/context/specs/nodejs-bindings.md`: dropped the `docs/nodejs-api.md` line and the
    `crates/iscc-napi/npm/*/package.json` table row

**Verification:**

- `grep -nE '^├── package\.json' …wasm-bindings.md`: no output. `grep -c 'package.json'` gives 2
    (the `pkg/` child line plus the prose)
- The four-path `grep -rn … .claude/context/specs/` printed nothing
- `grep -n 'go/vX' go-bindings.md | grep -v 'packages/go/vX'` printed nothing
- The existence loop printed exactly `packages/go/vX.Y.Z` and `scripts/gen_ffi_boundary_vectors.py`
- `mise run check`: exit 0 on the final tree (re-run after the wording tweak and handoff write). No
    test or script references these spec files (`grep -rl … tests scripts` was empty)

**Next:** the derived-facts parity check (`normal` [human]) needs its own scoping pass: which pages
enumerate Tier 1, and how each language's names map back to the Rust names. After that, sweep the
spec counts and Crate Structure trees (the wasm tree still omits `tests/unit.rs` and
`tests/data_edge.rs`).

**Notes (deviations from next.md, each a judgment call):**

- **napi `npm/*/package.json` row removed.** This was not one of the five sites: the check's regex
    excludes `*`, so it could not see the row. No `crates/iscc-napi/npm/` dir exists, `TARGETS` in
    `version_sync.py` has a single napi entry, and nodejs-bindings.md itself says the package
    "publishes **no per-platform sibling packages**". The row contradicted its own spec
- **Two Verification Criteria bullets rewritten.** Both wasm and go said "Version synced from root
    `Cargo.toml` via `mise run version:sync`". That is false, because `TARGETS` has no wasm or Go
    entry, and it contradicted the rewritten Version Sync sections. Now: wasm says "npm package
    version comes from root `Cargo.toml` (wasm-pack output patched by release.yml)", and go says
    "Module version set by `packages/go/vX.Y.Z` Git tags (no in-repo package-version string)". These
    are acceptance-criteria edits, so review should confirm or revert them
- Go wording says "package-version string", not "version constant": `packages/go/codec.go` defines
    `VSV0`/`VSV1` of type `Version`, the ISCC header version
- Untouched as scoped out: Tier 1 counts, Crate Structure tree omissions, `notes/`, and the
    `edge-vectors.md` historical script reference
