## 2026-03-05 — Add .NET CI job to ci.yml

**Done:** Added a `dotnet` CI job to `.github/workflows/ci.yml` that builds the FFI shared library
and runs the .NET smoke tests. The job follows the same pattern as existing CI jobs (checkout, rust
toolchain, rust cache, language setup, build, test) and is placed after the `c-ffi` job.

**Files changed:**

- `.github/workflows/ci.yml`: Added `dotnet` job with 7 steps — checkout, rust-toolchain,
    rust-cache, setup-dotnet (v8.0), `cargo build -p iscc-ffi`, `dotnet build` (test .csproj which
    pulls in library), and `dotnet test` with `-e LD_LIBRARY_PATH` flag

**Verification:**

- YAML validation passes (PyYAML `safe_load` exits 0)
- Job structure verified: `dotnet` job present with correct name, runs-on, and all 7 steps
- `dotnet test` command uses `-e LD_LIBRARY_PATH=${{ github.workspace }}/target/debug` (absolute
    path via GHA expression)
- Local smoke: `cargo build -p iscc-ffi` succeeds, `dotnet test` with absolute `LD_LIBRARY_PATH`
    passes (1 test, 0 failures)
- `mise run check` passes — all 15 pre-commit hooks green

**Next:** Expand the P/Invoke surface — either via csbindgen auto-generation of `NativeMethods.g.cs`
from `iscc.h`, or by manually adding idiomatic C# wrappers for gen functions with record result
types. csbindgen approach recommended first as it creates the complete P/Invoke layer.

**Notes:**

- Local `dotnet` is installed at `$HOME/.dotnet` (not on PATH by default in devcontainer). CI uses
    `actions/setup-dotnet@v4` which handles PATH setup automatically.
- Relative `LD_LIBRARY_PATH=target/debug` does NOT work with `dotnet test -e` — the vstest host
    child process resolves it relative to its own CWD. Absolute path is required. CI uses
    `${{ github.workspace }}/target/debug` which expands to an absolute path.
- The CI workflow now has 12 jobs: version-check, rust, python-test, python, nodejs, wasm, c-ffi,
    dotnet, java, go, ruby, bench.
