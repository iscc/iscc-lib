# Next Work Package

## Step: Remove WASM vestiges from Go CI, README, and docs

## Goal

The Go bindings are now pure Go (30/30 Tier 1 symbols) but three files still describe the old
wazero/WASM bridge architecture: the CI job wastefully builds a WASM binary, the per-crate README
shows a completely wrong API (Runtime/context pattern), and the howto guide has 460 lines of stale
code examples. This step brings all three in sync with the actual pure Go implementation.

## Scope

- **Create**: (none)
- **Modify**:
    - `.github/workflows/ci.yml` — strip WASM build steps from Go job (remove Rust toolchain,
        `wasm32-wasip1` target, cargo build, WASM copy)
    - `packages/go/README.md` — rewrite to reflect pure Go architecture and direct function API
    - `docs/howto/go.md` — rewrite all code examples from `rt.Method(ctx, ...)` to
        `iscc.Function(...)` pattern, remove Runtime/wazero/WASM references
- **Reference**:
    - `packages/go/code_meta.go` — actual `GenMetaCodeV0` signature
    - `packages/go/code_data.go` — `DataHasher` with `NewDataHasher()` → `Push()` → `Finalize()`
    - `packages/go/code_instance.go` — `InstanceHasher` with `NewInstanceHasher()` → `Push()` →
        `Finalize()`
    - `packages/go/codec.go` — `EncodeBase64`, `JsonToDataUrl`, `EncodeComponent`, `IsccDecode`,
        `IsccDecompose`
    - `packages/go/utils.go` — `TextClean`, `TextRemoveNewlines`, `TextTrim`, `TextCollapse`
    - `packages/go/simhash.go` — `SlidingWindow`, `AlgSimhash`
    - `packages/go/minhash.go` — `AlgMinhash256`
    - `packages/go/cdc.go` — `AlgCdcChunks`
    - `packages/go/conformance.go` — `ConformanceSelftest`

## Not In Scope

- Renaming `TestPureGo*` test prefixes — cosmetic, defer to a future cleanup step
- Removing vestigial "do NOT require the WASM binary" comments from test files — cosmetic
- Benchmark CI integration — separate concern, next priority after this cleanup
- PR from develop → main — do after this cleanup lands on develop
- Updating the root README.md or other binding docs — only Go-specific files

## Implementation Notes

### CI job (`.github/workflows/ci.yml`)

The Go job (lines 116-137) currently has 6 steps. After cleanup it needs only 4:

```yaml
go:
  name: Go (go test, go vet)
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: actions/setup-go@v5
      with:
        go-version-file: packages/go/go.mod
    - name: Run Go tests
      run: CGO_ENABLED=0 go test -v -count=1 ./...
      working-directory: packages/go
    - name: Run Go vet
      run: go vet ./...
      working-directory: packages/go
```

Remove: `dtolnay/rust-toolchain` (with `wasm32-wasip1` target), `Swatinem/rust-cache`, "Build WASM
module" step, and "Copy WASM to Go package" step. The Go job should have zero Rust dependencies.

### README (`packages/go/README.md`)

Key changes:

- **Line 10-12**: Replace wazero/WASM description with "Pure Go implementation of all ISCC
    algorithms — no cgo, no WASM, no binary artifacts"
- **Line 27**: Remove "The WASM binary is embedded" line
- **Quick Start (lines 29-57)**: Replace the `NewRuntime`/`context` example with direct call:
    ```go
    result, err := iscc.GenMetaCodeV0("ISCC Test Document!", nil, nil, 64)
    fmt.Println(result.Iscc)
    ```
    Note: No `context.Context`, no `Runtime`, no `Close`. Import is just `iscc "github.com/..."`.
- **API tables (lines 59-129)**: Already correct — no changes needed. Verify "All methods are on
    `*Runtime`" text on line 75 is updated to "Package-level functions"
- **Architecture (lines 131-139)**: Replace wazero description with pure Go description: native
    compiled code, no runtime overhead, standard `go get` distribution
- Return types are structs (e.g., `*MetaCodeResult`) not strings — update examples accordingly

### Howto guide (`docs/howto/go.md`)

Systematic replacement pattern throughout the file:

- `rt.GenMetaCodeV0(ctx, name, desc, meta, bits)` → `iscc.GenMetaCodeV0(name, desc, meta, bits)`
- `rt.GenTextCodeV0(ctx, text, bits)` → `iscc.GenTextCodeV0(text, bits)`
- `rt.GenImageCodeV0(ctx, pixels, bits)` → `iscc.GenImageCodeV0(pixels, bits)` (and so on)
- `rt.NewDataHasher(ctx)` → `iscc.NewDataHasher()`
- `hasher.Update(ctx, data)` → `hasher.Push(data)`
- `hasher.Finalize(ctx, bits)` → `hasher.Finalize(bits)`
- `hasher.Close(ctx)` → (remove — no Close needed)
- `rt.TextClean(ctx, text)` → `iscc.TextClean(text)` (returns `string`, not `(string, error)`)
- `rt.SlidingWindow(ctx, text, width)` → `iscc.SlidingWindow(text, width)` (returns
    `([]string, error)`)
- `rt.EncodeComponent(ctx, ...)` → `iscc.EncodeComponent(...)` (returns `(string, error)`)
- `rt.IsccDecode(ctx, code)` → `iscc.IsccDecode(code)` (returns `(*DecodeResult, error)`)
- `rt.ConformanceSelftest(ctx)` → `iscc.ConformanceSelftest()` (returns `(bool, error)`)

**Important**: Check actual Go function signatures. Some functions return `(Type, error)`, others
return just `Type` (e.g., `TextClean` returns `string`, `TextRemoveNewlines` returns `string`,
`TextTrim` returns `string`, `TextCollapse` returns `string`, `EncodeBase64` returns `string`). The
utility functions do NOT return errors — they are simple transformations.

Sections to update:

- Remove "Runtime setup" section entirely (lines 22-48)
- Update "Code generation" intro (line 55): "Package-level functions" not "methods on `*Runtime`"
- Return types are structs: `result, err := iscc.GenMetaCodeV0(...)` then `result.Iscc` for the code
    string
- Streaming section: `NewDataHasher()` / `NewInstanceHasher()` return `*DataHasher` /
    `*InstanceHasher` directly (no error, no context). Method is `Push` not `Update`. No `Close`.
- Error handling section (line 452-462): remove "WASM runtime failures" reference
- The `description` field in the front matter should be updated too

## Verification

- `grep -q 'wasm32-wasip1' .github/workflows/ci.yml; test $? -ne 0` — no WASM target in CI
- `grep -qi 'wazero\|wasm' packages/go/README.md; test $? -ne 0` — no WASM references in README
- `grep -qi 'wazero\|wasm\|Runtime\|NewRuntime' docs/howto/go.md; test $? -ne 0` — no old API
    references in howto
- `grep -q 'iscc.GenMetaCodeV0' docs/howto/go.md` — uses direct function call pattern
- `grep -q 'iscc.GenMetaCodeV0' packages/go/README.md` — README uses direct function call
- `mise run check` — all pre-commit/pre-push hooks pass (formatting, lint)

## Done When

All verification commands pass: no WASM/wazero/Runtime references remain in Go README or docs, Go CI
job has no Rust/WASM steps, code examples show the actual pure Go API, and `mise run check` passes.
