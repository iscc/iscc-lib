<!-- assessed-at: c30a374c075621d4273b0015bc706155c47d14cd -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — the Unicode contract is now fully gated; only fixture propagation remains

Iteration 157 landed the differential sweep as a committed, fail-closed CI gate
(`scripts/unicode_sweep.py` + `mise run unicode:sweep` + a 21st CI job), and it is **green in CI**.
That closes criterion 4 of the Rust-core Unicode contract — the last correctness criterion. What is
left on that contract is pure propagation: 3 of 11 binding surfaces still lack the boundary fixture,
and two independently-confirmed blind spots in the new gate are filed but unfixed.

## Rust Core Crate

**Status**: partially met — criteria 1, 2, **4** met; criterion 3 at 8 of 11 surfaces

- **Incremental scope**: `git diff 78bf9b6..HEAD --stat -- . ':!.claude'` = **5 files**, all
    additive (`scripts/unicode_sweep.py` +179 new, `tests/test_unicode_sweep.py` +164 new,
    `.github/workflows/ci.yml` +25, `mise.toml` +11, `docs/unicode.md` +15). **Zero** Rust source
    moved: `git diff --name-only -- crates/ .crap-baseline.json .iai-baseline.json` is empty.
    `git diff 78bf9b6..HEAD -- .claude/context/specs/` is **empty** — criteria judged against the
    same spec text as last iteration.
- 8 crates; `iscc-lib` holds **342** `#[test]` functions (unchanged). All 10 `gen_*_v0` functions
    pass the vendored `iscc-core/data.json` vectors. Version **0.5.0**. Working tree **clean**.
- **Criterion 1 (freeze rule) — MET.** `UNASSIGNED_SENTINEL: char = '\u{FFFF}'`, 731-range vendored
    table, unchanged.
- **Criterion 2 (table deps) — MET.** `unicode-general-category` 1.x / `unicode-normalization` 0.1.x
    freely upgradable; `utils/unicode16_case.rs` vendors `Cased` / `Case_Ignorable` at 16.0.0.
- **Criterion 3 (boundary vectors in every binding) — PARTIALLY met, 8 of 11 native surfaces,
    unchanged.** Rust (149), Python (150), WASM (151), Ruby (151), napi (153), Java (153), C# (154),
    Kotlin (154), plus the pure-Go port (150, 9 of 12 with 3 ruled skips). Ungated: **C FFI, C++,
    Swift**. Canonical fixture `crates/iscc-lib/tests/unicode_boundary.json` untouched (7
    `text_clean` + 5 `text_collapse`).
- **Criterion 4 (full-code-space AND sequence-class differential sweep) — MET. Verified
    independently this iteration, not inherited.**
    - The gate exists as tracked code: `scripts/unicode_sweep.py` (179 lines), the `unicode:sweep`
        mise task (rebuilds the extension with `maturin develop --release` first), and a standalone CI
        job `unicode-sweep` / check name `Unicode sweep (16.0.0 differential)`. The throwaway probe
        re-invented at 148/155/156 is gone.
    - **Spec coverage checked line by line.** The spec mandates four sequence classes; `CONTEXTS`
        carries all four — `base_mark` (`e` + c + U+0301), `jamo` (U+1100 + c + U+1161), `sigma`
        (U+0391 U+03A3 + c + U+0392), `marks` (U+0301 + c + U+0301) — plus `bare`, `ascii`, `space`,
        `upper`. Crucially every scalar is swept in every context, not just the unassigned ones, so
        the spec's "a per-code-point-only sweep does not satisfy this criterion" is honoured.
    - **Arithmetic pin re-derived, not copied:** 1,112,064 scalars (`0x110000` − 2,048 surrogates,
        computed here) × 8 contexts × 2 functions = **17,793,024** = `EXPECTED_COMPARISONS`. Both the
        scalar count and the comparison total are asserted before success is printed, so an empty run
        cannot read green.
    - **Guards probed here by direct call, not by reading the tests:** `check_oracle("15.1.0")` raises
        (fail-closed on a non-16.0.0 oracle); the local `.so` (07:28) is newer than every `.rs`, so
        the freshness guard is satisfied rather than bypassed; a spot sweep over 8 hand-picked scalars
        (U+0391, U+03A3, U+0378, U+20C1, U+A7F1, U+0295, U+1100, U+0041) → 128 comparisons, **0
        divergences**.
    - **CI proof:** the `Unicode sweep (16.0.0 differential)` check ran **twice on `9f80a32` and
        concluded `success` both times.** This is the sweep passing on a clean runner build, not a
        local artifact.
    - The oracle is the *installed* `iscc-core` 1.3.0 (pinned by `uv.lock`), and only
        `unidata_version` is version-asserted — not the `iscc-core` version. Worth revisiting if
        upstream adopts the freeze rule (`iscc-core#137`).
