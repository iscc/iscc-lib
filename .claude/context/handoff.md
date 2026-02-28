## 2026-02-28 — Add JsonToDataUrl to Go package (30/30)

**Done:** Implemented `JsonToDataUrl` public function in `packages/go/codec.go`, composing three
existing unexported helpers (`parseMetaJSON`, `jsonHasContext`, `buildMetaDataURL`) from
`code_meta.go`. Added 5 tests covering basic JSON, LD+JSON with `@context`, JCS key ordering,
invalid JSON error, and cross-check with Rust conformance vector `test_0016_meta_data_url`.

**Files changed:**

- `packages/go/codec.go`: Added `JsonToDataUrl` function between `EncodeBase64` and Component
    Encoding section
- `packages/go/codec_test.go`: Added 5 test functions and `encoding/base64` import for payload
    decoding in JCS/conformance tests

**Verification:**

- `cd packages/go && go build ./...` — exits 0
- `cd packages/go && go test ./... -count=1` — all tests pass (0.055s)
- `cd packages/go && go vet ./...` — exits 0
- `grep -q 'func JsonToDataUrl' packages/go/codec.go` — exits 0
- `grep -q 'JsonToDataUrl' packages/go/codec_test.go` — exits 0
- `mise run check` — all 14 hooks pass
- `cargo test -p iscc-lib` — all Rust tests pass

**Next:** Go bindings are now at 30/30 Tier 1 symbols with zero WASM dependencies. Suggested next
priorities: (1) CI simplification — remove old WASM build steps from Go CI job, (2) PR from develop
→ main — the Go rewrite is a major milestone, (3) remaining partially-met areas from target.md.

**Notes:** No surprises. The implementation is exactly as specified in next.md — a thin public
wrapper around three existing unexported helpers. No new dependencies needed since all helpers are
in the same package.
