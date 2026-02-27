# Next Work Package

## Step: Implement pure Go GenVideoCodeV0 and GenMixedCodeV0

## Goal

Implement the 7th and 8th gen functions for the pure Go rewrite: `GenVideoCodeV0` (video content
fingerprinting via WTA-Hash) and `GenMixedCodeV0` (multi-code SimHash combination). After this step,
only `GenIsccCodeV0` remains before all 9 gen functions are complete.

## Scope

- **Create**:
    - `packages/go/code_content_video.go` — `GenVideoCodeV0`, `SoftHashVideoV0`, `VideoCodeResult`
    - `packages/go/code_content_video_test.go` — conformance tests (3 vectors)
    - `packages/go/code_content_mixed.go` — `GenMixedCodeV0`, `softHashCodesV0`, `MixedCodeResult`
    - `packages/go/code_content_mixed_test.go` — conformance tests (2 vectors)
- **Modify**: (none)
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines 636–756 — Rust `soft_hash_video_v0`, `gen_video_code_v0`,
        `soft_hash_codes_v0`, `gen_mixed_code_v0`
    - `packages/go/code_content_audio.go` — pattern for gen function structure, `arraySplit` helper
    - `packages/go/code_content_image.go` — pattern for result structs, DCT usage
    - `packages/go/wtahash.go` — `AlgWtahash` (exported, takes `[]int64`)
    - `packages/go/simhash.go` — `AlgSimhash` (exported)
    - `packages/go/codec.go` — `decodeHeader`, `decodeBase32`, `decodeLength` (unexported)

## Not In Scope

- `GenIsccCodeV0` — that's the 9th and final gen function, a separate step
- `conformance_selftest` — comes after all 9 gen functions are done
- WASM bridge removal or cleanup — only after full conformance selftest passes
- Modifying existing files (no refactoring of codec.go, simhash.go, etc.)
- Adding an unexported `algSimhashInner` to Go — use existing `AlgSimhash` and discard the error
    (validation is trivially satisfied since all digests passed to it are equal length)

## Implementation Notes

### GenVideoCodeV0 (`code_content_video.go`)

Port from Rust `soft_hash_video_v0` + `gen_video_code_v0` (lib.rs lines 636–683):

1. **`SoftHashVideoV0(frameSigs [][]int32, bits uint32) ([]byte, error)`** (exported, matching Rust
    `pub fn soft_hash_video_v0`):

    - Validate `frameSigs` is non-empty → return error if empty
    - Deduplicate frame signatures. Rust uses `BTreeSet<&S>` (ordered dedup). For Go, convert each
        `[]int32` sig to a string key for a map (e.g., `fmt.Sprint(sig)` or manual byte encoding).
        Order doesn't matter for column-wise sum — just need unique sigs
    - Column-wise sum into `[]int64` to avoid overflow. Each frame is 380 `int32` values. Sum over
        all unique frames: `vecsum[c] += int64(sig[c])`
    - Call `AlgWtahash(vecsum, bits)` to produce the digest
    - Return digest bytes

2. **`GenVideoCodeV0(frameSigs [][]int32, bits uint32) (*VideoCodeResult, error)`**:

    - Call `SoftHashVideoV0` to get digest
    - Call `EncodeComponent(uint8(MTContent), uint8(STVideo), uint8(VSV0), bits, digest)`
    - Return `&VideoCodeResult{Iscc: "ISCC:" + component}`

3. **`VideoCodeResult`** struct with `Iscc string` field

4. **Deduplication approach**: A simple string-keyed map works. Convert each `[]int32` sig to a
    canonical string key (e.g., `fmt.Sprintf("%v", sig)` or serialize to bytes), store in
    `map[string][]int32`. Then iterate the map values for the column-wise sum. Alternatively, sort
    the sigs and skip consecutive equal ones — but map is simpler

5. **Conformance vector format**: `inputs[0]` is array of arrays (frame sigs, each 380 int32
    values), `inputs[1]` is bits. `outputs` has `iscc` string. 3 vectors total

