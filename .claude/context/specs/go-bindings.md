# Spec: Go Bindings — Pure Go Implementation

A pure Go module consumable via `go get`, providing idiomatic Go access to all ISCC functions. This
is a native Go implementation — not an FFI wrapper, CGO bridge, or WASM embed. Go developers get
`go get` with no binary artifacts, no C toolchain, and full source-level debugging.

## Architecture

**Pure Go (not FFI/WASM bridge):**

- Native Go implementation of all ISCC algorithms (CDC, MinHash, SimHash, DCT, WTA-Hash)
- No CGO, no WASM, no embedded binaries — just Go source code
- Idiomatic Go API with Go naming conventions, `error` returns, `[]byte` slices, and `io.Reader`
    support for streaming
- Lives in this repository under `packages/go/` as a Go sub-module
- Validated against the same conformance test vectors as all other bindings

**Why pure Go (not WASM/wazero bridge):**

- **Zero distribution friction** — `go get` fetches source code only, no binary artifacts in git
- **Native performance** — compiled to machine code, no WASM interpreter overhead. BLAKE3 and xxHash
    Go libraries have SIMD-optimized implementations
- **First-class debugging** — Go developers can step into ISCC code, profile it, read the source
- **Cross-compilation works** — `GOOS=linux GOARCH=arm64 go build` just works
- **No build artifacts in git** — avoids polluting git history with binaries

## Package Structure

```
packages/go/
├── go.mod                  # Module: github.com/iscc/iscc-lib/packages/go
├── go.sum
├── iscc.go                 # Main API (gen_*_v0 functions)
├── iscc_test.go            # Conformance tests against data.json
├── codec.go                # Header encode/decode, base32, component encoding
├── codec_test.go
├── text.go                 # Unicode normalization, cleaning, trimming, collapsing
├── text_test.go
├── algorithms.go           # CDC, MinHash, SimHash, sliding window
├── algorithms_test.go
├── dct.go                  # Nayuki fast recursive DCT
├── dct_test.go
├── wta.go                  # WTA-Hash (video fingerprinting)
├── wta_test.go
├── hasher.go               # DataHasher, InstanceHasher streaming types
├── hasher_test.go
├── constants.go            # Algorithm constants (META_TRIM_*, IO_READ_SIZE, etc.)
├── testdata/
│   └── data.json           # Vendored conformance vectors
└── README.md               # Per-package README
```

## Public API Surface

### Naming Conventions

Go conventions: `PascalCase` exported functions, `error` return values, `[]byte` for binary data.

| Rust Core Symbol       | Go Public API                                                      |
| ---------------------- | ------------------------------------------------------------------ |
| `gen_meta_code_v0`     | `GenMetaCodeV0(name string, ...) (*MetaCodeResult, error)`         |
| `gen_text_code_v0`     | `GenTextCodeV0(text string, ...) (*TextCodeResult, error)`         |
| `gen_data_code_v0`     | `GenDataCodeV0(data []byte, ...) (*DataCodeResult, error)`         |
| `gen_instance_code_v0` | `GenInstanceCodeV0(data []byte, ...) (*InstanceCodeResult, error)` |
| `gen_sum_code_v0`      | `GenSumCodeV0(path string, ...) (*SumCodeResult, error)`           |
| `text_clean`           | `TextClean(text string) string`                                    |
| `sliding_window`       | `SlidingWindow(text string, width int) []string`                   |
| `alg_minhash_256`      | `AlgMinhash256(features []uint32) []byte`                          |
| `DataHasher`           | `NewDataHasher() *DataHasher`                                      |
| `conformance_selftest` | `ConformanceSelftest() bool`                                       |

### Result Types

```go
type MetaCodeResult struct {
    ISCC        string  `json:"iscc"`
    Name        string  `json:"name"`
    MetaHash    string  `json:"metahash"`
    Description string  `json:"description,omitempty"`
    Meta        string  `json:"meta,omitempty"`
}

type SumCodeResult struct {
    ISCC     string `json:"iscc"`
    DataHash string `json:"datahash"`
    FileSize uint64 `json:"filesize"`
}
```

### Streaming Types

```go
type DataHasher struct { /* internal state */ }

func NewDataHasher() *DataHasher
func (h *DataHasher) Update(data []byte)
func (h *DataHasher) Finalize(bits int) (*DataCodeResult, error)
```

Streaming functions also accept `io.Reader` for file/network streaming.

## Dependencies

All well-maintained, pure Go:

| Module                               | Purpose                        |
| ------------------------------------ | ------------------------------ |
| `github.com/zeebo/blake3`            | BLAKE3 cryptographic hash      |
| `github.com/cespare/xxhash/v2`       | xxHash for feature hashing     |
| `golang.org/x/text/unicode/norm`     | Unicode NFKC/NFD normalization |
| `encoding/base32`, `encoding/base64` | Standard library encoding      |

## Distribution

Distributed via the Go module proxy — no upload step required. `go get` resolves the module directly
from the GitHub repository via `proxy.golang.org`.

```bash
go get github.com/iscc/iscc-lib/packages/go
```

No binary artifacts, no CGO, no platform-specific packages. Works on any platform Go supports.

## CI Integration

### CI Job (ci.yml)

```yaml
go:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: actions/setup-go@v5
      with:
        go-version: '1.23'
    - run: go vet ./...
      working-directory: packages/go
    - run: go test -v ./...
      working-directory: packages/go
      env:
        CGO_ENABLED: '0'
```

## Version Sync

| Target                   | What is synced            |
| ------------------------ | ------------------------- |
| `packages/go/version.go` | `const Version = "X.Y.Z"` |

Go module versions are determined by Git tags (`go/vX.Y.Z` sub-module tags). The embedded version
constant is synced for diagnostic purposes.

## Documentation

- **How-to guide**: `docs/howto/go.md`
- **API reference**: `docs/go-api.md`
- **Per-package README**: `packages/go/README.md`

## Verification Criteria

- [ ] `go test ./...` passes all conformance vectors from `data.json`
- [ ] Output of every `gen_*_v0` function matches `iscc-core` reference for every test vector
- [ ] Package installs cleanly via `go get` with no external dependencies beyond Go modules
- [ ] No cgo required (`CGO_ENABLED=0` works)
- [ ] Cross-compilation works (`GOOS`/`GOARCH` combinations)
- [ ] All 32 Tier 1 symbols accessible with idiomatic Go types and error handling
- [ ] API uses Go conventions: `PascalCase`, `error` returns, `[]byte`, `io.Reader`
- [ ] No binary artifacts committed to the repository
- [ ] `check-added-large-files` threshold at 256KB
- [ ] `go vet ./...` clean
- [ ] Version synced from root `Cargo.toml` via `mise run version:sync`
