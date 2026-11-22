---
name: gha-workflow-reviews
description: Reviewing GitHub Actions workflow diffs — release.yml static gates, action-major bump verification, and the traps that only show up at release time
metadata:
  type: project
---

# Reviewing GitHub Actions workflow changes

`ci.yml` and `docs.yml` are exercised by every CID push, so a green CI run *is* the verification.
`.github/workflows/release.yml` is **`workflow_dispatch`-only** — nothing in the CID loop or CI ever
runs it, so every claim about it must be verified statically or it is unverified. See
[[review-patterns]] for the general claim-probing stance and [[gate-reviews]] for the CI job matrix.

## release.yml static gates (iters 123 / 139 / 140)

Run all of these on any `release.yml` diff:

1. **YAML + expression parse** — `uv run prek run check-yaml --files <f>` and
    `uv run prek run yamlfix --files <f>` (both must print `Passed` with no
    `files were modified by this hook`), then
    `go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7 <f>` (module is in the Go cache;
    offline-safe). actionlint validates `${{ }}` expression syntax that `yaml.safe_load` cannot.
2. **Registry-guard shape** — 29 jobs. `prepare-release` carries the bare `inputs.version != ''`;
    all 28 others must match
    `${{ !cancelled() && !failure() && (inputs.version != '' || inputs.<flag>…) }}` exactly, and
    the per-flag token multiset must be unchanged (`inputs.version` 29, `pypi` 4, `npm` 6, `maven`
    4, `ffi` 3, `nuget` 4, `rubygems` 3, `maven-kotlin` 4, `crates-io` 1). `always()` in place of
    `!failure()` is **NEEDS_WORK** — it publishes after a failed build. Rationale in `decisions.md`
    2026-07-25.
3. **Guard invariant for any NEW job** — its `needs` chain must be gated by the same registry flag
    or a superset (`build-ffi` is `ffi || nuget`, feeding both `test-ffi` and `pack-nuget`).
    Relaxing the implicit `success()` is only safe under that condition; a job whose dependency is
    gated by a *different* flag would run against artifacts that were never built.
4. **Artifact wiring** — every `download-artifact` `name:`/`pattern:` must resolve to some
    `upload-artifact` `name:` (11 uploads, 20 downloads). Substitute `${{ … }}` with `*` before
    `fnmatch`.
5. **Matrix coverage** — adding a wheel target touches the build + test matrices only;
    `publish-pypi` collects via `pattern: wheels-*`.

Checks 2–5 are now automated (see the two committed-gate sections below); check 1 (actionlint /
expression syntax) is still manual. Never retype these into a heredoc again.

## Verifying an action-major bump (iter 140 recipe, ~2 min)

Steps (a)–(b) are now done for you by `--check-action-inputs`; (c) runtime and (d) defaults remain
manual and are where the real surprises live. Tag existence is the *weakest* useful check.

```bash
# (a) every @vN resolves
curl -s -o /dev/null -w '%{http_code}\n' https://api.github.com/repos/<o>/<r>/git/ref/tags/<vN>

# (b) fetch action.yml AT THE TAG REF and diff its inputs/outputs against workflow usage
curl -sf https://raw.githubusercontent.com/<o>/<r>/<vN>/action.yml
```

- **(b) inputs** — collect the real `with:` key set per `uses:` value with `yaml.safe_load` over the
    workflow (never grep), then assert each key is present under the action's `inputs`.
- **(b) outputs** — assert every `steps.<id>.outputs.<x>` the workflow reads is still under the
    action's `outputs` (`actions/cache@v6` still declares `cache-hit`, which `build-xcframework`
    depends on).
- **(c) runtime** — `runs.using` must be a runtime the hosted runners support (`node24` needs runner
    ≥ 2.327.1; GitHub-hosted is far past it).
- **(d) defaults** — read **every intervening** major's release-notes body, not just the target's.
    An input surviving is not the same as its default surviving. `gh api` or
    `https://api.github.com/repos/<o>/<r>/releases/tags/<vN>` → `.body`.

Also assert the diff is *only* `uses:` lines when the step claims a pure find/replace:

```bash
git diff HEAD~1..HEAD -- <file> | grep -E '^[+-]' | grep -v '^[+-][+-]' \
    | grep -vE '^[+-]\s*(- )?uses: '     # must be empty
```

### Default changes already checked and cleared (iter 140)

