# Define-Next Agent Memory — Archive

Archived scoping decisions from completed project phases. Moved here to reduce per-invocation
context loading. Full history preserved in git.

See MEMORY.md for current active entries.

## C# / .NET Binding Details (archived iteration 10 — phase complete)

- Three layers: C FFI → P/Invoke (NativeMethods.g.cs, csbindgen) → Idiomatic wrapper (IsccLib.cs)
- Package lives in `packages/dotnet/`, DLL name `"iscc_ffi"`
- csbindgen in build.rs, NativeMethods.g.cs committed (929 lines, 47 externs, 6 structs)
- Wrapper scoping: batched by marshaling complexity (string→string → byte[]→string → structs →
    arrays → streaming IDisposable)
- Jagged array marshaling: GCHandle.Alloc per inner array + fixed on outer + finally cleanup
- SafeHandle + IDisposable for IsccDataHasher / IsccInstanceHasher
- 91 tests (41 smoke + 50 conformance), System.Text.Json for data.json parsing
- NuGet pipeline: pack-nuget → test-nuget → publish-nuget in release.yml
- Cross-arch find bug fix: scope path pattern by target name `-path "*-${target}/*"`
- .csproj relative paths: count `../` from csproj location, not project root
