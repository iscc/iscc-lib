<!-- assessed-at: 34f4e0e -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — IDv1 minting fan-out COMPLETE on core + all 11 surfaces; decode enum-widening + doc sweep remain; CI GREEN

Iteration 188 added idiomatic `iscc::gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` + `IsccIdResult`
to the C++ header (PASS), the 11th and final minting surface. Core plus all 11 language surfaces now
mint IDv1. Two #43 items remain before v0.6.0 release-readiness: per-surface decode enum-widening
(surfaces with their own version enum still reject V1) and the repo-wide Tier-1 32→33 doc/count
sweep. Code pushed, CI green.

## Rust Core Crate

**Status**: met (feature-complete for v0.6.0)

- 33/33 Tier 1 symbols: `gen_iscc_id_v1(timestamp, hub_id, realm)` additive/clock-free,
    oracle-matched; IDv1 decode via generic `iscc_decode`/`iscc_decompose` (accepts `Id` Version 1).
    All 10 `gen_*_v0` conformant, no `unsafe` outside FFI, Unicode criteria met.
- Semver check-run reds (`enum_marked_non_exhaustive` on `enum Version`) but job is
    `continue-on-error: true` — informational until the human-gated v1.0.0 cut.
- Only unmet criterion is `crate >= 1.0.0` (0.5.0, human-gated).

## Python Bindings

**Status**: met — 33/33 symbols (minting); decode-widening pending

- `crates/iscc-py` exports `gen_iscc_id_v1` (iter 178) with differential grid + golden + validation
    tests. Retains `IsccResult`, streaming hashers, GIL `.detach(` sites, abi3-py310 wheels.
- IDv1 decode round-trip still gated by the Python `VS` IntEnum (defines only `V0`) — tracked as the
    remaining functional half of #43; minting done.

## Node.js Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-napi/src/lib.rs:328` exports `gen_iscc_id_v1(timestamp: f64, hub_id, realm)`, golden
    oracle-confirmed, `index.d.ts` + round-trip test. `f64` + ordered semantic `checked()` guard
    (2^52/4096/2, ts→hub→realm) before narrowing.

## WASM Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-wasm/src/lib.rs:371` exports `gen_iscc_id_v1` with the same ordered validation guard
    as napi; `wasm-pack test` covers golden `ISCC:MAIGHFECJMOPMIAB`, decode round-trip, invalid
    throws. `SumHasher` wrapper, Unicode gate, `blake3 wasm32_simd` feature intact.

## C FFI

**Status**: met — 33/33 symbols (iter 182)

- 50 `#[unsafe(no_mangle)]` externs; `iscc_gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` at
    `crates/iscc-ffi/src/lib.rs:613` + committed `iscc.h:386`, regen idempotent (freshness gate
    green). Golden + realm-error C tests. No `checked()` guard — core re-validates (exact-width).
- cbindgen headers, `docs/howto/c-cpp.md`, `examples/iscc_sum.c` + CMake all present. Unicode-gated.

## Java Bindings (JNI)

**Status**: met — 33/33 symbols (iter 184)

- `Java_..._genIsccIdV1` at `crates/iscc-jni/src/lib.rs:500` narrows to `(u64,u16,u8)` only *after*
    validating the 3 semantic thresholds (2^52/4096/2) in normative ts→hub→realm order. 4 mvn tests
    incl. `genIsccIdV1ValidationOrder`. Java CI job green. jni = 33.

## Ruby Bindings (Magnus)

**Status**: met — 33/33 symbols (iter 185)

- `gen_iscc_id_v1(i64, i64, i64)` at `crates/iscc-rb/src/lib.rs:219` validates via `checked()`
    (2^52/4096/2, ts→hub→realm) before narrowing to `(u64,u16,u8)`; module fn registered L526.
    Golden, validation-order, round-trip tests.
- Known pathological gap (filed `[review]`, normal): args `> i64::MAX` raise `RangeError` from
    Magnus marshalling *before* the ordered `checked()` runs — no realistic input reaches it.

## Swift & Kotlin Bindings (uniffi)

**Status**: met — 33/33 symbols (iter 186)

