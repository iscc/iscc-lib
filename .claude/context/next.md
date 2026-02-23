# Next Work Package

## Step: Implement hybrid IsccResult(dict) objects in Python

## Goal

Replace direct `_lowlevel` re-exports with typed `IsccResult(dict)` subclass wrappers so all 9
`gen_*_v0` Python functions return objects supporting both dict-style (`result['iscc']`) and
attribute-style (`result.iscc`) access with IDE completion.

## Scope

- **Modify**: `crates/iscc-py/python/iscc_lib/__init__.py` — add `IsccResult` base class, 9 typed
    subclasses, and 9 wrapper functions replacing the direct re-exports
- **Reference**: `.claude/context/specs/python-bindings.md` (full spec with class definitions, field
    annotations, wrapper function signatures, and export list)
- **Reference**: `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (current `_lowlevel` signatures)

Tests to update/add (excluded from 3-file limit):

- `tests/test_smoke.py` — add assertions for attribute access and `isinstance` checks
- `tests/test_conformance.py` — no changes needed (already uses `result["iscc"]` dict access which
    still works)

## Implementation Notes

### Base Class

```python
class IsccResult(dict):
    """ISCC result with both dict-style and attribute-style access."""

    def __getattr__(self, name):
        try:
            return self[name]
        except KeyError:
            raise AttributeError(name) from None
```

### Typed Subclasses

Create exactly 9 subclasses with class-level type annotations per the spec table in
`python-bindings.md`:

- `MetaCodeResult`: `iscc: str`, `name: str`, `metahash: str`, `description: str | None`,
    `meta: str | None`
- `TextCodeResult`: `iscc: str`, `characters: int`
- `ImageCodeResult`: `iscc: str`
- `AudioCodeResult`: `iscc: str`
- `VideoCodeResult`: `iscc: str`
- `MixedCodeResult`: `iscc: str`, `parts: list[str]`
- `DataCodeResult`: `iscc: str`
- `InstanceCodeResult`: `iscc: str`, `datahash: str`, `filesize: int`
- `IsccCodeResult`: `iscc: str`

**Important:** The type annotations are for IDE/type-checker documentation only. Do NOT define
`__init__` or `__init_subclass__` — the `dict.__init__` handles data storage, and `__getattr__`
handles attribute access delegation.

### Wrapper Functions

Each wrapper calls the `_lowlevel` function and wraps the result in the typed subclass. Import
`_lowlevel` functions with a prefix to avoid name conflicts:

```python
from iscc_lib._lowlevel import (
    gen_meta_code_v0 as _gen_meta_code_v0,
    ...
)

def gen_meta_code_v0(name, description=None, meta=None, bits=64) -> MetaCodeResult:
    """Generate an ISCC Meta-Code from content metadata."""
    return MetaCodeResult(_gen_meta_code_v0(name, description, meta, bits))
```

The wrapper function signatures MUST match `_lowlevel.pyi` exactly (same parameter names and
defaults). Check `_lowlevel.pyi` for each function's signature.

### Exports

Update `__all__` to include all 9 wrapper functions, `IsccResult`, and the 9 typed result classes
(19 total symbols).

### Test Updates

In `tests/test_smoke.py`, add tests verifying:

- `result.iscc` attribute access works
- `isinstance(result, dict)` is `True`
- `isinstance(result, IsccResult)` is `True`
- `isinstance(result, InstanceCodeResult)` is `True` (for the specific subclass)
- `json.dumps(result)` works without error

Existing conformance tests should pass unchanged since `result["iscc"]` still works.

### Edge Cases

- `IsccResult.__getattr__` must raise `AttributeError` (not `KeyError`) for missing attributes —
    this is required for `hasattr()`, `getattr()`, and pickling to work correctly
- The `from None` in `raise AttributeError(name) from None` suppresses the `KeyError` chain
- Do NOT add `__setattr__` or `__delattr__` — keep it simple, dict mutation through dict methods

## Verification

- `pytest tests/test_conformance.py` — all 49 tests pass (dict access still works)
- `pytest tests/test_smoke.py` — all tests pass including new attribute access tests
- `gen_meta_code_v0("Test").iscc` returns a valid ISCC string (attribute access)
- `gen_meta_code_v0("Test")["iscc"]` returns the same string (dict access)
- `isinstance(gen_meta_code_v0("Test"), dict)` is `True`
- `isinstance(gen_meta_code_v0("Test"), IsccResult)` is `True`
- `isinstance(gen_meta_code_v0("Test"), MetaCodeResult)` is `True`
- `json.dumps(gen_meta_code_v0("Test"))` succeeds
- `gen_instance_code_v0(b"").datahash` starts with `"1e20"` (attribute on subclass)
- `gen_text_code_v0("test").characters` returns an `int` (attribute on subclass)
- `ruff check crates/iscc-py/python/ tests/` clean
- `IsccResult` and all 9 typed result classes appear in `iscc_lib.__all__`

## Done When

The advance agent is done when all 9 `gen_*_v0` Python functions return typed `IsccResult(dict)`
subclass instances, both dict and attribute access work, all existing conformance tests pass
unchanged, and new smoke tests verify the hybrid behavior.
