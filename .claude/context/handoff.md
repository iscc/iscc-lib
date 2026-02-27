## 2026-02-27 — Implement pure Go GenIsccCodeV0

**Done:** Implemented `GenIsccCodeV0` — the 9th and final gen function for the pure Go rewrite. This
function assembles individual ISCC unit codes (Meta, Content, Data, Instance) into a composite
ISCC-CODE, faithfully porting the Rust `gen_iscc_code_v0` implementation.

**Files changed:**

- `packages/go/code_iscc.go`: New file with `IsccCodeResult` struct and `GenIsccCodeV0` function
- `packages/go/code_iscc_test.go`: Conformance test `TestPureGoGenIsccCodeV0` against 5 data.json
    vectors

**Verification:**

- `cd packages/go && go build ./...` — exits 0
- `cd packages/go && go test -run TestPureGoGenIsccCodeV0 -count=1 -v` — 5/5 vectors PASS
- `cd packages/go && go vet ./...` — exits 0
- `cd packages/go && go test ./...` — all tests pass (pure Go + WASM bridge)
- `grep -c 'func GenIsccCodeV0' packages/go/code_iscc.go` — returns 1
- `mise run check` — all 14 hooks pass

**Next:** All 9 gen functions are now complete (Meta, Text, Image, Audio, Video, Mixed, Data,
Instance, IsccCode). The next step should be implementing `conformance_selftest` — a function that
validates all 46 conformance vectors from data.json, ensuring the complete pure Go implementation
matches the reference. After that: WASM bridge cleanup (removing `iscc.go`, `iscc_ffi.wasm`, wazero
dependency, restoring large-file threshold).

**Notes:** Implementation was straightforward — all codec helpers (`decodeBase32`, `decodeHeader`,
`encodeUnits`, `encodeHeader`, `encodeBase32`, `decodeLength`) were already available in `codec.go`.
The `wide` mode logic is implemented but not exercised by test vectors (all 5 vectors use
`wide=false`). Test vector `test_0004_ordering` confirms that sort-by-MainType produces identical
output regardless of input order.
