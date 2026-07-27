<!-- assessed-at: 78bf9b6f13525a4be70c1c878fc76b3579bffbf9 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — the `Final_Sigma` case-table defect is fixed; criterion 4 is unblocked

Iteration 156 ran to completion (PASS) and landed real code: `text_collapse` no longer takes its
`Final_Sigma` decision from rustc's Unicode tables. **I re-derived the vendored tables from CPython
3.14 with my own code and re-ran the differential sweep from scratch this iteration — 1,112,064
scalars and 1,854 sequence cases, 0 divergences.** The defect that state.md called "the most
important fact in this file" one iteration ago is gone, and the criterion-4 sweep would now land
green — but it still does not exist as a runnable check.

## Rust Core Crate

**Status**: partially met — criteria 1, 2 met; criterion 3 at 8 of 11 surfaces; **criterion 4 still
not started, but no longer blocked**

- **Incremental scope**: `git diff 12524f9..HEAD --stat -- . ':!.claude'` = **6 files**
    (`crates/iscc-lib/src/utils.rs` +193, new `crates/iscc-lib/src/utils/unicode16_case.rs` +627,
    new `scripts/gen_unicode16_case.py` +185, `docs/unicode.md`, `crates/iscc-lib/CLAUDE.md`,
    `.crap-baseline.json`). `git diff 12524f9..HEAD -- .claude/context/specs/` is **empty** — the
    specs did not move, so criteria are judged against the same text as last iteration.
- 8 crates; `iscc-lib` now holds **342** `#[test]` functions (was 335: +7, all `Final_Sigma`/table
    tests). All 10 `gen_*_v0` functions pass the vendored `iscc-core/data.json` vectors. Version
    **0.5.0**. Working tree **clean**.
- **Criterion 1 (freeze rule) — MET.** `UNASSIGNED_SENTINEL: char = '\u{FFFF}'` in `utils.rs`,
    applied at both call sites; vendored 731-range / 819,533-code-point table untouched.
- **Criterion 2 (table deps) — MET, and its scope just widened.** `unicode-general-category` 1.x
    (16.0) and `unicode-normalization` 0.1.x (17.0) remain freely upgradable. **A third table
    dependency is now vendored rather than inherited**: `utils/unicode16_case.rs` holds `Cased` /
    `Case_Ignorable` frozen at 16.0.0.
- **Criterion 3 (boundary vectors in every binding) — PARTIALLY met, 8 of 11 native surfaces,
    unchanged.** Rust (149), Python (150), WASM (151), Ruby (151), napi (153), Java (153), C# (154),
    Kotlin (154), plus the pure-Go port (150, 9 of 12 with 3 ruled skips). Ungated: **C FFI, C++,
    Swift**. Canonical fixture `crates/iscc-lib/tests/unicode_boundary.json` untouched (7
    `text_clean` + 5 `text_collapse`) — correctly so, per the standing "do not extend the fixture in
    the fix step" constraint.
- **Criterion 4 (differential sweep) — STILL NOT STARTED as a runnable check, but it would now land
    GREEN. Verified independently this iteration, not inherited.**
    - **The fix.** `to_lowercase_unicode16` (`utils.rs:120`) pre-substitutes every `U+03A3` with the
        context-decided `U+03C2`/`U+03C3` using the vendored 16.0.0 tables, then delegates to
        `str::to_lowercase()` — which therefore never sees a capital sigma and can never fire its own
        17.0-table `Final_Sigma` branch. A `text.contains('\u{03A3}')` early return keeps non-Greek
        input on the original path. No `unsafe`.
    - **Tables re-derived from scratch, by different code than the generator.** I ran my own
        behavioural derivation over all 1,112,064 scalars on the project's CPython 3.14.6
        (`unicodedata.unidata_version == "16.0.0"`) and compared against the parsed module:
        `CASED_RANGES` **152 ranges / 4,311 code points** and `CASE_IGNORABLE_RANGES` **452 ranges /
        2,749 code points**, both **byte-identical** to my derivation, and both sorted, disjoint and
        maximally merged. These are exactly the two shape numbers the previous state.md flagged as
        "unchecked numbers … must be re-derived, not trusted." **They check out.**
    - **The defect is gone end-to-end.** Against `iscc-core` 1.3.0 on the freshly built extension
        (`_lowlevel.abi3.so` timestamped 05:44, after the 05:40 fix commit):
        `U+0391 U+03A3 U+0295 U+0392` → `U+03B1 U+03C3 U+0295 U+03B2` and `U+0295 U+03A3` →
        `U+0295 U+03C2`, both matching the reference. A standalone `rustc -O` probe confirms bare
        `str::to_lowercase()` on this toolchain **still** produces the old, divergent answers — so the
        fix is doing the work, not a toolchain change.
    - **My own sweep, run this iteration:** all **1,112,064** scalars × `text_clean` and
        `text_collapse` → **0 divergences**; **1,854** sequence cases spanning the spec's four
        mandated classes (base+Cn+mark, jamo+Cn+jamo, Σ+Cn+cased, Cn-between-marks) × both functions →
        **0 divergences**. This is a different construction from review's 17,793,024-comparison run
        and agrees with it.
    - **What is missing is only the gate.** `git ls-files | grep -i sweep` returns nothing; the
        review's own scope check confirms `scripts/unicode_sweep.py` was deliberately left
        uncommitted. The sweep remains a throwaway probe re-invented each time it is needed.
