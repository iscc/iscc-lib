# Security, Supply Chain, and Version Management

For a cryptographic identifier library targeting ISO standardization, supply-chain hygiene is table
stakes, not optional.

## Rust Dependencies

- **`cargo deny check`** — Run in CI. Checks licenses (deny copyleft in transitive deps), RustSec
    advisories, and duplicate versions. Configure via `deny.toml` at workspace root.
- **`cargo audit`** — Alternative/complement to `cargo deny` for RustSec advisory checks.
- **Dependabot** — Enable for Cargo, pip, and npm manifests. Configure weekly updates.

## Python Dependencies

- **`bandit`** — Static security scanner for Python code. Already used in iscc-sum.
- **`pip-audit`** or `uv audit` — Check Python dependencies against known vulnerabilities. Run in CI
    (non-blocking initially, then enforce).

## Node.js Dependencies

- **`npm audit`** or `pnpm audit` — Run in CI for advisory checks.

## Build and Release Integrity

- **Signed git tags** for releases (`git tag -s`). Required for reproducible provenance.
- **OIDC trusted publishing** for crates.io and PyPI (already covered in CI/CD section) — eliminates
    long-lived API keys.
- **`SOURCE_DATE_EPOCH`** — Set in release builds for reproducible timestamps. maturin respects this
    automatically.
- **SLSA provenance** — GitHub Actions can generate SLSA level 2+ provenance attestations. Consider
    adding `actions/attest-build-provenance@v2` to release workflows.

## Recommended CI Configuration

```yaml
# deny.toml (workspace root)
[advisories]
vulnerability = "deny"
unmaintained = "warn"

[licenses]
unlicensed = "deny"
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Unicode-3.0"]

[bans]
multiple-versions = "warn"
```

```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: cargo
    directory: /
    schedule: {interval: weekly}
  - package-ecosystem: pip
    directory: /
    schedule: {interval: weekly}
  - package-ecosystem: npm
    directory: /crates/iscc-node
    schedule: {interval: weekly}
  - package-ecosystem: github-actions
    directory: /
    schedule: {interval: weekly}
```

## No Network in Tests

Tests must not make network calls. Vendor conformance vectors as files. Mock or stub any external
service. This prevents flaky CI and ensures tests work in air-gapped environments.

---

## Version Management

### minijinja — Script-Based Bumping

```bash
# scripts/bump-version.sh <version>
# Updates version in:
#   - All Cargo.toml files
#   - pyproject.toml
#   - package.json
#   - Changelog headings
```

Release flow:

1. Run `scripts/bump-version.sh 2.13.0`
2. Commit: `git commit -m "Release 2.13.0"`
3. Tag: `git tag 2.13.0`
4. Push: `git push && git push --tags`
5. CI builds and publishes automatically

### kreuzberg — Automated Cross-Language Sync

```python
# scripts/sync_versions.py
# Reads version from workspace Cargo.toml, updates:
#   - All member Cargo.toml files
#   - pyproject.toml
#   - package.json (node + wasm)
#   - Ruby version.rb
#   - Java pom.xml
#   - C# Kreuzberg.csproj
#   - Go module
```

Additionally, a validation script runs in CI before publishing:

```bash
# scripts/publish/validate-version-consistency.sh
# Fails the build if ANY manifest has a mismatched version
```

### Recommended Approach

Use a version sync script (Python or Rust) that:

1. Reads the canonical version from `Cargo.toml` workspace
2. Updates all language-specific manifests
3. Is called by the task runner (`mise run version:sync`)
4. Has a validation mode for CI (`mise run version:check`)

**Alternative — `release-plz`**: For Rust-centric release automation, `release-plz` can automate
version bumping, changelog generation (via `git-cliff`), and crates.io publishing through PR-based
workflows. It also integrates `cargo-semver-checks` for breaking change detection. However, it is
primarily Rust/crates.io focused — non-Rust manifests (pyproject.toml, package.json) still require a
supplementary sync mechanism, so a version sync script remains necessary for polyglot projects.
