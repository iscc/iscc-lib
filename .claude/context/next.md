# Next Work Package

## Step: Add 4 algorithm primitives to C FFI bindings

## Goal

Expand C FFI bindings from 17/23 to 21/23 Tier 1 symbols by adding `alg_simhash`, `alg_minhash_256`,
`alg_cdc_chunks`, and `soft_hash_video_v0`. This brings C FFI to parity with Node.js and WASM (both
at 21/23). Requires new `#[repr(C)]` types for byte buffer returns since all 4 functions return byte
arrays — a new pattern for the FFI crate.

## Scope

- **Modify**: `crates/iscc-ffi/src/lib.rs` — add 2 `#[repr(C)]` types (`IsccByteBuffer`,
    `IsccByteBufferArray`), 4 FFI functions, 2 free functions, Rust unit tests
- **Modify**: `crates/iscc-ffi/tests/test_iscc.c` — add C-level test cases for the 4 new functions
- **Reference**: `crates/iscc-lib/src/simhash.rs` (`alg_simhash` signature:
    `&[impl AsRef<[u8]>]   -> Vec<u8>`), `crates/iscc-lib/src/minhash.rs` (`alg_minhash_256`
    signature: `&[u32] ->   Vec<u8>`), `crates/iscc-lib/src/cdc.rs` (`alg_cdc_chunks` signature:
    `(&[u8], bool, u32) ->   Vec<&[u8]>`), `crates/iscc-lib/src/lib.rs` line 530
    (`soft_hash_video_v0` signature: `(&[Vec<i32>], u32) -> IsccResult<Vec<u8>>`),
    `crates/iscc-napi/src/lib.rs` (Node.js equivalents for reference),
    `crates/iscc-ffi/cbindgen.toml` (cbindgen config)

## Not In Scope

- `DataHasher`/`InstanceHasher` streaming classes for C FFI — separate step after reaching 21/23
- Adding these functions to other bindings — Node.js and WASM already have them
- Structured return types for gen functions (returning full result objects instead of `.iscc`
    strings)
- Changes to the Rust core crate — all 4 functions are already exported and tested
- Updating `cbindgen.toml` — `#[repr(C)]` types with `iscc_` prefix export should work automatically

## Implementation Notes

### New C-compatible types

Add two `#[repr(C)]` structs for returning byte arrays across the FFI boundary:

```rust
/// Heap-allocated byte buffer returned to C callers.
#[repr(C)]
pub struct IsccByteBuffer {
    /// Pointer to the byte data (NULL if error).
    pub data: *mut u8,
    /// Number of bytes.
    pub len: usize,
}

/// Array of byte buffers (for alg_cdc_chunks).
#[repr(C)]
pub struct IsccByteBufferArray {
    /// Pointer to array of IsccByteBuffer elements.
    pub buffers: *mut IsccByteBuffer,
    /// Number of buffers.
    pub count: usize,
}
```

### Helper function

Add `vec_to_byte_buffer(v: Vec<u8>) -> IsccByteBuffer` helper that converts owned `Vec<u8>` to the
C-compatible struct (shrink_to_fit + as_mut_ptr + mem::forget, same pattern as
`vec_to_c_string_array`).

Add a null/error `IsccByteBuffer` constant or function:
`IsccByteBuffer { data: ptr::null_mut(), len: 0 }`.

### FFI function signatures

1. **`iscc_alg_simhash`** — input is array of byte buffers (digests)

    ```c
    IsccByteBuffer iscc_alg_simhash(
        const uint8_t *const *digests,  // array of pointers to digest bytes
        const size_t *digest_lens,       // array of lengths for each digest
        size_t num_digests               // number of digests
    );
    ```

    - Validate: if `digests` or `digest_lens` is NULL (and `num_digests > 0`), set error and return
        null buffer
    - Reconstruct `Vec<&[u8]>` from pointer pairs via `from_raw_parts`
    - Call `iscc_lib::alg_simhash(&slices)` — `&[u8]` implements `AsRef<[u8]>`
    - Convert result `Vec<u8>` to `IsccByteBuffer`

2. **`iscc_alg_minhash_256`** — input is `u32` array

    ```c
    IsccByteBuffer iscc_alg_minhash_256(
        const uint32_t *features,
        size_t features_len
    );
    ```

    - Validate null pointer (when `features_len > 0`)
    - `from_raw_parts` to `&[u32]`
    - Call `iscc_lib::alg_minhash_256(features)`
    - Convert 32-byte result to `IsccByteBuffer`

