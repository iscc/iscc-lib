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

## Encoding Utilities

Two encoding functions are public Tier 1 API.

| Function           | Signature (Rust)            | Behavior                                                                                                                                                    |
| ------------------ | --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `encode_base64`    | `fn(data: &[u8]) -> String` | Standard base64 encoding (RFC 4648, no padding)                                                                                                             |
| `json_to_data_url` | `fn(json: &str) -> String`  | Accepts a JSON string, base64-encodes it, returns a `data:application/json;base64,...` data URL. Uses `application/ld+json` when `@context` key is present. |

`json_to_data_url` enables all bindings to convert native dict/object types to data URLs by
serializing to JSON once (language-specific) then delegating encoding to Rust.

**Verified when:**

- [ ] `iscc_lib::encode_base64(&[0, 1, 2, 3])` returns the correct base64 string
- [ ] `iscc_lib::json_to_data_url("{\"key\":\"value\"}")` returns
    `"data:application/json;base64,..."`
- [ ] `iscc_lib::json_to_data_url("{\"@context\":...}")` returns
    `"data:application/ld+json;base64,..."`
- [ ] Output matches iscc-core behavior for identical input
- [ ] Both functions are accessible from all bindings

## Codec Operations

Three codec functions are public Tier 1 API.

### iscc_decompose

Decomposes a composite ISCC-CODE into its constituent ISCC-UNIT strings.

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

### encode_component

Encodes a raw digest into an ISCC unit string with the correct header. Promoted from Tier 2 to Tier
1 so all bindings get it without custom codec reimplementation. Existing Rust implementation lives
in `crates/iscc-lib/src/codec.rs`.

| Function           | Signature (Rust)                                                                              | Behavior                                              |
| ------------------ | --------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| `encode_component` | `fn(mtype: u8, stype: u8, version: u8, bit_length: u32, digest: &[u8]) -> IsccResult<String>` | Construct ISCC unit from header fields and raw digest |

Uses `u8` for enum fields (not Rust enums) so bindings can pass integers directly. Each binding
provides language-idiomatic enum wrappers (Python `IntEnum`, TS `const enum`, etc.) that map to the
same values as `codec::MainType`/`SubType`/`Version`.

Rejects `MainType::Iscc` (composite codes are assembled in `gen_iscc_code_v0`). Returns error if
`digest.len() < bit_length / 8`.

**Verified when:**

- [ ] `encode_component(3, 0, 0, 64, &digest)` returns a valid Data-Code ISCC unit string
- [ ] Output matches iscc-core `encode_component` for identical inputs
- [ ] Returns error for `MainType::Iscc` (mtype=5)
- [ ] Returns error when digest is shorter than `bit_length / 8`
- [ ] Function is accessible from all bindings

### iscc_decode

Decodes an ISCC unit string into its header components and raw digest. Inverse of
`encode_component`. Existing helpers in `codec.rs` (`decode_base32`, `decode_header`,
`decode_length`) provide the building blocks.

| Function      | Signature (Rust)                                               | Behavior                                                                 |
| ------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `iscc_decode` | `fn(iscc_unit: &str) -> IsccResult<(u8, u8, u8, u8, Vec<u8>)>` | Decode ISCC unit into (maintype, subtype, version, length_index, digest) |

Strips optional `ISCC:` prefix, base32-decodes, parses header, computes bit_length via
`decode_length()`, returns exactly `bit_length / 8` bytes from the tail as digest. Returns `u8` for
enum fields (same pattern as `encode_component`).

**Verified when:**

- [ ] Round-trip: `iscc_decode(encode_component(mt, st, vs, bits, digest))` recovers original fields
- [ ] Output matches iscc-core `iscc_decode` for identical inputs
- [ ] Handles both `"ISCC:..."` prefixed and bare base32 input
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

## Algorithm Constants

Four algorithm configuration constants are public Tier 1 API, defined as `pub const` in the Rust
core. These are standardized parameters from ISO 24138 — read-only by design.

| Constant                | Type    | Value     | Description                                    |
| ----------------------- | ------- | --------- | ---------------------------------------------- |
| `META_TRIM_NAME`        | `usize` | 128       | Max UTF-8 byte length for name metadata        |
| `META_TRIM_DESCRIPTION` | `usize` | 4096      | Max UTF-8 byte length for description metadata |
| `IO_READ_SIZE`          | `usize` | 4_194_304 | Buffer size for streaming file reads (4 MB)    |
| `TEXT_NGRAM_SIZE`       | `usize` | 13        | Character n-gram width for text features       |

Each binding exposes these as module-level constants and optionally as a `core_opts` namespace
object for iscc-core API parity.

**Verified when:**

- [ ] Constants are `pub const` in the Rust core crate root
- [ ] `iscc_lib::META_TRIM_NAME == 128`
- [ ] `iscc_lib::META_TRIM_DESCRIPTION == 4096`
- [ ] `iscc_lib::IO_READ_SIZE == 4_194_304`
- [ ] `iscc_lib::TEXT_NGRAM_SIZE == 13`
- [ ] Constants are accessible from all bindings
- [ ] Python binding exposes `core_opts.meta_trim_name` (SimpleNamespace) for iscc-core parity

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
| Encoding    | `encode_base64`, `json_to_data_url`                                  |
| Codec       | `iscc_decompose`, `encode_component`, `iscc_decode`                  |
| Constants   | `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`,           |
|             | `TEXT_NGRAM_SIZE`                                                    |
| Streaming   | `DataHasher`, `InstanceHasher`                                       |
| Diagnostics | `conformance_selftest`                                               |

Total: 9 gen functions + 4 text utils + 4 algorithm primitives + 1 soft hash + 2 encoding + 3 codec
\+ 4 constants + 2 streaming types + 1 diagnostic = **30 public symbols** (+ the `codec` module
remains Tier 2 Rust-only for header encode/decode internals).

## Quality Gates

- [ ] `cargo test -p iscc-lib` passes (all existing + new tests)
- [ ] `cargo clippy -p iscc-lib -- -D warnings` clean
- [ ] `cargo fmt -p iscc-lib --check` clean
- [ ] No `unsafe` without documented justification
- [ ] Crate has zero binding dependencies (no PyO3, napi, wasm-bindgen)
- [ ] All new public functions have doc comments with examples
- [ ] All new public functions have unit tests
