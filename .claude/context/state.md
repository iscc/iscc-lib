<!-- assessed-at: 12524f926316cd8d085fc6510300f08451a234ed -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — a rustc-table-dependent `Final_Sigma` divergence found while scoping criterion 4

Iteration 155 produced **no code**: `update-state` ran, then `define-next` hit the runner's 1200 s
wall (`"status":"TIMEOUT"`, 0 turns) and `advance`/`review` never ran. But the timed-out agent left
an uncommitted `next.md` claiming a new conformance defect — and **I reproduced it independently
this iteration**: `text_collapse`'s `Final_Sigma` decision is taken from *rustc's* Unicode tables
(17.0), not the declared 16.0.0, so the Rust core disagrees with `iscc-core` on `U+0295`. Criterion
4 is therefore not merely "not started" — a sweep run today lands **red**.

## Rust Core Crate

**Status**: partially met — criteria 1 and 2 met; criterion 3 at 8 of 11 surfaces; **criterion 4 is
blocked by a newly confirmed defect**

- **Incremental scope: the code diff is empty.** `git diff 0b6b3be..HEAD --stat` = 4 files, **all
    under `.claude/`** (`state.md`, two memory files, `iterations.jsonl`).
    `git diff 0b6b3be..HEAD --stat -- . ':!.claude'` is empty, as is
    `git diff 0b6b3be..HEAD -- .claude/context/specs/`. Every section below that depends on source
    files is carried forward from the iteration-155 assessment; the one section that changed is this
    one, and it changed because of **measurement**, not because of a commit.
- 8 crates; `iscc-lib` holds **335** `#[test]` functions; all 10 `gen_*_v0` functions pass the
    vendored `iscc-core/data.json` vectors. Version **0.5.0**. Working tree carries one modified
    file: `.claude/context/next.md` (see CI/CD section).
- **Criterion 1 (freeze rule) — MET.** `UNASSIGNED_SENTINEL: char = '\u{FFFF}'` at
    `crates/iscc-lib/src/utils.rs:35`, applied at both call sites (`:122`, `:213`); vendored
    731-range / 819,533-code-point table untouched.
- **Criterion 2 (table deps) — MET.** `unicode-general-category` 1.x (16.0) and
    `unicode-normalization` 0.1.x (17.0), both freely upgradable under the freeze rule.
- **Criterion 3 (boundary vectors in every binding) — PARTIALLY met, 8 of 11 native surfaces**,
    unchanged: Rust (149), Python (150), WASM (151), Ruby (151), napi (153), Java (153), C# (154),
    Kotlin (154), plus the pure-Go port (150, 9 of 12 with 3 ruled skips). Ungated: **C FFI, C++,
    Swift**, plus the 4 sibling `data.json` locations. Canonical fixture
    `crates/iscc-lib/tests/unicode_boundary.json` unchanged (7 `text_clean` + 5 `text_collapse`).
    Drift gate (`tests/test_vendored_fixtures.py`, 5 registered copies vs 7 matching tracked paths)
    untouched and consistent.
