## 2026-02-24 — Create Development documentation page

**Done:** Created `docs/development.md` — a single consolidated Development page for contributors
covering dev container setup, CID autonomous workflow, quality gates, project structure, and mise
task runner. Added "Development" nav entry in `zensical.toml` after Benchmarks.

**Files changed:**

- `docs/development.md`: New file with 5 major sections (Dev Container Setup, CID Autonomous
    Workflow, Quality Gates, Project Structure, Mise Task Runner) adapted from README content and
    polished for the docs site audience. Uses admonitions for tips, tables for tool/task reference,
    and tree diagram for project structure.
- `zensical.toml`: Added `{ "Development" = "development.md" }` to the nav array after Benchmarks.

**Verification:**

- `uv run zensical build` exits 0 — site builds with `/development/` page in output
- `grep -c "Development" zensical.toml` returns 1
- `grep -c "## " docs/development.md` returns 19 (well above the ≥5 threshold)
- `cargo clippy --workspace --all-targets -- -D warnings` clean (no Rust changes)
- `mise run check` passes — all 14 pre-commit hooks green

**Next:** The Development page is in place, which unblocks the README rewrite. The next step should
rewrite `README.md` to be a concise project introduction that links to the docs site for development
workflow details, as specified in the documentation spec.

**Notes:** Content was adapted from README sections, not copy-pasted. The project structure tree was
expanded to include all 5 crates, `docs/`, `benchmarks/`, `reference/`, and `.github/workflows/`
directories with brief descriptions. The CID context files table was added to the CID section since
it is contributor-relevant information not present in the README. The `cid:orchestrate` and
`cid:stats` tasks were included in the Mise Tasks table since they exist in `mise.toml` but were not
mentioned in the README.
