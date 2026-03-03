## 2026-03-03 — Add CI feature matrix testing for issue #16

**Done:** Added 5 new steps to the `rust` job in `.github/workflows/ci.yml`: two clippy feature-flag
checks (`--no-default-features`, `--all-features`) and three test steps (`--no-default-features`,
`--all-features`, `--no-default-features --features text-processing`). This is a YAML-only change
targeting `iscc-lib` feature combinations.

**Files changed:**

- `.github/workflows/ci.yml`: Added 5 steps to the `rust` job — 2 clippy checks and 3 test runs for
    feature flag combinations

**Verification:**

- [x] YAML valid: `python -c "import yaml; yaml.safe_load(...)"` exits 0
- [x] `grep -c 'no-default-features'` returns 3 (2 test + 1 clippy = 3 lines with the flag)
- [x] `grep -c 'all-features'` returns 2 (1 test + 1 clippy)
- [x] `grep 'text-processing'` shows the text-processing-only test step
- [x] Only `.github/workflows/ci.yml` modified (no Rust source changes)
- [x] `mise run check` — all 14 pre-commit hooks pass

**Next:** Issue #16 should now be fully resolved. The next step could be closing issue #16 and
moving on to the next target item, or verifying CI passes on the pushed commit before closing.

**Notes:** The `no-default-features` grep count is 3 (not 2) because there are 3 lines containing
that flag: one clippy step, one test step, and the text-processing test step (which also uses
`--no-default-features --features text-processing`). The verification criteria in next.md said "at
least 2", so this is correct. No surprises or shortcuts taken.
