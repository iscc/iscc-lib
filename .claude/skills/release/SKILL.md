---
name: release
description: >-
  End-to-end release workflow for iscc-lib. Bumps version, syncs manifests,
  runs quality gates, commits, creates PR to main, and publishes to all
  registries. Self-healing: diagnoses and fixes failures before retrying.
disable-model-invocation: false
user-invocable: true
argument-hint: <version> [--dry-run] [--skip-publish]
---

# Release Workflow for iscc-lib

Execute a robust, self-healing release workflow. The version argument is required.

**Invocation:** `/release 0.0.3` or `/release 0.0.3 --dry-run`

**Flags:**

- `--dry-run` — run all checks and prepare the commit but do NOT push, create PR, tag, or publish
- `--skip-publish` — do everything through PR merge but do NOT push the tag (which triggers
    publishing)

## Phase 1: Pre-flight Checks

Before touching anything, validate the environment is ready.

**Required tools:** `git`, `gh` (authenticated), `mise`, `cargo`, `uv`, `python`.

### Step 1.1 — Parse arguments

Extract the version from `$ARGUMENTS`. It must be a valid semver string (e.g., `0.0.3`, `0.1.0`,
`1.0.0`). Detect `--dry-run` and `--skip-publish` flags if present.

If no version is provided, stop and ask the user for one.

### Step 1.2 — Branch and working tree

```
git status
git branch --show-current
```

- Must be on `develop` branch
- Working tree must be clean (no uncommitted changes). Untracked files in `.claude/` are OK
- If there are uncommitted changes, list them and ask the user whether to stash or abort

### Step 1.3 — Pull latest

```
git pull --ff-only
```

If this fails (diverged history), stop and explain the situation.

### Step 1.4 — Current version check

Read the current version from the `version` field under `[workspace.package]` in root `Cargo.toml`.
Verify the requested version is strictly higher than the current version (compare major.minor.patch
numerically). If not, stop and explain.

### Step 1.5 — CI status

Check that the latest CI workflow run on `develop` is passing:

```
gh run list --workflow ci.yml --branch develop --limit 1 --json status,conclusion,headSha
```

If CI is not green, warn the user and ask whether to proceed anyway.

## Phase 2: Version Bump

### Step 2.1 — Update canonical version

Edit the `version` field under `[workspace.package]` in root `Cargo.toml` to set the new version.

### Step 2.2 — Propagate to all manifests

```
uv run python scripts/version_sync.py
```

This updates all 9 sync targets (manifests, docs, scripts). If it fails, read the error output,
diagnose the issue, fix it, and retry.

### Step 2.3 — Update Cargo.lock

```
cargo check --workspace
```

This regenerates the lockfile entries for workspace members with the new version. Do NOT use
`cargo update -w` which can also upgrade external dependency versions.

### Step 2.4 — Validate consistency

```
uv run python scripts/version_sync.py --check
```

All targets must report OK. If any mismatch, diagnose and fix manually, then re-run the check.

## Phase 3: Quality Gates

### Step 3.1 — Format

```
mise run format
```

This applies all pre-commit auto-fix hooks. If it modifies files, that's expected — the changes will
be included in the release commit.

### Step 3.2 — Lint

```
mise run lint
```

If linting fails:

1. Read the error output carefully
2. Attempt to fix the issues (formatting, clippy warnings, ruff violations)
3. Re-run `mise run lint`
4. If it fails again after one fix attempt, stop and show the errors to the user

### Step 3.3 — Test

```
mise run test
```

If tests fail:

1. Read test output to identify the failing test(s)
2. Determine if the failure is related to the version bump (unlikely) or a pre-existing issue
3. If pre-existing: warn the user and ask whether to proceed
4. If version-related: attempt to fix, re-run tests
5. If tests fail twice, stop and show the errors to the user

## Phase 4: Commit and Push

### Step 4.1 — Stage release changes

Stage only the files modified by the version bump and sync. Expected files:

```
git add Cargo.toml Cargo.lock pyproject.toml mise.toml \
  crates/iscc-napi/package.json crates/iscc-jni/java/pom.xml \
  scripts/test_install.py README.md crates/iscc-jni/README.md \
  docs/howto/java.md docs/java-api.md
git status
```

Only stage files that were actually modified (some doc files may not exist yet — that's OK,
`git add` will skip them). Review staged files before committing. Do NOT use `git add -A` which can
stage untracked files from `.claude/` and other directories.

### Step 4.2 — Commit

Create the release commit. Use the exact format:

```
git commit -m "$(cat <<'EOF'
Release <version>
EOF
)"
```

### Step 4.3 — Push develop

If `--dry-run`, skip this step and report what would be pushed.

```
git push origin develop
```

If push fails due to pre-push hooks:

1. Read the hook output
2. Fix the issue (likely a formatting or test failure)
3. Stage the fix and create a new commit (do NOT amend — the failed push means the commit exists
    locally)
