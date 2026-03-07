## 2026-03-07 — Review of: NuGet publish pipeline in release.yml

**Verdict:** PASS_WITH_NOTES

**Summary:** NuGet publish pipeline added to `release.yml` with 3 new jobs (`pack-nuget`,
`test-nuget`, `publish-nuget`) following the existing build→smoke-test→publish pattern. The
`.csproj` has correct NuGet metadata. Review fixed two bugs: (1) cross-architecture `find` pattern
that would copy wrong native libraries for arm64 targets, (2) incorrect README Include path
(`../../README.md` → `../README.md`) that caused `dotnet pack` to fail with NU5019. Both fixes
verified. The C# / .NET issue is now fully resolved — all CID-actionable items complete.

**Verification:**

- [x] `grep -c 'nuget' .github/workflows/release.yml` returns 33 (>0) — input exists
- [x] `grep 'pack-nuget\|test-nuget\|publish-nuget'` returns 5 (>=3) — all 3 job names present
- [x] `grep 'inputs.nuget'` returns 4 — input used in build-ffi condition + 3 new jobs
- [x] `grep 'PackageReadmeFile'` exits 0 — NuGet metadata present
- [x] `grep 'runtimes'` exits 0 — runtime content include present
- [x] `grep 'NUGET_API_KEY'` exits 0 — secret reference present
- [x] `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds (0 warnings, 0 errors)
- [x] `dotnet pack -c Release` succeeds — produces `Iscc.Lib.0.2.0.nupkg` (after README path fix)
- [x] YAML validation passes (`yaml.safe_load`)
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets --exclude iscc-rb -- -D warnings` clean

**Issues found:**

- **Fixed: cross-architecture find bug** — `find . -path "*/iscc-ffi-v*/*"` matched ALL extracted
    targets sharing the same lib name (e.g., both linux-x64 and linux-arm64 `libiscc_ffi.so`). The
    last `cp` wins, so arm64 targets would get x86_64 libraries. Fixed by scoping the path pattern
    to include the target name: `-path "*-${target}/*"`.
- **Fixed: README Include path** — `../../README.md` resolved to `packages/README.md` (nonexistent),
    not `packages/dotnet/README.md`. Changed to `../README.md`. Without this fix, `dotnet pack`
    fails with NU5019. The next.md spec had the wrong path.

**Codex review:** One P1 finding about the README path — confirmed real and fixed. Codex correctly
identified that `dotnet pack` would fail with NU5019 due to the incorrect relative path.

**Next:** The C# / .NET bindings issue is now fully resolved (deleted from issues.md). The next
`normal`-priority issue is the C++ idiomatic header-only wrapper. Consider starting with the
`iscc.hpp` header file and conformance tests before CI/release integration.

**Notes:** Release pipeline now has 7 registry inputs (crates-io, pypi, npm, maven, ffi, rubygems,
nuget). The `build-ffi` job is shared between FFI and NuGet pipelines. Manual human action still
needed: NuGet.org account setup (API key as `NUGET_API_KEY` secret, package ID reservation). The
SafeHandles.cs cosmetic refactor (mentioned in spec) is deferred indefinitely — no functional
impact.
