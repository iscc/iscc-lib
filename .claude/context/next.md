# Next Work Package

## Step: Go module scaffold with wazero runtime and memory helpers

## Goal

Create the foundational `packages/go/` Go module with wazero-based WASM runtime initialization and
memory management helpers (string/bytes marshaling via `iscc_alloc`/`iscc_dealloc`), proving the
end-to-end Go→WASM→Rust bridge works by calling `iscc_conformance_selftest()`.

## Scope

- **Create**: `packages/go/go.mod`, `packages/go/iscc.go`, `packages/go/iscc_test.go`
- **Modify**: `mise.toml` (add Go to `[tools]` section)
- **Reference**: `crates/iscc-ffi/src/lib.rs` (FFI function signatures, alloc/dealloc, free
    functions), `notes/02-language-bindings.md` (binding architecture)

## Not In Scope

- ISCC function wrappers (`GenMetaCodeV0`, `GenTextCodeV0`, etc.) — those come in the next step
- Streaming hasher wrappers (`DataHasher`, `InstanceHasher`) — separate step after gen functions
- Go CI job in `.github/workflows/ci.yml` — separate step
- `.devcontainer/Dockerfile` Go installation — mise handles it for now; Dockerfile update is
    separate
- Release-optimized WASM binary (wasm-opt, size reduction) — the debug binary works for scaffold
- `packages/go/README.md` — created after Go API is complete
- `docs/howto/go.md` — created after Go API is complete
- Root README Go sections — created after Go API is complete

## Implementation Notes

### Go installation

Add a `[tools]` section to `mise.toml` with `go = "latest"`. Run `mise install` to make Go
available. Verify with `go version`.

### Module structure

The Go module path should be `github.com/iscc/iscc-lib/packages/go` (matching target.md's `go get`
path). Use a package name of `iscc` so consumers import as:

```go
import iscc "github.com/iscc/iscc-lib/packages/go"
```

### WASM binary

Build with `cargo build -p iscc-ffi --target wasm32-wasip1` (the `wasm32-wasip1` target should
already be installed from the previous iteration). Copy the binary to `packages/go/iscc_ffi.wasm`.
Add `packages/go/*.wasm` to the root `.gitignore` — the binary is ~10.5 MB debug and should not be
checked in. Tests require a pre-built binary; fail fast with a clear message if missing.

Embed via `//go:embed iscc_ffi.wasm` into a `var wasmModule []byte`.

### Wazero runtime initialization

Use `wazero` (github.com/tetratelabs/wazero) as the pure-Go WASM runtime. Key pattern:

```go
type Runtime struct {
    runtime wazero.Runtime
    module  api.Module
}

func NewRuntime(ctx context.Context) (*Runtime, error) {
    r := wazero.NewRuntime(ctx)
    // WASI is needed because iscc-ffi targets wasm32-wasip1
    wasi_snapshot_preview1.MustInstantiate(ctx, r)
    mod, err := r.Instantiate(ctx, wasmModule)
    // ...
}

func (rt *Runtime) Close(ctx context.Context) error {
    return rt.runtime.Close(ctx)
}
```

### Memory helpers

The Go bridge needs these low-level helpers on the `Runtime` type:

1. **`alloc(ctx, size uint32) uint32`** — calls `iscc_alloc`, returns WASM pointer
2. **`dealloc(ctx, ptr, size uint32)`** — calls `iscc_dealloc`
3. **`writeString(ctx, s string) (ptr, size uint32)`** — alloc + write UTF-8 bytes + null terminator
    into WASM memory; returns pointer and byte count (including null)
4. **`readString(ctx, ptr uint32) string`** — read null-terminated C string from WASM memory
5. **`freeString(ctx, ptr uint32)`** — calls `iscc_free_string`

All helpers take `context.Context` as first argument (wazero convention). Use
`mod.ExportedFunction("iscc_alloc").Call(ctx, uint64(size))` for function calls and
`mod.Memory().Read(offset, size)` / `.Write(offset, data)` for memory access.

### Error handling

Use idiomatic Go error returns. If a WASM function returns NULL (pointer 0), read the error from
`iscc_last_error()` and return it as `error`. Pattern:

```go
func (rt *Runtime) lastError(ctx context.Context) string {
    results, _ := rt.module.ExportedFunction("iscc_last_error").Call(ctx)
    ptr := uint32(results[0])
    if ptr == 0 { return "" }
    return rt.readString(ctx, ptr) // don't free — iscc_last_error returns borrowed pointer
}
```

### Test strategy

Tests must first build the WASM binary if missing. Use `TestMain` to check for the binary and skip
all tests with a clear message if Go can't find it. Key tests:

1. `TestRuntimeInit` — creates and closes a Runtime successfully
2. `TestConformanceSelftest` — calls `iscc_conformance_selftest()`, asserts returns true (proves
    end-to-end bridge works)
3. `TestAllocDealloc` — allocates memory, writes bytes, reads them back, deallocates
4. `TestWriteReadString` — roundtrips a Unicode string through WASM memory
5. `TestTextClean` — calls `iscc_text_clean("Hello  World")` via writeString → call → readString →
    freeString, asserts result equals "Hello World" (proves string marshaling works end-to-end)

### Wazero API notes

- Wazero functions return `[]uint64` — cast to `uint32` for WASM32 pointers
- `api.Module.Memory()` gives access to linear memory for read/write
- WASI instantiation is required for `wasm32-wasip1` modules (provides `fd_write`, etc.)
- Use `wazero.NewModuleConfig().WithStdout(io.Discard).WithStderr(io.Discard)` to suppress WASI
    stdout/stderr noise in tests

## Verification

- `mise install` installs Go and `go version` exits 0
- `cd packages/go && go mod tidy` exits 0 (Go module is valid with all dependencies resolved)
- `cd packages/go && go test -v ./...` passes all tests (≥5 tests: runtime init, conformance
    selftest, alloc/dealloc, string roundtrip, text_clean end-to-end)
- `cd packages/go && CGO_ENABLED=0 go test ./...` passes (proves no cgo dependency)
- The `iscc_conformance_selftest` test returns true (proves the full Rust ISCC core runs correctly
    inside the WASM module via the Go bridge)

## Done When

All verification criteria pass: Go module compiles and tests pass with `CGO_ENABLED=0`, including a
successful `iscc_conformance_selftest()` call proving the end-to-end Go→wazero→WASM→Rust bridge
works.
