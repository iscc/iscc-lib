## 2026-03-22 — Guard Swift release build against main/tag SHA mismatch

**Done:** Added a provenance guard step to the `build-xcframework` job in `release.yml` that fails
fast on tag-triggered releases if `main` HEAD has diverged from the tagged commit SHA. The guard is
conditional on tag triggers only (`startsWith(github.ref, 'refs/tags/v')`) so `workflow_dispatch`
re-triggers are unaffected.

**Files changed:**

- `.github/workflows/release.yml`: Added "Verify main matches tag" step between checkout and
    rust-toolchain in the `build-xcframework` job (lines 1259-1269). Compares `github.sha` (tag's
    commit) against `git rev-parse HEAD` (main's current HEAD after checkout). Exits 1 with
    `::error::` annotations if they differ.

**Verification:**

- YAML valid: `yaml.safe_load` exits 0
- Guard step exists: `grep -c` returns 1
- Conditional on tags: `startsWith(github.ref, 'refs/tags/v')` confirmed
- Tag SHA comparison: `github.sha` present in step
- Main HEAD comparison: `git rev-parse HEAD` present in step
- `mise run format`: no changes
- `mise run check`: all 15 pre-commit hooks pass

**Next:** The corresponding issue in issues.md ("Swift release job checks out `ref: main` instead of
tag SHA") can be marked resolved — the guard addresses the race condition with a fail-fast approach.
Two remaining issues: (1) root `Package.swift` CI smoke test (normal), (2) programming language
logos for docs site (low). The root Package.swift smoke test would be a good next step.

**Notes:** No tests to write for this step — it's a CI workflow change, not Rust code. The guard is
the minimal safe fix; the deeper architectural fix (checkout tag SHA instead of main) is explicitly
out of scope per next.md because it would break the git-auto-commit-action flow.
