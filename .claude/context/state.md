<!-- assessed-at: 3f0387e -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — IDv1 fan-out; JNI minting attempt NEEDS_WORK (unpushed). 6 of 11 surfaces mint IDv1; CI GREEN on develop

Iteration 183 attempted `gen_iscc_id_v1` on the JNI (Java) surface. The code is committed locally
(`8201277`) but reviewed **NEEDS_WORK** and **not pushed**: the wrapper's input validation violates
the normative cross-surface validation order (ts→hub→realm). 6 surfaces mint IDv1
(core+Python+Go+Node+WASM+C FFI); the JNI redo plus 4 more (Ruby, Kotlin/Swift, C#, C++) remain.

## Rust Core Crate

**Status**: met (feature-complete for v0.6.0)

- 33/33 Tier 1 symbols: `gen_iscc_id_v1(timestamp, hub_id, realm)` additive/clock-free,
    oracle-matched; IDv1 decode via generic `iscc_decode`/`iscc_decompose` (accepts `Id` Version 1).
    All 10 `gen_*_v0` conformant, no `unsafe` outside FFI, Unicode criteria met.
- Semver check-run reds (`enum_marked_non_exhaustive` on `enum Version`) but job is
    `continue-on-error: true` — informational until the human-gated v1.0.0 cut.
- Only unmet criterion is `crate >= 1.0.0` (0.5.0, human-gated).

## Python Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-py` exports `gen_iscc_id_v1` (iter 178) with differential grid + golden + validation
    tests. Retains `IsccResult`, streaming hashers, GIL `.detach(` sites, abi3-py310 wheels.
- IDv1 decode round-trip still gated by the Python `VS` IntEnum (tracked under #43); minting done.

## Node.js Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-napi/src/lib.rs:328` exports `gen_iscc_id_v1(timestamp: f64, hub_id, realm)`, golden
    oracle-confirmed, `index.d.ts` + round-trip test. JS-number coercion resolved (iter 181): `f64`
    \+ ordered semantic `checked()` guard (2^52/4096/2, ts→hub→realm) before narrowing.

## WASM Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-wasm/src/lib.rs:371` exports `gen_iscc_id_v1` with the same ordered validation guard
    as napi; `wasm-pack test` covers golden `ISCC:MAIGHFECJMOPMIAB`, decode round-trip, invalid
    throws. `SumHasher` wrapper, Unicode gate, `blake3 wasm32_simd` feature intact.

## C FFI

**Status**: met — 33/33 symbols (iter 182)

- 50 `#[unsafe(no_mangle)]` externs; `iscc_gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` at
    `crates/iscc-ffi/src/lib.rs:613` + committed `iscc.h:386`, regen idempotent (freshness gate
    green). Golden + realm-error C tests. No `checked()` guard — core re-validates ranges
    (exact-width types).
- cbindgen headers, `docs/howto/c-cpp.md`, `examples/iscc_sum.c` + CMake all present. Unicode-gated.

## Other Bindings (Java, Kotlin, C#, C++, Ruby, Swift)

**Status**: partially met — each usable at 32/33; no accepted IDv1 minting

- **Java (jni):** `genIsccIdV1` present in the tree (`crates/iscc-jni/src/lib.rs:499`,
    `IsccLib.java`, 3 new mvn tests) but the slice is **NEEDS_WORK and unpushed** — wrapper uses
    wide narrowing guards (hub 0-65535, realm 0-255) and skips the timestamp check, so multi-invalid
    inputs report the wrong field vs the normative ts→hub→realm order (rust-core.md ~L542). Redo
    owed; counts 32 usable.
- **Ruby (rb), Swift/Kotlin (uniffi), C++ (cpp):** each at 32, no `gen_iscc_id_v1`. uniffi 0.32
    (172) and MSRV fix (173) in place.
- **C# (.NET):** `NativeMethods.g.cs` carries the regenerated `iscc_gen_iscc_id_v1` P/Invoke decl
    (FFI build side-effect, iter 182), but the idiomatic C# consumer + golden test are still owed —
    32 usable.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 ==
    `VENDORED_COPIES`.

## Documentation

**Status**: partially met

- Docs site (23 pages, 12 READMEs, 12 CLAUDE.md) met; `docs/howto/go.md` + `packages/go/README.md`
    on `GenIsccIDV1`.
- Tabbed examples do not yet cover a non-Go IDv1 surface; Tier-1 count text still reads 32/30 in
    stale sites (wasm CLAUDE.md "30 Tier 1", core CLAUDE.md "32") — separate 32→33 sweep step under
    #43.

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` issue: iai text benchmarks are ASCII-only, so the >10% perf gate is blind to the
    Unicode freeze path — a coverage gap, not a missing benchmark.

## CI/CD and Publishing

**Status**: met (CI green on develop); release-readiness still open

- **CI GREEN on `origin/develop` = `5516c00`** (unchanged this cycle): check-suite `success`; all 23
    check names pass except `Semver (cargo-semver-checks)` which is `continue-on-error: true`
    (informational until v1.0.0) — expected non-blocking red, not a CI failure.
- **HEAD `3f0387e` is NOT covered by CI:** the JNI IDv1 commits are unpushed and
    `git diff origin/develop..HEAD -- . ':!.claude'` is non-empty (75 lines of JNI code,
    NEEDS_WORK). That code is a rejected work-in-progress, not merged state.
- Dependency freshness met (criterion 0.8 at 171, uniffi 0.32 at 172); zero `# held:`/`authorized`
    pin comments in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion **unmet**: 0 critical, 4 open `normal`.

## Open Issues

**11 entries — 0 `critical`, 4 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.** No new issue this
cycle (review chose to fix the JNI ordering bug in the next iteration rather than file it).

- **NORMAL:** ISCC-IDv1 unsupported outside Go (`[human]`, the live v0.6.0 work — 6 surfaces done,
    JNI in progress, 4 remain); codec input cleaning diverges from `iscc_clean` (`[review]`); iai
    text benchmarks ASCII-only (`[review]`); go1.27 tripwire (`[review]`, blocked on upstream).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**CI is green — no CI fix needed.** Redo the JNI `gen_iscc_id_v1` slice with napi-style ordered
semantic validation (validate 2^52/4096/2 thresholds in ts→hub→realm order *before* narrowing, plus
a multi-invalid-input ordering test), then continue the #43 fan-out to the remaining typed-int
surfaces (rb, uniffi→Swift/Kotlin, dotnet C# consumer, cpp), widen each surface's version enum so
`iscc_decode(gen_iscc_id_v1(...))` round-trips, and run the Tier-1 32→33 doc/count sweep — the last
CID-doable v0.6.0 release-readiness blockers.
