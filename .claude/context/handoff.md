## 2026-02-27 — Implement pure Go GenMetaCodeV0 and GenTextCodeV0

**Done:** Implemented xxh32, GenTextCodeV0, and GenMetaCodeV0 as pure Go functions matching the Rust
reference implementation. All 21 conformance vectors pass (5 text + 16 meta). Added
`github.com/zeebo/blake3` dependency for BLAKE3 hashing. Used Go stdlib `json.Marshal` for JCS (RFC
8785\) canonicalization instead of an external library — Go's json.Marshal already produces sorted
keys with compact separators, which is JCS-compatible for the string/null values in our conformance
vectors.

**Files changed:**

- `packages/go/xxh32.go`: xxh32 hash function (standalone, ~80 lines) with prime constants, 4-lane
    accumulation, remainder processing, and avalanche mixing
- `packages/go/xxh32_test.go`: xxh32 unit tests (canonical empty/seed=0 vector + determinism +
    property tests)
- `packages/go/code_content_text.go`: GenTextCodeV0, softHashTextV0, TextCodeResult struct
- `packages/go/code_content_text_test.go`: 5 conformance vector tests from data.json
- `packages/go/code_meta.go`: GenMetaCodeV0, softHashMetaV0, softHashMetaV0WithBytes,
    metaNameSimhash, interleaveDigests, slidingWindowBytes, decodeDataURL, parseMetaJSON,
    jsonHasContext, buildMetaDataURL, multiHashBlake3, MetaCodeResult struct
- `packages/go/code_meta_test.go`: 16 conformance vector tests from data.json
- `packages/go/go.mod`: added `github.com/zeebo/blake3` (+ transitive
    `github.com/klauspost/cpuid/v2`)
- `packages/go/go.sum`: updated with new dependency checksums

**Verification:**

- [x] `cd packages/go && go build ./...` exits 0
- [x] `cd packages/go && go test -run TestXxh32 -count=1 -v` — 8 tests PASS
- [x] `cd packages/go && go test -run TestPureGoGenTextCodeV0 -count=1 -v` — 5/5 text vectors PASS
- [x] `cd packages/go && go test -run TestPureGoGenMetaCodeV0 -count=1 -v` — 16/16 meta vectors PASS
- [x] `cd packages/go && go vet ./...` exits 0
- [x] `cd packages/go && go test ./...` — all tests pass (pure Go + WASM bridge)
- [x] `grep -c 'func GenTextCodeV0' packages/go/code_content_text.go` returns 1
- [x] `grep -c 'func GenMetaCodeV0' packages/go/code_meta.go` returns 1
- [x] `mise run check` — all 14 hooks pass

**Next:** Implement gen_data_code_v0 + gen_instance_code_v0 (next sub-step of gen functions layer).
These need CDC + MinHash + BLAKE3 (all available now) plus DataHasher/InstanceHasher streaming
types. 4 conformance vectors for data + 3 for instance.

**Notes:** Test function names use `TestPureGo*` prefix to avoid conflicts with existing WASM bridge
tests in iscc_test.go (which already has `TestGenMetaCodeV0` and `TestGenTextCodeV0`). The JCS
approach uses Go stdlib json.Marshal instead of an external library — this works because Go's
json.Marshal already sorts map keys and uses compact output. For full RFC 8785 compliance (e.g.,
float number formatting), a dedicated library would be needed, but our conformance vectors only
contain string values.
