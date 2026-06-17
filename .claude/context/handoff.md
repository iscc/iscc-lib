> **HUMAN REVIEW REQUESTED (still active):** TWO CID loops are running concurrently on the same
> `develop` checkout and racing each other. `ps aux` shows two `mise run cid:run 100 1800` trees —
> pts/5 (started 07:16, pid `41239`) currently running iter-97 `review` (pid `79104`), and pts/6
> (started 09:09, pid `78441`) running THIS iter-98 `advance` (pid `87873`). During this iteration
> the pts/5 loop committed `57ff556 cid(review): ...` on top of my `3d8994b cid(define-next)`, and
> its prek pre-commit hooks transiently stashed my unstaged `Cargo.toml`/`Cargo.lock` (I caught a
> momentary revert-to-0.23 mid-run; the changes were restored intact after the hook's stash pop).
> **A human must kill the duplicate loop** (keep one; e.g. terminate pgid `78441`). I did NOT push.
> Two loops committing to the same branch will collide on push by non-fast-forward.

## 2026-06-17 — PyO3 incremental bump 0.23 → 0.24 (first migration increment)

**Done:** Bumped the workspace `pyo3` pin from `"0.23"` to `"0.24"` (keeping
`features = ["abi3-py310"]`), refreshed `Cargo.lock` to `pyo3 0.24.2`, and verified the Python
binding builds, lints, and passes the full test suite unchanged. The 0.23→0.24 migration required
**zero source changes** — every idiom in `crates/iscc-py/src/lib.rs` (`dict.into()`,
`PyBytes::new(py, _).into()`, `.into_pyobject(py)?.into()`, and the raw `pyo3::ffi::*` CPython-C-API
calls incl. `Bound::from_owned_ptr` / `downcast_into_unchecked`) compiles clean on 0.24.

**Files changed:**

- `Cargo.toml`: `[workspace.dependencies]` `pyo3` version `"0.23"` → `"0.24"` (abi3-py310 feature
    preserved). Only hand-edited file.
- `Cargo.lock` (generated):
    `pyo3`/`pyo3-build-config`/`pyo3-ffi`/`pyo3-macros`/`pyo3-macros-backend` `0.23.5` → `0.24.2`,
    transitive `target-lexicon` `0.12.16` → `0.13.5` (via `cargo update -p   pyo3`).
- `crates/iscc-py/src/lib.rs`: **no change needed** — clean compile + clippy on 0.24.

**Verification:** All next.md criteria green.

- `grep` confirms `pyo3 = { version = "0.24", features = ["abi3-py310"] }` and `abi3-py310` present;
    `crates/iscc-py/pyproject.toml` still `requires-python = ">=3.10"`.
- `cargo tree -p iscc-py -i pyo3` → `pyo3 v0.24.2`.
- `cargo build -p iscc-py` → clean. `cargo clippy -p iscc-py -- -D warnings` → clean (no
    deprecations). `cargo fmt --check` → clean.
- `uv run maturin develop` → built `iscc_lib-0.4.0-cp310-abi3-linux_x86_64.whl` (single abi3 wheel,
    `abi3-py310` intact).
- `uv run pytest -q` → **286 passed** (conformance, smoke, streaming, gil, algo, text utils, new
    symbols, benchmarks) — no test-count regression. Benchmarks still show iscc-lib ahead of
    iscc-core; no perf concern.
- `mise run check` (`prek run --all-files`) → all 15 pre-commit-stage hooks **passed** (no race
    artifact this run). Note: `prek run --all-files` runs the pre-commit stage only; the pre-push
    gates (clippy `-D warnings`, pytest) were verified separately above.

**Next:** Continue the incremental PyO3 migration one minor per step — next increment `0.24 → 0.25`,
then 0.26 → 0.27 → 0.28 → 0.29. The RustSec advisories only clear at **0.29**; each hop stays a
self-contained, locally-verifiable step (bump pin → `cargo update -p pyo3` → build/clippy/fmt →
maturin → pytest). Watch for real source migration work at later minors (0.25+ may tighten
`IntoPyObject`/lifetime rules that 0.24 did not). Other open v1.0.0 gates: the `iai-callgrind`
perf-regression gate and CI verification of the (locally-committed, unpushed) CRAP Phase 3 gate.

**Notes:**

- **Concurrency is the only real hazard.** The code change is minimal, behavior-preserving, and
    fully verified. Do NOT interpret a future push failure as a code-quality problem until the
    duplicate loop is gone — see the banner.
- I staged ONLY my own files: `Cargo.toml`, `Cargo.lock`, `.claude/context/handoff.md`,
    `.claude/agent-memory/advance/MEMORY.md`. I did NOT stage `iterations.jsonl` (runner-owned) nor
    any context files the concurrent loop is writing (learnings/issues/state/review-memory).
- `maturin` is not on PATH in this devcontainer — invoke via `uv run maturin` (uv warns that
    `VIRTUAL_ENV` differs from the project venv `/home/dev/.venvs/iscc-lib`; harmless, it uses the
    project venv).
- No `unsafe` introduced; the pre-existing `unsafe` raw-FFI blocks in `lib.rs` were untouched and
    still compile on 0.24, confirming `Bound::from_owned_ptr` and friends remain in the 0.24 API.
