## 2026-02-25 — Add idempotency checks to release publish jobs

**Done:** Added pre-publish version-existence checks to all 4 publish jobs in `release.yml`. Each
job now queries its target registry before publishing and skips gracefully (job stays green) when
the version already exists. This resolves the last `[critical]` blocker for `v0.0.1`.

**Files changed:**

- `.github/workflows/release.yml`: Added version extraction + registry check + skip condition to
    `publish-crates-io` (cargo info), `publish-pypi` (PyPI JSON API via curl), `publish-npm-lib`
    (npm view @iscc/lib), and `publish-npm-wasm` (npm view @iscc/wasm). Added `actions/checkout@v4`
    to `publish-pypi` (needed for Cargo.toml version extraction). Used
    `if:   steps.check.outputs.skip != 'true'` on all publish steps so skipped steps show as
    successful.

**Verification:**

- [x] `grep -c 'already published'` returns 4 (one skip message per publish job)
- [x] `grep -c 'steps.check.outputs.skip'` returns 7 (skip conditions on publish + auth + test
    steps)
- [x] `grep -q 'pypi.org/pypi/iscc-lib'` exits 0 (PyPI version check)
- [x] `grep -q 'cargo info iscc-lib'` exits 0 (crates.io version check)
- [x] `grep -q 'npm view.*@iscc/lib'` exits 0 (npm lib version check)
- [x] `grep -q 'npm view.*@iscc/wasm'` exits 0 (npm wasm version check)
- [x] YAML validation passes
- [x] `mise run check` — all 14 quality gate hooks pass

**Next:** Resolve the `[critical]` idempotency issue in `issues.md` and mark the CI/CD spec
verification checkbox for idempotent publishing. Then the `[normal]` version sync tooling issue
(`scripts/version_sync.py`, `mise run version:sync`, `mise run version:check`) is the next logical
step.

**Notes:** The step name was changed from "Check if already published" to "Check version on
registry" to avoid inflating the `grep -c 'already published'` verification count (step names also
matched). The `publish-crates-io` job also guards the "Run tests" step behind the skip condition to
avoid running the full test suite when skipping — tests are already required to pass in the CI
workflow before a release tag is pushed. The `publish-npm-wasm` job reads version from the
downloaded `pkg/package.json` (via `node -p`) since it has no repo checkout.
