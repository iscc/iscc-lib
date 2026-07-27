<!-- assessed-at: 0b8f2e2b1ef5700afa606419ad2dd3340cc5c535 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — the Unicode contract is gated and the gate itself is now hardened; only fixture propagation remains

Iteration 158 closed both blind spots the 157 review filed in the new differential sweep gate: a
bare invocation now refuses, an empty source set fails closed, and divergence retention is bounded.
I reproduced all three fixes here by direct in-process probe rather than trusting the handoff. The
Unicode contract's only open work is propagation: 3 of 11 binding surfaces still lack the boundary
fixture.

## Rust Core Crate

**Status**: partially met — criteria 1, 2, 4 met; criterion 3 at 8 of 11 surfaces

- **Incremental scope**: `git diff c30a374..HEAD --stat -- . ':!.claude'` = **5 files**
    (`scripts/unicode_sweep.py` +92/−32, `tests/test_unicode_sweep.py` +69/−13, `docs/unicode.md`
    +10/−8, `.github/workflows/ci.yml` 1 line, `mise.toml` 1 line). **Zero Rust source moved**:
    `git diff --name-only -- crates/ .crap-baseline.json .iai-baseline.json` is empty, as is
    `git diff c30a374..HEAD -- .claude/context/specs/` — criteria judged against unchanged spec
    text.
- 8 crates; `iscc-lib` holds **342** `#[test]` functions (unchanged). All 10 `gen_*_v0` functions
    pass the vendored `iscc-core/data.json` vectors. Version **0.5.0**. Working tree **clean**.
- **Criterion 1 (freeze rule) — MET.** `UNASSIGNED_SENTINEL: char = '\u{FFFF}'` + 731-range vendored
    table, untouched.
- **Criterion 2 (table deps) — MET.** `unicode-general-category` 1.x / `unicode-normalization` 0.1.x
    freely upgradable; `utils/unicode16_case.rs` vendors `Cased` / `Case_Ignorable` at 16.0.0.
- **Criterion 3 (boundary vectors in every binding) — PARTIALLY met, 8 of 11 native surfaces,
    unchanged this iteration.** Rust (149), Python (150), WASM (151), Ruby (151), napi (153), Java
    (153), C# (154), Kotlin (154), plus the pure-Go port (150). Ungated: **C FFI, C++, Swift**.
    Canonical `crates/iscc-lib/tests/unicode_boundary.json` untouched (7 `text_clean` + 5
    `text_collapse`).
- **Criterion 4 (full-code-space AND sequence-class differential sweep) — MET, and now hardened.**
    Everything below was measured here, not inherited:
    - **Arithmetic pin re-derived in-process:** `len(list(scalar_values()))` = **1,112,064** =
        `0x110000 − 2048`; `len(CONTEXTS)` = 8, namely bare, ascii, base_mark, jamo, sigma, marks,
        space, upper — all four spec-mandated sequence classes present; × 2 functions = **17,793,024**
        = `EXPECTED_COMPARISONS`. Both counts asserted before the success line.
    - **Blind spot 1 (stale extension / vacuous pass) — CLOSED.** A bare
        `uv run scripts/unicode_sweep.py` exits **1** in ~2 s with **0 bytes on stdout**, stderr
        naming `mise run unicode:sweep`. `check_rebuilt` is an exact membership test: `[]`,
        `--rebuild`, `--rebuilt=true` and `-r` are all refused; only `--rebuilt` passes. Both
        authoritative paths supply it after an unconditional `maturin develop --release` (`mise.toml`
        `&&`-chains build → sweep; `ci.yml:455` is the step right after
        `Build Python bindings (release)`). `grep -c "default=0.0"` → **0**; `check_extension_fresh`
        with an empty list, a nonexistent dir, and an unrelated dir all raise `SystemExit`.
    - **Blind spot 2 (unbounded divergence retention / OOM) — CLOSED.** Probe with a deliberately
        wrong subject over 100 scalars: **800 comparisons, 800 divergences, exactly 20 samples
        retained**. The *reporting* path is bounded too — an in-process `main(["--rebuilt"])` over 30
        scalars printed exactly **20** `DIVERGENCE` lines, then a "showing first 20 of 240
        divergences" line, then `TOTAL 240 comparisons, 240 divergences`, return code 1.
    - `check_oracle("15.1.0")` raises. Local `.so` (09:00:09) is newer than every `.rs` (07:26), so
        the mtime guard is satisfied rather than bypassed.
    - **CI proof:** `Unicode sweep (16.0.0 differential)` concluded `success` on `5581696` (=
        `origin/develop`), i.e. the `--rebuilt` wiring is exercised for real on a clean runner.
    - The success line `TOTAL 17793024 comparisons, 0 divergences` is byte-frozen by contract.
