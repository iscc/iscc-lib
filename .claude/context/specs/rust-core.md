# Spec: Rust Core Crate — Extended API Surface

The `iscc-lib` Rust core crate exposes a Tier 1 public API that all binding crates (Python, Node.js,
WASM, C FFI) wrap. Beyond the 9 `gen_*_v0` functions, the crate provides text utilities, algorithm
primitives, streaming hashers, codec operations, and a conformance selftest — everything that
`iscc/iscc-sdk` currently imports from `iscc-core`.

Use deepwiki MCP to query `iscc/iscc-core` for exact signatures and behavior of the reference
implementations.

## Structured Return Types

All 9 `gen_*_v0` functions return structured result types (not plain strings). Each type carries the
same fields as the corresponding iscc-core Python dict. See `specs/python-bindings.md` for the
complete field listing per function.

**Verified when:**

- [ ] Each `gen_*_v0` function returns a struct with named fields, not `String`
- [ ] All fields from `specs/python-bindings.md` are present in the corresponding struct
- [ ] Conformance tests verify additional fields (metahash, name, characters, datahash, filesize,
    parts) match iscc-core output for every test vector

## Text Utility Functions

Four text processing functions are public Tier 1 API, callable from all bindings:

| Function               | Signature (Rust)                          | Behavior                                                                                                         |
| ---------------------- | ----------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `text_clean`           | `fn(text: &str) -> String`                | NFKC normalize, remove control chars (except newlines), collapse consecutive empty lines, strip outer whitespace |
| `text_remove_newlines` | `fn(text: &str) -> String`                | Replace all whitespace sequences (including newlines) with single space                                          |
| `text_trim`            | `fn(text: &str, nbytes: usize) -> String` | Truncate to `nbytes` UTF-8 bytes at a valid character boundary, trim whitespace                                  |
| `text_collapse`        | `fn(text: &str) -> String`                | NFD normalize, lowercase, remove whitespace/control/mark/punctuation chars, NFKC recombine                       |

These functions exist internally today as `pub(crate)`. They become `pub` in `lib.rs` (not just the
utils module).

**Verified when:**

- [ ] `iscc_lib::text_clean("hello\tworld")` returns `"helloworld"`
- [ ] `iscc_lib::text_remove_newlines("hello\nworld")` returns `"hello world"`
- [ ] `iscc_lib::text_trim("hello world", 5)` returns `"hello"`
- [ ] `iscc_lib::text_trim("é", 1)` returns `""` (1-byte truncation splits 2-byte char)
- [ ] `iscc_lib::text_collapse("Hello, World!")` returns `"helloworld"`
- [ ] `iscc_lib::text_collapse("café")` returns `"cafe"`
- [ ] All four functions are accessible from Python bindings as `iscc_lib.text_clean()` etc.
- [ ] All four functions are accessible from Node.js, WASM, and C FFI bindings

## Algorithm Primitives

Four algorithm functions are public Tier 1 API, callable from all bindings. These are the building
blocks used by `iscc-sdk` for granular content processing (text chunking, text fingerprinting).

| Function          | Signature (Rust)                                         | Behavior                                                      |
| ----------------- | -------------------------------------------------------- | ------------------------------------------------------------- |
| `sliding_window`  | `fn(text: &str, width: usize) -> Vec<String>`            | Generate overlapping character n-grams of given width         |
| `alg_minhash_256` | `fn(features: &[u32]) -> Vec<u8>`                        | 64-dimensional MinHash compressed to 256-bit (32-byte) digest |
| `alg_cdc_chunks`  | `fn(data: &[u8], utf32: bool, avg_chunk_size: u32) -> …` | Content-defined chunking with gear rolling hash               |
| `alg_simhash`     | `fn(hash_digests: &[Vec<u8>]) -> Vec<u8>`                | Similarity-preserving hash from equal-length digests          |

