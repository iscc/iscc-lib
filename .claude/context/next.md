# Next Work Package

## Step: Dependency refresh slice 4 — GitHub Actions versions in ci.yml + docs.yml

## Goal

Bring the pinned GitHub Actions in the two non-release workflows up to their current major versions
(most are 1–3 majors behind and still on the node20 runtime), continuing the `normal` `[human]`
issue "Dependency review and refresh across the project" (spec: `.claude/context/specs/ci-cd.md` →
"Dependency Freshness"). `ci.yml` changes are directly verified by the CI run on push;
`.github/workflows/release.yml` is deliberately deferred to its own slice.

## Scope

- **Create**: none
- **Modify**: `.github/workflows/ci.yml`, `.github/workflows/docs.yml` (2 non-test, non-doc files)
- **Reference**: `.claude/context/specs/ci-cd.md` (§ "Dependency Freshness"),
    `.claude/context/issues.md` (dependency issue + its progress log), `.claude/context/handoff.md`
    (slice-4 risk notes), `Cargo.toml` (`# held:` comment style to mirror if any action must be held
    back)

## Not In Scope

- **`.github/workflows/release.yml` — do not touch.** It is the next slice on its own: 97 `uses:`
    refs, and its `actions/upload-artifact@v4` ↔ `actions/download-artifact@v4` pairs must be bumped
    together, with no way to verify short of a real release run.
- **`.pre-commit-config.yaml` — do not touch.** Both pinned repos were checked against upstream on
    2026-07-24 and are already current: `pre-commit/pre-commit-hooks` `v6.0.0` is the latest release
    and `executablebooks/mdformat` `1.0.0` is the latest tag (= PyPI 1.0.0). There is no bump to
    make, so there is no mdformat reformat wave in this step.
- **`mise.toml`** — has no `[tools]` section; nothing to pin.
- Do not pin actions to commit SHAs, add `.github/dependabot.yml` / `renovate.json`, or introduce
    any new job, step, matrix entry, or permission.
- Do not change runtime versions selected by the actions (`node-version: '20'`,
    `python-version: '3.10' / '3.14' / '3.12'`, `go-version-file`, JDK/.NET versions) — those are
    support-policy decisions, not dependency pins.
- Do not touch `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`,
    `taiki-e/install-action@v2`, `ruby/setup-ruby@v1`, `obi1kenobi/cargo-semver-checks-action@v2` —
    all already on their latest major line (verified 2026-07-24).

## Implementation Notes

Mechanical version bumps only. Upstream latest majors were surveyed via
`gh api repos/<owner>/<repo>/releases/latest` on 2026-07-24; each major's release notes were read
and the breaking changes evaluated against actual usage in these two files.

**`.github/workflows/ci.yml`** (67 `uses:` lines total — 9 distinct refs change):

| Action                              | From  | To    | N   | Evaluation                                                                                                                                                                                                                                                                            |
| ----------------------------------- | ----- | ----- | --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `actions/checkout`                  | `@v4` | `@v7` | 18  | v5/v6 = node24 + creds persisted to a separate file; v7 blocks fork checkout for `pull_request_target`/`workflow_run` — neither trigger is used here, no job pushes                                                                                                                   |
| `actions/setup-python`              | `@v5` | `@v7` | 2   | v6 = node24; v7 removed the `pip-install` input (unused). `allow-prereleases: true` is still supported — keep it                                                                                                                                                                      |
| `astral-sh/setup-uv`                | `@v4` | `@v9` | 1   | v5 enables cache by default, v6 changed `activate-environment`/`working-directory` defaults, v7 dropped `server-url`, v8 dropped the old `manifest-file` format, v9 sets `prune-cache: false`. We pass **no inputs** and drive everything through `uv sync` / `uv run`, so none apply |
| `actions/setup-node`                | `@v4` | `@v7` | 1   | v5+ auto-caching only triggers when `package.json` declares `packageManager`/`devEngines.packageManager`; `crates/iscc-napi/package.json` declares neither, so no lockfile lookup happens (its `package-lock.json` is gitignored)                                                     |
| `actions/setup-java`                | `@v4` | `@v5` | 2   | node24 only                                                                                                                                                                                                                                                                           |
| `actions/setup-go`                  | `@v5` | `@v7` | 1   | v6 = node24 + stricter toolchain selection; we use `go-version-file: packages/go/go.mod` (`go 1.26.1`), which stays supported                                                                                                                                                         |
| `actions/setup-dotnet`              | `@v4` | `@v6` | 1   | node24 only                                                                                                                                                                                                                                                                           |
| `actions/upload-artifact`           | `@v4` | `@v7` | 2   | node24 + `@actions/artifact` v4 backend. Safe because ci.yml has **zero** `download-artifact` steps — no cross-major pairing inside this workflow                                                                                                                                     |
| `github/codeql-action/upload-sarif` | `@v3` | `@v4` | 1   | v4 is the current line (latest tag `v4.37.3`); v3 is on the deprecation path                                                                                                                                                                                                          |

