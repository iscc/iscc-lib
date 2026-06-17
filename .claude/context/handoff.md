## 2026-06-17 — Review of: PyO3 incremental bump 0.24 → 0.25

**Verdict:** PASS

**Summary:** The advance agent bumped the workspace `pyo3` pin from `0.24` to `0.25` (preserving
`features = ["abi3-py310"]`) and refreshed `Cargo.lock` to `0.25.1` — a clean, minimal,
behavior-preserving dependency bump that needed zero source changes. All verification is green:
build, clippy (`-D warnings`), fmt, the maturin abi3 wheel, 286 pytest, and all 15 pre-commit hooks
pass. Scope matches next.md exactly (only `Cargo.toml` + generated `Cargo.lock`;
`crates/iscc-py/src/lib.rs` untouched).

**Verification:**

- [x] `grep 'pyo3 = { version = "0.25"' Cargo.toml` matches & `cargo tree -p iscc-py -i pyo3` →
    `pyo3 v0.25.1`
- [x] `cargo build -p iscc-py` — clean (~26s)
- [x] `cargo clippy -p iscc-py -- -D warnings` — clean, no new deprecations
- [x] `cargo fmt --check` — clean (exit 0)
- [x] `uv run maturin develop -m crates/iscc-py/Cargo.toml` — built
    `iscc_lib-0.4.0-cp310-abi3-linux_x86_64.whl`
- [x] `uv run pytest` — **286 passed**, 1 pre-existing iscc_core Pydantic-V1/Py3.14 warning
    (unrelated to pyo3); benchmarks still well ahead of iscc-core (no perf concern)
- [x] `abi3-py310` still present in `Cargo.toml` & `crates/iscc-py/pyproject.toml` still
    `requires-python = ">=3.10"`
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- (none) — no API break (no codec/Tier 1 symbol touched), no perf regression. Scanned all 4 unpushed
    commits via `git diff @{upstream}..HEAD`: no gate circumvention (every `continue-on-error` /
    `--fail-above` / `--fail-regression` match is documentation prose in state/next/MEMORY/handoff
    files, NOT a workflow change — no `.github/workflows/` file was touched). Concurrency hazard
    cleared: `ps aux` shows a single `mise run cid:run` tree (pid 78441), no duplicate loop.

**Codex review:** No blocking findings. Codex confirmed the commit "only advances PyO3 and its
lockfile entries plus internal handoff notes. The iscc-py crate builds, tests, and passes clippy
with the new dependency, and I found no introduced correctness issues."

**Next:** Continue the incremental PyO3 migration: **0.25 → 0.26**. Same self-contained recipe (bump
pin in root `Cargo.toml` → `cargo update -p pyo3` → build/clippy(`-D warnings`)/fmt →
`uv run maturin develop` → `uv run pytest`). The predicted `IntoPyObject`/lifetime breaking changes
have NOT appeared at 0.24 or 0.25 — treat that prediction skeptically for 0.26 too, but keep
watching `-D warnings` for deprecations. RustSec advisories only clear at 0.29, so keep going one
minor per step. Other open `normal` work unchanged: CRAP `--fail-above 30` hardening ([review]
issue, needs HUMAN REVIEW before amending the spec) and the `iai-callgrind` perf-regression CI gate.

**Notes:**

- This push sends a clean 4-commit fast-forward (`@{upstream}...HEAD` = `0 4`): update-state,
    define-next, advance, and the iter-98 log. The iter-98 9-commit backlog (CRAP Phase 3 + PyO3
    0.24) was already pushed and CI-verified green, so this push is just the 0.25 increment +
    context.
- No `unsafe` introduced; the pre-existing raw-FFI `unsafe` blocks in `lib.rs`
    (`Bound::from_owned_ptr`, `downcast_into_unchecked`, raw `pyo3::ffi::*`) all still compile on
    0.25, confirming those APIs remain stable through this minor.
- `maturin`/`prek` are not on PATH — invoke via `uv run`; uv warns `VIRTUAL_ENV` differs from the
    project venv `/home/dev/.venvs/iscc-lib` (harmless).
