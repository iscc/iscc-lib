# Next Work Package

## Step: Add alg_simhash, alg_minhash_256, and alg_cdc_chunks to Python bindings

## Goal

Expose the three algorithm primitive functions in the Python bindings, bringing the count from 17/23
to 20/23 Tier 1 symbols. These are the next-simplest batch — pure functions with straightforward
type mappings.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-py/src/lib.rs` — add 3 `#[pyfunction]` wrappers + register in module
    - `crates/iscc-py/python/iscc_lib/__init__.py` — re-export 3 functions + add to `__all__`
    - `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` — add type stubs for 3 functions
- **Reference**:
    - `crates/iscc-lib/src/simhash.rs` — Rust `alg_simhash` signature
    - `crates/iscc-lib/src/minhash.rs` — Rust `alg_minhash_256` signature
    - `crates/iscc-lib/src/cdc.rs` — Rust `alg_cdc_chunks` signature
    - `reference/iscc-core/iscc_core/simhash.py` — Python reference API
    - `reference/iscc-core/iscc_core/minhash.py` — Python reference API
    - `reference/iscc-core/iscc_core/cdc.py` — Python reference API

## Implementation Notes

Follow the established thin-wrapper pattern used by all other bindings in `lib.rs`.

### `alg_simhash`

- Rust signature: `pub fn alg_simhash(hash_digests: &[impl AsRef<[u8]>]) -> Vec<u8>`
- Python reference: `alg_simhash(hash_digests: list[bytes]) -> bytes`
- PyO3 wrapper: accept `Vec<Vec<u8>>`, pass directly to `iscc_lib::alg_simhash(&hash_digests)`.
    `Vec<Vec<u8>>` satisfies `&[impl AsRef<[u8]>]`. Returns `Vec<u8>` which PyO3 auto-converts to
    `bytes`.
- No default parameters needed. No error handling needed (function doesn't return Result, but does
    handle empty input gracefully by returning zero bytes).

### `alg_minhash_256`

- Rust signature: `pub fn alg_minhash_256(features: &[u32]) -> Vec<u8>`
- Python reference: `alg_minhash_256(features: List[int]) -> bytes`
- PyO3 wrapper: accept `Vec<u32>`, pass `&features` to `iscc_lib::alg_minhash_256`. Returns
    `Vec<u8>` → `bytes`.
- No default parameters. No Result return. Straightforward passthrough.

### `alg_cdc_chunks`

- Rust signature:
    `pub fn alg_cdc_chunks(data: &[u8], utf32: bool, avg_chunk_size: u32) -> Vec<&[u8]>`
- Python reference: `alg_cdc_chunks(data, utf32, avg_chunk_size=1024) -> Generator[bytes]`
- PyO3 wrapper: accept `data: &[u8], utf32: bool, avg_chunk_size: u32` with
    `#[pyo3(signature = (data, utf32, avg_chunk_size=1024))]` for the default.
- The Rust function returns `Vec<&[u8]>` (borrowed from input). Convert to owned for Python:
    `iscc_lib::alg_cdc_chunks(data, utf32, avg_chunk_size).into_iter().map(|c| c.to_vec()).collect::<Vec<Vec<u8>>>()`
    PyO3 converts `Vec<Vec<u8>>` to `list[bytes]`.
- iscc-core returns a generator, but returning `list[bytes]` is acceptable (the caller typically
    collects all chunks anyway).

### Python wrapper (`__init__.py`)

These are simple passthrough functions — no `IsccResult` wrapping needed. Re-export directly from
`_lowlevel` (same pattern as `encode_base64`, `sliding_window`, `text_clean`, etc.):

```python
from iscc_lib._lowlevel import (
    alg_simhash as alg_simhash,
    alg_minhash_256 as alg_minhash_256,
    alg_cdc_chunks as alg_cdc_chunks,
    ...
)
```

Add all three to `__all__`.

### Type stubs (`_lowlevel.pyi`)

Add signatures matching Python reference:

- `def alg_simhash(hash_digests: list[bytes]) -> bytes`
- `def alg_minhash_256(features: list[int]) -> bytes`
- `def alg_cdc_chunks(data: bytes, utf32: bool, avg_chunk_size: int = 1024) -> list[bytes]`

### Tests

Add tests in `tests/` (e.g., extend existing test file or create `tests/test_algo.py`):

- `alg_simhash`: test with known digests, verify output is bytes of correct length
- `alg_simhash`: empty input returns 32 zero bytes
- `alg_minhash_256`: test with known u32 features, verify output is 32 bytes
- `alg_cdc_chunks`: test with known data, verify chunk count and that chunks concatenate back to
    original data
- `alg_cdc_chunks`: empty data returns `[b""]` (one empty chunk)
- `alg_cdc_chunks`: default avg_chunk_size works (omit parameter)

Cross-validate against iscc-core if possible: call iscc-core's `alg_simhash` / `alg_minhash_256` /
`alg_cdc_chunks` with the same inputs and compare outputs byte-for-byte.

Build with: `VIRTUAL_ENV=/home/dev/.venvs/iscc-lib maturin develop -m crates/iscc-py/Cargo.toml`

## Verification

- `maturin develop` builds successfully
- `cargo test -p iscc-lib` passes (250 tests, no regressions)
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `pytest tests/` passes with new algo tests included
- `from iscc_lib import alg_simhash, alg_minhash_256, alg_cdc_chunks` works in Python
- All three functions produce output matching iscc-core for the same inputs
- `ty` check passes on the Python package (stubs present in `_lowlevel.pyi`)

## Done When

All verification criteria pass: the three algo primitive functions are callable from Python, produce
correct output matching iscc-core, and all existing tests continue to pass.
