## 2026-02-27 — Review of: Propagate 7 new Tier 1 symbols to Python bindings

**Verdict:** PASS

**Summary:** Clean, well-scoped implementation propagating 3 functions (`encode_component`,
`iscc_decode`, `json_to_data_url`) and 4 constants (`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`,
`IO_READ_SIZE`, `TEXT_NGRAM_SIZE`) to Python bindings. All 172 tests pass, clippy clean, all 14
pre-commit hooks pass. The advance agent correctly identified that `__all__` had 34 entries (not 35
as next.md estimated), bringing the final count to 41 after adding 7 new symbols.

**Verification:**

- [x] `cargo test -p iscc-py` passes — 0 Rust unit tests (no Rust-level tests in iscc-py), clean
- [x] `cargo clippy -p iscc-py -- -D warnings` clean
- [x] `maturin develop` succeeds — wheel built and installed
- [x] `uv run pytest tests/ -x` passes — 172 tests (159 existing + 13 new), 0 failures
- [x] `uv run ruff check crates/iscc-py/python/` clean
- [x] `uv run ruff format --check crates/iscc-py/python/` clean
- [x] `python -c "from iscc_lib import encode_component, iscc_decode, json_to_data_url"` exits 0
- [x] `python -c "from iscc_lib import META_TRIM_NAME, ...; assert META_TRIM_NAME == 128"` exits 0
- [x] `python -c "import iscc_lib; assert len(iscc_lib.__all__) >= 41"` — count is 41 (34 + 7, not
    42 as next.md estimated with incorrect baseline of 35)
- [x] All 14 pre-commit hooks pass (`mise run check`)

**Issues found:**

- (none)

**Codex review:** Codex (gpt-5.2) reviewed HEAD (advance commit) and found two minor items: (1)
`test_encode_component_roundtrip` assertion `result.isalnum() or "-" in result` is loose — true but
the roundtrip test provides stronger validation. (2) Stub docs say `mtype: 0–6` but `mtype=5` (ISCC)
is rejected — this is technically accurate since 5 maps to a valid enum variant but is rejected by
`encode_component` business logic, and the `:raises ValueError:` doc covers it. Neither item is
actionable.

**Next:** All 30 Tier 1 Rust symbols are now accessible from Python (41 entries in `__all__`
including result types, hashers, and `__version__`). Next logical step is issue #5 layer 2: adding
dict `meta` parameter acceptance to `gen_meta_code_v0` in the Python binding using
`json_to_data_url` from `_lowlevel`. Alternatively, issue #4 (PIL pixel data for
`gen_image_code_v0`) is another small Python-only wrapper enhancement. Both are independent and can
be done in any order. After Python binding parity, propagate the 7 new symbols to Node.js, WASM, C
FFI, Go, and Java bindings (6 remaining bindings × 7 symbols each).

**Notes:** The `__all__` count discrepancy (41 vs estimated 42) is a planning estimation error, not
an implementation bug. The baseline was 34, not 35 — all 7 new symbols were correctly added. Issues
#6 (enums) and #8 (core_opts) are partially addressed by the 4 constants and `encode_component`
being available, but the `IntEnum` wrappers and `SimpleNamespace` are still outstanding.
