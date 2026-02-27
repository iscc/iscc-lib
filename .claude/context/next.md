# Next Work Package

## Step: Add MT/ST/VS IntEnums and core_opts to Python

## Goal

Add the two remaining iscc-core drop-in extensions to the Python bindings: `MT`/`ST`/`VS` IntEnum
classes (Issue #6) and `core_opts` SimpleNamespace (Issue #8). Also wrap `iscc_decode` to return
IntEnum-typed tuple values instead of raw integers (the Python-side part of Issue #7). This closes
all Python iscc-core API gaps.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/python/iscc_lib/__init__.py` — add IntEnum classes, wrap `iscc_decode`, add
        `core_opts`
    - `tests/test_new_symbols.py` — add tests for enums, core_opts, iscc_decode wrapping
- **Reference**:
    - `reference/iscc-core/iscc_core/constants.py` — MT, ST, ST_CC, ST_ISCC, VS enum definitions
    - `reference/iscc-core/iscc_core/options.py` — core_opts structure
    - `.claude/context/specs/python-bindings.md` — spec for all 3 extensions
    - `crates/iscc-lib/src/codec.rs` — Rust MainType/SubType/Version enum values

## Not In Scope

- Modifying Rust core code — all changes are pure Python in `__init__.py`
- Adding `ST_CC`, `ST_ISCC`, `ST_ID`, `LN` or other specialized iscc-core enum types — only `MT`,
    `ST`, `VS` per spec
- Propagating these extensions to other bindings (Node.js, WASM, C FFI, Java, Go) — that's the next
    phase
- Updating `_lowlevel.pyi` type stubs — the public API types are in `__init__.py` (pure Python)
- Publishing to PyPI

## Implementation Notes

### IntEnum classes (Issue #6)

Add three `enum.IntEnum` classes to `__init__.py` (add `import enum` at top):

**`MT`** — MainType, 8 values matching Rust `MainType` enum:

```python
class MT(enum.IntEnum):
    META = 0
    SEMANTIC = 1
    CONTENT = 2
    DATA = 3
    INSTANCE = 4
    ISCC = 5
    ID = 6
    FLAKE = 7
```

**`ST`** — SubType, 8 values matching Rust `SubType` enum. Must include all values 0-7 so
`iscc_decode` can wrap any valid ISCC code. `TEXT` is an alias for `NONE` (both are value 0) — in
Python IntEnum the first definition wins and the second becomes an alias:

```python
class ST(enum.IntEnum):
    NONE = 0
    IMAGE = 1
    AUDIO = 2
    VIDEO = 3
    MIXED = 4
    SUM = 5
    ISCC_NONE = 6
    WIDE = 7
    TEXT = 0  # Alias — IntEnum allows duplicate values as aliases
```

**`VS`** — Version, 1 value:

```python
class VS(enum.IntEnum):
    V0 = 0
```

Place these classes after the `import` block but before the result type classes.

### Wrap iscc_decode (Issue #7 Python layer)

Currently `iscc_decode` is a direct pass-through from `_lowlevel`. Change it to a wrapper that
converts raw integer fields to IntEnum types:

```python
from iscc_lib._lowlevel import iscc_decode as _iscc_decode


def iscc_decode(iscc: str) -> tuple[MT, ST, VS, int, bytes]:
    """Decode an ISCC unit string into header components and raw digest."""
    mt, st, vs, length, digest = _iscc_decode(iscc)
    return MT(mt), ST(st), VS(vs), length, digest
```

### core_opts SimpleNamespace (Issue #8)

Add after the IntEnum classes:

```python
from types import SimpleNamespace

core_opts = SimpleNamespace(
    meta_trim_name=META_TRIM_NAME,
    meta_trim_description=META_TRIM_DESCRIPTION,
    io_read_size=IO_READ_SIZE,
    text_ngram_size=TEXT_NGRAM_SIZE,
)
```

The constants are already imported from `_lowlevel` and re-exported. `core_opts` provides the
attribute-access pattern that `iscc-core` uses (`ic.core_opts.meta_trim_name`).

### Update `__all__`

Add `"MT"`, `"ST"`, `"VS"`, `"core_opts"` to `__all__`. `iscc_decode` is already there.

### Tests

Add to `tests/test_new_symbols.py`:

1. `MT` values: `MT.META == 0`, `MT.FLAKE == 7`, `isinstance(MT.DATA, int)` (IntEnum is int)
2. `ST` values: `ST.NONE == 0`, `ST.TEXT == 0` (alias), `ST.WIDE == 7`
3. `VS` values: `VS.V0 == 0`
4. `core_opts` attributes: `core_opts.meta_trim_name == 128`,
    `core_opts.meta_trim_description == 4096`, `core_opts.io_read_size == 4_194_304`,
    `core_opts.text_ngram_size == 13`
5. `iscc_decode` returns IntEnum types: decode a known ISCC code, check `isinstance(result[0], MT)`,
    `isinstance(result[1], ST)`, `isinstance(result[2], VS)`
6. Round-trip: `encode_component(MT.DATA, ST.NONE, VS.V0, 64, digest)` → `iscc_decode(result)` →
    verify fields match

## Verification

- `uv run pytest tests/test_new_symbols.py -x` passes (25 existing + ~10 new tests)
- `uv run pytest tests/ -x` passes (all ~184+ tests)
- `uv run ruff check crates/iscc-py/python/` clean
- `uv run ruff format --check crates/iscc-py/python/` clean
- `python -c "from iscc_lib import MT, ST, VS; assert MT.DATA == 3; assert ST.TEXT == 0; assert VS.V0 == 0"`
    exits 0
- `python -c "from iscc_lib import core_opts; assert core_opts.meta_trim_name == 128"` exits 0
- `python -c "from iscc_lib import iscc_decode, MT; r = iscc_decode('GABTQLB6CQ6ILWLO'); assert isinstance(r[0], MT)"`
    exits 0

## Done When

All verification criteria pass — `MT`, `ST`, `VS` IntEnums, `core_opts` SimpleNamespace, and
IntEnum-typed `iscc_decode` are all functional and exported in `__all__`.
