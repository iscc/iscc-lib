## 2026-02-27 — Review of: Add MT/ST/VS IntEnums and core_opts to Python

**Verdict:** PASS

**Summary:** Clean, focused implementation adding three `IntEnum` classes (`MT`, `ST`, `VS`), a
`core_opts` `SimpleNamespace`, and an `iscc_decode` wrapper returning IntEnum-typed values to the
Python bindings. 14 new tests cover all enum values, aliases, core_opts attributes, and iscc_decode
IntEnum wrapping. All 198 tests pass, all 14 pre-commit hooks pass, no quality gate circumvention.

**Verification:**

- [x] `uv run pytest tests/test_new_symbols.py -x` passes — 39 tests (25 existing + 14 new)
- [x] `uv run pytest tests/ -x` passes — 198 tests, 0 failures
- [x] `uv run ruff check crates/iscc-py/python/` clean
- [x] `uv run ruff format --check crates/iscc-py/python/` clean
- [x] `python -c "from iscc_lib import MT, ST, VS; assert MT.DATA == 3; assert ST.TEXT == 0; assert VS.V0 == 0"`
    exits 0
- [x] `python -c "from iscc_lib import core_opts; assert core_opts.meta_trim_name == 128"` exits 0
- [x] `python -c "from iscc_lib import iscc_decode, MT; r = iscc_decode('GAA2XTPPAERUKZ4J'); assert isinstance(r[0], MT)"`
    exits 0 (note: the `GABTQLB6CQ6ILWLO` code from next.md was invalid — advance agent correctly
    substituted a valid code and documented the reason)
- [x] All 14 pre-commit hooks pass (`mise run check`)

**Issues found:**

- (none)

**Codex review:** Codex (gpt-5.2) flagged the `ST.TEXT=0` alias for `ST.NONE=0` as potentially
confusing, but this is correct — the Rust `SubType` enum and iscc-core both have TEXT and NONE
sharing value 0. The first definition (NONE) is canonical in IntEnum, TEXT is the alias. It also
raised test brittleness with hardcoded values, but tests cross-reference both hardcoded values and
module constants (e.g., `core_opts.meta_trim_name == 128` AND `== META_TRIM_NAME`). No actionable
findings.

**Next:** Issues #6, #7, #8 Python-side work is complete but issues remain open for other-bindings
propagation (Node.js TS const enums, WASM, C FFI, Java, Go). Issue #5 (dict meta) also needs
propagation to other bindings. The define-next agent should prioritize either: (a) batching
enum/core_opts/iscc_decode propagation to remaining bindings, or (b) tackling Issue #5 dict meta
propagation, or (c) addressing any other items from issues.md/state.md gaps. The `__all__` count is
now 45 entries (41 prior + MT, ST, VS, core_opts).

**Notes:** The third verification criterion in next.md used an invalid test vector
(`GABTQLB6CQ6ILWLO` — 8-byte body but header indicates 128-bit/16-byte digest). The advance agent
handled this well by substituting a valid code (`GAA2XTPPAERUKZ4J`) and documenting the discrepancy.
Future define-next should validate test vectors before including them in verification criteria.
