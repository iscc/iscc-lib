<!-- assessed-at: 0d9ac0f7455c9fa56272d32cc0a64dab0e3a2597 -->

# Project State

## Status: IN_PROGRESS

## Phase: All target sections met except human-held publishing items; the dependency-majors refresh is the only CID-schedulable backlog

Iteration 164 took the .NET slice of the authorized dependency refresh: `packages/dotnet` moved to
`xunit.v3` 3.x + `Microsoft.NET.Test.Sdk` 18.x. The whole iteration touched exactly two tracked
files (one `.csproj`, one `CLAUDE.md` line) with zero test-source edits. CI is green over the whole
tree.

## Rust Core Crate

**Status**: met, except the human-held v1.0.0 cut

- Nothing under `crates/`, `benches/` or `specs/rust-core.md` moved since 47a87bc: 32 Tier 1
    symbols, 342 `#[test]`, all 10 `gen_*_v0` conformant against `tests/data.json`, no `unsafe`
    outside the FFI crates.
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
    incl. aarch64. Nothing under `crates/iscc-py/` or `tests/` moved this iteration.
- 441 collected pytest tests, all inside the green `python-test` matrix run.
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
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns via `include_str!`, zero skips; runs
    under `wasm-pack test --node` in the CI `wasm` job.

## C FFI

**Status**: met — Unicode-gated at 159

- 47 `#[unsafe(no_mangle)]` externs in `crates/iscc-ffi/src/lib.rs`; untouched.
- `tests/unicode_boundary_vectors.h` (tracked, generated) is consumed by two tests —
    `crates/iscc-ffi/tests/test_iscc.c` and `packages/cpp/tests/test_iscc.cpp` — from a single
    artifact, with no vendored copy and no public-interface leak.

## Other Bindings (C#, C++, Java, Kotlin, Go, Ruby, Swift)

**Status**: met — every surface Unicode-gated

- **C# moved this iteration**: `Iscc.Lib.Tests.csproj` now references `xunit.v3` and
    `xunit.runner.visualstudio` at `3.*`, `Microsoft.NET.Test.Sdk` at `18.*`, plus
    `<OutputType>Exe</OutputType>` (v3 test projects are stand-alone executables); zero
    `Include="xunit"` v2 refs remain. Test sources, `Iscc.Lib.csproj` (`net8.0`) and the CI
    invocation are unchanged, and the suite reports the same 104 results as the pre-bump tree.
    VSTest runner mode is retained deliberately — `dotnet test -e` carries the P/Invoke library path
    (decisions.md, 2026-07-27).
- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    and `packages/{cpp,dotnet,go,kotlin,swift}` carry the full 32-symbol surface. No other binding
    source moved.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 tracked
    paths, matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`. A 13th boundary vector
    costs 12 suites at once.
- The go1.27 hazard remains its own tracked issue: go1.27 ships Unicode 17.0 tables, so
    `packages/go` (177 `func Test`) reacquires the `Final_Sigma` defect unless a freeze lands with
    the bump. The skip map stays unconditional by standing ruling; the red is the intended trigger.

## Documentation

**Status**: met

- Nothing under `docs/` moved. Page-list machinery unchanged: 23 documentation pages across
    `zensical.toml` nav, `ORDERED_PAGES` and `docs/llms.txt`; 11 `docs/howto/*.md`; 12 crate/package
    READMEs; 12 crate/package `CLAUDE.md` (`packages/dotnet/CLAUDE.md` had one descriptive line
    updated to name xunit.v3). `docs/unicode.md` names all 11 gated surfaces.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- 12 criterion benches (criterion 0.7) + `benches/iai_benches.rs` (iai-callgrind 0.16, 11 fns / 16
    cases); 18 pytest-benchmark fixtures; documented speedups 1.3x-158x.
- `.iai-baseline.json` and `.crap-baseline.json` byte-untouched (CRAP still 105 entries) — correct,
    since no Rust source moved.

## CI/CD and Publishing

**Status**: partially met — green, with only human-held publishing work left

- **CI GREEN and it covers HEAD.** `origin/develop` = `75b8810` (the 164 review commit): check-runs
    API reports **45 runs, 23 distinct names, 0 non-success**. HEAD `0d9ac0f` is one `cid(log)`
    commit ahead and `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty. Working tree
    clean.
- Job shape unchanged: 21 job keys → 22 jobs → 23 check names (`python-test` is a `[3.10, 3.14]`
    matrix; `python` is an `if: always()` aggregator). `ci.yml`, `docs.yml` and `release.yml`
    byte-untouched since 47a87bc; zero `@main` action refs; 97 `uses:` refs; 8 registry toggles;
    `workflow_dispatch`-only. The `specs/ci-cd.md` job table stays exhaustive and gated from two
    places (prek hook + `tests/test_check_ci_job_table.py::test_real_repo_passes`).
- PR **#44** `develop` → `main` ("Release 0.6.0") still OPEN — not shipped; version **0.5.0**,
    `scripts/version_sync.py` 21 targets consistent.
- Enforcing gates unchanged: iai perf (>10% Ir), coverage + CRAP (`--fail-regression` is CI-only, so
    a green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can red with no code
    change), docs page-list parity, `unicode-sweep`, CI job-table parity.
- Reproducibility gap, observed not filed: `packages/dotnet` is the only ecosystem with no lockfile
    (Cargo, uv, Gemfile, Gradle all pin) and its two test packages float on `3.*` / `18.*`, so CI
    can resolve a newer 3.x/18.x than any review saw.

## Open Issues

**7 entries in `issues.md` — 2 `normal`, 5 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
Count unchanged at 164; only the `xunit`/`Test.Sdk` bullet inside the dependency issue was struck.

- **NORMAL:** dependency review/refresh (remaining majors: Gradle wrapper 8.12.1 + JUnit 6.x, `jni`
    0.22, `magnus` 0.8, `release.yml` actions — authorized one per step); `go1.27` reds the Go
    boundary suite unless the freeze lands with it (a standing tripwire, not schedulable work).
- **LOW / CID skips:** update the upstream `iscc-core#137` thread (human-only), the three
    gate-script remainders deferred at 146 (trigger-contingent), v1.0.0 (HELD), docs language logos,
    npm OIDC (ruled out for v0.6.0).

## Next Milestone

Every target section except CI/CD is met and CI is green over the whole tree. The only
CID-schedulable item left is the next slice of the authorized dependency-majors refresh — the JVM
build tooling (Gradle wrapper, JUnit 6.x, spanning two build systems) and then the two source-level
API migrations (`jni` 0.22, `magnus` 0.8), taken one per step. Which slice comes first and how it is
sized is define-next's call. Beyond that the backlog is human-held (v1.0.0 cut, npm OIDC, docs
logos, the upstream thread).
