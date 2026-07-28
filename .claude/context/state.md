<!-- assessed-at: 1b7b792540ab32061829dede24ecb7537316e28f -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 endgame — two authorized dependency majors are the last release criterion

The human unblocked the loop on 2026-07-28: `target.md` now names **v0.6.0** as the active
milestone, both held dependency majors (`criterion` 0.8, `uniffi` 0.32) are authorized as separate
steps, and the spec drift the loop could not fix itself was corrected by hand. Iteration 170 closed
the last floating-dependency surface (.NET lock file). CI is green; open issues are down to 2
`normal` (one of them the release-gating dependency issue) and 6 `low`.

## Rust Core Crate

**Status**: met, except the human-held v1.0.0 cut

- No source under `crates/iscc-lib/`, `benches/` or `tests/` moved since 8ea3e02: 32 Tier 1 symbols,
    342 `#[test]`, all 10 `gen_*_v0` conformant, no `unsafe` outside the FFI crates. All four
    Unicode criteria still met (declared 16.0.0 + sentinel freeze, `Final_Sigma` case freeze,
    boundary vectors on 11 of 11 surfaces, fail-closed differential sweep gate).
- Only unmet target line: `crate is >= 1.0.0`. Version is **0.5.0**; that cut is human-gated and
    explicitly out of scope for v0.6.0. `cargo-semver-checks` runs informational.
- Root `Cargo.toml` no longer carries any `# held:` pin. The two rationale comments now read
    `authorized 2026-07-28 (pending bump)` — the versions themselves are unchanged (`Cargo.lock`:
    criterion **0.7.0**, uniffi **0.31.2**), so neither bump has landed.
- `specs/rust-core.md` L149-157 stays STALE (claims the Go `Final_Sigma` defect unfixed; a
    `str::to_lowercase()` equivalence falsified by measurement at 156). Now correctable without
    escalation under the widened review policy, but nobody has filed it.

## Python Bindings

**Status**: met

- 32 Tier 1 symbols, `IsccResult`, streaming hashers, 12 `.detach(` GIL sites, abi3-py310 wheels
    incl. aarch64; 441 collected pytest tests. Nothing under `crates/iscc-py/` or `tests/` moved.
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

**Status**: met — every surface Unicode-gated; only `packages/dotnet` changed

- **.NET reproducibility gap closed (170).** `packages/dotnet/Iscc.Lib.Tests/packages.lock.json` is
    tracked, the three test `PackageReference` entries are exact (`18.8.1` / `3.2.2` / `3.1.5`), and
    a repo-wide grep for a floating `Version="…*"` finds nothing. Every ecosystem now has a
    lockfile.
- `crates/iscc-jni` (jni 0.22, 1152 lines, 33 natives, all JUnit-covered), `crates/iscc-rb` (magnus
    0.8.2), `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`) and
    `packages/{cpp,go,kotlin,swift}` are unchanged since 169.
- `uniffi` 0.32 will regenerate the two checked-in bindings
    (`packages/swift/Sources/IsccLib/iscc_uniffi.swift`,
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt`). Kotlin verifies locally;
    Swift does not — the `swift` CI job is its only verification.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 tracked
    paths, matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`.
- go1.27 remains a standing tripwire (upstream at rc2, no final tag): the bump reds 5 Go boundary
    cases unless the 731-range freeze table lands in the same step.

## Documentation

**Status**: met

- Nothing under `docs/` moved since 2840c7f (161): 23 pages across `zensical.toml` nav,
    `ORDERED_PAGES` and `docs/llms.txt`; 11 `docs/howto/*.md`; 12 crate/package READMEs; 12
    crate/package `CLAUDE.md`. `docs/unicode.md` names all 11 gated surfaces.
- **The doc-drift issue is fully resolved.** `crates/iscc-rb/CLAUDE.md:108` now teaches
    `ruby.str_from_slice(&bytes)` (170), and the human deleted both stale claims in
    `specs/java-bindings.md` — greps for `v0.21` and `1060` there are empty.
- Remaining cosmetic item is human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- 12 criterion benches (criterion 0.7) + `benches/iai_benches.rs` (iai-callgrind 0.16, 11 fns / 16
    cases); 18 pytest-benchmark fixtures; documented speedups 1.3x-158x.
- `.iai-baseline.json` and `.crap-baseline.json` byte-untouched (CRAP still 105 entries) — correct,
    since no `iscc-lib` core source moved. The criterion 0.8 bump touches these two bench targets.

## CI/CD and Publishing

**Status**: partially met — green, with the v0.6.0 dependency work outstanding

- **CI GREEN.** `origin/develop` = `6ee27dc` (the 170 review commit): check-runs API reports **45
    runs, 23 distinct names, 0 non-success**. HEAD `1b7b792` is two commits ahead; its only
    non-`.claude` change is the **comment-only** pin-rationale edit in root `Cargo.toml`, so the
    green run covers every line of code in the tree. Working tree is clean.
- Job shape changed for the first time since 162: the `dotnet` job gained a
    `dotnet restore … --locked-mode` step and build/test now pass `--no-restore`. Job *count* is
    unchanged (21 job keys → 22 jobs → 23 check names), so the gated job-table parity still holds.
- `release.yml` untouched: zero `@main` action refs, 97 `uses:`, 8 registry toggles,
    `workflow_dispatch`-only. Action freshness was re-probed 2026-07-28 across all 25 distinct
    `uses:` refs and is a confirmed no-op — that bullet is gone from the dependency issue.
- PR **#44** `develop` → `main` ("Release 0.6.0") is OPEN; version **0.5.0**,
    `scripts/version_sync.py` 21 targets consistent.
- Enforcing gates unchanged: iai perf (>10% Ir), coverage + CRAP (`--fail-regression` is CI-only, so
    a green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can red with no code
    change), docs page-list parity, `unicode-sweep`, CI job-table parity.
- Dependency freshness is the one unmet criterion: `criterion` 0.7 → 0.8 and `uniffi` 0.31 → 0.32,
    both authorized, both still pending.

## Open Issues

**8 entries in `issues.md` — 2 `normal`, 6 `low`, zero `critical`, and now zero HUMAN REVIEW
REQUESTED markers.**

- **NORMAL:** the dependency refresh (release-gating, two authorized steps); the go1.27 tripwire
    (parked on an upstream final release).
- **LOW:** upstream `iscc-core#137` thread, the three gate-script remainders deferred at 146, v1.0.0
    (HELD), **MSRV asserted but never verified** (new — a v1.0.0 prerequisite, `[human]`, CID must
    not act unprompted), npm OIDC (deferred for v0.6.0), docs language logos.
- Loop policy widened at HEAD: `review` may now correct a *mechanically checkable fact* (version,
    path, file list, count) in a sub-spec under `.claude/context/specs/` without escalation;
    `target.md` and anything touching rationale/criteria/scope still need the human.

## Next Milestone

Close the dependency refresh issue — the last remaining v0.6.0 release criterion. Both items are
authorized and independently schedulable, and the issue body carries the binding constraints for
each. Everything else in the backlog is human-gated (v1.0.0 cut and its MSRV prerequisite, npm OIDC,
docs logos, the upstream thread) or parked on upstream (go1.27).
