## 2026-06-17 — Fix the CI `Coverage + CRAP` cargo-crap install flake (make CI green)

**Done:** Added `--force` to the `cargo binstall` invocation in the `Install cargo-crap` step of the
`coverage` (`Coverage + CRAP`) CI job so the binary is installed on every run regardless of the
metadata `Swatinem/rust-cache@v2` restores. This stops `cargo binstall` from short-circuiting with
"already installed" while the actual `~/.cargo/bin/cargo-crap` binary is absent, which had been
killing every subsequent `cargo crap` step with `error: no such command: crap`.

**Files changed:**

- `.github/workflows/ci.yml`: line 314 changed from `cargo binstall -y cargo-crap@0.2.2` to
    `cargo binstall -y --force cargo-crap@0.2.2`. Single-line change; no other `coverage`-job step
    altered.

**Verification:**

- `grep -n 'cargo binstall -y --force cargo-crap@0.2.2' .github/workflows/ci.yml` → matches exactly
    one line (314).
- `uv run python -c "import yaml; yaml.safe_load(...)"` → `YAML OK` (system python3 lacks pyyaml;
    used the uv-managed env instead).
- `git diff .github/workflows/ci.yml` → isolated 1-line change (`-`/`+` on the install step only).
- `mise run check` → all 15 pre-commit hooks pass, including `check yaml` and `YAML formatting`
    (yamlfix did not need to reformat the file).
- CI-side confirmation (next push reaches the CRAP report/gate steps and the run goes GREEN) cannot
    be run locally — review agent confirms on the next push.

**Next:** With CI green again, resume the deferred PyO3 incremental migration **0.25 → 0.26** (prior
handoff's "Next", blocked by the red CI). Same recipe: bump pin in root `Cargo.toml` →
`cargo update -p pyo3` → build/clippy(`-D warnings`)/fmt → `uv run maturin develop` →
`uv run pytest` (286 tests). RustSec advisories only clear at 0.29, so keep going one minor per
step. Also still open: CRAP `--fail-above 30` hardening ([review] issue, needs HUMAN REVIEW) and the
`iai-callgrind` perf-regression CI gate.

**Notes:**

- The only working-tree change I introduced is `.github/workflows/ci.yml`. `iterations.jsonl` shows
    as modified in `git status` but that is the CID runner's log (a read-only context file managed
    by the loop) — I did NOT stage or touch it.
- No Rust/Python source changed, so the pre-push gates (clippy/cargo test/pytest) are not implicated
    by this change; `mise run check` runs the pre-commit stage, which is the relevant gate for a
    workflow-YAML edit and is clean.
- `--force` was chosen over disabling `cache-bin` on the rust-cache step (per next.md) because it is
    explicit, self-documenting, and independent of rust-cache internals; cargo-crap is a small
    binary so the per-run re-download cost is negligible.
- Did NOT edit `ci-cd.md` prose — it documents the install generically ("installed via
    `cargo binstall`"), which remains accurate after the fix (per next.md "Not In Scope").
