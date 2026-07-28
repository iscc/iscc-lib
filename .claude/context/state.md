<!-- assessed-at: 52919020f7b565de1e3fb63f73b00ab74abfce0f -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — CI RED on develop: CRAP gate fixed at 176, but Semver (#[non_exhaustive] Version) still red

Iteration 176 re-baselined `.crap-baseline.json` (reviewed PASS, pushed) which cleared the CRAP
regression gate — one of the two gates the pushed IDv1-decode batch had tripped. The other,
`cargo-semver-checks`, is **still failing** on `origin/develop` (tip `a961c75`): marking public
`enum Version` `#[non_exhaustive]` requires a major bump vs the 0.5.0 baseline. `gen_iscc_id_v1`
minting still exists nowhere and Go still carries the names the target says to rename/delete. HEAD
`5291902` is the `cid(log)` commit — its code is identical to the red develop tip, so the red covers
HEAD.

## Rust Core Crate

**Status**: partially met — IDv1 decode in source, minting absent, Semver gate red on it

- **IDv1 decode works in core:** `codec::Version` is `#[non_exhaustive]` `V0`/`V1`;
    `validate_version` permits `V1` only for MainType `Id`, so `iscc_decode`/`iscc_decompose` accept
    `Id` Version 1 (oracle-matched). The `decode_header` varnibble-truncation bug is fixed
    (range-checked `u8::try_from`), reviewed PASS.
- **CI RED (sole remaining gate):** `cargo-semver-checks` fails `enum_marked_non_exhaustive` on
    `enum Version` — marking a public enum `#[non_exhaustive]` requires a major bump (exit 100,
    enforcing check-run). This is the ONLY red gate now; the CRAP regression that also fired at 175
    was cleared at 176 by re-baselining `decode_header` (CC 16, 100% covered) into
    `.crap-baseline.json`.
- **`gen_iscc_id_v1` minting absent** — grep of `crates/iscc-lib/src/` finds no definition; 32 of 33
    targeted Tier 1 symbols present. Only other unmet line: `crate >= 1.0.0` (0.5.0, human-gated).
- All 10 `gen_*_v0` conformant, no `unsafe` outside FFI, Unicode criteria unchanged.

## Python Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-py` exports 32 symbols with `IsccResult`, streaming hashers, 12 `.detach(` GIL sites,
    abi3-py310 wheels. IDv1 decode inherits from core; **no `gen_iscc_id_v1`**.
- Carries the `iscc_decode` differential conformance test (`test_iscc_decode_conformance.py`);
    pushed and part of the (green-except-semver) CI run.

## Node.js Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-napi` exports 32 symbols with streaming classes, Unicode-gated. IDv1 decode inherits
    from core; **no `gen_iscc_id_v1`.** Untouched this iteration.

## WASM Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-wasm` exports 32 symbols, `SumHasher` wrapper, Unicode-gated. IDv1 decode inherits
    from core; **no `gen_iscc_id_v1`.** `ci.yml` guard asserting the `blake3 wasm32_simd` feature
    stays enabled is pushed and part of the CI run.

## C FFI

**Status**: partially met — 32/33 symbols

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`, Unicode-gated; untouched. IDv1
    decode inherits from core; **no `gen_iscc_id_v1`.**

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
- Tabbed multi-language examples do not yet cover an IDv1 surface.

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` issue: iai text benchmarks are ASCII-only, so the >10% perf gate is blind to the
    Unicode freeze path — a coverage gap, not a missing benchmark.

## CI/CD and Publishing

**Status**: RED on develop — top priority. Dependency freshness met; release-readiness blocked

- **CI is FAILING on `origin/develop` = `a961c75`:** check-runs API reports 45 runs, **2 failures,
    all `Semver (cargo-semver-checks)`**. Cause: `#[non_exhaustive]` on public `enum Version` from
    the IDv1 batch, rejected vs the 0.5.0 semver baseline. The CRAP regression that co-fired at 175
    is now GREEN (176 re-baseline). Local `mise run check` catches neither — semver runs only in CI
    vs the last release.
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

**Get CI green on develop first** — the sole remaining red gate is `cargo-semver-checks` rejecting
`#[non_exhaustive]` on public `enum Version` (it wants a major bump vs 0.5.0). This needs a
decision: drop the marker, allowlist/configure the lint as an accepted pre-1.0 exception, or move
the semver baseline. Only after CI is green, resume experimental ISCC-IDv1: `gen_iscc_id_v1` minting
across core and all 11 surfaces plus the Go rename (`EncodeIsccID` → `GenIsccIDV1`, delete
`DecodeIsccID`/`IsccIDv1Result`) and the Tier-1 32→33 doc sweep — the last CID-doable v0.6.0
release-readiness blockers.
