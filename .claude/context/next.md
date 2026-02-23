# Next Work Package

## Step: Add GitHub Pages deployment workflow

## Goal

Create a CI workflow that builds documentation with zensical and deploys to GitHub Pages on push to
main, making the docs site accessible at lib.iscc.codes.

## Scope

- **Create**: `.github/workflows/docs.yml`
- **Modify**: (none)
- **Reference**: `notes/05-developer-tooling.md` (lines 293-317 have the exact workflow template),
    `zensical.toml`, `.github/workflows/ci.yml` (for action version consistency)

## Implementation Notes

The workflow template from `notes/05-developer-tooling.md` lines 293-317 is the starting point. Key
details:

1. **Trigger**: `push` to `main` branch only (not PRs — docs deploy only from main)

2. **Permissions**: The job needs `contents: read`, `pages: write`, and `id-token: write` for the
    GitHub Pages deployment action

3. **Environment**: Add `environment: github-pages` with `url: ${{ steps.deploy.outputs.page_url }}`
    to the deploy step for proper GitHub Pages integration. The deploy step needs an `id: deploy`

4. **Concurrency**: Add a concurrency group to prevent overlapping deployments:

    ```yaml
    concurrency:
      group: pages
      cancel-in-progress: false
    ```

5. **Steps** (in order):

    - `actions/checkout@v4`
    - `actions/setup-python@v5` with `python-version: '3.12'`
    - `astral-sh/setup-uv@v4` (match CI workflow's v4, not v5 from notes)
    - `uv sync --group dev` (installs zensical + mkdocstrings)
    - `uv run zensical build` (produces `site/` directory)
    - `actions/upload-pages-artifact@v3` with `path: site`
    - `actions/deploy-pages@v4` with `id: deploy`

6. **No maturin/Rust needed**: The mkdocstrings config uses `allow_inspection = false` with
    `paths = ["crates/iscc-py/python"]`, reading `.pyi` stubs directly. No compilation required.

7. **Action versions**: Use `setup-uv@v4` to match the existing `ci.yml` (not `v5` as in the notes
    template). All other actions match what `ci.yml` already uses.

## Verification

- `yamllint` or `python -c "import yaml; yaml.safe_load(open(...))"` validates the YAML syntax
- Workflow file exists at `.github/workflows/docs.yml`
- Workflow triggers only on `push` to `main` (not on PRs)
- Workflow has correct permissions (`pages: write`, `id-token: write`, `contents: read`)
- Workflow has concurrency group to prevent overlapping deployments
- Workflow installs Python + uv, syncs dev deps, builds with `uv run zensical build`, uploads
    `site/` artifact, and deploys via `actions/deploy-pages@v4`
- No Rust toolchain or maturin steps in the workflow (docs build from static `.pyi` stubs)
- All existing tests still pass (`cargo test -p iscc-lib`)

## Done When

The `docs.yml` workflow file exists with correct YAML, proper permissions, concurrency control, and
a complete build-upload-deploy pipeline — and all existing tests still pass.
