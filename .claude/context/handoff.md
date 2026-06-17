## 2026-06-17 — Review of: Migrate PyO3 0.27 → 0.28 (issue #1, incremental toward 0.29)

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent bumped the workspace `pyo3` pin `0.27` → `0.28` (resolves 0.28.3),
regenerating `Cargo.lock` (pyo3 + 4 sibling crates bumped, transitive `indoc`/`memoffset`/`unindent`
dropped) with zero compiler-forced source edits. All build/clippy/fmt/maturin/pytest gates pass. The
Codex review surfaced — and I verified against the pyo3 macros-backend source — a **silent behavior
change** the compiler did not flag: PyO3 0.28 flipped the unspecified `#[pymodule]` `gil_used`
default from `true` to `false`. I applied a one-line conservative fix (`gil_used = true`) restoring
pre-0.28 semantics for the raw-FFI `extract_frame_sigs` path.

**Verification:** (all criteria from next.md)

- [x] `grep -n 'pyo3' Cargo.toml` → single match, line 35, `version = "0.28"` with `abi3-py310`
- [x] `grep -A1 'name = "pyo3"' Cargo.lock` → `version = "0.28.3"`
- [x] `cargo build -p iscc-py` → exit 0
- [x] `cargo clippy -p iscc-py -- -D warnings` → exit 0, clean (incl. after my fix)
- [x] `cargo fmt --all --check` → exit 0
- [x] `uv run maturin develop -m crates/iscc-py/Cargo.toml` → exit 0, `cp310-abi3` wheel installed
- [x] `uv run pytest` → 286 passed, 1 warning (known pre-existing iscc_core Pydantic-V1/Py3.14)
- [x] `cargo tree -p iscc-py -i pyo3` → single `pyo3 v0.28.3`, no duplicates
- [x] `allow_threads` count in `crates/iscc-py/src/lib.rs` = 0 (no deprecated APIs remain)
- [x] `mise run check` → all 15 pre-commit hooks Passed
- [x] `cargo clippy --workspace --all-targets -- -D warnings` → exit 0 (pre-push defense)

**Issues found:**

- **PyO3 0.28 silently flipped the `#[pymodule]` free-threaded default (fixed in this review).**
    Verified against cached macros-backend source: 0.27.2 `module.rs:407` used
    `options.gil_used.map_or(true, …)` (unspecified ⇒ `true`); 0.28.3 `module.rs:392` uses
    `options.gil_used.is_some_and(…)` (unspecified ⇒ `false`). With `false`, on free-threaded
    CPython *source* builds the `_lowlevel` module would import without re-enabling the GIL, and
    `extract_frame_sigs` reads shared Python lists via raw borrowed `PyList_GetItem` pointers that
    are not free-threading-safe (use-after-free under concurrent mutation). Impact is narrow — the
    published artifact is an `abi3-py310` wheel that free-threaded interpreters do not load, and the
    project ships no free-threaded target — but the bump should be behavior-neutral, so I restored
    the prior default with `#[pymodule(name = "_lowlevel", gil_used = true)]` plus an explanatory
    comment. Rebuilt + retested green.

**Codex review:** [P2] "Opt out of free-threaded mode until FFI is audited" — Cargo.toml:35. Codex
correctly identified the `gil_used` default flip and the unsafe `extract_frame_sigs` FFI path.
Verified and addressed in-review (see above). No other findings.

**Next:** Continue the incremental migration **PyO3 0.28 → 0.29** (issue #1) — the FINAL hop, where
the two RustSec advisories shipped in the published wheel clear (the stated endpoint of issue #1).
`0.29.0` is already available. Proven recipe holds: bump `Cargo.toml` line 35 →
`cargo update -p pyo3` → build / clippy (`-D warnings`) / fmt → `uv run maturin develop` →
`uv run pytest` (286). **Important for define-next/advance:** do NOT trust "compiles clean" as proof
of behavior-neutrality — the 0.28 hop compiled clean yet flipped a runtime default. Diff the pyo3
macros-backend default-handling (and read the 0.28→0.29 migration guide section) for any further
silent default changes, and keep the explicit `gil_used = true`. After 0.29 lands, verify the
RustSec advisories actually clear (`cargo audit` if available) before closing issue #1.

**Notes:**

- The `gil_used = true` fix is a no-op on all tested and shipped paths (GIL-enabled CPython, abi3
    wheels, pytest, conformance) — it only restores safe behavior on the untested from-source
    free-threaded build. If the project ever wants real free-threaded support, that requires a
    deliberate audit of the `extract_frame_sigs` raw-FFI borrows before flipping to
    `gil_used =   false`; the in-code comment documents this. Not filing a separate issue — the
    conservative default is restored and self-documented.
- Still open and unstarted: CRAP `--fail-above 30` hardening (issue #2 — HUMAN REVIEW REQUESTED on
    the spec) and the `iai-callgrind` perf-regression CI gate (issue #3 — valgrind unavailable in
    the devcontainer). Both untouched per Not-In-Scope.
- No backward-compat or perf concern for the core: iscc-py is a binding crate, pyo3 is an internal
    dep, and no compute path changed. pytest video/text benches still show iscc-lib ~14×/~30× faster
    than iscc-core.
- Single CID loop confirmed running (PID 424); no concurrent-loop race. Gate-circumvention scan over
    all 5 unpushed commits is clean (the `continue-on-error` grep hits are markdown context text
    discussing issue #2, not code/YAML changes).
