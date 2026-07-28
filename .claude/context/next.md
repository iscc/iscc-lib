# Next Work Package

## Step: Add `gen_iscc_id_v1` to the Python binding + differential test

## Goal

Begin the #43 Part 2 fan-out by exposing the core's `gen_iscc_id_v1` on the Python surface (the
flagship drop-in for `iscc-core`) and pinning it to the reference with a pytest differential test —
a named resolution criterion of issue "ISCC-IDv1 is unsupported outside Go". CI is already GREEN on
develop (the state.md "Semver RED" is a phantom: the `Semver` job is `continue-on-error: true`,
ci.yml L355, and both runs on tip `98205f2` concluded `success`), so no "fix CI" step is owed.

## Alternatives Considered

- **Chosen:** Python binding for `gen_iscc_id_v1` — the reference-parity anchor; its differential
    infrastructure (`iscc_core` dev-dep, `tests/test_iscc_decode_conformance.py`) already exists, so
    this is the strongest correctness signal and the cleanest template for the remaining surfaces.
- **Rejected — one "11-surface fan-out" step (handoff suggestion):** each binding tech uses a
    different wrapper pattern (PyO3 dict vs napi vs wasm vs C ABI vs JNI vs Magnus vs UniFFI), so it
    is NOT genuinely-identical fan-out and blows the file budget. Each surface is its own step.
- **Rejected — FFI first (unblocks cpp/dotnet):** heavier (C ABI + `iscc.h` regen + freshness gate),
    a poor first template; do it after the reference-parity Python anchor lands.

## Scope

- **Modify**: `crates/iscc-py/src/lib.rs` (add `#[pyfunction] gen_iscc_id_v1` returning
    `PyDict{"iscc"}`, register in `#[pymodule]`); `crates/iscc-py/python/iscc_lib/__init__.py`
    (import as `_gen_iscc_id_v1`, add `IsccIdResult(IsccResult)` with `iscc: str`, public wrapper,
    re-export, `__all__`); `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (stub, docstring-only
    body)
- **Create**: `tests/test_iscc_id_v1.py` (differential test — does not count toward budget)
- **Reference**: `.claude/context/specs/rust-core.md` → "ISCC-IDv1 Operations (Experimental)";
    `gen_iscc_code_v0` wrapper pattern (lib.rs:332, __init__.py:270);
    `tests/test_iscc_decode_conformance.py` (import-`iscc_core` differential pattern);
    `crates/iscc-py/CLAUDE.md` ("adding a Tier 1 function")

## Not In Scope

- The Tier-1 32→33 count/doc sweep — a separate #43 step; do NOT edit any "32 symbols" text or
    `docs/**`, `notes/**`, `README`/`CLAUDE.md` count strings here.
- Any other binding surface (napi, wasm, ffi, jni, rb, uniffi, dotnet, cpp, go) or the Go rename.
- Any core (`crates/iscc-lib`) change — `gen_iscc_id_v1`/`IsccIdResult` already exist there.
- A "now"/clock convenience or a `decode_iscc_id_v1` — both deliberately excluded by spec.

## Implementation Notes

- Signature mirrors the reference: `gen_iscc_id_v1(timestamp, hub_id=0, realm_id=0)`. **`timestamp`
    is required — no clock default** (core is clock-free). Wrapper returns `IsccIdResult`. Call core
    as `iscc_lib::gen_iscc_id_v1(timestamp, hub_id, realm_id)`; map `Err` → `PyValueError`. No
    `py.detach` (trivial compute). `realm_id` maps to the core's `realm: u8`; `hub_id: u16`,
    `timestamp: u64`.
- `_lowlevel.pyi` stub body is a docstring only — no trailing `...` (ruff PIE790/PYI048).
- Differential test: `import iscc_core`, assert
    `iscc_lib.gen_iscc_id_v1(ts,hub,realm)["iscc"] ==   iscc_core.gen_iscc_id_v1(ts, hub, realm)["iscc"]`
    over a small grid of explicit timestamps × hub × realm∈{0,1} (never pass `timestamp=None` —
    the reference would call the clock). Include the golden
    `gen_iscc_id_v1(1751831876325218, 1, 0)["iscc"] == "ISCC:MAIGHFECJMOPMIAB"`, assert `realm_id=2`
    raises `ValueError`, and verify both `result["iscc"]` and `result.iscc` access.

## Verification

- `maturin develop -m crates/iscc-py/Cargo.toml` builds, then `pytest tests/test_iscc_id_v1.py`
    passes (differential vs installed `iscc_core` + golden + validation).
- `python -c "from iscc_lib import gen_iscc_id_v1, IsccIdResult; assert gen_iscc_id_v1(1751831876325218,1,0)['iscc']=='ISCC:MAIGHFECJMOPMIAB'"`
    exits 0.
- `pytest` (full Python suite) still passes — no regression.
- `ruff check crates/iscc-py tests` and `ruff format --check crates/iscc-py tests` clean.
- `uv run ty check` clean (the new `.pyi` stub type-checks).
- `cargo clippy -p iscc-py --all-targets -- -D warnings` clean.

## Done When

`gen_iscc_id_v1` is importable from `iscc_lib`, returns an `IsccIdResult` matching `iscc_core`
byte-for-byte, and all lint/type/clippy/pytest gates stay clean.
