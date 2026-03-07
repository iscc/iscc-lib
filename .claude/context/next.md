# Next Work Package

## Step: Add .NET project version to version sync pipeline

## Goal

Add the .NET `Iscc.Lib.csproj` project version to the centralized version sync system so that
`mise run version:sync` and `mise run version:check` keep the .NET package version consistent with
the workspace version from root `Cargo.toml`. This is a prerequisite for NuGet publishing.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — add `<Version>0.2.0</Version>` to PropertyGroup
    - `scripts/version_sync.py` — add .NET sync target (get/sync functions + TARGETS entry), update
        module docstring to list the new target
- **Reference**:
    - `scripts/version_sync.py` — existing sync target pattern (get/sync function pairs)
    - `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — current .csproj structure

## Not In Scope

- NuGet publish pipeline (`release.yml` changes) — that's a separate, larger step
- SafeHandles.cs extraction — cosmetic refactor, not blocking anything
- Adding `<Version>` to the test project (`Iscc.Lib.Tests.csproj`) — test projects aren't published
- Adding NuGet packaging metadata (`<Authors>`, `<Description>`, `<PackageLicenseExpression>`,
    `<RepositoryUrl>`, etc.) — save for the NuGet publish step

## Implementation Notes

Follow the existing pattern in `version_sync.py`:

1. **`.csproj` Version property**: Add `<Version>0.2.0</Version>` inside the existing
    `<PropertyGroup>`. The .NET SDK uses this property for both assembly version and NuGet package
    version.

2. **Get function** (`_get_csproj_version`): Extract version from `<Version>X.Y.Z</Version>` using
    regex. Pattern: `r'<Version>(\d+\.\d+\.\d+)</Version>'`.

3. **Sync function** (`_sync_csproj`): Replace the version string inside `<Version>` tags. Pattern:
    `r'(<Version>)\d+\.\d+\.\d+(</Version>)'` → `rf'\g<1>{version}\2'`.

4. **TARGETS entry**: Add
    `("packages/dotnet/Iscc.Lib/Iscc.Lib.csproj", _get_csproj_version, _sync_csproj)` to the
    TARGETS list.

5. **Docstring**: Add `- packages/dotnet/Iscc.Lib/Iscc.Lib.csproj — .NET package version` to the
    module docstring's "Synced targets" list.

## Verification

- `uv run scripts/version_sync.py --check` exits 0 and output includes
    `OK: packages/dotnet/Iscc.Lib/Iscc.Lib.csproj = 0.2.0`
- `grep '<Version>0.2.0</Version>' packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` exits 0
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds
- `cargo clippy --workspace --all-targets --exclude iscc-rb -- -D warnings` clean

## Done When

All verification criteria pass — `version:check` reports .NET version as OK, the .csproj builds, and
lint is clean.
