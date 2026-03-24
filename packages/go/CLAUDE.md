# CLAUDE.md — Go Package

Pure Go implementation of ISO 24138:2024 (ISCC). All algorithms are implemented in native Go with
zero cgo dependencies — builds with `CGO_ENABLED=0`, cross-compiles with `GOOS`/`GOARCH`, and
distributes via standard `go get`.

## Package Role

- Self-contained reimplementation of all ISCC algorithms in Go (not a binding over `iscc-ffi`)
- Produces identical output to the Rust `iscc-lib` core and Python `iscc-core` reference
- Package name: `iscc` (import path: `github.com/iscc/iscc-lib/packages/go`)
- No cgo, no shared libraries, no binary artifacts — pure compiled Go

## File Layout

| File                    | Purpose                                                                                            |
| ----------------------- | -------------------------------------------------------------------------------------------------- |
| `go.mod`                | Module definition, Go version, dependencies                                                        |
| `codec.go`              | Type enums, varnibble encoding, header codec, base32/64, component encoding, ISCC decompose/decode |
| `code_meta.go`          | `GenMetaCodeV0` — Meta-Code from name/description/meta                                             |
| `code_content_text.go`  | `GenTextCodeV0` — Content-Code for text via MinHash                                                |
| `code_content_image.go` | `GenImageCodeV0` — Content-Code for images via DCT                                                 |
| `code_content_audio.go` | `GenAudioCodeV0` — Content-Code for audio via multi-stage SimHash                                  |
| `code_content_video.go` | `GenVideoCodeV0` — Content-Code for video via WTA-Hash                                             |
| `code_content_mixed.go` | `GenMixedCodeV0` — Mixed Content-Code from multiple Content-Codes                                  |
| `code_data.go`          | `GenDataCodeV0` — Data-Code via CDC + MinHash; `DataHasher` streaming                              |
| `code_instance.go`      | `GenInstanceCodeV0` — Instance-Code via BLAKE3; `InstanceHasher` streaming                         |
| `code_iscc.go`          | `GenIsccCodeV0` — composite ISCC-CODE assembly                                                     |
| `code_sum.go`           | `GenSumCodeV0` — single-pass file ISCC-SUM (Data + Instance + compose)                             |
| `conformance.go`        | `ConformanceSelftest` — validates all 9 gen functions against vendored vectors                     |
| `utils.go`              | `TextClean`, `TextCollapse`, `TextTrim`, `TextRemoveNewlines`                                      |
| `simhash.go`            | `AlgSimhash`, `SlidingWindow`                                                                      |
| `minhash.go`            | `AlgMinhash256` — 64-dimensional MinHash with bit-interleaved compression                          |
| `dct.go`                | `algDct` — fast DCT (Nayuki algorithm) for image hashing                                           |
| `wtahash.go`            | `AlgWtahash` — WTA-Hash with 256 permutation pairs                                                 |
| `xxh32.go`              | `xxh32` — xxHash32 implementation for text/data code features                                      |
| `cdc.go`                | `AlgCdcChunks` — Content-Defined Chunking (FastCDC gear hash)                                      |
| `testdata/data.json`    | Vendored conformance vectors (embedded via `//go:embed`)                                           |
| `*_test.go`             | Tests (conformance + unit tests for every module)                                                  |

## Dependencies

| Dependency                | Purpose                                              |
| ------------------------- | ---------------------------------------------------- |
| `github.com/zeebo/blake3` | BLAKE3 hashing (Instance-Code, Meta-Code metahash)   |
| `golang.org/x/text`       | Unicode normalization (NFKC, NFD) for text utilities |

No other external dependencies. All algorithm primitives (xxHash32, SimHash, MinHash, CDC, DCT,
WTA-Hash) are implemented from scratch in Go.

## Public API

### Code Generators

All `Gen*` functions return typed result structs and `error`, following idiomatic Go conventions.

| Function            | Input                             | Result Struct         |
| ------------------- | --------------------------------- | --------------------- |
| `GenMetaCodeV0`     | name, \*description, \*meta, bits | `*MetaCodeResult`     |
| `GenTextCodeV0`     | text, bits                        | `*TextCodeResult`     |
| `GenImageCodeV0`    | pixels (1024 bytes), bits         | `*ImageCodeResult`    |
| `GenAudioCodeV0`    | chromaprint []int32, bits         | `*AudioCodeResult`    |
| `GenVideoCodeV0`    | frameSigs [][]int32, bits         | `*VideoCodeResult`    |
| `GenMixedCodeV0`    | codes []string, bits              | `*MixedCodeResult`    |
| `GenDataCodeV0`     | data []byte, bits                 | `*DataCodeResult`     |
| `GenInstanceCodeV0` | data []byte, bits                 | `*InstanceCodeResult` |
| `GenIsccCodeV0`     | codes []string, wide bool         | `*IsccCodeResult`     |
| `GenSumCodeV0`      | path, bits, wide, addUnits        | `*SumCodeResult`      |

### Streaming Hashers

| Type             | Pattern                                                     |
| ---------------- | ----------------------------------------------------------- |
| `DataHasher`     | `NewDataHasher()` -> `Push([]byte)` -> `Finalize(bits)`     |
| `InstanceHasher` | `NewInstanceHasher()` -> `Push([]byte)` -> `Finalize(bits)` |

### Codec Functions (exported)

`EncodeComponent`, `EncodeBase64`, `JsonToDataUrl`, `IsccDecode`, `IsccDecompose`

### Algorithm Primitives (exported)

`AlgSimhash`, `AlgMinhash256`, `AlgCdcChunks`, `AlgWtahash`, `SlidingWindow`, `SoftHashVideoV0`

### Text Utilities (exported)

