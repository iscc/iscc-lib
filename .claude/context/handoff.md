## 2026-07-28 — Add `gen_iscc_id_v1` to the Python binding + differential test

**Done:** Exposed the core's `gen_iscc_id_v1` on the Python surface (the `iscc-core` drop-in) and
pinned it to the reference with a pytest differential test. Additive-only; no other #43 surface, no
doc/count sweep, no core change.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: added
    `#[pyfunction] gen_iscc_id_v1(timestamp, hub_id=0,   realm_id=0)` returning `PyDict{"iscc"}`
    (Err→`PyValueError`, no `py.detach`), registered in `#[pymodule]`.
- `crates/iscc-py/python/iscc_lib/__init__.py`: import `_gen_iscc_id_v1`,
    `IsccIdResult(IsccResult)`, public wrapper, re-export, `__all__` (2 entries).
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: docstring-only stub (no trailing `...`).
- `tests/test_iscc_id_v1.py` (new): differential vs installed `iscc_core` over ts×hub×realm{0,1},
    golden, defaults, and 3 validation cases.

**Verification:**

- `maturin develop -m crates/iscc-py/Cargo.toml` — built OK.
- `pytest tests/test_iscc_id_v1.py` — 7 passed. Full `pytest` — 451 passed, no regression.
- Golden smoke: `gen_iscc_id_v1(1751831876325218,1,0)['iscc']=='ISCC:MAIGHFECJMOPMIAB'` — exit 0.
- `ruff check` + `ruff format --check` (crates/iscc-py tests) — clean.
- `uv run ty check` — clean. `cargo clippy -p iscc-py --all-targets -- -D warnings` — clean.
- `mise run check` — all prek pre-commit hooks Passed; no context files reformatted.
- Ref signature confirmed via oracle: `iscc_core.gen_iscc_id_v1` is
    `(timestamp=None, hub_id=0,   realm_id=0)`; golden and `(0,0,1)→ISCC:MEIAAAAAAAAAAAAA` match.

**Next:** Continue the #43 fan-out — next cleanest reference-parity surface is FFI (`iscc-ffi`,
unblocks dotnet/cpp, needs `iscc.h` regen + freshness gate), then napi/wasm/jni/rb/uniffi, the Go
`EncodeIsccID`→`gen_iscc_id_v1` rename + `DecodeIsccID` deletion, and finally the Tier-1 32→33
doc/count sweep.

**Notes:**

- Wrapper param is `realm_id` (mirrors the reference's public name); it maps to the core's
    `realm: u8`. Tests always pass `timestamp` explicitly — `timestamp=None` would make the
    reference call the system clock.
- No `py.detach` (trivial compute), matching `gen_iscc_code_v0`.
- Tier 1 is 33 symbols in code; doc/count text still reads 32 by design (separate sweep step).
