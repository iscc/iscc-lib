# Next Work Package

## Step: Rename Go `EncodeIsccID` → `GenIsccIDV1` and delete `DecodeIsccID` / `IsccIDv1Result`

## Goal

Complete the Go surface of #43: replace the superseded `EncodeIsccID(realm, hubID, timestamp)` with
the canonical `GenIsccIDV1(timestamp, hubID, realm)` returning `*IsccIdResult`, and **delete**
`DecodeIsccID` and `IsccIDv1Result` (the reference has no IDv1 decoder; generic `IsccDecode` covers
it). Unblocks the waiting `iscc/iscc-monitor` port and removes the only actively-wrong IDv1 API.

## Alternatives Considered

- **Chosen:** Go rename — the only surface carrying superseded, must-delete API; a single-file
    non-test change that fully closes one #43 sub-item and has a concrete downstream consumer.
- **Rejected:** `gen_iscc_id_v1` minting on the napi/wasm surface — also #43, but one of 10 additive
    minting slices with no removal urgency; the Go divergence is the higher-value fix now.

## Scope

- **Modify**: `packages/go/iscc_id.go` (only non-test/non-doc file)
- **Modify (tests, excluded from budget)**: `packages/go/iscc_id_test.go`
- **Modify (docs, excluded from budget)**: `docs/howto/go.md` (§Experimental ISCC-IDv1, ~L392–403),
    `packages/go/README.md` (API table ~L92–93), `packages/go/CLAUDE.md` (codec-function list ~L80)
- **Reference**: `.claude/context/specs/go-bindings.md` → "ISCC-IDv1 Support" (canonical Go shape,
    field-extraction recipe, golden vector); `packages/go/codec.go` (`IsccDecode` result fields:
    `Subtype uint8`, `Digest []byte`)

## Not In Scope

- Adding `gen_iscc_id_v1` to any other surface (napi/wasm/ffi/jni/rb/uniffi) — each is its own step.
- Widening the Python `VS` enum or any other decode-round-trip work.
- The Tier-1 32→33 doc/count sweep and the `iscc_clean` codec-cleaning issue.
- Touching `codec.go` decode logic — `IsccDecode` already accepts `Id` Version 1 (do not change it).

## Implementation Notes

- New signature: `GenIsccIDV1(timestamp uint64, hubID uint16, realm uint8) (*IsccIdResult, error)`.
    Body packing and validation stay exactly as current `EncodeIsccID` — keep validation order
    (timestamp `< 2^52`, hubID `< 2^12`, realm `<= 1`) and the `iscc:`-prefixed error idiom.
- Add a new struct `IsccIdResult` with one exported field `ISCC string` carrying the `iscc` JSON
    tag; return it wrapping `"ISCC:" + encodeBase32(component)`. Delete both `IsccIDv1Result` and
    `DecodeIsccID` entirely.
- Field-extraction recipe (document + use in the round-trip test): `d, _ := IsccDecode(code)`, then
    `n := binary.BigEndian.Uint64(d.Digest)`, `timestamp := n >> 12`, `hubID := uint16(n & 0xFFF)`,
    `realm := d.Subtype`.
- Rewrite `iscc_id_test.go`: keep the golden-vector and validation tests against `GenIsccIDV1`;
    convert the decode/round-trip tests to `IsccDecode` + the extraction recipe over the boundary
    grid (hubID 0/4095, realm 0/1, max 52-bit timestamp). Keep the existing `IsccDecode`/
    `IsccDecompose`/`decodeHeader` Version-1 tests unchanged.
- Update the three docs to describe `GenIsccIDV1` + the extraction recipe; drop the "Go-only, other
    bindings reject Version 1" warning wording in `docs/howto/go.md` (core now accepts it).

## Verification

- `CGO_ENABLED=0 go test -count=1 ./...` passes in `packages/go/`.
- `go vet ./...` clean in `packages/go/`.
- `! grep -rq 'DecodeIsccID\|IsccIDv1Result' packages/go` (symbols fully removed from the tree).
- A Go test asserts `GenIsccIDV1(1751831876325218, 1, 0).ISCC == "ISCC:MAIGHFECJMOPMIAB"`.
- `grep -q GenIsccIDV1 docs/howto/go.md && grep -q GenIsccIDV1 packages/go/README.md` (docs
    updated).

## Done When

All verification checks pass: Go exposes `GenIsccIDV1` returning `*IsccIdResult`, `DecodeIsccID` /
`IsccIDv1Result` are gone, the golden vector round-trips through `IsccDecode`, and the three docs
describe the new shape and extraction recipe.
