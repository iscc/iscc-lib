# Next Work Package

## Step: Propagate the Unicode boundary fixture to the C# and Kotlin test suites (slice 4)

## Goal

Advance the `normal` issue "Declare and gate a Unicode data version (DECIDED)", remainder **(b)**,
by gating the Unicode 16.0.0 freeze rule on two more native surfaces — **C# (.NET)** and **Kotlin
(UniFFI/JVM)** — taking criterion 3 from 6 of 11 to 8 of 11. Both surfaces read the *canonical*
fixture through a build-config indirection, so **no tracked vendored copy is added** and the
iteration-152 drift-gate table stays untouched.

**Deviation from state.md's "Next Milestone" (C# alone, "do not pick Kotlin yet").** That
recommendation rests on one claim that I measured false while scoping: Kotlin *is* buildable and
testable in this container. `~/.gradle/wrapper/dists/gradle-8.12.1-bin` is already unpacked and
`~/.gradle/caches` is populated from iteration 128, so after `cargo build -p iscc-uniffi` the
command `./gradlew cleanTest test --offline` in `packages/kotlin` really re-executes the suite (**9
tests, 0 failures, 6 s**, verified by the freshly written
`build/test-results/test/TEST-uniffi.iscc_uniffi.ConformanceTest.xml`). Pairing the two keeps the
proven 2-surface-per-slice cadence of iterations 150/151/153. **Swift stays deferred** — `swift` is
genuinely absent here, so it cannot be verified locally.

## Scope

- **Create**:
    - `packages/dotnet/Iscc.Lib.Tests/UnicodeBoundaryTests.cs`
    - `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/UnicodeBoundaryTest.kt`
- **Modify** (2 non-test/non-doc files, within the 3-file budget):
    - `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` — add one `<Content Include=...>` item
        that links the canonical fixture into the test output directory
    - `packages/kotlin/build.gradle.kts` — add one `systemProperty(...)` line inside the existing
        `tasks.withType<Test>` block
- **Modify** (docs, excluded from the budget):
    - `docs/unicode.md` — the "Both vector families are checked into the repository…" paragraph
        (~lines 99-101) currently names the Python, Node.js, WASM, Java and Ruby suites; add C# and
        Kotlin
    - `packages/dotnet/CLAUDE.md` and `packages/kotlin/CLAUDE.md` — both carry a "File Layout" tree
        that enumerates the test files; add the new file to each