- **Criterion 4 (differential sweep) — NOT started, AND it would fail today. NEW FINDING, reproduced
    from scratch this iteration; this is the most important fact in this file.**
    - **The defect.** `text_collapse` lowercases with `str::to_lowercase()`
        (`crates/iscc-lib/src/utils.rs:220`). Rust and CPython implement the *same* `Final_Sigma`
        algorithm but read *different tables*: `rustc 1.97.1` ships Unicode **17.0**, the declared
        version is **16.0.0** (= CPython 3.14.6, `unicodedata.unidata_version == "16.0.0"`).
        `U+0295 LATIN LETTER PHARYNGEAL VOICED FRICATIVE` is `Ll` (hence `Cased`) in 16.0 and `Lo`
        (not `Cased`) in 17.0.

        | input                         | `iscc-core` 1.3.0 / CPython 3.14 | Rust core today               | pure-Go port                  |
        | ----------------------------- | -------------------------------- | ----------------------------- | ----------------------------- |
        | `U+0391 U+03A3 U+0295 U+0392` | `U+03B1 U+03C3 U+0295 U+03B2`    | `U+03B1 U+03C2 U+0295 U+03B2` | `U+03B1 U+03C3 U+0295 U+03B2` |
        | `U+0295 U+03A3`               | `U+0295 U+03C2`                  | `U+0295 U+03C3`               | `U+0295 U+03C2`               |

        Measured directly: a standalone `rustc -O` probe for the Rust column, `uv run python` on the
        project venv for the reference column, and a `/tmp` Go module with a `replace` to
        `packages/go` for the Go column. **Go matches the reference; the Rust core is the outlier** —
        the same shape as the original Unicode issue, and every one of the 10 Rust-backed surfaces
        inherits it.

    - **The blast radius is exactly one assigned scalar, and I measured that rather than assuming
        it.** I dumped the two `Final_Sigma` predicates (`Cased`, `Case_Ignorable`) for all
        **1,112,064** scalars from both runtimes using the behavioural probes `(ch+"Σ")` and
        `("A"+ch+"Σ")`, and diffed: **100** scalars classify differently. **99 of them are `Cn` in
        Unicode 16** (`U+1ACF…`, new 17.0 combining marks) and are therefore replaced by the sentinel
        *before* lowercasing, where both runtimes agree (`U+FFFF` is neither `Cased` nor
        `Case_Ignorable` in either table — verified). **Exactly 1 assigned scalar, `U+0295`, survives
        to produce divergent output.** This independently corroborates the uncommitted `next.md`'s
        "exactly one divergent scalar" claim by a different method, and it is also positive evidence
        that the sentinel map + its map-then-lowercase ordering are doing their job for the other 99.

    - **Nothing gates it.** Grep across `crates/iscc-lib/src`, `tests/` and `docs/` finds no `U+0295`
        anywhere; the only sigma coverage is `test_sentinel_preserves_final_sigma_context` and the
        `U+0391 U+03A3 U+0378 U+0392` fixture row, both of which pass either way. The divergence is
        invisible to every existing gate — vectors, CRAP, iai, clippy, all 22 CI checks.

    - **Severity, stated honestly.** Bounded in reach (one IPA/phonetic code point adjacent to a
        capital sigma in `text_collapse` → Text-Code and the `text_collapse` API), but it is precisely
        the failure *class* the freeze rule exists to abolish: **the core's hash output is a function
        of the compiler version.** A future rustc reclassification widens the set with no source
        change and no gate firing.

    - `specs/rust-core.md:103-118` (criterion 4) requires the divergence set to be **empty** — "any
        nonzero result is a defect, not a residual to accept" — and mandates sequence classes as well
        as the 1,112,064 scalars. So the sweep gate cannot land green until this is fixed.
- **Stale/now-wrong spec prose, human-owned (CID must not edit specs):**
    `specs/rust-core.md:149-157` still describes the Go `Final_Sigma` defect in the present tense
    (fixed at iteration 147 — I re-verified Go returns `ς` correctly), **and its claim that "Rust
    `str::to_lowercase()` does the same" as the reference is now measurably incomplete** — same
    algorithm, different tables. The criterion-1 and criterion-3 **Verified when** boxes remain
    unchecked despite being met / partially met. Spec checkboxes are not a progress signal in this
    repo.
- `cargo-semver-checks` stays informational (`continue-on-error: true`) until v1.0.0, which is HELD.

## Python Bindings

