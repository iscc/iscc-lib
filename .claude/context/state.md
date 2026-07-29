<!-- assessed-at: 40b72ced8d9bd25d40f5f72f0d3f48910ee04375 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — `gen_iscc_id_v1` minting now on core + Python + Go; CI is GREEN

Iteration 179 finished the Go IDv1 sub-item (reviewed PASS): the superseded
`EncodeIsccID`/`DecodeIsccID`/`IsccIDv1Result` are gone, replaced by the canonical
`GenIsccIDV1(timestamp, hubID, realm)`, with decode via the generic `IsccDecode` path. Core, Python
and Go now carry all 33 targeted Tier 1 symbols; **9 language surfaces still lack IDv1 minting.** CI
is GREEN on `develop` (the `Semver` red is `continue-on-error`/informational).

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

## Other Bindings (Go, Java, Kotlin, C#, C++, Ruby, Swift)

**Status**: partially met — Go IDv1 complete; other 6 at 32/33 with no IDv1 minting

- **Go: DONE.** `packages/go/iscc_id.go` now has `GenIsccIDV1(timestamp uint16..., realm)`; no
    `EncodeIsccID`/`DecodeIsccID`/`IsccIDv1Result` remain anywhere under `packages/go/`.
    `IsccDecode` accepts MainType `Id` Version 1 for decode round-trip.
- **Java, Kotlin, C#, C++, Ruby, Swift:** each at 32 symbols, no `gen_iscc_id_v1`. Untouched; uniffi
    0.32 (172) and MSRV fix (173) in place.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 ==
    `VENDORED_COPIES`.

## Documentation

**Status**: partially met

- Docs site (23 pages, 12 READMEs, 12 CLAUDE.md) met; `docs/howto/go.md` + `packages/go/README.md`
    updated to `GenIsccIDV1`.
- Tabbed multi-language examples do not yet cover a non-Go IDv1 surface; Tier-1 count text still
    reads 32 in stale sites (separate 32→33 sweep step under #43).

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` issue: iai text benchmarks are ASCII-only, so the >10% perf gate is blind to the
    Unicode freeze path — a coverage gap, not a missing benchmark.

## CI/CD and Publishing

**Status**: met (CI green); dependency freshness met; release-readiness still open

- **CI GREEN on `origin/develop` = `2223d25`:** all 23 check names pass except
    `Semver (cargo-semver-checks)` which is `continue-on-error: true` (informational until v1.0.0) —
    expected non-blocking red, not a CI failure. HEAD `40b72ce` is the `cid(log)` commit;
    `git diff origin/develop..HEAD -- . ':!.claude'` is empty, so green covers HEAD.
- Dependency freshness met (criterion 0.8 at 171, uniffi 0.32 at 172); zero `# held:`/`authorized`
    pin comments in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion **unmet**: 0 critical, 4 open `normal`.

## Open Issues

**11 entries — 0 `critical`, 4 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.**

- **NORMAL:** ISCC-IDv1 unsupported outside Go (`[human]`, the live v0.6.0 work — Go now done, 9
    surfaces remain); codec input cleaning diverges from `iscc_clean` (`[review]`); iai text
    benchmarks ASCII-only (`[review]`); go1.27 tripwire (`[review]`, blocked on upstream).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**CI is green — no CI fix needed.** Continue the #43 IDv1 fan-out: propagate experimental
`gen_iscc_id_v1` minting from core to the remaining 9 language surfaces (napi, wasm, ffi, jni, rb,
uniffi→Swift/Kotlin, dotnet, cpp), widen each surface's own version enum so
`iscc_decode(gen_iscc_id_v1(...))` round-trips (Python `VS`, etc.), and run the Tier-1 32→33
doc/count sweep — the last CID-doable v0.6.0 release-readiness blockers.
