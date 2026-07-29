## 2026-07-29 — Tier-1 32→33 doc/count sweep + `gen_iscc_id_v1` API-doc entries

**Done:** Bumped every shipped-artifact Tier-1 symbol count from NN→33 (adding `gen_iscc_id_v1` to
each breakdown), added a `gen_iscc_id_v1` per-symbol entry + field-extraction recipe to the four
hand-maintained API pages, and added an "ISCC-IDv1 (experimental)" mint+decode example to the 9
howto pages that lacked one. Docs-only; no source/signature/test changes. Closes the last CID-doable
#43 / v0.6.0 release blocker.

**Files changed (18):**

- Counts → 33: `crates/iscc-lib/CLAUDE.md`, `crates/iscc-wasm/CLAUDE.md`, `docs/ruby-api.md`,
    `docs/java-api.md`, `notes/00-overview.md` (item-8, 22→33),
    `notes/04-api-compatibility-safety.md` (32→33 + breakdown + code-comment),
    `.claude/agents/advance.md`. Canonical breakdown taken from `specs/rust-core.md:693` (10 gen + 1
    ISCC-IDv1 + 4 text + 4 algo + 1 soft + 2 enc + 3 codec + 5 const + 2 streaming + 1 diag = 33).
- API entries: `docs/rust-api.md` (`gen_iscc_id_v1`), `docs/java-api.md` (`genIsccIdV1`),
    `docs/ruby-api.md` (`gen_iscc_id_v1` + `IdCodeResult` result-class row), `docs/c-ffi-api.md`
    (`iscc_gen_iscc_id_v1`).
- Howto mint examples: `rust.md`, `python.md`, `nodejs.md`, `wasm.md`, `java.md`, `kotlin.md`,
    `swift.md`, `dotnet.md`, `c-cpp.md` (go/ruby already had them).

**Verification:**

- [x] Stale-count grep (next.md cmd) → empty (exit 1).
- [x] `grep -l gen_iscc_id_v1 rust-api ruby-api` + `genIsccIdV1 java-api` +
    `iscc_gen_iscc_id_v1   c-ffi-api` → all four listed.
- [x] `grep -riLE "gen_?iscc_?id_?v1" docs/howto/*.md` → no filenames (all 11 covered).
- [x] `uv run zensical build` → "No issues found", exit 0.
- [x] `mise run check` → exit 0, all 18 prek hooks Passed (incl. mdformat, Docs page list parity, CI
    job table parity). Ran `mise run format` first; mdformat reflowed the new tables/prose to
    `--wrap 100` and is now idempotent.

**Per-surface signatures used (verified against source, not guessed):** rust
`gen_iscc_id_v1(u64,u16,u8)->IsccIdResult{iscc}`; python `gen_iscc_id_v1(ts,hub_id=0,realm_id=0)`,
decode `vs==VS.V1`; napi/wasm bare-string return, `d.subtype`/`d.version==1`; java
`genIsccIdV1(long,int,int)->String`; kotlin `genIsccIdV1(ULong,UShort,UByte)->IsccIdResult`; swift
`genIsccIdV1(UInt64,UInt16,UInt8) throws`; dotnet
`GenIsccIdV1(ulong,ushort,byte)->IsccIdResult {Iscc}`; c-ffi `iscc_gen_iscc_id_v1(u64,u16,u8)`; cpp
`gen_iscc_id_v1(uint64_t,uint16_t,uint8_t)`. Golden
`gen_iscc_id_v1(1751831876325218,1,0) == "ISCC:MAIGHFECJMOPMIAB"` used consistently. Decode recipe:
`ts=n>>12`, `hub=n&0xFFF`, `realm=subtype` on the 8-byte BE body; no dedicated decoder on any
surface.

**Next:** #43 is fully closed after this. Remaining open normals are `[review]`/upstream-blocked
(Ruby wide-input validation order, codec `iscc_clean` divergence, iai ASCII benchmarks) — none
release-gating. The codec `iscc_clean` two-surface (Rust + Go) drop-in gap is the most substantive
follow-up.

**Notes:**

- wasm CLAUDE.md needed a small restructure (not a pure count bump): its "30 functions + 2 result
    structs + 3 streaming" partition became "33 Tier 1 symbols + 3 result structs (added
    `IsccIdResult`) + wasm-only `SumHasher`". No behaviour claim changed.
- The two source files the #43 issue flagged as stale (`crates/iscc-uniffi/src/lib.rs:3`,
    `crates/iscc-rb/src/lib.rs:7`) already read 33 — left untouched, confirmed by the empty grep.
- `docs/java-api.md` "All 10 `gen*V0` methods return an ISCC string" left as-is — accurate:
    `genIsccIdV1` is not a `*V0` method (though it also returns a String, so the sentence holds).
- CI reminder: `zensical build` wipes `site/`, so `scripts/gen_llms_full.py` must run after it in
    docs CI (not run here — no site artifact committed).
