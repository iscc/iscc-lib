<!-- assessed-at: c331d4b03c96d6ae1e01b8b47fad2a4cf01befcf -->

# Project State

## Status: IN_PROGRESS

## Phase: All target sections met except the human-held publishing items; backlog is the authorized dependency refresh

Iteration 163 rewrote the CI job table in `specs/ci-cd.md` to cover all 21 `ci.yml` job keys and
added `scripts/check_ci_job_table.py` as a real parity gate (prek hook + a repo-anchored pytest).
That closes the last CID-schedulable documentation gap; the only remaining `normal` work CID may act
on is the dependency-majors refresh.

## Rust Core Crate

**Status**: met, except the human-held v1.0.0 cut

- No file under `crates/`, `benches/` or `.claude/context/specs/rust-core.md` moved since 47a87bc:
    32 Tier 1 symbols, 342 `#[test]`, all 10 `gen_*_v0` conformant against `tests/data.json`, no
    `unsafe` outside the FFI crates.
- All four Unicode criteria remain MET: declared 16.0.0 + sentinel freeze, the `Final_Sigma` case
    freeze, boundary vectors on 11 of 11 surfaces, and the fail-closed differential sweep gate.
- Only unmet target line: `crate is >= 1.0.0`. Version is **0.5.0**; the cut is human-gated and
    HELD. `cargo-semver-checks` runs informational.
- `specs/rust-core.md` L149-157 stays STALE (claims the Go `Final_Sigma` defect unfixed, and a
    `str::to_lowercase()` equivalence falsified by measurement at 156); criterion checkboxes
    unchecked though all four hold. Human-owned file, not edited.

## Python Bindings

**Status**: met

- 32 Tier 1 symbols, `IsccResult`, streaming hashers, 12 `.detach(` GIL sites, abi3-py310 wheels
    incl. aarch64. Nothing under `crates/iscc-py/` moved.
- **441 collected pytest tests** (was 432): +9 from `tests/test_check_ci_job_table.py`. All of it is
    inside the green `python-test` matrix run.
- The 17.8M-comparison Unicode sweep deliberately stays out of `pytest` / `mise run test` / pre-push
    — do not wire it in.

## Node.js Bindings

**Status**: met — Unicode-gated since 153

- `crates/iscc-napi` exports all 32 symbols with streaming classes; untouched this iteration.
- `__tests__/unicode_boundary.test.mjs`: `node:test`, metadata guard + 12 vectors, zero skips.

## WASM Bindings

**Status**: met — Unicode-gated since 151

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    dep is intentional feature-unification (do not prune). Untouched.
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns via `include_str!`, zero skips; runs
    under `wasm-pack test --node` in the CI `wasm` job.

## C FFI

**Status**: met — Unicode-gated at 159

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`; untouched.
- `tests/unicode_boundary_vectors.h` (tracked, generated, non-ASCII as 3-digit octal) is consumed by
    two tests — `crates/iscc-ffi/tests/test_iscc.c` and `packages/cpp/tests/test_iscc.cpp` — from a
    single artifact, with no vendored copy and no public-interface leak.

## Other Bindings (C++, Java, Kotlin, Go, Ruby, C#, Swift)

**Status**: met — every surface Unicode-gated

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    and `packages/{cpp,dotnet,go,kotlin,swift}` carry the full 32-symbol surface. No binding source
    moved this iteration.
- Swift closed the last surface at 161: `Tests/IsccLibTests/UnicodeBoundaryTests.swift` compares
    `unicodeScalars` arrays, never strings (Swift's string equality folds canonical equivalence,
    which would make the sequence vectors vacuous).
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = **8**
    tracked paths, exactly matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`. A 13th
    boundary vector costs 12 suites at once.
- The go1.27 hazard remains its own tracked issue: go1.27 ships Unicode 17.0 tables, so
    `packages/go` (177 `func Test`) reacquires the `Final_Sigma` defect unless a freeze lands with
    the bump. The skip map stays unconditional by standing ruling; the red is the intended trigger.

