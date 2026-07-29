<!-- assessed-at: 13c08ca -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — CI RED on develop: the iscc_clean routing (iters 191/192) regressed two iai perf benches >10%; the ENFORCING Perf gate blocks green

The `iscc_clean` refactor landed (empty-cleaned guard fixed the `iscc_decompose` regression,
reviewer PASS iter 192) and is pushed. But routing every codec-input site through `iscc_clean` added
instruction cost to the composite `gen_iscc_code_v0` / `gen_mixed_code_v0` paths — the CI-only
`iai-callgrind` regression check (not run by `mise run check`, so the reviewer missed it) now FAILS.
#43 IDv1 stays fully closed on core + all 11 surfaces.

## Rust Core Crate

**Status**: partially met — feature-complete; `iscc_clean` routing landed but introduced a perf
regression that reds CI

- 33/33 Tier 1 symbols; `gen_iscc_id_v1` oracle-matched; IDv1 decode via generic `iscc_decode`. All
    10 `gen_*_v0` conformant, no `unsafe` outside FFI, Unicode criteria met.
- `iscc_clean` (`codec.rs:532`) now routes the four codec-input sites and rejects empty-cleaned
    input (`"Empty ISCC string"`, line 558) — closes the earlier `iscc_decompose` `Ok([])` gap.
- **New regression, live on develop:** the routing costs `bench_iscc_code.four_units` +36.82%
    (11,968→16,375 Ir) and `bench_mixed_code.two_codes` +18.30% (10,460→12,374 Ir), both past the
    10% iai gate. The composite gen functions now call `iscc_clean` per component. Fix = reduce the
    per-component cleaning overhead or re-baseline with justification (baseline change is human/gate
    territory, not a silent bump).
- Semver check-run reds (`enum_marked_non_exhaustive` on `enum Version`) but `continue-on-error`.
- `crate >= 1.0.0` unmet (0.5.0, human-gated).

## Python Bindings

**Status**: met — 33/33 symbols; IDv1 mint + decode both work

- `crates/iscc-py` exports `gen_iscc_id_v1`; `VS` IntEnum has `V1 = 1` so `iscc_decode` round-trips.
    Retains `IsccResult`, streaming hashers, GIL `.detach(` sites, abi3-py310 wheels.

## Node.js Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-napi` exports `gen_iscc_id_v1(f64, hub_id, realm)`, golden oracle-confirmed, ordered
    `checked()` guard (2^52/4096/2, ts→hub→realm) before narrowing. No version enum → decodes V1.

## WASM Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-wasm` exports `gen_iscc_id_v1` with the same ordered validation guard; `wasm-pack`
    test covers golden `ISCC:MAIGHFECJMOPMIAB`, decode round-trip, invalid throws. `wasm32_simd`
    blake3 feature intact.

## C FFI

**Status**: met — 33/33 symbols

- 50 `#[unsafe(no_mangle)]` externs; `iscc_gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` + committed
    `iscc.h`, regen idempotent (freshness gate green). Golden + realm-error C tests. cbindgen
    headers, `docs/howto/c-cpp.md`, `examples/iscc_sum.c` + CMake present.

## Java Bindings (JNI)

**Status**: met — 33/33 symbols

- `Java_..._genIsccIdV1` narrows to `(u64,u16,u8)` only after validating the 3 thresholds in
    ts→hub→realm order. 4 mvn tests incl. `genIsccIdV1ValidationOrder`. Java CI green.

## Ruby Bindings (Magnus)

**Status**: met — 33/33 symbols

- `gen_iscc_id_v1(i64, i64, i64)` validates via `checked()` before narrowing; golden/validation/
    round-trip tests. Known pathological gap (open `normal` `[review]`): args `> i64::MAX` raise
    `RangeError` from Magnus marshalling before the ordered `checked()` runs.

## Swift & Kotlin Bindings (uniffi)

**Status**: met — 33/33 symbols

