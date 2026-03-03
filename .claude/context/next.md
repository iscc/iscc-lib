# Next Work Package

## Step: Vendor iscc-core v1.3.0 data.json and fix Go conformance loader

## Goal

Vendor the updated conformance test vectors from iscc-core v1.3.0 (4 new Meta-Code tests) and fix
the Go conformance loader that will break on the new `_metadata` top-level key. This resolves the
data foundation for the critical iscc-core v1.3.0 conformance issue.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-lib/tests/data.json` — replace with v1.3.0 vectors (copy from reference)
    - `packages/go/testdata/data.json` — replace with v1.3.0 vectors (same copy)
    - `packages/go/conformance.go` — fix JSON unmarshaling to tolerate `_metadata` key; update vector
        count comment from 46 to 50
- **Reference**:
    - `reference/iscc-core/iscc_core/data.json` (after checkout to v1.3.0)
    - `crates/iscc-lib/src/conformance.rs` — verify Rust loader needs no changes (it accesses sections
        by name, not iterating all keys)
    - `crates/iscc-lib/src/lib.rs` — verify META_TRIM_META (128,000) and JCS float canonicalization
        already implemented
    - `packages/go/code_meta.go` — verify Go JCS handling via `json.Marshal`

## Not In Scope

- Implementing `iscc_validate` codec validation tightening (we don't have this function)
- Implementing `iscc_nph_compare` (new in v1.3.0, not conformance-critical)
- Fixing Go JCS number canonicalization if `json.Marshal` doesn't match RFC 8785 for edge-case
    floats — that would be a separate step touching `packages/go/code_meta.go`
- Updating binding conformance tests other than Go (Rust, Python, Node.js, WASM, Java, C FFI all
    access data.json by section name and handle `_metadata` gracefully)
- Updating the `reference/iscc-core` shallow clone commit permanently (it's gitignored)
- Closing the critical issue in issues.md (review agent handles that)

## Implementation Notes

### Vendoring data.json

1. Update the reference clone to v1.3.0:
    ```bash
    cd reference/iscc-core && git fetch --depth 1 origin tag v1.3.0 && git checkout v1.3.0
    ```
2. Copy `reference/iscc-core/iscc_core/data.json` to both:
    - `crates/iscc-lib/tests/data.json`
    - `packages/go/testdata/data.json`

### Go conformance loader fix

The current Go `ConformanceSelftest()` on line 29 of `conformance.go` parses as:

```go
var data map[string]map[string]vectorEntry
```

This tries to unmarshal ALL top-level keys as `map[string]vectorEntry`. The new `_metadata` key has
flat string values (`"generated"`, `"generator"`, `"description"`), not `vectorEntry` structs with
`inputs`/`outputs`. This will cause `json.Unmarshal` to fail.

**Fix approach**: Parse as `map[string]json.RawMessage` first, then unmarshal each section
individually (skipping keys starting with `_`). Or: use a two-pass approach where you parse to
`map[string]json.RawMessage`, delete keys starting with `_`, then unmarshal the remaining into
`map[string]map[string]vectorEntry`. The simpler approach is to change the parse type.

### Why Rust core needs no changes

The Rust `conformance.rs` accesses each function section by name (e.g., `data["gen_meta_code_v0"]`)
using `serde_json::Value`, which naturally ignores unrecognized keys. The 4 new vectors
(`test_0017`–`test_0020`) are under `gen_meta_code_v0` and will be automatically iterated by
`run_meta_tests`. The existing implementation already handles:

- **JCS float canonicalization**: `serde_json_canonicalizer` crate (see tests at ~line 1094)
- **META_TRIM_META = 128,000**: constant + pre-decode/post-decode validation (lines 317, 333-338)
- **Description trimming**: `text_trim(text_clean(desc), META_TRIM_DESCRIPTION)` where
    `META_TRIM_DESCRIPTION = 4096`

### Other bindings (no changes needed)

- Python, Node.js, WASM, Java, C FFI conformance tests all access data.json by section name (not
    iterating all top-level keys), so the `_metadata` key is naturally ignored.

## Verification

- `cargo test -p iscc-lib` passes (should be 318+ tests — existing 314 + 4 new conformance vectors
    auto-discovered by `run_meta_tests`)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cd packages/go && go test ./...` passes
- `cd packages/go && go vet ./...` clean
- `grep -c 'test_0017\|test_0018\|test_0019\|test_0020' crates/iscc-lib/tests/data.json` returns 4
    (new vectors present)
- `grep '_metadata' crates/iscc-lib/tests/data.json` finds the metadata header (confirms v1.3.0)

## Done When

All verification criteria pass — the vendored data.json contains v1.3.0 vectors (50 total), Rust
core passes all conformance tests including the 4 new ones, and Go conformance loads and passes
without `_metadata` parsing errors.
