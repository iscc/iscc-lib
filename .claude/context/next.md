# Next Work Package

## Step: Add 9 gen\_\*\_v0 Go wrappers with conformance tests

## Goal

Implement all 9 `gen_*_v0` Go wrappers in `packages/go/iscc.go` and add conformance tests against
`data.json` vectors in `iscc_test.go`. This completes the Go binding's core function surface,
matching the target requirement that "All 9 `gen_*_v0` functions are accessible with idiomatic Go
types and error handling."

## Scope

- **Create**: (none)
- **Modify**: `packages/go/iscc.go`, `packages/go/iscc_test.go`
- **Reference**: `crates/iscc-ffi/src/lib.rs` (FFI signatures), `crates/iscc-lib/tests/data.json`
    (conformance vectors), `packages/go/iscc.go` (existing patterns)

## Not In Scope

- Remaining 12 Tier 1 function wrappers (text utilities `TextRemoveNewlines`/`TextTrim`/
    `TextCollapse`, algorithm primitives
    `SlidingWindow`/`AlgMinhash256`/`AlgCdcChunks`/`AlgSimhash`, `SoftHashVideoV0`, `EncodeBase64`,
    `IsccDecompose`, `DataHasher`/`InstanceHasher` streaming types)
- Structured return types — return `(string, error)` with just the ISCC string, matching the Node.js
    and WASM binding pattern. Structured types can be added later
- `io.Reader` support for streaming functions (Data-Code, Instance-Code) — accept `[]byte` for now
- Go CI job in `.github/workflows/ci.yml` — that's the next step
- `packages/go/README.md`
- Performance optimization (e.g., `CompileModule` once + `InstantiateModule` per test)
- Modifying any Rust code or FFI signatures

## Implementation Notes

### Memory helpers needed

The existing `writeString`/`readString`/`freeString` pattern handles string-in/string-out functions.
Three additional unexported helpers are needed for the remaining data types:

1. **`writeBytes(ctx, data []byte) (ptr, size uint32, err error)`** — allocates WASM memory, writes
    raw bytes (no null terminator). Used by `GenImageCodeV0`, `GenDataCodeV0`, `GenInstanceCodeV0`.
    For empty `data`, pass ptr=0 and size=0 to the FFI function (Rust FFI handles null/0-len as
    empty slice).

2. **`writeI32Slice(ctx, values []int32) (ptr uint32, count uint32, err error)`** — allocates
    `len*4` bytes, writes i32 values in little-endian format (wasm32 is LE). Used by
    `GenAudioCodeV0`. For empty slice, return ptr=0, count=0. Use `binary.LittleEndian.PutUint32`
    to encode each i32 as 4 bytes.

3. **`writeStringArray(ctx, strings []string) (ptrsPtr uint32, count uint32, cleanup func(), err error)`**
    — allocates individual null-terminated strings + a pointer array (array of uint32 WASM
    pointers, each pointing to a string). Returns the WASM pointer to the pointer array, the count,
    and a cleanup function that deallocs all strings and the pointer array. Used by
    `GenMixedCodeV0` and `GenIsccCodeV0`.

For `GenVideoCodeV0`, a dedicated helper is needed: allocate each frame's `[]int32` data in WASM
memory, then build two parallel arrays (pointers-to-frames and frame-lengths) and write those to
WASM memory. Consider `writeI32ArrayOfArrays(ctx, frames [][]int32)` returning pointers to the
frame-pointers array and frame-lengths array, plus a cleanup function.

### Function signatures (PascalCase, idiomatic Go)

All functions are methods on `*Runtime` taking `context.Context` as first arg:

```go
func (rt *Runtime) GenMetaCodeV0(ctx context.Context, name string, description, meta *string, bits uint32) (string, error)
func (rt *Runtime) GenTextCodeV0(ctx context.Context, text string, bits uint32) (string, error)
func (rt *Runtime) GenImageCodeV0(ctx context.Context, pixels []byte, bits uint32) (string, error)
func (rt *Runtime) GenAudioCodeV0(ctx context.Context, cv []int32, bits uint32) (string, error)
func (rt *Runtime) GenVideoCodeV0(ctx context.Context, frameSigs [][]int32, bits uint32) (string, error)
func (rt *Runtime) GenMixedCodeV0(ctx context.Context, codes []string, bits uint32) (string, error)
func (rt *Runtime) GenDataCodeV0(ctx context.Context, data []byte, bits uint32) (string, error)
func (rt *Runtime) GenInstanceCodeV0(ctx context.Context, data []byte, bits uint32) (string, error)
func (rt *Runtime) GenIsccCodeV0(ctx context.Context, codes []string) (string, error)
```

