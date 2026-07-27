<!-- assessed-at: 1c18b68219d33287eb6e6485a415b74188ca348e -->

# Project State

## Status: IN_PROGRESS

## Phase: All target sections met except human-held publishing items; two Rust dependency majors are the last CID-schedulable work

Iteration 166 took the JVM test-framework slice of the authorized dependency refresh: JUnit 5.14.4 →
6.1.2 in both build systems (Gradle and Maven), with the platform launcher renumbered 1.14.4 →
6.1.2. Tracked changes touch five files (two manifests, three doc/javadoc comments); no test logic
moved. CI is green over the whole tree.

## Rust Core Crate

**Status**: met, except the human-held v1.0.0 cut

- Nothing under `crates/iscc-lib/`, `benches/` or `specs/rust-core.md` moved since 0d9ac0f: 32 Tier
    1 symbols, 342 `#[test]`, all 10 `gen_*_v0` conformant against `tests/data.json`, no `unsafe`
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

## Other Bindings (Java, Kotlin, C#, C++, Go, Ruby, Swift)

**Status**: met — every surface Unicode-gated

- **The two JVM surfaces moved this iteration.** `crates/iscc-jni/java/pom.xml` pins `junit-jupiter`
    6.1.2; `packages/kotlin/build.gradle.kts` pins `junit-jupiter` **and** `junit-platform-launcher`
    at 6.1.2 (JUnit 6 unifies Platform and Jupiter numbering), and the stale
    `// held: JUnit 6.x deferred` comment is gone. A repo-wide sweep of tracked non-`.claude` files
    finds zero `5.14.4`, `1.14.4` or `JUnit 5` strings.
- Test sources unchanged in count: Kotlin 9 `@Test` in `ConformanceTest.kt` + 3 in
    `UnicodeBoundaryTest.kt` (→ 9 + 13 cases); Java 29 annotations in `IsccLibTest.java` + 3 in
    `UnicodeBoundaryTest.java` (→ 69 + 13 = 82 cases). Only a javadoc line changed in
    `IsccLibTest.java`.
- `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`), `packages/dotnet`
    (xunit.v3 3.x since 164) and `packages/{cpp,go,swift}` carry the full 32-symbol surface; no
    other binding source moved.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 tracked
    paths, matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`. A 13th boundary vector
    costs 12 suites at once.
- The go1.27 hazard remains its own tracked issue: go1.27 ships Unicode 17.0 tables, so
    `packages/go` (177 `func Test`) reacquires the `Final_Sigma` defect unless a freeze lands with
    the bump. The skip map stays unconditional by standing ruling; the red is the intended trigger.
- Cosmetic drift, human-owned, not filed: `specs/java-bindings.md:42` still says "JUnit 5
    conformance tests" in a file-tree comment.

## Documentation

**Status**: met

- Nothing under `docs/` moved. Page-list machinery unchanged: 23 documentation pages across
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

- **CI GREEN and it covers HEAD.** `origin/develop` = `6d78f8e` (the 166 review commit carrying the
    JUnit bump): check-runs API reports **45 runs, 23 distinct names, 0 non-success** — so both the
    `java` and `kotlin` jobs really did resolve and run JUnit 6.1.2. HEAD `1c18b68` is one
    `cid(log)` commit ahead and `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty.
    Working tree clean; all four roles of 166 logged OK.
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
- Dependency freshness: the issue body now lists **only** `jni` 0.22 and `magnus` 0.8 as remaining
    majors — nine ecosystem slices plus the JVM build-tooling and test-framework steps are closed.
- Reproducibility gap, observed not filed: `packages/dotnet` is the only ecosystem with no lockfile
    (Cargo, uv, Gemfile, Gradle all pin) and its two test packages float on `3.*` / `18.*`.

## Open Issues

**7 entries in `issues.md` — 2 `normal`, 5 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
Count unchanged at 166; only the JUnit bullet inside the dependency issue was struck.

- **NORMAL:** dependency review/refresh (remaining majors: `jni` 0.22 in
    `crates/iscc-jni/src/lib.rs` and `magnus` 0.8 in `crates/iscc-rb/src/lib.rs` — both real
    source-level API migrations, authorized one per step); `go1.27` reds the Go boundary suite
    unless the freeze lands with it (a standing tripwire, not schedulable work).
- **LOW / CID skips:** update the upstream `iscc-core#137` thread (human-only), the three
    gate-script remainders deferred at 146 (trigger-contingent), v1.0.0 (HELD), docs language logos,
    npm OIDC (ruled out for v0.6.0).

## Next Milestone

Every target section except CI/CD is met and CI is green over the whole tree. The only
CID-schedulable work left is the tail of the authorized dependency-majors refresh: the two
source-level Rust API migrations, `jni` 0.22 and `magnus` 0.8, taken one per step. Which one goes
first and how it is sliced is define-next's call. Beyond that the backlog is human-held (v1.0.0 cut,
npm OIDC, docs logos, the upstream thread).
