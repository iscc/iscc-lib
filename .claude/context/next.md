# Next Work Package

## Step: Add .NET CI job to ci.yml

## Goal

Add a `dotnet` CI job that builds the FFI shared library and runs the .NET smoke tests, validating
the scaffold end-to-end in CI before expanding the P/Invoke surface. This unblocks all future .NET
changes by giving them CI coverage from day one. Part of the "Implement C# / .NET bindings via
csbindgen" `normal` issue.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml` (add `dotnet` job)
- **Reference**: `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj`,
    `packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj`, `packages/dotnet/Iscc.Lib/IsccLib.cs`,
    `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs`

## Not In Scope

- Expanding the P/Invoke surface (csbindgen, NativeMethods.g.cs) — that's the next step after CI
- Adding idiomatic C# wrappers or record result types
- Adding conformance tests against `data.json`
- Pinning NuGet package versions — advisory for now, pin when adding conformance tests
- Adding `dotnet` to `release.yml` — no NuGet publishing pipeline yet
- Version sync integration for .NET project version
- Documentation (`docs/howto/dotnet.md`, README C# section)

## Implementation Notes

Follow the pattern of existing CI jobs (especially `c-ffi` which also builds `iscc-ffi`):

**Job structure**: Name `dotnet`, display name `C# / .NET (dotnet build, test)`, runs on
`ubuntu-latest`. Place after the `c-ffi` job in the YAML since it also depends on the FFI crate.

**Steps** (in order):

1. `actions/checkout@v4`
2. `dtolnay/rust-toolchain@stable` (for building `iscc-ffi`)
3. `Swatinem/rust-cache@v2`
4. `actions/setup-dotnet@v4` with `dotnet-version: '8.0'` — use the official GHA action (NOT the
    Microsoft install script used in the devcontainer Dockerfile)
5. `cargo build -p iscc-ffi` — step name: "Build FFI native library" — builds `libiscc_ffi.so` to
    `target/debug/`
6. `dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` — step name: "Build .NET
    projects" — builds both library and test projects since test references library
7. `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=${{ github.workspace }}/target/debug`
    — step name: "Run .NET tests"

**Key detail from learnings**: `dotnet test` requires the `-e LD_LIBRARY_PATH=<path>` flag to pass
the library path to the vstest host child process. Shell-level `env:` on the step alone is NOT
sufficient because dotnet's test host spawns a child process that doesn't inherit the shell env
vars. Use the `-e` flag directly on the `dotnet test` command.

**CI env path**: Use `${{ github.workspace }}/target/debug` for the absolute path (not relative).

## Verification

- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` exits 0 (valid YAML)
- `.github/workflows/ci.yml` contains a job named `dotnet`
- The `dotnet` job includes steps: checkout, rust-toolchain, rust-cache, setup-dotnet,
    `cargo build -p iscc-ffi`, `dotnet build`, and `dotnet test`
- `dotnet test` command uses `-e LD_LIBRARY_PATH` flag
- Local smoke:
    `cargo build -p iscc-ffi && dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=target/debug`
    passes (1 test, 0 failures)
- `mise run check` passes (pre-commit hooks including YAML validation)

## Done When

All verification criteria pass — the `dotnet` CI job is syntactically correct, follows existing CI
patterns, and the same build+test sequence succeeds locally.
