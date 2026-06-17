## 2026-06-17 — PyO3 incremental bump 0.24 → 0.25

**Done:** Bumped the workspace `pyo3` pin from `0.24` to `0.25` (preserving
`features = ["abi3-py310"]`) and refreshed `Cargo.lock` to `0.25.1`. Like the 0.23→0.24 hop, this
needed **zero source changes** — the next.md prediction that 0.25 would force
`IntoPyObject`/lifetime edits did not materialize for this codebase. Build, clippy (`-D warnings`),
fmt, the maturin abi3 wheel, and all 286 pytest tests pass green.

**Files changed:**

- `Cargo.toml`: `pyo3` pin `0.24` → `0.25` in `[workspace.dependencies]` (line 35),
    `features = ["abi3-py310"]` unchanged.
- `Cargo.lock`: refreshed via `cargo update -p pyo3` — `pyo3`, `pyo3-build-config`, `pyo3-ffi`,
    `pyo3-macros`, `pyo3-macros-backend` all `0.24.2` → `0.25.1` (5 packages).
- `crates/iscc-py/src/lib.rs`: **untouched** (no API changes required).

**Verification:**

- `grep 'pyo3 = { version = "0.25"' Cargo.toml` matches; `cargo tree -p iscc-py -i pyo3` →
    `pyo3 v0.25.1`
- `cargo build -p iscc-py` — clean
- `cargo clippy -p iscc-py -- -D warnings` — clean, no new deprecation warnings
- `cargo fmt --check` — clean (exit 0)
- `uv run maturin develop` — built `iscc_lib-0.4.0-cp310-abi3-linux_x86_64.whl`
- `uv run pytest` — **286 passed**, no count regression vs the 0.24 baseline. (The single warning is
    the pre-existing `iscc_core` Pydantic-V1/Python-3.14 `UserWarning` from the reference package
    used in comparison tests — unrelated to pyo3.)
- `abi3-py310` still present in `Cargo.toml`; `crates/iscc-py/pyproject.toml` still
    `requires-python = ">=3.10"`
- `mise run check` — all 15 pre-commit hooks pass
- pre-push stage (`uv run prek run --hook-stage pre-push --all-files`) — all pass (Rust linting,
    Rust tests, Python type checking, Ruff S/C901, Ruby linting, Python tests), exit 0
- Benchmarks (in pytest run) still well ahead of iscc-core — no perf concern.

**Next:** Continue the incremental PyO3 migration: **0.25 → 0.26**. Same self-contained recipe (bump
pin in root `Cargo.toml` → `cargo update -p pyo3` → build/clippy(`-D warnings`)/fmt →
`uv run maturin develop` → `uv run pytest`). RustSec advisories only clear at 0.29, so keep going
one minor per step. Note: the predicted `IntoPyObject`/lifetime breaking changes have not appeared
at 0.24 or 0.25 — they may still hit at 0.26+, so keep watching `-D warnings` for deprecations.
Other open `normal` work unchanged: CRAP `--fail-above 30` hardening and the `iai-callgrind` perf
gate.

**Notes:**

- No `unsafe` introduced; the pre-existing raw-FFI `unsafe` blocks in `lib.rs`
    (`Bound::from_owned_ptr`, `downcast_into_unchecked`, raw `pyo3::ffi::*`) all still compile on
    0.25, confirming those APIs remain stable through this minor.
- No API break (no codec/Tier 1 public symbol touched); binding semantics (result-dict keys/value
    types) unchanged.
- `maturin`/`prek` are not on PATH — invoke via `uv run`; uv warns `VIRTUAL_ENV` differs from the
    project venv `/home/dev/.venvs/iscc-lib` (harmless).
- Working tree also shows `.claude/context/iterations.jsonl` modified (CID loop bookkeeping) — NOT
    staged in this commit per protocol.