| Action                 | Behavioural delta                                                                   | Status here                                                                                                                                                                                                                        |
| ---------------------- | ----------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `setup-node@v5+`       | auto-caches when `package.json` has `packageManager`; then FAILS if no lockfile     | safe — no tracked `package.json` has the field, no lockfile tracked. **Recheck if either lands**                                                                                                                                   |
| `setup-node@v6`        | auto-caching limited to npm                                                         | moot, see above                                                                                                                                                                                                                    |
| `setup-node@v7`        | dropped the dummy `NODE_AUTH_TOKEN` export                                          | npm publish jobs use OIDC trusted publishing (2026-10-17): no token, Node 24, and **no `registry-url`** (it would write an `.npmrc` referencing the unset var)                                                                     |
| `checkout@v6+`         | auth token persisted to `$RUNNER_TEMP`, not `.git/config`                           | `git push`/`fetch` still work, so `prepare-release`'s `main` commit + tag force-push are fine; only authenticated git *inside a Docker container action* needs runner ≥ 2.329.0. `persist-credentials` still defaults `true` at v7 |
| `setup-python@v7`      | removed `pip-install`                                                               | unused (both sites pass only `python-version`)                                                                                                                                                                                     |
| `upload-artifact@v7`   | new `archive:` input                                                                | defaults `true` = old behaviour                                                                                                                                                                                                    |
| `download-artifact@v5` | breaking path change for single downloads **by `artifact-ids:`**                    | zero `artifact-ids:` uses in this repo                                                                                                                                                                                             |
| `download-artifact@v8` | `digest-mismatch` default warn → **error**; `skip-decompress` added (default false) | strict kept deliberately (`decisions.md` 2026-07-25). If a publish fails on it, investigate the artifact — do NOT relax the input                                                                                                  |

## Pinning conventions

- Repo convention is **floating majors**, not SHAs (`decisions.md` 2026-07-25). One exception:
    `astral-sh/setup-uv@v9.0.0`, because upstream stopped publishing floating majors after `v7`.
- A floating `@vN` is a *publisher convention*, not a guarantee — confirm `git/ref/tags/v<N>`
    returns 200 before writing `@vN`.
- `release.yml` has **no** `setup-uv` step (it invokes no `uv`/`uvx`); the issue ledger claimed
    otherwise until iter 140. Don't reintroduce it.
- The one unpinned ref left is `rubygems/configure-rubygems-credentials@main`. Upstream's README
    shows `@main` everywhere *and* recommends SHA pinning; upstream publishes only exact tags (no
    floating `v2`), and `main` is ahead of `v2.1.0`. Filed as a `[review]` issue awaiting Titusz's
    tag-vs-SHA call — **do not pass a step that picks a SHA unilaterally**, it would reopen a
    decided convention.

## Cross-platform CI (ci.yml)

- Windows GHA runners default to `pwsh`. Any `run:` step using bash syntax (`$(...)`,
    `$GITHUB_OUTPUT`, `grep`, `sed`) inside a matrix that includes Windows MUST declare
    `shell: bash` — check this whenever a step is added to a cross-platform matrix (`build-ffi` is
    the usual offender).

## release.yml checks 1–2 are a committed gate since iter 142

`scripts/check_release_workflow.py` now enforces, on every edit of `.github/workflows/release.yml`
(prek hook `check-release-workflow`, scoped by `files:` regex) and in CI
(`tests/test_check_release_workflow.py`):

