# Next Work Package

## Step: Add io.Reader support to Go streaming hashers

## Goal

Add `UpdateFrom(ctx, io.Reader)` methods to Go `DataHasher` and `InstanceHasher`, making the
streaming API idiomatic for Go developers who work with files, network streams, and other
`io.Reader` sources. The target architecture describes "io.Reader support for streaming" as part of
the Go wrapper design.

## Scope

- **Create**: none
- **Modify**: `packages/go/iscc.go`, `packages/go/iscc_test.go`
- **Reference**: `packages/go/iscc.go` (current hasher API),
    `crates/iscc-py/python/iscc_lib/__init__.py` (Python BinaryIO chunked-read pattern for
    reference)

## Not In Scope

- Adding `io.Reader` overloads to `GenDataCodeV0`/`GenInstanceCodeV0` one-shot functions (users
    wanting streaming from io.Reader should use the hasher + UpdateFrom)
- Implementing `io.Writer` or `io.WriterTo` interfaces
- Changing the existing `Update(ctx, []byte)` method signature
- Adding convenience constructors that accept `io.Reader` directly

## Implementation Notes

Add two `UpdateFrom` methods with identical structure:

```go
// UpdateFrom reads all data from r and feeds it into the hasher in chunks.
// Uses 64 KiB internal buffer. Returns any read or update error.
func (h *DataHasher) UpdateFrom(ctx context.Context, r io.Reader) error {
    buf := make([]byte, 64*1024)
    for {
        n, err := r.Read(buf)
        if n > 0 {
            if updateErr := h.Update(ctx, buf[:n]); updateErr != nil {
                return updateErr
            }
        }
        if err == io.EOF {
            return nil
        }
        if err != nil {
            return fmt.Errorf("iscc: read: %w", err)
        }
    }
}
```

Same pattern for `InstanceHasher.UpdateFrom`.

The 64 KiB buffer size matches the Python binding's chunked-read pattern. The method delegates to
the existing `Update` method, so all WASM memory management is handled by the existing code path.

For tests, use `bytes.NewReader` to create `io.Reader` instances from conformance test data:

1. `TestDataHasherUpdateFrom` — feed a conformance vector via `UpdateFrom(bytes.NewReader(data))`,
    verify result matches `GenDataCodeV0` one-shot result
2. `TestInstanceHasherUpdateFrom` — same pattern for InstanceHasher
3. `TestDataHasherUpdateFromMultiChunk` — use `io.LimitedReader` or a custom small-buffer reader to
    force multiple `Read` calls, verify identical result to one-shot

Add `"bytes"` to the import block in the test file if not already present.

## Verification

- `mise exec -- go test ./packages/go/ -v -run TestDataHasherUpdateFrom` passes
- `mise exec -- go test ./packages/go/ -v -run TestInstanceHasherUpdateFrom` passes
- `mise exec -- go test ./packages/go/ -v` passes all tests (36 existing + new tests)
- `grep -c 'func.*UpdateFrom' packages/go/iscc.go` returns 2
- `grep 'io.Reader' packages/go/iscc.go` finds the method signatures
- `cargo clippy --workspace --all-targets -- -D warnings` remains clean

## Done When

All verification criteria pass — both `UpdateFrom` methods exist, new tests pass, and all 36+
existing Go tests continue to pass.
