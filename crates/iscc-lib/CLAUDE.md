# CLAUDE.md — iscc-lib core crate

Pure Rust implementation of ISO 24138:2024 (ISCC) -- the hub of the hub-and-spoke workspace model.

## Crate Role

- This is the **core crate** that all binding crates (`iscc-py`, `iscc-napi`, `iscc-wasm`,
    `iscc-ffi`, `iscc-jni`) depend on
- Must remain **pure Rust** -- zero binding dependencies (no PyO3, napi, wasm-bindgen)
- Publishable to crates.io as `iscc-lib`
- All ISCC algorithm logic lives here; binding crates only translate the API

## Module Layout

| Module           | Visibility   | Purpose                                                                                              |
| ---------------- | ------------ | ---------------------------------------------------------------------------------------------------- |
| `lib.rs`         | public       | Crate root: re-exports, `IsccError`/`IsccResult`, all 9 `gen_*_v0` functions                         |
| `types.rs`       | `pub`        | Result structs (`MetaCodeResult`, `TextCodeResult`, etc.)                                            |
| `codec.rs`       | `pub`        | Tier 2: `MainType`/`SubType`/`Version` enums, header encode/decode, base32, `iscc_decompose`         |
| `streaming.rs`   | `pub`        | `DataHasher` and `InstanceHasher` streaming types                                                    |
| `conformance.rs` | `pub`        | `conformance_selftest()` -- runtime conformance checker                                              |
| `utils.rs`       | `pub`        | `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`, `multi_hash_blake3` (pub(crate)) |
| `cdc.rs`         | `pub`        | Content-defined chunking (`alg_cdc_chunks`)                                                          |
| `minhash.rs`     | `pub`        | MinHash algorithm (`alg_minhash_256`)                                                                |
| `simhash.rs`     | `pub`        | SimHash and sliding window (`alg_simhash`, `sliding_window`)                                         |
| `dct.rs`         | `pub(crate)` | Discrete Cosine Transform for image hashing                                                          |
| `wtahash.rs`     | `pub(crate)` | Winner-Take-All hash for video fingerprinting                                                        |

## API Tier Rules

**Tier 1 -- public API, bound in all languages (30 symbols at crate root):**

- 9 gen functions: `gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`, `gen_audio_code_v0`,
    `gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`, `gen_instance_code_v0`,
    `gen_iscc_code_v0`
- 4 text utilities: `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
- 4 algorithm primitives: `sliding_window`, `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`
- 1 soft hash: `soft_hash_video_v0`
- 2 encoding utilities: `encode_base64`, `json_to_data_url`
- 3 codec operations: `iscc_decompose`, `encode_component`, `iscc_decode`
- 4 algorithm constants: `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`,
    `TEXT_NGRAM_SIZE`
- 2 streaming types: `DataHasher`, `InstanceHasher`
- 1 diagnostic: `conformance_selftest`

**Tier 2 -- `pub mod codec`, Rust-only, not exposed through bindings:**

- `MainType`, `SubType`, `Version` enums
- `encode_header`, `decode_header`, `encode_base32`, `decode_base32`
- `encode_length`, `decode_length`, `encode_units`, `decode_units`

**Internal -- `pub(crate)` or private, free to change:**

- `dct::alg_dct`, `wtahash::alg_wtahash`
- Helper functions in `lib.rs` (`interleave_digests`, `soft_hash_meta_v0`, `soft_hash_text_v0`,
    `soft_hash_audio_v0`, `soft_hash_codes_v0`, etc.)
- `utils::multi_hash_blake3`

**Rules:**

- Adding to Tier 1 requires deliberate review
- Bindings translate the API; they must not define semantics
- Do not expose Rust enums directly in bindings (they break when variants are added)
- All result structs use `#[non_exhaustive]` so fields can be added in minor versions

## Sync Core Design Pattern

All operations are CPU-bound. The core is synchronous -- no async, no tokio.

**One-shot functions:** `gen_*_v0(inputs, bits) -> IsccResult<*CodeResult>`

**Streaming types** (for Data-Code and Instance-Code on large files):

```
new() -> update(&[u8]) -> ... -> update(&[u8]) -> finalize(bits) -> IsccResult<*CodeResult>
```

