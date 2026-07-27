# Next Work Package

## Step: Make the `specs/ci-cd.md` CI job table exhaustive and gate it against `ci.yml`

## Goal

Rewrite the CI job table in `.claude/context/specs/ci-cd.md` so it lists all 21 job keys of
`.github/workflows/ci.yml` (it lists 14), and add a parity gate so the table cannot drift again —
picking up the `[human]` issue "Make the CI job table in `specs/ci-cd.md` exhaustive", whose text
explicitly leaves the `check_docs_nav.py`-style gating to this step's judgment.

## Alternatives Considered

- **Chosen:** exhaustive table + parity gate — the only `normal` non-dependency item, zero runtime
    risk, and the table has already drifted through three iterations, so the gate is what makes the
    fix durable rather than a snapshot.
- **Rejected:** the `xunit` 3.x / `Microsoft.NET.Test.Sdk` 18.x major bump — also authorized, but a
    real behaviour change in a published test surface; better taken with the spec baseline correct
    and after this documentation-only step, per the issue's "one per step" rule.

## Scope

- **Create**: `scripts/check_ci_job_table.py`, `tests/test_check_ci_job_table.py`
- **Modify**: `.claude/context/specs/ci-cd.md`, `.pre-commit-config.yaml`
- **Reference**: `.github/workflows/ci.yml` (the 21 `jobs:` keys and their steps — the sole source
    for the descriptions), `scripts/check_docs_nav.py` + `tests/test_check_docs_nav.py` (the
    checker/test pattern to follow), `scripts/check_release_workflow.py` (pyyaml usage)

File budget: 2 non-test, non-doc files (`scripts/check_ci_job_table.py`, `.pre-commit-config.yaml`).

## Not In Scope

- Any edit to `.github/workflows/ci.yml` itself — no new job, no rename, no reordering. The table
    describes CI as it is; a red gate here means the table is wrong, not the workflow.
- A new CI job for the checker. The pytest anchor test carries it into CI via `python-test`, which
    is the established pattern (`check_docs_nav.py`).
- Gating any other table in `ci-cd.md` (workflow files, auth, build matrices) or `release.yml` /
    `docs.yml` job lists.
- The handoff's "no floating branch ref" assertion in `scripts/check_release_workflow.py` — a new
    policy that needs Titusz's sign-off.
- Editing `issues.md`. The review agent resolves the issue after verifying the fix.

## Implementation Notes

- **Table shape:** first column is the literal `ci.yml` job key in backticks (that is what makes the
    table mechanically checkable), second column what the job checks. Exactly one row per key, all
    21\. Derive each description from that job's steps in `ci.yml` — do not carry over a stale
    description or write one from memory (e.g. the `java` row's "49 tests" claim is unverified).
- Add one sentence under the table for the two shapes a reader cannot infer from a key list:
    `python-test` is a `[3.10, 3.14]` matrix and `python` is its `if: always()` aggregator, so 21
    keys surface as 22 check names.
- **Also fix the stale prose at `ci-cd.md` L35-37**: it names `astral-sh/setup-uv@v4`,
    `actions/setup-python@v5`, `actions/setup-node@v4`; the real pins are `@v9.0.0`, `@v7`, `@v7`.
    Drop the version numbers from the prose rather than re-pinning them — versions belong in
    `ci.yml`, and re-stating them just re-arms the same drift.
- **Checker:** `yaml.safe_load` on `ci.yml` for `jobs:` keys (`pyyaml` is a dev-group dep, so it is
    importable both under `uv run` and in-process from pytest); regex over the table section only
    for the spec side — anchor the scan between the `## CI Workflow — Quality Gates` heading and the
    next `## ` heading so other tables cannot feed it rows. No `tomllib`: CI's `python-test` matrix
    includes Python 3.10.
- Report every mismatch (missing rows and rows naming a nonexistent job) before exiting non-zero,
    and print an `OK: …` line with the compared job count on success.
- **Count floor:** a set-equality gate passes vacuously on two empty sets — fail loudly if either
    side yields fewer than 10 keys. Do not make the exact number (21) a pass/fail condition; it
    drifts with every legitimate CI change.
- Give the core function explicit `Path` arguments (`ci_yml=`, `spec_md=`) rather than reading
    module constants, so tests and review can point it at throwaway copies and mutate them.
- **prek wiring:** new hook `check-ci-job-table`, `language: system`,
    `entry: uv run scripts/check_ci_job_table.py`, `pass_filenames: false`, `files:` matching both
    `^\.github/workflows/ci\.yml$` and `^\.claude/context/specs/ci-cd\.md$`. Mirror the
    `check-docs-nav` comment noting that a `files:`-scoped hook never sees deletions, which is why
    the pytest anchor exists.
- **Tests:** load the script via `importlib.util.spec_from_file_location` (see
    `tests/test_check_docs_nav.py`); anchor one test on the real tracked files, and cover at least
    the three failure modes — a row missing, a row naming a job that does not exist, and the count
    floor tripping.

## Verification

- `uv run scripts/check_ci_job_table.py` exits 0 and prints an `OK: …` line naming the number of
    jobs compared.
- The gate fires on a real break: in a throwaway tree (`git archive HEAD | tar -x -C /tmp/ci-tbl`),
    deleting one table row makes the checker exit non-zero and name that job key; restoring it
    returns exit 0.
- `uv run pytest -q tests/test_check_ci_job_table.py` passes.
- `uv run prek run check-ci-job-table --files .github/workflows/ci.yml` and
    `... --files .claude/context/specs/ci-cd.md` both report `Passed` (not `Skipped`).
- `mise run check` — all hooks pass, no file modified by the run.

## Done When

The CI job table names every `ci.yml` job key, the parity gate is wired into prek and pytest, and
all five verification criteria pass.