- **Two blind spots in the new gate — both filed, both reproduced here by direct probe:**
    1. `check_extension_fresh` watches only `*.rs` mtimes, so a `cargo update` or rustc bump leaves a
        stale `.so` reporting a false green from a bare `uv run scripts/unicode_sweep.py`. I
        confirmed the vacuous-pass variant too: passing a nonexistent source dir raises nothing
        (`max(..., default=0.0)`). Mitigated, not fixed, by both authoritative paths rebuilding
        unconditionally (`decisions.md` 2026-07-27 records the operating rule).
    2. `sweep()` retains **every** divergence though `main()` prints at most 20. Measured: a probe
        with a deliberately-wrong subject over 100 scalars produced 800 comparisons and retained
        **800** tuples — 1:1. At full scale that is up to 17.8M two-string tuples, i.e. an OOM before
        any diagnostic reaches the log.
- **Accepted residual (`decisions.md` 2026-07-27):** unconditional lowercase mappings still come
    from rustc's tables. It now **has** its guard — that is exactly what this gate measures.
- **Stale spec prose, human-owned (CID must not edit specs):** `specs/rust-core.md:149-157` still
    describes the Go `Final_Sigma` defect in the present tense (fixed at 147) and still claims Rust
    `str::to_lowercase()` matches the reference (falsified at 155, repaired in code at 156).
    Criterion-1/-3/-4 **Verified when** boxes remain unchecked despite 1, 2 and 4 being met.
- `cargo-semver-checks` stays informational (`continue-on-error: true`) until v1.0.0, which is HELD.

## Python Bindings

