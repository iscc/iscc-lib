# Next Work Package

## Step: Implement DataHasher and InstanceHasher streaming types

## Goal

Add `DataHasher` and `InstanceHasher` — the two remaining functional Tier 1 symbols (21 and 22 of
22). These provide the streaming `new() → update(&[u8]) → finalize()` API pattern central to the
library's design, enabling incremental processing of large files without loading entire contents
into memory.

## Scope

- **Create**: `crates/iscc-lib/src/streaming.rs` (both structs, impls, and tests)
- **Modify**: `crates/iscc-lib/src/lib.rs` (add `pub mod streaming;` +
    `pub use   streaming::{DataHasher, InstanceHasher};` re-exports)
- **Reference**: `reference/iscc-core/iscc_core/code_data.py` (DataHasherV0 class, lines 70-136),
    `reference/iscc-core/iscc_core/code_instance.py` (InstanceHasherV0 class, lines 73-135),
    `notes/03-async-and-streaming.md` (API shape specification), `crates/iscc-lib/src/lib.rs`
    (existing `gen_data_code_v0` and `gen_instance_code_v0` for algorithm reference),
    `crates/iscc-lib/src/cdc.rs` (`alg_cdc_chunks`, `DATA_AVG_CHUNK_SIZE`),
    `crates/iscc-lib/src/minhash.rs` (`alg_minhash_256`), `crates/iscc-lib/src/utils.rs`
    (`multi_hash_blake3`)

## Implementation Notes

### InstanceHasher (simpler — implement first)

Port from Python `InstanceHasherV0` (code_instance.py lines 73-135).

Fields: `hasher: blake3::Hasher` and `filesize: u64`.

- `new()` — initialize `blake3::Hasher::new()` and `filesize: 0`
- `update(&mut self, data: &[u8])` — increment filesize by data.len(), call hasher.update(data)
- `finalize(self, bits: u32) -> IsccResult<InstanceCodeResult>` — call `self.hasher.finalize()` to
    get the digest, then replicate the logic from `gen_instance_code_v0`. Format the multihash
    manually from the digest: `format!("1e20{}", hex::encode(digest.as_bytes()))`. Then call
    `encode_component` and prefix with "ISCC:".

**Important**: The existing `gen_instance_code_v0` calls both `blake3::hash(data)` (for
encode_component) and `multi_hash_blake3(data)` (for datahash). Both produce the same BLAKE3 hash.
In streaming mode, we only have one `blake3::Hasher`, which gives us one digest — use it for both.

Also implement `Default` for `InstanceHasher` (delegates to `new()`).

### DataHasher (more complex — CDC tail handling)

Port from Python `DataHasherV0` (code_data.py lines 70-136).

Fields: `chunk_features: Vec<u32>` and `tail: Vec<u8>`.

- `new()` — initialize empty `chunk_features` and empty `tail`

- `update(&mut self, data: &[u8])` — port from Python `push()`:

    1. Prepend tail: `let combined = [self.tail.as_slice(), data].concat();` (if tail is empty, this
        is just data)
    2. Run CDC: `let chunks = cdc::alg_cdc_chunks(&combined, false, cdc::DATA_AVG_CHUNK_SIZE);`
    3. Process all chunks except the last — hash each with `xxhash_rust::xxh32::xxh32(chunk, 0)` and
        push to `self.chunk_features`
    4. The last chunk becomes the new tail:
        `self.tail = chunks.last().unwrap_or(&b"".as_slice()).to_vec();`
    5. If chunks is empty (shouldn't happen with CDC, but defensive), set tail to combined

    **Critical detail from Python**: In `push()`, iterate chunks and only process `prev_chunk` (the
    chunk *before* the current one). This means all chunks except the last get hashed. The last
    chunk becomes the tail. This handles the boundary case where CDC splits right at the boundary of
    two `update()` calls — the tail carries over to the next call.

- `finalize(self, bits: u32) -> IsccResult<DataCodeResult>` — port from Python `_finalize()` and
    `code()`:

    1. If `self.tail` is not empty, hash it and append to features:
        `self.chunk_features.push(xxhash_rust::xxh32::xxh32(&self.tail, 0));`
    2. If tail is empty AND features is empty (empty input case), push hash of empty bytes:
        `self.chunk_features.push(xxhash_rust::xxh32::xxh32(b"", 0));`
    3. Compute digest: `minhash::alg_minhash_256(&self.chunk_features)`
    4. Encode: `codec::encode_component(MainType::Data, SubType::None, Version::V0, bits, &digest)`
    5. Return `DataCodeResult { iscc: format!("ISCC:{component}") }`

Also implement `Default` for `DataHasher` (delegates to `new()`).

### Module setup

In `streaming.rs`, add the file-level docstring:
`//! Streaming hash types for incremental ISCC code generation.`

Use `use crate::{...}` imports to access `cdc`, `codec`, `minhash`, `utils`, types, and errors.

In `lib.rs`:

- Add `pub mod streaming;` alongside other module declarations
- Add `pub use streaming::{DataHasher, InstanceHasher};` alongside other re-exports

### Tests

Add `#[cfg(test)] mod tests` in `streaming.rs`:

1. **InstanceHasher equivalence tests**: Feed the same data through `InstanceHasher` (streaming) and
    `gen_instance_code_v0` (one-shot). Verify identical `iscc`, `datahash`, and `filesize` fields.
    Test cases:

    - Empty data: single `finalize()` with no `update()`
    - Small data: single `update()` call
    - Multi-chunk: split data across 3+ `update()` calls
    - Conformance vectors: use the `gen_instance_code_v0` test vectors from `data.json`

2. **DataHasher equivalence tests**: Feed the same data through `DataHasher` (streaming) and
    `gen_data_code_v0` (one-shot). Verify identical `iscc` field. Test cases:

    - Empty data: single `finalize()` with no `update()`
    - Small data: single `update()` call
    - Multi-chunk: split data into varying chunk sizes (1-byte, 256-byte, 1024-byte chunks)
    - Conformance vectors: use the `gen_data_code_v0` test vectors from `data.json`

3. **Edge cases**:

    - `DataHasher::new()` followed immediately by `finalize(64)` (empty input)
    - `InstanceHasher::new()` followed immediately by `finalize(64)` (empty input)
    - Various `bits` values (64, 128, 256)

## Verification

- `cargo test -p iscc-lib` passes (all 214 existing tests + new streaming tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `DataHasher` produces identical output to `gen_data_code_v0` for all conformance vectors
- `InstanceHasher` produces identical output to `gen_instance_code_v0` for all conformance vectors
- Multi-chunk streaming produces same results as single-shot for both hashers
- Empty input handling works for both hashers
- `DataHasher` and `InstanceHasher` are importable as `iscc_lib::DataHasher` /
    `iscc_lib::InstanceHasher`

## Done When

All verification criteria pass — both streaming types produce output identical to their one-shot
counterparts across all conformance vectors and edge cases, and are publicly accessible as Tier 1
API.
