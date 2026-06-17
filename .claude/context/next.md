# Next Work Package

## Step: Migrate PyO3 0.25 → 0.26 (issue #1, incremental toward 0.29)

## Goal

Advance the incremental PyO3 migration one minor version (0.25 → 0.26) so the published Python wheel
moves closer to 0.29, where two RustSec advisories clear. This is the in-progress v1.0.0 hardening
item (issue #1: "Update PyO3 to latest release"); one reviewed minor per step keeps each
breaking-change surface small and verifiable.

## Scope

- **Modify**: `Cargo.toml` (bump the `pyo3` workspace dependency pin from `"0.25"` to `"0.26"` on
    line 35 — keep `features = ["abi3-py310"]` intact)
- **Modify (only if the build/clippy demands it)**: `crates/iscc-py/src/lib.rs` — the single file
    holding all PyO3 bindings. The last two hops (0.23→0.24, 0.24→0.25) needed ZERO source changes;
    expect the same, but apply minimal fixes if `-D warnings` surfaces deprecations.
- **Reference**: `crates/iscc-py/Cargo.toml` (consumes the workspace dep with `extension-module`),
    `crates/iscc-py/pyproject.toml` (`features = ["pyo3/extension-module"]`, `abi3` per-platform
    wheel), `crates/iscc-py/CLAUDE.md` (binding rules), PyO3 migration guide
    https://pyo3.rs/main/migration (0.25 → 0.26 section), `issues.md` issue #1.

## Not In Scope

- Do NOT jump the pin past 0.26 (no 0.26 → 0.27 in this step; one minor per cycle).
- Do NOT expect the RustSec advisories to clear yet — they only clear at 0.29. Their continued
    presence at 0.26 is EXPECTED and is NOT a failure of this step. Do not add advisory-clearance as
    a pass/fail gate here.
- Do NOT touch the `iscc-lib` core crate (it carries no PyO3 dependency), nor any other binding.
- Do NOT refactor `lib.rs` beyond the minimum needed to clear deprecation warnings (no rewriting the
    raw `pyo3::ffi::*` call sites unless 0.26 actually breaks them).
- Do NOT change the CRAP gate or start the `iai-callgrind` perf gate (separate open issues).
- Do NOT alter `abi3-py310` or the `extension-module` feature wiring.

## Implementation Notes

Established recipe (proven on the last two hops):

1. Edit `Cargo.toml` line 35: `pyo3 = { version = "0.26", features = ["abi3-py310"] }`.
2. `cargo update -p pyo3` to regenerate `Cargo.lock` (a generated file — does not count toward the
    3-file limit; commit it).
3. `cargo build -p iscc-py` then `cargo clippy -p iscc-py -- -D warnings`. Watch specifically for
    `IntoPyObject` / lifetime / `Bound` API deprecations. The ~14 `Ok(dict.into())` → `PyObject`
    returns are the most likely 0.26 touchpoint if anything breaks; the raw `pyo3::ffi::*` C-API
    calls (`PySequence_List`, `PyList_GetItem`, `PyLong_AsLong`, `Bound::from_owned_ptr`,
    `downcast_into_unchecked`) map to stable CPython API and have survived every hop so far.
4. `cargo fmt --all` (or `mise run format`) before committing.
5. Rebuild the extension into the venv: `uv run maturin develop -m crates/iscc-py/Cargo.toml`.
6. `uv run pytest` — all tests must pass.

If `-D warnings` flags a deprecation that is not yet a hard error in 0.26, still fix it (treat
warnings as errors), so the next hop has no accumulated debt. Keep changes minimal and idiomatic.

## Verification

- `grep -n 'pyo3 = { version = "0.26"' Cargo.toml` — matches exactly one line (35).
- `grep -A1 'name = "pyo3"' Cargo.lock` — shows a `0.26.x` version.
- `grep -n 'abi3-py310' Cargo.toml` — still present on the pyo3 workspace dep.
- `cargo build -p iscc-py` exits 0.
- `cargo clippy -p iscc-py -- -D warnings` exits 0 (clean).
- `cargo fmt --all --check` exits 0.
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` exits 0.
- `uv run pytest` passes (no new failures; ~286 tests collected, incl. the 7 `test_gil.py`
    concurrency tests).

## Done When

The pyo3 pin is at 0.26 with a regenerated `Cargo.lock`, `abi3-py310` preserved, and all of
`cargo build`/`cargo clippy -D warnings`/`cargo fmt --check`/`maturin develop`/`pytest` pass green.
