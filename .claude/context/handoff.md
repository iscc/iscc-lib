## 2026-02-27 — Implement pure Go codec module (enums, header codec, base32, component encoding)

**Done:** Created `packages/go/codec.go` with the complete pure Go codec module — type enums
(MainType, SubType, Version as typed constants), varnibble header encoding/decoding, base32/base64
encoding, length encoding/decoding, unit encoding/decoding, `EncodeComponent`, `IsccDecompose`, and
`IsccDecode`. All functions use only Go standard library (no external dependencies). The module
coexists cleanly with the existing WASM bridge code in `iscc.go`.

**Files changed:**

- `packages/go/codec.go`: New file — pure Go codec implementation (~370 lines). Exports:
    `EncodeBase64`, `EncodeComponent`, `IsccDecompose`, `IsccDecode`, plus type constants
    (`MTMeta`..`MTFlake`, `STNone`..`STWide`, `VSV0`). Internal helpers: `encodeHeader`,
    `decodeHeader`, `encodeLength`, `decodeLength`, `encodeBase32`, `decodeBase32`, `encodeUnits`,
    `decodeUnits`, and bit manipulation functions.
- `packages/go/codec_test.go`: New file — 48 test functions covering varnibble roundtrips, header
    encode/decode for all MainTypes, length encode/decode with roundtrips, base32/base64 encoding,
    unit encode/decode, EncodeComponent with known vectors, IsccDecode with conformance vectors from
    data.json, IsccDecompose with conformance vectors, edge cases (invalid input, truncated data,
    dashes).

**Verification:**

- `cd packages/go && go build ./...` → exits 0 ✓
- `cd packages/go && go test -run TestCodec -count=1 -v` → 48 tests PASS ✓
- `cd packages/go && go vet ./...` → exits 0 ✓
- `grep -c 'func Encode\|func Iscc\|func encodeHeader\|func decodeHeader' codec.go` → 6 ✓
- `mise run check` → all 14 hooks pass ✓

**Next:** Implement `packages/go/utils.go` — text normalization (NFKC/NFD), cleaning, trimming,
collapsing. This is the next module in the dependency chain (codec → text utils → algorithms → gen
functions). The text utils will need `golang.org/x/text/unicode/norm` as the first external
dependency.

**Notes:** The `DecodeResult` struct from `iscc.go` is reused directly by `IsccDecode` — no
duplication. The existing WASM bridge `iscc.go` and `iscc_test.go` are untouched. The codec test
file independently loads `data.json` conformance vectors (relative path
`../../crates/iscc-lib/tests/data.json`) for verification without depending on the WASM runtime.
