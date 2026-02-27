# Next Work Package

## Step: Implement pure Go GenIsccCodeV0

## Goal

Implement `GenIsccCodeV0` — the 9th and final gen function for the pure Go rewrite. This function
assembles individual ISCC unit codes (Meta, Content, Data, Instance) into a composite ISCC-CODE,
completing the full set of code generation functions needed before conformance selftest and cleanup.

## Scope

- **Create**:
    - `packages/go/code_iscc.go` — `GenIsccCodeV0` function + `IsccCodeResult` struct
    - `packages/go/code_iscc_test.go` — conformance test against 5 data.json vectors
- **Modify**: (none)
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines 819–934 — Rust `gen_iscc_code_v0` implementation
    - `crates/iscc-lib/src/codec.rs` lines 329–397 — `encodeUnits` / `decodeUnits`
    - `packages/go/codec.go` — Go `encodeUnits`, `decodeBase32`, `decodeHeader`, `decodeLength`,
        `encodeHeader`, `encodeBase32` (all already implemented)
    - `packages/go/code_content_mixed.go` — pattern for prefix stripping, decoding, error handling
    - `crates/iscc-lib/tests/data.json` — `gen_iscc_code_v0` section (5 vectors)

## Not In Scope

- `conformance_selftest` — that's the next step after all 9 gen functions are done
- WASM bridge cleanup (removing `iscc.go`, `iscc_ffi.wasm`, wazero dep) — separate step
- Restoring `.pre-commit-config.yaml` large-file threshold — part of cleanup step
- Adding `GenIsccCodeV0` to Go README API table — doc updates after all gen functions are done
- Wide mode testing — no test vectors exercise `wide=true`; implement the logic but don't add extra
    unit tests for it

## Implementation Notes

Port `gen_iscc_code_v0` from Rust (`crates/iscc-lib/src/lib.rs` lines 819–934). Key algorithm steps:

1. **Clean inputs**: strip `"ISCC:"` prefix from each code string
2. **Validate**: minimum 2 codes, each at least 16 base32 chars
3. **Decode each code**: `decodeBase32` + `decodeHeader` →
    `(MainType, SubType, Version, length, body)`
4. **Sort by MainType** ascending — use `sort.Slice` on the decoded entries' MainType value
5. **Validate mandatory**: last two sorted entries must be `MTData` + `MTInstance`
6. **Wide mode**: `wide && len==2 && types==[Data,Instance] && both decodeLength>=128`. Test
    vectors never use wide — always pass `false`. Implement the logic but it won't be exercised
7. **SubType determination**:
    - If wide → `STWide`
    - Else: collect SubTypes of Semantic/Content codes. If any exist and all same → use that
        SubType. If different SubTypes → error. If no Semantic/Content codes and len==2 → `STSum`.
        If no Semantic/Content codes and len>2 → `STIsccNone`
8. **Encode units**: call `encodeUnits(optionalMainTypes)` on the MainTypes from all entries except
    the last two (Data+Instance). These are the "optional" types: Meta, Content, Semantic
9. **Build digest body**: for each decoded entry, take first `bytesPerUnit` bytes from body (8 for
    standard, 16 for wide). Concatenate all
10. **Encode header + digest**: `encodeHeader(MTIscc, subtype, VSV0, encodedLength)` + digest →
    `encodeBase32` to get the final code string
11. **Return**: `&IsccCodeResult{Iscc: "ISCC:" + code}`

**Result struct**: `IsccCodeResult` with single `Iscc string` field (matches Rust `IsccCodeResult`).

**Codec functions available** (all in `packages/go/codec.go`, same `iscc` package):

- `decodeBase32(s string) ([]byte, error)` — unexported
- `decodeHeader(data []byte) (MainType, SubType, Version, uint32, []byte, error)` — unexported
- `decodeLength(mtype MainType, length uint32, stype SubType) uint32` — unexported
- `encodeUnits(mainTypes []MainType) (uint32, error)` — unexported
- `encodeHeader(mtype MainType, stype SubType, version Version, length uint32) ([]byte, error)` —
    unexported
- `encodeBase32(data []byte) string` — unexported

**Test pattern**: Follow `code_data_test.go`. The `gen_iscc_code_v0` vectors have:

- `inputs`: `[[code1, code2, ...]]` — single element: an array of ISCC code strings (WITHOUT prefix)
- `outputs`: `{"iscc": "ISCC:..."}` — expected composite ISCC-CODE
- No `wide` or `bits` parameter in the vectors — call `GenIsccCodeV0(codes, false)`
- Test function name: `TestPureGoGenIsccCodeV0`
- Parse `inputs[0]` as `[]string` (the array of code strings), no `inputs[1]`

**5 conformance vectors**:

- `test_0000_standard`: 4 codes (Meta+Content+Data+Instance)
- `test_0001_no_meta`: 3 codes (Content+Data+Instance)
- `test_0002_no_meta_content_256`: 3 codes (Content-256bit+Data+Instance)
- `test_0003_no_meta_content_128`: 3 codes (Content-128bit+Data+Instance)
- `test_0004_ordering`: Same codes as test_0000 in different order (must produce same output)

## Verification

- `cd packages/go && go build ./...` exits 0
- `cd packages/go && go test -run TestPureGoGenIsccCodeV0 -count=1 -v` — 5/5 vectors PASS
- `cd packages/go && go vet ./...` exits 0
- `cd packages/go && go test ./...` — all tests pass (pure Go + WASM bridge)
- `grep -c 'func GenIsccCodeV0' packages/go/code_iscc.go` returns 1

## Done When

All 5 verification checks pass, confirming GenIsccCodeV0 produces correct ISCC-CODEs for all
conformance vectors and coexists cleanly with existing pure Go and WASM bridge code.
