## 2026-02-28 — Implement pure Go ConformanceSelftest

**Done:** Implemented `ConformanceSelftest() (bool, error)` as a pure Go package-level function that
validates all 46 conformance vectors from `data.json` in a single call. The function uses
`//go:embed testdata/data.json` for compile-time embedding, mirrors the Rust `conformance.rs`
structure with 9 section runners (one per gen function), and reports failures via
`fmt.Fprintf(os.Stderr, ...)`.

**Files changed:**

- `packages/go/testdata/data.json`: Copied from `crates/iscc-lib/tests/data.json` (conformance
    vectors)
- `packages/go/conformance.go`: New file with `ConformanceSelftest()`, 9 `run*Tests` section
    runners, `decodeStream` helper, and `vectorEntry` struct for JSON parsing
- `packages/go/conformance_test.go`: New file with `TestPureGoConformanceSelftest`

**Verification:**

- [x] `cd packages/go && go build ./...` exits 0
- [x] `cd packages/go && go test -run TestPureGoConformanceSelftest -count=1 -v` — PASS (all 46
    vectors)
- [x] `cd packages/go && go vet ./...` exits 0
- [x] `cd packages/go && go test ./...` — all tests pass (pure Go + WASM bridge, 9.581s)
- [x] `grep -c 'func ConformanceSelftest' packages/go/conformance.go` returns 1
- [x] `test -f packages/go/testdata/data.json` exits 0
- [x] `mise run check` — all 14 hooks pass

**Next:** All 9 gen functions + ConformanceSelftest are complete — the pure Go implementation is
functionally complete with 30/30 Tier 1 symbol coverage. The next step should be WASM bridge
cleanup: removing `iscc.go`, `iscc_ffi.wasm`, the wazero dependency, and restoring the
`.pre-commit-config.yaml` large-file threshold to 256KB.

**Notes:** The `runMetaCase` helper was extracted as a separate function (rather than inline in
`runMetaTests`) because the meta input parsing is more complex (null/string/object for meta,
null/string for description). All other section runners use inline parsing with `continue` on error.
The `vectorEntry` type is unexported and only used within this file. The `decodeStream` helper is
shared between `runDataTests` and `runInstanceTests`.
