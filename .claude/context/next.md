# Next Work Package

## Step: Migrate PyO3 0.28 → 0.29 (FINAL hop — closes issue #1 "Update PyO3 to latest release")

## Goal

Advance the workspace `pyo3` pin the final minor version (0.28 → 0.29.0) — the endpoint of issue #1,
where the two RustSec advisories shipped inside the published Python wheel (missing `Sync` bound on
`PyCFunction::new_closure`; OOB read in `BoundTupleIterator`/`BoundListIterator::nth_back`) finally
clear. After this lands, the published wheel no longer ships vulnerable PyO3 code and issue #1 can
be closed.

## Scope

- **Modify**: `Cargo.toml` (line 35 `pyo3` workspace pin `0.28` → `0.29`, keep `abi3-py310`)
- **Modify**: `Cargo.lock` (regenerated via `cargo update -p pyo3` — generated file, not counted)
- **Modify**: `crates/iscc-py/src/lib.rs` (ONLY if new deprecations/breakages surface under
    `-D warnings`)
- **Reference**: `crates/iscc-py/Cargo.toml` (consumes pin via
    `workspace = true, features = ["extension-module"]`)
- **Reference**: `crates/iscc-py/pyproject.toml` (maturin config; carries NO pyo3 version, no edit
    needed)
- **Reference**: PyO3 0.28→0.29 migration guide https://pyo3.rs/v0.29.0/migration/ and the 0.29
    CHANGELOG
- **Reference**: `.claude/context/handoff.md` (the 0.27→0.28 review — warns "compiles clean" is NOT
    proof of behavior-neutrality)

No docs/README reference the pyo3 version (grepped `docs/`, `README.md`, `crates/iscc-py/README.md`
— zero hits), so this is a pure internal binding change with no doc files in scope.

## Not In Scope

- Do NOT touch the CRAP `--fail-above 30` hardening (issue #2 — HUMAN REVIEW REQUESTED on the spec).
- Do NOT start the `iai-callgrind` perf gate (issue #3 — valgrind unavailable in the devcontainer).
- Do NOT flip the `Semver (cargo-semver-checks)` gate from `continue-on-error: true` to enforcing —
    that is a deliberate one-line follow-up tied to the v1.0.0 cut.
- Do NOT cut or prepare a v1.0.0 release (low issue, human-directed).
- Do NOT enable free-threaded mode or flip `gil_used` to `false`; keep the explicit
    `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697). Real free-threading needs a
    deliberate audit of the raw-FFI `extract_frame_sigs` borrows first.
- Do NOT refactor the 8 raw `pyo3::ffi::*` C-API sites or `#[pyo3(signature = ...)]` macros unless a
    0.29 breakage forces it; keep edits to the minimum the compiler demands.

## Implementation Notes

Proven recipe (held for 0.24→0.28; 0.26 and 0.27 each needed a small mechanical source edit):

1. Bump `Cargo.toml` line 35: `pyo3 = { version = "0.29", features = ["abi3-py310"] }`. Confirmed
    0.29.0 is the latest on crates.io and still supports `abi3-py310`.
2. `cargo update -p pyo3` to regenerate `Cargo.lock` (pyo3 + sibling crates pyo3-build-config,
    pyo3-ffi, pyo3-macros, pyo3-macros-backend bump together).
3. `cargo build -p iscc-py`, then `cargo clippy -p iscc-py -- -D warnings` — fix any new deprecation
    warnings. Recent precedent: 0.26 needed `allow_threads`→`detach` + `PyObject`→`Py<PyAny>`; 0.27
    needed `downcast`/`downcast_into_unchecked` → `cast`/`cast_into_unchecked`. Expect 0.29 may
    also require a small mechanical rename — treat `-D warnings` as the gate, port exactly what it
    flags.
4. `cargo fmt --all --check`.
5. `uv run maturin develop -m crates/iscc-py/Cargo.toml` (builds the `cp310-abi3` wheel locally).
6. `uv run pytest` (286 tests expected to pass; one pre-existing unrelated iscc_core
    Pydantic-V1/Py3.14 warning is fine).

**Do NOT trust "compiles clean" as proof of behavior-neutrality.** The 0.28 hop compiled clean yet
silently flipped the unspecified `#[pymodule]` `gil_used` default from `true` to `false`. Read the
0.28→0.29 migration guide's default-handling section and diff the pyo3 macros-backend defaults for
any further silent flip; keep the explicit `gil_used = true` (lib.rs:697).

The two RustSec advisories the bump targets live in PyO3's closure/iterator internals
(`PyCFunction::new_closure`, `nth_back` on bound tuple/list iterators) — this crate does not call
those APIs, so the fix is purely getting patched PyO3 into the lock file; no compute path changes.

Current lib.rs state for reference: 735 lines, 7 `detach`, 17 `Py<PyAny>`, 0 `allow_threads`, 8 raw
`pyo3::ffi::*` sites (`PySequence_List`, `PyList_GetItem`, `PyList_Size`, `PyLong_AsLong`,
`PyErr_Occurred`, `PyList_Check`) plus `Bound::from_owned_ptr().cast_into_unchecked()` (lib.rs:24).
These have survived every hop — do not pre-emptively rewrite them.

**Advisory verification caveat:** `cargo-audit` and `cargo-deny` are NOT installed in the
devcontainer and are NOT wired into CI/mise, so the advisories cannot be confirmed cleared with a
tool locally. The mechanical proxy is: `Cargo.lock` resolves `pyo3 0.29.x` with no pyo3 `< 0.29`
entries remaining — the patched releases ship the advisory fixes, so removing the older versions
from the lock removes the vulnerable code from the wheel.

## Verification

- `grep -n 'pyo3' Cargo.toml` → single match, line 35, `version = "0.29"` with `abi3-py310`
- `grep -A1 'name = "pyo3"' Cargo.lock` → `version = "0.29.x"`, and no other pyo3 `< 0.29` entry
    exists in `Cargo.lock`
- `cargo build -p iscc-py` exits 0
- `cargo clippy -p iscc-py -- -D warnings` exits 0, clean
- `cargo fmt --all --check` exits 0
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` exits 0 (cp310-abi3 wheel installed)
- `uv run pytest` passes (286 tests, only the known pre-existing iscc_core warning)
- `cargo tree -p iscc-py -i pyo3` shows a single `pyo3 v0.29.x` (no duplicate versions)
- `grep -n 'gil_used = true' crates/iscc-py/src/lib.rs` still present (default not regressed)
- `cargo clippy --workspace --all-targets -- -D warnings` exits 0 (pre-push defense)

## Done When

The workspace `pyo3` pin is `0.29`, `Cargo.lock` resolves a single `pyo3 0.29.x` with no older pyo3
entries remaining, the explicit `gil_used = true` default is preserved, and the full build /
clippy(`-D warnings`) / fmt / `maturin develop` / `pytest` chain passes green — clearing the issue
#1 advisories from the shipped wheel.