- **Residual observed here, deliberately not filed:** `check_extension_fresh` tests the **union** of
    `RUST_SOURCE_DIRS`, so one renamed/emptied dir still passes while the other yields sources — I
    confirmed this by probe. It is redundant behind `--rebuilt` and a renamed core crate is not a
    silent event; the 158 review recorded it in memory rather than issues.md. Agreed — do not
    refile.
- **Accepted residual (`decisions.md` 2026-07-27, two entries):** unconditional lowercase mappings
    still come from rustc's tables — that is precisely what this gate measures. The `--rebuilt` flag
    is a *trusted caller assertion*, not an observation; the three rejected alternatives (widen the
    mtime set, embed a build fingerprint as a public symbol, shell out to maturin) are ruled out. Do
    not "fix" the residual by adding a fingerprint symbol — it breaks the 32-symbol Tier 1 story.
- **Stale spec prose, human-owned (CID must not edit specs):** `specs/rust-core.md:149-157` still
    describes the Go `Final_Sigma` defect in the present tense (fixed at 147) and still claims Rust
    `str::to_lowercase()` matches the reference (falsified at 155, repaired in code at 156). The
    criterion-1/-3/-4 **Verified when** boxes remain unchecked despite 1, 2 and 4 being met.
- `cargo-semver-checks` stays informational (`continue-on-error: true`) until v1.0.0, which is HELD.

## Python Bindings

**Status**: met — Unicode-gated since 150; still the vehicle of the criterion-4 gate

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/` moved this iteration.
- The sweep runs **through** the Python bindings (`iscc_lib.text_clean` vs `iscc_core.text_clean` on
    CPython 3.14), so this is the surface proven equivalent to the reference at 17.8M-comparison
    scale.
- `pytest --collect-only` → **393** tests (was 389). `tests/test_unicode_sweep.py` is now **14**
    tests (was 10): the 4 additions pin the `--rebuilt` refusal (including the mise-task name in the
    message), the empty/missing source-dir fail-closed, and the 20-sample cap alongside exact
    counts. No prior case was removed. The full 17.8M sweep stays out of `pytest`, `mise run test`
    and the pre-push hooks — **do not wire it in**.
- `tests/test_unicode_boundary.py` and `tests/test_vendored_fixtures.py` (the 152 drift gate)
    unchanged. Known blind spot, recorded not fixed: the drift gate discovers copies by basename, so
    keep canonical basenames when propagating.

## Node.js Bindings

**Status**: met — Unicode-gated since 153

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes; untouched this iteration.
- `__tests__/unicode_boundary.test.mjs`: `node:test`, metadata guard + 12 vectors, zero skips,
    importing the **snake_case** exports. The local `.node` addon is gitignored — the recurring
    "checked-in stale artifact" claim stays refuted.

## WASM Bindings

**Status**: met — Unicode-gated since 151

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched.
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns via `include_str!`, zero skips; runs
    only under `wasm-pack test --node` (the CI `wasm` job).

## C FFI

**Status**: met, with the cross-cutting Unicode criterion pending — the top propagation target

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched this iteration.
- Propagation readiness re-verified here: `include/iscc.h` declares `iscc_text_clean` (L383) and
    `iscc_text_collapse` (L434), but `tests/test_iscc.c` (**459** lines) has **zero** hits for
    either and no JSON reader, and the `c-ffi` job compiles it as one bare `gcc -I include`
    invocation plus a cbindgen header-freshness check.
- The only remaining propagation surface buildable in this container (`gcc` present; C++ needs the
    absent `cmake`, Swift the absent `swift`). The open design call is how the C test receives the
    12 vectors.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met; Java, Ruby, C# and Kotlin Unicode-gated, C++ and Swift pending

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` carry the full 32-symbol surface. No binding source
    file moved this iteration.
