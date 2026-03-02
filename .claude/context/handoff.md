## 2026-03-02 — Expose `add_units` in Python binding (issue #21)

**Done:** Added `add_units: bool` parameter to `gen_sum_code_v0` in both the PyO3 Rust binding and
the Python wrapper. When `add_units=True`, the returned dict includes a `"units"` key with
`[Data-Code, Instance-Code]` ISCC strings. When `False` (default), the key is omitted. Updated
`SumCodeResult` type annotation to include `units: list[str] | None`.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: Added `add_units` param to `#[pyo3(signature)]` (now 4 params),
    passed through to `iscc_lib::gen_sum_code_v0`, added `if let Some(units)` dict insertion
- `crates/iscc-py/python/iscc_lib/__init__.py`: Added `units: list[str] | None` type annotation to
    `SumCodeResult`, added `add_units: bool = False` param to wrapper, passed through to lowlevel
    call
- `tests/test_smoke.py`: Added 3 tests: `test_gen_sum_code_v0_units_enabled` (verifies units list
    with 2 ISCC strings), `test_gen_sum_code_v0_units_disabled` (verifies units key absent by
    default), `test_gen_sum_code_v0_units_attribute_access` (verifies `result.units` attribute
    access)

**Verification:**

- `cargo build -p iscc-py` — compiles clean
- `cargo clippy -p iscc-py -- -D warnings` — clean
- `mise run check` — 14/14 hooks pass
- `mise run test` — all tests pass (312 Rust + 207 Python, including 3 new smoke tests)
- `gen_sum_code_v0(path, add_units=True)` returns dict with `"units"` key containing 2 ISCC strings
- `gen_sum_code_v0(path)` (default) returns dict without `"units"` key

**Next:** Expose `add_units` in Node.js binding (`crates/iscc-napi`). The NAPI pattern is already
documented in learnings.md — `NapiSumCodeResult` struct needs an `Option<Vec<String>>` field for
`units`, and the `gen_sum_code_v0` function needs `Option<bool>` for `add_units`.

**Notes:** The Python extension must be rebuilt with `maturin develop` after changing the Rust
binding code. `mise run test` triggers `cargo test` which rebuilds the Rust code, but the installed
Python `.so` is a separate artifact. The `maturin develop` step is needed before `pytest` can pick
up the new signature. This is a known dev workflow requirement.
