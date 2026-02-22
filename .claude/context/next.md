# Next Work Package

## Step: Add GitHub Actions CI workflow

## Goal

Create a CI workflow that runs all existing quality gates on push/PR, protecting the Rust core and
Python bindings from regressions. This is the first CI/CD deliverable from the target and the
highest-impact next step per the review agent handoff.

## Scope

- **Create**: `.github/workflows/ci.yml`
- **Modify**: (none)
- **Reference**:
    - `notes/06-build-cicd-publishing.md` — CI matrix patterns, maturin action usage
    - `mise.toml` — the `test` and `lint` task definitions (commands to mirror in CI)
    - `Cargo.toml` — workspace structure, Rust version (`1.85`), workspace members
    - `pyproject.toml` — Python version requirement (`>=3.10`), dev dependencies, pytest config

## Implementation Notes

Create `.github/workflows/ci.yml` with the following structure:

### Trigger

```yaml
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
```

### Jobs

**1. `rust` job** (ubuntu-latest):

- Checkout with `actions/checkout@v4`
- Install Rust stable toolchain (`dtolnay/rust-toolchain@stable`)
- Cache cargo registry/target (`Swatinem/rust-cache@v2`)
- Run `cargo fmt --all --check`
- Run `cargo clippy --workspace --all-targets -- -D warnings`
- Run `cargo test --workspace`

**2. `python` job** (ubuntu-latest, needs: rust is NOT required — run in parallel):

- Checkout with `actions/checkout@v4`
- Install Rust stable toolchain (`dtolnay/rust-toolchain@stable`)
- Install Python 3.10 (`actions/setup-python@v5` with `python-version: "3.10"`)
- Install uv (`astral-sh/setup-uv@v4`)
- Install dev dependencies: `uv sync --group dev`
- Build Python bindings: `uv run maturin develop --manifest-path crates/iscc-py/Cargo.toml`
- Run ruff: `uv run ruff check && uv run ruff format --check`
- Run pytest: `uv run pytest`

### Key details

- Use `concurrency` with `cancel-in-progress: true` to avoid redundant runs on rapid pushes
- Both jobs run on `ubuntu-latest` only for now (cross-platform matrix is a future step)
- Pin Python to `3.10` (the minimum from `abi3-py310`)
- The `python` job needs Rust toolchain because `maturin develop` compiles the PyO3 crate
- Do NOT use `mise` in CI — call `cargo`, `uv`, and tools directly for transparency and reliability
- Use `Swatinem/rust-cache@v2` for cargo caching (also benefits the python job)

## Verification

- `.github/workflows/ci.yml` is valid YAML (no syntax errors)
- Workflow defines two jobs (`rust` and `python`) that mirror the local quality gates
- `cargo fmt --all --check` runs in CI
- `cargo clippy --workspace --all-targets -- -D warnings` runs in CI
- `cargo test --workspace` runs in CI
- `uv run ruff check` and `uv run ruff format --check` run in CI
- `uv run maturin develop` + `uv run pytest` run in CI
- All pre-commit hooks still pass locally (`uv run prek run --all-files`)

## Done When

The CI workflow file exists, is valid YAML, and mirrors all local quality gates (Rust
fmt/clippy/test and Python ruff/pytest) so that pushing to GitHub will automatically validate the
codebase.