- Shared `crates/iscc-uniffi` exports `gen_iscc_id_v1` + `IsccIdResult`; core takes exact-width
    `(u64,u16,u8)`. 22 unit tests incl. golden + round-trip. Regenerated `genIsccIdV1` in both
    `packages/swift` and `packages/kotlin`.

## C# / .NET Bindings

**Status**: met — 33/33 symbols

- `GenIsccIdV1(ulong, ushort, byte)` + `IsccIdResult` record over the generated P/Invoke decl.
    Golden matches `iscc_core` byte-for-byte; 107 dotnet tests. Exact-width → exempt from Ruby gap.

## C++ Bindings (cpp)

**Status**: met — 33/33 symbols

- `packages/cpp/include/iscc/iscc.hpp` mints via `gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` +
    `IsccIdResult`. Golden matches `iscc_core`; 72 assertions under cmake/ASAN; CI green.

## Documentation

**Status**: partially met — 32→33 sweep + IDv1 entries done; one doc-defect issue open

- Docs site (23 pages, 12 READMEs, 12 CLAUDE.md) builds/deploys. Tier-1 count reads 33 across all
    shipped artifacts. `gen_iscc_id_v1` per-symbol entries in `docs/{rust,java,ruby,c-ffi}-api.md`;
    all 11 `docs/howto/*.md` carry an IDv1 mint+decode example.
- Open `normal` `[review]`: `docs/c-ffi-api.md` documents FFI structs under unprefixed names
    (`IsccDecodeResult`) but cbindgen emits `iscc_IsccDecodeResult`. Pre-existing, page-wide.

## Benchmarks

**Status**: met (existence) — the perf gate just caught a real regression (see CI)

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures. The iai gate FIRED on the
    `iscc_clean` routing (2 benches >10%) — working as designed; the committed baseline is now
    stricter than HEAD's actual cost.
- Open `normal` `[review]`: iai text benchmarks are ASCII-only, so the gate is blind to the Unicode
    freeze path.

## CI/CD and Publishing

**Status**: NOT met — CI RED on develop; the Perf gate blocks

- **CI check-suite FAILING on `origin/develop` = `e80cda5`** (which covers HEAD's code — the only
    unpushed commit `13c08ca` touches `iterations.jsonl` alone). `Perf (iai-callgrind)` = `failure`,
    an ENFORCING job (NOT `continue-on-error`): `bench_iscc_code.four_units` +36.82% and
    `bench_mixed_code.two_codes` +18.30%. `Semver` also reds but is `continue-on-error` (expected).
    All other 21 check names pass. Run: actions/runs/30445408022.
- Dependency freshness met (criterion 0.8, uniffi 0.32); zero `# held:`/`authorized` pins in root
    `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion **unmet**: 0 critical, 5 open `normal`.

## Open Issues

**12 entries — 0 `critical`, 5 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.** Composition
unchanged this cycle.

- **NORMAL (all `[review]`):** c-ffi-api type names vs generated `iscc.h`; Ruby `gen_iscc_id_v1`
    validation order for `> i64::MAX`; Go codec input cleaning diverges from `iscc_clean` (Rust half
    now landed — Go half unblocked, must replicate the empty-cleaned guard); iai ASCII-only text
    benchmarks; go1.27 tripwire (upstream-blocked, ~Aug 2026).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**Fix the red Perf (iai-callgrind) gate first — CI is failing on develop.** The `iscc_clean` routing
raised `gen_iscc_code_v0` and `gen_mixed_code_v0` instruction cost past the 10% threshold on two
benches; the fix is either trimming the per-component cleaning overhead in those composite paths or
a justified baseline update (a baseline bump masks a real 20-37% cost increase, so prefer the code
fix). Nothing else can go green until this lands. After CI is green, the Go `iscc_clean` port and
the remaining CID-doable `normal` `[review]` issues (c-ffi-api type names, Ruby wide-input
validation, iai ASCII-only benches) gate v0.6.0 readiness; go1.27 stays upstream-blocked; the
release cut is human-gated.
