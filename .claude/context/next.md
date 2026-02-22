# Next Work Package

## Step: Expose all 9 gen\_\*\_v0 functions in PyO3 bindings with type stub

## Goal

Complete the Python bindings by exposing the remaining 8 `gen_*_v0` functions through PyO3 and
adding the `_lowlevel.pyi` type stub. This fixes the pre-push `ty` hook failure and makes all ISCC
functions callable from Python.

## Scope

- **Create**: `crates/iscc-py/python/iscc/_lowlevel.pyi` — type stub for the native module
- **Modify**: `crates/iscc-py/src/lib.rs` — add PyO3 wrappers for 8 remaining functions
- **Modify**: `crates/iscc-py/python/iscc/__init__.py` — re-export all 9 functions
- **Reference**: `crates/iscc-lib/src/lib.rs` (Rust API signatures), `crates/iscc-py/src/lib.rs`
    (existing `gen_instance_code_v0` wrapper pattern)

## Implementation Notes

### PyO3 wrapper pattern

Follow the existing `gen_instance_code_v0` wrapper exactly — `#[pyfunction]` with
`#[pyo3(signature)]` for defaults, map `IsccError` to `PyValueError`:

```rust
#[pyfunction]
#[pyo3(signature = (data, bits=64))]
fn gen_instance_code_v0(data: &[u8], bits: u32) -> PyResult<String> {
    iscc_lib::gen_instance_code_v0(data, bits).map_err(|e| PyValueError::new_err(e.to_string()))
}
```

### Type mappings (Rust core → PyO3 wrapper)

| Rust core signature | PyO3 wrapper type | Python type       |
| ------------------- | ----------------- | ----------------- |
| `&[u8]`             | `&[u8]`           | `bytes`           |
| `&str`              | `&str`            | `str`             |
| `Option<&str>`      | `Option<&str>`    | `str \| None`     |
| `&[i32]`            | `Vec<i32>`        | `list[int]`       |
| `&[Vec<i32>]`       | `Vec<Vec<i32>>`   | `list[list[int]]` |
| `&[&str]`           | `Vec<String>`     | `list[str]`       |
| `bool`              | `bool`            | `bool`            |

For `Vec` types, pass a reference (`&codes_vec`) to the Rust core function. For `Vec<String>` →
`&[&str]`, convert via: `let refs: Vec<&str> = codes.iter().map(|s| s.as_str()).collect();`

### Default arguments (matching iscc-core Python API)

| Function                                                       | Defaults     |
| -------------------------------------------------------------- | ------------ |
| `gen_meta_code_v0(name, description=None, meta=None, bits=64)` | 3 defaults   |
| `gen_text_code_v0(text, bits=64)`                              | 1 default    |
| `gen_image_code_v0(pixels, bits=64)`                           | 1 default    |
| `gen_audio_code_v0(cv, bits=64)`                               | 1 default    |
| `gen_video_code_v0(frame_sigs, bits=64)`                       | 1 default    |
| `gen_mixed_code_v0(codes, bits=64)`                            | 1 default    |
| `gen_data_code_v0(data, bits=64)`                              | 1 default    |
| `gen_instance_code_v0(data, bits=64)`                          | already done |
| `gen_iscc_code_v0(codes, wide=false)`                          | 1 default    |

### `_lowlevel.pyi` stub

Declare all 9 functions with Python type annotations. Example:

```python
def gen_meta_code_v0(
    name: str,
    description: str | None = None,
    meta: str | None = None,
    bits: int = 64,
) -> str: ...


def gen_instance_code_v0(data: bytes, bits: int = 64) -> str: ...
```

### `__init__.py` update

Import and re-export all 9 functions in `__all__`.

### Register all functions in the module

Add `m.add_function(wrap_pyfunction!(fn_name, m)?)?;` for each of the 9 functions in the
`iscc_lowlevel` module function.

## Verification

- `cargo build -p iscc-py` succeeds
- `cargo clippy -p iscc-py -- -D warnings` is clean
- `VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop -m crates/iscc-py/Cargo.toml` succeeds
- `python -c "from iscc import gen_meta_code_v0, gen_text_code_v0, gen_image_code_v0, gen_audio_code_v0, gen_video_code_v0, gen_mixed_code_v0, gen_data_code_v0, gen_instance_code_v0, gen_iscc_code_v0; print('OK')"`
    prints OK
- `python -c "from iscc import gen_meta_code_v0; print(gen_meta_code_v0('Hello'))"` prints
    `ISCC:AAAWKLHFPV6OPKDG`
- `uv run ty check` passes (no `unresolved-import` error on `iscc._lowlevel`)
- `pytest tests/` passes (existing smoke tests still work)

## Done When

All 9 `gen_*_v0` functions are importable from the `iscc` Python package, `ty` type checking passes,
and all verification criteria above succeed.