**Status**: met — Unicode-gated since 150; host to the vendored-fixture drift gate

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/` has moved.
- `tests/test_unicode_boundary.py` (12 vectors + metadata guard) and
    `tests/test_vendored_fixtures.py` (the 152 drift gate) both unchanged and both collected by
    `testpaths = ["tests"]`, so they run in the `python-test` 3.10/3.14 matrix and the pre-push
    pytest hook.
- Known blind spot, recorded not fixed: the drift gate discovers copies by basename
    (`{"data.json", "unicode_boundary.json"}`), so a copy vendored under a different filename
    escapes the set check. Mitigation is to keep canonical basenames when propagating.
- Inherits the `U+0295` divergence from the core (`iscc_lib.text_collapse` is a thin wrapper).

## Node.js Bindings

**Status**: met — Unicode-gated since 153

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes; untouched.
- `__tests__/unicode_boundary.test.mjs`: `node:test`, metadata guard + 12 vectors, zero skips,
    importing the **snake_case** exports `text_clean` / `text_collapse`.
- The local `.node` addon is gitignored — the recurring "checked-in stale artifact" claim stays
    refuted. Inherits the `U+0295` divergence.

## WASM Bindings

**Status**: met — Unicode-gated since 151

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched.
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns via `include_str!`, zero skips, ungated
    by the `conformance` feature; runs only under `wasm-pack test --node` (the CI `wasm` job).
    Inherits the `U+0295` divergence.

## C FFI

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched.
- Still the cheapest remaining *propagation* target that is buildable in this container (`gcc`
    present; C++ needs the absent `cmake`, Swift the absent `swift`) — but the cost is plumbing, not
    coverage: `tests/test_iscc.c` has no JSON parser and no text-function assertion, and CI compiles
    it as one bare `gcc` invocation, so the fixture must arrive as a generated C table or a
    hand-rolled reader.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met; Java, Ruby, C# and Kotlin Unicode-gated, C++ and Swift pending

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` carry the full 32-symbol surface. No binding source
    file has moved since the last assessment.
