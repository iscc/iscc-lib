## 2026-02-23 — Make docs workflow green without Pages enabled

**Done:** Split `docs.yml` from a single `docs` job into two jobs: `build` (validates docs compile
correctly, permissions limited to `contents: read`) and `deploy` (deploys to GitHub Pages with
`continue-on-error: true` at job level so the workflow reports green even when Pages isn't enabled).

**Files changed:**

- `.github/workflows/docs.yml`: Split single `docs` job into `build` + `deploy` jobs. Build job
    retains all original build steps (checkout, setup-python, setup-uv, install deps, zensical
    build, upload artifact). Deploy job depends on build, has `continue-on-error: true`, and
    contains only the `actions/deploy-pages@v4` step with the `github-pages` environment.

**Verification:** YAML validated successfully. cargo test (163 tests), clippy, and fmt all clean.
All 8 verification criteria from next.md satisfied: two jobs, proper permissions separation,
job-level `continue-on-error`, preserved triggers/concurrency/build steps.

**Next:** The project appears code-complete. All 9 gen functions, all 5 binding crates, CI, release
workflow, docs, and benchmarks are done. The docs workflow will now pass even without Pages enabled.
Consider running update-state to confirm code-complete status and mark the project ready for initial
release. Titusz can enable GitHub Pages in repo settings at any time — the deploy job will
automatically start succeeding.

**Notes:** No surprises. This was a straightforward workflow restructuring. The `continue-on-error`
is at the job level (not step level) as specified, which means the deploy job itself will show as
failed in the GitHub UI when Pages isn't enabled, but the overall workflow status will be green.
