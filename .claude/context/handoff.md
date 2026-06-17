## 2026-06-17 — Migrate PyO3 0.26 → 0.27 (issue #1, incremental toward 0.29)

**Done:** Bumped the `pyo3` workspace pin `0.26 → 0.27` (resolves to 0.27.2), regenerated
`Cargo.lock`, and cleared the two new 0.27 deprecations in `crates/iscc-py/src/lib.rs` (the only
binding file with a PyO3 dependency). Both deprecations were in the `to_pylist` helper:
`Bound::downcast` → `Bound::cast` and `Bound::downcast_into_unchecked` →
`Bound::cast_into_unchecked` (pure renames with identical signatures/semantics — no behavioral
change).

**Files changed:**

- `Cargo.toml`: line 35 pin `pyo3 = { version = "0.27", features = ["abi3-py310"] }` (`abi3-py310`
    preserved)
- `Cargo.lock`: regenerated via `cargo update -p pyo3` (pyo3 + pyo3-ffi/build-config/macros/
    macros-backend all 0.26.0 → 0.27.2)
- `crates/iscc-py/src/lib.rs`: `to_pylist` — `obj.downcast::<PyList>()` → `obj.cast::<PyList>()` and
    `.downcast_into_unchecked()` → `.cast_into_unchecked()`. Both verified against pyo3 0.27.2
    source: `cast<U>(&self) -> Result<&Bound, CastError>` (same shape as the old `downcast`, error
    type renamed `DowncastError` → `CastError` but discarded by the `if let Ok` here);
    `unsafe fn cast_into_unchecked<U>(self) -> Bound` (same `unsafe`/owned shape as before).

**Verification:** All criteria green.

- `grep -n 'pyo3' Cargo.toml` → single match, line 35, version "0.27"
- `Cargo.lock` shows `pyo3 0.27.2` (+ 4 sibling crates 0.27.2)
- `cargo build -p iscc-py` — exit 0
- `cargo clippy -p iscc-py -- -D warnings` — exit 0, clean (the two deprecation warnings the bump
    surfaced are resolved)
- `cargo fmt --all --check` — exit 0
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` — exit 0, `cp310-abi3` wheel installed
- `uv run pytest` — 286 passed, 1 warning (pre-existing iscc_core Pydantic-V1 / Py3.14
    `UserWarning`, confirmed unrelated via `pytest -W error`)
- `cargo tree -p iscc-py -i pyo3` — single `pyo3 v0.27.2`
- No deprecated APIs remain: 0 `allow_threads`, 0 `PyResult<PyObject>`, 0 `downcast` in lib.rs; 7
    `detach`, 17 `Py<PyAny>` intact
- `mise run check` (pre-commit stage) + `prek --hook-stage pre-push` (clippy, Rust tests, ty check,
    ruff S/C901, Ruby lint, pytest) — all Passed, exit 0

**Next:** Continue the incremental migration **PyO3 0.27 → 0.28** (issue #1). The proven recipe
holds: bump pin → `cargo update -p pyo3` → build/clippy(`-D warnings`)/fmt →
`uv run maturin develop` → `uv run pytest` (286). 0.27 was the second consecutive hop requiring a
source edit (after 0.26), both small deprecation renames — expect later hops may also touch source;
treat `-D warnings` as the gate. The raw `pyo3::ffi::*` CPython C-API sites (`PySequence_List`,
`PyList_GetItem`, `PyLong_AsLong`, `Bound::from_owned_ptr`) and `#[pyo3(signature = ...)]` macros
have survived every hop incl. 0.27. RustSec advisories still clear only at 0.29 (2 hops away —
expected, not a failure). Also open and unstarted: CRAP `--fail-above 30` hardening (issue #2, HUMAN
REVIEW REQUESTED on the spec edit) and the `iai-callgrind` perf-regression CI gate (issue #3).

**Notes:**

- The 0.27 `downcast` → `cast` rename is the `Bound`/`Borrowed` cast family unification PyO3 has
    been rolling out; semantics are identical (still `isinstance`-based subtype-aware cast). The
    error type changed `DowncastError` → `CastError` but the code discards it (`if let Ok`), so no
    callers are affected.
- Scope was tight (Cargo.toml + Cargo.lock + lib.rs — one binding file, well within the 3-file
    limit). No public API change, no hot-path change (the edits are in the video-frame extraction
    helper, not a benchmarked `gen_*_v0` compute path; pytest video benchmark shows iscc-lib still
    ~3.5× faster than iscc-core, unchanged).
- Ran `mise run format` equivalent via the prek hooks; all formatting (mdformat, rustfmt, taplo)
    clean before commit, so no pre-push mdformat rejection expected on context files.
