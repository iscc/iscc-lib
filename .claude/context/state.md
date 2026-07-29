<!-- assessed-at: 385a263 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — IDv1 fan-out; 8 of 11 surfaces mint IDv1; CI GREEN on develop

Iteration 185 added `gen_iscc_id_v1` to the Ruby (Magnus) surface (PASS_WITH_NOTES); code is pushed
to `origin/develop` and CI-green. 8 surfaces now mint IDv1 (core + Python + Go + Node + WASM + C FFI

- Java + Ruby). Three surfaces remain (uniffi→Swift/Kotlin, C#, C++), plus the Tier-1 32→33
    doc/count sweep. One pathological Ruby edge case (>i64::MAX args) was filed as a `[review]`
    issue.

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
    green). Golden + realm-error C tests. No `checked()` guard — core re-validates ranges
    (exact-width types).
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
    Golden `ISCC:MAIGHFECJMOPMIAB`, validation-order, round-trip tests (129 runs/368 assertions).
- Known pathological gap (filed `[review]`, normal): args `> i64::MAX` raise `RangeError` from
    Magnus marshalling *before* the ordered `checked()` runs — no realistic input reaches it.

## Other Bindings (Kotlin, C#, C++, Swift)

**Status**: partially met — each usable at 32/33; no accepted IDv1 minting

- **Swift/Kotlin (uniffi), C++ (cpp):** each at 32, no `gen_iscc_id_v1`. uniffi 0.32 (172) and MSRV
    fix (173) in place. uniffi is a fixed-width FFI surface (exempt from the Ruby marshalling gap,
    confirm at its step).
- **C# (.NET):** `NativeMethods.g.cs` carries the regenerated `iscc_gen_iscc_id_v1` P/Invoke decl
    (FFI build side-effect, iter 182), but the idiomatic C# consumer + golden test are still owed —
    32 usable.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 ==
    `VENDORED_COPIES`.

## Documentation

**Status**: partially met

- Docs site (23 pages, 12 READMEs, 12 CLAUDE.md) met; `docs/howto/ruby.md` now covers Ruby
    `gen_iscc_id_v1` (iter 185) alongside `docs/howto/go.md`.
- Tabbed examples still do not present IDv1 uniformly; Tier-1 count text still reads 32/30 in stale
    sites (wasm CLAUDE.md "30 Tier 1", core CLAUDE.md "32") — separate 32→33 sweep step under #43.

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` issue: iai text benchmarks are ASCII-only, so the >10% perf gate is blind to the
    Unicode freeze path — a coverage gap, not a missing benchmark.

## CI/CD and Publishing

**Status**: met (CI green on develop); release-readiness still open

- **CI GREEN on `origin/develop` = `ca8b96e`** (Ruby IDv1 review commit): check-suite `success`; all
    23 check names pass except `Semver (cargo-semver-checks)` which is `continue-on-error: true`
    (informational until v1.0.0) — expected non-blocking red, not a CI failure.
- **HEAD `385a263` is covered:** only unpushed commit is the `cid(log)` touching iterations.jsonl;
    `git diff origin/develop..HEAD -- . ':!.claude'` is empty — green CI covers all HEAD code.
- Dependency freshness met (criterion 0.8 at 171, uniffi 0.32 at 172); zero `# held:`/`authorized`
    pin comments in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion **unmet**: 0 critical, 5 open `normal`.

## Open Issues

**12 entries — 0 `critical`, 5 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.** One new `normal`
`[review]` this cycle (Ruby >i64::MAX validation-order gap).

- **NORMAL:** Ruby `gen_iscc_id_v1` validation order breaks for args > i64::MAX (`[review]`, new);
    ISCC-IDv1 unsupported outside Go (`[human]`, live v0.6.0 work — 8 surfaces done, 3 remain:
    uniffi→Swift/Kotlin, dotnet C#, cpp); codec input cleaning diverges from `iscc_clean`
    (`[review]`); iai text benchmarks ASCII-only (`[review]`); go1.27 tripwire (`[review]`, blocked
    on upstream).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**CI is green — no CI fix needed.** Continue the #43 IDv1 fan-out to the three remaining surfaces
(uniffi→Swift/Kotlin, dotnet C#, cpp) — the uniffi step is higher-risk (one core edit + regenerated
checked-in artifacts); wide-int surfaces must validate the three semantic thresholds (2^52/4096/2)
in ts→hub→realm order before narrowing (napi/wasm/jni/rb precedent), and any arbitrary-precision
surface must validate magnitude *before* the marshalling narrow (Ruby lesson). Then run the deferred
Tier-1 32→33 doc/count sweep — the last CID-doable v0.6.0 release-readiness blockers.
