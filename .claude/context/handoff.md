## 2026-02-25 — Review of: Add idempotency checks to release publish jobs

**Verdict:** PASS

**Summary:** All 4 publish jobs in `release.yml` now have pre-publish version-existence checks that
skip gracefully when the version already exists on the target registry. The implementation matches
the spec exactly and resolves the last `[critical]` blocker for `v0.0.1`. Clean, minimal diff
touching only the intended file.

**Verification:**

- [x] `grep -c 'already published'` returns 4 — one skip message per publish job
- [x] `grep -c 'steps.check.outputs.skip'` returns 7 — skip conditions on publish + auth + test
    steps (exceeds minimum of 4)
- [x] `grep -q 'pypi.org/pypi/iscc-lib'` exits 0 — PyPI version check present
- [x] `grep -q 'cargo info iscc-lib'` exits 0 — crates.io version check present
- [x] `grep -q 'npm view.*@iscc/lib'` exits 0 — npm lib version check present
- [x] `grep -q 'npm view.*@iscc/wasm'` exits 0 — npm wasm version check present
- [x] YAML validation passes — `check yaml` hook and `yaml.safe_load` both succeed
- [x] `mise run check` — all 14 quality gate hooks pass

**Issues found:**

- (none)

**Resolved:**

- Deleted `[critical] Add idempotency checks to release publish jobs` from issues.md
- Checked spec verification criterion in `specs/ci-cd.md` (line 274): "Publishing an existing
    version skips gracefully instead of failing"

**Next:** The `[normal]` version sync tooling issue is the next logical step — create
`scripts/version_sync.py`, `mise run version:sync`, and `mise run version:check` tasks. This is the
last unchecked item in the Release/Version Sync verification section of `specs/ci-cd.md`. After
that, the remaining `[low]` issues (TypeScript port evaluation, JNI exception mapping, WASM
CLAUDE.md staleness) can be addressed.

**Notes:** No `[critical]` issues remain. The project is clear for the `v0.0.1` release once the
version sync tooling (normal priority) is in place. The advance agent also wisely guarded the "Run
tests" step in `publish-crates-io` behind the skip condition — avoids redundant test execution when
skipping the publish.
