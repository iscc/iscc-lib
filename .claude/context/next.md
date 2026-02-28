# Next Work Package

## Step: Remove WASM bridge from Go bindings

## Goal

Remove the now-redundant WASM/wazero bridge from the Go package, completing the pure Go rewrite.
This eliminates the 667KB binary artifact from git, removes the wazero dependency, and restores the
large-file guard to 256KB.

## Scope

- **Delete**: `packages/go/iscc.go` (1,357-line WASM bridge), `packages/go/iscc_ffi.wasm` (667KB
    binary), `packages/go/iscc_test.go` (46 WASM bridge tests — all covered by pure Go tests)
- **Modify**: `packages/go/codec.go` (relocate `DecodeResult` struct and 4 algorithm constants from
    `iscc.go`), `packages/go/go.mod` (remove wazero dependency), `.pre-commit-config.yaml` (restore
    `--maxkb=256`)
- **Reference**: `packages/go/iscc.go` (read before deletion — identify all types/constants that
    must be relocated), `packages/go/codec.go` (current state, find the "defined in iscc.go"
    comment)

## Not In Scope

- Refactoring pure Go code (algorithm files, gen functions) — they are complete and passing
- Updating CI workflow (`ci.yml`) — the Go CI job already runs `go test ./...` which will work fine
    without the WASM bridge; any CI simplification (removing WASM build steps) is a separate concern
- Renaming `WasmDataHasher`/`WasmInstanceHasher` — these types live in `iscc.go` and will be deleted
    entirely, not renamed
- Updating documentation or README for the Go package — docs already describe pure Go
- Adding `.gitignore` entries for `*.wasm` — the file is deleted from git, not just ignored

## Implementation Notes

**Relocate shared definitions before deletion:**

The following definitions in `iscc.go` are used by pure Go files and must be moved to `codec.go`
before `iscc.go` is deleted:

1. **4 constants** (lines 23-28):

    ```go
    const (
        MetaTrimName        = 128
        MetaTrimDescription = 4096
        IoReadSize          = 4_194_304
        TextNgramSize       = 13
    )
    ```

    Used by: `code_meta.go` (MetaTrimName, MetaTrimDescription), `code_content_text.go`
    (TextNgramSize). Place them near the top of `codec.go` with the existing codec constants.

2. **`DecodeResult` struct** (lines 33-40):

    ```go
    type DecodeResult struct {
        Maintype uint8
        Subtype  uint8
        Version  uint8
        Length   uint8
        Digest   []byte
    }
    ```

    Used by: `codec.go` `IsccDecode` function (line 544) and `conformance.go`. Move it to `codec.go`
    near the `IsccDecode` function. Update the comment on line 542-543 that says "Reuses the
    DecodeResult struct defined in iscc.go" to just describe the struct locally.

**Deletion order:**

1. First, apply modifications to `codec.go` (add constants + DecodeResult)
2. Then delete `iscc.go`, `iscc_test.go`, `iscc_ffi.wasm`
3. Update `go.mod`: remove the `github.com/tetratelabs/wazero v1.11.0` require line
4. Run `cd packages/go && go mod tidy` to clean up `go.sum`
5. Fix `.pre-commit-config.yaml`: change `args: [--maxkb=1024]` to `args: [--maxkb=256]`

**Verification sequence:**

Run `cd packages/go && go build ./...` first (catches any missing type/constant). Then
`go test ./...` (all pure Go tests pass). Then `go vet ./...` (clean).

## Verification

- `cd packages/go && go build ./...` exits 0 (no compilation errors after removing iscc.go)
- `cd packages/go && go test ./... -count=1` passes (all pure Go tests including conformance)
- `cd packages/go && go vet ./...` exits 0
- `test ! -f packages/go/iscc.go` exits 0 (WASM bridge file deleted)
- `test ! -f packages/go/iscc_ffi.wasm` exits 0 (WASM binary deleted)
- `test ! -f packages/go/iscc_test.go` exits 0 (WASM bridge tests deleted)
- `grep -q wazero packages/go/go.mod; test $? -ne 0` (wazero dependency removed)
- `grep 'maxkb=256' .pre-commit-config.yaml` exits 0 (threshold restored)
- `grep -q 'type DecodeResult struct' packages/go/codec.go` exits 0 (struct relocated)
- `grep -q 'MetaTrimName' packages/go/codec.go` exits 0 (constants relocated)
- `mise run check` passes (all pre-commit hooks clean, including large-file check)

## Done When

All verification criteria pass: WASM bridge files are deleted, shared types/constants are relocated
to codec.go, wazero dependency is removed, large-file threshold is restored to 256KB, and all Go
tests pass.
