## 2026-06-17 — Migrate PyO3 0.25 → 0.26 (issue #1, incremental toward 0.29)

**Done:** Bumped the `pyo3` workspace dependency pin from `0.25` → `0.26` (preserving
`features = ["abi3-py310"]`), regenerated `Cargo.lock`, and cleared the two new 0.26 deprecations in
`crates/iscc-py/src/lib.rs` so the build is warning-free under `-D warnings`. Unlike the 0.23→0.24
and 0.24→0.25 hops (which needed zero source changes), 0.26 deprecated two APIs that the bindings
use, so minimal idiomatic fixes were required.

**Files changed:**

- `Cargo.toml`: pyo3 pin `0.25` → `0.26` (line 35), `abi3-py310` intact.
- `Cargo.lock`: regenerated via `cargo update -p pyo3` (pyo3 + pyo3-build-config + pyo3-ffi +
    pyo3-macros + pyo3-macros-backend all `0.25.1` → `0.26.0`). Generated file, does not count
    toward the 3-file limit.
- `crates/iscc-py/src/lib.rs`: two deprecation fixes —
    1. `Python::allow_threads` → `Python::detach` (7 call sites; same semantics, just renamed in
        0.26). Affected: `gen_image_code_v0`, `gen_data_code_v0`, `gen_instance_code_v0`,
        `gen_sum_code_v0`, and the three streaming `update()` methods.
    2. `PyResult<PyObject>` → `PyResult<Py<PyAny>>` (17 return-type sites). The `pyo3::PyObject` type
        alias is deprecated in 0.26 in favor of `Py<PyAny>`; `Py` and `PyAny` are already in scope
        via `pyo3::prelude::*`. The `Ok(dict.into())` / `.into_pyobject(py)?.into()` bodies still
        compile unchanged — `Bound<PyDict>::into()` infers `Py<PyAny>` exactly as it did for the
        alias.
    - rustfmt collapsed the now-single-expression `gen_sum_code_v0` `.detach(...)` closure to one line
        (no semantic change).

**Verification:** All next.md criteria green.

- `grep 'pyo3 = { version = "0.26"' Cargo.toml` → 1 match (line 35); `abi3-py310` still present.
- `grep -A1 'name = "pyo3"' Cargo.lock` → `version = "0.26.0"`.
- `cargo build -p iscc-py` → exit 0, no warnings.
- `cargo clippy -p iscc-py -- -D warnings` → exit 0, clean.
- `cargo fmt --all --check` → exit 0.
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` → built `cp310-abi3` wheel, installed.
- `uv run pytest` → **286 passed**, 1 warning (pre-existing, unrelated — `iscc_core`'s Pydantic V1 /
    Python 3.14 `UserWarning`, surfaced only by the benchmark comparison against the reference
    impl).

No public API change to `iscc-lib` (core has no PyO3 dep — untouched). No hot-path perf change (only
the GIL-release call was renamed, not its behavior). No API-BREAK.

**Next:** Continue the incremental PyO3 migration **0.26 → 0.27** (issue #1). The pattern now has a
proven recipe even when source changes are needed: bump pin → `cargo update -p pyo3` →
build/clippy(`-D warnings`)/fmt → `uv run maturin develop` → `uv run pytest` (286 tests). Watch for
further 0.27 deprecations — the raw `pyo3::ffi::*` C-API call sites (`PySequence_List`,
`PyList_GetItem`, `PyLong_AsLong`, `Bound::from_owned_ptr`, `downcast_into_unchecked`) and the
`#[pyo3(signature = ...)]` macros have survived every hop so far, but the deprecation churn at 0.26
suggests later hops may also touch source. RustSec advisories still only clear at 0.29 (3 hops away;
expected, not a failure). Also still open: CRAP `--fail-above 30` hardening ([review] issue, HUMAN
REVIEW REQUESTED) and the `iai-callgrind` perf-regression CI gate.

**Notes:**

- **Out-of-scope working-tree noise (not mine):** running `mise run check` (which runs prek
    `--all-files`) reformatted `.claude/context/next.md` and
    `.claude/agent-memory/define-next/MEMORY.md` via the mdformat hook — both were committed
    non-conforming by the prior `cid(define-next)` commit (dcf57f1). Per scope I do NOT own those
    files, so I reverted the mdformat changes (`git checkout   --`) to keep my commit clean.
    **Heads-up for review/push:** the pre-push mdformat hook runs on all files in the push range, so
    the non-conforming `next.md`/`define-next/MEMORY.md` may block the push (see learnings "Pre-push
    mdformat blocks on non-conforming context files"). If the push is rejected, reformat those two
    context files with `uv run mdformat --wrap 100 --number` and amend the define-next commit
    (mechanical, no semantic change) — this is the recurring define-next formatting gap, not a fault
    of this advance step.
- `mise run check` otherwise passes: all code-relevant hooks (Rust formatting, TOML formatting,
    Ruff, YAML, etc.) are green. The only "Failed" hook was mdformat on the two non-owned context
    files above.
- `Python::detach` is the 0.26 rename of `allow_threads` (identical GIL-release semantics on
    standard abi3 builds); docstrings say "Releases the GIL" which remains accurate, so they were
    left as-is.
