# Next Work Package

## Step: PyO3 incremental bump 0.24 → 0.25

## Goal

Advance the PyO3 security migration (issue: "Update PyO3 to latest release") by one minor version,
from the current `0.24` pin to `0.25`, keeping the Python bindings building and all tests green.
This is the next hop on the path to `0.29.0`, where the two shipped RustSec advisories finally
clear.

## Scope

- **Modify**: `Cargo.toml` (bump the `pyo3` pin in `[workspace.dependencies]`, line ~35),
    `crates/iscc-py/src/lib.rs` (only if PyO3 0.25 API changes force source edits — see notes)
- **Modify (generated, excluded from file limit)**: `Cargo.lock` (refresh via
    `cargo update -p pyo3`)
- **Reference**: `.claude/context/handoff.md` (PASS verdict + recipe),
    `.claude/context/learnings.md` ("PyO3 minor bumps" under Tooling), the PyO3 0.25 migration guide
    (https://pyo3.rs/main/migration), `crates/iscc-py/src/lib.rs`, `crates/iscc-py/pyproject.toml`,
    `crates/iscc-py/CLAUDE.md` (abi3 constraint, type-mapping rules)

## Not In Scope

- **Do NOT jump past 0.25.** This is one reviewed minor hop. Do not bump straight to 0.26–0.29 — the
    issue mandates incremental migration, one minor per step.
- Do not add `cargo-audit` / `cargo-deny` config or claim the advisories are cleared — they only
    clear at 0.29, so advisory state is not a criterion for this step.
- Do not touch the CRAP gate (`--fail-above 30` hardening) or the `iai-callgrind` perf gate, or any
    file under `.github/workflows/` — those are separate open issues.
- Do not change the `abi3-py310` target or the `>=3.10` `requires-python` floor.
- Do not refactor the existing raw-FFI `unsafe` blocks or change binding semantics (result-dict
    keys/value types stay identical) beyond what 0.25 strictly requires to compile.

## Implementation Notes

- Edit root `Cargo.toml` line ~35: `pyo3 = { version = "0.25", features = ["abi3-py310"] }`. The
    `iscc-py` crate consumes it via `pyo3 = { workspace = true, features = ["extension-module"] }`,
    and `pyproject.toml` keeps the `pyo3/extension-module` maturin feature with no version — leave
    both as-is. `pyo3` is used by no other crate, so the blast radius is just `crates/iscc-py/`.
- Run `cargo update -p pyo3` to refresh `Cargo.lock` to a `0.25.x` release, then
    `cargo build -p iscc-py` and fix whatever the compiler flags.
- PyO3 0.25 tightens `IntoPyObject` / lifetime rules that 0.24 did not require, so unlike the
    0.23→0.24 hop, source edits may be needed. The most likely touchpoints in `lib.rs` are the ~14
    `Ok(dict.into())` returns (converting `Bound<'py, PyDict>` → `PyObject`) and the
    `into_pyobject(py)?.into()` chain near line 452. Follow the migration guide; prefer the minimal
    change that compiles clean under `-D warnings` (watch for new deprecation warnings, which clippy
    treats as errors).
- The pre-existing raw `pyo3::ffi::*` calls (`Bound::from_owned_ptr`, `downcast_into_unchecked`,
    etc.) map to the stable CPython C API and historically survive minor bumps — only patch them if
    0.25 actually breaks the signatures.
- Build/test via uv: `uv run maturin develop -m crates/iscc-py/Cargo.toml` then `uv run pytest`
    (maturin/pytest are not on PATH directly). `uv` may warn that `VIRTUAL_ENV` differs from the
    project venv — harmless.
- Run `mise run format` before committing so the pre-push mdformat/fmt hooks do not reject the push.

## Verification

- `grep 'pyo3 = { version = "0.25"' Cargo.toml` matches, and `cargo tree -p iscc-py -i pyo3` shows
    `pyo3 v0.25.x`
- `cargo build -p iscc-py` — clean
- `cargo clippy -p iscc-py -- -D warnings` — clean, no new deprecation warnings
- `cargo fmt --check` — clean
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` — builds the abi3 wheel
    (`iscc_lib-0.4.0-cp310-abi3-*.whl`)
- `uv run pytest` — 286 tests pass (no count regression vs the 0.24 baseline)
- `abi3-py310` still present in `Cargo.toml`; `crates/iscc-py/pyproject.toml` still
    `requires-python = ">=3.10"`
- `mise run check` — all pre-commit hooks pass

## Done When

PyO3 is pinned at `0.25` with `Cargo.lock` refreshed, `abi3-py310` preserved, and build, clippy,
fmt, the maturin wheel, and all 286 pytest tests pass green.
