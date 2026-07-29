---
name: iscc-idv1-fanout
description: ISCC-IDv1 (#43) fan-out facts — core done, per-surface minting slices, JS-number validation, doc sweep
metadata:
  type: project
---

# ISCC-IDv1 (#43) fan-out

**Core DONE (33 symbols).** Part 1 (codec `Version` accepts V1) + Part 2
(`gen_iscc_id_v1(ts:u64,hub:u16,realm:u8)` + `IsccIdResult`) landed in core by 177. Golden:
`gen_iscc_id_v1(1751831876325218,1,0)=="ISCC:MAIGHFECJMOPMIAB"`. Validation order: ts≥2^52 →
hub≥2^12 → realm∉{0,1}. **No `decode_iscc_id_v1` on any surface** (Titusz — reference has none; the
generic `iscc_decode`/`iscc_decompose` covers V1). Spec = `specs/rust-core.md` "ISCC-IDv1
Operations". 174/175/176 history (decode_header truncation fix, CRAP re-baseline) → MEMORY-archive.

Signature per binding follows reference `gen_iscc_id_v1(timestamp,hub_id,realm)` with each lang's
casing; **timestamp REQUIRED (core is clock-free) — never default to None/clock**.

## Per-surface = SEPARATE steps (not identical fan-out)

Every binding tech wraps differently (PyO3 dict / napi / wasm / C ABI+iscc.h / JNI / Magnus /
UniFFI-gen), so each is its own slice:

- (a) `gen_iscc_id_v1` on each of 11 surfaces.
- (b) Go rename `EncodeIsccID`→`GenIsccIDV1`, DELETE `DecodeIsccID`/`IsccIDv1Result` — DONE 179.
- (c) Python differential test vs `iscc_core` (dev-dep, imported directly like
    `tests/test_iscc_decode_conformance.py`) — bundled with Python step (178).
- (d) Tier-1 32→33 count/doc sweep — its own slice; see #43 for exact sites incl. 3 non-Markdown
    (`crates/iscc-uniffi/src/lib.rs:3`, `crates/iscc-rb/src/lib.rs:7`,
    `.claude/agents/advance.md:146`).

**Decode round-trip does NOT always need enum-widening:** napi + wasm `iscc_decode` return `version`
as bare `u8` → V1 round-trips already, minting-only. Python's `VS` IntEnum lists only V0 → its slice
must widen it + round-trip test.

## Done slices

- **178 = Python:** reference-parity anchor + diff test.

- **179 = Go rename (b):** `GenIsccIDV1(timestamp,hubID,realm)→*IsccIdResult{ISCC}`; decode extracts
    via `binary.BigEndian.Uint64(d.Digest)` → `>>12`/`&0xFFF`/`d.Subtype`. Old `EncodeIsccID` etc.
    were develop-only (no API break).

- **180 = napi:** bare-`string` return (mirrors `gen_meta_code_v0`, NOT object form),
    `timestamp:   f64` (valid ts \<2^52 exact in f64, avoids napi u64→BigInt);
    `index.d.ts`/`index.js` are napi-build-generated (don't hand-edit); napi CLAUDE.md/README carry
    no count.

- **181 = wasm + settle JS-number validation:** handoff routed wasm here to fix the coercion
    approach once. Both napi AND wasm `iscc_decode` return `version:u8` → NO widening, minting-only.
    **JS-number coercion fix (issues.md `normal`) bundled** — napi u16/u8 params coerce garbage
    (ToUint32/truncate) BEFORE the wrapper runs, so take timestamp/hub_id/realm all as `f64` on both
    surfaces and validate finite + integral (`fract()==0`) + range (`0≤ts<2^52`, `0≤hub<4096`,
    `realm∈{0,1}`) before narrowing. Changing napi `(f64,u16,u8)`→`(f64,f64,f64)` is fine
    (experimental, develop-only). wasm test = Rust `#[wasm_bindgen_test]` via
    `wasm-pack test --node crates/iscc-wasm --features conformance`.

- **182 = ffi:** `iscc_gen_iscc_id_v1(u64,u16,u8)`, typed ints NO `checked()` (core re-checks),
    regen `iscc.h` via cbindgen + git-diff-empty freshness gate. **Surprise:** `iscc-ffi`'s
    csbindgen `build.rs` rewrites tracked `NativeMethods.g.cs` on EVERY build → pre-push clippy
    rejects the push; ANY future FFI-symbol step must regen+commit `NativeMethods.g.cs` in-step too.

- **183/184 = jni:** `genIsccIdV1(long ts,int hubId,int realm)→String`, Rust name
    `Java_..._genIsccIdV1` (`_1`!). Golden+realm+round-trip tests + Java native decl landed 183
    (`8201277`, unpushed). Decode round-trips WITHOUT enum-widening (JNI `IsccDecodeResult.version`
    is a plain `int`). jni README/CLAUDE carry no numeric count → no doc edit. Body mirrors
    `genTextCodeV0`; `throw_and_default` on `Err`. **183 NEEDS_WORK — wide-narrowing guard was
    WRONG:** guarding `hubId∈0..=65535`, `realm∈0..=255` and skipping the ts check violates the
    normative ts→hub→realm order for MULTI-invalid inputs (`(2^52,65536,0)` reports hub not ts;
    `(0,4096,256)` reports realm not hub). **184 fix = validate the 3 SEMANTIC thresholds
    (2^52/4096/2) in ts→hub→realm order BEFORE narrowing** (mirror napi's `checked`), + a
    multi-invalid ordering test. Wide-int surfaces (jni, and rb/dotnet IF they take
    wider-than-needed ints) all need this; Go/ffi exact-width → delegate to core.

- **185 = rb (Magnus):** wide-int surface. Take all 3 params as `i64` in the native fn, add a
    `checked(v:i64, max_exclusive:i64, name)->Result<i64,Error>` helper rejecting `v<0||v>=max` with
    `RuntimeError`, call in ts(2^52)→hub(4096)→realm(2) order BEFORE narrowing to `(u64,u16,u8)`.
    Native returns `RHash{"iscc"}` mirroring `gen_text_code_v0`; register `_gen_iscc_id_v1` arity 3;
    Ruby wrapper `self.gen_iscc_id_v1(ts,hub,realm)` (positional) + `IdCodeResult < Result`. **NO
    enum-widening** — `iscc_decode` returns `version` as a plain Integer → round-trips already.
    Tests live in the Ruby suite (golden/round-trip/order), NOT the Python differential file.
    Rebuild `.so` (`rake compile:dev`) before `rake test` — cargo test does NOT run Magnus. Docs:
    add IDv1 section to `docs/howto/ruby.md` with the `unpack1("Q>")`→`>>12`/`&0xFFF`/subtype
    recipe. rb's OWN header docstring `Symbols (32 of 32)` is bumped to 33 in-step (it enumerates
    names); the repo-wide sweep is still separate.

## Remaining after 185

uniffi→Swift/Kotlin (1 core edit + 3 regen artifacts), dotnet (C# consumer + regen
`NativeMethods.g.cs`), cpp, + the 32→33 doc/count sweep.
