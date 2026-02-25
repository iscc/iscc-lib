# Next Work Package

## Step: Merge PR #2 and tag v0.0.1 release

## Goal

Merge the open PR #2 (develop → main) and create the `v0.0.1` tag on main to mark the experimental
release milestone. This completes the v0.0.1 development cycle and triggers the release workflow.

## Scope

- **Create**: none
- **Modify**: none (this is a git/GitHub operations step, no source code changes)
- **Reference**: `.github/workflows/release.yml`, `.claude/context/state.md`

## Not In Scope

- Fixing any release workflow failures (OIDC trusted publishing is not configured on registry side —
    expected to fail; that's a human-gated registry setup task)
- Bumping the version to 0.0.2 or any other version
- Creating a GitHub Release (the tag is sufficient; a GitHub Release can be added later)
- Deleting the `develop` branch after merge (it continues as the CID working branch)
- Any code changes or documentation updates

## Implementation Notes

PR #2 is already open (`develop` → `main`), CI is fully green (all 7 jobs SUCCESS), and GitHub
reports `mergeStateStatus: CLEAN` / `mergeable: MERGEABLE`. The PR title and body were written for
iterations 25-30 but iteration 31's ruff fix commits are also included.

Steps:

1. **Update PR #2** — update the title and body to reflect the full scope (iterations 25-31 +
    interactive session work). Use `gh pr edit 2 --title "..." --body "..."`.

2. **Merge PR #2** — use `gh pr merge 2 --merge` (merge commit, not squash — preserve the full
    commit history). Do NOT use `--delete-branch` since `develop` is the ongoing CID working
    branch.

3. **Switch to main and pull** — `git checkout main && git pull origin main` to get the merge commit
    locally.

4. **Create the v0.0.1 tag** — `git tag v0.0.1` on main. Then `git push origin v0.0.1` to push the
    tag. This triggers the release workflow (`.github/workflows/release.yml`).

5. **Switch back to develop** — `git checkout develop` to leave the working directory on the CID
    branch.

Important:

- Do NOT use `--squash` for the merge — the full commit history is valuable for traceability.
- Do NOT delete the `develop` branch after merge.
- Do NOT wait for or check release workflow results — OIDC publishing is not configured on registry
    side, so publish jobs are expected to fail. The tag itself is the milestone marker.
- The release workflow triggers on `push: tags: [v*.*.*]` — pushing the tag is sufficient.

## Verification

- `gh pr view 2 --json state` shows `"state": "MERGED"`
- `git tag --list v0.0.1` outputs `v0.0.1` (tag exists locally)
- `git ls-remote --tags origin v0.0.1` shows the tag on the remote
- `git branch --show-current` outputs `develop` (back on the CID working branch)
- `git log main --oneline -1` shows the merge commit

## Done When

All verification criteria pass: PR #2 is merged, v0.0.1 tag exists on both local and remote, and the
working directory is back on the `develop` branch.