## Documentation

**Status**: met

- Nothing under `docs/` moved. Page-list machinery unchanged: **23** documentation pages across
    `zensical.toml` nav, `ORDERED_PAGES` and `docs/llms.txt`; 11 `docs/howto/*.md`; 12 crate/package
    READMEs; 12 crate/package `CLAUDE.md`. `docs/unicode.md` names all 11 gated surfaces.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- 12 criterion benches (criterion 0.7) + `benches/iai_benches.rs` (iai-callgrind 0.16, 11 fns / 16
    cases); 18 pytest-benchmark fixtures; documented speedups 1.3x-158x.
- `.iai-baseline.json` and `.crap-baseline.json` byte-untouched (CRAP still 105 entries) — correct,
    since no Rust source moved.

## CI/CD and Publishing

**Status**: partially met — green, with only human-held publishing work left

- **CI GREEN and it covers HEAD.** `origin/develop` = `95740f4` (the 163 review commit): check-runs
    API reports **45 runs, 23 distinct names, 0 non-success**. HEAD `c331d4b` is one `cid(log)`
    commit ahead and `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty. Working tree
    clean.
- **The `specs/ci-cd.md` job-table drift is fixed and gated.** The table now has exactly 21
    backticked rows, one per `ci.yml` job key, with no missing and no extra entries (verified by an
    independent PyYAML+regex comparison). `scripts/check_ci_job_table.py` (121 lines) enforces it
    from two places: the `check-ci-job-table` prek hook (scoped to `ci.yml` and `ci-cd.md`) and
    `tests/test_check_ci_job_table.py::test_real_repo_passes`, which anchors on the real tree and
    therefore rides the CI `python-test` job. The hook comment documents the known prek limitation
    (a row-only deletion via another path does not trigger `files:` matching) and names the pytest
    anchor as the backstop.
- Job shape unchanged: **21 job keys → 22 jobs → 23 check names** (`python-test` is a `[3.10, 3.14]`
    matrix; `python` is an `if: always()` aggregator). `ci.yml`, `docs.yml` and `release.yml`
    byte-untouched since 47a87bc; zero `@main` action refs; 97 `uses:` refs; 8 registry toggles;
    `workflow_dispatch`-only.
- PR **#44** `develop` → `main` ("Release 0.6.0") still OPEN — not shipped; version **0.5.0**,
    `scripts/version_sync.py` 21 targets consistent.
- Enforcing gates unchanged: iai perf (>10% Ir), coverage + CRAP (`--fail-regression` is CI-only, so
    a green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can red with no code
    change), docs page-list parity, `unicode-sweep`, and now CI job-table parity.

## Open Issues

**7 entries in `issues.md` — 2 `normal`, 5 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
The `[human]` "make the CI job table exhaustive" issue was resolved and deleted at 163.

- **NORMAL:** dependency review/refresh (major bumps, one per step); `go1.27` reds the Go boundary
    suite unless the freeze lands with it (a standing tripwire, not schedulable work).
- **LOW / CID skips:** update the upstream `iscc-core#137` thread (human-only), the three
    gate-script remainders deferred at 146 (trigger-contingent), v1.0.0 (HELD), docs language logos,
    npm OIDC (ruled out for v0.6.0).

## Next Milestone

Every target section except CI/CD is met, CI is green over the whole tree, and the documentation
drift closed at 163. The single remaining CID-schedulable item is the authorized dependency-majors
refresh — the issue lists xunit 3.x, `Microsoft.NET.Test.Sdk` 18.x, the Gradle wrapper, JUnit 6.x,
and the riskier `jni` 0.22 / `magnus` 0.8 source rewrites, and requires they be taken one per step.
Which one comes first, and how it is sized, is define-next's call. Beyond that the backlog is
human-held (v1.0.0 cut, npm OIDC, docs logos, the upstream thread).
