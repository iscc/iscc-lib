<!-- assessed-at: 0b6b3be9efa1a2b1392839d8714da8d8b211a67b -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — Unicode boundary propagation at 8 of 11 native surfaces (C# + Kotlin gated at 154)

Iteration 154 propagated the canonical Unicode boundary fixture to the C# and Kotlin suites without
adding a single tracked copy — both link the canonical file through a build-config indirection. CI
is green on the pushed tip and covers every line of working-tree code. Criterion 3 now stands at 8
of 11 native surfaces plus the pure-Go port; criterion 4 (the differential sweep) is the last
Unicode item with no partial credit and has still not started.

## Rust Core Crate

**Status**: partially met — criteria 1 and 2 met; criterion 3 at 8 of 11 surfaces; criterion 4 not
started

- Incremental scope: `git diff 2c175f2..HEAD --stat` = 23 files, **6 outside `.claude/`**:
    `packages/dotnet/Iscc.Lib.Tests/UnicodeBoundaryTests.cs` (new, 76 lines),
    `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/UnicodeBoundaryTest.kt` (new, 84 lines),
    `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` (+3), `packages/kotlin/build.gradle.kts`
    (+7), two package `CLAUDE.md` files (+1 each) and `docs/unicode.md` (±3 prose lines). Verified
    empty at HEAD: `git diff 2c175f2..HEAD -- .claude/context/specs/`, `-- .github/`, and
    `-- 'crates/*/src' .crap-baseline.json .iai-baseline.json`. No spec text moved, no workflow
    moved, no source file moved — the CRAP, iai and semver gates could not react to this diff.
- 8 crates; `iscc-lib` holds **335** `#[test]` functions (unchanged — both new suites live in
    package test trees, not in the core crate). All 10 `gen_*_v0` functions still pass the vendored
    `iscc-core/data.json` vectors. Version **0.5.0**. Working tree clean.
- **Criterion 1 (freeze rule) — MET**, unchanged at HEAD: `UNASSIGNED_SENTINEL: char = '\u{FFFF}'`
    at `crates/iscc-lib/src/utils.rs:35`, applied at both call sites (`:122`, `:213`) inside the
    fused iterator; vendored 731-range / 819,533-code-point table untouched.
- **Criterion 2 (table deps) — MET**, unchanged: `unicode-general-category` 1.x (16.0) and
    `unicode-normalization` 0.1.x (17.0) in `Cargo.toml`, both freely upgradable under the freeze
    rule.
- **Criterion 3 (boundary vectors in every binding) — PARTIALLY met, 8 of 11 native surfaces (was
    6).**
    - Canonical fixture `crates/iscc-lib/tests/unicode_boundary.json` unchanged (SHA-256 `3ccc4418…`),
        verified by parsing, not by prose: `_metadata.unicode_data_version == "16.0.0"`, **7
        `text_clean` + 5 `text_collapse`** cases including the four sequence vectors that discriminate
        the ruled sentinel map from the superseded delete filter.
    - **Tally definition** (anchored in `docs/unicode.md` → "Cross-implementation consistency", read
        at HEAD): the 11 native surfaces are Python, Node.js, WASM, C FFI, Java, Ruby, C#, C++, Swift,
        Kotlin **and the Rust crate itself**. `packages/go` is a separate pure-Go reimplementation
        (gated, not one of the 11); UniFFI is the shared mechanism behind Kotlin/Swift, not an
        independent surface.
    - **Gated: Rust (149), Python (150), WASM (151), Ruby (151), napi (153), Java (153), C# (154),
        Kotlin (154)** + the **pure-Go port** (150, 9 of 12 vectors, exactly 3 ruled skips with a
        stale-key guard). `git ls-files | grep -i unicode_boundary` returns 11 paths — 9 test files, 1
        canonical fixture, 1 vendored Go copy — matching the tally exactly.
    - **Both new suites read in full and verified independently this iteration.** Each carries a
        metadata guard asserting `16.0.0` plus the exact 7 / 5 section counts (a truncated fixture
        cannot degrade to a zero-iteration loop), then asserts plain equality against `outputs.result`
        for every case — no `delete_filter_output` oracle column is copied, and there are no skips.
        C#: `Lazy<JsonElement>` + `[Fact]` guard + two `[Theory]`/`[MemberData]` methods calling
        PascalCase `IsccLib.TextClean`/`TextCollapse`. Kotlin: gson + `@TestFactory`/`DynamicTest`
        (each vector is its own `testcase` in the Gradle XML) calling camelCase
        `textClean`/`textCollapse`.
    - **No new tracked copy — both link the canonical fixture.** C# via a csproj
        `<Content Include="..\..\..\crates\iscc-lib\tests\unicode_boundary.json" Link="testdata\unicode_boundary.json">`
        item read through `AppContext.BaseDirectory`; Kotlin via an `iscc.fixtureDir` system
        property set in `build.gradle.kts`. Ruled deliberate in `decisions.md` (2026-07-26, "Binding
        boundary suites link the canonical fixture instead of vendoring a copy") — the asymmetry with
        the sibling `data.json` copies in the same two test trees is intentional, not cleanup debt.
    - **Gradle staleness hole found and closed by review, verified in the diff at HEAD:**
        `build.gradle.kts` now declares
        `inputs.file("$fixtureDir/unicode_boundary.json").withPathSensitivity(PathSensitivity.NONE)`.
        Without it, an out-of-tree fixture edit left `./gradlew test` `UP-TO-DATE` and silently
        skipped all 13 boundary tests. CI was never affected (fresh checkout, no build-dir cache) —
        this was a local/CID verification hole. **Standing question for every future slice: is the
        fixture a declared input of that surface's build system? A green run does not answer it.**
    - **CI reachability confirmed from `ci.yml`, not from the handoff:** job `dotnet` (L140) runs
        `cargo build -p iscc-ffi` → `dotnet build …Iscc.Lib.Tests.csproj` → `dotnet test`, and job
        `kotlin` (L259) runs `cargo build -p iscc-uniffi` → `./gradlew test`. Both rebuild the native
        artifact first, so a stale local library cannot reach CI; both discover the new files without
        any workflow edit.
    - **Drift gate untouched and still consistent:** `tests/test_vendored_fixtures.py` registers 5
        copies (4 `data.json` + 1 `unicode_boundary.json`) and
        `git ls-files -- '*data.json' '*unicode_boundary.json'` returns exactly the **7** matching
        paths. Slice 4 added no tracked copy, so the table needed no edit.
    - **Remaining gap — 3 native surfaces + 4 sibling `data.json` copies.** Ungated: **C FFI, C++,
        Swift**; `packages/{dotnet,kotlin,swift}` and `packages/go` carry a `data.json` copy but no
        `unicode_boundary.json`.
    - **Three-axis cost survey re-run at HEAD** (fixture plumbing × text-function coverage × local
        buildability):
        - **C FFI — cheapest remaining, and the only one buildable here.** `gcc` is present and CI
            compiles `crates/iscc-ffi/tests/test_iscc.c` as a single `gcc` invocation (ci.yml L130-139).
            But the file has **zero** text-function hits (case-insensitive) and no JSON parser — its
            `json` matches are the `iscc_json_to_data_url` case. Needs a generated C vector table or a
            hand-rolled reader.
        - **Swift — plumbing exists, but no toolchain and it is the one surface that must vendor.**
            `ConformanceTests.swift` parses via `Bundle.module` + `JSONSerialization`, and
            `Package.swift` declares `resources: [.copy("data.json")]`, so a boundary fixture needs a
            **tracked copy** registered in `VENDORED_COPIES`. Zero text-function tests today; `swift` is
            ABSENT in this container.
        - **C++ — last.** `packages/cpp/tests/test_iscc.cpp` has 3 text hits but no vector file and no
            JSON parser anywhere in the suite, and **`cmake` is ABSENT**, so it is CI-verified only.
- **Criterion 4 (differential sweep) — NOT started.** `ls scripts/` shows no sweep harness (9 files,
    none matching `sweep`). The requirement is **zero** divergence over both the **1,112,064 Unicode
    scalar values** and the sequence classes; `decisions.md` (2026-07-26, "Sentinel conformance
    accepted on sequence evidence") records that a per-code-point-only sweep scored the superseded
    pre-filter 0 mismatches while it failed 504 of 1,270 sequence cases — a code-point-only sweep is
    false assurance and is explicitly forbidden.
- **Stale spec text, human-owned (CID must not edit specs):** `specs/rust-core.md:149-157` still
    describes the Go `Final_Sigma` defect in the present tense although the fix landed at iteration
    147; the criterion-1 and criterion-3 **Verified when** boxes remain unchecked despite being met
    / partially met. Spec checkboxes are not a progress signal in this repo.
- `cargo-semver-checks` stays informational (`continue-on-error: true`) until v1.0.0, which is HELD.

## Python Bindings

**Status**: met — Unicode-gated since 150; host to the vendored-fixture drift gate

- All 32 Tier 1 symbols; `IsccResult`, streaming hashers, 12 `.detach(` GIL-release sites;
    abi3-py310 wheels incl. aarch64. Nothing under `crates/iscc-py/` moved this iteration.
- `tests/test_unicode_boundary.py` reads the canonical fixture by relative path (12 vectors + a
    metadata guard). `tests/test_vendored_fixtures.py` re-verified structurally at HEAD: its
    `VENDORED_COPIES` table still matches the git index exactly, so the set-equality check that
    catches an unregistered *or deleted* tracked copy is green without modification.
- Known blind spot, recorded not fixed: discovery keys on the two basenames
    (`FIXTURE_BASENAMES = {"data.json", "unicode_boundary.json"}`), so a copy vendored under a
    different filename escapes the set check. Mitigation is to keep the canonical basenames when
    propagating — noted in learnings.md and in the Unicode issue.
- Both files are collected by `testpaths = ["tests"]` and therefore run in the `python-test`
    3.10/3.14 CI matrix and in the `always_run` pre-push pytest hook.

## Node.js Bindings

**Status**: met — Unicode-gated since 153

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes; untouched this iteration.
- `__tests__/unicode_boundary.test.mjs` (74 lines): `node:test` suite, 1 metadata guard + 7
    `text_clean` + 5 `text_collapse` cases over the canonical fixture, zero skips. Imports the
    **snake_case** exports `text_clean` / `text_collapse` from `../index.js`.
- The local `.node` addon is gitignored (`crates/iscc-napi/.gitignore:4:*.node`) — the recurring
    "checked-in stale artifact" claim in older handoffs was never true and stays refuted.

## WASM Bindings

**Status**: met — Unicode-gated since 151

- `crates/iscc-wasm` exports all 32 symbols; `SumHasher` wrapper present; the `blake3 wasm32_simd`
    feature dep is intentional feature-unification (do not prune). Untouched this iteration.
- `tests/unicode_boundary.rs`: 3 `#[wasm_bindgen_test]` fns over the canonical fixture via
    `include_str!`, zero skips, ungated by the `conformance` feature; runs only under
    `wasm-pack test --node` (the CI `wasm` job).

## C FFI

**Status**: met, with the cross-cutting Unicode criterion pending

- `crates/iscc-ffi/src/lib.rs` exposes **47** `#[unsafe(no_mangle)]` externs; cbindgen header plus
    csbindgen C# generation run from `build.rs` on every `cargo build`. Untouched.
- **Now the cheapest remaining propagation target that can actually be built here** (`gcc` present;
    C++ needs the absent `cmake`, Swift the absent `swift`). The cost is plumbing, not coverage:
    `tests/test_iscc.c` has neither a JSON parser nor any text-function assertion, and CI compiles
    it as one bare `gcc` command with no third-party includes — so the fixture must arrive as a
    generated C table (build-script or checked-in generator output) or a hand-rolled reader. Scope
    it deliberately if picked.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met; Java, Ruby, C# and Kotlin Unicode-gated, C++ and Swift pending

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` carry the full 32-symbol surface. **No binding
    source file changed this iteration** — only two test files and their two build configs.
- **C# — gated since 154**: `UnicodeBoundaryTests.cs`, 13 tests (1 `[Fact]` guard + 12 theory
    cases), zero skips, fixture linked via csproj
    `<Content Include … Link="testdata\unicode_boundary.json">`.
- **Kotlin — gated since 154**: `UnicodeBoundaryTest.kt`, 13 tests via `@TestFactory`/`DynamicTest`,
    zero skips, fixture located by the `iscc.fixtureDir` system property and declared as a Gradle
    task input. The `@TestFactory` shape is load-bearing: a plain `@Test` loop would report 3
    `testcase` entries and hide per-vector names.
- **Java — gated since 153**: `UnicodeBoundaryTest.java` (gson + `@TestFactory`), canonical fixture
    by relative path, resolving because surefire's default working directory is the pom basedir.
- **Ruby — gated since 151**: `crates/iscc-rb/test/test_unicode_boundary.rb` via `File.expand_path`,
    12 vectors, zero skips.
- Go: `Final_Sigma` fix holds (`packages/go/utils.go:125` uses `cases.Lower(language.Und)`; the
    per-call `cases.Caser` is deliberate — do not hoist it). `packages/go` sits at **177**
    `func Test` and its vendored `testdata/unicode_boundary.json` is byte-identical to the canonical
    file (SHA-256 `3ccc4418…` on both, re-verified) — machine-enforced since 152.
- **Standing ruling (decisions.md 2026-07-26):** the Go skip map stays **unconditional** — do not
    build-tag it for go1.27. Go passes the two post-16.0 vectors today *by accident*, so a go1.27
    bump will red 5 cases; that red is the intended trigger to land the 731-range freeze table in
    `packages/go/utils.go` in the **same commit** as the toolchain bump. CI pins Go via
    `go-version-file: packages/go/go.mod` (`go 1.26.1`).
- Swift and C++ suites still contain **zero** and **3** text-function hits respectively (checked
    case-insensitively); neither has a boundary fixture.

## Documentation

**Status**: met

- `docs/unicode.md` is the only doc touched: the vector-family paragraph now names "Python, Node.js,
    WASM, Java, Ruby, C#, and Kotlin binding suites (which all read the canonical fixture directly),
    and … the pure-Go package via the vendored copy" — accurate at HEAD.
- Page-list machinery green: `uv run scripts/check_docs_nav.py` → **23 pages** consistent across
    nav, `ORDERED_PAGES` and `llms.txt` (prose-only edit, no page added). 12 crate/package READMEs,
    12 crate/package CLAUDE.md files (+ root), 11 `docs/howto/*.md`;
    `uv run scripts/version_sync.py --check` reports **21/21 OK**.
- The two package CLAUDE.md file-layout tables were updated in the same commit to list the new test
    files — small, correct, and the kind of drift that usually gets missed.
- Known incompleteness, deliberately left: the pure-Go admonition explains the *skips* but not that
    go1.27 will also red 5 currently-green cases. Worth widening when the Go freeze table lands.
- Remaining item is cosmetic and human-owned: `Add programming language logos to docs site` (`low`).

## Benchmarks

**Status**: met

- `crates/iscc-lib/benches/benchmarks.rs`: 12 criterion benches (criterion 0.7, `black_box` from
    `std::hint`). `benches/iai_benches.rs`: iai-callgrind 0.16, 11 functions / 16 cases. 18
    pytest-benchmark fixtures in `tests/test_benchmarks.py`; documented speedups 1.3x–158x.
    Untouched — no perf surface moved, so neither the iai nor the CRAP gate could react.

## CI/CD and Publishing

**Status**: partially met — green and covering all working-tree code

- **Latest CI: GREEN.** check-runs API on `ceb32fd` (= `origin/develop`): **43 check-runs, 22
    distinct check names, 0 non-success, 0 in progress.** That run includes both new suites and both
    build-config edits, so the C# and Kotlin gates are proven in CI and not only locally. HEAD is
    `0b6b3be`, one commit ahead (the `cid(log)` commit for iteration 154), and
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **empty** — the green run covers
    every line of code. (~2× runs because PR **#44** `develop` → `main` is OPEN, titled "Release
    0.6.0" — not shipped; version is still **0.5.0**.)
- `ci.yml` / `release.yml` / `docs.yml` byte-unchanged since the last assessment (`git diff` over
    `.github/` is empty): 21 jobs → 22 check names, matching the API exactly. `release.yml` keeps 97
    `uses:` refs, 8 registry toggles, `workflow_dispatch`-only. `specs/ci-cd.md` stands at 44
    checked / 8 unchecked.
- Slice 4 needed **no CI edit**, as predicted: all eight gated suites ride existing jobs
    (`python-test`, `nodejs`, `wasm`, `java`, `ruby`, `dotnet`, `kotlin`, `go`), and every one of
    those jobs rebuilds its native artifact before testing.
- Enforcing and green: iai-callgrind perf (>10% Ir), coverage + CRAP (`--fail-regression` is
    **CI-only**, so a green `mise run check` proves nothing), `cargo-deny` (live advisory DB — can
    red with no code change), docs page-list parity. `cargo-semver-checks` informational.
- Iteration 154 ran clean end to end: four roles, all `"status":"OK"`, verdict `PASS_WITH_NOTES`, no
    crash and no timeout in `iterations.jsonl`.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` + inline
    comment (still `@main` at `release.yml:895`); major dependency bumps **one per step** (xunit
    3.x, Test.Sdk 18.x, Gradle wrapper — the Kotlin build already emits a "Deprecated Gradle
    Version" warning for wrapper 8.12.1 vs Kotlin plugin 2.4.10 — JUnit 6.x, plus the `jni` 0.22 /
    `magnus` 0.8 source rewrites, one crate per step); make the `specs/ci-cd.md` job table
    exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**8 entries in `issues.md` — 4 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
None opened or closed this iteration; nothing is parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)", still the only correctness item.**
    Remainders: **(b)** propagate the fixture to the 3 remaining native surfaces (C FFI, C++, Swift)
    and the 4 sibling `data.json` locations, and **(a2)** the criterion-4 differential sweep as a
    runnable check. The entry carries the `VENDORED_COPIES` registration requirement (Swift is the
    surface expected to need it) and the go1.27 bump checklist. Its `**Upstream:**` section holds a
    human-owned task (update `iscc-core#137` to propose the sentinel mechanism) — explicitly *not*
    CID's.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (trigger-contingent — do not
    act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Land criterion 4 — the Unicode differential sweep as a runnable check** (`issues.md` → "Declare
and gate a Unicode data version", item (a2)).

- It is the only remaining Unicode piece with **no partial credit**, it protects every future table
    bump on every surface at once, and it is fully local — no missing toolchain, unlike the three
    surfaces left for propagation.
- **Non-negotiable scope:** the sweep must cover **both** the 1,112,064 Unicode scalar values *and*
    the sequence classes. `decisions.md` (2026-07-26, "Sentinel conformance accepted on sequence
    evidence") measured that a code-point-only sweep scored the superseded pre-filter 0 mismatches
    while it failed 504 of 1,270 sequence cases; shipping a code-point-only sweep would be false
    assurance and would contradict a standing decision.
- **Treat it as a new gate script, and scope its blind spots explicitly** (this is where the last
    three iterations found their real defects): is it fail-open when the reference table or oracle
    is missing? Does a zero-case run read as green? Does it distinguish "skipped" from "passed"?
    Which CI job runs it, and is its input set declared so a cached/UP-TO-DATE run cannot silently
    skip it? The oracle used in 148 was a `unicodedata2==16.0.0` reimplementation of `iscc-core`'s
    `text_clean`/`text_collapse`; the probe harness was throwaway and **not** committed, so it has
    to be rebuilt, not recovered.

**Alternative if the sweep is judged too large for one step: propagation slice 5 = C FFI.** It is
the cheapest remaining surface on the buildability axis (`gcc` present; `cmake` and `swift` are
ABSENT), but it is plumbing-heavy: `tests/test_iscc.c` has no JSON reader and no text-function
coverage, and CI compiles it with one bare `gcc` invocation, so the fixture must become a generated
C table or a hand-rolled reader. **Swift stays after that** (needs a *tracked* vendored copy via
SwiftPM `resources:` — register it in `VENDORED_COPIES`, keep the basename), **C++ last**.

Reuse the pattern proven at 150–154 verbatim: read the canonical fixture in place wherever the
language can; assert `unicode_data_version` plus per-section counts (7 / 5) as a metadata guard so a
truncated fixture cannot degrade to a zero-iteration loop; never copy the `delete_filter_output`
oracle (plain equality against `outputs.result` already reds a delete-filter regression); give any
skip list a stale-key guard. Carry forward the question iteration 154 surfaced: **is the fixture a
declared input of that surface's build system?** A green run does not answer it — Gradle reported
`UP-TO-DATE` and skipped all 13 tests until `inputs.file(...)` was declared. Two standing hazards:
never write the fixture through the Write/Edit tools (the `\uXXXX` escapes decode to literal UTF-8 —
use `cp`, or Python with `ensure_ascii=True`), and do not relabel the `e U+A7F1 U+0301` row's oracle
back to `e U+015A` (that is the category-override design's output, not a delete filter's).