- **Accepted residual, now on the record** (`decisions.md` 2026-07-27): only the *conditional*
    `Final_Sigma` mapping is frozen. Every *unconditional* lowercase mapping still comes from
    rustc's tables. Measured at zero divergence today, but its only possible guard is the sweep —
    which is exactly why criterion 4 must land next rather than propagation.
- **Stale spec prose, human-owned (CID must not edit specs):** `specs/rust-core.md:149-157` still
    describes the Go `Final_Sigma` defect in the present tense (fixed at 147), and its claim that
    "Rust `str::to_lowercase()` does the same" as the reference is the sentence this iteration's
    predecessor falsified and this iteration's code repaired — the prose is now wrong in a second
    way, since the crate no longer calls bare `to_lowercase()` for sigma. Criterion-1 and -3
    **Verified when** boxes remain unchecked despite being met.
- `cargo-semver-checks` stays informational (`continue-on-error: true`) until v1.0.0, which is HELD.

## Python Bindings

**Status**: met — Unicode-gated since 150; host to the vendored-fixture drift gate

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/` moved this iteration.
- `tests/test_unicode_boundary.py` (12 vectors + metadata guard) and
    `tests/test_vendored_fixtures.py` (the 152 drift gate) unchanged; both collected by
    `testpaths = ["tests"]`, so they run in the `python-test` 3.10/3.14 matrix and the pre-push
    pytest hook. Review reports 379 pytest passes.
- **Now inherits the *fixed* behaviour** — the sweep above was executed through `iscc_lib.*`, so the
    Python surface is the one that has actually been proven equivalent to the reference at scale.
- Known blind spot, recorded not fixed: the drift gate discovers copies by basename
    (`{"data.json", "unicode_boundary.json"}`), so a copy vendored under a different filename
    escapes the set check. Mitigation is to keep canonical basenames when propagating.

## Node.js Bindings

**Status**: met — Unicode-gated since 153

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes; untouched.
- `__tests__/unicode_boundary.test.mjs`: `node:test`, metadata guard + 12 vectors, zero skips,
    importing the **snake_case** exports `text_clean` / `text_collapse`.
- The local `.node` addon is gitignored — the recurring "checked-in stale artifact" claim stays
    refuted. Inherits the `Final_Sigma` fix from the core.

## WASM Bindings

**Status**: met — Unicode-gated since 151

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched.
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns via `include_str!`, zero skips, ungated
    by the `conformance` feature; runs only under `wasm-pack test --node` (the CI `wasm` job).

## C FFI

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched.
- Still the cheapest remaining *propagation* target buildable in this container (`gcc` present; C++
    needs the absent `cmake`, Swift the absent `swift`) — but the cost is plumbing, not coverage:
    `tests/test_iscc.c` has no JSON parser and no text-function assertion, and CI compiles it as one
    bare `gcc` invocation, so the fixture must arrive as a generated C table or a hand-rolled
    reader.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met; Java, Ruby, C# and Kotlin Unicode-gated, C++ and Swift pending

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` carry the full 32-symbol surface. No binding source
    file moved this iteration.
- C# (154, 13 tests, fixture linked via csproj `<Content Include … Link=>`), Kotlin (154, 13 tests
    via `@TestFactory`, fixture located by `iscc.fixtureDir` **and declared as a Gradle task input**
    — without `inputs.file(...)` a fixture edit left `./gradlew test` UP-TO-DATE and silently
    skipped all 13), Java (153, gson + `@TestFactory`, resolving because surefire's CWD is the pom
    basedir), Ruby (151, `File.expand_path`, 12 vectors) — all zero-skip.
