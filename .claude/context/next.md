# Next Work Package

## Step: Update Go docs for 30/30 Tier 1 symbols

## Goal

Update `docs/howto/go.md` and `packages/go/README.md` to document the 7 newly added Go symbols (4
constants + 3 functions) and replace stale "planned" text, closing the last documentation gap
identified by the state assessment.

## Scope

- **Create**: (none)
- **Modify**: `docs/howto/go.md`, `packages/go/README.md`
- **Reference**: `packages/go/iscc.go` (exact method signatures, constants, `DecodeResult` struct),
    `docs/howto/java.md` (for codec operations section pattern), `docs/howto/python.md` (for
    constants/codec section style)

## Not In Scope

- Deleting resolved issues from `issues.md` — the review agent handles issue cleanup
- Updating the root `README.md` — it already covers all 6 bindings and lists the 9 `gen_*_v0`
    functions
- Adding new Go tests or modifying Go source code
- Updating `docs/architecture.md` or `docs/development.md`

## Implementation Notes

### `docs/howto/go.md` — add two new sections

**1. Codec operations section** (insert after "Algorithm primitives", before "Conformance testing"):

Add a `## Codec operations` section covering these 6 functions:

- `EncodeBase64(ctx, data []byte) (string, error)` — encode bytes to base64
- `JsonToDataUrl(ctx, jsonStr string) (string, error)` — convert JSON string to
    `data:application/json;base64,...` URL
- `EncodeComponent(ctx, mtype, stype, version uint8, bitLength uint32, digest []byte) (string, error)`
    — construct an ISCC unit from raw header fields and digest
- `IsccDecode(ctx, isccUnit string) (*DecodeResult, error)` — decode an ISCC unit string into its
    header components and raw digest; returns `*DecodeResult` with `Maintype`, `Subtype`, `Version`,
    `Length`, `Digest` fields
- `IsccDecompose(ctx, isccCode string) ([]string, error)` — decompose a composite ISCC-CODE into
    individual unit codes
- `SoftHashVideoV0(ctx, frameSigs [][]int32, bits uint32) ([]byte, error)` — compute video soft hash
    from frame signatures

Include short Go code examples showing encode/decode roundtrip and decompose usage. Look at
`docs/howto/java.md` for the codec section structure to follow a consistent pattern across guides.

**2. Algorithm constants section** (insert after "Codec operations"):

Add a `## Constants` section listing the 4 package-level constants:

```go
iscc.MetaTrimName        // 128 — max byte length for name normalization
iscc.MetaTrimDescription // 4096 — max byte length for description normalization
iscc.IoReadSize          // 4_194_304 — default read buffer size (4 MB)
iscc.TextNgramSize       // 13 — n-gram size for text similarity hashing
```

Emphasize these are package-level constants (imported directly), not `Runtime` methods.

### `packages/go/README.md` — update API Overview

Replace the stale Utilities subsection (lines 78-83) which says "Additional utilities (text
processing, algorithm primitives, streaming hashers) are planned." with a complete listing organized
into subsections matching the full 30/30 symbol set:

- **Text processing**: `TextClean`, `TextRemoveNewlines`, `TextTrim`, `TextCollapse`
- **Algorithm primitives**: `SlidingWindow`, `AlgMinhash256`, `AlgCdcChunks`, `AlgSimhash`,
    `SoftHashVideoV0`
- **Codec operations**: `EncodeBase64`, `JsonToDataUrl`, `EncodeComponent`, `IsccDecode`,
    `IsccDecompose`
- **Streaming**: `DataHasher`, `InstanceHasher` (with `NewDataHasher`/`NewInstanceHasher` + `Update`
    - `Finalize` + `Close`)
- **Constants**: `MetaTrimName`, `MetaTrimDescription`, `IoReadSize`, `TextNgramSize`
- **Diagnostics**: `ConformanceSelftest`

Use a table format consistent with the existing Code Generators table in the same file.

## Verification

- `grep -c 'EncodeComponent' docs/howto/go.md` returns ≥ 1
- `grep -c 'IsccDecode' docs/howto/go.md` returns ≥ 1
- `grep -c 'IsccDecompose' docs/howto/go.md` returns ≥ 1
- `grep -c 'JsonToDataUrl' docs/howto/go.md` returns ≥ 1
- `grep -c 'EncodeBase64' docs/howto/go.md` returns ≥ 1
- `grep -c 'SoftHashVideoV0' docs/howto/go.md` returns ≥ 1
- `grep -c 'MetaTrimName' docs/howto/go.md` returns ≥ 1
- `grep 'are planned' packages/go/README.md` returns no matches (stale text removed)
- `grep -c 'EncodeComponent' packages/go/README.md` returns ≥ 1
- `grep -c 'DataHasher' packages/go/README.md` returns ≥ 1
- `uv run zensical build` exits 0 (docs site builds cleanly)

## Done When

All verification criteria pass — both Go documentation files are updated to cover the full 30/30
Tier 1 symbol set with no stale "planned" text remaining.
