# Next Work Package

## Step: Add selective publishing inputs to release.yml

## Goal

Add `workflow_dispatch` boolean inputs to `release.yml` so each registry (crates.io, PyPI, npm) can
be published independently via the GitHub Actions UI. This is the first of two critical release
readiness issues and unblocks the first `v0.0.1` publish.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml`
- **Reference**: `.claude/context/specs/ci-cd.md` (trigger configuration and job conditions
    sections), `.claude/context/issues.md` (issue description)

## Not In Scope

- Idempotency checks (version-exists skipping) — that is a separate critical issue to be addressed
    in the next step
- Version sync tooling (`scripts/version_sync.py`, `mise.toml` tasks) — separate `[normal]` issue
- Any changes to `ci.yml` or `docs.yml`
- OIDC configuration on registry websites (crates.io, PyPI) — those are manual steps outside this
    codebase
- Testing the workflow by actually triggering it (we verify structure only)

## Implementation Notes

The spec in `.claude/context/specs/ci-cd.md` defines the exact target YAML structure. Follow it
precisely:

1. **Add `inputs:` block** under the existing `workflow_dispatch:` key:

    ```yaml
    workflow_dispatch:
      inputs:
        crates-io:
          description: Publish iscc-lib to crates.io
          type: boolean
          default: false
        pypi:
          description: Publish iscc-lib to PyPI
          type: boolean
          default: false
        npm:
          description: Publish @iscc/lib and @iscc/wasm to npm
          type: boolean
          default: false
    ```

2. **Add `if:` conditions** to each job (use the exact expressions from the spec):

    - `publish-crates-io`: add `if: startsWith(github.ref, 'refs/tags/v') || inputs.crates-io`
    - `build-wheels`: add `if: startsWith(github.ref, 'refs/tags/v') || inputs.pypi`
    - `build-sdist`: add `if: startsWith(github.ref, 'refs/tags/v') || inputs.pypi`
    - `publish-pypi`: keep existing `needs: [build-wheels, build-sdist]`, add
        `if: startsWith(github.ref, 'refs/tags/v') || inputs.pypi`
    - `build-napi`: add `if: startsWith(github.ref, 'refs/tags/v') || inputs.npm`
    - `build-wasm`: add `if: startsWith(github.ref, 'refs/tags/v') || inputs.npm`
    - `publish-npm-lib`: change existing `if: startsWith(github.ref, 'refs/tags/v')` to
        `if: startsWith(github.ref, 'refs/tags/v') || inputs.npm`
    - `publish-npm-wasm`: change existing `if: startsWith(github.ref, 'refs/tags/v')` to
        `if: startsWith(github.ref, 'refs/tags/v') || inputs.npm`

3. **Important edge case**: `publish-pypi` already has `needs: [build-wheels, build-sdist]`. When
    adding `if:`, GitHub Actions will automatically skip a job whose dependencies were skipped. But
    be explicit — add the `if:` anyway so the intent is clear and the job doesn't attempt to run if
    only the `if:` on a dependency caused the skip. Same logic applies to `publish-npm-lib` (needs
    `build-napi`) and `publish-npm-wasm` (needs `build-wasm`).

4. **Do NOT change** the `permissions`, `concurrency`, trigger events (`push.tags`), or any build
    step logic. Only add `inputs:` and `if:` conditions.

## Verification

- `grep -q 'crates-io:' .github/workflows/release.yml` exits 0 — input defined
- `grep -q 'pypi:' .github/workflows/release.yml` exits 0 — input defined
- `grep -c 'inputs\.' .github/workflows/release.yml` returns 8 or more — all jobs have conditions
- `grep -q "inputs.crates-io" .github/workflows/release.yml` exits 0 — crates.io condition present
- `grep -q "inputs.pypi" .github/workflows/release.yml` exits 0 — PyPI condition present
- `grep -q "inputs.npm" .github/workflows/release.yml` exits 0 — npm condition present
- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0 — valid
    YAML

## Done When

All verification criteria pass, confirming that `release.yml` has three boolean `workflow_dispatch`
inputs and every job chain has the correct `if:` condition matching the spec.
