# Next Work Package

## Step: Add Python API reference page with docstrings

## Goal

Create the Python API reference documentation page using mkdocstrings, with descriptive docstrings
for all 9 `gen_*_v0` functions. This directly addresses the target requirement "Covers Rust API,
Python API" and is the highest-value documentation content page.

## Scope

- **Create**: `docs/api.md`
- **Modify**: `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (add docstrings), `zensical.toml` (add
    api.md to nav)
- **Reference**: `crates/iscc-lib/src/api.rs` (Rust function signatures and doc comments),
    `crates/iscc-py/python/iscc_lib/__init__.py` (re-export structure), iscc-usearch API reference
    page style (https://usearch.iscc.codes/)

## Implementation Notes

### 1. Add docstrings to `_lowlevel.pyi`

Each of the 9 functions needs a concise docstring describing:

- What the function generates (which ISCC code type)
- Parameter descriptions with types already visible from annotations
- Return value description (JSON string with `iscc` field)

Example pattern:

```python
def gen_meta_code_v0(
    name: str,
    description: str | None = None,
    meta: str | None = None,
    bits: int = 64,
) -> str:
    """Generate an ISCC Meta-Code from content metadata.

    :param name: Title or name of the content (max 128 chars after normalization).
    :param description: Optional text description (max 4096 chars after normalization).
    :param meta: Optional JSON metadata for deterministic encoding.
    :param bits: Bit length of the code body (default 64).
    :return: JSON string containing ``iscc`` and ``metahash`` fields.
    """
    ...
```

Use Sphinx-style docstrings (`:param:`, `:return:`) matching the `docstring_style = "sphinx"`
setting in `zensical.toml`.

Reference `crates/iscc-lib/src/api.rs` for accurate parameter descriptions. Key details from
learnings:

- All functions return JSON strings (not parsed objects)
- `gen_audio_code_v0` takes `list[int]` (Chromaprint signed i32 features)
- `gen_video_code_v0` takes `list[list[int]]` (list of Chromaprint frame signatures)
- `gen_mixed_code_v0` takes `list[str]` (ISCC code strings, optional "ISCC:" prefix)
- `gen_data_code_v0` and `gen_instance_code_v0` take `bytes` (raw binary data)
- `gen_iscc_code_v0` takes `list[str]` codes + `bool` wide flag

### 2. Create `docs/api.md`

Create an API reference page that uses mkdocstrings to auto-generate documentation from the Python
module. Structure:

```markdown
# API Reference

Brief intro explaining these are the Python bindings and all functions return JSON strings.

## Functions

::: iscc_lib
    options:
      show_root_heading: false
      members_order: source
```

The `:::iscc_lib` directive tells mkdocstrings to document the `iscc_lib` module. Since
`__init__.py` re-exports all 9 functions, mkdocstrings will document them all. The handler is
already configured in `zensical.toml` with `paths = ["crates/iscc-py/python/iscc_lib"]`.

griffe (mkdocstrings' backend) uses static analysis, not import, so it should resolve the
`_lowlevel.pyi` stubs without needing the native module installed.

If griffe fails to resolve the re-exports from `_lowlevel`, fall back to documenting
`iscc_lib._lowlevel` directly:

```markdown
::: iscc_lib._lowlevel
```

### 3. Update `zensical.toml` nav

Change the nav from:

```toml
nav = ["index.md"]
```

to:

```toml
nav = ["index.md", "api.md"]
```

## Verification

- `uv run zensical build` succeeds without errors
- `site/api/index.html` exists and contains documentation for all 9 `gen_*_v0` functions
- Each function shows its signature with type annotations
- Each function shows its docstring description
- All existing tests still pass (`cargo test -p iscc-lib` — 143 tests)

## Done When

The advance agent is done when `uv run zensical build` produces a site with an API reference page
showing all 9 documented `gen_*_v0` functions with signatures and descriptions.
