## 2026-06-17 — Review of: PyO3 incremental bump 0.23 → 0.24 (first migration increment)

**Verdict:** PASS

**Summary:** The advance agent bumped the workspace `pyo3` pin from `0.23` to `0.24` (preserving
`features = ["abi3-py310"]`) and refreshed `Cargo.lock` — a clean, minimal, behavior-preserving
dependency bump that needed zero source changes. All verification is green: build, clippy, fmt,
maturin wheel, 286 pytest, and all 15 pre-commit hooks pass. Scope matches next.md exactly (only
`Cargo.toml` + generated `Cargo.lock`; `crates/iscc-py/src/lib.rs` untouched).

**Concurrency note (resolved):** The iter-97 handoff carried a HUMAN REVIEW REQUESTED banner about
two CID loops racing `develop`. `ps aux` now shows a single `mise run cid:run` tree (pts/6, pid
78441\) — the duplicate is gone. The hazard is cleared, so this review pushes the accumulated
backlog.

**Verification:**

- [x] `grep 'pyo3 = { version = "0.24"' Cargo.toml` matches & `cargo tree -p iscc-py -i pyo3` →
    `pyo3 v0.24.2`
- [x] `cargo build -p iscc-py` — clean (finished in ~23s)
- [x] `cargo clippy -p iscc-py -- -D warnings` — clean, no deprecations
- [x] `cargo fmt --check` — clean
- [x] `maturin develop` + `uv run pytest` — abi3 wheel built
    (`iscc_lib-0.4.0-cp310-abi3-linux_x86_64.whl`), **286 passed**, no count regression; benchmarks
    still well ahead of iscc-core (no perf concern)
- [x] `abi3-py310` still present in `Cargo.toml` & `crates/iscc-py/pyproject.toml` still
    `requires-python = ">=3.10"`
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- (none) — no API break (no codec public symbol touched), no gate circumvention in any of the 9
    unpushed commits (verified `git diff @{upstream}..HEAD`; line-284 `continue-on-error` is the
    informational `semver` job, the CRAP `coverage` job remains enforcing), no perf regression.

**Codex review:** No blocking regressions. Codex confirmed the commit "only bumps PyO3 and its
lockfile dependencies plus internal handoff notes … builds cleanly and the Python bindings remain
compatible with the configured abi3 Python targets."

**Next:** Continue the incremental PyO3 migration: **0.24 → 0.25**. Same self-contained recipe (bump
pin in root `Cargo.toml` → `cargo update -p pyo3` → build/clippy(`-D warnings`)/fmt →
`uv run maturin develop` → `uv run pytest`). Watch for real source work starting at 0.25 (PyO3 0.25+
tightens `IntoPyObject`/lifetime rules that 0.24 did not require). RustSec advisories only clear at
0.29 — keep going one minor per step. Other open `normal` work: the CRAP `--fail-above 30` hardening
([review] issue) and the `iai-callgrind` perf-regression CI gate.

**Notes:**

- This push sends a **9-commit batch** (origin/develop is 9 behind; local is 9 ahead, 0 behind — a
    clean fast-forward). It includes the iter-97 CRAP Phase 3 work (reviewed PASS_WITH_NOTES in iter
    97 but never pushed because of the duplicate-loop hazard). CI on this push verifies the CRAP
    Phase 3 enforcing gate **and** the PyO3 0.24 bump together for the first time — watch the
    `Coverage + CRAP` and `Python` jobs on the resulting CI run.
- `maturin`/`prek` are not on PATH directly — invoke via `uv run maturin` / `uv run prek` (uv warns
    `VIRTUAL_ENV` differs from the project venv `/home/dev/.venvs/iscc-lib`; harmless).
- No `unsafe` introduced; the pre-existing raw-FFI `unsafe` blocks in `lib.rs` still compile on
    0.24, confirming `Bound::from_owned_ptr` / `downcast_into_unchecked` remain in the 0.24 API.
