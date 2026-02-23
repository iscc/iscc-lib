## 2026-02-23 — Add OIDC release workflow for crates.io and PyPI

**Done:** Created `.github/workflows/release.yml` with tag-triggered OIDC trusted publishing for
both crates.io and PyPI. The workflow includes 4 jobs: `publish-crates-io` (Rust core with
pre-publish tests), `build-wheels` (cross-platform matrix: Linux x86_64/aarch64, macOS universal2,
Windows x64), `build-sdist`, and `publish-pypi` (downloads all artifacts and publishes via trusted
publishing).

**Files changed:**

- `.github/workflows/release.yml`: New release workflow with OIDC auth for crates.io and PyPI

**Verification:** YAML validates successfully. All 10 verification criteria from next.md are met:
tag trigger + workflow_dispatch, OIDC permissions on both publish jobs, crates-io-auth-action,
cross-platform wheel matrix, sdist job, pypi-publish action, correct dependency chain, no hardcoded
tokens, pre-publish test step. Existing workspace tests (143 Rust tests) continue passing. Clippy
and fmt checks clean.

**Next:** The release workflow is ready but requires repo-side configuration before first use: (1)
crates.io trusted publishing must be configured for the `iscc-lib` crate, (2) PyPI trusted
publishing must be configured for the `iscc-lib` project. Both are admin-side settings, not code
changes. After that, the remaining gaps from state.md are: docs deployment (GitHub Pages not
enabled), docs content (Rust API page, benchmark results), and npm publishing workflow (requires
`NODE_AUTH_TOKEN` secret + `@iscc` org setup).

**Notes:** No surprises. The workflow follows the minijinja-style approach (notes/06 Approach 1)
with separate jobs. Used `merge-multiple: true` on `download-artifact` to flatten all wheel
artifacts into a single `dist/` directory for pypi-publish. Artifact names use `wheels-$os-$target`
pattern to avoid name collisions in the matrix. No npm publishing is included per the work package
instructions.
