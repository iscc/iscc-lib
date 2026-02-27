# Next Work Package

## Step: Implement pure Go GenMetaCodeV0 and GenTextCodeV0

## Goal

Port the first two `gen_*_v0` functions to pure Go, completing the first sub-step of the gen
functions layer (step 5). These functions depend only on already-ported algorithms (SimHash,
MinHash, text utils, codec) plus BLAKE3 and xxh32, enabling conformance validation against 21
data.json vectors (16 meta + 5 text).

## Scope

- **Create**:
    - `packages/go/xxh32.go` — xxh32 hash function (standalone, ~50 lines)
    - `packages/go/code_content_text.go` — `GenTextCodeV0`, `softHashTextV0`, `TextCodeResult`
    - `packages/go/code_meta.go` — `GenMetaCodeV0`, `softHashMetaV0`, `softHashMetaV0WithBytes`,
        `metaNameSimhash`, `interleaveDigests`, `slidingWindowBytes`, `decodeDataURL`,
        `parseMetaJson`, `buildMetaDataURL`, `multiHashBlake3`, `MetaCodeResult`
    - `packages/go/xxh32_test.go` — xxh32 unit tests
    - `packages/go/code_content_text_test.go` — GenTextCodeV0 conformance tests (5 vectors)
    - `packages/go/code_meta_test.go` — GenMetaCodeV0 conformance tests (16 vectors)
- **Modify**:
    - `packages/go/go.mod` / `packages/go/go.sum` — add `github.com/zeebo/blake3` dependency + a JCS
        (RFC 8785 JSON canonicalization) library
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines 50–413 — Rust gen_meta_code_v0, gen_text_code_v0,
        soft_hash_meta_v0, soft_hash_text_v0, interleave_digests, meta helpers
    - `crates/iscc-lib/src/utils.rs` lines 148–158 — multi_hash_blake3
    - `crates/iscc-lib/src/simhash.rs` lines 124–140 — sliding_window_bytes
    - `crates/iscc-lib/tests/data.json` — conformance vectors (gen_meta_code_v0: 16, gen_text_code_v0:
        5\)
    - `packages/go/simhash.go` — existing AlgSimhash, SlidingWindow
    - `packages/go/minhash.go` — existing AlgMinhash256
    - `packages/go/codec.go` — existing EncodeComponent and type constants
    - `packages/go/utils.go` — existing TextClean, TextRemoveNewlines, TextTrim, TextCollapse

## Not In Scope

- Other gen functions (gen_image, gen_audio, gen_video, gen_mixed, gen_data, gen_instance, gen_iscc)
    — those come in subsequent sub-steps
- `DataHasher` / `InstanceHasher` streaming types
- `conformance_selftest` function — needs all 9 gen functions
- Modifying the existing WASM bridge (`iscc.go`) or its tests (`iscc_test.go`)
- Removing WASM artifacts or wazero dependency — cleanup is the final step
- Exporting `JsonToDataUrl` as a public function — it exists in the WASM bridge; the pure Go version
    is an internal helper in code_meta.go for now

## Implementation Notes

### xxh32 (packages/go/xxh32.go)

Implement xxh32 (32-bit xxHash) as a standalone unexported function
`xxh32(data []byte, seed uint32) uint32`. This is a well-known algorithm (~50 lines): prime
constants, 4-lane accumulation for inputs ≥16 bytes, remaining bytes processing, final avalanche
mixing. Reference: xxHash specification or the Rust `xxhash_rust::xxh32` crate. Verify against known
test vectors (e.g., `xxh32([]byte{}, 0) == 0x02CC5D05`).

### GenTextCodeV0 (packages/go/code_content_text.go)

Pattern from Rust (`lib.rs` lines 398–413):

```go
func GenTextCodeV0(text string, bits uint32) (*TextCodeResult, error) {
    collapsed := TextCollapse(text)
    characters := len([]rune(collapsed))
    hashDigest := softHashTextV0(collapsed)
    component, err := EncodeComponent(MTContent, STText, VSV0, bits, hashDigest)
    if err != nil { return nil, err }
    return &TextCodeResult{Iscc: "ISCC:" + component, Characters: characters}, nil
}
```

`softHashTextV0`: `SlidingWindow(text, 13)` → xxh32 each n-gram's UTF-8 bytes with seed 0 →
`AlgMinhash256(features)`.

Return type `TextCodeResult` struct: `Iscc string`, `Characters int`.

### GenMetaCodeV0 (packages/go/code_meta.go)

This is the most complex gen function. Follow the Rust implementation in `lib.rs` lines 276–378
exactly. Key functions to port:

1. **`metaNameSimhash(name string) []byte`** — `TextCollapse(name)` → `SlidingWindow(collapsed, 3)`
    → BLAKE3 hash each n-gram (producing 32-byte digests) → `AlgSimhash(hashes)`. Uses **width-3**
    n-grams (NOT the TextNgramSize=13 used by text code).

