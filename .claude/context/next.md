# Next Work Package

## Step: Implement pure Go GenImageCodeV0 and GenAudioCodeV0

## Goal

Port the Image-Code and Audio-Code generation functions to pure Go, completing 6 of 9 gen functions
in the Go rewrite. This unblocks GenVideoCodeV0 (which also uses DCT) and brings the rewrite closer
to WASM bridge removal.

## Scope

- **Create**: `packages/go/code_content_image.go`, `packages/go/code_content_audio.go`,
    `packages/go/code_content_image_test.go`, `packages/go/code_content_audio_test.go`
- **Modify**: (none)
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines 416–633 (helpers, `soft_hash_image_v0`, `gen_image_code_v0`,
        `array_split`, `soft_hash_audio_v0`, `gen_audio_code_v0`)
    - `packages/go/dct.go` (`algDct` function signature)
    - `packages/go/simhash.go` (`AlgSimhash` function signature)
    - `packages/go/codec.go` (`EncodeComponent`, `MTContent`, `STImage`, `STAudio`, `VSV0`)
    - `packages/go/code_content_text.go` (pattern: result struct, gen function, soft hash helper)
    - `packages/go/code_data_test.go` (pattern: conformance test harness)

## Not In Scope

- GenVideoCodeV0, GenMixedCodeV0, GenIsccCodeV0 — separate step(s)
- Refactoring existing gen functions or algorithms
- WASM bridge removal or cleanup — wait until all 9 gen functions are done
- Adding an unexported `algSimhashInner` to Go — use existing `AlgSimhash` and discard the error
    (validation is trivially satisfied when all digests are 4 bytes)
- Exporting soft hash helper functions — these are unexported (`softHashImageV0`, `softHashAudioV0`)

## Implementation Notes

### GenImageCodeV0 (`code_content_image.go`, ~90-100 lines)

Port from Rust `soft_hash_image_v0` + `gen_image_code_v0` (lib.rs lines 480–550):

1. **Input validation**: exactly 1024 pixels (`[]byte`), bits ≤ 256
2. **Unexported helpers** (all in the same file):
    - `transposeMatrix(matrix [][]float64) [][]float64` — swap rows/cols
    - `flatten8x8(matrix [][]float64, col, row int) []float64` — extract 8×8 block at (col, row)
    - `computeMedian(values []float64) float64` — sort + pick middle (avg for even length)
    - `bitsToBytes(bits []bool) []byte` — MSB-first per byte
3. **`softHashImageV0(pixels []byte, bits uint32) ([]byte, error)`**:
    - Row-wise DCT (32 rows of 32 pixels → `algDct`)
    - Transpose
    - Column-wise DCT
    - Transpose back
    - Extract 8×8 blocks at positions `(0,0), (1,0), (0,1), (1,1)` — note `(col, row)` ordering
    - For each block: compute median, compare each value `> median` → bool bitstring
    - Break early when `len(bitstring) >= bits`
    - Convert first `bits` bools to bytes
4. **`GenImageCodeV0(pixels []byte, bits uint32) (*ImageCodeResult, error)`**:
    - Call `softHashImageV0`, then `EncodeComponent(MTContent, STImage, VSV0, bits, digest)`
    - Return `&ImageCodeResult{Iscc: "ISCC:" + component}`
5. **Result struct**: `ImageCodeResult{Iscc string}`

### GenAudioCodeV0 (`code_content_audio.go`, ~100-120 lines)

Port from Rust `array_split` + `soft_hash_audio_v0` + `gen_audio_code_v0` (lib.rs lines 557–633):

1. **Unexported helper**:
    - `arraySplit[T any](slice []T, n int) [][]T` — distribute elements evenly, first `len%n` parts
        get one extra. Returns empty sub-slices for excess parts (same as Python
        `more_itertools.divide`)
2. **`softHashAudioV0(cv []int32) []byte`** (returns exactly 32 bytes):
    - Convert each `int32` to 4-byte big-endian `[]byte` → `digests [][]byte`
    - Empty input: return 32 zero bytes
    - **Stage 1**: `AlgSimhash(digests)` → 4-byte result → append to `parts`
    - **Stage 2**: `arraySplit(digests, 4)` → for each quarter: if empty, append 4 zero bytes; else
        `AlgSimhash(quarter)` → append 4 bytes. Total: 16 bytes
    - **Stage 3**: sort `cv` copy by value → convert sorted to digests →
        `arraySplit(sortedDigests, 3)` → for each third: same empty handling → append 4 bytes. Total:
        12 bytes
    - Return `parts` (4 + 16 + 12 = 32 bytes)
3. **`GenAudioCodeV0(cv []int32, bits uint32) (*AudioCodeResult, error)`**:
    - Call `softHashAudioV0`, then `EncodeComponent(MTContent, STAudio, VSV0, bits, digest)`
    - Return `&AudioCodeResult{Iscc: "ISCC:" + component}`
4. **Result struct**: `AudioCodeResult{Iscc string}`

### Key details

- `AlgSimhash` on 4-byte inputs returns 4 bytes (output length = input digest length). Discard the
    error with `result, _ := AlgSimhash(...)` since all digests are guaranteed equal length
- `int32` to big-endian: `binary.BigEndian.PutUint32(buf[:], uint32(v))` — Go handles two's
    complement naturally via the `uint32()` cast
- Test file naming: `TestPureGoGenImageCodeV0`, `TestPureGoGenAudioCodeV0` (prefix avoids collision
    with WASM bridge tests)
- Image conformance vectors: input[0] is `[]float64` (JSON numbers) → cast to `[]byte`
- Audio conformance vectors: input[0] is `[]float64` (JSON numbers) → cast to `[]int32`

## Verification

- `cd packages/go && go build ./...` exits 0
- `cd packages/go && go test -run TestPureGoGenImageCodeV0 -count=1 -v` — 3/3 image vectors PASS
- `cd packages/go && go test -run TestPureGoGenAudioCodeV0 -count=1 -v` — 5/5 audio vectors PASS
- `cd packages/go && go vet ./...` exits 0
- `cd packages/go && go test ./...` — all tests pass (pure Go + WASM bridge)
- `grep -c 'func GenImageCodeV0' packages/go/code_content_image.go` returns 1
- `grep -c 'func GenAudioCodeV0' packages/go/code_content_audio.go` returns 1

## Done When

All 7 verification commands pass, confirming 8 new conformance vectors (3 image + 5 audio) produce
correct ISCC codes matching the reference implementation.
