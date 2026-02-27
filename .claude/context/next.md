# Next Work Package

## Step: Propagate 7 Tier 1 symbols to Go binding

## Goal

Add the 7 missing Tier 1 symbols (`encode_component`, `iscc_decode`, `json_to_data_url`, and 4
constants) to the Go/wazero binding, bringing it from 23/30 to 30/30 — completing all language
bindings.

## Scope

- **Create**: none
- **Modify**: `packages/go/iscc.go` (add constants, `DecodeResult` struct, 3 new methods)
- **Reference**:
    - `crates/iscc-ffi/src/lib.rs` — C FFI signatures for `iscc_json_to_data_url`,
        `iscc_encode_component`, `iscc_decode`, `iscc_free_decode_result`, `IsccDecodeResult` struct
        layout
    - `packages/go/iscc.go` — existing patterns: `callStringResult`, `writeString`, `writeBytes`,
        `readByteBuffer`, `freeByteBuffer`, `dealloc`
    - `packages/go/iscc_test.go` — existing test patterns
    - `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` — test vectors for the
        new symbols

## Not In Scope

- Rebuilding the WASM binary (`iscc_ffi.wasm`) — the binary already exports all FFI functions
    including the 3 new ones (it was rebuilt when C FFI was updated)
- Changing Go module path, version, or `go.mod` dependencies
- Adding Go-side `MT`/`ST`/`VS` enum types — Go constants use plain `int` (idiomatic Go)
- Updating documentation (`docs/howto/go.md`) — separate step if needed
- Refactoring existing Go bridge code

## Implementation Notes

### Constants (4 symbols)

Add package-level constants at the top of `iscc.go` (after imports, before `Runtime` struct):

```go
const (
    MetaTrimName        = 128
    MetaTrimDescription = 4096
    IoReadSize          = 4_194_304
    TextNgramSize       = 13
)
```

### `JsonToDataUrl` method

Simple string→string pattern, identical to `TextClean`:

1. `writeString` the JSON input
2. Call `iscc_json_to_data_url(ptr)` — returns a string pointer
3. Use `callStringResult` to read and free the result
4. `dealloc` the input string

### `EncodeComponent` method

Signature:
`EncodeComponent(ctx, mtype, stype, version uint8, bitLength uint32, digest []byte) (string, error)`

1. `writeBytes` the digest
2. Call `iscc_encode_component(mtype, stype, version, bit_length, digest_ptr, digest_len)` — 6 i64
    params (wazero uses uint64 for all WASM i32 values), returns string pointer
3. Use `callStringResult` to read and free the result
4. `dealloc` the digest bytes

### `IsccDecode` method — struct return via sret

This is the most complex. The C FFI function `iscc_decode` returns an `IsccDecodeResult` struct
(`#[repr(C)]`, 16 bytes on wasm32).

**WASM struct layout** (wasm32, `#[repr(C)]`):

- offset 0: `ok` (bool = 1 byte)
- offset 1: `maintype` (u8)
- offset 2: `subtype` (u8)
- offset 3: `version` (u8)
- offset 4: `length` (u8)
- offset 5–7: padding (3 bytes, alignment to 4 for IsccByteBuffer)
- offset 8: `digest.data` (u32 pointer)
- offset 12: `digest.len` (u32)
- Total: 16 bytes

**sret calling convention**: WASM uses sret for struct returns. The function signature becomes
`iscc_decode(sret_ptr, iscc_str_ptr)` where `sret_ptr` is a pre-allocated 16-byte region.

Steps:

1. Allocate 16 bytes for sret: `rt.alloc(ctx, 16)`
2. `writeString` the ISCC unit string
3. Call `iscc_decode(sret_ptr, str_ptr)` — no return value (result written to sret)
4. Read 16 bytes from `sret_ptr`
5. Parse: `ok = raw[0] != 0`, `maintype = raw[1]`, `subtype = raw[2]`, `version = raw[3]`,
    `length = raw[4]`
6. If `!ok`, read `lastError`, free sret via `dealloc(sret_ptr, 16)`, return error
7. Digest: `dataPtr = LittleEndian.Uint32(raw[8:12])`, `dataLen = LittleEndian.Uint32(raw[12:16])`
8. Read `dataLen` bytes from `dataPtr` into Go `[]byte`
9. Free: call `iscc_free_decode_result(sret_ptr)` (single i32 param — struct passed by pointer in
    WASM per learnings), then `dealloc(sret_ptr, 16)`

Define a Go `DecodeResult` struct:

```go
type DecodeResult struct {
    Maintype uint8
    Subtype  uint8
    Version  uint8
    Length   uint8
    Digest   []byte
}
```

### `iscc_free_decode_result` in WASM

Per the learnings: "WASM sret ABI for struct returns: `iscc_free_byte_buffer` and
`iscc_free_byte_buffer_array` take the struct by pointer (single i32 param) on wasm32." The same
applies to `iscc_free_decode_result` — pass the sret pointer directly.

### Tests

Add tests in `packages/go/iscc_test.go`:

1. `TestConstants` — verify all 4 constant values
2. `TestJsonToDataUrl` — test with a simple JSON object, verify `data:application/json;base64,...`
    prefix
3. `TestJsonToDataUrlLdJson` — test with `@context` key, verify
    `data:application/ld+json;base64,...`
4. `TestEncodeComponent` — encode a known Meta-Code component, verify output matches expected
5. `TestIsccDecode` — decode a known ISCC unit, verify all fields
6. `TestIsccDecodeInvalid` — decode an invalid string, verify error returned
7. `TestEncodeDecodeRoundtrip` — encode then decode, verify round-trip consistency

Use the same test vectors as Java tests (in `IsccLibTest.java`).

## Verification

- `cd packages/go && mise exec -- go test -v -run TestConstants ./...` passes
- `cd packages/go && mise exec -- go test -v -run TestJsonToDataUrl ./...` passes
- `cd packages/go && mise exec -- go test -v -run TestEncodeComponent ./...` passes
- `cd packages/go && mise exec -- go test -v -run TestIsccDecode ./...` passes
- `cd packages/go && mise exec -- go test -v -run TestEncodeDecodeRoundtrip ./...` passes
- `cd packages/go && mise exec -- go test -count=1 ./...` passes (all existing + new tests)
- `cd packages/go && mise exec -- go vet ./...` clean
- `grep -c 'MetaTrimName\|MetaTrimDescription\|IoReadSize\|TextNgramSize' packages/go/iscc.go`
    returns 4 (one per constant)
- `grep -c 'func (rt \*Runtime)' packages/go/iscc.go` returns at least 26 (23 existing methods + 3
    new)

## Done When

All verification criteria pass, confirming 30/30 Tier 1 symbols are accessible from Go with correct
behavior matching the C FFI/WASM/Java/Node.js implementations.
