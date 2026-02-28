# Next Work Package

## Step: Implement pure Go ConformanceSelftest

## Goal

Implement `ConformanceSelftest() (bool, error)` as a pure Go package-level function that validates
all 46 conformance vectors from `data.json` in a single call. This is the last functional piece
before the WASM bridge can be removed — it completes the 30/30 Tier 1 symbol coverage in pure Go.

## Scope

- **Create**: `packages/go/testdata/data.json` (copy from `crates/iscc-lib/tests/data.json`)
- **Create**: `packages/go/conformance.go`
- **Create**: `packages/go/conformance_test.go`
- **Reference**:
    - `crates/iscc-lib/src/conformance.rs` — Rust implementation (authoritative, port from this)
    - `packages/go/code_meta_test.go` — Go JSON parsing pattern for data.json vectors
    - `packages/go/code_data_test.go` — `"stream:<hex>"` decoding pattern for Data/Instance vectors
    - `packages/go/code_iscc_test.go` — GenIsccCodeV0 vector parsing pattern

## Not In Scope

- Removing the WASM bridge (`iscc.go`, `iscc_ffi.wasm`, wazero dependency) — that's a separate
    cleanup step after ConformanceSelftest is verified
- Restoring the `.pre-commit-config.yaml` large-file threshold to 256KB
- Modifying any existing pure Go gen function files
- Updating README or documentation (happens after WASM cleanup)

## Implementation Notes

**Embedding data.json**: Use `//go:embed testdata/data.json` to embed conformance vectors at compile
time. This mirrors Rust's `include_str!("../tests/data.json")` and ensures the function works when
the package is installed via `go get` (no file system dependency). Copy the file to
`packages/go/testdata/data.json` — Go convention for test/embedded data.

**Function signature**: `func ConformanceSelftest() (bool, error)` — returns `(true, nil)` if all 46
vectors pass, `(false, nil)` if any vector mismatches, and `(false, error)` if data.json cannot be
parsed. This is idiomatic Go (error for infrastructure failures, bool for conformance result). No
naming conflict with the WASM bridge method `(rt *Runtime) ConformanceSelftest(ctx, ...)`.

**Structure**: Mirror `crates/iscc-lib/src/conformance.rs` directly:

1. Parse embedded JSON into `map[string]map[string]vectorEntry`
2. Run 9 section runners (one per gen function):
    - `runMetaTests`, `runTextTests`, `runImageTests`, `runAudioTests`, `runVideoTests`,
        `runMixedTests`, `runDataTests`, `runInstanceTests`, `runIsccTests`
3. Each section runner iterates vectors, parses inputs, calls the gen function, compares
    `result.Iscc` against expected `iscc` output
4. Use `fmt.Fprintf(os.Stderr, ...)` for failure messages (matches Rust's `eprintln!`)

**Input parsing per gen function** (copy patterns from existing `*_test.go` files):

- **Meta**: `[name, description, meta, bits]` — meta can be null/string/object
- **Text**: `[text, bits]`
- **Image**: `[pixels_array, bits]` — flat `[]uint8`
- **Audio**: `[chromaprint_array, bits]` — `[]int32`
- **Video**: `[frame_sigs_array, bits]` — `[][]int32`
- **Mixed**: `[codes_array, bits]` — `[]string`
- **Data**: `["stream:<hex>", bits]` — hex-decode after stripping prefix
- **Instance**: `["stream:<hex>", bits]` — same as Data
- **IsccCode**: `[codes_array]` — `[]string`, always `wide=false`

**Key detail**: For Data and Instance vectors, decode `"stream:<hex>"` — strip the `"stream:"`
prefix, then hex-decode the remainder. Empty hex after prefix = empty `[]byte`.

## Verification

- `cd packages/go && go build ./...` exits 0
- `cd packages/go && go test -run TestPureGoConformanceSelftest -count=1 -v` — the pure Go selftest
    passes (returns `true, nil`)
- `cd packages/go && go vet ./...` exits 0
- `cd packages/go && go test ./...` — all tests pass (pure Go + existing WASM bridge tests)
- `grep -c 'func ConformanceSelftest' packages/go/conformance.go` returns 1
- `test -f packages/go/testdata/data.json` exits 0
- `mise run check` — all hooks pass

## Done When

The advance agent is done when `ConformanceSelftest()` is a pure Go package-level function that
validates all 46 conformance vectors, `go test -run TestPureGoConformanceSelftest` passes, and all
existing tests continue to pass.
