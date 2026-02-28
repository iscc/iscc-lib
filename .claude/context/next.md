# Next Work Package

## Step: Add JsonToDataUrl to Go package (30/30)

## Goal

Implement the `JsonToDataUrl` public function in the Go package — the only missing Tier 1 symbol.
This completes Go bindings at 30/30 and moves the Go section from "partially met" to "met".

## Scope

- **Create**: (none)
- **Modify**: `packages/go/codec.go` (add `JsonToDataUrl` function), `packages/go/codec_test.go`
    (add tests)
- **Reference**: `crates/iscc-lib/src/lib.rs` (Rust `json_to_data_url` + `build_meta_data_url`),
    `packages/go/code_meta.go` (existing unexported helpers: `parseMetaJSON`, `jsonHasContext`,
    `buildMetaDataURL`)

## Not In Scope

- Renaming `TestPureGo*` test prefixes to `Test*` — cosmetic cleanup for a future iteration
- Go CI job simplification (removing old WASM build steps) — separate step
- PR from develop → main — separate step after this is verified
- Refactoring the existing unexported helpers in code_meta.go — they work fine as-is

## Implementation Notes

The function composes three existing unexported helpers already in `code_meta.go`:

```go
// JsonToDataUrl converts a JSON string to a data-URL with base64-encoded canonical JSON.
//
// Parses and canonicalizes the JSON (sorted keys, compact format). If the JSON
// contains an "@context" key, uses "application/ld+json" media type; otherwise
// uses "application/json".
func JsonToDataUrl(jsonStr string) (string, error) {
    canonical, err := parseMetaJSON(jsonStr)
    if err != nil {
        return "", err
    }
    hasContext := jsonHasContext(jsonStr)
    return buildMetaDataURL(canonical, hasContext), nil
}
```

Place the function in `codec.go` alongside `EncodeBase64` and `EncodeComponent` (matching the Rust
grouping of encoding utilities).

**Tests to add** (in `codec_test.go`):

1. Basic JSON → `data:application/json;base64,...` prefix
2. JSON with `@context` → `data:application/ld+json;base64,...` prefix
3. JCS key ordering: `{"b":1,"a":2}` → decoded payload has keys sorted `{"a":2,"b":1}`
4. Invalid JSON → returns error
5. Cross-check with Rust output for conformance vector `test_0016_meta_data_url` input
    (`{"some": "object"}`) — verify identical data URL output

Match the Rust test patterns from `lib.rs` lines 1912-1995. The Go `encoding/json` Marshal produces
JCS-compatible output for string-only JSON (sorted keys, compact) — no external JCS library needed.

## Verification

- `cd packages/go && go build ./...` exits 0
- `cd packages/go && go test ./... -count=1` passes (all existing + new JsonToDataUrl tests)
- `cd packages/go && go vet ./...` exits 0
- `grep -q 'func JsonToDataUrl' packages/go/codec.go` exits 0 (function exists)
- `grep -q 'JsonToDataUrl' packages/go/codec_test.go` exits 0 (tests exist)
- `mise run check` passes (all hooks green)

## Done When

All verification criteria pass, confirming `JsonToDataUrl` is implemented, tested, and the Go
package compiles and passes all tests including the new ones.
