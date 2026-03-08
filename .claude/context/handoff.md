## 2026-03-08 — Fix stale .NET docs claiming NuGet is unavailable

**Done:** Replaced the misleading `!!! note "Build from source"` admonition (which stated "NuGet
publishing is not yet available") with a collapsed `??? tip "Build from source"` admonition. The
build-from-source instructions are preserved but reframed as an optional developer workflow, while
`dotnet add package Iscc.Lib` stands alone as the primary installation method.

**Files changed:**

- `docs/howto/dotnet.md`: Changed admonition type from `!!! note` to `??? tip` (collapsed by
    default), removed stale "NuGet publishing is not yet available" claim, reworded intro line to
    "To build from source instead of using the NuGet package:"

**Verification:**

- `grep -c "not yet available" docs/howto/dotnet.md` → 0 matches, exit code 1 (stale text removed)
- `grep -c "Build from source" docs/howto/dotnet.md` → 1 match, exit code 0 (section preserved)
- `uv run zensical build` → "Build finished in 1.05s" (no broken page)
- `mise run check` → all 15 hooks passed

**Next:** Two `normal` priority issues remain: Conan recipe fix and Zensical "View as Markdown" 404.
After those, only `low` priority issues remain. The CID loop may want to signal idle since the
remaining issues are relatively independent and low-priority.

**Notes:** Straightforward docs-only change. No code changes, no tests needed. The NuGet pipeline
(`pack-nuget` → `test-nuget` → `publish-nuget`) was confirmed to exist in `release.yml`.
