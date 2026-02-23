## 2026-02-23 — Add alg_simhash, alg_minhash_256, and alg_cdc_chunks to Python bindings

**Done:** Added three algorithm primitive functions (`alg_simhash`, `alg_minhash_256`,
`alg_cdc_chunks`) to the Python bindings, bringing the count from 17/23 to 20/23 Tier 1 symbols. All
three are thin PyO3 wrappers following the established pattern, with direct re-exports in
`__init__.py` (no `IsccResult` wrapping needed since they return raw `bytes`/`list[bytes]`).

**Files changed:**

- `crates/iscc-py/src/lib.rs`: Added 3 `#[pyfunction]` wrappers + registered in module
- `crates/iscc-py/python/iscc_lib/__init__.py`: Added 3 re-exports + 3 entries in `__all__`
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: Added type stubs with docstrings for 3 functions
- `tests/test_algo.py`: New test file with 18 tests covering all three functions

**Verification:** All gates pass:

- `maturin develop` builds successfully
- `cargo test -p iscc-lib`: 230 tests pass (180 unit + 28 + 22 integration), no regressions
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `pytest tests/`: 116 tests pass (18 new + 98 existing)
- All three functions importable from Python and return correct types
- Cross-validation against iscc-core passes for all three functions (byte-for-byte match)
- `mise run check`: all 14 pre-commit hooks pass

**Next:** Remaining 3 Tier 1 symbols: `soft_hash_video_v0`, `DataHasher`, `InstanceHasher`. The
streaming types (`DataHasher`, `InstanceHasher`) require PyO3 `#[pyclass]` wrappers with
`update()`/`finalize()` methods — more complex than the function wrappers done so far.
`soft_hash_video_v0` is a pure function but may not be commonly needed externally. Alternatively,
expand horizontally to other binding crates (Node.js, WASM, C FFI algo primitives).

**Notes:** No surprises. The `alg_cdc_chunks` wrapper converts borrowed `Vec<&[u8]>` to owned
`Vec<Vec<u8>>` for Python, matching the established pattern. The `alg_simhash` wrapper accepts
`Vec<Vec<u8>>` which satisfies Rust's `&[impl AsRef<[u8]>]` requirement. Default parameter for
`avg_chunk_size=1024` uses `#[pyo3(signature)]` attribute.
