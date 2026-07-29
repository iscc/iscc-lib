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
- wasm binding DONE (iter 181, #43):
    `#[wasm_bindgen] gen_iscc_id_v1(timestamp, hub_id, realm) ->   Result<String, JsError>` in
    `crates/iscc-wasm/src/lib.rs` (after `iscc_decompose`), bare-string. wasm now 33 Tier 1 symbols.
    `iscc_decode` returns `version: u8` → V1 round-trips, no enum widening. Rust
    `#[wasm_bindgen_test]` in `tests/unit.rs`.
- JS-NUMBER VALIDATION SETTLED (iter 181): both JS-number surfaces (napi + wasm) take ALL THREE
    params as `f64` and validate via a per-file `fn checked(value: f64, max_exclusive: f64, name)`
    BEFORE narrowing — rejects `!is_finite()`, `fract()!=0.0`, `<0.0`, `>=max_exclusive` (`2^52`=
    `4_503_599_627_370_496.0`, hub `4096.0`, realm `2.0`). Fixes napi's silent `f64 as u64` truncate
    (the `[review]` normal issue). Reason for `f64` on hub/realm too: napi/wasm coerce JS numbers to
    `u16`/`u8` before the wrapper runs, so integer params make the low/non-integral end
    unvalidatable. napi sig CHANGED `(f64,u16,u8)`→`(f64,f64,f64)` (develop-only, no API-break).
    ffi/jni/rb/uniffi are typed-int surfaces (no JS coercion hazard) — do NOT copy this pattern
    there.
- ffi binding DONE (iter 182, #43):
    `#[unsafe(no_mangle)] pub unsafe extern "C" fn   iscc_gen_iscc_id_v1(timestamp: u64, hub_id: u16, realm: u8) -> *mut c_char`
    in `crates/iscc-ffi/src/lib.rs` (after `iscc_gen_iscc_code_v0`), body
    `clear_last_error()`+`result_to_c_string(iscc_lib::gen_iscc_id_v1(...).map(|r| r.iscc))`. TYPED
    ints, NO `checked()` guard (core re-checks ranges), NO NULL guard (no ptr args). `# Safety` line
    for consistency only. Regenerated committed `iscc.h` via
    `cbindgen --config   crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h`
    (never hand-edit; +21 lines, idempotent). C test `tests/test_iscc.c` tests 29 (golden) + 30
    (realm=2 → ASSERT_NULL + last_error non-NULL); `ULL` suffix on the ts literal. csbindgen picks
    up the symbol on build but `NativeMethods.g.cs` stays unused until the dotnet step (not
    committed here).
- dotnet binding DONE (iter 187, #43): C#
    `public static IsccIdResult GenIsccIdV1(ulong timestamp,   ushort hubId, byte realm)` in
    `packages/dotnet/Iscc.Lib/IsccLib.cs` (before `GenDataCodeV0`), mirrors `GenMetaCodeV0`:
    `unsafe { byte* r = NativeMethods.iscc_gen_iscc_id_v1(...); return new   IsccIdResult(ConsumeNativeString(r)); }`.
    `ConsumeNativeString` already throws `IsccException(GetLastError())` on NULL → no binding
    guard (exact-width unsigned args; core re-checks). `IsccIdResult(string Iscc)` record in
    `Results.cs`. NO enum widening — `DecodeResult.Version` is raw `byte`, V1 round-trips today. The
    P/Invoke decl `NativeMethods.g.cs:293` was ALREADY present (committed in the ffi step's
    csbindgen output), so NO regen — `git diff --quiet -- NativeMethods.g.cs` stays clean after
    `cargo build -p iscc-ffi`. Tests in `SmokeTests.cs` (golden + out-of-range-ts throws + decode
    round-trip); 107 pass. Verify: `cargo build -p iscc-ffi` then
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e   LD_LIBRARY_PATH=$(pwd)/target/debug`.
- STILL PENDING (#43): minting on cpp surface only; 32→33 doc/count sweep
    (`packages/dotnet/CLAUDE.md`/`README.md`, docs API pages, issues.md). Go DONE 179, napi 180,
    wasm 181, ffi 182, jni 184, rb 185, uniffi(Swift+Kotlin) 186, dotnet 187.
- The iter-174 "no-default-features fails to COMPILE" note was a PHANTOM (review iter 177): that
    test is already `#[cfg(feature = "meta-code")]`-gated; all feature combos pass. No work owed.
- Go CI job runs only `go test`+`go vet` — no gofmt gate. The go1.26 gofmt alignment drift in
    codec_test.go + conformance.go was fixed (gofmt -w) in iter 147; tree is gofmt-clean now.
- `TextCollapse` (utils.go) uses per-call `cases.Lower(language.Und)` for Final_Sigma conformance
    (iter 147). NEVER hoist the Caser to a package-level var — Casers are stateful/not
    goroutine-safe, and hoisting measured no faster (~140ns construction).
