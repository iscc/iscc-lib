## 2026-02-23 — Complete Python bindings to 23/23 Tier 1 symbols

**Done:** Added the remaining 3 Tier 1 symbols (`soft_hash_video_v0`, `DataHasher`,
`InstanceHasher`) to the Python bindings. `soft_hash_video_v0` is a thin `#[pyfunction]` returning
`PyBytes`. `DataHasher` and `InstanceHasher` use `#[pyclass]` with `Option<Inner>` pattern for
consuming `finalize()`, wrapped by pure-Python classes in `__init__.py` that add `BinaryIO` support
and optional constructor `data` parameter (matching iscc-core's `DataHasherV0`/`InstanceHasherV0`).

**Files changed:**

- `crates/iscc-py/src/lib.rs`: Added `soft_hash_video_v0` pyfunction, `PyDataHasher` and
    `PyInstanceHasher` pyclasses with `new`/`update`/`finalize` methods, registered all 3 in module
    init
- `crates/iscc-py/python/iscc_lib/__init__.py`: Added `DataHasher` and `InstanceHasher` wrapper
    classes with `BinaryIO` support and optional constructor data, re-exported `soft_hash_video_v0`,
    updated `__all__` with all 3 symbols
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: Added type stubs for `soft_hash_video_v0`,
    `DataHasher`, and `InstanceHasher` classes
- `tests/test_streaming.py`: New test file with 31 tests covering all 3 symbols including
    conformance vector validation

**Verification:** All 147 Python tests pass (116 existing + 31 new). Cargo clippy clean
workspace-wide. All 14 pre-commit hooks pass. All 23 Tier 1 symbols importable from `iscc_lib`.
DataHasher/InstanceHasher streaming results match gen_data_code_v0/gen_instance_code_v0 for all
conformance vectors. soft_hash_video_v0 cross-validated against iscc-core.

**Next:** The 23/23 Tier 1 Python binding milestone is complete. Next steps could be: (1) expanding
to other binding crates (Node.js already has 9 gen functions — add the algorithm primitives and
streaming hashers), (2) WASM bindings expansion, (3) documentation updates to reflect complete API,
or (4) performance benchmarks comparing streaming vs one-shot for large data.

**Notes:** The `Option<Inner>` pattern for `#[pyclass]` wrappers works cleanly — `inner.take()`
consumes the hasher on `finalize()`, and subsequent calls get a clear "already finalized"
ValueError. The Python wrapper classes delegate to `_DataHasher`/`_InstanceHasher`
(underscore-prefixed lowlevel imports) to avoid name collision. State.md needs updating to reflect
147 Python tests and 23/23 Tier 1 symbols.
