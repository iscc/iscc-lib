## 2026-02-27 — Accept dict meta in Python gen_meta_code_v0

**Done:** Added `dict` support for the `meta` parameter in `gen_meta_code_v0` Python wrapper. When
`meta` is a `dict`, it is serialized to compact JSON via
`json.dumps(separators=(",", ":"), ensure_ascii=False)` and then converted to a data URL via
`json_to_data_url()`. The Rust core handles JCS canonicalization internally. Added 5 new tests
covering dict meta, ld+json media type detection, string/dict equivalence, and regression tests for
string and None meta values.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/__init__.py`: Added `import json as _json`, widened
    `gen_meta_code_v0` signature to `meta: str | dict | None`, added dict→data URL conversion before
    calling `_gen_meta_code_v0`
- `tests/test_new_symbols.py`: Added 5 new tests (`test_gen_meta_dict_meta_basic`,
    `test_gen_meta_dict_meta_ld_json`, `test_gen_meta_dict_meta_matches_string`,
    `test_gen_meta_str_meta_still_works`, `test_gen_meta_dict_meta_none_still_works`), updated
    module docstring and imports

**Verification:**

- `uv run pytest tests/test_new_symbols.py -x` passes — 18 tests (13 existing + 5 new)
- `uv run pytest tests/ -x` passes — 177 tests, 0 failures
- `uv run ruff check crates/iscc-py/python/` clean
- `uv run ruff format --check crates/iscc-py/python/` clean
- `cargo clippy -p iscc-py -- -D warnings` clean
- `python -c "from iscc_lib import gen_meta_code_v0; r = gen_meta_code_v0('Test', meta={'key': 'val'}); assert r['meta'].startswith('data:application/json;base64,')"`
    exits 0
- All 14 pre-commit hooks pass (`mise run check`)

**Next:** Issue #5 (dict meta) is now complete. Next logical steps are: (1) Issue #4 — PIL pixel
data for `gen_image_code_v0` (Python-only wrapper), (2) Issue #6 — `MT`/`ST`/`VS` IntEnum classes
for Python, (3) Issue #8 — `core_opts` SimpleNamespace. All are independent Python-wrapper-only
changes.

**Notes:** Used `_json` import alias to avoid polluting the module namespace (json is only used
internally). The implementation is minimal — 3 lines of logic added to the wrapper function. No Rust
changes required.
