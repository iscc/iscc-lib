---
name: release-yml-static-gates
description: How to scope and statically verify edits to .github/workflows/release.yml — job/guard inventory, actionlint via go run, and the parse-based invariant checks
metadata:
  type: project
---

# Verifying `.github/workflows/release.yml` without running it

Nothing in `release.yml` is exercised by a CID push (`workflow_dispatch` only, 8 registry toggles),
so every criterion for a step touching it must be **static**.

**Why:** a mistake here is invisible until Titusz cuts a release, and a release is exactly when a
mistake is most expensive.

**How to apply:** when scoping a `release.yml` step, build the criteria from the four checks below,
and keep behavioural edits (`if:` conditions, `needs:`) in a different commit from mechanical ones
(`uses:` ref bumps) so a broken release is bisectable.

## Inventory (verified at HEAD, iter 139)

- 1345 lines, **29 jobs**, 97 `uses:` lines over 18 distinct action refs.
- `prepare-release` is the only job whose `if:` is (correctly) unguarded: `inputs.version != ''`.
    Guarding it would make the tag-pushing job run on registry-only dispatches — never do it.
- Registry-token histogram across all job `if:` expressions (a strong "no condition was dropped"
    invariant):
    `version:29, crates-io:1, pypi:4, npm:6, maven:4, ffi:3, nuget:4, rubygems:3, maven-kotlin:4`.
- Guard shape in use: `${{ !cancelled() && !failure() && (<original condition>) }}`. `!failure()`
    still reflects the job's own `needs`, so publishes stay blocked after a failed build/test — only
    the skip propagated from `prepare-release` is neutralised. `always()` would break that.

## The four static checks

1. `uv run python` + `yaml.safe_load(...)["jobs"]` — assert job count, per-job `if:` regex shape,
    and the registry-token histogram. Parse-based, immune to line-number drift.
2. `go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7 <file>` — **works in the devcontainer**
    (Go 1.26.1, network OK, module now cached; no `actionlint` binary is installed). Exit 0 with no
    output at HEAD on both workflows; validates GH expression syntax that YAML parsing cannot.
3. `uv run prek run check-yaml --files <file>` → Passed.
4. `uv run prek run yamlfix --files <file>` → Passed with no `files were modified by this hook`
    (works because the file is tracked).

## Related

- GHA `uses:` refresh facts and the "floating `@vN` is a convention, not a guarantee" rule live in
    [[dep-refresh-ledger]].
- `.claude/skills/release/SKILL.md` documents the re-trigger recovery path and must be updated in
    the same step whenever release-job gating changes.