4. Retry push
5. If it fails twice, stop and show the errors

## Phase 5: PR and Merge

If `--dry-run`, skip this phase entirely and summarize what would happen.

### Step 5.1 — Create or update PR

Check if a PR from `develop` to `main` already exists:

```
gh pr list --head develop --base main --json number,state,url
```

If a PR exists and is open, report its URL. If no PR exists, create one:

```
gh pr create -B main -H develop --title "Release <version>" --body "$(cat <<'EOF'
## Release <version>

Version bump and manifest sync for release <version>.

Publishes to: crates.io, PyPI, npm (@iscc/lib, @iscc/wasm), Maven Central.
EOF
)"
```

### Step 5.2 — Wait for CI

Watch PR checks until they complete:

```
gh pr checks <pr-number> --watch --fail-fast
```

This blocks until all checks finish (exit 0 = all passed, exit 1 = failure, exit 8 = pending
timeout). If CI fails, show which check failed and stop.

### Step 5.3 — Ask to merge

**Do NOT merge automatically.** Ask the user:

> PR #N is green and ready to merge: <url> Shall I merge it? (squash / merge commit / rebase)

Wait for explicit confirmation. Then merge:

```
gh pr merge <pr-number> --<method> --delete-branch=false
```

(`--delete-branch=false` because we keep `develop` alive.)

## Phase 6: Tag and Release

### Step 6.1 — Switch to main and pull

```
git checkout main
git pull --ff-only
```

### Step 6.2 — Create tag

```
git tag v<version>
```

### Step 6.3 — Push tag

If `--skip-publish`, skip this step and Step 6.4. Report that the tag `v<version>` was created
locally and can be pushed later with `git push origin v<version>`. Then go directly to Step 6.5.

```
git push origin v<version>
```

This triggers `.github/workflows/release.yml` which publishes to all registries.

### Step 6.4 — Monitor release workflow

Watch the release workflow triggered by the tag push:

```
gh run list --workflow release.yml --limit 1 --json status,conclusion,url,headBranch
```

Verify the latest run's `headBranch` matches `v<version>`. If the workflow hasn't appeared yet, wait
a few seconds and retry (tags may take a moment to trigger). Once found, watch it:

```
gh run watch <run-id>
```

Report progress. If the workflow fails, show the URL and which job failed.

### Step 6.5 — Switch back to develop

```
git checkout develop
git pull --ff-only
```

Ensure develop is up to date after the merge.

## Phase 7: Post-Release Verification

If `--skip-publish`, skip this phase.

### Step 7.1 — Verify registries

After the release workflow completes, verify each registry has the new version. For each registry,
check availability and report pass/fail:

```
# PyPI
curl -sf "https://pypi.org/pypi/iscc-lib/<version>/json" > /dev/null && echo "PyPI: OK"

# crates.io
cargo info iscc-lib 2>/dev/null | grep -q "<version>" && echo "crates.io: OK"

# npm @iscc/lib
npm view @iscc/lib@<version> version && echo "npm @iscc/lib: OK"

# npm @iscc/wasm
npm view @iscc/wasm@<version> version && echo "npm @iscc/wasm: OK"

# Maven Central (may take up to 30 minutes to index)
curl -sf "https://search.maven.org/solrsearch/select?q=g:io.iscc+AND+a:iscc-lib+AND+v:<version>&rows=1&wt=json" | grep -q '"numFound":1' && echo "Maven Central: OK"
```

Maven Central indexing can lag. If Maven shows "NOT FOUND" but the release workflow succeeded, tell
the user it may take up to 30 minutes to appear and suggest re-checking later with:

```
uv run scripts/test_install.py --version <version>
```

### Step 7.2 — Summary

Print a final summary:

```
Release <version> complete!

  Commit:   <sha>
  Tag:      v<version>
  PR:       <url>

  Registries:
    crates.io      <version>  OK
    PyPI           <version>  OK
    npm @iscc/lib  <version>  OK
    npm @iscc/wasm <version>  OK
    Maven Central  <version>  OK / pending indexing

  Post-release:
    uv run scripts/test_install.py --version <version>
```

## Self-Healing Rules

When any step fails:

1. **Read the full error output** — don't guess what went wrong
2. **Identify the root cause** — is it a code issue, environment issue, or transient failure?
3. **Attempt one fix** — apply the most likely fix and retry the step
4. **If the fix doesn't work, stop** — show the error to the user and ask for guidance
5. **Never retry more than twice** — infinite retry loops waste time
6. **Never skip a failing step silently** — every failure must be reported
7. **Never force-push or use destructive git operations** — ask the user first

## Important Constraints

- This skill handles real releases with real side effects. Be careful and precise
- Never guess registry credentials or authentication — they are configured via GitHub secrets
- Never modify files outside the version bump scope without asking
- The `develop` branch is never deleted — it's the long-lived working branch
- Go module publishing happens automatically via the git tag (no explicit publish step)
