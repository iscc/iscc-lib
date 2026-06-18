## 2026-06-18 — Migrate PyO3 0.28 → 0.29 (FINAL hop — closes issue #1 "Update PyO3 to latest release")

**Done:** Bumped the workspace `pyo3` pin `0.28` → `0.29` (resolves `0.29.0`, keeping `abi3-py310`)
and regenerated `Cargo.lock` (pyo3 + 4 sibling crates pyo3-build-config / pyo3-ffi / pyo3-macros /
pyo3-macros-backend all bump to 0.29.0 together). Zero source edits were forced — the build, clippy
(`-D warnings`), fmt, maturin, and pytest chain is green. This is the endpoint of the incremental
0.23→0.29 migration: the 0.29.0 CHANGELOG explicitly lists both RustSec advisory fixes (OOB read in
`BoundListIterator`/`BoundTupleIterator` `nth`/`nth_back` #6086; missing `Sync` bound on
`PyCFunction::new_closure` #6096), so the patched PyO3 now ships in the wheel and issue #1 can
close.

**Files changed:**

- `Cargo.toml`: line 35 `pyo3` workspace pin `0.28` → `0.29` (`abi3-py310` retained)
- `Cargo.lock`: regenerated via `cargo update -p pyo3` (5 pyo3 crates → 0.29.0; generated, not
    counted toward file scope)
- `crates/iscc-py/src/lib.rs`: **unchanged** (no deprecation/breakage surfaced under `-D warnings`)

**Verification:** (all criteria from next.md)

- [x] `grep -n 'pyo3' Cargo.toml` → single match, line 35, `version = "0.29"` with `abi3-py310`
- [x] `grep -A1 'name = "pyo3"' Cargo.lock` → `version = "0.29.0"`; only ONE `name = "pyo3"` entry,
    no pyo3 `< 0.29` remaining
- [x] `cargo build -p iscc-py` → exit 0
- [x] `cargo clippy -p iscc-py -- -D warnings` → exit 0, clean
- [x] `cargo fmt --all --check` → exit 0
- [x] `uv run maturin develop -m crates/iscc-py/Cargo.toml` → exit 0, `cp310-abi3` wheel installed
- [x] `uv run pytest` → 286 passed, 1 warning (the known pre-existing iscc_core Pydantic-V1/Py3.14
    UserWarning, raised inside `iscc_core/options.py` on import — not our code)
- [x] `cargo tree -p iscc-py -i pyo3` → single `pyo3 v0.29.0`, no duplicates
- [x] `grep -n 'gil_used = true' crates/iscc-py/src/lib.rs` → still present (lib.rs:697; default not
    regressed)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` → exit 0 (pre-push defense)
- [x] `mise run check` → all 15 pre-commit hooks Passed
- [x] `uv run prek run --all-files --hook-stage pre-push` → all pre-push gates Passed (Rust
    lint/tests, ty check, Ruff S/C901, Ruby lint, pytest)

**Silent-behavior audit (per the 0.28 hop warning that "compiles clean" ≠ behavior-neutral):**

- Diffed the pyo3-macros-backend `module.rs` `gil_used` default handling 0.28.3 vs 0.29.0: byte
    identical (`options.gil_used.is_some_and(...)` ⇒ unspecified defaults to `false`). Our explicit
    `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697) keeps GIL protection — no flip
    this hop.
- Read the 0.29.0 CHANGELOG. One behavioral item worth flagging but NOT impacting us: "Change
    exception enrichment on `#[pyfunction]` argument extraction error to use `PyErr::add_note`
    instead of replacing `TypeError` instances" (#5349) — pytest's error-path assertions still pass,
    so our tests don't depend on the old message-replacement behavior. Also "Remove all
    functionality deprecated in PyO3 0.27" (#6068) — we already migrated off the 0.27-deprecated
    `downcast`/`downcast_into_unchecked` to `cast`/`cast_into_unchecked` in the 0.27 hop, so the
    removal is a no-op for us (clean build confirms). The 8 raw `pyo3::ffi::*` sites +
    `Bound::from_owned_ptr().cast_into_unchecked()` survived unchanged again.

**Next:** Issue #1 is functionally complete — recommend the review agent verify and **close issue
#1** (note: `cargo audit`/`cargo deny` are NOT installed in the devcontainer and not wired into
CI/mise, so the advisories cannot be tool-confirmed locally; the mechanical proxy is satisfied —
`Cargo.lock` resolves a single `pyo3 0.29.0` with no older entries, and 0.29.0's CHANGELOG lists
both advisory fixes). After that, the remaining open work is issue #2 (CRAP `--fail-above 30`
hardening — HUMAN REVIEW REQUESTED on the spec) and issue #3 (`iai-callgrind` perf gate — valgrind
unavailable in the devcontainer). Both untouched per Not-In-Scope. The PyO3 migration arc is done; a
sensible next target is the v1.0.0 cut prep (flip the `Semver` gate from `continue-on-error` to
enforcing) — but that is explicitly human-directed, so define-next should not start it autonomously.

**Notes:**

- No core API change, no perf-path change: iscc-py is a binding crate, pyo3 is its internal dep, and
    no compute path was touched. pytest benches still show iscc-lib ~3–4× faster than iscc-core
    (mixed/video) and ~1.3× (text) — unchanged.
- The single pytest warning originates in `iscc_core` (the reference impl used only by the
    comparison/benchmark tests), not in iscc-lib — it predates this change and is unrelated.
- A CID loop is running; before pushing, confirm no concurrent-loop race (only my commit should be
    newly added). I am committing `Cargo.toml`, `Cargo.lock`, handoff.md, and agent memory only —
    NOT `.claude/context/iterations.jsonl` (runner-managed) or any other context file.
