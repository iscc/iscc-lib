# Handoff

## 2026-07-27 — Make the `specs/ci-cd.md` CI job table exhaustive and gate it against `ci.yml`

**Done:** Rewrote the "CI Workflow — Quality Gates" table in `specs/ci-cd.md` to one row per
`ci.yml` job key — all 21, first column is the literal backticked key, descriptions re-derived from
each job's steps (the unverified "49 tests" java claim is gone). Added the matrix/aggregator note
(21 keys → 22 check names) and fixed the stale L35-37 action-pin prose by dropping version numbers.
Added `scripts/check_ci_job_table.py` as a parity gate (prek hook `check-ci-job-table` + pytest
anchor) so the table cannot drift again.

**Files changed:**

- `.claude/context/specs/ci-cd.md`: 21-row job table (backticked `ci.yml` keys), matrix/aggregator
    sentence, gate pointer sentence, de-versioned action-set prose
- `scripts/check_ci_job_table.py`: new checker — `yaml.safe_load` on `ci.yml` `jobs:` keys vs
    regex-scanned backticked first-column keys, scoped to the `## CI Workflow — Quality Gates`
    section only; reports all missing/nonexistent rows before exiting 1; count floor 10 on both
    sides; `Path` args (`ci_yml=`, `spec_md=`) on every function; `OK:` line with job count
- `tests/test_check_ci_job_table.py`: 8 tests — real-repo anchor (floor + zero errors + `main()` ==
    0), consistent fixture (with a backticked row in a *later* section proving anchoring), missing
    row, nonexistent-job row, count floor on both sides, `main()` exit 1, missing heading and
    missing `jobs:` mapping hard errors
- `.pre-commit-config.yaml`: hook `check-ci-job-table` (`language: system`, `pass_filenames: false`,
    `files:` alternation over `ci.yml` + `ci-cd.md`), with the check-docs-nav-style comment that
    `files:` never sees deletions — the pytest anchor covers that

**Verification:** (all this session)

- `uv run scripts/check_ci_job_table.py` → exit 0, prints the `OK:` line naming 21 jobs compared —
    re-run after mdformat normalized the table, still exit 0
- Throwaway-tree probe (`git archive HEAD | tar -x -C /tmp/ci-tbl`, new/changed files copied in):
    baseline exit 0; deleting the `kotlin` row → exit 1 with
    `error: spec table: missing 1 job row(s): ['kotlin']`; row restored → exit 0
- `uv run pytest -q tests/test_check_ci_job_table.py` → 8 passed
- `uv run prek run check-ci-job-table --files .github/workflows/ci.yml` → Passed;
    `--files .claude/context/specs/ci-cd.md` → Passed (neither Skipped)
- `mise run check` → all 18 hooks Passed, `git status --short` unchanged after the run
- Pre-push gates: `uv run ty check` → clean; `uv run pytest --timeout=120` → 440 passed (432 + 8
    new) with coverage enforcement green; ruff S/C901 are project-wide selects, covered by the
    passing `Ruff linting` hook

**Next:** The authorized dependency majors, one per step, starting with xunit 3.x /
`Microsoft.NET.Test.Sdk` 18.x in `packages/dotnet` (per the issue's ordering), then Gradle wrapper
8.12.1, JUnit 6.x, and the riskier `jni` 0.22 / `magnus` 0.8 migrations.

**Notes:**

- The spec prose now hardcodes "21 job keys / 22 check names" (next.md explicitly asked for that
    sentence). The *gate* does not pin the count — only a ≥10 floor — so a future job addition reds
    only if the table row is missing; the prose number will need a manual touch-up then, which the
    same-commit table edit makes hard to miss.
- Section regex accepts end-of-file as a section terminator (`(?=^## |\Z)`) so the gate keeps
    working if the table section ever becomes the last one in the file.
- Deliberately not gated (per next.md Not-In-Scope): other `ci-cd.md` tables, `release.yml` /
    `docs.yml` job lists, and the "no floating branch ref" assertion idea (needs Titusz's sign-off).
- issues.md untouched: the `[human]` issue "Make the CI job table in `specs/ci-cd.md` exhaustive" is
    implemented by this step — review verifies and resolves the ledger entry.
