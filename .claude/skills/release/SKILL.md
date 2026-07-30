---
name: release
description: >-
  End-to-end release workflow for iscc-lib. Bumps version, syncs manifests,
  runs quality gates, commits, creates PR to main, publishes to all
  registries, and writes consumer-facing GitHub release notes. Self-healing:
  diagnoses and fixes failures before retrying.
disable-model-invocation: false
user-invocable: true
argument-hint: <version> [--dry-run] [--skip-publish]
---

# Release Workflow for iscc-lib

Execute a robust, self-healing release workflow. The version argument is required.

**Invocation:** `/release 0.0.3` or `/release 0.0.3 --dry-run`

**Flags:**

- `--dry-run` — run all checks but do NOT commit, push, create PR, tag, or publish. Reports what
    would happen at each skipped step
- `--skip-publish` — do everything through PR merge but do NOT trigger the release workflow

## Phase 1: Pre-flight Checks

Before touching anything, validate the environment is ready.

**Required tools:** `git`, `gh` (authenticated), `mise`, `cargo`, `uv`, `python`, `curl`, `npm`.

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

### Step 1.6 — Publish credential check (npm token expiry)

Most registries publish via OIDC (crates.io, PyPI, RubyGems) and never expire. **npm is the
exception** — it authenticates with the `NPM_TOKEN` secret, a granular token whose write access
**expires within 90 days** (npm's hard maximum). An expired token is invisible until publish time,
where it fails with `npm error code E404 ... PUT https://registry.npmjs.org/@iscc%2f...` (npm
returns 404, not 401/403, on bad auth). This silently broke the v0.5.0 npm publish.

Before triggering the release, verify the npm token is current:

- Ask the user to open `https://www.npmjs.com/settings/<npm-user>/tokens` (they must be logged in).
- Confirm the release token (named `iscc-lib-ci-*`) is **not** marked `Expired` and is not within a
    few days of its expiry date.

If it is expired or near expiry, rotate it before proceeding:

1. On npm, **Generate New Token** → Granular Access Token. Name `iscc-lib-ci-<year>`; check **Bypass
    2FA**; **Packages and scopes** → Read and write → "Only select packages and scopes" → select
    the `@iscc` scope; **Expiration** → 90 days (the maximum for write tokens).
2. Copy the token, then update the GitHub secret at
    `https://github.com/iscc/iscc-lib/settings/secrets/actions/NPM_TOKEN` (paste value → Update
    secret; GitHub will require a sudo-mode re-auth that **the user** must complete).
3. **Entering the token value is the user's action** — never type credentials yourself. Use the
    clipboard (copy on npm → paste into the GitHub field) so the value stays out of the transcript.

If only the npm publish later fails on a stale token, fix the `NPM_TOKEN` secret, then **re-run the
failed jobs of that release run** with `gh run rerun <run-id> --failed`. This re-runs only the
failed npm publish jobs, reuses the existing build artifacts, picks up the new secret, and touches
no tags/release. Verified working for the v0.5.0 npm recovery (2026-06-18).

> `gh workflow run release.yml --ref main -f npm=true` is the fallback when the failed run's
> artifacts have expired: every `test-*`/`publish-*` job carries a `!cancelled() && !failure()`
> guard, so a registry-only dispatch runs the full build → test → publish chain for that registry.
> It rebuilds all artifacts from scratch, so prefer `gh run rerun <run-id> --failed` when the
> original artifacts still exist. See the re-trigger section at the end of this file.

> **Root-cause fix:** npm supports OIDC Trusted Publishing, which would remove `NPM_TOKEN` and this
> whole expiry class of failure (npm's own UI recommends it for CI/CD). Migrating the npm + wasm
> publish jobs to OIDC is tracked as a future improvement.

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
2. Attempt to fix the issues (formatting, clippy warnings, ruff violations). Lint fixes triggered by
    quality gates are within scope of the release — they are not "edits outside version scope"
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

Stage only the files modified by the version bump and sync. Use `git diff --name-only` to find which
files were actually modified, then stage only those. Expected candidates:

```
Cargo.toml Cargo.lock pyproject.toml mise.toml
crates/iscc-napi/package.json crates/iscc-jni/java/pom.xml
crates/iscc-rb/lib/iscc_lib/version.rb
packages/dotnet/Iscc.Lib/Iscc.Lib.csproj
packages/cpp/vcpkg.json packages/cpp/conanfile.py
scripts/test_install.py README.md crates/iscc-jni/README.md
docs/howto/java.md docs/java-api.md
```

Do NOT use a single `git add` with all paths — `git add` fails if any listed file does not exist.
Stage only files that appear in `git diff`. Do NOT use `git add -A` which can stage untracked files
from `.claude/` and other directories. Review staged files with `git status` before committing.

### Step 4.2 — Commit

If `--dry-run`, skip this step and Step 4.3. Report staged files and what the commit message would
be, then skip to Phase 6 (which also checks for `--dry-run`).

Create the release commit. Use the exact format:

```
git commit -m "$(cat <<'EOF'
Release <version>
EOF
)"
```

### Step 4.3 — Push develop

If `--dry-run`, skip (already handled in Step 4.2).

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

Publishes to: crates.io, PyPI, npm (@iscc/lib, @iscc/wasm), Maven Central, GitHub Releases (FFI), RubyGems, Go proxy, NuGet.
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

### Step 5.3 — Architecture pass

Before asking to merge, prepare an architecture-level summary of the accumulated release diff so the
human reviews the release as a whole — the per-step CID review never sees the codebase at this
altitude, and this is the last cheap moment to catch structural drift before it ships. Present:

```
git diff main...develop --stat | tail -20        # size and shape by area
git diff main...develop --stat -- crates/ | head # per-crate concentration
```

Summarize in a few sentences: which crates/packages changed and why, any public API surface changes
(note the `cargo semver-checks` result from CI if available), new dependencies, and anything that
moved between modules. Flag concentrations that look like scope creep or duplication across binding
crates. This is a reading aid for the human's judgment, not a gate — findings that need work become
issues.md entries, they do not block the release by themselves.

### Step 5.4 — Ask to merge

**Do NOT merge automatically.** Present the architecture summary from Step 5.3, then ask the user:

> PR #N is green and ready to merge: <url>. Shall I merge it?

Wait for explicit confirmation. Always use a **merge commit** (never squash or rebase — those
rewrite history and cause divergence on the long-lived `develop` branch):

```
gh pr merge <pr-number> --merge
```

(The default is to not delete the branch, which is what we want — `develop` stays alive.)

## Phase 6: Trigger Release Workflow

If `--dry-run`, skip this phase entirely. Report a dry-run summary: version bump was applied to
working tree files but not committed (Step 4.2 skipped the commit). The user can inspect the changes
with `git diff` and discard them with `git checkout -- .` if desired. Then jump to the final summary
(Phase 7 also skips in dry-run).

### Step 6.1 — Trigger the release workflow

If `--skip-publish`, skip this step and Step 6.2. Report that the PR was merged and the release can
be triggered later with `gh workflow run release.yml --ref main -f version=<version>`.

Trigger the release workflow via `workflow_dispatch`. The workflow handles everything: building the
Swift XCFramework, updating `Package.swift` with the correct checksum, committing to `main`,
creating the `v<version>` and `packages/go/v<version>` tags, creating the GitHub Release, and
publishing to all registries.

```
gh workflow run release.yml --ref main -f version=<version>
```

Wait a few seconds for the run to appear, then find it:

```
gh run list --workflow release.yml --event workflow_dispatch --limit 1 --json databaseId,status,conclusion,url
```

### Step 6.2 — Monitor release workflow

Once the run is found, watch it:

```
gh run watch <databaseId>
```

Report progress. If the workflow fails, show the URL and which job failed.

### Step 6.3 — Switch back to develop

```
git checkout develop
git pull --ff-only
```

Ensure develop is up to date after the merge.

## Phase 7: Release Notes and Post-Release Verification

If `--dry-run` or `--skip-publish`, skip this phase.

### Step 7.1 — Write GitHub release notes

The release workflow creates the GitHub Release with only auto-generated notes (PR credit + full
changelog link). Replace them with notes written for **downstream consumers** — people deciding
whether and how to upgrade — not for contributors.

**Gather facts first — never write notes from memory:**

1. Previous version: `git tag --sort=-v:refname | head -5` (the tag before `v<version>`).
2. Commit range: `git log v<prev>..v<version> --oneline` — the `cid(advance)` commits carry the
    feature-level messages.
3. Breaking changes: the `cargo semver-checks` failures CI reported on develop **before** the
    version bump name the exact breaking API changes (e.g. `enum_marked_non_exhaustive`). Also
    check for renames/removals in non-Rust surfaces (Go, bindings) in the commit log.
4. Issues closed by this release: `gh issue list --state closed --json number,title,closedAt` and
    match against the release window.
5. **Verify every claim against the code at the release tag** (grep the actual source) — work can
    land and be fully reverted within one release window, so a commit message alone is not evidence
    that a feature shipped.

**Structure** (sections in this order; omit empty ones):

- **Highlights** — headline features with API signatures where helpful, per-language availability,
    experimental/stability caveats
- **Breaking changes** — grouped per ecosystem; explicitly state which languages have none
- **Conformance and robustness** — behavior changes vs the `iscc-core` reference (stricter
    validation means previously-accepted inputs may now error — say so)
- **Performance** — with the consumer-visible effect, not implementation detail
- **Packaging and toolchains** — platform/wheel changes, MSRV, consumer version floors, dependency
    migrations visible to consumers, security patches (RUSTSEC advisories)
- **Published to** — registry list, docs link (`https://lib.iscc.codes/`), install-verification
    command
- **What's Changed** — preserve the auto-generated PR credit and `**Full Changelog**` link from the
    existing release body, and add closed-issue references

**Apply:** write the notes to a scratchpad file, then:

```
gh release edit v<version> --notes-file <path>
```

### Step 7.2 — Verify registries

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

# Go proxy (auto-published via git tag, may take a few minutes)
curl -sf "https://proxy.golang.org/github.com/iscc/iscc-lib/packages/go/@v/v<version>.info" > /dev/null && echo "Go proxy: OK"

# RubyGems
curl -sf "https://rubygems.org/api/v1/versions/iscc-lib.json" | grep -q "\"number\":\"<version>\"" && echo "RubyGems: OK"

# GitHub Releases (FFI tarballs + Swift XCFramework)
gh release view v<version> --json assets --jq '.assets[].name' | grep -q "iscc-ffi" && echo "GitHub Releases (FFI): OK"
gh release view v<version> --json assets --jq '.assets[].name' | grep -q "IsccLib.xcframework" && echo "GitHub Releases (Swift): OK"

# Kotlin Maven Central (may take up to 30 minutes to index)
curl -sf "https://search.maven.org/solrsearch/select?q=g:io.iscc+AND+a:iscc-lib-kotlin+AND+v:<version>&rows=1&wt=json" | grep -q '"numFound":1' && echo "Maven Central (Kotlin): OK"

# NuGet
curl -sf "https://api.nuget.org/v3-flatcontainer/iscc.lib/<version>/iscc.lib.nuspec" > /dev/null && echo "NuGet: OK"
```

Maven Central and Go proxy indexing can lag. If either shows "NOT FOUND" but the release workflow
succeeded (or the tag was pushed), tell the user it may take up to 30 minutes to appear and suggest
re-checking later with:

```
uv run scripts/test_install.py --version <version>
```

### Step 7.3 — Summary

Print a final summary:

```
Release <version> complete!

  Commit:   <sha>
  Tag:      v<version>
  PR:       <url>
  Notes:    https://github.com/iscc/iscc-lib/releases/tag/v<version>

  Registries:
    crates.io          <version>  OK
    PyPI               <version>  OK
    npm @iscc/lib      <version>  OK
    npm @iscc/wasm     <version>  OK
    Maven Central      <version>  OK / pending indexing
    Maven (Kotlin)     <version>  OK / pending indexing
    Go proxy           <version>  OK / pending indexing
    RubyGems           <version>  OK
    GitHub (FFI)       <version>  OK
    GitHub (Swift)     <version>  OK
    NuGet              <version>  OK

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

## Re-triggering a Failed Registry

If specific registries fail while others succeed, **prefer re-running the failed jobs of the same
release run** — this reuses build artifacts, picks up any rotated secrets, and touches no
tags/release:

```
gh run rerun <run-id> --failed
```

This is the verified recovery path (used for the v0.5.0 npm token failure). It only re-runs jobs
that reached a `failure` conclusion, so it works when a publish job ran and errored (bad token,
transient registry error). It does NOT help if a job was `skipped`.

### Re-triggering a registry with `-f <registry>=true`

If the original run's artifacts have expired (or a job was `skipped` rather than `failure`, which
`gh run rerun --failed` cannot re-run), dispatch the workflow with only the registry flag:

```
gh workflow run release.yml --ref main -f <registry>=true   # crates-io|pypi|npm|maven|ffi|rubygems|nuget|maven-kotlin
```

Every `test-*`/`assemble-*`/`pack-*`/`publish-*` job carries a `!cancelled() && !failure()` guard,
so a registry-only dispatch (no `version` input, `prepare-release` skipped) runs the full build →
test → publish chain for that registry instead of propagating the skip down the `needs` chain. The
guards are verified statically (YAML shape + expression lint); their first real-world confirmation
comes with the next release run. Prefer `gh run rerun <run-id> --failed` when possible — it reuses
existing build artifacts instead of rebuilding everything.

**Never pass `-f version=<version>` to recover** — that re-runs `prepare-release`, which
force-pushes the release tags and updates the GitHub Release.

## Important Constraints

- This skill handles real releases with real side effects. Be careful and precise
- Never guess registry credentials or authentication — they are configured via GitHub secrets and
    OIDC trusted publishing (PyPI, crates.io, RubyGems use OIDC; npm uses automation tokens; Maven
    uses GPG + OSSRH credentials; NuGet uses API key via `NUGET_API_KEY` secret)
- Never modify files outside the release scope without asking. Release scope includes: version bump
    files, formatting/lint fixes from quality gates, and any files the pre-commit hooks auto-fix
- The `develop` branch is never deleted — it's the long-lived working branch
- Go module publishing happens automatically via the git tag (no explicit publish step)
