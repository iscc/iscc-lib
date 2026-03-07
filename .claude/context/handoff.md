## 2026-03-07 — NuGet publish pipeline in release.yml

**Done:** Added the NuGet publish pipeline to `release.yml` with 3 new jobs (`pack-nuget`,
`test-nuget`, `publish-nuget`) following the existing build→smoke-test→publish pattern. Extended
`build-ffi` to also trigger on `inputs.nuget` so native libraries for all 5 platforms are reused.
Updated `.csproj` with NuGet package metadata and runtime asset includes.

**Files changed:**

- `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj`: Added NuGet metadata (Description, Authors, License,
    URLs, ReadmeFile, Tags) and `<ItemGroup>` for packaging README and `runtimes/` native libraries
- `.github/workflows/release.yml`: Added `nuget` boolean input; extended `build-ffi` `if` condition
    with `|| inputs.nuget`; added `pack-nuget` (downloads FFI artifacts, extracts shared libs into
    NuGet RID layout, runs `dotnet pack`), `test-nuget` (creates temp console app, installs from
    local source, runs conformance selftest), `publish-nuget` (version check against nuget.org API,
    `dotnet nuget push` with `NUGET_API_KEY` secret)

**Verification:**

- `grep -c 'nuget'` returns 16 (>0) — input exists
- `grep 'pack-nuget\|test-nuget\|publish-nuget'` returns 5 (>=3) — all 3 job names present
- `grep 'inputs.nuget'` returns 4 — input used in build-ffi condition + 3 new jobs
- `grep 'PackageReadmeFile'` exits 0 — NuGet metadata present
- `grep 'runtimes'` exits 0 — runtime content include present
- `grep 'NUGET_API_KEY'` exits 0 — secret reference present
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds (0 warnings, 0 errors)
- YAML validation passes (`yaml.safe_load`)
- `mise run check` — all pre-commit and pre-push hooks pass

**Next:** The .NET bindings issue is now functionally complete (NuGet pipeline was the last major
deliverable). Remaining items: SafeHandles.cs extraction (cosmetic refactor, deferred), and
NuGet.org account setup (manual human action — API key as `NUGET_API_KEY` secret, package ID
reservation). The .NET issue in issues.md can be mostly closed. Consider moving to other tracked
issues or documentation updates.

**Notes:** The `pack-nuget` job uses bash associative arrays (`declare -A`) for the target→RID and
target→lib mappings — this is fine since it runs on `ubuntu-latest` (bash 5+). The extraction step
handles both `.tar.gz` (Unix) and `.zip` (Windows) archives. The smoke test follows the exact
pattern from next.md — creates a temp console app, adds local package source, installs, runs
conformance selftest. The `find` command for copying shared libraries uses `-maxdepth 2` since
tarballs extract to a single-level directory.
