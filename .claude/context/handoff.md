## 2026-02-25 — Review of: Add selective publishing inputs to release.yml

**Verdict:** PASS

**Summary:** Clean, minimal implementation adding three `workflow_dispatch` boolean inputs and `if:`
conditions to all 8 jobs in `release.yml`. Exactly matches the spec in `specs/ci-cd.md`. No changes
to permissions, concurrency, triggers, or build logic — scope discipline is perfect.

**Verification:**

- [x] `grep -q 'crates-io:' .github/workflows/release.yml` exits 0 — input defined
- [x] `grep -q 'pypi:' .github/workflows/release.yml` exits 0 — input defined
- [x] `grep -c 'inputs\.' .github/workflows/release.yml` returns 8 — all jobs have conditions
- [x] `grep -q "inputs.crates-io" .github/workflows/release.yml` exits 0 — crates.io condition
    present
- [x] `grep -q "inputs.pypi" .github/workflows/release.yml` exits 0 — PyPI condition present
- [x] `grep -q "inputs.npm" .github/workflows/release.yml` exits 0 — npm condition present
- [x] `python3 -c "import yaml; yaml.safe_load(...)"` exits 0 — valid YAML
- [x] `mise run check` — all 14 quality gate hooks pass

**Issues found:**

- (none)

**Next:** The second critical release issue: add idempotency checks to each publish job so
re-publishing an existing version skips gracefully instead of failing the workflow. This is the last
blocker for the first `v0.0.1` publish.

**Notes:** Resolved `[critical] Implement selective publishing in release.yml` issue and updated the
CI/CD spec verification criteria to reflect the 5 newly-passing selective publishing checkboxes.
