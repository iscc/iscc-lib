<!-- assessed-at: d559cfc954a7dfc942078a4bf1fff6878242a811 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — `gen_iscc_id_v1` minting now on core + Python + Go + Node.js + WASM; CI GREEN

Iteration 181 landed the WASM `gen_iscc_id_v1` minting slice (reviewed PASS) and retrofitted the
same `f64` + `checked()` input validation onto napi, resolving the JS-number coercion issue on both
surfaces. Core, Python, Go, Node.js and WASM now carry all 33 targeted Tier 1 symbols; **7 language
surfaces still lack IDv1 minting.** CI is GREEN on `develop`.

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

- `crates/iscc-py` exports `gen_iscc_id_v1` (iter 178) with differential grid + golden + validation
    tests. Retains `IsccResult`, streaming hashers, GIL `.detach(` sites, abi3-py310 wheels.
- IDv1 decode round-trip still gated by the Python `VS` IntEnum (tracked under #43); minting done.

## Node.js Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-napi/src/lib.rs:328` exports `gen_iscc_id_v1(timestamp: f64, hub_id, realm)`, golden
    oracle-confirmed, `index.d.ts` declaration + `__tests__/iscc_id_v1.test.mjs` round-trip.
- **JS-number coercion issue RESOLVED (iter 181):** napi now validates via a shared `f64` +
    `checked()` guard, rejecting non-finite / non-integral / negative / out-of-range inputs before
    narrowing. The `normal` coercion issue was closed and deleted.

## WASM Bindings

**Status**: met — 33/33 symbols (iter 181)

- `crates/iscc-wasm/src/lib.rs:371` exports `gen_iscc_id_v1(timestamp, hub_id, realm)` with the same
    validation guard as napi; `wasm-pack test` covers golden `ISCC:MAIGHFECJMOPMIAB`, decode
    round-trip (maintype 6 / version 1 / subtype 0), and invalid-input throws. `SumHasher` wrapper,
    Unicode gate, `blake3 wasm32_simd` feature all intact.

## C FFI

**Status**: partially met — 32/33 symbols

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`, Unicode-gated. IDv1 decode
    inherits from core; **no `gen_iscc_id_v1`** (adding it needs `iscc.h` regen + freshness gate).
    Next fan-out target (typed-int, no JS-coercion hazard). Untouched.

## Other Bindings (Go, Java, Kotlin, C#, C++, Ruby, Swift)

**Status**: partially met — Go IDv1 complete; other 6 at 32/33 with no IDv1 minting

- **Go: DONE.** `packages/go/iscc_id.go` has `GenIsccIDV1(timestamp, hubID, realm)`; no
    `EncodeIsccID`/`DecodeIsccID`/`IsccIDv1Result` remain. `IsccDecode` accepts `Id` Version 1.
- **Java, Kotlin, C#, C++, Ruby, Swift:** each at 32 symbols, no `gen_iscc_id_v1`. Untouched; uniffi
    0.32 (172) and MSRV fix (173) in place.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 ==
    `VENDORED_COPIES`.

## Documentation

**Status**: partially met

- Docs site (23 pages, 12 READMEs, 12 CLAUDE.md) met; `docs/howto/go.md` + `packages/go/README.md`
    on `GenIsccIDV1`.
- Tabbed multi-language examples do not yet cover a non-Go IDv1 surface; Tier-1 count text still
    reads 32/30 in stale sites (wasm CLAUDE.md "30 Tier 1", core CLAUDE.md "32") — separate 32→33
    sweep step under #43.

## Benchmarks

**Status**: met (existence), with a known gate-blindness gap

- 12 criterion fns + iai (11 fns/16 cases) + 18 pytest-benchmark fixtures; baselines untouched.
- Open `normal` issue: iai text benchmarks are ASCII-only, so the >10% perf gate is blind to the
    Unicode freeze path — a coverage gap, not a missing benchmark.

## CI/CD and Publishing

**Status**: met (CI green); dependency freshness met; release-readiness still open

- **CI GREEN on `origin/develop` = `ba9f2a8`:** check-suite `success`; all 23 check names pass
    except `Semver (cargo-semver-checks)` which is `continue-on-error: true` (informational until
    v1.0.0) — expected non-blocking red, not a CI failure. HEAD `d559cfc` adds only a `cid(log)`
    commit; `git diff origin/develop..HEAD -- . ':!.claude'` is empty, so green covers HEAD.
- Dependency freshness met (criterion 0.8 at 171, uniffi 0.32 at 172); zero `# held:`/`authorized`
    pin comments in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion **unmet**: 0 critical, 4 open `normal`.

## Open Issues

**11 entries — 0 `critical`, 4 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.**

- **NORMAL:** ISCC-IDv1 unsupported outside Go/Node/WASM (`[human]`, the live v0.6.0 work — 4
    surfaces done, 7 remain); codec input cleaning diverges from `iscc_clean` (`[review]`); iai text
    benchmarks ASCII-only (`[review]`); go1.27 tripwire (`[review]`, blocked on upstream).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**CI is green — no CI fix needed.** Continue the #43 IDv1 fan-out: propagate experimental
`gen_iscc_id_v1` minting from core to the remaining 7 typed-int surfaces (ffi first — needs `iscc.h`
regen + freshness gate — then jni, rb, uniffi→Swift/Kotlin, dotnet, cpp; core re-checks ranges so no
JS-coercion hazard), widen each surface's version enum so `iscc_decode(gen_iscc_id_v1(...))`
round-trips (Python `VS`, etc.), and run the Tier-1 32→33 doc/count sweep across the stale sites in
#43 — the last CID-doable v0.6.0 release-readiness blockers.
