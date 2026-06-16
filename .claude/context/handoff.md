## 2026-06-16 — Review of: Add streaming `SumHasher` to the Python bindings

**Verdict:** PASS

**Summary:** The advance agent added a PyO3 `PySumHasher` (`#[pyclass(name = "SumHasher")]`) backed
by the core `iscc_lib::streaming::SumHasher`, plus a Pythonic `SumHasher` wrapper mirroring
`DataHasher`/`InstanceHasher`, a `.pyi` stub, docs, and 11 new tests. The implementation faithfully
copies the established `gen_sum_code_v0` dict-construction pattern (omitting `units` when `None`)
and the finalize-once `Option<inner>` pattern. Scope was clean — no WASM, core, GIL, or Tier 1
changes. This closes the Python half of issue #37.

**Verification:**

- [x] `cargo clippy -p iscc-py -- -D warnings` — clean
- [x] `maturin develop --release -m crates/iscc-py/Cargo.toml` — builds successfully (abi3-py310
    wheel)
- [x] `pytest tests/test_streaming.py` — 58 passed (47 prior + 11 new)
- [x] `pytest` (full suite) — 279 passed
- [x] `python -c "import iscc_lib; assert 'SumHasher' in iscc_lib.__all__; assert iscc_lib.SumHasher"`
    — exits 0
- [x] Test asserts `SumHasher(...).finalize(bits, wide, add_units)` ==
    `gen_sum_code_v0(tempfile, ...)` across the (64/128/256, wide on/off, units on/off) grid —
    present and passing
- [x] Test asserts second `finalize()` and post-`finalize()` `update()` each raise `ValueError` —
    present and passing
- [x] `ruff check` / `ruff format --check` / `ty check` — all clean
- [x] `mise run check` (15 pre-commit hooks) — all Passed

**Issues found:**

- (none) — implementation matches the core API (`SumHasher::finalize(bits, wide, add_units)`) and
    the `gen_sum_code_v0` reference exactly. Tests use real data and verify against both the
    path-based `gen_sum_code_v0` and the manual two-hasher path.
- Minor doc fix applied directly by reviewer: the "Result types and their fields" table in
    `docs/howto/python.md` listed `SumCodeResult` without its optional `units?` field — now that the
    new SumHasher example demonstrates `add_units=True`, the table was inconsistent. Added `units?`
    (the same `?`-suffix convention used for other optional fields). No behavior change.

**Codex review:** No introduced correctness issues found. Codex confirmed the binding follows the
existing streaming-wrapper patterns, is registered and exported, and that tests cover the main
behavior and finalize-once semantics. No actionable findings.

**Next:** The WASM `SumHasher` wrapper — the **only remaining half of issue #37** (the issue entry
in issues.md has been rescoped to "WASM bindings" with a progress note). Add a `SumHasher` class to
`crates/iscc-wasm` over the shared core struct, mirroring the WASM `DataHasher`/`InstanceHasher`
finalize-once pattern; verify with `wasm-pack test --node` against `gen_sum_code_v0` and the
two-hasher path. After that, the highest-impact backlog items are: npm `optionalDependencies` fix
(#38), PyO3 0.23→0.29 security migration, or GIL release (#39).

**Notes:**

- `SumHasher` is intentionally NOT a crate-root Tier 1 re-export (it's a Python/WASM streaming
    convenience, not a symbol bound in all languages). The binding correctly uses the full path
    `iscc_lib::streaming::SumHasher`. Do NOT bump the documented "32 Tier 1 symbols / 2 streaming
    types" counts for it.
- GIL release (#39) is still open and untouched — when that work lands it should also cover the new
    `PySumHasher.update()` alongside `PyDataHasher`/`PyInstanceHasher`.
- learnings.md was pruned to 200 lines (archived the full devcontainer exec-bit incident write-up to
    learnings-archive.md, kept the actionable one-liner).
