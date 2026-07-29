<!-- assessed-at: c4b2e1d5ad42bf9b82eaca238b479c8ea6cca929 -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 — `gen_iscc_id_v1` minting now on 6 surfaces (core+Python+Go+Node+WASM+C FFI); CI GREEN

Iteration 182 landed the C FFI `gen_iscc_id_v1` minting slice (reviewed PASS): `iscc.h` regenerated,
golden + realm-error C tests, `NativeMethods.g.cs` P/Invoke decl regenerated as a mechanical
side-effect. **6 of the 11 language surfaces now mint IDv1; 5 remain** (Java, Kotlin, C#, C++, Ruby,
Swift). CI is GREEN on `develop`.

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
- JS-number coercion resolved (iter 181): `f64` + `checked()` guard rejects non-finite /
    non-integral / negative / out-of-range inputs before narrowing.

## WASM Bindings

**Status**: met — 33/33 symbols

- `crates/iscc-wasm/src/lib.rs:371` exports `gen_iscc_id_v1` with the same validation guard as napi;
    `wasm-pack test` covers golden `ISCC:MAIGHFECJMOPMIAB`, decode round-trip, invalid-input throws.
    `SumHasher` wrapper, Unicode gate, `blake3 wasm32_simd` feature intact.

## C FFI

**Status**: met — 33/33 symbols (iter 182)

- **50** `#[unsafe(no_mangle)]` externs; `iscc_gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` at
    `crates/iscc-ffi/src/lib.rs:613` + committed `iscc.h:386`, regen idempotent (freshness gate
    green). Golden + realm-error C tests. No `checked()` guard needed — core re-validates ranges.
- cbindgen headers, `docs/howto/c-cpp.md`, `examples/iscc_sum.c` + CMake all present. Unicode-gated.

## Other Bindings (Java, Kotlin, C#, C++, Ruby, Swift)

**Status**: partially met — each at 32/33 with no IDv1 minting

- **Java (jni), Ruby (rb), Swift/Kotlin (uniffi), C++ (cpp):** each at 32 symbols, no
    `gen_iscc_id_v1`. uniffi 0.32 (172) and MSRV fix (173) in place.
- **C# (.NET):** `NativeMethods.g.cs` now carries the regenerated `iscc_gen_iscc_id_v1` P/Invoke
    decl (mechanical FFI-build side-effect, iter 182), but the idiomatic C# consumer + golden test
    are still owed in the dotnet step — surface counts 32 usable symbols.
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

- **CI GREEN on `origin/develop` = `5516c00`:** check-suite `success`; all 23 check names pass
    except `Semver (cargo-semver-checks)` which is `continue-on-error: true` (informational until
    v1.0.0) — expected non-blocking red, not a CI failure. HEAD `c4b2e1d` adds only a `cid(log)`
    commit; `git diff origin/develop..HEAD -- . ':!.claude'` is empty, so green covers HEAD.
- Dependency freshness met (criterion 0.8 at 171, uniffi 0.32 at 172); zero `# held:`/`authorized`
    pin comments in root `Cargo.toml`. Job shape unchanged (21 keys → 22 jobs → 23 names).
- PR **#44** `develop` → `main` OPEN; version **0.5.0**.
- v0.6.0 "no `critical`/`normal` issue open" criterion **unmet**: 0 critical, 4 open `normal`.

## Open Issues

**11 entries — 0 `critical`, 4 `normal`, 7 `low`, zero HUMAN REVIEW REQUESTED.**

- **NORMAL:** ISCC-IDv1 unsupported outside Go (`[human]`, the live v0.6.0 work — 6 surfaces done, 5
    remain); codec input cleaning diverges from `iscc_clean` (`[review]`); iai text benchmarks
    ASCII-only (`[review]`); go1.27 tripwire (`[review]`, blocked on upstream).
- **LOW:** upstream `iscc-core#137` thread, 88-bit ISCC-IDv0 upstream mint, gate-script remainders,
    v1.0.0 cut (HELD), MSRV asserted-not-verified, npm OIDC (deferred), docs language logos.

## Next Milestone

**CI is green — no CI fix needed.** Continue the #43 IDv1 fan-out: propagate experimental
`gen_iscc_id_v1` minting from core to the remaining 5 typed-int surfaces (jni next per handoff, then
rb, uniffi→Swift/Kotlin, dotnet C# consumer, cpp; core re-checks ranges so no JS-coercion hazard —
any FFI-symbol step must regen `iscc.h` and `NativeMethods.g.cs` in-step), widen each surface's
version enum so `iscc_decode(gen_iscc_id_v1(...))` round-trips (Python `VS`, etc.), and run the
Tier-1 32→33 doc/count sweep across the stale sites in #43 — the last CID-doable v0.6.0
release-readiness blockers.
