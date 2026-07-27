# Next Work Package

## Step: Migrate the .NET test project to xunit v3 and Microsoft.NET.Test.Sdk 18.x

## Goal

Take the first of the authorized dependency majors from "Dependency review and refresh across the
project" (`normal` `[human]`): move `packages/dotnet/Iscc.Lib.Tests` from `xunit` 2.\* /
`Microsoft.NET.Test.Sdk` 17.\* to `xunit.v3` 3.x / Test.Sdk 18.x, with the whole .NET conformance
suite still green under the exact `dotnet` CI invocation.

## Alternatives Considered

- **Chosen:** the xunit 3.x + Test.Sdk 18.x bump — the handoff's "Next", one manifest plus its own
    test sources, and fully verifiable locally (dotnet SDK 8.0.423 installed, nuget.org reachable).
    Both new packages declare `net8.0` support in their nuspec, so no consumer floor moves.
- **Rejected:** the Gradle wrapper 8.12.1 + JUnit 6.x major — spans two build systems
    (`packages/kotlin` and `crates/iscc-jni/java/pom.xml`), JUnit 6 renumbers the platform artifacts
    1.x→6.x, and the JVM slice carries the known consumer-floor hazard. Its own later step.

## Scope

- **Modify**: `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj`; its three test sources
    (`SmokeTests.cs`, `ConformanceTests.cs`, `UnicodeBoundaryTests.cs`) as the migration requires;
    `packages/dotnet/CLAUDE.md` (it documents "xunit 2.x"). Only if the CI invocation genuinely
    cannot stay byte-identical: `.github/workflows/ci.yml` **and** the `dotnet` row of the job table
    in `.claude/context/specs/ci-cd.md`, in the same edit.
- **Reference**: `.github/workflows/ci.yml` `dotnet` job (~lines 140-157) — the invocation that must
    keep working; `packages/dotnet/CLAUDE.md` (test patterns, P/Invoke pitfalls);
    <https://xunit.net/docs/getting-started/v3/migration>.

## Not In Scope

- `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — the published library, its `<Version>`, and its
    `net8.0` target framework stay untouched; this is a test-project-only bump.
- Raising `<TargetFramework>` or CI/release `dotnet-version: '8.0'` — a consumer floor, human-only.
    Both new packages support `net8.0`; if either turns out to demand a newer SDK, stop and report.
- Switching to Microsoft.Testing.Platform runner mode (`TestingPlatformDotnetTestSupport`,
    `UseMicrosoftTestingPlatformRunner`) — `dotnet test -e` is a VSTest feature and must survive.
- The other authorized majors (Gradle/JUnit, `jni` 0.22, `magnus` 0.8) and any dotnet edits in
    `.github/workflows/release.yml`.
- Editing `issues.md` — the entry stays until review verifies the slice.

## Implementation Notes

- Package moves (latest stable at scoping): `xunit` → `xunit.v3` (3.2.2),
    `xunit.runner.visualstudio` → 3.x (3.1.5), `Microsoft.NET.Test.Sdk` → 18.x (18.8.1). Keep the
    file's floating-wildcard style (`Version="18.*"`, `Version="3.*"`) unless restore resolves
    something unexpected. `xunit.abstractions` is not referenced here, so nothing to remove.
- v3 test projects are stand-alone executables: add `<OutputType>Exe</OutputType>`. The
    `using Xunit;` namespace is unchanged, and every `Assert` member in use (Equal, StartsWith,
    True, NotEmpty, Throws, Single, Null, NotNull, IsType, Empty, All) still exists in v3.
- **Main hazard:** `ConformanceTests` `MemberData` methods yield `JsonElement` inside `object[]`,
    which xunit v3's data serializer cannot round-trip. If v3 stops enumerating rows as individual
    test cases (or errors on them), yield the vector key `string` only and look up `DataJson` inside
    the test body — that restores one test case per vector without changing what is asserted.
- **Second hazard:** the v3 test host is a separate process. Run the CI command verbatim, including
    `-e LD_LIBRARY_PATH`. If `-e` no longer reaches the host, prefer exporting `LD_LIBRARY_PATH` in
    the CI step (and update the spec's `dotnet` row) over changing runner mode.
- `cargo build -p iscc-ffi` first — a missing/stale `target/debug/libiscc_ffi.so` shows up as
    `DllNotFoundException`, not a migration failure. There is no `Directory.Build.props` or
    `global.json`; xunit.analyzers warnings are not errors (no `TreatWarningsAsErrors`), but list
    any new warning in the handoff.
- This step touches no Rust source, so neither the CI-only CRAP baseline nor `.iai-baseline.json`
    moves — do not refresh either.

## Verification

- `cargo build -p iscc-ffi` then
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug` exits 0 with
    `Failed: 0` and **at least 85 passed** tests (row-level theory enumeration must survive).
- `grep -c 'Include="xunit"' packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` prints `0`, and
    the same file matches `Include="xunit.v3"`, `Include="Microsoft.NET.Test.Sdk" Version="18`, and
    still `<TargetFramework>net8.0</TargetFramework>`.
- `grep -ri 'xunit 2' packages/dotnet/` returns nothing (docs describe the shipped state).
- `mise run check` exits 0 and modifies no file.
- If `.github/workflows/ci.yml` was touched: `uv run scripts/check_ci_job_table.py` exits 0.

## Done When

The .NET suite runs green on xunit v3 + Test.Sdk 18.x under the unchanged CI invocation, no v2
package reference or doc claim remains, and all verification criteria pass.
