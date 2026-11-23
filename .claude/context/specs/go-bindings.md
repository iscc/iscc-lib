# Spec: Go Bindings — Pure Go Implementation

A pure Go module consumable via `go get`, providing idiomatic Go access to all ISCC functions. This
is a native Go implementation — not an FFI wrapper, CGO bridge, or WASM embed. Go developers get
`go get` with no binary artifacts, no C toolchain, and full source-level debugging.

## Architecture

**Pure Go (not FFI/WASM bridge):**

- Native Go implementation of all ISCC algorithms (CDC, MinHash, SimHash, DCT, WTA-Hash)
- No CGO, no WASM, no embedded binaries — just Go source code
- Idiomatic Go API with Go naming conventions, `error` returns, `[]byte` slices, and `Push`/
    `Finalize` hashers for streaming
- Lives in this repository under `packages/go/` as a Go sub-module
- Validated against the same conformance test vectors as all other bindings

**Why pure Go (not WASM/wazero bridge):**

- **Zero distribution friction** — `go get` fetches source code only, no binary artifacts in git
- **Native performance** — compiled to machine code, no WASM interpreter overhead. The BLAKE3 Go
    library has a SIMD-optimized implementation
- **First-class debugging** — Go developers can step into ISCC code, profile it, read the source
- **Cross-compilation works** — `GOOS=linux GOARCH=arm64 go build` just works
- **No build artifacts in git** — avoids polluting git history with binaries

## Package Structure

```
packages/go/
├── go.mod                  # Module: github.com/iscc/iscc-lib/packages/go
├── go.sum
├── code_*.go               # One file per code type (gen_*_v0 functions)
├── codec.go                # Header encode/decode, base32, component encoding
├── testdata/
│   └── data.json           # Vendored conformance vectors
└── README.md               # Per-package README
```

