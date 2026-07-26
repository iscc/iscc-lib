<!-- assessed-at: 2c175f2292d4d716d816ce95d57c27d456738c54 -->

# Project State

## Status: IN_PROGRESS

## Phase: Post-v0.5.0 — Unicode boundary propagation at 6 of 11 native surfaces (napi + Java gated at 153)

Iteration 153 propagated the canonical Unicode boundary fixture to the napi and JNI/Java suites —
two new test files, one documentation paragraph, zero source changes and **no new vendored copy**.
CI is green on the pushed tip and covers every line of working-tree code. Criterion 3 now stands at
6 of 11 native surfaces plus the pure-Go port; criterion 4 (the differential sweep) has still not
started.

## Rust Core Crate

**Status**: partially met — criteria 1 and 2 met; criterion 3 at 6 of 11 surfaces; criterion 4 not
started

- Incremental scope: `git diff 672d109..HEAD --stat` = 17 files, **3 outside `.claude/`**:
    `crates/iscc-napi/__tests__/unicode_boundary.test.mjs` (new, 74 lines),
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/UnicodeBoundaryTest.java` (new, 96 lines)
    and `docs/unicode.md` (±2 lines of prose). `git diff 672d109..HEAD -- .claude/context/specs/` is
    **empty** and `-- .github/` is **empty** — no spec text and no workflow moved, so no met→unmet
    flip from either side. **No file under any `crates/*/src/` changed**; neither
    `.crap-baseline.json` nor `.iai-baseline.json` was touched, so the CRAP, iai and semver gates
    could not react to this diff.
- 8 crates; `iscc-lib` holds **335** `#[test]` functions (unchanged — both new suites live in
    binding test trees, not in the core crate). All 10 `gen_*_v0` functions still pass the vendored
    `iscc-core/data.json` vectors. Version **0.5.0**. Working tree clean.
- **Criterion 1 (freeze rule) — MET**, unchanged at HEAD (`UNASSIGNED_SENTINEL: char = '\u{FFFF}'`
    in `utils.rs`, mapped at both call sites inside the fused iterator; vendored 731-range /
    819,533-code-point table untouched).
- **Criterion 2 (table deps) — MET**, unchanged: `unicode-general-category` 1.1.0 (16.0) and
    `unicode-normalization` 0.1.25 (17.0), both freely upgradable under the freeze rule.
- **Criterion 3 (boundary vectors in every binding) — PARTIALLY met, 6 of 11 native surfaces (was
    4).**
    - Canonical fixture `crates/iscc-lib/tests/unicode_boundary.json` unchanged (SHA-256 `3ccc4418…`):
        `_metadata.unicode_data_version == "16.0.0"`, **7 `text_clean` + 5 `text_collapse`** cases,
        including the four sequence vectors that discriminate the ruled sentinel map from the
        superseded delete filter (verified by parsing the file, not by reading prose).
    - **Tally definition** (anchored in `docs/unicode.md`, "Cross-implementation consistency"): the 11
        native surfaces are Python, Node.js, WASM, C FFI, Java, Ruby, C#, C++, Swift, Kotlin **and the
        Rust crate itself**. `packages/go` is a separate pure-Go reimplementation (gated, not one of
        the 11); UniFFI is the shared mechanism behind Kotlin/Swift, not an independent surface.
    - **Gated: Rust (149), Python (150), WASM (151), Ruby (151), napi (153), Java (153)** + the
        **pure-Go port** (150, 9 of 12 vectors, exactly 3 ruled skips with a stale-key guard).
        `git ls-files | grep -i unicode_boundary` returns exactly **8** paths — 1 canonical fixture, 1
        vendored Go copy, 6 test files — matching that tally with no surprises.
    - **Both new suites verified independently this iteration** (read in full, not trusted from the
        handoff): each carries a metadata guard asserting `16.0.0` plus the exact 7 / 5 section counts
        (so a truncated fixture cannot silently degrade to a zero-iteration loop), then loops every
        case asserting plain equality against `outputs.result` — no `delete_filter_output` oracle
        column is copied, correctly. Both read the canonical fixture **in place**: napi via
        `join(__dirname, '..', '..', 'iscc-lib', 'tests', …)`, Java via
        `Files.readString(Path.of("../../iscc-lib/tests/unicode_boundary.json"))` which resolves
        because surefire's default working directory is the pom basedir (the pre-existing
        `IsccLibTest.java` uses the identical relative path against `data.json`).
    - **CI reachability confirmed from `ci.yml`, not from the handoff:** job `nodejs` runs
        `npm install` → `npx napi build --platform` → `npm test`, whose script is
        `node --test __tests__/*.test.mjs`, so the new file is globbed; job `java` runs
        `cargo build -p iscc-jni` → `mvn test -f crates/iscc-jni/java/pom.xml`, and surefire
        auto-discovers `*Test.java`. Both jobs rebuild the native artifact first, so the local
        stale-addon hazard cannot reach CI. `gson` 2.14.0 and `junit-jupiter` 5.14.4 are already
        test-scoped in the pom — no dependency was added.
    - **Drift gate untouched and still consistent:** `VENDORED_COPIES` still registers 5 copies (4
        `data.json` + 1 `unicode_boundary.json`), and
        `git ls-files -- '*data.json'   '*unicode_boundary.json'` returns exactly the **7** paths it
        registers. Slice 3 deliberately added no tracked copy.
    - **Remaining gap — 5 native surfaces + 4 sibling `data.json` copies.** Ungated: **C FFI, Kotlin,
        Swift, C#, C++**; `packages/{dotnet,kotlin,swift}` and `packages/go` carry a `data.json` copy
        but no `unicode_boundary.json`.
    - **Two-axis slice-cost survey re-run at HEAD** (fixture-reading plumbing × text-function
        coverage), now with a third decisive axis — *local buildability*:
        - **C# — cheapest remaining.** `ConformanceTests.cs` already parses JSON via `System.Text.Json`
            from `Path.Combine(AppContext.BaseDirectory, "testdata", "data.json")`, the csproj already
            has a `<Content Include="testdata\data.json">` copy rule, and `SmokeTests.cs` already calls
            the text functions (PascalCase `TextClean`/`TextCollapse`). `dotnet` is present in this
            container. Note an option that avoids a new tracked copy entirely:
            `<Content Include="..\..\..\crates\iscc-lib\tests\unicode_boundary.json"   Link="testdata\unicode_boundary.json" />`
            — otherwise the new copy **must** be registered in `VENDORED_COPIES`.
        - **Kotlin / Swift — JSON plumbing but zero text-function tests, and each adds a tracked copy.**
            Kotlin loads `src/test/resources/data.json` as a classloader resource, Swift via
            `Bundle.module`. **Neither can be verified locally**: `swift` is absent and only the Gradle
            *wrapper* exists (`gradle` binary absent; the wrapper needs a network download and has a
            recorded history of bind-mount flakes).
        - **C FFI / C++ — most expensive, unchanged verdict.** `crates/iscc-ffi/tests/test_iscc.c` has
            **zero** text-function coverage and no JSON parser (its 3 `json` hits are the
            `iscc_json_to_data_url` case); `packages/cpp/tests/test_iscc.cpp` is a hand-written smoke
            suite with 1 file of text hits and no vector file anywhere. `gcc` is present, but **`cmake`
            is still absent**, so C++ remains CI-verified only.
- **Criterion 4 (differential sweep) — NOT started.** `scripts/` still has no sweep harness. The
    requirement is **zero** divergence over both the **1,112,064 Unicode scalar values** and the
    sequence classes; a per-code-point-only sweep is explicitly forbidden as false assurance.
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
    metadata guard). `tests/test_vendored_fixtures.py` (8 tests) re-verified structurally at HEAD:
    its `VENDORED_COPIES` table matches the git index exactly, so the set-equality check that
    catches an unregistered *or deleted* tracked copy is green without modification.
- Known blind spot, recorded not fixed: discovery keys on the two basenames, so a copy vendored
    under a different filename escapes the set check. Mitigation is to keep the canonical basenames
    when propagating — noted in learnings.md and in the Unicode issue.
- Both files are collected by `testpaths = ["tests"]` and therefore run in the `python-test`
    3.10/3.14 CI matrix and in the `always_run` pre-push pytest hook.

## Node.js Bindings

**Status**: met — Unicode-gated since 153

- `crates/iscc-napi` exports all 32 Tier 1 symbols with streaming classes; **no source file
    changed** — the slice was purely additive test code.
- `__tests__/unicode_boundary.test.mjs` (74 lines): `node:test` suite, 1 metadata guard + 7
    `text_clean` + 5 `text_collapse` cases over the canonical fixture, zero skips. Imports the
    **snake_case** exports `text_clean` / `text_collapse` from `../index.js`.
- The local `.node` addon has been rebuilt (dated Jul 26 21:09) so local runs are meaningful again.
    It remains gitignored (`crates/iscc-napi/.gitignore:4:*.node`; `git ls-files crates/iscc-napi/`
    lists 10 tracked files, none of them `.node`) — the recurring "checked-in stale artifact" claim
    in older handoffs was never true and is now moot.

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
- **Now the single most expensive propagation target** (C++ is equally expensive *and* unbuildable
    here). `tests/test_iscc.c` has neither a JSON parser nor any text-function coverage, so the
    fixture needs a generated C table or a hand-rolled reader. Scope it deliberately if picked;
    `gcc` is available locally, so at least verification is possible.

## Other Bindings (Java, Kotlin, Swift, Go, Ruby, C#, C++)

**Status**: met; Java and Ruby Unicode-gated, the rest pending

- `crates/iscc-jni`, `crates/iscc-rb`, `crates/iscc-uniffi` (32 exports, 21 tests, `publish=false`)
    plus `packages/{dotnet,cpp,go,kotlin,swift}` carry the full 32-symbol surface. Kotlin 2.3+
    consumer floor stands. **No binding source file changed this iteration** — only two test files.
- **Java — gated since 153**: `UnicodeBoundaryTest.java` uses gson + JUnit `@TestFactory` dynamic
    tests over the canonical fixture read by relative path, plus a `@Test` metadata guard. It is the
    first JVM surface with any text-function coverage at all.
- **Ruby — gated since 151**: `crates/iscc-rb/test/test_unicode_boundary.rb` reads the canonical
    fixture via `File.expand_path`, 12 vectors, zero skips.
- Go: `Final_Sigma` fix holds (`packages/go/utils.go:125` uses `cases.Lower(language.Und)`; the
    per-call `cases.Caser` is deliberate — do not hoist it). `packages/go` sits at **177**
    `func Test` and its vendored `testdata/unicode_boundary.json` is byte-identical to the canonical
    file (SHA-256 `3ccc4418…` on both) — machine-enforced since 152.
- **Standing ruling (decisions.md 2026-07-26):** the Go skip map stays **unconditional** — do not
    build-tag it for go1.27. Go passes the two post-16.0 vectors today *by accident* (its 15.0
    tables call `U+20C1` / `U+A7F1` `Cn`), so a go1.27 bump will red 5 cases; that red is the
    intended trigger to land the 731-range freeze table in `packages/go/utils.go` in the **same
    commit** as the toolchain bump. CI pins Go via `go-version-file: packages/go/go.mod`
    (`go 1.26.1`).
- Kotlin and Swift suites still contain **zero** text-function tests (verified case-insensitively);
    C# has 1 test file with text hits, C++ 1.

## Documentation

**Status**: met

- `docs/unicode.md` is the only doc touched: the vector-family paragraph now reads "by the Python,
    Node.js, WASM, Java, and Ruby binding suites (which all read the canonical fixture directly),
    and by the pure-Go package via the vendored copy" — accurate at HEAD.
- Page-list machinery green: `uv run scripts/check_docs_nav.py` → **23 pages** consistent across
    nav, `ORDERED_PAGES` and `llms.txt` (the edit was prose-only, no page added). 12 crate/package
    READMEs, 12 crate/package CLAUDE.md files (+ root), 11 `docs/howto/*.md`;
    `uv run scripts/version_sync.py --check` reports **21/21 OK**.
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

- **Latest CI: GREEN.** check-runs API on `32fbfc4` (= `origin/develop`): **43 check-runs, 22
    distinct check names, 0 non-success, 0 in progress.** That run includes both new test files, so
    the napi and Java gates are proven in CI and not only locally. HEAD is `2c175f2`, one commit
    ahead (the `cid(log)` commit for iteration 153), and
    `git diff --stat origin/develop..HEAD -- . ':!.claude'` is **empty** — the green run covers
    every line of code. (~2× runs because PR **#44** `develop` → `main` is OPEN, titled "Release
    0.6.0" — not shipped; version is still **0.5.0**.)
- `ci.yml` / `release.yml` / `docs.yml` byte-unchanged since the last assessment: 21 jobs → 22 check
    names, matching the API exactly. `release.yml` keeps 97 `uses:` refs, 8 registry toggles,
    `workflow_dispatch`-only. `specs/ci-cd.md` stands at 44 checked / 8 unchecked.
- Slice 3 needed **no CI edit**, as predicted: all six gated suites ride existing jobs
    (`python-test`, `nodejs`, `wasm`, `java`, `ruby`, `go`), and every one of those jobs rebuilds
    its native artifact before testing.
- Enforcing and green: iai-callgrind perf (>10% Ir), coverage + CRAP (`--fail-regression` is
    **CI-only**, so a green `mise run check` proves nothing), `cargo-deny`, docs page-list parity.
    `cargo-semver-checks` informational.
- Iteration 153 ran clean end to end: four roles, all `"status":"OK"`, verdict `PASS`, no crash and
    no timeout in `iterations.jsonl`.
- **Authorized for CID:** pin `rubygems/configure-rubygems-credentials` to `@v2.1.0` + inline
    comment (still `@main` at `release.yml:895`); major dependency bumps **one per step** (xunit
    3.x, Test.Sdk 18.x, Gradle wrapper, JUnit 6.x, plus the `jni` 0.22 / `magnus` 0.8 source
    rewrites, one crate per step); make the `specs/ci-cd.md` job table exhaustive.
- **Deferred by ruling:** npm OIDC trusted publishing is **not** for v0.6.0 (`NPM_TOKEN` valid until
    2026-09-16) and CID must not even prepare the YAML diff. No Dependabot/Renovate.

## Open Issues

**8 entries in `issues.md` — 4 `normal`, 4 `low`, zero `critical`, zero `HUMAN REVIEW REQUESTED`.**
None opened or closed this iteration; nothing is parked on Titusz.

- **NORMAL — "Declare and gate a Unicode data version (DECIDED)", still the only correctness item.**
    Remainders: **(b)** propagate the fixture to the 5 remaining native surfaces and the 4 sibling
    `data.json` locations, and **(a2)** the criterion-4 differential sweep as a runnable check. The
    entry also carries the `VENDORED_COPIES` registration requirement and the go1.27 bump checklist.
    Its `**Upstream:**` section holds a human-owned task (update `iscc-core#137` to propose the
    sentinel mechanism) — explicitly *not* CID's.
- **NORMAL:** dependency review and refresh (major bumps, one per step); the rubygems `@v2.1.0` pin;
    the exhaustive `specs/ci-cd.md` job table.
- **LOW / CID skips:** the three gate-script remainders deferred at 146 (trigger-contingent — do not
    act unprompted), v1.0.0 (HELD), docs language logos, npm OIDC.

## Next Milestone

**Continue Unicode boundary propagation with slice 4 — take C# next, alone or paired with C FFI.**

- **C# is unambiguously the cheapest remaining surface on all three axes.** It already parses JSON
    (`System.Text.Json` in `ConformanceTests.cs`), already has a fixture-copy mechanism
    (`<Content Include="testdata\data.json">` + `AppContext.BaseDirectory`), already calls
    `TextClean`/`TextCollapse` in `SmokeTests.cs`, and `dotnet` is installed in this container so
    the result is locally verifiable. Prefer the
    `<Content Include="..\..\..\crates\iscc-lib\tests\   unicode_boundary.json" Link="testdata\unicode_boundary.json" />`
    form, which keeps the canonical fixture single-sourced; if a tracked copy is created instead,
    it **must** be registered in `VENDORED_COPIES` in the same commit (the drift gate redding on an
    unregistered copy is the gate working, not a bug) and must keep the basename
    `unicode_boundary.json`.
- **Do not pick Kotlin or Swift yet** — both have zero text-function tests *and* cannot be built
    here (`swift` absent; only the Gradle wrapper exists, which needs a network download and has a
    history of bind-mount flakes), so verification would rest on CI alone.
- **C++ stays last:** no JSON parsing anywhere in its suite and **`cmake` is still absent** from
    this container. C FFI is equally plumbing-poor but at least `gcc` is present.

Reuse the pattern proven at 150–153 verbatim: read the canonical fixture in place wherever the
language can; assert `unicode_data_version` plus per-section counts (7 / 5) as a metadata guard so a
truncated fixture cannot degrade to a zero-iteration loop; never copy the `delete_filter_output`
oracle (plain equality against `outputs.result` already reds a delete-filter regression); give any
skip list a stale-key guard. Two standing hazards: never write the fixture through the Write/Edit
tools (the `\uXXXX` escapes decode to literal UTF-8 — use `cp`, or Python with `ensure_ascii=True`),
and do not relabel the `e U+A7F1 U+0301` row's oracle back to `e U+015A` (that is the
category-override design's output, not a delete filter's).

After propagation, the last Unicode item is **(a2)**, the criterion-4 differential sweep covering
both the 1,112,064 scalar values and the sequence classes. Neither task touches a hot path, so no
perf or CRAP gate should react.
