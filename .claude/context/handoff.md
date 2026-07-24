# Handoff

## 2026-07-24 — Go ISCC-IDv1 encode/decode (issue #43)

**Done:** Added experimental ISCC-IDv1 support to the pure-Go binding: new `EncodeIsccID` /
`DecodeIsccID` functions with `IsccIDv1Result` struct (realm, hub ID, 52-bit µs timestamp), plus a
`VSV1` constant and a relaxed `decodeHeader` that accepts Version=1 exactly when MainType==ID (all
other MainTypes still reject Version>0). `EncodeComponent`'s reject-Version>0 contract is untouched
— the ID header is built via the internal `encodeHeader`/`encodeLength` helpers as next.md
prescribed.

**Files changed:**

- `packages/go/iscc_id.go` (new): `EncodeIsccID`, `DecodeIsccID`, `IsccIDv1Result` — all three carry
    an "Experimental: ISCC-IDv1 is not part of ISO 24138…" doc-comment marker
- `packages/go/iscc_id_test.go` (new): 8 tests — known vector encode + decode (with and without
    `ISCC:` prefix), boundary round-trips (realm {0,1} × hubID {0,4095} × timestamp {0, vector,
    2^52−1}), `IsccDecode` V1 acceptance, Version=1 rejection for MTData, Version=2 rejection for
    MTId, input validation (`iscc:`-prefixed errors), non-ID rejection in `DecodeIsccID`
- `packages/go/codec.go`: added `VSV1 Version = 1` const; `decodeHeader` version check now
    `versionVal > 0 && (MainType(mtypeVal) != MTId || versionVal != uint32(VSV1))`
- `packages/go/README.md`: two rows for `EncodeIsccID`/`DecodeIsccID` in the Codec Operations table
    (mdformat realigned the table columns)

**Verification:**

- `go test ./...` and `CGO_ENABLED=0 go test ./...` both exit 0 from `packages/go/` (all existing +
    new tests); `go vet ./...` clean (no output, exit 0)
- Verbose run confirms all 8 new tests PASS, including
    `EncodeIsccID(0, 1, 1751831876325218) == "ISCC:MAIGHFECJMOPMIAB"`, both prefix variants decoding
    to realm 0 / hubID 1 / timestamp 1751831876325218, and `IsccDecode("ISCC:MAIGHFECJMOPMIAB")` →
    Maintype 6, Version 1, 8-byte digest
- `mise run format` applied (mdformat reflowed the README table), then `mise run check` exit 0 — all
    15 hooks Passed
- All five spec checkboxes in `specs/go-bindings.md` → "ISCC-IDv1 Support (Experimental)" are
    covered by asserted tests
- No `go.mod`/`go.sum` change (only stdlib `encoding/binary` added), no `data.json` /
    `ConformanceSelftest` change — per Not-In-Scope

**Next:** #43 is implemented — reviewer should verify and check the five spec boxes / close the
issue. Remaining v0.6.0 backlog per the prior review: #49 aarch64 Python wheels, dependency
review/refresh, npm OIDC migration, single-registry re-trigger fix.

**Notes:**

- Pre-existing gofmt drift (not from this change): `gofmt -l` under go1.26.1 flags
    `packages/go/codec_test.go` and `packages/go/conformance.go` (struct-tag/comment alignment rules
    changed in newer gofmt). Both files are untouched by this step and CI's `go` job runs only
    `go test` + `go vet` (no gofmt gate), so CI is unaffected. My three Go files are gofmt-clean.
    Left unfixed as out of scope — flagging for a possible hygiene follow-up.
- `packages/go/CLAUDE.md`'s "Codec Functions (exported)" list does not yet mention
    `EncodeIsccID`/`DecodeIsccID` — CLAUDE.md was not in next.md's Scope, so I left it. Reviewer may
    fix directly or file a follow-up.
- `TestDecodeHeaderRejectsVersion2ForID` goes slightly beyond the verification list to pin the
    exact-match (`versionVal == 1`) semantics of the relaxation, guarding against a future `>= 1`
    regression.
