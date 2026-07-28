<!-- assessed-at: c9069a0507cdf524d9ccb07c57b05b406c9304cd -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 endgame — one authorized dependency major left

Iteration 171 landed the `criterion` 0.7 → 0.8 dev-dependency bump, leaving `uniffi` 0.31 → 0.32 as
the single remaining item in the release-gating dependency issue and therefore the last v0.6.0
criterion. CI is green over every line of code in the tree. Open issues stay at 8 (2 `normal`, 6
`low`, zero `critical`, zero HUMAN REVIEW REQUESTED).

## Rust Core Crate

**Status**: met, except the human-held v1.0.0 cut

- No source under `crates/iscc-lib/src`, `crates/iscc-lib/tests` or `crates/iscc-lib/benches` moved
    since a7e84c8: 32 Tier 1 symbols, 342 `#[test]`, all 10 `gen_*_v0` conformant, no `unsafe`
    outside the FFI crates. All four Unicode criteria still met (declared 16.0.0 + sentinel freeze,
    `Final_Sigma` case freeze, boundary vectors on 11 of 11 surfaces, fail-closed sweep gate).
- Only unmet target line: `crate is >= 1.0.0`. Version is **0.5.0**; that cut is human-gated and
    explicitly out of scope for v0.6.0. `cargo-semver-checks` runs informational.
- The only code-tree change this iteration is the root `Cargo.toml` / `Cargo.lock` criterion pin
    (dev-dependency, `Cargo.lock` now criterion **0.8.2**). `rust-version = "1.85"` unchanged; the
    pin comment now records why criterion's rustc-1.86 floor does not touch the published MSRV.
- `Cargo.lock` still has `uniffi` **0.31.2** — that bump has not landed.
- `specs/rust-core.md` L149-157 stays STALE (claims the Go `Final_Sigma` defect unfixed; a
    `str::to_lowercase()` equivalence falsified by measurement at 156). Correctable without human
    escalation under the widened review policy, but nobody has filed it.

## Python Bindings

**Status**: met

- 32 Tier 1 symbols, `IsccResult`, streaming hashers, 12 `.detach(` GIL sites, abi3-py310 wheels
    incl. aarch64; 441 collected pytest tests. Nothing under `crates/iscc-py/` moved since 1f5ed02.
- The 17.8M-comparison Unicode sweep deliberately stays out of `pytest` / `mise run test` / pre-push
    — do not wire it in.

## Node.js Bindings

**Status**: met — Unicode-gated since 153

- `crates/iscc-napi` exports all 32 symbols with streaming classes; untouched.
- `__tests__/unicode_boundary.test.mjs`: `node:test`, metadata guard + 12 vectors, zero skips.

## WASM Bindings

**Status**: met — Unicode-gated since 151

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    dep is intentional feature-unification (do not prune). Untouched.
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns via `include_str!`, zero skips.

## C FFI