Note: The bound `alg_cdc_chunks` returns owned data (`Vec<Vec<u8>>`) suitable for FFI. The internal
implementation may use borrowed slices for performance.

Note: The bound `alg_simhash` takes `&[Vec<u8>]` (concrete type) rather than the internal
`&[impl AsRef<[u8]>]` generic, since generics cannot cross FFI boundaries.

**Verified when:**

- [ ] `iscc_lib::sliding_window("abcde", 3)` returns `["abc", "bcd", "cde"]`
- [ ] `iscc_lib::sliding_window("ab", 3)` returns `["ab"]` (input shorter than width)
- [ ] `iscc_lib::alg_minhash_256(&[0, 1, 2, 3])` returns a 32-byte `Vec<u8>`
- [ ] `iscc_lib::alg_cdc_chunks(data, false, 1024)` returns non-empty chunk list for non-empty input
- [ ] `iscc_lib::alg_simhash(&[vec![0u8; 32]])` returns a 32-byte `Vec<u8>`
- [ ] All four functions are accessible from Python bindings
- [ ] All four functions are accessible from Node.js, WASM, and C FFI bindings
- [ ] `alg_minhash_256` output matches iscc-core `alg_minhash_256` for identical input features
- [ ] `alg_cdc_chunks` output matches iscc-core `alg_cdc_chunks` for identical input data

## Soft Hash Functions

One soft hash function is public Tier 1 API. It is used by `iscc-sdk` directly for granular video
processing (generating per-segment simprints), independent of `gen_video_code_v0`.

| Function             | Signature (Rust)                                    | Behavior                                                          |
| -------------------- | --------------------------------------------------- | ----------------------------------------------------------------- |
| `soft_hash_video_v0` | `fn(frame_sigs: &[Vec<i32>], bits: u32) -> Vec<u8>` | 256-bit similarity hash from MPEG-7 frame signatures via WTA-Hash |

**Verified when:**

- [ ] `iscc_lib::soft_hash_video_v0(frame_sigs, 256)` returns a 32-byte `Vec<u8>`
- [ ] Output matches iscc-core `soft_hash_video_v0` for identical frame signatures
- [ ] Function is accessible from all bindings

## Encoding Utility

One encoding function is public Tier 1 API. Used by `iscc-sdk` to encode simprint digests for
granular features.

| Function        | Signature (Rust)            | Behavior                                        |
| --------------- | --------------------------- | ----------------------------------------------- |
| `encode_base64` | `fn(data: &[u8]) -> String` | Standard base64 encoding (RFC 4648, no padding) |

**Verified when:**

- [ ] `iscc_lib::encode_base64(&[0, 1, 2, 3])` returns the correct base64 string
- [ ] Output matches iscc-core `encode_base64` for identical input
- [ ] Function is accessible from all bindings

## ISCC Decompose

One codec function is public Tier 1 API. Decomposes a composite ISCC-CODE into its constituent
ISCC-UNIT strings.

| Function         | Signature (Rust)                                 | Behavior                                                         |
| ---------------- | ------------------------------------------------ | ---------------------------------------------------------------- |
| `iscc_decompose` | `fn(iscc_code: &str) -> IsccResult<Vec<String>>` | Split composite ISCC-CODE into individual ISCC-UNIT code strings |

The input is a composite ISCC-CODE string (with or without `"ISCC:"` prefix). The output is a list
of ISCC-UNIT strings, each with `"ISCC:"` prefix, ordered by MainType (Meta, Semantic, Content,
Data, Instance). For non-composite ISCC-UNITs, returns a single-element list containing the input.

**Verified when:**

- [ ] `iscc_decompose("ISCC:KEC...")` returns a `Vec<String>` with 4 elements (Meta, Content, Data,
    Instance)
- [ ] Each returned element starts with `"ISCC:"`
- [ ] Output matches iscc-core `iscc_decompose` for identical input
- [ ] Single ISCC-UNITs (non-composite) return a single-element list
- [ ] Function is accessible from all bindings

