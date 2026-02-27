## 2026-02-27 — Propagate 7 new Tier 1 symbols to Python bindings

**Done:** Added 3 PyO3 function wrappers (`encode_component`, `iscc_decode`, `json_to_data_url`) and
4 algorithm constants (`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`)
to the Python binding, bringing the exported symbol count from 34 to 41 in `__all__`.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: Added 3 `#[pyfunction]` wrappers and 4 `m.add()` constant
    registrations in the module init. `iscc_decode` returns `PyBytes` for the digest component.
- `crates/iscc-py/python/iscc_lib/__init__.py`: Imported and re-exported all 7 new symbols from
    `_lowlevel`. Added them to `__all__` (now 41 entries).
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: Added type stubs for 3 functions + 4 constants
    with `:param:`, `:return:`, `:raises:` docstrings matching existing style.
- `tests/test_new_symbols.py`: New test file with 13 tests covering happy paths, roundtrips, type
    checks, and error cases.

**Verification:** All 172 Python tests pass (159 existing + 13 new).
`cargo clippy --workspace --all-targets -D warnings` clean. All 14 pre-commit hooks pass. Ruff lint
and format clean. Python imports verified for all 7 symbols.

**Next:** The `__all__` count is 41 (not 42 as next.md estimated — the original count was 34, not
35). All 7 new symbols are correctly exported and tested. Next logical step is issue #5 layer 2:
dict meta acceptance for `gen_meta_code_v0` in the Python binding, which can now use
`json_to_data_url` from `_lowlevel`.

**Notes:** The next.md verification criterion `len(__all__) >= 42` is off by one because the
original `__all__` had 34 entries (not 35 as estimated). The actual count of 41 = 34 + 7 is correct.
All 30 Tier 1 Rust symbols are now accessible from Python.