**Status**: met — Unicode-gated at 159

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`; untouched.
- `tests/unicode_boundary_vectors.h` (tracked, generated) is consumed by both
    `crates/iscc-ffi/tests/test_iscc.c` and `packages/cpp/tests/test_iscc.cpp` from one artifact.

## Other Bindings (Java, Kotlin, C#, C++, Go, Ruby, Swift)

**Status**: met — every surface Unicode-gated; nothing moved since 1f5ed02 (170)

- `crates/iscc-jni` (jni 0.22, 33 natives, all JUnit-covered), `crates/iscc-rb` (magnus 0.8.2),
    `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`) are unchanged, as are all five
    directories under `packages/` (cpp, dotnet, go, kotlin, swift).
- Every ecosystem carries a lockfile since 170 (`packages/dotnet/Iscc.Lib.Tests/packages.lock.json`
    tracked, 3 exact test pins, `--locked-mode` restore in CI).
- `uniffi` 0.32 will regenerate the two checked-in bindings
    (`packages/swift/Sources/IsccLib/iscc_uniffi.swift`,
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt`). Kotlin verifies locally;
    Swift is locally verifiable too via the swift.org tarball recipe in `packages/swift/CLAUDE.md`
    (corrected in issues.md at 171) — the `swift` CI job stays final. The root `Cargo.toml` pin
    comment above `uniffi = "0.31"` still repeats the falsified "Swift is not verifiable locally"
    claim.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 tracked
    paths, matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`.
- go1.27 remains a standing tripwire (upstream at rc2, no final tag): the bump reds 5 Go boundary
    cases unless the 731-range freeze table lands in the same step.

## Documentation

**Status**: met

- Nothing under `docs/` moved since 2840c7f (161): 23 pages across `zensical.toml` nav,
    `ORDERED_PAGES` and `docs/llms.txt`; 11 `docs/howto/*.md`; 12 crate/package READMEs; 12
    crate/package `CLAUDE.md`. `docs/unicode.md` names all 11 gated surfaces.
- Both doc-drift surfaces (Ruby `CLAUDE.md` Magnus API, `specs/java-bindings.md`) stay closed.
- Remaining cosmetic item is human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- 12 criterion bench functions in `crates/iscc-lib/benches/benchmarks.rs` (now **criterion 0.8.2**,
    harness bumped with zero bench-source edits) + `benches/iai_benches.rs` (iai-callgrind 0.16, 11
    fns / 16 cases); 18 pytest-benchmark fixtures; documented speedups 1.3x-158x.
- `.iai-baseline.json` and `.crap-baseline.json` byte-untouched (CRAP still 105 entries) — correct,
    since no `iscc-lib` core source moved and criterion is not on the iai path.

## CI/CD and Publishing

**Status**: partially met — green, with the uniffi bump outstanding

- **CI GREEN.** `origin/develop` = `bc13859` (the 171 review commit): check-runs API reports **45
    runs, 23 distinct names, 0 non-success**. HEAD `c9069a0` is one commit ahead and is a
    `cid(log):` commit touching only `.claude/`, so the green run covers every line of code in the
    tree. Working tree clean.
- Job shape unchanged since 170: 21 job keys → 22 jobs → 23 check names; gated job-table parity
    holds. `release.yml` untouched — zero `@main` refs, 97 `uses:`, 8 registry toggles,
    `workflow_dispatch`-only. All 25 distinct action refs re-probed current on 2026-07-28.
- PR **#44** `develop` → `main` ("Release 0.6.0") is OPEN; version **0.5.0**,
    `scripts/version_sync.py` 21 targets consistent.
- Enforcing gates unchanged: iai perf (>10% Ir), coverage + CRAP (`--fail-regression` is CI-only, so
    a green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can red with no code
    change), docs page-list parity, `unicode-sweep`, CI job-table parity.
- Dependency freshness is the one unmet criterion, and it is now a single item: `uniffi` 0.31 →
    0.32. The criterion half was deleted from the issue after 171.

## Open Issues

**8 entries in `issues.md` — 2 `normal`, 6 `low`, zero `critical`, zero HUMAN REVIEW REQUESTED.**

- **NORMAL:** the dependency refresh (release-gating, now one authorized item — uniffi 0.32); the
    go1.27 tripwire (parked on an upstream final release).
- **LOW:** upstream `iscc-core#137` thread, the three gate-script remainders deferred at 146, v1.0.0
    (HELD), MSRV asserted but never verified (a v1.0.0 prerequisite, `[human]`), npm OIDC (deferred
    for v0.6.0), docs language logos.
- Review policy (widened at 171): `review` may correct a *mechanically checkable fact* (version,
    path, file list, count) in a sub-spec under `.claude/context/specs/` without escalation;
    `target.md` and anything touching rationale/criteria/scope still need the human.

## Next Milestone

Land the `uniffi` 0.31 → 0.32 bump — the last open item of the dependency-refresh issue and the last
CID-doable v0.6.0 release criterion. Its binding constraints (pure regeneration of both checked-in
bindings, stop-and-rescope if `crates/iscc-uniffi/src` needs edits) are recorded in the issue body.
Everything else in the backlog is human-gated (v1.0.0 cut and its MSRV prerequisite, npm OIDC, docs
logos, the upstream thread) or parked on upstream (go1.27).
