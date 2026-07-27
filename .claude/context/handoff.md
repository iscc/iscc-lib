# Handoff

## 2026-07-27 — Migrate the .NET test project to xunit v3 and Microsoft.NET.Test.Sdk 18.x

**Done:** Moved `packages/dotnet/Iscc.Lib.Tests` from `xunit` 2.\* / `Microsoft.NET.Test.Sdk` 17.\*
to `xunit.v3` 3.\* (resolved 3.2.2) / `xunit.runner.visualstudio` 3.\* (3.1.5) / Test.Sdk 18.\*
(18.8.1), keeping the floating-wildcard style. Added `<OutputType>Exe</OutputType>` (v3 test
projects are stand-alone executables). No test-source changes were needed — the whole suite is green
under the byte-identical CI invocation.

**Files changed:**

- `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj`: package swap (`xunit` → `xunit.v3` 3.\*,
    runner → 3.\*, Test.Sdk 17.\* → 18.\*) + `<OutputType>Exe</OutputType>`;
    `<TargetFramework>net8.0</TargetFramework>` unchanged
- `packages/dotnet/CLAUDE.md`: file-layout comment now says "xunit.v3 3.x, Microsoft.NET.Test.Sdk
    18.x" (was "xunit 2.x")

**Verification:** (all criteria from next.md)

- `cargo build -p iscc-ffi` exit 0, then
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug` →
    `Passed! - Failed: 0, Passed: 104, Skipped: 0, Total: 104` (floor was 85; row-level theory
    enumeration survived — 50 conformance + 12 boundary vectors report as individual results)
- `dotnet list … package`: Test.Sdk 18.8.1, xunit.runner.visualstudio 3.1.5, xunit.v3 3.2.2
- `grep -c 'Include="xunit"' …csproj` prints `0`; same file matches `Include="xunit.v3"` and
    `Include="Microsoft.NET.Test.Sdk" Version="18` and still `<TargetFramework>net8.0`
- `grep -ri 'xunit 2' packages/dotnet/` returns nothing
- `mise run check` exit 0, working tree unchanged after (18 hooks Passed)
- `.github/workflows/ci.yml` NOT touched, so the `check_ci_job_table.py` criterion is N/A (the
    always-on `CI job table parity` prek hook passed regardless)

**Next:** The next authorized major per the sequence: the JVM slice — Gradle wrapper + JUnit 6.x
across `packages/kotlin` and `crates/iscc-jni/java/pom.xml` (mind the renumbered platform artifacts
1.x→6.x and the Kotlin consumer-floor hazard), then `jni` 0.22, then `magnus` 0.8, one per step.

**Notes:**

- Neither hazard from next.md materialized. (1) `JsonElement` inside `[MemberData]` `object[]` rows
    still executes as one test case per vector under xunit v3 + VSTest — the 104 total proves it
    (fallback key-string refactor not needed, zero source edits). (2) `-e LD_LIBRARY_PATH` still
    reaches the v3 out-of-process test host — same 104 pass with only that env source, no
    `DllNotFoundException`.
- `dotnet build` reported 0 warnings — no new xunit.analyzers findings.
- No Rust source touched; CRAP baseline and `.iai-baseline.json` untouched as instructed.
- Ledger for review (I do not edit issues.md): under "Dependency review and refresh across the
    project" the xunit 3.x + Test.Sdk 18.x major is now done (this step). Still open: Gradle wrapper
    / JUnit 6.x, `jni` 0.22, `magnus` 0.8, `release.yml` action majors (release-run-gated).