### GenMixedCodeV0 (`code_content_mixed.go`)

Port from Rust `soft_hash_codes_v0` + `gen_mixed_code_v0` (lib.rs lines 685–756):

1. **`softHashCodesV0(ccDigests [][]byte, bits uint32) ([]byte, error)`** (unexported, matching Rust
    `fn soft_hash_codes_v0` which is not `pub`):

    - Validate at least 2 digests → error if < 2
    - `nbytes := bits / 8`
    - For each raw digest:
        - Call `decodeHeader(raw)` to get `(mtype, stype, _, blen, body, err)`
        - Validate `mtype == MTContent` → error if not
        - Call `decodeLength(mtype, blen, stype)` to get `unitBits`
        - Validate `unitBits >= bits` → error if too short
        - Build entry of `nbytes` length: first byte = `raw[0]` (header byte preserves type info), then
            `min(nbytes-1, len(body))` bytes from body, zero-pad remainder
    - Call `AlgSimhash(prepared)` — all entries are `nbytes` long, so validation passes
    - Return result (discard error since lengths are guaranteed equal)

2. **`GenMixedCodeV0(codes []string, bits uint32) (*MixedCodeResult, error)`**:

    - For each code string: strip `"ISCC:"` prefix if present, then `decodeBase32(clean)`
    - Collect all decoded bytes
    - Call `softHashCodesV0(decoded, bits)` to get digest
    - Call `EncodeComponent(uint8(MTContent), uint8(STMixed), uint8(VSV0), bits, digest)`
    - Return `&MixedCodeResult{Iscc: "ISCC:" + component, Parts: codes}` (copy input slice)

3. **`MixedCodeResult`** struct with `Iscc string` and `Parts []string` fields

4. **Conformance vector format**: `inputs[0]` is array of strings (ISCC codes without "ISCC:"
    prefix), `inputs[1]` is bits. `outputs` has `iscc` string and `parts` array. 2 vectors total

### Test files

Follow the pattern from `code_content_audio_test.go`:

- **`TestPureGoGenVideoCodeV0`**: Parse `gen_video_code_v0` vectors from data.json. Video
    `inputs[0]` is array of arrays of float64 (JSON numbers) → convert each inner array to
    `[]int32`. `inputs[1]` is bits. Compare output `Iscc` to expected
- **`TestPureGoGenMixedCodeV0`**: Parse `gen_mixed_code_v0` vectors from data.json. Mixed
    `inputs[0]` is array of strings (codes). `inputs[1]` is bits. Compare both `Iscc` and `Parts`

### Key details

- `AlgWtahash` takes `[]int64` — the column sums are already `int64`, direct match
- `AlgSimhash` returns `([]byte, error)` — safe to discard error with `_, _` in `softHashCodesV0`
    since all prepared entries have identical length (`nbytes`)
- `decodeHeader` returns 6 values in Go: `(MainType, SubType, Version, uint32, []byte, error)`
- Go `min()` builtin (Go 1.21+) available for `min(nbytes-1, len(body))`
- Test prefix `TestPureGo*` avoids collision with existing WASM bridge tests

## Verification

- `cd packages/go && go build ./...` exits 0
- `cd packages/go && go test -run TestPureGoGenVideoCodeV0 -count=1 -v` — 3/3 video vectors PASS
- `cd packages/go && go test -run TestPureGoGenMixedCodeV0 -count=1 -v` — 2/2 mixed vectors PASS
- `cd packages/go && go vet ./...` exits 0
- `cd packages/go && go test ./...` — all tests pass (pure Go + WASM bridge)
- `grep -c 'func GenVideoCodeV0' packages/go/code_content_video.go` returns 1
- `grep -c 'func GenMixedCodeV0' packages/go/code_content_mixed.go` returns 1
- `mise run check` — all hooks pass

## Done When

All verification criteria pass: both GenVideoCodeV0 and GenMixedCodeV0 produce correct ISCC codes
for all 5 conformance vectors, full Go test suite passes, and all quality gates are clean.
