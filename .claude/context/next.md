# Next Work Package

## Step: Python dict returns for all 9 gen functions

## Goal

Convert all 9 `gen_*_v0` PyO3 bindings from returning plain `String` to returning `PyDict` with all
fields from the Rust `*CodeResult` structs, making `iscc_lib` a drop-in replacement for `iscc-core`.

## Scope

- **Modify**: `crates/iscc-py/src/lib.rs` — convert each function to return `PyResult<PyObject>`
    building a Python dict from the result struct fields
- **Modify**: `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — change return types from `str` to
    `dict[str, Any]`
- **Reference**: `crates/iscc-lib/src/types.rs` (result struct fields),
    `.claude/context/specs/python-bindings.md` (field listing and optional-field rules),
    `crates/iscc-lib/tests/data.json` (conformance output fields)

Tests to update (excluded from 3-file limit):

- `tests/test_conformance.py` — verify all dict fields (not just `iscc` string)
- `tests/test_smoke.py` — update assertions for dict access

## Implementation Notes

### PyO3 Dict Construction Pattern

Each function changes from `.map(|r| r.iscc)` to building a `PyDict`. Use PyO3's `Python::with_gil`
and `PyDict::new_bound`. Pattern for each function:

```rust
use pyo3::types::PyDict;

fn gen_meta_code_v0(py: Python<'_>, ...) -> PyResult<PyObject> {
    let r = iscc_lib::gen_meta_code_v0(name, description, meta, bits)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let dict = PyDict::new(py);
    dict.set_item("iscc", r.iscc)?;
    dict.set_item("name", r.name)?;
    dict.set_item("metahash", r.metahash)?;
    if let Some(desc) = r.description {
        dict.set_item("description", desc)?;
    }
    if let Some(meta) = r.meta {
        dict.set_item("meta", meta)?;
    }
    Ok(dict.into())
}
```

### Per-Function Dict Fields

Follow the spec in `.claude/context/specs/python-bindings.md`:

| Function               | Always present                 | Conditionally present                             |
| ---------------------- | ------------------------------ | ------------------------------------------------- |
| `gen_meta_code_v0`     | `iscc`, `name`, `metahash`     | `description` (if provided), `meta` (if provided) |
| `gen_text_code_v0`     | `iscc`, `characters`           | —                                                 |
| `gen_image_code_v0`    | `iscc`                         | —                                                 |
| `gen_audio_code_v0`    | `iscc`                         | —                                                 |
| `gen_video_code_v0`    | `iscc`                         | —                                                 |
| `gen_mixed_code_v0`    | `iscc`, `parts`                | —                                                 |
| `gen_data_code_v0`     | `iscc`                         | —                                                 |
| `gen_instance_code_v0` | `iscc`, `datahash`, `filesize` | —                                                 |
| `gen_iscc_code_v0`     | `iscc`                         | —                                                 |

Key details:

- `description` and `meta` are **omitted** from the dict when `None` (matching iscc-core behavior
    where optional fields are simply absent from the returned dict)
- `characters` is `int` (Python), `parts` is `list[str]`, `filesize` is `int`
- `datahash` and `metahash` are hex strings with `1e20` prefix

### PyO3 Function Signature

Each function needs `py: Python<'_>` as first parameter (PyO3 automatically injects it). The return
type changes to `PyResult<PyObject>`. The `#[pyo3(signature = ...)]` stays the same.

Example signature change:

```rust
// Before:
fn gen_meta_code_v0(name: &str, ...) -> PyResult<String>
// After:
fn gen_meta_code_v0(py: Python<'_>, name: &str, ...) -> PyResult<PyObject>
```

### Type Stub Updates

Change `_lowlevel.pyi` return types from `-> str` to `-> dict[str, Any]`. Add
`from typing import Any` at the top. Update docstring `:return:` lines to describe the dict keys.

### Test Updates

**`tests/test_conformance.py`**: Change all assertions to verify dict fields:

- `assert result == tc["outputs"]["iscc"]` → `assert result["iscc"] == tc["outputs"]["iscc"]`
- For meta tests: also assert `result["name"]`, `result["metahash"]`, and conditionally
    `result.get("description")` and `result.get("meta")` against `tc["outputs"]`
- For text tests: assert `result["characters"] == tc["outputs"]["characters"]`
- For mixed tests: assert `result["parts"] == tc["outputs"]["parts"]`
- For instance tests: assert `result["datahash"]`, `result["filesize"]` against outputs

**`tests/test_smoke.py`**: Change `assert result == "ISCC:..."` to
`assert result["iscc"] == "ISCC:..."`

### Edge Case: PyDict::new API

In PyO3 0.22+, the API is `PyDict::new(py)` returning `Bound<'_, PyDict>`. Call `.into()` to convert
to `PyObject`. Check the PyO3 version in `Cargo.toml` to confirm the exact API.

## Verification

- `cargo test -p iscc-py` passes (Rust-side compilation check)
- `VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop -m crates/iscc-py/Cargo.toml` succeeds
- `pytest tests/test_conformance.py` — all 49 tests pass with dict access
- `pytest tests/test_smoke.py` — all 3 smoke tests pass with dict access
- `gen_meta_code_v0("Test")["iscc"]` returns a valid ISCC string
- `gen_meta_code_v0("Test")["name"]` returns `"Test"`
- `gen_meta_code_v0("Test")["metahash"]` starts with `"1e20"`
- `gen_text_code_v0("Hello world")["characters"]` returns an int
- `gen_instance_code_v0(b"")["datahash"]` starts with `"1e20"`
- `gen_instance_code_v0(b"")["filesize"]` returns `0`
- `gen_mixed_code_v0(codes)["parts"]` returns a list of strings
- `ruff check tests/` clean
- All conformance vector additional fields (name, metahash, characters, parts, datahash, filesize)
    match between `result[field]` and `tc["outputs"][field]`

## Done When

The advance agent is done when all 9 `gen_*_v0` Python functions return `dict` objects with all
fields matching iscc-core, all conformance tests verify every output field (not just `iscc`), and
`pytest` passes cleanly.
