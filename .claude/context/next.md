# Next Work Package

## Step: Propagate 7 new Tier 1 symbols to Python bindings

## Goal

Add `encode_component`, `iscc_decode`, `json_to_data_url`, and 4 algorithm constants
(`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`) to the Python
binding, bringing it from 23/30 to 30/30 Tier 1 symbols.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/src/lib.rs` — add 3 `#[pyfunction]` wrappers + 4 constants in module init
    - `crates/iscc-py/python/iscc_lib/__init__.py` — import and re-export the 7 new symbols, add to
        `__all__`
    - `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — add type stubs for 3 functions + 4 constants
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` — Rust signatures for `encode_component`, `iscc_decode`,
        `json_to_data_url`, and 4 constants
    - `crates/iscc-py/src/lib.rs` — existing PyO3 wrapper patterns (`iscc_decompose`, `encode_base64`
        for simple pass-through examples)

## Not In Scope

- dict `meta` parameter for `gen_meta_code_v0` (issue #5 layer 2) — separate step after these
    symbols land
- PIL pixel data for `gen_image_code_v0` (issue #4) — Python-only wrapper enhancement, separate step
- `MT`/`ST`/`VS` `IntEnum` classes (issue #6 binding-level enums) — separate step
- `core_opts` `SimpleNamespace` (issue #8 binding-level wrapper) — separate step
- Propagation to other bindings (Node.js, WASM, C FFI, Java, Go) — one binding at a time

## Implementation Notes

### PyO3 wrappers (`lib.rs`)

**`encode_component`**: Thin wrapper taking
`(mtype: u8, stype: u8, version: u8, bit_length: u32, digest: &[u8]) -> PyResult<String>`. Map error
with `map_err(PyValueError)`. Follow the existing `iscc_decompose` pattern.

**`iscc_decode`**: Returns `PyResult<(u8, u8, u8, u8, Py<PyBytes>)>`. The Rust function returns
`IsccResult<(u8, u8, u8, u8, Vec<u8>)>`. Wrap the `Vec<u8>` in `PyBytes::new(py, &digest)` for the
Python side to receive `bytes`. The first 4 elements are `u8` integers. Use the Python `py`
parameter (add `py: Python<'_>` to the function signature). Return a tuple.

**`json_to_data_url`**: Simplest — `(json: &str) -> PyResult<String>`. Map error with
`map_err(PyValueError)`.

**Constants**: In the `iscc_lowlevel` module init function, add:

```rust
m.add("META_TRIM_NAME", iscc_lib::META_TRIM_NAME)?;
m.add("META_TRIM_DESCRIPTION", iscc_lib::META_TRIM_DESCRIPTION)?;
m.add("IO_READ_SIZE", iscc_lib::IO_READ_SIZE)?;
m.add("TEXT_NGRAM_SIZE", iscc_lib::TEXT_NGRAM_SIZE)?;
```

Register all 3 functions with `wrap_pyfunction!` in the module init.

### Python wrapper (`__init__.py`)

Import the 7 new symbols from `_lowlevel`. Constants are simple re-exports (no wrapper needed).
Functions are also simple re-exports — no wrapper logic needed (unlike `gen_data_code_v0` which adds
streaming). Add all 7 to `__all__`.

### Type stubs (`_lowlevel.pyi`)

Add stubs for:

- `def encode_component(mtype: int, stype: int, version: int, bit_length: int, digest: bytes) -> str: ...`
- `def iscc_decode(iscc: str) -> tuple[int, int, int, int, bytes]: ...`
- `def json_to_data_url(json: str) -> str: ...`
- `META_TRIM_NAME: int`
- `META_TRIM_DESCRIPTION: int`
- `IO_READ_SIZE: int`
- `TEXT_NGRAM_SIZE: int`

Include docstrings matching the existing stub style (`:param:`, `:return:`, `:raises:`).

### Tests

Add a new test file `tests/test_new_symbols.py` (or extend `tests/test_smoke.py`) with:

- `test_encode_component_roundtrip` — encode a known digest, verify output is a valid ISCC string
- `test_iscc_decode_roundtrip` — encode then decode, verify components match
- `test_json_to_data_url_plain` — JSON without `@context` returns `data:application/json;base64,...`
- `test_json_to_data_url_ld` — JSON with `@context` returns `data:application/ld+json;base64,...`
- `test_constants` — verify `META_TRIM_NAME == 128`, `META_TRIM_DESCRIPTION == 4096`,
    `IO_READ_SIZE == 4_194_304`, `TEXT_NGRAM_SIZE == 13`
- Error cases: `encode_component` with invalid mtype, `iscc_decode` with invalid string,
    `json_to_data_url` with invalid JSON — all raise `ValueError`

### Build & format

Run `mise run format` before committing. Run
`VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop -m crates/iscc-py/Cargo.toml` to build the
updated native module before running pytest.

## Verification

- `cargo test -p iscc-py` passes (existing Rust unit tests still pass)
- `cargo clippy -p iscc-py -- -D warnings` clean
- `VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop -m crates/iscc-py/Cargo.toml` succeeds
- `uv run pytest tests/ -x` passes (all existing + new tests)
- `uv run ruff check crates/iscc-py/python/` clean
- `uv run ruff format --check crates/iscc-py/python/` clean
- `python -c "from iscc_lib import encode_component, iscc_decode, json_to_data_url"` exits 0
- `python -c "from iscc_lib import META_TRIM_NAME, META_TRIM_DESCRIPTION, IO_READ_SIZE, TEXT_NGRAM_SIZE; assert META_TRIM_NAME == 128"`
    exits 0
- `python -c "import iscc_lib; assert len(iscc_lib.__all__) >= 42"` exits 0 (was 35 + 7 new = 42)

## Done When

All verification criteria pass — 30/30 Tier 1 symbols are accessible from Python with correct
behavior and types.