- C# (154, `UnicodeBoundaryTests.cs`, 13 tests, fixture linked via csproj
    `<Content Include … Link=>`), Kotlin (154, `UnicodeBoundaryTest.kt`, 13 tests via
    `@TestFactory`, fixture located by the `iscc.fixtureDir` system property **and declared as a
    Gradle task input** — without `inputs.file(...)` a fixture edit left `./gradlew test`
    `UP-TO-DATE` and silently skipped all 13 tests), Java (153, gson + `@TestFactory`, resolving
    because surefire's CWD is the pom basedir), Ruby (151, `File.expand_path`, 12 vectors) — all
    zero-skip.
- **Go is the one surface that is currently *correct* on `U+0295`** — I probed `iscc.TextCollapse`
    directly and it returns the reference values for both sigma cases, because `x/text`'s
    `cases.Lower(language.Und)` is on Unicode 15.0 tables where `U+0295` is still `Ll`. **This adds
    a line to the go1.27 checklist that is not yet written down anywhere:** go1.27 brings 17.0
    tables, so Go will acquire the *same* `Final_Sigma` defect at that bump unless the case freeze
    lands there too — on top of the 5 boundary cases already predicted to red.
- Standing ruling (`decisions.md` 2026-07-26): the Go skip map stays **unconditional** — do not
    build-tag it for go1.27; the red is the intended trigger. CI pins Go via
    `go-version-file: packages/go/go.mod` (`go 1.26.1`). `packages/go` sits at **177** `func Test`
    and its vendored `testdata/unicode_boundary.json` is byte-identical to the canonical file.
- Swift and C++ suites still contain **zero** and **3** text-function hits respectively; neither has
    a boundary fixture. Swift is the one surface that must add a *tracked* vendored copy (SwiftPM
    `resources:`) and must register it in `VENDORED_COPIES`.

## Documentation

**Status**: met — but one published claim is now measurably too strong

- Page-list machinery green and unchanged: **23** pages consistent across nav, `ORDERED_PAGES` and
    `llms.txt`; 12 crate/package READMEs; 12 crate/package CLAUDE.md files; 11 `docs/howto/*.md`;
    `scripts/version_sync.py` **21/21** targets. No docs file moved this iteration.
- **`docs/unicode.md` overclaims invariance.** It states "**Output is invariant under table
    upgrades** … Mapping into it severs all dependence on" the table version, and that
    "`Final_Sigma` context match `iscc-core` exactly — including on multi-code-point sequences".
    Both are true for *unassigned* code points (the sentinel's job) but false for *assigned* code
    points that a later Unicode version **reclassifies** — `U+0295` is exactly that case, and
    `text_collapse` still reads `Cased`/`Case_Ignorable` from rustc's tables. The same crate-level
    note in `crates/iscc-lib/CLAUDE.md` ("preserving composition-blocking and `Final_Sigma`
    context") carries the same gap. Correct these in the same step as the fix, not separately.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint`). `benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases. 18
    pytest-benchmark fixtures in `tests/test_benchmarks.py`; documented speedups 1.3x–158x.
    Untouched.
- Relevant to the next step: `text_collapse` **is** benched, so a `Final_Sigma` fix will be measured
    by the enforcing >10% Ir iai gate. A `contains('Σ')` fast path is the obvious way to keep
    non-Greek inputs in the noise band.

## CI/CD and Publishing

**Status**: partially met — green, and covering every line of working-tree code

- **Latest CI: GREEN.** check-runs API on `ceb32fd` (= `origin/develop`, unmoved): **43 check-runs,
    22 distinct check names, 0 non-success, 0 in progress.** HEAD is `12524f9`, three commits ahead
    (`0b6b3be`, `baced83`, `12524f9` — all `.claude/`-only), and
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **empty**, so the green run genuinely
    covers all code. (~2× runs because PR **#44** `develop` → `main` is OPEN, titled "Release 0.6.0"
    — not shipped; version is still **0.5.0**.) Note this greenness is not evidence about `U+0295`:
    no gate exists for it.
- `ci.yml` / `release.yml` / `docs.yml` byte-unchanged: 21 jobs → 22 check names, matching the API
    exactly. `release.yml` keeps 97 `uses:` refs, 8 registry toggles, `workflow_dispatch`-only.
    `specs/ci-cd.md` stands at 44 checked / 8 unchecked.
- Enforcing gates green: iai-callgrind perf (>10% Ir), coverage + CRAP (`--fail-regression` is
    **CI-only**, so a green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can
    red with no code change), docs page-list parity. `cargo-semver-checks` informational.
- **Iteration 155 did not complete.** `iterations.jsonl`: `update-state` `OK`, then `define-next`
    `{"status":"TIMEOUT","turns":0,"duration_s":1200.1}`; no `advance`, no `review`, no
    `iteration_summary`. The runner still wrote `cid(log): iteration 155`. Nothing was reverted and
    nothing is corrupt — but 20 minutes of wall clock produced no commit.
- **There is an uncommitted `.claude/context/next.md` in the working tree** (+257/−183), written by
    that timed-out agent. It is a complete, well-scoped work package for the `Final_Sigma` fix
    (vendor 16.0.0 `Cased`/`Case_Ignorable`, pre-substitute σ/ς before delegating to
    `to_lowercase()`, 7 oracle-backed unit tests, CRAP/iai gate warnings, a `/tmp` sweep probe). **I
    verified its central claims and they hold** (the defect, the cause, "exactly one divergent
    scalar"). **I did not verify its table-shape invariants** — `CASED_RANGES` = 152 ranges / 4,311
    code points and `CASE_IGNORABLE_RANGES` = 452 ranges / 2,749 code points are unchecked numbers
    that would become hard-coded `EXPECTED_*` assertions; they must be re-derived, not trusted. The
    next `define-next` should adopt-and-verify this file rather than start from a blank page.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` + inline
    comment (still `@main` at `release.yml:895`); major dependency bumps **one per step** (xunit
    3.x, Test.Sdk 18.x, Gradle wrapper — Kotlin already warns 8.12.1 vs plugin 2.4.10 — JUnit 6.x,
    plus the `jni` 0.22 / `magnus` 0.8 source rewrites, one crate per step); make the
    `specs/ci-cd.md` job table exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**8 entries in `issues.md` — 4 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
None opened or closed since the last assessment; nothing is parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)", still the only correctness item,
    and it just grew a third remainder.** (a2) the criterion-4 differential sweep as a runnable
    check; (b) propagation to C FFI, C++, Swift and the 4 sibling `data.json` locations; and **(new,
    unfiled) the `Final_Sigma` case-table dependence measured above** — the issue text does not
    mention case *classification* anywhere, only category and normalization tables. The review agent
    should fold it in.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (trigger-contingent — do not
    act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Freeze `Final_Sigma` case classification at Unicode 16.0.0 — fix the divergence before building
the gate that would report it.**

This inverts the previous milestone deliberately, and the evidence for the inversion was produced
this iteration, not inherited: criterion 4 requires an **empty** divergence set, and today's set is
`{U+0295}`. Landing the sweep gate first would land it red.

- **Scope**: vendor the 16.0.0 `Cased` / `Case_Ignorable` classification (a checked-in PEP 723
    generator mirroring `scripts/gen_unicode16_unassigned.py`, derived from CPython 3.14's own
    tables — `unicodedata2` exposes categories, not these derived properties), apply `Final_Sigma`
    from it in `text_collapse`, and pin the behaviour with unit tests whose expected values come
    from `iscc-core` 1.3.0 on CPython 3.14 — **never** recomputed from the Rust code.
- **Do not extend `crates/iscc-lib/tests/unicode_boundary.json` in this step.** Nine binding suites
    assert exactly 7 `text_clean` + 5 `text_collapse` cases as a metadata guard; adding a vector
    reds all nine at once. A sigma vector belongs in a later, deliberate "extend the fixture + bump
    all nine count guards" slice.
- **Re-derive, do not trust, the two table-shape numbers** in the uncommitted `next.md` (152 / 452
    ranges). Everything else in that file that I could check, checked out.
- **Gates this will actually trip** — unlike the last five iterations, which moved only test files:
    CRAP (new functions and branches in a covered file → refresh `.crap-baseline.json` in the *same*
    commit) and iai (>10% Ir on the benched `text_collapse`; if a baseline refresh is needed, state
    the measured per-bench delta and the correctness justification in the handoff rather than
    refreshing quietly).
- **Correct the two overclaims in the same commit**: `docs/unicode.md`'s "invariant under table
    upgrades" / "`Final_Sigma` context match … exactly" and the matching note in
    `crates/iscc-lib/CLAUDE.md`. Leave `specs/rust-core.md:149-157` alone — specs are human-owned,
    even though its "Rust `str::to_lowercase()` does the same" sentence is what this defect
    falsifies.

**Then, in the following step, land criterion 4 — the sweep as a runnable check**, green. It must
cover **both** the 1,112,064 scalars *and* the sequence classes (`decisions.md` 2026-07-26: a
code-point-only sweep scored the superseded pre-filter 0 mismatches while it failed 504 of 1,270
sequence cases). Scope its blind spots explicitly — is it fail-open when the oracle is missing? does
a zero-case run read as green? does it distinguish "skipped" from "passed"? which CI job runs it,
and is its input set declared so a cached run cannot silently skip it (the exact hazard that hid 13
Kotlin tests at 154)?

**Propagation (C FFI → Swift → C++) stays queued behind both.** Two standing hazards for any fixture
work: never write `unicode_boundary.json` through the Write/Edit tools (the `\uXXXX` escapes decode
to literal UTF-8 — use `cp`, or Python with `ensure_ascii=True`), and do not relabel the
`e U+A7F1 U+0301` row's oracle back to `e U+015A` (that is the category-override design's output,
not a delete filter's).
