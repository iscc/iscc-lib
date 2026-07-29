<!-- assessed-at: 825b74a5d07dc0b1d22af11ee93dcf4d1baf89dc -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — `gen_iscc_id_v1` minting reached core + Python (33 symbols); CI is GREEN

Iteration 178 exposed `gen_iscc_id_v1` on the Python binding with a differential test (reviewed
PASS), so core **and** Python now carry all 33 targeted Tier 1 symbols. The other 10 language
surfaces still lack IDv1 minting and Go still uses the superseded names. **CORRECTION vs prior
state:** CI is **GREEN** on `develop` — the `Semver (cargo-semver-checks)` check-run reports
`failure` but its job is `continue-on-error: true` (informational until v1.0.0), so the CI workflow
run conclusion is `success`. The "CI RED on Semver" carried since iter 176 was a phantom.

## Rust Core Crate

**Status**: met (feature-complete for v0.6.0)

- **33/33 Tier 1 symbols present:** `gen_iscc_id_v1(timestamp, hub_id, realm)` at `lib.rs:1064` with
    `IsccIdResult` (`types.rs:95`), additive/clock-free, oracle-matched vs `iscc-core` 1.3.0; IDv1
    decode via generic `iscc_decode`/`iscc_decompose` (accepts `Id` Version 1). All 10 `gen_*_v0`
    conformant, no `unsafe` outside FFI, Unicode criteria unchanged.
- **Semver check-run reports `failure`** (`enum_marked_non_exhaustive` on public `enum Version`
    wants a major bump vs 0.5.0), but the job is `continue-on-error: true` — informational, does not
    fail CI. It becomes a real blocker only at the human-gated v1.0.0 cut.
- Only unmet criterion is `crate >= 1.0.0` (0.5.0, human-gated).

## Python Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-py` now exports `gen_iscc_id_v1` (iter 178, reviewed PASS): `__init__.py`, `.pyi`
    stub, `src/lib.rs`, plus `tests/test_iscc_id_v1.py` differential grid + golden + 3 validations.
- Retains `IsccResult`, streaming hashers, GIL `.detach(` sites, abi3-py310 wheels, and the
    `iscc_decode` differential conformance test. IDv1 decode round-trip still gated by the Python
    `VS` IntEnum (tracked under #43); minting is complete.

## Node.js Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-napi` exports 32 symbols with streaming classes, Unicode-gated. IDv1 decode inherits
    from core; **no `gen_iscc_id_v1`.** Untouched this iteration.

## WASM Bindings

**Status**: partially met — 32/33 symbols

- `crates/iscc-wasm` exports 32 symbols, `SumHasher` wrapper, Unicode-gated, `blake3 wasm32_simd`
    feature guarded in ci.yml. IDv1 decode inherits from core; **no `gen_iscc_id_v1`.** Untouched.

## C FFI

**Status**: partially met — 32/33 symbols

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`, Unicode-gated. IDv1 decode
    inherits from core; **no `gen_iscc_id_v1`.** Untouched.

## Other Bindings (Java, Kotlin, C#, C++, Go, Ruby, Swift)

**Status**: partially met — Go started IDv1 decode, others at 32/33 with no IDv1 minting

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

**Status**: met (CI green); dependency freshness met; release-readiness still open

- **CI GREEN on `origin/develop` = `dcbb0bb`:** CI workflow run `conclusion: success`. All 23 check
    names pass except `Semver (cargo-semver-checks)` which is `continue-on-error: true`
    (informational until v1.0.0) — expected non-blocking red, not a CI failure. HEAD `825b74a` is
    the `cid(log)` commit; `git diff origin/develop..HEAD -- . ':!.claude'` is empty, so green
    covers HEAD.
- Dependency freshness met (criterion 0.8 at 171, uniffi 0.32 at 172); zero `# held:`/`authorized`
    pin comments in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion **unmet**: 0 critical, 4 open `normal`.

## Open Issues

**11 entries — 0 `critical`, 4 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.**

- **NORMAL:** ISCC-IDv1 unsupported outside Go + Go uses superseded names (`[human]`, the live
    v0.6.0 work); codec input cleaning diverges from `iscc_clean` (`[review]`); iai text benchmarks
    ASCII-only (`[review]`); go1.27 tripwire (`[review]`, blocked on upstream).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**CI is green — no CI fix needed.** Continue the #43 IDv1 fan-out: propagate experimental
`gen_iscc_id_v1` minting from core to the remaining 10 language surfaces, do the Go rename
(`EncodeIsccID` → `GenIsccIDV1`, delete `DecodeIsccID`/`IsccIDv1Result`), widen each surface's own
version enum so `iscc_decode(gen_iscc_id_v1(...))` round-trips (Python `VS`, etc.), and run the
Tier-1 32→33 doc/count sweep — the last CID-doable v0.6.0 release-readiness blockers.
