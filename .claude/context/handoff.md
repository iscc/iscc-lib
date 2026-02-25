## 2026-02-25 — Add selective publishing inputs to release.yml

**Done:** Added three boolean `workflow_dispatch` inputs (`crates-io`, `pypi`, `npm`) and `if:`
conditions to all 8 jobs in `release.yml`. Tag pushes activate all jobs as before; manual dispatch
allows selecting individual registries via checkboxes in the GitHub Actions UI.

**Files changed:**

- `.github/workflows/release.yml`: Added `inputs:` block under `workflow_dispatch:` with three
    boolean inputs. Added `if: startsWith(github.ref, 'refs/tags/v') || inputs.<registry>` to all 8
    jobs (`publish-crates-io`, `build-wheels`, `build-sdist`, `publish-pypi`, `build-napi`,
    `build-wasm`, `publish-npm-lib`, `publish-npm-wasm`). Updated existing tag-only conditions on
    `publish-npm-lib` and `publish-npm-wasm` to include `|| inputs.npm`.

**Verification:**

- [x] `grep -q 'crates-io:' .github/workflows/release.yml` — input defined
- [x] `grep -q 'pypi:' .github/workflows/release.yml` — input defined
- [x] `grep -c 'inputs\.' .github/workflows/release.yml` returns 8 — all jobs have conditions
- [x] `grep -q "inputs.crates-io"` — crates.io condition present
- [x] `grep -q "inputs.pypi"` — PyPI condition present
- [x] `grep -q "inputs.npm"` — npm condition present
- [x] `python3 -c "import yaml; yaml.safe_load(...)"` — valid YAML
- [x] `mise run check` — all 14 quality gate hooks pass

**Next:** The second critical release issue: add idempotency checks to each publish job so
re-publishing an existing version skips gracefully. This is the remaining blocker for the first
`v0.0.1` publish.

**Notes:** Straightforward change — no surprises. The `permissions`, `concurrency`, trigger events,
and build step logic were left untouched as specified. No tests needed since this is a workflow
configuration change (verified structurally via grep and YAML parsing).