- C# (154, 13 tests), Kotlin (154, 13 tests, fixture registered as a Gradle task **input** — without
    it a fixture edit left `./gradlew test` UP-TO-DATE and silently skipped all 13), Java (153,
    surefire CWD = pom basedir), Ruby (151, 12 vectors) — all zero-skip.
- `packages/go` sits at **177** `func Test`; its vendored `testdata/unicode_boundary.json` is
    byte-identical to the canonical file. **The go1.27 checklist item stands and is recorded nowhere
    but here:** go1.27 brings Unicode 17.0 tables, so Go will reacquire the `Final_Sigma` defect the
    core shed at 156 unless an equivalent case freeze lands there too — on top of the 5 boundary
    cases already predicted to red. Standing ruling (`decisions.md` 2026-07-26): the Go skip map
    stays **unconditional**; the red is the intended trigger. CI pins Go via
    `go-version-file: packages/go/go.mod` (`go 1.26.1`).
- Swift and C++ suites still contain **zero** and **3** text-function hits respectively; neither has
    a boundary fixture. Swift is the one surface that must add a *tracked* vendored copy (SwiftPM
    `resources:`) and register it in `VENDORED_COPIES`.

## Documentation

**Status**: met

- `docs/unicode.md` "Differential sweep gate" section updated for the refusal: it now states the
    script refuses a bare invocation and directs readers to `mise run unicode:sweep` (or the CI
    job), replacing the review-corrected "is the only way to run it" absolute — accurate, since
    `uv run scripts/unicode_sweep.py --rebuilt` also runs it.
- Page-list machinery green: `scripts/check_docs_nav.py` → **23** pages consistent across nav,
    `ORDERED_PAGES` and `llms.txt`. 12 crate/package READMEs; 12 crate/package CLAUDE.md files; 11
    `docs/howto/*.md`; `scripts/version_sync.py --check` **21/21** targets at 0.5.0.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint`). `benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases. 18
    pytest-benchmark fixtures in `tests/test_benchmarks.py`; documented speedups 1.3x–158x.
- `.iai-baseline.json` and `.crap-baseline.json` are **byte-untouched** (CRAP still **105** entries)
    — correct, since no Rust source moved and the diff is a standalone script plus its tests.

## CI/CD and Publishing

**Status**: partially met — green, and covering every line of working-tree code

- **Latest CI: GREEN.** check-runs API on `5581696` (= `origin/develop`): **45 check-runs, 23
    distinct check names, 0 non-success, 0 in progress.** HEAD is `0b8f2e2`, exactly **one** commit
    ahead (`cid(log): iteration 158`), and `git diff --stat origin/develop..HEAD -- . ':!.claude'`
    is **empty** — the green run genuinely covers all code, including the hardened gate.
- Job shape re-derived from YAML: **21 job keys → 22 jobs** (`python-test` is a `[3.10, 3.14]`
    matrix; `python` is an `if: always()` aggregator) **→ 23 check names**, matching the API
    exactly. All counts unchanged from 157; only the `unicode-sweep` job's run line gained
    `--rebuilt`.
- PR **#44** `develop` → `main` is OPEN, titled "Release 0.6.0" — **not shipped**; version is still
    **0.5.0**. (This is why runs appear 2×.)
