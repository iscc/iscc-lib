<!-- assessed-at: cd899f3ef28f670dc15b744f61947f9db40c7cc8 -->

# Project State

## Status: IN_PROGRESS

## Phase: Dependency refresh exhausted of CID-schedulable majors; backlog is now review-filed cleanup plus human-held publishing items

Iteration 168 migrated `crates/iscc-jni` from jni 0.21 to 0.22 — the last Rust dependency major the
loop could schedule. It passed with notes and the review filed three fresh `normal` issues against
the crate it just touched, so the open-issue count went 7 → 10 while the dependency issue shrank to
three human/static-evidence items. CI is green over the whole tree.

## Rust Core Crate

**Status**: met, except the human-held v1.0.0 cut

- Nothing under `crates/iscc-lib/`, `benches/` or `.claude/context/specs/` moved since 8ea3e02: 32
    Tier 1 symbols, 342 `#[test]`, all 10 `gen_*_v0` conformant, no `unsafe` outside the FFI crates.
- All four Unicode criteria remain MET: declared 16.0.0 + sentinel freeze, the `Final_Sigma` case
    freeze, boundary vectors on 11 of 11 surfaces, and the fail-closed differential sweep gate.
- Only unmet target line: `crate is >= 1.0.0`. Version is **0.5.0**; the cut is human-gated and
    HELD. `cargo-semver-checks` runs informational.
- `specs/rust-core.md` L149-157 stays STALE (claims the Go `Final_Sigma` defect unfixed, and a
    `str::to_lowercase()` equivalence falsified by measurement at 156). Human-owned, not edited.

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

**Status**: met — every surface Unicode-gated; the Java bridge moved this iteration

- **Java/JNI is the only source that changed.** Root `Cargo.toml` pins `jni = "0.22"`, `Cargo.lock`
    resolves **0.22.4**, and the `# held: jni` comment is gone — **2 inline holds left** (criterion
    0.8, uniffi 0.32). `crates/iscc-jni/src/lib.rs` (1168 lines) carries 33 `Java_*` natives with
    zero `JNIEnv` / `GlobalRef` / `AutoLocal` call sites.
- The migration hand-rolled a `build_byte_array` (1 `Vec<i8>`, 4 call sites) that duplicates the
    non-deprecated `Env::byte_array_from_slice` with an extra alloc + copy per returned `byte[]`;
    worst on `algCdcChunks`. Filed as a `normal` issue at 168.
- No Java, Kotlin or fixture source moved: Java 29 + 3 `@Test` sources (69 + 13 = 82 cases) on JUnit
    6.1.2, Kotlin 9 + 3 sources (9 + 13 cases). 7 of the 33 natives are still called by no JUnit
    test — a whole-crate signature rewrite went green on 26 of 33. Filed at 168.
- `crates/iscc-rb` (magnus 0.8.2 since 167), `crates/iscc-uniffi` (32 exports, 21 tests,
    `publish=false`), `packages/dotnet` (xunit.v3) and `packages/{cpp,go,swift}` all unchanged.
