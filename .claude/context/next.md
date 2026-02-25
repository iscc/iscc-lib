# Next Work Package

## Step: Add 6 Go wrappers (text utils + encode + string-array functions)

## Goal

Implement the 6 simplest remaining Go wrappers — 3 text utilities, `EncodeBase64`, `SlidingWindow`,
and `IsccDecompose` — cutting the Go Tier 1 gap from 12 to 6 functions. These are all
string-returning or string-array-returning, building on established patterns without requiring byte
buffer or opaque pointer infrastructure.

## Scope

- **Create**: (none)
- **Modify**: `packages/go/iscc.go`, `packages/go/iscc_test.go`
- **Reference**: `crates/iscc-ffi/src/lib.rs` (FFI signatures at lines 605–755 and 1216–1230),
    `packages/go/iscc.go` (existing `TextClean` and `callStringResult` patterns)

## Not In Scope

- Byte-buffer-returning functions (`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `SoftHashVideoV0`)
    — these need new `IsccByteBuffer` read/free helpers, save for next step
- Streaming hashers (`DataHasher`, `InstanceHasher`) — need opaque pointer lifecycle management
- Root README Go section or `docs/howto/go.md` — documentation follows after all wrappers land
- Updating `packages/go/README.md` API table — wait until all 23 functions exist

## Implementation Notes

### Group A: Simple string-returning (copy `TextClean` pattern exactly)

1. **`TextRemoveNewlines(ctx, text string) (string, error)`** — calls
    `iscc_text_remove_newlines(text_ptr)`. Identical to `TextClean`: writeString → call →
    callStringResult.

2. **`TextCollapse(ctx, text string) (string, error)`** — calls `iscc_text_collapse(text_ptr)`. Same
    pattern.

3. **`TextTrim(ctx, text string, nbytes uint32) (string, error)`** — calls
    `iscc_text_trim(text_ptr, nbytes)`. Like `TextClean` but with an extra `uint64(nbytes)` arg.

4. **`EncodeBase64(ctx, data []byte) (string, error)`** — calls
    `iscc_encode_base64(data_ptr, data_len)`. Use `writeBytes` (existing helper) for the input,
    then `callStringResult` for the output.

### Group B: String-array-returning (new `readStringArray`/`freeStringArray` helpers)

- **`SlidingWindow(ctx, seq string, width uint32) ([]string, error)`** — calls
    `iscc_sliding_window(seq_ptr, width)`. Returns a null-terminated C string pointer array in WASM
    memory.

- **`IsccDecompose(ctx, isccCode string) ([]string, error)`** — calls
    `iscc_decompose(iscc_code_ptr)`. Same return pattern.

### New memory helpers needed

- **`readStringArray(ctx, ptr uint32) ([]string, error)`** — reads u32 pointers from WASM memory
    starting at `ptr` until hitting 0 (null terminator). For each non-zero u32, calls `readString`
    to get the Go string. Returns the collected `[]string`. In WASM32, pointers are 4 bytes
    (little-endian u32).

- **`freeStringArray(ctx, ptr uint32) error`** — calls `iscc_free_string_array(ptr)` to free the
    entire array (strings + outer pointer array). Must be called after `readStringArray` reads all
    strings but before returning to the caller.

- **`callStringArrayResult(ctx, fnName string, results []uint64) ([]string, error)`** — combines
    null-check, readStringArray, freeStringArray into a reusable pattern (mirrors `callStringResult`
    for single strings). Check if `results[0] == 0` → lastError, otherwise read + free + return.

### Test strategy

Unit tests with known inputs (no conformance vectors needed — `data.json` only covers gen\_\*\_v0):

- `TextRemoveNewlines`: "hello\\nworld" → "hello world"
- `TextTrim`: long string with nbytes=10 → trimmed result
- `TextCollapse`: "Hello, World!" → "helloworld" (lowercased, punctuation removed)
- `EncodeBase64`: known bytes → expected base64url string (e.g., `[]byte{0,1,2}` → "AAEC")
- `SlidingWindow`: "ABCDE" with width=3 → `["ABC", "BCD", "CDE"]`
- `SlidingWindow`: width < 2 → error (validates error propagation)
- `IsccDecompose`: use an ISCC-CODE from conformance vectors → expected units

### WASM32 memory layout for string arrays

The FFI returns a pointer to a null-terminated array of C string pointers. In WASM32:

- Outer pointer is u32 (4 bytes)
- Array contains u32 pointers, each 4 bytes, terminated by a zero u32
- Each string pointer leads to a null-terminated UTF-8 byte sequence
- `iscc_free_string_array` frees each string then the outer array

## Verification

- `CGO_ENABLED=0 go test -v -count=1 ./...` in `packages/go/` passes (14 existing + new tests)
- `go vet ./...` in `packages/go/` is clean
- `grep -c 'func (rt \*Runtime)' packages/go/iscc.go` outputs 17 (11 existing + 6 new)
- `grep 'TextRemoveNewlines\|TextTrim\|TextCollapse\|EncodeBase64\|SlidingWindow\|IsccDecompose' packages/go/iscc.go | wc -l`
    ≥ 6

## Done When

All 4 verification commands pass — the 6 new Go wrappers are implemented with tests, existing 14
tests still pass, and `go vet` is clean.
