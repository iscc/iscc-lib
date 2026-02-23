## 2026-02-23 — Add GitHub Pages deployment workflow

**Done:** Created `.github/workflows/docs.yml` that builds documentation with zensical and deploys
to GitHub Pages on push to main. The workflow uses Python 3.12 + uv to install dev dependencies and
build the docs site, then uploads and deploys via GitHub Pages actions.

**Files changed:**

- `.github/workflows/docs.yml`: New workflow file with docs build-upload-deploy pipeline

**Verification:** YAML syntax validated with `yaml.safe_load()`. All 143 Rust tests pass. Clippy and
fmt checks clean. Workflow matches all requirements: push-to-main only trigger, correct permissions
(contents: read, pages: write, id-token: write), concurrency group (pages, cancel-in-progress:
false), environment (github-pages with URL output), action versions matching ci.yml (setup-uv@v4),
no Rust toolchain steps.

**Next:** OIDC trusted publishing workflows (crates.io, PyPI, npm) would complete the CI/CD
pipeline. Alternatively, adding an architecture overview page (`docs/architecture.md`) would enrich
the documentation site before it goes live.

**Notes:** This was a straightforward single-file creation. The notes/05 template used `setup-uv@v5`
but next.md correctly specified `v4` to match the existing ci.yml — followed the next.md
instruction. The GitHub Pages environment and deployment settings (repository Settings > Pages >
Source: GitHub Actions) will need to be configured in the GitHub repository for the workflow to
succeed on first push.
