---
name: go-idv1
description: Go-only experimental ISCC-IDv1 support and exact body-length decode guards
metadata:
  type: project
---

# Go ISCC-IDv1 + decode guards

- Experimental ISCC-IDv1 in `packages/go/iscc_id.go` (`EncodeIsccID`/`DecodeIsccID`, iter 119, #43);
    `codec.go` `decodeHeader` accepts Version=1 ONLY for MainType ID (`VSV1` const).
- Go `IsccDecode` (iter 120) and Rust Tier 1 `iscc_decode` (iter 121) both enforce EXACT body length
    — two-branch "too short"/"too long" guards.
- Rust ISCC-IDv1 Part 1 DONE (iter 174, #43): `codec::Version` is now `#[non_exhaustive]` with
    `V0=0`,`V1=1`. MainType-aware private `validate_version(mtype, version)` accepts V1 ONLY for
    `MainType::Id`; called in `encode_header` (top) + `decode_header` (after enum decode),
    `encode_component` transitive. `TryFrom<u8>` stays context-free. Realm = raw `SubType` nibble
    (realm0→None, realm1→Image cosmetic) — never add realm variants. Pinned:
    `iscc_decode("ISCC:MAIGHFECJMOPMIAB")` → `(6,0,1,0,<8 bytes>)`.
- Rust ISCC-IDv1 Part 2 DONE (iter 177, #43): Tier 1
    `gen_iscc_id_v1(timestamp: u64, hub_id: u16,   realm: u8) -> IsccResult<IsccIdResult>` in
    `lib.rs` (before tests mod) + `IsccIdResult { iscc }` `#[non_exhaustive]` in `types.rs`.
    Clock-free (caller supplies ts). Validation order (first fails): `timestamp>=1<<52` "Timestamp
    overflow", `hub_id>=1<<12` "HUB-ID overflow", `realm∉{0,1}` "Realm-ID must be 0 (test) or 1
    (operational)". Body = `(ts<<12)|hub_id`, be_bytes, then
    `encode_component(Id, SubType::try_from(realm)?, V1, 64, &digest)` + "ISCC:". Golden
    `gen_iscc_id_v1(1751831876325218,1,0)=="ISCC:MAIGHFECJMOPMIAB"`. Tier 1 now 33 symbols. STILL
    PENDING: 11-surface binding fan-out, Go `EncodeIsccID` rename/`DecodeIsccID` deletion, Python
    differential test vs iscc_core (needs rebuilt wheel), 32→33 doc/count sweep.
- KNOWN pre-existing CI break (iter 174, out of scope):
    `cargo test -p iscc-lib --no-default-features` fails to COMPILE — `lib.rs:2072`
    `test_iscc_decode_rejects_uncomposable_sequence` calls `gen_meta_code_v0`/`gen_text_code_v0`
    without a feature gate. Reds ci.yml:42 `rust` job.
- Go CI job runs only `go test`+`go vet` — no gofmt gate. The go1.26 gofmt alignment drift in
    codec_test.go + conformance.go was fixed (gofmt -w) in iter 147; tree is gofmt-clean now.
- `TextCollapse` (utils.go) uses per-call `cases.Lower(language.Und)` for Final_Sigma conformance
    (iter 147). NEVER hoist the Caser to a package-level var — Casers are stateful/not
    goroutine-safe, and hoisting measured no faster (~140ns construction).
