## 2026-03-05 — Review of: Add .NET CI job to ci.yml

**Verdict:** PASS

**Summary:** Clean, minimal addition of the `dotnet` CI job to `ci.yml`. The 18-line YAML block
follows existing CI job patterns exactly (checkout, rust-toolchain, rust-cache, language setup,
build, test). All 6 verification criteria pass — YAML valid, job structure correct, local smoke test
green, pre-commit hooks pass.

**Verification:**

- [x] YAML validation — `yaml.safe_load` exits 0
- [x] `dotnet` job present with correct name (`C# / .NET (dotnet build, test)`),
    `runs-on:   ubuntu-latest`, and all 7 steps
- [x] Steps match spec: checkout, rust-toolchain, rust-cache, setup-dotnet (8.0),
    `cargo build -p   iscc-ffi`, `dotnet build` (test csproj), `dotnet test`
- [x] `dotnet test` uses `-e LD_LIBRARY_PATH=${{ github.workspace }}/target/debug` (absolute path)
- [x] Local smoke: `cargo build -p iscc-ffi` + `dotnet test` passes (1 test, 0 failures)
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets --exclude iscc-rb -- -D warnings` — clean

**Issues found:**

- (none)

**Codex review:** One P1 finding dismissed — Codex flagged `dotnet-version: '8.0'` as potentially
invalid, suggesting `'8.0.x'` instead. This is incorrect: `actions/setup-dotnet@v4` accepts channel
versions like `'8.0'` and resolves to the latest `8.0.x` SDK. The format is documented and widely
used. No change needed.

**Next:** Expand the P/Invoke surface. Recommended approach: add `csbindgen` to auto-generate
`NativeMethods.g.cs` from `iscc.h`, giving the complete P/Invoke layer for all FFI functions. Then
build idiomatic C# wrappers on top. Alternatively, manual P/Invoke declarations can be written
incrementally (start with gen functions that have simple signatures).

**Notes:**

- CI workflow now has 12 jobs (version-check, rust, python-test, python, nodejs, wasm, c-ffi,
    dotnet, java, go, ruby, bench). The `dotnet` job is placed after `c-ffi` since both build
    `iscc-ffi`.
- `dotnet test -e LD_LIBRARY_PATH` with absolute path is critical — relative paths fail because
    vstest spawns a child process with a different CWD.
- The .NET issue in issues.md is partially addressed (CI job done). The larger issue (csbindgen,
    wrappers, conformance tests, release pipeline, version sync, docs) remains.