Do **not** add `package-manager-cache: false` to the `setup-node` step preemptively — only if that
job actually fails with "Dependencies lock file is not found".

**`.github/workflows/docs.yml`** (5 `uses:` lines, all change): `actions/checkout@v4` → `@v7`,
`actions/setup-python@v5` → `@v7`, `astral-sh/setup-uv@v4` → `@v9`,
`actions/upload-pages-artifact@v3` → `@v5`, `actions/deploy-pages@v4` → `@v5`.

- `upload-pages-artifact` v4 stopped including hidden files in the artifact. Checked: the built
    `site/` tree contains no dotfiles (only `404.html`, `CNAME`, `index.html`, `llms.txt`,
    `objects.inv`, `assets/`, …), and this project does not use `.nojekyll` (artifact-based Pages
    deploys never run Jekyll), so the exclusion is a no-op here.
- `upload-pages-artifact@v5` + `deploy-pages@v5` are the matching current pair (v5 of the uploader
    moved to `upload-artifact` v7; `deploy-pages` v5 is node24). Bump both or neither.
- `docs.yml` only runs on push to `main`, so this file is **not** exercised by the develop CI run —
    verification for it is static (YAML parses, expected refs present, step count unchanged).

**If any single bump turns out to be unsafe**, hold that one ref back at its current version and add
a `# held: <reason>` YAML comment on the line above it, mirroring the `# held:` convention now used
in the root `Cargo.toml`. Do not weaken or delete a step to make a bump work.

Run `mise run format` before staging (yamlfix/mdformat normalize these files), then commit and push
so the CI run can validate the ci.yml changes.

## Verification

- `python3 -c "import yaml;[yaml.safe_load(open(f)) for f in ['.github/workflows/ci.yml','.github/workflows/docs.yml']]"`
    exits 0
- No stale refs remain in the two touched files — this command prints nothing (exit 1):
    `grep -nE 'actions/checkout@v[1-6]|actions/setup-python@v[1-6]|astral-sh/setup-uv@v[1-8]|actions/setup-node@v[1-6]|actions/setup-java@v[1-4]|actions/setup-go@v[1-6]|actions/setup-dotnet@v[1-5]|actions/upload-artifact@v[1-6]|codeql-action/upload-sarif@v[1-3]|upload-pages-artifact@v[1-4]|deploy-pages@v[1-4]' .github/workflows/ci.yml .github/workflows/docs.yml`
- Expected new refs present: `grep -c 'actions/checkout@v7' .github/workflows/ci.yml` → **18** and
    `grep -c 'actions/checkout@v7' .github/workflows/docs.yml` → **1**
- Step count unchanged: `grep -c 'uses:' .github/workflows/ci.yml` → **67**;
    `grep -c 'uses:' .github/workflows/docs.yml` → **5**
- `release.yml` left alone and internally consistent:
    `grep -c 'actions/checkout@v4' .github/workflows/release.yml` → **23**,
    `grep -c 'actions/upload-artifact@v4' .github/workflows/release.yml` → **11**,
    `grep -c 'actions/download-artifact@v4' .github/workflows/release.yml` → **20**
- `.pre-commit-config.yaml` untouched: `grep -c 'rev: v6.0.0' .pre-commit-config.yaml` → **1** and
    `grep -c 'rev: 1.0.0' .pre-commit-config.yaml` → **1**
- `mise run check` exits 0 with no file left rewritten (`git status --porcelain` shows only the
    intended changes)
- CI on the pushed develop commit is fully green:
    `gh api repos/iscc/iscc-lib/commits/<sha>/check-runs --jq '[.check_runs[]|select(.conclusion!="success")]|length'`
    → **0**, with all 20 CI jobs (including `Coverage + CRAP`, `Perf (iai-callgrind)`,
    `Audit (cargo-deny)`, `Semver`) present

## Done When

`ci.yml` and `docs.yml` reference only current-major GitHub Actions, every grep assertion above
holds on the working tree, `mise run check` exits 0, and the pushed develop commit shows 0
non-success CI check-runs.
