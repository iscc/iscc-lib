## 2026-03-05 — .NET project scaffold with ConformanceSelftest P/Invoke

**Done:** Created the `packages/dotnet/` project structure with an `Iscc.Lib` class library and
`Iscc.Lib.Tests` xUnit test project. The library exposes `IsccLib.ConformanceSelftest()` via a
manual `[DllImport]` P/Invoke declaration targeting `iscc_conformance_selftest` in `libiscc_ffi.so`.
Added .NET 8 SDK installation to the Dockerfile.

**Files changed:**

- `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj`: SDK-style .NET 8 class library project
- `packages/dotnet/Iscc.Lib/IsccLib.cs`: Public static class with P/Invoke for
    `iscc_conformance_selftest` and `ConformanceSelftest()` wrapper method
- `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj`: xUnit test project referencing the class
    library
- `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs`: Single test asserting `ConformanceSelftest()`
    returns `true`
- `.devcontainer/Dockerfile`: Added .NET 8 SDK installation via Microsoft's install script
    (system-wide under `/usr/share/dotnet`)

**Verification:**

- `cargo build -p iscc-ffi` — succeeds (builds `libiscc_ffi.so`)
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — succeeds (0 warnings, 0 errors)
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=.../target/debug` — 1 passed, 0
    failed. `ConformanceSelftest()` correctly calls into Rust FFI and returns `true`
- `mise run check` — all 15 pre-commit hooks pass
- Pre-push hooks — all pass (clippy, cargo test, pytest, security scan, etc.)

**Next:** Expand the .NET bindings to cover all 32 Tier 1 symbols. The next step should either (a)
set up csbindgen auto-generation of `NativeMethods.g.cs` from `iscc.h`, or (b) add idiomatic C#
wrappers for the gen functions with record result types. Option (a) is recommended first since it
creates the full P/Invoke layer that wrappers build on. CI job for .NET should also be added.

**Notes:**

- The `dotnet test` command requires `-e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug` to pass
    the library path to the vstest host process. Just setting `LD_LIBRARY_PATH` in the shell
    environment is not sufficient — the test host spawns a child process that doesn't inherit it. CI
    will need `env: LD_LIBRARY_PATH: ${{ github.workspace }}/target/debug` on the test step.
- .NET 8 SDK is installed at runtime in the current devcontainer (via `dotnet-install.sh` to
    `$HOME/.dotnet`). The Dockerfile change will take effect on next container rebuild.
- No `.sln` solution file was created per next.md — `dotnet test` works with project files directly.
