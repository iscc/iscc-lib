# Next Work Package

## Step: PyO3 incremental bump 0.23 → 0.24 (first migration increment)

## Goal

Begin the PyO3 security migration (issue: "Update PyO3 to latest release (security fixes)") by
advancing the workspace pin from `0.23` to `0.24` — the first of the planned incremental hops toward
`0.29.0`. This is a self-contained, locally verifiable step that keeps the Python wheel building and
all conformance tests green while moving toward clearing the two RustSec advisories (which only
fully clear at 0.29).

## Scope

- **Modify**: `Cargo.toml` (root — bump `pyo3` in `[workspace.dependencies]` from `"0.23"` to
    `"0.24"`, keep `features = ["abi3-py310"]`)
- **Modify**: `crates/iscc-py/src/lib.rs` (apply any PyO3 0.24 migration fixes the compiler
    surfaces; do not change behavior)
- **Reference**: `crates/iscc-py/Cargo.toml` (consumes the workspace pin with
    `features = ["extension-module"]`), `crates/iscc-py/pyproject.toml` (maturin
    `features = ["pyo3/extension-module"]` — no version there), `crates/iscc-py/CLAUDE.md` (abi3
    constraint, type-mapping rules), the PyO3 migration guide (https://pyo3.rs/v0.24.0/migration),
    `tests/test_conformance.py` + `tests/test_smoke.py` + `tests/test_gil.py` (the Python tests that
    must stay green)

## Not In Scope

- **Do NOT jump past 0.24** (no 0.25–0.29 in this step). The human explicitly asked for incremental
    migration following the PyO3 migration guide; each minor is its own reviewed step. The
    advisories clearing only happens at 0.29 — that is a later increment, not this one.
- **Do NOT** treat "RustSec advisories cleared" as a verification criterion for this step — at 0.24
    they are still present by design. The criterion here is "builds + tests stay green at 0.24".
- **Do NOT** add a `cargo-deny` / `cargo-audit` CI gate or `deny.toml` — none exists today and it is
    a separate concern.
- **Do NOT** touch `.github/workflows/ci.yml`, the `iai-callgrind` perf gate, the CRAP gate, or
    `.crap-baseline.json`. Keeping this step free of CI-infrastructure changes deliberately avoids
    entangling it with the just-landed (locally committed, not-yet-CI-verified) CRAP Phase 3 gate.
- **Do NOT** refactor the raw `pyo3::ffi::*` CPython-C-API code, change binding semantics, or
    add/remove result-dict keys.
- **Do NOT** change `abi3-py310` or the `>=3.10` minimum (one wheel per platform must be preserved).

## Implementation Notes

- The only version source is root `Cargo.toml` line ~35:
    `pyo3 = { version = "0.23", features = ["abi3-py310"] }`. `crates/iscc-py/Cargo.toml` references
    it via `pyo3 = { workspace = true, features = ["extension-module"] }`, and `pyo3` is used by
    **no other crate** — so the blast radius is just `crates/iscc-py/`.
- After bumping the pin, run `cargo update -p pyo3` to refresh `Cargo.lock` to a `0.24.x`, then
    `cargo build -p iscc-py` and fix whatever the compiler flags, following the 0.24 migration
    guide.
- Likely 0.24 touchpoints in `src/lib.rs`: the `Ok(dict.into())` → `PyObject` conversions (14 of
    them) and any `IntoPy`/`into()`-style conversions may need updating to the current
    `IntoPyObject` / `.into_any()` / `.unbind()` idiom if 0.24 tightens them. `.into_pyobject(py)?`
    (line ~452) is already on the modern API. The raw `pyo3::ffi::PySequence_List` /
    `PyList_GetItem` / `PyLong_AsLong` / `Bound::from_owned_ptr` / `downcast_into_unchecked` calls
    map to the stable CPython C API and are unlikely to change — but confirm `Bound::from_owned_ptr`
    still exists in 0.24 and adjust if it was renamed/deprecated.
- Keep changes minimal and behavior-preserving: bindings only translate the core API. Result dicts
    must keep exactly the same keys/value types (per `crates/iscc-py/CLAUDE.md`).
- `Cargo.lock` is a generated file and does not count toward the 3-file limit.

## Verification

- `grep -n 'pyo3 = { version = "0.24"' Cargo.toml` matches (workspace pin bumped), and
    `cargo tree -p iscc-py -i pyo3` reports a `0.24.x` resolved version.
- `cargo build -p iscc-py` succeeds with no errors.
- `cargo clippy -p iscc-py -- -D warnings` is clean.
- `cargo fmt --check` is clean.
- `maturin develop -m crates/iscc-py/Cargo.toml` builds the abi3 wheel, then `uv run pytest` passes
    all existing Python tests (conformance, smoke, streaming, gil, algo, text utils, new symbols) —
    no test count regression.
- `grep -n 'abi3-py310' Cargo.toml` still present (single-wheel-per-platform preserved) and
    `crates/iscc-py/pyproject.toml` still declares `requires-python = ">=3.10"`.

## Done When

The workspace `pyo3` pin is `0.24`, `crates/iscc-py` builds and passes `cargo clippy`/`cargo fmt`,
and `maturin develop` + `uv run pytest` are green with no behavior or test-count regression and
`abi3-py310` intact.
