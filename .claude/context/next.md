# Next Work Package

## Step: Complete Python bindings to 23/23 Tier 1 symbols

## Goal

Add the remaining 3 Tier 1 symbols (`soft_hash_video_v0`, `DataHasher`, `InstanceHasher`) to the
Python bindings, completing the 23/23 Tier 1 milestone. This makes `iscc_lib` a full drop-in
replacement for `iscc-core` at the API level.

## Scope

- **Modify**: `crates/iscc-py/src/lib.rs` — add `soft_hash_video_v0` as `#[pyfunction]`, add
    `DataHasher` and `InstanceHasher` as `#[pyclass]` wrappers
- **Modify**: `crates/iscc-py/python/iscc_lib/__init__.py` — re-export all 3 symbols, add Python
    wrapper classes for DataHasher/InstanceHasher with BinaryIO support
- **Modify**: `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — type stubs for all 3
- **Reference**: `crates/iscc-lib/src/streaming.rs` — Rust `DataHasher`/`InstanceHasher` API
- **Reference**: `crates/iscc-lib/src/lib.rs` lines 526-549 — `soft_hash_video_v0` signature
- **Reference**: `reference/iscc-core/iscc_core/code_data.py` — `DataHasherV0` (constructor takes
    optional `data`)
- **Reference**: `reference/iscc-core/iscc_core/code_instance.py` — `InstanceHasherV0` (constructor
    takes optional `data`)

## Implementation Notes

### `soft_hash_video_v0` (simple — follows existing `#[pyfunction]` pattern)

- Signature: `fn soft_hash_video_v0(frame_sigs: Vec<Vec<i32>>, bits: u32) -> PyResult<PyObject>`
- Use `#[pyo3(signature = (frame_sigs, bits=64))]` for Python default
- Returns `bytes` (not a dict) — this is a soft hash helper, not a `gen_*` function
- Maps `IsccError` → `PyValueError` with `map_err`
- Returns `PyBytes::new(py, &result)` since it produces raw bytes
- Re-export directly in `__init__.py` (like `alg_simhash`, no wrapper needed)

### `DataHasher` and `InstanceHasher` (`#[pyclass]` — new pattern)

- Use `Option<Inner>` pattern for consuming `finalize()`:
    ```rust
    #[pyclass(name = "DataHasher")]
    struct PyDataHasher {
        inner: Option<iscc_lib::DataHasher>,
    }
    ```
- `#[new]` creates `Self { inner: Some(iscc_lib::DataHasher::new()) }`
- `update(&mut self, data: &[u8])` calls `inner.as_mut().ok_or(already finalized)?.update(data)`
- `finalize(&mut self, py, bits)` calls `inner.take().ok_or(already finalized)?.finalize(bits)`,
    then builds a PyDict (same pattern as `gen_data_code_v0`/`gen_instance_code_v0`)
- Raise `PyValueError("DataHasher already finalized")` if called after finalize
- Register with `m.add_class::<PyDataHasher>()?` in the module init
- Use `#[pyo3(signature = (bits=64))]` on `finalize`

### Python wrappers in `__init__.py`

- Create `DataHasher` and `InstanceHasher` wrapper classes that accept optional `data` in `__init__`
    (matching iscc-core's `DataHasherV0(data=None)` / `InstanceHasherV0(data=None)`)
- `update()` accepts `bytes | BinaryIO` — use the `isinstance` inversion pattern:
    `if not isinstance(data, bytes): data = data.read()`
- `finalize()` returns `DataCodeResult` / `InstanceCodeResult` (the existing IsccResult subclasses)
- Import the lowlevel classes as `_DataHasher` / `_InstanceHasher` to avoid name collision

### Type stubs in `_lowlevel.pyi`

- Add `soft_hash_video_v0(frame_sigs: list[list[int]], bits: int = 64) -> bytes`
- Add `class DataHasher` with `__init__`, `update(data: bytes)`, `finalize(bits: int = 64) -> dict`
- Add `class InstanceHasher` with same shape

### Tests

- Add tests for `soft_hash_video_v0`:
    - Zero-vector input returns zero bytes
    - Range-vector input matches reference hex (from iscc-core test vectors)
    - Empty input raises ValueError
- Add tests for `DataHasher`:
    - Empty data matches `gen_data_code_v0(b"")`
    - Multi-chunk streaming matches one-shot
    - BinaryIO input works via `io.BytesIO`
    - Double finalize raises ValueError
- Add tests for `InstanceHasher`:
    - Empty data matches `gen_instance_code_v0(b"")`
    - Multi-chunk streaming matches one-shot (iscc, datahash, filesize)
    - BinaryIO input works via `io.BytesIO`
    - Double finalize raises ValueError
- Add conformance vector tests for both hashers (iterate `data.json` sections, compare streaming vs
    one-shot)

Build with: `VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop -m crates/iscc-py/Cargo.toml`

## Verification

- `VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop -m crates/iscc-py/Cargo.toml` succeeds
- `pytest tests/` passes (existing 116 + new tests, expect ~140+ total)
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `soft_hash_video_v0` returns correct bytes for reference test cases
- `DataHasher` streaming produces identical results to `gen_data_code_v0` for all conformance
    vectors
- `InstanceHasher` streaming produces identical results to `gen_instance_code_v0` for all
    conformance vectors
- All 23 Tier 1 symbols are importable from `iscc_lib`

## Done When

All verification criteria pass and Python bindings expose 23/23 Tier 1 symbols with full test
coverage including conformance vectors for both streaming hashers.
