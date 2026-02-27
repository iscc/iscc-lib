## 2026-02-27 — Implement pure Go GenVideoCodeV0 and GenMixedCodeV0

**Done:** Implemented GenVideoCodeV0 (video content fingerprinting via WTA-Hash on deduplicated
frame signatures) and GenMixedCodeV0 (multi-code SimHash combination) as pure Go functions. Both
produce correct ISCC codes for all 5 conformance vectors.

**Files changed:**

- `packages/go/code_content_video.go`: New file — `VideoCodeResult`, `GenVideoCodeV0`,
    `SoftHashVideoV0` (exported, matching Rust). Deduplicates frame sigs via string-keyed map,
    column-wise sums into int64, delegates to `AlgWtahash`
- `packages/go/code_content_video_test.go`: New file — 3 conformance vectors testing video code
    generation
- `packages/go/code_content_mixed.go`: New file — `MixedCodeResult`, `GenMixedCodeV0`,
    `softHashCodesV0` (unexported, matching Rust). Decodes ISCC strings, validates Content MainType,
    prepares fixed-length entries (header byte + body + zero-pad), delegates to `AlgSimhash`
- `packages/go/code_content_mixed_test.go`: New file — 2 conformance vectors testing mixed code
    generation, verifying both `Iscc` and `Parts` fields

**Verification:** All criteria pass:

- `go build ./...` — exits 0
- `TestPureGoGenVideoCodeV0` — 3/3 vectors PASS
- `TestPureGoGenMixedCodeV0` — 2/2 vectors PASS
- `go vet ./...` — exits 0
- `go test ./...` — all tests pass (pure Go + WASM bridge)
- Both function declarations confirmed present (grep -c returns 1)
- `mise run check` — all 14 hooks pass

**Next:** Implement `GenIsccCodeV0` — the 9th and final gen function. After that, implement
`conformance_selftest` and then WASM bridge cleanup. Only `GenIsccCodeV0` remains before all 9 gen
functions are complete.

**Notes:** No surprises. The implementation was straightforward — both functions closely mirror the
Rust reference. Video dedup uses `fmt.Sprintf("%v", sig)` for string keys (simple and correct).
Mixed code correctly handles the first-byte header preservation for type info encoding in the
SimHash input. 8/9 gen functions now complete (Meta, Text, Data, Instance, Image, Audio, Video,
Mixed — only ISCC remains).
