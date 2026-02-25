# Next Work Package

## Step: Add 4 byte-buffer Go wrappers (AlgSimhash, AlgMinhash256, AlgCdcChunks, SoftHashVideoV0)

## Goal

Implement the 4 algorithm-primitive Go wrappers that return byte data (`[]byte` or `[][]byte`),
bringing the Go binding from 17/23 to 21/23 Tier 1 symbols. These share a common pattern: call FFI →
read struct from WASM memory → copy bytes to Go → free via FFI.

## Scope

- **Create**: (none)
- **Modify**: `packages/go/iscc.go`, `packages/go/iscc_test.go`
- **Reference**: `crates/iscc-ffi/src/lib.rs` (FFI signatures at lines 782–975, free functions at
    lines 1244–1275), `packages/go/iscc.go` (existing helpers: `writeI32Slice`, `writeBytes`,
    `writeI32ArrayOfArrays`)

## Not In Scope

- Streaming hashers (`DataHasher`/`InstanceHasher`) — they need opaque pointer lifecycle, a
    different pattern; save for next step
- `io.Reader` support on the Go side — that wraps `DataHasher`/`InstanceHasher`
- Root README Go section or docs updates
- Changing the FFI crate's struct layout or function signatures
- Benchmarking Go wrappers

## Implementation Notes

### WASM ABI for struct returns — CRITICAL

`IsccByteBuffer` is `{ *mut u8: i32, len: usize: i32 }` on wasm32 — 8 bytes total.
`IsccByteBufferArray` is `{ *mut IsccByteBuffer: i32, count: usize: i32 }` — also 8 bytes.

The Rust wasm32-wasip1 ABI for functions returning structs uses an **sret (structure return)
pointer**: the caller allocates space in WASM memory and passes a pointer as the **first hidden
parameter**. The function writes the struct to that address and returns nothing (or returns the same
pointer). The advance agent MUST verify this by inspecting the actual WASM export signatures before
writing any wrapper code:

```go
// Check the actual exported function signature:
fn := rt.mod.ExportedFunction("iscc_alg_minhash_256")
// Inspect fn.Definition().ParamTypes() and fn.Definition().ResultTypes()
```

If the ABI uses sret, the calling pattern is:

1. `iscc_alloc(8)` to get an sret pointer for `IsccByteBuffer`
2. Call `iscc_alg_minhash_256(sretPtr, featuresPtr, featuresLen)` — sret pointer is arg 0
3. Read `data_ptr` (i32) and `len` (i32) from `sretPtr` in WASM memory
4. Copy `len` bytes from `data_ptr` to a Go `[]byte`
5. Call `iscc_free_byte_buffer(data_ptr, len)` — pass the struct fields as two i32 args
6. `iscc_dealloc(sretPtr, 8)` to free the sret allocation

Similarly, `iscc_free_byte_buffer` takes `IsccByteBuffer` **by value**. On wasm32, this is typically
lowered to two i32 parameters `(data_ptr, len)`. Verify with
`rt.mod.ExportedFunction("iscc_free_byte_buffer").Definition().ParamTypes()`.

**Alternative ABI**: If the compiler returns the struct packed as a single `i64` (two i32s), the
pattern is different — extract high/low halves. Check `ResultTypes()` to determine which ABI is
used. Do NOT assume — verify empirically.

### New private helpers needed

1. **`readByteBuffer(ctx, sretPtr uint32) (dataPtr uint32, dataLen uint32, err error)`** — reads the
    two i32 fields of `IsccByteBuffer` from WASM memory at the given address. Checks if
    `dataPtr == 0` (null = error) and falls back to `lastError`.

2. **`freeByteBuffer(ctx, dataPtr, dataLen uint32) error`** — calls `iscc_free_byte_buffer` passing
    the struct fields. No-op if `dataPtr == 0`.

3. **`callByteBufferResult(ctx, fnName string, sretPtr uint32) ([]byte, error)`** — orchestrates:
    read struct from sretPtr → check null → copy bytes from WASM → free byte buffer → dealloc sret
    → return Go `[]byte`. This is the analog of `callStringResult` for byte buffer returns.

