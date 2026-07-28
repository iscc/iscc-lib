<!-- assessed-at: 8ea3e025dd25d9cb17851e253085f33ec6df4882 -->

# Project State

## Status: IN_PROGRESS

## Phase: All target sections met except human-held publishing items; one Rust dependency major is the last CID-schedulable work

Iteration 167 took the Ruby slice of the authorized dependency refresh: workspace `magnus 0.7 → 0.8`
with both deprecated `old-api` families migrated to their `Ruby`-handle equivalents. Tracked changes
touch four files (`Cargo.toml`, `Cargo.lock`, `crates/iscc-rb/src/lib.rs`,
`crates/iscc-rb/CLAUDE.md`) and no test, fixture or Gemfile moved. CI is green over the whole tree.

## Rust Core Crate

**Status**: met, except the human-held v1.0.0 cut

- Nothing under `crates/iscc-lib/`, `benches/` or `specs/` moved since 1c18b68: 32 Tier 1 symbols,
    342 `#[test]`, all 10 `gen_*_v0` conformant against `tests/data.json`, no `unsafe` outside the
    FFI crates.
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

- **The Ruby surface moved this iteration.** Root `Cargo.toml` pins
    `magnus = { version = "0.8", features = ["rb-sys"] }`, `Cargo.lock` resolves magnus **0.8.2**,
    and the `# held: magnus` comment is gone (3 inline holds left: criterion 0.8, jni 0.22, uniffi
    0.32). `crates/iscc-rb/src/lib.rs` has zero `magnus::exception::` and zero `RString::from_slice`
    call sites; 33 `define_*` registrations still cover the full 32-symbol surface. Test files,
    `Gemfile*` and fixtures are byte-untouched.
- Doc drift, observed not filed: `crates/iscc-rb/CLAUDE.md:108` still teaches `RString::from_slice`
    as the pattern for copying slices into Ruby strings, while the code now uses
    `ruby.str_from_slice`. The architecture line in the same file was updated to "Magnus 0.8".
- No other binding source moved: Kotlin 9 + 3 `@Test` sources (→ 9 + 13 cases), Java 29 + 3 (→ 69 +
    13 = 82 cases) on JUnit 6.1.2 since 166; `crates/iscc-jni`, `crates/iscc-uniffi` (32 exports, 21
    tests, `publish=false`), `packages/dotnet` (xunit.v3 3.x since 164) and
    `packages/{cpp,go,swift}` all unchanged.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 tracked
    paths, matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`. A 13th boundary vector
    costs 12 suites at once.
- The go1.27 hazard remains its own tracked issue: go1.27 ships Unicode 17.0 tables, so
    `packages/go` (177 `func Test`) reacquires the `Final_Sigma` defect unless a freeze lands with
    the bump. The skip map stays unconditional by standing ruling; the red is the intended trigger.

## Documentation

**Status**: met

- Nothing under `docs/` moved (latest docs commit is 2840c7f, iteration 161). Page-list machinery
    unchanged: 23 documentation pages across `zensical.toml` nav, `ORDERED_PAGES` and
    `docs/llms.txt`; 11 `docs/howto/*.md`; 12 crate/package READMEs; 12 crate/package `CLAUDE.md`.
    `docs/unicode.md` names all 11 gated surfaces.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- 12 criterion benches (criterion 0.7) + `benches/iai_benches.rs` (iai-callgrind 0.16, 11 fns / 16
    cases); 18 pytest-benchmark fixtures; documented speedups 1.3x-158x.
- `.iai-baseline.json` and `.crap-baseline.json` byte-untouched (CRAP still 105 entries) — correct,
    since no `iscc-lib` core source moved.

## CI/CD and Publishing

**Status**: partially met — green, with only human-held publishing work left

- **CI GREEN and it covers HEAD.** `origin/develop` = `90bde13` (the 167 review commit carrying the
    magnus bump): check-runs API reports **45 runs, 23 distinct names, 0 non-success** — so the
    `ruby` job really did compile and test against magnus 0.8.2. HEAD `8ea3e02` is one `cid(log)`
    commit ahead and `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty. Working tree
    clean; all four roles of 167 logged OK.
- Job shape unchanged: 21 job keys → 22 jobs → 23 check names (`python-test` is a `[3.10, 3.14]`
    matrix; `python` is an `if: always()` aggregator). No workflow file changed this iteration (last
    workflow commit 21bb004, iteration 162): zero `@main` action refs; 97 `uses:` refs in
    `release.yml`; 8 registry toggles; `workflow_dispatch`-only. The `specs/ci-cd.md` job table
    stays exhaustive and gated from two places (prek hook + `test_check_ci_job_table.py`).
- PR **#44** `develop` → `main` ("Release 0.6.0") still OPEN — not shipped; version **0.5.0**,
    `scripts/version_sync.py` 21 targets consistent.
- Enforcing gates unchanged: iai perf (>10% Ir), coverage + CRAP (`--fail-regression` is CI-only, so
    a green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can red with no code
    change), docs page-list parity, `unicode-sweep`, CI job-table parity.
- Dependency freshness: the issue body now lists **`jni` 0.22 as the single remaining
    CID-schedulable major** — ten ecosystem slices are closed. Everything else in the issue is
    human/release-gated (release.yml action bumps, uniffi 0.32, criterion 0.8).
- Reproducibility gap, observed not filed: `packages/dotnet` is the only ecosystem with no lockfile
    (Cargo, uv, Gemfile, Gradle all pin) and its two test packages float on `3.*` / `18.*`.

## Open Issues

**7 entries in `issues.md` — 2 `normal`, 5 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
Count unchanged at 167; only the magnus bullet inside the dependency issue was struck.

- **NORMAL:** dependency review/refresh (one schedulable item left: `jni` 0.22 in
    `crates/iscc-jni/src/lib.rs` — `JNIEnv` → `EnvUnowned`/`Env`, `GlobalRef` → `Global`,
    `AutoLocal` → `Auto`, closure-based attachment and a mandatory per-function `ErrorPolicy` across
    ~41 sites); `go1.27` reds the Go boundary suite unless the freeze lands with it (a standing
    tripwire, not schedulable work).
- **LOW / CID skips:** update the upstream `iscc-core#137` thread (human-only), the three
    gate-script remainders deferred at 146 (trigger-contingent), v1.0.0 (HELD), docs language logos,
    npm OIDC (ruled out for v0.6.0).

## Next Milestone

Every target section except CI/CD is met and CI is green over the whole tree. The only
CID-schedulable work left is the last authorized dependency major: the `jni` 0.22 migration of
`crates/iscc-jni/src/lib.rs`, which unlike the nine mechanical slices before it is a real
source-level API rework touching the Java and Kotlin surfaces. Whether it lands as one package or
needs slicing is define-next's call. Beyond that the backlog is human-held (v1.0.0 cut, npm OIDC,
docs logos, the upstream thread).
