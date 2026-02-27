# Next Work Package

## Step: Implement pure Go codec module (enums, header codec, base32, component encoding)

## Goal

Begin the pure Go rewrite of the Go bindings (critical issue) by implementing the foundational codec
module — enums, varnibble header encoding/decoding, base32/base64, component encoding, ISCC
decompose, and ISCC decode. The codec is the dependency-free foundation that all subsequent Go
modules (text utils, algorithms, gen functions) will build upon.

## Scope

- **Create**: `packages/go/codec.go`, `packages/go/codec_test.go`
- **Modify**: (none — the new file coexists with existing WASM bridge code)
- **Reference**:
    - `crates/iscc-lib/src/codec.rs` — the Rust codec to port (1,541 lines)
    - `crates/iscc-lib/src/lib.rs` lines 210-230 — `iscc_decode` Tier 1 wrapper
    - `crates/iscc-lib/tests/data.json` — conformance test vectors for verification
    - `packages/go/iscc.go` — existing `DecodeResult` struct and API naming conventions

## Not In Scope

- Removing WASM infrastructure (delete binary, update .gitignore, restore large-file threshold) —
    that happens after all pure Go modules are complete and tests pass
- Modifying `iscc.go` or `iscc_test.go` — the WASM bridge stays untouched; both implementations
    coexist temporarily in the same package
- Text utilities (`text_clean`, `text_collapse`, etc.) — next step in the dependency chain
- Algorithm implementations (CDC, MinHash, SimHash, DCT, WTA-Hash)
- Any `gen_*_v0` functions
- Modifying `go.mod` — the codec uses only Go standard library (`encoding/base32`,
    `encoding/base64`, `fmt`, `strings`)

## Implementation Notes

### Enums (Go typed constants)

Port `MainType`, `SubType`, `Version` from Rust enums to Go typed constants using `iota`:

```go
type MainType uint8
const (
    MTMeta     MainType = iota // 0
    MTSemantic                 // 1
    MTContent                  // 2
    MTData                     // 3
    MTInstance                 // 4
    MTIscc                     // 5
    MTId                       // 6
    MTFlake                    // 7
)

type SubType uint8
const (
    STNone    SubType = iota // 0 (also TEXT in CC context)
    STImage                  // 1
    STAudio                  // 2
    STVideo                  // 3
    STMixed                  // 4
    STSum                    // 5
    STIsccNone               // 6
    STWide                   // 7
)
const STText = STNone // alias

type Version uint8
const ( VSV0 Version = 0 )
```

### Header encode/decode (varnibble format)

Port the varnibble encoding from `codec.rs` lines 159-308. The header uses variable-nibble encoding
where each field is 3 bits + an optional 4-bit extension. Key bit layout:

- Bits 0-2: MainType (3 bits, always)
- Bits 3-5: SubType (3 bits, always)
- Bit 6: version extension flag (0 = single nibble V0, 1 = extended)
- Bits 6-8 or 6-9: Version (variable)
- Remaining: length field (variable)
- Tail: remaining bytes after header

Use `extract_bits` / bit manipulation helpers (port `get_bit`, `extract_bits`, `bits_to_bytes` from
Rust). These are internal (unexported).

### Length encode/decode

Port `encode_length` and `decode_length` (lines 311-380). These convert between bit-lengths and
header length indices. Rules:

- Standard types (Meta through Flake): `length = (bit_length / 32) - 1`
- ISCC-CODE: `length = (bit_length / 64) - 1`
- ID type: `length = (bit_length / 8) - 1`

### Base32/Base64

- Base32: `encoding/base32.StdEncoding.WithPadding(base32.NoPadding)` — RFC 4648 uppercase, no
    padding. `decodeBase32` must uppercase input first (case-insensitive).
- Base64: `encoding/base64.RawURLEncoding` — RFC 4648 §5 URL-safe, no padding.

### Tier 1 public functions

- `EncodeBase64(data []byte) string`
- `EncodeComponent(mtype, stype uint8, version uint8, bitLength uint32, digest []byte) (string, error)`
    — rejects `MTIscc` (use `gen_iscc_code_v0` instead)
- `IsccDecompose(isccCode string) ([]string, error)` — strips "ISCC:" prefix, walks raw bytes
    extracting units
- `IsccDecode(iscc string) (*DecodeResult, error)` — reuses existing `DecodeResult` struct from
    `iscc.go` (same package). Strips "ISCC:" prefix and dashes.

### Internal helpers (unexported)

- `encodeHeader`, `decodeHeader` — varnibble format
- `encodeLength`, `decodeLength` — length index conversion
- `encodeBase32`, `decodeBase32` — base32 with no padding
- `encodeUnits`, `decodeUnits` — bitfield for ISCC-CODE optional components (needed later by
    `gen_iscc_code_v0`)

### Error handling

Use Go-idiomatic `error` returns (not panic). Define a package-level error sentinel or use
`fmt.Errorf("iscc: ...")` for error messages.

### Test strategy

Write `codec_test.go` with:

1. Round-trip tests: `encodeHeader` → `decodeHeader` for all MainType values
2. Known vector tests: decode known ISCC units from data.json and verify components
3. `EncodeComponent` → `IsccDecode` round-trips
4. `IsccDecompose` on known ISCC-CODE strings from data.json (extract from `gen_iscc_code_v0` test
    vectors)
5. Base32 encode/decode round-trips with known byte sequences
6. Edge cases: invalid input, truncated data, unknown enum values

Use `crates/iscc-lib/tests/data.json` vectors — specifically the `gen_meta_code_v0`,
`gen_data_code_v0` output ISCC strings can be decoded and re-encoded to verify codec correctness.

## Verification

- `cd packages/go && go build ./...` exits 0 (new codec code compiles alongside existing WASM code)
- `cd packages/go && go test -run TestCodec -count=1 -v` passes (all codec tests green)
- `cd packages/go && go vet ./...` exits 0
- `cd packages/go && grep -c 'func Encode\|func Iscc\|func encodeHeader\|func decodeHeader' codec.go`
    returns at least 6 (public + internal functions exist)

## Done When

All four verification commands pass, confirming the pure Go codec module compiles, is tested, and
coexists with the existing WASM bridge code without conflicts.
