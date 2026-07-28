<!-- assessed-at: 2916ce4ce14f0dcab693dd552bf1bf32b7150962 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — `gen_iscc_id_v1` minting landed in CORE (33 symbols); CI still RED on Semver

Iteration 177 added Tier 1 `gen_iscc_id_v1(timestamp, hub_id, realm)` + `IsccIdResult` to the pure
Rust core (reviewed PASS, oracle-verified), so the core now carries all 33 targeted Tier 1 symbols.
The 11 language surfaces still lack IDv1 minting and Go still uses the superseded names. CI remains
**RED on `origin/develop`** — the sole failing gate is `cargo-semver-checks` rejecting
`#[non_exhaustive]` on public `enum Version`. HEAD `2916ce4` is the `cid(log)` commit; its code is
identical to the red develop tip, so the red covers HEAD.

## Rust Core Crate

**Status**: met (feature-complete for v0.6.0), but its own change reds an enforcing CI gate

- **33/33 Tier 1 symbols present:** `gen_iscc_id_v1` minting now defined at `lib.rs:1064` with
    `IsccIdResult` (`types.rs:95`), additive/clock-free, oracle-matched vs `iscc-core` 1.3.0; IDv1
    decode via generic `iscc_decode`/`iscc_decompose` (accepts `Id` Version 1). All 10 `gen_*_v0`
    conformant, no `unsafe` outside FFI, Unicode criteria unchanged.
- **CI RED (sole gate):** `cargo-semver-checks` fails `enum_marked_non_exhaustive` on `enum Version`
    — marking a public enum `#[non_exhaustive]` needs a major bump vs 0.5.0 (exit 100, enforcing).
    `mise run check` does not run semver, so a green local check proves nothing here.
- Only other unmet criterion: `crate >= 1.0.0` (0.5.0, human-gated).

## Python Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-py` exports 32 symbols with `IsccResult`, streaming hashers, 12 `.detach(` GIL sites,
    abi3-py310 wheels. IDv1 decode inherits from core; **no `gen_iscc_id_v1`**. Untouched at 177.
- Carries the `iscc_decode` differential conformance test (`test_iscc_decode_conformance.py`).

## Node.js Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-napi` exports 32 symbols with streaming classes, Unicode-gated. IDv1 decode inherits
    from core; **no `gen_iscc_id_v1`.** Untouched.

## WASM Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-wasm` exports 32 symbols, `SumHasher` wrapper, Unicode-gated, `blake3 wasm32_simd`
    feature guarded in ci.yml. IDv1 decode inherits from core; **no `gen_iscc_id_v1`.** Untouched.

## C FFI

**Status**: partially met — 32/33 symbols

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`, Unicode-gated. IDv1 decode
    inherits from core; **no `gen_iscc_id_v1`.** Untouched.

## Other Bindings (Java, Kotlin, C#, C++, Go, Ruby, Swift)

**Status**: partially met — Go started IDv1, others at 32/33 with no IDv1 minting

- **Go:** `IsccDecode` accepts MainType `Id` Version 1. Target rename **not** done: `EncodeIsccID`,
    `DecodeIsccID`, `IsccIDv1Result` still in `packages/go/iscc_id.go`; no `GenIsccIDV1`.
- **Java, Kotlin, C#, C++, Ruby, Swift:** each at 32 symbols, no `gen_iscc_id_v1`. Untouched; uniffi
    0.32 (172) and MSRV fix (173) remain in place.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 ==
    `VENDORED_COPIES`.

## Documentation

**Status**: partially met

- Docs site (23 pages, 12 READMEs, 12 CLAUDE.md) met; a Go IDv1 how-to note is present.
- Tabbed multi-language examples do not yet cover an IDv1 surface; Tier-1 count text still reads 32
    by design (separate sweep step under #43).

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` issue: iai text benchmarks are ASCII-only, so the >10% perf gate is blind to the
    Unicode freeze path — a coverage gap, not a missing benchmark.

## CI/CD and Publishing

**Status**: RED on develop — top priority. Dependency freshness met; release-readiness blocked

- **CI FAILING on `origin/develop` = `98205f2`:** check-runs API reports 45 runs, **2 failures, all
    `Semver (cargo-semver-checks)`** — `#[non_exhaustive]` on public `enum Version`, rejected vs the
    0.5.0 baseline. The gen_iscc_id_v1 addition is additive (no new semver break) but did not clear
    the pre-existing one. CRAP regression that co-fired at 175 stayed green after the 176
    re-baseline.
- Dependency freshness met (criterion 0.8 at 171, uniffi 0.32 at 172); zero `# held:`/`authorized`
    pin comments in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion **unmet**: 0 critical, 4 open `normal`.

## Open Issues

**11 entries — 0 `critical`, 4 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.** The Semver CI
redness is not yet filed as an issue.

- **NORMAL:** ISCC-IDv1 unsupported outside Go + Go uses superseded names (`[human]`); codec input
    cleaning diverges from `iscc_clean` (`[review]`); iai text benchmarks ASCII-only (`[review]`);
    go1.27 tripwire (`[review]`, blocked on upstream).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**Get CI green on develop first** — the sole red gate is `cargo-semver-checks` rejecting
`#[non_exhaustive]` on public `enum Version` (it wants a major bump vs 0.5.0). This needs a
decision: drop the marker, allowlist/configure the lint as an accepted pre-1.0 exception, or move
the semver baseline. Only after CI is green, fan out experimental `gen_iscc_id_v1` minting from core
to the 11 language surfaces, do the Go rename (`EncodeIsccID` → `GenIsccIDV1`, delete
`DecodeIsccID`/`IsccIDv1Result`), and run the Tier-1 32→33 doc/count sweep — the last CID-doable
v0.6.0 release-readiness blockers.
