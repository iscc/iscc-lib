## 2026-11-22 — Go port rejects ISCC-UNITs above 256 bits (encode and decode)

**Done:** `encodeLength` now accepts only 32 to 256 bits (multiple of 32) for Meta through Flake,
with the Rust error wording. A checked helper `unitBitLength` replaces all four `decodeLength`
callers outside its own tests (`IsccDecompose`, `IsccDecode` before the `make`, mixed, the wide
check). The wide check is a short loop that stops at the first unit below 128 bits, so a later
over-long unit is only read when every earlier one reaches 128 bits.

**Files changed:**

- `packages/go/codec.go`: `encodeLength` bound + message + doc; `unitBitLength` (new, unexported);
    `decodeLength` doc notes it is unchecked; `IsccDecompose`/`IsccDecode` call the helper;
    `EncodeComponent` doc names the 32 to 256 range
- `packages/go/code_content_mixed.go`: `softHashCodesV0` calls `unitBitLength`
- `packages/go/code_iscc.go`: step 8 wide check as an in-order loop over `unitBitLength`
- `docs/howto/go.md`: line 493 comment ("multiple of 32 from 32 to 256")
- Tests: `codec_test.go` (helpers `overlongUnit`, `assertEncodeBoundErr`, `assertDecodeBoundErr`;
    encodeLength, unitBitLength, EncodeComponent, IsccDecode, IsccDecompose), and one `Above256`
    test each in `code_meta`, `code_content_{text,image,audio,video,mixed}`, `code_data` (gen +
    hasher), `code_instance` (gen + hasher), `code_iscc` (two wide cases) `_test.go`

**Verification:**

- `CGO_ENABLED=0 go test -count=1 ./...` ok; `GOARCH=386 CGO_ENABLED=0 go test -count=1 ./...` ok
- `go test -run Above256 -v ./...`: 17 tests PASS — encode (meta, text, image, audio, video, data,
    instance, both hashers' `Finalize`, `EncodeComponent`, `encodeLength`), decode (`IsccDecode`,
    `IsccDecompose`, `unitBitLength`), mixed, wide Data-288 (fails with `true`, succeeds with
    `false`) and wide Data-64 + Instance-288 (succeeds with `true`, equals the `false` result)
- `go vet ./...` and `GOARCH=386 go vet ./...` clean; `gofmt -l .` empty
- `git grep -n "decodeLength(" -- packages/go ':!*_test.go'`: `codec.go:313` (definition) and
    `codec.go:335` (inside `unitBitLength`) only
- `git grep -n ">= 32)" -- packages/go`: nothing.
    `git grep -n "32 to 256" -- packages/go   docs/howto/go.md`: `encodeLength` (277 doc, 286
    message), `EncodeComponent` comment (497), the how-to line, plus a pre-existing `codec.go:903`
    comment (`IsccNormalizeSeq`) and the test helper
- `mise run check` exits 0

**Next:** the `data-edge.json` rows (`diverges`) across all 12 harnesses. Per the iter-301 note, use
a gen function without input units (e.g. meta/data with `bits=288`) for the encode row; mixed at 288
already failed before this bound.

**Notes:**

- Decode tests assert the "an ISCC-UNIT is at most 256 bits" message, which only `unitBitLength`
    produces, so they pin the helper itself (the Rust decompose test passes through the encode bound
    instead, see the iter-301 nit). Encode tests assert "must be a multiple of 32 from 32 to 256"
- Image and video fail BEFORE `encodeLength` with their own errors (`softHashImageV0` "bits must be
    \<= 256", `AlgWtahash` "\<= 256"); their tests only require an error, as next.md allowed
- Arithmetic stays `uint32`; no new `int` conversions. Go-only step: no CRAP or iai impact