2. **`softHashMetaV0(name string, extra *string) []byte`** — `metaNameSimhash` for name. If extra
    is non-nil and non-empty: `TextCollapse(extra)` → `SlidingWindow(3)` → BLAKE3 each →
    `AlgSimhash` → `interleaveDigests(nameSimhash, extraSimhash)`. Otherwise return nameSimhash.

3. **`softHashMetaV0WithBytes(name string, extra []byte) []byte`** — `metaNameSimhash` for name. If
    extra is non-empty: `slidingWindowBytes(extra, 4)` → BLAKE3 each → `AlgSimhash` →
    `interleaveDigests`. Otherwise return nameSimhash.

4. **`interleaveDigests(a, b []byte) []byte`** — take first 16 bytes of each 32-byte digest,
    interleave in 4-byte chunks:
    `a[0:4] || b[0:4] || a[4:8] || b[4:8] || ... || a[12:16] ||  b[12:16]` → 32 bytes total.

5. **`slidingWindowBytes(data []byte, width int) [][]byte`** — byte-level sliding window. If
    `len(data) < width`, return `[][]byte{data}`. Otherwise `max(len-width+1, 1)` windows.

6. **`decodeDataURL(dataURL string) ([]byte, error)`** — split on first `,`, standard base64 decode
    the remainder.

7. **`parseMetaJson(metaStr string) ([]byte, error)`** — JSON parse → JCS (RFC 8785) canonical
    bytes. Use a JCS library. The conformance vectors test `{"some": "object"}` and
    `{"@context": "https://schema.org", "type": "..."}`.

8. **`buildMetaDataURL(jsonBytes []byte, hasContext bool) string`** — format
    `data:{mediaType};base64,{b64}` where mediaType is `application/ld+json` if hasContext, else
    `application/json`.

9. **`multiHashBlake3(data []byte) string`** — `blake3.Sum256(data)` → prepend `[0x1e, 0x20]`
    multihash prefix → hex encode the 34-byte result.

10. **`GenMetaCodeV0(name string, description, meta *string, bits uint32) (*MetaCodeResult, error)`**
    — orchestrates all of the above. Follow the two-branch pattern from Rust: meta-bytes path vs.
    description-text path.

Return type `MetaCodeResult` struct: `Iscc string`, `Name string`, `Description string` (empty if
none), `Meta string` (empty if none), `Metahash string`.

### Conformance test inputs

Data.json uses positional arrays for inputs. For `gen_meta_code_v0`:
`[name, description, meta, bits]` where meta can be `null`, a JSON object (dict), or a string (data
URL). For `gen_text_code_v0`: `[text, bits]`.

When the `meta` field is a JSON object (dict) in the test vector, the Go test must marshal it to a
JSON string before passing to `GenMetaCodeV0` (since the Go function takes `*string`, not an
arbitrary interface). This matches how the existing WASM bridge tests handle it.

Tests should validate the `iscc` field from `outputs`. Also validate `metahash`, `name`,
`description`, `meta` fields — they're in the conformance vectors and our MetaCodeResult includes
them.

### Codec constants

Use existing constants from `codec.go`. The MainType/SubType/Version integer constants should
already exist: `MTMeta=0`, `MTContent=2`, `STNone=0`, `STText=0`, `VSV0=0`. If specific constant
names don't exist, check the codec.go definitions and use what's there.

### BLAKE3 dependency

Use `github.com/zeebo/blake3` (mentioned in target spec, SIMD-optimized, pure Go). Import as
`blake3 "github.com/zeebo/blake3"`. The `Sum256(data)` function returns `[32]byte`. Add via
`cd packages/go && go get github.com/zeebo/blake3`.

### JCS dependency

For RFC 8785 JSON canonicalization, find a suitable Go library. Options:

- `github.com/nicktrav/jcs` — if available
- `github.com/cyberphone/json-canonicalization` — reference implementation by RFC author
- Minimal inline implementation (~30 lines): parse JSON → `json.Marshal` with sorted keys. Go's
    `encoding/json` already sorts map keys, but JCS also specifies specific number formatting and
    Unicode escaping. For the simple string-only conformance vectors, `json.Marshal` may suffice

Choose whichever approach passes all 16 meta conformance vectors.

## Verification

- `cd packages/go && go build ./...` exits 0
- `cd packages/go && go test -run TestXxh32 -count=1 -v` — xxh32 unit tests pass
- `cd packages/go && go test -run TestGenTextCodeV0 -count=1 -v` — 5 text conformance vectors pass
- `cd packages/go && go test -run TestGenMetaCodeV0 -count=1 -v` — 16 meta conformance vectors pass
- `cd packages/go && go vet ./...` exits 0
- `cd packages/go && go test ./...` — all tests pass (pure Go + existing WASM bridge tests)
- `grep -c 'func GenTextCodeV0' packages/go/code_content_text.go` returns 1
- `grep -c 'func GenMetaCodeV0' packages/go/code_meta.go` returns 1
- `mise run check` — all pre-commit/pre-push hooks pass

## Done When

All verification criteria pass — both gen functions produce ISCC codes matching all 21 conformance
vectors, all existing tests continue to pass, and code is clean.
