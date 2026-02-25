## 2026-02-25 — Merge PR #2 and tag v0.0.1 release

**Done:** Merged PR #2 (develop → main) with a merge commit, created and pushed the `v0.0.1` tag on
main to mark the experimental release milestone. Updated PR title/body to reflect full scope (31 CID
iterations + interactive work). Switched back to develop branch.

**Files changed:**

- No source files changed (git/GitHub operations only)

**Verification:**

- `gh pr view 2 --json state` → `"state": "MERGED"` ✓
- `git tag --list v0.0.1` → `v0.0.1` ✓
- `git ls-remote --tags origin v0.0.1` → tag exists on remote ✓
- `git branch --show-current` → `develop` ✓
- `git log main --oneline -1` → `4bdc899 Merge pull request #2 from iscc/develop` ✓

**Next:** The v0.0.1 release workflow will be triggered by the tag push. OIDC trusted publishing is
not configured on registry side, so publish jobs are expected to fail — that's a human-gated setup
task. The next development work should focus on post-v0.0.1 goals from target.md (version bump to
0.0.2, or new features/improvements).

**Notes:** Had to stash `iterations.jsonl` (modified by CID loop) before switching to main —
restored after switching back to develop. The `develop` branch is 2 commits ahead of
`origin/develop` (the define-next and update-state commits from this CID iteration). The release
workflow triggers on `push: tags: [v*.*.*]` — no additional action needed.
