## 2026-03-07 — Add .NET project version to version sync pipeline

**Done:** Added `<Version>0.2.0</Version>` to the .NET `Iscc.Lib.csproj` PropertyGroup and
integrated it into `scripts/version_sync.py` with `_get_csproj_version` / `_sync_csproj` functions
and a TARGETS entry. Updated the module docstring to list the new target.

**Files changed:**

- `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj`: Added `<Version>0.2.0</Version>` element
- `scripts/version_sync.py`: Added `_get_csproj_version`, `_sync_csproj` functions, TARGETS entry,
    and docstring update

**Verification:**

- `uv run scripts/version_sync.py --check` exits 0, output includes
    `OK: packages/dotnet/Iscc.Lib/Iscc.Lib.csproj = 0.2.0`
- `grep '<Version>0.2.0</Version>' packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` exits 0
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` succeeds (0 warnings, 0 errors)
- `cargo clippy --workspace --all-targets --exclude iscc-rb -- -D warnings` clean
- `mise run check` — all 15 pre-commit hooks pass

**Next:** NuGet publish pipeline in `release.yml` — add `nuget` boolean input, build/pack/publish
jobs, and smoke test. This is the final piece for .NET publishing.

**Notes:** Straightforward change following the existing pattern. No surprises. The regex patterns
match the exact `.csproj` XML format. No tests needed beyond the verification commands — the version
sync script itself is tested by CI's `version-check` job which runs `--check` on every push.
