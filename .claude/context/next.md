# Next Work Package

## Step: Migrate PyO3 0.27 → 0.28 (issue #1, incremental toward 0.29)

## Goal

Advance the workspace `pyo3` pin one minor version (0.27 → 0.28) toward the target of `0.29.0`,
where two RustSec advisories shipped inside the published Python wheel finally clear. This is the
next hop in issue #1's deliberate one-minor-per-step migration; it keeps the diff small and
reviewable.

## Scope

- **Modify**: `Cargo.toml` (line 35 `pyo3` workspace pin `0.27` → `0.28`)
- **Modify**: `Cargo.lock` (regenerated via `cargo update -p pyo3` — generated file, not counted)
- **Modify**: `crates/iscc-py/src/lib.rs` (ONLY if new deprecations/breakages surface under
    `-D warnings`)
- **Reference**: `crates/iscc-py/Cargo.toml` (consumes pin via
    `workspace = true, features = ["extension-module"]`)
- **Reference**: `crates/iscc-py/pyproject.toml` (maturin config; carries NO pyo3 version, no edit
    needed)
- **Reference**: PyO3 migration guide https://pyo3.rs/main/migration (0.27 → 0.28 section)

## Not In Scope

- Do NOT jump straight to 0.29 (or 0.28.x beyond the next minor) — one minor hop per step.
- Do NOT touch the CRAP `--fail-above 30` hardening (issue #2 — HUMAN REVIEW REQUESTED on the spec).
- Do NOT start the `iai-callgrind` perf gate (issue #3 — valgrind unavailable in the devcontainer).
- Do NOT refactor the raw `pyo3::ffi::*` C-API sites or `#[pyo3(signature = ...)]` macros unless a
    0.28 breakage forces it; keep edits to the minimum the compiler demands.
- Do NOT remove `abi3-py310` or alter the `extension-module` feature layering.

## Implementation Notes

Proven recipe (held for 0.24→0.27):

1. Bump `Cargo.toml` line 35: `pyo3 = { version = "0.28", features = ["abi3-py310"] }`.
2. `cargo update -p pyo3` to regenerate `Cargo.lock` (pyo3 + sibling crates pyo3-build-config,
    pyo3-ffi, pyo3-macros, pyo3-macros-backend bump together).
3. `cargo build -p iscc-py`, then `cargo clippy -p iscc-py -- -D warnings` — fix any new deprecation
    warnings. 0.26 needed `allow_threads`→`detach` + `PyObject`→`Py<PyAny>`; 0.27 needed
    `downcast`/`downcast_into_unchecked` → `cast`/`cast_into_unchecked`. Expect 0.28 may also
    require a small mechanical rename — treat `-D warnings` as the gate, port exactly what it
    flags.
4. `cargo fmt --all --check`.
5. `uv run maturin develop -m crates/iscc-py/Cargo.toml` (builds the `cp310-abi3` wheel locally).
6. `uv run pytest` (286 tests expected to pass; one pre-existing unrelated iscc_core
    Pydantic-V1/Py3.14 warning is fine).

Current lib.rs state for reference: 728 lines, 7 `detach`, 17 `Py<PyAny>`, 0 `allow_threads`. The
raw `pyo3::ffi::*` sites (`PySequence_List`, `PyList_GetItem`, `PyLong_AsLong`,
`Bound::from_owned_ptr`) have survived every hop — do not pre-emptively rewrite them.

RustSec advisories clear ONLY at 0.29 (one further hop after this) — their persistence after 0.28 is
NOT a verification failure for this step. Do not gate this step on advisory clearance.

## Verification

- `grep -n 'pyo3' Cargo.toml` → single match, line 35, `version = "0.28"` with `abi3-py310`
- `grep -A1 'name = "pyo3"' Cargo.lock` → `version = "0.28.x"`
- `cargo build -p iscc-py` exits 0
- `cargo clippy -p iscc-py -- -D warnings` exits 0, clean
- `cargo fmt --all --check` exits 0
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` exits 0 (cp310-abi3 wheel installed)
- `uv run pytest` passes (286 tests, only the known pre-existing iscc_core warning)
- `cargo tree -p iscc-py -i pyo3` shows a single `pyo3 v0.28.x` (no duplicate versions)
- No deprecated APIs remain in `crates/iscc-py/src/lib.rs`: `allow_threads` count is 0

## Done When

The workspace `pyo3` pin is `0.28`, `Cargo.lock` resolves a single `pyo3 0.28.x`, and the full build
/ clippy(`-D warnings`) / fmt / `maturin develop` / `pytest` chain passes green.
