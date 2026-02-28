## 2026-02-28 — Fix stale wazero/WASM references in docs

**Done:** Removed all stale wazero/WASM references from `docs/architecture.md` (6 edits) and
`docs/development.md` (3 edits). The Go module is now accurately represented as a pure Go
implementation throughout both documentation pages. Also added `release.yml` to the workflows
listing in architecture.md.

**Files changed:**

- `docs/architecture.md`: Updated Mermaid diagram (Go node → Pure Go, solid arrow to CORE instead of
    dotted to FFI), replaced stale Go subtree with actual file listing, changed build tool from
    `cargo + wazero` to `go`, updated per-binding streaming description, updated cross-language test
    matrix vector access, added `release.yml` to workflows listing
- `docs/development.md`: Changed included tools table from `Go bindings (wazero)` to `Go bindings`,
    replaced stale Go subtree with actual file listing, changed build tool from `cargo + wazero` to
    `go`

**Verification:**

- `grep -c 'wazero' docs/architecture.md` → 0 ✓
- `grep -c 'wazero' docs/development.md` → 0 ✓
- `grep -c 'iscc_ffi\.wasm' docs/architecture.md` → 0 ✓
- `grep -c 'iscc_ffi\.wasm' docs/development.md` → 0 ✓
- `grep -c 'cargo + wazero' docs/architecture.md` → 0 ✓
- `grep -c 'cargo + wazero' docs/development.md` → 0 ✓
- `grep -q 'release\.yml' docs/architecture.md` → exits 0 ✓
- `uv run zensical build` → succeeds ✓
- `mise run check` → all 14 hooks pass ✓

**Next:** The project is in maintenance mode with all functional requirements met. Remaining work is
human-dependent: merge PR #10 (develop → main), trigger 0.0.2 releases, configure Maven Central
publishing. The CID loop provides diminishing returns at this point.

**Notes:** The Mermaid diagram edit changes Go from a dotted dependency on FFI (`-.-> FFI`) to a
solid dependency directly on CORE (`--> CORE`). This accurately reflects that Go is a standalone
reimplementation, not a binding via FFI/WASM. The directory tree uses glob patterns
(`code_content_*.go`, `*_test.go`) to keep the listing concise — consistent with how the tree
represents the actual 36+ files without listing them all.