- Propagation invariant holds: `git ls-files -- '*data.json' '*unicode_boundary.json'` = 8 tracked
    paths, matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`.
- go1.27 is now its own tracked `normal` issue (was a bullet inside the dependency issue): the bump
    reds 5 Go boundary cases unless the 731-range freeze table lands in the same step. Standing
    tripwire, not schedulable until go1.27 exists (~Aug 2026).

## Documentation

**Status**: met

- Nothing under `docs/` moved (latest docs commit 2840c7f, iteration 161): 23 pages across
    `zensical.toml` nav, `ORDERED_PAGES` and `docs/llms.txt`; 11 `docs/howto/*.md`; 12 crate/package
    READMEs; 12 crate/package `CLAUDE.md`. `docs/unicode.md` names all 11 gated surfaces.
- Two doc surfaces teach APIs the code no longer uses (filed at 168 as one `normal` issue):
    `crates/iscc-rb/CLAUDE.md:108` (`RString::from_slice`) and `specs/java-bindings.md` ("jni crate
    (v0.21)", "~1060 lines" — actual 1168). The spec half carries a **HUMAN REVIEW REQUESTED**
    marker; that is the first such marker open in several iterations.
- Remaining cosmetic item is human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- 12 criterion benches (criterion 0.7) + `benches/iai_benches.rs` (iai-callgrind 0.16, 11 fns / 16
    cases); 18 pytest-benchmark fixtures; documented speedups 1.3x-158x.
- `.iai-baseline.json` and `.crap-baseline.json` byte-untouched (CRAP still 105 entries) — correct,
    since no `iscc-lib` core source moved.

## CI/CD and Publishing

**Status**: partially met — green, with only human-held publishing work left

- **CI GREEN and it covers HEAD.** `origin/develop` = `5bf5a26` (the 168 review commit carrying the
    jni bump): check-runs API reports **45 runs, 23 distinct names, 0 non-success**, so the `java`
    and `kotlin` jobs really did build and test against jni 0.22.4. HEAD `cd899f3` is one `cid(log)`
    commit ahead and `git diff --stat origin/develop..HEAD -- . ':!.claude'` is empty.
- Working tree is **not clean**: the runner's `decisions.md` → `decisions-archive.md` rotation (16
    lines) is uncommitted. Context-only, no source impact — grep BOTH files for rulings.
- Job shape unchanged: 21 job keys → 22 jobs → 23 check names (`python-test` is a `[3.10, 3.14]`
    matrix; `python` is an `if: always()` aggregator). No workflow file changed since 21bb004
    (iteration 162): zero `@main` action refs, 97 `uses:` in `release.yml`, 8 registry toggles,
    `workflow_dispatch`-only.
- PR **#44** `develop` → `main` ("Release 0.6.0") still OPEN — not shipped; version **0.5.0**,
    `scripts/version_sync.py` 21 targets consistent.
- Enforcing gates unchanged: iai perf (>10% Ir), coverage + CRAP (`--fail-regression` is CI-only, so
    a green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can red with no code
    change), docs page-list parity, `unicode-sweep`, CI job-table parity.
- Dependency freshness: **zero Rust majors left to schedule**. The issue body now lists three items
    — `uniffi` 0.32 and `criterion` 0.8 (both human-gated: binding regeneration / MSRV policy), and
    the `release.yml` action refresh, which is CID-doable but only on static evidence since that
    workflow never runs in CI.
- Reproducibility gap, observed and still unfiled by anyone: `packages/dotnet` is the only ecosystem
    with no lockfile, its two test packages floating on `3.*` / `18.*`. The 168 review deferred it
    to the audit pass due at iteration 170.

## Open Issues

**10 entries in `issues.md` — 5 `normal`, 5 `low`, zero `critical`, one HUMAN REVIEW REQUESTED
marker** (inside the doc-drift issue). Count rose from 7: the 168 review filed three, and go1.27 was
promoted out of the dependency issue into its own entry.

- **NORMAL:** dependency refresh (3 items, above); JNI `build_byte_array` regression; 7 untested JNI
    natives; binding docs teaching removed APIs; the go1.27 tripwire.
- **LOW / CID skips:** upstream `iscc-core#137` thread (human-only), the three gate-script
    remainders deferred at 146 (trigger-contingent), v1.0.0 (HELD), docs language logos, npm OIDC
    (ruled out for v0.6.0).

## Next Milestone

The dependency refresh no longer supplies work. What is left that CID can act on is the debt the 168
review left behind in `crates/iscc-jni` — a fresh allocation regression and a test surface that
proves only 26 of 33 natives — plus the Ruby doc-drift half and, as the last dependency item, the
static-evidence `release.yml` action-freshness pass. Everything beyond that is human-gated (v1.0.0
cut, npm OIDC, docs logos, the upstream thread, the `specs/java-bindings.md` edit).
