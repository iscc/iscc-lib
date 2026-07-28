<!-- assessed-at: fb9452f4eac5bfbe565fb4230559d841c51a13fe -->

# Project State

## Status: IN_PROGRESS

## Phase: v0.6.0 endgame — dependency refresh finished, one new `normal` issue in the way

Iteration 172 landed `uniffi` 0.31 → 0.32 as a pure regeneration of the checked-in Swift and Kotlin
bindings, which closed and deleted the release-gating dependency-refresh issue: every authorized
major has now landed. The bump surfaced one defect — `iscc-uniffi` no longer builds on the declared
MSRV 1.85 — filed as `normal` and CID-doable, so it is now the last thing between the tree and the
human's v0.6.0 release criteria. CI is green over every line of code.

## Rust Core Crate

**Status**: met, except the human-held v1.0.0 cut

- Nothing under `crates/iscc-lib/src`, `tests` or `benches` moved since a7e84c8: 32 Tier 1 symbols,
    342 `#[test]`, all 10 `gen_*_v0` conformant, no `unsafe` outside the FFI crates. All four
    Unicode criteria still met (declared 16.0.0 + sentinel freeze, `Final_Sigma` case freeze,
    boundary vectors on 11 of 11 surfaces, fail-closed sweep gate).
- Only unmet target line: `crate is >= 1.0.0`. Version is **0.5.0**; that cut is human-gated and
    explicitly out of scope for v0.6.0. `cargo-semver-checks` runs informational.
- Root `Cargo.toml` `rust-version = "1.85"` unchanged and still true for `iscc-lib`
    (`cargo +1.85.0 check -p iscc-lib --locked` passes per the 172 review); it is now false for the
    unpublished `iscc-uniffi`, which inherits it — see Other Bindings.
- The two stale claims in `specs/rust-core.md` L149-157 were corrected in place at 172 (the Go
    `Final_Sigma` defect is fixed and no longer "tracked in issues.md"). No known spec drift left in
    this section.

## Python Bindings

**Status**: met

- 32 Tier 1 symbols, `IsccResult`, streaming hashers, 12 `.detach(` GIL sites, abi3-py310 wheels
    incl. aarch64; 441 collected pytest tests. Nothing under `crates/iscc-py/` moved since 531caf4.
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

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`; untouched since ea69169.
- `tests/unicode_boundary_vectors.h` (tracked, generated) is consumed by both
    `crates/iscc-ffi/tests/test_iscc.c` and `packages/cpp/tests/test_iscc.cpp` from one artifact.

## Other Bindings (Java, Kotlin, C#, C++, Go, Ruby, Swift)

**Status**: met — every surface Unicode-gated; only the two generated UniFFI bindings moved

- `uniffi` 0.32 regenerated `packages/swift/Sources/IsccLib/iscc_uniffi.swift` and
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` (the C header came out
    byte-identical, so it is not in the commit). `crates/iscc-uniffi/src` is untouched since 7742d7f
    — no hand edits, public API of both bindings unchanged.
- **New `normal` defect:** uniffi 0.32 pulls `cargo_metadata` 0.23.1 (1.86) and `cargo-platform`
    0.3.3 (1.91) into `iscc-uniffi`'s default non-dev graph, so that crate's real floor is 1.91
    while it inherits `rust-version.workspace = true` (1.85). `publish = false`, consumers and CI
    unaffected; the manifest declaration is simply false.
