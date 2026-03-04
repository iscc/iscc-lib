# Next Work Package

## Step: Add avgChunkSize validation to Go AlgCdcChunks

## Goal

Add input validation to Go's `AlgCdcChunks` so it returns an `error` when `avgChunkSize < 2`,
matching the Rust core fix. This prevents an infinite hang when callers pass degenerate values like
0 or 1.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/go/cdc.go` — change `AlgCdcChunks` return type to `([][]byte, error)`, add validation
        guard, extract unexported `algCdcChunksUnchecked` for internal callers
    - `packages/go/code_data.go` — update `DataHasher.Push` to call `algCdcChunksUnchecked` (the
        constant `1024` is always valid, so no error handling needed in Push)
    - `docs/howto/go.md` — update the `AlgCdcChunks` signature in the "Algorithm primitives" section
        (line ~326) to show the `([][]byte, error)` return type
- **Reference**:
    - `crates/iscc-lib/src/cdc.rs` — Rust core pattern: `alg_cdc_chunks` (public, validates) +
        `alg_cdc_chunks_unchecked` (pub(crate), no validation)
    - `packages/go/cdc_test.go` — existing tests that need error return handling

## Not In Scope

- Changing `DataHasher.Push` signature to return error (Push always uses constant 1024, which is
    always valid — no API change needed)
- Adding validation to `algCdcParams` or `algCdcOffset` (these are unexported internal helpers)
- Updating the Go README table at `packages/go/README.md` (it lists function names without
    signatures — no change needed)
- Fixing the release smoke tests issue (separate `normal` priority issue)

## Implementation Notes

**Pattern to follow (mirrors Rust core):**

1. Rename current `AlgCdcChunks` implementation body to an unexported `algCdcChunksUnchecked`
    function with the same signature
    `func algCdcChunksUnchecked(data []byte, utf32 bool, avgChunkSize uint32) [][]byte`

2. Rewrite `AlgCdcChunks` as a thin wrapper that validates then delegates:

    ```go
    func AlgCdcChunks(data []byte, utf32 bool, avgChunkSize uint32) ([][]byte, error) {
        if avgChunkSize < 2 {
            return nil, fmt.Errorf("iscc: avgChunkSize must be >= 2, got %d", avgChunkSize)
        }
        return algCdcChunksUnchecked(data, utf32, avgChunkSize), nil
    }
    ```

3. In `code_data.go`, change `DataHasher.Push` to call `algCdcChunksUnchecked(data, false, 1024)`
    instead of `AlgCdcChunks(data, false, 1024)`.

4. Update all existing tests in `cdc_test.go` to handle the `([][]byte, error)` return — all
    existing calls use `avgChunkSize=1024` so `err` should be `nil` (assert no error).

5. Add new test `TestCdcChunksInvalidAvgChunkSize` that verifies:

    - `AlgCdcChunks(data, false, 0)` returns non-nil error
    - `AlgCdcChunks(data, false, 1)` returns non-nil error
    - `AlgCdcChunks(data, false, 2)` returns nil error (boundary)

6. Update `docs/howto/go.md` line ~326: change the signature from
    `AlgCdcChunks(data []byte, utf32 bool, avgChunkSize uint32) [][]byte` to
    `AlgCdcChunks(data []byte, utf32 bool, avgChunkSize uint32) ([][]byte, error)`

**Error message format**: Use `fmt.Errorf("iscc: avgChunkSize must be >= 2, got %d", avgChunkSize)`
to follow Go's error convention with package prefix.

## Verification

- `cd packages/go && go test ./... -count=1` passes (all existing tests + new validation test)
- `cd packages/go && go vet ./...` clean
- `grep -q 'avgChunkSize < 2' packages/go/cdc.go` exits 0 (validation guard present)
- `grep -q 'algCdcChunksUnchecked' packages/go/code_data.go` exits 0 (internal caller uses unchecked
    variant)
- `grep -q '(\\[\\]\\[\\]byte, error)' docs/howto/go.md` exits 0 (doc updated with error return)

## Done When

All five verification checks pass — Go tests green, vet clean, validation guard present in cdc.go,
internal caller uses unchecked variant, and howto doc reflects the new signature.
