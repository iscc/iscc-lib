## 2026-06-17 — Review of: Migrate PyO3 0.25 → 0.26 (issue #1, incremental toward 0.29)

**Verdict:** PASS

**Summary:** Bumped the `pyo3` workspace pin `0.25 → 0.26`, regenerated `Cargo.lock`, and cleared
the two new 0.26 deprecations in `crates/iscc-py/src/lib.rs` (the only binding file). Scope was
tight (Cargo.toml + Cargo.lock + lib.rs), the fixes are idiomatic, and every verification criterion
is green. Unlike the prior two zero-source hops, 0.26 was the first to require source changes.

**Verification:**

- [x] `pyo3 = { version = "0.26", features = ["abi3-py310"] }` — line 35, single match
- [x] `Cargo.lock` shows `pyo3 0.26.0` (+ pyo3-ffi/build-config/macros/macros-backend all 0.26.0)
- [x] `abi3-py310` preserved on the pyo3 workspace dep
- [x] `cargo build -p iscc-py` — exit 0
- [x] `cargo clippy -p iscc-py -- -D warnings` — exit 0, clean
- [x] `cargo fmt --all --check` — exit 0
- [x] `uv run maturin develop -m crates/iscc-py/Cargo.toml` — exit 0, `cp310-abi3` wheel installed
- [x] `uv run pytest` — 286 passed, 1 pre-existing warning (iscc_core Pydantic-V1 / Py3.14,
    unrelated)
- [x] No deprecated APIs remain — 0 `allow_threads`/`PyResult<PyObject>`; 7 `detach`, 17 `Py<PyAny>`
- [x] `cargo tree -p iscc-py -i pyo3` resolves to a single pyo3 v0.26.0

**Issues found:**

- (none in the advance work — clean, minimal, correct)

**Codex review:** No findings. "The PyO3 bump is accompanied by the necessary deprecation fixes, and
the affected crate builds and passes its Python test suite. No introduced correctness,
compatibility, or maintainability issues."

**Next:** Continue the incremental migration **PyO3 0.26 → 0.27** (issue #1). Recipe is proven, and
0.26 confirmed source changes can be required: bump pin → `cargo update -p pyo3` →
build/clippy(`-D warnings`)/fmt → `uv run maturin develop` → `uv run pytest` (286). Watch 0.27
deprecations — the raw `pyo3::ffi::*` C-API sites (`PySequence_List`, `PyList_GetItem`,
`PyLong_AsLong`, `Bound::from_owned_ptr`, `downcast_into_unchecked`) and `#[pyo3(signature = ...)]`
macros have survived every hop incl. 0.26, but the 0.26 deprecation churn says later hops may also
touch source. RustSec advisories still clear only at 0.29 (3 hops away — expected, not a failure).
Also open and unstarted: CRAP `--fail-above 30` hardening ([review], HUMAN REVIEW REQUESTED on the
spec edit) and the `iai-callgrind` perf-regression CI gate.

**Notes:**

- **Recurring define-next mdformat gap (NOT an advance defect):** `mise run check` (prek
    `--all-files`) reformatted `.claude/context/next.md` and
    `.claude/agent-memory/define-next/MEMORY.md` via the mdformat hook — both were committed
    non-conforming by the define-next commit (dcf57f1). Changes are purely line-rewrapping to 100
    cols, zero semantic change. To prevent the pre-push mdformat hook from rejecting the batch push,
    I staged the reformatted versions of those two files into this review commit (mechanical fix per
    protocol step 9). define-next should run `uv run mdformat --wrap 100 --number` (or
    `mise run format`) before committing to close this gap at the source. All code-relevant hooks
    (Rust fmt, TOML, Ruff, YAML, Ruby) passed.
- `Python::detach` is the 0.26 rename of `allow_threads` (identical GIL-release semantics on abi3
    builds). Docstrings saying "Releases the GIL" remain accurate and were correctly left unchanged.
- Single CID loop confirmed running (one `mise run cid:run`, one `iteration 101` agent) — no
    concurrency race this iteration.
