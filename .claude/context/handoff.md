## 2026-03-05 — Review of: .NET project scaffold with ConformanceSelftest P/Invoke

**Verdict:** PASS

**Summary:** Clean, well-scoped scaffold establishing the `packages/dotnet/` project structure and
proving end-to-end P/Invoke into the Rust FFI library. `ConformanceSelftest()` calls
`iscc_conformance_selftest` via `[DllImport]` and returns `true`. All verification criteria pass,
code is minimal and correct, no gate circumvention.

**Verification:**

- [x] `cargo build -p iscc-ffi` succeeds — builds `libiscc_ffi.so`
- [x] `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds — 0 warnings, 0 errors
- [x] `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=target/debug` — 1 passed, 0
    failed. `ConformanceSelftest()` correctly calls into Rust FFI and returns `true`
- [x] `.devcontainer/Dockerfile` contains .NET 8 SDK installation via Microsoft install script
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets --exclude iscc-rb -- -D warnings` — clean

**Issues found:**

- (none)

**Codex review:** One P2 finding — floating NuGet package versions (`17.*`, `2.*`) in the test
`.csproj` make restores non-deterministic. Advisory: acceptable for the scaffold phase since these
match the next.md template exactly. Should be pinned when adding full conformance tests and CI job.

**Next:** Two parallel tracks for the next step: (1) Set up `csbindgen` auto-generation of
`NativeMethods.g.cs` from `iscc.h` to create the full P/Invoke layer, OR (2) Add idiomatic C#
wrappers for gen functions with record result types. Track (1) is recommended first — it creates the
complete P/Invoke surface that wrappers build on. The CI job (`dotnet` in `ci.yml`) should also be
added soon since the scaffold now has a working test.

**Notes:**

- `dotnet test` requires `-e LD_LIBRARY_PATH=<path>` to pass the library path to the vstest host
    child process. Shell-level `LD_LIBRARY_PATH` alone is insufficient. CI will need
    `env: LD_LIBRARY_PATH: ${{ github.workspace }}/target/debug` on the test step.
- .NET SDK installed at runtime in current devcontainer (`$HOME/.dotnet`). The Dockerfile change
    takes effect on next container rebuild.
- `IsccLib` class is declared `partial` — ready for generated `NativeMethods` in a future step.
- Pin NuGet versions (`Microsoft.NET.Test.Sdk`, `xunit`, `xunit.runner.visualstudio`) to specific
    versions when solidifying the test project.
