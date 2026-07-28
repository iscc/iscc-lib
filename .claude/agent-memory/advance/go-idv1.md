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
    `iscc_decode("ISCC:MAIGHFECJMOPMIAB")` → `(6,0,1,0,<8 bytes>)`. Part 2 (`gen_iscc_id_v1` minting
    \+ `IsccIdResult`, clock-free `u64` timestamp arg) still pending → spec §"Part 2".
- KNOWN pre-existing CI break (iter 174, out of scope):
    `cargo test -p iscc-lib --no-default-features` fails to COMPILE — `lib.rs:2072`
    `test_iscc_decode_rejects_uncomposable_sequence` calls `gen_meta_code_v0`/`gen_text_code_v0`
    without a feature gate. Reds ci.yml:42 `rust` job.
- Go CI job runs only `go test`+`go vet` — no gofmt gate. The go1.26 gofmt alignment drift in
    codec_test.go + conformance.go was fixed (gofmt -w) in iter 147; tree is gofmt-clean now.
- `TextCollapse` (utils.go) uses per-call `cases.Lower(language.Und)` for Final_Sigma conformance
    (iter 147). NEVER hoist the Caser to a package-level var — Casers are stateful/not
    goroutine-safe, and hoisting measured no faster (~140ns construction).
