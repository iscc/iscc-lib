## 2026-02-28 — Remove WASM bridge from Go bindings

**Done:** Removed the WASM/wazero bridge (`iscc.go`, `iscc_test.go`, `iscc_ffi.wasm`) from the Go
package. Relocated `DecodeResult` struct and 4 algorithm constants (`MetaTrimName`,
`MetaTrimDescription`, `IoReadSize`, `TextNgramSize`) to `codec.go`. Removed wazero dependency from
`go.mod` and ran `go mod tidy`. Restored large-file threshold from 1024KB to 256KB in
`.pre-commit-config.yaml`.

**Files changed:**

- `packages/go/codec.go`: Added algorithm constants block and `DecodeResult` struct (relocated from
    `iscc.go`); updated `IsccDecode` comment to remove "defined in iscc.go" reference
- `packages/go/go.mod`: Removed `github.com/tetratelabs/wazero v1.11.0` from requires
- `packages/go/go.sum`: Cleaned by `go mod tidy` (removed wazero + golang.org/x/sys entries)
- `.pre-commit-config.yaml`: Changed `--maxkb=1024` to `--maxkb=256`
- `packages/go/iscc.go`: Deleted (1,357-line WASM bridge)
- `packages/go/iscc_test.go`: Deleted (46 WASM bridge tests)
- `packages/go/iscc_ffi.wasm`: Deleted (667KB binary)

**Verification:** All verification criteria pass:

- `go build ./...` exits 0
- `go test ./... -count=1` passes (all pure Go tests including conformance, 0.059s)
- `go vet ./...` exits 0
- All 3 WASM files confirmed deleted
- wazero confirmed absent from go.mod
- `--maxkb=256` confirmed in `.pre-commit-config.yaml`
- `DecodeResult` and constants confirmed in `codec.go`
- `mise run check` passes all 14 hooks

**Next:** The Go pure rewrite is complete. All 30/30 Tier 1 symbols are implemented in pure Go with
zero non-Go dependencies (only stdlib + `golang.org/x/text` + `github.com/zeebo/blake3`). Consider
updating CI to remove WASM build steps if they exist, or focus on the next target milestone (Python
bindings, additional platform support, or performance benchmarking).

**Notes:** The `go mod tidy` also removed `golang.org/x/sys` as an indirect dependency — it was only
needed transitively by wazero. The Go module now has a smaller dependency footprint. No surprises or
blockers encountered. The pure Go test naming convention (`TestPureGo*` prefix) is no longer needed
since the WASM bridge tests are gone, but renaming existing tests is out of scope for this step.