3. **`iscc_alg_cdc_chunks`** — returns array of byte chunks

    ```c
    IsccByteBufferArray iscc_alg_cdc_chunks(
        const uint8_t *data,
        size_t data_len,
        bool utf32,
        uint32_t avg_chunk_size
    );
    ```

    - Validate null pointer (when `data_len > 0`)
    - Call `iscc_lib::alg_cdc_chunks(data, utf32, avg_chunk_size)`
    - Convert each chunk `&[u8]` to owned `Vec<u8>` then to `IsccByteBuffer`
    - Collect into `Vec<IsccByteBuffer>`, shrink_to_fit, as_mut_ptr, mem::forget
    - Return `IsccByteBufferArray { buffers, count }`

4. **`iscc_soft_hash_video_v0`** — same frame_sigs pattern as `iscc_gen_video_code_v0`

    ```c
    IsccByteBuffer iscc_soft_hash_video_v0(
        const int32_t *const *frame_sigs,
        const size_t *frame_lens,
        size_t num_frames,
        uint32_t bits
    );
    ```

    - Same input validation/reconstruction as `iscc_gen_video_code_v0`
    - Call `iscc_lib::soft_hash_video_v0(&frames, bits)`
    - On error, set_last_error and return null buffer
    - On success, convert `Vec<u8>` to `IsccByteBuffer`

### Free functions

```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_free_byte_buffer(buf: IsccByteBuffer) { ... }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_free_byte_buffer_array(arr: IsccByteBufferArray) { ... }
```

- `iscc_free_byte_buffer`: reconstruct `Vec<u8>` from `data`/`len` and drop. No-op if `data` is
    NULL.
- `iscc_free_byte_buffer_array`: free each buffer's data, then free the array itself. No-op if
    `buffers` is NULL.

### Rust unit tests

Add tests in the existing `#[cfg(test)] mod tests` block:

- `test_alg_simhash_basic` — feed known digests, verify output length matches input digest size
- `test_alg_simhash_null` — NULL digests with count > 0 returns null buffer
- `test_alg_minhash_256_basic` — feed u32 features, verify output is 32 bytes
- `test_alg_minhash_256_null` — NULL features with len > 0 returns null buffer
- `test_alg_cdc_chunks_basic` — feed data, verify chunks concatenate to original
- `test_alg_cdc_chunks_null` — NULL data with len > 0 returns null array
- `test_soft_hash_video_v0_basic` — feed frame sigs, verify output length = bits/8
- `test_soft_hash_video_v0_null` — NULL frame_sigs returns null buffer
- `test_free_byte_buffer_null` — no-op safety test
- `test_free_byte_buffer_array_null` — no-op safety test

### C test cases

Add to `tests/test_iscc.c` after existing tests (before summary). Need `ASSERT_EQ` macro for integer
comparisons and byte buffer assertions:

- `alg_minhash_256` — feed known features, check `.len == 32` and `.data != NULL`, free buffer
- `alg_simhash` — feed single digest, check output length matches input, free buffer
- `alg_cdc_chunks` — feed "Hello World", check `.count >= 1`, verify chunk data, free array
- `soft_hash_video_v0` — feed minimal frame sigs, check `.len == 8` (default 64 bits), free buffer

### Edge cases

- For null pointer checks: when `num_digests == 0` or `features_len == 0`, allow NULL pointers
    (create empty slices with `&[]` or
    `std::slice::from_raw_parts(ptr::NonNull::dangling().as_ptr(),   0)`)
- `alg_cdc_chunks` always returns at least one chunk (empty bytes for empty input) — test this
- The `iscc_free_byte_buffer` must handle the pattern: reconstruct Vec via
    `Vec::from_raw_parts(data, len, len)` (capacity == len because of `shrink_to_fit`)

## Verification

- `cargo build -p iscc-ffi` compiles without errors
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `cargo test -p iscc-ffi` passes all tests (38 existing + ~10 new)
- `cargo test --workspace` passes with 268+ tests (no regressions)
- `cbindgen --crate iscc-ffi` generates valid C header containing `IsccByteBuffer`,
    `IsccByteBufferArray`, and all 4 new function declarations

## Done When

All verification criteria pass: 4 new FFI functions compile, cbindgen generates correct C header
types, clippy is clean workspace-wide, and all FFI tests (existing 38 + new algorithm primitive
tests) pass.
