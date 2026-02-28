# Next Work Package

## Step: Fix stale wazero/WASM references in docs

## Goal

Remove all stale wazero/WASM references from `docs/architecture.md` and `docs/development.md` to
reflect the Go pure rewrite. These are factual inaccuracies on the public documentation site — the
Go binding is now a pure Go implementation with no wazero, no WASM binary, and no cargo involvement.

## Scope

- **Create**: (none)
- **Modify**:
    - `docs/architecture.md` — 7 stale references
    - `docs/development.md` — 4 stale references
- **Reference**:
    - `packages/go/` directory listing — actual Go module structure (36 `.go` files, no WASM)
    - `docs/howto/go.md` — already-updated Go howto guide for consistent wording
    - `packages/go/README.md` — already-updated Go README for consistent wording

## Not In Scope

- Tab order standardization (needs human decision, low-priority issue)
- Rewriting the full Go section of architecture.md (only fix stale references)
- Updating `specs/ci-cd.md` standard action set (cosmetic, separate step if needed)
- Cleaning up vestigial WASM comments in Go test files (cosmetic, separate step)
- Any code changes

## Implementation Notes

### `docs/architecture.md` — 7 edits:

1. **Line 26 (Mermaid diagram)**: Change `GO["Go<br/><small>wazero · WASM</small>"] -.-> FFI` to
    show Go as a standalone pure Go module (no dependency on FFI). Use a solid arrow to CORE or a
    standalone node with `<small>Pure Go</small>`. The key point: Go does NOT depend on FFI or any
    Rust crate.
2. **Lines 79-83 (workspace tree)**: Replace the stale `packages/go/` subtree. Actual structure has
    `go.mod`, `go.sum`, `codec.go`, `conformance.go`, multiple `code_*.go` files, `README.md`. Show
    a representative subset (not all 36 files). No `iscc.go`, no `iscc_ffi.wasm`.
3. **Line 86 (workflows)**: Add `release.yml` to the workflows listing (currently missing).
4. **Line 99 (crate summary table)**: Change `cargo + wazero` to `go` for the `packages/go` row.
5. **Line 171 (per-binding adaptation table)**: Change Go row from
    `Sync API via wazero WASM calls. UpdateFrom(ctx, io.Reader) for streaming` to reflect pure Go
    sync API with `Push([]byte)` / `Finalize(bits)` streaming.
6. **Line 201 (cross-language test matrix)**: Change Go row from
    `Embedded via WASM conformance selftest` to `Relative path from test file` (Go tests load
    `../../crates/iscc-lib/tests/data.json`).

### `docs/development.md` — 4 edits:

1. **Line 47 (included tools table)**: Change `Go bindings (wazero)` to `Go bindings (pure Go)` or
    just `Go bindings`.
2. **Lines 198-202 (project structure tree)**: Replace the stale `packages/go/` subtree with actual
    files. Same representative subset as architecture.md.
3. **Line 228 (crate summary table)**: Change `cargo + wazero` to `go` for the `packages/go` row.

### Consistency guidance:

- Both files share identical directory trees and crate summary tables — keep them in sync.
- Use "pure Go" wording consistent with `packages/go/README.md` and `docs/howto/go.md`.
- The Go module line in directory trees should read: `# Go module (pure Go, no cgo)` — this is
    already used at line 79 of architecture.md and line 198 of development.md.

## Verification

- `grep -c 'wazero' docs/architecture.md` outputs `0`
- `grep -c 'wazero' docs/development.md` outputs `0`
- `grep -c 'iscc_ffi\.wasm' docs/architecture.md` outputs `0`
- `grep -c 'iscc_ffi\.wasm' docs/development.md` outputs `0`
- `grep -c 'cargo + wazero' docs/architecture.md` outputs `0`
- `grep -c 'cargo + wazero' docs/development.md` outputs `0`
- `grep -q 'release\.yml' docs/architecture.md` exits 0 (release workflow listed)
- `uv run zensical build` succeeds
- `mise run check` passes (all pre-commit hooks)

## Done When

All verification criteria pass — zero stale wazero/WASM references remain in either documentation
page, the Go module is accurately represented as pure Go throughout, and the docs site builds
cleanly.
