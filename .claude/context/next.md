# Next Work Package

## Step: Add Go streaming hashers (DataHasher + InstanceHasher)

## Goal

Implement the 2 remaining Tier 1 streaming types in the Go binding — `DataHasher` and
`InstanceHasher` — achieving full 23/23 Tier 1 parity. These structs wrap the WASM opaque pointer
lifecycle (`new/update/finalize/free`) and provide idiomatic Go access to incremental hashing.

## Scope

- **Create**: (none)
- **Modify**: `packages/go/iscc.go`, `packages/go/iscc_test.go`
- **Reference**: `crates/iscc-ffi/src/lib.rs` (lines 983–1187: FFI streaming hasher exports),
    `packages/go/iscc.go` (existing `callStringResult`, `writeBytes`, `lastError` helpers)

## Not In Scope

- `io.Reader` convenience wrappers (e.g., `HashReader(r io.Reader)`) — that's a future ergonomic
    enhancement; this step exposes the core `New/Update/Finalize/Close` lifecycle
- Updating `packages/go/README.md` with streaming hasher documentation
- Root README Go section updates
- Documentation how-to guides (`docs/howto/go.md`)
- Adding `io.Writer` interface implementation — keep the API simple for now

## Implementation Notes

### Structs

Add two exported struct types to `iscc.go`:

```go
type DataHasher struct {
    rt  *Runtime
    ptr uint32 // opaque WASM-side FfiDataHasher pointer
}

type InstanceHasher struct {
    rt  *Runtime
    ptr uint32 // opaque WASM-side FfiInstanceHasher pointer
}
```

### Factory methods on Runtime

- `func (rt *Runtime) NewDataHasher(ctx context.Context) (*DataHasher, error)` — calls
    `iscc_data_hasher_new()`, stores the returned `uint32` pointer. Check for 0 (NULL) as error
- `func (rt *Runtime) NewInstanceHasher(ctx context.Context) (*InstanceHasher, error)` — calls
    `iscc_instance_hasher_new()`, stores the returned `uint32` pointer

### Methods on each hasher

**Update(ctx, data []byte) error:**

1. Write `data` to WASM memory via existing `writeBytes` helper
2. Call `iscc_data_hasher_update(ptr, dataPtr, dataLen)` — returns a single `uint64`
3. Interpret as bool: 0 = false (error), nonzero = true (ok)
4. On false, read `rt.lastError(ctx)` for the message
5. Deallocate the data buffer afterward (the FFI copies into its internal state)

This mirrors how `GenDataCodeV0`/`GenInstanceCodeV0` already handle their `data []byte` parameter.

**Finalize(ctx, bits uint32) (string, error):**

1. Call `iscc_data_hasher_finalize(ptr, bits)` — returns a `*mut c_char` pointer as `uint64`
2. Use the same pattern as `callStringResult`: if pointer is 0 (null), read `lastError()`; otherwise
    read the string and free it via `freeString`
3. After finalize, the WASM-side inner is consumed — subsequent Update/Finalize calls will return
    errors from the FFI layer

**Close(ctx) error:**

1. Call `iscc_data_hasher_free(ptr)` — fire-and-forget, no meaningful return
2. Set `h.ptr = 0` after free to prevent double-free (FFI treats NULL as no-op, but being explicit
    is more Go-idiomatic)
3. Safe to call multiple times

Same 3 methods on `InstanceHasher` with `iscc_instance_hasher_*` functions.

### WASM export signatures

The FFI functions for streaming hashers use simple parameter types (no sret ABI needed):

- `iscc_data_hasher_new() -> i32` (pointer)
- `iscc_data_hasher_update(hasher: i32, data: i32, data_len: i32) -> i32` (bool)
- `iscc_data_hasher_finalize(hasher: i32, bits: i32) -> i32` (string pointer)
- `iscc_data_hasher_free(hasher: i32)` (void)

Same pattern for `iscc_instance_hasher_*`.

### Tests to add in `iscc_test.go`

1. **`TestDataHasherOneShot`** — single Update + Finalize, compare result to `GenDataCodeV0` with
    the same input bytes. Use a conformance vector's hex-decoded stream data
2. **`TestDataHasherMultiChunk`** — split data at an arbitrary offset into 2 Update calls, verify
    same result as one-shot
3. **`TestDataHasherEmpty`** — Finalize with no Update calls (should produce a valid ISCC for empty
    data — matches `GenDataCodeV0` with empty bytes)
4. **`TestDataHasherDoubleFinalize`** — second Finalize returns error
5. **`TestInstanceHasherOneShot`** — same as DataHasher pattern
6. **`TestInstanceHasherMultiChunk`** — split data across 2 Update calls
7. **`TestInstanceHasherEmpty`** — Finalize with no Update calls
8. **`TestInstanceHasherDoubleFinalize`** — second Finalize returns error

For tests 1–3 and 5–7, the expected result should match the corresponding `Gen*CodeV0` function
called with the same complete data, verifying streaming equivalence.

## Verification

- `CGO_ENABLED=0 mise exec -- go test -v -count=1 ./...` in `packages/go/` passes — all existing 27
    tests plus 8 new streaming hasher tests
- `mise exec -- go vet ./...` in `packages/go/` is clean
- `grep -c 'func (rt \*Runtime)' packages/go/iscc.go` outputs ≥ 45 (43 existing + 2 new factory
    methods)
- `grep -c 'type.*Hasher struct' packages/go/iscc.go` outputs 2 (DataHasher + InstanceHasher)
- `grep -c 'func Test' packages/go/iscc_test.go` outputs ≥ 35 (27 existing + 8 new)

## Done When

All verification criteria pass — the Go binding exposes 23/23 Tier 1 symbols with full streaming
hasher support, and all tests (existing + new) pass with `CGO_ENABLED=0`.