4. **`readByteBufferArray(ctx, sretPtr uint32) (buffersPtr uint32, count uint32, err error)`** —
    reads the two fields of `IsccByteBufferArray`.

5. **`freeByteBufferArray(ctx, buffersPtr, count uint32) error`** — calls
    `iscc_free_byte_buffer_array` passing the struct fields.

6. **`writeU32Slice(ctx, values []uint32) (ptr, allocSize, count uint32, err error)`** — needed for
    `AlgMinhash256` which takes `*const u32` features. Follows the exact same pattern as
    `writeI32Slice` but with `uint32` values (the encoding is identical since both are 4 bytes, but
    the Go types differ).

### Input helper for AlgSimhash byte-array-of-arrays

`AlgSimhash` takes `*const *const u8` (array of byte pointers) + `*const usize` (array of lengths) +
count. This is similar to the existing `writeI32ArrayOfArrays` but for `[][]byte`. Write a
`writeByteArrayOfArrays(ctx, digests [][]byte) (dataPtrsPtr, dataLensPtr, count uint32, cleanup func(), err error)`
helper following the same allocate-pointers+lengths-in-WASM pattern.

### Public wrappers

1. **`AlgSimhash(ctx, digests [][]byte) ([]byte, error)`** — build pointer+length arrays via
    `writeByteArrayOfArrays`, allocate sret, call
    `iscc_alg_simhash(sretPtr, digestsPtr, lensPtr,  count)`, read via `callByteBufferResult`.
    Returns SimHash bytes (length matches input digest length).

2. **`AlgMinhash256(ctx, features []uint32) ([]byte, error)`** — write u32 features via
    `writeU32Slice`, allocate sret, call FFI, read byte buffer result. Always returns 32 bytes.

3. **`AlgCdcChunks(ctx, data []byte, utf32 bool, avgChunkSize uint32) ([][]byte, error)`** — write
    input bytes via `writeBytes`, allocate sret (8 bytes for `IsccByteBufferArray`), call
    `iscc_alg_cdc_chunks(sretPtr, dataPtr, dataLen, utf32, avgChunkSize)`, read buffer array from
    sret, iterate: each `IsccByteBuffer` is at `buffersPtr + i*8` (8 bytes per struct on wasm32),
    copy each chunk's bytes to Go, then call `freeByteBufferArray`.

4. **`SoftHashVideoV0(ctx, frameSigs [][]int32, bits uint32) ([]byte, error)`** — reuse existing
    `writeI32ArrayOfArrays` helper, allocate sret, call FFI, read byte buffer result.

### Tests

Add tests with known inputs (algorithm primitives don't have dedicated conformance vectors — the gen
function conformance tests exercise them indirectly):

- `TestAlgSimhash` — pass 2+ digests of equal length (e.g., 4-byte digests), verify output length
    equals input digest length
- `TestAlgMinhash256` — pass known `[]uint32` features, verify output is exactly 32 bytes
- `TestAlgCdcChunks` — pass known data (e.g., 2048+ bytes), verify: returns ≥1 chunk, concatenation
    of all chunks equals the original input data
- `TestSoftHashVideoV0` — pass 2+ frame signatures (380-element `[]int32` each), verify output
    length = bits/8
- `TestSoftHashVideoV0Error` — pass empty frames → error (validates error propagation for byte
    buffer returns)
- `TestAlgCdcChunksEmpty` — pass empty data → returns 1 chunk of empty bytes

## Verification

- `CGO_ENABLED=0 go test -v -count=1 ./...` in `packages/go/` passes — all tests PASS (22 existing +
    new tests)
- `go vet ./...` in `packages/go/` is clean
- `grep 'AlgSimhash\|AlgMinhash256\|AlgCdcChunks\|SoftHashVideoV0' packages/go/iscc.go | wc -l`
    outputs ≥ 4 (public method definitions exist)
- `grep -c 'func (rt \*Runtime)' packages/go/iscc.go` outputs ≥ 38 (32 existing + ~6 new helpers and
    public methods)
- `mise run check` passes (all pre-commit hooks clean)

## Done When

All 4 byte-buffer-returning Go wrappers are implemented with tests, `go test` passes, and `go vet`
is clean.
