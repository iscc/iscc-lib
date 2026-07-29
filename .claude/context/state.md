<!-- assessed-at: 61807f35b3c4616288f7598cc2dd4d9704fc65b4 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — `gen_iscc_id_v1` minting now on core + Python + Go + Node.js; CI is GREEN

Iteration 180 landed the napi (Node.js) IDv1 minting slice (reviewed PASS_WITH_NOTES): napi now
exports `gen_iscc_id_v1` at 33/33 Tier 1 symbols. Core, Python, Go and Node.js carry all 33 targeted
symbols; **8 language surfaces still lack IDv1 minting.** CI is GREEN on `develop` (the `Semver` red
is `continue-on-error`/informational).

## Rust Core Crate

**Status**: met (feature-complete for v0.6.0)

- **33/33 Tier 1 symbols present:** `gen_iscc_id_v1(timestamp, hub_id, realm)` additive/clock-free,
    oracle-matched vs `iscc-core`; IDv1 decode via generic `iscc_decode`/`iscc_decompose` (accepts
    `Id` Version 1). All 10 `gen_*_v0` conformant, no `unsafe` outside FFI, Unicode criteria met.
- **Semver check-run reports `failure`** (`enum_marked_non_exhaustive` on public `enum Version`),
    but the job is `continue-on-error: true` — informational, does not fail CI. Becomes a real
    blocker only at the human-gated v1.0.0 cut.
- Only unmet criterion is `crate >= 1.0.0` (0.5.0, human-gated).

## Python Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-py` exports `gen_iscc_id_v1` (iter 178): `__init__.py`, `.pyi`, `src/lib.rs`, plus
    `tests/test_iscc_id_v1.py` differential grid + golden + validations.
- Retains `IsccResult`, streaming hashers, GIL `.detach(` sites, abi3-py310 wheels. IDv1 decode
    round-trip still gated by the Python `VS` IntEnum (tracked under #43); minting is complete.

## Node.js Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-napi/src/lib.rs:313` exports `gen_iscc_id_v1(timestamp: f64, hub_id, realm)` (iter
    180), golden vector oracle-confirmed, `index.d.ts` declaration + `__tests__/iscc_id_v1.test.mjs`
    round-trip. Streaming classes and Unicode gate intact.
- **Known gap (open `normal` issue):** the `f64`/JS-number path silently coerces invalid inputs
    (negative/NaN/non-integral/overflow) into valid-but-wrong IDs instead of throwing. Out of scope
    for the minting slice; the JS-number validation approach is to be settled on the wasm slice.

## WASM Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-wasm` exports 32 symbols, `SumHasher` wrapper, Unicode-gated, `blake3 wasm32_simd`
    feature guarded in ci.yml. IDv1 decode inherits from core; **no `gen_iscc_id_v1`.** Untouched.

## C FFI

**Status**: partially met — 32/33 symbols

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`, Unicode-gated. IDv1 decode
    inherits from core; **no `gen_iscc_id_v1`** (adding it needs `iscc.h` regen + freshness gate).
    Untouched.

## Other Bindings (Go, Java, Kotlin, C#, C++, Ruby, Swift)

**Status**: partially met — Go IDv1 complete; other 6 at 32/33 with no IDv1 minting

- **Go: DONE.** `packages/go/iscc_id.go` has `GenIsccIDV1(timestamp, hubID, realm)`; no
    `EncodeIsccID`/`DecodeIsccID`/`IsccIDv1Result` remain anywhere under `packages/go/`.
    `IsccDecode` accepts MainType `Id` Version 1 for decode round-trip.
- **Java, Kotlin, C#, C++, Ruby, Swift:** each at 32 symbols, no `gen_iscc_id_v1`. Untouched; uniffi
    0.32 (172) and MSRV fix (173) in place.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 ==
    `VENDORED_COPIES`.

## Documentation

**Status**: partially met

- Docs site (23 pages, 12 READMEs, 12 CLAUDE.md) met; `docs/howto/go.md` + `packages/go/README.md`
    on `GenIsccIDV1`.
- Tabbed multi-language examples do not yet cover a non-Go IDv1 surface; Tier-1 count text still
    reads 32 in stale sites (separate 32→33 sweep step under #43).

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` issue: iai text benchmarks are ASCII-only, so the >10% perf gate is blind to the
    Unicode freeze path — a coverage gap, not a missing benchmark.

## CI/CD and Publishing

**Status**: met (CI green); dependency freshness met; release-readiness still open

- **CI GREEN on `origin/develop` = `9bec4aa`:** check-suite conclusion `success`; all 23 check names
    pass except `Semver (cargo-semver-checks)` which is `continue-on-error: true` (informational
    until v1.0.0) — expected non-blocking red, not a CI failure. HEAD `61807f3` adds only `cid(log)`
    - `cid(audit)` commits; `git diff origin/develop..HEAD -- . ':!.claude'` is empty, so green covers
        HEAD.
- Dependency freshness met (criterion 0.8 at 171, uniffi 0.32 at 172); zero `# held:`/`authorized`
    pin comments in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion **unmet**: 0 critical, 5 open `normal`.

## Open Issues

**12 entries — 0 `critical`, 5 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.**

- **NORMAL:** JS-number IDv1 minting coerces invalid inputs instead of throwing (`[review]`, filed
    iter 180); ISCC-IDv1 unsupported outside Go (`[human]`, the live v0.6.0 work — Go+Node done, 8
    surfaces remain); codec input cleaning diverges from `iscc_clean` (`[review]`); iai text
    benchmarks ASCII-only (`[review]`); go1.27 tripwire (`[review]`, blocked on upstream).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**CI is green — no CI fix needed.** Continue the #43 IDv1 fan-out: propagate experimental
`gen_iscc_id_v1` minting from core to the remaining 8 language surfaces (wasm, ffi, jni, rb,
uniffi→Swift/Kotlin, dotnet, cpp), settling the JS-number input-validation approach on the wasm
slice; widen each surface's version enum so `iscc_decode(gen_iscc_id_v1(...))` round-trips (Python
`VS`, etc.); and run the Tier-1 32→33 doc/count sweep — the last CID-doable v0.6.0 release-readiness
blockers.
