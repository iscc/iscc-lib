# Handoff

## 2026-07-26 — Propagate the Unicode boundary fixture to the C# and Kotlin test suites (slice 4)

**Done:** Added Unicode 16.0.0 boundary conformance suites to the .NET binding
(`UnicodeBoundaryTests.cs`, xunit `[Theory]`/`[MemberData]` + `[Fact]` metadata guard) and the
Kotlin/UniFFI binding (`UnicodeBoundaryTest.kt`, JUnit 5 `@TestFactory`/`DynamicTest` + `@Test`
metadata guard). Both read the *canonical* fixture through build-config indirection — C# via a
csproj `<Content Link=...>` item that copies it into the test output directory, Kotlin via an
`iscc.fixtureDir` system property in `build.gradle.kts` — so no tracked vendored copy was added and
`VENDORED_COPIES` is untouched. Criterion 3 goes from 6 to 8 of 11 native surfaces.

**Files changed:**

- `packages/dotnet/Iscc.Lib.Tests/UnicodeBoundaryTests.cs`: new — 1 metadata guard (version
    `16.0.0`, 7 + 5 case counts) + 7 `text_clean` + 5 `text_collapse` theory cases, plain equality
    against `outputs.result`, zero skips
- `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj`: one
    `<Content   Include="..\..\..\crates\iscc-lib\tests\unicode_boundary.json"   Link="testdata\unicode_boundary.json">`
    item (PreserveNewest), next to the existing `data.json` item
- `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/UnicodeBoundaryTest.kt`: new — idiomatic twin
    of the iteration-153 Java suite; gson lazy load, `DynamicTest` per vector so each case is a
    named `testcase` in the Gradle XML, zero skips
- `packages/kotlin/build.gradle.kts`: one
    `systemProperty("iscc.fixtureDir",   "${rootProject.rootDir}/../../crates/iscc-lib/tests")` line
    inside the existing `tasks.withType<Test>` block (resolves identically locally and in CI, whose
    working-directory is `packages/kotlin`)
- `docs/unicode.md`: canonical-fixture sentence now names Python, Node.js, WASM, Java, Ruby, **C#,
    and Kotlin**; pure-Go clause intact
- `packages/dotnet/CLAUDE.md`, `packages/kotlin/CLAUDE.md`: File Layout trees list the new test
    files

**Verification:** (all commands run this session, in order)

- `cargo build -p iscc-ffi && cargo build -p iscc-uniffi` — exit 0 (fresh natives before any test
    run)
- `LD_LIBRARY_PATH=$PWD/target/debug dotnet test packages/dotnet/Iscc.Lib.Tests/` — **104 passed, 0
    failed, 0 skipped** (baseline 91 + 13 new; meets the ≥ 104 criterion exactly)
- Same with `--filter FullyQualifiedName~UnicodeBoundary` — **13 passed, 0 failed, 0 skipped**
- `./gradlew cleanTest test --offline` in `packages/kotlin` — BUILD SUCCESSFUL;
    `TEST-uniffi.iscc_uniffi.UnicodeBoundaryTest.xml` written 22:14 (newer than the source file)
    with `tests="13" skipped="0" failures="0" errors="0"`; all 12 vector names +
    `   boundaryFixtureMetadata()` appear as individual `testcase` entries
- `git ls-files -- '*unicode_boundary.json'` — exactly 2 paths; `git ls-files -- '*data.json'` —
    exactly 5; no tracked copy added
- `uv run pytest tests/test_vendored_fixtures.py -q` — **8 passed** (drift gate green,
    `VENDORED_COPIES` untouched)
- `git status --porcelain -- 'crates/*/src' .crap-baseline.json .iai-baseline.json   packages/dotnet/Iscc.Lib/NativeMethods.g.cs packages/kotlin/src/main`
    — empty (no core source, generated binding, or baseline moved)
- `git status --porcelain` — only the 8 intended files plus the runner's `iterations.jsonl`; all
    `bin/`, `obj/`, `build/`, `target/` outputs gitignored
- `uv run scripts/check_docs_nav.py` — `OK: 23 documentation pages consistent`
- `cargo test -p iscc-lib` — 281 + 28 + 22 + 4 + 1 passed, 0 failed;
    `cargo clippy --workspace --all-targets -- -D warnings` clean
- `mise run check` — all pre-commit hooks Passed; pre-push stage
    (`uv run prek run --hook-stage pre-push --all-files`) also all Passed (Rust tests, ty, security
    scan, complexity, Ruby lint, pytest)

**Next:** Per next.md's own analysis the remaining criterion-3 surfaces are Swift (no toolchain in
this container — needs a CI-verified slice and a tracked vendored copy registered in
`VENDORED_COPIES`), C FFI (needs a generated vector table or JSON reader in `test_iscc.c`), and C++
(no cmake here). Criterion 4 — the full-code-space + sequence-class differential sweep — is the only
piece with no partial credit and protects every future table bump; it is a reasonable next step
before the expensive surfaces.

**Notes:**

- The Kotlin suite deliberately uses `@TestFactory`/`DynamicTest` rather than the plain `@Test` loop
    style of `ConformanceTest.kt`: a looping `@Test` reports only 3 `testcase` entries in the Gradle
    XML, which would make next.md's `tests="13"` criterion unmeetable and hide per-vector failure
    names. This matches the iteration-153 Java twin.
- The metadata guard in both suites pins case counts (7/5), so a truncated fixture cannot silently
    degrade the parameterized runs (xunit additionally errors on empty `MemberData`).
- C# `EnumerateObject().Count()` (LINQ) is used for the count assertions — `GetPropertyCount()` is
    .NET 9+ and the project targets net8.0.
- No API, hot path, baseline, dependency manifest, or CI file was touched; the semver, iai, CRAP,
    and audit gates are inert for this diff. CI's `dotnet` and `kotlin` jobs pick the new tests up
    automatically (whole-assembly `dotnet test`; Gradle test discovery).
- The freshness hazard from next.md did not materialize: both natives rebuilt as fast no-ops before
    testing, and the discriminating `U+A7F1` vectors pass on both surfaces.