## Streaming Hashers

Two streaming processor types are public Tier 1 API. They enable processing large files without
loading them entirely into memory. Used by `iscc-sdk` in `code_sum` to generate Data-Code and
Instance-Code from file streams.

| Type             | API Pattern                                                      | Produces                            |
| ---------------- | ---------------------------------------------------------------- | ----------------------------------- |
| `DataHasher`     | `new() -> update(&[u8]) -> finalize(bits) -> DataCodeResult`     | Data-Code + datahash                |
| `InstanceHasher` | `new() -> update(&[u8]) -> finalize(bits) -> InstanceCodeResult` | Instance-Code + datahash + filesize |

Both types accumulate data via repeated `update()` calls, then produce the final ISCC code via
`finalize()`. The result types carry the same fields as the corresponding `gen_*_v0` structured
returns.

`DataHasher` internally accumulates CDC chunk features for MinHash. `InstanceHasher` internally
accumulates a BLAKE3 streaming hash and byte count.

**Verified when:**

- [ ] `DataHasher::new().update(data).finalize(64)` produces the same ISCC as
    `gen_data_code_v0(data, 64)`
- [ ] `InstanceHasher::new().update(data).finalize(64)` produces the same ISCC as
    `gen_instance_code_v0(data, 64)`
- [ ] Splitting data across multiple `update()` calls produces identical output to a single call
- [ ] `InstanceHasher` result includes correct `datahash` and `filesize`
- [ ] Both types are accessible from Python as classes with `update()` and `finalize()` methods
- [ ] Both types accept file-like objects in Python (anything with `.read()`)
- [ ] Both types are accessible from Node.js, WASM, and C FFI bindings

## Conformance Selftest

One diagnostic function is public Tier 1 API. Runs all vendored conformance vectors and reports
pass/fail.

| Function               | Signature (Rust) | Behavior                                               |
| ---------------------- | ---------------- | ------------------------------------------------------ |
| `conformance_selftest` | `fn() -> bool`   | Run all conformance vectors, return `true` if all pass |

**Verified when:**

- [ ] `iscc_lib::conformance_selftest()` returns `true`
- [ ] Function is accessible from all bindings
- [ ] `iscc_lib.conformance_selftest()` returns `True` in Python

## Complete Tier 1 API Summary

The full public API surface bound in all languages:

| Category    | Functions / Types                                                    |
| ----------- | -------------------------------------------------------------------- |
| Code gen    | `gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`,         |
|             | `gen_audio_code_v0`, `gen_video_code_v0`, `gen_mixed_code_v0`,       |
|             | `gen_data_code_v0`, `gen_instance_code_v0`, `gen_iscc_code_v0`       |
| Text utils  | `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`   |
| Algorithms  | `sliding_window`, `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash` |
| Soft hashes | `soft_hash_video_v0`                                                 |
| Encoding    | `encode_base64`                                                      |
| Codec       | `iscc_decompose`                                                     |
| Streaming   | `DataHasher`, `InstanceHasher`                                       |
| Diagnostics | `conformance_selftest`                                               |

Total: 9 gen functions + 4 text utils + 4 algorithm primitives + 1 soft hash + 1 encoding + 1 codec

- 2 streaming types + 1 diagnostic = **22 public symbols** (+ the `codec` module remains Tier 2
    Rust-only).

## Quality Gates

- [ ] `cargo test -p iscc-lib` passes (all existing + new tests)
- [ ] `cargo clippy -p iscc-lib -- -D warnings` clean
- [ ] `cargo fmt -p iscc-lib --check` clean
- [ ] No `unsafe` without documented justification
- [ ] Crate has zero binding dependencies (no PyO3, napi, wasm-bindgen)
- [ ] All new public functions have doc comments with examples
- [ ] All new public functions have unit tests
