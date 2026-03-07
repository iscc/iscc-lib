## 2026-03-07 — C# streaming hasher Finalize() return types

**Done:** Changed `IsccDataHasher.Finalize()` to return `DataCodeResult` and
`IsccInstanceHasher.Finalize()` to return `InstanceCodeResult` instead of raw `string`. Updated 3
test methods in SmokeTests.cs to use `.Iscc` property on the returned records.

**Files changed:**

- `packages/dotnet/Iscc.Lib/IsccDataHasher.cs`: Return type `string` → `DataCodeResult`, docstring
    updated, wraps `ConsumeNativeString` in `new DataCodeResult(...)`
- `packages/dotnet/Iscc.Lib/IsccInstanceHasher.cs`: Return type `string` → `InstanceCodeResult`,
    docstring updated, wraps `ConsumeNativeString` in `new InstanceCodeResult(...)`
- `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs`: Updated `DataHasher_MatchesGenDataCodeV0`,
    `DataHasher_ChunkedUpdate_MatchesSingleUpdate`, and `InstanceHasher_MatchesGenInstanceCodeV0` to
    use `var` and compare `.Iscc` properties

**Verification:** All 91 .NET tests pass (41 smoke + 50 conformance). `DataCodeResult Finalize`
found once in DataHasher, `InstanceCodeResult Finalize` found once in InstanceHasher, zero
`string Finalize` signatures remain. `mise run check` passes all 15 hooks. `cargo build -p iscc-ffi`
and `dotnet build` both succeed with 0 errors.

**Next:** .NET bindings are now feature-complete for the current API surface. Priority items: (1)
Documentation — `docs/howto/dotnet.md`, `packages/dotnet/README.md`, and README C# section; (2)
NuGet publish pipeline in `release.yml`; (3) Version sync integration for .NET project version.

**Notes:** No surprises — the change was purely mechanical. The 9 pre-existing xUnit1026 warnings
about unused `vectorName` parameters in ConformanceTests remain (they serve test identification, not
worth suppressing).
