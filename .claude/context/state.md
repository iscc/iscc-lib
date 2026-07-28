<!-- assessed-at: 2916519e0aa6def669cfde6291e5c05be7c45980 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — ISCC-IDv1 decode landed in core (NEEDS_WORK), a critical truncation bug is open, whole batch unpushed/CI-unverified

Iteration 174 advanced the Rust core codec to accept ISCC-IDv1 (Version 1) for MainType `Id`, then
review returned **NEEDS_WORK**: the change made a latent `decode_header` truncation bug reachable
(malformed headers silently canonicalize to valid ISCCs), filed `critical`. `gen_iscc_id_v1` still
exists nowhere and Go still carries the names the target says to rename/delete. HEAD sits 4 commits
(576-line code delta) ahead of the green `origin/develop`; none of it has seen CI.

## Rust Core Crate

**Status**: partially met — IDv1 decode now in source, minting absent, one critical bug open

- **IDv1 decode now works in core:** `codec::Version` (codec.rs:103-116) is `#[non_exhaustive]` with
    `V0`/`V1`, and `validate_version` (127-133) permits `V1` only for MainType `Id`, so
    `iscc_decode`/`iscc_decompose` accept MainType `Id` Version 1 (byte-matching the `iscc_core`
    oracle per the 174 review). 7 new codec tests.
- **`gen_iscc_id_v1` minting is absent** — 32 of the 33 targeted Tier 1 symbols present; only a Go
    code comment references the symbol name.
- **Open `critical` bug:** `decode_header` (codec.rs:321-323) narrows fields with `as u8` before
    `TryFrom`, so a multi-nibble malformed header wraps and canonicalizes to a valid ISCC; adding
    `V1` made the v1-wrapping class newly reachable. Fix owed inside codec.rs.
- All 10 `gen_*_v0` conformant, no `unsafe` outside FFI, Unicode criteria unchanged. Only other
    unmet line: `crate >= 1.0.0` (0.5.0, human-gated). **All of the above is
    unpushed/CI-unverified.**

## Python Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-py` exports 32 symbols with `IsccResult`, streaming hashers, 12 `.detach(` GIL sites,
    abi3-py310 wheels. IDv1 decode inherits from core once rebuilt; **no `gen_iscc_id_v1`**.
- HEAD (out-of-loop `2c4e487`) updated the `iscc_decode` docstring in `_lowlevel.pyi` and added
    `tests/test_iscc_decode_conformance.py` (108 lines, differential vs `iscc_core`) — unpushed.

## Node.js Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-napi` exports 32 symbols with streaming classes, Unicode-gated. IDv1 decode inherits
    from core; **no `gen_iscc_id_v1`.** Crate untouched this iteration.

## WASM Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-wasm` exports 32 symbols, `SumHasher` wrapper, Unicode-gated. IDv1 decode inherits
    from core; **no `gen_iscc_id_v1`.**
- HEAD added a `ci.yml` guard asserting the `blake3 wasm32_simd` feature stays enabled — good
    hardening, unpushed/CI-unverified.

## C FFI

**Status**: partially met — 32/33 symbols

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`, Unicode-gated; untouched. IDv1
    decode inherits from core; **no `gen_iscc_id_v1`.**

## Other Bindings (Java, Kotlin, C#, C++, Go, Ruby, Swift)

**Status**: partially met — Go started IDv1, others at 32/33 with no IDv1 minting

- **Go:** `IsccDecode` in `codec.go` accepts MainType `Id` Version 1 and normalizes unit sequences —
    the decode half. But the target's rename is **not** done: `EncodeIsccID`, `DecodeIsccID`, and
    `IsccIDv1Result` still exist in `packages/go/iscc_id.go`; there is no `GenIsccIDV1`.
- **Java, Kotlin, C#, C++, Ruby, Swift:** each at 32 symbols, no `gen_iscc_id_v1`. Crates untouched;
    uniffi 0.32 (172) and MSRV fixes (173) remain in place.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 ==
    `VENDORED_COPIES`.

## Documentation

**Status**: partially met

- HEAD added a Go IDv1 how-to note (`docs/howto/go.md` +13, `packages/go/README.md` +4). Docs site
    (23 pages, 12 READMEs, 12 CLAUDE.md) otherwise unchanged and met.
- Tabbed multi-language examples do not yet cover an IDv1 surface.

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` issue: iai text benchmarks are ASCII-only, so the >10% perf gate is blind to the
    Unicode freeze path — a gate-coverage gap, not a missing benchmark.

## CI/CD and Publishing

**Status**: dependency freshness met; HEAD unverified; release-readiness blocked by IDv1 + critical
bug

- **CI GREEN only up to `origin/develop` = `ab2d0e1`** (173 review): check-runs API reports 45 runs,
    23 names, 0 non-success. **HEAD `2916519` is 4 commits ahead with a 576-line, 12-file code delta
    (core `codec.rs`/`lib.rs`, Go, `ci.yml`, `.pyi`, new tests, `tools/cid.py`) that CI has NOT
    run.** Batch is NEEDS_WORK and unpushed; treat all HEAD code as unverified.
- Dependency freshness met (criterion 0.8 at 171, uniffi 0.32 at 172); zero `# held:`/`authorized`
    pin comments in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion is **unmet**: one open `critical`
    (`decode_header` truncation) plus 4 open `normal` issues, several CID-doable.

## Open Issues

**12 entries — 1 `critical`, 4 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED** (up from 11; the 174
review filed the `decode_header` truncation critical).

- **CRITICAL:** `decode_header` truncates varnibble fields, canonicalizing malformed input
    (`[review]`) — fix owed before the batch is pushable.
- **NORMAL:** ISCC-IDv1 unsupported outside Go + Go uses superseded names (`[human]`); codec input
    cleaning diverges from `iscc_clean` (`[review]`, partly addressed by unpushed HEAD, still open);
    iai text benchmarks ASCII-only (`[review]`); go1.27 tripwire (`[review]`, blocked on upstream).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

Land the `decode_header` truncation fix (`critical`, scoped to codec.rs: range-checked
`u8::try_from` + a rejection test for `"MDFZAAAAAAAAAAAAAA"`), then push the whole IDv1-decode batch
green — it is the prerequisite the 174 review named. After that, deliver the rest of experimental
ISCC-IDv1 across the core and all 11 surfaces: `gen_iscc_id_v1` minting plus the Go rename
(`EncodeIsccID` → `GenIsccIDV1`, delete `DecodeIsccID`/`IsccIDv1Result`). These IDv1 and
codec-cleaning `normal` issues are the last CID-doable v0.6.0 release-readiness blockers.
