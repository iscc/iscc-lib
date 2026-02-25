# Next Work Package

## Step: Add idempotency checks to release publish jobs

## Goal

Add pre-publish version-existence checks to all 4 publish jobs in `release.yml` so that
re-publishing an already-published version skips gracefully instead of failing the workflow. This is
the last `[critical]` blocker before the first `v0.0.1` release.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml`
- **Reference**: `.claude/context/specs/ci-cd.md` (Idempotency section), `.claude/context/issues.md`
    (`[critical] Add idempotency checks to release publish jobs`)

## Not In Scope

- Version sync tooling (`scripts/version_sync.py`, `mise run version:sync`) — that is the `[normal]`
    issue and comes after this step
- Changing the build jobs (`build-wheels`, `build-sdist`, `build-napi`, `build-wasm`) — only the
    publish steps need idempotency guards
- Adding new CI jobs or modifying `ci.yml`
- OIDC configuration changes or authentication modifications
- Java/Maven Central publishing (not yet in release.yml)

## Implementation Notes

Add a version-check step before each publish step in the 4 publish jobs. The pattern is: extract the
workspace version, query the registry, and set a GitHub Actions output flag that the publish step
uses to conditionally skip.

### Version extraction

Each publish job needs the workspace version. Extract it from root `Cargo.toml`:

```yaml
  - name: Get workspace version
    id: version
    run: |
      VERSION=$(grep -m1 '^version' Cargo.toml | sed 's/.*"\(.*\)"/\1/')
      echo "version=$VERSION" >> "$GITHUB_OUTPUT"
```

The `publish-crates-io` job already checks out the repo. For `publish-pypi` and `publish-npm-lib`,
the checkout step is also already present. For `publish-npm-wasm`, there is no checkout — read the
version from the downloaded `pkg/package.json` artifact instead.

### Per-registry checks

**crates.io** (`publish-crates-io` job): Insert before the "Publish iscc-lib" step:

```yaml
  - name: Check if already published
    id: check
    run: |
      VERSION="${{ steps.version.outputs.version }}"
      if cargo info iscc-lib 2>/dev/null | grep -q "version: $VERSION"; then
        echo "Version $VERSION already published to crates.io, skipping"
        echo "skip=true" >> "$GITHUB_OUTPUT"
      else
        echo "skip=false" >> "$GITHUB_OUTPUT"
      fi
```

Then add `if: steps.check.outputs.skip != 'true'` to the "Authenticate with crates.io" and "Publish
iscc-lib" steps.

**PyPI** (`publish-pypi` job): This job does not check out the repo — it only downloads wheel
artifacts. Add a checkout step for version extraction, then query the PyPI JSON API:

```yaml
  - uses: actions/checkout@v4
  - name: Get workspace version
    id: version
    run: |
      VERSION=$(grep -m1 '^version' Cargo.toml | sed 's/.*"\(.*\)"/\1/')
      echo "version=$VERSION" >> "$GITHUB_OUTPUT"
  - name: Check if already published
    id: check
    run: |
      VERSION="${{ steps.version.outputs.version }}"
      if curl -sf "https://pypi.org/pypi/iscc-lib/$VERSION/json" > /dev/null 2>&1; then
        echo "Version $VERSION already published to PyPI, skipping"
        echo "skip=true" >> "$GITHUB_OUTPUT"
      else
        echo "skip=false" >> "$GITHUB_OUTPUT"
      fi
```

Then add `if: steps.check.outputs.skip != 'true'` to the download-artifacts and publish steps.

**npm @iscc/lib** (`publish-npm-lib` job): Insert before the "Publish to npm" step:

```yaml
  - name: Get workspace version
    id: version
    run: |
      VERSION=$(grep -m1 '^version' Cargo.toml | sed 's/.*"\(.*\)"/\1/')
      echo "version=$VERSION" >> "$GITHUB_OUTPUT"
  - name: Check if already published
    id: check
    run: |
      VERSION="${{ steps.version.outputs.version }}"
      if npm view "@iscc/lib@$VERSION" version 2>/dev/null; then
        echo "Version $VERSION already published to npm, skipping"
        echo "skip=true" >> "$GITHUB_OUTPUT"
      else
        echo "skip=false" >> "$GITHUB_OUTPUT"
      fi
```

**npm @iscc/wasm** (`publish-npm-wasm` job): This job has no checkout. Read version from the
downloaded `pkg/package.json`:

```yaml
  - name: Get package version
    id: version
    run: |
      VERSION=$(node -p "require('./pkg/package.json').version")
      echo "version=$VERSION" >> "$GITHUB_OUTPUT"
  - name: Check if already published
    id: check
    run: |
      VERSION="${{ steps.version.outputs.version }}"
      if npm view "@iscc/wasm@$VERSION" version 2>/dev/null; then
        echo "Version $VERSION already published to npm, skipping"
        echo "skip=true" >> "$GITHUB_OUTPUT"
      else
        echo "skip=false" >> "$GITHUB_OUTPUT"
      fi
```

Note: the version step must come AFTER the download-artifact step so `pkg/package.json` exists.

### Key details

- Each check must output a clear log message when skipping
- The `if:` condition on publish steps must allow the job to succeed (green) when skipping —
    skipping a step via `if:` still shows the job as successful
- `curl -sf` (silent + fail) returns non-zero on HTTP 404 — correct for PyPI check
- `cargo info` requires network (queries crates.io) — fine in CI
- `npm view` returns non-zero when the version doesn't exist — correct for npm check
- Do NOT change permissions, concurrency, triggers, build logic, or job conditions — only add
    version-check steps and `if:` conditions on publish steps

## Verification

- `grep -c 'already published' .github/workflows/release.yml` returns 4 (one message per publish
    job)
- `grep -c 'steps.check.outputs.skip' .github/workflows/release.yml` returns at least 4 (skip
    conditions on publish steps)
- `grep -q 'pypi.org/pypi/iscc-lib' .github/workflows/release.yml` exits 0 (PyPI version check)
- `grep -q 'cargo info iscc-lib' .github/workflows/release.yml` exits 0 (crates.io version check)
- `grep -q 'npm view.*@iscc/lib' .github/workflows/release.yml` exits 0 (npm lib version check)
- `grep -q 'npm view.*@iscc/wasm' .github/workflows/release.yml` exits 0 (npm wasm version check)
- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0 (valid
    YAML)
- `mise run check` passes (all pre-commit hooks)

## Done When

All 8 verification criteria pass — every publish job has a version-existence check that logs a
message and skips the publish step when the version already exists on the target registry, and the
workflow file remains valid YAML passing all quality gates.
