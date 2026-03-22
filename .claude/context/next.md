# Next Work Package

## Step: Guard Swift release build against main/tag SHA mismatch

## Goal

Add a provenance guard to the `build-xcframework` release job that fails on tag-triggered releases
if `main` HEAD has diverged from the tag's commit SHA. This prevents building XCFramework binaries
from source code different than what was tagged, addressing the "Swift release job checks out
`ref: main` instead of tag SHA" normal issue.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml` (add a guard step after the checkout in
    `build-xcframework` job, ~lines 1255-1258)
- **Reference**: `.claude/context/issues.md` (issue description and suggested fixes),
    `.github/workflows/release.yml` (full `build-xcframework` job, lines 1248-1296)

## Not In Scope

- Rearchitecting the checksum-commit-back flow (too complex for one step — the guard is a safe
    fail-fast approach that prevents the race without redesigning the workflow)
- Changing `ref: main` to `${{ github.sha }}` for the checkout (this would break the
    git-auto-commit-action which commits back to main)
- Adding a root `Package.swift` CI smoke test (separate issue)
- Modifying any other release job or the build script
- Changing the XCFramework cache key (already fixed in iteration 4)

## Implementation Notes

The `build-xcframework` job currently checks out `ref: main` (line 1258) because it needs to:

1. Build the XCFramework from source
2. Compute the checksum
3. Commit the checksum update back to main (via git-auto-commit-action)
4. Force-update the tag to include the checksum commit

The race condition: if `main` moves after the tag is created (concurrent merge, hotfix), the
XCFramework is built from different source than what was tagged.

**Fix: Add a guard step immediately after checkout** that compares `github.sha` (the tag's commit
SHA) with `HEAD` (main's current HEAD after checkout). If they differ, the job fails with a clear
error message.

```yaml
  - name: Verify main matches tag
    if: startsWith(github.ref, 'refs/tags/v')
    run: |
      TAG_SHA="${{ github.sha }}"
      MAIN_SHA=$(git rev-parse HEAD)
      if [ "$TAG_SHA" != "$MAIN_SHA" ]; then
        echo "::error::main HEAD ($MAIN_SHA) differs from tag SHA ($TAG_SHA)."
        echo "::error::This means main has moved since the tag was created."
        echo "::error::Re-tag after main stabilizes, or re-trigger with workflow_dispatch."
        exit 1
      fi
```

**Key design decisions:**

- **Conditional on tag trigger** (`if: startsWith(github.ref, 'refs/tags/v')`): For
    `workflow_dispatch` re-triggers (e.g., `--ref main -f swift=true`), we intentionally use current
    main — no guard needed
- **Placed after checkout**: Must be after `actions/checkout@v4` so `git rev-parse HEAD` works
- **Placed before cache/build steps**: Fails fast before expensive operations
- **Uses `github.sha`**: On tag-triggered runs, this is the commit the tag points to — exactly what
    we want to compare against
- **Clear error messages**: Uses `::error::` annotations so the failure is obvious in the GHA UI

The guard step should go between the checkout step (line 1255-1258) and the rust-toolchain step
(line 1259).

## Verification

- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0 (valid
    YAML)
- `grep -c 'Verify main matches tag' .github/workflows/release.yml` returns `1` (guard step exists)
- `grep -A2 'Verify main matches tag' .github/workflows/release.yml | grep "startsWith(github.ref"`
    confirms the step is conditional on tag triggers only
- `grep -A10 'Verify main matches tag' .github/workflows/release.yml | grep 'github.sha'` confirms
    the tag SHA comparison is present
- `grep -A10 'Verify main matches tag' .github/workflows/release.yml | grep 'git rev-parse HEAD'`
    confirms main HEAD is compared
- `mise run format` produces no changes

## Done When

All verification criteria pass — the `build-xcframework` job has a guard step that fails fast on
tag-triggered releases if main HEAD has diverged from the tagged commit.
