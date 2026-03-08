# Next Work Package

## Step: Fix stale .NET docs claiming NuGet is unavailable

## Goal

Update `docs/howto/dotnet.md` to remove the misleading "NuGet publishing is not yet available" note
block (lines 21-39). The NuGet publish pipeline (`pack-nuget` / `test-nuget` / `publish-nuget`)
already exists in `release.yml`, so the documentation is stale and actively misleading developers.

## Scope

- **Create**: (none)
- **Modify**: `docs/howto/dotnet.md`
- **Reference**: `docs/howto/dotnet.md` (current content), `.github/workflows/release.yml` (to
    confirm NuGet pipeline exists)

## Not In Scope

- Adding NuGet.org account setup instructions (that's an infrastructure task, not a docs fix)
- Updating `packages/dotnet/README.md` (it does not have this stale claim)
- Fixing the Conan recipe or "View as Markdown" issues (separate normal-priority issues)
- Adding version badges or other README enhancements

## Implementation Notes

Current lines 21-39 contain a `!!! note "Build from source"` admonition that says "NuGet publishing
is not yet available" and provides build-from-source instructions as the primary install path.

**Fix approach:**

1. Remove the `!!! note "Build from source"` admonition block entirely (lines 21-39)
2. Replace it with a `??? tip "Build from source"` collapsible admonition (collapsed by default)
    that keeps the build-from-source instructions for contributors who want to build from source,
    but reframes it as an optional developer workflow rather than the primary install method
3. The `dotnet add package Iscc.Lib` command on line 18 should stand alone as the primary
    installation instruction (no warning/caveat note undermining it)

**Admonition syntax reference** (MkDocs Material):

- `!!! note "Title"` = always-open note
- `??? tip "Title"` = collapsible, closed by default
- `???+ tip "Title"` = collapsible, open by default

Use `??? tip "Build from source"` (collapsed by default) to keep the build instructions accessible
but de-emphasized.

## Verification

- `grep -c "not yet available" docs/howto/dotnet.md` exits with code 1 (string not found)
- `grep -c "Build from source" docs/howto/dotnet.md` exits with code 0 (build-from-source section
    still exists for contributors)
- `uv run zensical build 2>&1 | tail -1` shows successful build (no broken page)
- `mise run check` passes (formatting hooks)

## Done When

All four verification commands pass, confirming the stale NuGet claim is removed while
build-from-source instructions remain available in a de-emphasized form.
