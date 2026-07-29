---
name: go-idv1
description: Go-only experimental ISCC-IDv1 support and exact body-length decode guards
metadata:
  type: project
---

# Go ISCC-IDv1 + decode guards

- Experimental ISCC-IDv1 in `packages/go/iscc_id.go`. Iter 179 (#43): renamed to
    `GenIsccIDV1(timestamp uint64, hubID uint16, realm uint8) (*IsccIdResult, error)` (single `ISCC`
    field, `json:"iscc"`); `DecodeIsccID`/`IsccIDv1Result` DELETED (ref has no IDv1 decoder). Decode
    via generic `IsccDecode` + unpack: `n:=binary.BigEndian.Uint64(d.Digest)`, `ts:=n>>12`,
    `hub:=uint16(n&0xFFF)`, `realm:=d.Subtype`. `codec.go` `decodeHeader` accepts Version=1 ONLY for
    MainType ID (`VSV1` const) — untouched.
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
    `gen_iscc_id_v1(1751831876325218,1,0)=="ISCC:MAIGHFECJMOPMIAB"`. Tier 1 now 33 symbols.
- Python binding DONE (iter 178, #43):
    `#[pyfunction] gen_iscc_id_v1(timestamp, hub_id=0,   realm_id=0)` in `crates/iscc-py/src/lib.rs`
    (returns `PyDict{"iscc"}`, no `py.detach`, Err→ `PyValueError`) + module reg; `_lowlevel.pyi`
    stub; `__init__.py` `IsccIdResult(IsccResult)` + wrapper + re-export + `__all__`. Differential
    test `tests/test_iscc_id_v1.py` vs installed `iscc_core` (grid ts×hub×realm{0,1} + golden +
    validation). Ref sig `(timestamp=None, hub_id=0,   realm_id=0)` — `realm_id` maps to core
    `realm`; NEVER pass `timestamp=None` (ref reads clock).
- napi binding DONE (iter 180, #43):
    `#[napi(js_name = "gen_iscc_id_v1")] pub fn   gen_iscc_id_v1(timestamp: f64, hub_id: u16, realm: u8) -> napi::Result<String>`
    in `crates/iscc-napi/src/lib.rs` (bare-string convention like `gen_meta_code_v0`, NOT object
    form; `timestamp as u64`; `.map(|r| r.iscc)`; Err→`napi::Error::from_reason`). Use `f64` not
    `u64` — valid ts `<2^52` exact in f64, avoids forcing JS `BigInt`. NO version-enum widening:
    napi `iscc_decode` returns `version: u8`, so V1 round-trips today
    (`maintype 6/version 1/subtype 0`). Test `__tests__/iscc_id_v1.test.mjs` (golden + 3
    validation-throws + decode round-trip); build `npm run build:debug` first. `index.d.ts`
    camelCases the param to `hubId` (cosmetic, positional call). napi CLAUDE.md/README carry NO
    symbol count → no doc edit.
- STILL PENDING (#43): minting on wasm, ffi, jni, rb, uniffi, dotnet, cpp surfaces; per-surface
    decode enum widening (Python `VS` etc. to accept V1) + round-trip test; 32→33 doc/count sweep.
    Go rename DONE iter 179, napi DONE iter 180.
- The iter-174 "no-default-features fails to COMPILE" note was a PHANTOM (review iter 177): that
    test is already `#[cfg(feature = "meta-code")]`-gated; all feature combos pass. No work owed.
- Go CI job runs only `go test`+`go vet` — no gofmt gate. The go1.26 gofmt alignment drift in
    codec_test.go + conformance.go was fixed (gofmt -w) in iter 147; tree is gofmt-clean now.
- `TextCollapse` (utils.go) uses per-call `cases.Lower(language.Und)` for Final_Sigma conformance
    (iter 147). NEVER hoist the Caser to a package-level var — Casers are stateful/not
    goroutine-safe, and hoisting measured no faster (~140ns construction).
