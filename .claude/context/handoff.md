## 2026-02-22 — Add GitHub Actions CI workflow

**Done:** Created `.github/workflows/ci.yml` with two parallel jobs (`rust` and `python`) that
mirror all local quality gates. The `rust` job runs fmt, clippy, and tests. The `python` job builds
PyO3 bindings via maturin and runs ruff + pytest. Concurrency control cancels redundant runs.

**Files changed:**

- `.github/workflows/ci.yml`: New CI workflow with `rust` and `python` jobs on ubuntu-latest

**Verification:** All local quality gates pass:

- `cargo test --workspace`: 143 tests passed
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `cargo fmt --all --check`: clean
- `uv run prek run --all-files`: all 14 hooks passed (including `check yaml` and `YAML formatting`)

**Next:** The CI workflow is ready to validate on push to GitHub. Suggested next steps:

1. **Cross-platform CI matrix** — extend the workflow with macOS and Windows runners
2. **State.md update** — reflect CI addition and overall project completion status
3. **Benchmarks** — criterion for Rust, pytest-benchmark for Python (per notes/09)
4. **Release/publish workflows** — tag-triggered workflows for crates.io and PyPI publishing

**Notes:** The workflow deliberately does NOT use `mise` in CI — it calls `cargo`, `uv`, and tools
directly for transparency and reliability, as specified in the work package. Python is pinned to
3.10 (the minimum from abi3-py310). Both jobs use `Swatinem/rust-cache@v2` since the python job also
needs Rust compilation for `maturin develop`. The workflow uses `dtolnay/rust-toolchain@stable`
rather than pinning a specific Rust version — this is intentional for CI (stable tracks the latest
release, and the workspace `rust-version = "1.85"` in Cargo.toml provides MSRV enforcement
separately).
