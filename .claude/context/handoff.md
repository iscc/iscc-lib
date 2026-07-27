# Handoff

## 2026-07-27 — Review of: make the `specs/ci-cd.md` CI job table exhaustive and gate it

**Verdict:** PASS_WITH_NOTES

**Summary:** The job table now carries one backticked row per `ci.yml` job key — all 21 — and every
description was re-derived from that job's steps (I checked all 21 against `ci.yml`; all accurate,
the unverified "49 tests" java claim is gone). `scripts/check_ci_job_table.py` is a real gate: it
reds on the pre-change table, on a deleted row, and on a job added to `ci.yml` without a row. Scope
was exactly 2 non-test non-doc files; `ci.yml` and `issues.md` untouched as required.

**Verification:**

- [x] `uv run scripts/check_ci_job_table.py` exits 0 —
    `OK: 21 CI jobs consistent between ci.yml and specs/ci-cd.md.`
- [x] Gate fires on a real break — throwaway tree (`git archive HEAD | tar -x`): deleting the
    `kotlin` row → exit 1, `missing 1 job row(s): ['kotlin']`; restored → exit 0. Also probed:
    adding a job to `ci.yml` with no row → exit 1 naming it; and the **pre-change** spec
    (`git show HEAD~1:…`) → floor error + all 21 missing, i.e. the gate catches the drift it exists
    for
- [x] `uv run pytest -q tests/test_check_ci_job_table.py` — 9 passed (8 + 1 I added)
- [x] `prek run check-ci-job-table --files .github/workflows/ci.yml` and
    `--files .claude/context/specs/ci-cd.md` both `Passed`; an unrelated file `Skipped` (scoping
    correct)
- [x] `mise run check` — 18 hooks Passed, no file modified. Pre-push preflight
    (`prek run --hook-stage pre-push --all-files`) exit 0, 441 tests

**Issues found:**

- **Fixed in review:** set parity silently deduplicated rows, so a job listed twice passed while the
    table's "exactly one row per key" invariant was violated (Codex P2). `spec_job_rows()` now keeps
    row order and `run_checks` reports duplicates; test added, probed on a real duplicated `kotlin`
    row → exit 1.
- Nit (not filed): `main()` re-parses `ci.yml` to print the count on success.
- Resolved and deleted from issues.md: `[human]` "Make the CI job table in `specs/ci-cd.md`
    exhaustive". Its `**Spec:**` target is the table itself, which this step rewrote.

**Codex review:** one finding, the duplicate-row hole above — accepted and fixed. No others.

**Next:** The authorized dependency majors, **one per step**: start with `xunit` 3.x +
`Microsoft.NET.Test.Sdk` 18.x in `packages/dotnet/*/*.csproj` (xunit 3 is a real behaviour change —
new runner model, tightened `Assert` API — so expect the test project to need edits, and verify the
`dotnet` CI job's exact invocation still works). Then Gradle wrapper 8.12.1 / JUnit 6.x, then the
`jni` 0.22 and `magnus` 0.8 API migrations, one crate per step.

**Notes:**

- The spec prose hardcodes "21 job keys → 22 check names"; the gate deliberately pins no count (only
    a ≥10 floor). A future job addition reds the gate via the missing row, and the prose number must
    be corrected by hand in that same edit.
- Section anchoring is load-bearing on the real file, not just the fixture: `ci-cd.md` has 14
    backticked first-column rows under `## Version Management`, which an unanchored scan would read
    as bogus job rows. The baseline exit 0 is itself the proof the anchor works.
- No quality-gate weakening anywhere in the unpushed range (the only `continue-on-error` match is
    the table accurately describing the pre-existing informational `semver` job).
