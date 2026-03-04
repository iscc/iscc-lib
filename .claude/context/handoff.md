## 2026-03-04 — Add avgChunkSize validation to Go AlgCdcChunks

**Done:** Added input validation to Go's `AlgCdcChunks` so it returns `([][]byte, error)` and
rejects `avgChunkSize < 2`, matching the Rust core pattern. Extracted `algCdcChunksUnchecked` for
internal callers. Updated `DataHasher.Push` to use the unchecked variant. Updated howto docs.

**Files changed:**

- `packages/go/cdc.go`: Split `AlgCdcChunks` into validating wrapper plus `algCdcChunksUnchecked`.
    Added `fmt` import.
- `packages/go/code_data.go`: Changed `Push` to call `algCdcChunksUnchecked` (constant 1024 always
    valid).
- `packages/go/cdc_test.go`: Updated all 12 existing `AlgCdcChunks` test calls to handle error
    return. Added `TestCdcChunksInvalidAvgChunkSize` testing sizes 0, 1 (error), and 2 (boundary).
- `docs/howto/go.md`: Updated `AlgCdcChunks` signature in Algorithm primitives section.

**Verification:** All 5 checks from next.md pass: go test (156 tests), go vet clean, validation
guard present, unchecked variant used in code_data.go, doc updated.

**Next:** All issues in issues.md are low priority. Consider elevating one to normal or creating a
release PR from develop to main.

**Notes:** The `fmt` import was added to cdc.go for `fmt.Errorf`. No surprises. Formatting hooks
modified some unrelated files on first run but settled on second pass.