- **Go is now consistent with the core rather than ahead of it.** `packages/go` was already correct
    on `U+0295` (its `x/text` `cases.Lower` uses Unicode 15.0 tables where `U+0295` is still `Ll`);
    the Rust core has now converged on the same reference answer for a *principled* reason instead
    of an accidental one. **The go1.27 checklist item stands and is still unwritten anywhere but
    here:** go1.27 brings 17.0 tables, so Go will acquire the `Final_Sigma` defect the core just
    shed unless an equivalent case freeze lands there too — on top of the 5 boundary cases already
    predicted to red. `packages/go` sits at **177** `func Test`; its vendored
    `testdata/unicode_boundary.json` is byte-identical to the canonical file.
- Standing ruling (`decisions.md` 2026-07-26): the Go skip map stays **unconditional** — do not
    build-tag it for go1.27; the red is the intended trigger. CI pins Go via
    `go-version-file: packages/go/go.mod` (`go 1.26.1`).
- Swift and C++ suites still contain **zero** and **3** text-function hits respectively; neither has
    a boundary fixture. Swift is the one surface that must add a *tracked* vendored copy (SwiftPM
    `resources:`) and register it in `VENDORED_COPIES`.

## Documentation

**Status**: met — and the two overclaims flagged last iteration are corrected

- **`docs/unicode.md` fixed.** "Output is invariant under table upgrades" is narrowed to "severs the
    *unassigned* classification from the tables a dependency ships", and a new **"Case-property
    freeze (`Final_Sigma`)"** section documents the rustc-table dependence, the `U+0295` `Ll`→`Lo`
    reclassification, the two vendored tables with their exact shapes, and the deliberate
    `Cased ∖ Case_Ignorable` restriction. `crates/iscc-lib/CLAUDE.md` gained the matching
    crate-level rule ("Do not replace the wrapper with a bare `.to_lowercase()`").
- Page-list machinery green: `scripts/check_docs_nav.py` reports **23** pages consistent across nav,
    `ORDERED_PAGES` and `llms.txt`; 12 crate/package READMEs; 12 crate/package CLAUDE.md files; 11
    `docs/howto/*.md`; `scripts/version_sync.py --check` **21/21** targets at 0.5.0.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint`). `benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases. 18
    pytest-benchmark fixtures in `tests/test_benchmarks.py`; documented speedups 1.3x–158x.
- **The predicted iai risk did not materialise, and that is verifiable rather than asserted:**
    `.iai-baseline.json` is **untouched** by this diff (`git diff --name-only` returns nothing for
    it), so the `Final_Sigma` change cleared the enforcing >10% Ir gate with no baseline refresh —
    the `contains('Σ')` fast path worked. Review measured 16/16 within band, with
    `bench_text_code.chars_1000` at **−3.83% Ir** (an improvement).

## CI/CD and Publishing

**Status**: partially met — green, and covering every line of working-tree code

- **Latest CI: GREEN.** check-runs API on `6742dc0` (= `origin/develop`): **43 check-runs, 22
    distinct check names, 0 non-success, 0 in progress.** HEAD is `78bf9b6`, exactly **one** commit
    ahead (`cid(log): iteration 156`), and `git diff --stat origin/develop..HEAD -- . ':!.claude'`
    is **empty** — so the green run genuinely covers all code, including the `Final_Sigma` fix.
- PR **#44** `develop` → `main` is OPEN, titled "Release 0.6.0" — **not shipped**; version is still
    **0.5.0**. (This is why runs appear ~2×.)
- Iteration 156 completed cleanly: `iterations.jsonl` shows `update-state`/`define-next`/`advance`/
    `review` all `OK` and an `iteration_summary` with verdict **PASS** (223 turns, 4,912 s). The
    iteration-155 `define-next` TIMEOUT left no residue — its uncommitted `next.md` was adopted and
    committed, and the working tree is clean.
- `ci.yml` / `release.yml` / `docs.yml` byte-unchanged: 21 jobs → 22 check names, matching the API
    exactly. `release.yml` keeps 97 `uses:` refs, 8 registry toggles, `workflow_dispatch`-only.
    `specs/ci-cd.md` stands at 44 checked / 8 unchecked.