- Iteration 158 completed cleanly: `iterations.jsonl` shows all four roles `OK` and an
    `iteration_summary` with verdict **PASS** (193 turns, 2,764 s). Working tree clean.
- `release.yml` / `docs.yml` byte-unchanged: 97 `uses:` refs, 8 registry toggles,
    `workflow_dispatch`-only; `rubygems/configure-rubygems-credentials@main` still unpinned at
    `release.yml:895`. **`specs/ci-cd.md` remains drifted** — 44 checked / 8 unchecked, job table
    still 14 rows against 21 real job keys. Owned by the existing human-authorized issue.
- Enforcing gates green: iai-callgrind perf (>10% Ir, baseline untouched), coverage + CRAP (105
    entries; `--fail-regression` is **CI-only**, so a green `mise run check` proves nothing),
    `cargo-deny` (live advisory DB — can red with no code change), docs page-list parity, and the
    Unicode sweep. `cargo-semver-checks` informational.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` + inline
    comment; major dependency bumps **one per step** (xunit 3.x, Test.Sdk 18.x, Gradle wrapper —
    Kotlin warns 8.12.1 vs plugin 2.4.10 — JUnit 6.x, plus the `jni` 0.22 / `magnus` 0.8 source
    rewrites, one crate per step); make the `specs/ci-cd.md` job table exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**8 entries in `issues.md` — 4 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
One closed this iteration ("Harden the Unicode differential sweep gate"), none opened. Nothing is
parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)".** (a1) and (a2) are both ✅; the
    entry's remainder is **(b)** propagation to C FFI, C++, Swift and the sibling `data.json`
    locations. The (a2) body now documents the 158 hardening.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (trigger-contingent — do not
    act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Propagation slice 5 — the C FFI boundary vectors.** With the sweep gate hardened and its issue
closed, this is the last autonomously completable surface of criterion 3: C++ has no `cmake` and
Swift no toolchain in this container, so both are CI-proof-only. Readiness verified this iteration —
`iscc_text_clean` / `iscc_text_collapse` are exported and declared in `include/iscc.h`, and
`tests/test_iscc.c` runs in the `c-ffi` job; it simply has no JSON reader and no text-function
coverage.

The design call is how the C test receives the 12 vectors. Recommended (the 158 review's
recommendation, and it matches an established in-repo pattern): a checked-in PEP 723 generator that
emits a `unicode_boundary_vectors.h` of `static const char *` UTF-8 literals from the canonical
`crates/iscc-lib/tests/unicode_boundary.json`, gated by "re-run the generator, then
`git status --porcelain <header>` must be empty" — the same shape as `scripts/gen_unicode16_*.py`.
Prefer it over hand-rolling a JSON parser in C (more code, more risk) and over hand-pinned strings
(drifts silently). **A generated header is derived, not byte-identical, so it must NOT be added to
`VENDORED_COPIES`** in `tests/test_vendored_fixtures.py` — the regeneration-no-op check is its
equivalent, and the work package should say so explicitly.

Lighter human-authorized alternatives if a smaller step is wanted: pin
`rubygems/configure-rubygems-credentials` to `@v2.1.0` + `# exact tag:` comment (one line, still
`@main` at `release.yml:895`); or make the `specs/ci-cd.md` job table exhaustive (14 rows vs 21 job
keys).

Two standing hazards for any fixture work: never write `unicode_boundary.json` through the
Write/Edit tools (the `\uXXXX` escapes decode to literal UTF-8 — use `cp`, or Python with
`ensure_ascii=True`), and do not relabel the `e U+A7F1 U+0301` row's oracle back to `e U+015A`.
Adding a *vector* to the canonical fixture remains a separate, deliberate slice — nine binding
suites assert exactly 7 `text_clean` + 5 `text_collapse` as a metadata guard, so a new row reds all
nine at once. Propagation invariant re-verified this iteration:
`git ls-files -- '*data.json' '*unicode_boundary.json'` = **7** tracked paths, matching
`VENDORED_COPIES`.
