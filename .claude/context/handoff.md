## 2026-03-08 — Review of: Fix stale .NET docs claiming NuGet is unavailable

**Verdict:** PASS

**Summary:** The advance agent correctly replaced the misleading `!!! note "Build from source"`
admonition (which stated "NuGet publishing is not yet available") with a collapsed
`??? tip "Build from source"` admonition. The change is minimal, correctly scoped, and all
verification criteria pass. Docs build succeeds with no broken pages.

**Verification:**

- [x] `grep -c "not yet available" docs/howto/dotnet.md` exits with code 1 — stale text removed
- [x] `grep -c "Build from source" docs/howto/dotnet.md` exits with code 0 — section preserved
- [x] `uv run zensical build` — "Build finished in 0.62s", no broken pages
- [x] `mise run check` — all 15 hooks passed

**Issues found:**

- (none) — clean docs-only fix

**Codex review:** Codex noted (P2) that presenting `dotnet add package Iscc.Lib` as the universal
default may mislead users on unsupported RIDs (only 5 platform targets bundled: linux-x64,
linux-arm64, osx-x64, osx-arm64, win-x64). The collapsed "Build from source" tip adequately covers
the alternative. This is an enhancement suggestion, not a correctness issue — the five supported
RIDs cover the vast majority of .NET developers. No action needed for this iteration.

**Next:** Two `normal` priority issues remain: Conan recipe fix (declares shared-library but never
packages the native binary) and Zensical "View as Markdown" 404. After those, only `low` priority
issues remain and the CID loop should signal idle.

**Notes:**

- The `.NET docs still say NuGet publishing is not yet available` issue has been deleted from
    issues.md (resolved by this iteration)
- state.md still references this as an open issue in the C# / .NET and Documentation sections — the
    next update-state cycle will clear those references
- Consider creating a PR from `develop` to `main` when ready for the next release
