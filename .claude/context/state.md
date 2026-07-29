<!-- assessed-at: 13474ec -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — IDv1 (#43) fully closed; codec `iscc_clean` Rust routing in-flight (NEEDS_WORK regression at HEAD, unpushed); CI green on pushed tip

Iteration 191 attempted to route the four Rust codec-input sites through a shared `iscc_clean`
helper. The port is faithful and fixes the three documented divergences, but the reviewer caught a
correctness regression in the Tier-1 `iscc_decompose` (empty-cleaned input silently returns `Ok([])`
instead of erroring) → NEEDS_WORK. That advance commit (`2537e18`) is **unpushed and sits at HEAD**;
green CI still only covers the last-clean tip `b2f56b6`. #43 IDv1 remains fully closed on core + all
11 surfaces; the remaining v0.6.0 blockers are CID-doable `normal` `[review]` issues.

## Rust Core Crate

**Status**: partially met — feature-complete but HEAD carries an unpushed Tier-1 regression

- 33/33 Tier 1 symbols; `gen_iscc_id_v1` oracle-matched; IDv1 decode via generic `iscc_decode`. All
    10 `gen_*_v0` conformant, no `unsafe` outside FFI, Unicode criteria met.
- **In-flight NEEDS_WORK (`2537e18`, unpushed):** the shared `iscc_clean` helper now routes the four
    codec-input sites, but `iscc_decompose` returns `Ok([])` for inputs that clean to `""` (`"   "`,
    `"-"`, `"iscc:"`, `"----"`) where the reference and HEAD~1 errored — a fresh divergence in a
    stability-committed Tier-1 API. Fix is small: guard the empty cleaned code + tests. Not covered
    by green CI (green tip = `b2f56b6`, pre-`iscc_clean`).
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

- `crates/iscc-wasm` exports `gen_iscc_id_v1` with the same ordered validation guard; the
    `wasm-pack` test covers golden `ISCC:MAIGHFECJMOPMIAB`, decode round-trip, invalid throws. The
    `wasm32_simd` blake3 feature is intact.

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
    `(u64,u16,u8)` (core validates). 22 unit tests incl. golden + round-trip. Regenerated
    `genIsccIdV1` in both `packages/swift` and `packages/kotlin`.

## C# / .NET Bindings

**Status**: met — 33/33 symbols

- `GenIsccIdV1(ulong, ushort, byte)` + `IsccIdResult` record over the generated P/Invoke decl.
    Golden matches `iscc_core` byte-for-byte; 107 dotnet tests. Exact-width → exempt from Ruby gap.

## C++ Bindings (cpp)

**Status**: met — 33/33 symbols

- `packages/cpp/include/iscc/iscc.hpp` mints via `gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` +
    `IsccIdResult`. Golden matches `iscc_core`; 72 assertions under cmake/ASAN; CI green.
    `DecodeResult.version` raw `uint8_t` → decodes V1.

## Documentation

**Status**: partially met — 32→33 sweep + IDv1 entries done; one doc-defect issue open

- Docs site (23 pages, 12 READMEs, 12 CLAUDE.md) builds/deploys. Tier-1 count reads 33 across all
    shipped artifacts. `gen_iscc_id_v1` per-symbol entries in `docs/{rust,java,ruby,c-ffi}-api.md`;
    all 11 `docs/howto/*.md` carry an IDv1 mint+decode example.
- Open `normal` `[review]`: `docs/c-ffi-api.md` documents FFI structs under unprefixed names
    (`IsccDecodeResult`) but cbindgen emits `iscc_IsccDecodeResult`, so no snippet compiles
    verbatim. Pre-existing, page-wide.

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` `[review]`: iai text benchmarks are ASCII-only, so the >10% perf gate is blind to
    the Unicode freeze path.

## CI/CD and Publishing

**Status**: met on the pushed tip; HEAD not covered by green

- **CI GREEN on `origin/develop` = `b2f56b6`**: all 23 check names pass except the
    cargo-semver-checks run, which is `continue-on-error: true` (expected non-blocking red).
- **HEAD `13474ec` is NOT covered by green:** unpushed commits include `2537e18` (advance) whose
    codec.rs/lib.rs/tests changes carry the NEEDS_WORK regression above. The origin..HEAD code diff
    (excluding `.claude`) = 4 files (codec.rs, lib.rs, codec_clean.rs, .crap-baseline.json). This
    code has never run through CI.
- Dependency freshness met (criterion 0.8, uniffi 0.32); zero `# held:`/`authorized` pins in root
    `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion **unmet**: 0 critical, 5 open `normal`.

## Open Issues

**12 entries — 0 `critical`, 5 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.** Composition
unchanged this cycle.

- **NORMAL (all `[review]`):** c-ffi-api type names vs generated `iscc.h`; Ruby `gen_iscc_id_v1`
    validation order for `> i64::MAX`; Go codec input cleaning diverges from `iscc_clean` (the Rust
    half is the in-flight NEEDS_WORK work above); iai ASCII-only text benchmarks; go1.27 tripwire
    (upstream-blocked, ~Aug 2026).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**No CI fix needed — CI is green on the pushed tip.** The immediate goal is closing the codec
`iscc_clean` empty-cleaned-input regression in `iscc_decompose` so the in-flight Rust routing at
HEAD can land clean and be pushed under green CI; the reviewer scoped the fix as a small empty-input
guard + tests. After that, the Go half of the same divergence and the remaining three CID-doable
`normal` `[review]` issues (c-ffi-api type names, Ruby wide-input validation order, iai ASCII-only
benchmarks) gate v0.6.0 release readiness. go1.27 stays upstream-blocked; the release cut is
human-gated.