- **Reference**:
    - `crates/iscc-lib/tests/unicode_boundary.json` — the canonical fixture (read it, never rewrite
        it)
    - `packages/dotnet/Iscc.Lib.Tests/ConformanceTests.cs` — the `System.Text.Json` loader
        (`Path.Combine(AppContext.BaseDirectory, "testdata", ...)`), the `Lazy<JsonElement>` cache and
        the `[Theory]` + `[MemberData]` `IEnumerable<object[]>` pattern to mirror
    - `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — existing `IsccLib.TextClean` /
        `IsccLib.TextCollapse` call sites (PascalCase, `string -> string`)
    - `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/ConformanceTest.kt` — gson loader + JUnit 5
        style to mirror; `packages/kotlin/build.gradle.kts` line 34 for the existing
        `${rootProject.rootDir}/../..` idiom
    - `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/UnicodeBoundaryTest.java` — the JVM
        reference implementation of this exact suite (iteration 153); the Kotlin file is its idiomatic
        twin
    - `tests/test_vendored_fixtures.py` — the drift gate; read it to confirm why this slice must
        **not** touch `VENDORED_COPIES`
    - `.claude/context/issues.md` → "Declare and gate a Unicode data version (DECIDED)"

## Not In Scope

- **Do not vendor a tracked copy of the fixture.** Neither
    `packages/dotnet/Iscc.Lib.Tests/testdata/` nor `packages/kotlin/src/test/resources/` gains a
    `unicode_boundary.json` file in the git index. `VENDORED_COPIES` in
    `tests/test_vendored_fixtures.py` stays unchanged and `git ls-files -- '*unicode_boundary.json'`
    must still return exactly 2 paths (canonical + the pure-Go copy).
- No other surface this step: C FFI, C++ and Swift are later slices. C FFI needs a JSON reader or a
    generated C vector table (`crates/iscc-ffi/tests/test_iscc.c` has neither a parser nor any
    text-function coverage); C++ has no `cmake` here; Swift has no toolchain here.
- Do **not** copy the `delete_filter_output` oracles into the binding tests — plain equality against
    `outputs.result` already reds a delete-filter regression (mutation-verified at iterations
    150/151/153), and copying oracles re-opens the iteration-149 mislabel hazard.
- Do not start criterion 4 (the 1,112,064-scalar + sequence-class differential sweep). Separate
    step.
- No changes under `crates/*/src/`, no `.crap-baseline.json` / `.iai-baseline.json` refresh. Do not
    regenerate `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` or
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` — both are generated and
    both are already current (`cargo build -p iscc-ffi` and `cargo build -p iscc-uniffi` were run
    while scoping and produced **no** tree diff).
- No dependency edits: xunit + `Microsoft.NET.Test.Sdk` and JUnit 5 + gson are already declared in
    both projects. The authorized xunit 3.x / Test.Sdk 18.x / Gradle-wrapper / JUnit 6.x majors are
    separate one-per-step items — do not bundle them here.
- Do not add or modify a CI job: the `dotnet` job already runs
    `dotnet test packages/dotnet/Iscc.Lib.Tests/` over the whole assembly and the `kotlin` job
    already runs `./gradlew test` in `packages/kotlin`; both build their native library first.
- No edits under `.claude/context/specs/` (human-owned) and do not delete the issue from issues.md —
    the review agent handles issue resolution.

## Implementation Notes

**Fixture shape (verified at HEAD — do not re-derive).** Top-level keys are `_metadata`,
`text_clean`, `text_collapse`. `_metadata.unicode_data_version == "16.0.0"`. `text_clean` has **7**
cases, `text_collapse` has **5**. Each case is
`{"inputs": ["<string>"], "outputs": {"result": "<string>"}}` — one string input, one string output.
The file is pure ASCII with JSON escapes (astral code points as surrogate pairs); `System.Text.Json`
and gson both decode them correctly and both languages use UTF-16 strings, so plain equality is
exact.

**Native artifacts: build both first.** Both were rebuilt while scoping, so a rebuild is a fast
no-op, but run it anyway — a stale native silently reds vectors.

- C#: `cargo build -p iscc-ffi`, then run tests with `LD_LIBRARY_PATH=$PWD/target/debug` (this is
    exactly what the CI job does via `dotnet test -e`).
- Kotlin: `cargo build -p iscc-uniffi`; `build.gradle.kts` already points `jna.library.path` and
    `LD_LIBRARY_PATH` at `target/debug`.
- **Discriminating freshness probe:** `text_clean` of `"a" + U+A7F1 + "b"` must return `"ab"`; a
    pre-sentinel build returns `"aSb"`. Rows containing U+0378 do **not** discriminate (U+0378 is
    `Cn` in every Unicode version). Measured through the freshly built `libiscc_ffi.so` while
    scoping: the three sequence vectors return the sentinel-design values (`e`+U+0301,
    U+1100+U+1161, `e`+U+0301), so the core under both bindings is correct.

**C# — how the fixture reaches the test.** Add to `Iscc.Lib.Tests.csproj`, next to the existing
`data.json` item:

```xml
<Content Include="..\..\..\crates\iscc-lib\tests\unicode_boundary.json" Link="testdata\unicode_boundary.json">
  <CopyToOutputDirectory>PreserveNewest</CopyToOutputDirectory>
</Content>
```

I probed this exact shape in a throwaway project at the same directory depth: MSBuild copies the
file to `bin/Debug/net8.0/testdata/unicode_boundary.json`, so
`Path.Combine(AppContext.BaseDirectory, "testdata", "unicode_boundary.json")` finds it and **no new
file enters the git index** (`packages/dotnet/.gitignore:1` ignores `bin/`). Backslash separators
are correct here — MSBuild normalizes them on Linux and the existing `data.json` item already uses
them.

Then write `UnicodeBoundaryTests.cs` in namespace `Iscc.Lib.Tests` mirroring `ConformanceTests.cs`:
a `static readonly Lazy<JsonElement>` loader, one `[Fact]` metadata guard (version `16.0.0`, 7
`text_clean` cases, 5 `text_collapse` cases — so a truncated fixture cannot silently degrade the
`[Theory]` to a zero-case run), and two `[Theory]` + `[MemberData]` methods yielding
`[caseName, caseElement]` per section, each asserting
`Assert.Equal(outputs.GetProperty("result").GetString(), IsccLib.TextClean(inputs[0].GetString()!))`
(and `TextCollapse` for the other section). **Zero skips** — C# wraps the same Rust core, so all 12
vectors must pass.

**Kotlin — how the fixture reaches the test.** Add one line inside the existing
`tasks.withType<Test>` block in `build.gradle.kts`, mirroring the `nativeLibDir` idiom two lines
above it:

```kotlin
systemProperty("iscc.fixtureDir", "${rootProject.rootDir}/../../crates/iscc-lib/tests")
```

`rootProject.rootDir` is `packages/kotlin` both locally and in CI (the CI job sets
`working-directory: packages/kotlin`), so the property resolves to the canonical fixture directory
in both. Prefer this over relying on the `Test` task's default working directory — an explicit
property fails loudly if it is ever unset instead of resolving against a surprising cwd.

Then write `UnicodeBoundaryTest.kt` in package `uniffi.iscc_uniffi`, JUnit 5 + gson like
`ConformanceTest.kt`: read the file with
`File(System.getProperty("iscc.fixtureDir") ?: error("iscc.fixtureDir not set"), "unicode_boundary.json").readText()`,
parse with `JsonParser.parseString(...).asJsonObject`, one `@Test` metadata guard (same three
assertions) and one `@Test` per section looping every case with
`assertEquals(expected, textClean(input), caseName)`. Kotlin function names are camelCase top-level
functions: `textClean(text: String): String`, `textCollapse(text: String): String` (verified in
`iscc_uniffi.kt`). `@TestFactory`/`DynamicTest` is also fine if the failure message still names the
case. **Zero skips.**

**Docs edit:** in `docs/unicode.md` the sentence "…exercised by the Rust test suite, by the Python,
Node.js, WASM, Java, and Ruby binding suites (which all read the canonical fixture directly), and by
the pure-Go package via the vendored copy…" needs C# and Kotlin added to the canonical-fixture list.
Keep the pure-Go clause intact. Run `mise run format` before committing so mdformat reflows the
paragraph.

**Environment facts measured while scoping (do not re-probe):**

- `dotnet` 8.0.423 is installed; NuGet restore works (network reachable) and takes ~4 s.
    `dotnet test packages/dotnet/Iscc.Lib.Tests/` currently reports **91 passed, 0 failed, 0
    skipped** in ~190 ms — that is the baseline the new tests add to.
- Gradle: use `--offline` locally (all deps cached); CI runs without it. `./gradlew test` alone can
    report `BUILD SUCCESSFUL` while the `test` task is **UP-TO-DATE and did not execute** — always
    use `cleanTest test` locally and confirm the timestamp on `build/test-results/test/TEST-*.xml`.
- `packages/dotnet/{bin,obj}` and `packages/kotlin/build/` are gitignored, so every build output of
    this step is invisible to `git ls-files` and to the drift gate.

## Verification

- `cargo build -p iscc-ffi` exits 0, then
    `LD_LIBRARY_PATH=$PWD/target/debug dotnet test packages/dotnet/Iscc.Lib.Tests/` exits 0 with **0
    failed, 0 skipped** and a total **≥ 104** (the 91 existing plus at least the 12 vectors and the
    metadata guard)
- `LD_LIBRARY_PATH=$PWD/target/debug dotnet test packages/dotnet/Iscc.Lib.Tests/ --filter FullyQualifiedName~UnicodeBoundary`
    exits 0 reporting **≥ 13 passed, 0 failed, 0 skipped**
- `cargo build -p iscc-uniffi` exits 0, then `./gradlew cleanTest test --offline` run from
    `packages/kotlin` exits 0, and
    `packages/kotlin/build/test-results/test/TEST-uniffi.iscc_uniffi.UnicodeBoundaryTest.xml`
    exists, is newer than the source file, and carries `failures="0" errors="0" skipped="0"` with
    `tests="13"` or more
- `git ls-files -- '*unicode_boundary.json'` returns exactly **2** paths (canonical +
    `packages/go/testdata/unicode_boundary.json`), and `git ls-files -- '*data.json'` still returns
    exactly **5** — no tracked copy was added by this step
- `uv run pytest tests/test_vendored_fixtures.py -q` — **8 passed** (drift gate green with
    `VENDORED_COPIES` untouched)
- `git status --porcelain -- 'crates/*/src' .crap-baseline.json .iai-baseline.json packages/dotnet/Iscc.Lib/NativeMethods.g.cs packages/kotlin/src/main`
    is **empty** (no core source, generated binding or baseline moved, so the CRAP, iai and semver
    gates cannot react to this diff)
- `git status --porcelain` lists no build artifact as untracked-and-stageable beyond the runner's
    own files (all `bin/`, `obj/`, `build/`, `target/` outputs are gitignored)
- `uv run scripts/check_docs_nav.py` exits 0 reporting **23** pages (prose-only docs edit, no page
    added)
- The canonical-fixture sentence in `docs/unicode.md` names **C#** and **Kotlin** in addition to
    Python, Node.js, WASM, Java and Ruby, with the pure-Go clause intact
- `mise run check` exits 0 (all prek hooks) and `git status --porcelain` is clean afterwards
- `cargo test -p iscc-lib` still passes and `cargo clippy --workspace --all-targets -- -D warnings`
    is clean (sanity — the core is untouched)

## Done When

The C# and Kotlin suites each run all 12 canonical Unicode boundary vectors plus a metadata guard
with zero skips and zero failures against freshly built native libraries, `docs/unicode.md` and both
package `CLAUDE.md` layouts name the new suites, no new tracked fixture copy exists, and every
verification command above passes.
