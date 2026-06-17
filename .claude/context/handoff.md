## 2026-06-17 — Review of: Migrate PyO3 0.26 → 0.27 (issue #1, incremental toward 0.29)

**Verdict:** PASS

**Summary:** Clean, tightly-scoped minor bump of the workspace `pyo3` pin `0.26 → 0.27` (resolves to
0.27.2). The diff is exactly Cargo.toml (line 35 pin), regenerated Cargo.lock (pyo3 + 4 sibling
crates 0.26.0 → 0.27.2), and two mechanical deprecation renames in `crates/iscc-py/src/lib.rs`
(`to_pylist`: `downcast` → `cast`, `downcast_into_unchecked` → `cast_into_unchecked`). No public API
change, no hot-path change, `abi3-py310` preserved. All verification green; Codex found no issues.

**Verification:**

- [x] `grep -n 'pyo3' Cargo.toml` → single match, line 35, `version = "0.27"` with `abi3-py310`
- [x] `grep -A1 'name = "pyo3"' Cargo.lock` → `version = "0.27.2"`
- [x] `cargo build -p iscc-py` → exit 0
- [x] `cargo clippy -p iscc-py -- -D warnings` → exit 0, clean
- [x] `cargo fmt --all --check` → exit 0
- [x] `uv run maturin develop -m crates/iscc-py/Cargo.toml` → exit 0, `cp310-abi3` wheel installed
- [x] `uv run pytest` → 286 passed, 1 pre-existing unrelated iscc_core Pydantic-V1/Py3.14 warning
- [x] `cargo tree -p iscc-py -i pyo3` → single `pyo3 v0.27.2`
- [x] No deprecated APIs remain in lib.rs: `allow_threads`=0, `downcast`=0, `PyResult<PyObject>`=0;
    `detach`=7, `Py<PyAny>`=17 intact
- [x] `mise run check` → all 15 pre-commit hooks Passed (incl. mdformat — no context-file churn)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` → exit 0 (pre-push defense)

**Issues found:**

- (none) — diff matches next.md scope exactly; no gate circumvention (the `--fail-above`/
    `continue-on-error` grep hits are all markdown context text discussing open issue #2, not code)

**Codex review:** No issues. "The PyO3 version bump, lockfile update, and deprecation renames are
consistent and preserve the existing behavior. Relevant Rust and Python verification passes, and no
introduced correctness issue was found."

**Next:** Continue the incremental migration **PyO3 0.27 → 0.28** (issue #1). The proven recipe
holds: bump the `Cargo.toml` line 35 pin → `cargo update -p pyo3` → build / clippy (`-D warnings`) /
fmt → `uv run maturin develop` → `uv run pytest` (286). 0.27 was the second consecutive hop needing
a source edit (after 0.26), both small mechanical renames — expect later hops may also touch source;
treat `-D warnings` as the gate. The raw `pyo3::ffi::*` C-API sites (`PySequence_List`,
`PyList_GetItem`, `PyLong_AsLong`, `Bound::from_owned_ptr`) and `#[pyo3(signature = ...)]` macros
have survived every hop. RustSec advisories clear only at 0.29 (2 hops away) — not a verification
failure for the 0.28 step.

**Notes:**

- 4 unpushed commits (advance, define-next, update-state, log) form a clean fast-forward over
    `origin/develop` (`git rev-list --left-right --count` = `0 4`). Pushed as one batch.
- Also open and unstarted: CRAP `--fail-above 30` hardening (issue #2, HUMAN REVIEW REQUESTED on the
    spec edit — `--fail-regression` lets new uncovered high-CRAP funcs through) and the
    `iai-callgrind` perf-regression CI gate (issue #3). Both are `normal` priority and actionable if
    define-next chooses to prioritize them over the next PyO3 hop.
- No backward-compat or perf concern: iscc-py is a binding crate (not the stability-committed core),
    pyo3 is an internal dep, and the edits are in the video-frame extraction helper, not a
    benchmarked `gen_*_v0` compute path (pytest video bench still ~3× faster than iscc-core).
