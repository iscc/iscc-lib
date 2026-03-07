# Next Work Package

## Step: NuGet publish pipeline in release.yml

## Goal

Add the NuGet publish pipeline to `release.yml` so that `Iscc.Lib` can be published to nuget.org
with bundled native libraries for 5 platforms. This is the last major deliverable for the C# / .NET
bindings issue.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — add NuGet package metadata (Description, Authors,
        License, PackageReadmeFile, RepositoryUrl, PackageTags) and `runtimes/**` content include for
        native library bundling
    - `.github/workflows/release.yml` — add `nuget` boolean input to `workflow_dispatch`; extend
        `build-ffi` job's `if` condition to also trigger on `inputs.nuget`; add 3 new jobs:
        `pack-nuget`, `test-nuget`, `publish-nuget`
- **Reference**:
    - `.claude/context/specs/dotnet-bindings.md` — spec for distribution/publishing section
    - `.github/workflows/release.yml` — existing pipeline patterns (build-ffi, test-ffi, publish-ffi;
        build-jni, test-jni, publish-maven) as templates
    - `packages/dotnet/README.md` — the per-package README to reference in PackageReadmeFile

## Not In Scope

- **SafeHandles.cs extraction** — cosmetic refactor moving SafeHandle subclasses to a separate file;
    deferred as it has no functional impact
- **NuGet.org account setup** — manual human action (API key as `NUGET_API_KEY` secret, package
    reservation); the pipeline should be built assuming the secret exists
- **OIDC trusted publishing for NuGet** — NuGet doesn't natively support OIDC like PyPI/crates.io;
    use `NUGET_API_KEY` secret per spec
- **Testing the actual publish** — we can't test the publish without the API key; verification
    focuses on pipeline structure and local smoke test pattern
- **Multi-targeting (.NET 6, .NET 9)** — keep `net8.0` only for now

## Implementation Notes

### .csproj NuGet metadata

Add to the existing `<PropertyGroup>`:

```xml
<Description>High-performance ISCC (ISO 24138) library for .NET</Description>
<Authors>ISCC Foundation</Authors>
<PackageLicenseExpression>Apache-2.0</PackageLicenseExpression>
<PackageProjectUrl>https://lib.iscc.codes</PackageProjectUrl>
<RepositoryUrl>https://github.com/iscc/iscc-lib</RepositoryUrl>
<PackageReadmeFile>README.md</PackageReadmeFile>
<PackageTags>iscc;content-identification;hash;fingerprint;iso-24138</PackageTags>
```

Add an `<ItemGroup>` for packaging:

```xml
<ItemGroup>
  <None Include="../../README.md" Pack="true" PackagePath="" />
  <Content Include="runtimes/**" Pack="true" PackagePath="runtimes" CopyToOutputDirectory="PreserveNewest" Condition="Exists('runtimes')" />
</ItemGroup>
```

The `README.md` path is relative: `../../README.md` = `packages/dotnet/README.md` (the per-package
README, not the root README). The `runtimes/` directory only exists during CI release builds. The
`Condition` prevents build errors when it's absent during local dev.

### release.yml changes

**1. Add `nuget` input** (after `rubygems`):

```yaml
nuget:
  description: Publish Iscc.Lib to NuGet
  type: boolean
  default: false
```

**2. Extend `build-ffi` condition** to also trigger for NuGet:

```yaml
if: startsWith(github.ref, 'refs/tags/v') || inputs.ffi || inputs.nuget
```

This reuses existing FFI cross-compilation for all 5 platforms without duplication.

**3. Add `pack-nuget` job** (needs: `build-ffi`):

- Download all 5 FFI artifacts via `actions/download-artifact@v4` with `pattern: ffi-*`
- Extract shared libraries from tarballs/zips. Handle both `.tar.gz` (Unix) and `.zip` (Windows)
- Map Rust targets to NuGet RIDs and copy shared libs:
    - `x86_64-unknown-linux-gnu` → `runtimes/linux-x64/native/libiscc_ffi.so`
    - `aarch64-unknown-linux-gnu` → `runtimes/linux-arm64/native/libiscc_ffi.so`
    - `aarch64-apple-darwin` → `runtimes/osx-arm64/native/libiscc_ffi.dylib`
    - `x86_64-apple-darwin` → `runtimes/osx-x64/native/libiscc_ffi.dylib`
    - `x86_64-pc-windows-msvc` → `runtimes/win-x64/native/iscc_ffi.dll`
- All `runtimes/` subdirs go under `packages/dotnet/Iscc.Lib/runtimes/`
- Setup .NET SDK 8 via `actions/setup-dotnet@v4`
- `dotnet pack -c Release -o nupkg` in `packages/dotnet`
- Upload `.nupkg` as artifact `nuget-package`

**4. Add `test-nuget` smoke test job** (needs: `pack-nuget`):

Follow the same pattern as other smoke tests. On `ubuntu-latest`:

```bash
# Create temp console app
dotnet new console -o smoke
cd smoke
# Add local package source pointing to the downloaded nupkg
dotnet nuget add source "$PWD/../nupkg" -n local
dotnet add package Iscc.Lib --source local
# Write smoke test
cat > Program.cs << 'EOF'
using Iscc.Lib;
if (!IsccLib.ConformanceSelftest())
    throw new Exception("Conformance selftest failed");
Console.WriteLine("NuGet smoke test passed");
EOF
dotnet run
```

**5. Add `publish-nuget` job** (needs: `pack-nuget`, `test-nuget`):

- Get workspace version from `Cargo.toml`
- Version check against nuget.org API for idempotency:
    `curl -sf "https://api.nuget.org/v3-flatcontainer/iscc.lib/$VERSION/iscc.lib.nuspec"` — if this
    succeeds, version is already published, set `skip=true`
- Download `.nupkg` artifact
- `dotnet nuget push *.nupkg --api-key $NUGET_API_KEY --source https://api.nuget.org/v3/index.json`
- Uses `secrets.NUGET_API_KEY`

### Windows artifact handling

The `build-ffi` Windows job produces a `.zip` (not `.tar.gz`). The `pack-nuget` job runs on
`ubuntu-latest` and must handle both: `tar xzf` for `.tar.gz` files, `unzip` for `.zip` files. The
extraction step should iterate over downloaded artifacts and use the correct tool per extension.

### README path for PackageReadmeFile

The `<PackageReadmeFile>README.md</PackageReadmeFile>` requires the file to be included as a
`<None Include="..." Pack="true" PackagePath="">` item. The path `../../README.md` resolves from the
`.csproj` location (`packages/dotnet/Iscc.Lib/`) to `packages/dotnet/README.md`.

## Verification

- `grep -c 'nuget' .github/workflows/release.yml | grep -v '^0$'` — `nuget` input exists
- `grep 'pack-nuget\|test-nuget\|publish-nuget' .github/workflows/release.yml | wc -l` returns at
    least 3 — all 3 job names present
- `grep 'inputs.nuget' .github/workflows/release.yml | wc -l` returns at least 4 — input used in
    build-ffi condition + 3 new jobs
- `grep 'PackageReadmeFile' packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` exits 0 — NuGet metadata
    present
- `grep 'runtimes' packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` exits 0 — runtime content include
    present
- `grep 'NUGET_API_KEY' .github/workflows/release.yml` exits 0 — secret reference present
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds — .csproj changes don't break
    local build (runtimes/ absent is OK)
- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0 — valid
    YAML

## Done When

All verification criteria pass — release.yml has `nuget` input with pack/test/publish jobs following
the existing pipeline pattern, and the .csproj has NuGet package metadata with runtime asset
includes.
