# Next Work Package

## Step: Create Go how-to guide and add to navigation

## Goal

Create `docs/howto/go.md` — a comprehensive how-to guide for Go developers — and add it to the
`zensical.toml` navigation. This fills the largest remaining documentation gap and makes the Go
binding discoverable on the documentation site.

## Scope

- **Create**: `docs/howto/go.md`
- **Modify**: `zensical.toml` (add Go entry to How-to Guides nav)
- **Reference**: `docs/howto/python.md` (structure and section pattern to follow),
    `docs/howto/nodejs.md` (streaming pattern), `packages/go/README.md` (API details and code
    examples), `packages/go/iscc.go` (actual Go API signatures)

## Not In Scope

- Creating `docs/howto/java.md` — that is a separate step
- Adding Go code tabs to existing multi-language tabbed blocks on other pages
- Adding a Go API reference page
- Modifying Go binding code or tests
- Updating the stale "Additional utilities" note in `packages/go/README.md`

## Implementation Notes

Follow the exact structure established by `docs/howto/python.md` and `docs/howto/nodejs.md`:

1. **YAML front matter**: `icon: lucide/package` (Go uses packages), `description:` one-liner
2. **Title and intro**: "# Go" + one paragraph overview mentioning wazero, no cgo, embedded WASM
3. **Installation**: `go get github.com/iscc/iscc-lib/packages/go` — note that the WASM binary is
    embedded, no external files needed
4. **Runtime setup**: explain the `NewRuntime(ctx)` / `defer rt.Close(ctx)` lifecycle pattern —
    this is unique to Go (other bindings don't have a runtime concept). Emphasize that `Runtime`
    is the entry point for all ISCC operations
5. **Code generation**: all 9 `gen_*_v0` functions with Go examples. Use the same subsection
    ordering as Python (Meta, Text, Image, Audio, Video, Mixed, Data, Instance, ISCC-CODE). Each
    example should show idiomatic Go with `context.Background()`, `error` checking via
    `if err != nil`, and `log.Fatal(err)`. Reference `packages/go/iscc.go` for exact function
    signatures (all methods are on `*Runtime`, accept `context.Context` first, return
    `(string, error)`)
6. **Streaming**: `DataHasher` and `InstanceHasher` with `NewDataHasher`/`NewInstanceHasher` →
    `Update(ctx, []byte)` → `Finalize(ctx)` → `Close(ctx)` pattern. Show chunked file reading.
    Reference `packages/go/iscc.go` for hasher struct definitions
7. **Text utilities**: `TextClean`, `TextRemoveNewlines`, `TextTrim`, `TextCollapse` — brief
    section like the Python page
8. **Algorithm primitives**: `SlidingWindow`, `AlgMinhash256`, `AlgCdcChunks`, `AlgSimhash` —
    mention availability, brief example of `SlidingWindow`
9. **Conformance testing**: `ConformanceSelftest` example
10. **Error handling**: idiomatic Go `error` returns (no exceptions). Brief example

For the `zensical.toml` nav, insert Go after WebAssembly in the How-to Guides list:

```toml
{ "Go" = "howto/go.md" },
```

Key Go API details to get right (read from `iscc.go`):

- All methods require `context.Context` as first argument
- `GenMetaCodeV0(ctx, name, description, meta, bits)` — `description` and `meta` are `*string`
    (pointer for nullable), `bits` is `int32`
- Data/Instance functions accept `[]byte` directly
- Video functions accept `[][]int32` for frame signatures
- Mixed-Code accepts `[]string` for ISCC code strings
- Hashers: `NewDataHasher(ctx)` returns `(*DataHasher, error)`, `Update(ctx, []byte)` returns
    `error`, `Finalize(ctx)` returns `(string, error)`, `Close(ctx)` returns `error`

Run `mise run format` before committing to satisfy pre-commit hooks (mdformat, etc.).

## Verification

- `test -f docs/howto/go.md` exits 0 — file exists
- `grep 'go get github.com/iscc/iscc-lib/packages/go' docs/howto/go.md` exits 0 — install command
- `grep 'NewRuntime' docs/howto/go.md` exits 0 — runtime setup documented
- `grep 'GenMetaCodeV0' docs/howto/go.md` exits 0 — code generation documented
- `grep 'DataHasher' docs/howto/go.md` exits 0 — streaming documented
- `grep 'TextClean' docs/howto/go.md` exits 0 — text utilities documented
- `grep 'ConformanceSelftest' docs/howto/go.md` exits 0 — conformance testing documented
- `grep '"Go"' zensical.toml` exits 0 — nav entry present
- `grep 'howto/go.md' zensical.toml` exits 0 — nav path correct
- `uv run zensical build` exits 0 — site builds successfully with the new page

## Done When

All verification criteria pass: `docs/howto/go.md` exists with complete Go how-to content (install,
runtime, all 9 gen functions, streaming, text utils, conformance, error handling), `zensical.toml`
nav includes Go, and `uv run zensical build` succeeds.
