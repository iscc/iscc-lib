## 2026-02-23 — Fix `ty` type checker error in Python streaming functions

**Done:** Replaced `hasattr(data, "read")` with `not isinstance(data, bytes)` in both
`gen_data_code_v0` and `gen_instance_code_v0` wrapper functions. This gives `ty` proper type
narrowing (`BinaryIO` in the `if` body) while preserving identical runtime behavior.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/__init__.py`: Changed type-narrowing guard on lines 135 and 142
    from `if hasattr(data, "read")` to `if not isinstance(data, bytes)`

**Verification:**

- `uv run ty check crates/iscc-py/python/iscc_lib/__init__.py` → "All checks passed!" (0 errors)
- `pytest tests/` → 63 passed (including all streaming tests)
- `ruff check` → clean
- `cargo test -p iscc-lib` → 143 passed
- `mise run check` → all 14 pre-commit hooks passed

**Next:** All quality gates now pass (pre-commit and pre-push). The project is feature-complete with
all 9 `gen_*_v0` functions, bindings for Python/Node.js/WASM/C, documentation, benchmarks, and CI.
Consider working on any remaining documentation, release preparation, or performance optimization
tasks from the target state.

**Notes:** Minimal, surgical fix — two lines changed, no new code, no behavioral change. The
`isinstance` inversion is semantically equivalent: anything that isn't `bytes` in the
`bytes | BinaryIO` union is `BinaryIO`, so `.read()` is valid for both the type checker and at
runtime.