- Shared `crates/iscc-uniffi/src/lib.rs:286` exports `gen_iscc_id_v1` + `IsccIdResult`; core takes
    exact-width unsigned `(u64,u16,u8)` straight through (core validates, no binding guard). 22 unit
    tests incl. golden `ISCC:MAIGHFECJMOPMIAB` + round-trip. Regenerated `genIsccIdV1` in both
    `packages/swift/.../iscc_uniffi.swift` and `packages/kotlin/.../iscc_uniffi.kt`. UniFFI checksum
    stable; regen idempotent.

## C# / .NET Bindings

**Status**: met — 33/33 symbols (iter 187)

- `GenIsccIdV1(ulong, ushort, byte)` at `packages/dotnet/Iscc.Lib/IsccLib.cs:287` + `IsccIdResult`
    record (`Results.cs:35`) over the generated `iscc_gen_iscc_id_v1` P/Invoke decl. Golden matches
    `iscc_core` byte-for-byte; 107 dotnet tests. Exact-width args → exempt from Ruby wide-input gap.

## C++ Bindings (cpp)

**Status**: met — 33/33 symbols (iter 188)

- `packages/cpp/include/iscc/iscc.hpp:564` mints via `gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` +
    `IsccIdResult` (`:211`) over the shipped `iscc_gen_iscc_id_v1` FFI symbol. Golden matches
    `iscc_core` byte-for-byte; 72 C++ assertions pass under cmake/ASAN; CI job green. Fixed-width
    FFI passthrough → exempt from wide-input gap; core re-validates. `DecodeResult.version` is raw
    `uint8_t` (no enum), so cpp already decodes V1.

## Documentation

**Status**: partially met

- Docs site (23 pages, 12 READMEs, 12 CLAUDE.md) met; `docs/howto/ruby.md` + `docs/howto/go.md`
    cover `gen_iscc_id_v1`.
- Tabbed examples still do not present IDv1 uniformly; Tier-1 count text still reads 32 across many
    shipped sites (per-crate/package CLAUDE.md + README, `docs/`, `notes/`,
    `iscc-uniffi/src/lib.rs`, `iscc-rb/src/lib.rs`) — the deferred repo-wide 32→33 sweep, plus
    `gen_iscc_id_v1` per-symbol API-doc entries in `docs/rust-api.md`, `docs/java-api.md`,
    `docs/ruby-api.md`, `docs/c-ffi-api.md` and the 11 `docs/howto/*.md` pages (#43).

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` issue: iai text benchmarks are ASCII-only, so the >10% perf gate is blind to the
    Unicode freeze path — a coverage gap, not a missing benchmark.

## CI/CD and Publishing

**Status**: met (CI green on develop); release-readiness still open

- **CI GREEN on `origin/develop` = `169793c`** (cpp IDv1 review commit): all 23 check names pass
    except `Semver (cargo-semver-checks)` which is `continue-on-error: true` (informational until
    v1.0.0) — expected non-blocking red, not a CI failure.
- **HEAD `34f4e0e` is covered:** only unpushed commit is the `cid(log)` touching iterations.jsonl;
    `git diff origin/develop..HEAD -- . ':!.claude'` is empty — green CI covers all HEAD code.
- Dependency freshness met (criterion 0.8 at 171, uniffi 0.32 at 172); zero `# held:`/`authorized`
    pin comments in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion **unmet**: 0 critical, 5 open `normal`.

## Open Issues

**12 entries — 0 `critical`, 5 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.** No new issue this
cycle (cpp step found none).

- **NORMAL:** Ruby `gen_iscc_id_v1` validation order breaks for args > i64::MAX (`[review]`);
    ISCC-IDv1 unsupported outside Go (`[human]`, live v0.6.0 work — minting now done on all 11
    surfaces; remaining half is decode enum-widening + doc sweep); codec input cleaning diverges
    from `iscc_clean` (`[review]`); iai text benchmarks ASCII-only (`[review]`); go1.27 tripwire
    (`[review]`, blocked on upstream).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**CI is green — no CI fix needed.** #43 minting fan-out is complete on all 11 surfaces. The two
remaining #43 items are the last CID-doable v0.6.0 release blockers: (1) **decode enum-widening** —
surfaces with their own version enum still reject IDv1 decode (Python `VS` IntEnum defines only
`V0`, so `iscc_decode(gen_iscc_id_v1(...))` raises `1 is not a valid VS`); each such surface needs
V1 added + a round-trip test. This is the more functional gap. (2) the repo-wide **Tier-1 32→33
doc/count sweep** (stale `32` counts + `gen_iscc_id_v1` per-symbol API-doc entries).