- `crates/iscc-jni` (jni 0.22, 33 natives, all JUnit-covered), `crates/iscc-rb` (magnus 0.8.2),
    `packages/{cpp,dotnet,go}` unchanged. Every ecosystem carries a lockfile since 170.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 tracked
    paths, matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`.
- go1.27 remains a standing tripwire (upstream at rc2, no final tag): the bump reds 5 Go boundary
    cases unless the 731-range freeze table lands in the same step.

## Documentation

**Status**: met

- Nothing under `docs/` moved since 2840c7f (161): 23 pages across `zensical.toml` nav,
    `ORDERED_PAGES` and `docs/llms.txt`; 11 `docs/howto/*.md`; 12 crate/package READMEs; 12
    crate/package `CLAUDE.md`. `docs/unicode.md` names all 11 gated surfaces.
- All known doc/spec drift is closed: Ruby `CLAUDE.md` (170), `specs/java-bindings.md` (human),
    `specs/kotlin-bindings.md` and `specs/rust-core.md` (172).
- Remaining cosmetic item is human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- 12 criterion bench functions in `crates/iscc-lib/benches/benchmarks.rs` (criterion 0.8.2) +
    `benches/iai_benches.rs` (iai-callgrind 0.16, 11 fns / 16 cases); 18 pytest-benchmark fixtures;
    documented speedups 1.3x-158x.
- `.iai-baseline.json` and `.crap-baseline.json` byte-untouched (CRAP still 105 entries) — correct,
    since no `iscc-lib` core source moved.

## CI/CD and Publishing

**Status**: met on dependency freshness; release-readiness blocked by one open `normal` issue

- **CI GREEN.** `origin/develop` = `d78ea63` (the 172 review commit): check-runs API reports **45
    runs, 23 distinct names, 0 non-success**. HEAD `fb9452f` is one commit ahead and is a
    `cid(log):` commit touching only `.claude/`, so the green run covers every line of code in the
    tree.
- Job shape unchanged since 170: 21 job keys → 22 jobs → 23 check names; gated job-table parity
    holds. `release.yml` untouched — zero `@main` refs, 97 `uses:`, 8 registry toggles,
    `workflow_dispatch`-only.
- **Dependency freshness is now met**: both authorized majors landed — criterion 0.8 at 171, uniffi
    0.32 at 172 — and the dependency-refresh issue was deleted. Root `Cargo.toml` now carries
    **zero** `# held:` and **zero** `authorized …` pin comments; the uniffi comment records the
    correct (locally verifiable) Swift recipe instead of the previously falsified claim.
- PR **#44** `develop` → `main` ("Release 0.6.0") is OPEN; version **0.5.0**, version-consistency
    gate green in CI.
- Enforcing gates unchanged: iai perf (>10% Ir), coverage + CRAP (`--fail-regression` is CI-only, so
    a green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can red with no code
    change), docs page-list parity, `unicode-sweep`, CI job-table parity.
- Outstanding confirmation from 172: the macOS `swift` job ran green on `d78ea63`, so the Linux-only
    local Swift verification is now backed by CI.

## Open Issues

**8 entries in `issues.md` — 2 `normal`, 6 `low`, zero `critical`, zero HUMAN REVIEW REQUESTED.**
Composition changed at 172 even though the count did not: the dependency-refresh entry was deleted,
the uniffi MSRV entry was added.

- **NORMAL:** `iscc-uniffi` no longer builds on the declared MSRV 1.85 (`[review]`, CID-doable, not
    blocked on anyone); the go1.27 tripwire (parked on an upstream final release, ~Aug 2026).
- **LOW:** upstream `iscc-core#137` thread, the three gate-script remainders deferred at 146, v1.0.0
    (HELD), MSRV asserted but never verified (a v1.0.0 prerequisite, `[human]`), npm OIDC (deferred
    for v0.6.0), docs language logos.
- v0.6.0 requires "no `critical` or `normal` issue open that is not blocked on the human or on an
    upstream release". go1.27 qualifies as blocked; the MSRV floor issue does not.

## Next Milestone

Correct the `iscc-uniffi` MSRV declaration so the crate states its real floor instead of inheriting
a false workspace value — the only CID-doable `normal` issue and the last unmet v0.6.0
release-readiness criterion. Raising the *root* `rust-version` is explicitly the human's call. After
that the backlog is entirely human-gated (v1.0.0 cut and its MSRV-verification prerequisite, npm
OIDC, docs logos, the upstream thread) or trigger-gated (go1.27), and the loop is at IDLE pending
the human's release cut.
