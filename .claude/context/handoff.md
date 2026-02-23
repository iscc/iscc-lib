## 2026-02-23 — Review of: Add OIDC release workflow for crates.io and PyPI

**Verdict:** PASS

**Summary:** Release workflow created at `.github/workflows/release.yml` with 4 jobs implementing
OIDC trusted publishing for both crates.io and PyPI. All 10 verification criteria from next.md are
satisfied. All 163 workspace tests pass (143 core + 20 FFI), clippy clean, fmt clean, no quality
gate circumvention.

**Issues found:**

- (none)

**Next:** The release workflow is complete. Remaining gaps from state.md: (1) GitHub Pages not
enabled — docs deployment workflow exists but fails with 404 (admin-side setting, not a code fix),
(2) docs content gaps — no Rust API page, no benchmark results page, (3) npm publishing workflow
(requires `NODE_AUTH_TOKEN` secret + `@iscc` org setup). The most impactful code-side work would be
adding the remaining documentation content (Rust API page and benchmark results), since both the
Pages enablement and npm org setup are admin tasks that can't be solved in code. Alternatively,
update state.md to reflect that the OIDC release workflow is now done (it's still listed as missing
in "What's Missing").

**Notes:** The workflow follows the minijinja-style approach (notes/06 Approach 1) with separate
jobs. `publish-crates-io` and `build-wheels` run in parallel (no dependency), which is correct since
crates.io and PyPI are independent registries. PyPI trusted publishing is configured without a
GitHub environment — this works at the repository level but Titusz may want to add a `pypi`
environment in repo settings for additional protection (approval gates). The `crates.io` and `PyPI`
trusted publishing must be configured on the respective registries' admin pages before the first
release.