1. **Registry-guard shape** — `prepare-release` keeps the bare `if: inputs.version != ''`; every
    other job matches `${{ !cancelled() && !failure() && (…) }}` with `inputs.version != ''` as an
    alternative; declared `workflow_dispatch` inputs and job-`if` references agree in both
    directions (catches a typo'd flag *and* a silently dropped one). Nothing is hardcoded except
    the `prepare-release` job id — no frozen job counts or flag histograms.
2. **Artifact wiring** — every `download-artifact` `name:`/`pattern:` resolves to some
    `upload-artifact` `name:` after expanding `${{ matrix.<k> }}` against
    `strategy.matrix.include`. At HEAD: 34 expanded uploads, 21 expanded download refs.
3. (bonus) **`needs:` graph** — every `needs:` entry names a declared job id.

Run `uv run scripts/check_release_workflow.py` instead of retyping heredocs. **Known limitation
(accepted, `decisions.md` 2026-07-25):** the matcher is a strict approximation of glob intersection,
so a pairing that wildcards on both sides in different positions (`gem-*` upload vs a hypothetical
`*-linux` download) would be reported as an error even though it overlaps. It errs strict, never
lax; if that pairing ever appears, replace it with a real intersection, never a looser match.

## Check 4 (action-input compatibility) is a committed gate since iter 144

`uv run scripts/check_release_workflow.py --check-action-inputs` fetches each distinct `uses:` ref's
published `action.yml`/`action.yaml` from `raw.githubusercontent.com` (stdlib `urllib`, 20s timeout,
one cached fetch per ref) and validates every `with:` key against declared `inputs` and every
`steps.<id>.outputs.<x>` read against declared `outputs`. Runs in the CI-only `release-workflow` job
— never in prek or pytest, which stay network-free. **Nothing is hand-verified any more; run the
command.**

Review recipe on any `release.yml` action bump (~90 s):

```bash
uv run scripts/check_release_workflow.py --check-action-inputs   # exit 0 AND zero "warning: skipped"
https_proxy=http://127.0.0.1:9 http_proxy=http://127.0.0.1:9 \
    uv run scripts/check_release_workflow.py --check-action-inputs # exit 0, 18 skip warnings
```

**Zero `warning: skipped` lines is part of the pass** — a fully rate-limited run is green and
indistinguishable from a real pass by exit code alone (accepted trade-off, `decisions.md`
2026-07-26). Read the log, not the status.

**Proving the gate on any change to it** — synthetic typos only prove string comparison. Use a real
cross-major regression in a temp copy:

```bash
cp .github/workflows/release.yml /tmp/p.yml
sed -i 's|actions/download-artifact@v8|actions/download-artifact@v3|' /tmp/p.yml
uv run scripts/check_release_workflow.py --check-action-inputs /tmp/p.yml  # 14 errors, exit 1
```

(`@v999` on any ref exercises the 404 → `ActionNotFoundError` path.)

**Blind spots probed in iter 144, all filed as a `normal` `[review]` issue, none biting at HEAD:**
required-input omission is not caught (the check is one-directional); `except OSError` misses
`http.client.IncompleteRead` and `yaml.YAMLError`, so a truncated/HTML response *does* red the job
despite the documented fail-open contract; Docker-type actions' native `with: args`/`entrypoint`
would false-positive; PyYAML YAML 1.1 booleans mangle an input named `on`/`off`/`yes`/`no`; and
job-level `uses:` (reusable workflows) is not scanned at all — only `job.steps[*].uses`.

## ci.yml fix that only a CI run can prove (iter 260, ~25 min)

next.md said "review pushes and reads the run". Order that worked:

1. Local criteria + `mise run check`, then read the ACTION'S SOURCE at the pinned tag before
    believing next.md's account of it:
    `gh api 'repos/<o>/<r>/contents/src/<file>?ref=<tag>' --jq .content | base64 -d` (also
    `action.yml` and `README.md`). Read the whole install flow, not only the lines next.md cites.
2. Push the define-next + advance commits FIRST:
    `git -c credential.helper='!gh auth git-credential' push origin HEAD:develop` (pre-push gates
    run, ~3 min). `concurrency: cancel-in-progress` is on, so do not push again before the jobs
    under test finish.
3. `gh run list --branch develop --workflow CI --limit 3 --json databaseId,headSha,status`, wait on
    the named jobs, then read each job's step log:
    `gh run view <run> --job <job-id> --log | grep -i "install-dotnet.sh\|dotnet-install:"`. A
    green job proves the result, the LOG proves the mechanism.
4. Write the handoff with the measured result, commit, push the review commit.

What it found: next.md's Implementation Notes cited two real source lines and still drew a false
conclusion ("a leg sees only the runtimes it lists"). `setup-dotnet` runs
`install-dotnet.sh --runtime dotnet --channel LTS` before every requested version, so a private
`DOTNET_INSTALL_DIR` also holds the latest LTS runtime, and `LatestMajor` follows it. Advance had
"checked the action source" at exactly the cited lines; Codex missed it too. For any roll-forward or
"newest installed" claim, ask what ELSE the installer puts in the directory.

The `gh` CLI prints a long "hint: … gh event listen" block to stderr on each poll: add `2>/dev/null`
to polling loops.

### Iter 262: the same recipe on a Gradle + setup-dotnet step (~15 min, PASS)

- `skip-lts-prepass: true` is confirmed on CI: the log prints "Skipping LTS runtime pre-pass", no
    `--channel LTS` call, and the SDK 8 host (first installed) still runs SDK 10 and runtime 10.
- Kotlin plugin resolution:
    `./gradlew -p packages/kotlin buildEnvironment --refresh-dependencies --info --console=plain` to
    a log, then grep `kotlin-gradle-plugins-bom` and count `plugins.gradle.org` (expected 0). Read
    the FAILED job's log first (`gh run view <run> --job <id> --log | grep -A14 'went wrong'`) to
    check next.md's diagnosis against the real error.
- The background Codex run executes Gradle in the same tree too. `.kotlin/` is gitignored since 262.
- Wait on named jobs with one loop over
    `gh run view <run> --json jobs --jq '[.jobs[] | select(.name|test("Kotlin|\\.NET")) | select(.status!="completed")] | length'`.

### Iter 267: a plain `run:` step in an existing job (Go `GOARCH=386`, ~12 min, PASS)

- No action source to read. Local proof first: same `=== RUN` count on both architectures
    (`go test -v`), `file` on a `go test -c` binary, and the reverted fix failing on 386 only.
- `gh run view <run> --job <id> --log` prints nothing while OTHER jobs of the run are still running.
    Read a finished job's log at once with
    `gh api --allow-escape-sequences repos/iscc/iscc-lib/actions/jobs/<job-id>/logs > /tmp/x.log`,
    then `grep -a` (the log carries ANSI escapes).
- The Go job finishes about 1 minute after the push; waiting for the whole run is not needed.

### Iter 274: freshness steps for generated files (`c-ffi` `.cs` diff, `kotlin` UniFFI diff, ~25 min, PASS)

- One script, one log (`/tmp/<x>/probe.sh`): `git archive HEAD | tar -x` + `git init` + commit,
    extract each step's `run` with PyYAML into a file, run it with
    `bash --noprofile --norc -eo pipefail <file>` and `RUNNER_TEMP` set. A cold debug build of
    `iscc-ffi` + `iscc-uniffi` (+ `--features bindgen`) in the throwaway tree takes about 2 minutes.
- Mutations that paid off: the guarded file's own previous version
    (`git log -1 --format=%h -- <path>`, then `git show <sha>~1:<path>`), an EMPTY committed file, a
    whitespace-only change (must stay green) and one added inner space (must fail). Advance had
    mutated `.kt` and `.h`; take the file it left out (`.swift`).
- **A step that collects failures in a variable is green without its last `exit "$var"` line**, and
    an anchor test that greps for paths and flags does not see the line go. Mutate the workflow file
    (drop the exit line, swap step order, remove one language) and run the anchor test on each; add
    the missing assertion as a step-9 fix.
- **A `git diff` after a build proves nothing unless the build script ran.** In a warm tree a change
    to `NativeMethods.g.cs` alone does not rerun csbindgen's `build.rs` (rc=0, false green). On CI
    it runs on a full rust-cache hit: the `Build FFI crate` log shows `Compiling iscc-ffi` BEFORE
    `Compiling iscc-lib` (the build-script unit was dirty). Read that line, not the job colour.
- The check step runs `cargo run --features bindgen`, which rebuilds `libiscc_uniffi.so` with that
    feature, so the Gradle tests after it load that build. Same exports; noted, not filed.
- `maturin develop` after `cargo clippy`/`cargo test` recompiles `pyo3-ffi`, `pyo3` and `iscc-py`
    (about 15 s, then 0.3 s when repeated): the pre-push batch took 60 s in total.

## release.yml rewiring step (285, ~25 min, PASS_WITH_NOTES)

- **What a green static gate cannot say, and where to read it instead:** an action input's real
    behaviour is in the action SOURCE (`gh api "repos/<o>/<r>/contents/src/<f>.ts?ref=<tag>"`):
    `softprops/action-gh-release@v3` with `overwrite_files: false` skips by asset NAME and never
    checks the asset state, so a `starter` (half-uploaded) asset survives a rerun. Codex found it;
    read every recovery path of a new input before the handoff.
- **A tool version on a runner is in the image readme**
    (`gh api repos/actions/runner-images/contents/images/macos/macos-15-arm64-Readme.md`): a script
    run as bare `python3` there got 3.15, so `itertools.pairwise` (3.10+) is fine.
- **`failure()` is transitive**: compute each job's ancestor set with PyYAML to see which chains a
    new `needs` edge couples, and to check any "every job waits for X" sentence in the specs
    (`ci-cd.md` → Job Conditions, `swift-bindings.md` → Release Workflow Integration,
    `docs/development.md` → release section). A rewiring step leaves those stale: realign the
    sub-spec facts in the review.
- **vcpkg port end to end (31 s)**: `sudo apt-get install -y zip cmake ninja-build`, shallow clone
    `microsoft/vcpkg` into `/tmp`, `bootstrap-vcpkg.sh -disableMetrics`, then
    `vcpkg install iscc --overlay-ports=<dir with only portfile.cmake + vcpkg.json>`. It installs
    only while the release named in `vcpkg.json` exists, so it cannot be a CI check on `develop`.
