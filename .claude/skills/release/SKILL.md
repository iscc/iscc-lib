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

**Required tools:** `git`, `gh` (authenticated), `mise`, `cargo`, `uv`, `python`, `curl`, `npm`,
`bundle`.

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

### Step 1.6 — npm trusted publishing

All OIDC registries (crates.io, PyPI, RubyGems, npm) publish without stored tokens, so there is no
credential expiry to check. npm uses Trusted Publishing: each package (`@iscc/lib`, `@iscc/wasm`)
has a trusted publisher on npmjs.com (Settings → Trusted Publisher) for GitHub Actions, `iscc` /
`iscc-lib`, workflow `release.yml`, no environment, with **Allow npm publish** enabled. The publish
jobs need `id-token: write`, npm >= 11.5.1 (Node 24) and no `registry-url` in `setup-node` (it would
write an `.npmrc` that references an unset `NODE_AUTH_TOKEN`).

If an npm publish fails with `E404`/`ENEEDAUTH`, first check that both trusted publisher entries
still exist and match the fields above (they cannot be edited, only deleted and recreated, which
requires the user's 2FA passkey). Then **re-run the failed jobs of that release run** with
`gh run rerun <run-id> --failed`. This re-runs only the failed publish jobs, reuses the existing
build artifacts, and touches no tags/release.

> `gh workflow run release.yml --ref v<version> -f npm=true` is the fallback when the failed run's
> artifacts have expired: every `test-*`/`publish-*` job carries a `!cancelled() && !failure()`
> guard, so a registry-only dispatch runs the full build → test → publish chain for that registry.
> It rebuilds all artifacts from scratch, so prefer `gh run rerun <run-id> --failed` when the
> original artifacts still exist. See the re-trigger section at the end of this file.

## Phase 2: Version Bump

### Step 2.1 — Update canonical version

Edit the `version` field under `[workspace.package]` in root `Cargo.toml` to set the new version.

### Step 2.2 — Propagate to all manifests

```
uv run python scripts/version_sync.py
```

This rewrites every sync target, the `TARGETS` list in `scripts/version_sync.py`. If it fails, read
the error output, diagnose the issue, fix it, and retry.

### Step 2.3 — Update the lockfiles

```
cargo update -w
uv lock
(cd crates/iscc-rb && bundle lock)
```

`Cargo.lock`, `uv.lock` and `crates/iscc-rb/Gemfile.lock` record the workspace version and are not
sync targets: their own tool writes them. Each command changes the version entries of the workspace
packages only and leaves every external dependency at its locked version (`cargo update -w` updates
workspace members only). If `git diff` shows any other change in a lockfile, stop and show it to the
user.

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

- `Cargo.toml` (Step 2.1) and the files in the `TARGETS` list in `scripts/version_sync.py` (Step
    2.2): the version is written into them directly.
- The three lockfiles of Step 2.3: `Cargo.lock`, `uv.lock` and `crates/iscc-rb/Gemfile.lock`. All
    three belong in the release commit. If one is missing from `git diff`, run its command from Step
    2.3 before staging.

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

Publishes to: crates.io, PyPI, npm (@iscc/lib, @iscc/wasm), Maven Central (Java, Kotlin), RubyGems, NuGet, Go proxy, GitHub Releases (FFI archives, Swift XCFramework).
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

Trigger the release workflow via `workflow_dispatch`. The workflow handles everything, in this
order:

1. `build-ffi` builds the FFI archives. It is the only build that runs before the tag exists.
2. `prepare-release` builds the Swift XCFramework and writes the release checksums: the XCFramework
    checksum into `Package.swift`, and the hashes of the FFI archives into
    `packages/cpp/portfile.cmake` (vcpkg) and `packages/cpp/conandata.yml` (Conan). It commits them
    to `main`, creates the `v<version>` and `packages/go/v<version>` tags on that commit and
    creates the GitHub Release.
3. `test-ffi` smoke-tests the Linux FFI archive at the same time as `prepare-release`. Every other
    job waits for `prepare-release` and then builds, tests and publishes its registry.

A failing `build-ffi` leg therefore blocks the tag and every publish job. A failing `test-ffi`
blocks `publish-ffi` only: the tag and the GitHub Release can exist without FFI archives.

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

Report progress. If the workflow fails, show the URL and which job failed, then follow
"Re-triggering a Failed Registry" at the end of this file. A failure of one registry chain (for
example one Ruby series in `test-gem`) holds back that registry only.

### Step 6.3 — Fast-forward develop to main

```
git fetch origin
git checkout develop
git merge --ff-only origin/main
git push origin develop
```

`main` now holds the merge commit of the release PR and the checksum commit of `prepare-release`
(`Package.swift`, `packages/cpp/portfile.cmake`, `packages/cpp/conandata.yml`). `develop` must carry
both: without the checksum commit its vcpkg and Conan recipes pin the hashes of the previous
release. If the fast-forward fails because `develop` has moved on, stop and ask the user before
merging.

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

The two GitHub Releases lines check asset names only. Also compare the hashes of the FFI archives
and the XCFramework zip with the tag, using the commands under "Assets that the tag pins by hash" at
the end of this file.

Maven Central and Go proxy indexing can lag. If either shows "NOT FOUND" but the release workflow
succeeded (or the tag was pushed), tell the user it may take up to 30 minutes to appear and suggest
re-checking later with:

```
uv run scripts/test_install.py --version <version>
```

### Step 7.3 — Retire met requirements

A shipped milestone leaves text behind that only costs attention from then on. Shrink it in one
`cid(human)` commit on `develop`, and show the user the diff before committing:

1. `target.md`: replace the "Current Release Milestone" section with the next milestone. Ask the
    user what it is if nothing defines it yet
2. `.claude/context/specs/`: for every requirement of the shipped milestone that a test or a CI gate
    now enforces, cut the prose down to one line that names the test or gate. Keep rationale only
    where the code cannot show it
3. `decisions.md` and `decisions-archive.md`: delete entries that a later decision superseded
4. `issues.md`: delete entries the release resolved, and ask the user about `low` entries parked for
    more than one release

Leave requirements of milestones that have not shipped untouched.

### Step 7.4 — Summary

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

First check whether the tag exists: `git ls-remote --tags origin v<version>`.

- **No tag yet** — nothing was published. `gh run rerun <run-id> --failed` continues the run with
    its artifacts. If a fix has to land on `main` first, merge it and dispatch again with
    `-f version=<version>`.
- **The tag exists and `prepare-release` succeeded** — re-run the failed jobs of the same run with
    `gh run rerun <run-id> --failed`.
- **Anything else** — stop and ask the user: see "Assets that the tag pins by hash".

`gh run rerun --failed` reuses the build artifacts, picks up rotated secrets and touches no tags. It
re-runs the jobs that reached `failure` and the jobs that wait for them, not `skipped` jobs. It is
the verified recovery path (the v0.5.0 npm token failure).

### Re-triggering a registry with `-f <registry>=true`

When the run's artifacts have expired or a job was `skipped`, dispatch the workflow with only the
registry flag, from the release tag, so the jobs build the released revision:

```
gh workflow run release.yml --ref v<version> -f <registry>=true   # crates-io|pypi|npm|maven|rubygems|nuget|maven-kotlin
```

If the fix for the failure landed on `main` after the tag (the v0.7.0 Maven publish needed a plugin
bump, PR #50), dispatch with `--ref main` instead, but only after
`git diff v<version> origin/main --stat` shows nothing except that fix.

Every `test-*`/`assemble-*`/`pack-*`/`publish-*` job carries a `!cancelled() && !failure()` guard,
so a registry-only dispatch (no `version` input, `prepare-release` skipped) runs the full build →
test → publish chain for that registry. `scripts/check_release_workflow.py` checks the guards, and a
`maven`-only dispatch published the v0.7.0 Java artifact this way. Never dispatch `-f ffi=true` for
a tagged release: it uploads rebuilt archives (next section).

### Assets that the tag pins by hash

The tagged commit pins the hash of six release assets: the five FFI archives (SHA512 in
`packages/cpp/portfile.cmake`, SHA256 in `packages/cpp/conandata.yml`) and `IsccLib.xcframework.zip`
(`releaseChecksum` in `Package.swift`). A rebuild does not reproduce their bytes, and vcpkg, Conan
and SwiftPM reject a download whose hash differs. The workflow uploads files without comparing
hashes. Once the tag exists:

- **Never delete or move the tags** `v<version>` and `packages/go/v<version>`: the Go proxy keeps
    the module it fetched from the tag, and SwiftPM pins the tagged revision.
- **Never dispatch `-f version=<version>` or `-f ffi=true`, and do not re-run `prepare-release`**:
    each can build a pinned asset again and publish it.
- **For any other failure** (`prepare-release` failed, `test-ffi` failed, a release asset is missing
    or unfinished): stop and ask the user. Report which jobs failed and the output of the comparison
    below.

Facts the user will need for that decision:

- The FFI archives that the tag pins are the artifacts of the run that built them, kept for 90 days
    (`gh run download <run-id> --name ffi-<target>`). `test-ffi` tested them only if it passed in
    that run.
- A successful `prepare-release` saved the XCFramework zip in the Actions cache under the key its
    log names (`Cache saved with key: xcf-…`). `gh cache list --key xcf-` shows whether the entry
    still exists; GitHub evicts an entry after 7 days without use. A failed job saves nothing.
- If no file with a pinned hash is left, the way out is a patch release.

Compare the release assets with the tag:

```
git fetch origin --tags
# Swift: both commands must print the same hash
git show v<version>:Package.swift | grep 'releaseChecksum ='
gh release view v<version> --json assets --jq '.assets[] | select(.name == "IsccLib.xcframework.zip") | .digest'
# FFI: the hash of each of the five archives must be in the tagged conandata.yml
git show v<version>:packages/cpp/conandata.yml
gh release view v<version> --json assets --jq '.assets[] | select(.name | startswith("iscc-ffi-")) | "\(.name) \(.state) \(.digest)"'
```

For a local file, `sha256sum <file>` prints the hash to compare.

## Important Constraints

- This skill handles real releases with real side effects. Be careful and precise
- Never guess registry credentials or authentication — they are configured via GitHub secrets and
    OIDC trusted publishing (PyPI, crates.io, RubyGems and npm use OIDC; Maven uses GPG + OSSRH
    credentials; NuGet uses API key via `NUGET_API_KEY` secret)
- Never modify files outside the release scope without asking. Release scope includes: version bump
    files, formatting/lint fixes from quality gates, and any files the pre-commit hooks auto-fix
- The `develop` branch is never deleted — it's the long-lived working branch
- Go module publishing happens automatically via the git tag (no explicit publish step)
