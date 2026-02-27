## 2026-02-27 — Implement pure Go GenImageCodeV0 and GenAudioCodeV0

**Done:** Implemented `GenImageCodeV0` and `GenAudioCodeV0` as pure Go functions with all helper
functions. Image-Code uses 2D DCT + median-threshold bitstring extraction from 8×8 low-frequency
blocks. Audio-Code uses multi-stage SimHash (overall + quarters + sorted thirds) on Chromaprint
features. Both reuse existing `algDct`, `AlgSimhash`, `EncodeComponent`, and `bitsToBytes` from the
Go codebase.

**Files changed:**

- `packages/go/code_content_image.go`: New file — `GenImageCodeV0`, `softHashImageV0`,
    `transposeMatrix`, `flatten8x8`, `computeMedian` (ImageCodeResult struct)
- `packages/go/code_content_audio.go`: New file — `GenAudioCodeV0`, `softHashAudioV0`, `arraySplit`
    generic helper (AudioCodeResult struct)
- `packages/go/code_content_image_test.go`: New file — 3 image conformance vectors
- `packages/go/code_content_audio_test.go`: New file — 5 audio conformance vectors

**Verification:**

- `cd packages/go && go build ./...` — exits 0
- `cd packages/go && go test -run TestPureGoGenImageCodeV0 -count=1 -v` — 3/3 image vectors PASS
- `cd packages/go && go test -run TestPureGoGenAudioCodeV0 -count=1 -v` — 5/5 audio vectors PASS
- `cd packages/go && go vet ./...` — exits 0
- `cd packages/go && go test ./...` — all tests pass (pure Go + WASM bridge)
- `grep -c 'func GenImageCodeV0' packages/go/code_content_image.go` — returns 1
- `grep -c 'func GenAudioCodeV0' packages/go/code_content_audio.go` — returns 1
- `mise run check` — all 14 hooks pass

**Next:** Implement `GenVideoCodeV0` and `GenMixedCodeV0` as the next gen function sub-step. Video
code reuses the same DCT + WTA-Hash + SimHash pipeline (DCT per frame, WTA-Hash per frame, SimHash
across frames). Mixed code processes multiple ISCC content codes via decompose + sort + group +
SimHash. After these two, only `GenIsccCodeV0` remains to complete all 9 gen functions.

**Notes:** No surprises. The `AlgSimhash` function returns output length equal to input digest
length (4 bytes for 4-byte digests), so we take `[:4]` when appending to parts. The `arraySplit`
generic helper was implemented as a Go generic function `arraySplit[T any]` since it's used with
both `[][]byte` slices. No WASM bridge naming collisions — `GenImageCodeV0` and `GenAudioCodeV0` are
package-level functions while the WASM bridge wraps them as methods on the `Runtime` struct.
