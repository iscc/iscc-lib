# Next Work Package

## Step: Go ISCC-IDv1 encode/decode (issue #43)

## Goal

Add experimental ISCC-IDv1 support to the pure-Go binding: `EncodeIsccID` / `DecodeIsccID` exposing
realm, hub-id, and timestamp, plus Version=1 acceptance in `decodeHeader` for MainType `ID` only —
at parity with iscc-core's `iscc_id.py`. This closes the last non-CI Go target gap and unblocks
`iscc/iscc-monitor` deleting its interim in-repo codec port (their ADR-0011).

## Scope

- **Create**: `packages/go/iscc_id.go` (`EncodeIsccID` / `DecodeIsccID` functions + `IsccIDv1Result`
    type), `packages/go/iscc_id_test.go` (unit + round-trip tests)
- **Modify**: `packages/go/codec.go` (add `VSV1 Version = 1` const; relax the `decodeHeader` version
    check to accept Version=1 when MainType==ID, still rejecting Version>0 for all other MainTypes),
    `packages/go/README.md` (add the two experimental functions to the codec-functions API table)
- **Reference**: `reference/iscc-core/iscc_core/iscc_id.py` (`gen_iscc_id_v1` — the authoritative
    algorithm), `.claude/context/specs/go-bindings.md` → "ISCC-IDv1 Support (Experimental)" (the
    five checkbox acceptance criteria), `packages/go/codec.go` (existing `encodeHeader`,
    `encodeLength`, `IsccDecode`, `decodeLength` helpers to reuse)

## Not In Scope

- Do NOT port ISCC-IDv0 (`gen_iscc_id_v0`, `soft_hash_iscc_id_v0`, `iscc_id_incr`,
    `alg_simhash_from_iscc_id`) — the wallet/blockchain legacy path is not requested by #43.
- Do NOT add ISCC-IDv1 to the other 11 language bindings (Rust core, Python, WASM, etc.) — Go-only.
    The Tier 1 count stays 32; these are Go-local experimental additions.
- Do NOT relax the public `EncodeComponent` to accept Version>0 — build the ID header directly via
    the internal `encodeHeader`/`encodeLength` helpers so `EncodeComponent`'s contract (reject
    Version>0) stays intact.
- Do NOT change `data.json` / vendored conformance vectors or the `ConformanceSelftest` vector count
    — ISCC-IDv1 is not in the ISO conformance set; assert the single known vector inline in the
    test.
- Do NOT bump `go.mod`/`go.sum` — only `encoding/binary` (stdlib) is needed.

## Implementation Notes

**Algorithm (from `iscc_id.py::gen_iscc_id_v1`), verified to round-trip the spec vector:**

- 64-bit body: `body = (timestamp << 12) | hubID`; `timestamp` is 52-bit µs-since-epoch (must be
    `< 2^52`), `hubID` is the low 12 bits (0–4095). Pack big-endian into 8 bytes
    (`binary.BigEndian.PutUint64`).
- Header nibbles: MainType=`MTId` (6), SubType=`realm` (0=test, 1=operational), Version=`VSV1` (1),
    length index=0 (canonical 64-bit body). Realm must be 0 or 1.
- `EncodeIsccID(realm uint8, hubID uint16, timestamp uint64) (string, error)`: validate the three
    ranges (return an `iscc:`-prefixed error on overflow), then
    `encodedLen, _ := encodeLength(MTId, 64)` (→ 0),
    `header, _ := encodeHeader(MTId, SubType(realm), VSV1, encodedLen)`, concatenate
    `header +   digest`, base32-encode, and return **with** the `"ISCC:"` prefix (the spec vector
    includes it).
- `DecodeIsccID(code string) (*IsccIDv1Result, error)`: delegate to the existing `IsccDecode` (it
    already strips the `ISCC:` prefix + dashes and, once `decodeHeader` accepts V1, returns
    Maintype=6/Version=1/8-byte Digest). Guard `Maintype==MTId`, `Version==VSV1`, `len(Digest)==8`,
    then `body := binary.BigEndian.Uint64(Digest)`; `Timestamp = body >> 12`,
    `HubID = uint16(body & 0xFFF)`, `Realm = result.Subtype`.
- `IsccIDv1Result` struct: `Realm uint8`, `HubID uint16`, `Timestamp uint64`. Give both functions
    and the struct a doc-comment marker that they are **experimental** (ISCC-IDv1 is not part of ISO
    24138 and may change in a minor release).

**`decodeHeader` change (codec.go ~line 268):** replace the unconditional
`if versionVal > 0 { return ...invalid Version }` with a guard that permits exactly
`MainType(mtypeVal)==MTId && versionVal==1` and rejects every other `versionVal > 0`. This is the
only behavioral change to existing code; the header roundtrip test (codec_test.go:161-175) only uses
Version 0 and is unaffected. `decodeLength(MTId, 0, realm)` already returns 64 → `IsccDecode` reads
the 8-byte body correctly.

**Verified vector (do not re-derive):** `EncodeIsccID(0, 1, 1751831876325218)` →
`"ISCC:MAIGHFECJMOPMIAB"`; component hex is `60106394824b1cf62001` (2-byte header `6010` + 8-byte
body).

## Verification

- From `packages/go/`: `go test ./...` passes (all existing tests + the new `iscc_id_test.go`), and
    `CGO_ENABLED=0 go test ./...` also passes (pure-Go invariant holds).
- From `packages/go/`: `go vet ./...` is clean.
- `DecodeIsccID("ISCC:MAIGHFECJMOPMIAB")` and `DecodeIsccID("MAIGHFECJMOPMIAB")` (no prefix) both
    return `Realm=0, HubID=1, Timestamp=1751831876325218` — asserted by a test.
- `EncodeIsccID(0, 1, 1751831876325218)` returns `"ISCC:MAIGHFECJMOPMIAB"` — asserted by a test.
- Round-trip holds for boundary values (hubID 0 and 4095, realm 0 and 1, timestamp `2^52 - 1`):
    `DecodeIsccID(EncodeIsccID(...))` reproduces the inputs — asserted by a test.
- `IsccDecode("ISCC:MAIGHFECJMOPMIAB")` returns `Maintype==6`, `Version==1`, `len(Digest)==8` (no
    longer errors with "invalid Version: 1") — asserted by a test.
- Version>0 is still rejected for a non-ID MainType — a test constructs a Data-Code-shaped header
    with Version=1 (via `encodeHeader(MTData, STNone, VSV1, ...)`) and asserts `decodeHeader`
    returns an error.
- `EncodeIsccID` returns an `iscc:`-prefixed error for `timestamp >= 2^52`, `hubID >= 4096`, and
    `realm` ∉ {0,1} — asserted by a test.
- The two experimental functions carry an "experimental" doc-comment marker and appear in
    `packages/go/README.md`'s codec-functions table.

## Done When

The new `EncodeIsccID`/`DecodeIsccID` round-trip the known vector and boundary values, `IsccDecode`
accepts ISCC-IDv1 while every other MainType still rejects Version>0, and `go test`/`go vet` (incl.
`CGO_ENABLED=0`) are green in `packages/go/`.
