# Next Work Package

## Step: Pin `rubygems/configure-rubygems-credentials` to the exact tag `@v2.1.0`

## Goal

Replace the only `@main` action reference in the repository — the OIDC credential step in the
RubyGems publish job — with the exact tag `@v2.1.0` plus an inline `# exact tag:` comment, closing
the RULED `normal` issue "Pin `rubygems/configure-rubygems-credentials` off the `@main` branch"
before the pending v0.6.0 release runs that job.

## Alternatives Considered

- **Chosen:** the one-line pin — it is RULED and authorized, it removes an unreviewed-upstream-head
    trust anchor from the release pipeline that a v0.6.0 dispatch would actually execute, and its
    tiny diff keeps this iteration's push attributable: the push carries three commits CI has never
    seen (`acf178a`, `e38c17e`, `c7cc0bc` — 785 lines of rewritten `tools/cid.py` /
    `tests/test_cid.py`) to the gates for the first time.
- **Rejected:** making the `specs/ci-cd.md` job table exhaustive (14 rows vs 21 real jobs) — also
    authorized, but a larger diff landing on the same push that first exposes the rewritten runner
    to CI, and it changes no executable behaviour. Next step if this one passes.

## Scope

- **Modify**: `.github/workflows/release.yml` (one `uses:` line + a preceding comment)
- **Reference**: `.github/workflows/ci.yml` lines 61-63 (the `# exact tag:` comment precedent),
    `.pre-commit-config.yaml` (the `check-release-workflow` hook),
    `scripts/check_release_workflow.py`

## Not In Scope

- **Do not SHA-pin.** `decisions.md` 2026-07-26 ruled option (a): exact tag, tags-never-SHAs stands.
- Do not touch the other non-version refs in `release.yml` (`dtolnay/rust-toolchain@stable`,
    `pypa/gh-action-pypi-publish@release/v1`, `ruby/setup-ruby@v1`,
    `oxidize-rb/actions/cross-gem@v1`, `nttld/setup-ndk@v1`, `PyO3/maturin-action@v1`,
    `rust-lang/crates-io-auth-action@v1`) — they are deliberate upstream-published pointers.
- Do not add a "no floating branch ref" check to `scripts/check_release_workflow.py` or anywhere
    else. Distinguishing `@main` from `@stable` is a new policy gate and needs Titusz's sign-off.
- Do not edit `.claude/context/issues.md` — review owns issue resolution.
- Do not start the `specs/ci-cd.md` job table or any dependency major bump.
- Do not rebase, reset or amend the three unpushed commits; leave `.claude/skills/release/SKILL.md`
    alone (release-job *gating* is unchanged).

## Implementation Notes

- Target is `.github/workflows/release.yml` line 895, inside the `publish-rubygems` job's step
    `Configure RubyGems credentials (OIDC trusted publishing)`. The step passes **no `with:` keys**,
    so this is purely a trust-anchor choice with zero input-compatibility risk.
- The action publishes **only exact tags** — `git/matching-refs/tags` returns `v1.0.0`, `v2.0.0`,
    `v2.1.0` and nothing else, so `@v2` does not resolve. That is exactly the `astral-sh/setup-uv`
    situation, so mirror its comment shape: a short `# exact tag: …` comment (why the major tag is
    not used) on the line(s) directly above the `uses:` line, indented to match the step body.
- Nothing functional is lost: `v2.1.0` was published 2026-06-12 ("Switch to Node 24 and adopt
    pending major dependency updates"); `main`'s head (2026-06-24) is a single transitive
    `yaml 2.8.3 → 2.9.0` dependabot bump.
- `https://raw.githubusercontent.com/rubygems/configure-rubygems-credentials/v2.1.0/action.yml`
    returns HTTP 200 and declares `gem-server` / `audience` / `role-to-assume`, so the
    `--check-action-inputs` fetch keeps resolving the ref after the change (it splits the ref on the
    first `@` to build that URL) — the resolved count must not drop.
- Run `mise run format` before committing; yamlfix owns this file and must leave the new comment
    untouched.

## Verification

- `grep -c 'uses: rubygems/configure-rubygems-credentials@v2\.1\.0' .github/workflows/release.yml`
    prints `1`, and `grep -c 'uses: .*@main' .github/workflows/release.yml` prints `0`
- `uv run scripts/check_release_workflow.py` exits 0
- `uv run --no-project --with pyyaml python scripts/check_release_workflow.py --check-action-inputs`
    (the `ci.yml` `release-workflow` job invocation) exits 0, emits **no** `warning: skipped` line,
    and prints an `action-inputs: resolved <T> of <T> action refs (0 skipped)` summary
- `uv run pytest -q tests/test_check_release_workflow.py` passes
- `uv run prek run --files .github/workflows/release.yml` — every hook Passed and no file was
    modified by a hook
- `go run github.com/rhysd/actionlint/cmd/actionlint@v1.7.7 .github/workflows/release.yml` exits 0
    with no output

## Done When

`release.yml` pins the RubyGems credential action to `@v2.1.0` with an explanatory `# exact tag:`
comment, no `@main` reference remains in the file, and all six checks above pass.