`TextClean`, `TextCollapse`, `TextTrim`, `TextRemoveNewlines`

## Build Commands

```bash
# Run all tests (pure Go, no cgo required)
CGO_ENABLED=0 go test -v -count=1 ./...

# Run tests (default, cgo may be enabled but not needed)
go test -v ./...

# Run a specific test
go test -v -run TestPureGoConformanceSelftest ./...

# Vet
go vet ./...

# Build check (verify compilation)
go build ./...
```

No Makefile or build script — standard `go test` and `go build` suffice. The `CGO_ENABLED=0` flag in
CI confirms the package has no cgo dependencies.

## Test Patterns

### Conformance tests

- **`conformance_test.go`** — calls `ConformanceSelftest()` which validates all 46 conformance
    vectors from the embedded `testdata/data.json` in a single call
- **`code_*_test.go`** — per-function conformance tests that read
    `../../crates/iscc-lib/tests/data.json` and verify every output field (iscc, name, description,
    meta, metahash, datahash, filesize, etc.)
- Conformance tests that read from `../../crates/iscc-lib/tests/data.json` use `t.Skipf` or
    `t.Fatalf` if the file is missing (the embedded copy in `testdata/data.json` always works)

### Unit tests

- **`codec_test.go`** — varnibble roundtrip, header encode/decode, length encoding, base32/64, unit
    encoding, `EncodeComponent`, `IsccDecode`, `IsccDecompose`
- **`simhash_test.go`** — `AlgSimhash` edge cases, `SlidingWindow` with Unicode
- **`minhash_test.go`** — `minhashFn`, `minhashCompress`, `AlgMinhash256`
- **`dct_test.go`** — fast DCT known values, error cases
- **`wtahash_test.go`** — WTA-Hash, permutation table validation
- **`xxh32_test.go`** — canonical xxHash32 test vectors
- **`cdc_test.go`** — CDC params, offset, chunk reassembly, UTF-32 alignment
- **`utils_test.go`** — text cleaning, collapsing, trimming, newline removal
- **`code_sum_test.go`** — `GenSumCodeV0` equivalence, result fields, wide mode, units

### Test conventions

- All tests use package-internal access (same `package iscc`)
- Helper comparison functions (`byteSliceEqual`, `boolSliceEqual`, `mainTypeSliceEqual`) defined in
    `codec_test.go`
- No test classes — standalone `Test*` functions following Go conventions
- No mocks — all tests use real data and algorithm outputs

## Publishing

Go modules are published automatically via the Go module proxy. When a version tag is pushed to
`main`, the Go module proxy (`proxy.golang.org`) picks up the module at
`github.com/iscc/iscc-lib/packages/go`. No explicit publish step is needed — the proxy indexes
modules from public Git repositories on demand.

Install: `go get github.com/iscc/iscc-lib/packages/go`

Documentation: [pkg.go.dev](https://pkg.go.dev/github.com/iscc/iscc-lib/packages/go)

## Architecture Decisions

### Pure Go (no cgo, no FFI)

Unlike most other language packages in this monorepo (which wrap `iscc-ffi` via C interop), the Go
package reimplements all ISCC algorithms in native Go. This eliminates:

- Cross-compilation complexity (no C toolchain needed)
- Runtime shared library loading
- Platform-specific binary artifacts

The tradeoff is that algorithm updates must be ported to Go separately (not automatic from the Rust
core).

### Embedded conformance vectors

`conformance.go` uses `//go:embed testdata/data.json` to embed the conformance vectors at compile
time. This enables `ConformanceSelftest()` to work without filesystem access — useful for
applications that want to verify ISCC correctness at startup.

### Streaming pattern

`DataHasher` and `InstanceHasher` follow the `New() -> Push([]byte) -> Finalize(bits)` pattern
matching the Rust core's streaming API. `GenSumCodeV0` demonstrates single-pass file processing by
feeding both hashers from the same read loop.

## Common Pitfalls

- **Conformance vector path**: per-function conformance tests reference
    `../../crates/iscc-lib/tests/data.json` via relative path. Tests will skip or fail if run from a
    directory other than `packages/go/`. The embedded `testdata/data.json` copy used by
    `ConformanceSelftest` is always available.
- **Unicode character counting**: `SlidingWindow` and `TextCollapse` operate on `[]rune`, not bytes.
    Use `len([]rune(s))` for character counts, not `len(s)`.
- **JSON canonicalization**: `parseMetaJSON` relies on Go's `json.Marshal` producing sorted keys and
    compact format (matching JCS/RFC 8785 for string/boolean/null values). This works for all
    conformance vectors but is not a full JCS implementation.
- **CDC tail handling**: `DataHasher.Push()` retains the last CDC chunk as `tail` for the next
    `Push` call (mirrors the Python `prev_chunk` pattern). Forgetting to call `Finalize()` loses the
    tail data.
- **SimHash threshold**: the majority-vote threshold uses `count*2 >= n` (not `count > n/2`) to
    match the Python reference behavior for even counts.
- **MinHash wrapping arithmetic**: `minhashFn` relies on Go's natural uint64 wrapping multiplication
    to match Rust's `wrapping_mul`. Do not add overflow checks.
- **Image pixels**: `GenImageCodeV0` expects exactly 1024 bytes (32x32 grayscale, values 0-255). The
    caller is responsible for resizing and converting images to this format.
- **Video frame signatures**: `GenVideoCodeV0` expects each frame signature to have 380 int32 values
    (MPEG-7 format). The caller extracts these from video frames externally.
- **Module path**: the import path is `github.com/iscc/iscc-lib/packages/go`, not
    `github.com/iscc/iscc-lib`. The `go` subdirectory under `packages/` is the Go module root.