- Enforcing gates green: iai-callgrind perf (>10% Ir, baseline untouched), coverage + CRAP
    (`.crap-baseline.json` refreshed **in the same commit** as the code — now **105** entries, +5
    for `to_lowercase_unicode16`, `cased_lookahead`, `in_ranges`, `is_cased_in_unicode16`,
    `is_case_ignorable_in_unicode16`; `--fail-regression` is **CI-only**, so a green
    `mise run check` proves nothing), `cargo-deny` (live advisory DB — can red with no code change),
    docs page-list parity. `cargo-semver-checks` informational.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` + inline
    comment (still `@main` at `release.yml:895`); major dependency bumps **one per step** (xunit
    3.x, Test.Sdk 18.x, Gradle wrapper — Kotlin already warns 8.12.1 vs plugin 2.4.10 — JUnit 6.x,
    plus the `jni` 0.22 / `magnus` 0.8 source rewrites, one crate per step); make the
    `specs/ci-cd.md` job table exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**8 entries in `issues.md` — 4 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
No entry opened or closed this iteration; nothing is parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)", still the only correctness item,
    but its blocker cleared.** The entry was updated in place at 156 and now records the fix and the
    17,793,024-comparison result. Remainders: **(a2)** the criterion-4 sweep as a runnable check —
    the issue text already specifies its fail-closed requirements (assert
    `unidata_version == "16.0.0"`, assert the comparison total so a zero-case run cannot read green,
    rebuild the extension first because a stale `.so` silently measures the previous commit); and
    **(b)** propagation to C FFI, C++, Swift and the 4 sibling `data.json` locations.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (trigger-contingent — do not
    act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Land criterion 4 — the differential sweep as a permanent, fail-closed check — now that it
passes.**

The previous milestone inverted the order to fix the defect first; that fix is done and
independently re-verified, so the original order resumes. This is the highest-value remaining work
because the accepted residual recorded in `decisions.md` 2026-07-27 — *unconditional* lowercase
mappings still come from rustc's tables — has **no other possible guard**. A future rustc that
changes a non-sigma mapping would silently change hash output, and today nothing would notice.

- **Scope**: commit the sweep (the review already ran it as a throwaway; recover that shape rather
    than reinventing it) covering **both** the 1,112,064 scalars *and* the sequence classes. The
    sequence half is mandatory, not decorative: `decisions.md` 2026-07-26 records a code-point-only
    sweep scoring 0 on the superseded pre-filter while it failed 504 of 1,270 sequence cases.
- **Design its blind spots explicitly, because a sweep that fails open is worse than none**: assert
    `unicodedata.unidata_version == "16.0.0"` and fail closed if the oracle is missing; assert the
    exact comparison total so an empty run cannot read as green; distinguish "skipped" from
    "passed"; and **rebuild the Python extension as part of the check** — at 156 the committed `.so`
    was older than `src/utils.rs`, and review had to `maturin develop --release` before the sweep
    measured the right commit. That staleness hazard will silently green the gate otherwise.
- **Wire it to a named CI job and declare its input set**, so a cached or up-to-date-marked run
    cannot skip it — the exact hazard that hid 13 Kotlin tests at 154. Note which of the 22 check
    names it lands under; if it adds one, the count moves and `specs/ci-cd.md` should follow.
- **Cost note**: a 1.1M-scalar × 2-function sweep through the Python bindings runs in well under the
    560 s I allowed it, so this is affordable as a real CI job rather than a nightly.

**Then propagation (C FFI → Swift → C++)** to close criterion 3's remaining 3 of 11 surfaces. Two
standing hazards for any fixture work: never write `unicode_boundary.json` through the Write/Edit
tools (the `\uXXXX` escapes decode to literal UTF-8 — use `cp`, or Python with `ensure_ascii=True`),
and do not relabel the `e U+A7F1 U+0301` row's oracle back to `e U+015A` (that is the
category-override design's output, not a delete filter's). Adding a *vector* to
`crates/iscc-lib/tests/unicode_boundary.json` remains a separate, deliberate slice — nine binding
suites assert exactly 7 `text_clean` + 5 `text_collapse` as a metadata guard, so a new row reds all
nine at once. The propagation invariant to preserve:
`git ls-files -- '*data.json' '*unicode_boundary.json'` = **7** tracked paths, matching
`VENDORED_COPIES` in `tests/test_vendored_fixtures.py`.