**Status**: met — Unicode-gated since 150; now also the *vehicle* of the criterion-4 gate

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/` moved this iteration.
- The sweep gate runs **through** the Python bindings (`iscc_lib.text_clean` vs
    `iscc_core.text_clean` on CPython 3.14), so the Python surface is the one proven equivalent to
    the reference at 17.8M- comparison scale.
- `pytest --collect-only` reports **389** tests (was 379): `tests/test_unicode_sweep.py` adds **10**
    (9 from advance + 1 review-added, which proves the eight contexts actually discriminate the
    superseded delete-filter design — the arithmetic pin alone would not catch a *swapped* context
    set). The suite exercises `sweep()` with tiny explicit scalar lists only; the full 17.8M sweep
    is deliberately **not** in `pytest`, `mise run test` or the pre-push hooks — do not wire it in.
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

**Status**: met, with the cross-cutting Unicode criterion pending — now the top propagation target

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched.
- The only propagation target buildable in this container (`gcc` present; C++ needs the absent
    `cmake`, Swift the absent `swift`). Cost is plumbing, not coverage: `tests/test_iscc.c` has no
    JSON parser and no text-function assertion, and CI compiles it as one bare `gcc` invocation, so
    the fixture must arrive as a generated C table or hand-pinned expected strings — a design call
    that has not been made.

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
    but here:** go1.27 brings Unicode 17.0 tables, so Go will acquire the `Final_Sigma` defect the
    core shed at 156 unless an equivalent case freeze lands there too — on top of the 5 boundary
    cases already predicted to red. Standing ruling (`decisions.md` 2026-07-26): the Go skip map
    stays **unconditional**; the red is the intended trigger. CI pins Go via
    `go-version-file: packages/go/go.mod` (`go 1.26.1`).
- Swift and C++ suites still contain **zero** and **3** text-function hits respectively; neither has
    a boundary fixture. Swift is the one surface that must add a *tracked* vendored copy (SwiftPM
    `resources:`) and register it in `VENDORED_COPIES`.

## Documentation

**Status**: met

- `docs/unicode.md` gained a **"Differential sweep gate"** section documenting the residual it
    guards, the exact 1,112,064 × 8 × 2 = 17,793,024 shape, the CI job name, and — after review's
    correction — an honest scope statement (the `U+A7F1` boundary vector also catches normalization
    changes, so the sweep is not the *only* place such a change would surface).
- Page-list machinery green: `scripts/check_docs_nav.py` → **23** pages consistent across nav,
    `ORDERED_PAGES` and `llms.txt` (existing page, no nav wiring needed). 12 crate/package READMEs;
    12 crate/package CLAUDE.md files; 11 `docs/howto/*.md`; `scripts/version_sync.py --check`
    **21/21** targets at 0.5.0.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint`). `benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases. 18
    pytest-benchmark fixtures in `tests/test_benchmarks.py`; documented speedups 1.3x–158x.
- `.iai-baseline.json` and `.crap-baseline.json` are both **byte-untouched** by this diff —
    expected, since no Rust source moved and the new code is a standalone script plus tests.

## CI/CD and Publishing

**Status**: partially met — green, and covering every line of working-tree code

- **Latest CI: GREEN.** check-runs API on `9f80a32` (= `origin/develop`): **45 check-runs, 23
    distinct check names, 0 non-success, 0 in progress.** HEAD is `c30a374`, exactly **one** commit
    ahead (`cid(log): iteration 157`), and `git diff --stat origin/develop..HEAD -- . ':!.claude'`
    is **empty** — the green run genuinely covers all code, including the new gate.
- **The check-name count moved: 22 → 23.** `ci.yml` now has **21 job keys → 22 jobs** (`python-test`
    is a 3.10/3.14 matrix; `python` is an `if: always()` aggregator) **→ 23 check names**, matching
    the API exactly. The new job is `unicode-sweep` / `Unicode sweep (16.0.0 differential)`: its own
    job rather than a `python-test` step, because the sweep needs a `--release` extension and
    CPython 3.14 specifically — the 3.10 leg carries pre-16.0 tables and could only skip, the
    fail-open shape the gate exists to avoid. It is unconditional (no `skipif` on the CI path).
- PR **#44** `develop` → `main` is OPEN, titled "Release 0.6.0" — **not shipped**; version is still
    **0.5.0**. (This is why runs appear 2×.)
- Iteration 157 completed cleanly: `iterations.jsonl` shows all four roles `OK` and an
    `iteration_summary` with verdict **PASS_WITH_NOTES** (235 turns, 5,575 s). Working tree clean.
- `release.yml` / `docs.yml` byte-unchanged: 97 `uses:` refs, 8 registry toggles,
    `workflow_dispatch`-only. **`specs/ci-cd.md` is now drifted** — 44 checked / 8 unchecked, but
    its job table still lists 14 rows against 21 real jobs. That drift is owned by the existing
    human-authorized "make the job table exhaustive" issue.
- Enforcing gates green: iai-callgrind perf (>10% Ir, baseline untouched), coverage + CRAP (**105**
    entries, unchanged; `--fail-regression` is **CI-only**, so a green `mise run check` proves
    nothing), `cargo-deny` (live advisory DB — can red with no code change), docs page-list parity,
    and now the Unicode sweep. `cargo-semver-checks` informational.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` + inline
    comment (still `@main` at `release.yml:895`); major dependency bumps **one per step** (xunit
    3.x, Test.Sdk 18.x, Gradle wrapper — Kotlin warns 8.12.1 vs plugin 2.4.10 — JUnit 6.x, plus the
    `jni` 0.22 / `magnus` 0.8 source rewrites, one crate per step); make the `specs/ci-cd.md` job
    table exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**9 entries in `issues.md` — 5 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
One opened this iteration (`[review]`), none closed; nothing is parked on Titusz.

- **NEW, NORMAL — "Harden the Unicode differential sweep gate" `[review]`.** The two blind spots
    described under Rust Core, both of which I reproduced here by direct probe rather than trusting
    the filing. Small, fully verifiable in this container, and it closes known holes in a gate that
    just landed. Note the coupling the issue itself flags: fixing (2) requires updating
    `test_sweep_reports_divergence_with_wrong_oracle`, which asserts `len(divergences) == count`.
- **NORMAL — "Declare and gate a Unicode data version (DECIDED)".** Remainder **(a2) is now done**;
    what is left is **(b)** propagation to C FFI, C++, Swift and the 4 sibling `data.json`
    locations.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table (now 14 rows vs 21 jobs).
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (trigger-contingent — do not
    act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Harden the sweep gate that just landed, then resume fixture propagation.**

Both candidates are legitimate; the hardening issue is the better next step and this is a judgment
call, not a formality:

1. **Harden `scripts/unicode_sweep.py`** (the new `normal` issue). It is small, fully verifiable in
    this container, and closes two holes I confirmed by direct probe — not inherited claims. The
    OOM-on-broad-regression one (retain only the first 20 samples, count separately) is a strict
    improvement with no design question attached, and it converts the *most* informative failure
    mode from unusable back into usable. The freshness guard is the judgment call: `decisions.md`
    2026-07-27 already rejected both "hash the full build-input closure" and "always shell out to
    maturin", so anything landed must preserve the fast fail-fast signal and must not pretend to a
    completeness it cannot have. Widening the mtime set to `Cargo.toml`/`Cargo.lock` is explicitly
    called out in that ruling as buying a *false* sense of completeness — so if it is done, say so
    in the comment. Also close the vacuous `default=0.0` pass, which is unambiguous.
2. **Then propagation slice 5 — C FFI**, the only one of the 3 remaining surfaces buildable here
    (`gcc` present; `cmake` and `swift` absent, so C++ and Swift are CI-proof-only). It needs a
    design call first: `tests/test_iscc.c` has no JSON reader, so either generate a C vector header
    from the canonical fixture at build time or hand-pin expected strings. Prefer generation — a
    hand-pinned copy is an untracked eighth vendored copy in spirit.

Two standing hazards for any fixture work: never write `unicode_boundary.json` through the
Write/Edit tools (the `\uXXXX` escapes decode to literal UTF-8 — use `cp`, or Python with
`ensure_ascii=True`), and do not relabel the `e U+A7F1 U+0301` row's oracle back to `e U+015A`.
Adding a *vector* to `crates/iscc-lib/tests/unicode_boundary.json` remains a separate, deliberate
slice — nine binding suites assert exactly 7 `text_clean` + 5 `text_collapse` as a metadata guard,
so a new row reds all nine at once. The propagation invariant to preserve:
`git ls-files -- '*data.json' '*unicode_boundary.json'` = **7** tracked paths (verified this
iteration), matching `VENDORED_COPIES` in `tests/test_vendored_fixtures.py`.
