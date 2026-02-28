# Next Work Package

## Step: Fix Go representation in architecture diagram

## Goal

Correct the Mermaid diagram and accompanying text in `docs/architecture.md` so that Go is shown as a
standalone pure Go reimplementation rather than a binding crate that depends on the Rust core. Also
clean up 5 vestigial "do NOT require the WASM binary" comments in Go test files. Both are
post-WASM-removal cleanup items flagged by the review agent.

## Scope

- **Create**: (none)
- **Modify**:
    - `docs/architecture.md` — Mermaid diagram + explanatory text below it
    - `packages/go/minhash_test.go` — remove WASM comment (test file)
    - `packages/go/utils_test.go` — remove WASM comment (test file)
    - `packages/go/codec_test.go` — remove WASM comment (test file)
    - `packages/go/cdc_test.go` — remove WASM comment (test file)
    - `packages/go/simhash_test.go` — remove WASM comment (test file)
- **Reference**: `docs/architecture.md` (current diagram at lines 19-30)

## Not In Scope

- Rewriting the entire architecture page or other sections beyond the diagram/text
- Changing the hub-and-spoke description — it's still accurate for the 5 Rust-based binding crates
- Modifying `docs/development.md` (already updated in the previous iteration)
- Renaming `TestPureGo*` test function prefixes (cosmetic, separate concern)
- Touching any Go source (non-test) files

## Implementation Notes

**Mermaid diagram change:** Show Go as a separate standalone node, not connected to CORE. Options:

1. Add Go as a disconnected node with a different style (recommended):

    ```mermaid
    graph TD
        PY[...] --> CORE[...]
        NAPI[...] --> CORE
        WASM[...] --> CORE
        FFI[...] --> CORE
        JNI[...] --> CORE
        GO["Go module<br/><small>Pure Go reimplementation</small>"]
        style GO fill:#e8f5e9,stroke:#4caf50
    ```

    The `style` line gives Go a visually distinct appearance (green tint) to signal it's different.

2. Alternatively, use a dotted line with a label: `GO -.->|"reimplements API"| CORE` — but a
    disconnected node is cleaner and more accurate.

**Text change:** Replace "All binding crates are thin wrappers — they contain no algorithm logic.
This ensures that every language produces identical results for the same inputs." with something
like: "The five binding crates (Python, Node.js, WASM, C FFI, Java) are thin wrappers — they contain
no algorithm logic. The Go module is a standalone reimplementation of the same algorithms in pure
Go. All languages produce identical results for the same inputs, verified by shared conformance test
vectors."

**Go test file comments:** In each of the 5 test files, the second line reads:
`// These tests do NOT require the WASM binary — they test pure Go functions.` Replace with a
simpler comment like `// These tests verify pure Go functions.` or just remove the line entirely
(the first line already says "Tests for the pure Go ... module"). Removing the line is cleanest —
the "do NOT require WASM" phrasing is a historical artifact with no current value.

## Verification

- `grep -c 'GO.*-->.*CORE' docs/architecture.md` returns 0
- `grep -q 'standalone reimplementation\|pure Go reimplementation' docs/architecture.md` exits 0
- `grep -c 'All binding crates are thin wrappers' docs/architecture.md` returns 0
- `grep -rc 'WASM binary' packages/go/` returns 0 (no matches)
- `uv run zensical build` succeeds (site builds with updated Mermaid diagram)
- `cd packages/go && go test ./... && go vet ./...` passes (Go tests still work after comment
    changes)
- `mise run check` passes (all pre-commit/pre-push hooks clean)

## Done When

All verification criteria pass: Go is correctly represented as standalone in the architecture
diagram, the explanatory text is accurate, and all vestigial WASM comments are removed from Go test
files.
