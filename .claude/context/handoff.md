## 2026-02-27 — Review of: Accept PIL pixel data in Python gen_image_code_v0

**Verdict:** PASS

**Summary:** Clean, minimal implementation widening `gen_image_code_v0` in the Python wrapper to
accept `bytes | bytearray | memoryview | Sequence[int]` for the `pixels` parameter. The 3-line
change (`isinstance` check + `bytes()` conversion) follows the exact same pattern already used by
`gen_data_code_v0` and `gen_instance_code_v0`. All 184 tests pass, all 14 pre-commit hooks pass, no
quality gate circumvention. Issue #4 resolved and removed from issues.md.

**Verification:**

- [x] `uv run pytest tests/test_new_symbols.py -x` passes — 25 tests (18 existing + 7 new)
- [x] `uv run pytest tests/ -x` passes — 184 tests, 0 failures
- [x] `uv run ruff check crates/iscc-py/python/` clean
- [x] `uv run ruff format --check crates/iscc-py/python/` clean
- [x] `python -c "from iscc_lib import gen_image_code_v0; r = gen_image_code_v0(list(range(256))*4); assert 'iscc' in r"`
    exits 0
- [x] `python -c "from iscc_lib import gen_image_code_v0; r = gen_image_code_v0(bytearray(b'\\x80'*1024)); assert 'iscc' in r"`
    exits 0
- [x] All 14 pre-commit hooks pass (`mise run check`)

**Issues found:**

- (none)

**Codex review:** Codex (gpt-5.2) found no functional regressions or actionable issues. Confirmed
the change is small, self-contained, and correctly applies `bytes()` conversion.

**Next:** Issue #4 is done. Three remaining Python iscc-core drop-in extensions: (1) Issue #6 —
`MT`/`ST`/`VS` IntEnum classes for Python, (2) Issue #7 — `iscc_decode` returning IntEnum-typed
tuples (depends on #6), (3) Issue #8 — `core_opts` SimpleNamespace. Issues #6 and #8 are independent
additive changes to `__init__.py` and could potentially be batched. Issue #5 (dict meta) was
resolved in iteration 5 but remains in issues.md for the other-bindings propagation. After these
Python extensions, propagate new Tier 1 symbols to remaining bindings (Node.js, WASM, C FFI, Java,
Go).

**Notes:** The `Sequence` import was already present in `__init__.py` (line 6). Test coverage is
thorough: 4 type-acceptance tests + 3 equivalence tests verifying identical output across all input
types. The test count grew from 177 to 184 across the last two iterations (dict meta + PIL pixels).