- `DataHasher`: CDC + xxh32 + MinHash, tail buffering between `update()` calls
- `InstanceHasher`: BLAKE3 incremental hashing
- Both must produce output identical to their one-shot counterparts for the same byte stream

## Key Types and Traits

- `IsccError` -- single error enum (`InvalidInput` variant), defined in `lib.rs`
- `IsccResult<T>` -- type alias for `Result<T, IsccError>`
- `*CodeResult` structs in `types.rs` -- each `gen_*_v0` returns a dedicated struct
    - All carry an `iscc: String` field (the ISCC code with `"ISCC:"` prefix)
    - Some carry additional fields: `name`, `description`, `meta`, `metahash`, `characters`,
        `datahash`, `filesize`, `parts`
- `codec::MainType`, `codec::SubType`, `codec::Version` -- `#[repr(u8)]` enums with `TryFrom<u8>`

## Conformance Rules

- Correctness baseline: vendored `tests/data.json` from `iscc-core`
- All 9 `gen_*_v0` functions must match `iscc-core` output for every test vector
- `conformance_selftest()` runs all vectors at runtime (non-panicking, returns bool)
- Each `gen_*_v0` also has `#[test]` functions that run the same vectors and panic on mismatch
- Streaming types (`DataHasher`/`InstanceHasher`) are tested against one-shot functions at multiple
    chunk sizes including byte-at-a-time
- No network access in tests -- all vectors are `include_str!` from `tests/data.json`
- `stream:` prefix in vector data means hex-encoded bytes; decode with `hex::decode`

## Build and Test Commands

```bash
cargo test -p iscc-lib                      # All tests (unit + conformance)
cargo test -p iscc-lib -- --test-threads=1  # Sequential (for debugging)
cargo clippy -p iscc-lib -- -D warnings     # Lint
cargo fmt -p iscc-lib --check               # Format check
cargo bench -p iscc-lib                     # Criterion benchmarks
```

## Dependencies

- `blake3` -- BLAKE3 hashing (Instance-Code, metahash)
- `xxhash-rust` -- xxh32 feature hashing (Data-Code, Text-Code)
- `data-encoding` -- base32/base64 encoding
- `hex` -- hex encode/decode for conformance vector parsing
- `unicode-normalization` -- NFKC/NFD normalization
- `unicode-general-category` -- Unicode category classification for text filtering
- `thiserror` -- error derive macro
- `serde_json` -- JSON parsing for meta processing and conformance vectors
- `serde_json_canonicalizer` -- RFC 8785 (JCS) compliant JSON serialization for metadata
    canonicalization
- `serde` + `criterion` -- dev-dependencies only

## Common Pitfalls

- **Do not add async or tokio** -- the core is synchronous by design. Async adaptation happens in
    binding crates.
- **Do not add PyO3, napi, or wasm-bindgen** -- this crate must stay pure Rust.
- **Do not make `pub(crate)` modules `pub`** without updating the tier documentation and getting
    review.
- **`SubType::TEXT` is an alias for `SubType::None`** (both value 0) -- use `SubType::TEXT` in
    Content-Code context for clarity.
- **`encode_component` rejects `MainType::Iscc`** -- ISCC-CODEs are composites assembled directly in
    `gen_iscc_code_v0`.
- **`bits` parameter** is always in bits (64, 128, 256), not bytes. The codec encodes it as
    `(bits / 32) - 1`.
- **Result structs are `#[non_exhaustive]`** -- do not construct them with struct literal syntax
    outside this crate; use the `gen_*_v0` functions.
- **`DataHasher.update()` tail buffering** -- the last CDC chunk is retained as a tail until
    `finalize()` or the next `update()`. This matches the Python `push()` / `prev_chunk` pattern.
- **Conformance vector format** -- `data.json` inputs use `"stream:<hex>"` for byte data, JSON
    arrays for integer vectors, and plain strings for text. Read the test code for parsing patterns.
- **Text normalization order matters** -- `text_clean` applies NFKC first, then control-char
    removal, then line collapsing. `text_collapse` applies NFD, lowercase, filter C/M/P categories,
    then NFKC. Do not reorder these steps.
- **JSON canonicalization** -- the crate uses `serde_json_canonicalizer` for RFC 8785 (JCS)
    compliant serialization of JSON metadata, matching iscc-core's `jcs.canonicalize()` behavior.
