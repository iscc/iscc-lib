# Next Work Package

## Step: .NET project scaffold with ConformanceSelftest P/Invoke

## Goal

Establish the `packages/dotnet/` project structure and prove end-to-end P/Invoke into the Rust FFI
library by calling `iscc_conformance_selftest()` from C#. This validates the fundamental technology
stack (P/Invoke → iscc-ffi shared library) before expanding to all 32 symbols. Part of the
"Implement C# / .NET bindings via csbindgen" `normal` issue.

## Scope

- **Create**:
    - `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — .NET 8 class library
    - `packages/dotnet/Iscc.Lib/IsccLib.cs` — public static class with `ConformanceSelftest()` method
        and inline `[DllImport]` P/Invoke declaration for `iscc_conformance_selftest`
    - `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` — xUnit test project (excluded from
        3-file limit)
    - `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — single test asserting `ConformanceSelftest()`
        returns `true` (excluded from 3-file limit)
- **Modify**:
    - `.devcontainer/Dockerfile` — add .NET 8 SDK installation
- **Reference**:
    - `.claude/context/specs/dotnet-bindings.md` — full spec for package structure, API design, CI
    - `crates/iscc-ffi/include/iscc.h` — C FFI header (source of truth for P/Invoke signatures)
    - `crates/iscc-ffi/src/lib.rs` — Rust FFI source (for understanding function signatures)

## Not In Scope

- Full `csbindgen` binding generation of all P/Invoke declarations — separate step after scaffold
    works; this step uses one manual `[DllImport]` declaration to prove the pipeline
- Idiomatic C# wrappers for the other 31 Tier 1 symbols — future step
- Result record types (`MetaCodeResult`, `DataCodeResult`, etc.) — future step
- SafeHandle subclasses and memory management patterns — future step
- Streaming types (`IsccDataHasher`, `IsccInstanceHasher`) — future step
- Conformance tests against `data.json` — future step (this step only tests selftest)
- `Directory.Build.props` shared build properties — add when needed
- `.sln` solution file — `dotnet test` works with project files directly
- CI job in `ci.yml` — future step
- Release pipeline / NuGet packaging — future step
- Version sync integration — future step
- Documentation (`docs/howto/dotnet.md`, README C# section) — future step

## Implementation Notes

### .NET SDK in Dockerfile

The Dockerfile uses `node:20-bookworm` (Debian 12). .NET 8 SDK is NOT in default Debian repos. Use
Microsoft's install script for a clean, version-pinned install. Place in the root section (before
`USER $USERNAME`) since it installs system-wide:

```dockerfile
# Install .NET SDK 8 (for C# bindings)
RUN curl -fsSL https://dot.net/v1/dotnet-install.sh | bash -s -- --channel 8.0 --install-dir /usr/share/dotnet
ENV DOTNET_ROOT=/usr/share/dotnet
ENV PATH="$DOTNET_ROOT:$PATH"
```

### Runtime .NET installation

Since the devcontainer won't be rebuilt during this CID iteration, the advance agent must install
.NET SDK at runtime before testing:

```bash
curl -fsSL https://dot.net/v1/dotnet-install.sh | bash -s -- --channel 8.0 --install-dir $HOME/.dotnet
export DOTNET_ROOT=$HOME/.dotnet
export PATH="$DOTNET_ROOT:$PATH"
dotnet --version  # verify
```

### Class library `.csproj`

Minimal SDK-style project targeting .NET 8:

```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
    <PackageId>Iscc.Lib</PackageId>
    <RootNamespace>Iscc.Lib</RootNamespace>
    <AssemblyName>Iscc.Lib</AssemblyName>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
  </PropertyGroup>
</Project>
```

### P/Invoke for ConformanceSelftest

The C declaration in `iscc.h`:

```c
bool iscc_conformance_selftest(void);
```

The C# P/Invoke mapping:

```csharp
using System.Runtime.InteropServices;

namespace Iscc.Lib;

/// <summary>ISCC library — ISO 24138:2024 International Standard Content Code.</summary>
public static partial class IsccLib
{
    [DllImport("iscc_ffi", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.U1)]
    private static extern bool iscc_conformance_selftest();

    /// <summary>Run all conformance tests against vendored test vectors.</summary>
    public static bool ConformanceSelftest() => iscc_conformance_selftest();
}
```

Key details:

- DLL name `"iscc_ffi"` matches `libiscc_ffi.so` (Linux), `iscc_ffi.dll` (Windows),
    `libiscc_ffi.dylib` (macOS) — .NET resolves platform-specific names automatically
- `[return: MarshalAs(UnmanagedType.U1)]` for C `bool` → C# `bool` marshaling
- `CallingConvention.Cdecl` matches Rust's `extern "C"`

### Test project `.csproj`

```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
    <Nullable>enable</Nullable>
    <ImplicitUsings>enable</ImplicitUsings>
    <IsPackable>false</IsPackable>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="Microsoft.NET.Test.Sdk" Version="17.*" />
    <PackageReference Include="xunit" Version="2.*" />
    <PackageReference Include="xunit.runner.visualstudio" Version="2.*" />
  </ItemGroup>
  <ItemGroup>
    <ProjectReference Include="..\Iscc.Lib\Iscc.Lib.csproj" />
  </ItemGroup>
</Project>
```

### Running the test

The Rust FFI shared library must be built first and discoverable via `LD_LIBRARY_PATH`:

```bash
cargo build -p iscc-ffi
LD_LIBRARY_PATH=target/debug dotnet test packages/dotnet/Iscc.Lib.Tests/
```

## Verification

- `cargo build -p iscc-ffi` succeeds (builds the native library)
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds (compiles the C# library)
- `LD_LIBRARY_PATH=target/debug dotnet test packages/dotnet/Iscc.Lib.Tests/` passes
    (ConformanceSelftest returns true)
- `.devcontainer/Dockerfile` contains `dotnet` installation commands

## Done When

All verification criteria pass — `dotnet test` calls `iscc_conformance_selftest()` via P/Invoke into
the Rust FFI shared library and the test passes.