Use `*string` for optional `description` and `meta` in `GenMetaCodeV0` — `nil` maps to NULL pointer
in FFI. `GenIsccCodeV0` has no `bits` parameter; the FFI `wide` bool defaults to `false` (all test
vectors use standard width).

### FFI function names and signatures

From `crates/iscc-ffi/src/lib.rs`:

| Go wrapper          | FFI function                | FFI params                                                                    |
| ------------------- | --------------------------- | ----------------------------------------------------------------------------- |
| `GenMetaCodeV0`     | `iscc_gen_meta_code_v0`     | `name *c_char, description *c_char, meta *c_char, bits u32` → `*c_char`       |
| `GenTextCodeV0`     | `iscc_gen_text_code_v0`     | `text *c_char, bits u32` → `*c_char`                                          |
| `GenImageCodeV0`    | `iscc_gen_image_code_v0`    | `pixels *u8, pixels_len usize, bits u32` → `*c_char`                          |
| `GenAudioCodeV0`    | `iscc_gen_audio_code_v0`    | `cv *i32, cv_len usize, bits u32` → `*c_char`                                 |
| `GenVideoCodeV0`    | `iscc_gen_video_code_v0`    | `frame_sigs **i32, frame_lens *usize, num_frames usize, bits u32` → `*c_char` |
| `GenMixedCodeV0`    | `iscc_gen_mixed_code_v0`    | `codes **c_char, num_codes usize, bits u32` → `*c_char`                       |
| `GenDataCodeV0`     | `iscc_gen_data_code_v0`     | `data *u8, data_len usize, bits u32` → `*c_char`                              |
| `GenInstanceCodeV0` | `iscc_gen_instance_code_v0` | `data *u8, data_len usize, bits u32` → `*c_char`                              |
| `GenIsccCodeV0`     | `iscc_gen_iscc_code_v0`     | `codes **c_char, num_codes usize, wide bool` → `*c_char`                      |

### Call pattern (same as existing TextClean)

Each wrapper follows: marshal args → call FFI → check NULL result (ptr==0) → readString → freeString
→ return. On NULL result, call `lastError()` for the error message.

### WASM pointer size

WASM32 uses 4-byte pointers. When building pointer arrays (for string arrays and video frame
pointers), each element is a `uint32`. Use `binary.LittleEndian.PutUint32` to write pointer values
into the array buffer.

### Conformance tests

Read `../../crates/iscc-lib/tests/data.json` via `os.ReadFile` (relative path from test working
directory). Parse with `encoding/json`. Use Go subtests `t.Run(vectorName, ...)` for readable
output.

Special handling per function type:

- **String functions** (meta, text): JSON inputs are strings. For meta, `inputs[2]` is `null` (JSON)
    for no metadata → pass `nil` for `*string`. Empty `description` (`""`) should be passed as a
    pointer to an empty string (not nil).
- **Byte array functions** (image, data, instance): For image, `inputs[0]` is `[]float64` from JSON
    → convert each to `byte`. For data/instance, `inputs[0]` has `"stream:"` prefix — strip prefix,
    hex-decode the remainder with `encoding/hex`. Empty after prefix = empty `[]byte`.
- **i32 array function** (audio): `inputs[0]` is `[]float64` → convert each to `int32`.
- **Nested i32 arrays** (video): `inputs[0]` is `[][]float64` → convert to `[][]int32`.
- **String array functions** (mixed, iscc): `inputs[0]` is `[]interface{}` → convert each to string.
- **Bits parameter**: JSON numbers decode as `float64` in Go — cast to `uint32`.
- **iscc_code**: no bits param; `inputs` is just `[codes_array]`.

Expected output: `outputs["iscc"]` (string). Assert equality with Go wrapper return value.

Total conformance vectors: 46 (16 meta + 5 text + 3 image + 5 audio + 3 video + 2 mixed + 4 data + 3
instance + 5 iscc).

## Verification

- `cd packages/go && CGO_ENABLED=0 go test -v -count=1 ./...` passes all tests (existing 5 + new
    conformance tests covering 46 vectors across 9 gen functions)
- `cd packages/go && go vet ./...` exits 0
- All 9 `Gen*CodeV0` methods exist on `*Runtime` type
- Each conformance test vector produces an ISCC string matching the expected output from `data.json`

## Done When

All 9 `Gen*CodeV0` Go wrappers are implemented and all conformance test vectors from `data.json`
pass via `go test ./...`.