Algorithms (CDC, MinHash, SimHash, DCT, WTA-Hash, xxh32) and utilities live in one `.go` file each,
next to a `_test.go` file. The tree lists the load-bearing entries, not every file.

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
| `alg_minhash_256`      | `AlgMinhash256(features []uint32) ([]byte, error)`                 |
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
func (h *DataHasher) Push(data []byte) error
func (h *DataHasher) Finalize(bits uint32) (*DataCodeResult, error)
```

Hashers are single-use: the first `Finalize` call consumes the hasher, even when it fails. After
that, `Push` and `Finalize` return an "already finalized" error.

## ISCC-IDv1 Support (Experimental)

GitHub: https://github.com/iscc/iscc-lib/issues/43

The Go binding decodes and encodes **ISCC-IDv1** — the timestamp-based ID produced by `iscc-hub` and
specified in iscc-core's `iscc_core/iscc_id.py` — at parity with the Python reference. ISCC-IDv1 is
**not part of ISO 24138** and still evolving, so the API is marked experimental in doc comments (may
change in a minor release).

**Structure** (per `iscc_id.py` / iscc-hub):

- 80-bit code = 16-bit header + 64-bit body; 16 base32 chars (RFC 4648 uppercase, no padding),
    optional `ISCC:` prefix
- Header nibbles: MainType = 6 (`ID`), SubType = realm (0 = sandbox/test, 1 = operational), Version
    = 1, Length = 0 (canonical 64-bit body)
- Body (big-endian `uint64`): `timestamp = body >> 12` (52-bit microseconds since epoch),
    `hub_id = body & 0xFFF` (low 12 bits, issuing-hub slot 0–4095)

**API shape** — canonical definition lives in `rust-core.md` → "ISCC-IDv1 Operations
(Experimental)"; this section covers only what is Go-specific:

- `GenIsccIDV1(timestamp uint64, hubID uint16, realm uint8) (*IsccIdResult, error)` — the Go casing
    of the cross-binding name `gen_iscc_id_v1`, using the reference parameter order
    `(timestamp, hub_id, realm)`. The result carries only `ISCC string`, mirroring iscc-core's
    `{"iscc": ...}`
- `decodeHeader` accepts Version = 1 when MainType = `ID` (with the matching `decodeLength` rule),
    so generic `IsccDecode` returns header + 64-bit body instead of rejecting with
    `iscc: invalid Version: 1`; all other MainTypes continue to reject Version > 0
- Go keeps its own error idiom (`iscc:`-prefixed messages); only the validation *order* is normative

**Rename note, and one deletion.** Iteration 119 shipped `EncodeIsccID(realm, hubID, timestamp)` /
`DecodeIsccID` — the shape issue #43 originally called "preferred". On 2026-07-28 the human settled
both halves differently:

- `EncodeIsccID` → `GenIsccIDV1`, with the reference parameter order, matching `iscc_core`'s
    `gen_iscc_id_v1` and every other binding
- `DecodeIsccID` and its `IsccIDv1Result` type are **deleted**, not renamed. `iscc-core` has no IDv1
    decoder; generic `IsccDecode` covers it. Go callers extract the fields directly:
    `n := binary.BigEndian.Uint64(body)`, then `timestamp := n >> 12`, `hubID := uint16(n & 0xFFF)`,
    `realm := st`

Neither old symbol appeared in a tagged release: `GenIsccIDV1` is the first IDv1 API any release
shipped (0.6.0), so no published consumer was affected.

**Conformance vector:** `ISCC:MAIGHFECJMOPMIAB` → realm 0, hub_id 1, timestamp 1751831876325218
(from iscc-hub's `schema.py` example).

**Downstream context:** `iscc/iscc-monitor` ships an interim in-repo port of this codec
(`internal/index/iscc.go`, tracked monitor-side in ADR-0011) and wants to delete it once the Go
binding supports ISCC-IDv1.

**Verified when:**

- `GenIsccIDV1(1751831876325218, 1, 0)` returns `ISCC:MAIGHFECJMOPMIAB` (round-trip through
    `IsccDecode` holds for boundary values: hub_id 0 and 4095, realm 0 and 1, max 52-bit timestamp)
- `packages/go` exports neither `DecodeIsccID` nor `IsccIDv1Result`, and the field-extraction recipe
    is documented in `docs/howto/go.md` and `packages/go/README.md`
- `IsccDecode` accepts MainType `ID` with Version 1 and returns the header + 64-bit body
- Version > 0 is still rejected for every MainType other than `ID`
- Public ISCC-IDv1 symbols carry an "experimental" doc-comment marker

## Dependencies

All well-maintained, pure Go:

| Module                               | Purpose                        |
| ------------------------------------ | ------------------------------ |
| `github.com/zeebo/blake3`            | BLAKE3 cryptographic hash      |
| `golang.org/x/text/unicode/norm`     | Unicode NFKC/NFD normalization |
| `encoding/base32`, `encoding/base64` | Standard library encoding      |

The xxh32 feature hash is implemented in the package (`packages/go/xxh32.go`), so it needs no
module.

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

Go module versions are determined by Git tags (`packages/go/vX.Y.Z` sub-module tags). The package
embeds no package-version string, so there is no sync target.

## Documentation

- **How-to guide**: `docs/howto/go.md`
- **Per-package README**: `packages/go/README.md`

## Verification Criteria

- `go test ./...` passes all conformance vectors from `data.json`
- Output of every `gen_*_v0` function matches `iscc-core` reference for every test vector
- Package installs cleanly via `go get` with no external dependencies beyond Go modules
- No cgo required (`CGO_ENABLED=0` works)
- Cross-compilation works (`GOOS`/`GOARCH` combinations)
- All Tier 1 symbols accessible with idiomatic Go types and error handling
- API uses Go conventions: `PascalCase`, `error` returns, `[]byte`
- No binary artifacts committed to the repository
- `check-added-large-files` threshold at 256KB
- `go vet ./...` clean
- Module version set by `packages/go/vX.Y.Z` Git tags
