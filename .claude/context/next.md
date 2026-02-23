# Next Work Package

## Step: Fix `ty` type checker error in Python streaming functions

## Goal

Replace `hasattr(data, "read")` with `isinstance(data, bytes)` inversion in `gen_data_code_v0` and
`gen_instance_code_v0` so the `ty` type checker passes. This is the sole blocking issue from the
last review — pre-push hooks fail until it's fixed.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/python/iscc_lib/__init__.py` — fix 2 type-narrowing patterns (lines ~135, ~142)
- **Reference**:
    - `.claude/context/handoff.md` — exact error messages and recommended fix
    - `.claude/context/learnings.md` — `ty` limitation with `hasattr()` documented

## Implementation Notes

**The fix** (identical for both functions): replace

```python
if hasattr(data, "read"):
    data = data.read()
```

with

```python
if not isinstance(data, bytes):
    data = data.read()
```

This gives `ty` proper type narrowing: in the `if` body, `data` is `BinaryIO` (the only remaining
type in the `bytes | BinaryIO` union after ruling out `bytes`), so `.read()` is valid. Runtime
behavior is identical — anything that isn't `bytes` is treated as a stream and `.read()` is called.

**No other changes needed.** The `BinaryIO` import and type annotations are already correct. Tests
already pass — the fix only affects static analysis.

**Verify with**: `uv run ty check crates/iscc-py/python/iscc_lib/__init__.py` — should report 0
errors after the fix.

## Verification

- `uv run ty check crates/iscc-py/python/iscc_lib/__init__.py` reports 0 errors
- `pytest tests/` passes (all 63 tests including streaming tests)
- `ruff check crates/iscc-py/python/iscc_lib/__init__.py` clean
- `cargo test -p iscc-lib` still passes (no Rust changes)

## Done When

`ty` type checker passes with zero errors on `__init__.py` and all existing tests remain green.
