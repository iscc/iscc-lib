<!-- assessed-at: 7845114 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — IDv1 (#43) fully closed on core + all 11 surfaces; CI GREEN; remaining CID-doable `normal` issues gate release readiness

Iteration 190 landed the mechanical repo-wide Tier-1 32→33 doc/count sweep and added
`gen_iscc_id_v1` per-symbol entries to the 4 hand-maintained API pages plus IDv1 mint+decode
examples to all 11 howto pages (reviewed PASS_WITH_NOTES). Both halves of the IDv1 functional work
(mint + decode) and the doc sweep are now done on all 11 surfaces — #43 is fully closed. The
remaining v0.6.0 release blockers are CID-doable `normal` `[review]` issues, not functional gaps.

## Rust Core Crate

**Status**: met (feature-complete for v0.6.0)

- 33/33 Tier 1 symbols: `gen_iscc_id_v1(timestamp, hub_id, realm)` additive/clock-free,
    oracle-matched; IDv1 decode via generic `iscc_decode`/`iscc_decompose` (accepts `Id` Version 1).
    All 10 `gen_*_v0` conformant, no `unsafe` outside FFI, Unicode criteria met.
- Semver check-run reds (`enum_marked_non_exhaustive` on `enum Version`) but job is
    `continue-on-error: true` — informational until the human-gated v1.0.0 cut.
- Only unmet criterion is `crate >= 1.0.0` (0.5.0, human-gated).

## Python Bindings

**Status**: met — 33/33 symbols; IDv1 mint + decode both work

- `crates/iscc-py` exports `gen_iscc_id_v1` (iter 178); `VS` IntEnum gained `V1 = 1` (iter 189) so
    `iscc_decode(gen_iscc_id_v1(...))` round-trips. Retains `IsccResult`, streaming hashers, GIL
    `.detach(` sites, abi3-py310 wheels.

## Node.js Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-napi/src/lib.rs:328` exports `gen_iscc_id_v1(timestamp: f64, hub_id, realm)`, golden
    oracle-confirmed, `index.d.ts` + round-trip test. `f64` + ordered semantic `checked()` guard
    (2^52/4096/2, ts→hub→realm) before narrowing. No version enum → already decodes V1.

## WASM Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-wasm/src/lib.rs:371` exports `gen_iscc_id_v1` with the same ordered validation guard
    as napi; `wasm-pack test` covers golden `ISCC:MAIGHFECJMOPMIAB`, decode round-trip, invalid
    throws. Returns a bare string (no `IsccIdResult`); `blake3 wasm32_simd` feature intact.

## C FFI

**Status**: met — 33/33 symbols

- 50 `#[unsafe(no_mangle)]` externs; `iscc_gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` at
    `crates/iscc-ffi/src/lib.rs:613` + committed `iscc.h:386`, regen idempotent (freshness gate
    green). Golden + realm-error C tests. Core re-validates (exact-width).
- cbindgen headers, `docs/howto/c-cpp.md`, `examples/iscc_sum.c` + CMake all present.

## Java Bindings (JNI)

**Status**: met — 33/33 symbols

- `Java_..._genIsccIdV1` at `crates/iscc-jni/src/lib.rs:500` narrows to `(u64,u16,u8)` only after
    validating the 3 semantic thresholds (2^52/4096/2) in normative ts→hub→realm order. 4 mvn tests
    incl. `genIsccIdV1ValidationOrder`. Java CI job green.

## Ruby Bindings (Magnus)

**Status**: met — 33/33 symbols

- `gen_iscc_id_v1(i64, i64, i64)` at `crates/iscc-rb/src/lib.rs:219` validates via `checked()`
    (2^52/4096/2, ts→hub→realm) before narrowing; module fn registered L526. Golden/validation/
    round-trip tests.
- Known pathological gap (open `normal` `[review]`): args `> i64::MAX` raise `RangeError` from
    Magnus marshalling before the ordered `checked()` runs — no realistic input reaches it.

## Swift & Kotlin Bindings (uniffi)

**Status**: met — 33/33 symbols

- Shared `crates/iscc-uniffi/src/lib.rs:286` exports `gen_iscc_id_v1` + `IsccIdResult`; core takes
    exact-width unsigned `(u64,u16,u8)` (core validates, no binding guard). 22 unit tests incl.
    golden + round-trip. Regenerated `genIsccIdV1` in both `packages/swift` and `packages/kotlin`.

## C# / .NET Bindings

**Status**: met — 33/33 symbols

- `GenIsccIdV1(ulong, ushort, byte)` at `packages/dotnet/Iscc.Lib/IsccLib.cs:287` + `IsccIdResult`
    record over the generated `iscc_gen_iscc_id_v1` P/Invoke decl. Golden matches `iscc_core`
    byte-for-byte; 107 dotnet tests. Exact-width args → exempt from Ruby wide-input gap.

## C++ Bindings (cpp)

**Status**: met — 33/33 symbols

- `packages/cpp/include/iscc/iscc.hpp:564` mints via `gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` +
    `IsccIdResult` over the shipped FFI symbol. Golden matches `iscc_core` byte-for-byte; 72 C++
    assertions pass under cmake/ASAN; CI job green. `DecodeResult.version` is raw `uint8_t` → cpp
    already decodes V1.

## Documentation

**Status**: partially met — 32→33 sweep + IDv1 entries done; one doc-defect issue open

- Docs site (23 pages, 12 READMEs, 12 CLAUDE.md) builds/deploys. Tier-1 count reads 33 across all
    shipped artifacts (stale-32 grep clean). `gen_iscc_id_v1` per-symbol entries present in
    `docs/{rust,java,ruby,c-ffi}-api.md`; all 11 `docs/howto/*.md` carry an IDv1 mint+decode
    example.
- Open `normal` `[review]` doc defect: `docs/c-ffi-api.md` documents FFI structs under unprefixed
    names (`IsccDecodeResult`) but cbindgen emits `iscc_IsccDecodeResult`, so no snippet — including
    the new IDv1 example — compiles verbatim. Pre-existing, page-wide, not a regression.

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` `[review]`: iai text benchmarks are ASCII-only, so the >10% perf gate is blind to
    the Unicode freeze path — a coverage gap, not a missing benchmark.

## CI/CD and Publishing

**Status**: met (CI green on develop); release-readiness still open

- **CI GREEN on `origin/develop` = `b2f56b6`** (IDv1 doc-sweep review commit): all 23 check names
    pass except `Semver (cargo-semver-checks)` which is `continue-on-error: true` — expected
    non-blocking red, not a CI failure.
- **HEAD `7845114` is covered:** unpushed commits are only `cid(log)`/`cid(audit)` touching
    iterations.jsonl + metrics.jsonl; `git diff origin/develop..HEAD -- . ':!.claude'` is empty.
- Dependency freshness met (criterion 0.8 @171, uniffi 0.32 @172); zero `# held:`/`authorized` pins
    in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open (not human/upstream-blocked)" criterion **unmet**: 0
    critical, 5 open `normal` — four are CID-doable `[review]`, one (go1.27) upstream-blocked.

## Open Issues

**12 entries — 0 `critical`, 5 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.** Composition shifted
this cycle: the resolved #43 IDv1 sweep issue was deleted; a new `docs/c-ffi-api.md` type-name gap
was filed (`normal` `[review]`).

- **NORMAL (all `[review]`):** c-ffi-api type names vs generated `iscc.h`; Ruby `gen_iscc_id_v1`
    validation order for `> i64::MAX`; codec input cleaning diverges from `iscc_clean` (most
    substantive follow-up, Rust+Go); iai ASCII-only text benchmarks; go1.27 tripwire (upstream-
    blocked, ~Aug 2026).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**CI is green — no CI fix needed. IDv1 (#43) functional + doc work is fully closed on all 11
surfaces.** The remaining v0.6.0 release blockers are the four CID-doable `normal` `[review]`
issues; clearing them (plus the human-gated release cut itself) is what makes v0.6.0 releasable. The
codec `iscc_clean` divergence is the most substantive; the c-ffi-api type-name gap and Ruby
wide-input validation order are smaller, well-scoped fixes. go1.27 stays upstream-blocked.
