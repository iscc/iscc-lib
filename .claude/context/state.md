<!-- assessed-at: 2c4e4871cb1a0b4976ff33b5c9f822adccd84bdc -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — new ISCC-IDv1 scope opened; first partial step landed out-of-loop and is unpushed/CI-unverified

An out-of-loop human/agent commit (`2c4e487`, non-`cid()`) rewrote `target.md` and every binding
spec to require **33 Tier 1 symbols** and **experimental ISCC-IDv1 support on the core plus all 11
surfaces**, then landed a partial first step: `iscc_decode` now normalizes unit sequences (core +
Go) and Go's `IsccDecode` accepts MainType `ID` Version 1. `gen_iscc_id_v1` exists nowhere, the Rust
core codec still rejects Version 1, and Go still carries the old names the target says to rename or
delete. The whole 471-line code delta sits ahead of `origin/develop` and CI has not run on it.

## Rust Core Crate

**Status**: partially met (was met at 32 symbols; new IDv1 scope is largely unmet)

- 32 of the 33 targeted Tier 1 symbols present; **`gen_iscc_id_v1` is absent** (only a Go code
    comment references it). All 10 `gen_*_v0` conformant, no `unsafe` outside FFI, Unicode criteria
    unchanged.
- **IDv1 decode unmet in core:** `codec::Version` (codec.rs:99-109) still rejects any non-V0 with
    `invalid Version: {value}`, so `iscc_decode`/`iscc_decompose` do not accept MainType `ID`
    Version 1.
- HEAD reworked `iscc_decode` to normalize a unit sequence into its composite before decoding
    (matching `iscc_core.iscc_decode`), adding `iscc_normalize`, 4 tests, and removing the old
    trailing-byte-rejection test. **Unpushed and not CI-verified.**
- Only other unmet line remains `crate is >= 1.0.0` (0.5.0, human-gated). `rust-version = "1.85"`
    still true for `iscc-lib`.

## Python Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-py` still exports 32 symbols with `IsccResult`, streaming hashers, 12 `.detach(` GIL
    sites, abi3-py310 wheels. **No `gen_iscc_id_v1`; no Version-1 ID decode** (inherits core).
- HEAD updated the `iscc_decode` docstring in `_lowlevel.pyi` and added
    `tests/test_iscc_decode_conformance.py` (108 lines, differential vs `iscc_core`) — unpushed,
    CI-unverified.

## Node.js Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-napi` exports 32 symbols with streaming classes, Unicode-gated. **No
    `gen_iscc_id_v1`, no Version-1 ID decode.** Crate untouched this iteration.

## WASM Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-wasm` exports 32 symbols, `SumHasher` wrapper, Unicode-gated. **No `gen_iscc_id_v1`,
    no Version-1 ID decode.**
- HEAD added a CI guard (`ci.yml`) asserting the `blake3 wasm32_simd` feature stays enabled — good
    hardening, but unpushed/CI-unverified.

## C FFI

**Status**: partially met — 32/33 symbols

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`, Unicode-gated; untouched. **No
    `gen_iscc_id_v1`, no Version-1 ID decode.**

## Other Bindings (Java, Kotlin, C#, C++, Go, Ruby, Swift)

**Status**: partially met — Go started IDv1, others at 32/33 with no IDv1

- **Go:** `IsccDecode` in `codec.go` now accepts MainType `ID` Version 1 and normalizes unit
    sequences — the decode half of IDv1. But the target's rename is **not** done: `EncodeIsccID`,
    `DecodeIsccID`, and `IsccIDv1Result` still exist in `packages/go/iscc_id.go`; there is no
    `GenIsccIDV1`. Unpushed/CI-unverified.
- **Java, Kotlin, C#, C++, Ruby, Swift:** each still at 32 symbols, no `gen_iscc_id_v1`, no
    Version-1 ID decode. Crates untouched; uniffi 0.32 and MSRV fixes from 172-173 remain in place.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 paths ==
    `VENDORED_COPIES`.

## Documentation

**Status**: partially met

- HEAD added a Go IDv1 how-to note (`docs/howto/go.md` +13, `packages/go/README.md` +4). The rest of
    the docs site (23 pages, 12 READMEs, 12 CLAUDE.md) is unchanged and met.
- Tabbed multi-language examples do not yet cover an IDv1 surface, and the IDv1 scope is only
    partially documented.

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` issue: the iai text benchmarks are ASCII-only, so the >10% perf gate is blind to the
    Unicode freeze path — a gate-coverage gap, not a missing benchmark.

## CI/CD and Publishing

**Status**: dependency freshness met; HEAD code unverified; release-readiness blocked by IDv1 scope

- **CI GREEN only up to `origin/develop` = `ab2d0e1`** (173 review): check-runs API reports 45 runs,
    23 names, 0 non-success. **HEAD `2c4e487` is 2 commits ahead with a 471-line, 11-file code delta
    (core `lib.rs`, Go, `ci.yml`, `.pyi`, new tests) that CI has NOT run.** Next push is its first
    CI exposure; treat all HEAD code as unverified.
- Dependency freshness met (criterion 0.8 at 171, uniffi 0.32 at 172); zero `# held:`/`authorized`
    pin comments in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 release-readiness criterion "no `critical`/`normal` issue open that is not blocked on human
    or upstream" is **unmet**: the new ISCC-IDv1 `normal` issue and the codec-cleaning `normal`
    issue are CID-doable and open.

## Open Issues

**11 entries — 4 `normal`, 7 `low`, zero `critical`, zero HUMAN REVIEW REQUESTED** (up from 8; the
out-of-loop commit added the IDv1 scope issues).

- **NORMAL:** ISCC-IDv1 unsupported outside Go + Go uses superseded names (`[human]`, the big new
    scope); codec input cleaning diverges from `iscc_clean` (`[review]` — partly addressed by the
    unpushed HEAD `iscc_decode` rework, still open); iai text benchmarks ASCII-only (`[review]`);
    go1.27 tripwire (`[review]`, blocked on upstream final tag).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

Deliver experimental ISCC-IDv1 across the core and all 11 surfaces per the expanded `target.md`:
`gen_iscc_id_v1` for minting plus generic `iscc_decode`/`iscc_decompose` acceptance of MainType `ID`
Version 1, and the Go rename (`EncodeIsccID` → `GenIsccIDV1`, delete
`DecodeIsccID`/`IsccIDv1Result`). First, the out-of-loop HEAD code (`iscc_decode` normalization + Go
Version-1 accept + WASM CI guard) must be pushed and pass CI — it is currently unverified. The
codec-cleaning and IDv1 `normal` issues are the last CID-doable v0.6.0 release-readiness blockers.
