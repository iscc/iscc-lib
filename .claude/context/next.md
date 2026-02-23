# Next Work Package

## Step: Add OIDC release workflow for crates.io and PyPI

## Goal

Create a tag-triggered GitHub Actions release workflow that publishes the Rust core crate to
crates.io and cross-platform Python wheels to PyPI, both using OIDC trusted publishing (no API
keys). This is the highest-impact remaining work for release-readiness.

## Scope

- **Create**: `.github/workflows/release.yml`
- **Modify**: none
- **Reference**: `notes/06-build-cicd-publishing.md` (publishing patterns, auth methods, build
    matrices), `.github/workflows/ci.yml` (existing CI patterns), `crates/iscc-py/pyproject.toml`
    (maturin config), `crates/iscc-py/Cargo.toml` (cdylib config), `crates/iscc-lib/Cargo.toml`
    (publishable core crate), root `Cargo.toml` (workspace metadata)

## Implementation Notes

Create a single `.github/workflows/release.yml` triggered on version tag push (`v*.*.*`). Use the
minijinja-style approach (notes/06 Approach 1) with separate jobs for crates.io and PyPI. The
workflow must have these jobs:

### 1. `publish-crates-io` — Rust core crate

- Triggered only when tag matches `v*.*.*`
- Uses `dtolnay/rust-toolchain@stable` + `Swatinem/rust-cache@v2`
- Runs `cargo test --workspace` as a pre-publish sanity check
- Publishes `iscc-lib` (the core crate, which has `publish` not set to `false`)
- OIDC auth: `permissions: id-token: write` + `rust-lang/crates-io-auth-action@v1`
- Then `cargo publish -p iscc-lib`

### 2. `build-wheels` — Cross-platform Python wheels

- Matrix build using `PyO3/maturin-action@v1`:
    - Linux: `x86_64`, `aarch64` targets on `ubuntu-latest`
    - macOS: `universal2-apple-darwin` on `macos-14`
    - Windows: `x64` on `windows-latest`
- maturin args: `--release --out dist --manifest-path crates/iscc-py/Cargo.toml`
- Upload artifacts via `actions/upload-artifact@v4`
- Also build sdist (source distribution) as a separate job/step

### 3. `publish-pypi` — Upload wheels to PyPI

- Depends on `build-wheels`
- Runs on `ubuntu-latest`
- Downloads all wheel artifacts via `actions/download-artifact@v4`
- OIDC auth: `permissions: id-token: write` + `pypa/gh-action-pypi-publish@release/v1`
- No API key needed — trusted publishing is configured in PyPI project settings

### Key design decisions:

- Use `workflow_dispatch` as an additional trigger (allows manual re-runs if publishing partially
    fails)
- Top-level `permissions: contents: read` with job-level `id-token: write` only where needed
    (principle of least privilege)
- Use `concurrency` group to prevent simultaneous releases
- The `abi3-py310` flag is already configured in workspace dependencies
    (`pyo3 = { features =   ["abi3-py310"] }`) so maturin will automatically produce abi3 wheels
- Do NOT include npm publishing — npm requires `NODE_AUTH_TOKEN` secret (not OIDC) and the `@iscc`
    org doesn't exist yet. That's a separate step
- Keep the build matrix minimal (3 platforms × 1-2 targets each) — match the patterns in notes/06

### Workflow structure:

```yaml
name: Release
on:
  push:
    tags: ['v*.*.*']
  workflow_dispatch:
permissions:
  contents: read
concurrency:
  group: release
  cancel-in-progress: false  # Never cancel a release in progress
jobs:
  publish-crates-io: ...
  build-wheels: ...
  build-sdist: ...
  publish-pypi:
    needs: [build-wheels, build-sdist]
    ...
```

## Verification

- `.github/workflows/release.yml` exists and is valid YAML
- Workflow triggers on `push: tags: ['v*.*.*']` and `workflow_dispatch`
- `publish-crates-io` job has `permissions: id-token: write` and uses
    `rust-lang/crates-io-auth-action@v1`
- `build-wheels` job has matrix covering Linux (x86_64, aarch64), macOS (universal2), Windows (x64)
- `build-sdist` job or step exists
- `publish-pypi` job uses `pypa/gh-action-pypi-publish@release/v1` with
    `permissions: id-token: write`
- `publish-pypi` depends on `build-wheels` and `build-sdist`
- No hardcoded API keys or tokens — all auth is OIDC
- `cargo test --workspace` or `cargo test -p iscc-lib` runs as pre-publish check
- YAML parses without errors (validate with `python -c "import yaml; yaml.safe_load(open(...))"`)

## Done When

The release workflow exists at `.github/workflows/release.yml`, passes YAML validation, and
implements OIDC trusted publishing for both crates.io and PyPI with cross-platform wheel builds.
